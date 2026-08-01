| ============================================================================
|  Metal Slug 1 - asm/input_test_nibble_xor_05d1c0.s
|  ----------------------------------------------------------------------------
|  Wave U (InputMask event dispatchers) - sub-cluster #5, backend TestNibbleXor
|
|  InputMask_TestChannelNibbleXor_05d1c0  @ $05d1c0  (26 bytes, 4 callers
|                                                    de v3 desde thunks del
|                                                    sub-cluster #4a)
|
|  TERCERA variante del backend de test del cluster InputMask, distinta
|  a las dos anteriores:
|
|      cluster #1 ($5cff8, "algun bit activo"):
|          move.b (a2,d0.w), d0
|          and.b  d1, d0
|          bne.w  .Lset_c          <-- Z=0 si algun bit -> C=1
|
|      cluster #2 v2 ($5d082, "TODOS los bits activos"):
|          move.b (a2,d0.w), d0
|          and.b  d1, d0
|          cmp.b  d1, d0
|          beq.w  .Lset_c          <-- d0==d1 (todos)  -> C=1
|
|      cluster #5 v3 ($5d1c0, "exclusion XOR sobre nibble bajo"):
|          move.b (a2,d0.w), d0
|          andi.b #$f, d0          <-- solo nibble bajo
|          eor.b  d1, d0           <-- XOR: coincide si d0 XOR d1 == 0
|          beq.w  .Lset_c          <-- d0==d1 (exactamente) -> C=1
|
|  Este es el "detector de direccion exclusiva" en el D-pad: comprueba
|  que SOLO ese bit del nibble bajo este activo, no combinaciones. Los
|  cuatro bits del nibble bajo son las 4 direcciones del joypad (up,
|  down, left, right) - solo se debe activar UNA direccion a la vez.
|
|  Firma C conceptual:
|      void InputMask_TestChannelNibbleXor_05d1c0(void);
|          // Entrada:
|          //   a2 = buffer de eventos, d0 = slot, d1 = mask de direccion
|          // Salida (via CCR):
|          //   C=0 si el nibble bajo NO coincide EXACTAMENTE con d1
|          //   C=1 si el nibble bajo == d1 (direccion pulsada aislada)
|
|  Absorbe ClearXN_05d1ce + SetXN_05d1d4 (falsos positivos Wave F).
|
|  OCTAVO par (ClearXN,SetXN) absorbido del proyecto.
|  ============================================================================

        .text
        .globl  InputMask_TestChannelNibbleXor_05d1c0
        .type   InputMask_TestChannelNibbleXor_05d1c0, @function
        .section .text.InputMask_TestChannelNibbleXor_05d1c0, "ax", @progbits

InputMask_TestChannelNibbleXor_05d1c0:
        move.b  (a2, d0.w), d0              | +00  10 32 00 00
        andi.b  #0xf, d0                    | +04  02 00 00 0f  (solo nibble bajo)
        eor.b   d1, d0                      | +08  b3 00        (XOR con mask)
        beq.w   .Lset_c                     | +0a  67 00 00 08  (=0 => exacto)
        andi.b  #0xee, ccr                  | +0e  02 3c 00 ee  C=0 (no exacto)
        rts                                 | +12  4e 75
.Lset_c:
        ori.b   #0x11, ccr                  | +14  00 3c 00 11  C=1 (exacto)
        rts                                 | +18  4e 75
        .size   InputMask_TestChannelNibbleXor_05d1c0, .-InputMask_TestChannelNibbleXor_05d1c0
