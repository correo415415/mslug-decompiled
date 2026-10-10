/*
 * Metal Slug 1 — Handlers de interrupción del 68000
 * ====================================================
 * Rutinas apuntadas por la vector table del cartucho:
 *
 *   vec[25]=$8F6 IRQ2 principal (VBlank periférico)
 *   vec[26]=$8D6 VBlank del LSPC (interrupción de fotograma)
 *   vec[27]=$8F4 stub rte
 *   vec[28..31]=$8F0 stub rte único compartido
 *
 * La organización es: el LSPC dispara VBlank ($8D6) para servicio rápido
 * (llama al callback del juego); IRQ2 ($8F6) sirve como "coordinador"
 * asíncrono que hace las tareas pesadas cuando el BIOS lo requiere.
 *
 * NOTA IMPORTANTE DE CODEGEN:
 * ----------------------------
 * GCC 13 para m68k no soporta el atributo `naked`. Además, con funciones
 * que terminan en `rte`, GCC añade un `rts` (4E75) implícito al final
 * pese a `noreturn` + `__builtin_unreachable()`, rompiendo el tamaño
 * exacto de la sección.
 *
 * Solución adoptada: emitimos cada handler como asm top-level con su
 * propia `.section .text.<Name>`. Es la misma técnica que usan otras
 * decomps bit-exact (pmret, sonic-retro). La documentación semántica
 * queda en los comentarios adjuntos al bloque asm.
 */

#include "mslug.h"

/* ---------------------------------------------------------------------
 * VBlankHandler_08D6 ($08D6, 26 bytes) — vector 26 (VBlank del LSPC)
 * ---------------------------------------------------------------------
 * Bytes originales:
 *   $08D6: 33FC 0002 003C 000C     move.w #2, ($3C000C).L      ; REG_LSPCMODE = 2
 *   $08DE: 48E7 0080               movem.l a0, -(sp)           ; push a0
 *   $08E2: 2079 0010 6EA8          movea.l ($106EA8).L, a0     ; a0 = g_vblank_callback
 *   $08E8: 4E90                    jsr (a0)                    ; llama al callback
 *   $08EA: 4CDF 0100               movem.l (sp)+, a0           ; pop a0
 *   $08EE: 4E73                    rte
 *
 * Semántica:
 *   REG_LSPCMODE = 2;
 *   push a0; a0 = g_vblank_callback; (*a0)(); pop a0;
 *   rte;
 * -------------------------------------------------------------------- */
__asm__(
    ".section .text.VBlankHandler_08D6, \"ax\"                          \n"
    ".globl   VBlankHandler_08D6                                        \n"
    "VBlankHandler_08D6:                                                \n"
    "    move.w  #2, 0x3C000C           /* 33FC 0002 003C000C */        \n"
    "    movem.l %a0, -(%sp)            /* 48E7 0080          */        \n"
    "    movea.l 0x106EA8, %a0          /* 2079 00106EA8      */        \n"
    "    jsr     (%a0)                  /* 4E90               */        \n"
    "    movem.l (%sp)+, %a0            /* 4CDF 0100          */        \n"
    "    rte                            /* 4E73               */        \n"
);

/* ---------------------------------------------------------------------
 * IrqStubRte_08F0 ($08F0, 2B), IrqStubRte_08F4 ($08F4, 2B)
 * ---------------------------------------------------------------------
 * Handlers de vector 27..31: un simple `rte` para IRQs no usadas.
 * Nota: $08F2 es VBlankCallbackDefault (`rts`) que ya está registrado
 * como stub; los stubs $08F0 y $08F4 son "rte" y NO deben confundirse.
 * -------------------------------------------------------------------- */
__asm__(
    ".section .text.IrqStubRte_08F0, \"ax\"                             \n"
    ".globl   IrqStubRte_08F0                                           \n"
    "IrqStubRte_08F0: rte                                               \n"
);

__asm__(
    ".section .text.IrqStubRte_08F4, \"ax\"                             \n"
    ".globl   IrqStubRte_08F4                                           \n"
    "IrqStubRte_08F4: rte                                               \n"
);

