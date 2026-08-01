| ============================================================================
|  Metal Slug 1 - asm/input_test_bit_05cff8.s
|  ----------------------------------------------------------------------------
|  Wave U (InputMask event dispatchers) - backend #3
|
|  InputMask_TestChannelBit_05cff8  @ $05cff8  (22 bytes, 10 callers matcheados
|                                              como bra.w desde thunks Wave U +
|                                              fall-through desde $5cfc8)
|
|  Tercer y ultimo backend del cluster InputMask. Recibe a2 = buffer de
|  input y (d0.w, d1) = (slot, mask). Prueba si el bit ya esta activo
|  en el slot: si lo esta, retorna C=1 (SetXN); si no, retorna C=0
|  (ClearXN). El caller usa el CCR para decidir si emitir el evento.
|
|  Firma C conceptual (dos brazos hermanos que retornan por CCR):
|      void InputMask_TestChannelBit_05cff8(void);
|          // Entrada:
|          //   a2 = puntero al buffer de bytes de estado de eventos
|          //   d0 = slot (usado como word-index en a2[d0.w])
|          //   d1 = mascara de bit a probar
|          // Salida (via CCR):
|          //   C=0 (via andi.b #$ee) si bit NO estaba activo
|          //   C=1 (via ori.b #$11)  si bit YA estaba activo
|
|  Flujo:
|      1. move.b (a2, d0.w), d0   ->  d0 = estado actual del slot.
|      2. and.b  d1, d0            ->  d0 = bit(s) coincidentes.
|      3. bne.w  $5d008            ->  si hay bit activo, saltar al brazo
|                                      SetC.
|      4. andi.b #$ee, ccr         ->  C=0, X=0 (bit inactivo -> evento nuevo).
|      5. rts.
|      Brazo alternativo (alcanzable via bne.w $5d008):
|      6. ori.b  #$11, ccr         ->  C=1, X=1 (bit ya activo -> ya emitido).
|      7. rts.
|
|  --------------------------------------------------------------------------
|  ABSORCION DE DOS FALSOS POSITIVOS Wave F
|  --------------------------------------------------------------------------
|  Los ultimos 12 bytes de esta funcion estaban registrados como dos
|  helpers CCR independientes en ccr_helpers.c:
|    - $05d002..$05d007 (andi.b #$ee,ccr;rts) = ClearXN_05d002  (rama C=0)
|    - $05d008..$05d00d (ori.b  #$11,ccr;rts) = SetXN_05d008    (rama C=1)
|  Evidencia forense:
|    - ClearXN_05d002: 0 callers externos desde codigo matcheado.
|    - SetXN_05d008:   0 callers externos desde codigo matcheado.
|    - $05cff8: 10 callers matcheados (los 10 InputEvtThunk_* de la Wave
|               U que cargan a2 con $10E200/$10E206).
|  Al absorberlos: -12 B en ccr_helpers.c, +22 B aqui, neto +10 B.
|
|  Este es el SEXTO par (ClearXN,SetXN) absorbido del proyecto:
|    - T#7/T#8   absorbieron ClearXN_027d2c + SetXN_027d4a
|    - T#9/T#10  absorbieron ClearXN_027cca + SetXN_027ce8
|    - T#11/T#12 absorbieron ClearXN_027c06 + SetXN_027c24
|    - T#13/T#14 absorbieron ClearXN_027c68 + SetXN_027c86
|    - T#15/T#16 absorbieron ClearXN_027d8e + SetXN_027dac
|    - U backends absorben ClearXN_05cfc2 (single, no par),
|                          ClearXN_05d002 + SetXN_05d008 (par)
|
|  Hallazgos forenses (asm a mano):
|    1. Retorno por CCR con dos brazos hermanos (mismo idioma que el
|       cluster $027Cxx probe/revert).
|    2. and.b entre d0 (cargado de memoria por byte) y d1 (mask del
|       evento). Es un test-and-report, no un test-and-clear: el bit
|       no se modifica -- por eso el thunk caller es quien lo pone
|       tras recibir C=0.
|  ============================================================================

        .text
        .globl  InputMask_TestChannelBit_05cff8
        .type   InputMask_TestChannelBit_05cff8, @function
        .section .text.InputMask_TestChannelBit_05cff8, "ax", @progbits

InputMask_TestChannelBit_05cff8:
        move.b  (a2, d0.w), d0              | +00  10 32 00 00
        and.b   d1, d0                      | +04  c0 01
        bne.w   .Lset_c                     | +06  66 00 00 08
        andi.b  #0xee, ccr                  | +0a  02 3c 00 ee   C=0 (bit inactivo)
        rts                                 | +0e  4e 75
.Lset_c:
        ori.b   #0x11, ccr                  | +10  00 3c 00 11   C=1 (bit activo)
        rts                                 | +14  4e 75
        .size   InputMask_TestChannelBit_05cff8, .-InputMask_TestChannelBit_05cff8
