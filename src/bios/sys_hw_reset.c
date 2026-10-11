/*
 * Metal Slug 1 — Sys_HW_Reset ($0868, 60 bytes)
 * ================================================
 * Rutina de reinicio del hardware LSPC / paleta / video enable, seguida
 * de un blanqueado en cero de casi toda la RAM de trabajo del juego.
 *
 * Se llama:
 *   - desde BiosEntry_DEMO ($0852) — antes de arrancar una demo attract
 *   - desde SoftReset       ($085E) — como parte del reset de arranque
 *
 * Bytes originales:
 *   $0868: 08B9 0007 0010FD80    bclr    #7, ($10FD80).L   ; BIOS var - clear bit
 *   $0870: 13FC 0000 003A001F    move.b  #0, ($3A001F).L   ; REG_NOSHADOW  (LSPC: sal shadow)
 *   $0878: 4279 0040 1FFE        clr.w   ($401FFE).L       ; PALETTE ram end (bank sync)
 *   $087E: 13FC 0000 003A000F    move.b  #0, ($3A000F).L   ; REG_ENVIDEO=0 (video OFF)
 *   $0886: 4279 0040 1FFE        clr.w   ($401FFE).L       ; PALETTE ram end (bank sync)
 *   $088C: 7200                  moveq   #0, d1            ; valor de fill
 *   $088E: 303C E40C             move.w  #$E40C, d0        ; contador dbf (0xE40C+1 = 58381 bytes)
 *   $0892: 41F9 0010 0080        lea     ($100080).L, a0   ; base = RAM+$80
 *   $0898: 6000 0004             bra.w   $089E             ; salta al dbf inicial
 *   $089C: 10C1                  move.b  d1, (a0)+         ; body: store byte y avanza
 *   $089E: 51C8 FFFC             dbf     d0, $089C         ; loop hasta d0 = -1
 *   $08A2: 4E75                  rts
 *
 * Notas del hardware Neo Geo (referencia neogeodev wiki):
 *  - $3A001F : REG_NOSHADOW — poner a 0 desactiva el modo shadow (LSPC).
 *  - $3A000F : REG_ENVIDEO  — poner a 0 apaga la salida de video.
 *  - $401FFE : dirección de la última word de la paleta activa. Escribir
 *              ahí sincroniza los bancos de paleta (patrón defensivo de
 *              muchos juegos Neo Geo tras tocar registros LSPC).
 *  - $10FD80 : BIOS_USER_REQUEST (bit 7 = "user mode").
 *  - $100080..$10E48D : bloque de 58382 B de RAM que se limpia a 0.
 *
 * Peculiaridades de codegen (por qué asm inline puntual):
 *  - GCC 13 no genera `bclr #imm, abs.l` desde un `&= ~mask` — usa
 *    `move.b/or/and`; forzamos con asm inline (1 opcode, 8B exactos).
 *  - GCC no emite `clr.w abs.l` (prefiere `move.w #0, abs.l`); forzamos.
 *  - El bucle tiene una forma muy concreta: `bra.w` al dbf, luego el body,
 *    luego el `dbf` que salta hacia atrás. Es la forma clásica del bucle
 *    "downcount con predecrement" del compilador original SN Systems, y
 *    GCC prefiere `subq/beq` o dbf directo sin la rama inicial. Se emite
 *    en asm inline como un único bloque monolítico.
 */

#include "mslug.h"

__attribute__((section(".text.Sys_HW_Reset")))
void Sys_HW_Reset(void)
{
    /* --- Reset LSPC / video / paleta -------------------------------- */
    __asm__ volatile(
        "bclr   #7, 0x10FD80        \n"   /* 08B9 0007 0010FD80  (8 B) */
        "move.b #0, 0x3A001F        \n"   /* 13FC 0000 003A001F  (8 B) — REG_NOSHADOW */
        "clr.w  0x401FFE            \n"   /* 4279 0040 1FFE      (6 B) */
        "move.b #0, 0x3A000F        \n"   /* 13FC 0000 003A000F  (8 B) — REG_ENVIDEO */
        "clr.w  0x401FFE            \n"   /* 4279 0040 1FFE      (6 B) */
        ::: "cc", "memory");

    /* --- Blanqueado a cero de $100080..$10E48D (58382 bytes) --------
     *
     * Semántica equivalente en C:
     *     u8  *p = (u8*)0x00100080;
     *     u16  n = 0xE40C;               // dbf termina cuando d0 == -1
     *     do { *p++ = 0; } while (--n != 0xFFFF);
     *
     * Se emite como bloque monolítico para reproducir exactamente la
     * forma "moveq d1=0 ; movew #imm,d0 ; lea abs,a0 ; bra ; body ; dbf".
     */
    __asm__ volatile(
        "moveq   #0, %%d1           \n"   /* 7200                (2 B) */
        "move.w  #0xE40C, %%d0      \n"   /* 303C E40C           (4 B) */
        "lea     0x100080, %%a0     \n"   /* 41F9 00100080       (6 B) */
        "bra.w   1f                 \n"   /* 6000 0004           (4 B) */
        "0: move.b %%d1, (%%a0)+    \n"   /* 10C1                (2 B) */
        "1: dbf    %%d0, 0b         \n"   /* 51C8 FFFC           (4 B) */
        ::: "cc", "memory", "d0", "d1", "a0");
}
