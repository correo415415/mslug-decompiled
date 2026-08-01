/*
 * Metal Slug 1 — Stubs "return constante" (4 bytes cada uno)
 * ============================================================
 * Funciones que retornan un valor entero fijo (0, -1) o hacen un `clr.w d0`
 * antes del `rts`. Se usan como callbacks por defecto que ceden un valor
 * neutro al llamador (p. ej. "no hay más items", "estado ok = 0", etc.).
 *
 * Patrones exactos observados en el ROM:
 *   moveq #0,d0  ; rts     ->  70 00 4E 75    (return 0)
 *   moveq #-1,d0 ; rts     ->  70 FF 4E 75    (return -1 / 0xFFFFFFFF)
 *   clr.w d0    ; rts      ->  42 40 4E 75    (short-return 0, high word intacta)
 *
 * GCC 13 -Os -mpcrel emite exactamente estos 4 bytes desde el C obvio.
 */

#include "mslug.h"

/* ---- return 0 --------------------------------------------------------- */
/* Ret0_000437D6 ABSORBIDO por Geom_Proj_Clamp_0436DE (Wave FF batch 2 - FP #31)
 * $0437D6..$0437D9 (4 B: moveq #0, d0; rts) es la rama overflow del helper
 * geometrico (retorno d0=0 cuando la reduccion detecta que el rango de
 * proyeccion excede $20000), no una funcion trivial independiente. */

/* ---- return -1 -------------------------------------------------------- */
__attribute__((section(".text.RetMinus1_00077144")))
int RetMinus1_00077144(void) { return -1; }

__attribute__((section(".text.RetMinus1_0008F826")))
int RetMinus1_0008F826(void) { return -1; }

/* ---- clr.w d0 ; rts (retorna short 0, no toca la palabra alta de d0) - */
__attribute__((section(".text.ClrWD0_00043E88")))
short ClrWD0_00043E88(void) { return 0; }
