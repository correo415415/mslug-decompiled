| ============================================================================
|  Metal Slug 1 - asm/handler_conditional_hit_08b558.s
|  ----------------------------------------------------------------------------
|  Wave Z batch 2 - #14
|
|  Handler_ConditionalHitCounter_08B558  @ $08B558  (54 bytes)
|
|  Handler condicional que ejecuta una secuencia solo si $58(a6)==1:
|    1. Incrementa $66(a6) en 10 (probable "puntuacion por hit" o similar).
|    2. Invoca subrutina $28758 (probable check de estado).
|    3. Si $28758 retorna con C=1 (bcs): limpia bit 0 del flag $13(a6)
|       (probable "hit ya procesado").
|    4. Lee GlobalFlag_106F28; si bit 0 esta encendido, tail-calls a
|       $06E224 (Entity_SpawnAndTag, Z-batch2 #7).
|
|  Absorbe JsrAbsThunk_08b586 (Wave I): los ultimos 8 B de la funcion
|  (`jsr $6E224.l; rts`) fueron catalogados como thunk independiente por
|  el escaner Wave I. Es el 21 falso positivo del proyecto.
|
|  Firma C conceptual:
|
|      /* Handler que, cuando $58(self)==1, suma 10 a $66(self), invoca
|       * $28758 y limpia bit 0 de $13 si el probe reporto exito; luego
|       * si el flag global $106F28 esta encendido, spawna via $06E224. */
|      void Handler_ConditionalHitCounter(struct Entity *self /*a6*/);
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Handler_ConditionalHitCounter_08B558
        .type   Handler_ConditionalHitCounter_08B558, @function
        .section .text.Handler_ConditionalHitCounter_08B558, "ax", @progbits

Handler_ConditionalHitCounter_08B558:
        cmpi.b  #0x1, 0x58(a6)                 | +00  if (self->field58 != 1)
        bne.w   .Lexit                         | +06     goto exit
        addi.w  #0xa, 0x66(a6)                 | +0a  self->field66 += 10 (score bonus)
        jsr     0x28758.l                      | +10  Sub_00028758 (state check)
        bcc.w   .Lcheck_global                 | +16  if (!C) skip flag clear
        bclr.b  #0x0, 0x13(a6)                 | +1a  self->flags13 &= ~1 (hit done)
.Lcheck_global:
        move.b  0x106f28.l, d0                 | +20  d0 = GlobalFlag_106F28
        andi.b  #0x1, d0                       | +26  d0 &= 0x01
        beq.w   .Lexit                         | +2a  if (bit 0 clear) exit
        jsr     0x6e224.l                      | +2e  Entity_SpawnAndTag (Z-batch2 #7)
.Lexit:
        rts                                    | +34

        .size   Handler_ConditionalHitCounter_08B558, .-Handler_ConditionalHitCounter_08B558
