| ============================================================================
|  Metal Slug 1 - asm/init_jsr_then_tail_call_001320.s
|  ----------------------------------------------------------------------------
|  Wave Z batch 2 - #12
|
|  Init_JsrThenTailCall_001320  @ $001320  (18 bytes)
|
|  Init pequeno de zona baja. Ejecuta tres pasos:
|    1. jsr $46AC6.l               (subrutina de init pesada)
|    2. move.b #$FF, $106ED2.l     (publica sentinela en global)
|    3. bra.w $FE0                 (tail-call largo, sin rts propio)
|
|  Firma C conceptual:
|
|      /* Encadena la subrutina de init pesada $46AC6, publica $FF en el
|       * flag global $106ED2, y hace tail-call a $FE0 (sin retorno propio). */
|      void Init_JsrThenTailCall(void);
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Init_JsrThenTailCall_001320
        .type   Init_JsrThenTailCall_001320, @function
        .section .text.Init_JsrThenTailCall_001320, "ax", @progbits

Init_JsrThenTailCall_001320:
        jsr     0x46ac6.l                      | +00  Sub_00046AC6 (init pesado)
        move.b  #0xff, 0x106ed2.l              | +06  publica sentinela $FF
        bra.w   .Ltail                         | +0e  tail-call a $FE0 (no rts)

        .equ    .Ltail, Sub_00000FE0

        .size   Init_JsrThenTailCall_001320, .-Init_JsrThenTailCall_001320
