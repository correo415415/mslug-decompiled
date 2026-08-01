/*
 * Metal Slug 1 — Cabecera principal de la decompilación (C puro)
 * ================================================================
 * Tipos, direcciones absolutas y prototipos utilizados por el código C
 * reconstruido. Todo lo declarado aquí debe existir realmente en la ROM
 * original (verificado con Ghidra o análisis manual del binario).
 *
 * Convenciones de codegen:
 *   - Se compila con `-mpcrel` para que `lea` y `jsr` de símbolos cercanos
 *     usen el modo PC-relativo corto (misma forma que el ROM original).
 *   - `fp` (frame pointer del task actual) vive en el registro A6, tal como
 *     lo mantiene el compilador Nazca/SN Systems original.
 *   - `_a0_ptr` es un alias de A0 usado para forzar el patrón
 *     `lea addr,a0 ; move.l a0,dst` en vez de `move.l #addr,dst`.
 *   - Para evitar la fusión de `clr.w` contiguos en `clr.l` (que GCC hace
 *     agresivamente con -Os), separamos stores con `TASK_BARRIER()`.
 */
#ifndef MSLUG_H
#define MSLUG_H

/* ---- Tipos básicos ---------------------------------------------------- */
typedef unsigned char       u8;
typedef unsigned short      u16;
typedef unsigned int        u32;
typedef signed   char       s8;
typedef signed   short      s16;
typedef signed   int        s32;

/* ---- Task del juego --------------------------------------------------- */
/* Nodo de la lista doblemente enlazada de tasks activos. Descubierto en el
 * análisis de FUN_00000518 y familia. Tamaño real >= 0x70 bytes; los
 * offsets se van rellenando a medida que se decompilan las rutinas que los
 * tocan. Por ahora lo dejamos como buffer opaco y accedemos con macros. */
typedef struct Task {
    u8 _raw[0x70];
} Task;

/* Acceso a campos por offset numérico (útil mientras no conocemos todos
 * los nombres). Al ser un cast a puntero no-volátil, GCC puede fusionar
 * dos escrituras contiguas si no las separamos con TASK_BARRIER(). */
#define TASK_W(off)  (*(u16 *)((u8 *)fp + (off)))
#define TASK_L(off)  (*(u32 *)((u8 *)fp + (off)))
#define TASK_B(off)  (*(u8  *)((u8 *)fp + (off)))

/* Barrera de compilador (no emite código) — evita que GCC fusione stores
 * consecutivos en el mismo objeto. Es el equivalente C de un "; " entre
 * instrucciones que el compilador original mantenía separadas. */
#define TASK_BARRIER() __asm__ volatile("" ::: "memory")

/* ---- Registros dedicados (convención Nazca/SN Systems) ---------------- */
/* fp = task actual, siempre en A6. Este es el único register global
 * omnipresente: está vivo en todas las funciones "de task" del juego. */
register Task *fp __asm__("a6");

/* Los alias A0/A1/A2/D0 se declaran sólo en las unidades que los usan,
 * vía `#define USE_A0` etc. antes de `#include "mslug_regs.h"`. Meterlos
 * globalmente en este header hace que GCC 13 salve/restaure registros
 * "por si acaso" en funciones cortas y rompa el matching bit-a-bit. */

/* ---- Direcciones de RAM globales del juego ---------------------------- */
/* $106EA8 : callback de VBlank instalado por el juego. */
#define g_vblank_callback   (*(void (**)(void))0x00106EA8)

/* $106EAC : "modo LSPC" (0 = normal). */
#define g_lspc_mode         (*(volatile u16 *)0x00106EAC)

/* ---- Variables del BIOS (work area en $10F300..$10FFFF) --------------- */
#define BIOS_PLAYER_MOD1    (*(volatile u8  *)0x0010FD82)

/* ---- Rutinas del BIOS ($C00000+) — llamadas por absolute long JSR ---- */
extern void BIOS_FIX_CLEAR(void);          /* $C004C2 */

