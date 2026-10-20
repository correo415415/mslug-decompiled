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

/// Hardware de la Neo Geo (registros con nombre de la documentación de
/// neogeodev).
pub mod hw {
    /// `REG_VRAMADDR`: dirección de VRAM para el siguiente acceso.
    pub const VRAM_ADDR: u32 = 0x3C_0000;
    /// `REG_VRAMRW`: lee/escribe la VRAM y avanza `VRAM_ADDR` en `VRAM_MOD`.
    pub const VRAM_RW: u32 = 0x3C_0002;
    /// `REG_VRAMMOD`: incremento de la dirección de VRAM.
    pub const VRAM_MOD: u32 = 0x3C_0004;
    /// `REG_LSPCMODE`: modo del chip de vídeo (al leer: línea, bit 3 = PAL).
    pub const LSPC_MODE: u32 = 0x3C_0006;
    /// Bit de `LSPC_MODE` (lectura): sistema de 50 Hz.
    pub const LSPC_MODE_PAL: u16 = 1 << 3;
    /// Bit de `LSPC_MODE` (escritura) que se quita mientras trabaja la BIOS.
    pub const LSPC_MODE_TIMER_IRQ: u16 = 1 << 4;
    /// `REG_IRQACK`: reconoce interrupciones.
    pub const IRQ_ACK: u32 = 0x3C_000C;
    pub const IRQ_ACK_TIMER: u16 = 2;
    pub const IRQ_ACK_VBLANK: u16 = 4;
    pub const IRQ_ACK_ALL: u16 = 7;
    /// Cualquier escritura reinicia el perro guardián.
    pub const WATCHDOG: u32 = 0x30_0001;
    /// `REG_SOUND`: escribir = orden al Z80; leer = respuesta del Z80.
    pub const SOUND: u32 = 0x32_0000;
    /// Orden de sonido: sonido de moneda.
    pub const SOUND_CMD_COIN: u8 = 0x0F;
    /// RAM de paleta (banco seleccionado): 256 paletas de 16 colores.
    pub const PALETTE_RAM: u32 = 0x40_0000;
    /// Último color de la paleta 255 (color de fondo de la pantalla).
    pub const BACKDROP_COLOR: u32 = 0x40_1FFE;
    /// `REG_PALBANK0` / `REG_PALBANK1`: elige el banco de paleta visible.
    pub const PAL_BANK0: u32 = 0x3A_001F;
    pub const PAL_BANK1: u32 = 0x3A_000F;
    /// `REG_NOSHADOW`: cualquier escritura quita el modo sombra.
    pub const NO_SHADOW: u32 = 0x3A_0001;
}

/// Rutinas de la BIOS (llamadas del sistema).
pub mod bios {
    /// Tratamiento del vblank mientras manda la BIOS.
    pub const SYSTEM_INT1: u32 = 0xC0_0438;
    /// Devuelve el control a la BIOS (fin de una petición `USER`).
    pub const SYSTEM_RETURN: u32 = 0xC0_0444;
    /// Lee mandos, monedas y botones de inicio.
    pub const SYSTEM_IO: u32 = 0xC0_044A;
    /// Borra la capa fija.
    pub const FIX_CLEAR: u32 = 0xC0_04C2;
    /// Mensajes de la BIOS en la capa fija.
    pub const MESS_OUT: u32 = 0xC0_04CE;
    /// Comprueba (o descuenta) créditos según `bios_ram::CREDIT_REQUEST`.
    pub const CREDIT_CHECK: u32 = 0xC0_0450;
}

/// Variables de la BIOS en la RAM de trabajo.
pub mod bios_ram {
    /// Bit 7 = el juego está en marcha (atiende él el vblank).
    pub const SYSTEM_MODE: u32 = 0x10_FD80;
    pub const SYSTEM_MODE_GAME: u8 = 1 << 7;
    /// 0 = consola (AES), distinto de 0 = recreativa (MVS).
    pub const MVS_FLAG: u32 = 0x10_FD82;
    /// Qué pide la BIOS en `USER`: 0 = iniciar, 1 = prueba, 2 = demo,
    /// 3 = título.
    pub const USER_REQUEST: u32 = 0x10_FDAE;
    /// Ajustes "soft DIP" que el juego deja por defecto en consola.
    pub const SOFT_DIP_LIVES: u32 = 0x10_FD88;
    pub const SOFT_DIP_DIFFICULTY: u32 = 0x10_FD8B;
    /// En qué está el juego según la BIOS: 0 = arrancando, 1 = título /
    /// demo, 2 = partida.
    pub const USER_MODE: u32 = 0x10_FDAF;
    pub const USER_MODE_ATTRACT: u8 = 1;
    pub const USER_MODE_GAME: u8 = 2;
    /// Petición `USER` de título (ver `USER_REQUEST`).
    pub const USER_REQUEST_TITLE: u8 = 3;
    /// Petición de créditos a la BIOS: 4 bytes (jugadores 1-4); tras
    /// `bios::CREDIT_CHECK` queda distinto de 0 si el jugador tiene crédito.
    pub const CREDIT_REQUEST: [u32; 4] = [0x10_FDB0, 0x10_FDB1, 0x10_FDB2, 0x10_FDB3];
    /// Segundos que quedan en el título de la recreativa (BCD).
    pub const COMPULSION_TIMER: u32 = 0x10_FDDA;
    /// Arranque "en caliente" (desde el título con créditos).
    pub const HARD_START: u32 = 0x10_FEC5;
}

/// Arranque, interrupciones y bucle de frame.
pub mod system {
    /// Rutina a la que salta la interrupción del temporizador.
    pub const TIMER_IRQ_HOOK: u32 = 0x10_6EA8;
    /// Copia de `REG_LSPCMODE` que mantiene el juego.
    pub const LSPC_MODE_SHADOW: u32 = 0x10_6EAC;
    /// 1 = la máquina es de 50 Hz.
    pub const IS_PAL: u32 = 0x10_6EAE;
    /// Copia de `REG_VRAMADDR` que se restaura al acabar cada vblank.
    pub const VRAM_ADDR_SHADOW: u32 = 0x10_6EE4;
    /// Distinto de 0: el trabajo del vblank para este frame ya está hecho
    /// y el bucle de frame puede seguir.
    pub const FRAME_READY: u32 = 0x10_6ED8;
    /// Vblanks desde el último frame del juego.
    pub const VBLANKS_THIS_FRAME: u32 = 0x10_6ED9;
    /// Vblanks que aún hay que saltarse sin hacer nada.
    pub const VBLANK_DELAY: u32 = 0x10_6EDA;
    /// Hay que volcar la paleta en el próximo vblank.
    pub const PAL_FLUSH_REQUEST: u32 = 0x10_6EDB;
    /// Distinto de 0: no volcar la paleta (se está escribiendo).
    pub const PAL_FLUSH_BLOCK: u32 = 0x10_6EDC;
    /// Vblanks seguidos sin que el bucle de frame avance; a partir de 10 se
    /// deja de alimentar al perro guardián.
    pub const STALL_VBLANKS: u32 = 0x10_6EDD;
    /// El juego ya ha completado algún frame.
    pub const FRAME_STARTED: u32 = 0x10_6EDE;
    /// Vueltas de espera del bucle de frame (word).
    pub const IDLE_SPINS: u32 = 0x10_6EE0;
    /// Vblanks sin trabajo (word).
    pub const SKIPPED_VBLANKS: u32 = 0x10_6EE2;
    /// Frames del juego (byte, da la vuelta).
    pub const FRAME_COUNTER: u32 = 0x10_6F28;
    /// Última respuesta del driver de sonido leída en el frame.
    pub const SOUND_REPLY_FRAME: u32 = 0x10_6F29;
    /// Parámetros que fija el bucle de frame al empezar.
    pub const FRAME_PARAM_A: u32 = 0x10_6F2A;
    pub const FRAME_PARAM_B: u32 = 0x10_6F2B;
    /// Flag por frame: el actual (lo ponen las tareas) y el del anterior.
    pub const FRAME_FLAG: u32 = 0x10_E48B;
    pub const FRAME_FLAG_PREV: u32 = 0x10_E48A;
    /// Flag del modo de atracción.
    pub const ATTRACT_FLAG: u32 = 0x10_E486;
    /// Primer word de la RAM (bit 7 del primer byte: leer siempre los mandos).
    pub const RAM_FIRST_WORD: u32 = 0x10_0000;
    /// Zona de la RAM del juego que se borra al volver a la BIOS.
    pub const GAME_RAM: u32 = 0x10_0080;
    pub const GAME_RAM_LEN: u32 = 0xE40C;
    /// Inicio de la BIOS: flag distinto de 0 si viene de un arranque en frío.
    pub const BIOS_START_FLAG: u32 = 0x10_FE80;
    /// Cola de llamadas diferidas al vblank: 8 entradas de
    /// (rutina, contexto) y punteros de lectura / escritura.
    pub const DEFERRED_QUEUE: u32 = 0x10_6EE6;
    pub const DEFERRED_LEN: u8 = 8;
    pub const DEFERRED_READ: u32 = 0x10_6F26;
    pub const DEFERRED_WRITE: u32 = 0x10_6F27;
    /// Pareja de flags que se inicializan juntos (0 en recreativa, 4 en
    /// consola).
    pub const DUAL_FLAG: [u32; 2] = [0x10_81BF, 0x10_81C0];
    /// Ajustes del título.
    pub const TITLE_FLAG_A: u32 = 0x10_6ECC;
    pub const TITLE_FLAG_B: u32 = 0x10_6ECD;
    pub const TITLE_FLAG_C: u32 = 0x10_007B;
    /// Tabla de récords en RAM y valores por defecto en la ROM.
    pub const HISCORES: u32 = 0x10_0002;
    pub const DEFAULT_HISCORES: u32 = 0x2F_53B2;
    pub const HISCORES_LEN: u32 = 120;
}

