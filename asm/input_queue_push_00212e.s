| ============================================================================
|  Metal Slug 1 - asm/input_queue_push_00212e.s
|  ----------------------------------------------------------------------------
|  Wave Y - #9
|
|  InputQueue_InitAndPushOp4_00212E  @ $00212E  (110 bytes, 1 caller)
|
|  Reinicia por completo la cola circular de opcodes en $108184 (buffer de
|  32 bytes con head en $1081A6 y tail en $1081A4) e inmediatamente empuja
|  el opcode $04 como primer elemento. Idioma clasico de arranque: init de
|  estructura + priming con un evento fijo (probablemente "boot completo").
|
|  Layout de la cola:
|      $108184..$1081A3 : ring buffer de 32 opcodes de 1 byte
|      $1081A4          : tail (word, indice del proximo pop)
|      $1081A6          : head (word, indice del proximo push)
|      $1081A8, $1081A9, $1081AA : flags/estados auxiliares (limpiados a init)
|      $1081AC          : last-opcode-pushed (sentinela $FF = ninguno)
|
|  Firma C conceptual:
|
|      /* Reinicia la cola circular de opcodes y encola inmediatamente el
|       * opcode $04. Constante inmediata en el codigo -> funcion NO
|       * parametrica; es un helper especifico del arranque post-BIOS. */
|      void InputQueue_InitAndPushOp4(void);
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. El opcode a encolar es literal `#$04` HARDCODEADO en la funcion.
|       GCC nunca emitiria una funcion asi; usaria parametro. Reescribir
|       la funcion como generic push perderia la coincidencia byte-a-byte.
|    2. La rama "queue full" (colision head vs tail post-avance) NO tira
|       el opcode: en su lugar RESETEA la sentinela last-value a $FF, con
|       lo que el proximo push del MISMO opcode volveria a hacerse aunque
|       el filtro de deduplicacion consecutiva estuviera activo. Es un
|       protocolo forzado de recuperacion no expresable en C idiomatico.
|    3. Dos rts separados: uno "push exitoso" en $2190 y otro "queue full"
|       en $219A, cada uno tras su propia rama. GCC hubiese fusionado el
|       epilogo (jump al final comun). Ademas el offset del beq final
|       (67 00 00 0a) apunta LITERALMENTE a $2192, que es el `move.b #$FF`
|       de la sentinela - fall-through natural al rts vecino.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  InputQueue_InitAndPushOp4_00212E
        .type   InputQueue_InitAndPushOp4_00212E, @function
        .section .text.InputQueue_InitAndPushOp4_00212E, "ax", @progbits

InputQueue_InitAndPushOp4_00212E:
                                              | ---- Reset de la estructura de cola ----
        clr.l   0x1081a4.l                    | +00  tail (word) + head (word) = 0
        clr.b   0x1081a8.l                    | +06  flag_a = 0
        clr.b   0x1081a9.l                    | +0c  flag_b = 0
        clr.b   0x1081aa.l                    | +12  flag_c = 0
        move.b  #0xff, 0x1081ac.l             | +18  last_pushed = 0xFF (sentinel)
                                              |
                                              | ---- Push especializado del opcode 4 ----
        move.b  #0x4, d0                      | +20  d0 = opcode a encolar (LITERAL)
        cmpi.b  #0x20, d0                     | +24  if (opcode >= 0x20)
        bcc.w   .Lstore_only                  | +28     saltar la deduplicacion
        cmp.b   0x1081ac.l, d0                | +2c  if (opcode == last_pushed)
        beq.w   .Lexit_dup                    | +32     goto exit (dedup)
        move.b  d0, 0x1081ac.l                | +36  last_pushed = opcode
.Lstore_only:
        lea.l   0x108184.l, a0                | +3c  a0 = &queue_buffer[0]
        move.w  0x1081a6.l, d1                | +42  d1 = head
        move.b  d0, (a0, d1.w)                | +48  queue[head] = opcode
        addq.w  #0x1, d1                      | +4c  head_next = head + 1
        andi.w  #0x1f, d1                     | +4e  head_next &= 0x1F (mask 32)
        cmp.w   0x1081a4.l, d1                | +52  if (head_next == tail)
        beq.w   .Lqueue_full                  | +58     goto queue_full (no publish)
        move.w  d1, 0x1081a6.l                | +5c  head = head_next  (publish)
.Lexit_dup:
        rts                                   | +62
.Lqueue_full:
        move.b  #0xff, 0x1081ac.l             | +64  last_pushed = 0xFF (reset dedup)
        rts                                   | +6c

        .size   InputQueue_InitAndPushOp4_00212E, .-InputQueue_InitAndPushOp4_00212E
