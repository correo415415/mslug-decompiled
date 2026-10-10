/*
 * Metal Slug 1 — Stubs `rts` triviales (2 bytes cada uno)
 * =========================================================
 * Funciones que en el ROM original consisten en un único `rts` (opcode
 * $4E75). Son puntos de retorno vacíos usados como:
 *   - callbacks por defecto (p. ej. VBlankCallbackDefault en $0008F2)
 *   - "targets de fallthrough" al final de bloques condicionales
 *   - handlers de eventos aún no implementados en esa versión del juego
 *
 * Todos comparten el mismo cuerpo: `void f(void) {}` → GCC emite `4E75`.
 * Lo único que cambia entre ellos es la **dirección** donde deben vivir,
 * que se fija con `__attribute__((section(".text.NombreÚnico")))` y luego
 * el linker las coloca en su offset exacto.
 *
 * A medida que se decompilen las funciones que los llaman, muchos de estos
 * stubs pasarán a tener un nombre semántico real y podrán mudarse a su
 * archivo correspondiente (ej: VBlankCallbackDefault podría ser un handler
 * de VBlank que en versiones posteriores del juego sí hace trabajo).
 */

#include "mslug.h"

/* Cada función va en su propia sección .text.<Nombre> para que el matcher
 * unitario pueda extraerla y el linker maestro pueda ubicarla en la
 * dirección CPU correspondiente indicada en el registry. */

#define STUB_RTS(name)                                       \
    __attribute__((section(".text." #name)))                 \
    void name(void) {}

/* --- $0008F2 : callback de VBlank por defecto ------------------------- */
/* Target de ResetIrqCallback. Consume el evento sin hacer nada. */
STUB_RTS(VBlankCallbackDefault)

/* NOTA: $0009A6 (GameFrame_EarlyExit) NO es una función independiente,
 *       es el rts final de GameFrame — el destino del beq.w interno.
 *       Vive dentro de GameFrame y no debe registrarse aparte. */

/* --- Resto de stubs rts triviales (nombres provisionales) ------------- */
/* Direcciones extraídas del análisis previo (archivadas en
 * _archive_asm_naked/trivial_rts.c). Todos son `void f(void){}`. */
STUB_RTS(Stub_00024FB6)
STUB_RTS(Stub_00025880)
STUB_RTS(Stub_00030702)
STUB_RTS(Stub_00032E8E)
STUB_RTS(Stub_00032F1C)
STUB_RTS(Stub_00032F3A)
STUB_RTS(Stub_00032FF0)
STUB_RTS(Stub_000332BA)
STUB_RTS(Stub_0003DF54)
STUB_RTS(Stub_0003EE1C)
STUB_RTS(Stub_000423EA)
STUB_RTS(Stub_000434CE)
STUB_RTS(Stub_000434DC)
STUB_RTS(Stub_00044558)
STUB_RTS(Stub_000448A4)
STUB_RTS(Stub_0004698A)
STUB_RTS(Stub_000469CC)
STUB_RTS(Stub_0004FB3A)
/* Stub_00051C80 absorbido por Collision_ProbeRange_051C08 (Wave KK#2).
 * FP #48: los 2 B en $051C80 son el `rts` de la rama "sin colision"
 * alcanzada por `bcc.w` desde $051C38, no un stub independiente. */
STUB_RTS(Stub_0005204E)
STUB_RTS(Stub_0005E8B8)
STUB_RTS(Stub_0005EA94)
STUB_RTS(Stub_00060E44)
STUB_RTS(Stub_00068B46)
STUB_RTS(Stub_0006EF0E)
STUB_RTS(Stub_00072DE8)
STUB_RTS(Stub_00077C26)
STUB_RTS(Stub_00077D86)
STUB_RTS(Stub_0007962A)
STUB_RTS(Stub_00079950)
STUB_RTS(Stub_00088B3C)
STUB_RTS(Stub_0008CC64)
STUB_RTS(Stub_0008D182)
STUB_RTS(Stub_0008E4E4)
STUB_RTS(Stub_0008EA4E)
STUB_RTS(Stub_0008EAEC)
STUB_RTS(Stub_0008EB54)