/* ---- Rutinas del juego con dirección conocida ------------------------- */
extern void VBlankCallbackDefault(void);   /* $0008F2 (stub rts) */
extern void FUN_000020e2(void);
extern void FUN_0000212e(void);
extern void FUN_00099afc(void);
extern void FUN_00028108(void);   /* target del tail-call de EntitySetField38AndUpdate */
extern void StateMachineRun(void); /* $0005022A: runtime común de state machine */

/* ---- Funciones decompiladas en C puro (ya matched bit-a-bit) ---------- */
void ResetIrqCallback(void);            /* $0009A8, 12 B */
void EntityResetState(void);            /* $0267E2, 18 B */
void GameFrame(void);                   /* $00097A, 46 B */
void EntitySetField38AndUpdate(void);   /* $028134,  8 B */
void EntitySetSpriteMap(void);          /* $028CD4, 28 B */
void ClampD0ToRange(void);              /* $00219C, 10 B */
void InputGuardCall219c(void);          /* $002352, 28 B */

/* ========================================================================
 * Entity / Sprite subsystem
 * ------------------------------------------------------------------------
 * Tipos y constantes consolidados a partir de las funciones Wave S ya
 * matcheadas. Cada campo listado esta corroborado por al menos una
 * funcion decompilada; los offsets que aun no se han visto en la ROM se
 * marcan como pad y llevaran nombre cuando se descubran.
 * ======================================================================== */

/* Centinela usado en toda la ROM para marcar "slot vacio" o "sin cota":
 * un puntero de 32 bits con todos los bits a 1. Aparece explicitamente
 * en Table_LookupPointerBounded, Entity_HasLinkedSlots y en $0004ae. */
#define ENTITY_NIL          ((void *)0xFFFFFFFFUL)

/* Layout tentativo de una entidad del juego. Solo los campos observados
 * por las funciones Wave S estan nombrados; el resto es padding.
 *
 * Evidencias por campo:
 *   +0x11 flags11 : Entity_CopyTransform ($05dd02) copia como byte
 *   +0x22 pos_x   : Entity_CopyTransform copia como word
 *   +0x24 pos_y   : Entity_CopyTransform copia como word
 *   +0x38 flags38 : Entity_CopyTransform copia como word
 *   +0x3a flags3a : Entity_CopyTransform copia como byte
 *   +0x3c slot_parent : Entity_HasLinkedSlots ($028d70) compara con NIL
 *   +0x40 slot_child  : Entity_HasLinkedSlots compara con NIL
 *
 * Nota: coexiste deliberadamente con `typedef struct Task { u8 _raw[0x70]; }`
 * porque ambos apuntan a la misma zona de memoria; el nombre Task se usa
 * en el scheduler y funciones de servicio, mientras que Entity refleja la
 * vista semantica cuando el codigo lee/escribe campos concretos. */
typedef struct Entity {
    u8   _pad00[0x11];            /* +0x00..+0x10  aun no observados */
    u8   flags11;                  /* +0x11 flag/estado corto */
    u8   _pad12[0x22 - 0x12];      /* +0x12..+0x21 */
    s16  pos_x;                    /* +0x22 coordenada X (16 bits con signo) */
    s16  pos_y;                    /* +0x24 coordenada Y */
    u8   _pad26[0x38 - 0x26];      /* +0x26..+0x37 */
    u16  flags38;                  /* +0x38 flags visuales/animacion */
    u8   flags3a;                  /* +0x3a flag corto */
    u8   _pad3b[0x3c - 0x3b];      /* +0x3b */
    void *slot_parent;             /* +0x3c NIL si no enlazada arriba */
    void *slot_child;              /* +0x40 NIL si no enlazada abajo */
    u8   _pad44[0x70 - 0x44];      /* +0x44..+0x6f resto del task-frame */
} Entity;
_Static_assert(sizeof(Entity) == 0x70, "Entity debe compartir tamano con Task");
_Static_assert(__builtin_offsetof(Entity, flags11)     == 0x11, "Entity.flags11");
_Static_assert(__builtin_offsetof(Entity, pos_x)       == 0x22, "Entity.pos_x");
_Static_assert(__builtin_offsetof(Entity, pos_y)       == 0x24, "Entity.pos_y");
_Static_assert(__builtin_offsetof(Entity, flags38)     == 0x38, "Entity.flags38");
_Static_assert(__builtin_offsetof(Entity, flags3a)     == 0x3a, "Entity.flags3a");
_Static_assert(__builtin_offsetof(Entity, slot_parent) == 0x3c, "Entity.slot_parent");
_Static_assert(__builtin_offsetof(Entity, slot_child)  == 0x40, "Entity.slot_child");

