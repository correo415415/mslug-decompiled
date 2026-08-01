| ============================================================================
|  Metal Slug 1 - asm/input_test_all_bits_05d082.s
|  ----------------------------------------------------------------------------
|  Wave U (InputMask event dispatchers v2) - backend #3 (detector de combo)
|
|  InputMask_TestChannelAllBits_05d082  @ $05d082  (24 bytes, 0 callers
|                                                   directos matcheados;
|                                                   solo alcanzable por
|                                                   fall-through desde $5d052)
|
|  Variante "detector de combinacion completa" del backend #3 del cluster
|  #1 (InputMask_TestChannelBit_05cff8). La diferencia critica:
|
|      cluster #1  ($5cff8, "algun bit activo"):
|          move.b (a2,d0.w), d0
|          and.b  d1, d0
|          bne.w  .Lset_c            <-- Z=0 (al menos un bit) -> C=1
|          andi.b #$ee, ccr          <-- Z=1 (ninguno)         -> C=0
|
|      cluster #2  ($5d082, "TODOS los bits activos"):
|          move.b (a2,d0.w), d0
|          and.b  d1, d0
|          cmp.b  d1, d0
|          beq.w  .Lset_c            <-- d0==d1 (todos)        -> C=1
|          andi.b #$ee, ccr          <-- d0!=d1 (algunos falta)-> C=0
|
|  Es la firma del "detector de combo completo": el sub-cluster v2 lo
|  usa porque sus mascaras (mask=$30/$60/$a0) son OR de bits, y el codigo
|  quiere disparar solo cuando el usuario pulsa TODA la combinacion, no
|  solo alguno de los bits que la componen.
|
|  Firma C conceptual (dos brazos hermanos que retornan por CCR):
|      void InputMask_TestChannelAllBits_05d082(void);
|          // Entrada:
|          //   a2 = puntero al buffer de bytes de estado de eventos
|          //   d0 = slot (usado como word-index en a2[d0.w])
|          //   d1 = mascara de combinacion a probar
|          // Salida (via CCR):
|          //   C=0 (via andi.b #$ee) si NO todos los bits estan activos
|          //   C=1 (via ori.b  #$11) si TODOS los bits estan activos
|
|  --------------------------------------------------------------------------
|  ABSORCION DE DOS FALSOS POSITIVOS Wave F
|  --------------------------------------------------------------------------
|  Los ultimos 12 bytes de esta funcion estaban registrados como:
|    - $05d08e..$05d093 (andi.b #$ee,ccr;rts) = ClearXN_05d08e  (rama C=0)
|    - $05d094..$05d099 (ori.b  #$11,ccr;rts) = SetXN_05d094    (rama C=1)
|  Evidencia forense:
|    - ClearXN_05d08e: 0 callers externos desde codigo matcheado.
|    - SetXN_05d094:   0 callers externos desde codigo matcheado.
|  Es el SEPTIMO par (ClearXN,SetXN) absorbido del proyecto.
|
|  Hallazgos forenses (asm a mano):
|    1. cmp.b d1,d0 en vez de tst.b d0: hand-writer eligiendo entre dos
|       primitivas para dos semanticas distintas. GCC habria emitido
|       el mismo idioma (bne o beq) segun compilase !=0 o ==mask.
|    2. Los tres backends ($5cff8, $5d082) comparten estructura pero
|       divergen en la primitiva de test. Micro-optimizacion humana:
|       el hand-writer escogio el test mas barato para cada semantica.
|  ============================================================================

        .text
        .globl  InputMask_TestChannelAllBits_05d082
        .type   InputMask_TestChannelAllBits_05d082, @function
        .section .text.InputMask_TestChannelAllBits_05d082, "ax", @progbits

InputMask_TestChannelAllBits_05d082:
        move.b  (a2, d0.w), d0              | +00  10 32 00 00
        and.b   d1, d0                      | +04  c0 01
        cmp.b   d1, d0                      | +06  b0 01
        beq.w   .Lset_c                     | +08  67 00 00 08
        andi.b  #0xee, ccr                  | +0c  02 3c 00 ee   C=0 (no todos)
        rts                                 | +10  4e 75
.Lset_c:
        ori.b   #0x11, ccr                  | +12  00 3c 00 11   C=1 (todos)
        rts                                 | +16  4e 75
        .size   InputMask_TestChannelAllBits_05d082, .-InputMask_TestChannelAllBits_05d082
