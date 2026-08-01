/*
 * Metal Slug 1 — Alias de registros globales (opt-in por unidad)
 * =================================================================
 * Este header declara los `register global` que emulan la convención
 * Nazca/SN Systems: cada unidad de compilación que necesite un registro
 * dedicado (A0/A1/A2 para punteros, D0 para argumento primario) DEBE
 * incluir este header además de `mslug.h`.
 *
 * ¿Por qué no meterlos en `mslug.h`? Porque GCC 13 se vuelve muy
 * conservador cuando ve varios `register global` a la vez y en algunas
 * funciones cortas empieza a salvar/restaurar registros no usados en
 * pila (p. ej. `move.l a3,-(sp)` al inicio y `movea.l (sp)+,a3` al
 * final), lo que rompe el matching bit-a-bit. Aislando los aliases a
 * los .c que verdaderamente los usan, el resto de unidades compila
 * limpiamente sin prólogos parásitos.
 *
 * Cada .c que use registros dedicados debe listar SOLO los que necesita.
 * Aquí exponemos todos como macros que se ACTIVAN definiendo símbolos
 * antes del include:
 *
 *     #define USE_A0
 *     #define USE_A1
 *     #define USE_A2
 *     #define USE_D0
 *     #include "mslug_regs.h"
 */

#ifdef USE_A0
register void (*_a0_ptr)(void) __asm__("a0");
#endif

#ifdef USE_A1
register void (*_a1_ptr)(void) __asm__("a1");
#endif

#ifdef USE_A2
register void (*_a2_tbl)(void) __asm__("a2");
#endif

#ifdef USE_D0
register unsigned short _d0_w __asm__("d0");
#endif