/* Bloque de parametros consumido por Sprite_InvokeBlit8Params ($05022a):
 * la funcion carga dos punteros (a0 y a1) y seis words (d0..d5) desde
 * campos consecutivos apuntados por a2, y salta al blit en $51de2. */
typedef struct SpriteCmd {
    void *ptr0;                    /* +0x00 -> a0 al llamar a $51de2 */
    void *ptr1;                    /* +0x04 -> a1 */
    u16   w0;                      /* +0x08 -> d0 */
    u16   w1;                      /* +0x0a -> d1 */
    u16   w2;                      /* +0x0c -> d2 */
    u16   w3;                      /* +0x0e -> d3 */
    u16   w4;                      /* +0x10 -> d4 */
    u16   w5;                      /* +0x12 -> d5 */
} SpriteCmd;
_Static_assert(sizeof(SpriteCmd) == 0x14, "SpriteCmd 20 bytes");

/* ======================================================================
 *  Estructuras inferidas en Waves HH / II
 * ====================================================================== */

/* Descriptor de escena. Tabla de 256 entradas en $916C8, indexada por
 * SceneLoader_Main_043568 (HH#1) con `(idx & $FF) * 8`.
 *   config_ptr   -> publicado en $10815C (config global de la escena)
 *   entry_script -> bytecode de entities recorrido en dos pasadas */
typedef struct SceneDescriptor {
    void *config_ptr;              /* +0x00 -> $10815C */
    void *entry_script;            /* +0x04 -> a0 del interprete */
} SceneDescriptor;
_Static_assert(sizeof(SceneDescriptor) == 0x08, "SceneDescriptor 8 bytes");

/* Entrada del bytecode de escena (HH#1). Stride $E; la lista termina
 * cuando type == 2. type == 0 usa el spawner "tipo A" ($13982), cualquier
 * otro valor no-cero usa el "tipo B" ($13952). */
typedef struct SceneEntry {
    u8    type;                    /* +0x00  0 = A, 2 = fin, otro = B */
    u8    subop;                   /* +0x01  pasado a $1390E */
    void *template_ptr;            /* +0x02  template de entity */
    u8    payload[8];              /* +0x06  params para $51ABE (a1) */
} SceneEntry;
_Static_assert(sizeof(SceneEntry) == 0x0e, "SceneEntry 14 bytes");

/* Entrada de lista de tiles del Fix Layer (II#1/II#2). La consumen
 * ListCursor_Reinit_05DBC2 (pintar, backend $5DA56) y
 * ListCursor_ReinitClipped_05DBDC (borrar, backend $5DA9C).
 * Centinela de fin de lista: tile == $FFFF. */
typedef struct ListEntry {
    u16 tile;                      /* +0x00 -> d0 ($FFFF = fin) */
    u16 cols;                      /* +0x02 -> d1 */
    u16 rows;                      /* +0x04 -> d2 */
} ListEntry;
_Static_assert(sizeof(ListEntry) == 0x06, "ListEntry 6 bytes");

