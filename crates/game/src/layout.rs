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
    /// SCB3: el sprite se encadena a la derecha del anterior.
    pub const SCB3_STICKY: u16 = 0x40;
    /// SCB2: sin reducción.
    pub const SCB2_FULL_SIZE: u16 = 0x0FFF;
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
    /// Acceso a la tarjeta de memoria (parámetros en `bios_ram::CARD_*`).
    pub const CARD: u32 = 0xC0_0468;
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
    /// Órdenes de sistema: reiniciar el driver, preparar dos voces, parar
    /// un sonido, ajustes de efecto y de música (con su valor detrás).
    pub const CMD_SILENCE: u8 = 5;
    pub const CMD_RESET: u8 = 6;
    pub const CMD_STOP: u8 = 0x0C;
    pub const CMD_EFFECT_SETTING: u8 = 0x0A;
    pub const CMD_MUSIC_SETTING: u8 = 0x0B;
    /// Códigos de sonido (`hi << 8 | lo`): por encima de `CODE_LIMIT` no
    /// hay sonido; los de byte alto 1, 2 o 3 son mudos; los de byte alto 0
    /// o `>= MUSIC_FIRST` (músicas y voces) no suenan en la demo de la
    /// recreativa con `DEMO_SOUND` desactivado ni, si `SOUND_LOCK` está
    /// puesto, mientras dura el bloqueo.
    pub const CODE_LIMIT: u16 = 0x2000;
    pub const MUSIC_FIRST: u16 = 0x1000;
    /// Bloquea músicas y voces (`REPLY_AUX`, compartido con el driver).
    pub const SOUND_LOCK: u32 = REPLY_AUX;
    /// Ajuste de la recreativa (`DEMO SOUND`) y del cartucho: sonido en la
    /// demo.
    pub const DEMO_SOUND: u32 = 0x10_FD8D;
    pub const MVS_DEMO_SOUND: u32 = 0xD0_0046;
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
    /// Animación de paleta de una entrada (`$002C30`): datos desde
    /// `ANIM_DATA` (64 bytes por paso) y sus campos.
    pub const ANIM_DATA: u32 = 0x01_4E00;
    pub const ENTRY_ANIM_DATA: u32 = 0x0A;
    pub const ENTRY_ANIM_LEN: u32 = 0x08;
    pub const ENTRY_ANIM_LEFT: u32 = 0x10;
    pub const ENTRY_ANIM_STEP: u32 = 0x0E;
    pub const ENTRY_ANIM_MODE: u32 = 0x0F;
    pub const ANIMATING: u8 = 1 << 1;
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
    /// Texto de pausa: "PAUSE" y su borrado (cadenas de la ROM acabadas en
    /// 0), dónde va en la capa fija, paso de columna y atributos.
    pub const TEXT_SHOW: u32 = 0x01_3D32;
    pub const TEXT_HIDE: u32 = 0x01_3D38;
    pub const TEXT_AT: u16 = 0x725C;
    pub const TEXT_COLUMN: u16 = 0x20;
    pub const TEXT_ATTR: u16 = 0x2300;
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
    pub const DEMO_TIMEOUT_RUN: u32 = 0x00_1C94;
    pub const CREDITS_WAIT: u32 = 0x00_1C74;
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
    pub const SCRIPT_START_PRESSED: u32 = 0x00_0F76;
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
    pub const INDEX_MASK: u16 = 0x3E;
    pub const LAG: u16 = 0x15;
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
    // --- demo jugable (escena del attract tras el Game Over)
    /// Se pone a 0 al empezar la demo.
    pub const DEMO_FRAME: u32 = super::system::FRAME_COUNTER;
    /// Cuenta las demos: el bit 0 elige la demo y la misión que se juega.
    pub const DEMO_COUNT: u32 = super::system::TITLE_FLAG_C;
    /// Estado de la tabla de modos de cada demo.
    pub const DEMO_STATES: [usize; 2] = [1, 4];
    pub const DEMO_MISSION: u32 = 0x10_6ED0;
    pub const DEMO_TITLE_TASK: u32 = 0x09_1630;
    pub const DEMO_TIMEOUT_TASK: u32 = 0x00_1C88;
    /// La demo acaba sola tras estos frames.
    pub const DEMO_TIMEOUT: u16 = 0x4B0;
    pub const DEMO_TIMEOUT_TIMER: u32 = 0x22;
    /// Tabla de modos: (estado publicado en `KEY_LATCH[0]`, rutina para la
    /// tarea de la misión o ninguna, ¿crea antes la tarea del rango?).
    pub const MODES: [(u8, Option<u32>, bool); 11] = [
        (0, Some(0x09_67FE), false),
        (1, Some(0x09_6840), false),
        (2, Some(0x09_6882), false),
        (3, Some(0x09_68C4), false),
        (4, Some(0x09_6906), false),
        (5, Some(0x09_6948), false),
        (6, Some(0x09_6840), true),
        (7, Some(0x09_698A), false),
        (8, Some(0x09_6906), false),
        (9, Some(0x09_6906), false),
        (10, None, false),
    ];
    /// Tarea donde se instala la rutina de la misión.
    pub const MISSION_TASK: u32 = 0x10_0260;
    pub const RANK_TASK: u32 = 0x00_1B4C;
    pub const CREDITS_TASK: u32 = 0x00_1C44;
    /// Tareas fijas de la misión: (bloque, rutina).
    pub const MISSION_SLOTS: [(u32, u32); 2] = [(0x10_08A0, 0x05_EA96), (0x10_0800, 0x04_42E6)];
    // --- partida del modo de mensaje
    pub const PLAY_TIMER: u8 = 2;
    /// Estado de inicio de cada jugador (BIOS): 1 entra, 2 continúa.
    pub const PLAY_START_KEYS: u32 = super::memcard::START_KEYS_SRC;
    pub const PLAY_ENTERING: u8 = 1;
    pub const PLAY_CONTINUING: u8 = 2;
    /// Lista a la que se vuelve si no juega nadie.
    pub const PLAY_NOBODY: u32 = 0x00_0E42;
    /// 0xFF = la escena se retiene un frame.
    pub const HOLD: u32 = 0x10_6ED6;
    pub const ENDING_TASK: u32 = 0x03_DBC8;
    /// Índice del selector y dónde se guarda en los datos de la tarjeta.
    pub const SELECTION: u32 = super::memcard::MESSAGE_INDEX;
    pub const SAVE: u32 = super::memcard::SAVE;
    pub const SAVE_SELECTION: u32 = 0x15;
    pub const SAVE_START_KEYS: u32 = 0x16;
    /// Con el bit 0 de depuración se crea además esta tarea.
    pub const MISSION_DEBUG_TASK: u32 = 0x05_EFCA;
    pub const DEBUG: u32 = 0x10_0001;
    /// Tareas de la partida que se crean al empezar; la última recibe
    /// `+MISSION_HUD_FIELD = MISSION_HUD_MODE`.
    pub const MISSION_TASKS: [u32; 3] = [0x28_DB6A, 0x02_9588, 0x04_69E2];
    pub const MISSION_HUD_FIELD: u32 = 0x98;
    pub const MISSION_HUD_MODE: u8 = 0xD;
    pub const MISSION_TIMER: u32 = 0x44;
    pub const MISSION_TIMER_START: u8 = 0x3C;
    pub const MISSION_FLAG_B1: u32 = 0x10_81B1;
    /// Modo del HUD de la misión (lo copia la tarea `MISSION_TASKS[2]`).
    pub const MISSION_HUD: u32 = 0x10_81B0;
    /// Flags que ponen (0xFF) o quitan (0) tareas de un solo frame:
    /// (flag, rutina que lo pone, rutina que lo quita).
    pub const FLAG_TASKS: [(u32, u32, u32); 2] = [(0x10_81AE, 0x28_DB5A, 0x28_DB6A), (0x10_6F4A, 0x02_9588, 0x02_9598)];
    pub const SLUG_FRAME_FLAG: u32 = 0x10_6F4B;
    /// Retardo antes de arrancar los créditos en la misión.
    pub const CREDITS_DELAY: u16 = 0xF;
    pub const CREDITS_TIMER: u32 = 0x34;
    pub const CREDITS_NEXT: u32 = 0x07_A970;
    pub const CREDITS_MAX_MISSION: u8 = 6;
    /// Rutina de arranque de la tarea de control de cada jugador.
    pub const PLAYER_START: [u32; 2] = [0x02_575C, 0x02_5766];
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

/// Tarea de la misión (`system::mission`).
pub mod mission {
    /// Misión en curso (escena) dentro de la tarea.
    pub const INDEX: u32 = 0x21;
    /// Desplazamiento de la próxima aparición en su tabla.
    pub const CURSOR: u32 = 0x70;
    pub const SCRIPTED_PAD: u32 = 0x10_E39E;
    /// La misión 7 no tiene apariciones (ni pone el cursor a 0).
    pub const NO_SPAWNS: u8 = 7;
    /// Bucle de cada misión (por índice; la 6 no existe).
    pub const RUN: [u32; 8] =
        [0x09_6820, 0x09_6862, 0x09_68A4, 0x09_68E6, 0x09_6928, 0x09_696A, 0, 0x09_69A6];
    pub const ENTRIES: [(u32, u8); 7] = [
        (0x09_67FE, 0), (0x09_6840, 1), (0x09_6882, 2), (0x09_68C4, 3), (0x09_6906, 4), (0x09_6948, 5), (0x09_698A, 7),
    ];
    pub const EDGE_ENABLE: u32 = super::scroll::EDGE_ENABLE;
    pub const CAMERA_X: u32 = 0x10_6F50;
    pub const CAMERA_B: u32 = 0x10_6F5C;
    pub const DEBUG: u32 = 0x10_0001;
    pub const DEBUG_HUD: u8 = 1 << 0;
    pub const DEBUG_PICK_A: u8 = 1 << 2;
    pub const DEBUG_PICK_B: u8 = 1 << 7;
    pub const DEBUG_HUD_ROUTINE: u32 = 0x09_6A80;
    pub const DEBUG_PICK_ROUTINE: u32 = 0x09_6B24;
    /// Tablas de apariciones por misión (punteros) y última válida.
    pub const SPAWN_TABLES: u32 = 0x09_6B9C;
    pub const LAST_TABLE: u8 = 7;
    /// Entradas de 20 bytes: tipo, X (+2), rutina (+6); `END` = fin.
    pub const ENTRY_LEN: u16 = 0x14;
    pub const ENTRY_X: u32 = 2;
    pub const ENTRY_ROUTINE: u32 = 6;
    pub const END: u16 = 0xFFFF;
    /// Se crean al quedar a menos de esta distancia del borde izquierdo.
    pub const SPAWN_AHEAD: u16 = 0x140;
    pub const KIND_FIXED: u8 = 2;
    pub const SPAWN_PARENT: u32 = 0x10_08A0;
    pub const ENEMY_PARENT: u32 = 0x10_0800;
    pub const FIXED_FLAG: u8 = 1 << 1;
    /// La tarea creada guarda aquí su entrada.
    pub const ENTRY_PTR: u32 = 0x3C;
}

/// Colisión con el mapa (`system::collision`).
pub mod collision {
    /// Tipos de tile: 64 registros de 32 bytes {máscara sólida.l,
    /// pendiente.l, ..., flags (+$0C), siguiente tipo (+$0D)} y la segunda
    /// máscara a `+$10`. El tipo `$11` es el de las plataformas móviles.
    pub const TILE_TYPES: u32 = 0x27_8988;
    pub const TILE_TYPE_LEN: u32 = 0x20;
    pub const SECOND_MASK: u32 = 0x10;
    pub const T_FLAGS: u32 = 0x0C;
    pub const T_NEXT: u32 = 0x0D;
    pub const PLATFORM_TILE: u32 = 0x27_8BA8;
    pub const KIND_MASK: u8 = 0x3F;
    /// Por debajo de esta Y no se mira el mapa.
    pub const MAP_BOTTOM: i16 = 0x1FF;
    /// Entidad: tipos de tile que ignora (`+$6B`), `+$5B` bit 2 = atraviesa
    /// las celdas con los dos bits altos, `+$5A` bit 6 = pisa suelo.
    pub const IGNORE: u32 = 0x6B;
    pub const GROUND: u32 = 0x5B;
    pub const GROUND_PASS: u8 = 1 << 2;
    pub const PASS_BITS: u8 = 0xC0;
    pub const STATE: u32 = 0x5A;
    pub const ON_FLOOR: u8 = 1 << 6;
    pub const X: u32 = 0x22;
    pub const Y: u32 = 0x24;
    /// Plataforma bajo la entidad sondeada (`$F` = ninguna).
    pub const PLATFORM_UNDER: u32 = 0x10_6F44;
    /// Caída: base de `a5`, posición de trabajo (X/Y y fracciones), campos
    /// de la entidad y movimiento con colisión (sin portar).
    pub const SCRATCH_BASE: u32 = 0x10_8080;
    pub const WORK_POS: [u32; 2] = [0x10_6F38, 0x10_6F3A];
    pub const WORK_SUB: [u32; 2] = [0x10_6F3C, 0x10_6F3D];
    pub const SUB_X: u32 = 0x26;
    pub const SUB_Y: u32 = 0x27;
    pub const VEL_Y: u32 = 0x2A;
    pub const ACC_Y: u32 = 0x2E;
    pub const GRAVITY: u32 = 0x34;
    pub const NO_GRAVITY: u16 = 0xFFFF;
    pub const MAX_FALL: u16 = 0x400;
    pub const FALL_ACC: u16 = 0xFFC0;
}

/// Movimiento con colisión (`system::motion`).
pub mod motion {
    /// Velocidad y aceleración X, tope de velocidad, gravedad alternativa.
    pub const VEL_X: u32 = 0x28;
    pub const ACC_X: u32 = 0x2C;
    pub const MAX_SPEED: u16 = 0x800;
    pub const GRAVITY2: u32 = 0x64;
    /// `+$69`: paredes (bit 0 = a la derecha, bit 1 = a la izquierda).
    pub const WALLS: u32 = 0x69;
    pub const WALL_RIGHT: u8 = 1 << 0;
    pub const WALL_LEFT: u8 = 1 << 1;
    /// Zona de trabajo: píxeles que se mueve este frame, fracciones nuevas
    /// y cerrojos de avance (un frame).
    pub const STEP: [u32; 2] = [0x10_6F2C, 0x10_6F2E];
    pub const NEW_SUB: [u32; 2] = [0x10_6F30, 0x10_6F31];
    pub const LATCH: [u32; 2] = [0x10_6F3E, 0x10_6F40];
    /// Y de apoyo (`+$82`) con la que se mueven los vehículos, y su máscara.
    pub const BASE_Y: u32 = 0x82;
    pub const BASE_Y_MASK: u16 = 0x3FF;
    /// Escenas: sin bordes de pantalla y la del pozo (los jugadores no
    /// bajan de `PIT_Y`).
    pub const SCENE: u32 = 0x10_6ECE;
    pub const OPEN_SCENE: u8 = 9;
    pub const PIT_SCENE: u8 = 3;
    pub const PIT_Y: i16 = 0x1F0;
    /// Bordes de la pantalla para los jugadores; `FLAGS2` bit 7 = dentro.
    pub const LEFT_EDGE: i16 = 0x1E;
    pub const RIGHT_EDGE: i16 = 0x117;
    pub const INSIDE: u8 = 1 << 7;
    /// El Slug respeta los bordes salvo con el bit 5 de `+$8D`.
    pub const SLUG: u32 = 0x10_0580;
    pub const SLUG_FLAGS: u32 = 0x8D;
    pub const SLUG_FREE: u8 = 1 << 5;
    /// Tipos de tile que paran el avance en X (marcan el bit 5 de `+$5A`),
    /// techo (corta la subida) y tipo de las plataformas móviles.
    pub const WALL_KINDS: [u8; 8] = [0x01, 0x10, 0x26, 0x27, 0x0C, 0x0D, 0x0E, 0x0F];
    pub const HIT_WALL: u8 = 1 << 5;
    pub const CEILING_KIND: u8 = 0x28;
    pub const PLATFORM_KIND: u8 = 0x11;
    /// Tipo de tile: tabla de pendiente (`+4`) = {gravedad.w, alturas.w[8]
    /// (+2), fracción Y.b[8] (+$12), gravedad alternativa.w (+$1A)}.
    pub const T_SLOPE: u32 = 4;
    pub const T_SUB_Y: u32 = 0x12;
    pub const T_GRAVITY2: u32 = 0x1A;
    /// Tipo de tile: `+9` ≠ 0 = no se puede apoyar subiendo (motor E).
    pub const T_NO_CLIMB: u32 = 9;
    /// Pendientes de 45° con la velocidad vertical ×4.
    pub const STEEP_UP: u16 = 0x1000;
    pub const STEEP_DOWN: u16 = 0xF000;
    pub const ID: u32 = 0x6C;
    /// Avance con pendiente (`Entity_MoveXY_Probe_026992`): gravedad
    /// (pendiente) actual, empuje del scroll en la escena `PUSH_SCENE` y
    /// escalado de dy en pendientes suaves (`$AA/256`).
    pub const GRAVITY: u32 = 0x34;
    pub const NO_SLOPE: u16 = 0xFFFF;
    pub const SLOPE_STEEP: u16 = 0x1000;
    pub const SLOPE_GENTLE: u16 = 0x400;
    pub const SLOPE_FLAT: u16 = 0x80;
    pub const GENTLE_UP: u16 = 0x400;
    pub const GENTLE_DOWN: u16 = 0xFC00;
    pub const GENTLE_SCALE: i32 = 0xAA;
    pub const PUSH_SCENE: u8 = 1;
    pub const PUSH_ACTIVE: u32 = 0x10_6F5E;
    /// `+$5B` bit 3: sin avance vertical en llano.
    pub const FLAGS_5B: u32 = 0x5B;
    pub const NO_VERTICAL: u8 = 1 << 3;
    /// Los jugadores no bajan de `PIT_Y` en la escena del pozo.
    pub const PIT_FLOOR: i16 = 0x1F0;
}

/// Motor de colisión sobre pendientes (`system::slope`).
pub mod slope {
    /// Desplazamientos del punto de sondeo del suelo (6 pares x, y) según
    /// hacia dónde se mueve la entidad: derecha, izquierda, quieta.
    pub const PROBE_RIGHT: u32 = 0x02_6DC2;
    pub const PROBE_LEFT: u32 = 0x02_6DDA;
    pub const PROBE_NONE: u32 = 0x02_6DF2;
    /// Tipos de tile en los que el suelo está 8 / 16 píxeles más abajo, y
    /// rampas (también 8 más abajo si la entidad puede subirlas).
    pub const DEEP_KINDS: [u8; 9] = [0x0C, 0x0D, 0x0E, 0x0F, 0x1C, 0x1D, 0x1E, 0x1F, 0x34];
    pub const DEEPER_KINDS: [u8; 8] = [0x2C, 0x2D, 0x2E, 0x2F, 0x3C, 0x3D, 0x3E, 0x3F];
    pub const RAMP_KINDS: [u8; 4] = [0x20, 0x21, 0x30, 0x31];
    /// Rampas que suben hacia la derecha (A) y hacia la izquierda (B).
    pub const RAMP_A: [u8; 2] = [0x20, 0x30];
    pub const RAMP_B: [u8; 2] = [0x21, 0x31];
    /// Escalones que se suben yendo a la derecha / a la izquierda.
    pub const STEP_UP_RIGHT: [u8; 2] = [0x23, 0x33];
    pub const STEP_UP_LEFT: [u8; 2] = [0x22, 0x32];
    /// Paredes que paran a los vehículos.
    pub const WALL_KINDS: [u8; 2] = [0x10, 0x27];
    /// Tabla de pendiente del llano (la de las rampas que no se suben).
    pub const FLAT_SLOPE: u32 = 0x27_8704;
    /// Tipo de tile: tipo de celda (`+$0A`, 1 = con segunda máscara para
    /// el llano), tabla de pendiente (`+$04`), máscara de plataforma
    /// (`+8` dentro de la máscara), segunda máscara (`+$10`).
    pub const T_KIND_FLAGS: u32 = 0x0A;
    pub const T_SLOPE: u32 = 4;
    pub const PLATFORM_MASK: u32 = 8;
    pub const SECOND_MASK: u32 = 0x10;
    /// Tipo de las plataformas móviles.
    pub const PLATFORM_TILE: u32 = 0x27_8BA8;
    pub const PLATFORM_KIND: u8 = 0x11;
    /// Tabla de pendiente: gravedad (`+0`), alturas por columna (`+2`),
    /// fracción Y por columna (`+$12`), gravedad alternativa (`+$1A`).
    pub const S_HEIGHTS: u32 = 2;
    pub const S_SUB_Y: u32 = 0x12;
    pub const S_GRAVITY2: u32 = 0x1A;
    /// Entidad.
    pub const X: u32 = 0x22;
    pub const Y: u32 = 0x24;
    pub const SUB: u32 = 0x26;
    pub const VEL_X: u32 = 0x28;
    pub const GRAVITY: u32 = 0x34;
    pub const GRAVITY2: u32 = 0x64;
    pub const NO_SLOPE: u16 = 0xFFFF;
    pub const FACING: u32 = 0x6A;
    /// `+$13`: puede subir rampas (bit 4) / escalones (bit 5); `+$5B`: lo
    /// hace aunque vaya de espaldas.
    pub const FLAGS_13: u32 = 0x13;
    pub const RAMP_OK: u8 = 1 << 4;
    pub const STEP_OK: u8 = 1 << 5;
    pub const FLAGS_5B: u32 = 0x5B;
    pub const RAMP_FORCE: u8 = 1 << 4;
    pub const STEP_FORCE: u8 = 1 << 5;
    /// `+$5A`: subiendo una rampa (bit 3), contra una pared (bit 5).
    pub const STATE: u32 = 0x5A;
    pub const ON_RAMP: u8 = 1 << 3;
    pub const HIT_WALL: u8 = 1 << 5;
    /// Zona de trabajo de la colisión.
    pub const WORK_POS: [u32; 2] = [0x10_6F38, 0x10_6F3A];
    pub const WORK_SUB: [u32; 2] = [0x10_6F3C, 0x10_6F3D];
    pub const STEP: [u32; 2] = [0x10_6F2C, 0x10_6F2E];
    pub const NEW_SUB: [u32; 2] = [0x10_6F30, 0x10_6F31];
}

/// Ruedas del Slug (`system::wheels`): eslabones de la cadena del chasis.
pub mod wheels {
    /// Tipo (`+$98`; 0 = la rueda atraviesa las celdas con los dos bits
    /// altos, `+$5B` bit 2), tipos de tile ignorados, flags de la tarea.
    pub const KIND: u32 = 0x98;
    pub const FLAGS_5B: u32 = 0x5B;
    pub const PASS_TOP: u8 = 1 << 2;
    pub const IGNORE: u32 = 0x6B;
    pub const IGNORE_INIT: u8 = 1 << 1;
    pub const FLAGS_INIT: u8 = 1 << 6;
    /// Atributos del sprite: iniciales, bits que se conservan y capa.
    pub const ATTR: u32 = 0x38;
    pub const ATTR_INIT: u16 = 0xF000;
    pub const ATTR_KEEP: u16 = 0xFFE3;
    pub const ATTR_DEPTH: u16 = 0x10;
    /// Con el modo de depuración (`DEBUG` bit 4) el sprite se ve encima.
    pub const DEBUG: u32 = 0x10_0001;
    pub const DEBUG_WHEELS: u8 = 1 << 4;
    pub const DEBUG_ATTR: u16 = 0xF000;
    /// Dónde se escriben en la capa fija empujones, X e Y (depuración).
    pub const DEBUG_AT: [u16; 8] = [0x7412, 0x7413, 0x7415, 0x7416, 0x7417, 0x7419, 0x741A, 0x741B];
    /// Guion de animación y sprites.
    pub const SCRIPT: u32 = 0x29_DC90;
    pub const GFX: [u16; 4] = [7, 9, 8, 6];
    /// Tabla de cada rueda (delantera, trasera, del medio): distancia al
    /// centro del Slug en `+2`.
    pub const TABLES: [u32; 3] = [0x27_9668, 0x27_966E, 0x27_9674];
    pub const TABLE_DX: u32 = 2;
    /// Si es 0, el empujón vertical sólo mueve la Y del suelo, no la rueda.
    pub const PUSH_GROUND_ONLY: u32 = 0x27_9666;
    /// Rutinas del bucle (delantera/trasera y la del medio).
    pub const RUN: u32 = 0x03_01F6;
    pub const MIDDLE_RUN: u32 = 0x03_0156;
    /// Campos de la rueda.
    pub const X: u32 = 0x22;
    pub const Y: u32 = 0x24;
    pub const VEL_X: u32 = 0x28;
    pub const VEL_Y: u32 = 0x2A;
    pub const ACC_X: u32 = 0x2C;
    pub const ACC_Y: u32 = 0x2E;
    pub const FIELD_14: u32 = 0x14;
    pub const FIELD_16: u32 = 0x16;
    pub const FIELD_18: u32 = 0x18;
    pub const FIELD_1A: u32 = 0x1A;
    pub const FLAGS_13: u32 = 0x13;
    pub const STATE: u32 = 0x5A;
    pub const SURFACE_CHANGED: u8 = 1 << 4;
    pub const RISING: u32 = 0x5C;
    pub const RISING_ON: u8 = 0xFF;
    pub const FACING: u32 = 0x6A;
    pub const FACINGS: [u8; 2] = [0x7F, 0xFF];
    /// Distancia al centro del Slug y sentido de marcha con el que toma
    /// los permisos de subida del Slug (`ANY_SIDE` = nunca).
    pub const OFFSET: u32 = 0x7C;
    pub const CLIMB_SIDE: u32 = 0x7E;
    pub const CLIMB_SIDES: [u16; 2] = [0, 0x8000];
    pub const ANY_SIDE: u16 = 0xFFFF;
    /// `+$13`/`+$5B` bits 4–5: puede subir rampas y escalones.
    pub const CLIMB_BITS: u8 = 0x30;
    /// Posición del frame anterior.
    pub const PREV_X: u32 = 0x80;
    pub const PREV_Y: u32 = 0x82;
    /// Si pisa suelo, Y del suelo, empujón vertical pendiente.
    pub const ON_GROUND: u32 = 0x84;
    pub const GROUND_Y: u32 = 0x9A;
    pub const PUSH: u32 = 0x8E;
    /// La del medio: Y de la delantera y de la trasera.
    pub const FRONT_Y: u32 = 0x8A;
    pub const REAR_Y: u32 = 0x8C;
    /// Enlaces de la cadena (delantera, trasera, del medio).
    pub const LINKS: [u32; 3] = [0x70, 0x74, 0x78];
    /// Del Slug: pendiente (`NO_SLOPE` = en el aire), Y de apoyo, parada
    /// en seco (`+$13` bit 3), apoyado sobre las ruedas (`+$5B` bit 3).
    pub const GRAVITY: u32 = 0x34;
    pub const NO_SLOPE: u16 = 0xFFFF;
    pub const STEEP_UP: u16 = 0x1000;
    pub const STEEP_DOWN: u16 = 0xF000;
    pub const OWNER_BASE_Y: u32 = 0x82;
    pub const OWNER_FLAGS_13: u32 = 0x13;
    pub const OWNER_STOPPED: u8 = 1 << 3;
    pub const OWNER_ON_WHEELS: u8 = 1 << 3;
    /// Física en el aire: tipo de tile, hueco sobre la rueda, velocidad por
    /// debajo de la cual mira si ya pisa, separaciones máximas respecto a
    /// la Y de apoyo, gravedad y desnivel entre ruedas.
    pub const KIND_MASK: u8 = 0x3F;
    pub const HEAD_ROOM: u16 = 4;
    pub const SLOW_FALL: u16 = 8;
    pub const AIR_GAP: i16 = 0x18;
    pub const MAX_GAP: i16 = 0x12;
    pub const GRAVITY_STEP: u16 = 0xC0;
    pub const LEVEL_GAP: u16 = 0x1C;
}

/// Efecto de suelo bajo los pies (`system::ground_fx`).
pub mod ground_fx {
    /// Escenas en las que existe; en la `ALT_GFX_SCENE` usa otros sprites.
    pub const SCENE: u32 = 0x10_6ECE;
    pub const SCENES: [u8; 2] = [0, 9];
    pub const GFX: u16 = 0x10;
    pub const ALT_GFX: u16 = 0x138;
    pub const ALT_GFX_SCENE: u8 = 5;
    pub const ATTR: u32 = 0x38;
    pub const ATTR_INIT: u16 = 0xE000;
    /// Tablas de guiones de cada variante (A…E): impacto suelto, de la
    /// copia suelta, de aterrizaje y de marcha (en los campos
    /// `TABLE_FIELDS`, por ese orden).
    pub const TABLE_FIELDS: [u32; 4] = [0x74, 0x78, 0x7C, 0x80];
    pub const VARIANTS: [[u32; 4]; 5] = [
        [0x2D_CB84, 0x2D_CA62, 0x2D_CA82, 0x2D_CA1A],
        [0x2D_CA8A, 0x2D_CA4A, 0x2D_CA6A, 0x2D_C98A],
        [0x2D_CABC, 0x2D_CA52, 0x2D_CA72, 0x2D_C9BA],
        [0x2D_CAEE, 0x2D_CA5A, 0x2D_CA7A, 0x2D_C9EA],
        [0x2D_CB84, 0x2D_CA62, 0x2D_CA82, 0x2D_CA1A],
    ];
    pub const DETACH_SCRIPTS: u32 = 0x78;
    pub const LAND_SCRIPTS: u32 = 0x7C;
    pub const WALK_SCRIPTS: u32 = 0x80;
    pub const NO_SCRIPT: u32 = 0xFFFF_FFFF;
    /// Guion de fotogramas de cada animación de marcha (se copia a `FRAMES`).
    pub const WALK_FRAMES: u32 = 0x2D_C946;
    pub const FRAMES: u32 = 0x5C;
    /// Rutinas de la tarea.
    /// Dirección de vuelta de la comprobación de escena en cada alta.
    pub const START_RETURN: [u32; 5] = [0x07_768A, 0x07_76B8, 0x07_76E6, 0x07_7714, 0x07_7742];
    pub const TRACK: u32 = 0x07_78A2;
    pub const WAIT: u32 = 0x07_78FC;
    pub const LAND: u32 = 0x07_7912;
    pub const LAND_RUN: u32 = 0x07_7958;
    pub const WALK: u32 = 0x07_776C;
    pub const WALK_RUN: u32 = 0x07_7778;
    pub const DETACHED: u32 = 0x07_7A16;
    pub const DETACHED_RUN: u32 = 0x07_7A60;
    pub const FINISH: u32 = 0x07_7A8E;
    /// Impactos de disparo (A…D): su tabla de guiones (en `DETACH_SCRIPTS`)
    /// y las rutinas de la tarea. Con `SHOT_SCRIPTS[0]` el impacto se
    /// esparce al azar en X (de `SHOT_JITTER_MIN` a +7).
    pub const SHOT_SCRIPTS: [u32; 4] = [0x2D_CA6A, 0x2D_CA72, 0x2D_CA7A, 0x2D_CA82];
    pub const SHOT_RUN: u32 = 0x07_7AD6;
    pub const SHOT_START_RETURN: [u32; 4] = [0x07_7A9A, 0x07_7AAA, 0x07_7ABA, 0x07_7ACA];
    pub const SHOT_FIRE: u32 = 0x07_7B68;
    pub const SHOT_JITTER_MIN: i16 = -8;
    pub const SHOT_JITTER_MASK: u16 = 0xF;
    /// Rutina de la dueña cuando se está dando de baja.
    pub const OWNER_FINISHING: u32 = 0x52A;
    /// Campos.
    pub const X: u32 = 0x22;
    pub const Y: u32 = 0x24;
    pub const VEL_X: u32 = 0x28;
    pub const GRAVITY: u32 = 0x34;
    pub const WALK_IDX: u32 = 0x70;
    pub const NO_IDX: u16 = 0xFFFF;
    pub const MATERIAL: u32 = 0x72;
    pub const IN_AIR: u32 = 0x73;
    pub const PREV_X: u32 = 0x84;
    pub const PREV_Y: u32 = 0x86;
    pub const CELL: u32 = 0x88;
    pub const FLIP: u8 = 1 << 6;
    /// Material de la celda (bits altos del mapa): sin efecto (`PLAIN`,
    /// `PLAIN_C0`), agua (`MATERIAL_40`) y el `$80` con celdas hondas.
    pub const MATERIAL_BITS: u8 = 0xC0;
    pub const PLAIN: u8 = 0x00;
    pub const PLAIN_C0: u8 = 0xC0;
    pub const MATERIAL_40: u8 = 0x40;
    pub const MATERIAL_80: u8 = 0x80;
    pub const DEEP_A: u8 = 0x82;
    pub const DEEP_B: u8 = 0x8C;
    /// Índices de la animación de marcha: a la derecha, a la izquierda,
    /// celda honda; +1 en el tramo del mapa `SHALLOW_FROM..SHALLOW_TO`.
    pub const IDX_RIGHT: u16 = 4;
    pub const IDX_LEFT: u16 = 8;
    pub const IDX_DEEP: u16 = 2;
    pub const CAMERA_X: u32 = 0x10_6F50;
    pub const SHALLOW_FROM: u16 = 0xD20;
    pub const SHALLOW_TO: u16 = 0xE58;
    /// Nivel del agua (Y del mundo), más bajo en `LOW_WATER_SCENES`.
    pub const WATER: u16 = 0x167;
    pub const LOW_WATER: u16 = 0x1EF;
    pub const LOW_WATER_SCENES: [u8; 2] = [6, 9];
    /// Tipo de tile: `+9` ≠ 0 = no se puede estar de pie encima.
    pub const T_NO_STAND: u32 = 9;
}

/// Personaje del jugador (`system::player`).
pub mod player {
    /// Alta al empezar la misión: posición, sprites (cuerpo y dos de
    /// accesorios), tipo de entidad, el brazo y el estado de espera.
    pub const X: u32 = 0x22;
    pub const Y: u32 = 0x24;
    pub const SPAWN_X: u16 = 0x60;
    pub const SPAWN_Y: u16 = 0x171;
    pub const SPAWN_GFX: [u16; 3] = [0x176, 0x190, 0x191];
    pub const KIND: u32 = 0x1C;
    pub const KIND_PLAYER: u16 = 0x1B;
    pub const ARM_TASK: u32 = 0x03_94A8;
    pub const IDLE: u32 = 0x03_6D64;
    /// Alta al reaparecer en la misión (P1, P2): posición por defecto,
    /// sprites, tipo, ¿en pausa?, bits de atributo, brazo y la rutina que
    /// sigue según el tipo de entrada del marcador (0, 1, 2).
    pub const RESPAWN_X: [u16; 2] = [0x78, 0x50];
    pub const RESPAWN_Y: [u16; 2] = [0x1D8, 0x1E0];
    pub const RESPAWN_GFX: [[u16; 3]; 2] = [[0x176, 0x190, 0x191], [0x177, 0x190, 0x192]];
    pub const RESPAWN_KIND: [u16; 2] = [0x1B, 0x1C];
    pub const RESPAWN_PAUSED: [bool; 2] = [false, true];
    pub const RESPAWN_ATTR: [u16; 2] = [0x2, 0x0];
    pub const RESPAWN_ARM: [u32; 2] = [0x03_94A8, 0x03_94E6];
    pub const RESPAWN_ATTR_KEEP: u16 = 0xFFE3;
    pub const RESPAWN_ATTR_SET: u16 = 0x9FF0;
    pub const RESPAWN_WAIT: [u32; 2] = [0x59, 0x45];
    pub const RESPAWN_WAIT_FRAMES: u8 = 0x7F;
    pub const RESPAWN_NEXT: [u32; 3] = [0x03_3956, 0x03_6C8C, 0x03_37A8];
    /// Entidad del jugador 1 (las demás se colocan a la izquierda).
    pub const P1_ENTITY: u32 = 0x10_0440;
    /// Efecto de suelo bajo los pies (variante C).
    pub const GROUND_FX_TASK: u32 = 0x07_76E2;
    /// `+$12` bit 4: en pausa.
    pub const PAUSED: u8 = 1 << 4;
    /// Atributos del sprite: iniciales, «apareciendo» y la prioridad que
    /// se limpia cada frame.
    pub const ATTR: u32 = 0x38;
    pub const ATTR_INIT: u16 = 0x8010;
    pub const ATTR_SPAWNING: u16 = 0x2;
    pub const ATTR_CLEAR_PRIO: u16 = 0xFFFC;
    /// Velocidades, aceleraciones y fracciones.
    pub const VEL_X: u32 = 0x28;
    pub const VEL_Y: u32 = 0x2A;
    pub const ACC_X: u32 = 0x2C;
    pub const ACC_Y: u32 = 0x2E;
    pub const SUB_X: u32 = 0x26;
    pub const SUB_Y: u32 = 0x27;
    /// Atributo del sprite (el de este frame y el de siempre) y la tabla
    /// de campos de los que sale mientras parpadea.
    pub const SPRITE_ATTR: u32 = 0x14;
    pub const SPRITE_ATTR_BASE: u32 = 0x16;
    pub const BLINK_FIELDS: u32 = 0x03_29EE;
    pub const INVULN_TIMER: u32 = 0x87;
    /// Flags: atraviesa celdas de los bits altos (`+$5B` bit 2), sondea el
    /// suelo normal (`+$6B` bit 6), parado en seco y arriba/abajo
    /// mantenidos (`+$13` bits 3, 4 y 5).
    pub const FLAGS_5B: u32 = 0x5B;
    pub const PASS_TOP: u8 = 1 << 2;
    pub const IGNORE: u32 = 0x6B;
    pub const PLAIN_FLOOR: u8 = 1 << 6;
    pub const FLAGS_13: u32 = 0x13;
    pub const STOPPED: u8 = 1 << 3;
    pub const UP_HELD: u8 = 1 << 4;
    pub const DOWN_HELD: u8 = 1 << 5;
    /// Rutina que recibe los golpes.
    pub const HIT_HANDLER: u32 = 0x60;
    pub const HIT_ROUTINE: u32 = 0x03_2500;
    pub const LEVELS: [u32; 2] = [0x32, 0x33];
    /// Bombas y munición iniciales, estado.
    pub const BOMBS: u32 = 0x80;
    pub const START_BOMBS: u8 = 0x0A;
    pub const AMMO: u32 = 0x82;
    pub const BYTE_85: u32 = 0x85;
    pub const STATUS_FLAGS: u32 = 0x8C;
    pub const BYTE_21: u32 = 0x21;
    pub const CLEAR_WORDS: [u32; 2] = [0x72, 0x36];
    pub const CLEAR_BYTES: [u32; 4] = [0x91, 0x21, 0x71, 0x70];
    pub const FIELD_7C: u32 = 0x7C;
    pub const FIELD_7E: u32 = 0x7E;
    /// Dirección (adelante, arriba, abajo) y su historial.
    pub const AIM_DIR: u32 = 0x78;
    pub const AIM_HISTORY: u32 = 0x79;
    pub const AIM_FORWARD: u8 = 1;
    pub const AIM_UP: u8 = 4;
    pub const AIM_DOWN: u8 = 8;
    pub const FACING_FLAGS: u32 = 0x3A;
    pub const FACING_LEFT: u8 = 1 << 0;
    /// Tareas de control de cada jugador, piloto y mando (`NO_CONTROLLER`
    /// = ninguno).
    pub const CONTROL_TASKS: [u32; 2] = [0x10_0300, 0x10_03A0];
    pub const PILOT: u32 = 0x68;
    pub const CONTROLLER: u32 = 0x6D;
    pub const NO_CONTROLLER: u8 = 0xFF;
}

/// Jugador en el suelo (`system::player_ground`): aterrizaje, espera con
/// la pistola y la cola común que elige el siguiente estado.
pub mod player_ground {
    /// Al aterrizar tras bajarse del Slug: invulnerable unos frames (al
    /// golpe sólo con la escena cargada).
    pub const LAND_INVULN_FRAMES: u8 = 0x1E;
    pub const LAND_SCENE_LOADED: u32 = 0x10_6ED3;
    pub const LAND_TIMER_45: u32 = 0x45;
    pub const LAND_TIMER_59: u32 = 0x59;
    pub const VEL_X: u32 = 0x28;
    pub const VEL_Y: u32 = 0x2A;
    pub const ACC_X: u32 = 0x2C;
    pub const ACC_Y: u32 = 0x2E;
    pub const SUB_X: u32 = 0x26;
    pub const BYTE_21: u32 = 0x21;
    pub const FACING: u32 = 0x3A;
    pub const FACING_LEFT: u8 = 1 << 0;
    pub const HITBOX_CB: u32 = 0x48;
    pub const HITBOX_STAND: u32 = 0x03_26E0;
    pub const NO_HITBOX: u32 = 0xFFFF_FFFF;
    pub const FLAGS_5B: u32 = 0x5B;
    pub const GROUNDED: u8 = 1 << 7;
    pub const HIT_HANDLER: u32 = 0x60;
    pub const HIT_ROUTINE: u32 = 0x03_2500;
    pub const STATE: u32 = 0x70;
    pub const STATE_LANDED: u8 = 1;
    pub const STATE_IDLE: u8 = 0x21;
    /// Estados del cuerpo en los que la espera conserva la animación.
    pub const KEEP_ANIM: core::ops::Range<u8> = 0x13..0x24;
    pub const WEAPON: u32 = 0x71;
    pub const WEAPON_PISTOL: u8 = 0;
    pub const WEAPON_HEAVY: u8 = 1;
    pub const MODE: u32 = 0x72;
    pub const MODE_FORWARD: u16 = 2;
    pub const POSE: u32 = 0x74;
    pub const OFFSET_X: u32 = 0x7C;
    pub const OFFSET_Y: u32 = 0x7E;
    pub const AIM: u32 = 0x78;
    pub const AIM_HISTORY: u32 = 0x79;
    pub const AIM_FORWARD: u8 = 1;
    pub const AMMO: u32 = 0x82;
    pub const PISTOL_AMMO: u16 = 0x0A;
    pub const CROUCH_AMMO: u16 = 4;
    pub const RELOADED: u32 = 0x85;
    pub const FLAGS_88: u32 = 0x88;
    pub const WANTS_CROUCH: u8 = 1 << 0;
    /// `+$8C`: montado (0), brazo disparando (1), ocupado (2), bit 3 y
    /// arma nueva (4, `entity::STATUS_WEAPON_CHANGED`).
    pub const STATUS: u32 = 0x8C;
    pub const RIDING: u8 = 1 << 0;
    pub const ARM_FIRING: u8 = 1 << 1;
    pub const ARM_BUSY: u8 = 1 << 2;
    pub const STATUS_BIT3: u8 = 1 << 3;
    pub const NEW_WEAPON: u8 = 1 << 4;
    /// Guiones: aterrizaje y espera.
    pub const LAND_SCRIPT: u32 = 0x27_9A96;
    pub const IDLE_SCRIPT: u32 = 0x27_9828;
    /// Arma soltada (tarea hija).
    pub const DROPPED_WEAPON: u32 = 0x03_8BE4;
    /// Estados (rutinas).
    pub const LAND_LOOP: u32 = 0x03_3FAE;
    pub const IDLE: u32 = 0x03_3D64;
    pub const IDLE_LOOP: u32 = 0x03_3E2E;
    pub const IDLE_TAIL: u32 = 0x03_41A4;
    pub const STAND: u32 = 0x03_4704;
    pub const WALK_RIGHT: u32 = 0x03_4B38;
    pub const WALK_LEFT: u32 = 0x03_4D32;
    pub const TURN_TO_LEFT: u32 = 0x03_5BF8;
    pub const TURN_TO_RIGHT: u32 = 0x03_5ABA;
    pub const CROUCH_ENTER: u32 = 0x03_7C74;
    pub const CROUCH_IDLE: u32 = 0x03_827A;
    pub const CROUCH: u32 = 0x03_3A5E;
    pub const CROUCH_B: u32 = 0x03_3AFC;
    pub const RELOAD: u32 = 0x03_3B9A;
    pub const REENTER: u32 = 0x03_45B8;
    pub const MELEE: u32 = 0x03_5D34;
    pub const THROW: u32 = 0x03_60BC;
    pub const SHOOT_DOWN: u32 = 0x03_873C;
    pub const SHOOT_UP: u32 = 0x03_437E;
    pub const SHOOT: u32 = 0x03_42C4;
    pub const KNOCK_HOLD: u32 = 0x03_7018;
    pub const JUMP_START: u32 = 0x03_6914;
    /// De pie: bucle, golpe sin suelo y guiones (adelante y apuntando
    /// arriba; el original tiene otros dos para una rama que nunca se toma,
    /// `$02ABCC` siempre devuelve C = 0).
    pub const STAND_LOOP: u32 = 0x03_4806;
    /// Parada tras andar: bucle y la pose con la que se cambia a la de pie.
    pub const STAND_FROM_WALK_LOOP: u32 = 0x03_463E;
    pub const STAND_FROM_WALK_POSE: u32 = 0x03_9E54;
    pub const KNOCKBACK: u32 = 0x03_6D64;
    pub const STAND_SCRIPT: u32 = 0x27_96A4;
    pub const STAND_UP_SCRIPT: u32 = 0x27_96C4;
    pub const STATE_STAND: u8 = 0x10;
    pub const MODE_UP: u16 = 1;
    pub const AIM_UP: u8 = 4;
    /// Historial de apuntado con el que el cambio de pose pide transición.
    pub const AIM_HISTORY_TO_UP: u8 = 0x14;
    pub const AIM_HISTORY_TO_FORWARD: u8 = 0x41;
    pub const FLAGS_69: u32 = 0x69;
    pub const WANTS_POSE: u8 = 1 << 5;
    /// Guiones del disparo de pie: adelante y arriba, normal y al cambiar
    /// de modo.
    pub const SHOOT_SCRIPT: u32 = 0x27_96E4;
    pub const SHOOT_SCRIPT_FROM_UP: u32 = 0x27_9716;
    pub const SHOOT_UP_SCRIPT: u32 = 0x27_96EE;
    pub const SHOOT_UP_SCRIPT_FROM_FWD: u32 = 0x27_9720;
    /// Giro: estado, guiones (normal y apuntando arriba), velocidad hacia
    /// el lado al que gira, bucles y la cola de andar.
    pub const STATE_TURN: u8 = 0x31;
    pub const TURN_SCRIPT: u32 = 0x27_9A3C;
    pub const TURN_UP_SCRIPT: u32 = 0x27_9A5A;
    pub const TURN_SPEED: u16 = 0x300;
    pub const TURN_RIGHT_LOOP: u32 = 0x03_5B54;
    pub const TURN_LEFT_LOOP: u32 = 0x03_5C98;
    /// Con el lanzallamas la acción no espera a que el brazo esté libre.
    pub const WEAPON_FLAME: u8 = 4;
    /// Agachado: estado, guiones (agacharse y agachado quieto), golpe y
    /// caja de golpe, bucles y los estados a los que se pasa.
    pub const STATE_CROUCH: u8 = 0x30;
    pub const CROUCH_SCRIPT: u32 = 0x27_9864;
    pub const CROUCH_IDLE_SCRIPT: u32 = 0x27_983C;
    pub const CROUCH_HIT_ROUTINE: u32 = 0x03_2598;
    pub const HITBOX_CROUCH: u32 = 0x03_2734;
    pub const CROUCH_ENTER_LOOP: u32 = 0x03_7CD0;
    pub const CROUCH_IDLE_LOOP: u32 = 0x03_8306;
    pub const CROUCH_EXIT: u32 = 0x03_7EC2;
    pub const CROUCH_REENTER: u32 = 0x03_8044;
    pub const CRAWL_RIGHT: u32 = 0x03_842C;
    pub const CRAWL_LEFT: u32 = 0x03_855A;
    /// Gatear hacia el lado contrario al que mira.
    pub const CRAWL_RIGHT_BACK: u32 = 0x03_8688;
    pub const CRAWL_LEFT_BACK: u32 = 0x03_86E2;
    /// Gatear: guiones (adelante y hacia atrás), velocidad y bucles.
    pub const CRAWL_SCRIPT: u32 = 0x27_9A82;
    pub const CRAWL_BACK_SCRIPT: u32 = 0x27_9A78;
    pub const CRAWL_MOVE_SPEED: u16 = 0x120;
    pub const CRAWL_RIGHT_LOOP: u32 = 0x03_84BA;
    pub const CRAWL_LEFT_LOOP: u32 = 0x03_85E8;
    /// Lanzar agachado: guion, bucle y el agachado quieto C al acabar.
    pub const CROUCH_THROW_SCRIPT: u32 = 0x27_9D1A;
    pub const CROUCH_THROW_LOOP: u32 = 0x03_8976;
    pub const CROUCH_IDLE_C: u32 = 0x03_820E;
    pub const CROUCH_MELEE: u32 = 0x03_8A28;
    pub const CROUCH_THROW: u32 = 0x03_88F0;
    pub const CROUCH_EMPTY: u32 = 0x03_8086;
    pub const KNOCKBACK_DELAY: u32 = 0x03_6FC2;
    /// Velocidad al agacharse con un lado pulsado.
    pub const CRAWL_SPEED: u16 = 0x300;
    /// `+$8D`, bit 6: agachado este frame.
    pub const FLAGS_8D: u32 = 0x8D;
    pub const CROUCHED: u8 = 1 << 6;
    /// Agachado quieto tras disparar (B) o lanzar (C): otros guiones.
    pub const CROUCH_IDLE_B: u32 = 0x03_81A2;
    pub const CROUCH_IDLE_B_SCRIPT: u32 = 0x27_9850;
    pub const CROUCH_IDLE_C_SCRIPT: u32 = 0x27_985A;
    /// Disparo agachado: guion, bucle y la recarga agachado.
    pub const CROUCH_SHOOT_SCRIPT: u32 = 0x27_9D06;
    pub const CROUCH_SHOOT_LOOP: u32 = 0x03_87C2;
    pub const CROUCH_RELOAD: u32 = 0x03_8AE6;
    /// Levantarse: guion y bucle; al acabar puede seguir andando.
    pub const CROUCH_EXIT_SCRIPT: u32 = 0x27_986E;
    pub const CROUCH_EXIT_LOOP: u32 = 0x03_7F16;
    pub const WALK_LOOP_RIGHT: u32 = 0x03_51D8;
    pub const WALK_LOOP_LEFT: u32 = 0x03_54F2;
    /// Poses de espera de pie (estado 5): dos guiones y sus bucles.
    pub const STATE_POSE: u8 = 5;
    pub const POSE_A_SCRIPT: u32 = 0x27_973E;
    pub const POSE_B_SCRIPT: u32 = 0x27_981E;
    pub const POSE_A_LOOP: u32 = 0x03_3AC0;
    pub const POSE_B_LOOP: u32 = 0x03_3B5E;
    /// Lanzar una granada de pie: estado, guion, bucle y `+$3B` (se borra).
    pub const STATE_THROW: u8 = 0x33;
    pub const THROW_SCRIPT: u32 = 0x27_9D2E;
    pub const THROW_LOOP: u32 = 0x03_6152;
    pub const BYTE_3B: u32 = 0x3B;
    /// Cuchillo: estado, lista de ataques (`+$4C`), guiones (quieto, avance,
    /// retroceso y el del otro cuchillo), caja de golpe, velocidad y bucles.
    pub const STATE_MELEE: u8 = 0x33;
    pub const ATTACKS: u32 = 0x4C;
    pub const MELEE_ATTACKS: u32 = 0x03_2638;
    pub const MELEE_SCRIPT: u32 = 0x27_9D56;
    pub const MELEE_FWD_SCRIPT: u32 = 0x27_9D76;
    pub const MELEE_BACK_SCRIPT: u32 = 0x27_9E30;
    pub const MELEE_ALT_SCRIPT: u32 = 0x27_9EFE;
    pub const HITBOX_MELEE: u32 = 0x03_2788;
    pub const MELEE_SPEED: u16 = 0x300;
    pub const MELEE_RUN: u32 = 0x03_5DE0;
    pub const MELEE_ALT_LOOP: u32 = 0x03_5F40;
    /// Andar: estado, guiones (adelante y apuntando arriba), la pose que
    /// marca el cambio de apuntado, velocidad, bucles y los estados a los
    /// que se pasa (parar, disparar andando).
    /// Andar disparando: estados, guiones (mismo apuntado, transición y
    /// los alternativos del andar lanzado) y desplazamientos del brazo.
    pub const STATE_WALK_SHOOT: u8 = 0x22;
    pub const STATE_WALK_LONG_SHOOT: u8 = 0x23;
    pub const STATE_SHOOT_KEEP: u8 = 0x24;
    pub const WALK_SHOOT_SCRIPT: u32 = 0x27_98E0;
    pub const WALK_SHOOT_UP_SCRIPT: u32 = 0x27_98EA;
    pub const WALK_SHOOT_FROM_UP: u32 = 0x27_98F4;
    pub const WALK_SHOOT_UP_FROM_FWD: u32 = 0x27_98FE;
    pub const WALK_SHOOT_ALT_SCRIPT: u32 = 0x27_9908;
    pub const WALK_SHOOT_UP_ALT_SCRIPT: u32 = 0x27_9912;
    pub const WALK_SHOOT_OFFSET: (u16, u16) = (0, 0xFFFF);
    pub const WALK_LONG_SHOOT_OFFSET: (u16, u16) = (1, 3);
    pub const WALK_LONG_SHOOT_UP_OFFSET: (u16, u16) = (0, 4);
    pub const STATE_WALK: u8 = 0x11;
    pub const WALK_SCRIPT: u32 = 0x27_9882;
    pub const WALK_UP_SCRIPT: u32 = 0x27_9878;
    pub const WALK_AIM_POSE: u32 = 0x03_A058;
    pub const WALK_SPEED: u16 = 0x300;
    pub const WALK_RIGHT_LOOP: u32 = 0x03_4BCA;
    pub const WALK_LEFT_LOOP: u32 = 0x03_4DC4;
    pub const STAND_FROM_WALK: u32 = 0x03_45FA;
    pub const WALK_SHOOT: [u32; 2] = [0x03_4ECA, 0x03_51B0];
    pub const WALK_SHOOT_UP: [u32; 2] = [0x03_503E, 0x03_51C4];
    /// Andar ya lanzado: estado, guiones (adelante y apuntando arriba),
    /// bucles, parar desde aquí y disparar (adelante y arriba) por lado.
    pub const STATE_WALK_LONG: u8 = 0x12;
    pub const WALK_LONG_SCRIPT: u32 = 0x27_9926;
    pub const WALK_LONG_UP_SCRIPT: u32 = 0x27_991C;
    pub const WALK_LONG_RIGHT_RUN: u32 = 0x03_5276;
    pub const WALK_LONG_LEFT_RUN: u32 = 0x03_5590;
    pub const IDLE_STATE_CHECK: u32 = 0x03_4046;
    pub const WALK_LONG_SHOOT: [u32; 2] = [0x03_57AA, 0x03_57D2];
    pub const WALK_LONG_SHOOT_UP: [u32; 2] = [0x03_57BE, 0x03_57E6];
    /// Parar de andar: guion y bucle.
    pub const STOP_SCRIPT: u32 = 0x27_9A96;
    pub const STOP_LOOP: u32 = 0x03_411A;
}

/// Jugador en el aire (`system::player_air`): golpe recibido, caída,
/// selección de acción y animación en el aire, cola común y la
/// publicación del estado (muerte, caída al vacío).
pub mod player_air {
    pub const X: u32 = 0x22;
    pub const Y: u32 = 0x24;
    pub const VEL_X: u32 = 0x28;
    pub const VEL_Y: u32 = 0x2A;
    pub const ACC_X: u32 = 0x2C;
    pub const ACC_Y: u32 = 0x2E;
    /// Gravedad del golpe recibido y límites de velocidad.
    pub const KNOCK_GRAVITY: u16 = 0xFF80;
    pub const MAX_VEL_X: u16 = 0x300;
    pub const MAX_VEL_Y: u16 = 0x400;
    /// Frenado cuando no se pulsa dirección y queda velocidad horizontal.
    pub const BRAKE: u16 = 0x18;
    /// Aceleración según la dirección mantenida (0 ninguna, 1 izq., 2 der., 3 ambas).
    pub const VEL_TABLE: u32 = 0x03_24D0;
    pub const BYTE_21: u32 = 0x21;
    pub const ATTR: u32 = 0x38;
    pub const ATTR_BIT0: u16 = 1 << 0;
    pub const FACING: u32 = 0x3A;
    pub const FACING_LEFT: u8 = 1 << 0;
    pub const DELAY: u32 = 0x46;
    pub const DELAY_FRAMES: u8 = 0x0C;
    /// Retrollamada de la caja de golpe y rutina que recibe los golpes.
    pub const HITBOX_CB: u32 = 0x48;
    pub const HITBOX_AIR: u32 = 0x03_2830;
    pub const HIT_HANDLER: u32 = 0x60;
    pub const HIT_ROUTINE: u32 = 0x03_2500;
    pub const ATTACKS: u32 = 0x4C;
    pub const MELEE_ATTACKS: u32 = 0x03_25E4;
    pub const DEATH_KIND: u32 = 0x58;
    pub const DEATH_DROWN: u8 = 5;
    pub const FIELD_66: u32 = 0x66;
    pub const FLAGS_69: u32 = 0x69;
    pub const WALL_MASK: u8 = 0x03;
    pub const WALL_RIGHT: u8 = 1 << 1;
    pub const WALL_GRAB: u8 = 1 << 2;
    pub const WANTS_POSE: u8 = 1 << 5;
    /// Pose del brazo: estado (`+$70`), arma, modo, puntero a la pose y
    /// desplazamiento del brazo.
    pub const STATE: u32 = 0x70;
    pub const STATE_JUMP: u8 = 0x25;
    pub const STATE_FALL: u8 = 0x26;
    /// Salto: guiones (con carrerilla, apuntando abajo con y sin
    /// carrerilla; parado usa el del golpe), velocidad, gravedad y el
    /// contador que decide si es salto largo (B mantenido en el frame 4).
    pub const JUMP_RUN_SCRIPT: u32 = 0x27_9C78;
    pub const JUMP_DOWN_SCRIPT: u32 = 0x27_9B0E;
    pub const FALL_DOWN_SCRIPT: u32 = 0x27_9B04;
    pub const MODE_DOWN: u16 = 3;
    pub const JUMP_VEL: u16 = 0x9CD;
    pub const JUMP_GRAVITY: u16 = 0xFF3F;
    pub const JUMP_TIMER: u32 = 0x90;
    pub const JUMP_TIMER_FRAMES: u8 = 5;
    pub const JUMP_HOLD_FRAME: u8 = 4;
    pub const JUMP_WAIT: u32 = 0x03_6A36;
    pub const JUMP_AIR: u32 = 0x03_6A70;
    pub const MAX_JUMP_VEL_Y: u16 = 0x780;
    pub const WEAPON: u32 = 0x71;
    pub const MODE: u32 = 0x72;
    pub const POSE: u32 = 0x74;
    pub const OFFSET_X: u32 = 0x7C;
    pub const OFFSET_Y: u32 = 0x7E;
    pub const AIM: u32 = 0x78;
    pub const AIM_HISTORY: u32 = 0x79;
    pub const AIM_FORWARD: u8 = 1;
    pub const AIM_DOWN: u8 = 8;
    pub const AMMO: u32 = 0x82;
    pub const RING_ID: u32 = 0x86;
    /// `+$8C`: montado (0), brazo disparando (1), brazo ocupado (2), bit 3.
    pub const STATUS: u32 = 0x8C;
    pub const RIDING: u8 = 1 << 0;
    pub const ARM_FIRING: u8 = 1 << 1;
    pub const ARM_BUSY: u8 = 1 << 2;
    pub const STATUS_BIT3: u8 = 1 << 3;
    /// `+$8D`: en el aire (1) y bajando (5).
    pub const AIR: u32 = 0x8D;
    pub const IN_AIR: u8 = 1 << 1;
    pub const DESCENDING: u8 = 1 << 5;
    pub const FLAGS_13: u32 = 0x13;
    pub const DEAD: u8 = 1 << 0;
    pub const MELEE_HIT: u8 = 1 << 1;
    pub const DROWNED: u8 = 1 << 3;
    /// Guion del golpe recibido.
    pub const KNOCK_SCRIPT: u32 = 0x27_9B2C;
    /// Estados (rutinas) a los que se pasa.
    pub const KNOCK_AIR_CTRL: u32 = 0x03_6DCA;
    pub const KNOCK_DELAY_WAIT: u32 = 0x03_6FF6;
    pub const KNOCK_HOLD_WAIT: u32 = 0x03_7046;
    pub const LAND: u32 = 0x03_3EDE;
    pub const RIDE_SLUG: u32 = 0x03_64A2;
    pub const HANG_RING: u32 = 0x03_6638;
    pub const WALL_DROP: u32 = 0x03_68E0;
    pub const WALL_DROP_VEL: u16 = 0x400;
    pub const DEATH_PIT: u32 = 0x03_7B8E;
    /// Salto al bajarse del Slug: guion, impulso, gravedad, caja, velocidad
    /// y aceleración X por dirección mantenida, tope X y rutinas.
    pub const JUMP_OFF_SCRIPT: u32 = 0x27_9690;
    pub const JUMP_OFF_VEL: u16 = 0x87F;
    pub const JUMP_OFF_GRAVITY: u16 = 0xFF6F;
    pub const HITBOX_KNOCK: u32 = 0x03_2884;
    pub const JUMP_OFF_VEL_X: u32 = 0x03_24E8;
    pub const JUMP_OFF_ACC_X: u32 = 0x03_24D8;
    pub const JUMP_OFF_MAX_VX: u16 = 0x200;
    pub const JUMP_OFF_AIR: u32 = 0x03_6814;
    pub const LAND_INVULN: u32 = 0x03_3EC2;
    /// Bajada en paracaídas: tarea del paracaídas, estado, animación,
    /// velocidad inicial y máxima, altura a la que se suelta y rutinas.
    pub const PARACHUTE_TASK: u32 = 0x03_8CF6;
    pub const STATE_PARACHUTE: u8 = 4;
    pub const PARACHUTE_SCRIPT: u32 = 0x27_9B2C;
    pub const PARACHUTE_VEL_Y: u16 = 0xFF00;
    pub const PARACHUTE_MAX_VEL: u16 = 0x400;
    pub const PARACHUTE_FLOOR_Y: i16 = 0x170;
    pub const PARACHUTE_RUN: u32 = 0x03_39AA;
    pub const PARACHUTE_LANDED: u32 = 0x03_3E8C;
    pub const PARACHUTE_DROP: u32 = 0x03_6C8C;
    pub const NO_HITBOX: u32 = 0xFFFF_FFFF;
    pub const FRESH: u8 = 1 << 6;
    pub const ATTR_KEEP: u16 = 0xFFE3;
    pub const ATTR_AIR: u16 = 0x9FF0;
    /// `STATUS` bit 4: acaba de tocar suelo (suena el arma).
    pub const LANDED: u8 = 1 << 4;
    pub const LANDING_SOUNDS: u32 = 0x03_2D28;
    pub const DEATH_GROUND: u32 = 0x03_3572;
    pub const DEATH_AIR: u32 = 0x03_3578;
    /// Márgenes de la caída al vacío (la escena `PIT_SCENE` usa otros).
    pub const SCENE: u32 = 0x10_6ECE;
    pub const PIT_SCENE: u8 = 3;
    pub const PIT_MARGINS: [u32; 2] = [0x03_24C6, 0x03_24BC];
    /// Depuración: inmortal (bit 3 del byte bajo).
    pub const DEBUG: u32 = 0x10_0001;
    pub const DEBUG_IMMORTAL: u8 = 1 << 3;
    /// Depuración: el salto no mueve al jugador (bit 5).
    pub const DEBUG_FREEZE: u8 = 1 << 5;
    /// Con el temporizador a 0 el jugador muere; en la escena
    /// `WATER_SCENE`, por encima de `WATER_Y`, el tile `WATER_TILE` ahoga.
    pub const RANK_TIMER: u32 = 0x10_6E92;
    pub const WATER_SCENE: u8 = 1;
    pub const WATER_Y: i16 = 0x10C;
    pub const WATER_TILE: u8 = 0x40;
    /// Sonido de la muerte: el del primer jugador y el del resto.
    pub const P1: u32 = 0x10_0440;
    pub const DEATH_SOUNDS: [u16; 2] = [0x1123, 0x1087];
    /// Guiones de las poses del brazo ([saltando, cayendo]).
    pub const POSE_DOWN: [u32; 2] = [0x27_9B0E, 0x27_9B04];
    pub const POSE_FORWARD: [u32; 2] = [0x27_9B22, 0x27_9B18];
    pub const POSE_MELEE: u32 = 0x27_9D56;
    pub const POSE_THROW_FWD: u32 = 0x27_9D4C;
    pub const POSE_THROW: u32 = 0x27_9D42;
    pub const POSE_SHOOT_DOWN: [u32; 2] = [0x27_9C6E, 0x27_9AFA];
    pub const POSE_SHOOT_UP: [u32; 2] = [0x27_9C3C, 0x27_9AD2];
    pub const POSE_SHOOT_FWD: [u32; 2] = [0x27_9C32, 0x27_9AC8];
    pub const POSE_SHOOT: [u32; 2] = [0x27_9C28, 0x27_9ABE];
    /// Anillas a las que agarrarse (lista circular de 8 entradas de 6
    /// bytes: X, Y, id, dueño). Las ofrecidas van de `RING_HEAD` a
    /// `RING_SPLIT`, las cogidas de `RING_SPLIT` a `RING_TAIL`.
    pub const RINGS: u32 = 0x10_E362;
    pub const RING_HEAD: u32 = 0;
    pub const RING_TAIL: u32 = 2;
    pub const RING_SPLIT: u32 = 4;
    pub const RING_COUNT: u32 = 6;
    pub const RING_ENTRIES: u32 = 8;
    pub const RING_LEN: u16 = 6;
    pub const RING_SIZE: u16 = 0x30;
    pub const RING_FREE: u8 = 0xFE;
    pub const RING_TAKEN: u8 = 0xFF;
    pub const RING_MAX: u16 = 4;
    pub const PLAYERS: [u32; 2] = [0x10_0440, 0x10_04E0];
    /// Ventana de agarre alrededor de la anilla (medio ancho/alto).
    pub const RING_REACH: (u16, u16) = (0x18, 0x20);
    /// Caída libre al aparecer: indicador del jugador e invulnerabilidad.
    pub const MARKER_TASKS: [u32; 2] = [0x03_2112, 0x03_2142];
    pub const FREE_FALL_RUN: u32 = 0x03_6D20;
    pub const P1_ENTITY: u32 = 0x10_0440;
    pub const FREE_FALL_SHIELD: [u32; 2] = [0x59, 0x45];
    pub const FREE_FALL_SHIELD_BITS: u8 = 0x3C;
    pub const FREE_FALL_LANDED: u8 = 0x50;
}

/// Brazo del jugador (`system::player_arm`): tarea hija que dibuja el
/// brazo y el arma encima del cuerpo y elige su animación según el estado
/// del cuerpo.
pub mod player_arm {
    /// Una pose: su rutina y las tablas de guiones (una por arma y jugador,
    /// 10 punteros) entre las que elige.
    pub type Pose = (u32, &'static [u32]);
    /// Alta: sprites de cada jugador (cuerpo del brazo y dos de arma),
    /// tipo de entidad y guion inicial.
    pub const GFX: [[u16; 3]; 2] = [[0x176, 0x190, 0x191], [0x177, 0x190, 0x192]];
    pub const KIND: u32 = 0x1C;
    pub const KINDS: [u16; 2] = [0x1B, 0x1C];
    pub const START_SCRIPT: u32 = 0x27_AAEA;
    pub const RUN: u32 = 0x03_9532;
    /// Atributos del sprite: la prioridad que se pone al nacer (sólo P1) y
    /// la que se limpia cada frame.
    pub const ATTR: u32 = 0x38;
    pub const ATTR_PRIO: [u16; 2] = [0x2, 0x0];
    pub const ATTR_CLEAR_PRIO: u16 = 0xFFFC;
    /// `+$12` bit 4: segundo jugador.
    pub const FLAGS_12: u32 = 0x12;
    pub const SECOND: u8 = 1 << 4;
    /// `+$13` bit 0: muerto (el cuerpo lo libera).
    pub const FLAGS_13: u32 = 0x13;
    pub const DEAD: u8 = 1 << 0;
    pub const SPRITE_ATTR: u32 = 0x14;
    pub const SPRITE_ATTR_BASE: u32 = 0x16;
    pub const IGNORE: u32 = 0x6B;
    pub const PLAIN_FLOOR: u8 = 1 << 6;
    pub const LEVELS: [u32; 2] = [0x32, 0x33];
    /// Byte que el reinicio borra por un registro sin inicializar (que tras
    /// reservar los sprites apunta a la tabla de sprites).
    pub const STRAY: u32 = 0x87;
    pub const SPRITE_TABLE: u32 = 0x10_82C8;
    pub const RAM: core::ops::Range<u32> = 0x10_0000..0x11_0000;
    pub const STATUS: u32 = 0x8C;
    pub const STATUS_BIT2: u8 = 1 << 2;
    pub const MODE: u32 = 0x72;
    pub const BYTE_91: u32 = 0x91;
    pub const FIELD_36: u32 = 0x36;
    pub const ATTACK: u32 = 0x48;
    pub const NO_ATTACK: u32 = 0xFFFF_FFFF;
    /// Campos del cuerpo que se miran: estado, arma, desplazamiento del
    /// brazo, orientación, flags, la pose que pide y su modo.
    pub const OWNER_STATE: u32 = 0x70;
    pub const OWNER_WEAPON: u32 = 0x71;
    pub const OWNER_FLAGS_69: u32 = 0x69;
    pub const OWNER_WANTS_POSE: u8 = 1 << 5;
    pub const OWNER_POSE: u32 = 0x74;
    pub const OWNER_BYTE_21: u32 = 0x21;
    pub const OWNER_AIM: u32 = 0x78;
    pub const OWNER_AIM_UP: u8 = 1 << 2;
    pub const OWNER_HISTORY: u32 = 0x79;
    pub const OWNER_BOMBS: u32 = 0x80;
    pub const OFFSET_X: u32 = 0x7C;
    pub const OFFSET_Y: u32 = 0x7E;
    pub const X: u32 = 0x22;
    pub const Y: u32 = 0x24;
    pub const FACING: u32 = 0x3A;
    pub const FACING_LEFT: u8 = 1 << 0;
    pub const FLAGS_5A: u32 = 0x5A;
    pub const FLAG_5A_7: u8 = 1 << 7;
    pub const BYTE_59: u32 = 0x59;
    /// Segundo jugador: su arma usa la segunda mitad de cada tabla.
    pub const SECOND_PLAYER: u32 = 0x10_04E0;
    pub const SECOND_WEAPON_BASE: u16 = 5;
    /// Atributo del sprite según el nibble bajo de `+$87` (tabla de campos).
    pub const INVULN: u32 = 0x87;
    pub const ATTR_FIELDS: u32 = 0x03_29EE;
    /// Contador de las poses que alternan y la tabla de ataque que fijan.
    pub const TOGGLE: u32 = 0x30;
    pub const HIT_LIST: u32 = 0x4C;
    pub const MELEE_HITS: u32 = 0x03_92A4;
    /// Llamadas del guion: valor al azar en `+$47` y el sonido del golpe
    /// cuerpo a cuerpo (si `+$5A` bit 2).
    pub const RAND_47: u32 = 0x47;
    pub const RAND_47_MASK: u8 = 0x0A;
    pub const HIT_SOUND: u8 = 1 << 2;
    pub const MELEE_SOUND: u16 = 0x10FB;
    /// Estados del cuerpo con trato especial: los que repiten la pose al
    /// cambiar de modo (y el par de poses que lo hacen) y los de aire.
    pub const STATES_REPOSE: [(u8, [u32; 2]); 3] = [
        (0x10, [0x03_9E54, 0x03_9F56]),
        (0x11, [0x03_A058, 0x03_A108]),
        (0x12, [0x03_A1B8, 0x03_A268]),
    ];
    pub const STATES_KEEP: [u8; 3] = [0x21, 0x22, 0x23];
    pub const STATES_AIR: [(u8, u32); 4] = [(0x25, 0x03_A60A), (0x27, 0x03_A60A), (0x26, 0x03_A656), (0x28, 0x03_A656)];
    pub const POSE00: Pose = (0x03_9460, &[0x03_947C]);
    /// Montado en el Slug (dos poses) y la segunda caída.
    pub const SLUG_RIDE_A: Pose = (0x03_C632, &[0x03_C654]);
    pub const SLUG_RIDE_B: Pose = (0x03_C67E, &[0x03_C69A]);
    pub const FALL2: Pose = (0x03_C6C4, &[0x03_C6E6]);
    pub const CROUCH: Pose = (0x03_98A8, &[0x03_98CA]);
    pub const CROUCH_B: Pose = (0x03_98F4, &[0x03_9916]);
    pub const IDLE: Pose = (0x03_9940, &[0x03_9962]);
    pub const CROUCH_WEAPON_EMPTY: Pose = (0x03_998C, &[0x03_99AE]);
    pub const SPAWN_LAND: Pose = (0x03_99D8, &[0x03_99FA]);
    pub const SPAWN_LAND_B: Pose = (0x03_9A24, &[0x03_9A46]);
    pub const SHOOT_STAND_A: Pose = (0x03_9A70, &[0x03_9A8C]);
    pub const SHOOT_STAND_B: Pose = (0x03_9AB6, &[0x03_9AD8]);
    pub const SHOOT_STAND_C: Pose = (0x03_9B02, &[0x03_9B24]);
    pub const SHOOT_STAND_D: Pose = (0x03_9B4E, &[0x03_9B6A]);
    pub const SHOOT_STAND_UP_A: Pose = (0x03_9B94, &[0x03_9BBE, 0x03_9C1C]);
    pub const SHOOT_STAND_UP_B: Pose = (0x03_9C8E, &[0x03_9D10, 0x03_9CBE]);
    pub const MELEE_A: Pose = (0x03_9D82, &[0x03_9D9E]);
    pub const MELEE_B: Pose = (0x03_9DC8, &[0x03_9DE4]);
    pub const POSE: Pose = (0x03_9E0E, &[0x03_9E2A]);
    pub const STAND: Pose = (0x03_9E54, &[0x03_9E92, 0x03_9EE4, 0x03_9F2C]);
    pub const STAND_ALT: Pose = (0x03_9F56, &[0x03_9F94, 0x03_9FE6, 0x03_A02E]);
    pub const WALK: Pose = (0x03_A058, &[0x03_A096, 0x03_A0DE]);
    pub const WALK_ALT: Pose = (0x03_A108, &[0x03_A146, 0x03_A18E]);
    pub const WALK_LOOP: Pose = (0x03_A1B8, &[0x03_A1F6, 0x03_A23E]);
    pub const WALK_LOOP_ALT: Pose = (0x03_A268, &[0x03_A2A6, 0x03_A2EE]);
    pub const WALK_SHOOT_A: Pose = (0x03_A318, &[0x03_A334]);
    pub const WALK_SHOOT_B: Pose = (0x03_A35E, &[0x03_A37A]);
    pub const WALK_SHOOT_C: Pose = (0x03_A3A4, &[0x03_A3C6]);
    pub const WALK_SHOOT_D: Pose = (0x03_A3F0, &[0x03_A412]);
    pub const WALK_SHOOT_E: Pose = (0x03_A43C, &[0x03_A458]);
    pub const WALK_SHOOT_F: Pose = (0x03_A482, &[0x03_A49E]);
    pub const TURN_A: Pose = (0x03_A4C8, &[0x03_A4EA]);
    pub const TURN_B: Pose = (0x03_A514, &[0x03_A536]);
    pub const JUMP_MELEE: Pose = (0x03_A560, &[0x03_A58A, 0x03_A5D2]);
    pub const JUMP: Pose = (0x03_A60A, &[0x03_A62C]);
    pub const FALL: Pose = (0x03_A656, &[0x03_A678]);
    pub const AIR_UNREF: Pose = (0x03_A6A2, &[0x03_A6C4]);
    pub const AIR_B: Pose = (0x03_A6EE, &[0x03_A710]);
    pub const AIR_SHOOT_A: Pose = (0x03_A73A, &[0x03_A760, 0x03_A7B2, 0x03_A7FA]);
    /// Segunda entrada de `AirShootA` (sólo se llega por la tabla de animaciones).
    pub const AIR_SHOOT_A2: Pose = (0x03_A824, &[0x03_A84A, 0x03_A89A, 0x03_A8E2]);
    pub const AIR_SHOOT_B: Pose = (0x03_A90C, &[0x03_A932, 0x03_A984, 0x03_A9CC]);
    pub const AIR_SHOOT_C: Pose = (0x03_A9F6, &[0x03_AA1C, 0x03_AA6E, 0x03_AAB6]);
    pub const JUMP_SHOOT_A: Pose = (0x03_AAE0, &[0x03_AB06, 0x03_AB58, 0x03_ABAA, 0x03_ABF2]);
    pub const FALL_SHOOT_A: Pose = (0x03_AC1C, &[0x03_AC42, 0x03_AC94, 0x03_ACE6, 0x03_AD2E]);
    pub const JUMP_SHOOT_B: Pose = (0x03_AD58, &[0x03_AD7E, 0x03_ADD0, 0x03_AE22, 0x03_AE6A]);
    pub const FALL_SHOOT_B: Pose = (0x03_AE94, &[0x03_AEBA, 0x03_AF0C, 0x03_AF5E, 0x03_AFA6]);
    pub const JUMP_SHOOT_C: Pose = (0x03_AFD0, &[0x03_AFF6, 0x03_B048, 0x03_B09A, 0x03_B0E2]);
    pub const FALL_SHOOT_C: Pose = (0x03_B10C, &[0x03_B132, 0x03_B184, 0x03_B1D6, 0x03_B21E]);
    pub const JUMP_SHOOT_D: Pose = (0x03_B248, &[0x03_B26E, 0x03_B2C0, 0x03_B312, 0x03_B35A]);
    pub const FALL_SHOOT_D: Pose = (0x03_B384, &[0x03_B3AA, 0x03_B3FC, 0x03_B44E, 0x03_B496]);
    pub const JUMP_SHOOT_DOWN_A: Pose = (0x03_B4C0, &[0x03_B4E6, 0x03_B538, 0x03_B580]);
    pub const FALL_SHOOT_DOWN_A: Pose = (0x03_B5AA, &[0x03_B5D0, 0x03_B622, 0x03_B66A]);
    pub const JUMP_SHOOT_DOWN_B: Pose = (0x03_B694, &[0x03_B6BA, 0x03_B70C, 0x03_B754]);
    pub const FALL_SHOOT_DOWN_B: Pose = (0x03_B77E, &[0x03_B7A4, 0x03_B7F6, 0x03_B83E]);
    pub const JUMP_SHOOT_DOWN_C: Pose = (0x03_B868, &[0x03_B88E, 0x03_B8E0, 0x03_B928]);
    pub const FALL_SHOOT_DOWN_C: Pose = (0x03_B952, &[0x03_B978, 0x03_B9CA, 0x03_BA12]);
    pub const SHOOT_DOWN_UNREF_A: Pose = (0x03_BA3C, &[0x03_BA62, 0x03_BAB4, 0x03_BAFC]);
    pub const SHOOT_DOWN_UNREF_B: Pose = (0x03_BB26, &[0x03_BB4C, 0x03_BB9E, 0x03_BBE6]);
    pub const DEATH_A: Pose = (0x03_BC10, &[0x03_BC38]);
    pub const DEATH_B: Pose = (0x03_BC62, &[0x03_BC8A]);
    pub const DEATH_C: Pose = (0x03_BCB4, &[0x03_BCDC]);
    pub const DEATH_D: Pose = (0x03_BD06, &[0x03_BD2E]);
    pub const DEATH_E: Pose = (0x03_BD58, &[0x03_BD80]);
    pub const DEATH_F: Pose = (0x03_BDAA, &[0x03_BDD2]);
    pub const DEATH_G: Pose = (0x03_BDFC, &[0x03_BE24]);
    pub const DEATH_H: Pose = (0x03_BE4E, &[0x03_BE76]);
    pub const SPAWN_FALL: Pose = (0x03_BEA0, &[0x03_BEC2]);
    pub const CROUCH_ENTER_A: Pose = (0x03_BEEC, &[0x03_BF0E]);
    pub const CROUCH_UNREF: Pose = (0x03_BF38, &[0x03_BF5A]);
    pub const CROUCH_ENTER_B: Pose = (0x03_BF84, &[0x03_BFA6]);
    pub const CROUCH_IDLE_A: Pose = (0x03_BFD0, &[0x03_BFFC]);
    pub const CROUCH_IDLE_B: Pose = (0x03_C06E, &[0x03_C09A]);
    pub const CROUCH_SHOOT: Pose = (0x03_C10C, &[0x03_C138]);
    pub const CRAWL_A: Pose = (0x03_C1AA, &[0x03_C1CC]);
    pub const CRAWL_UNREF: Pose = (0x03_C1F6, &[0x03_C218]);
    pub const CRAWL_B: Pose = (0x03_C242, &[0x03_C264]);
    pub const CROUCH_GRENADE: Pose = (0x03_C28E, &[0x03_C2AA]);
    pub const MELEE_C: Pose = (0x03_C2D4, &[0x03_C2FC, 0x03_C344]);
    pub const AIR_MELEE_A: Pose = (0x03_C36E, &[0x03_C394, 0x03_C3E6, 0x03_C42E]);
    pub const AIR_MELEE_B: Pose = (0x03_C458, &[0x03_C47E, 0x03_C4D0, 0x03_C518]);
    pub const CROUCH_MELEE: Pose = (0x03_C542, &[0x03_C55E]);
    pub const CROUCH_RELOAD: Pose = (0x03_C588, &[0x03_C5B2, 0x03_C5FA]);
}

/// Plataformas móviles (`system::platforms`).
pub mod platforms {
    /// Anillo de 16 muestras de 12 bytes: cola de lectura, fin de las
    /// muestras válidas, cabeza de escritura y último identificador dado.
    pub const RING: u32 = 0x10_E3BE;
    pub const RING_LEN: u16 = 0xC0;
    pub const SAMPLE_LEN: u16 = 0x0C;
    pub const TAIL: u32 = 0x10_E47E;
    pub const WRAP: u32 = 0x10_E480;
    pub const HEAD: u32 = 0x10_E482;
    pub const NEXT_ID: u32 = 0x10_E484;
    pub const IDS: u8 = 0x0E;
    /// Campos de una muestra (el identificador va en el byte 0).
    pub const S_X: u32 = 2;
    pub const S_Y: u32 = 4;
    pub const S_W: u32 = 6;
    pub const S_DX: u32 = 8;
    pub const S_DY: u32 = 0x0A;
    /// Grosor de una plataforma (se está encima hasta esta distancia).
    pub const DEPTH: u16 = 0x14;
    /// Byte de identificadores de la entidad (alto: el suyo; bajo: el de la
    /// plataforma en la que está) y valores especiales del nibble.
    pub const ID: u32 = 0x6C;
    pub const NONE: u8 = 0x0F;
    pub const LEAVING: u8 = 0x0E;
    pub const X: u32 = 0x22;
    pub const Y: u32 = 0x24;
    /// `FLAGS2`: bit 6 = ya movida este frame, bit 7 = la plataforma la
    /// arrastra aunque toque pared.
    pub const MOVED: u8 = 1 << 6;
    pub const RIDE_ALWAYS: u8 = 1 << 7;
    /// `+$6B`: bits de contacto con pared.
    pub const STATE: u32 = 0x6B;
    pub const WALL_BITS: u8 = (1 << 6) | (1 << 1) | (1 << 0);
    pub const EDGE: i16 = 0x1E;
    pub const LAYER_POS: [u32; 2] = [0x10_6F6C, 0x10_6F6E];
    /// En esta escena, con el bloqueo puesto, no se mueve en X.
    pub const SCENE: u32 = 0x10_6ECE;
    pub const LOCKED_X_SCENE: u8 = 1;
    pub const LOCK_X: u32 = 0x10_6F5E;
    /// Objetos: sujeto (`+$9B`) y base de `a5` de la colisión.
    pub const HELD: u32 = 0x9B;
    pub const SCRATCH_BASE: u32 = 0x10_8080;
    /// Posición de trabajo de la colisión (copia de X/Y de la entidad).
    pub const WORK_POS: [u32; 2] = [0x10_6F38, 0x10_6F3A];
}

/// Comida (`system::food`).
pub mod food {
    /// Por variante: tipo de premio (`+$9C`), guion de animación, gráfico.
    pub const VARIANTS: [(u8, u32, u16); 20] = [
        (0, 0x2F_6B78, 0x1AC), (0, 0x2F_6B22, 0x1AC), (1, 0x2F_6AE0, 0x1AC), (0, 0x2F_68FC, 0x1AC),
        (0, 0x2F_6BCE, 0x1C6), (0, 0x2F_6CCA, 0x1C6), (0, 0x2F_6E4C, 0x1C6), (1, 0x2F_7002, 0x1AC),
        (0, 0x2F_706C, 0x1AC), (0, 0x2F_70B8, 0x1C6), (0, 0x2F_7186, 0x1AC), (0, 0x2F_7204, 0x1AC),
        (0, 0x2F_7290, 0x1C6), (0, 0x2F_73B8, 0x1C6), (0, 0x2F_745E, 0x1C6), (0, 0x2F_74A0, 0x1AC),
        (2, 0x2F_75F2, 0x1C6), (1, 0x2F_76E8, 0x1AC), (0, 0x2F_755A, 0x1AC), (0, 0x2F_75B0, 0x1C6),
    ];
    /// Entrada de cada variante en la ROM.
    pub const ENTRIES: [u32; 20] = [
        0x09_AEC4, 0x09_AED8, 0x09_AEEC, 0x09_AF02, 0x09_AF16, 0x09_AF2A, 0x09_AF3E, 0x09_AF52, 0x09_AF68, 0x09_AF7C,
        0x09_AF90, 0x09_AFA4, 0x09_AFB8, 0x09_AFCC, 0x09_AFE0, 0x09_AFF4, 0x09_B008, 0x09_B01E, 0x09_B034, 0x09_B048,
    ];
    pub const KIND: u32 = 0x9C;
    pub const NO_FLIP: u32 = 0x9A;
    pub const FLIP: u32 = 0x3A;
    pub const LIFE_PARAM: u32 = 0x99;
    pub const FOREVER: u16 = 0x7FFF;
    pub const TIMER: u32 = 0x70;
    pub const START_DELAY: u8 = 8;
    pub const BLINK_FRAMES: u8 = 0x1E;
    pub const ATTR: u16 = 0x8000;
    pub const ATTR_WORD: u32 = 0x38;
    pub const ATTR_MASK: u16 = 0xFFE3;
    /// Estados.
    pub const WAIT: u32 = 0x09_B0A8;
    pub const BLINK: u32 = 0x09_B0D8;
    pub const BLINK_RUN: u32 = 0x09_B0FA;
    pub const FREE: u32 = 0x09_B474;
}

/// Animaciones de la capa fija (`system::hud::fix_anim_*`).
pub mod fix_anim {
    /// Tarea: celda de la capa fija, cuadro actual y frames que le quedan.
    pub const CELL: u32 = 0x22;
    pub const CURSOR: u32 = 0x3C;
    pub const LEFT: u32 = 0x46;
    /// Cuadros de 8 bytes {tile, columnas, filas, frames}.
    pub const FRAME_LEN: u32 = 8;
    pub const FRAMES: u32 = 6;
    pub const END: u16 = 0xFFFF;
    pub const BLANK: u16 = 0xFF;
}

/// Golpes entre entidades (`system::hits`).
pub mod hits {
    /// Lista de cajas de ataque de la entidad: registros de `ATTACK_LEN`
    /// bytes = {tipo de víctima, clase (+1), daño.w (+2), potencia (+4),
    /// forma (+5: 0 punto, 4 rectángulo), siempre activa (+6), …, caja
    /// (+$0A)}; `END` = fin.
    pub const ATTACKS: u32 = 0x4C;
    /// Sonido del golpe que acierta (`Attack_ApplyThenSound10FA_05e8da`).
    pub const HIT_SOUND: u16 = 0x10FA;
    pub const ATTACK_LEN: u32 = 0x50;
    pub const END: u8 = 0xFF;
    pub const NONE: u32 = 0xFFFF_FFFF;
    pub const CLASS: u32 = 1;
    pub const DAMAGE: u32 = 2;
    pub const POWER: u32 = 4;
    pub const SHAPE: u32 = 5;
    pub const ALWAYS: u32 = 6;
    pub const BOX_HEADER: u32 = 0x0A;
    /// Cajas de golpe de la víctima: registros de `BOX_LEN` bytes =
    /// {marca.l (`NONE` = fin; el byte +1 es el aguante), inmunidades.l
    /// (+2, `NONE` = las de por defecto), rectángulo (+$0A)}.
    pub const HITBOX: u32 = 0x48;
    pub const BOX_LEN: u32 = 0x50;
    pub const IMMUNITY: u32 = 2;
    pub const TOUGHNESS: u32 = 1;
    pub const DEFAULT_IMMUNITY: u32 = 0x27_928C;
    /// Víctimas: Slug y raíces de las capas de enemigos.
    pub const SLUG: u32 = 0x10_0580;
    pub const ENEMY_ROOTS: [u32; 2] = [0x10_0800, 0x10_08A0];
    /// Campos de las entidades.
    pub const POS: [u32; 2] = [0x22, 0x24];
    pub const FLIP: u32 = 0x3A;
    pub const FLAGS2: u32 = 0x13;
    pub const FLAGS3: u32 = 0x5A;
    pub const DEAD: u8 = 1 << 0;
    pub const SPENT: u8 = 1 << 1;
    pub const BLOCKED: u8 = 1 << 2;
    pub const HIT: u8 = 1 << 3;
    pub const HIT_FLASH: u8 = 1 << 0;
    pub const INVULNERABLE: u32 = 0x45;
    pub const ATTACK_CLASS: u32 = 0x58;
    pub const ATTACKER: u32 = 0x50;
    pub const DODGE: u32 = 0x6B;
    pub const DODGED_CLASS: u8 = 0x0B;
    pub const IMPACT: [u32; 2] = [0x54, 0x56];
    pub const HP: u32 = 0x66;
    /// Puntos por daño: jugador dueño del ataque, último jugador que le
    /// dio, bit de «da puntos», puntuaciones (8 dígitos) y tabla de puntos
    /// BCD por clase de ataque.
    pub const OWNER_PLAYER: u32 = 0x68;
    pub const LAST_PLAYER: u32 = 0x6E;
    pub const GIVES_POINTS: u8 = 1 << 1;
    pub const SCORES: [u32; 2] = [0x10_6E94, 0x10_6E9C];
    pub const DAMAGE_POINTS: u32 = 0x05_188C;
    pub const SCORE_SCRATCH: u32 = 0x10_81B6;
    /// Pantalla {izq, der, abajo, arriba} y márgenes por defecto.
    pub const SCREEN: (u16, u16, u16, u16) = (0, 0x140, 0x100, 0x1F0);
    pub const DEFAULT_MARGINS: u32 = 0x02_8926;
}

/// Fuerte de la escena 4 y ayudantes comunes de sus piezas (`system::fort`).
pub mod fort {
    /// Registro de aparición (`+$3C` de la tarea): byte 0 distinto de 0 =
    /// sin gráficos propios; flags (+1), X (+2), Y (+4), margen (+$0A),
    /// atributos (+$0C), gráficos (+$0E) y guion de animación (+$10).
    pub const REC_PTR: u32 = 0x3C;
    pub const REC_NO_GFX: u32 = 0;
    pub const REC_FLAGS: u32 = 1;
    pub const REC_X: u32 = 2;
    pub const REC_Y: u32 = 4;
    pub const REC_MARGIN: u32 = 0x0A;
    pub const REC_ATTR: u32 = 0x0C;
    pub const REC_GFX: u32 = 0x0E;
    pub const REC_ANIM: u32 = 0x10;
    pub const NO_GFX: u16 = 0xFFFF;
    pub const NO_ANIM: u32 = 0xFFFF_FFFF;
    /// En las misiones `$0B` y `$0C` la X del registro es de 12 bits.
    pub const MISSION: u32 = 0x10_6ECE;
    pub const WRAPPED_MISSIONS: [u8; 2] = [0x0B, 0x0C];
    pub const X_MASK: u16 = 0x0FFF;
    /// Campos de la tarea.
    pub const X: u32 = 0x22;
    pub const Y: u32 = 0x24;
    pub const ATTR: u32 = 0x38;
    pub const FLAGS: u32 = 0x12;
    pub const FLAGS2: u32 = 0x13;
    pub const FLAGS3: u32 = 0x5A;
    /// `FLAGS2`: muerta (bit 0) y golpeada este frame (bit 3).
    pub const DEAD: u8 = 1 << 0;
    pub const HIT: u8 = 1 << 3;
    /// `FLAGS3` bit 0: suena al recibir golpes.
    pub const HIT_SOUND_ON: u8 = 1 << 0;
    pub const ATTACK_CLASS: u32 = 0x58;
    pub const HITBOX: u32 = 0x48;
    pub const NO_HITBOX: u32 = 0xFFFF_FFFF;
    /// Caja de golpe: bordes izquierdo y derecho (+$0A, +$0C), y X del
    /// impacto en la tarea (+$54).
    pub const HITBOX_LEFT: u32 = 0x0A;
    pub const HITBOX_RIGHT: u32 = 0x0C;
    pub const IMPACT_X: u32 = 0x54;
    pub const HP: u32 = 0x66;
    pub const MARGIN: u32 = 0x70;
    pub const INDEX: u32 = 0x72;
    pub const BLIT: u32 = 0x74;
    pub const WRECK: u32 = 0x78;
    pub const WATCH: u32 = 0x7C;
    pub const VARIANT: u32 = 0x20;
    pub const STATE: u32 = 0x21;
    pub const STATE_GONE: u8 = 0xFF;
    /// Sonido del golpe por clase de ataque (5 entradas, `0xFFFF` = nada).
    pub const HIT_SOUNDS: u32 = 0x05_E766;
    pub const HIT_CLASSES: u16 = 5;
    pub const NO_SOUND: u16 = 0xFFFF;
    /// El fuerte: margen, vida, caja de golpe y estados.
    pub const FORT_MARGIN: u16 = 0xC0;
    pub const FORT_HP: u16 = 0x140;
    pub const FORT_HITBOX: u32 = 0x2E_C41E;
    /// A la izquierda de esta X ya no se le puede dar; a la derecha de
    /// `FULL_HP_X` recupera la vida.
    pub const NO_HIT_X: i16 = -0x40;
    pub const FULL_HP_X: i16 = 0xC0;
    pub const RUN: u32 = 0x08_675A;
    pub const DESTROYED: u32 = 0x08_67D4;
    /// Por variante: restos, lista de la siguiente oleada, trampilla y si
    /// ésta va 16 píxeles más abajo.
    pub const WRECKS: [u32; 5] = [0x2E_DCB0, 0x2E_DCBA, 0x2E_DCFE, 0x2E_DD0C, 0x2E_DD20];
    pub const WATCHES: [u32; 5] = [0x0E_DC8C, 0x0E_DCD2, 0x0E_DD2A, 0x0E_DDA6, 0x0E_DE22];
    pub const HATCHES: [u32; 5] = [0x08_6E4A, 0x08_6E74, 0x08_6E9E, 0x08_6EC8, 0x08_6EF2];
    pub const HATCH_DX: u16 = 0x50;
    pub const HATCH_LOW: [bool; 5] = [true, false, true, false, true];
    pub const HATCH_DY: u16 = 0x10;
    /// Piezas: casamatas (cuatro orientaciones) y cajas (una por variante).
    pub const PILLBOX: u32 = 0x08_686A;
    pub const PILLBOX_LOWER: u32 = 0x08_6892;
    pub const PILLBOX_RIGHT: u32 = 0x08_68A6;
    pub const PILLBOX_RIGHT_UPPER: u32 = 0x08_68BA;
    pub const CRATE: [u32; 5] = [0x08_6CD0, 0x08_6CE4, 0x08_6CF8, 0x08_6D0C, 0x08_6D20];
    /// Pieza: rutina, desplazamiento X e Y desde el fuerte, atributos,
    /// tabla de dibujo y lista de la siguiente oleada.
    pub type Piece = (u32, u16, u16, Option<u16>, u32, Option<u32>);
    /// Trampilla: margen por variante (las 5 y 6 se colocan solas desde su
    /// registro), listas de las oleadas, temporizador y esperas, tablas de
    /// animación de abrir y cerrar (por variante, `NO_ANIM` = ninguna) y
    /// estados.
    pub const HATCH_MARGIN: [u16; 7] = [0x30, 0x30, 0x30, 0x20, 0x20, 0x30, 0x30];
    pub const HATCH_WATCHES: [u32; 7] = [0x0E_DE9E, 0x0E_DEDA, 0x0E_DEFA, 0x0E_DF1A, 0x0E_DF52, 0x0E_DF72, 0x0E_DFAA];
    pub const HATCH_FINAL_WATCHES: [u32; 5] = [0x0E_DCB2, 0x0E_DD0A, 0x0E_DD86, 0x0E_DE02, 0x0E_DE7E];
    pub const FINAL_WATCH: u32 = 0x8C;
    pub const TIMER: u32 = 0x72;
    pub const WATCH_FLAG: u16 = 0x82;
    pub const WATCH_FLAG_BYTE: u32 = 0x82;
    pub const CLOSED_FRAMES: u16 = 0x5A;
    pub const TROOPS_FRAMES: u16 = 0x28;
    pub const OPEN_FRAMES: u16 = 0x50;
    pub const OPEN_ANIMS: u32 = 0x2E_BFAE;
    pub const CLOSE_ANIMS: u32 = 0x2E_BFCA;
    /// `STATE` = 1: trampilla de un fuerte (se abre del todo al caer éste).
    pub const IN_FORT: u8 = 1;
    pub const WAIT_CLOSED: u32 = 0x08_6F64;
    pub const WAIT_CLOSED_RUN: u32 = 0x08_6F70;
    pub const OPENING: u32 = 0x08_6FB4;
    pub const OPENING_RUN: u32 = 0x08_6FDC;
    pub const SPAWN_TROOPS: u32 = 0x08_7004;
    pub const SPAWN_TROOPS_RUN: u32 = 0x08_7024;
    pub const WAIT_OPEN: u32 = 0x08_7068;
    pub const WAIT_OPEN_RUN: u32 = 0x08_7074;
    pub const CLOSING: u32 = 0x08_70B8;
    pub const CLOSING_RUN: u32 = 0x08_70E0;
    pub const FINAL_OPEN: u32 = 0x08_7108;
    pub const FINAL_SPAWN: u32 = 0x08_7130;
    /// Cajas: gráficos por variante, vida, zoom completo, caja de golpe
    /// que parpadea (activa 1 de cada 4 frames; tabla por variante) y vida
    /// del frame anterior (para pasar el daño al fuerte).
    pub const CRATE_GFX: [u16; 5] = [0xE7, 0xE8, 0xE9, 0xEA, 0xEB];
    pub const CRATE_HP: u16 = 0x14;
    pub const SHRINK_Y: u32 = 0x32;
    pub const SHRINK_X: u32 = 0x33;
    pub const FLIP: u32 = 0x3A;
    pub const FULL_SIZE: u8 = 0xFF;
    pub const CRATE_RUN: u32 = 0x08_6D58;
    pub const CRATE_BURST: u32 = 0x08_6DB4;
    pub const PULSE_HITBOXES: u32 = 0x2E_C712;
    pub const PULSE_MASK: u16 = 3;
    pub const PULSE_HITBOX: u32 = 0x84;
    pub const LAST_HP: u32 = 0x80;
    pub const PIECES: [&[Piece]; 5] = [
        &[
            (PILLBOX_RIGHT, 0x28, 0x50, Some(0x60), 0x2E_D738, Some(0x0E_E066)),
            (PILLBOX, 0x68, 0x50, Some(0x60), 0x2E_D74C, Some(0x0E_E07C)),
            (PILLBOX_RIGHT, 0xA8, 0x50, Some(0x60), 0x2E_D760, Some(0x0E_E092)),
            (CRATE[0], 0x30, 0x08, None, 0x2E_D774, None),
            (CRATE[0], 0xA8, 0x08, None, 0x2E_D788, None),
        ],
        &[
            (PILLBOX_RIGHT, 0x28, 0x40, Some(0x70), 0x2E_D79C, Some(0x0E_E0A8)),
            (PILLBOX, 0x68, 0x40, Some(0x70), 0x2E_D7B0, Some(0x0E_E0BE)),
            (PILLBOX_RIGHT, 0xA8, 0x40, Some(0x70), 0x2E_D7C4, Some(0x0E_E0D4)),
            (CRATE[1], 0x28, 0x00, None, 0x2E_D7D8, None),
            (CRATE[1], 0xA0, 0x00, None, 0x2E_D7EC, None),
        ],
        &[
            (PILLBOX_RIGHT_UPPER, 0x28, 0x50, Some(0x60), 0x2E_D800, Some(0x0E_E0EA)),
            (PILLBOX_RIGHT_UPPER, 0x68, 0x50, Some(0x60), 0x2E_D814, Some(0x0E_E100)),
            (PILLBOX_RIGHT_UPPER, 0xA8, 0x50, Some(0x60), 0x2E_D828, Some(0x0E_E116)),
            (CRATE[2], 0x28, 0x08, None, 0x2E_D83C, None),
            (CRATE[2], 0xA0, 0x08, None, 0x2E_D850, None),
        ],
        &[
            (PILLBOX_LOWER, 0x18, 0x40, Some(0x70), 0x2E_D88C, Some(0x0E_E12C)),
            (PILLBOX_RIGHT, 0x48, 0x40, Some(0x70), 0x2E_D8A0, Some(0x0E_E130)),
            (PILLBOX_RIGHT, 0x78, 0x40, Some(0x70), 0x2E_D8B4, Some(0x0E_E146)),
            (PILLBOX_RIGHT, 0xA8, 0x40, Some(0x70), 0x2E_D8C8, Some(0x0E_E15C)),
            (CRATE[3], 0x28, 0x08, None, 0x2E_D8DC, None),
            (CRATE[3], 0xA0, 0x08, None, 0x2E_D8F0, None),
        ],
        &[
            (PILLBOX_RIGHT_UPPER, 0x28, 0x50, Some(0x60), 0x2E_D904, Some(0x0E_E172)),
            (PILLBOX_RIGHT_UPPER, 0x68, 0x50, Some(0x60), 0x2E_D918, Some(0x0E_E188)),
            (PILLBOX_RIGHT_UPPER, 0xA8, 0x50, Some(0x60), 0x2E_D92C, Some(0x0E_E19E)),
            (CRATE[4], 0x28, 0x00, None, 0x2E_D940, None),
            (CRATE[4], 0x98, 0x00, None, 0x2E_D954, None),
        ],
    ];
}

/// Indicador «GO» (`system::go_arrow`).
pub mod go_arrow {
    pub const CELL: u32 = 0x22;
    pub const ARROW_CELL: u16 = 0x7408;
    pub const LAST_PROGRESS: u32 = 0x70;
    pub const QUIET: u32 = 0x30;
    pub const QUIET_FRAMES: u16 = 0x78;
    pub const PROGRESS: u32 = 0x10_6F5C;
    pub const ENEMIES: [u32; 2] = [0x10_6E88, 0x10_6E8A];
    pub const SCENE_LOADED: u32 = 0x10_6ED3;
    /// Parpadeos que quedan y cuántos se hacen.
    pub const BLINKS: u32 = 0x10_E2EF;
    pub const BLINK_COUNT: u8 = 2;
    pub const VARIANT: u32 = 0x10_E2EE;
    pub const ANIMS: [u32; 3] = [0x2F_3574, 0x2F_35FE, 0x2F_3688];
    pub const SOUND: u16 = 0x10D5;
    pub const RUN: u32 = 0x08_C880;
    pub const PLAY: u32 = 0x08_C8FA;
    pub const PLAY_RUN: u32 = 0x08_C90E;
}

/// Piloto automático de los tramos guiados (`system::autopilot`).
pub mod autopilot {
    /// Tramo pedido (0 = ninguno, 1–4) y guion de cada uno (−1 = ninguno).
    pub const LEVEL: u32 = 0x10_E39C;
    pub const SCRIPTS: u32 = 0x2D_F442;
    /// Bytes de un mando sintético: antes, ahora, recién pulsado y otro.
    pub const PAD_BEFORE: u32 = 1;
    pub const PAD_NOW: u32 = 2;
    pub const PAD_PRESSED: u32 = 3;
    pub const PAD_EXTRA: u32 = 4;
    /// Al arrancar un tramo se ignora la entrada real 2 frames.
    pub const HOLDOFF: u8 = 2;
    /// Jugadores invulnerables durante el tramo.
    pub const SHIELD: u32 = 0x45;
    pub const SHIELD_ON: u8 = 2;
    pub const RUN: u32 = 0x07_92F0;
}

/// Soldados rasos (`system::grunt`).
pub mod grunt {
    /// Posición, fracción, velocidad y aceleración (X, Y).
    pub const POS: [u32; 2] = [0x22, 0x24];
    pub const SUB: [u32; 2] = [0x26, 0x27];
    pub const VEL: [u32; 2] = [0x28, 0x2A];
    pub const ACC: [u32; 2] = [0x2C, 0x2E];
    /// Parámetros de aparición (`+$98` orientación, `+$99` velocidad o
    /// retardo, `+$9A` tipo), orientación de dibujo y atributos.
    pub const PARAM_A: u32 = 0x98;
    pub const PARAM_B: u32 = 0x99;
    pub const KIND: u32 = 0x9A;
    pub const FACING: u32 = 0x3A;
    pub const ATTR: u32 = 0x38;
    pub const ATTR_SOLDIER: u16 = 0x8018;
    pub const ATTR_HOPPER: u16 = 0xD000;
    /// Contador y posición del scroll a la que actúa.
    pub const TIMER: u32 = 0x70;
    pub const TRIGGER: u32 = 0x72;
    pub const SCROLL: u32 = 0x10_6F50;
    /// Gráficos según el tipo.
    pub const KIND_GFX_BASE: u16 = 0xFA;
    pub const GFX_A: [u16; 3] = [0x14A, 0xF6, 0x13D];
    pub const GFX_B: [u16; 2] = [0x149, 0xF7];
    /// Plantilla 162 (salta arriba y abajo): gráfico, animaciones,
    /// aceleraciones, duración (8 + 0..7 frames), límites y estados.
    pub const HOPPER_GFX: u16 = 0xF8;
    pub const HOP_DOWN_ANIM: u32 = 0x2F_3C84;
    pub const HOP_UP_ANIM: u32 = 0x2F_3CB4;
    pub const HOP_DOWN_ACC: u16 = 0x0E;
    pub const HOP_UP_ACC: u16 = 0xFFF2;
    pub const HOP_FRAMES: u16 = 8;
    pub const HOP_RANDOM: u16 = 7;
    pub const HOP_BOTTOM: i16 = 0x1D0;
    pub const HOP_TOP: i16 = 0x160;
    pub const HOP_DOWN: u32 = 0x08_DB7A;
    pub const HOP_DOWN_RUN: u32 = 0x08_DBA6;
    pub const HOP_UP: u32 = 0x08_DBE2;
    pub const HOP_UP_RUN: u32 = 0x08_DC0E;
}

/// Bichos sencillos de las escenas (`system::critters`): el que sube y
/// baja (plantilla 182), el que salta (183) y el que corre (184).
pub mod critters {
    pub const ATTR: u32 = 0x38;
    pub const ATTR_BOBBER: u16 = 0xC000;
    pub const VEL_Y: u32 = 0x2A;
    pub const ACC_Y: u32 = 0x2E;
    pub const TIMER: u32 = 0x70;
    /// Sube y baja: gráfico, animación, aceleración, duración (6 + 0..7),
    /// límites y estados.
    pub const BOBBER_GFX: u16 = 0x12A;
    pub const BOBBER_ANIM: u32 = 0x2F_4036;
    pub const BOBBER_ACC: u16 = 4;
    pub const BOBBER_FRAMES: u16 = 6;
    pub const BOBBER_BOTTOM: i16 = 0x1D0;
    pub const BOBBER_TOP: i16 = 0x160;
    pub const BOBBER_DOWN: u32 = 0x08_E4FE;
    pub const BOBBER_DOWN_RUN: u32 = 0x08_E52A;
    pub const BOBBER_UP: u32 = 0x08_E566;
    pub const BOBBER_UP_RUN: u32 = 0x08_E592;
    /// Salta: gráfico, animaciones (espera, salto, caída), velocidad y
    /// aceleración de la espera, espera (12 + 0..15), salto (−$40 − 0..63,
    /// aceleración 4, 11 frames) y estados.
    pub const LEAPER_GFX: u16 = 0x124;
    pub const LEAPER_ANIM: [u32; 3] = [0x2F_4072, 0x2F_407E, 0x2F_408A];
    pub const LEAPER_WAIT_VEL: u16 = 0xFD00;
    pub const LEAPER_WAIT_ACC: u16 = 0x10;
    pub const LEAPER_WAIT: (u16, u16) = (0xC, 0xF);
    pub const LEAPER_RISE: u16 = 0x40;
    pub const LEAPER_JUMP: (u16, u16, u16) = (0x40, 0x3F, 4);
    pub const LEAPER_JUMP_FRAMES: u16 = 0xB;
    pub const LEAPER_WAIT_RUN: u32 = 0x08_E622;
    pub const LEAPER_HOP: u32 = 0x08_E65C;
    pub const LEAPER_HOP_RUN: u32 = 0x08_E68E;
    pub const LEAPER_FALL: u32 = 0x08_E6B8;
    pub const LEAPER_FALL_RUN: u32 = 0x08_E6CA;
    /// Corre: gráfico, animación y estado.
    pub const RUNNER_GFX: u16 = 0x128;
    pub const RUNNER_ANIM: u32 = 0x2F_4096;
    pub const RUNNER_RUN: u32 = 0x08_E716;
    /// Banda de X de la pantalla en que sigue vivo (una vez visto).
    pub const SEEN: u8 = 1 << 7;
    pub const FLAGS2: u32 = 0x13;
    pub const X: u32 = 0x22;
    pub const Y: u32 = 0x24;
    pub const VIEW_X: (i16, i16) = (0, 0x140);
    pub const VIEW_Y: (i16, i16) = (0x100, 0x1F0);
}

/// Objetos del suelo y test de fuera de la vista (`system::items`).
/// Soldado de guardia que embiste (Charger) y escaramuzador
/// (`asm/enemies/soldiers/charger_states_04290c.s`).
pub mod charger {
    pub const X: u32 = 0x22;
    pub const VEL_X: u32 = 0x28;
    pub const ATTR: u32 = 0x38;
    pub const FACING: u32 = 0x3A;
    pub const ON_HIT: u32 = 0x4C;
    pub const ANIM_SLOT: u32 = 0x5C;
    /// Columna de casa y radio (en el mundo), temporizador del ataque,
    /// marca de apoyado, enfado, continuación diferida y objetivo.
    pub const HOME: u32 = 0x70;
    pub const RADIUS: u32 = 0x72;
    pub const TIMER: u32 = 0x74;
    pub const GROUNDED: u32 = 0x75;
    pub const ANGRY: u32 = 0x76;
    pub const RESUME: u32 = 0x78;
    pub const TARGET: u32 = 0x7C;
    /// Parámetros de la plantilla: radio (columnas de 8), golpes que le
    /// quedan, base del temporizador, recarga, tipo de golpe y marcha.
    pub const P_RADIUS: u32 = 0x98;
    pub const STRIKES: u32 = 0x99;
    pub const P_TIMER: u32 = 0x9A;
    pub const P_RELOAD: u32 = 0x9B;
    pub const STRIKE_KIND: u32 = 0x9C;
    pub const GAIT: u32 = 0x9D;
    /// El escaramuzador guarda la marca de apoyado aquí.
    pub const SK_GROUNDED: u32 = 0x70;
    pub const GFX: u16 = 0xE;
    pub const PRIORITY: u16 = 0x8000;
    pub const ATTR_KEEP: u16 = 0xFFE3;
    pub const ATTR_SET: u16 = 0x18;
    pub const SHADOW: u32 = 0x07_76E2;
    pub const TIMER_TABLE: u32 = 0x2B_74EC;
    pub const RELOAD_TABLE: u32 = 0x2B_746A;
    pub const SPEED_TABLE: u32 = 0x2B_75D0;
    pub const SK_SPEED_TABLE: u32 = 0x2B_76B4;
    pub const SCREEN_MID: i16 = 0xA0;
    pub const WALK_SPEED: u16 = 0x100;
    pub const WINDUP_KIND: u8 = 4;
    pub const HURT_HANDLER: u32 = 0x05_7494;
    pub const MARGINS: u32 = 0x28_838C;
    /// Guiones de sprite.
    pub const S_WALK_A: u32 = 0x04_27FA;
    pub const S_WALK_B: u32 = 0x04_287E;
    pub const S_WALK_FAST: u32 = 0x29_B744;
    pub const S_IDLE: u32 = 0x29_B816;
    pub const S_IDLE_ANGRY: u32 = 0x2B_5C10;
    pub const S_HURT: u32 = 0x2B_70D2;
    pub const S_STRIKES: [u32; 5] = [0x29_BA70, 0x29_B9CE, 0x29_BBEC, 0x29_BC64, 0x29_BB4C];
    pub const STRIKE0_SLOT: u8 = 5;
    pub const S_WINDUP_A: u32 = 0x29_BD84;
    pub const S_WINDUP_B: u32 = 0x29_B956;
    /// Estados.
    pub const IDLE: u32 = 0x04_2ACC;
    pub const IDLE_LOOP: u32 = 0x04_2B00;
    pub const WALK_A: u32 = 0x04_2A44;
    pub const WALK_B: u32 = 0x04_2A56;
    pub const WALK_FAST: u32 = 0x04_2A6E;
    pub const WALK_LOOP: u32 = 0x04_2A9C;
    pub const HURT: u32 = 0x04_2B5C;
    pub const HURT_LOOP: u32 = 0x04_2B88;
    pub const ATTACK: u32 = 0x04_2BA6;
    pub const ATTACK_ANIM: u32 = 0x04_2BE6;
    pub const ATTACK_LOOP: u32 = 0x04_2C6C;
    pub const ATTACK_EXIT: u32 = 0x04_2CB0;
    pub const WINDUP_A: u32 = 0x04_2CCA;
    pub const WINDUP_A_LOOP: u32 = 0x04_2CE0;
    pub const WINDUP_B: u32 = 0x04_2D02;
    pub const WINDUP_B_LOOP: u32 = 0x04_2D18;
    /// Salida al terminar los golpes (el soldado normal).
    pub const LEAVE: u32 = 0x05_9042;
    pub const SK_RUN: u32 = 0x04_2D5C;
    pub const SK_MAIN: u32 = 0x04_2D90;
    pub const SK_HURT: u32 = 0x04_2DFA;
    pub const SK_HURT_LOOP: u32 = 0x04_2E26;
}

/// Decorados de la escena del fuerte (`asm/props/fortress_04e580.s`).
pub mod props {
    pub const ATTR: u32 = 0x38;
    /// Carteles A y B: gráficos, guion de sprite, celdas de colisión que
    /// estampan y estado de cada frame.
    pub const SIGN_GFX: [u16; 2] = [0x186, 0x1B6];
    pub const S_SIGN: [u32; 2] = [0x29_49AE, 0x29_49BE];
    pub const SIGN_CELLS: [u32; 2] = [0x29_5F6E, 0x29_5F94];
    pub const SIGN_LOOP: [u32; 2] = [0x04_F742, 0x04_F7A0];
    /// Márgenes de la vista con los que se dan de baja.
    pub const MARGINS: u32 = 0x29_625E;
    /// Decorado fijo: registro de aparición, atributos por defecto, escala
    /// (`$FF` = sin encoger), espejo y el estado de cada frame.
    pub const SPAWN_RECORD: u32 = 0x3C;
    pub const FIXED_ATTR: u16 = 0x8000;
    pub const SHRINK_Y: u32 = 0x32;
    pub const SHRINK_X: u32 = 0x33;
    pub const FLIP: u32 = 0x3A;
    pub const FIXED_LOOP: u32 = 0x04_D72E;
    /// Sensor: margen (`+$70`), caja de vigilancia, bit 2 de `+$8C` del
    /// jugador que lo dispara, guion al saltar y sus estados.
    pub const MARGIN: u32 = 0x70;
    pub const SENSOR_MARGIN: u16 = 0x90;
    pub const SENSOR_BOX: u32 = 0x29_5FB0;
    pub const PLAYER_STATUS: u32 = 0x8C;
    pub const SENSOR_TRIGGER: u8 = 1 << 2;
    pub const S_SENSOR_TRIP: u32 = 0x29_48A6;
    pub const SENSOR_WATCH: u32 = 0x04_F47A;
    pub const SENSOR_LOOP: u32 = 0x04_F480;
    pub const SENSOR_TRIP: u32 = 0x04_F4B6;
    pub const SENSOR_TRIP_LOOP: u32 = 0x04_F4C8;
    /// Entrada alta fija que cambian los guiones (`FixTile_*`): índice,
    /// gráficos ($C2 / $C1), duración (sin fin) y modo.
    pub const FIX_TILE_ENTRY: u16 = 0x11;
    pub const FIX_TILE_GFX: [u16; 2] = [0xC2, 0xC1];
    pub const FIX_TILE_LEN: u16 = 0xFFFF;
    pub const FIX_TILE_MODE: u8 = 1;
}

/// Contacto entre cajas de entidades (`$028998` / `$028A96` / `$028B14`).
pub mod contact {
    pub const X: u32 = 0x22;
    pub const Y: u32 = 0x24;
    /// Bit 0 = espejo en X, bit 1 = espejo en Y.
    pub const FLAGS_3A: u32 = 0x3A;
    pub const MIRROR_X: u8 = 1 << 0;
    pub const MIRROR_Y: u8 = 1 << 1;
    /// Descriptor de caja: {activa ($80).b, tipo.b, X de referencia.w,
    /// ...} y tras la cabecera {X mín., X máx., Y mín., Y máx.}.
    pub const HITBOX: u32 = 0x60;
    pub const NIL: u32 = 0xFFFF_FFFF;
    pub const ACTIVE: u8 = 0x80;
    pub const KIND_STRONG: u8 = 2;
    pub const HEADER: u32 = 6;
    pub const HEADER_X: u32 = 2;
    pub const BOX_X: u32 = 0;
    pub const BOX_Y: u32 = 4;
    /// Banderas de contacto: bits 0/1 = lado, 2 = contacto fuerte,
    /// 3 = inmune.
    pub const CONTACT: u32 = 0x69;
    pub const STRONG: u8 = 1 << 2;
    pub const IMMUNE: u8 = 1 << 3;
    /// `+$6B` bit 6: la víctima no marca al que la toca.
    pub const FLAGS_6B: u32 = 0x6B;
    pub const NO_MARK_ATTACKER: u8 = 1 << 6;
    /// Huecos de los jugadores 1 y 2 y del Slug.
    pub const SLOTS: [u32; 3] = [0x10_0440, 0x10_04E0, 0x10_0580];
}

pub mod items {
    pub const X: u32 = 0x22;
    pub const Y: u32 = 0x24;
    /// `FLAGS2` bit 7: la entidad ya ha estado en pantalla.
    pub const SEEN: u8 = 1 << 7;
    /// Pantalla (sin márgenes) y zona lejana en la que se da por vista.
    pub const VIEW_X: (i16, i16) = (0, 0x140);
    pub const VIEW_Y: (i16, i16) = (0x100, 0x1F0);
    pub const FAR_X: (i16, i16) = (-0x80, 0x200);
    pub const FAR_Y: (i16, i16) = (0, 0x280);
    pub const DEFAULT_MARGINS: u32 = 0x05_DD4C;
    /// Márgenes de los objetos del suelo, estado «en el suelo» (`+$90` = 1)
    /// y contador de los que lo están, y `+$9F` = darlo de baja aunque no
    /// se haya visto.
    pub const MARGINS: u32 = 0x09_A5F4;
    pub const STATE: u32 = 0x90;
    pub const ON_GROUND: u8 = 1;
    pub const ON_GROUND_COUNT: u32 = 0x10_E48B;
    pub const ALWAYS_CULL: u32 = 0x9F;
    /// Caja, atributos y bandera «se le puede dar» de `FLAGS_13`.
    pub const HITBOX: u32 = 0x60;
    pub const ATTR: u32 = 0x38;
    pub const FLAGS_13: u32 = 0x13;
    pub const HITTABLE: u8 = 1 << 3;
    pub const ATTR_KEEP: u16 = 0xFFE3;
    /// Prisionero atado (Pow, plantilla 293): gráficos, caja, prioridad,
    /// guiones de sprite, sonido al soltarse y estados.
    pub const POW_GFX: u16 = 0x1AB;
    pub const POW_HITBOX: u32 = 0x09_A672;
    pub const POW_ATTR: u16 = 0x8000;
    pub const S_POW: u32 = 0x2F_772A;
    pub const S_POW_FREED: u32 = 0x2F_773C;
    pub const SND_POW_FREED: u16 = 0x102E;
    pub const POW_LOOP: u32 = 0x09_B400;
    pub const POW_FREED: u32 = 0x09_B43E;
    pub const POW_FREED_LOOP: u32 = 0x09_B45A;
    pub const POW_GONE: u32 = 0x09_B474;
    /// Muestra de rastro que deja el objeto (plataforma para los jugadores):
    /// desplazamiento y ancho.
    pub const TRAIL_DX: u16 = 0x10;
    pub const TRAIL_DY: u16 = 0x10;
    pub const TRAIL_W: u16 = 0x20;
    /// Objetos que suelta otra entidad: tabla de manejadores por tipo,
    /// tipos válidos (el resto da el `SPAWN_DEFAULT`), tarea madre, desplazamiento
    /// delante y lo que baja al reaparecer.
    pub const FACING: u32 = 0x3A;
    pub const SPAWN_HANDLERS: u32 = 0x09_A6DE;
    pub const SPAWN_MASK: u8 = 0x3F;
    pub const SPAWN_KINDS: u8 = 0x1E;
    pub const SPAWN_DEFAULT: u8 = 0xA;
    pub const SPAWN_PARENT: u32 = 0x10_0800;
    pub const SPAWN_DX: u16 = 0x10;
    pub const P_KIND: u32 = 0x98;
    pub const P_ARG: u32 = 0x99;
    pub const RESPAWN_DY: u16 = 4;
    /// Cajas de munición (plantillas 284–287): sprite, frames sin poder
    /// recogerse, guiones, bucles, el arma que da cada una y la caja de
    /// recogida.
    pub const AMMO_GFX: u16 = 0x17D;
    /// `+$45`: frames que faltan para poder recogerlo.
    pub const NO_PICKUP: u32 = 0x45;
    pub const AMMO_NO_PICKUP: u8 = 8;
    pub const AMMO_SCRIPTS: [u32; 4] = [0x2F_67DC, 0x2F_67E8, 0x2F_67F4, 0x2F_6800];
    pub const AMMO_LOOPS: [u32; 4] = [0x09_A950, 0x09_A9AE, 0x09_AA0C, 0x09_AA6A];
    pub const AMMO_WEAPONS: [u8; 4] = [4, 3, 1, 2];
    pub const PICKBOX_A: u32 = 0x09_A6CE;
    /// Jugador que recoge: entidades, estado (2 = con aviso al recoger),
    /// sonido, parpadeo del arma nueva y `+$8C` bit 4 (arma nueva).
    pub const PLAYERS: [u32; 2] = [0x10_0440, 0x10_04E0];
    pub const PLAYER_STATE: u32 = 0x70;
    pub const PLAYER_STATE_ALERT: u8 = 2;
    pub const SND_WEAPON: u16 = 0x10FD;
    pub const PLAYER_FLASH: u32 = 0x87;
    pub const WEAPON_FLASH: u8 = 5;
    pub const PLAYER_STATUS: u32 = 0x8C;
    pub const NEW_WEAPON: u8 = 1 << 4;
    /// Recogido: estados de entrada (A–D), sprite del A (los demás siguen),
    /// frames y su contador, prioridad, bucle y la subida (sprite, guion y
    /// bucle).
    pub const TAKEN: [u32; 4] = [0x09_AE26, 0x09_AE2E, 0x09_AE36, 0x09_AE3E];
    pub const TAKEN_GFX: u16 = 0x1A6;
    pub const TAKEN_TIMER: u32 = 0x70;
    pub const TAKEN_FRAMES: u16 = 2;
    pub const TAKEN_ATTR: u16 = 0xC000;
    pub const TAKEN_LOOP: u32 = 0x09_AE6A;
    pub const TAKEN_RISE: u32 = 0x09_AE8E;
    pub const RISE_GFX: u16 = 0x180;
    pub const S_RISE: u32 = 0x2F_674C;
    pub const RISE_LOOP: u32 = 0x09_AEAA;
}

/// Recogida de objetos (`system::pickup`).
pub mod pickup {
    pub const X: u32 = 0x22;
    pub const Y: u32 = 0x24;
    /// `FLAGS2` bits 0 y 3: la entidad no toca objetos.
    pub const UNTOUCHABLE: u8 = (1 << 3) | (1 << 0);
    /// Espejos de la entidad (`+$3A`: bit 0 en X, bit 1 en Y).
    pub const FLIP: u32 = 0x3A;
    pub const FLIP_X: u8 = 1 << 0;
    pub const FLIP_Y: u8 = 1 << 1;
    /// Se pueden recoger objetos (escena en marcha).
    pub const ENABLED: u32 = 0x10_6ED2;
    pub const PLAYER_ONE: u32 = 0x10_0440;
    /// Objeto: aún no se puede recoger, jugador que lo coge, tipo de premio
    /// y valor.
    pub const NO_PICKUP: u32 = 0x45;
    pub const PLAYER: u32 = 0x68;
    pub const KIND: u32 = 0x9C;
    pub const VALUE: u32 = 0x98;
    /// Cajas de contacto {dx, ancho, dy, alto}: la general y la de la comida.
    pub const ITEM_BOX: u32 = 0x09_A6CE;
    pub const FOOD_BOX: u32 = 0x09_A6D6;
    pub const FOOD_SOUND: u16 = 0x1121;
    /// Marcador de puntos que sale al recoger.
    pub const SCORE_POPUP: u32 = 0x09_B51E;
    /// Premios: valor máximo, el del tipo grande, racha y su duración.
    pub const MAX_VALUE: i8 = 0x0E;
    pub const KIND_BIG: u8 = 2;
    pub const BIG_VALUE: i8 = 0x0F;
    pub const STREAK: u32 = 0x10_E489;
    pub const STREAK_TIMER: u32 = 0x10_E488;
    pub const STREAK_FRAMES: u8 = 0x2D;
    pub const STREAK_RUN: u32 = 0x09_B83E;
}

/// Eventos de la misión (`system::events`).
pub mod events {
    pub const SCENE: u32 = 0x10_6ECE;
    /// Lista de eventos de cada escena; la de la última escena de la
    /// recreativa se cambia por otra si hay varias ranuras.
    pub const STREAMS: u32 = 0x04_4266;
    pub const ARCADE_STREAM: (u32, u32) = (0x0F_1CD4, 0x0F_260E);
    pub const BIOS_SLOTS: u32 = 0x10_FD83;
    /// Tareas de apoyo que se crean al empezar (bajo `HELPER_PARENT`) y el
    /// byte que se borra antes.
    pub const HELPERS: [u32; 2] = [0x08_C864, 0x07_9298];
    pub const HELPER_PARENT: u32 = 0x10_08A0;
    pub const HELPER_FLAG: u32 = 0x10_E2EE;
    pub const IDLE: u32 = 0x400;
    pub const RUN: u32 = 0x04_4368;
    pub const WATCH: u32 = 0x04_42C4;
    pub const PERIODIC: u32 = 0x04_482A;
    /// Campos de la tarea que ejecuta una lista.
    pub const CURSOR: u32 = 0x70;
    pub const PAUSED: u32 = 0x74;
    pub const WAIT: u32 = 0x76;
    pub const WATCH_FLAG: u32 = 0x78;
    pub const THRESHOLD: u32 = 0x7A;
    /// Campos de la tarea que repite una orden.
    pub const P_TIMES: u32 = 0x7C;
    pub const P_EVERY: u32 = 0x7E;
    pub const P_TIMER: u32 = 0x80;
    pub const P_UNTIL: u32 = 0x82;
    pub const P_MAX_ALIVE: u32 = 0x84;
    /// Órdenes: código en el byte 0, parámetro (umbral o espera) en `+2`.
    pub const OP_PARAM: u32 = 2;
    pub const NO_THRESHOLD: u16 = 0xFFFF;
    pub const OP_SPAWN: u8 = 0x00;
    pub const OP_PERIODIC: u8 = 0x01;
    pub const OP_BLOCK: u8 = 0x02;
    pub const OP_RANDOM: u8 = 0x03;
    pub const OP_PAST_Y: u8 = 0x04;
    pub const OP_PAUSE_FLAG: u8 = 0x05;
    pub const OP_PAUSE: u8 = 0x06;
    pub const OP_WAIT_ENEMIES: u8 = 0x07;
    pub const OP_RESUME_FLAG: u8 = 0x08;
    pub const OP_NOP: u8 = 0x09;
    pub const OP_PAST_X: u8 = 0x0A;
    pub const OP_BOTH_PAST_Y: u8 = 0x0B;
    pub const OP_END: u8 = 0x0C;
    pub const OP_BLOCK_END: u8 = 0x0D;
    pub const OP_RANDOM_END: u8 = 0x0E;
    pub const SPAWN_LEN: u32 = 0x12;
    pub const PERIODIC_LEN: u32 = 0x0A;
    /// Scroll recorrido, cámara (X, Y) y posición de la capa móvil.
    pub const CAMERA_PROGRESS: u32 = 0x10_6F5C;
    pub const CAMERA_X: u32 = 0x10_6F50;
    pub const CAMERA_Y: u32 = 0x10_6F54;
    pub const LAYER_POS: [u32; 2] = [0x10_6F6C, 0x10_6F6E];
    pub const SCREEN_BOTTOM: u16 = 0x200;
    pub const PAUSE_FLAG: u32 = 0x10_8179;
    pub const ENEMY_COUNT: u32 = 0x10_6E88;
    pub const SPAWNED: u32 = 0x10_6E8A;
    pub const START_STATE: [u32; 2] = [0x10_FDB6, 0x10_FDB7];
    pub const PLAYERS: [u32; 2] = [0x10_0440, 0x10_04E0];
    pub const X: u32 = 0x22;
    pub const Y: u32 = 0x24;
    /// Apariciones: tabla de rutinas iniciales, madre, flags de `+1`.
    pub const TEMPLATES: u32 = 0x0E_8000;
    pub const SPAWN_PARENT: u32 = 0x10_0800;
    pub const ONLY_MASK: u8 = 0x03;
    pub const ONLY_WITH_SLUG: u8 = 0x03;
    pub const SLUG_ACTIVE: u32 = 0x10_0609;
    pub const ACTIVE_PLAYERS: u32 = 0x10_6ED1;
    pub const LAYER_MASK: u8 = 0x0C;
    pub const KEEP_LOOSE: u8 = 1 << 4;
    pub const SCREEN_Y: u8 = 1 << 6;
    pub const SCREEN_X: u8 = 1 << 7;
    pub const COUNTED: u8 = 1 << 1;
    pub const ON_LAYER: u8 = 1 << 6;
    pub const LAYER: u32 = 0x11;
    pub const PARAMS: u32 = 0x98;
    pub const PARAMS_LEN: u32 = 8;
    pub const SCENE_LOADED: u32 = 0x10_6ED3;
}

/// Tarea del HUD de cada jugador (`system::player_hud`).
pub mod player_hud {
    /// Mensajes grandes: banco de letras, fin de texto, posición y paleta
    /// por jugador, y la zona que se borra.
    pub const BIG_FONT: u16 = 0x400;
    pub const BIG_TEXT_END: u8 = 0xFF;
    pub const MSG_POS: [u16; 2] = [0x7063, 0x7343];
    pub const MSG_PALETTE: u8 = 3;
    pub const TEXT_INSERT_COIN: u32 = 0x27_85C4;
    pub const TEXT_PUSH_START: u32 = 0x27_85D0;
    pub const TEXT_CONTINUE: u32 = 0x27_85DC;
    pub const TEXT_GAME_OVER: u32 = 0x27_85E8;
    pub const TEXT_PLEASE_WAIT: u32 = 0x27_85F4;
    pub const MSG_CLEAR_POS: [u16; 2] = [0x7043, 0x72C3];
    pub const MSG_BLANK: u16 = 0x300;
    pub const MSG_CLEAR_SIZE: (u16, u16) = (0x10, 3);
    /// Rótulos fijos (por jugador): nombre, marco de vida y bloques.
    pub const LABEL_POS: [u16; 2] = [0x7065, 0x73E5];
    pub const LABEL_TEXT: [u32; 2] = [0x27_85B8, 0x27_85BE];
    pub const LABEL_ATTR: u16 = 0x5300;
    /// Vidas en reserva: posición por jugador, atributos y caracteres.
    pub const LIVES_POS: [u16; 2] = [0x70E5, 0x7465];
    pub const LIVES_ATTR: u16 = 0x5300;
    pub const DIGIT_0: u16 = 0x30;
    pub const DIGIT_BLANK: u16 = 0x20;
    pub const LIFE_FRAME_POS: [u16; 2] = [0x7044, 0x73C4];
    pub const LIFE_FRAME_LEFT: u16 = 0x7AA5;
    pub const LIFE_FRAME_MID: u16 = 0x7AA6;
    pub const LIFE_FRAME_RIGHT: u16 = 0x7AAF;
    pub const LIFE_FRAME_LEN: u16 = 6;
    /// Barra de vida del Slug: estado en `BAR_STATE` (0 sin Slug, 1
    /// llenándose, 2 al día), posición 16.16 en `BAR`, velocidad del
    /// muelle en `BAR_VEL`; vida del Slug y umbral de parpadeo; atributos
    /// normal y de parpadeo; piezas.
    pub const BAR_POS: u32 = BAR;
    pub const SLUG_HP: u32 = 0x10_05E6;
    pub const BAR_LOW: u16 = 0x11;
    pub const BAR_FILL_STEP: i16 = 2;
    pub const BAR_ATTR: u16 = 0x7A00;
    pub const BAR_ATTR_BLINK: u16 = 0xDA00;
    pub const BAR_PIECE: u16 = 8;
    pub const BAR_LEFT: u16 = 0xA5;
    pub const BAR_FULL: u16 = 0xAE;
    pub const BAR_EMPTY: u16 = 0xA6;
    pub const BAR_RIGHT: u16 = 0xAF;
    pub const LABEL_BLOCKS: [[(u16, u16, u16, u16); 2]; 2] =
        [[(0x7143, 0x7AB0, 8, 3), (0x71C3, 0x7AE4, 4, 1)], [(0x72C3, 0x7AB0, 8, 3), (0x7343, 0x7AE4, 4, 1)]];
    /// Marcador: puntuación (7 cifras desde el byte 1), sin dibujar en los
    /// modos `NO_SCORE_MODES` (de `script::KEY_LATCH[0]`).
    pub const SCORE_DIGITS: [u32; 2] = [0x10_6E95, 0x10_6E9D];
    pub const SCORE_POS: [u16; 2] = [0x7063, 0x73C3];
    pub const SCORE_ATTR: u16 = 0x2300;
    pub const SCORE_LEN: u32 = 7;
    pub const NO_SCORE_MODES: [u8; 3] = [0xB, 0xC, 0xD];
    /// Munición y bombas: parpadean `COUNTER_BLINK` frames al cambiar.
    pub const COUNTER_BLINK: u8 = 4;
    pub const COUNTER_ATTR: [u16; 2] = [0xE300, 0xF300];
    pub const AMMO_POS: [u16; 2] = [0x71A4, 0x7324];
    /// Pistola: símbolo de infinito (tres caracteres, de derecha a izquierda).
    pub const AMMO_INFINITE: u16 = 0xE3F2;
    pub const BOMBS_POS: [u16; 2] = [0x7204, 0x7384];
    /// Marco de bombas (soldado) o de proyectiles (Slug).
    pub const BOMBS_FRAME_POS: [u16; 2] = [0x71C3, 0x7343];
    pub const BOMBS_FRAME: [u16; 2] = [0x7AE4, 0x7AE0];
    /// Campos del soldado: munición, ¿arma con munición?, bombas.
    pub const SOLDIER_AMMO: u32 = 0x82;
    pub const SOLDIER_HAS_AMMO: u32 = 0x71;
    pub const SOLDIER_BOMBS: u32 = 0x80;
    /// Proyectiles del Slug.
    pub const SLUG_SHELLS: u32 = 0x10_0580 + 0x90;
    /// Campos de la tarea del HUD.
    pub const BUSY: u32 = 0x20;
    pub const WAITING: u32 = 0x21;
    pub const PLAYER: u32 = 0x70;
    pub const SLOT: u32 = 0x72;
    pub const GAUGE_CACHE: u32 = 0x76;
    pub const LIVES: u32 = 0x77;
    pub const ENTITY: u32 = 0x78;
    pub const RESUME: u32 = 0x7C;
    pub const SAVED_KEY: u32 = 0x82;
    pub const BLINK: [u32; 2] = [0x84, 0x85];
    pub const BAR: u32 = 0x86;
    pub const BAR_VEL: u32 = 0x8A;
    pub const AMMO: u32 = 0x8E;
    pub const BOMBS: u32 = 0x90;
    pub const BAR_STATE: u32 = 0x92;
    pub const IN_SLUG: u32 = 0x93;
    /// Tarea del soldado: activa (`+$66`), flags de `+$13` (bit 0 = muerto).
    pub const ENTITY_ACTIVE: u32 = 0x66;
    pub const DEAD: u8 = 1 << 0;
    pub const FLAG2_BIT3: u8 = 1 << 3;
    pub const FREEZE_TIMER: u8 = 0x3C;
    /// Tarea del guion (su `TIMER_B` distinto de 0 = no atiende) y pausa.
    pub const SCRIPT_TASK: u32 = 0x10_01C0;
    pub const SCRIPTED_PAUSE: u32 = 0x10_E39C;
    /// 0xFF = que la escena espere este frame.
    pub const HOLD_SCENE: u32 = 0x10_6ED6;
    /// Estado de inicio de cada jugador (BIOS): 0 nada, 1 juega/entra,
    /// 2 continuando, 3 acabado.
    pub const START_STATE: u32 = 0x10_FDB6;
    pub const KEYS: u32 = 0x10_6ECC;
    pub const SCENE: u32 = 0x10_6ECE;
    pub const MISSION: u32 = 0x10_6ED0;
    pub const LAST_MISSION: u8 = 6;
    pub const SCENE_BUSY: u32 = 0x10_6ED2;
    pub const SCENE_LOADED: u32 = 0x10_6ED3;
    pub const RANK_TIMER: u32 = 0x10_6E92;
    pub const CONTROL_LAYOUT: u32 = 0x10_6F2A;
    pub const TWO_PLAYER_LAYOUT: u8 = 2;
    pub const ONE_ALIVE_DELAY: u8 = 0x1E;
    pub const BIOS_SLOTS: u32 = 0x10_FD83;
    pub const BIOS_CONTINUE: u32 = 0x10_FD8A;
    pub const DEMO_SELECT: u32 = 0x10_007B;
    pub const FREE_HANDLER: u32 = 0x400;
    /// Tablas de la ROM: estados de la demo, punteros a créditos, tareas
    /// del soldado (tarea, rutina), filas de descriptores.
    pub const DEMO_STATES: u32 = 0x02_5770;
    pub const CREDITS: u32 = 0x02_674A;
    pub const SOLDIERS: u32 = 0x02_5AC8;
    pub const ROWS: [u32; 2] = [0x02_54A8, 0x02_5588];
    pub const IDLE_ROW: [u32; 2] = [0x02_5488, 0x02_5498];
    /// Tablas de punteros por jugador (P1, P2).
    pub const SCORES: u32 = 0x02_594A;
    pub const BOMB_GAUGES: u32 = 0x02_5942;
    pub const GAME_OVER_TEXT: u32 = 0x02_5922;
    pub const CONTINUE_TASKS: u32 = 0x02_5B2C;
    pub const WATCH_TASKS: u32 = 0x02_5CC6;
    pub const MSG_CLEAR: u32 = 0x02_5668;
    pub const MSG_INSERT_COIN: u32 = 0x02_5670;
    pub const MSG_PUSH_START: u32 = 0x02_5678;
    pub const MSG_PLEASE_WAIT: u32 = 0x02_5680;
    pub const MSG_GAME_OVER: u32 = 0x02_5690;
    pub const DRAW_LIFE: u32 = 0x02_5698;
    pub const DRAW_BOMBS: u32 = 0x02_56A0;
    pub const DRAW_SCORE: u32 = 0x02_56A8;
    /// Dibujos sin portar del alta (P1, P2).
    pub const DRAW_LABELS: [u32; 2] = [0x02_5F02, 0x02_5F4A];
    pub const DRAW_CONTINUES: [u32; 2] = [0x02_60E2, 0x02_60EA];
    pub const DRAW_LIFE_BAR: [u32; 2] = [0x02_614E, 0x02_6156];
    pub const COUNT_CONTINUE: u32 = 0x05_1A86;
    pub const CONTINUE_MARK: u32 = 0x09_B872;
    pub const MARK_PLAYER: u32 = 0x98;
    /// Indicador de bombas.
    pub const BOMB_GAUGE: [u32; 2] = [0x10_6F4C, 0x10_6F4E];
    pub const GAUGE_AT: [u16; 2] = [0x707C, 0x72FC];
    pub const GAUGE_CELLS: u8 = 14;
    pub const GAUGE_FULL: u16 = 0x238A;
    pub const GAUGE_EMPTY: u16 = 0x2320;
    pub const COLUMN: u16 = 0x20;
    pub const BLINK_MASK: u8 = 0x1F;
    pub const BLINK_OFF: u8 = 0x18;
    pub const DONE_DELAY: u8 = 0x1E;
    /// Estados (rutinas de la tarea).
    pub const INSERT_COIN: u32 = 0x02_57EC;
    pub const INSERT_COIN_RUN: u32 = 0x02_57FA;
    pub const PUSH_START: u32 = 0x02_5882;
    pub const PUSH_START_RUN: u32 = 0x02_5890;
    pub const WAIT_SPAWN: u32 = 0x02_5952;
    pub const WAIT_SPAWN_RUN: u32 = 0x02_596A;
    pub const BIND: [u32; 3] = [0x02_59A0, 0x02_59AC, 0x02_59B8];
    pub const PLAYING: u32 = 0x02_59FC;
    pub const IN_PLAY: u32 = 0x02_5A0E;
    pub const RESPAWN: u32 = 0x02_5AD8;
    pub const GAME_OVER: u32 = 0x02_5B34;
    pub const GAME_OVER_WAIT: u32 = 0x02_5BCE;
    pub const COUNTDOWN: u32 = 0x02_5C44;
    pub const COUNTDOWN_RUN: u32 = 0x02_5C80;
    pub const WATCH: u32 = 0x02_5CCE;
    pub const WATCH_RUN: u32 = 0x02_5D0E;
    pub const WATCH_END: u32 = 0x02_5D5C;
    pub const FINAL: u32 = 0x02_5D64;
    pub const FINAL_RUN: u32 = 0x02_5D8A;
    pub const DONE: u32 = 0x02_5DD2;
    pub const DONE_RUN: u32 = 0x02_5E00;
    pub const PLEASE_WAIT: u32 = 0x02_5E48;
    pub const PLEASE_WAIT_RUN: u32 = 0x02_5E56;
}

/// Tiempo de la misión (`system::rank`).
pub mod rank {
    /// Segundos que quedan (más de `MAX` = sin límite).
    pub const TIME: u32 = super::script::RANK_TIMER;
    pub const MAX: u16 = 99;
    /// Ajuste de la BIOS que elige el tiempo inicial y tabla de tiempos.
    pub const BIOS_SETTING: u32 = 0x10_FD8C;
    pub const START_TIMES: u32 = 0x07_9968;
    /// Posición en la capa fija de las decenas y de las unidades.
    pub const AT: [u16; 2] = [0x7243, 0x7283];
    pub const COLUMN: u16 = 0x20;
    pub const ROW_STEP: u16 = 0x10;
    /// Primer carácter de cada cifra 0–9 y de «sin límite» y en blanco.
    pub const DIGITS: u32 = 0x28_DECC;
    pub const INFINITE: (u16, u16) = (0x63B6, 0x63B8);
    pub const BLANK: u16 = 0x4B40;
    /// Tareas de los jugadores y bit 3 de `+$8D` (para la cuenta atrás).
    pub const PLAYER_TASKS: [u32; 2] = [0x10_0440, 0x10_04E0];
    pub const PLAYER_FLAGS: u32 = 0x8D;
    pub const PLAYER_FLAG3: u8 = 1 << 3;
    pub const SCENE_LOADED: u32 = super::scene::LOADED;
    pub const SCRIPTED_PAUSE: u32 = 0x10_E39C;
    pub const FRAME: u32 = super::system::FRAME_COUNTER;
    /// Con este tiempo o menos parpadea y pita cada segundo.
    pub const HURRY: u16 = 10;
    pub const HURRY_MASK: u8 = 0x10;
    pub const SOUND_TICK: u16 = 0x10D7;
    /// Sólo cuenta por debajo de este valor.
    pub const COUNTING_BELOW: u16 = 0x64;
    /// Campos de la tarea y sus rutinas.
    pub const DELAY: u32 = 0x30;
    pub const START_DELAY: u16 = 0x5A;
    pub const WAITING: u32 = 0x21;
    pub const TASK: u32 = 0x00_1B4C;
    pub const DELAY_RUN: u32 = 0x00_1B70;
    pub const RUN: u32 = 0x00_1B80;
    pub const WAIT_RUN: u32 = 0x00_1C34;
    /// Aviso de «TIME UP».
    pub const TIME_UP: u32 = 0x04_6A48;
}

/// Textos de la capa fija que el juego mantiene encima de todo: créditos
/// (recreativa y consola) y nivel de dificultad (recreativa).
pub mod hud {
    /// Cifras del último número en hexadecimal y sus caracteres.
    pub const HEX_DIGITS: u32 = 0x10_E21E;
    pub const HEX_CHARS: u32 = 0x05_D71C;
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

/// Cargador de escena: cámaras / capas de scroll y estado del nivel.
pub mod scene {
    /// Tabla de la ROM con un par (guion de scroll, capas) por escena.
    pub const TABLE: u32 = 0x09_16C8;
    pub const TABLE_ENTRY: u32 = 8;
    /// Guion de scroll en curso (lo ejecuta la máquina de scroll).
    pub const SCRIPT_PC: u32 = 0x10_815C;
    /// Capas de la escena: registros de 14 bytes {tipo, -, cámara.l,
    /// caja[4 words]} acabados en uno de tipo `LAYER_END`, cuyo byte 1 dice
    /// cuántos sprites hardware se reservan al final.
    pub const LAYER_SIZE: u32 = 14;
    pub const LAYER_TYPE: u32 = 0;
    pub const LAYER_END_RESERVE: u32 = 1;
    pub const LAYER_CAMERA: u32 = 2;
    pub const LAYER_BOX: u32 = 6;
    pub const LAYER_END: u8 = 2;
    /// Tipo 0: sprites de la zona alta (desde el corte); otro: de la baja.
    pub const LAYER_HIGH: u8 = 0;
    /// Cada capa usa una ventana de 32 columnas de sprite.
    pub const LAYER_SPRITES: u16 = 0x20;
    pub const LAYER_ROWS: u32 = 0x400;
    /// Las cuatro cámaras (contexto de capa, `0x7C` bytes cada una).
    pub const CAMERAS: [u32; 4] = [0x10_6F6C, 0x10_7FE8, 0x10_8064, 0x10_80E0];
    /// Campos de una cámara.
    pub const C_POS: [u32; 2] = [0x00, 0x02];
    pub const C_LONGS: [u32; 2] = [0x04, 0x08];
    pub const C_FLAGS: u32 = 0x0C;
    pub const C_LINK: u32 = 0x0E;
    pub const C_BOX: u32 = 0x16;
    pub const C_WORDS: [u32; 4] = [0x1E, 0x20, 0x22, 0x24];
    pub const C_SPRITE_BASE: u32 = 0x28;
    pub const C_ORIGIN: [u32; 2] = [0x2A, 0x2C];
    pub const C_ROWS: u32 = 0x32;
    pub const C_ROWS_LEN: u32 = 32;
    pub const C_SCALE: [u32; 2] = [0x74, 0x76];
    pub const C_SCALE_ONE: u16 = 0x100;
    pub const C_FLAG72: u32 = 0x72;
    pub const C_LONG78: u32 = 0x78;
    /// Mapa de colisión de la primera cámara (4 KB, ver `sprite_draw`).
    pub const COLLMAP_CELLS: u32 = 0x10_6F6C + 0x7C;
    pub const COLLMAP_LEN: u32 = 0x1000;
    /// Cámara principal: X, Y, Z (longs 16.16), y otras palabras.
    pub const CAMERA_X: u32 = 0x10_6F50;
    pub const CAMERA_Y: u32 = 0x10_6F54;
    pub const CAMERA_Z: u32 = 0x10_6F58;
    pub const CAMERA_W5C: u32 = 0x10_6F5C;
    pub const CAMERA_W5E: u32 = 0x10_6F5E;
    /// Límites de la vista: (variable, valor inicial).
    pub const VIEW_INIT: [(u32, u16); 9] = [
        (0x10_8160, 0),
        (0x10_8162, 0),
        (0x10_8164, 0xA0),
        (0x10_8166, 0x180),
        (0x10_8168, 0),
        (0x10_816A, 0),
        (0x10_816C, 0xFFFF),
        (0x10_816E, 0xFFFF),
        (0x10_8170, 0),
    ];
    /// Copias de la cámara al empezar (Z, X alta, `W5C`).
    pub const START_W5C: u32 = 0x10_8172;
    pub const START_X: u32 = 0x10_8174;
    pub const START_Z: u32 = 0x10_8176;
    pub const FROZEN: u32 = 0x10_8179;
    /// Suavizado vertical de la cámara (objetivo y velocidad).
    pub const SMOOTH: [u32; 2] = [0x10_817A, 0x10_817E];
    /// Bytes y palabras de la misión que se ponen a 0 al cargar.
    pub const CLEAR_BYTES: [u32; 2] = [0x10_E39C, 0x10_E39D];
    pub const CLEAR_WORDS: [u32; 19] = [
        0x10_6F42, 0x10_E27C + 0x30, 0x10_E2AE + 0x30,
        0x10_E2F2, 0x10_E2F4, 0x10_E2F6, 0x10_E2F8,
        0x10_E33A, 0x10_E33C, 0x10_E33E, 0x10_E340,
        0x10_E362, 0x10_E364, 0x10_E366, 0x10_E368,
        0x10_6F4C, 0x10_6F4E, 0x10_E47E, 0x10_E480,
    ];
    pub const CLEAR_WORD_LAST: u32 = 0x10_E482;
    /// La escena está cargada.
    pub const LOADED: u32 = 0x10_6ED3;
    /// Puntero por misión (`script::KEY_LATCH[0]` = misión, `MISSIONS`
    /// entradas en la tabla `MISSION_TABLE`; si no, `NIL`).
    pub const MISSION_PTR: u32 = 0x10_81B2;
    pub const MISSION_TABLE: u32 = 0x04_CB44;
    pub const MISSIONS: u8 = 6;
    pub const MISSION_KEY: u32 = 0x10_6ECE;
    pub const NIL: u32 = 0xFFFF_FFFF;
}

/// Máquina del guion de scroll (`scene::SCRIPT_PC`): posición de la cámara
/// principal, límites, avance de la misión y suavizado vertical.
/// Varias direcciones coinciden con las de `scene` (que las inicializa).
pub mod scroll {
    pub const PC: u32 = super::scene::SCRIPT_PC;
    /// Posición X, Y suavizada e Y real (longs 16.16).
    pub const POS_X: u32 = 0x10_6F50;
    pub const POS_Y_SMOOTH: u32 = 0x10_6F54;
    pub const POS_Y: u32 = 0x10_6F58;
    /// Avance máximo alcanzado en la escena.
    pub const PROGRESS: u32 = 0x10_6F5C;
    /// Si no es 0, se usan estas velocidades (16.16) en vez de las del que llama.
    pub const SEED_LATCH: u32 = 0x10_6F5E;
    pub const SEED_VEL: [u32; 2] = [0x10_6F60, 0x10_6F64];
    /// Desplazamiento entero del frame.
    pub const DELTA_X: u32 = 0x10_8160;
    pub const DELTA_Y: u32 = 0x10_8162;
    /// Pareja de palabras que fija el op `$10`.
    pub const PAIR: [u32; 2] = [0x10_8164, 0x10_8166];
    /// Límites (enteros) y pendiente (Y por X, en 1/256) del tramo.
    pub const MIN_X: u32 = 0x10_8168;
    pub const MIN_Y: u32 = 0x10_816A;
    pub const MAX_X: u32 = 0x10_816C;
    pub const MAX_Y: u32 = 0x10_816E;
    pub const SLOPE: u32 = 0x10_8170;
    /// Referencias para medir el avance: avance, X e Y al empezar el tramo.
    pub const BASE_PROGRESS: u32 = 0x10_8172;
    pub const BASE_X: u32 = 0x10_8174;
    pub const BASE_Y: u32 = 0x10_8176;
    /// Eje del avance: 0 = X hacia atrás, 2 = Y hacia atrás,
    /// 3 = Y hacia delante, otro = X hacia delante.
    pub const AXIS: u32 = 0x10_8178;
    pub const AXIS_X_BACK: u8 = 0;
    pub const AXIS_Y_BACK: u8 = 2;
    pub const AXIS_Y_FORWARD: u8 = 3;
    /// El op `$03` deja el eje en X hacia delante.
    pub const AXIS_X_FORWARD: u8 = 1;
    /// Si no es 0, la prueba de llegada al borde siempre dice que sí.
    pub const EDGE_OFF: u32 = 0x10_8179;
    /// Flag de la misión que activa la prueba de llegada al borde.
    pub const EDGE_ENABLE: u32 = 0x10_E39A;
    /// Suavizado vertical: acumulado y velocidad (16.16) y paso entero.
    pub const SMOOTH_ACC: u32 = 0x10_817A;
    pub const SMOOTH_VEL: u32 = 0x10_817E;
    pub const SMOOTH_DELTA: u32 = 0x10_8182;
    pub const SMOOTH_LIMIT: i32 = 0x8_0000;
    pub const SMOOTH_DAMP: i32 = 0x4000;

    /// Órdenes del guion (palabra) y su tamaño en bytes.
    pub const OP_BIND_PATH: u16 = 0x00;
    pub const OP_SET_FIELDS: u16 = 0x01;
    pub const OP_END_FRAME: u16 = 0x02;
    pub const OP_WARP: u16 = 0x03;
    pub const OP_SET_TILE: u16 = 0x04;
    pub const OP_DETACH: u16 = 0x05;
    pub const OP_WAIT: u16 = 0x06;
    pub const OP_SHOW: u16 = 0x07;
    pub const OP_HIDE: u16 = 0x08;
    pub const OP_PROBE_SHOW: u16 = 0x09;
    pub const OP_SPRITE_LIST: u16 = 0x0A;
    pub const OP_CLEAR_TILES: u16 = 0x0B;
    pub const OP_LSPC_MODE: u16 = 0x0C;
    pub const OP_CALL_ARGS: u16 = 0x0D;
    pub const OP_SPAWN: u16 = 0x0E;
    pub const OP_LIMITS: u16 = 0x0F;
    pub const OP_PAIR: u16 = 0x10;
    pub const OP_TILE_TABLE: u16 = 0x11;
    pub const OP_AXIS: u16 = 0x12;
    pub const OP_SOUND: u16 = 0x13;
    pub const OP_CALL_JUMP: u16 = 0x14;
    pub const OP_SET_POS: u16 = 0x15;
    pub const OP_SET_PROGRESS: u16 = 0x16;

    /// Códigos 68000 de las rutinas embebidas en los guiones de escena.
    pub const EMB_CMPI_B: u16 = 0x0C39;
    pub const EMB_CMPI_W: u16 = 0x0C79;
    pub const EMB_MOVE_B: u16 = 0x13FC;
    pub const EMB_MOVE_L: u16 = 0x23FC;
    pub const EMB_SCS: u16 = 0x55C0;
    pub const EMB_SHI: u16 = 0x52C0;
    pub const EMB_SNE: u16 = 0x56C0;
    pub const EMB_SEQ: u16 = 0x57C0;
    pub const EMB_LEA_A0: u16 = 0x41FA;
    pub const EMB_LEA_A1: u16 = 0x43FA;
    pub const EMB_RTS: u16 = 0x4E75;

    /// Campos de una cámara de capa (`scene::CAMERAS`) que usa el scroll.
    pub const C_DELTA: [u32; 2] = [0x00, 0x02];
    pub const C_POS: [u32; 2] = [0x04, 0x08];
    pub const C_FLAGS: u32 = 0x0C;
    pub const C_VISIBLE: u8 = 1 << 0;
    pub const C_PATH: u32 = 0x0E;
    pub const C_TILE_TABLE: u32 = 0x12;
    /// Ventana visible de la capa en el mapa: (x, y) en píxeles y
    /// (ancho, alto) en celdas.
    pub const C_BOX: [u32; 4] = [0x16, 0x18, 0x1A, 0x1C];
    pub const C_ORIGIN_CELL: [u32; 2] = [0x1E, 0x20];
    pub const C_TILE_BASE: [u32; 2] = [0x22, 0x24];
    pub const C_TILE_ROW: u32 = 0x26;
    pub const C_TILE_ATTR: u32 = 0x27;
    pub const C_SPRITE_BASE: u32 = 0x28;
    /// Columnas que repinta el vblank: primera y última (excluida), y
    /// posición de pantalla (X, Y) ya preparada para SCB4 / SCB3.
    pub const C_COL_FIRST: u32 = 0x2A;
    pub const C_COL_END: u32 = 0x2C;
    pub const C_SCREEN_X: u32 = 0x2E;
    pub const C_SCREEN_Y: u32 = 0x30;
    /// Atributos y fila de tile por columna (32 bytes cada uno).
    pub const C_COL_ATTR: u32 = 0x32;
    pub const C_COL_ROW: u32 = 0x52;
    pub const C_SCROLL_MODE: u32 = 0x72;
    pub const C_SCALE: [u32; 2] = [0x74, 0x76];
    pub const C_COLLMAP_SRC: u32 = 0x78;
    pub const C_COLLMAP: u32 = 0x7C;
    pub const COLUMNS: u16 = 32;
    pub const VISIBLE_COLUMNS: u16 = 0x16;
    /// Rutina diferida al vblank que repinta las columnas de una capa.
    pub const COLUMN_WRITE: u32 = 0x05_1F94;

    /// Rutinas originales que el scroll aún llama.
    pub const TICKS: [u32; 4] = [0x07_7092, 0x08_F172, 0x09_97CC, 0x08_F8E2];

    /// Marcador de depuración de la misión: lista de registros de 16 bytes
    /// {.., rel_x.b, rel_y.b, x.w, y.w, ...} con el avance en `+$10` del
    /// siguiente.
    pub const MARK_SIZE: u32 = 0x10;
    pub const MARK_PROGRESS: u32 = 0x10;
    pub const MARK_REL: [u32; 2] = [0x02, 0x03];
    pub const MARK_POS: [u32; 2] = [0x04, 0x06];
    pub const MARK_Y_BASE: u16 = 0x200;
    pub const MARK_GFX: u32 = 0x24_3608;
    pub const MARK_KEY: u16 = 0xFFFF;
    /// Bit de la variable de depuración que dibuja el marcador.
    pub const DEBUG: u32 = 0x10_0001;
    pub const DEBUG_MARK: u8 = 1 << 0;

    /// Mapa de colisión de la primera cámara.
    pub const COLL_CAMERA: u32 = 0x10_6F6C;
    pub const COLL_ORIGIN: [u32; 2] = [0x04, 0x08];
    /// Posición de la entidad que estampa celdas y X (con signo) entre las
    /// que se estampan.
    pub const ENTITY_POS: [u32; 2] = [0x22, 0x24];
    pub const CELL_CLIP: (i16, i16) = (-0x50, 0x188);
    /// Límite X de la cámara (lo pone un jefe cada frame) y la tarea fija
    /// que lo borra.
    pub const CAMERA_LOCK_X: u32 = 0x10_6F42;
    pub const LOCK_X_TASK: u32 = 0x10_08A0;
    pub const CLEAR_LOCK_X: u32 = 0x05_EA96;
    pub const CLEAR_LOCK_X_RUN: u32 = 0x05_EA9C;
}

/// Foco de la cámara: los dos jugadores y la ventana en la que la cámara
/// los mantiene.
/// Prisionero suelto por el escenario (`system::pow`).
pub mod pow {
    /// Hacia dónde mira (bit 0 = izquierda) y `+$9A` ≠ 0 = salir mirando
    /// a la izquierda.
    pub const FACING: u32 = 0x3A;
    pub const FACING_LEFT: u8 = 1 << 0;
    pub const SPAWN_LEFT: u32 = 0x9A;
    /// Objeto que suelta (`+$9B`) y sus parámetros (`+$9C`).
    pub const ITEM: u32 = 0x9B;
    pub const ITEM_PARAMS: u16 = 0x9C;
    /// Gráficos, prioridad, velocidades y aceleraciones, y `+$70` apoyado.
    pub const GFX: u16 = 0x182;
    pub const ATTR: u32 = 0x38;
    pub const ATTR_BASE: u16 = 0x8000;
    pub const ATTR_KEEP: u16 = 0xFFE3;
    pub const PRIORITY: u16 = 0x18;
    pub const MOTION: [u32; 4] = [0x28, 0x2A, 0x2C, 0x2E];
    pub const GROUNDED: u32 = 0x70;
    /// Sombra (tarea hija) y márgenes con los que se va de la vista.
    pub const SHADOW: u32 = 0x07_76E2;
    pub const MARGINS: u32 = 0x03_EFE4;
    /// Sentado: guion y bucle.
    pub const S_SEATED: u32 = 0x03_F292;
    pub const SEATED_LOOP: u32 = 0x03_FC4C;
    /// Liberado: estado, guion, bucle, el estado de irse andando y el
    /// vitoreo (estado, guion y bucle).
    pub const FREED: u32 = 0x03_FC6A;
    pub const S_FREED: u32 = 0x03_F39E;
    pub const FREED_LOOP: u32 = 0x03_FC7A;
    pub const WALK_OFF: u32 = 0x03_FF14;
    pub const CHEER: u32 = 0x03_FCA4;
    pub const S_CHEER: u32 = 0x03_F3C2;
    pub const CHEER_LOOP: u32 = 0x03_FCC0;
    /// De pie esperando (`Pow_Idle_03fe66`) y el saludo al darle.
    pub const IDLE: u32 = 0x03_FE66;
    pub const S_IDLE: u32 = 0x03_F070;
    pub const IDLE_LOOP: u32 = 0x03_FE7E;
    pub const IDLE_WAVE: u32 = 0x03_FE9C;
    pub const S_IDLE_WAVE: u32 = 0x03_F0CC;
    pub const IDLE_WAVE_LOOP: u32 = 0x03_FEAC;
    /// Andar: gráficos, base de animaciones (`+$48`), `+$13` bits 0 y 3
    /// que se limpian, `+$98` hacia dónde (0 = como mira), `+$5C` frames
    /// hasta pararse, velocidad, `+$5A` bit 5 = pared y `+$72` columna del
    /// borde cruzado.
    pub const GFX_WALK: u16 = 0x181;
    pub const ANIM_BASE: u32 = 0x48;
    pub const ANIM_BASE_WALK: u32 = 0x03_F01C;
    pub const FLAGS_13: u32 = 0x13;
    pub const FLAGS_13_CLEAR: u8 = (1 << 3) | (1 << 0);
    pub const KIND: u32 = 0x98;
    pub const WALK_TIMER: u32 = 0x5C;
    pub const WALK_FRAMES: u16 = 0xF;
    pub const WALK_AGAIN_FRAMES: u16 = 0x1E;
    pub const X: u32 = 0x22;
    pub const VEL_X: u32 = 0x28;
    pub const VEL_Y: u32 = 0x2A;
    pub const WALK_SPEED: u16 = 0x180;
    pub const STATE: u32 = 0x5A;
    pub const HIT_WALL: u8 = 1 << 5;
    pub const EDGE: u32 = 0x72;
    /// Estados de andar y sus guiones (salto y caída: aún originales).
    pub const PICK_WALK: u32 = 0x03_FF24;
    pub const WALK: u32 = 0x03_FF62;
    pub const WALK_LOOP: u32 = 0x03_FF86;
    pub const WALK_TURN: u32 = 0x04_0014;
    pub const WALK_TURN_LOOP: u32 = 0x04_0034;
    pub const WALK_STOP: u32 = 0x04_0054;
    pub const WALK_STOP_LOOP: u32 = 0x04_0068;
    pub const JUMP: u32 = 0x04_008C;
    pub const FALL: u32 = 0x04_01A8;
    pub const S_WALK: u32 = 0x03_F40A;
    pub const S_WALK_TURN: u32 = 0x03_F194;
    pub const S_WALK_STOP: u32 = 0x03_F488;
    /// Rescate: caja que tocan los jugadores (`$03EF9A`, datos), jugador
    /// que lo rescata (`+$71`), contadores de rescatados por jugador, voz,
    /// pasos rápidos hacia el centro (`+$30`), `+$9B` ≠ 0 = hace la
    /// reverencia, carrera final y parpadeo de salida (`+$78`).
    pub const RESCUE_BOX: u32 = 0x03_EF9A;
    pub const RESCUER: u32 = 0x71;
    pub const RESCUED_COUNT: [u32; 2] = [0x10_6F4C, 0x10_6F4E];
    pub const THANKS_SOUND: u16 = 0x113A;
    pub const SCREEN_MID: u16 = 0xC0;
    pub const RESCUED_SPEED: u16 = 0x700;
    pub const STEPS: u32 = 0x30;
    pub const RESCUED_STEPS: u16 = 4;
    pub const BOWS: u32 = 0x9B;
    pub const DASH_SPEED: u16 = 0xFC00;
    pub const EXIT_TIMER: u32 = 0x78;
    pub const EXIT_FRAMES: u8 = 0x1E;
    pub const RESCUED: u32 = 0x04_0268;
    pub const RESCUED_LOOP: u32 = 0x04_02C0;
    pub const RESCUED_BOW: u32 = 0x04_0304;
    pub const RESCUED_BOW_LOOP: u32 = 0x04_0318;
    pub const RESCUED_RUN: u32 = 0x04_032E;
    pub const RESCUED_RUN_LOOP: u32 = 0x04_0342;
    pub const RESCUED_DASH: u32 = 0x04_0358;
    pub const RESCUED_DASH_LOOP: u32 = 0x04_0374;
    pub const RESCUED_EXIT: u32 = 0x04_0392;
    pub const RESCUED_EXIT_LOOP: u32 = 0x04_039E;
    pub const S_BOW: u32 = 0x03_F49E;
    pub const S_RUN_START: u32 = 0x03_F8F4;
    pub const S_DASH: u32 = 0x03_F89E;
}

/// Marcador de puntos flotante y sus cifras (`system::score`).
pub mod score {
    pub const X: u32 = 0x22;
    pub const Y: u32 = 0x24;
    pub const FACING: u32 = 0x3A;
    pub const ATTR: u32 = 0x38;
    pub const PARENT: u32 = 0x0C;
    /// Jugador que se lleva los puntos (`+$68`, copiado a `+$6E`; negativo
    /// = nadie) y las puntuaciones, con la zona donde se dejan los puntos.
    pub const PLAYER: u32 = 0x68;
    pub const LAST_PLAYER: u32 = 0x6E;
    pub const SCORES: [u32; 2] = [0x10_6E94, 0x10_6E9C];
    pub const SCORE_POINTS: u32 = 0x10_81BA;
    /// Tipo (`+$98`), tabla alternativa (`+$9C`), puntos BCD y número de
    /// cifras por tipo (normal y alternativa).
    pub const KIND: u32 = 0x98;
    pub const ALT: u32 = 0x9C;
    pub const VALUES: [u32; 2] = [0x2F_792C, 0x2F_794C];
    pub const COUNTS: [u32; 2] = [0x2F_798C, 0x2F_7994];
    /// Cifras que quedan por crear, cifras que aún caen (`+$21`, $FF =
    /// parpadeando) y los puntos que quedan por repartir (una cifra por
    /// nibble).
    pub const DIGITS_LEFT: u32 = 0x72;
    pub const FALLING: u32 = 0x21;
    pub const BLINKING: u8 = 0xFF;
    pub const PENDING: u32 = 0x80;
    /// Ancho de una cifra y desplazamiento vertical al azar (máscara + base).
    pub const DIGIT_STEP: u16 = 8;
    pub const Y_JITTER: u16 = 0xE;
    pub const Y_BASE: u16 = 9;
    /// Contadores: espera y vida (frames), parpadeo, bucle, salida y
    /// márgenes de la vista.
    pub const TIMER: u32 = 0x70;
    pub const LIFETIME: u32 = 0x84;
    pub const HOLD_FRAMES: u16 = 0x1E;
    pub const LIFE_FRAMES: u16 = 0x96;
    pub const BLINK_FRAMES: u16 = 0xA;
    pub const RUN_LOOP: u32 = 0x09_B5F6;
    pub const FREE: u32 = 0x09_B47C;
    pub const MARGINS: u32 = 0x09_A6C6;
    /// Cifra: tarea, valor (`+$74`), retardo (`+$70`), gráficos por jugador,
    /// prioridad, salto (velocidad y gravedad) y guiones por valor.
    pub const DIGIT: u32 = 0x09_B66A;
    pub const DIGIT_VALUE: u32 = 0x74;
    pub const DIGIT_DELAY: u32 = 0x70;
    pub const DIGIT_GFX: [(u16, u16); 2] = [(0x1B8, 0x1B9), (0x1CD, 0x1CE)];
    pub const DIGIT_ATTR: u16 = 0xC000;
    pub const ATTR_KEEP: u16 = 0xFFE3;
    pub const DIGIT_PRIORITY: u16 = 0x1C;
    pub const VEL_X: u32 = 0x28;
    pub const VEL_Y: u32 = 0x2A;
    pub const ACC_X: u32 = 0x2C;
    pub const ACC_Y: u32 = 0x2E;
    pub const DIGIT_VEL_Y: u16 = 0x886;
    pub const DIGIT_ACC_Y: u16 = 0xFEDD;
    pub const DIGIT_SCRIPTS: u32 = 0x2F_77A4;
    pub const DIGIT_LOOP: u32 = 0x09_B702;
    /// Encima del marcador: altura, rebote y estados.
    pub const LIFT_DY: u16 = 0x10;
    pub const LIFT_VEL: u16 = 0x400;
    pub const LIFT_FRAMES: u16 = 2;
    pub const LIFT: u32 = 0x09_B75A;
    pub const LIFT_LOOP: u32 = 0x09_B772;
    pub const HOLD: u32 = 0x09_B7BC;
    pub const HOLD_LOOP: u32 = 0x09_B7C2;
    pub const BLINK: u32 = 0x09_B7F8;
    pub const BLINK_LOOP: u32 = 0x09_B7FE;
    /// Sprite del frame (par / impar) y contador de frames.
    pub const SPRITE: u32 = 0x14;
    pub const SPRITE_EVEN: u32 = 0x16;
    pub const SPRITE_ODD: u32 = 0x18;
    pub const FRAME_COUNTER: u32 = 0x10_6F28;
    /// Rutinas de una tarea que ya no está.
    pub const INACTIVE: [u32; 3] = [0xFFFF_FFFF, 0x00_052A, 0x00_0400];
}

pub mod focus {
    pub const PLAYERS: [u32; 2] = [0x10_0440, 0x10_04E0];
    /// Rutinas de un jugador que no está en juego.
    pub const INACTIVE: [u32; 3] = [0xFFFF_FFFF, 0x00_052A, 0x00_0400];
    pub const X: u32 = 0x22;
    pub const Y: u32 = 0x24;
    /// Bit 7: la Y del jugador cuenta.
    pub const FLAGS: u32 = 0x5B;
    pub const HAS_Y: u8 = 1 << 7;
    /// Altura de la pantalla para invertir la Y.
    pub const SCREEN_H: u16 = 0x200;
    /// Ventana horizontal y vertical del foco.
    pub const WINDOW_X: (u16, u16) = (0x1E, 0x117);
    pub const WINDOW_Y: (u16, u16) = (0, 0xD0);
    pub const SPEED_LIMIT: i32 = 0x8_0000;
}

/// Manual de juego («cómo jugar»): la tarea que crea el inicio de misión
/// cuando la BIOS tiene activado `script::PLAY_MANUAL`.
pub mod manual {
    /// Rutina de cada fase (la tarea pasa de una a otra).
    pub const START: u32 = 0x08_C008;
    pub const FADE_IN: u32 = 0x08_C10A;
    pub const LIGHT_UP: u32 = 0x08_C15E;
    pub const STEPS: u32 = 0x08_C1AC;
    pub const DIM: u32 = 0x08_C1EA;
    pub const FADE_OUT: u32 = 0x08_C23A;
    pub const WAIT_END: u32 = 0x08_C296;
    /// Sonido al empezar, gráficos y guion de animación del manual.
    pub const SOUND: u16 = 0x2B;
    pub const GFX: u16 = 0x12E;
    pub const ANIM: u32 = 0x2F_2DBE;
    pub const POS: (u16, u16) = (0xA0, 0x1C3);
    pub const X: u32 = 0x22;
    pub const Y: u32 = 0x24;
    pub const Z: u32 = 0x26;
    pub const FIELD_38: u32 = 0x38;
    pub const FIELD_38_INIT: u16 = 0x4000;
    pub const FLAGS_12: u32 = 0x12;
    pub const FLAGS_12_SET: u8 = 1 << 6;
    /// Escena de fondo del manual.
    pub const SCENE: u8 = 9;
    /// Guion de pasos: registros de 8 bytes {frames.w, valor.w, b4, b5, -}
    /// acabados en frames = `STEP_END`.
    pub const STEP_TIMER: u32 = 0x70;
    pub const STEP_VALUE: u32 = 0x76;
    pub const STEP_OFFSET: u32 = 0x78;
    pub const STEP_SCRIPT: u32 = 0x7C;
    pub const STEP_BYTES: [u32; 2] = [0x92, 0x93];
    pub const STEP_SIZE: u32 = 8;
    pub const STEP_END: u16 = 0xFFFF;
    pub const STEPS_SCRIPT: u32 = 0x2E_F8AA;
    /// Puntero a un bloque propio (`+$90`).
    pub const SELF_PTR: u32 = 0x72;
    pub const SELF_BLOCK: u32 = 0x90;
    /// Tarea hija con posición y un byte de tipo.
    pub const CHILD: u32 = 0x02_AE50;
    pub const CHILD_POS: (u16, u16) = (0xA0, 0x1FF);
    pub const CHILD_KIND: u32 = 0x98;
    pub const CHILD_KIND_VALUE: u8 = 3;
    /// Rutina que se pone en la tarea del jugador 1 (reiniciada).
    pub const PLAYER_TASK: u32 = 0x10_0440;
    pub const PLAYER_HANDLER: u32 = 0x03_35A6;
    /// Iconos y textos del manual (tareas hijas).
    pub const ICONS: [u32; 6] = [0x08_C2B8, 0x08_C322, 0x08_C37E, 0x08_C3DA, 0x08_C436, 0x08_C5B2];
    /// Brillo de dos canales (bytes) y fase.
    pub const LEVEL: [u32; 2] = [0x32, 0x33];
    pub const PHASE: u32 = 0x34;
    pub const PHASE_END: i16 = 0x3F;
    pub const LEVEL_MAX: u16 = 0xFF;
    /// Tablas de seno (palabras): de subida y de bajada.
    pub const SINE_UP: u32 = 0x2C_072C;
    pub const SINE_DOWN: u32 = 0x2C_07AC;
    /// Rectángulo de la capa fija que se borra al empezar los pasos
    /// (dirección, carácter, columnas, filas).
    pub const CLEAR_RECT: (u16, u16, u16, u16) = (0x7084, 0x2320, 0x20, 0x19);
    /// Al acabar: ajuste de sonido, fundido y espera.
    pub const END_SETTING: u8 = 0x80;
    pub const END_FADE: u16 = 2;
    pub const END_WAIT: u16 = 0x3C;
    pub const TIMER: u32 = 0x70;
}

/// Iconos del manual de juego (tareas hijas de `manual::START`): la base,
/// cuatro ranuras que se encienden al pulsar su botón y el ancla del texto.
pub mod manual_icons {
    pub const X: u32 = 0x22;
    pub const Y: u32 = 0x24;
    pub const Z: u32 = 0x26;
    pub const ATTR: u32 = 0x38;
    pub const ATTR_ICON: u16 = 0xE000;
    pub const ATTR_ANCHOR: u16 = 0x8000;
    pub const FLAGS_12: u32 = 0x12;
    pub const FLAGS_12_SET: u8 = 1 << 6;
    /// Fila de todos los iconos.
    pub const ROW_Y: u16 = 0x12B;
    /// Base: gráficos, guion inicial, X, rutina del bucle y tabla de
    /// guiones por nibble bajo del byte `+$92` de la madre (-1 = ninguno).
    pub const BASE_GFX: u16 = 0xB1;
    pub const BASE_ANIM: u32 = 0x2F_2E0A;
    pub const BASE_X: u16 = 0x40;
    pub const BASE_RUN: u32 = 0x08_C2F0;
    pub const BASE_TABLE: u32 = 0x2F_2DCA;
    pub const NO_ANIM: u32 = 0xFFFF_FFFF;
    /// Ranuras: (rutina de alta, gráficos, X, rutina encendida, guion
    /// encendido, bit del botón).
    pub const SLOTS: [(u32, u16, u16, u32, u32, u8); 4] = [
        (0x08_C322, 0xAE, 0x70, 0x08_C4FA, 0x2F_2F22, 0x10),
        (0x08_C37E, 0xAF, 0xA0, 0x08_C526, 0x2F_2F2E, 0x20),
        (0x08_C3DA, 0xB0, 0xD0, 0x08_C552, 0x2F_2F3A, 0x40),
        (0x08_C436, 0xB2, 0x100, 0x08_C57E, 0x2F_2F46, 0x80),
    ];
    pub const SLOT_ANIM: u32 = 0x2F_2ED6;
    pub const SLOT_RUN: u32 = 0x08_C4A0;
    /// Guiones al pulsar y al soltar.
    pub const SLOT_PRESSED: u32 = 0x2F_2EE2;
    pub const SLOT_RELEASED: u32 = 0x2F_2F02;
    /// Bit propio y estado (0 = suelto, 1 = pulsado).
    pub const SLOT_BIT: u32 = 0x8A;
    pub const SLOT_DOWN: u32 = 0x76;
    /// Bytes de la madre: botones nuevos (`+$93`) y mantenidos (`+$92`).
    pub const MOTHER_NEW: u32 = 0x93;
    pub const MOTHER_HELD: u32 = 0x92;
    /// Rutina de las versiones encendidas (sólo animan).
    pub const LIT_RUN: u32 = 0x08_C5AA;
    /// La cuarta ranura borra antes un trozo de la capa fija
    /// (dirección, carácter, columnas, filas).
    pub const SLOT4_TILES: (u16, u16, u16, u16) = (0x73D7, 0x2E80, 4, 4);
    /// Ancla del texto: gráficos, guion, posición y rutina del bucle.
    pub const ANCHOR_GFX: u16 = 0x97;
    pub const ANCHOR_ANIM: u32 = 0x2F_2F52;
    pub const ANCHOR_POS: (u16, u16) = (0xE0, 0x160);
    pub const ANCHOR_RUN: u32 = 0x08_C678;
    /// Ancla que se suelta: posición, gráfico, velocidad, guion, cajas de
    /// ataque, bit de choque (`+$13`), márgenes, la explosión y el bucle.
    pub const DROP_POS: (u16, u16) = (0x140, 0x180);
    pub const DROP_GFX: u16 = 0x179;
    pub const VEL_X: u32 = 0x28;
    pub const DROP_VEL: u16 = 0xF800;
    pub const DROP_ANIM: u32 = 0x2E_F80E;
    pub const ATTACKS: u32 = 0x4C;
    pub const DROP_ATTACKS: u32 = 0x2E_F84C;
    pub const DROP_HIT: u8 = 1 << 1;
    pub const DROP_MARGINS: u32 = 0x2E_F8A0;
    pub const DROP_FX: u32 = 0x07_7F6A;
    pub const DROP_RUN: u32 = 0x08_C77A;
    /// Idioma del texto: 0 = japonés, 1 = inglés, 2 = portugués, 3 = español.
    pub const LANGUAGE: u32 = 0x8A;
    pub const LANG_JP: u8 = 0;
    pub const LANG_EN: u8 = 1;
    pub const LANG_PT: u8 = 2;
    pub const LANG_ES: u8 = 3;
    /// Región e idioma de la BIOS (región: 0 = Japón, 2 = Europa).
    pub const BIOS_REGION: u32 = 0x10_FD83;
    pub const BIOS_LANGUAGE: u32 = 0x10_FD92;
    pub const REGION_JAPAN: u8 = 0;
    pub const REGION_EUROPE: u8 = 2;
    /// Cursor de texto del ancla: guion, posición en la capa fija,
    /// temporizador, carácter y desplazamiento.
    pub const TEXT_SCRIPT: u32 = 0x7C;
    pub const TEXT_AT: u32 = 0x86;
    pub const TEXT_AT_INIT: u32 = 0x7084;
    pub const TEXT_TIMER: u32 = 0x70;
    pub const TEXT_CHAR: u32 = 0x80;
    pub const TEXT_OFFSET: u32 = 0x82;
    pub const TEXT_PAGE: u32 = 0x76;
    pub const TEXT_FIRST: u32 = 0x1F_0056 + 0x10_0000;
    /// Páginas de texto de cada idioma (por `LANGUAGE`): pares {guion,
    /// posición en la capa fija}. La página la pide la madre en `TEXT_PAGE`.
    pub const TEXT_PAGES: [u32; 4] = [0x2E_FACC, 0x2E_FC2C, 0x2E_FD8C, 0x2E_FEEC];
    /// Entradas del guion de texto (8 bytes): duración (`TEXT_END` = fin),
    /// código y guion de animación del ancla (0 = ninguno).
    pub const TEXT_END: u16 = 0xFFFF;
    pub const ENTRY_SIZE: u32 = 8;
    pub const ENTRY_CODE: u32 = 2;
    pub const ENTRY_ANIM: u32 = 4;
    /// Códigos: nada, borrar el recuadro, soltar el ancla, soltar el Slug y
    /// salto de línea; el resto son caracteres.
    pub const CODE_NONE: u16 = 0;
    pub const CODE_CLEAR: u16 = 1;
    pub const CODE_DROP: u16 = 2;
    pub const CODE_SLUG: u16 = 3;
    pub const CODE_NEWLINE: u16 = 0xD;
    /// Recuadro del texto (dirección, carácter, columnas, filas).
    pub const CLEAR_RECT: (u16, u16, u16, u16) = (0x7084, 0x2320, 0x20, 8);
    pub const DROP_TASK: u32 = 0x08_C730;
    /// El Slug que suelta el manual: posición y variante de caída.
    pub const SLUG_TASK: u32 = 0x02_AE50;
    pub const SLUG_POS: (u16, u16) = (0xA0, 0x1FF);
    pub const SLUG_KIND: u32 = 0x98;
    pub const SLUG_KIND_MANUAL: u8 = 3;
    /// Japonés: caracteres de 2×2 que avanzan dos columnas; línea nueva
    /// fija; paleta 4 (9 para los bloques 5–7 y desde el D).
    pub const JP_STEP: u32 = 0x40;
    pub const JP_NEWLINE: u32 = 0x70A9;
    pub const JP_PALETTE: u16 = 4;
    pub const JP_PALETTE_ALT: u16 = 9;
    pub const JP_ALT_BLOCKS: [u16; 3] = [0x500, 0x600, 0x700];
    pub const JP_ALT_FROM: u16 = 0xD00;
    pub const JP_BLOCK_MASK: u16 = 0xFF00;
    pub const PALETTE_SHIFT: u16 = 12;
    pub const GLYPH_SIZE: u16 = 2;
    /// Resto: un carácter por columna con la paleta y banco `LATIN_ATTR`;
    /// la línea nueva vuelve a la primera columna y baja dos filas; las
    /// tildes se dibujan encima (`ACCENT_*`) o debajo (`ACCENTS_BELOW`).
    pub const LATIN_STEP: u32 = 0x20;
    pub const LATIN_ATTR: u16 = 0x2300;
    pub const LATIN_NEWLINE_MASK: u32 = 0xF00F;
    pub const LATIN_NEWLINE_ADD: u32 = 0xA2;
    pub const ACCENT_ABOVE: u16 = 0xE6;
    pub const ACCENTS_BELOW: core::ops::RangeInclusive<u16> = 0xE7..=0xEF;
    pub const ACCENT_TILE: u16 = 0x10;
    /// El ancla se va cuando pasa de esta X.
    pub const EXIT_X: u16 = 0x150;
    /// Campo de movimiento que se pone a 0.
    pub const MOVE_FIELD: u32 = 0x2C;
    /// Palabras de movimiento que borra `$0267E2` y el largo que va entre ellas.
    pub const MOTION_WORDS: [u32; 3] = [0x2E, 0x2C, 0x26];
    pub const MOTION_LONG: u32 = 0x28;
}

/// Disparo del jugador (`system::player_fire`).
pub mod player_fire {
    /// Tareas de los jugadores: si quien dispara no es una, el jugador es
    /// su padre (`+$0C`).
    pub const PLAYERS: [u32; 2] = [0x10_0440, 0x10_04E0];
    pub const PARENT: u32 = 0x0C;
    /// Del que dispara: contador de ráfaga y hacia dónde mira.
    pub const BURST: u32 = 0x7A;
    pub const FACING: u32 = 0x3A;
    pub const FACING_LEFT: u8 = 1 << 0;
    /// Del jugador: munición.
    pub const AMMO: u32 = 0x82;
    pub const X: u32 = 0x22;
    pub const Y: u32 = 0x24;
    pub const ATTR: u32 = 0x38;
    /// Bala de pistola: sonido, plantilla y altura sobre el jugador.
    pub const PISTOL_SOUND: u16 = 0x10F7;
    pub const PISTOL_BULLET: u32 = 0x03_093A;
    pub const PISTOL_HEIGHT: u16 = 0x1C;
    /// Lo que baja la bala al disparar agachado.
    pub const LOW_DROP: u16 = 0xC;
    /// De la bala: ángulo (en 1/32 de vuelta, × 8), ráfaga y desviación.
    pub const ANGLE: u32 = 0x98;
    pub const BULLET_BURST: u32 = 0x9A;
    pub const SPREAD: u32 = 0x9B;
    /// Ángulos: adelante, arriba, atrás y abajo.
    pub const ANGLE_RIGHT: u8 = 0;
    pub const ANGLE_UP: u8 = 8;
    pub const ANGLE_LEFT: u8 = 0x10;
    pub const ANGLE_DOWN: u8 = 0x18;
    /// Granadas: quedan en `+$80` del jugador; la tarea cuelga de
    /// `GRENADE_PARENT`, y sale `GRENADE_DX` delante (o detrás).
    pub const BOMBS: u32 = 0x80;
    pub const GRENADE_PARENT: u32 = 0x10_08A0;
    pub const GRENADE: u32 = 0x03_3346;
    pub const GRENADE_DX: u16 = 0x10;
    /// Llamarada: sonido, plantilla, altura sobre el jugador y lo que se
    /// adelanta en las diagonales (hacia atrás de donde mira).
    pub const FLAME_SOUND: u16 = 0x10F4;
    pub const FLAME: u32 = 0x03_08C2;
    pub const FLAME_HEIGHT: u16 = 0x14;
    pub const FLAME_AHEAD: u16 = 0x10;
    pub const ANGLE_DIAGONAL: u8 = 0x17;
    pub const STATUS: u32 = 0x8C;
    pub const ARM_BUSY: u8 = 1 << 2;
}

/// Seno y coseno (`system::trig`): tablas de palabras en 1/256, con el
/// ángulo en 1/256 de vuelta.
pub mod trig {
    pub const COS: u32 = 0x2C_07AC;
    pub const SIN: u32 = 0x2C_072C;
}

/// Bala de pistola del jugador (`system::bullet`).
pub mod bullet {
    /// Lo que deja el que dispara: ángulo (× 8), ráfaga y desviación; la
    /// bala los copia a su ángulo y su ráfaga.
    pub const ANGLE_SRC: u32 = 0x98;
    pub const BURST_SRC: u32 = 0x9A;
    pub const SPREAD: u32 = 0x9B;
    pub const ANGLE: u32 = 0x70;
    pub const ANGLE_MASK: u16 = 0xF8;
    pub const BURST: u32 = 0x7A;
    pub const FLAGS_6B: u32 = 0x6B;
    pub const BULLET_FLAG: u8 = 1 << 4;
    /// Del abuelo (el jugador): `+$6B` bit 6 = la bala gira con el brazo.
    pub const FOLLOW_FACING: u8 = 1 << 6;
    pub const ATTACKS: u32 = 0x4C;
    pub const ATTACK_LIST: u32 = 0x03_0804;
    pub const ATTR: u32 = 0x38;
    pub const ATTR_INIT: u16 = 0xD000;
    /// Destello (tarea hija), sprite y guion por ángulo (`NO_SCRIPT` = ninguno).
    pub const FLASH: u32 = 0x07_7A96;
    pub const GFX_FIELD: u32 = 0x9E;
    pub const GFX: u16 = 2;
    pub const SCRIPTS: u32 = 0x29_D452;
    /// Variantes `$0308B0` / `$0308C2`: su tabla de guiones y el gráfico
    /// de la del vehículo.
    pub const SCRIPTS_ALT: u32 = 0x29_D4D2;
    pub const GFX_VEHICLE: u16 = 0x1D6;
    pub const NO_SCRIPT: u32 = 0xFFFF_FFFF;
    /// Desviación según la configuración (`CONTROL_LAYOUT`) o el modo de
    /// pruebas (`DEBUG` bit 1).
    pub const DEBUG: u32 = 0x10_0000;
    pub const DEBUG_SPREAD: u8 = 1 << 1;
    pub const CONTROL_LAYOUT: u32 = 0x10_6F2A;
    /// Velocidad y el paso atrás de las ráfagas impares.
    pub const SPEED: i16 = 0x800;
    pub const NUDGE: i16 = -0x800;
    pub const VEL_X: u32 = 0x28;
    pub const VEL_Y: u32 = 0x2A;
    pub const PARENT: u32 = 0x0C;
    pub const FACING: u32 = 0x3A;
    pub const FACING_LEFT: u8 = 1 << 0;
    /// `+$13`: bit 6 (se pone cada frame) y bit 1 (ha dado a algo).
    pub const FLAGS_13: u32 = 0x13;
    pub const FRESH: u8 = 1 << 6;
    pub const HIT: u8 = 1 << 1;
    /// Vida en frames.
    pub const FUSE: u32 = 0x76;
    /// Rutinas: bucle, explosión en el suelo y en el aire, baja.
    pub const RUN: u32 = 0x03_0B28;
    pub const EXPLODE_GROUND: u32 = 0x03_0BB6;
    pub const EXPLODE_AIR: u32 = 0x03_0BD6;
    pub const GONE: u32 = 0x03_0BF6;
    pub const GROUND_FX: u32 = 0x03_1C72;
    pub const AIR_FX: u32 = 0x03_1CCA;
    /// Superficies en las que se pierde sin explotar: tipo `VANISH_KIND`
    /// o celda con los bits altos a medias.
    pub const VANISH_KIND: u8 = 0x38;
    pub const CELL_HIGH: u8 = 0xC0;
    pub const MARGINS: u32 = 0x03_08A8;
}

/// Granada de mano del jugador (`system::grenade`).
pub mod grenade {
    pub const X: u32 = 0x22;
    pub const Y: u32 = 0x24;
    pub const VEL_X: u32 = 0x28;
    pub const VEL_Y: u32 = 0x2A;
    pub const ACC_X: u32 = 0x2C;
    pub const ACC_Y: u32 = 0x2E;
    pub const ATTR: u32 = 0x38;
    pub const FACING: u32 = 0x3A;
    pub const FACING_LEFT: u8 = 1 << 0;
    pub const PARENT: u32 = 0x0C;
    pub const FLAGS_13: u32 = 0x13;
    pub const HIT: u8 = 1 << 1;
    pub const BURST: u8 = 1 << 3;
    pub const ARMED: u32 = 0x3B;
    pub const HITBOX_CB: u32 = 0x48;
    pub const ATTACKS: u32 = 0x4C;
    pub const BOUNCES: u32 = 0x5C;
    pub const FLAGS_6B: u32 = 0x6B;
    pub const FLAGS_6B_BIT4: u8 = 1 << 4;
    pub const START_Y: u32 = 0x82;
    pub const NONE: u32 = 0xFFFF_FFFF;
    /// Lanzamiento: velocidades, gravedad, atributos, guion, ataques y
    /// gráficos; sale `RAISE` por debajo del padre.
    pub const THROW_VEL: (u16, u16) = (0xFAE2, 0x0CCB);
    pub const THROW_GRAVITY: u16 = 0xFEFA;
    pub const ATTR_THROWN: u16 = 0xD000;
    pub const SCRIPT: u32 = 0x28_D3E2;
    pub const ATTACKS_FLY: u32 = 0x28_D152;
    pub const GFX: [u16; 3] = [0x14C, 1, 0x14C];
    pub const POOL: u32 = 0x10_08A0;
    pub const RAISE: u16 = 0x10;
    /// Bote: velocidades, gravedad y lo que baja.
    pub const BOUNCE_VEL: (u16, u16) = (0xFCE0, 0x03C0);
    pub const BOUNCE_GRAVITY: u16 = 0xFF88;
    pub const BOUNCE_DROP: u16 = 8;
    /// Por debajo de esta velocidad vertical frena en horizontal.
    pub const DRAG_BELOW: i16 = -0x100;
    /// Con este flag la granada se para al tocar suelo (otro movimiento).
    pub const STOP_MODE: u32 = 0x10_81AE;
    /// Se pierde por arriba o por la derecha.
    pub const MIN_Y: i16 = 0x100;
    pub const MAX_X: u16 = 0x150;
    /// Explosión y estallido: sonido, gráficos, guiones y ataques.
    pub const SOUND: u16 = 0x1027;
    pub const EXPLODE_GFX: u16 = 0x178;
    pub const EXPLODE_SCRIPT: u32 = 0x29_E76C;
    pub const EXPLODE_ATTACKS: u32 = 0x28_D29A;
    pub const BURST_GFX: u16 = 0xD;
    pub const BURST_SCRIPT: u32 = 0x28_D52E;
    pub const BURST_ATTACKS: u32 = 0x28_D1F6;
    /// Rutinas.
    pub const FLY: u32 = 0x28_D91E;
    pub const BLAST: u32 = 0x28_D74E;
    pub const FADE: u32 = 0x28_D76E;
}

/// Estallidos y humo de corta vida (`system::burst`).
pub mod burst {
    pub const X: u32 = 0x22;
    pub const Y: u32 = 0x24;
    pub const PARENT: u32 = 0x0C;
    /// Gráficos que hereda del proyectil (`GFX_SHARED` se conserva; si no,
    /// cada efecto usa los suyos).
    pub const GFX_FIELD: u32 = 0x9E;
    pub const GFX_SHARED: u16 = 0x1D6;
    pub const GFX_GROUND: u16 = 4;
    pub const GFX_AIR: u16 = 2;
    pub const GFX_SMOKE: u16 = 8;
    /// Guiones: estallido en el suelo (dos al azar), en el aire, el otro
    /// estallido en el suelo, chispa y humo del tubo de escape.
    pub const GROUND_SCRIPTS: [u32; 2] = [0x29_DA80, 0x29_DB0E];
    pub const GROUND_B_SCRIPTS: [u32; 2] = [0x29_DB88, 0x29_DC16];
    pub const AIR_SCRIPT: u32 = 0x2D_D8B6;
    pub const SPARK_SCRIPT: u32 = 0x29_E46A;
    pub const EXHAUST_SCRIPT: u32 = 0x29_E440;
    /// Humo que cae del Slug: guion según `+$94` del dueño.
    pub const DROP_SCRIPTS: u32 = 0x03_19DC;
    pub const OWNER_DROP: u32 = 0x94;
    pub const LAYER_A: u32 = 0x34;
    pub const LAYER_B: u32 = 0x64;
    /// Dueño: bit 2 de `+$8D` (escape), puntero a la pose (`+$70`, con el
    /// desplazamiento del tubo en +8/+A) y el ángulo del cañón (`+$80`).
    pub const OWNER_FLAGS: u32 = 0x8D;
    pub const OWNER_EXHAUST: u8 = 1 << 2;
    pub const OWNER_POSE: u32 = 0x70;
    pub const OWNER_ANGLE: u32 = 0x80;
    pub const ANGLE_MAX: i16 = 0x20;
    pub const ANGLE_OVER: u16 = 0x40;
    pub const ANGLE_MIN: i16 = -0x20;
    pub const EXHAUST_RADIUS: i16 = 0x10;
    pub const SMOKE_SHIFT: u16 = 4;
    /// Bucle común.
    pub const RUN: u32 = 0x03_1A9C;
}

/// Escombros de la escena 3 (`system::debris`).
pub mod debris {
    /// Sólo salen en esta escena.
    pub const SCENE: u32 = 0x10_6ECE;
    pub const ONLY_SCENE: u8 = 3;
    /// Tarea: entrada, bucle, sprite y guion.
    pub const TASK: u32 = 0x03_D76E;
    pub const RUN: u32 = 0x03_D788;
    pub const GFX: u16 = 0x1BD;
    pub const SCRIPT: u32 = 0x03_D7A0;
    pub const X: u32 = 0x22;
    pub const Y: u32 = 0x24;
    pub const FACING: u32 = 0x3A;
    pub const FACING_LEFT: u8 = 1 << 0;
    /// Atributos: se quitan los bits de `!ATTR_KEEP` y se pone la capa.
    pub const ATTR: u32 = 0x38;
    pub const ATTR_KEEP: u16 = 0xFFE3;
    pub const LAYER_FRONT: u16 = 0x10;
    pub const LAYER_BACK: u16 = 0x04;
    /// Polvo al saltar del Slug: tarea, bucle, sprite, atributos y guion.
    pub const DUST_TASK: u32 = 0x03_D8FA;
    pub const DUST_RUN: u32 = 0x03_D92C;
    pub const DUST_GFX: u16 = 0x17C;
    pub const DUST_ATTR: u16 = 0x2000;
    pub const DUST_ATTR_KEEP: u16 = 0xFFE3;
    pub const DUST_ATTR_SET: u16 = 0x10;
    pub const DUST_SCRIPT: u32 = 0x2F_7D3A;
    /// Pareja de polvaredas del Slug: la tarea (que se acaba sola), y
    /// desplazamiento X, Y y si se voltea de cada una de las 10 variantes
    /// (`$031E5E` + el desplazamiento de `DUST_PAIR_ENTRIES`).
    pub const DUST_PAIR_TASK: u32 = 0x03_1E38;
    pub const DUST_PAIR: [(i16, i16, bool); 10] = [
        (-0x20, 0, false), (0x20, 0, true), (-0x20, -8, false), (0x20, -8, true), (-0x20, 8, false),
        (0x20, 8, true), (-0x13, -0xC, false), (0x13, -0xC, true), (-0x13, 0xC, false), (0x13, 0xC, true),
    ];
    pub const DUST_PAIR_ENTRIES: [u32; 10] =
        [0x03_1E5E, 0x03_1E76, 0x03_1E94, 0x03_1EB2, 0x03_1ED6, 0x03_1EF2, 0x03_1F16, 0x03_1F36, 0x03_1F5C, 0x03_1F7C];
    pub const FLIP: u32 = 0x3A;
}

/// Slug parado (`system::slug_idle`).
pub mod slug_idle {
    /// Guiones de la espera por índice de pendiente (0–4), juego A y B
    /// (`NO_SCRIPT` = ninguno).
    pub const SCRIPTS_A: u32 = 0x27_9614;
    pub const SCRIPTS_B: u32 = 0x27_9628;
    pub const NO_SCRIPT: u32 = 0xFFFF_FFFF;
    /// `+$20`: base y el índice de pendiente / 4.
    pub const BYTE_20: u32 = 0x20;
    pub const BYTE_20_BASE: u8 = 0x41;
    pub const ACC_X: u32 = 0x2C;
    pub const VEL_X: u32 = 0x28;
    pub const VEL_Y: u32 = 0x2A;
    pub const HANDLER_60: u32 = 0x60;
    pub const NO_HANDLER: u32 = 0xFFFF_FFFF;
    pub const HITBOX_CB: u32 = 0x48;
    pub const IDLE_HITBOX: u32 = 0x02_999E;
    /// Rutinas: el bucle parado y a dónde pasa (inclinado, salto, se monta
    /// un jugador, destruido, fuera de la pantalla).
    pub const IDLE: u32 = 0x02_DE18;
    pub const ANGLED: u32 = 0x02_DEB4;
    pub const JUMP: u32 = 0x02_E092;
    pub const HUNKER: u32 = 0x02_E3B0;
    pub const DEATH_EXPLODE: u32 = 0x02_DA38;
    pub const DEATH_START: u32 = 0x02_DCC0;
    pub const DEATH_FINISH: u32 = 0x02_DCBC;
    /// `+$8C` bit 5 (se quita al entrar) y `+$8D` bits 4 y 5 (se ponen).
    pub const FLAGS_8C: u32 = 0x8C;
    pub const BIT_8C_5: u8 = 1 << 5;
    pub const FLAGS_8D: u32 = 0x8D;
    pub const BIT_8D_4: u8 = 1 << 4;
    /// `+$8D` bit 5: lleva piloto / no se destruye todavía.
    pub const MANNED: u8 = 1 << 5;
    pub const FLAGS_13: u32 = 0x13;
    pub const DESTROYED: u8 = 1 << 0;
    /// `+$13` bits 4 y 5: las ruedas delantera y trasera están parejas.
    pub const LEVEL_BITS: u8 = (1 << 4) | (1 << 5);
    pub const RANK_TIMER: u32 = 0x10_6E92;
    /// Márgenes de «fuera de la pantalla» según el flag de un frame.
    pub const VIEW_FLAG: u32 = 0x10_6F4A;
    pub const MARGINS_FLAGGED: u32 = 0x27_964E;
    pub const MARGINS: u32 = 0x27_965A;
    /// Ruedas (delantera, trasera): Y, estado (bit 4 = suelo blando) y
    /// el contador `+$8E`.
    pub const WHEEL_FRONT: u32 = 0x74;
    pub const WHEEL_REAR: u32 = 0x78;
    pub const WHEEL_Y: u32 = 0x24;
    pub const WHEEL_STATE: u32 = 0x5A;
    pub const WHEEL_SOFT: u8 = 1 << 4;
    pub const WHEEL_PULSE: u32 = 0x8E;
    pub const WHEEL_GROUND: u32 = 0x9A;
    /// Diferencias de altura entre ruedas que cuentan como «a nivel».
    pub const LEVEL_DIFFS: [u16; 10] = [0, 1, 2, 8, 7, 9, 0x20, 0x1F, 0x21, 0x22];
    /// Posición con la Y de apoyo y zona de trabajo del movimiento.
    pub const X: u32 = 0x22;
    pub const BASE_Y: u32 = 0x82;
    pub const BASE_Y_MASK: u16 = 0x3FF;
    pub const SUB: u32 = 0x26;
    pub const WORK_POS: [u32; 2] = [0x10_6F38, 0x10_6F3A];
    pub const WORK_SUB: [u32; 2] = [0x10_6F3C, 0x10_6F3D];
    /// Montarse: escena cargada, recogida activa, caja de contacto con
    /// cada jugador y sus tareas.
    pub const SCENE_LOADED: u32 = 0x10_6ED3;
    pub const ENABLED: u32 = 0x10_6ED2;
    pub const BOARD_BOX: u32 = 0x03_EF9A;
    pub const PLAYERS: [u32; 2] = [0x10_0440, 0x10_04E0];
    /// Del jugador: `+$8C` bit 0 (montado), `+$8D` bits 1 (en el aire) y 5
    /// (bajando), velocidad vertical y su `+$68`.
    pub const P_STATUS: u32 = 0x8C;
    pub const P_RIDING: u8 = 1 << 0;
    pub const P_AIR: u32 = 0x8D;
    pub const P_IN_AIR: u8 = 1 << 1;
    pub const P_DESCENDING: u8 = 1 << 5;
    pub const P_VEL_Y: u32 = 0x2A;
    pub const PILOT: u32 = 0x68;
    /// Del Slug: quién lo controla y el aviso de montado (`+$89` = `0xFF`).
    pub const CONTROLLER: u32 = 0x6D;
    pub const SLUG: u32 = 0x10_0580;
    pub const BOARDED: u32 = 0x89;
    pub const BOARDED_ON: u8 = 0xFF;
    /// Agacharse al montarse (`Slug_Hunker_02e3b0`): guiones por índice de
    /// pendiente, punteros de ataque, base de `+$20`, la tarea del brazo
    /// del jugador montado, la caja de golpe y su bucle.
    pub const HUNKER_SCRIPTS: u32 = 0x27_9600;
    pub const HUNKER_ATTACKS: u32 = 0x02_A024;
    pub const HUNKER_BYTE_20: u8 = 0x28;
    pub const ARM_OVERLAY: u32 = 0x03_D944;
    pub const HUNKER_LOOP: u32 = 0x02_E42A;
    /// `+$6B` bit 0 (se pone al agacharse) y `+$8C` bit 1 (se quita).
    pub const FLAGS_6B: u32 = 0x6B;
    pub const BIT_6B_0: u8 = 1 << 0;
    pub const BIT_8C_1: u8 = 1 << 1;
    /// Al acabar el guion: caja de golpe, `+$60` y la conducción.
    pub const DRIVE_HITBOX: u32 = 0x02_9A14;
    pub const DRIVE_HANDLER_60: u32 = 0x02_9600;
    pub const DRIVE: u32 = 0x02_E9C8;
    /// Sin suelo: caída; A+B: el jugador se baja.
    pub const FALL: u32 = 0x02_D02E;
    pub const EJECT: u32 = 0x02_BBF2;
    /// Rutina según el lado del golpe (0 = ninguno).
    pub const HIT_HANDLERS: u32 = 0x02_A060;
    /// `+$13` bit 3: golpeado (al destruirse sin piloto se pone también).
    pub const HIT: u8 = 1 << 3;
    /// Tick de daño: invulnerable (`+$45`), depuración (bit 3 = vida fija
    /// `HP_RESET`, bit 5 = `HP_DEBUG`), vida y el sonido del golpe.
    pub const INVULNERABLE: u32 = 0x45;
    pub const DEBUG: u32 = 0x10_0001;
    pub const DEBUG_HP_RESET: u8 = 1 << 3;
    pub const DEBUG_HP_BIG: u8 = 1 << 5;
    pub const HP: u32 = 0x66;
    pub const HP_RESET: u16 = 0x30;
    pub const HP_DEBUG: u16 = 0x300;
    pub const HIT_SOUND: u16 = 0x1089;
    /// Física E: con el bit 5 de `+$5A` la Y de apoyo baja `BASE_Y_DROP`.
    pub const FLAGS_5A: u32 = 0x5A;
    pub const BIT_5A_5: u8 = 1 << 5;
    pub const BASE_Y_DROP: u16 = 0x10;
    /// Máscara de jugadores: byte `+$96`/`+$97` según la paridad del frame.
    pub const PLAYER_MASK: u32 = 0x96;
    pub const MASK_BLOCKED: u8 = 0xF0;
    pub const FRAME_COUNTER: u32 = 0x10_6F28;
    /// Botones que bajan al jugador del Slug (A y B a la vez).
    pub const EJECT_BUTTONS: u8 = 0x30;
}

/// Slug (SV-001): la tarea fija `task::SLUG` y quien la pone en marcha.
pub mod slug {
    /// Rutinas con las que el Slug no cuenta como vivo (libre, dándose de
    /// baja, en espera, destruido).
    pub const INACTIVE_HANDLERS: [u32; 4] = [0xFFFF_FFFF, 0x52A, 0x400, 0x2_AE3E];
    /// Byte de la tarea que lo suelta con la variante (0–4; más se queda en 4).
    pub const DROP_KIND: u32 = 0x98;
    pub const DROP_KIND_MAX: u8 = 4;
    /// Rutina de entrada de cada variante (`Slug_DropVariant0_02aef8`…).
    pub const DROP_VARIANTS: [u32; 5] = [0x02_AEF8, 0x02_AF1C, 0x02_AF40, 0x02_AF64, 0x02_AF88];
    /// Donde el Slug guarda la Y de quien lo suelta.
    pub const DROP_Y: u32 = 0x82;
    pub const Y: u32 = 0x24;
    /// Gráficos de la entrada en escena (el segundo juego siempre es 1).
    pub const DROP_GFX: u16 = 3;
    pub const EXTRA_GFX: u16 = 1;
    /// Tarea que acompaña a cada variante (`DROP_VARIANTS` en el mismo orden).
    pub const DROP_COMPANIONS: [u32; 5] = [0x04_59A8, 0x04_59C0, 0x04_59D8, 0x04_59F0, 0x04_5ABE];
    /// Tareas comunes de la entrada: (rutina) × 2, guion, cajas de golpe y
    /// bucle de la caída.
    pub const DROP_TASKS: [u32; 2] = [0x03_1FCA, 0x02_FB9A];
    pub const DROP_ANIM: u32 = 0x27_92B0;
    pub const DROP_HITBOX: u32 = 0x02_9698;
    pub const DROP_FALL: u32 = 0x02_B004;
    /// Velocidades (palabras) y bytes que la entrada pone a cero.
    pub const DROP_CLEAR_WORDS: [u32; 4] = [0x28, 0x2A, 0x2C, 0x2E];
    pub const DROP_CLEAR_BYTES: [u32; 3] = [0x26, 0x27, 0x8E];
    pub const HITBOX: u32 = 0x4C;
    /// Atributos del sprite.
    pub const ATTR: u32 = 0x38;
    pub const ATTR_INIT: u16 = 0x8000 | 0xC;
    pub const ATTR_DROP: u16 = 0x2;
    /// Campos que prepara `Slug_Init_02a0f8`.
    pub const TIMER_1C: u32 = 0x1C;
    pub const TIMER_1C_INIT: u16 = 0x1E;
    pub const TIMER_1C_ADD: u16 = 0x74;
    pub const FIELD_16: u32 = 0x16;
    pub const FIELD_14: u32 = 0x14;
    pub const FLAGS_12: u32 = 0x12;
    pub const FLAGS_13: u32 = 0x13;
    pub const FLAGS_5B: u32 = 0x5B;
    pub const FLAGS_6B: u32 = 0x6B;
    pub const LEVELS: [u32; 2] = [0x32, 0x33];
    pub const BYTE_3A: u32 = 0x3A;
    pub const TIMER_30: u32 = 0x30;
    pub const BYTE_8C: u32 = 0x8C;
    pub const HANDLER_60: u32 = 0x60;
    pub const HANDLER_60_INIT: u32 = 0x02_95B4;
    pub const PLAYER_MASK: u32 = 0x96;
    pub const PLAYER_MASK_INIT: u16 = 0xF0F0;
    pub const FIELD_66: u32 = 0x66;
    pub const FIELD_66_INIT: u16 = 0x30;
    pub const DAMAGE_IDX: u32 = 0x88;
    pub const DAMAGE: u32 = 0x86;
    pub const CLEAR_WORDS: [u32; 2] = [0x94, 0x36];
    pub const CLEAR_BYTES: [u32; 2] = [0x92, 0x8F];
    pub const BYTE_90: u32 = 0x90;
    pub const BYTE_90_INIT: u8 = 0xA;
    /// Tarea de las cajas y su cadena de tres (rutinas en orden de alta y
    /// el campo donde se guarda cada una).
    pub const SHADOW_TASK: u32 = 0x07_773E;
    pub const CHAIN: [(u32, u32); 3] = [(0x03_010C, 0x7C), (0x03_0068, 0x74), (0x03_00BA, 0x78)];
    /// Enlaces de la cadena (cada eslabón apunta a los tres).
    pub const CHAIN_LINKS: [u32; 3] = [0x70, 0x74, 0x78];
    pub const CHAIN_KIND: u32 = 0x98;
    pub const FIELD_98_LONG: u32 = 0x98;
    pub const FIELD_98_INIT: u32 = 0x1_0000;
    /// Caída de la entrada en escena: invulnerabilidad (frames, `+$45`)
    /// mientras cae y mientras baja hasta el suelo.
    pub const INVULNERABLE: u32 = 0x45;
    pub const DROP_INVULNERABLE: u8 = 0x5A;
    pub const DESCEND_INVULNERABLE: u8 = 0x46;
    /// Velocidades y aceleraciones (palabras).
    pub const VEL_X: u32 = 0x28;
    pub const VEL_Y: u32 = 0x2A;
    pub const ACC_X: u32 = 0x2C;
    /// Velocidad vertical fija mientras cae y tope al bajar.
    pub const DROP_VEL_Y: u16 = 0xFC00;
    pub const MAX_FALL: u16 = 0x400;
    /// Atributos del sprite en el aire: se quitan los bits de `!AIR_ATTR_KEEP`
    /// y se ponen los de `AIR_ATTR`.
    pub const AIR_ATTR_KEEP: u16 = 0xFFE3;
    pub const AIR_ATTR: u16 = 0x9FE0 | 0xC;
    /// Por encima de esta Y la caída pasa a la bajada.
    pub const DESCEND_Y: i16 = 0x180;
    /// Rutinas: bajada (entrada y bucle) y el estado parado del Slug.
    pub const DROP_DESCEND: u32 = 0x02_B05E;
    pub const DESCEND_RUN: u32 = 0x02_B09E;
    pub const IDLE_ENTER: u32 = 0x02_DD20;
    /// Lo que la bajada suma a la Y de apoyo cada vez que entra.
    pub const DESCEND_STEP: u16 = 8;
    /// Las tres ruedas (eslabones de la cadena): delantera, trasera y la
    /// del medio. De cada una: Y del suelo bajo ella, si lo pisa, su X/Y y
    /// el bit de pared.
    pub const WHEELS: [u32; 3] = [0x74, 0x78, 0x7C];
    pub const WHEEL_X: u32 = 0x22;
    pub const WHEEL_Y: u32 = 0x24;
    pub const WHEEL_GROUND: u32 = 0x9A;
    pub const WHEEL_ON_GROUND: u32 = 0x84;
    pub const WHEEL_STATE: u32 = 0x5A;
    pub const WHEEL_HIT_WALL: u8 = 1 << 5;
    pub const WHEEL_Y_MASK: u16 = 0x1FF;
    /// Ruedas que tienen que pisar para que cuente como apoyado.
    pub const WHEELS_TO_LAND: u8 = 2;
    /// Inclinación del terreno (la última que se ha aplicado al sprite) y
    /// su tope.
    pub const ANGLE: u32 = 0x80;
    pub const MAX_SLOPE: i16 = 0x40;
    /// Índice de animación por pendiente y umbrales del producto de la
    /// pendiente (fuerte, suave).
    pub const ANIM_IDX: u32 = 0x94;
    pub const STEEP_SLOPE: i16 = 0x384;
    pub const GENTLE_SLOPE: i16 = 0x5A;
    /// Piloto (0 = nadie, 1/2 = jugador) y el último que lo ha llevado.
    pub const PILOT: u32 = 0x68;
    pub const LAST_PILOT: u32 = 0x85;
    pub const TURN_TIMER: u32 = 0x84;
    /// Contador que baja hasta 0 cada frame.
    pub const PULSE: u32 = 0x8E;
    pub const X: u32 = 0x22;
    pub const FLAGS_8D: u32 = 0x8D;
    pub const AIRBORNE: u8 = 1 << 1;
    /// `+$12` bit 4: se está pulsando izquierda o derecha.
    pub const STEERING: u8 = 1 << 4;
    /// `+$12` bit 3: lo pone el guion de daños.
    pub const DAMAGE_BIT: u8 = 1 << 3;
    /// `+$13` bit 0: no se actualiza el sprite de daños.
    pub const NO_DAMAGE_SPRITE: u8 = 1 << 0;
    /// `+$5B` bit 3: apoyado sobre las ruedas.
    pub const ON_WHEELS: u8 = 1 << 3;
    /// `+$6B` bit 6: sondea el suelo como cualquier entidad.
    pub const PLAIN_FLOOR: u8 = 1 << 6;
    /// Guion del sprite de daños (sólo con la vida a 1 o menos).
    pub const DAMAGE_SCRIPT: u32 = 0x02_F886;
    pub const DAMAGE_HP: u16 = 1;
    pub const FIELD_14_SRC: u32 = 0x14;
    /// Tabla inclinación → índice de sprite y guiones de cada sprite
    /// (`NO_SCRIPT` = ninguno), y el tope del índice.
    pub const ANGLE_SPRITES: u32 = 0x2B_0BEE;
    pub const ANGLE_SCRIPTS: u32 = 0x2B_0C30;
    pub const NO_SCRIPT: u32 = 0xFFFF_FFFF;
    pub const ANGLE_MAX: u16 = 0x40;
    pub const ANGLE_CENTER: i16 = 0x20;
    pub const BYTE_20: u32 = 0x20;
    pub const BYTE_20_TILT: u8 = 0x14;
    /// Orugas (tarea hija): entrada y bucle (normal y con el Slug sin
    /// sprite de daños), sus cuatro sprites, el fotograma (0, 2, 4, 6) y
    /// el campo de la tarea del que sale el sprite de cada fotograma.
    pub const TRACK_START: u32 = 0x02_FB9A;
    pub const TRACK_RUN: u32 = 0x02_FBCC;
    pub const TRACK_RUN_PLAIN: u32 = 0x02_FC6A;
    pub const TRACK_GFX: [u16; 4] = [0x196, 0x197, 0x198, 0x199];
    pub const TRACK_FRAME: u32 = 0x70;
    pub const TRACK_FRAME_MASK: u16 = 6;
    pub const TRACK_FIELDS: [u32; 4] = [0x16, 0x18, 0x1A, 0x1C];
    /// Con más vida que esto las orugas no se mueven.
    pub const TRACK_MAX_HP: u16 = 0x10;
    /// Cursor del guion y última orden de fotograma (se copian del Slug;
    /// `NO_SPRITE` en la palabra alta = sin orden).
    pub const SCRIPT_CURSOR: u32 = 0x3C;
    pub const LAST_SPRITE: u32 = 0x40;
    pub const NO_SPRITE: u16 = 0xFFFF;
    /// Las orugas van una capa por debajo del chasis.
    pub const TRACK_DEPTH: u16 = 2;
    /// Rutina del Slug destruido: entonces las orugas se dan de baja.
    pub const DESTROYED: u32 = 0x02_DC5C;
}

/// Iconos que siguen a una entidad (el «POW» sobre el Slug, la burbuja).
pub mod icons {
    /// Sprite, guion (el del POW cambia en la escena `POW_ALT_SCENE`) y
    /// altura sobre el dueño de cada icono.
    pub const POW_GFX: u16 = 0x1AD;
    pub const POW_SCRIPT: u32 = 0x03_2292;
    pub const POW_ALT_SCRIPT: u32 = 0x03_2286;
    pub const POW_ALT_SCENE: u8 = 5;
    pub const POW_HEIGHT: u16 = 0x28;
    pub const POW_RUN: u32 = 0x03_2010;
    /// Márgenes con los que el POW mira si se ve.
    pub const POW_MARGINS: u32 = 0x03_2070;
    pub const BUBBLE_GFX: u16 = 0x1B0;
    pub const BUBBLE_SCRIPT: u32 = 0x03_232A;
    pub const BUBBLE_HEIGHT: u16 = 0x38;
    pub const BUBBLE_RUN: u32 = 0x03_20AC;
    /// Rutina que da de baja el icono.
    pub const GONE: u32 = 0x03_21BC;
    pub const SCENE: u32 = 0x10_6ECE;
    pub const SCENE_LOADED: u32 = 0x10_6ED3;
    /// Campos: altura, atributos (`HIDDEN` hasta el primer frame) y flags.
    pub const HEIGHT: u32 = 0x5C;
    pub const ATTR: u32 = 0x38;
    pub const HIDDEN: u16 = 0xFFFF;
    pub const X: u32 = 0x22;
    pub const Y: u32 = 0x24;
    pub const FLAGS_12: u32 = 0x12;
    pub const FLAGS_12_SET: u8 = 1 << 6;
    /// Del dueño: `+$8D` bit 5 = el POW se muestra; `+$13` bit 0 = se acaba.
    pub const OWNER_FLAGS: u32 = 0x8D;
    pub const OWNER_SHOW: u8 = 1 << 5;
    pub const OWNER_FLAGS_13: u32 = 0x13;
    pub const OWNER_DONE: u8 = 1 << 0;
    /// Indicador «1UP»/«2UP»: dueño esperado, (gráfico, guion, altura).
    pub const PLAYER_OWNER: [u32; 2] = [0x10_0440, 0x10_04E0];
    pub const PLAYER: [(u16, u32, u16); 2] = [(0xA2, 0x03_21C2, 0x30), (0xA3, 0x03_2224, 0x38)];
    pub const PLAYER_RUN: u32 = 0x03_2180;
}

/// Torreta del Slug (y de otros vehículos): sigue al chasis, apunta con
/// el mando del piloto según una de las cinco tablas de dirección y
/// dispara ráfagas.
pub mod turret {
    /// Tabla de ángulos de cada variante (16 entradas de 4 bytes, una por
    /// dirección del mando; negativo = ninguna) y ángulo inicial. La
    /// variante `SNAP` gira deprisa y recuerda el último ángulo pedido.
    pub const TABLES: [u32; 5] = [0x28_9504, 0x28_9544, 0x28_9584, 0x28_95C4, 0x28_9604];
    pub const START_ANGLE: [u16; 5] = [0x4000, 0x8000, 0xC000, 0, 0];
    pub const SNAP: usize = 4;
    pub const TRACK: u32 = 0x04_5A38;
    pub const SNAP_TRACK: u32 = 0x04_5B02;
    /// Campos.
    pub const VARIANT: u32 = 0x70;
    pub const TABLE: u32 = 0x74;
    pub const ANGLE: u32 = 0x34;
    pub const TURN_SPEED: u32 = 0x78;
    pub const LAST_TARGET: u32 = 0x7E;
    pub const STICK: u32 = 0x71;
    pub const SHOTS: u32 = 0x7C;
    pub const SHOT_DELAY: u32 = 0x7D;
    pub const RECOIL: u32 = 0x3B;
    pub const RECOIL_FRAMES: u8 = 5;
    pub const SHOT_INDEX: u32 = 0x81;
    pub const SHOT_COUNT: u32 = 0x80;
    pub const PILOT: u32 = 0x68;
    pub const CONTROLLER: u32 = 0x6D;
    pub const NONE: u8 = 0xFF;
    pub const TIMER_59: u32 = 0x59;
    pub const ATTR: u32 = 0x38;
    pub const X: u32 = 0x22;
    pub const Y: u32 = 0x24;
    pub const FLIP: u32 = 0x3A;
    pub const PALETTE: u32 = 0x14;
    /// Reducción vertical y horizontal.
    pub const SHRINK: [u32; 2] = [0x33, 0x32];
    pub const GFX: u32 = 0x3C;
    pub const FLAGS_8C: u32 = 0x8C;
    pub const FLAGS_8C_FIRED: u8 = 1 << 2;
    pub const FLAGS_8C_HIT: u8 = 1 << 3;
    /// Sprite: el genérico y el de los jugadores.
    pub const GFX_DEFAULT: u16 = 2;
    pub const GFX_PLAYER: u16 = 0x1D6;
    /// Del dueño: registro del soporte (puntero a desplazamiento base y
    /// desplazamiento propio en `+4`).
    pub const OWNER_MOUNT: u32 = 0x70;
    /// Giro: aceleración por frame y, en la variante rápida, la de lejos.
    pub const TURN_ACCEL: u16 = 0x100;
    pub const SNAP_ACCEL: u16 = 0x200;
    pub const SNAP_ACCEL_FAR: u16 = 0x800;
    pub const SNAP_FAR: u16 = 0x5000;
    /// Fotogramas por sector de ángulo (×3) y retroceso (×4).
    pub const FRAMES: u32 = 0x28_9204;
    /// Mando.
    pub const FIRE: u8 = 0x10;
    pub const STICK_MASK: u8 = 0x0F;
    pub const BURST: u8 = 3;
    pub const FIRE_SOUND: u16 = 0x108C;
    pub const PLAYERS: [u32; 2] = [0x10_0440, 0x10_04E0];
    pub const PAD_PTR: u32 = 0x72;
    pub const PAD_HELD: u32 = 2;
    pub const PAD_PRESSED: u32 = 3;
    pub const SCRIPTED_INPUT_ACTIVE: u32 = 0x10_E39D;
    pub const PAD_LIVE_P1: u32 = 0x10_E200;
    pub const PAD_SCRIPTED: [u32; 2] = [0x10_E2E2, 0x10_E2E8];
    /// Disparo: bala, destello y punta del cañón por sector (x, y).
    pub const SHOT: u32 = 0x04_5FDE;
    pub const FLASH: u32 = 0x03_0C14;
    pub const MUZZLE: u32 = 0x29_D752;
    pub const SHOT_ANGLE: u32 = 0x98;
    pub const SHOT_SLOT: [u32; 2] = [0x9A, 0x9B];
    /// Tipos de tile en los que la punta del cañón aún deja salir el
    /// destello.
    pub const PASS_KINDS: [u8; 3] = [0x10, 0x20, 0x21];
    /// Bit de la máscara de jugadores del dueño por el que se pierde la
    /// torreta.
    pub const LOST_BIT: u8 = 8;
    pub const PLAYER_MASK: u32 = 0x96;
    pub const FRAME_COUNTER: u32 = 0x10_6F28;
    pub const EXPLODE: u32 = 0x04_5C20;
    /// Si el dueño desaparece o acaba: bala suelta en el almacén de
    /// enemigos.
    pub const ENEMY_ROOT: u32 = 0x10_0800;
    pub const LOOSE_SHOT: u32 = 0x04_5D3A;
    /// Bala suelta: atributos, impulso (X hacia donde mira, Y), gravedad,
    /// vida, guion, sonido al consumirse y bucle.
    pub const SHOT_ATTR: u16 = 0xD000;
    pub const SHOT_ATTR_KEEP: u16 = 0xFFE3;
    pub const SHOT_ATTR_SET: u16 = 0x10;
    pub const SHOT_VEL: (u16, u16) = (0x38, 0x384);
    pub const SHOT_GRAVITY: u16 = 0xFFEC;
    pub const SHOT_LIFE: u32 = 0x45;
    pub const SHOT_LIFE_FRAMES: u8 = 0x3C;
    pub const SHOT_SCRIPT: u32 = 0x29_D090;
    pub const SHOT_SOUND: u16 = 0x1022;
    pub const SHOT_FLY: u32 = 0x04_5D8C;
    pub const FACING_LEFT: u8 = 1 << 0;
    pub const VEL_X: u32 = 0x28;
    pub const VEL_Y: u32 = 0x2A;
    pub const ACC_X: u32 = 0x2C;
    pub const ACC_Y: u32 = 0x2E;
    pub const OWNER_FLAGS_13: u32 = 0x13;
    pub const OWNER_DONE: u8 = 1 << 0;
}

/// Escena de Game Over / Continue (la del attract sin créditos y la del fin
/// de partida): la tarea principal y las que crea.
pub mod game_over {
    /// Rutina de cada fase de la tarea principal.
    pub const BOOT: u32 = 0x08_F91A;
    pub const SPAWN: u32 = 0x08_F96A;
    pub const FALL: u32 = 0x08_FBA2;
    pub const LANDED: u32 = 0x08_FCCA;
    pub const SETTLE: u32 = 0x08_FD2E;
    pub const FINAL: u32 = 0x08_FD68;
    pub const WAIT_CREDIT: u32 = 0x08_FDAA;
    pub const CONTINUE: u32 = 0x08_FDF8;
    pub const PATH: u32 = 0x08_FE32;
    pub const HOLD: u32 = 0x08_FE9C;
    pub const END: u32 = 0x08_FEB6;
    pub const IDLE: u32 = 0x08_FECA;
    /// Escena y lista de entradas de sprite que carga.
    pub const SCENE: u8 = 10;
    pub const ENTRIES: u32 = 0x2F_4A32;
    pub const X: u32 = 0x22;
    pub const Y: u32 = 0x24;
    pub const Y_FRAC: u32 = 0x27;
    pub const VEL_Y: u32 = 0x2A;
    pub const START: (u16, u16, u16) = (0x90, 0x1C0, 0xFF00);
    /// Estado que miran las hijas (`+$20`, `+$21`).
    pub const STATE: [u32; 2] = [0x20, 0x21];
    pub const TIMER: u32 = 0x82;
    pub const FIELD_84: u32 = 0x84;
    pub const COUNT: u32 = 0x86;
    pub const ORIGIN: [u32; 2] = [0x88, 0x8A];
    pub const START_TIMER: u16 = 0xE;
    /// Fundido del principio (tipo, parámetro).
    pub const FADE: (usize, u16) = (1, 2);
    /// Por debajo de esta Y no se mueve; alturas de los golpes y del suelo.
    pub const MOVE_FLOOR: i16 = 0x140;
    pub const HIT_HIGH: u16 = 0x1A8;
    pub const HIT_LOW: u16 = 0x190;
    pub const GROUND: i16 = 0x178;
    /// Hijas de la escena: (rutina, copia la posición, parámetro `+$80`).
    pub const CHILD_PARAM: u32 = 0x80;
    pub const CHILD_TIMER: u32 = 0x82;
    pub const CONTINUE_TEXT: u32 = 0x09_1514;
    pub const FIGURES: [(u32, Option<u16>); 29] = [
        (0x09_0DB0, None), (0x09_0DE2, None),
        (0x09_0F00, Some(0)), (0x09_0F68, Some(0)), (0x09_0F00, Some(1)),
        (0x09_0E86, Some(0)), (0x09_0E86, Some(1)), (0x09_0E86, Some(2)), (0x09_0E86, Some(3)),
        (0x09_0E86, Some(4)), (0x09_0E86, Some(5)), (0x09_0E86, Some(6)), (0x09_0E86, Some(7)),
        (0x09_0E86, Some(8)), (0x09_0E86, Some(9)),
        (0x09_076E, None), (0x09_0728, None), (0x09_07B4, None), (0x09_07FA, None),
        (0x09_0840, None), (0x09_088E, None), (0x09_08D4, None), (0x09_091A, None),
        (0x09_0968, None), (0x09_09AE, None), (0x09_09F4, None), (0x09_0A3A, None),
        (0x09_0A80, None), (0x09_0AC6, None),
    ];
    /// Sonido del golpe y del final.
    pub const SOUND_HIT: u16 = 0x10DF;
    pub const SOUND_END: u16 = 0x10DE;
    /// Lo que sale en cada golpe (con su temporizador `+$82`).
    pub const HIT_HIGH_FX: ([u32; 3], u16) = ([0x09_05C2, 0x09_068C, 0x09_034E], 0x20);
    pub const HIT_LOW_FX: ([u32; 3], u16) = ([0x09_05C2, 0x09_06E8, 0x09_04E4], 0x1E);
    pub const LANDED_TIMER: u16 = 0x10;
    pub const LANDED_FX: [u32; 3] = [0x09_0630, 0x09_00E4, 0x09_016C];
    pub const SETTLE_TIMER: u16 = 0x18;
    /// Cámara cuyos sprites se ocultan al asentarse.
    pub const CAMERA: u32 = 0x10_6F6C;
    pub const FINAL_TIMER: u16 = 0x1B;
    pub const FINAL_MARK_AT: u16 = 3;
    pub const FINAL_MARK: u32 = 0x10_A2D1;
    pub const WAIT_TIMER: u16 = 0x10;
    /// Se espera a que este byte sea distinto de 0 (créditos para continuar).
    pub const CREDIT_READY: u32 = 0x10_A2CF;
    pub const CONTINUE_FX: [u32; 2] = [0x09_004C, 0x09_0CBA];
    pub const CONTINUE_MID: u32 = 0x08_FFF6;
    pub const PATH_TIMER: u16 = 0x11;
    pub const PATH_TABLE: u32 = 0x2F_4BF8;
    pub const PATH_LEN: i16 = 0x14;
    pub const PATH_FX: [u32; 2] = [0x08_FECC, 0x08_FF2C];
    pub const HOLD_TIMER: u16 = 0x96;
    /// Título «CONTINUE?» en la capa fija (dirección, texto, atributos).
    pub const TITLE: (u16, u32, u16) = (0x711B, 0x2F_4C98, 0x2300);
    /// Al acabar, la escena deja de esperar.
    pub const SCENE_WAIT: u32 = 0x10_6ED2;

    // --- letras de «GAME OVER» (14), comunes a `GO_Letter_V0_090728` …
    /// (rutina, gráficos, bits de prioridad en `ATTR`, estado inicial `+$21`,
    /// índice, entrada de paleta al golpe, gráficos al caer, espera, guion).
    pub const LETTERS: [(u32, u16, u16, u8, u16, u16, u16, u16, u32); 14] = [
        (0x09_0728, 0xC5, 0x1C, 0, 0, 0xB7, 0xBF, 0, 0x2F_4D9E),
        (0x09_076E, 0xC7, 0x18, 0, 1, 0xB9, 0xC1, 0, 0x2F_4DAA),
        (0x09_07B4, 0xC5, 0x14, 0, 2, 0xB7, 0xBF, 0, 0x2F_4E08),
        (0x09_07FA, 0xC7, 0x10, 0, 3, 0xB9, 0xC1, 0, 0x2F_4E14),
        (0x09_0840, 0xC5, 0x1C, 0xFF, 4, 0xB7, 0xBF, 0x20, 0x2F_4E42),
        (0x09_088E, 0xC5, 0x18, 0, 5, 0xB7, 0xBF, 0, 0x2F_4E4E),
        (0x09_08D4, 0xC5, 0x14, 0, 6, 0xB7, 0xBF, 0, 0x2F_4E5A),
        (0x09_091A, 0xC6, 0x10, 0xFF, 7, 0xB8, 0xC0, 0x20, 0x2F_4E66),
        (0x09_0968, 0xC5, 0x10, 0, 8, 0xB7, 0xBF, 0, 0x2F_4E72),
        (0x09_09AE, 0xC9, 0x0C, 0, 9, 0xBB, 0xC3, 0, 0x2F_4E7E),
        (0x09_09F4, 0xC8, 0x10, 0, 0xA, 0xBA, 0xC2, 0, 0x2F_4E8A),
        (0x09_0A3A, 0xC6, 0x10, 0, 0xB, 0xB8, 0xC0, 0, 0x2F_4E96),
        (0x09_0A80, 0xC5, 0x08, 0, 0xC, 0xB7, 0xBF, 0, 0x2F_4EA2),
        (0x09_0AC6, 0xC5, 0x04, 0, 0xD, 0xB7, 0xBF, 0, 0x2F_4EAE),
    ];
    pub const LETTER_ATTR: u16 = 0x4000;
    pub const ATTR: u32 = 0x38;
    pub const ATTR_PRIO_MASK: u16 = 0xFFE3;
    pub const LETTER_RUN: u32 = 0x09_0B18;
    pub const LETTER_INDEX: u32 = 0x80;
    pub const LETTER_HIT_GFX: u32 = 0x7C;
    pub const LETTER_FALL_GFX: u32 = 0x7E;
    pub const LETTER_WAIT: u32 = 0x86;
    pub const LETTER_TIMER: u32 = 0x82;
    /// Posición de cada letra: base + desplazamiento (por índice y fase de
    /// la madre `+$84`) + posición de la madre.
    pub const LETTER_BASE: u32 = 0x2F_4A46;
    pub const LETTER_OFFSETS: u32 = 0x2F_4A7C;
    pub const PHASE: u32 = 0x84;
    pub const SPRITE_ENTRY: u32 = 0x14;
    /// Animación de paleta al golpe (duración y modo).
    pub const LETTER_PAL_ANIM: (u16, u8) = (1, 2);
    pub const LEVELS: [u32; 2] = [0x32, 0x33];
    pub const LETTER_DIM: u8 = 0xFC;
    pub const LETTER_FADE_STEP: u8 = 0x12;
    pub const LETTER_FADE_END: u8 = 0x20;
    /// La letra de índice `GLOW_LETTER` lanza el brillo.
    pub const GLOW_LETTER: u16 = 5;
    pub const GLOW: u32 = 0x09_0C30;
    pub const GLOW_RUN: u32 = 0x09_0C8E;
    pub const GLOW_GFX: u16 = 0xB6;
    pub const GLOW_OFFSET: (u16, u16) = (0x6A, 0x30);
    pub const GLOW_LEVEL: u8 = 0x15;
    pub const GLOW_FADE: (usize, u16) = (2, 2);
    pub const GLOW_ATTR: u16 = 0xE000;
    pub const GLOW_ANIM: u32 = 0x2F_5338;
    pub const GLOW_RATE: u32 = 0x82;
    pub const GLOW_WAIT: u32 = 0x86;
    pub const GLOW_DRIFT_RUN: u32 = 0x09_0D2E;
    /// Destino relativo a la posición de partida y desplazamiento inicial.
    pub const GLOW_DRIFT_TARGET: (u16, u16) = (0x10, 0x40);
    pub const GLOW_DRIFT_START: (u16, u16) = (8, 0x58);
    pub const GLOW_DRIFT_TARGET_POS: [u32; 2] = [0x8C, 0x8E];
    pub const GLOW_DRIFT_FADE: (usize, u16) = (3, 1);
    pub const GLOW_DRIFT_DIM: u8 = 0x10;
    /// Al bajar el nivel de este valor avisa a la madre y se da de baja.
    pub const GLOW_DRIFT_END: u8 = 0x1F;

    // --- figuras, piezas y fondos que se mueven
    /// Figuras grandes: (gráficos, atributos, prioridad, guion).
    pub const FIGURE_A: (u16, u16, Option<u16>, u32) = (0xC4, 0x2000, None, 0x2F_4EBA);
    pub const FIGURE_B: (u16, u16, Option<u16>, u32) = (0xC4, 0x0000, Some(0), 0x2F_4EEC);
    pub const FIGURE_RUN: u32 = 0x09_0E22;
    pub const FIGURE_HIT_GFX: u16 = 0xB5;
    /// Rutina de baja (`$090E7E`) a la que pasan al final.
    pub const FINISH: u32 = 0x09_0E7E;
    /// Piezas: posición = tabla por índice `+$80` + madre.
    pub const PART: (u16, u16, u32) = (0xBC, 0xBC, 0x2F_4EF8);
    pub const PART_RUN: u32 = 0x09_0EC6;
    pub const PART_OFFSETS: u32 = 0x2F_4A4A;
    /// Fondos que se desplazan: (gráficos, estado `+$21`, prioridad,
    /// velocidad X, tabla de X inicial, guion).
    pub const SCROLLERS: [(u16, u8, u16, u16, u32, u32); 2] = [
        (0xC4, 0, 0x10, 0x200, 0x2F_4A72, 0x2F_4ED4),
        (0xC4, 1, 0x0C, 0x180, 0x2F_4A78, 0x2F_4EE0),
    ];
    pub const SCROLLER_RUN: u32 = 0x09_0FCE;
    pub const SCROLLER_WAIT: u16 = 0x10;
    pub const SCROLLER_Y_OFFSET: u16 = 0x10;
    pub const SCROLLER_DIM_FROM: i8 = -0x60;
    pub const SCROLLER_DIM_X: i16 = 0x20;
    pub const SCROLLER_WRAP: (i16, u16) = (0x110, 0x10);
    pub const VEL_X: u32 = 0x28;
    pub const X_FRAC: u32 = 0x26;
    pub const GRAVITY: u32 = 0x2E;
    // --- chispas
    pub const PARTICLE: u32 = 0x09_1130;
    pub const PARTICLE_RUN: u32 = 0x09_128A;
    pub const PARTICLE_GFX: u16 = 0xE0;
    pub const PARTICLE_POS: (u16, u16) = (0x20, 0x128);
    /// Dos clases: (velocidad X, gravedad, guion normal); una de cada cuatro
    /// usa `PARTICLE_RARE`.
    pub const PARTICLE_KINDS: [(u16, u16, u32); 2] = [(0x300, 0x90, 0x2F_51D0), (0x200, 0xA0, 0x2F_512A)];
    pub const PARTICLE_RARE: u32 = 0x2F_5276;
    pub const PARTICLE_VEL_Y: u16 = 0xC00;
    pub const PARTICLE_ATTR: u16 = 0x2000;
    pub const PARTICLE_FADE: u8 = 6;
    pub const PARTICLE_FLOOR: i16 = 0x120;

    // --- sprites que entran deslizándose / siguen a la madre y piezas
    /// Deslizantes (derecha, izquierda): (gfx, X, Y, desplazamiento +$8C,
    /// paso +$28, columna de la tabla +$80, atributo, guion).
    pub const SLIDERS: [(u16, u16, u16, u16, u16, u16, u16, u32); 2] = [
        (0xD9, 0x130, 0x180, 0x00D8, 0xFFE8, 0, 0xD000, 0x2F_503E),
        (0xDA, 0x020, 0x138, 0xFF28, 0x0018, 2, 0xE000, 0x2F_504A),
    ];
    pub const SLIDE: u32 = 0x08_FF88;
    pub const SLIDE_OFFSET: u32 = 0x8C;
    pub const SLIDE_STEP: u32 = 0x28;
    pub const SLIDE_COLUMN: u32 = 0x80;
    /// Tabla del rebote al llegar (4 bytes por paso; columna X en +$80).
    pub const SLIDE_TABLE: u32 = 0x2F_4C48;
    pub const SLIDE_STEPS: i16 = 0x14;
    pub const SLIDE_MARK: u8 = 6;
    pub const SOUND_SLIDE: u16 = 0x10E0;
    /// Seguidores de la madre: (gfx, dX, dY, sin animar paleta, atributo,
    /// sonido, guion).
    pub const FOLLOWERS: [(u16, u16, u16, u16, u16, Option<u16>, u32); 2] = [
        (0xDB, 0x10, 0x40, 0, 0x2000, Some(0x10E1), 0x2F_5056),
        (0xDD, 0x00, 0x20, 1, 0x0000, None, 0x2F_5062),
    ];
    pub const FOLLOW: u32 = 0x09_0098;
    pub const FOLLOW_OFFSET: [u32; 2] = [0x8C, 0x8E];
    pub const FOLLOW_NO_PAL: u32 = 0x80;
    pub const FOLLOW_PAL_DONE: u32 = 0x84;
    /// Animación de paleta del seguidor: (primer paso, frames, modo).
    pub const FOLLOW_PAL: (u16, u16, u8) = (0xDE, 2, 1);
    /// Piezas A y B: (gfx, X, Y, atributo, guion, rutina del bucle).
    pub const PROP_A: (u16, u16, u16, u16, u32, u32) = (0xBE, 0xAE, 0x148, 0x4000, 0x2F_5328, 0x09_0128);
    pub const PROP_B: (u16, u16, u16, u16, u32, u32) = (0xBD, 0xAC, 0x148, 0x8000, 0x2F_531C, 0x09_01B6);
    pub const PROP_C: (u16, u16, u16, u16, u32, u32) = (0xFF, 0x98, 0x198, 0xE000, 0x2F_5368, 0x09_033C);
    pub const PROP_C_TASK: u32 = 0x09_02FE;
    pub const PROP_STEPS: i16 = 0x10;
    pub const PROP_A_TABLE: u32 = 0x2F_4B68;
    pub const PROP_B_TABLE: u32 = 0x2F_4BA8;
    pub const PROP_B_TABLE2: u32 = 0x2F_4BE8;
    pub const PROP_B_STEPS2: i16 = 4;
    /// En este paso de la primera tabla se anima la paleta.
    pub const PROP_B_FLASH_AT: i16 = 0xA;
    /// Paleta de la pieza B: (primer paso de la propia entrada, entradas
    /// altas y su primer paso).
    pub const PROP_B_PAL: (u16, [(u16, u16); 3]) = (0x100, [(0x10, 0x19B), (0x11, 0x19C), (0x12, 0x19D)]);
    pub const PROP_B_PAL_MODES: [(u16, u8); 2] = [(2, 3), (1, 6)];
    pub const PROP_B_GFX2: u16 = 0x101;
    pub const PROP_B_ENTRIES: u32 = 0x2F_4A38;

    // --- efectos de los golpes: zoom, temblor, destello, carteles
    /// Sprite de efectos: (gfx, atributo, prioridad).
    pub const FX_SPRITE: (u16, u16, u16) = (0xDF, 0xC000, 0x1C);
    pub const ZOOM_RUN: u32 = 0x09_03A4;
    pub const ZOOM_PATH: u32 = 0x09_0452;
    pub const ZOOM_PATH_RUN: u32 = 0x09_0474;
    pub const ZOOM_WAIT: u32 = 0x09_04C6;
    /// Centro (X, Y) y semitamaños iniciales (+$8C, +$8E).
    pub const ZOOM_CENTER: (u16, u16) = (0xEC, 0x14C);
    pub const ZOOM_SIZE: u32 = 0x8C;
    pub const ZOOM_SIZE_Y: u32 = 0x8E;
    pub const ZOOM_START_SIZE: u16 = 0xF0;
    pub const ZOOM_SCRIPT: u32 = 0x2F_50B4;
    /// A partir de este temporizador empieza a encogerse.
    pub const ZOOM_SHRINK_AT: i16 = 0x20;
    /// Por debajo de este tamaño baja (Y final, semitamaño Y).
    pub const ZOOM_DROP: (i16, u16, u16) = (0x24, 0x158, 0xC);
    pub const ZOOM_REST: (u16, u16) = (0xEC, 0x158);
    pub const ZOOM_SETTLE_FRAMES: i16 = 5;
    pub const ZOOM_PATH_TABLE: u32 = 0x2F_4B24;
    pub const ZOOM_PATH_STEPS: i16 = 9;
    pub const ZOOM_PATH_END: i16 = 0xD;
    pub const SHAKE: u32 = 0x09_04E4;
    pub const SHAKE_RUN: u32 = 0x09_0544;
    pub const SHAKE_POS: (u16, u16) = (0xC8, 0x140);
    pub const SHAKE_SCRIPT: u32 = 0x2F_50C4;
    pub const SHAKE_TABLE: u32 = 0x2F_4B48;
    pub const FLASH: u32 = 0x09_0592;
    pub const FLASH_RUN: u32 = 0x09_05BA;
    /// Destello: (gfx, X, Y, guion).
    pub const FLASH_SPRITE: (u16, u16, u16, u32) = (0xE0, 0xA8, 0x180, 0x2F_50FC);
    pub const BANNER_POS: (u16, u16) = (0xA0, 0x180);
    /// Carteles A–D: (gfx, atributo, guion, rutina del bucle).
    pub const BANNERS: [(u16, u16, u32, u32); 4] = [
        (0xE2, 0xF000, 0x2F_506E, 0x09_0600),
        (0xE2, 0xF000, 0x2F_506E, 0x09_066E),
        (0xE1, 0x0000, 0x2F_507A, 0x09_06CA),
        (0xE2, 0x0000, 0x2F_5086, 0x09_06CA),
    ];
}

/// Tareas de fundido de la paleta: cada alta crea una con su rutina y le
/// deja `d0` en `PARAM`.
pub mod fade {
    /// (entrada original, rutina de la tarea).
    pub const SPAWNERS: [(u32, u32); 4] =
        [(0x05_239E, 0x05_23EE), (0x05_23B2, 0x05_24AA), (0x05_23C6, 0x05_23FA), (0x05_23DA, 0x05_24B6)];
    pub const PARAM: u32 = 0x70;
    /// Rutinas de las tareas: (entrada, hacia blanco) en orden — salir de
    /// negro, salir de blanco, entrar en negro, entrar en blanco — y sus
    /// bucles.
    pub const FROM_BLACK: u32 = 0x05_23EE;
    pub const FROM_WHITE: u32 = 0x05_23FA;
    pub const TO_BLACK: u32 = 0x05_24AA;
    pub const TO_WHITE: u32 = 0x05_24B6;
    pub const LOOP_OUT: u32 = 0x05_2454;
    pub const LOOP_IN: u32 = 0x05_2514;
    /// Tabla (paso, frames por paso) para los parámetros 0–3; con otro
    /// parámetro el paso es 1 y el parámetro son los frames.
    pub const SPEEDS: u32 = 0x05_2570;
    pub const SPEEDS_LAST: u16 = 3;
    /// Campos de la tarea: frames por paso, cuenta, nivel acumulado y paso.
    pub const RATE: u32 = 0x70;
    pub const WAIT: u32 = 0x72;
    pub const DONE: u32 = 0x74;
    pub const STEP: u32 = 0x8C;
    pub const LEVEL_MAX: u16 = 0x1F;
    /// Modo de fundido que pone cada una (a negro = 1, a blanco = `$FF`).
    pub const MODE_BLACK: u8 = 1;
    pub const MODE_WHITE: u8 = 0xFF;
    /// `palette::FADE_READY`: `$FF` al acabar; 0 mientras se funde.
    pub const READY: u32 = 0x10_A2CF;
}

/// Listas de la misión que se renuevan cada frame (algunas tienen dos
/// copias que se alternan según la paridad del frame).
pub mod lists {
    /// Contador de frames (bit 0 = paridad).
    pub const FRAME: u32 = super::system::FRAME_COUNTER;
    /// Dos listas de plataformas (frames pares / impares); su recuento en `+$30`.
    pub const PLATFORMS: [u32; 2] = [0x10_E27C, 0x10_E2AE];
    pub const PLATFORM_COUNT: u32 = 0x30;
    /// Tres anillos de 4 palabras: {anterior, nuevo, actual, extra}.
    pub const RINGS: [u32; 3] = [0x10_E2F2, 0x10_E33A, 0x10_E362];
    /// Estela: tres palabras que se desplazan una posición.
    pub const TRAIL: [u32; 3] = [0x10_E47E, 0x10_E480, 0x10_E482];
    /// Tarea con un byte por paridad (`+$96` / `+$97`).
    pub const PARITY_TASK: u32 = 0x10_0580;
    pub const PARITY_BYTE: u32 = 0x96;
}

/// Brazo del jugador montado en el Slug (`SlugCannon_ArmOverlay_03d944`).
pub mod slug_arm {
    /// Tareas de los jugadores y la del Slug (con quién lo controla en
    /// `+$6D`); `RIDE_ENABLED` = se puede montar.
    pub const PLAYERS: [u32; 2] = [0x10_0440, 0x10_04E0];
    pub const SLUG: u32 = 0x10_0580;
    pub const CONTROLLER: u32 = 0x6D;
    pub const RIDE_ENABLED: u32 = 0x10_0609;
    pub const PILOT: u32 = 0x68;
    /// Gráficos de cada jugador: el propio y dos comunes.
    pub const GFX: [[u16; 3]; 2] = [[0x176, 0x190, 0x191], [0x177, 0x190, 0x192]];
    /// `+$1C` según el jugador; `+$12` bit 4 = es el jugador 2.
    pub const TIMER_1C: u32 = 0x1C;
    pub const TIMER_1C_INIT: [u16; 2] = [0x1B, 0x1C];
    pub const FLAGS_12: u32 = 0x12;
    pub const PLAYER2: u8 = 1 << 4;
    /// Guion por personaje del jugador (`+$71`; 5 por jugador).
    pub const CHARACTER: u32 = 0x71;
    pub const SCRIPTS: u32 = 0x03_DA02;
    pub const SCRIPTS_PER_PLAYER: u32 = 5;
    /// Desplazamiento (x, y) sobre el Slug según su inclinación (`+$80`).
    pub const OFFSETS: u32 = 0x03_DAA8;
    pub const SLOPE: u32 = 0x80;
    pub const OFFSET_X: u32 = 0x5C;
    pub const OFFSET_Y: u32 = 0x5E;
    pub const X: u32 = 0x22;
    pub const Y: u32 = 0x24;
    pub const ATTR: u32 = 0x38;
    pub const ATTR_KEEP: u16 = 0xFFE3;
    pub const ATTR_SET: u16 = 0x4;
    /// Bucle y sonido al acabar.
    pub const RUN: u32 = 0x03_DA4C;
    pub const DONE: u32 = 0x03_DAA0;
    pub const DONE_SOUND: u16 = 0x108B;
}

/// Jugador montado en el Slug (`Player_RideSlug_0364a2`).
pub mod player_ride {
    /// `+$8C` bits que se quitan al montarse (brazo, ocupado, 3) y el bit 2
    /// (ocupado: no se agacha el Slug); `+$3A` bit 0 (mirar a la izquierda).
    pub const FLAGS_8C: u32 = 0x8C;
    pub const CLEAR_8C: u8 = (1 << 1) | (1 << 2) | (1 << 3);
    pub const BUSY: u8 = 1 << 2;
    pub const FACING: u32 = 0x3A;
    pub const FACING_LEFT: u8 = 1 << 0;
    pub const HITBOX_CB: u32 = 0x48;
    pub const HITBOX: u32 = 0x03_28D8;
    pub const OFFSET_X: u32 = 0x7C;
    pub const OFFSET_Y: u32 = 0x7E;
    pub const RIDE_OFFSET_Y: u16 = 0x10;
    pub const STATE: u32 = 0x70;
    pub const STATE_RIDE: u8 = 2;
    pub const POSE: u32 = 0x74;
    pub const BYTE_21: u32 = 0x21;
    pub const SCRIPT: u32 = 0x27_9F08;
    pub const SPEEDS: [u32; 4] = [0x28, 0x2C, 0x2A, 0x2E];
    pub const LOOP: u32 = 0x03_6508;
    pub const X: u32 = 0x22;
    pub const Y: u32 = 0x24;
    pub const PILOT: u32 = 0x68;
    /// Slug: su tarea, `+$8D` bits 1 (en el aire) y 4, `+$13` bit 0.
    pub const SLUG: u32 = 0x10_0580;
    pub const SLUG_8D: u32 = 0x8D;
    pub const SLUG_IN_AIR: u8 = 1 << 1;
    pub const SLUG_READY: u8 = 1 << 4;
    pub const SLUG_13: u32 = 0x13;
    pub const SLUG_DESTROYED: u8 = 1 << 0;
    /// Bajarse: rutina, cuánto baja, invulnerabilidad al saltar con A+B.
    pub const POSE2: u32 = 0x03_66FE;
    pub const JUMP_OFF: u32 = 0x03_6796;
    pub const JUMP_OFF_DY: u16 = 0x20;
    pub const INVULN: u32 = 0x45;
    pub const BYTE_59: u32 = 0x59;
    pub const EJECT_INVULN: u8 = 0x3C;
    pub const SCENE_LOADED: u32 = 0x10_6ED3;
}

/// Slug conducido (`Slug_DriveAlt_02e9c8`).
pub mod slug_drive {
    /// Guiones por índice de pendiente y punteros de ataque (`+$48`).
    pub const SCRIPTS: u32 = 0x27_9344;
    pub const ATTACKS: u32 = 0x02_A024;
    pub const NO_SCRIPT: u32 = 0xFFFF_FFFF;
    pub const BYTE_8B: u32 = 0x8B;
    pub const BYTE_8B_DRIVE: u8 = 9;
    pub const BYTE_20: u32 = 0x20;
    pub const BYTE_20_BASE: u8 = 0x28;
    pub const LOOP: u32 = 0x02_EA26;
    /// Velocidad al empujar a la derecha / izquierda.
    pub const PUSH_RIGHT: u16 = 0x2A0;
    pub const PUSH_LEFT: u16 = 0xFD60;
    /// Al acabar el guion: caja de golpe y `+$60`.
    pub const HITBOX_CB: u32 = 0x48;
    pub const DONE_HITBOX: u32 = 0x02_9A14;
    pub const HANDLER_60: u32 = 0x60;
    pub const DONE_HANDLER_60: u32 = 0x02_9600;
    /// Rutinas a las que pasa.
    pub const IDLE_SLOPE: u32 = 0x02_B4D2;
    pub const IDLE_FLAT: u32 = 0x02_B38C;
    pub const CRUISE_RIGHT: u32 = 0x02_BF64;
    pub const CRUISE_LEFT: u32 = 0x02_C07A;
    pub const TURN: u32 = 0x02_E924;
    pub const FALL: u32 = 0x02_D02E;
    pub const JUMP: u32 = 0x02_F614;
    pub const FIRE: u32 = 0x02_C24A;
    pub const IDLE_ENTER_B: u32 = 0x02_DD74;
    pub const EJECT: u32 = 0x02_BBF2;
    pub const HIT_HANDLERS: u32 = 0x02_A060;
    /// Disparo del cañón: munición (`+$90`), disparos en curso (`+$8F`,
    /// tope `MAX_SHOTS`) y espera (`+$8E`).
    pub const AMMO: u32 = 0x90;
    pub const SHOTS: u32 = 0x8F;
    pub const MAX_SHOTS: u8 = 2;
    pub const COOLDOWN: u32 = 0x8E;
    /// Configuración de controles y el botón de disparo según ella.
    pub const CONTROL_LAYOUT: u32 = 0x10_6F2A;
    /// Aviso de montado (`+$89`) y quién controla (`+$6D`).
    pub const BOARDED: u32 = 0x89;
    pub const CONTROLLER: u32 = 0x6D;
    pub const NO_CONTROLLER: u8 = 0xFF;
    /// Sonidos de motor que se piden una vez (mientras `+$8B` ≠ 0, que
    /// luego se borra).
    pub const ENGINE_SOUND_A: u16 = 0x108F;
    pub const ENGINE_SOUND_B: u16 = 0x1092;
    /// Sonido al tocar el suelo según el terreno: el normal y el de los
    /// tipos con bits `$40`/`$80` (A y B).
    pub const LAND_SOUND_A: [u16; 2] = [0x1090, 0x1052];
    pub const LAND_SOUND_B: [u16; 2] = [0x1091, 0x1051];
    /// Parado en llano (`Slug_IdleFlat_02b38c`): guiones y ataques por
    /// pendiente, base de `+$20`, `+$60`, la caja de daño (`+$4C`) y el bucle.
    pub const FLAT_SCRIPTS: u32 = 0x27_92F4;
    pub const FLAT_ATTACKS: u32 = 0x02_9FFC;
    pub const FLAT_BYTE_20: u8 = 0x14;
    pub const FLAT_HANDLER_60: u32 = 0x02_95B4;
    pub const DAMAGE_BOX: u32 = 0x4C;
    pub const FLAT_DAMAGE_BOX: u32 = 0x02_9698;
    pub const FLAT_LOOP: u32 = 0x02_B3E0;
    /// Desde llano: acelerar a la derecha / izquierda, rutina al agacharse
    /// según la pendiente y al saltar según la dirección.
    pub const ACCEL_RIGHT: u32 = 0x02_B8DE;
    pub const ACCEL_LEFT: u32 = 0x02_BA44;
    pub const CROUCH_BY_SLOPE: u32 = 0x02_A078;
    pub const JUMP_BY_DIR: u32 = 0x02_C9B0;
    /// Fuera de la pantalla: márgenes y la baja.
    pub const MARGINS: u32 = 0x27_964E;
    pub const DEATH_FINISH: u32 = 0x02_DCBC;
    /// Pendiente guardada (`+$80`) y la tabla que dice si cuenta como llano
    /// (byte ≠ 0) o cuesta (0), indexada con signo.
    pub const SLOPE: u32 = 0x80;
    pub const FLAT_BY_SLOPE: u32 = 0x02_A558;
}

/// Disparo del cañón del Slug (`Slug_FireIdle_02c190` …
/// `Slug_FireRecoil_02c432`).
pub mod slug_fire {
    /// Guiones por pendiente del disparo y del retroceso.
    pub const SCRIPTS: u32 = 0x27_9498;
    pub const RECOIL_SCRIPTS: u32 = 0x27_94E8;
    /// Base de `+$20` parado y en llano.
    pub const IDLE_BYTE_20: u8 = 0x14;
    pub const FLAT_BYTE_20: u8 = 0x3C;
    /// Bit de la máscara de jugadores con el que sólo sale humo con caída.
    pub const MASK_BIT: u8 = 4;
    /// Lado (`+$84`, `$FF` = izquierda) y proyectiles: a la derecha parado,
    /// en llano y a la izquierda; humo de la boca y humo con caída.
    pub const SIDE: u32 = 0x84;
    pub const SHELL_IDLE: u32 = 0x03_11C0;
    pub const SHELL_FLAT: u32 = 0x03_12B2;
    pub const SHELL_LEFT: u32 = 0x03_1240;
    pub const SMOKE: u32 = 0x03_19F0;
    pub const SMOKE_DROP: u32 = 0x03_1944;
    /// Espera tras disparar y por debajo de cuánto deja agacharse/saltar.
    pub const COOLDOWN: u8 = 0x1E;
    pub const COOLDOWN_FREE: u8 = 0x10;
    /// `+$8D` bit 0: ha disparado.
    pub const FLAGS_8D: u32 = 0x8D;
    pub const FIRED: u8 = 1 << 0;
    /// Botones de dirección (izquierda o derecha) que mantienen la marcha.
    pub const MOVE_BUTTONS: u8 = 0x0C;
    /// Tope de la velocidad X.
    pub const MAX_VEL: u16 = 0x540;
    /// Estados.
    pub const FIRE_FLAT: u32 = 0x02_C24A;
    pub const LOOP: u32 = 0x02_C33A;
    pub const RECOIL: u32 = 0x02_C432;
    pub const RECOIL_LOOP: u32 = 0x02_C46E;
    pub const MOVE_RIGHT: u32 = 0x02_C572;
    pub const MOVE_LEFT: u32 = 0x02_C648;
}

/// Proyectil del cañón del Slug (`VehicleLaunch_*`), el humo de su rastro
/// y su explosión.
pub mod slug_shell {
    /// Campos: cajas de ataque, ángulo (1/256), cuadro, alcance, rapidez,
    /// recorrido, contador del rastro (`$FF` = sin rastro) y su rutina.
    pub const ATTACKS: u32 = 0x4C;
    pub const ANGLE: u32 = 0x34;
    pub const FRAME: u32 = 0x30;
    pub const RANGE: u32 = 0x72;
    pub const SPEED: u32 = 0x36;
    pub const TRAVEL: u32 = 0x70;
    pub const TRAIL: u32 = 0x20;
    pub const NO_TRAIL: u8 = 0xFF;
    pub const TRAIL_FX: u32 = 0x74;
    pub const X: u32 = 0x22;
    pub const Y: u32 = 0x24;
    pub const VEL_X: u32 = 0x28;
    pub const VEL_Y: u32 = 0x2A;
    pub const ACC_X: u32 = 0x2C;
    pub const ACC_Y: u32 = 0x2E;
    pub const ATTR: u32 = 0x38;
    pub const ATTR_FLY: u16 = 0xD000;
    pub const ATTR_KEEP: u16 = 0xFFE3;
    pub const ANIM_TIMER: u32 = 0x46;
    /// `+$13` bit 1: la caja ha chocado; `+$5A` bit 5: contra la pared.
    pub const FLAGS_13: u32 = 0x13;
    pub const BLOCKED: u8 = 1 << 1;
    pub const WALL: u8 = 1 << 5;
    /// Dureza de la caja golpeada con la que desaparece sin estallar.
    pub const ARMOURED: u8 = 6;
    /// Del Slug: índice de pendiente, ángulo del cañón y puntero al cuadro.
    pub const SLUG_SLOPE_IDX: u32 = 0x94;
    pub const SLUG_ANGLE: u32 = 0x80;
    pub const SLUG_POSE: u32 = 0x70;
    /// Ángulo del cañón + 8 si es positivo, + $10 si no.
    pub const ANGLE_OFFSET: (u16, u16) = (8, 0x10);
    /// Tabla por pendiente {-, -, dx, dy, ángulo} del disparo parado.
    pub const OFFSETS: u32 = 0x03_115C;
    pub const OFFSET_LEN: u32 = 10;
    /// Cajas de ataque, guion y gráfico.
    pub const SHELL_ATTACKS: u32 = 0x03_0E20;
    pub const SCRIPT: u32 = 0x29_FA8A;
    pub const GFX: u16 = 0x58;
    pub const SOUND: u16 = 0x1084;
    /// Destello de la boca.
    pub const FLASH: u32 = 0x07_7AB6;
    /// Rapideces y alcances (parado, a la izquierda, en llano, fuerte, al
    /// empujar) y velocidad de salida.
    pub const SPEED_BASE: u16 = 0x600;
    pub const SPEED_FAST: u16 = 0xC00;
    pub const LAUNCH_SPEED: u16 = 0x700;
    pub const RANGE_IDLE: u16 = 0x4400;
    pub const RANGE_LEFT: u16 = 0x6000;
    pub const RANGE_FLAT: u16 = 0x1000;
    pub const RANGE_FAST: u16 = 0x7400;
    pub const RANGE_PUSH: u16 = 0x26AA;
    pub const MUZZLE_DY: u16 = 8;
    /// Configuraciones de controles con disparo fuerte (varios botones /
    /// empujando) y con rastro en cada frame.
    pub const LAYOUT_MULTI: u8 = 5;
    pub const LAYOUT_PUSH: u8 = 3;
    pub const LAYOUT_EVERY_A: u8 = 3;
    pub const LAYOUT_EVERY_B: u8 = 2;
    pub const TRAIL_MASK: u8 = 0x0B;
    /// Planeo: aceleraciones y tope de caída.
    pub const GLIDE_ACC: (u16, u16) = (0xFFE0, 0xFEEA);
    pub const MAX_FALL: u16 = 0x700;
    /// Márgenes de fuera de la pantalla.
    pub const MARGINS: u32 = 0x03_1154;
    /// Estalla: marca, sonido, las tres cajas de la onda y la explosión.
    pub const IMPACT_FLAG: u32 = 0x10_A2D1;
    pub const CRASH_SOUND: u16 = 0x10F2;
    pub const BLAST_ATTACKS: [u32; 3] = [0x03_0F68, 0x03_100C, 0x03_10B0];
    pub const EXPLOSION_GFX: u16 = 0x178;
    pub const EXPLOSION_SCRIPT: u32 = 0x29_E64A;
    pub const EXPLOSION_ATTR: u16 = 0x2000;
    /// Humo del rastro.
    pub const SMOKE: u32 = 0x03_1B78;
    pub const SMOKE_GFX: u16 = 4;
    pub const SMOKE_SCRIPT: u32 = 0x03_23CC;
    pub const SMOKE_FRAMES: u16 = 2;
    /// Estados.
    pub const FLY: u32 = 0x03_1542;
    pub const CRUISE: u32 = 0x03_156C;
    pub const FALL: u32 = 0x03_1594;
    pub const GLIDE: u32 = 0x03_168A;
    pub const GLIDE_LOOP: u32 = 0x03_16BE;
    pub const CRASH: u32 = 0x03_17D2;
    pub const CRASH_JMP: u32 = 0x03_18B4;
    pub const DESPAWN: u32 = 0x03_18B8;
    pub const RELEASE: u32 = 0x03_18D4;
    pub const BLAST_A: u32 = 0x03_1826;
    pub const BLAST_B: u32 = 0x03_184E;
    pub const BLAST_C: u32 = 0x03_1876;
    pub const BLAST_END: u32 = 0x03_18AC;
    pub const EXPLOSION: u32 = 0x03_1BD0;
    pub const EXPLOSION_LOOP: u32 = 0x03_1C02;
    pub const SMOKE_LOOP: u32 = 0x03_1B98;
}

/// Explosiones de una animación (`system::explosion`).
pub mod explosion {
    pub const X: u32 = 0x22;
    pub const Y: u32 = 0x24;
    pub const ATTR: u32 = 0x38;
    pub const FLIP: u32 = 0x3A;
    pub const SCALE: [u32; 2] = [0x32, 0x33];
    pub const HITBOX: u32 = 0x48;
    pub const MARGINS: u32 = 0x70;
    pub const LAYER_POS: [u32; 2] = [0x10_6F6C, 0x10_6F6E];
    /// Variantes: guion, márgenes, gráfico, giro al azar y si quita la caja
    /// de golpe (las que no, usan el bucle de la grande).
    pub const KINDS: [(u32, u32, u16, bool, bool); 15] = [
        (0x2D_F066, 0x2D_F22E, 0xD, false, false), // Explosion_Big_077e74
        (0x2D_F180, 0x2D_F1FC, 0x16C, false, true), // Explosion_Water_077eda
        (0x2D_D4BA, 0x2D_F1FC, 0x4, true, true), // Explosion_Std_077efe
        (0x2D_D4BA, 0x2D_F1FC, 0x12D, true, true), // Explosion_StdSndB_077f22
        (0x2D_D4D6, 0x2D_F1FC, 0x145, true, true), // Explosion_StdB_077f46
        (0x2D_D594, 0x2D_F206, 0x4, true, true), // Explosion_Fire_077f6a
        (0x2D_D594, 0x2D_F206, 0x82, true, true), // Explosion_FireSndB_077f8e
        (0x2D_D37E, 0x2D_F210, 0x82, true, true), // Explosion_Flash_077fb2
        (0x2D_D37E, 0x2D_F210, 0x4, true, true), // Explosion_FlashSndB_077fd6
        (0x2D_D39A, 0x2D_F210, 0x4, true, true), // Explosion_FlashSndB_077fd6+$24
        (0x2D_D6A6, 0x2D_F210, 0x4, true, true), // Explosion_Spark_07801e
        (0x2D_D6D2, 0x2D_F1FC, 0x4, true, true), // Explosion_SmokePuff_078042
        (0x2D_D594, 0x2D_F206, 0x6C, true, true), // Explosion_FireLoud_078066
        (0x2D_D37E, 0x2D_F210, 0x6C, true, true), // Explosion_FlashLoud_07808a
        (0x2D_D594, 0x2D_F206, 0x4, true, true), // Explosion_FireFront_0780ae
    ];
    /// La de delante: capa.
    pub const FRONT: usize = 14;
    pub const FRONT_KEEP: u16 = 0x0FFF;
    pub const FRONT_ATTR: u16 = 0xC000;
    pub const RUN: u32 = 0x07_810A;
    pub const RUN_BIG: u32 = 0x07_7EB4;
    /// Trozos en arco (`Explosion_VarTbl_078218` … `Debris_ArcB_07875e`, uno
    /// cada `ARC_STRIDE` bytes): guion, márgenes y gráfico.
    pub const ARC_FIRST: u32 = 0x07_8218;
    pub const ARC_STRIDE: u32 = 0x1E;
    pub const ARCS: [(u32, u32, u16); 46] = [
        (0x2D_DAD0, 0x2D_F21A, 0xA), // $078218
        (0x2D_DB62, 0x2D_F21A, 0xA), // $078236
        (0x2D_DBF4, 0x2D_F21A, 0xA), // $078254
        (0x2D_DC86, 0x2D_F21A, 0xA), // $078272
        (0x2D_DD18, 0x2D_F224, 0xA), // $078290
        (0x2D_DDAA, 0x2D_F21A, 0xA), // $0782AE
        (0x2D_DE3C, 0x2D_F21A, 0xA), // $0782CC
        (0x2D_DEB2, 0x2D_F21A, 0xA), // $0782EA
        (0x2D_DF28, 0x2D_F224, 0xA), // $078308
        (0x2D_DF9E, 0x2D_F224, 0xA), // $078326
        (0x2D_E014, 0x2D_F21A, 0xA), // $078344
        (0x2D_E08A, 0x2D_F224, 0xA), // $078362
        (0x2D_E100, 0x2D_F224, 0xA), // $078380
        (0x2D_E176, 0x2D_F21A, 0xA), // $07839E
        (0x2D_E1EC, 0x2D_F21A, 0xA), // $0783BC
        (0x2D_DE3C, 0x2D_F21A, 0x9E), // $0783DA
        (0x2D_DEB2, 0x2D_F21A, 0x9E), // $0783F8
        (0x2D_DF28, 0x2D_F224, 0x9E), // $078416
        (0x2D_DF9E, 0x2D_F224, 0x9E), // $078434
        (0x2D_E014, 0x2D_F21A, 0x9E), // $078452
        (0x2D_E08A, 0x2D_F224, 0x9E), // $078470
        (0x2D_E100, 0x2D_F224, 0x9E), // $07848E
        (0x2D_E176, 0x2D_F21A, 0x9E), // $0784AC
        (0x2D_E1EC, 0x2D_F21A, 0x9E), // $0784CA
        (0x2D_E08A, 0x2D_F224, 0xAD), // $0784E8
        (0x2D_E100, 0x2D_F224, 0xAD), // $078506
        (0x2D_E176, 0x2D_F21A, 0xAD), // $078524
        (0x2D_E1EC, 0x2D_F21A, 0xAD), // $078542
        (0x2D_E262, 0x2D_F21A, 0xA), // $078560
        (0x2D_E262, 0x2D_F21A, 0x72), // $07857E
        (0x2D_E2D8, 0x2D_F21A, 0xA), // $07859C
        (0x2D_E2D8, 0x2D_F21A, 0x72), // $0785BA
        (0x2D_E2D8, 0x2D_F21A, 0x88), // $0785D8
        (0x2D_E34E, 0x2D_F224, 0xA), // $0785F6
        (0x2D_E34E, 0x2D_F224, 0x72), // $078614
        (0x2D_E34E, 0x2D_F224, 0x77), // $078632
        (0x2D_E3C4, 0x2D_F21A, 0xA), // $078650
        (0x2D_E3C4, 0x2D_F21A, 0x72), // $07866E
        (0x2D_E43A, 0x2D_F21A, 0xA), // $07868C
        (0x2D_E43A, 0x2D_F21A, 0x72), // $0786AA
        (0x2D_E43A, 0x2D_F21A, 0x77), // $0786C8
        (0x2D_E43A, 0x2D_F21A, 0x88), // $0786E6
        (0x2D_E43A, 0x2D_F21A, 0x28), // $078704
        (0x2D_E4B0, 0x2D_F21A, 0xB), // $078722
        (0x2D_E5F4, 0x2D_F21A, 0xB), // $078740
        (0x2D_E66A, 0x2D_F224, 0xB), // $07875E
    ];
    pub const ARC_RUN: u32 = 0x07_87F8;
    /// Impulso: Y inicial, gravedad, azar de X (con signo al azar) y de Y,
    /// tope de caída.
    pub const ARC_VEL_Y: u16 = 0x7F8;
    pub const ARC_GRAVITY: u16 = 0xFFBC;
    pub const ARC_RAND_X: u16 = 0x3FF;
    pub const ARC_RAND_Y: u16 = 0x7FF;
    pub const ARC_MIN_VEL_Y: i16 = -0x500;
    pub const VEL_X: u32 = 0x28;
    pub const VEL_Y: u32 = 0x2A;
    pub const ACC_X: u32 = 0x2C;
    pub const ACC_Y: u32 = 0x2E;
    /// Índice de `Explosion_Fire_077f6a` en `KINDS`.
    pub const FIRE: usize = 5;
}

/// Slug golpeado (`Slug_HitReact_02d63e` … `Slug_HitLandB_02d980`).
pub mod slug_hit {
    /// Guiones por pendiente: sacudida (dos variantes), vuelo y aterrizaje.
    pub const REACT_SCRIPTS: [u32; 2] = [0x27_954C, 0x27_9560];
    pub const LAUNCH_SCRIPTS: [u32; 2] = [0x27_9574, 0x27_9588];
    pub const LAND_SCRIPTS: [u32; 2] = [0x27_959C, 0x27_95B0];
    pub const NO_SCRIPT: u32 = 0xFFFF_FFFF;
    pub const BYTE_20: u32 = 0x20;
    pub const BYTE_20_HIT: u8 = 0x96;
    pub const FLAGS_8C: u32 = 0x8C;
    pub const BIT_8C_5: u8 = 1 << 5;
    pub const FLAGS_8D: u32 = 0x8D;
    pub const IN_AIR: u8 = 1 << 1;
    pub const FLAGS_13: u32 = 0x13;
    pub const BIT_13_3: u8 = 1 << 3;
    pub const TIMER_45: u32 = 0x45;
    pub const TIMER_59: u32 = 0x59;
    pub const TIMER_FRAMES: u8 = 0x28;
    pub const HANDLER_60: u32 = 0x60;
    pub const REACT_HANDLER_60: u32 = 0x02_95B4;
    pub const HITBOX_CB: u32 = 0x48;
    pub const DRIVE_HITBOX: u32 = 0x02_9A14;
    pub const VEL_Y: u32 = 0x2A;
    pub const ACC_X: u32 = 0x2C;
    pub const ACC_Y: u32 = 0x2E;
    /// Impulso y gravedad del vuelo y tope de la velocidad vertical.
    pub const LAUNCH_VEL_Y: u16 = 0x3FC;
    pub const LAUNCH_GRAVITY: u16 = 0xFFAB;
    pub const MAX_VEL_Y: u16 = 0x400;
    pub const MARGINS: u32 = 0x27_964E;
    /// Rutinas.
    pub const REACT_LOOP: u32 = 0x02_D6CE;
    pub const LAUNCH_LOOP: [u32; 2] = [0x02_D78C, 0x02_D852];
    pub const LAND: [u32; 2] = [0x02_D8C8, 0x02_D980];
    pub const LAND_LOOP: [u32; 2] = [0x02_D908, 0x02_D9C0];
    pub const IDLE_FLAT: u32 = 0x02_B38C;
    pub const FALL: u32 = 0x02_D02E;
    pub const IDLE_ENTER_B: u32 = 0x02_DD74;
    pub const DEATH_EXPLODE: u32 = 0x02_DA38;
    pub const DEATH_LAUNCH: u32 = 0x02_DB52;
    pub const DEATH_FINISH: u32 = 0x02_DCBC;
}

/// Muerte de los humanos (soldados, POW…): `asm/pow/human_death_049ff2.s`.
pub mod human_death {
    pub const X: u32 = 0x22;
    pub const Y: u32 = 0x24;
    pub const VEL_X: u32 = 0x28;
    pub const VEL_Y: u32 = 0x2A;
    pub const ACC_X: u32 = 0x2C;
    pub const ACC_Y: u32 = 0x2E;
    pub const TIMER: u32 = 0x30;
    pub const ZOOM: [u32; 2] = [0x32, 0x33];
    pub const FIELD_34: u32 = 0x34;
    pub const ATTR: u32 = 0x38;
    pub const FACING: u32 = 0x3A;
    pub const FLAGS_12: u32 = 0x12;
    pub const BIT_12_1: u8 = 1 << 1;
    pub const BIT_12_6: u8 = 1 << 6;
    pub const FLAGS_13: u32 = 0x13;
    pub const BIT_13_3: u8 = 1 << 3;
    pub const DEAD: u32 = 0x21;
    pub const HITBOX: u32 = 0x48;
    pub const ATTACKER: u32 = 0x50;
    pub const HIT_X: u32 = 0x54;
    pub const HIT_KIND: u32 = 0x58;
    pub const FLAGS_5A: u32 = 0x5A;
    pub const BIT_5A_5: u8 = 1 << 5;
    pub const COUNT: u32 = 0x5C;
    pub const HANDLER_60: u32 = 0x60;
    pub const GROUNDED: u32 = 0x70;
    pub const BEHIND: u32 = 0x71;
    pub const FRAME: u32 = 0x72;
    pub const FRAME_LEFT: u32 = 0x73;
    /// Sonido (grito) que se pide al morir.
    pub const CRY: u32 = 0x74;
    pub const GIB: u32 = 0x76;
    pub const NIL: u32 = 0xFFFF_FFFF;
    /// Soft-DIP de la sangre (0 = sin sangre).
    pub const BLOOD: u32 = 0x10_FD8F;
    /// Tablas de estados por tipo de humano, indexadas por el tipo de daño
    /// (`HIT_KINDS`; los de fuera, `HIT_KIND_DEFAULT`).
    pub const STATE_TABLES: [u32; 4] = [0x04_9D8A, 0x04_9E12, 0x04_9E9A, 0x04_9F22];
    pub const HIT_KINDS: u8 = 0x22;
    pub const HIT_KIND_DEFAULT: u8 = 0xF;
    pub const BODY_ATTR: u16 = 0xC000;
    /// Gráficos (con sangre, sin sangre).
    pub const GFX_BODY: (u16, u16) = (0xE, 0x2F);
    pub const GFX_BLAST: (u16, u16) = (0x154, 0x155);
    pub const GFX_BLOOD: (u16, u16) = (0x17A, 0x1BA);
    pub const GFX_BURST: u16 = 0x19;
    pub const GFX_LAUNCH: u16 = 0x8A;
    pub const GFX_FLAME: u16 = 0x4;
    /// Entrada de sprite en uso (`+$14`) y base de los gráficos con la que
    /// `$00236E` deja `d4` (gráfico · 64 + `GFX_BASE`).
    pub const ENTRY: u32 = 0x14;
    pub const GFX_BASE: u32 = 0x01_4E00;
    /// Posición de lectura del anillo de entradas libres.
    pub const FREE_READ: u32 = 0x10_82C4;
    /// Anillo de posiciones de los caídos.
    pub const RING: u32 = 0x10_E33A;
    pub const RING_SIZE: u16 = 0x20;
    pub const RING_MAX: u16 = 4;
    /// Despawn: X + `CULL_X.0` ≥ `CULL_X.1` o Y < `CULL_Y`.
    pub const CULL_X: (u16, u16) = (0x20, 0x180);
    pub const CULL_Y: u16 = 0xE0;
    pub const FADE_FRAMES: u8 = 0xA;
    pub const FALL_ACC: u16 = 0xFF40;
    pub const TASK_FINISHING: u32 = 0x00_052A;
    /// Voltereta hacia atrás / adelante: secuencias de 5 guiones.
    pub const TUMBLE_BACK: u32 = 0x04_A408;
    pub const TUMBLE_FWD: u32 = 0x04_A4B0;
    pub const TUMBLE_VEL: u16 = 0x80;
    /// Guiones.
    pub const S_TUMBLE_LAND: u32 = 0x04_B6B8;
    pub const S_COLLAPSE: [u32; 2] = [0x04_B57C, 0x04_B61A];
    pub const S_FALL_BACK: u32 = 0x04_B1F6;
    pub const S_FALL_FWD: u32 = 0x04_B28C;
    pub const S_SLUMP: u32 = 0x04_B34A;
    pub const S_CRUMPLE: [u32; 2] = [0x04_AD90, 0x04_AE22];
    pub const S_BLAST: u32 = 0x04_B13C;
    pub const S_SHOT: u32 = 0x04_B136;
    pub const S_FLIP_B: u32 = 0x04_B78C;
    pub const S_FLIP: u32 = 0x04_B74C;
    pub const S_KNOCKDOWN: u32 = 0x04_B7B8;
    pub const S_SIT: u32 = 0x04_ACFE;
    pub const S_BURST: u32 = 0x04_AF08;
    pub const S_BURST_HITBOX: u32 = 0x04_AEB4;
    pub const S_BURST_LAND: u32 = 0x04_AF9E;
    pub const S_BURST_GROUND: u32 = 0x04_B078;
    pub const S_LAUNCHED: u32 = 0x04_B0C6;
    pub const S_LAUNCH: u32 = 0x04_B7EC;
    pub const S_ON_FIRE: u32 = 0x04_B82A;
    pub const S_BURNING: u32 = 0x04_B898;
    pub const S_BURN_RUN: u32 = 0x2B_604A;
    pub const S_FLAME: u32 = 0x04_B926;
    pub const S_BLOOD: [u32; 2] = [0x04_BAD2, 0x04_B9F0];
    pub const GIBS: u32 = 0x04_AF5A;
    pub const FALL_VEL_X: u16 = 0x400;
    pub const FALL_VEL_Y: u16 = 0x100;
    pub const FLIP_VEL_X: u16 = 0xFF38;
    pub const BURST_VEL_Y: u16 = 0xCC6;
    pub const BURST_ACC_Y: u16 = 0xFF93;
    pub const BURST_RAND: u16 = 0x3FF;
    pub const LAUNCH_VEL: (u16, u16, u16) = (0xFFFC, 0x870, 0xFFB8);
    pub const LAUNCH_ZOOM: u16 = 0x2800;
    pub const BACK_VEL: (u16, u16) = (0xF800, 0x100);
    pub const UP_VEL_Y: u16 = 0x800;
    pub const FIRE_ATTR: u16 = 0x8000;
    pub const LAYER_KEEP: u16 = 0xFFE3;
    pub const BLOOD_LAYER: u16 = 0x10;
    pub const FIRE_FRAMES: u16 = 0x1E;
    pub const BURN_MUSIC: u16 = 0x1042;
    pub const BURN_VEL: u16 = 0x200;
    pub const BURN_FRAMES: u16 = 0x28;
    pub const BURN_TURNS: (u8, u8) = (0xA, 0xFF);
    pub const BURN_ACC_Y: u16 = 0xFFC0;
    pub const FLAME_DY: u16 = 0x1E;
    pub const BLOOD_DX: u16 = 8;
    /// Rutinas.
    pub const TUMBLE_BACK_STATE: u32 = 0x04_A438;
    pub const TUMBLE_BACK_LOOP: u32 = 0x04_A476;
    pub const TUMBLE_BACK_LAND: u32 = 0x04_A4A2;
    pub const TUMBLE_FWD_STATE: u32 = 0x04_A4E0;
    pub const TUMBLE_FWD_LOOP: u32 = 0x04_A51E;
    pub const COLLAPSE: u32 = 0x04_A54A;
    pub const ANIM_LOOP: u32 = 0x04_A388;
    pub const FADE: u32 = 0x04_A39E;
    pub const FADE_LOOP: u32 = 0x04_A3AA;
    pub const BLINK: u32 = 0x04_A3C4;
    pub const BLINK_LOOP: u32 = 0x04_A3D0;
    pub const FLIP_LOOP: u32 = 0x04_A652;
    pub const KNOCKDOWN: u32 = 0x04_A672;
    pub const SIT_LOOP: u32 = 0x04_A69C;
    pub const BURST_RISE: u32 = 0x04_A754;
    pub const BURST_FALL: u32 = 0x04_A76A;
    pub const BURST_LAND: u32 = 0x04_A798;
    pub const BURST_GROUND_LOOP: u32 = 0x04_A7D8;
    pub const LAUNCHED: u32 = 0x04_A7F4;
    pub const LAUNCH_LOOP: u32 = 0x04_A85E;
    pub const BACK_LOOP: u32 = 0x04_A8DE;
    pub const UP_LOOP: u32 = 0x04_A920;
    pub const FIRE_LOOP: u32 = 0x04_A982;
    pub const BURNING: u32 = 0x04_A9B0;
    pub const BURN_LOOP: u32 = 0x04_AA1E;
    pub const BURNED_DOWN: u32 = 0x04_AA62;
    pub const FLAME: u32 = 0x04_AA76;
    pub const FLAME_LOOP: u32 = 0x04_AA90;
    pub const BLOOD_SPLASH: [u32; 2] = [0x04_AB10, 0x04_AB52];
    pub const BLOOD_LOOP: u32 = 0x04_AB90;
}

/// Ataque suicida del Slug: se lanza hacia delante y explota
/// (`Slug_DestroyedSlideInit_02bbf2`, `Slug_Destroyed_02fc70`,
/// `Slug_SelfDestructAttack_02fe6a` …).
pub mod slug_attack {
    pub const START_MUSIC: u16 = 0x10E9;
    pub const SLIDE_SOUND: u16 = 0x10B2;
    pub const ROLL_SOUND: u16 = 0x10AF;
    pub const VEL_X: u32 = 0x28;
    pub const VEL_Y: u32 = 0x2A;
    pub const ACC_X: u32 = 0x2C;
    pub const ACC_Y: u32 = 0x2E;
    pub const SPEED: u32 = 0x36;
    pub const SLIDE_VEL_X: u16 = 0x600;
    /// Frames que le quedan antes de explotar; por debajo de `SLIDE_RESET`
    /// vuelve a la velocidad de salida; cerca del borde baja a `NEAR_EDGE`.
    pub const FUSE: u32 = 0x92;
    pub const FUSE_FRAMES: u8 = 0x30;
    pub const SLIDE_RESET: u8 = 0x29;
    pub const NEAR_EDGE: i8 = 0x0A;
    pub const NEAR_EDGE_X: i16 = 0xC0;
    pub const MAX_X: i16 = 0x117;
    /// Guiones por pendiente y `+$20` = `SLIDE_BYTE_20` + índice.
    pub const SLIDE_SCRIPTS: u32 = 0x27_936C;
    pub const NO_SCRIPT: u32 = 0xFFFF_FFFF;
    pub const BYTE_20: u32 = 0x20;
    pub const SLIDE_BYTE_20: u8 = 0x19;
    pub const HANDLER_60: u32 = 0x60;
    pub const SLIDE_HANDLER_60: u32 = 0x02_964C;
    pub const HITBOX_CB: u32 = 0x48;
    pub const ATTACKS: u32 = 0x4C;
    pub const ATTACKS_RAM: u32 = 0x02_9834;
    pub const ATTACKS_BOOM: u32 = 0x02_9790;
    pub const ATTACKS_BLAST: u32 = 0x02_98D8;
    pub const NIL: u32 = 0xFFFF_FFFF;
    pub const FRAME_COUNTER: u32 = 0x10_6F28;
    pub const CONTROL_LAYOUT: u32 = 0x10_6F2A;
    pub const LAYOUT_BOOST: u8 = 3;
    pub const DAMAGE_SCRIPT_C: u32 = 0x02_FA3C;
    pub const X: u32 = 0x22;
    pub const Y: u32 = 0x24;
    pub const BLAST_POS: (u16, u16) = (0xA0, 0x180);
    pub const SLOPE: u32 = 0x80;
    pub const ANGLE_CENTER: i16 = 0x20;
    pub const ANGLE_MAX: u16 = 0x40;
    pub const ANGLE_SCRIPTS: u32 = 0x2B_0C30;
    pub const FLAGS_8C: u32 = 0x8C;
    pub const BIT_8C_5: u8 = 1 << 5;
    pub const FLAGS_8D: u32 = 0x8D;
    pub const BIT_8D_2: u8 = 1 << 2;
    pub const BIT_8D_4: u8 = 1 << 4;
    pub const STEPS: u32 = 0x3B;
    pub const FLAGS_5A: u32 = 0x5A;
    pub const BIT_5A_5: u8 = 1 << 5;
    pub const BASE_Y: u32 = 0x82;
    pub const BASE_Y_DROP: u16 = 0x10;
    pub const EXPLODED: u32 = 0x10_A2D1;
    /// Rutinas.
    pub const SLIDE_START: u32 = 0x02_BC56;
    pub const SLIDE_LOOP: u32 = 0x02_BC78;
    pub const SLIDE: u32 = 0x02_BBA4;
    pub const ROLL: u32 = 0x02_FC8E;
    pub const ROLL_LOOP: u32 = 0x02_FD1A;
    pub const SELF_DESTRUCT: u32 = 0x02_FE6A;
    pub const SELF_DESTRUCT_LOOP: u32 = 0x02_FE76;
    pub const BLAST: u32 = 0x02_FF4A;
    pub const BLAST_END: u32 = 0x02_FF86;
    pub const DEATH_FADE: u32 = 0x02_DC5C;
}

/// Destrucción del Slug (`Slug_DeathExplode_02da38` … `Slug_DeathStart_02dcc0`).
pub mod slug_death {
    pub const TIMER_59: u32 = 0x59;
    pub const TIMER_45: u32 = 0x45;
    pub const KILL_TIMER_45: u8 = 0xFE;
    pub const HP: u32 = 0x66;
    pub const FLAGS_13: u32 = 0x13;
    pub const DESTROYED: u8 = 1 << 0;
    pub const FLAGS_8C: u32 = 0x8C;
    pub const BIT_8C_5: u8 = 1 << 5;
    pub const FLAGS_8D: u32 = 0x8D;
    pub const BIT_8D_4: u8 = 1 << 4;
    pub const FLAGS_5B: u32 = 0x5B;
    pub const BIT_5B_7: u8 = 1 << 7;
    /// Sprites de la explosión.
    pub const KILL_GFX: [u16; 4] = [0x19C, 0x19D, 0x19E, 0x19A];
    pub const EXPLODE_SOUND: u16 = 0x1034;
    pub const SMOKE: u32 = 0x02_F672;
    /// Humos (sprite, guion, atributos) y su bucle.
    pub const SMOKES: [(u16, u32, u16); 2] = [(0x4, 0x2D_D6A6, 0x2000), (0x163, 0x02_F6C0, 0xD000)];
    pub const SMOKE_RUN: u32 = 0x02_F692;
    pub const ATTR: u32 = 0x38;
    /// Cajas de ataque de la explosión.
    pub const ATTACKS: u32 = 0x4C;
    pub const BLAST_ATTACKS: u32 = 0x02_973C;
    pub const X: u32 = 0x22;
    pub const BASE_Y: u32 = 0x82;
    pub const BASE_Y_MASK: u16 = 0x3FF;
    pub const WORK_POS: [u32; 2] = [0x10_6F38, 0x10_6F3A];
    /// Caja de golpe (su izquierda/derecha en `+$0A`) y X del impacto.
    pub const HITBOX_CB: u32 = 0x48;
    pub const BOX_LEFT: u32 = 0x0A;
    pub const HIT_X: u32 = 0x54;
    pub const POSE_SCRIPTS: [u32; 2] = [0x27_954C, 0x27_9560];
    pub const NO_SCRIPT: u32 = 0xFFFF_FFFF;
    pub const BYTE_20: u32 = 0x20;
    pub const BYTE_20_HIT: u8 = 0x96;
    pub const MARGINS: u32 = 0x27_964E;
    pub const DAMAGE_SCRIPT_B: u32 = 0x02_F8AE;
    pub const VEL_Y: u32 = 0x2A;
    pub const ACC_Y: u32 = 0x2E;
    pub const LAUNCH_VEL_Y: u16 = 0x3FC;
    pub const LAUNCH_GRAVITY: u16 = 0xFFAB;
    pub const MAX_VEL_Y: u16 = 0x400;
    pub const FADE_TIMER: u32 = 0x91;
    pub const LAUNCH_FRAMES: u8 = 0x3C;
    pub const FADE_FRAMES: u8 = 0x28;
    pub const HANDLER_60: u32 = 0x60;
    /// Rutinas.
    pub const EXPLODE: u32 = 0x02_DA38;
    pub const EXPLODE_LOOP: u32 = 0x02_DAE4;
    pub const LAUNCH_LOOP: u32 = 0x02_DC0C;
    pub const FADE: u32 = 0x02_DC5C;
    pub const FADE_LOOP: u32 = 0x02_DC7E;
    pub const SET_IDLE: u32 = 0x02_DCAA;
    pub const IDLE: u32 = 0x00_0400;
    /// Música de la pérdida y burbuja del piloto.
    pub const LOST_MUSIC: u16 = 0x10E9;
    pub const BUBBLE: u32 = 0x03_207C;
}

/// Lanzador de trozos (`Spawner_Handler_077c98`) y esquirlas de la
/// explosión del Slug (`Template_08121C` …).
pub mod spawner {
    pub const INIT: u32 = 0x07_7C98;
    pub const RUN: u32 = 0x07_7CAE;
    /// Campos: descripción, temporizador (`BURST` = todo de golpe) y cuenta.
    pub const DESC: u32 = 0x70;
    pub const TIMER: u32 = 0x74;
    pub const COUNT: u32 = 0x76;
    pub const BURST: u8 = 0xFF;
    /// Descripción: seguir al dueño, periodo, cuenta, desplazamiento X,
    /// azar X, desplazamiento Y, azar Y, elegir al azar de una tabla de
    /// `TABLE_LEN` y rutina (o tabla).
    pub const D_FOLLOW: u32 = 0;
    pub const D_PERIOD: u32 = 1;
    pub const D_COUNT: u32 = 2;
    pub const D_DX: u32 = 4;
    pub const D_RAND_X: u32 = 6;
    pub const D_DY: u32 = 8;
    pub const D_RAND_Y: u32 = 0xA;
    pub const D_RANDOM: u32 = 0xD;
    pub const D_HANDLER: u32 = 0xE;
    pub const TABLE_LEN: u16 = 0x10;
    pub const X: u32 = 0x22;
    pub const Y: u32 = 0x24;
    pub const SUB: [u32; 2] = [0x26, 0x27];
    pub const FLIP: u32 = 0x3A;
    pub const SCALE: [u32; 2] = [0x32, 0x33];
    pub const FLAGS_12: u32 = 0x12;
    pub const FLAGS_12_KEEP: u8 = 0x40;
    pub const VEL_X: u32 = 0x28;
    pub const VEL_Y: u32 = 0x2A;
    /// Trozos del Slug.
    pub const SLUG_DEBRIS: u32 = 0x2E_3C54;
    /// Esquirlas: entradas por cuadrante, rapidez, frames entre pasos y
    /// explosiones que deja.
    pub const SHARDS: [u32; 4] = [0x08_121C, 0x08_123C, 0x08_1260, 0x08_1284];
    pub const QUADRANT: u16 = 0x40;
    pub const QUADRANT_MASK: u16 = 0x3F;
    pub const SHARD_SPEED: i16 = 0x2000;
    pub const STEP_TIMER: u32 = 0x70;
    pub const STEP_PERIOD: u32 = 0x72;
    pub const BLASTS: u32 = 0x74;
    pub const SHARD_STEP: u16 = 3;
    pub const SHARD_BLASTS: u16 = 4;
    pub const SHARD_RUN: u32 = 0x08_12BE;
    pub const BLAST: u32 = 0x07_7EFE;
    /// Cola de vídeo: lo volcado y su tramo.
    pub const QUEUE_USED: u32 = 0x10_E1E8;
    pub const QUEUE_SPAN: [u32; 2] = [0x10_E1FA, 0x10_E1F6];
}

/// Salto del Slug (`Slug_JumpCrouch_02c908` … `Slug_JumpLand_02cc1a`).
pub mod slug_jump {
    /// Velocidad horizontal del salto a la derecha / izquierda; con `+$94`
    /// igual a `DIR_BLOCK_RIGHT` / `_LEFT` salta en vertical.
    pub const DIR_MODE: u32 = 0x94;
    pub const DIR_BLOCK_RIGHT: u16 = 3;
    pub const DIR_BLOCK_LEFT: u16 = 4;
    pub const VEL_RIGHT: u16 = 0x2AA;
    pub const VEL_LEFT: u16 = 0xFD56;
    /// Agacharse antes de saltar: guiones, ataques, base de `+$20`, `+$60`.
    pub const CROUCH_SCRIPTS: u32 = 0x27_9308;
    pub const CROUCH_ATTACKS: u32 = 0x02_A038;
    pub const CROUCH_BYTE_20: u8 = 0x2D;
    pub const HANDLER_60: u32 = 0x60;
    pub const CROUCH_HANDLER_60: u32 = 0x02_95B4;
    /// Despegue: velocidad y gravedad, guiones, ataques, base de `+$20`.
    pub const LAUNCH_VEL_Y: u16 = 0x4A4;
    pub const LAUNCH_ACC_Y: u16 = 0xFF9D;
    pub const LAUNCH_SCRIPTS: u32 = 0x27_931C;
    pub const LAUNCH_ATTACKS: u32 = 0x02_A04C;
    pub const LAUNCH_BYTE_20: u8 = 0x32;
    /// Aterrizaje: duración (`+$91`), velocidad que le queda, guiones y
    /// ataques, base de `+$20`.
    pub const LAND_TIMER: u32 = 0x91;
    pub const LAND_FRAMES: u8 = 0x11;
    pub const LAND_VEL_X: u16 = 0x80;
    pub const LAND_SCRIPTS: u32 = 0x27_9330;
    pub const LAND_ATTACKS: u32 = 0x02_9FFC;
    pub const LAND_BYTE_20: u8 = 0x37;
    /// En el aire: aceleración según la dirección del mando y topes.
    pub const STEER_ACC: u32 = 0x02_C900;
    pub const MAX_VEL_X: u16 = 0x2A0;
    pub const MAX_VEL_Y: u16 = 0x400;
    /// Inclinación al tocar una cuesta: guiones por sprite de inclinación.
    pub const TILT_SCRIPTS: u32 = 0x2B_0C30;
    /// Campos.
    pub const VEL_X: u32 = 0x28;
    pub const VEL_Y: u32 = 0x2A;
    pub const ACC_X: u32 = 0x2C;
    pub const ACC_Y: u32 = 0x2E;
    pub const Y: u32 = 0x24;
    pub const BASE_Y: u32 = 0x82;
    pub const BYTE_20: u32 = 0x20;
    pub const HITBOX_CB: u32 = 0x48;
    pub const SLOPE: u32 = 0x80;
    pub const FLAGS_5B: u32 = 0x5B;
    pub const BIT_5B_3: u8 = 1 << 3;
    pub const LANDED: u8 = 1 << 7;
    pub const FLAGS_8C: u32 = 0x8C;
    pub const BIT_8C_5: u8 = 1 << 5;
    pub const FLAGS_8D: u32 = 0x8D;
    pub const IN_AIR: u8 = 1 << 1;
    pub const TILTED: u8 = 1 << 2;
    pub const NO_SCRIPT: u32 = 0xFFFF_FFFF;
    pub const TILT_MAX: u16 = 0x40;
    /// Rutinas.
    pub const CROUCH_LOOP: u32 = 0x02_C95C;
    pub const LAUNCH: u32 = 0x02_CA0C;
    pub const AIR: u32 = 0x02_CA8A;
    pub const AIR_FIRE: u32 = 0x02_CC02;
    pub const LAND: u32 = 0x02_CC1A;
    pub const LAND_LOOP: u32 = 0x02_CCB0;
    pub const LAND_SLOPE: u32 = 0x02_CE1C;
    pub const LAND_SLOPE_LOOP: u32 = 0x02_CE7E;
    pub const FIRE_CROUCH: u32 = 0x02_D286;
    pub const FIRE_CROUCH_AIR: u32 = 0x02_D298;
    pub const FIRE_AIR_B: u32 = 0x02_D380;
    pub const FIRE_FLAT: u32 = 0x02_C24A;
    pub const IDLE_ENTER_B: u32 = 0x02_DD74;
    pub const IDLE_SLOPE: u32 = 0x02_B4D2;
    pub const FALL: u32 = 0x02_D02E;
    pub const EJECT: u32 = 0x02_BBF2;
    /// Tablas de rutina: golpes (en tierra / en el aire), agacharse al
    /// aterrizar según la pendiente, saltar según la dirección.
    pub const HIT_HANDLERS: u32 = 0x02_A060;
    pub const HIT_HANDLERS_AIR: u32 = 0x02_A06C;
    pub const CROUCH_LAND: u32 = 0x02_A08C;
    pub const JUMP_BY_DIR: u32 = 0x02_C9B0;
    pub const MARGINS: u32 = 0x27_964E;
    pub const DEATH_FINISH: u32 = 0x02_DCBC;
}

/// Escena de mensaje y diálogo de la tarjeta de memoria
/// (`BootScr_Message_Prepare_001416`, `MemCard_LoadDialog_*`).
/// Paracaídas del jugador (`system::parachute`).
pub mod parachute {
    pub const X: u32 = 0x22;
    pub const Y: u32 = 0x24;
    pub const ATTR: u32 = 0x38;
    pub const FACING: u32 = 0x3A;
    pub const LINK: u32 = 0x48;
    pub const NO_LINK: u32 = 0xFFFF_FFFF;
    pub const BLINK: u32 = 0x71;
    pub const GFX: u16 = 0x18D;
    /// El paracaídas se dibuja detrás del soldado.
    pub const DEPTH_BEHIND: u16 = 0x20;
    /// Velocidad horizontal del soldado y su conversión a fotograma.
    pub const VEL_X: u32 = 0x28;
    pub const SPEED_BIAS: u16 = 0x400;
    pub const FRAMES: u16 = 10;
    /// Animaciones por fotograma (abierto y balanceo), plegado y suelto.
    pub const OPEN_FRAMES: u32 = 0x28_F8D4;
    pub const SWAY_FRAMES: u32 = 0x28_FDBC;
    pub const FOLD_SCRIPT: u32 = 0x28_FE2C;
    pub const RELEASE_SCRIPT: u32 = 0x03_8EC8;
    /// Soldado: flags (`+$8C` bit 2 = balanceo), estado, muerto.
    pub const OWNER_FLAGS: u32 = 0x8C;
    pub const OWNER_SWAY: u8 = 1 << 2;
    pub const OWNER_STATE: u32 = 0x70;
    pub const STATE_PARACHUTE: u8 = 4;
    pub const STATE_GONE: u8 = 0x26;
    pub const OWNER_DEAD: u32 = 0x13;
    /// Rutinas.
    pub const OPEN: u32 = 0x03_8D08;
    pub const OPEN_RUN: u32 = 0x03_8D0E;
    pub const SWAY: u32 = 0x03_8D88;
    pub const SWAY_RUN: u32 = 0x03_8DB0;
    pub const FOLD: u32 = 0x03_8E10;
    pub const FOLD_RUN: u32 = 0x03_8E32;
    pub const RELEASE: u32 = 0x03_8E4A;
    pub const RELEASE_RUN: u32 = 0x03_8E6A;
}

/// Selector de guion del modo de prueba (`system::msg_select`).
pub mod msg_select {
    pub const SEED_COUNTER: u32 = super::rng::SEED_COUNTER;
    /// Índice elegido y el guardado en la tarjeta.
    pub const INDEX: u32 = super::memcard::MESSAGE_INDEX;
    pub const SAVED_INDEX: u32 = 0x10_E3B7;
    /// Al entrar el índice se limita a `FIRST_LIMIT - 1`; después, a `COUNT`.
    pub const FIRST_LIMIT: u8 = 6;
    pub const COUNT: u8 = 16;
    /// Tabla de 16 entradas: lista de escenas y puntero al número de fila
    /// del rótulo (negativo = sin rótulo).
    pub const TABLE: u32 = 0x00_0C0A;
    pub const ENTRY_LEN: u32 = 8;
    pub const TEXT: u32 = 4;
    /// Filas de rótulos (punteros a listas de glifos terminadas en `ROW_END`).
    pub const ROWS: u32 = 0x05_EB9E;
    pub const ROW_PALETTE: u32 = 0x05_EB98;
    pub const ROW_ORIGIN: u16 = 0x704D;
    pub const ROW_STEP: u16 = 0x40;
    pub const ROW_END: u16 = 0xFFFF;
    pub const TILE_BANK: u16 = 0x1000;
    pub const TILE_MASK: u16 = 0x1FFF;
    /// Animación de la paleta: entrada, primer paso, frames y modo.
    pub const ROW_PALETTE_ANIM: (u16, u16, u16, u8) = (1, 0x192, 1, 1);
}

pub mod memcard {
    /// Petición a la BIOS: orden, modo, opción, búfer, tamaño y resultado.
    pub const CARD_COMMAND: u32 = 0x10_FDC4;
    pub const CARD_READ: u8 = 2;
    pub const CARD_MODE: u32 = 0x10_FDCE;
    pub const CARD_MODE_GAME: u16 = 0x201;
    pub const CARD_OPTION: u32 = 0x10_FDD0;
    pub const CARD_BUFFER: u32 = 0x10_FDC8;
    pub const CARD_SIZE: u32 = 0x10_FDCC;
    pub const CARD_ERROR: u32 = 0x10_FDC6;
    /// Pila que ocupa el original antes de llamar a la BIOS: vuelta de la
    /// lectura y 15 registros guardados (la vuelta de la BIOS la pone la
    /// propia llamada).
    pub const CARD_FRAME: u32 = 4 + 15 * 4;
    /// Datos guardados (búfer de lectura, `SAVE_LEN` bytes) y los 8 bytes
    /// de récords que se borran si no se cargan.
    pub const SAVE: u32 = 0x10_E3A2;
    pub const SAVE_LEN: u16 = 0x1C;
    pub const SAVE_SCORES: core::ops::Range<u32> = 0x14..0x1C;
    /// Escena: índice de mensaje, mensaje en curso, la copia de las
    /// teclas de inicio y el diálogo (tarea y rutina extra en `+$98`).
    pub const MESSAGE_INDEX: u32 = 0x10_6ED0;
    pub const BUSY: u32 = 0x10_6ED2;
    pub const START_KEYS_SRC: u32 = 0x10_FDB6;
    pub const START_KEYS: u32 = 0x10_E3A0;
    pub const DIALOG: u32 = 0x09_8288;
    pub const EXTRA: u32 = 0x98;
    pub const DIALOG_EXTRA: u32 = 0x05_164A;
    /// Escena de fondo del diálogo (descriptor) y sus velocidades.
    pub const BACKDROP: u32 = 0x29_70F8;
    pub const BACKDROP_IDLE: u32 = 0x04_422E;
    /// Diálogo: fondo (glifo `BLANK`, 0x26 × 0x1B desde `FIX_ORIGIN`),
    /// textos, cursor, tiempo, opción y teclas pulsadas.
    pub const FIX_ORIGIN: u16 = 0x7000;
    pub const BLANK: u16 = 0xFF;
    pub const BG_SIZE: (u16, u16) = (0x26, 0x1B);
    pub const TEXTS: [(u16, u32); 4] = [(0x7204, 0x09_8692), (0x724A, 0x09_869E), (0x724E, 0x09_86A6), (0x7116, 0x09_86AC)];
    pub const PROMPT: u16 = 0x7116;
    pub const PROMPT_SIZE: (u16, u16) = (0x1A, 2);
    pub const DONE_TEXT: (u16, u32) = (0x7152, 0x09_86C8);
    pub const CURSOR_GLYPHS: u32 = 0x09_8700;
    pub const CURSORS: [u16; 2] = [0x720A, 0x720E];
    pub const GLYPH_END: u16 = 0xFFFF;
    pub const GLYPH_STEP: u16 = 0x40;
    pub const TIMER: u32 = 0x70;
    pub const TIMEOUT: u16 = 0x12C;
    pub const DONE_DELAY: u16 = 0x1E;
    pub const CHOICE: u32 = 0x72;
    pub const KEYS: u32 = 0x74;
    /// Teclas de inicio de cada jugador (y si están en juego, 1 o 2).
    pub const PLAYER_KEYS: [u32; 2] = [0x10_FD96, 0x10_FD9C];
    pub const KEY_UP: u8 = 1 << 0;
    pub const KEY_DOWN: u8 = 1 << 1;
    pub const KEY_A: u8 = 1 << 4;
    pub const RUN: u32 = 0x09_8308;
    pub const EXIT_DELAY: u32 = 0x09_83F6;
}

/// Rótulo «MISSION START» / «MISSION COMPLETE» (`system::banner`).
pub mod banner {
    /// Letra: (glifo o `None` = número de la misión, fila, turno de caída,
    /// X e Y de destino).
    pub type Letter = (Option<u8>, u8, u16, u16, u16);
    pub const SCENE: u32 = 0x10_6ECE;
    pub const STAGE: u32 = 0x10_6ECF;
    pub const FINAL_SCENE: u8 = 5;
    pub const SCENE_BUSY: u32 = 0x10_6ED2;
    /// Sonidos por fase al abrir y al completar «START».
    pub const START_SOUNDS: u32 = 0x28_DF00;
    pub const END_SOUNDS: u32 = 0x28_DF10;
    pub const COMPLETE_SOUND: u16 = 0x20;
    pub const COMPLETE_DONE_SOUND: u16 = 0x1137;
    /// Campos de la tarea raíz.
    pub const STATE: u32 = 0x20;
    pub const CLOSING: u8 = 0xFF;
    pub const LANDED: u32 = 0x21;
    pub const KIND: u32 = 0x7B;
    pub const DELAY: u32 = 0x74;
    pub const START_Y: u16 = 0x1FF;
    pub const HOLD_FRAMES: i16 = 0x26;
    pub const CLOSE_FRAMES: u16 = 0x28;
    /// Byte del padre que «START» pone a 0 al acabar.
    pub const OWNER_BUSY: u32 = 0x21;
    pub const FADE_KIND: usize = 0;
    pub const FADE_PARAM: u16 = 2;
    /// Campos de cada letra.
    pub const X: u32 = 0x22;
    pub const Y: u32 = 0x24;
    pub const ATTR: u32 = 0x38;
    pub const FLIP: u32 = 0x3A;
    pub const VEL_X: u32 = 0x28;
    pub const VEL_Y: u32 = 0x2A;
    pub const SHRINK_X: u32 = 0x33;
    pub const SHRINK_Y: u32 = 0x32;
    pub const SPEED: u32 = 0x36;
    pub const PALETTE: u32 = 0x14;
    pub const PALETTE_SRC: u32 = 0x18;
    pub const GLYPH: u32 = 0x20;
    pub const ROW: u32 = 0x21;
    pub const TARGET_X: u32 = 0x70;
    pub const TARGET_Y: u32 = 0x72;
    pub const START_DELAY: u32 = 0x76;
    pub const BLINK: u32 = 0x78;
    pub const BLINK_START: u16 = 0x1FF;
    pub const BLINK_STEP: u16 = 0x18;
    pub const BLINK_GROW: u16 = 0x100;
    pub const LANDED_SOUND: u32 = 0x7A;
    pub const NEAR: i16 = 0x30;
    /// La fila del número parpadea con el bit 3 del contador del rótulo.
    pub const ROW_BLINK: u16 = 1 << 3;
    pub const GFX: u16 = 0x1BE;
    pub const LANDED_GFX: u16 = 0x1AA;
    /// Guiones de las letras (0–13) y de los números (0–5).
    pub const LETTERS: u32 = 0x2D_F684;
    pub const LAST_LETTER: u8 = 13;
    pub const DIGITS: u32 = 0x2D_F71C;
    pub const LAST_DIGIT: u8 = 5;
    /// Salida: punto del que huyen, aceleración y márgenes de la vista.
    pub const AWAY_FROM: (u16, u16) = (0xA0, 0x190);
    pub const SPEED_STEP: u16 = 0x1FF;
    pub const MARGINS: u32 = 0x2D_F734;
    /// Tablas de `atan2`: logaritmo y exponencial.
    pub const LOG: u32 = 0x05_DE18;
    pub const EXP: u32 = 0x05_DF18;
    /// Rutinas.
    pub const START_WAIT: u32 = 0x07_A9C0;
    pub const COMPLETE_WAIT: u32 = 0x07_AA44;
    pub const WAIT: u32 = 0x07_AA78;
    pub const WAIT_RUN: u32 = 0x07_AA7E;
    pub const CLOSE: u32 = 0x07_AA94;
    pub const CLOSE_RUN: u32 = 0x07_AACA;
    pub const LETTER_DROP: u32 = 0x07_AAE0;
    pub const LETTER_DROP_RUN: u32 = 0x07_AAEC;
    pub const DIGIT_DROP: u32 = 0x07_AB5C;
    pub const DIGIT_DROP_RUN: u32 = 0x07_AB68;
    pub const LETTER_TICK: u32 = 0x07_ABD0;
    pub const ATTACH: u32 = 0x07_ABF4;
    pub const ATTACH_RUN: u32 = 0x07_AC14;
    pub const FLY_OFF: u32 = 0x07_AC4A;
    pub const FLY_OFF_RUN: u32 = 0x07_AC56;
    /// Letras de cada rótulo, en el orden en que se crean.
    pub const START: [Letter; 14] = [
        (Some(12), 1, 0x18, 0xDA, 0x178),
        (Some(11), 1, 0x16, 0xC4, 0x178),
        (Some(6), 1, 0x14, 0xAD, 0x178),
        (Some(5), 1, 0x12, 0x94, 0x178),
        (Some(11), 1, 0x10, 0x7E, 0x178),
        (Some(2), 1, 0x0E, 0x64, 0x178),
        (None, 0, 0x0E, 0xF0, 0x1A8),
        (Some(4), 0, 0x0C, 0xD0, 0x1A8),
        (Some(3), 0, 0x0A, 0xB7, 0x1A8),
        (Some(1), 0, 0x08, 0xA4, 0x1A8),
        (Some(2), 0, 0x06, 0x90, 0x1A8),
        (Some(2), 0, 0x04, 0x77, 0x1A8),
        (Some(1), 0, 0x02, 0x64, 0x1A8),
        (Some(0), 0, 0x00, 0x50, 0x1A8),
    ];
    pub const START_FINAL: [Letter; 18] = [
        (Some(12), 1, 0x22, 0xDA, 0x168),
        (Some(11), 1, 0x20, 0xC6, 0x168),
        (Some(6), 1, 0x1E, 0xAF, 0x168),
        (Some(5), 1, 0x1C, 0x96, 0x168),
        (Some(11), 1, 0x1A, 0x80, 0x168),
        (Some(2), 1, 0x18, 0x66, 0x168),
        (Some(4), 0, 0x16, 0xE0, 0x198),
        (Some(3), 0, 0x14, 0xC7, 0x198),
        (Some(1), 0, 0x12, 0xB4, 0x198),
        (Some(2), 0, 0x10, 0xA0, 0x198),
        (Some(2), 0, 0x0E, 0x87, 0x198),
        (Some(1), 0, 0x0C, 0x74, 0x198),
        (Some(0), 0, 0x0A, 0x60, 0x198),
        (Some(9), 0, 0x08, 0xCD, 0x1B8),
        (Some(5), 0, 0x06, 0xB4, 0x1B8),
        (Some(4), 0, 0x04, 0x9C, 0x1B8),
        (Some(1), 0, 0x02, 0x89, 0x1B8),
        (Some(13), 0, 0x00, 0x76, 0x1B8),
    ];
    pub const COMPLETE: [Letter; 17] = [
        (Some(12), 1, 0x20, 0x102, 0x178),
        (Some(10), 1, 0x1E, 0xEC, 0x178),
        (Some(11), 1, 0x1C, 0xD4, 0x178),
        (Some(10), 1, 0x1A, 0xBC, 0x178),
        (Some(9), 1, 0x18, 0xA5, 0x178),
        (Some(8), 1, 0x16, 0x8D, 0x178),
        (Some(0), 1, 0x14, 0x74, 0x178),
        (Some(3), 1, 0x12, 0x5B, 0x178),
        (Some(7), 1, 0x10, 0x42, 0x178),
        (None, 0, 0x0E, 0xF0, 0x1A8),
        (Some(4), 0, 0x0C, 0xD0, 0x1A8),
        (Some(3), 0, 0x0A, 0xB7, 0x1A8),
        (Some(1), 0, 0x08, 0xA4, 0x1A8),
        (Some(2), 0, 0x06, 0x90, 0x1A8),
        (Some(2), 0, 0x04, 0x77, 0x1A8),
        (Some(1), 0, 0x02, 0x64, 0x1A8),
        (Some(0), 0, 0x00, 0x50, 0x1A8),
    ];
    pub const COMPLETE_FINAL: [Letter; 21] = [
        (Some(12), 1, 0x28, 0x104, 0x168),
        (Some(10), 1, 0x26, 0xEE, 0x168),
        (Some(11), 1, 0x24, 0xD6, 0x168),
        (Some(10), 1, 0x22, 0xBE, 0x168),
        (Some(9), 1, 0x20, 0xA7, 0x168),
        (Some(8), 1, 0x1E, 0x8F, 0x168),
        (Some(0), 1, 0x1C, 0x76, 0x168),
        (Some(3), 1, 0x1A, 0x5D, 0x168),
        (Some(7), 1, 0x18, 0x44, 0x168),
        (Some(4), 0, 0x16, 0xE0, 0x198),
        (Some(3), 0, 0x14, 0xC7, 0x198),
        (Some(1), 0, 0x12, 0xB4, 0x198),
        (Some(2), 0, 0x10, 0xA0, 0x198),
        (Some(2), 0, 0x0E, 0x87, 0x198),
        (Some(1), 0, 0x0C, 0x74, 0x198),
        (Some(0), 0, 0x0A, 0x60, 0x198),
        (Some(9), 0, 0x08, 0xCD, 0x1B8),
        (Some(5), 0, 0x06, 0xB4, 0x1B8),
        (Some(4), 0, 0x04, 0x9C, 0x1B8),
        (Some(1), 0, 0x02, 0x89, 0x1B8),
        (Some(13), 0, 0x00, 0x76, 0x1B8),
    ];
}

/// Soldado rebelde (`system::soldier`).
pub mod soldier {
    pub const X: u32 = 0x22;
    pub const Y: u32 = 0x24;
    pub const VEL_X: u32 = 0x28;
    pub const VEL_Y: u32 = 0x2A;
    pub const ACC_X: u32 = 0x2C;
    pub const ACC_Y: u32 = 0x2E;
    pub const ATTR_38: u32 = 0x38;
    pub const FACING: u32 = 0x3A;
    pub const VARIANT_11: u32 = 0x11;
    pub const FLAGS2: u32 = 0x13;
    pub const FLAGS2_UP: u8 = 1 << 4;
    pub const FLAGS2_DOWN: u8 = 1 << 5;
    pub const FLAGS_5A: u32 = 0x5A;
    pub const PROBE_EDGE: u8 = 1 << 5;
    pub const FLAGS_6B: u32 = 0x6B;
    pub const FLAGS_6B_SET: u8 = 1 << 3;
    /// `+$70` semilla (su byte alto decide la variante 2), `+$90` radio.
    pub const SEED: u32 = 0x70;
    pub const RADIUS: u32 = 0x90;
    /// `+$72`: lado, detrás, activo, borde sólido / hueco delante.
    pub const FLAGS_72: u32 = 0x72;
    pub const SIDE: u8 = 1 << 0;
    pub const BEHIND: u8 = 1 << 1;
    pub const ACTIVE: u8 = 1 << 4;
    pub const EDGE_SOLID: u8 = 1 << 5;
    pub const EDGE_GAP: u8 = 1 << 6;
    /// `+$73`: sin objetivo y preferencias.
    pub const FLAGS_73: u32 = 0x73;
    pub const NO_TARGET: u8 = 1 << 0;
    pub const PREF_SET: u8 = 1 << 1;
    pub const PREF_FWD: u8 = 1 << 2;
    pub const PREF_BACK: u8 = 1 << 3;
    /// `+$74`: anclas de agarre (bits 0–2), puede rendirse, huye; `+$75`
    /// subestado.
    pub const FLAGS_74: u32 = 0x74;
    pub const CAN_SURRENDER: u8 = 1 << 3;
    pub const FLEES: u8 = 1 << 4;
    pub const SUBSTATE: u32 = 0x75;
    pub const GROUNDED: u32 = 0x78;
    pub const TARGET: u32 = 0x7A;
    pub const DISTANCE: u32 = 0x7E;
    /// Umbrales y preferencias de movimiento.
    pub const NEAR: u32 = 0x80;
    pub const FAR: u32 = 0x82;
    pub const PREF_A: u32 = 0x84;
    pub const PREF_B: u32 = 0x86;
    pub const PREF_C: u32 = 0x88;
    pub const PREF_D: u32 = 0x8A;
    pub const PREF_FULL: u16 = 0x400;
    pub const PREF_HALF: u16 = 0x80;
    pub const NO_TARGET_PREF: u16 = 0x20;
    pub const TIMER_SURRENDER: u32 = 0x8C;
    pub const TIMER_LEAVE: u32 = 0x8E;
    /// Parámetros de la variante.
    pub const P_KIND: u32 = 0x98;
    pub const P_SURRENDER: u32 = 0x99;
    pub const P_AGGRO: u32 = 0x9A;
    pub const P_LEAVE: u32 = 0x9B;
    pub const P_SHIFT: u32 = 0x9C;
    pub const DEFAULT_AGGRO: u8 = 0x1E;
    pub const DEFAULT_LEAVE: u16 = 0xA;
    pub const PRESET: [(u32, u8); 4] = [(0x98, 2), (0x99, 0xA), (0x9A, 0x1E), (0x9B, 0xA)];
    pub const FLAGS_PRESET: u8 = 1 << 1;
    /// Tablas por dificultad (tipo 2 = 8 valores al azar por fila).
    pub const SURRENDER_TABLE: u32 = 0x2B_74EC;
    pub const AGGRO_TABLE: u32 = 0x2B_746A;
    pub const TABLE_RANDOM: u16 = 2;
    pub const RANDOM_ROWS: u32 = 0x40;
    pub const ACTIVE_PLAYERS: u32 = 0x10_6ED1;
    pub const PROBE_VEL_Y: u16 = 0xFF00;
    pub const GFX: u16 = 0xE;
    pub const ATTR: u16 = 0x8000;
    pub const LAYER_KEEP: u16 = 0xFFE3;
    pub const LAYER: u16 = 0x18;
    pub const CHILD: u32 = 0x07_76E2;
    /// Anillo de posiciones recientes (cabeza, cola, entradas de 4 B).
    pub const RING: u32 = 0x10_E33A;
    pub const RING_SIZE: u16 = 0x20;
    /// Zona de X en la que el soldado se acerca; centro de la pantalla.
    pub const ACTIVE_X: (u16, u16) = (0x30, 0x120);
    pub const CENTER_X: u16 = 0xA0;
    pub const WALK_START: u32 = 0x05_7558;
    pub const FALL_START: u32 = 0x05_8412;
    pub const FACE_FALL: u32 = 0x05_8E08;
    /// Andar: animación, tabla de velocidad por dificultad y velocidad
    /// elegida.
    pub const WALK_ANIM: u32 = 0x29_B744;
    pub const WALK_SPEED_TABLE: u32 = 0x2B_756E;
    pub const SPEED: u32 = 0x36;
    /// Estados (manejadores) del soldado.
    pub const WALK_LOOP: u32 = 0x05_7582;
    pub const AIRBORNE: u32 = 0x05_7706;
    pub const APPROACH_SLUG: u32 = 0x05_7728;
    pub const APPROACH_PLAYER: u32 = 0x05_77BC;
    pub const LEAP_SLUG: u32 = 0x05_78DC;
    pub const LEAP_PLAYER: u32 = 0x05_799E;
    pub const RUN_TOWARD: u32 = 0x05_804C;
    pub const MELEE: u32 = 0x05_85AE;
    pub const BRAKE: u32 = 0x05_85F6;
    pub const FLEE: u32 = 0x05_8658;
    pub const JUMP: u32 = 0x05_88F6;
    pub const SURRENDER: u32 = 0x05_8968;
    pub const EDGE_FLEE: u32 = 0x05_8A50;
    pub const GRENADE_HIGH: u32 = 0x05_8B1E;
    pub const GRENADE_LOW: u32 = 0x05_8C8E;
    pub const LEAVE: u32 = 0x05_9062;
    /// En el aire: uno de cuatro estados al azar.
    pub const AIRBORNE_NEXT: [u32; 4] = [0x05_8A50, 0x05_8658, 0x05_8412, 0x05_8A50];
    /// Muerte en el suelo / en el aire; con el byte a 0 cuenta como golpe.
    pub const DEATH_GROUND: u32 = 0x04_A014;
    pub const DEATH_AIR: u32 = 0x04_A034;
    pub const HITS_LIVE: u32 = 0x10_6ED3;
    /// Márgenes de la pantalla para desaparecer.
    pub const DESPAWN_MARGINS: u32 = 0x2B_71D4;
    /// Cuerpo a cuerpo: lista de cajas de ataque y zona de X en que se prueba.
    pub const MELEE_ATTACKS: u32 = 0x05_7440;
    pub const ATTACKS: u32 = 0x4C;
    pub const MELEE_HIT: u8 = 1 << 1;
    pub const MELEE_X: (u16, u16) = (0x20, 0x300);
    /// Gravedad propia y la de reserva que se intercambian al moverse
    /// (`+$60` largo = sin guardar).
    pub const GRAVITY: u32 = 0x34;
    pub const GRAVITY_SAVE: u32 = 0x60;
    pub const GRAVITY_ALT: u32 = 0x62;
    /// Columna del borde de plataforma que tiene delante.
    pub const EDGE_COL: u32 = 0x77;
    pub const SUB_X: u32 = 0x26;
    /// Cámara (para pasar a coordenadas del mundo, con la Y hacia arriba).
    pub const CAMERA_X: u32 = 0x10_6F50;
    pub const CAMERA_Y: u32 = 0x10_6F54;
    pub const WORLD_Y: u16 = 0x200;
    /// Listas de bordes de cada paridad de frame (entradas de 6 B: x, y,
    /// alto.b, columna.b; recuento en `+$30`).
    pub const FRAME: u32 = 0x10_6F28;
    pub const EDGES: [u32; 2] = [0x10_E2AE, 0x10_E27C];
    pub const EDGE_COUNT: u32 = 0x30;
    pub const EDGE_LEN: u16 = 6;
    /// Anillo de zonas en que no se rinde (cabeza, cola; entradas de 8 B:
    /// x0, y0, x1, y1).
    pub const ZONES: u32 = 0x10_E2F2;
    pub const ZONE_LEN: u16 = 8;
    pub const ZONES_SIZE: u16 = 0x40;
    /// El Slug: sus anclas de agarre (`+$70` → tabla de 3 desplazamientos)
    /// y las ocupadas (`+$96` / `+$97`).
    pub const SLUG: u32 = 0x10_0580;
    pub const SLUG_ANCHORS: u32 = 0x70;
    pub const SLUG_TAKEN: [u32; 2] = [0x96, 0x97];
    pub const PLAYERS: [u32; 2] = [0x10_0440, 0x10_04E0];
    pub const ANCHOR_MASK: u8 = 7;
    /// Más estados.
    pub const RUN_LOOP: u32 = 0x05_809E;
    pub const IDLE: u32 = 0x05_8144;
    pub const IDLE_LOOP: u32 = 0x05_8196;
    pub const RUN_BY_TABLE: u32 = 0x05_8076;
    pub const STEP_LEFT: u32 = 0x05_8500;
    pub const STEP_RIGHT: u32 = 0x05_84C4;
    pub const STEP_LOOP: u32 = 0x05_8538;
    pub const FIDGET: u32 = 0x05_83CC;
    pub const FIDGET_LOOP: u32 = 0x05_83EC;
    pub const HURT_LOOP: u32 = 0x05_8424;
    pub const LAND: u32 = 0x05_8464;
    pub const LAND_LOOP: u32 = 0x05_8494;
    pub const MELEE_LOOP: u32 = 0x05_85D8;
    pub const BRAKE_LOOP: u32 = 0x05_8608;
    pub const FLEE_LOOP: u32 = 0x05_8682;
    pub const FLEE_STOP: u32 = 0x05_86B8;
    pub const FLEE_STOP_LOOP: u32 = 0x05_86CE;
    pub const RETREAT_TURN: u32 = 0x05_879E;
    pub const RETREAT: u32 = 0x05_87AC;
    pub const RETREAT_LOOP: u32 = 0x05_8800;
    /// Retirada: animación y velocidades (hacia delante, ×2 si no está del
    /// lado del jugador; reculando).
    pub const ANIM_RETREAT: u32 = 0x2B_5BCC;
    pub const RETREAT_SPEED: u16 = 0x100;
    pub const RETREAT_BACK_SPEED: u16 = 0xFF00;
    pub const STAND: u32 = 0x05_88AE;
    pub const STAND_LOOP: u32 = 0x05_88C4;
    pub const SURRENDER_LOOP: u32 = 0x05_89C8;
    pub const SURRENDER_FLEE: u32 = 0x05_89F8;
    pub const SURRENDER_FLEE_LOOP: u32 = 0x05_8A20;
    pub const EDGE_FLEE_LOOP: u32 = 0x05_8A82;
    pub const HOP_FLEE: u32 = 0x05_8AA8;
    pub const HOP_FLEE_LOOP: u32 = 0x05_8AD2;
    pub const TAUNT: u32 = 0x05_8E08;
    /// Burla: guiones al azar (4), el bucle, el final, la marca de «ya
    /// sin objetivo» y las vueltas tras darse de baja.
    pub const TAUNT_ANIMS: u32 = 0x05_8DF8;
    pub const TAUNT_ANIM_MASK: u16 = 0xC;
    pub const TAUNT_LOOP: u32 = 0x05_8E14;
    pub const TAUNT_LOOP_RUN: u32 = 0x05_8E32;
    pub const TAUNT_END: u32 = 0x05_8ED2;
    pub const TAUNT_END_LOOP: u32 = 0x05_8EE4;
    pub const ANIM_TAUNT_END: u32 = 0x29_BFC4;
    pub const TAUNT_LOST: u32 = 0x30;
    pub const AFTER_DESPAWN_TAUNT: u32 = 0x05_8ED0;
    pub const AFTER_DESPAWN_TAUNT_END: u32 = 0x05_8F1C;
    pub const SPAWN_STAND: u32 = 0x05_91C8;
    /// Muerte en el suelo de los estados de huida (la variante 1).
    pub const DEATH_GROUND_B: u32 = 0x04_A024;
    /// Animaciones de cada estado.
    pub const ANIM_RUN: u32 = 0x29_B7C8;
    pub const ANIM_RUN_TABLE: u32 = 0x2B_5D14;
    pub const ANIM_IDLE: u32 = 0x29_B816;
    pub const ANIM_IDLE_ALT: u32 = 0x2B_5F28;
    pub const ANIM_FIDGET: u32 = 0x29_BF34;
    pub const ANIM_HURT: u32 = 0x2B_5B92;
    pub const ANIM_LAND_HARD: u32 = 0x2B_7030;
    pub const ANIM_LAND: u32 = 0x2B_5C22;
    pub const ANIM_STEP_RIGHT: [u32; 2] = [0x2B_5C56, 0x2B_5F56];
    pub const ANIM_STEP_LEFT: [u32; 2] = [0x2B_5CA8, 0x2B_5FD0];
    pub const ANIM_MELEE: u32 = 0x2B_70D2;
    pub const MELEE_BOXES: u32 = 0x05_7494;
    pub const ANIM_BRAKE: u32 = 0x2B_5CFA;
    pub const ANIM_FLEE: u32 = 0x29_B956;
    pub const ANIM_FLEE_STOP: u32 = 0x2B_5C10;
    pub const ANIM_STAND: u32 = 0x29_BD84;
    pub const ANIM_JUMP_FWD: u32 = 0x2B_5D58;
    pub const ANIM_JUMP_BACK: u32 = 0x2B_5E68;
    pub const ANIM_SURRENDER: [u32; 3] = [0x29_BA70, 0x29_BBEC, 0x29_BC64];
    pub const ANIM_SURRENDER_FLEE: u32 = 0x29_BB4C;
    pub const ANIM_HOP_FLEE: u32 = 0x29_B9CE;
    /// Pose (`+$5C`) según hacia dónde mire y la altura del objetivo.
    pub const POSE: u32 = 0x5C;
    pub const POSES: [[u8; 3]; 2] = [[0x04, 0x05, 0x10], [0x11, 0x12, 0x13]];
    pub const POSE_LOW_Y: u16 = 0x180;
    pub const POSE_ABOVE: i16 = 0x60;
    pub const POSE_BELOW: i16 = -0x40;
    pub const POSE_SURRENDER_FLEE: u8 = 6;
    /// Salto hacia atrás al ver un hueco.
    pub const EDGE_JUMP: (u16, u16, u16) = (0xFE67, 0x65E, 0xFF5D);
    pub const RUN_SPEED: u16 = 0x100;
    pub const STEP_SPEED: u16 = 0x80;
    pub const HOP_SPEED: u16 = 0x190;
    pub const BACK_SPEED: u16 = 0xFF00;
    /// Baja de la tarea, y la vuelta que dejan en la pila los estados que
    /// llaman a la baja con `bsr` (la tarea muere sin desapilarla).
    pub const TASK_FINISH: u32 = 0x00_0518;
    pub const AFTER_DESPAWN_WALK: u32 = 0x05_7704;
    pub const AFTER_DESPAWN_APPROACH_SLUG: u32 = 0x05_77BA;
    pub const AFTER_DESPAWN_APPROACH_PLAYER: u32 = 0x05_783C;
    pub const AFTER_DESPAWN_IDLE: u32 = 0x05_83CA;
    pub const AFTER_DESPAWN_FIDGET: u32 = 0x05_8410;
    pub const AFTER_DESPAWN_HURT: u32 = 0x05_8452;
    pub const AFTER_DESPAWN_BRAKE: u32 = 0x05_8656;
    pub const AFTER_DESPAWN_FLEE: u32 = 0x05_86B6;
    pub const AFTER_DESPAWN_FLEE_STOP: u32 = 0x05_879C;
    pub const AFTER_DESPAWN_RETREAT: u32 = 0x05_88AC;
    pub const AFTER_DESPAWN_STAND: u32 = 0x05_88F4;
    pub const AFTER_DESPAWN_EDGE_FLEE: u32 = 0x05_8AA6;
    pub const AFTER_DESPAWN_HOP_FLEE: u32 = 0x05_8B1C;
    /// Apariciones (`$058F1E`..`$059330`): marca de `+$12`, gráficos,
    /// guiones, tiempos y estados.
    pub const FLAGS_12: u32 = 0x12;
    pub const SPAWNED: u8 = 1 << 1;
    pub const SPAWN_KIND: u8 = 1;
    pub const SPAWN_SOUND: u16 = 0xFFFF;
    pub const SPAWN_SPEED_TABLE: u32 = 0x2B_75D0;
    pub const ANIM_SPAWN_ENTER: [u32; 2] = [0x29_B744, 0x2B_604A];
    pub const ANIM_SPAWN_WAIT: u32 = 0x2B_6EF8;
    pub const ANIM_SPAWN_STAND: [u32; 2] = [0x29_BD84, 0x29_BD40];
    pub const ANIM_SPAWN_BRAKE: u32 = 0x2B_5CFA;
    pub const ANIM_SPAWN_LEAP: u32 = 0x2B_5D58;
    pub const ANIM_SPAWN_HURT: u32 = 0x2B_5B92;
    pub const ANIM_FALL_FAST_X: u32 = 0x29_B7C8;
    pub const ANIM_FALL_FAST_Y: u32 = 0x2B_5C22;
    pub const SPAWN_WAIT_FRAMES: u8 = 0xB4;
    pub const COUNTER: u32 = 0x5C;
    pub const LEAP_VEL: (u16, u16, u16) = (0xFE15, 0x7AA, 0xFF63);
    pub const SPAWN_ENTER: u32 = 0x05_9086;
    pub const SPAWN_ENTER_LOOP: u32 = 0x05_90CA;
    pub const SPAWN_WAIT: u32 = 0x05_9146;
    pub const SPAWN_WAIT_LOOP: u32 = 0x05_9168;
    pub const SPAWN_STAND_LOOP: u32 = 0x05_91F4;
    pub const SPAWN_BRAKE: u32 = 0x05_9228;
    pub const SPAWN_BRAKE_LOOP: u32 = 0x05_923A;
    pub const SPAWN_LEAP: u32 = 0x05_925A;
    pub const SPAWN_LEAP_LOOP: u32 = 0x05_928C;
    pub const SPAWN_HURT: u32 = 0x05_92B2;
    pub const SPAWN_HURT_LOOP: u32 = 0x05_92C8;
    pub const SPAWN_RECOVER: u32 = 0x05_92F6;
    pub const SPAWN_RECOVER_LOOP: u32 = 0x05_9304;
    pub const AFTER_DESPAWN_SPAWN_ENTER: u32 = 0x05_9144;
    pub const AFTER_DESPAWN_SPAWN_WAIT: u32 = 0x05_91C6;
    pub const AFTER_DESPAWN_SPAWN_STAND: u32 = 0x05_9226;
    pub const AFTER_DESPAWN_SPAWN_BRAKE: u32 = 0x05_9258;
    pub const AFTER_DESPAWN_SPAWN_LEAP: u32 = 0x05_92B0;
    pub const AFTER_DESPAWN_SPAWN_HURT: u32 = 0x05_92F4;
    pub const AFTER_DESPAWN_SPAWN_RECOVER: u32 = 0x05_9330;
}
