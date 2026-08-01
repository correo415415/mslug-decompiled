| ============================================================================
|  Metal Slug 1 - asm/entity_alloc_by_player_slot_09b9f6.s
|  ----------------------------------------------------------------------------
|  Wave Y - #7
|
|  Entity_AllocByPlayerSlot_09B9F6  @ $09B9F6  (62 bytes, 1 caller)
|
|  Comprueba si el entity en a1 corresponde a uno de los dos slots de
|  jugador conocidos ($100440 = P1, $1004E0 = P2). Si es asi, reserva un
|  nuevo entity via Task_AllocFromFreeList_0004AE (T#4) + Entity_CopyTransform_05DD02
|  (S#4), y lo inicializa con el indice del jugador (0 para P1, 1 para P2)
|  en $68(a0), $98(a0)=3 y limpia $9c(a0). Si a1 no es ninguno de los dos
|  slots de jugador, rts sin hacer nada.
|
|  Firma C conceptual:
|
|      /* Reserva un entity hijo asociado al jugador identificado por
|       * (a1==P1 -> idx=0; a1==P2 -> idx=1; otro -> no-op). */
|      void Entity_AllocByPlayerSlot(struct Entity *player_slot /*a1*/,
|                                    struct Entity *parent /*a6*/);
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. Comparaciones dobles con `cmpi.l #imm, d1; beq/bne` con addq.b #1,d0
|       INTERCALADO para computar el indice segun el resultado. GCC emitiria
|       switch con jump table o if/else if consecutivos, no un incremento
|       between compares.
|    2. `move.l d0, -(a7)` para preservar d0 sobre los dos jsr y luego
|       `move.l (a7)+, d0` no matchea ningun ABI GCC (d0 es caller-saved).
|       Uso explicito del stack como local en un helper hand-coded.
|    3. `move.b #$3, $98(a0)` como constante literal, sin nombre. Es un
|       initial-state opcode del entity - por identificar.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Entity_AllocByPlayerSlot_09B9F6
        .type   Entity_AllocByPlayerSlot_09B9F6, @function
        .section .text.Entity_AllocByPlayerSlot_09B9F6, "ax", @progbits

Entity_AllocByPlayerSlot_09B9F6:
        move.l  a1, d1                         | +00  d1 = a1  (para cmpi.l)
        clr.b   d0                             | +02  d0 = 0  (default player idx)
        cmpi.l  #0x100440, d1                  | +04  if (a1 == P1_SLOT)
        beq.w   .Lalloc                        | +0a     goto .Lalloc con d0=0
        addq.b  #0x1, d0                       | +0e  d0 = 1  (P2 idx tentativo)
        cmpi.l  #0x1004e0, d1                  | +10  if (a1 != P2_SLOT)
        bne.w   .Lout                          | +16     rts (no-op)
.Lalloc:
        move.l  d0, -(a7)                      | +1a  push player_idx (over the jsrs)
        lea     .LScriptTemplate(pc), a1       | +1c  a1 = &script_template  (pc-rel)
        jsr     0x4ae.l                        | +20  Task_AllocFromFreeList (T#4)
        jsr     0x5dd02.l                      | +26  Entity_CopyTransform (S#4)
        move.l  (a7)+, d0                      | +2c  pop player_idx
        move.b  d0, 0x68(a0)                   | +2e  a0->player_idx = d0
        move.b  #0x3, 0x98(a0)                 | +32  a0->field_98 = 3
        clr.b   0x9c(a0)                       | +38  a0->field_9c = 0
.Lout:
        rts                                    | +3c

        .equ    .LScriptTemplate, ScriptTemplate_09B51E

        .size   Entity_AllocByPlayerSlot_09B9F6, .-Entity_AllocByPlayerSlot_09B9F6
