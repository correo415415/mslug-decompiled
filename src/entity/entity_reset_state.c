/*
 * Metal Slug 1 — EntityResetState ($0267E2, 18 bytes)
 * =====================================================
 * Reinicia a cero un pequeño bloque de campos "misc" de la entidad
 * apuntada por fp (A6). Los offsets +$26, +$28, +$2C y +$2E son campos
 * que el motor usa como estado transitorio del task y que deben quedar
 * limpios cuando el task cambia de estado o se recicla.
 *
 * Bytes originales:
 *   $0267E2: 426E 002E     clr.w  46(fp)     ; fp->field_2E = 0
 *   $0267E6: 426E 002C     clr.w  44(fp)     ; fp->field_2C = 0
 *   $0267EA: 42AE 0028     clr.l  40(fp)     ; fp->field_28 = 0
 *   $0267EE: 426E 0026     clr.w  38(fp)     ; fp->field_26 = 0
 *   $0267F2: 4E75          rts
 *
 * NOTA de codegen:
 *   El orden es descendente de offsets (2E → 2C → 28 → 26). GCC -Os con
 *   dos `clr.w` en offsets adyacentes ($2E y $2C) intentaría fusionarlos
 *   en un `clr.l 44(fp)` — pero el compilador original los mantiene
 *   separados. Usamos TASK_BARRIER() entre stores para prohibir la fusión.
 *
 * En C:
 *     fp->field_2E = 0;
 *     fp->field_2C = 0;
 *     fp->field_28 = 0;   (long)
 *     fp->field_26 = 0;
 */

#include "mslug.h"

void EntityResetState(void)
{
    TASK_W(0x2E) = 0;  TASK_BARRIER();
    TASK_W(0x2C) = 0;  TASK_BARRIER();
    TASK_L(0x28) = 0;  TASK_BARRIER();
    TASK_W(0x26) = 0;
}
