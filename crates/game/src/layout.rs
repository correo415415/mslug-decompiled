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
        (0x10_06C0, 0x03_240C),
        (0x10_0440, super::routine::IDLE),
        (0x10_04E0, super::routine::IDLE),
        (0x10_0620, 0x03_2406),
        (0x10_0580, super::routine::IDLE),
        (0x10_0760, 0x02_9582),
        (0x10_0300, super::routine::IDLE),
        (0x10_03A0, super::routine::IDLE),
        (0x10_0260, super::routine::IDLE),
        (0x10_01C0, super::routine::SCHEDULER_BOOTSTRAP),
    ];
    /// Tareas que la de arranque reinicia después (las entidades de los
    /// jugadores y otra).
    pub const BOOT_RESET: [u32; 3] = [0x10_0440, 0x10_04E0, 0x10_0580];
    /// Otro puntero de la tarea que se pone a `NIL` al darla de baja.
    pub const LINK: u32 = 0x48;
    /// Ranuras de paleta de la tarea: hasta 4 índices (words) y cuántas
    /// están ocupadas.
    pub const PAL_SLOTS: u32 = 0x16;
    pub const PAL_SLOT_COUNT: u32 = 0x1E;
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
    /// Vuelca una cola de sprites a la VRAM.
    pub const SPRITE_QUEUE_RENDER: u32 = 0x05_B400;
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
    pub const SCENE_TASKS: [u32; 8] =
        [0x10_0260, 0x10_0800, 0x10_08A0, 0x10_0440, 0x10_04E0, 0x10_0580, 0x10_0300, 0x10_03A0];
}