/// Cola de órdenes al driver de sonido (Z80).
pub mod sound {
    /// Pide el sonido de moneda en el próximo vblank (lo pide la BIOS por
    /// el vector `COIN_SOUND` de la cabecera).
    pub const COIN_SOUND_REQUEST: u32 = 0x10_007A;
    /// Cola circular de 32 órdenes.
    pub const QUEUE: u32 = 0x10_8184;
    pub const QUEUE_LEN: u16 = 32;
    pub const QUEUE_READ: u32 = 0x10_81A4;
    pub const QUEUE_WRITE: u32 = 0x10_81A6;
    /// Última respuesta del Z80 y la última ya procesada.
    pub const REPLY: u32 = 0x10_81A8;
    pub const REPLY_SEEN: u32 = 0x10_81A9;
    pub const REPLY_AUX: u32 = 0x10_81AA;
    /// Última orden de sistema (< `0x20`) encolada; no se repite.
    pub const LAST_SYSTEM_CMD: u32 = 0x10_81AC;
    /// Las órdenes por debajo de este valor son de sistema.
    pub const FIRST_SOUND_CODE: u8 = 0x20;
    /// Orden inicial que se encola al arrancar.
    pub const CMD_INIT: u8 = 4;
}

/// Planificador de tareas.
pub mod task {
    /// Lista circular de tareas fijas, empezando en la primera.
    pub const ARENA: u32 = 0x10_0080;
    /// Última tarea fija.
    pub const LAST_FIXED: u32 = 0x10_0940;
    /// Tabla de la ROM con (bloque, rutina) de cada tarea fija.
    pub const FIXED_TABLE: u32 = 0x27_8000;
    /// Bloques libres para tareas nuevas.
    pub const POOL: u32 = 0x10_0A80;
    pub const POOL_COUNT: u32 = 160;
    pub const SIZE: u32 = 0xA0;
    /// Cabeza de la lista de bloques libres (`0xFFFFFFFF` = vacía).
    pub const FREE_LIST: u32 = 0x10_6E80;
    pub const NIL: u32 = 0xFFFF_FFFF;
    /// Tarea en curso.
    pub const CURRENT: u32 = 0x10_6E84;
    /// Tareas marcadas en el frame anterior / en este, y total ejecutadas.
    pub const MARKED_PREV: u32 = 0x10_6E88;
    pub const MARKED: u32 = 0x10_6E8A;
    pub const RUN_COUNT: u32 = 0x10_6E8C;
    /// Pila guardada antes de llamar a cada tarea.
    pub const SAVED_SP: u32 = 0x10_6E8E;
    /// Campos de una tarea.
    pub const HANDLER: u32 = 0x00;
    pub const PREV: u32 = 0x04;
    pub const NEXT: u32 = 0x08;
    pub const OWNER: u32 = 0x0C;
    pub const PRIORITY: u32 = 0x10;
    pub const FLAGS: u32 = 0x12;
    pub const FLAGS_MARKED: u8 = 1 << 1;
    pub const FLAGS2: u32 = 0x13;
    pub const FLAGS2_FRESH: u8 = 1 << 6;
    pub const TIMER_A: u32 = 0x44;
    pub const TIMER_B: u32 = 0x45;
    pub const TIMER_C: u32 = 0x59;
    pub const FLAGS3: u32 = 0x5A;
    pub const FLAGS4: u32 = 0x69;
    /// Punteros de la tarea que se ponen a "ninguno" (`NIL`), de 4 en 4.
    pub const PTR_SLOTS_FIRST: u32 = 0x10;
    pub const PTR_SLOTS_LAST: u32 = 0x9C;
    /// Bloque al que apuntan las altas cuando el almacén está vacío (las
    /// escrituras de una tarea que no se pudo crear caen ahí).
    pub const OVERFLOW_BLOCK: u32 = 0x10_09E0;
    /// Tarea de arranque y las tareas fijas a las que pone rutina.
    pub const BOOT: u32 = 0x10_0120;
    /// (tarea, rutina) que instala la tarea de arranque, en orden.
    pub const BOOT_INSTALL: [(u32, u32); 12] = [
        (0x10_08A0, super::routine::IDLE),
        (0x10_0800, super::routine::IDLE),
        (0x10_06C0, super::routine::PLAYERS_FRAME_TASK_88),
        (0x10_0440, super::routine::IDLE),
        (0x10_04E0, super::routine::IDLE),
        (0x10_0620, super::routine::PLAYERS_FRAME_TASK_8D),
        (0x10_0580, super::routine::IDLE),
        (0x10_0760, super::routine::SLUG_FRAME_TASK),
        (0x10_0300, super::routine::IDLE),
        (0x10_03A0, super::routine::IDLE),
        (0x10_0260, super::routine::IDLE),
        (0x10_01C0, super::routine::SCHEDULER_BOOTSTRAP),
    ];
    /// Tareas que la de arranque reinicia después (las entidades de los
    /// jugadores y otra).
    pub const BOOT_RESET: [u32; 3] = [0x10_0440, 0x10_04E0, 0x10_0580];
    /// Tarea fija del Slug (SV-001).
    pub const SLUG: u32 = 0x10_0580;
    /// Bytes de las entidades de los jugadores y del Slug que tres tareas
    /// fijas borran en cada frame.
    pub const FRAME_FLAG_8D: u32 = 0x8D;
    pub const FRAME_FLAG_88: u32 = 0x88;
    /// Otro puntero de la tarea que se pone a `NIL` al darla de baja.
    pub const LINK: u32 = 0x48;
    /// Ranuras de paleta de la tarea: hasta 4 índices (words) y cuántas
    /// están ocupadas.
    pub const PAL_SLOTS: u32 = 0x16;
    pub const PAL_SLOT_COUNT: u32 = 0x1E;
    /// Entrada de sprite en uso (índice en la tabla de sprites).
    pub const SPRITE_ENTRY: u32 = 0x14;
    /// Posición (x, y), atributos de sprite, orientación y capa.
    pub const POS_X: u32 = 0x22;
    pub const POS_Y: u32 = 0x24;
    pub const SPRITE_ATTR: u32 = 0x38;
    pub const FACING: u32 = 0x3A;
    pub const LAYER: u32 = 0x11;
    /// Bits de `SPRITE_ATTR` que las escenas limpian.
    pub const SPRITE_ATTR_KEEP: u16 = 0xFFE3;
    /// Bits de `SPRITE_ATTR` que se recalculan con la profundidad (y).
    pub const SPRITE_ATTR_DEPTH_KEEP: u16 = 0xE01F;
    /// Animación por guion: cursor, inicio, contador, espera y bucle.
    pub const ANIM_PC: u32 = 0x3C;
    pub const ANIM_START: u32 = 0x40;
    pub const ANIM_STEPS: u32 = 0x3B;
    pub const ANIM_WAIT: u32 = 0x46;
    pub const ANIM_LOOP: u32 = 0x47;
    /// Bit de `FLAGS4`: la animación acaba de empezar.
    pub const FLAGS4_ANIM_STARTED: u8 = 1 << 5;
    /// Cursor del guion que interpreta la tarea (ver `script`).
    pub const SCRIPT_CURSOR: u32 = 0x70;
    /// Valores iniciales de los campos de una tarea (offset, valor).
    /// Original: `Entity_InitFields_05DC34`.
    pub const RESET_WORDS: [(u32, u16); 8] =
        [(0x14, 0), (0x16, 0xFFFF), (0x18, 0xFFFF), (0x1A, 0xFFFF), (0x1C, 0xFFFF), (0x1E, 0), (0x38, 0), (0x26, 0)];
    pub const RESET_BYTES: [(u32, u8); 13] = [
        (0x12, 0), (0x13, 0), (0x5A, 0), (0x69, 0), (0x5B, 0), (0x6B, 0), (0x32, 0xFF), (0x33, 0xFF),
        (0x3A, 0), (0x59, 0), (0x45, 0), (0x44, 0), (0x6A, 0),
    ];
    pub const RESET_LONGS: [u32; 2] = [0x28, 0x2C];
    /// Otro byte de flags de la tarea que el bucle de frame borra.
    pub const FLAGS5: u32 = 0x5B;
    /// Flags que sobreviven a cada pasada del planificador.
    pub const FLAGS3_KEEP_BEFORE: u8 = 0x01;
    pub const FLAGS4_KEEP_BEFORE: u8 = 0x17;
    pub const FLAGS3_KEEP_AFTER: u8 = 0xFA;
    pub const FLAGS4_KEEP_AFTER: u8 = 0x68;
    /// Bit 0 de `FLAGS`: tarea fija (no se libera).
    pub const FLAGS_FIXED: u8 = 1 << 0;
}