/* Entrada de lista de sprites del attract mode (HH#2 / corregido en II#2).
 * Stride $14, centinela $FFFF en el primer word. Las 8 listas viven en
 * $096BBC..$0975A2 y se seleccionan con ClampAndLookup8_096B7E, que NO es
 * un dispatcher de codigo sino un table-of-tables indexado por el indice
 * de escena attract en $21(a6). Iteradas por AttractCuller_Cam0/Cam1. */
typedef struct AttractSprite {
    u16 kind;                      /* +0x00 ($FFFF = fin de lista) */
    u16 x_world;                   /* +0x02 test de viewport */
    u8  data[0x10];                /* +0x04 payload para $5DCCE */
} AttractSprite;
_Static_assert(sizeof(AttractSprite) == 0x14, "AttractSprite 20 bytes");

/* Campos del contexto activo (a6) confirmados en Waves II#1 e II#2.
 * No es una struct completa todavia: son los offsets verificados por
 * varias funciones independientes. Se consolidara cuando se cierren
 * mas consumidores del contexto.
 *   $08  void *linked      puntero parent/sibling (CompareField10)
 *   $10  u8    order_key   clave de comparacion/prioridad
 *   $22  u16   vram        offset Fix Layer, cargado con movea.w
 *   $3C  void *list_ptr    descriptor de lista activa (ListEntry*)
 *   $46  u16   list_size   numero de entradas de la lista */

/* Slot de jugador (II#1). Bases conocidas: $100440 (P1), $1004E0 (P2).
 * El primer long actua como guarda de validez: $FFFFFFFF, $52A y $400
 * marcan el slot como no utilizable (SlotExtractCoords_05E2D8). */
typedef struct PlayerSlot {
    u32 state_or_magic;            /* +0x00 guardas de validez */
    u8  pad02[0x1e];               /* +0x04 */
    u16 x;                         /* +0x22 coordenada mundo X */
    u16 y;                         /* +0x24 coordenada mundo Y */
    u8  pad26[0x35];               /* +0x26 */
    u8  flags;                     /* +0x5B bit 7 = coordenada Y valida */
} PlayerSlot;
_Static_assert(__builtin_offsetof(PlayerSlot, x)     == 0x22, "PlayerSlot.x");
_Static_assert(__builtin_offsetof(PlayerSlot, y)     == 0x24, "PlayerSlot.y");
_Static_assert(__builtin_offsetof(PlayerSlot, flags) == 0x5b, "PlayerSlot.flags");

/* Los 4 sistemas de camara del engine (CC#1, cerrados en II#2).
 * Bases: $106F6C, $107FE8, $108064, $1080E0 (stride $7C).
 * El campo +$E actua como flag "sistema activo": Reset4CameraLongs_043D6C
 * lo pone a 0 en los cuatro, y CameraApplyAll4_043D86 lo testea con
 * `tst.l $E(a0)` antes de aplicar la transformacion. */
#define CAMERA_SYSTEM_COUNT   4
#define CAMERA_SYSTEM_STRIDE  0x7c
#define CAMERA_ACTIVE_OFFSET  0x0e

/* Campos del sistema de camara confirmados en Wave JJ#1 por
 * CameraApplyOne_043DAA y sus tres hooks de probe.
 *   +$0E  u32   active    gate; 0 = sistema desactivado
 *   +$72  u8    flags     bit0 -> hook Probe82, bit1 -> hook ProbeF6
 *   +$74  u16   scale_x   multiplicador de hud_x (punto fijo 8.8)
 *   +$76  u16   scale_y   multiplicador de hud_y (punto fijo 8.8)
 *   +$78  void *linked    enlace leido por los tres hooks */
#define CAMERA_FLAGS_OFFSET   0x72
#define CAMERA_SCALE_X_OFFSET 0x74
#define CAMERA_SCALE_Y_OFFSET 0x76
#define CAMERA_LINKED_OFFSET  0x78
#define CAMERA_FLAG_HOOK_B    0x01
#define CAMERA_FLAG_HOOK_C    0x02

