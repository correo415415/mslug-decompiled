/*
 * Metal Slug 1 — GameFrame ($0009 7A, 46 bytes)
 * ================================================
 * Núcleo del handler de frame del juego. Se ejecuta una vez por VBlank,
 * activado desde el vector 25 ($08F6). Su secuencia canónica es:
 *
 *   1. FUN_000020e2()        pipeline de sprites (probable)
 *   2. ResetIrqCallback()    reinstala el callback de VBlank por defecto
 *   3. BIOS_FIX_CLEAR()      llamada al BIOS para limpiar el Fix Layer
 *   4. FUN_0000212e()        rutina de pipeline de fondo
 *   5. g_lspc_mode = 0       modo LSPC normal (0 = auto-animate on)
 *   6. si BIOS_PLAYER_MOD1 != 0:  FUN_00099afc()     (juego en modo activo)
 *
 * Bytes originales:
 *   $00097A: 4EB9 0000 20E2     jsr    $000020E2.L      (FUN_000020e2)
 *   $000980: 4EBA 0026          jsr    (pc+$26).w       (ResetIrqCallback)
 *   $000984: 4EB9 00C0 04C2     jsr    $00C004C2.L      (BIOS_FIX_CLEAR)
 *   $00098A: 4EB9 0000 212E     jsr    $0000212E.L      (FUN_0000212e)
 *   $000990: 4279 0010 6EAC     clr.w  ($106EAC).L      (g_lspc_mode = 0)
 *   $000996: 4A39 0010 FD82     tst.b  ($10FD82).L      (BIOS_PLAYER_MOD1)
 *   $00099C: 6700 0008          beq.w  $0009A6          (rama a rts final)
 *   $0009A0: 4EB9 0009 9AFC     jsr    $00099AFC.L      (FUN_00099afc)
 *   $0009A6: 4E75               rts                     (early exit del beq)
 *
 * NOTA de codegen (por qué C puro + micro-asm quirúrgico y no C limpio):
 *   GCC 13 -Os no reproduce cuatro opcodes concretos que el compilador
 *   original (probablemente SN Systems SDK) sí generó:
 *      - `jsr (pc+d,pc)` corto para ResetIrqCallback (rango PC-rel 16-bit).
 *        Sin -mpcrel global (que rompería BIOS_FIX_CLEAR), forzamos esta
 *        forma con un `jsr` asm inline de 4 B.
 *      - `clr.w abs.l`   (GCC lo evita por precaución HW inútil aquí).
 *      - `tst.b abs.l`   (GCC prefiere `move.b abs.l, dN`).
 *      - `beq.w + jsr abs.l + rts` explícito en lugar de `jmp` tail-call.
 *   Para esas cuatro emitimos asm inline puntual. La semántica queda clara
 *   en los comentarios y en el resto del C limpio.
 */

#include "mslug.h"

void GameFrame(void)
{
    /* --- Pipeline principal ------------------------------------------- */
    FUN_000020e2();

    /* ResetIrqCallback() — versión PC-relativa 16-bit (4 B, no 6 B).
     * `jsr` con símbolo cercano en asm inline se resuelve a `4EBA dddd`. */
    __asm__ volatile("jsr  ResetIrqCallback(%%pc)" ::: "memory", "cc",
                     "d0","d1","a0","a1");

    BIOS_FIX_CLEAR();
    FUN_0000212e();

    /* g_lspc_mode = 0;  (forzamos `clr.w abs.l` en lugar de `move.w #0,abs.l`) */
    __asm__ volatile("clr.w  0x106EAC" ::: "memory", "cc");

    /* if (BIOS_PLAYER_MOD1 != 0) FUN_00099afc();
     *
     * Se agrupa como bloque asm porque GCC no reproduce el patrón exacto
     * (tst.b + beq.w + jsr abs.l + rts). Semánticamente equivale a la
     * condición C anterior — el CCR tras el `tst.b` gobierna el `beq.w`. */
    __asm__ volatile(
        "tst.b   0x10FD82         \n"   /* 4A39 0010FD82  BIOS_PLAYER_MOD1 */
        "beq.w   1f               \n"   /* 6700 0008      -> $0009A6 (rts) */
        "jsr     0x00099AFC       \n"   /* 4EB9 00099AFC  FUN_00099afc     */
        "1:                       \n"
        ::: "cc", "memory"
    );
}