/// Paleta: copia en RAM que se vuelca en el vblank.
pub mod palette {
    /// 256 paletas de 32 bytes; el primer word de cada una es la marca de
    /// "modificada" (el color 0 es siempre transparente).
    pub const SHADOW: u32 = 0x10_A2D4;
    pub const COUNT: u32 = 256;
    pub const SIZE: u32 = 32;
    /// Paletas que se revisan en el bucle normal (la 255 va aparte).
    pub const SCANNED: u32 = 0xC4;
    pub const BACKDROP: u32 = 255;
    /// Distinto de 0: hay paletas modificadas.
    pub const ANY_DIRTY: u32 = 0x10_A2CE;
    /// Cambios de banco pendientes.
    pub const BANK_SWAP_PENDING: u32 = 0x10_A2D0;
    pub const BANK_SWAP_ACTIVE: u32 = 0x10_A2D1;
    pub const FADE_STEP: u32 = 0x10_A2D2;
    /// Otras variables del módulo de paleta que se borran al iniciar.
    pub const STATE_BYTES: [u32; 6] = [0x10_A2C8, 0x10_A2CA, 0x10_A2CB, 0x10_A2CC, 0x10_A2CF, 0x10_A2D0];
    /// Fundido de toda la pantalla: modo (0 = no, `FADE_TO_WHITE`, otro =
    /// a negro), modo del frame anterior y nivel de cada componente (R, G, B).
    pub const FADE_MODE: u32 = 0x10_A2C8;
    pub const FADE_MODE_PREV: u32 = 0x10_A2C9;
    pub const FADE_TO_WHITE: u8 = 0xFF;
    /// Valor de `FADE_MODE_PREV` que no obliga a restaurar los colores.
    pub const FADE_PREV_IGNORED: u8 = 0x80;
    pub const FADE_LEVEL: [u32; 3] = [0x10_A2CA, 0x10_A2CB, 0x10_A2CC];
    /// Durante la actualización: ya se ha hecho la pasada de la paleta de
    /// fondo.
    pub const BACKDROP_PASS: u32 = 0x10_A2CD;
    /// `FADE_STEP` distinto de este valor: se está borrando a blanco el
    /// otro banco de la RAM de paleta (16 paletas por vblank).
    pub const FADE_STEP_DONE: u8 = 0xFF;
    pub const BANK_CLEAR_STEPS: u8 = 16;
    pub const BANK_CLEAR_PALETTES: u32 = 16;
    pub const WHITE: u16 = 0x7FFF;
    /// Colores por paleta y bytes por color en la ROM (R, G, B, relleno).
    pub const COLORS: u32 = 16;
    pub const SOURCE_COLOR_SIZE: u32 = 4;
    /// Tabla de la ROM: color RGB555 → color Neo Geo.
    pub const RGB555_TO_NEOGEO: u32 = 0x00_2F30;
    /// Tabla de la ROM con 32 punteros a tablas de 32 pasos: cuánto se
    /// avanza una componente que dista `n` en el paso `k`.
    pub const STEP_TABLES: u32 = 0x01_2F30;
    pub const COMPONENT_MAX: u8 = 0x1F;
    /// Campos de una entrada de paleta (las mismas entradas que
    /// `sprites::TABLE`): flags, colores (ROM), periodo del fundido
    /// (`NO_FADE` = ninguno), colores destino, paso, incremento del paso y
    /// temporizador.
    pub const ENTRY_COLORS: u32 = 0x02;
    pub const ENTRY_PERIOD: u32 = 0x08;
    pub const ENTRY_TARGET: u32 = 0x0A;
    pub const ENTRY_STEP: u32 = 0x0E;
    pub const ENTRY_STEP_INC: u32 = 0x0F;
    pub const ENTRY_TIMER: u32 = 0x10;
    pub const NO_FADE: u16 = 0xFFFF;
    pub const FADE_STEPS: i8 = 0x20;
    /// Bits de flags de una entrada.
    pub const E_LOAD: u8 = 1 << 0;
    pub const E_FADING: u8 = 1 << 1;
    pub const E_NO_SCREEN_FADE: u8 = 1 << 3;
    pub const E_CLEAR: u8 = 1 << 6;
    pub const E_SKIP: u8 = 1 << 7;
    /// Flags que quedan en una entrada borrada.
    pub const E_CLEARED: u8 = 0x80;
}

