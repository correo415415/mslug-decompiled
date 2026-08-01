| ============================================================================
|  Metal Slug 1 - asm/input_ctx_switch_05cfc8.s
|  ----------------------------------------------------------------------------
|  Wave U (InputMask event dispatchers) - backend #2 (brazo alternativo)
|
|  InputMask_ReadCtxSwitchPlayer_05cfc8  @ $05cfc8  (48 bytes, 0 callers
|                                                    directos matcheados;
|                                                    solo alcanzable como
|                                                    brazo alternativo de
|                                                    InputMask_CheckChannelAvail_05cfa8)
|
|  Segundo backend del cluster InputMask, cabeza del brazo "canal no
|  libre o P2": elige el buffer de input segun $6d(a6) (1=P1 -> $100300,
|  2=P2 -> $1003a0), carga el vector activo en a2 y llama al probe real
|  del canal en $5d674(pc).
|
|  NO tiene rts propio: cae por fall-through logico al backend
|  InputMask_TestChannelBit_05cff8 en $05CFF8 (test del bit en a2[d0]).
|
|  Firma C conceptual:
|      void InputMask_ReadCtxSwitchPlayer_05cfc8(void);
|          // Entrada:
|          //   d0 = layer/slot (2 o 3)
|          //   d1 = mask bit del evento
|          //   a6 = actor con $6d = id de jugador (1 o 2)
|          // Efectos:
|          //   a2 = &$5cc08 (default) o $100300[+$72] o $1003a0[+$72]
|          //   luego jsr $5d674 (probe/procesador real de eventos)
|
|  Flujo:
|      1. lea $5cc08(pc), a2   ->  a2 default = tabla de contexto local
|         (a $5cc08 = codigo previo del mismo modulo, no analizado aun).
|      2. cmpi.b #1, $6d(a6)   ->  ¿es P1?
|         Si no, bne al paso 4.
|      3. lea $100300.l, a1 ; movea.l $72(a1), a2   ->  a2 = P1 input buf.
|      4. cmpi.b #2, $6d(a6)   ->  ¿es P2?
|         Si no, bne al fall-through ($5cff8).
|      5. lea $1003a0.l, a1 ; movea.l $72(a1), a2   ->  a2 = P2 input buf.
|      6. jsr $5d674(pc)       ->  probe real del canal.
|      7. Fall-through natural a $05cff8.
|
|  Hallazgos forenses:
|    1. Fall-through DE VUELTA al backend contiguo $05CFF8 sin jmp ni rts:
|       la funcion no cierra su propia salida, la delega al backend
|       siguiente. Mismo idioma que Task_AllocFromFreeList (T#4) cediendo
|       su rts al thunk contiguo $0004fe.
|    2. Estructura "if id=1 elif id=2" con dos lecturas identicas del
|       campo $72 en dos structs distintas: GCC habria usado indexado.
|    3. jsr $5d674(pc) apunta a codigo aun no matcheado -> se registra
|       como simbolo externo Sub_00005D674.
|  ============================================================================

        .text
        .globl  InputMask_ReadCtxSwitchPlayer_05cfc8
        .type   InputMask_ReadCtxSwitchPlayer_05cfc8, @function
        .section .text.InputMask_ReadCtxSwitchPlayer_05cfc8, "ax", @progbits

InputMask_ReadCtxSwitchPlayer_05cfc8:
        lea     Sub_00005CC08(pc), a2       | +00  45 fa fc 3e   a2 = &$5cc08
        cmpi.b  #1, 0x6d(a6)                | +04  0c 2e 00 01 00 6d
        bne.w   .Lcheck_p2                  | +0a  66 00 00 0c
        lea     0x100300.l, a1              | +0e  43 f9 00 10 03 00
        movea.l 0x72(a1), a2                | +14  24 69 00 72
.Lcheck_p2:
        cmpi.b  #2, 0x6d(a6)                | +18  0c 2e 00 02 00 6d
        bne.w   .Lcall_probe                | +1e  66 00 00 0c
        lea     0x1003a0.l, a1              | +22  43 f9 00 10 03 a0
        movea.l 0x72(a1), a2                | +28  24 69 00 72
.Lcall_probe:
        jsr     Sub_00005D674(pc)           | +2c  4e ba 06 7e
                                            |      -> $5d674 (probe real,
                                            |         aun sin decompilar)
                                            |      fall-through a $05cff8
        .size   InputMask_ReadCtxSwitchPlayer_05cfc8, .-InputMask_ReadCtxSwitchPlayer_05cfc8

| Alias PC-relativo: $5cc08 es codigo del mismo modulo anterior. Distancia:
| $5cc08 - ($5cfca + 2) = $5cc08 - $5cfcc = -$3c4  ->  fc 3e OK.
| El simbolo Sub_00005CC08 se resuelve via --defsym en tools/symbols.py.
