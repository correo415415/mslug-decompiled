/*
 * Metal Slug 1 — Range guards y filtros de dispatch
 * ====================================================
 * Funciones muy cortas que actúan como "puertas" antes de invocar rutinas
 * más grandes. Comparan un valor de entrada (típicamente d0) contra un
 * umbral y, si supera el filtro, hacen fallthrough al código contiguo
 * (que suele ser la implementación real); si no, retornan.
 *
 * Es un patrón muy usado por el SDK Nazca/SN Systems para separar las
 * comprobaciones de rango del cuerpo real de una tabla de dispatch.
 */

#include "mslug.h"

/* ---------------------------------------------------------------------
 * ClampD0ToRange  ($00219C, 10 bytes)
 * ---------------------------------------------------------------------
 * Guardián de rango sobre d0.w. Si d0 < $2000, cae en la función siguiente
 * (fallthrough a $0021A6). Si d0 >= $2000, retorna inmediatamente.
 *
 * Bytes originales:
 *   $00219C: 0C40 2000       cmpi.w #$2000, d0
 *   $0021A0: 6500 0004       bcs.w  $0021A6      ; salto si d0 < $2000 (unsigned)
 *   $0021A4: 4E75            rts                 ; camino "fuera de rango"
 *
 * Semántica en pseudo-C:
 *     if (d0 >= 0x2000) return;
 *     // fallthrough al código de la función contigua ($0021A6+)
 *
 * En C puro no podemos expresar el "fallthrough al byte siguiente" sin
 * ayuda del linker: colocamos la función y confiamos en que la entrada
 * siguiente del registro ocupe justo $0021A6. El bloque asm inline emite
 * exactamente los 10 bytes esperados.
 * -------------------------------------------------------------------- */
void ClampD0ToRange(void)
{
    __asm__ volatile(
        "cmpi.w  #0x2000, %%d0    \n"   /* 0C40 2000 */
        "bcs.w   1f               \n"   /* 6500 0004  -> fallthrough */
        "rts                      \n"   /* 4E75 */
        "1:                       \n"
        ::: "cc"
    );
    __builtin_unreachable();
}

/* ---------------------------------------------------------------------
 * InputGuardCall219c  ($002352, 28 bytes)
 * ---------------------------------------------------------------------
 * Filtro de input usado como pre-condición antes de invocar ClampD0ToRange.
 * Semantica (en pseudo-C):
 *
 *     if (BIOS_flag_10FDAF == 1 && flag_10FD8D == 0) return;
 *     ClampD0ToRange();   // fallthrough al código tras la guarda
 *
 * Bytes originales:
 *   $002352: 0C39 0001 0010 FDAF   cmpi.b #1, ($10FDAF).L
 *   $00235A: 6600 000C             bne.w  $002368            ; !=1 → llamada
 *   $00235E: 4A39 0010 FD8D        tst.b  ($10FD8D).L
 *   $002364: 6700 0006             beq.w  $00236C            ; ==0 → rts
 *   $002368: 4EBA FE32             jsr    (pc+(-462)).w      ; → $00219C
 *   $00236C: 4E75                  rts
 *
 * $10FDAF y $10FD8D son variables BIOS relacionadas con el pad de
 * jugador 1 (bit-mask de teclas o edge-detect). La semántica exacta se
 * confirmará al decompilar los write-sites de esas variables.
 * -------------------------------------------------------------------- */
void InputGuardCall219c(void)
{
    __asm__ volatile(
        "cmpi.b  #1, 0x10FDAF        \n"   /* 0C39 0001 0010FDAF */
        "bne.w   1f                  \n"   /* 6600 000C          */
        "tst.b   0x10FD8D            \n"   /* 4A39 0010FD8D      */
        "beq.w   2f                  \n"   /* 6700 0006          */
        "1: jsr ClampD0ToRange(%%pc) \n"   /* 4EBA FE32          */
        "2:                          \n"
        ::: "cc", "memory"
    );
}