/// Tabla de sprites del juego y colas de dibujo.
pub mod sprites {
    /// 256 entradas de 32 bytes; el primer word de una entrada libre es
    /// `FREE_MARK`.
    pub const TABLE: u32 = 0x10_82C8;
    pub const COUNT: u32 = 256;
    pub const SIZE: u32 = 32;
    pub const FREE_MARK: u16 = 0x8800;
    /// Pila de índices libres (0..`FREE_COUNT`) y su tamaño.
    pub const FREE_STACK: u32 = 0x10_81C2;
    pub const FREE_COUNT: u16 = 0x30;
    pub const FREE_TOP: u32 = 0x10_82C2;
    pub const STATE_WORDS: [u32; 2] = [0x10_82C4, 0x10_82C6];
    /// Posición de lectura del anillo de libres (al reservar).
    pub const FREE_READ: u32 = 0x10_82C4;
    /// Entradas de sprite de tarea: se busca una con los mismos gráficos
    /// entre las `SEARCH_COUNT` entradas desde `FREE_INDEX_BASE`.
    pub const SEARCH_COUNT: u16 = 47;
    /// Una entrada con alguno de estos bits no se comparte.
    pub const NOT_SHARED: u8 = 0x84;
    /// Primer byte de una entrada recién reservada.
    pub const TAKEN_MARK: u8 = 0x01;
    /// Bit del primer byte de la tarea que impide reutilizar sus entradas.
    pub const TASK_NO_REUSE: u8 = 0x04;
    /// Una tarea tiene como mucho estas entradas.
    pub const TASK_MAX_ENTRIES: u16 = 3;
    /// La pila de índices libres es en realidad un anillo de `FREE_COUNT`:
    /// posición de escritura (al liberar).
    pub const FREE_WRITE: u32 = 0x10_82C6;
    /// Las entradas liberadas se devuelven al anillo restando este índice.
    pub const FREE_INDEX_BASE: u16 = 0x94;
    /// Primer byte de una entrada recién liberada.
    pub const RELEASED_MARK: u8 = 0x88;
    /// Campos de una entrada: gráficos y referencias.
    pub const ENTRY_DATA: u32 = 0x02;
    pub const ENTRY_REFS: u32 = 0x06;
    pub const ENTRY_AUX: u32 = 0x08;
    /// Primer byte de una entrada cargada vacía.
    pub const EMPTY_MARK: u8 = 0x40;
    /// Listas de la ROM con las paletas fijas que se cargan al arrancar
    /// cada escena: (marca << 8 | índice, número de paleta) acabadas en
    /// `0xFFFF`. La baja empieza en la entrada `LOW_LIST_FIRST` y sus
    /// datos en `LOW_LIST_DATA`; la alta no desplaza el índice.
    pub const LOW_LIST: u32 = 0x00_09FA;
    pub const LOW_LIST_FIRST: u32 = 0x74;
    pub const LOW_LIST_MAX: u32 = 0x20;
    pub const LOW_LIST_DATA: u32 = 0x01_4E00;
    pub const HIGH_LIST: u32 = 0x00_09B4;
    pub const HIGH_LIST_MAX: u32 = 0x74;
    pub const HIGH_LIST_DATA: u32 = 0x01_CE00;
    pub const LIST_END: u16 = 0xFFFF;
    /// Reparto de los sprites hardware: inicio, final y corte de la zona
    /// del juego, y dos variables que se ponen a 0.
    pub const BAND_FIRST: u32 = 0x10_E1F6;
    pub const BAND_END: u32 = 0x10_E1F8;
    pub const BAND_SPLIT: [u32; 2] = [0x10_E1FA, 0x10_E1FC];
    pub const BAND_CLEAR: [u32; 2] = [0x10_E1F4, 0x10_E1FE];
    pub const BAND_TOTAL: u16 = 0x17C;
    /// SCB3 (altura/posición vertical) del sprite 1: escribir 0 lo oculta.
    pub const VRAM_SCB3_FIRST: u16 = 0x8201;
    pub const STATE_BYTES: [u32; 2] = [0x10_A2C9, 0x10_A2D1];
    /// Base de las variables del renderizador (el original las direcciona
    /// relativas a ella).
    pub const RENDER_BASE: u32 = 0x10_8080;
    /// Hay colas de sprites listas para volcar en el vblank.
    pub const FLUSH_PENDING: u32 = 0x10_E1EC;
    /// Las dos colas de dibujo (descriptor de 12 bytes cada una).
    pub const QUEUES: [u32; 2] = [0x10_E1CC, 0x10_E1D8];
    pub const QUEUE_ACTIVE: u32 = 0x08;
    pub const QUEUE_LIMIT: u32 = 0x04;
    pub const QUEUE_RESULT: u32 = 0x0A;
    /// Más campos del descriptor de cola: primer y último sprite hardware
    /// de la zona y hasta dónde se llegó al escribir SCB1.
    pub const QUEUE_FIRST: u32 = 0x00;
    pub const QUEUE_LAST: u32 = 0x02;
    pub const QUEUE_REACHED: u32 = 0x06;
    /// Claves de ordenación de la lista de dibujo (longs: clave alta, en la
    /// palabra baja el registro); `DRAW_LIST` apunta a esas palabras bajas.
    pub const SORT_KEYS: u32 = 0x10_D4A8;
    pub const SORT_KEY_SIZE: u32 = 4;
    /// La segunda cola sólo se ordena si empieza antes de esto.
    pub const SECOND_SORT_LIMIT: u16 = 0x344;
    /// Registros de columna de sprite (12 bytes): puntero a los tiles,
    /// atributos, SCB2 (bit 15 = último registro del objeto, bit 12 =
    /// tiles al revés, byte bajo `FULL_HEIGHT` = sin relleno), SCB3 y SCB4
    /// (byte bajo = número de tiles).
    pub const RECORDS: u32 = 0x10_C2D8;
    pub const RECORD_SIZE: u32 = 12;
    pub const R_TILES: u32 = 0x00;
    pub const R_ATTR: u32 = 0x04;
    pub const R_SCB2: u32 = 0x06;
    pub const R_SCB3: u32 = 0x08;
    pub const R_SCB4: u32 = 0x0A;
    pub const R_LAST: u8 = 0x80;
    pub const R_REVERSED: u16 = 1 << 12;
    pub const FULL_HEIGHT: u8 = 0xFF;
    pub const MAX_TILES: u8 = 0x1F;
    /// VRAM: palabras de SCB1 por sprite y bases de SCB2 y SCB4.
    pub const SCB1_WORDS: u16 = 0x40;
    pub const SCB2: u16 = 0x8000;
    pub const SCB3: u16 = 0x8200;
    pub const SCB3_FIRST: u16 = 0x8201;
    pub const SCB4: u16 = 0x8400;
    /// Resultado de volcar cada cola.
    pub const QUEUE_DONE: [u32; 2] = [0x10_E1E8, 0x10_E1EA];
    /// Lista de entradas que se vuelcan y longitud de cada cola en ella.
    pub const DRAW_LIST: u32 = 0x10_D4AA;
    pub const DRAW_LIST_SECOND: u32 = 0x348;
    pub const QUEUE_FILL: [u32; 2] = [0x10_E1C8, 0x10_E1CA];
    pub const DRAW_COUNT: u32 = 0x10_C2D4;
    /// Variables de la capa fija.
    pub const FIX_STATE: [u32; 3] = [0x10_E1FA, 0x10_E1F6, 0x10_E1F8];
    /// Zona de VRAM de la capa fija y de los sprites que se borran al
    /// iniciar.
    pub const VRAM_FIX_CLEAR: u16 = 0x8200;
    pub const VRAM_FIX_CLEAR_WORDS: u32 = 0x17D;
    pub const VRAM_SCB1_CLEAR_WORDS: u32 = 0x5F40;
}

/// Pausa.
pub mod pause {
    pub const ACTIVE: u32 = 0x10_E272;
    /// Contador de parpadeo del texto de pausa.
    pub const BLINK: u32 = 0x10_E273;
    pub const ENABLED: u32 = 0x10_E274;
    /// Bit de `pads::START_PRESSED` del botón de inicio de cada jugador.
    pub const START_BIT: [u8; 2] = [0x02, 0x08];
    /// Sonidos de pausa y de fin de pausa.
    pub const SOUND_PAUSE: u16 = 0x10E0;
    pub const SOUND_UNPAUSE: u16 = 0x10E1;
    /// Rutinas originales (aún sin portar): texto "PAUSE" visible / borrado,
    /// petición de sonido y parada de la música.
    pub const SHOW_TEXT: u32 = 0x01_3D46;
    pub const HIDE_TEXT: u32 = 0x01_3D3E;
    pub const SOUND_REQUEST: u32 = 0x00_2352;
    pub const SOUND_STOP: u32 = 0x00_2222;
    /// El texto parpadea con periodo de 32 frames; se borra en el 24.
    pub const BLINK_PERIOD_MASK: u8 = 0x1F;
    pub const BLINK_HIDE_AT: u8 = 0x18;
}