/* ---------------------------------------------------------------------
 * IRQ2_08F6 ($08F6, 106 bytes) — vector 25 (IRQ2 del 68000)
 * ---------------------------------------------------------------------
 * Bytes originales:
 *   $08F6: 4A39 0010 FD80          tst.b  ($10FD80).L         ; BIOS_USER_REQUEST
 *   $08FC: 6B00 0008                bmi.w  $0906              ; bit 7 set -> local
 *   $0900: 4EF9 00C0 0438           jmp    ($C00438).L        ; BIOS_INT2_HANDLER
 *   $0906: 48E7 FFFE                movem.l d0-fp, -(sp)      ; salva d0..a6
 *   $090A: 4EB9 0000 1E5E           jsr    ($001E5E).L        ; job pesado principal
 *   $0910: 4EBA 004E                jsr    (pc+0x4E,pc)       ; -> LspcModeCheck_0960
 *   $0914: 3039 003C 0004           move.w ($3C0004).L, d0    ; snapshot REG_LSPC_LO
 *   $091A: 48A7 8000                movem.w d0, -(sp)         ; push snapshot
 *   $091E: 3039 0010 6EAC           move.w ($106EAC).L, d0    ; g_lspc_mode
 *   $0924: 0240 FFEF                andi.w #$FFEF, d0         ; clear bit 4
 *   $0928: 33C0 003C 0006           move.w d0, ($3C0006).L    ; REG_LSPC_HI (con bit 4 off)
 *   $092E: 4EB9 00C0 04CE           jsr    ($C004CE).L        ; BIOS_CDDA_CONTROL
 *   $0934: 4C9F 0001                movem.w (sp)+, d0         ; pop snapshot
 *   $0938: 48B9 0001 003C 0004      movem.w d0, ($3C0004).L   ; restaura REG_LSPC_LO
 *   $0940: 4EB9 00C0 044A           jsr    ($C0044A).L        ; BIOS_MESS_OUT
 *   $0946: 33F9 0010 6EAC 003C 0006 move.w ($106EAC).L, ($3C0006).L
 *   $0950: 33F9 0010 6EE4 003C 0000 move.w ($106EE4).L, ($3C0000).L
 *   $095A: 4CDF 7FFF                movem.l (sp)+, d0-fp
 *   $095E: 4E73                     rte
 *
 * Semántica (alto nivel):
 *   if (bit7($10FD80) == 0) jmp BIOS_INT2_HANDLER;   // deja que el BIOS gestione
 *   // salva registros, ejecuta job pesado
 *   FUN_00001E5E();
 *   LspcModeCheck_0960();
 *   snapshot = REG_LSPC_LO;
 *   REG_LSPC_HI = g_lspc_mode & ~0x10;
 *   BIOS_CDDA_CONTROL();
 *   REG_LSPC_LO = snapshot;
 *   BIOS_MESS_OUT();
 *   REG_LSPC_HI = g_lspc_mode;
 *   REG_LSPC     = *(u16*)$106EE4;
 *   rte
 * -------------------------------------------------------------------- */
__asm__(
    ".section .text.IRQ2_08F6, \"ax\"                                          \n"
    ".globl   IRQ2_08F6                                                        \n"
    "IRQ2_08F6:                                                                \n"
    "    tst.b   0x10FD80                    /* 4A39 0010FD80          */     \n"
    "    bmi.w   1f                          /* 6B00 0008              */     \n"
    "    jmp     0xC00438                    /* 4EF9 00C00438          */     \n"
    "1:  movem.l %d0-%fp, -(%sp)             /* 48E7 FFFE              */     \n"
    "    jsr     FUN_00001E5E                /* 4EB9 00001E5E          */     \n"
    "    jsr     LspcModeCheck_0960(%pc)     /* 4EBA 004E              */     \n"
    "    move.w  0x3C0004, %d0               /* 3039 003C0004          */     \n"
    "    movem.w %d0, -(%sp)                 /* 48A7 8000              */     \n"
    "    move.w  0x106EAC, %d0               /* 3039 00106EAC          */     \n"
    "    andi.w  #0xFFEF, %d0                /* 0240 FFEF              */     \n"
    "    move.w  %d0, 0x3C0006               /* 33C0 003C0006          */     \n"
    "    jsr     0xC004CE                    /* 4EB9 00C004CE          */     \n"
    "    movem.w (%sp)+, %d0                 /* 4C9F 0001              */     \n"
    "    movem.w %d0, 0x3C0004               /* 48B9 0001 003C0004     */     \n"
    "    jsr     0xC0044A                    /* 4EB9 00C0044A          */     \n"
    "    move.w  0x106EAC, 0x3C0006          /* 33F9 00106EAC 003C0006 */     \n"
    "    move.w  0x106EE4, 0x3C0000          /* 33F9 00106EE4 003C0000 */     \n"
    "    movem.l (%sp)+, %d0-%fp             /* 4CDF 7FFF              */     \n"
    "    rte                                 /* 4E73                   */     \n"
);

/* ---------------------------------------------------------------------
 * LspcModeCheck_0960 ($0960, 26 bytes)
 * ---------------------------------------------------------------------
 * Helper interno de IRQ2. Traduce el bit 3 del registro alto del LSPC
 * ($3C0006) en un flag byte guardado en $106EAE (probable
 * "field_shift_flag" para el modo interlace).
 *
 * Bytes originales:
 *   $0960: 7000                    moveq #0, d0
 *   $0962: 3239 003C 0006          move.w ($3C0006).L, d1
 *   $0968: 0801 0003               btst   #3, d1
 *   $096C: 6700 0004               beq.w  $0972
 *   $0970: 7001                    moveq  #1, d0
 *   $0972: 13C0 0010 6EAE          move.b d0, ($106EAE).L
 *   $0978: 4E75                    rts
 *
 * En C: g_field_shift_flag = (REG_LSPC_HI & 0x08) ? 1 : 0;
 * -------------------------------------------------------------------- */
__asm__(
    ".section .text.LspcModeCheck_0960, \"ax\"                                 \n"
    ".globl   LspcModeCheck_0960                                               \n"
    "LspcModeCheck_0960:                                                       \n"
    "    moveq   #0, %d0                     /* 7000                   */     \n"
    "    move.w  0x3C0006, %d1               /* 3239 003C0006          */     \n"
    "    btst    #3, %d1                     /* 0801 0003              */     \n"
    "    beq.w   1f                          /* 6700 0004              */     \n"
    "    moveq   #1, %d0                     /* 7001                   */     \n"
    "1:  move.b  %d0, 0x106EAE               /* 13C0 00106EAE          */     \n"
    "    rts                                                                   \n"
);
