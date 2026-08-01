| ============================================================================
|  Metal Slug 1 - asm/entity_state_publishers_057044.s
|  ----------------------------------------------------------------------------
|  Wave BB batch 2 - #1..#11
|
|  Cluster de 11 helpers cortos del subsistema "state machine per-entity"
|  en el rango $057044..$057225 (272 B netos, 11 funciones). Todos operan
|  sobre `a6` = self (convencion `a6` implicita del proyecto) y publican
|  flags en los bytes de estado `$72(a6)` y `$74(a6)`, mas un byte de
|  substate en `$75(a6)`.
|
|  El cluster forma parte de un subsistema mas amplio ($057000..$057540)
|  con 4 dispatchers grandes contaminados por thunks Wave H/D/I (por eso
|  aparecen como MATCHED parciales: $057000, $05702A, $0570A8, $057226).
|  Esos dispatchers se atacaran en oleadas siguientes absorbiendo los
|  falsos positivos correspondientes. Esta oleada BB batch 2 solo cubre
|  los 11 huecos limpios entre esos dispatchers.
|
|  Descubrimiento semantico: los flags per-entity siguen un layout consistente:
|      $72(a6)   byte de bit-flags principales (bit 4 = "active" / "dirty")
|      $74(a6)   byte de bit-flags secundarios (bits 0..4 = state selector)
|      $75(a6)   byte de substate (0..3 = variantes finas dentro del state)
|      $5C(a6)   byte de sprite frame / tile-id publicado
|      $24(a6)   word de coordenada Y del entity
|      $3A(a6)   byte de gate/tag (bit 0 = "team A/B")
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text

|  ----------------------------------------------------------------------------
|  #1  EntityFrame_FrameSelectByGate3A_057044  @ $057044  (54 B)
|
|  Selector de frame de sprite por gate $3A(a6) y clamp Y. Estructura:
|
|    if ((self->tag_3A & 1) == 0) {
|        d1 = 4;  d2 = 5;  d3 = 0x10;    // team A: frames 4/5/16
|    } else {
|        d1 = 0x11; d2 = 0x12; d3 = 0x13; // team B: frames 17/18/19
|    }
|    if (self->y_24 < 0x180) {
|        goto H02;                        // tail-call al selector fino
|    }
|    self->frame_5C = d3;                 // fuera de banda: frame borde
|
|  Publica en d1/d2/d3 los tres frames "posibles" derivados del team.
|  Si el entity aun esta dentro de la banda de juego (`Y < 0x180`), no
|  retorna: hace **tail-call por branch** a la funcion vecina H02, que usa
|  la cache d1/d2/d3 para el commit fino por distancia. Solo cuando el
|  entity ya esta muy abajo en pantalla (Y >= 0x180 = 384) commitea `d3`
|  directamente en `$5C(a6)` y retorna.
|  Esto explica el displacement real del ROM (`bcs.w #+$8`): salta al
|  INICIO de H02 ($05707A), no al `rts` local.
| 
|  Idiomas hand-coded:
|    - Publica en 3 registros de datos "para llevar" y solo commitea uno
|      en el fall-through. GCC no genera esto porque no puede reservar
|      d1/d2/d3 vivos entre funciones sin ABI extension.
|
|  Firma C conceptual:
|
|      /* Selecciona los 3 frames candidatos segun team ($3A bit 0) y
|       * publica el frame "borde" (d3) en $5C si el entity esta fuera
|       * de la banda de juego. */
|      void EntityFrame_FrameSelectByGate3A(struct Entity *self /*a6*/);
|  ----------------------------------------------------------------------------

        .globl  EntityFrame_FrameSelectByGate3A_057044
        .type   EntityFrame_FrameSelectByGate3A_057044, @function
        .section .text.EntityFrame_FrameSelectByGate3A_057044, "ax", @progbits