/// Estado de los mandos por frame.
pub mod pads {
    /// Copias de trabajo de los dos mandos (6 bytes cada una).
    pub const WORK: [u32; 2] = [0x10_6EB0, 0x10_6EB6];
    pub const REPEAT: u32 = 0x10_6EC8;
    pub const REPEAT_DEFAULT: u16 = 0x1002;
    pub const SNAPSHOT_FLAGS: u32 = 0x10_E20C;
    /// Demo grabada: modo (1 = reproduciendo), puntero y contadores.
    pub const DEMO_MODE: u32 = 0x10_6ECA;
    pub const DEMO_PTR: u32 = 0x10_6EBE;
    pub const DEMO_WORDS: [u32; 3] = [0x10_6EC2, 0x10_6EC4, 0x10_6EBC];
    pub const DEMO_BYTE: u32 = 0x10_6EC6;
    /// Tabla de la ROM con las demos y flag de la ROM que pide borrarlas.
    pub const DEMO_TABLE: u32 = 0x0E_7C00;
    pub const DEMO_CLEAR_FLAG: u32 = 0x02_5118;
    pub const DEMO_BUFFER_LEN: u32 = 0x200;
    /// Número de demo que se pasa para no reproducir ninguna.
    pub const NO_DEMO: u16 = 0xFFFF;
    pub const DEMO_COUNT: u16 = 2;
    /// Rutinas de tarea con las que un jugador no cuenta como activo.
    pub const INACTIVE_HANDLERS: [u32; 4] = [0xFFFF_FFFF, 0x52A, 0x400, 0x2_AE3E];
    /// Bit 7 de este byte de la tarea del jugador: está en juego.
    pub const PLAYER_TASK_ALIVE: u32 = 0x5B;
    /// Mandos tal como los deja la BIOS (botones mantenidos de P1 y P2) y
    /// botones de inicio/selección.
    pub const BIOS_P1: u32 = 0x10_FD96;
    pub const BIOS_P2: u32 = 0x10_FD9C;
    pub const BIOS_START: u32 = 0x10_FDAC;
    /// Estado de cada jugador según la BIOS (0 = no juega, 3 = sin
    /// créditos / acabado, 1 = juega).
    pub const BIOS_PLAYER_STATE: [u32; 2] = [0x10_FDB6, 0x10_FDB7];
    /// Los cuatro bytes de estado de jugador de la BIOS (se borran juntos).
    pub const BIOS_PLAYER_STATE_ALL: [u32; 4] = [0x10_FDB6, 0x10_FDB7, 0x10_FDB8, 0x10_FDB9];
    pub const BIOS_PLAYER_PLAYING: u8 = 1;
    /// `PLAYER_START`: bits de los jugadores que pulsaron inicio (la BIOS
    /// los pone; el juego deja puestos los que acepta).
    pub const BIOS_START_FLAG: u32 = 0x10_FDB4;
    /// Por cada combinación de `BIOS_START_FLAG` (bit 0 = jugador 1, bit 1
    /// = jugador 2): nuevo `USER_MODE`, (sin uso) y nuevo estado de cada
    /// jugador (0 = no cambia).
    pub const START_TABLE: [[u8; 4]; 4] = [[0, 0, 0, 0], [2, 1, 1, 0], [2, 2, 0, 1], [2, 3, 1, 1]];
    pub const BIOS_PLAYER_OFF: u8 = 0;
    pub const BIOS_PLAYER_GAME_OVER: u8 = 3;
    /// Campos de un mando de trabajo (6 bytes).
    pub const PAD_PREV: u32 = 1;
    pub const PAD_HELD: u32 = 2;
    pub const PAD_PRESSED: u32 = 3;
    pub const PAD_REPEAT: u32 = 4;
    pub const PAD_REPEAT_TIMER: u32 = 5;
    /// Retardo de la primera repetición y de las siguientes (los dos bytes
    /// de `REPEAT`).
    pub const REPEAT_FIRST: u32 = 0x10_6EC8;
    pub const REPEAT_NEXT: u32 = 0x10_6EC9;
    /// Botones de inicio: mantenidos y recién pulsados.
    pub const START_HELD: u32 = 0x10_E20C;
    pub const START_PRESSED: u32 = 0x10_E20D;
    /// Mandos "en juego" de cada jugador (copia de `PAD_LIVE` si juega).
    pub const PAD_INGAME: [u32; 2] = [0x10_E212, 0x10_E218];
    /// Lectura de los mandos con demo (grabar / reproducir), sin portar.
    pub const DEMO_READ: u32 = 0x02_511A;
    /// Jugadores activos en este frame.
    pub const ACTIVE_PLAYERS: u32 = 0x10_6ED1;
}

/// Rutinas originales (entradas a las que se llama o a las que se salta).
pub mod routine {
    /// `rts` suelto: gancho por defecto de la interrupción del temporizador.
    pub const NOP: u32 = 0x00_08F2;
    /// Rutina de tarea que no hace nada (`rts`).
    pub const IDLE: u32 = 0x00_0400;
    /// `rts` que deja la tarea de arranque al acabar.
    pub const BOOT_DONE: u32 = 0x00_0B90;
    /// Rutina de la tarea que arranca la partida / el título.
    pub const SCHEDULER_BOOTSTRAP: u32 = 0x00_0E8E;
    pub const SCHEDULER: u32 = 0x00_0656;
    /// Punto del planificador al que vuelven las tareas.
    pub const SCHEDULER_AFTER_TASK: u32 = 0x00_06CA;
    /// Rutina de una tarea que se está dando de baja.
    pub const TASK_FINISHING: u32 = 0x00_052A;
    /// Borrado a blanco de 16 paletas del otro banco (diferido al vblank).
    pub const PAL_BANK_CLEAR_STEP: u32 = 0x01_387E;
    /// Tareas fijas que borran cada frame un byte de las entidades
    /// (`task::FRAME_FLAG_*`): del Slug y de los dos jugadores.
    pub const SLUG_FRAME_TASK: u32 = 0x02_9582;
    pub const PLAYERS_FRAME_TASK_8D: u32 = 0x03_2406;
    pub const PLAYERS_FRAME_TASK_88: u32 = 0x03_240C;
    /// Bucle de frame: preparación (una vez) y espera de cada frame (la
    /// rutina que queda en la tarea del frame).
    pub const FRAME_LOOP: u32 = 0x00_1F84;
    pub const FRAME_WAIT: u32 = 0x00_1FE2;
    /// Inicialización de los subsistemas y arranque del planificador
    /// (peticiones `USER` de demo y de título).
    pub const INIT_SUBSYSTEMS: u32 = 0x00_097A;
    /// Peticiones `USER` 0 (iniciar) y 1 (modo de prueba).
    pub const USER_INIT: u32 = 0x00_080C;
    pub const USER_TEST: u32 = 0x00_0832;
    /// Vuelta a la BIOS (borra la RAM del juego).
    pub const RETURN_TO_BIOS: u32 = 0x00_085E;
    /// Lectura de los mandos del frame.
    pub const INPUT_READ_FRAME: u32 = 0x05_CC1A;
    /// Botón de pausa; C = el juego está en pausa.
    pub const PAUSE_POLL: u32 = 0x01_3C3C;
    /// Entidad de un jugador si está vivo (d0 = jugador; C = sí).
    pub const PLAYER_ENTITY_IF_ALIVE: u32 = 0x05_E3A2;
    /// Tabla de la ROM con las tareas de control de cada jugador.
    pub const PLAYER_TASKS: u32 = 0x05_E3F4;
    /// Entradas de la cabecera Neo Geo a las que salta la BIOS.
    pub const HEADER_USER: u32 = 0x00_0122;
    pub const HEADER_COIN_SOUND: u32 = 0x00_0134;
    /// Petición `USER` de la BIOS y tabla de saltos por tipo de petición.
    pub const USER_REQUEST: u32 = 0x00_07CC;
    pub const USER_REQUEST_TABLE: u32 = 0x00_07FC;
    /// Vblank (interrupción de nivel 1 en MVS, vector 2 en la tabla).
    pub const VBLANK_IRQ: u32 = 0x00_08F6;
    /// Interrupción del temporizador.
    pub const TIMER_IRQ: u32 = 0x00_08D6;
    /// Guion de arranque: entradas del arranque (completo, partida,
    /// título), el reparto común, el despacho y el avance.
    pub const SCRIPT_GAME: u32 = 0x00_0EF0;
    pub const SCRIPT_TITLE: u32 = 0x00_0F1A;
    pub const SCRIPT_DISPATCH: u32 = 0x00_0FC6;
    pub const SCRIPT_NEXT: u32 = 0x00_0FE0;
}

