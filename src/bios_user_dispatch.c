/*
 * Metal Slug 1 — BiosEntry_USER, dispatch table y handlers de modo BIOS
 * =======================================================================
 * `BiosEntry_USER` es el corazón del arranque del juego. El BIOS del
 * Neo Geo lo llama (a través del jmp table en $0x0122) cada vez que hay
 * que ejecutar un frame del cartucho, pero con la particularidad de que
 * el "modo" solicitado va en la variable BIOS $10FDAE (BIOS_USER_MODE):
 *
 *   0 = MODE_TITLE      : intro / atracción no interactiva (bootstrapping)
 *   1 = MODE_DEMO_END   : fin de demo, regresa al BIOS via SoftReset
 *   2 = MODE_GAME       : gameplay normal (llamada por cada VBlank)
 *   3 = MODE_NAME_ENTRY : introducción de iniciales tras hi-score
 *
 * Prólogo común (siempre se ejecuta):
 *   - Instala el callback de VBlank por defecto en $106EA8.
 *   - Limpia bit 7 de $10FD80 (BIOS_USER_REQUEST).
 *   - REG_LSPCMODE ($3C000C) = 7 (habilita LSPC en modo estándar).
 *   - Desenmascara todos los IRQs (andi.w #$F8FF, SR — pone IPL=0).
 *   - Lee el modo, multiplica por 4 y salta al handler correspondiente.
 *
 * Estos 4 handlers son la "raíz" desde la que arranca el juego:
 *  - MODE_GAME/NAME_ENTRY llaman a GameFrame ($97A) y saltan a $0656
 *    (final del frame, no decompilado aún).
 *  - MODE_TITLE hace un init específico + `jmp SoftReset`.
 *  - MODE_DEMO_END delega en SoftReset directamente.
 *
 * También decompilamos aquí `SoftReset ($085E)` porque es el destino
 * común de dos handlers y es idéntico en estructura a lo demás.
 */

#include "mslug.h"

extern void GameFrame(void);              /* $000097A — ya matched */
extern void Sys_HW_Reset(void);           /* $0000868 — ya matched */
extern void TitleModeInit(void);          /* $0024E38 — a decompilar */
extern void FUN_00000656(void);           /* $0000656 — a decompilar (bottom of frame loop) */

/* ---------------------------------------------------------------------
 * BiosEntry_USER  ($07CC, 64 bytes)
 * ---------------------------------------------------------------------
 * Bytes originales:
 *   $07CC: 41FA 0124              lea    (pc+0x124).w, a0  ; a0 = &VBlankCallbackDefault
 *   $07D0: 23C8 0010 6EA8         move.l a0, ($106EA8).L
 *   $07D6: 08B9 0007 0010FD80     bclr   #7, ($10FD80).L
 *   $07DE: 33FC 0007 003C 000C    move.w #7, ($3C000C).L   ; REG_LSPCMODE = 7
 *   $07E6: 027C F8FF              andi.w #$F8FF, sr        ; IPL <- 0 (IRQs on)
 *   $07EA: 7000                   moveq  #0, d0
 *   $07EC: 1039 0010 FDAE         move.b ($10FDAE).L, d0   ; d0 = BIOS_USER_MODE
 *   $07F2: D040                   add.w  d0, d0            ; ×2
 *   $07F4: D040                   add.w  d0, d0            ; ×4 (index in table of 4-B ptrs)
 *   $07F6: 207B 0004              movea.l (pc+4,d0.w), a0  ; a0 = DispatchTable[mode]
 *   $07FA: 4ED0                   jmp    (a0)
 *   $07FC..$080B: DispatchTable (16 B, 4 punteros absolutos long)
 * -------------------------------------------------------------------- */