EntityFrame_FrameSelectByGate3A_057044:
        btst.b  #0x0, 0x3a(a6)                 | +00  if (self->tag_3A & 1)
        bne.w   .L1_teamB                      | +06     goto teamB
        move.b  #0x4, d1                       | +0a  d1 = 4  (team A frame lo)
        move.b  #0x5, d2                       | +0e  d2 = 5  (team A frame mid)
        move.b  #0x10, d3                      | +12  d3 = 16 (team A frame border)
        bra.w   .L1_check_y                    | +16  goto check_y
.L1_teamB:
        move.b  #0x11, d1                      | +1a  d1 = 17 (team B frame lo)
        move.b  #0x12, d2                      | +1e  d2 = 18 (team B frame mid)
        move.b  #0x13, d3                      | +22  d3 = 19 (team B frame border)
.L1_check_y:
        cmpi.w  #0x180, 0x24(a6)               | +26  if (self->y < 0x180)
        bcs.w   EntityFrame_FrameSelectByXDist_05707A
                                              | +2c     tail-call al selector fino H02
        move.b  d3, 0x5c(a6)                   | +30  self->frame_5C = d3 (border)
        rts                                    | +34

        .size   EntityFrame_FrameSelectByGate3A_057044, .-EntityFrame_FrameSelectByGate3A_057044