/// Generador de números aleatorios del juego: una tabla de 32 valores que
/// se rellena a partir de una semilla.
pub mod rng {
    pub const TABLE: u32 = 0x10_E230;
    pub const LEN: u32 = 32;
    pub const INDEX: u32 = 0x10_E270;
    pub const MUL: u16 = 0xB7C7;
    pub const ADD: u16 = 0x81F5;
    /// Contador que avanza en cada escena del guion y sirve de semilla.
    pub const SEED_COUNTER: u32 = 0x10_007C;
}

/// Límites de la cámara.
pub mod camera {
    pub const LIMIT_X: u32 = 0x10_E1E4;
    pub const LIMIT_Y: u32 = 0x10_E1E6;
    pub const DEFAULT_LIMITS: (u16, u16) = (0xA0, 0x178);
    /// Los que deja el guion de arranque.
    pub const BOOT_LIMITS: (u16, u16) = (0xF0, 0x1E8);
}

/// Guion de arranque: la tarea `TASK` recorre una lista de
/// rutinas de la ROM (una por escena: logo, demo, título, partida...).
/// Cada una hace lo suyo durante varios frames y, al acabar, salta a
/// `routine::SCRIPT_NEXT`, que avanza el cursor. `END` seguido de una
/// dirección continúa en otra lista.
pub mod script {
    pub const END: u32 = 0xFFFF_FFFF;
    /// Listas por modo de arranque.
    pub const DEMO: u32 = 0x00_0B92;
    pub const HARD_START: u32 = 0x00_0BBA;
    pub const TITLE: u32 = 0x00_0BCE;
    pub const GAME_AES: u32 = 0x00_0BDA;
    pub const GAME_MVS: u32 = 0x00_0BE6;
    pub const START_PRESSED: u32 = 0x00_0E6E;
    /// Tarea que ejecuta el guion.
    pub const TASK: u32 = 0x10_01C0;
    /// Bytes de la tarea que se borran al arrancar el guion.
    pub const TASK_CLEAR: [u32; 4] = [0x12, 0x13, 0x5B, 0x5A];
    /// Teclas de crédito: las que se ponen a 0 en la demo y las que se
    /// ponen a `0xFF` en todas las escenas.
    pub const KEY_RESET: [u32; 2] = [0x10_6ECC, 0x10_6ECD];
    pub const KEY_LATCH: [u32; 2] = [0x10_6ECE, 0x10_6ECF];
    /// Activa el texto de pausa / lo desactiva (escenas sin pausa).
    pub const PAUSE_ENABLED: u32 = super::pause::ENABLED;
    /// Tareas que se paran entre escenas (con sus hijas); las dos últimas
    /// sólo pierden las hijas en la variante "conservar".
    /// 0xFF mientras la escena actual no ha terminado de dibujarse (la
    /// escena espera a que lo ponga a 0 su tarea).
    pub const SCENE_BUSY: u32 = 0x10_6ED2;
    /// Distinto de 0: la escena pide volver al título.
    pub const BACK_TO_TITLE: u32 = 0x10_6ED5;
    /// Tareas que crea cada escena (rutina inicial).
    pub const LOGO_TASK: u32 = 0x09_1330;
    pub const CONTINUE_TASK: u32 = 0x09_13AC;
    pub const CONTINUE_OPTIONS_TASK: u32 = 0x09_9B06;
    pub const HISCORE_TASK: u32 = 0x09_77D6;
    pub const GAME_OVER_TASK: u32 = 0x08_F91A;
    pub const HISCORE_TABLE_TASK: u32 = 0x09_77EA;
    /// Botones A-D en el byte de recién pulsados de un mando.
    pub const PAD_PRESSED: u32 = 3;
    pub const BUTTONS: u8 = 0xF0;
    /// Música de la pantalla de continuar.
    pub const CONTINUE_MUSIC: u16 = 0x10E0;
    /// Inicio de misión: tecla 1 bloqueada con este valor, ajuste de la
    /// BIOS que activa el manual de juego (`PLAY MANUAL` de los soft DIP),
    /// tarea fija donde se crea el manual con su rutina y contador que lo
    /// acompaña.
    pub const MISSION_KEY_LATCH: u8 = 9;
    pub const PLAY_MANUAL: u32 = 0x10_FD8E;
    pub const MANUAL_TASK: u32 = 0x10_0300;
    pub const MANUAL_HANDLER: u32 = 0x08_C008;
    pub const RANK_TIMER: u32 = 0x10_6E92;
    pub const SCENE_TASKS: [u32; 8] =
        [0x10_0260, 0x10_0800, 0x10_08A0, 0x10_0440, 0x10_04E0, 0x10_0580, 0x10_0300, 0x10_03A0];
}

/// Textos de la capa fija que el juego mantiene encima de todo: créditos
/// (recreativa y consola) y nivel de dificultad (recreativa).
pub mod hud {
    /// Qué hay que dibujar: bit 0 = créditos, bit 1 = borrar créditos,
    /// bit 2 = nivel, bit 3 = borrar nivel.
    pub const FLAGS: u32 = 0x10_81BE;
    pub const DRAW_CREDITS: u8 = 1 << 0;
    pub const CLEAR_CREDITS: u8 = 1 << 1;
    pub const DRAW_LEVEL: u8 = 1 << 2;
    pub const CLEAR_LEVEL: u8 = 1 << 3;
    /// Lo que se pide cada frame desde las escenas (créditos y nivel).
    pub const EVERY_FRAME: u8 = DRAW_CREDITS | DRAW_LEVEL;
    /// Ajuste de la BIOS de recreativa: bit 1 = no mostrar créditos, bit 0
    /// = no mostrar nivel.
    pub const BIOS_HIDE: u32 = 0x10_FD91;
    pub const HIDE_CREDITS: u8 = 1 << 1;
    pub const HIDE_LEVEL: u8 = 1 << 0;
    /// Número de ranuras de la recreativa (1 = un contador por jugador).
    pub const BIOS_SLOTS: u32 = 0x10_FD83;
    /// Créditos (BCD, uno por jugador): en recreativa la copia de la BIOS
    /// si es válida, si no la de la RAM de respaldo; en consola los del
    /// juego.
    pub const CREDITS_MVS: u32 = 0x10_FE00;
    pub const CREDITS_MVS_VALID: u32 = 0x10_FE80;
    pub const CREDITS_BACKUP: u32 = 0xD0_0034;
    pub const CREDITS_AES: u32 = 0x10_81BF;
    /// Atributos de los caracteres (paleta y banco) y carácter en blanco.
    pub const ATTR: u16 = 0x2300;
    pub const BLANK: u8 = b' ';
    /// "CREDITS " (la S se quita con menos de 2 créditos).
    pub const CREDITS_TEXT: &[u8; 8] = b"CREDITS ";
    /// Posiciones en la capa fija (dirección de VRAM; +0x20 = columna
    /// siguiente, +1 = fila siguiente).
    pub const CREDITS_P1: u16 = 0x707D;
    pub const CREDITS_P2: u16 = 0x737D;
    pub const COLUMN: u16 = 0x20;
    /// Tramos que se borran: (inicio, caracteres).
    pub const CREDITS_CLEAR: [(u16, u16); 2] = [(0x701D, 16), (0x72FD, 17)];
    pub const CREDITS_AREA: [(u16, u16); 4] = [(0x701D, 3), (0x719D, 4), (0x72FD, 4), (0x749D, 4)];
    /// Nivel: "LEVEL-" en caracteres propios seguido de la cifra.
    pub const LEVEL_POS: u16 = 0x721D;
    pub const LEVEL_TILES: [u8; 6] = [0x20, 0x8B, 0x8C, 0x8D, 0x8E, 0x8F];
    pub const LEVEL_CLEAR: u16 = 8;
    /// Zona visible de la capa fija: sólo se escribe dentro.
    pub const FIX_FIRST: u16 = 0x7000;
    pub const FIX_LAST: u16 = 0x74FF;
    /// Marco que se borra al empezar cada escena: (inicio, columnas, filas).
    pub const BORDER: [(u16, u16, u16); 4] = [(0x7000, 40, 2), (0x701E, 40, 2), (0x7000, 1, 32), (0x74E0, 1, 32)];
}