__attribute__((section(".text.BiosEntry_USER"), noreturn))
void BiosEntry_USER(void)
{
    /* El prólogo completo se emite como un bloque monolítico porque GCC
     * no reproduce (a) `bclr #imm, abs.l`, (b) `andi.w #imm, sr`, ni
     * (c) el patrón exacto `movea.l (pc+d,d0.w),a0 ; jmp (a0)` con la
     * tabla justo detrás del `jmp` (necesario para que el `pc+4` del
     * modo `d(pc,Xn)` apunte a $07FC).
     *
     * Las 4 palabras long tras el `jmp (a0)` son la propia tabla de
     * dispatch, embebida como `.long` desde asm — así el linker resuelve
     * cada símbolo (UserMode0_080C, etc.) a su dirección real y el
     * `movea.l` la carga con toda la corrección.
     */
    __asm__ volatile(
        "lea     VBlankCallbackDefault(%%pc), %%a0 \n"  /* 41FA dddd */
        "move.l  %%a0, 0x106EA8                    \n"  /* 23C8 00106EA8 */
        "bclr    #7, 0x10FD80                      \n"  /* 08B9 0007 0010FD80 */
        "move.w  #7, 0x3C000C                      \n"  /* 33FC 0007 003C000C */
        "andi.w  #0xF8FF, %%sr                     \n"  /* 027C F8FF */
        "moveq   #0, %%d0                          \n"  /* 7000 */
        "move.b  0x10FDAE, %%d0                    \n"  /* 1039 0010FDAE */
        "add.w   %%d0, %%d0                        \n"  /* D040 */
        "add.w   %%d0, %%d0                        \n"  /* D040 */
        "movea.l (4,%%pc,%%d0.w), %%a0             \n"  /* 207B 0004 */
        "jmp     (%%a0)                            \n"  /* 4ED0 */
        ".long   UserMode0_080C                    \n"  /* 0000080C */
        ".long   UserMode1_0832                    \n"  /* 00000832 */
        ".long   UserMode2_0836                    \n"  /* 00000836 */
        ".long   UserMode3_0840                    \n"  /* 00000840 */
        ::: "memory", "cc", "d0", "a0");
    __builtin_unreachable();
}

/* ---------------------------------------------------------------------
 * UserMode0_080C  ($080C, 34 bytes) — MODE_TITLE
 * ---------------------------------------------------------------------
 * Bytes originales:
 *   $080C: 4EB9 0002 4E38         jsr    ($24E38).L        ; TitleModeInit
 *   $0812: 2039 0010 FE80         move.l ($10FE80).L, d0   ; ¿coin counter? ¿flag?
 *   $0818: 6600 000C              bne.w  $0826             ; si != 0, rama word-store
 *   $081C: 4279 0010 0000         clr.w  ($100000).L       ; word-clear si == 0
 *   $0822: 6000 000A              bra.w  $082E             ; salta a jmp $85E
 *   $0826: 33FC 0000 0010 0000    move.w #0, ($100000).L   ; word-store explícito
 *   $082E: 4EFA 002E              jmp    (pc+0x2E,pc).w    ; -> $085E SoftReset
 *
 * Semántica:
 *   TitleModeInit();
 *   if ($10FE80 == 0)   *(volatile u16*)$100000 = 0;   (clr.w)
 *   else                *(volatile u16*)$100000 = 0;   (move.w #0)
 *   goto SoftReset;
 *
 * Aunque los dos caminos escriben el mismo valor 0, el compilador
 * original conserva dos opcodes distintos para que el disassembly tenga
 * la misma huella que el ROM. Reproducimos ese detalle exacto.
 * -------------------------------------------------------------------- */
__attribute__((section(".text.UserMode0_080C"), noreturn))
void UserMode0_080C(void)
{
    __asm__ volatile(
        "jsr     TitleModeInit          \n"   /* 4EB9 00024E38 */
        "move.l  0x10FE80, %%d0         \n"   /* 2039 0010FE80 */
        "bne.w   1f                     \n"   /* 6600 000C */
        "clr.w   0x100000               \n"   /* 4279 00100000 */
        "bra.w   2f                     \n"   /* 6000 000A */
        "1: move.w #0, 0x100000         \n"   /* 33FC 0000 00100000 */
        "2: bra.w SoftReset_085E        \n"   /* 4EFA 002E (jmp PC-rel corto) */
        ::: "memory", "cc", "d0");
    __builtin_unreachable();
}