|  ----------------------------------------------------------------------------
|  #2  EntityFrame_FrameSelectByXDist_05707A  @ $05707A  (46 B)
|
|  Selector de frame de sprite por distancia X respecto a otro entity.
|  Recibe puntero al otro entity en `d0` (idioma "arg por registro de
|  datos"), y usa los 3 frames cacheados en d1/d2/d3 por #1.
|
|  Algoritmo:
|
|    if (d0 == 0) goto commit;               // sin target: commit d1 (frame lo)
|    a0 = (Entity*)d0;
|    d0 = a0->y_24 - self->y_24;             // Y-distance (NO X pese al nombre
|                                            // legacy: el desplazamiento
|                                            // usado es $24, coord Y)
|    if (d0 > 0x60) {                        // muy lejos abajo
|        d1 = d2;                            // usa frame mid
|        goto commit;
|    }
|    if (d0 >= 0xFFC0) {                     // dentro de banda [-0x40..+0x60]
|        goto commit;                        // usa frame lo (d1 sin cambio)
|    }
|    d1 = d3;                                // muy lejos arriba (o negativo
|                                            //   fuera de banda): frame border
|  commit:
|    self->frame_5C = d1;
|
|  Nota importante: pese al sufijo "XDist" original de mi hipotesis, el
|  desplazamiento usado es `$24(a6)` que resulta ser COORD Y en el
|  layout del entity. Reetiquetare a "ByYDist" en un pase futuro tras
|  confirmar con otros callers.
|
|  Idiomas hand-coded:
|    - Recibe puntero por `d0` (no ABI GCC): a1 se establece via `movea.l d0,a0`.
|    - Consume d1/d2/d3 pre-cargados por H01 (no ABI).
|    - Compara con `#$FFC0` = -0x40 en signed word interpretado como unsigned
|      (bge.w = mayor-o-igual sin signo, o sea >=0xFFC0). Idioma clasico
|      para "distancia dentro de +-0x40 tomada como unsigned wrap".
|
|  Firma C conceptual:
|
|      /* Selecciona el frame de sprite del entity `self` en funcion de
|       * la distancia Y respecto a `other` (pasado por d0 = puntero).
|       * Usa la cache d1/d2/d3 preparada por FrameSelectByGate3A. */
|      void EntityFrame_FrameSelectByXDist(struct Entity *other /*d0*/,
|                                          struct Entity *self /*a6*/);
|  ----------------------------------------------------------------------------

        .globl  EntityFrame_FrameSelectByXDist_05707A
        .type   EntityFrame_FrameSelectByXDist_05707A, @function
        .section .text.EntityFrame_FrameSelectByXDist_05707A, "ax", @progbits

EntityFrame_FrameSelectByXDist_05707A:
        tst.l   d0                             | +00  if (other == 0)
        beq.w   .L2_commit                     | +02     goto commit
        movea.l d0, a0                         | +06  a0 = other
        move.w  0x24(a0), d0                   | +08  d0 = other->y
        sub.w   0x24(a6), d0                   | +0c  d0 -= self->y
        cmpi.w  #0x60, d0                      | +10  if (dy <= 0x60)
        ble.w   .L2_check_below                | +14     goto check_below
        move.b  d2, d1                         | +18  d1 = d2 (frame mid)
        bra.w   .L2_commit                     | +1a  goto commit
.L2_check_below:
        cmpi.w  #0xffc0, d0                    | +1e  if (dy >= -0x40 unsigned)
        bge.w   .L2_commit                     | +22     goto commit (frame lo)
        move.b  d3, d1                         | +26  d1 = d3 (frame border)
.L2_commit:
        move.b  d1, 0x5c(a6)                   | +2a  self->frame_5C = d1
        rts                                    | +2e

        .size   EntityFrame_FrameSelectByXDist_05707A, .-EntityFrame_FrameSelectByXDist_05707A


|  ----------------------------------------------------------------------------
|  #3  EntityState_PublishByProbeN_05717A  @ $05717A  (34 B)
|
|  Publica state 3 con flag `$72 bit 4` derivado del bit N de un probe:
|
|    if (probe_result_via_N_bit(jsr $5E9B6) is negative) {
|        clear bit 4 of $72(a6);         // desactiva "active"
|    } else {
|        set   bit 4 of $72(a6);         // activa "active"
|    }
|    set   bit 3 of $74(a6);             // set state = 3
|
|  El `bpl.w` inmediatamente despues del `jsr` explota el bit N que la
|  funcion probada deja en CCR como retorno booleano (idioma retorno-por-CCR
|  ya visto en Wave S/T/U/Z: `Sub_0005E9B6` es un probe con retorno signed).
|
|  Firma C conceptual:
|
|      /* Activa/desactiva la flag "active" ($72 bit 4) del entity segun
|       * el signo del resultado del probe $5E9B6, y publica state 3
|       * en $74(a6). */
|      void EntityState_PublishByProbeN(struct Entity *self /*a6*/);
|  ----------------------------------------------------------------------------

        .globl  EntityState_PublishByProbeN_05717A
        .type   EntityState_PublishByProbeN_05717A, @function
        .section .text.EntityState_PublishByProbeN_05717A, "ax", @progbits

EntityState_PublishByProbeN_05717A:
        jsr     0x5e9b6.l                      | +00  probe (retorno via CCR/N)
        bpl.w   .L3_active                     | +06  if (N == 0) goto active
        bclr.b  #0x4, 0x72(a6)                 | +0a  clear "active" bit
        bra.w   .L3_set_state                  | +10  goto set_state
.L3_active:
        bset.b  #0x4, 0x72(a6)                 | +14  set "active" bit
.L3_set_state:
        bset.b  #0x3, 0x74(a6)                 | +1a  state = 3
        rts                                    | +20

        .size   EntityState_PublishByProbeN_05717A, .-EntityState_PublishByProbeN_05717A


|  ----------------------------------------------------------------------------
|  #4  EntityState_PublishByProbeN_ClearSub75_05719C  @ $05719C  (40 B)
|
|  Variante de #3 con reset del substate. Estructura identica a #3 hasta
|  el `bset.b #3, $74(a6)`, y luego `move.b #0, $75(a6)` como cierre.
|
|  Idioma "clone-of-clone" del cluster de state publishers: el juego mantiene
|  dos variantes casi identicas (con y sin reset de $75) porque el estado
|  anterior determina si el substate debe preservarse (H03) o reiniciarse
|  a 0 (H04).
|  ----------------------------------------------------------------------------

        .globl  EntityState_PublishByProbeN_ClearSub75_05719C
        .type   EntityState_PublishByProbeN_ClearSub75_05719C, @function
        .section .text.EntityState_PublishByProbeN_ClearSub75_05719C, "ax", @progbits

EntityState_PublishByProbeN_ClearSub75_05719C:
        jsr     0x5e9b6.l                      | +00  probe (retorno via CCR/N)
        bpl.w   .L4_active                     | +06  if (N == 0) goto active
        bclr.b  #0x4, 0x72(a6)                 | +0a  clear "active" bit
        bra.w   .L4_set_state                  | +10  goto set_state
.L4_active:
        bset.b  #0x4, 0x72(a6)                 | +14  set "active" bit
.L4_set_state:
        bset.b  #0x3, 0x74(a6)                 | +1a  state = 3
        move.b  #0x0, 0x75(a6)                 | +20  substate = 0
        rts                                    | +26

        .size   EntityState_PublishByProbeN_ClearSub75_05719C, .-EntityState_PublishByProbeN_ClearSub75_05719C


|  ----------------------------------------------------------------------------
|  #5  EntityState_SetSubstate2_0571C4  @ $0571C4  (20 B)
|
|  Publica combinacion state 3 / substate 2 sin probe previo (assume
|  active): bset $72 bit 4 + bset $74 bit 3 + move $75 = 2.
|
|  Es una variante "fija" de #3/#4 que salta el probe y siempre activa
|  el entity. Substate 2 es una de las 4 variantes finas (0/1/2/3) que
|  el subsistema define.
|
|  Firma C conceptual:
|
|      /* Publica (state=3, substate=2, active) sin probe. */
|      void EntityState_SetSubstate2(struct Entity *self /*a6*/);
|  ----------------------------------------------------------------------------

        .globl  EntityState_SetSubstate2_0571C4
        .type   EntityState_SetSubstate2_0571C4, @function
        .section .text.EntityState_SetSubstate2_0571C4, "ax", @progbits

EntityState_SetSubstate2_0571C4:
        bset.b  #0x4, 0x72(a6)                 | +00  set "active" bit
        bset.b  #0x3, 0x74(a6)                 | +06  state = 3
        move.b  #0x2, 0x75(a6)                 | +0c  substate = 2
        rts                                    | +12

        .size   EntityState_SetSubstate2_0571C4, .-EntityState_SetSubstate2_0571C4


|  ----------------------------------------------------------------------------
|  #6  EntityState_SetSubstate1_0571D8  @ $0571D8  (20 B)
|
|  Clon exacto de #5 con substate 1 en lugar de 2.
|  ----------------------------------------------------------------------------

        .globl  EntityState_SetSubstate1_0571D8
        .type   EntityState_SetSubstate1_0571D8, @function
        .section .text.EntityState_SetSubstate1_0571D8, "ax", @progbits

EntityState_SetSubstate1_0571D8:
        bset.b  #0x4, 0x72(a6)                 | +00  set "active" bit
        bset.b  #0x3, 0x74(a6)                 | +06  state = 3
        move.b  #0x1, 0x75(a6)                 | +0c  substate = 1
        rts                                    | +12

        .size   EntityState_SetSubstate1_0571D8, .-EntityState_SetSubstate1_0571D8


|  ----------------------------------------------------------------------------
|  #7  EntityState_SetSubstate3_0571EC  @ $0571EC  (20 B)
|
|  Clon exacto de #5 con substate 3.
|  ----------------------------------------------------------------------------

        .globl  EntityState_SetSubstate3_0571EC
        .type   EntityState_SetSubstate3_0571EC, @function
        .section .text.EntityState_SetSubstate3_0571EC, "ax", @progbits

EntityState_SetSubstate3_0571EC:
        bset.b  #0x4, 0x72(a6)                 | +00  set "active" bit
        bset.b  #0x3, 0x74(a6)                 | +06  state = 3
        move.b  #0x3, 0x75(a6)                 | +0c  substate = 3
        rts                                    | +12

        .size   EntityState_SetSubstate3_0571EC, .-EntityState_SetSubstate3_0571EC


|  ----------------------------------------------------------------------------
|  #8  EntityState_SetState74Bit4_057200  @ $057200  (14 B)
|
|  Publica "active" ($72 bit 4) + estado $74 bit 4 sin substate. Es el
|  unico state-setter del cluster que usa el bit 4 de $74 (no el 3).
|
|  Firma C conceptual:
|
|      /* Publica (state74_bit4=1, active) sin substate. */
|      void EntityState_SetState74Bit4(struct Entity *self /*a6*/);
|  ----------------------------------------------------------------------------

        .globl  EntityState_SetState74Bit4_057200
        .type   EntityState_SetState74Bit4_057200, @function
        .section .text.EntityState_SetState74Bit4_057200, "ax", @progbits

EntityState_SetState74Bit4_057200:
        bset.b  #0x4, 0x72(a6)                 | +00  set "active" bit
        bset.b  #0x4, 0x74(a6)                 | +06  state = 74_bit4
        rts                                    | +0c

        .size   EntityState_SetState74Bit4_057200, .-EntityState_SetState74Bit4_057200


|  ----------------------------------------------------------------------------
|  #9  EntityState_SetState74Bit0_05720E  @ $05720E  (8 B)
|
|  Minimo del cluster: solo `bset.b #0, $74(a6)` + rts.
|
|  Contrapartida per-entity de los `SetXN`/`ClearXN` de Wave F: el mismo
|  patron de 6+2=8 B, pero con destino un byte de memoria per-entity en
|  lugar del registro CCR. Por eso NO pueden ir en `ccr_helpers.c` a
|  pesar de ser la misma silueta binaria.
|
|  Firma C conceptual:
|
|      void EntityState_SetState74Bit0(struct Entity *self /*a6*/);
|  ----------------------------------------------------------------------------

        .globl  EntityState_SetState74Bit0_05720E
        .type   EntityState_SetState74Bit0_05720E, @function
        .section .text.EntityState_SetState74Bit0_05720E, "ax", @progbits

EntityState_SetState74Bit0_05720E:
        bset.b  #0x0, 0x74(a6)                 | +00  state = 74_bit0
        rts                                    | +06

        .size   EntityState_SetState74Bit0_05720E, .-EntityState_SetState74Bit0_05720E


|  ----------------------------------------------------------------------------
|  #10 EntityState_SetState74Bit2_057216  @ $057216  (8 B)
|
|  Clon de #9 con bit 2.
|  ----------------------------------------------------------------------------

        .globl  EntityState_SetState74Bit2_057216
        .type   EntityState_SetState74Bit2_057216, @function
        .section .text.EntityState_SetState74Bit2_057216, "ax", @progbits

EntityState_SetState74Bit2_057216:
        bset.b  #0x2, 0x74(a6)                 | +00  state = 74_bit2
        rts                                    | +06

        .size   EntityState_SetState74Bit2_057216, .-EntityState_SetState74Bit2_057216


|  ----------------------------------------------------------------------------
|  #11 EntityState_SetState74Bit1_05721E  @ $05721E  (8 B)
|
|  Clon de #9 con bit 1.
|  ----------------------------------------------------------------------------

        .globl  EntityState_SetState74Bit1_05721E
        .type   EntityState_SetState74Bit1_05721E, @function
        .section .text.EntityState_SetState74Bit1_05721E, "ax", @progbits

EntityState_SetState74Bit1_05721E:
        bset.b  #0x1, 0x74(a6)                 | +00  state = 74_bit1
        rts                                    | +06

        .size   EntityState_SetState74Bit1_05721E, .-EntityState_SetState74Bit1_05721E
