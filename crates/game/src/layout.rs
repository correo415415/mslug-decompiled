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
    /// Pendientes de 45° con la velocidad vertical ×4.
    pub const STEEP_UP: u16 = 0x1000;
    pub const STEEP_DOWN: u16 = 0xF000;
    pub const ID: u32 = 0x6C;
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

/// Objetos del suelo y test de fuera de la vista (`system::items`).
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
}

/// Recogida de objetos (`system::pickup`).
pub mod pickup {
    pub const X: u32 = 0x22;
    pub const Y: u32 = 0x24;
    /// `FLAGS2` bits 0 y 3: la entidad no toca objetos.
    pub const UNTOUCHABLE: u8 = (1 << 3) | (1 << 0);
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
}

/// Foco de la cámara: los dos jugadores y la ventana en la que la cámara
/// los mantiene.
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
    /// Campo de movimiento que se pone a 0.
    pub const MOVE_FIELD: u32 = 0x2C;
    /// Palabras de movimiento que borra `$0267E2` y el largo que va entre ellas.
    pub const MOTION_WORDS: [u32; 3] = [0x2E, 0x2C, 0x26];
    pub const MOTION_LONG: u32 = 0x28;
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