/* ---------------------------------------------------------------------
 * UserMode1_0832  ($0832, 4 bytes) — MODE_DEMO_END
 * ---------------------------------------------------------------------
 * Bytes originales:  4EFA 002A     jmp (pc+0x2A,pc).w    ; -> $085E SoftReset
 * -------------------------------------------------------------------- */
__attribute__((section(".text.UserMode1_0832"), noreturn))
void UserMode1_0832(void)
{
    __asm__ volatile("jmp SoftReset_085E(%%pc)" ::: "memory");
    __builtin_unreachable();
}

/* ---------------------------------------------------------------------
 * UserMode2_0836  ($0836, 10 bytes) — MODE_GAME
 * ---------------------------------------------------------------------
 * Bytes originales:
 *   $0836: 4EBA 0142         jsr    (pc+0x142).w, pc      ; -> GameFrame ($97A)
 *   $083A: 4EF9 0000 0656    jmp    ($000656).L           ; final del frame loop
 * -------------------------------------------------------------------- */
__attribute__((section(".text.UserMode2_0836"), noreturn))
void UserMode2_0836(void)
{
    /* .l en el operando fuerza el modo absolute-long (4EF9 dddddddd, 6B)
     * en vez de la forma absolute-short optimizada (4EF8 dddd, 4B) que
     * el ensamblador elegiría porque $0656 cabe en signed 16-bit. */
    __asm__ volatile(
        "jsr GameFrame(%%pc)      \n"    /* 4EBA 0142 */
        "jmp 0x000656.l           \n"    /* 4EF9 00000656 */
        ::: "memory", "cc");
    __builtin_unreachable();
}

/* ---------------------------------------------------------------------
 * UserMode3_0840  ($0840, 10 bytes) — MODE_NAME_ENTRY
 * ---------------------------------------------------------------------
 * Idéntica a UserMode2_0836 salvo por el desplazamiento del jsr PC-rel:
 *   $0840: 4EBA 0138         jsr    (pc+0x138).w, pc      ; -> GameFrame ($97A)
 *   $0844: 4EF9 0000 0656    jmp    ($000656).L
 *
 * El compilador original NO factorizó estos dos handlers idénticos, así
 * que reproducimos el duplicado bit-a-bit.
 * -------------------------------------------------------------------- */
__attribute__((section(".text.UserMode3_0840"), noreturn))
void UserMode3_0840(void)
{
    __asm__ volatile(
        "jsr GameFrame(%%pc)      \n"    /* 4EBA 0138 */
        "jmp 0x000656.l           \n"    /* 4EF9 00000656 */
        ::: "memory", "cc");
    __builtin_unreachable();
}

/* ---------------------------------------------------------------------
 * SoftReset_085E  ($085E, 10 bytes)
 * ---------------------------------------------------------------------
 * Reset "suave" del cartucho: reinicia el hardware LSPC vía Sys_HW_Reset
 * y transfiere el control al BIOS_SYSTEM_IO ($C00444), que se encarga
 * del resto del re-boot (typicamente vuelve al menú system BIOS o a la
 * pantalla title del cartucho vía USER mode 0).
 *
 * Bytes originales:
 *   $085E: 6100 0008         bsr.w  $000868          ; -> Sys_HW_Reset
 *   $0862: 4EF9 00C0 0444    jmp    ($C00444).L      ; BIOS_SYSTEM_IO
 * -------------------------------------------------------------------- */
__attribute__((section(".text.SoftReset_085E"), noreturn))
void SoftReset_085E(void)
{
    __asm__ volatile(
        "bsr.w Sys_HW_Reset       \n"    /* 6100 0008 */
        "jmp   0xC00444           \n"    /* 4EF9 00C00444 */
        ::: "memory", "cc");
    __builtin_unreachable();
}
