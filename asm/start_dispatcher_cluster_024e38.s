| ============================================================================
|  Metal Slug 1 - asm/start_dispatcher_cluster_024e38.s
|  ----------------------------------------------------------------------------
|  Wave BB batch 1 - #1..#4
|
|  Cluster contiguo de 3 funciones + 1 tabla de datos que forman el
|  subsistema "procesador del boton START del sistema Neo Geo" en el rango
|  $024E38..$024FB5 (382 B netos, 358 B nuevos + 16 B de tabla).
|
|  El sistema Neo Geo enruta los pulsos de START P1 y START P2 a un latch
|  de bits en $10FDB4 (bit 0 = P1_start_pending, bit 1 = P2_start_pending).
|  Este cluster consume ese latch, aplica la logica de "estado combinado
|  P1+P2" via tabla de 4 filas indexada por (latch & $3) * 4, y actualiza
|  las mascaras publicas de start en $10FDB6/$10FDB7 y los contadores
|  BCD 2-digitos en $1081BF/$1081C0.
|
|  Layout tras Wave BB batch 1:
|
|     $024E10  ClearXN_024e10 + SetXN_024e16       ( 12 B, ccr_helpers)   YA
|     $024E1C..$024E37  (28 B fuera de simbolos, gap del layout original)
|     $024E38  TitleModeInit                       ( 54 B, BB1 #1)
|     $024E6E  JsrAbsThunk_024e6e                  (  8 B, Wave I)         YA
|     $024E76  Player_Start_Inner                  (272 B, BB1 #2)
|     $024F86  Start_Decoder                       ( 32 B, BB1 #3)
|     $024FA6  StartInputTable                     ( 16 B, BB1 #4 tabla)
|     $024FB6  Stub_00024FB6 = Demo_Start_Inner    (  2 B, trivial_rts)    YA
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text

|  ----------------------------------------------------------------------------
|  #1  TitleModeInit_024E38  @ $024E38  (54 B, 1 caller)
|
|  Inicializador del modo Title/Demo. Estructura:
|
|    1. `jsr $2126` (=Input_QueuePush_00212E, Wave S/T ya matcheada), que
|       reinyecta el ultimo evento pendiente en la cola de input.
|    2. `move.b #1, $106ECC.l` + `move.b #1, $106ECD.l`: publica un "1"
|       en los dos start-acknowledge flags (P1 y P2 simultaneos).
|    3. `clr.b $10007B.l`: reset del flag "attract-mode-alive".
|    4. `jsr $97D3C.l`: llamada al Title_ResetScreen (target zona alta,
|       aun no matcheado).
|    5. Ramificacion segun modo (`$10FD82` = P2-active flag):
|         - Si $10FD82 != 0 (dual player, `bne.w`): cae al thunk vecino
|                              JsrAbsThunk_024e6e ($024E6E) que hace
|                              `jsr $99AFC; rts`. TAIL-CALL POR FALLTHROUGH.
|         - Si $10FD82 == 0 (single player): `jsr $99AE2` + `bra.w $024E74`
|                              que aterriza en el rts del thunk vecino.
|           EPILOGO COMPARTIDO con JsrAbsThunk_024e6e.
|
|  Firma C conceptual:
|
|      /* Inicializa el modo Title/Demo del arcade: rearma la cola de
|       * input, publica los start-ack de P1/P2, resetea el flag
|       * attract-alive, prepara la pantalla de titulo y elige la rama
|       * de attract single-player o dual-player. */
|      void TitleModeInit(void);
|
|  Nota forense: el `bra.w $24E74` (+$2c a distancia $8) apunta 2 bytes
|  ANTES del inicio del thunk vecino (a su rts final). Esta es la firma
|  clasica del proyecto para "epilogo absorbido en thunk siguiente".
|  ----------------------------------------------------------------------------

        .globl  TitleModeInit_024E38
        .type   TitleModeInit_024E38, @function
        .section .text.TitleModeInit_024E38, "ax", @progbits

TitleModeInit_024E38:
        jsr     0x2126.l                       | +00  Input_QueuePush_00212E
        move.b  #0x1, 0x106ecc.l               | +06  StartAck_P1 = 1
        move.b  #0x1, 0x106ecd.l               | +0e  StartAck_P2 = 1
        clr.b   0x10007b.l                     | +16  AttractAlive = 0
        jsr     0x9773c.l                      | +1c  Title_ResetScreen
        tst.b   0x10fd82.l                     | +22  if (dual_player_mode)
                                              | +28  bne.w $024E6E (cae al
                                              |       thunk vecino JsrAbsThunk_024e6e
                                              |       que hace jsr $99AFC; rts).
                                              |       Distancia = $C hacia adelante
                                              |       desde PC-tras-opcode ($024E62)
                                              |       hasta $024E6E. Emitido como
                                              |       opcode literal porque el
                                              |       destino esta fuera de esta
                                              |       seccion y no es relocatable
                                              |       via .equ (destino absoluto
                                              |       fuera de rango 16-bit).
        .short  0x6600, 0x000C                 | +28  bne.w #+$C
        jsr     0x99ae2.l                      | +2c  Attract_SingleTick
                                              | +32  bra.w $024E74 (=$024E6E+6,
                                              |       rts del thunk vecino).
                                              |       Distancia = $8 hacia adelante
                                              |       desde PC-tras-opcode ($024E6C)
                                              |       hasta $024E74. Idem opcode
                                              |       literal.
        .short  0x6000, 0x0008                 | +32  bra.w #+8

        .size   TitleModeInit_024E38, .-TitleModeInit_024E38


|  ----------------------------------------------------------------------------
|  #2  Player_Start_Inner_024E76  @ $024E76  (272 B, 1+ callers)
|
|  Procesador dual-jugador del boton START del sistema Neo Geo. Estructura:
|
|    1. Lee el latch bits: d0 = $10FDB4 & $3.
|    2. Convierte a indice de tabla: d0 <<= 2 (stride 4 bytes por fila).
|    3. Consume el latch: bclr.b #0,$10FDB4 y bclr.b #1,$10FDB4.
|    4. Prepara punteros: a1 = &$10FDB6 (mascaras P1/P2 publicas),
|       a0 = &StartInputTable ($24FA6, PC-rel).
|    5. RAMA P1 ($024E9C..$024F01):
|         - Si mascara P1 ($10FDB6) es 1 o 3, skip a rama P2.
|         - Si StartAck_P1 ($106ECC) != 0, skip.
|         - Si dual-mode ($10FD82) != 0, bypasa el gate y sigue.
|         - Si single-mode Y P1_counter_flag ($1081BF) == 0, skip.
|         - Si tabla[d0+2] == 0, skip (no hay transicion).
|         - Publica tabla[d0+2] en $10FDB6.
|         - bset.b #0,$10FDB4 (re-latch P1 processed).
|         - Si dual-mode: skip decoder.
|         - Si single-mode: preserva d0, decodea el contador BCD del
|           P1_counter ($1081BF) via Start_Decoder (#3), publica el
|           resultado en $1081BF, restaura d0.
|    6. RAMA P2 ($024F02..$024F6D): estructura simetrica sobre bit 1,
|       usa columnas [d0+3] de la tabla y contador $1081C0.
|    7. EPILOGO ($024F6E..$024F84):
|         - Recarga latch d1 = $10FDB4 & $3.
|         - Si d1 != 0 (hubo alguna transicion), publica tabla[d0] en
|           $10FDAF (el mismo puerto de gate global de debug HUD Wave X
|           y Wave AA2 - se reutiliza como flag "start pressed en frame").
|         - rts.
|
|  Firma C conceptual:
|
|      /* Consume el latch de start-bits $10FDB4 y actualiza las
|       * mascaras publicas de P1/P2 en $10FDB6/$10FDB7. En modo single
|       * player tambien decrementa los contadores BCD $1081BF/$1081C0
|       * de "presses restantes" via Start_Decoder. Si hubo alguna
|       * transicion, publica un flag en $10FDAF. */
|      void Player_Start_Inner(void);
|
|  Idiomas hand-coded incompatibles con GCC:
|    - Tabla PC-rel embebida entre dos funciones ($024FA6 dentro de esta
|      seccion .text): GCC nunca coloca datos entre funciones en la
|      misma section .text.
|    - `move.b $XX(a0,d0.w),(a1); beq.w exit`: doble uso del flag Z
|      publicado por el propio `move.b`.
|    - `jsr $24F86(pc)`: PC-relative jsr a una funcion "hermana" en la
|      misma seccion, imposible en C sin `__attribute__((section))`
|      combinado con opciones globales de code layout.
|    - Preservacion `move.l d0,-(a7)` / `move.l (a7)+,d0` alrededor de
|      la jsr al decoder (no ABI GCC).
|  ----------------------------------------------------------------------------

        .globl  Player_Start_Inner_024E76
        .type   Player_Start_Inner_024E76, @function
        .section .text.Player_Start_Inner_024E76, "ax", @progbits

Player_Start_Inner_024E76:
        move.b  0x10fdb4.l, d0                 | +00  d0 = latch
        andi.w  #0x3, d0                       | +06  d0 &= 0x3
        lsl.w   #0x2, d0                       | +0a  d0 <<= 2 (stride 4)
        bclr.b  #0x0, 0x10fdb4.l               | +0c  consume P1 bit
        bclr.b  #0x1, 0x10fdb4.l               | +14  consume P2 bit
        lea.l   0x10fdb6.l, a1                 | +1c  a1 = &mask[P1]
        lea     .L2_tbl(pc), a0                | +22  a0 = &StartInputTable
                                              |
                                              | ---- rama P1 ----
        cmpi.b  #0x1, (a1)                     | +26  if (mask[P1] == 1)
        beq.w   .L2_p2                         | +2a     skip
        cmpi.b  #0x3, (a1)                     | +2e  if (mask[P1] == 3)
        beq.w   .L2_p2                         | +32     skip
        tst.b   0x106ecc.l                     | +36  if (StartAck_P1 != 0)
        bne.w   .L2_p2                         | +3c     skip
        tst.b   0x10fd82.l                     | +40  if (dual_mode)
        bne.w   .L2_p1_gate_ok                 | +46     bypass single-gate
        tst.b   0x1081bf.l                     | +4a  if (P1_counter == 0)
        beq.w   .L2_p2                         | +50     skip
.L2_p1_gate_ok:
        cmpi.b  #0x0, 0x2(a0, d0.w)            | +54  if (tbl[d0+2] == 0)
        beq.w   .L2_p2                         | +5a     skip
        move.b  0x2(a0, d0.w), (a1)            | +5e  mask[P1] = tbl[d0+2]
        beq.w   .L2_p2                         | +62  (redundante: Z ya set)
        bset.b  #0x0, 0x10fdb4.l               | +66  re-latch bit 0
        tst.b   0x10fd82.l                     | +6e  if (dual_mode)
        bne.w   .L2_p2                         | +74     skip decoder
        move.l  d0, -(a7)                      | +78  preserve d0
        move.b  0x1081bf.l, d0                 | +7a  d0 = P1_counter
        jsr     .L2_decoder(pc)                | +80  Start_Decoder
        move.b  d0, 0x1081bf.l                 | +84  P1_counter = d0
        move.l  (a7)+, d0                      | +8a  restore d0
                                              |
                                              | ---- rama P2 (simetrica) ----
.L2_p2:
        cmpi.b  #0x1, 0x1(a1)                  | +8c  if (mask[P2] == 1)
        beq.w   .L2_epilogue                   | +92     skip
        cmpi.b  #0x3, 0x1(a1)                  | +96  if (mask[P2] == 3)
        beq.w   .L2_epilogue                   | +9c     skip
        tst.b   0x106ecd.l                     | +a0  if (StartAck_P2 != 0)
        bne.w   .L2_epilogue                   | +a6     skip
        tst.b   0x10fd82.l                     | +aa  if (dual_mode)
        bne.w   .L2_p2_gate_ok                 | +b0     bypass single-gate
        tst.b   0x1081c0.l                     | +b4  if (P2_counter == 0)
        beq.w   .L2_epilogue                   | +ba     skip
.L2_p2_gate_ok:
        cmpi.b  #0x0, 0x3(a0, d0.w)            | +be  if (tbl[d0+3] == 0)
        beq.w   .L2_epilogue                   | +c4     skip
        move.b  0x3(a0, d0.w), 0x1(a1)         | +c8  mask[P2] = tbl[d0+3]
        beq.w   .L2_epilogue                   | +ce  (redundante: Z ya set)
        bset.b  #0x1, 0x10fdb4.l               | +d2  re-latch bit 1
        tst.b   0x10fd82.l                     | +da  if (dual_mode)
        bne.w   .L2_epilogue                   | +e0     skip decoder
        move.l  d0, -(a7)                      | +e4  preserve d0
        move.b  0x1081c0.l, d0                 | +e6  d0 = P2_counter
        jsr     .L2_decoder(pc)                | +ec  Start_Decoder
        move.b  d0, 0x1081c0.l                 | +f0  P2_counter = d0
        move.l  (a7)+, d0                      | +f6  restore d0
                                              |
                                              | ---- epilogo ----
.L2_epilogue:
        move.b  0x10fdb4.l, d1                 | +f8  d1 = new latch
        andi.b  #0x3, d1                       | +fe  d1 &= 0x3
        beq.w   .L2_exit                       | +102 if (no transition) skip
        move.b  (a0, d0.w), 0x10fdaf.l         | +106 debug_gate_flag = tbl[d0]
.L2_exit:
        rts                                    | +10e

        .equ    .L2_tbl,     StartInputTable_024FA6
        .equ    .L2_decoder, Start_Decoder_024F86

        .size   Player_Start_Inner_024E76, .-Player_Start_Inner_024E76


|  ----------------------------------------------------------------------------
|  #3  Start_Decoder_024F86  @ $024F86  (32 B, 2 callers)
|
|  Decoder BCD 2-digitos con auto-wrap. Decrementa el nibble bajo de d0
|  con wrap 0->9 y decremento del nibble alto (BCD countdown).
|
|  Algoritmo:
|
|      d1_saved = d1;
|      d1 = d0;
|      d0 &= 0x0F;             // aislar nibble bajo
|      d1 &= 0xF0;             // aislar nibble alto
|      d0 -= 1;
|      if (d0 < 0) {           // underflow del nibble bajo
|          d0 = 9;             //   wrap del nibble bajo a 9
|          d1 -= 0x10;         //   decrementar nibble alto
|      }
|      d0 |= d1;               // recombinar
|      d1 = d1_saved;
|      return d0;
|
|  Es el 4to elemento del pipeline aritmetico decimal del juego, tras:
|    - Wave X#4   Sub_LongDivide_05D920  (32-iter shift-and-subtract)
|    - Wave Z2#4  BCD_AddClamp99999999_051A10 (sumador BCD 8-digitos)
|    - Wave AA2#5 Sub_Divide10_047656 (divisor por 10 iterativo <100)
|    - Wave BB1#3 Start_Decoder_024F86 (decrementador BCD 2-digitos
|                                       con underflow y wrap)
|
|  Firma C conceptual:
|
|      /* Decrementa el contador BCD 2-digitos (nibble alto = decenas,
|       * nibble bajo = unidades) con wrap 00->99. */
|      uint8_t Start_Decoder(uint8_t bcd /*d0*/);
|
|  Idiomas hand-coded:
|    - Preserva d1 con `move.l d1,-(a7)` / `move.l (a7)+,d1`.
|    - Usa `subq.b #1,d0` + `bpl.w` para detectar underflow por el bit
|      N sin operacion extra.
|  ----------------------------------------------------------------------------

        .globl  Start_Decoder_024F86
        .type   Start_Decoder_024F86, @function
        .section .text.Start_Decoder_024F86, "ax", @progbits

Start_Decoder_024F86:
        move.l  d1, -(a7)                      | +00  preserve d1
        move.b  d0, d1                         | +02  d1 = d0
        andi.b  #0xf, d0                       | +04  d0 = low nibble
        andi.b  #0xf0, d1                      | +08  d1 = high nibble
        subq.b  #0x1, d0                       | +0c  --d0
        bpl.w   .L3_no_wrap                    | +0e  if (d0 >= 0) skip wrap
        move.b  #0x9, d0                       | +12  wrap: d0 = 9
        subi.b  #0x10, d1                      | +16  --high nibble
.L3_no_wrap:
        or.b    d1, d0                         | +1a  recombine
        move.l  (a7)+, d1                      | +1c  restore d1
        rts                                    | +1e

        .size   Start_Decoder_024F86, .-Start_Decoder_024F86


|  ----------------------------------------------------------------------------
|  #4  StartInputTable_024FA6  @ $024FA6  (16 B, tabla de datos)
|
|  Tabla PC-relative embebida entre Start_Decoder (#3) y Stub_00024FB6.
|  4 filas de 4 bytes cada una, indexada por (latch & $3) * 4 = d0<<2.
|
|  Layout: cada fila codifica el estado combinado de P1+P2 tras la
|  transicion pendiente:
|
|      d0=$00 (row 0, latch=%00): {00,00,00,00} - no hay transicion
|      d0=$04 (row 1, latch=%01): {02,01,01,00} - solo P1 press
|      d0=$08 (row 2, latch=%10): {02,02,00,01} - solo P2 press
|      d0=$0C (row 3, latch=%11): {02,03,01,01} - P1+P2 simultaneo
|
|  Los cuatro campos por fila corresponden a los desplazamientos usados
|  por Player_Start_Inner:
|      +0  -> $10FDAF  (debug gate flag global)
|      +1  -> no leido directamente (padding/reserva)
|      +2  -> $10FDB6 (mask P1)
|      +3  -> $10FDB7 (mask P2)
|
|  Es la primera tabla de datos incrustada del proyecto reconstruida
|  byte-a-byte. Se expone tanto por nombre semantico (StartInputTable)
|  como por dumping literal (.byte) para reproduccion bit-exacta.
|  ----------------------------------------------------------------------------

        .globl  StartInputTable_024FA6
        .type   StartInputTable_024FA6, @object
        .section .text.StartInputTable_024FA6, "ax", @progbits
                                              |
                                              | Nota: pese a ser tabla de datos
                                              | conceptualmente, en la ROM esta
                                              | fisicamente entre codigo y esta
                                              | integrada en la misma seccion
                                              | .text que el resto del proyecto
                                              | (el linker script solo procesa
                                              | secciones .text.*). Colocada en
                                              | $024FA6 por dir CPU absoluta.

StartInputTable_024FA6:
        .byte   0x00, 0x00, 0x00, 0x00         | row 0 (latch = %00): idle
        .byte   0x02, 0x01, 0x01, 0x00         | row 1 (latch = %01): P1 press
        .byte   0x02, 0x02, 0x00, 0x01         | row 2 (latch = %10): P2 press
        .byte   0x02, 0x03, 0x01, 0x01         | row 3 (latch = %11): P1+P2

        .size   StartInputTable_024FA6, .-StartInputTable_024FA6
