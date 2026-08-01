| ============================================================================
|  Metal Slug 1 - asm/input_check_channel_05cfa8.s
|  ----------------------------------------------------------------------------
|  Wave U (InputMask event dispatchers) - backend #1
|
|  InputMask_CheckChannelAvail_05cfa8  @ $05cfa8  (32 bytes, 18 callers matcheados)
|
|  Primer backend del cluster InputMask. Comprueba si el canal de entrada
|  actual esta disponible (bit 7 de $100000 CLEAR y $44 de la struct del
|  jugador principal $1001c0 !=0). Si ambas condiciones se cumplen, sale
|  por rts limpiando C=0 (evento aceptado). Si no, salta a la funcion
|  hermana InputMask_ReadCtxSwitchPlayer_05cfc8 que trata el caso de
|  segundo jugador y llama al probe real de "bit ya activado".
|
|  Firma C conceptual (no reproducible por GCC 1:1 - retorno por CCR y
|  fall-through logico al brazo hermano):
|      /* Retorna C=0 si "acepta el evento", o transfiere el control a
|       * InputMask_ReadCtxSwitchPlayer_05cfc8 si hay que probar el bit
|       * del contexto. */
|      void InputMask_CheckChannelAvail_05cfa8(void);
|          // Entrada:
|          //   d0 = layer/slot (2 o 3)
|          //   d1 = mask bit del evento
|          //   a6 = estructura del jugador (con $6d = id P1/P2)
|
|  Flujo:
|      1. btst.b #7, $100000.l   ->  bit 7 del flag global de canales.
|         Si BNE (=bit puesto): saltar al brazo hermano $5cfc8.
|      2. lea $1001c0.l, a4       ->  struct del jugador principal (P1).
|         tst.b $44(a4).
|         Si BEQ (=cero): saltar al brazo hermano $5cfc8.
|      3. andi.b #$ee, ccr       ->  C=0, X=0 (canal libre para el evento).
|      4. rts.
|
|  --------------------------------------------------------------------------
|  ABSORCION DE FALSO POSITIVO Wave F
|  --------------------------------------------------------------------------
|  Los ultimos 6 bytes ($05cfc2..$05cfc7 = "andi.b #$ee,ccr ; rts")
|  estaban registrados como ClearXN_05cfc2. Evidencia forense:
|    - ClearXN_05cfc2: 0 callers externos desde codigo matcheado.
|    - $05cfa8: 18 callers matcheados (los 18 InputEvtThunk_* que no
|                cargan a2 - Wave U#00..17 con backend=$5cfa8).
|  Al absorberlo: -6 B en ccr_helpers.c, +32 B aqui, neto +26 B.
|
|  Hallazgos forenses:
|    1. Retorno por CCR (mascara $EE = mismo idioma que los brazos "exito"
|       de todo el cluster $027Cxx).
|    2. Dos ramas condicionales al MISMO target ($5cfc8), una via bne y otra
|       via beq. Un compilador consolidaria las dos condiciones en un
|       unico test (bit7 |or| !$44).
|    3. lea absoluta a $1001c0 (P1 base) mientras que el brazo hermano
|       $5cfc8 elige P1 o P2 dinamicamente segun $6d(a6). Duplicacion
|       de carga tipica del programador humano.
|  ============================================================================

        .text
        .globl  InputMask_CheckChannelAvail_05cfa8
        .type   InputMask_CheckChannelAvail_05cfa8, @function
        .section .text.InputMask_CheckChannelAvail_05cfa8, "ax", @progbits

InputMask_CheckChannelAvail_05cfa8:
        btst.b  #7, 0x100000                | +00  08 39 00 07 00 10 00 00
        bne.w   InputMask_ReadCtxSwitchPlayer_05cfc8   | +08  66 00 00 16
        lea     0x1001c0.l, a4              | +0c  49 f9 00 10 01 c0
        tst.b   0x44(a4)                    | +12  4a 2c 00 44
        beq.w   InputMask_ReadCtxSwitchPlayer_05cfc8   | +16  67 00 00 08
        andi.b  #0xee, ccr                  | +1a  02 3c 00 ee
        rts                                 | +1e  4e 75
        .size   InputMask_CheckChannelAvail_05cfa8, .-InputMask_CheckChannelAvail_05cfa8