/// Guiones de animación de sprites: cada tarea con sprite tiene un cursor
/// en un guion de la ROM (órdenes de 1 byte + argumentos) que dibuja un
/// fotograma, espera, salta, cambia campos de la tarea...
pub mod anim {
    /// Campos de la tarea.
    pub const CURSOR: u32 = 0x3C;
    /// Última orden de fotograma (para repetirla).
    pub const LAST_SPRITE: u32 = 0x40;
    /// Fotogramas desde que empezó el guion (satura en 0xFF).
    pub const STEPS: u32 = 0x3B;
    /// Frames que faltan de la espera actual.
    pub const WAIT: u32 = 0x46;
    /// Vueltas que faltan del bucle actual.
    pub const LOOP: u32 = 0x47;
    /// Campos del sprite de la tarea.
    pub const X: u32 = 0x22;
    pub const Y: u32 = 0x24;
    pub const ATTR: u32 = 0x38;
    pub const FLIP: u32 = 0x3A;
    pub const SHRINK_X: u32 = 0x33;
    pub const SHRINK_Y: u32 = 0x32;
    pub const SPRITE_ENTRY: u32 = 0x14;
    pub const SPRITE_ENTRY_ALT: u32 = 0x1C;
    pub const HITBOX: u32 = 0x48;
    pub const FIELD_4C: u32 = 0x4C;
    pub const FIELD_60: u32 = 0x60;
    pub const STATE_6B: u32 = 0x6B;
    pub const STATE_8D: u32 = 0x8D;
    /// Flags de `FLAGS4`: el guion está en marcha.
    pub const FLAGS4_RUNNING: u8 = 1 << 5;
    /// Bits de `task::FLAGS`.
    pub const FLAGS_HIDDEN: u8 = 1 << 2;
    pub const FLAGS_SKIP_HIDDEN: u8 = 1 << 3;
    pub const FLAGS_PARITY: u8 = 1 << 4;
    pub const FLAGS_PARITY_CHECK: u8 = 1 << 5;
    pub const FLAGS_BACK_LAYER: u8 = 1 << 6;
    /// Órdenes.
    pub const OP_WAIT: u8 = 0x00;
    pub const OP_JUMP: u8 = 0x01;
    pub const OP_SPRITE: u8 = 0x02;
    pub const OP_LOOP: u8 = 0x03;
    pub const OP_SOUND: u8 = 0x04;
    pub const OP_SET_BYTE: u8 = 0x05;
    pub const OP_SET_WORD: u8 = 0x06;
    pub const OP_SET_LONG: u8 = 0x07;
    pub const OP_CALL: u8 = 0x08;
    pub const OP_SET_HITBOX: u8 = 0x09;
    pub const OP_SET_4C: u8 = 0x0A;
    pub const OP_SET_60: u8 = 0x0B;
    pub const OP_COPY_ENTRY: u8 = 0x0C;
    pub const OP_AND_BYTE: u8 = 0x0D;
    pub const OP_AND_WORD: u8 = 0x0E;
    pub const OP_AND_LONG: u8 = 0x0F;
    pub const OP_OR_BYTE: u8 = 0x10;
    pub const OP_OR_WORD: u8 = 0x11;
    pub const OP_OR_LONG: u8 = 0x12;
    pub const OP_EOR_BYTE: u8 = 0x13;
    pub const OP_EOR_WORD: u8 = 0x14;
    pub const OP_EOR_LONG: u8 = 0x15;
    pub const OP_REPEAT: u8 = 0x16;
    pub const OP_SPRITE_PARITY: u8 = 0x17;
    pub const OP_SPRITE_PARITY_CONT: u8 = 0x18;
    pub const OP_SET_70: u8 = 0x19;
    pub const OP_WAIT_TABLE: u8 = 0x1A;
    pub const OP_INDEX_18: u8 = 0x1B;
    pub const OP_INDEX_1C: u8 = 0x1C;
    pub const OP_BACK: u8 = 0x1D;
    /// Fotograma en la posición de la tarea, sin desplazamiento ni recorte.
    pub const OP_SPRITE_AT_TASK: u8 = 0x1E;
    pub const OP_SOUND_STOP: u8 = 0x1F;
    pub const OP_COUNT: u8 = 0x20;
    /// Campos de una orden de fotograma: flags, gráfico, atributos y
    /// desplazamiento.
    pub const SPR_FLAGS: u32 = 1;
    pub const SPR_GFX: u32 = 2;
    pub const SPR_ATTR: u32 = 6;
    pub const SPR_DX: u32 = 8;
    pub const SPR_DY: u32 = 10;
    pub const SPR_LEN_ABS: u32 = 8;
    pub const SPR_LEN_REL: u32 = 12;
    /// Bits de `SPR_FLAGS`.
    pub const SPR_FLIP: u8 = 0x03;
    pub const SPR_SKIP_IF_HIDDEN: u8 = 1 << 2;
    pub const SPR_ABSOLUTE: u8 = 1 << 3;
    pub const SPR_PARITY: u8 = 1 << 4;
    /// Atributos "sin cambio".
    pub const KEEP: u16 = 0xFFFF;
    /// Fuera de esta caja (coordenadas de pantalla) no se dibuja.
    pub const CLIP_X: (i16, i16) = (-0x100, 0x200);
    pub const CLIP_Y: (i16, i16) = (0, 0x300);
    /// Tabla de la ROM con el retroceso de `OP_BACK` según el argumento.
    pub const BACK_TABLE: u32 = 0x02_93E4;
    /// Variable de depuración (como `input::INPUT_GATE`, el byte bajo).
    pub const DEBUG: u32 = 0x10_0001;
    /// Bit de la variable de depuración (`INPUT_GATE`) que dibuja todo.
    pub const DEBUG_DRAW_ALL: u8 = 1 << 1;
    pub const DEBUG_HITBOX: u8 = 1 << 4;
    /// Rutinas originales que usa el guion.
    pub const DEBUG_ENTRY: u32 = 0x02_F84A;
    pub const WORLD_TO_SCREEN: u32 = 0x04_40D0;
    pub const ALLOC_SPRITE: u32 = 0x00_236E;
    /// Campos de la tarea que pueden cambiar las órdenes (`< TASK_FIELDS`).
    pub const TASK_FIELDS: u16 = 0xA0;
    /// Byte de `FLAGS2` que impide usar la entrada alternativa.
    pub const FLAGS2_NO_ALT: u8 = 1 << 0;
    /// Bits de `task::FLAGS3`: usar la entrada alternativa.
    pub const FLAGS3_ALT: u8 = 1 << 7;
    pub const FLAGS3_ALT_REQUEST: u8 = 1 << 0;
    /// Bit de `task::TIMER_C` que limita cuándo se dibuja.
    pub const TIMER_C_GATED: u8 = 1 << 0;
    pub const STATE_6B_DRAW: u8 = 1 << 6;
    pub const STATE_6B_PLAYER: u8 = 1 << 0;
    /// Profundidad: los atributos en este rango llevan la profundidad
    /// calculada a partir de la Y.
    pub const DEPTH_ATTR_FIRST: u16 = 0x4000;
    pub const DEPTH_ATTR_SPAN: u16 = 0x7FFF;
    pub const DEPTH_KEEP: u16 = 0xE01F;
    /// Ranuras de sprite por tarea, y entradas de la zona del juego en
    /// las que se busca una que ya tenga los mismos gráficos.
    pub const MAX_ENTRIES: u16 = 3;
    pub const SHARED_SEARCH: u16 = 47;
    pub const ENTRY_IN_USE: u8 = 0x84;
    /// Posición de lectura del anillo de entradas libres.
    pub const FREE_READ: u32 = 0x10_82C4;
    /// Cámara: posición (se resta para pasar a pantalla) y Y de pantalla.
    pub const CAMERA_X: u32 = 0x10_6F50;
    pub const CAMERA_Y: u32 = 0x10_6F54;
    pub const SCREEN_Y_BASE: u16 = 0x200;
}

