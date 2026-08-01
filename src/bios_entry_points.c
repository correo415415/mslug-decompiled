/*
 * Metal Slug 1 — Puntos de entrada BIOS (tabla en $0x122)
 * =========================================================
 * El BIOS del Neo Geo llama al cartucho a través de 4 entry points fijos
 * situados en la tabla de saltos $0x122..$0x139. Cada uno es un `jmp abs.l`
 * (opcode 0x4EF9) a la función correspondiente del cartucho:
 *
 *   $0122: jmp $000007CC   ; USER   — dispatcher principal (arranque)
 *   $0128: jmp $0000084A   ; PLAYER — llamada al iniciar partida
 *   $012E: jmp $00000852   ; DEMO   — llamada al iniciar demo
 *   $0134: jmp $000022BE   ; COIN_SOUND — SFX al insertar moneda
 *
 * Estos entry points están documentados por el BIOS de Neo Geo:
 *   https://wiki.neogeodev.org/index.php?title=Jump_table
 *
 * Cada una de esas 4 funciones vive fuera de la tabla; son las que
 * decompilamos aquí. Todas son cortas y muy determinadas por el ABI
 * del BIOS, por lo que se prestan a matching bit-a-bit inmediato.
 */

#include "mslug.h"

/* ---------------------------------------------------------------------
 * BiosEntry_COIN_SOUND  ($22BE, 8 bytes)
 * ---------------------------------------------------------------------
 * Llamado por el BIOS cuando se inserta una moneda. Simplemente marca
 * la variable de RAM `$10007A` a 1 — el juego, en su siguiente frame,
 * detectará el flag y disparará el SFX correspondiente por su propia
 * cadena de sonido.
 *
 * Bytes originales:
 *   $22BE: 13FC 0001 0010 007A     move.b #1, ($10007A).L
 *   $22C6: 4E75                    rts
 *
 * En C:  *(volatile u8*)0x10007A = 1;
 * -------------------------------------------------------------------- */
__attribute__((section(".text.BiosEntry_COIN_SOUND")))
void BiosEntry_COIN_SOUND(void)
{
    /* GCC al ver un store byte con literal `1` a una dirección absoluta
     * emite `move.b #1, abs.l` exactamente (opcode $13FC). No requiere
     * asm inline. */
    *(volatile u8 *)0x0010007A = 1;
}

/* ---------------------------------------------------------------------
 * BiosEntry_PLAYER  ($084A, 8 bytes)
 * ---------------------------------------------------------------------
 * Llamado por el BIOS cuando el jugador arranca partida. Delega toda la
 * lógica en la rutina `FUN_00024E76` y retorna.
 *
 * Bytes originales:
 *   $084A: 4EB9 0002 4E76     jsr    $00024E76.L
 *   $0850: 4E75               rts
 *
 * En C:  Player_Start_Inner();
 * -------------------------------------------------------------------- */
extern void Player_Start_Inner(void);   /* $00024E76 — se decompilará después */

__attribute__((section(".text.BiosEntry_PLAYER")))
void BiosEntry_PLAYER(void)
{
    Player_Start_Inner();
    /* Barrera para forzar `rts` explícito en lugar de tail-call `jmp`. */
    __asm__ volatile("" ::: "memory");
}

/* ---------------------------------------------------------------------
 * BiosEntry_DEMO  ($0852, 10 bytes)
 * ---------------------------------------------------------------------
 * Llamado por el BIOS antes de reproducir una demo attract. Ejecuta la
 * rutina `FUN_00024FB6` (probable "init modo demo") y a continuación
 * llama a la rutina de reset HW compartida en `$0868` (Sys_HW_Reset),
 * mediante `bsr.w` corto porque está a distancia PC-rel de 14 bytes.
 *
 * Bytes originales:
 *   $0852: 4EB9 0002 4FB6     jsr    $00024FB6.L      (DemoInit inner)
 *   $0858: 6100 000E          bsr.w  $00000868        (Sys_HW_Reset)
 *   $085C: 4E75               rts
 * -------------------------------------------------------------------- */
extern void Demo_Start_Inner(void);   /* $00024FB6 — a decompilar después */
extern void Sys_HW_Reset(void);       /* $00000868 — a decompilar después */

__attribute__((section(".text.BiosEntry_DEMO")))
void BiosEntry_DEMO(void)
{
    Demo_Start_Inner();
    /* La segunda llamada debe emitirse como `bsr.w` (opcode $6100 dddd)
     * porque el target está a menos de 32K bytes. GCC prefiere `jsr abs.l`
     * (6B) — forzamos el opcode corto con asm inline puntual (4B). */
    __asm__ volatile("bsr.w Sys_HW_Reset" ::: "memory", "cc",
                     "d0","d1","a0","a1");
}