/* ======================================================================
 *  Sprites hardware del Neo Geo (Wave JJ#2)
 * ======================================================================
 * El hardware ofrece 381 sprites (indices 0..$17C) descritos en cuatro
 * bancos de VRAM accedidos por el puerto MMIO $3C0000 (direccion) /
 * $3C0002 (dato) / $3C0004 (autoincremento):
 *
 *   SCB1  $0000..$7FFF   tile map del sprite (64 tiles por sprite)
 *   SCB2  $8000..$81FF   coeficiente de shrink ($FFF = sin reduccion)
 *   SCB3  $8200..$83FF   Y, bit sticky (bit 6) y altura (bits 5..0)
 *   SCB4  $8400..$85FF   X
 *
 * El bit sticky encadena un sprite al anterior: un objeto ancho se compone
 * de N sprites contiguos, el primero no-sticky y los N-1 restantes sticky.
 * SpriteRange_InitChain_0139BE implementa exactamente ese armado. */
#define NEOGEO_SPRITE_COUNT    0x17d   /* 381 sprites (0..$17C)        */
#define NEOGEO_SPRITE_LAST     0x17c
#define NEOGEO_VRAM_ADDR_PORT  0x3c0000
#define NEOGEO_VRAM_DATA_PORT  0x3c0002
#define NEOGEO_VRAM_INC_PORT   0x3c0004
#define NEOGEO_SCB2_BASE       0x8000  /* shrink                        */
#define NEOGEO_SCB3_BASE       0x8200  /* Y / sticky / altura           */
#define NEOGEO_SCB4_BASE       0x8400  /* X                             */
#define NEOGEO_SCB3_STICKY     0x0040  /* bit 6: encadenar al anterior  */
#define NEOGEO_SHRINK_NONE     0x0fff  /* SCB2 neutro (sin reduccion)   */

/* Globales del asignador de sprites (Scratch_Alloc_01390E, Wave JJ#2).
 * Los 381 sprites se reparten en DOS POOLS que crecen en sentidos
 * opuestos: el pool B desde 0 hacia arriba y el pool A desde la marca
 * `380 - head - mid` hacia arriba. El bytecode de escena (HH#1) manda las
 * entradas con type==0 al pool A y el resto al pool B. */
#define SPRITE_ALLOC_UNK_F4    0x10e1f4  /* reservado, reset a 0        */
#define SPRITE_ALLOC_POOLB_LIM 0x10e1f6  /* limite superior del pool B  */
#define SPRITE_ALLOC_MARK_F8   0x10e1f8  /* 380 - head                  */
#define SPRITE_ALLOC_MARK_FA   0x10e1fa  /* 380 - head - mid            */
#define SPRITE_ALLOC_POOLA_CUR 0x10e1fc  /* cursor del pool A           */
#define SPRITE_ALLOC_POOLB_CUR 0x10e1fe  /* cursor del pool B           */

/* ---- Prototipos Wave S (asm 68000 puro, ABI por registros absolutos) --
 * Estas funciones se implementan en asm/*.s y no siguen el ABI de GCC:
 * los parametros van en registros fijos (a2, a6, a0, ...). Los prototipos
 * de aqui son puramente documentales para que el resto del arbol C sepa
 * que el simbolo existe y pueda referenciarlo por nombre. NO se puede
 * llamarlas directamente desde C sin envoltorio. */
extern void Sprite_InvokeBlit8Params(void);   /* $05022A, a2 = SpriteCmd* */
extern void Entity_HasLinkedSlots(void);      /* $028D70, a6 = Entity*  */
extern void Table_LookupPointerBounded(void); /* $000772, a0/a1/a6/d0    */
extern void Entity_CopyTransform(void);       /* $05DD02, a0 = dst, a6 = src */

/* ---- Referencias externas resueltas por symbols.py -------------------- */
extern void Sub_028d8e(void);                  /* $028D8E script interpreter */

#endif /* MSLUG_H */