/// Pantalla de título (la primera escena: logos animados, copyright,
/// cuenta atrás y "PUSH START" / "INSERT COIN").
pub mod title {
    /// Rutina de la animación por guion que corre en cada sprite (sin
    /// portar).
    pub const ANIM_RUN: u32 = 0x02_8D70;
    /// Rutinas de las tareas de la escena.
    pub const START: u32 = 0x09_1338;
    pub const IDLE: u32 = 0x09_13AA;
    pub const COUNTDOWN: u32 = 0x09_1578;
    pub const COUNTDOWN_TICK: u32 = 0x09_1594;
    pub const TEXT: u32 = 0x09_1514;
    pub const TEXT_HIDDEN: u32 = 0x09_152C;
    pub const TEXT_SHOWN: u32 = 0x09_1558;
    pub const SPRITE_ANIM: u32 = 0x09_150C;
    /// Los cuatro sprites: (rutina, gráficos, x, y, atributos, guion), en
    /// el orden en que se crean.
    pub const SPRITES: [(u32, u16, u16, u16, u16, u32); 4] = [
        (0x09_148C, 0xDA, 0x20, 0x138, 0xE000, 0x2F_504A),
        (0x09_144A, 0xD9, 0x130, 0x180, 0xD000, 0x2F_503E),
        (0x09_1408, 0xDE, 0xA0, 0x180, 0x2000, 0x2F_5056),
        (0x09_14CE, 0xDD, 0x90, 0x160, 0x0000, 0x2F_5062),
    ];
    /// Posición inicial de la escena y estado (bytes `+$20`/`+$21`).
    pub const POS: (u16, u16) = (0x90, 0x1C0);
    pub const STATE: [u32; 2] = [0x20, 0x21];
    /// Campos de la tarea de texto: contador, valor y periodo de parpadeo.
    pub const BLINK_TIMER: u32 = 0x82;
    pub const BLINK_AUX: u32 = 0x84;
    pub const BLINK_PERIOD_FIELD: u32 = 0x86;
    pub const BLINK_PERIOD: u16 = 0x10;
    /// Copyright: posición, texto (ROM) y atributos.
    pub const COPYRIGHT_POS: u16 = 0x711B;
    pub const COPYRIGHT_TEXT: u32 = 0x2F_4C98;
    pub const COPYRIGHT_ATTR: u16 = 0x2300;
    /// Mensaje parpadeante: zona (2 filas), atributos de borrado, y tabla
    /// de la ROM con un mensaje por estado de créditos (listas de
    /// caracteres acabadas en `0xFFFF`).
    pub const MESSAGE_POS: u16 = 0x7158;
    pub const MESSAGE_BLANK: u16 = 0x3480;
    pub const MESSAGE_COLS: u16 = 0x14;
    pub const MESSAGES: u32 = 0x2F_4CB8;
    /// Cuenta atrás: rótulo (ROM), posición, cifras y atributos.
    pub const TIMER_LABEL: u32 = 0x2F_4CB2;
    pub const TIMER_LABEL_POS: u16 = 0x7236;
    pub const TIMER_DIGITS: [u16; 2] = [0x72B6, 0x72D6];
    pub const TIMER_ATTR: u16 = 0x5300;
    /// Rótulo grande del modo alterno: dos bloques (posición, primer
    /// carácter) de columnas × filas.
    pub const BANNER: [(u16, u16); 2] = [(0x71C3, 0xC8C0), (0x71C7, 0xC9C0)];
    pub const BANNER_SIZE: (u16, u16) = (0x0C, 4);
}

/// Capa fija: códigos de los textos.
pub mod fix {
    pub const TEXT_END: u8 = 0xFE;
    pub const TEXT_NEWLINE: u8 = 0xFD;
    pub const LIST_END: u16 = 0xFFFF;
    /// Siguiente fila de caracteres dentro de un bloque.
    pub const TILE_ROW_STEP: u16 = 0x10;
}

/// Estado de créditos para los mensajes del título (bits).
pub mod credits {
    pub const P1: u16 = 1 << 0;
    pub const P2: u16 = 1 << 1;
    pub const CONSOLE: u16 = 1 << 2;
}

/// Alta de sprites en la lista de dibujo (`system::sprite`).
pub mod sprite_draw {
    /// Fin del almacén de registros de columna (en bytes desde
    /// `sprites::RECORDS`) y su dirección.
    pub const RECORDS_BYTES: u16 = 0x11D0;
    pub const RECORDS_END: u32 = super::sprites::RECORDS + RECORDS_BYTES as u32;
    /// SCB4 (X << 7) entre estos valores = columna invisible (entre el
    /// borde derecho y la vuelta por la izquierda).
    pub const OFFSCREEN_FIRST: u16 = 0xA000;
    pub const OFFSCREEN_END: u16 = 0xF800;
    /// Paso en SCB4 de una columna de 16 píxeles sin reducción.
    pub const COLUMN_STEP: u16 = 0x800;
    /// Temporales de la reducción horizontal: acumulador de fracción y paso.
    pub const SHRINK_FRACTION: u32 = 0x10_E1ED;
    pub const SHRINK_STEP: u32 = 0x10_E1EE;
    /// Tabla de reducción vertical de los trozos de hasta 16 tiles
    /// (fila = tiles − 1, columna = nibble bajo de la reducción).
    pub const SHRINK_V_TABLE: u32 = 0x05_A8BA;
    /// Reflejo en el agua: desactivado si este byte no es 0 (es el mismo
    /// que `script::KEY_LATCH[0]`, a `0xFF` fuera de la partida), sólo con
    /// la cámara hasta `WATER_CAMERA_MAX`; se mira el mapa de colisión
    /// `WATER_PROBE_DY` por debajo de la tarea y el agua es la celda
    /// `WATER_CELL`. La línea del agua está en la Y de mundo `WATER_LINE_Y`
    /// y el reflejo se dibuja con esta reducción vertical.
    pub const WATER_OFF: u32 = 0x10_6ECE;
    pub const WATER_CAMERA_MAX: i16 = 0x780;
    pub const WATER_PROBE_DY: u16 = 8;
    pub const WATER_CELL: u8 = 0x3A;
    pub const WATER_LINE_Y: u16 = 0xE8;
    pub const REFLECTION_SHRINK_V: u8 = 0x7F;
    /// Alto de la pantalla en el paso mundo → pantalla.
    pub const SCREEN_HEIGHT: u16 = 0x180;
    /// Mapa de colisión: origen, primer tile y celdas (4 KB: bloques de 2
    /// columnas × 64 filas de 8×8).
    pub const COLLMAP: u32 = 0x10_6F6C;
    pub const CM_ORIGIN_X: u32 = 0x04;
    pub const CM_ORIGIN_Y: u32 = 0x08;
    pub const CM_TILE_X: u32 = 0x22;
    pub const CM_TILE_Y: u32 = 0x24;
    pub const CM_CELLS: u32 = 0x7C;
}
