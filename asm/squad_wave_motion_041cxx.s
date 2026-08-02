| ============================================================================
|  Metal Slug 1 - asm/squad_wave_motion_041cxx.s
|  ----------------------------------------------------------------------------
|  Wave TT (parte 1/2) - nucleo de movimiento del subsistema "escuadron":
|  formacion de 8 entities con vuelo ondulatorio senoidal (los enjambres
|  voladores del juego). Cluster $041C1A..$041FB4, 20 funciones, 824 B.
|
|  Mapa del cluster (islas ya matcheadas intercaladas, se conservan):
|    $041C1A Squad_ComputeTargetPos_041C1A   (56 B)  <- promueve PcThunkTarget_041c1a
|    $041C52 Squad_InitFormationSlot_041C52  (68 B)  -> cae en SetTaskB_041c96
|    $041C9C Squad_PickSwoopState_041C9C     (62 B)  -> cae en JsrPcThunk_041cda
|    $041CE0 Squad_SteerTowardTarget_041CE0 (120 B)  -> cae en ClearXN_041d58
|    $041D5E Squad_HaltVelocity_041D5E        (8 B)  -> cae en SetXN_041d66
|    $041D6C Squad_TurnRateStepClamp_041D6C  (44 B)  -> cae en SetXN_041d98
|    $041D9E Squad_TurnRateClampHi_041D9E    (12 B)  -> cae en SetXN_041daa
|    $041DB6 Squad_BobYFast_041DB6           (38 B)  -> bra Squad_BobYApply
|    $041DDC Squad_BobYWide_041DDC           (38 B)  <- promueve PcThunkTarget_041ddc
|    $041E02 Squad_BobYNarrow_041E02         (34 B)  <- promueve PcThunkTarget_041e02
|    $041E24 Squad_BobYApply_041E24          (24 B)  -> cae en ClearXN_041e3c
|    $041E42 Squad_BobYRestore_041E42         (6 B)  -> cae en SetXN_041e48
|    $041E4E Squad_StateDispatch_041E4E      (34 B)
|    $041E70 SquadAnim_State4Select_041E70   (58 B)  -> cae en JsrAbsThunk_041eaa
|    $041EB2 SquadAnim_State5Select_041EB2   (52 B)  -> cae en JsrAbsThunk_041ee6
|    $041EEE SquadAnim_ApproachSelect_041EEE (38 B)  -> cae en JsrAbsThunk_041f14
|    $041F1C SquadAnim_ArriveSelect_041F1C   (22 B)  -> cae en JsrAbsThunk_041f32
|    $041F3A Squad_TagSharedBit_041F3A       (14 B)
|    $041F48 Squad_PhaseStepToTarget_041F48  (48 B)  2 entradas (+4 / +1)
|    $041F84 Squad_ApplyLeaderDelta_041F84   (48 B)
|
|  Layout de campos de entity confirmado por este cluster:
|    +0x22/+0x24 = pos X/Y      +0x28/+0x2A = velocidad X/Y (salida sincos)
|    +0x36 = amplitud/turn-rate  +0x76 = heading (angulo 0..255) cacheado
|    +0x78 = heading objetivo    +0x7A = acumulador de fase senoidal
|    +0x7C = flag de template ($FF = variante espejada, copiado del lider)
|    +0x80/+0x81 = estado compartido leido/cacheado   +0x82..+0x85 = registro
|    de miembro (dx,dy,dz,bit-id, sembrado por Squad_SpawnEight en $041FB4)
|    +0x89 = estado de vuelo     +0x8A/+0x8C = target X/Y   +0x8E = delta bob
|    +0x0C = puntero a entity/estructura compartida (lider del escuadron)
|
|  Callees externos (aun sin matchear, abs.l literales):
|    $440D0  = transformacion de coordenada de formacion (camara/scroll)
|    $5E018  = Angle_FromDelta (atan2 de tabla: dx,dy -> angulo 0..255 en d0)
|    $5E1EA  = obtiene en a0 la entity objetivo (jugador mas cercano)
|    $5E9B6  = RNG global (estado en RAM, devuelve byte en d0)
|    $2C072C = tabla seno (256 palabras s16); $2C07AC = +64 palabras = coseno
|
|  HALLAZGO: dos nuevos bloques ASSERT trap#15 nop-patched del build de
|  desarrollo de Nazca (mismo patron que Wave SS): ASSERT(d1 != 0) en
|  $041D74 y ASSERT(estado < 6) en $041E56 delante del jump table $28633C.
| ============================================================================

| ----------------------------------------------------------------------------
|  Squad_ComputeTargetPos_041C1A  @ $041C1A  (56 B)
|
|    /* Calcula la posicion objetivo del slot de formacion (+0x89 & 0xF)
|       leyendo la tabla de offsets $286124 (16 pares de words X,Y), la
|       transforma a coordenadas de mundo via $440D0 (Y entra -640) y
|       clampa X a $110. Resultado en +0x8A/+0x8C (y d0/d1). */
|    void Squad_ComputeTargetPos(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.Squad_ComputeTargetPos_041C1A, "ax", @progbits
        .global Squad_ComputeTargetPos_041C1A
Squad_ComputeTargetPos_041C1A:
        lea     0x286124.l, a0                  | +000  tabla de formacion
        move.b  0x89(a6), d3                    | +006  estado de vuelo
        andi.w  #0xF, d3                        | +00a  slot = estado & 15
        lsl.w   #2, d3                          | +00e  *4 (pares de words)
        move.w  (a0,d3.w), d0                   | +010  X de formacion
        move.w  2(a0,d3.w), d1                  | +014  Y de formacion
        subi.w  #0x280, d1                      | +018  Y -= 640 (altura pantalla*?)
        jsr     0x440d0.l                       | +01c  a coords de mundo
        cmpi.w  #0x110, d0                      | +022  X > 272 ?
        ble.w   .Lx_ok                          | +026
        move.w  #0x110, d0                      | +02a  clamp X = 272
.Lx_ok:
        move.w  d0, 0x8a(a6)                    | +02e  target X
        move.w  d1, 0x8c(a6)                    | +032  target Y
        rts                                     | +036

| ----------------------------------------------------------------------------
|  Squad_InitFormationSlot_041C52  @ $041C52  (68 B)
|
|    /* Inicializa el vuelo: slot aleatorio 0..3, target de formacion,
|       turn-rate aleatorio de la tabla $286154 (4 words) y heading
|       inicial = atan2(target - pos). Cae en SetTaskB_041c96, que
|       almacena el angulo d0 como byte en la entity (heading). */
|    void Squad_InitFormationSlot(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.Squad_InitFormationSlot_041C52, "ax", @progbits
        .global Squad_InitFormationSlot_041C52
Squad_InitFormationSlot_041C52:
        jsr     0x5e9b6.l                       | +000  RNG
        andi.b  #3, d0                          | +006  slot 0..3
        addi.b  #0, d0                          | +00a  (+base 0: artefacto del
                                                |        compilador, tabla base)
        move.b  d0, 0x89(a6)                    | +00e  estado = slot
        jsr     Squad_ComputeTargetPos_041C1A(pc) | +012
        jsr     0x5e9b6.l                       | +016  RNG
        lea     0x286154.l, a0                  | +01c  tabla de turn-rates
        andi.w  #3, d0                          | +022
        lsl.w   #1, d0                          | +026
        move.w  (a0,d0.w), 0x36(a6)             | +028  turn-rate aleatorio
        move.w  0x8a(a6), d0                    | +02e  dx = target - pos
        move.w  0x8c(a6), d1                    | +032
        sub.w   0x22(a6), d0                    | +036
        sub.w   0x24(a6), d1                    | +03a
        jsr     0x5e018.l                       | +03e  Angle_FromDelta -> d0
        | --- cae en SetTaskB_041c96 (isla): guarda heading y rts ------------

| ----------------------------------------------------------------------------
|  Squad_PickSwoopState_041C9C  @ $041C9C  (62 B)
|
|    /* Decision de picado: fija estado 4, recalcula el target y elige el
|       proximo estado segun la posicion vertical del objetivo (a0, entity
|       devuelta por $5E1EA): 8/9 si target_y < obj->pos_y, A/B en caso
|       contrario (bit bajo aleatorio). Cae en JsrPcThunk_041cda. */
|    void Squad_PickSwoopState(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.Squad_PickSwoopState_041C9C, "ax", @progbits
        .global Squad_PickSwoopState_041C9C
Squad_PickSwoopState_041C9C:
        jsr     0x5e1ea.l                       | +000  a0 = entity objetivo
        movem.l a0, -(sp)                       | +006  preserva a0
        move.b  #4, 0x89(a6)                    | +00a  estado = 4
        jsr     Squad_ComputeTargetPos_041C1A(pc) | +010
        movem.l (sp)+, a0                       | +014
        move.w  #8, d0                          | +018  candidato: estado 8
        move.w  0x8c(a6), d1                    | +01c  target Y
        cmp.w   0x24(a0), d1                    | +020  vs pos Y del objetivo
        blt.w   .Lstate_picked                  | +024
        move.w  #0xA, d0                        | +028  estado A (por debajo)
.Lstate_picked:
        move.b  d0, 0x89(a6)                    | +02c
        jsr     0x5e9b6.l                       | +030  RNG
        andi.b  #1, d0                          | +036
        add.b   d0, 0x89(a6)                    | +03a  variante aleatoria +0/+1
        | --- cae en JsrPcThunk_041cda (isla) --------------------------------

| ----------------------------------------------------------------------------
|  Squad_SteerTowardTarget_041CE0  @ $041CE0  (120 B)
|
|    /* Guiado hacia el target: distancia Manhattan |dx|+|dy| contra el
|       umbral d5 (entra del caller). Si dist < d5 -> Squad_HaltVelocity
|       (llegada, X/N set via SetXN_041d66). Si no, calcula el angulo
|       deseado con Angle_FromDelta; si |dy| <= 0x20 lo adopta como
|       heading directamente; si difiere del cacheado gira +-2*turn_rate
|       hacia el, y refresca la velocidad con Squad_SinCosVelocity (en
|       squad_spawn_states, jsr pc-rel cruzando archivos). Cae en
|       ClearXN_041d58 = "aun no llegado". */
|    bool Squad_SteerTowardTarget(Entity *e /*a6*/, u16 arrive_dist /*d5*/);
| ----------------------------------------------------------------------------
        .section .text.Squad_SteerTowardTarget_041CE0, "ax", @progbits
        .global Squad_SteerTowardTarget_041CE0
Squad_SteerTowardTarget_041CE0:
        move.w  0x8a(a6), d0                    | +000  dx = target - pos
        move.w  0x8c(a6), d1                    | +004
        sub.w   0x22(a6), d0                    | +008
        sub.w   0x24(a6), d1                    | +00c
        move.w  d0, d2                          | +010  d2 = |dx|
        cmpi.w  #0, d0                          | +012
        bge.w   .Ldx_pos                        | +016
        neg.w   d2                              | +01a
.Ldx_pos:
        add.w   d1, d2                          | +01c  d2 += dy
        cmpi.w  #0, d1                          | +01e
        bge.w   .Ldy_pos                        | +022
        sub.w   d1, d2                          | +026  dy<0: d2 -= 2*dy
        sub.w   d1, d2                          | +028  (= |dx| + |dy|)
.Ldy_pos:
        cmp.w   d5, d2                          | +02a  dist < umbral ?
        bcs.w   Squad_HaltVelocity_041D5E       | +02c  si: llegado (SetXN)
        movem.w d1, -(sp)                       | +030  preserva dy
        jsr     0x5e018.l                       | +034  Angle_FromDelta -> d0
        movem.w (sp)+, d1                       | +03a
        cmpi.w  #0x20, d1                       | +03e  |dy| pequeno ?
        bgt.w   .Lturn                          | +042
        move.b  d0, 0x76(a6)                    | +046  adopta heading directo
.Lturn:
        cmp.b   0x76(a6), d0                    | +04a  ya apuntando ?
        beq.w   .Lrefresh                       | +04e
        move.w  0x36(a6), d1                    | +052  giro = 2*turn_rate
        lsl.w   #1, d1                          | +056
        neg.w   d1                              | +058  sentido negativo...
        sub.b   0x76(a6), d0                    | +05a  (deseado - actual)
        cmpi.b  #0, d0                          | +05e
        blt.w   .Lturn_neg                      | +062
        neg.w   d1                              | +066  ...o positivo
.Lturn_neg:
        add.w   d1, 0x76(a6)                    | +068  gira el heading
        move.b  0x76(a6), d0                    | +06c
        andi.w  #0xFF, d0                       | +070  angulo 0..255
.Lrefresh:
        jsr     Squad_SinCosVelocity_0420A4(pc) | +074  vel = sincos(heading)
        | --- cae en ClearXN_041d58 (isla): return false ---------------------

| ----------------------------------------------------------------------------
|  Squad_HaltVelocity_041D5E  @ $041D5E  (8 B)
|
|    /* Detiene la entity: velocidad X/Y = 0. Cae en SetXN_041d66 =
|       return true ("llegado"). Alcanzada por bcs desde el guiado. */
|    bool Squad_HaltVelocity(Entity *e /*a6*/);   // siempre true
| ----------------------------------------------------------------------------
        .section .text.Squad_HaltVelocity_041D5E, "ax", @progbits
        .global Squad_HaltVelocity_041D5E
Squad_HaltVelocity_041D5E:
        clr.w   0x28(a6)                        | +000  vel X = 0
        clr.w   0x2a(a6)                        | +004  vel Y = 0
        | --- cae en SetXN_041d66 (isla): return true ------------------------

| ----------------------------------------------------------------------------
|  Squad_TurnRateStepClamp_041D6C  @ $041D6C  (44 B)
|
|    /* Ajusta el turn-rate/amplitud +0x36 en d1 con saturacion en d0:
|       ASSERT(d1 != 0) [trap#15 nop-patched, build de desarrollo];
|       +0x36 += d1; si d1 > 0 clampa por arriba (rama $041D9E), si
|       d1 < 0 por abajo. CCR X/N = "toco el limite" (SetXN_041d98 /
|       SetXN_041daa) o "aun no" (ClearXN_041db0). */
|    bool Squad_TurnRateStepClamp(Entity *e /*a6*/, s16 step /*d1*/,
|                                 s16 bound /*d0*/);
| ----------------------------------------------------------------------------
        .section .text.Squad_TurnRateStepClamp_041D6C, "ax", @progbits
        .global Squad_TurnRateStepClamp_041D6C
Squad_TurnRateStepClamp_041D6C:
        | ---- ASSERT(d1 != 0) ------------------------------------------------
        cmpi.w  #0, d1                          | +000
        bne.w   .Lassert_ok                     | +004
        nop                                     | +008
        nop                                     | +00a
        cmpi.w  #0, d1                          | +00c
        nop                                     | +010
        trap    #15                             | +012
.Lassert_ok:
        add.w   d1, 0x36(a6)                    | +014  amplitud += paso
        cmpi.w  #0, d1                          | +018
        bgt.w   Squad_TurnRateClampHi_041D9E    | +01c  paso positivo
        cmp.w   0x36(a6), d0                    | +020  limite inferior
        blt.w   ClearXN_041db0                  | +024  aun por encima: false
        move.w  d0, 0x36(a6)                    | +028  clamp al limite
        | --- cae en SetXN_041d98 (isla): return true ------------------------

| ----------------------------------------------------------------------------
|  Squad_TurnRateClampHi_041D9E  @ $041D9E  (12 B)
|
|    /* Rama de paso positivo: clamp por arriba contra d0. */
| ----------------------------------------------------------------------------
        .section .text.Squad_TurnRateClampHi_041D9E, "ax", @progbits
        .global Squad_TurnRateClampHi_041D9E
Squad_TurnRateClampHi_041D9E:
        cmp.w   0x36(a6), d0                    | +000  limite superior
        bgt.w   ClearXN_041db0                  | +004  aun por debajo: false
        move.w  d0, 0x36(a6)                    | +008  clamp al limite
        | --- cae en SetXN_041daa (isla): return true ------------------------

| ----------------------------------------------------------------------------
|  Squad_BobYFast_041DB6 / Squad_BobYWide_041DDC / Squad_BobYNarrow_041E02
|
|    /* Tres variantes del "bob" senoidal vertical (trio no factorizado,
|       mismo idioma que los pares clonados documentados en waves previas):
|       leen la tabla seno $2C072C con la fase +0x7A y difieren solo en
|       el incremento de fase y la escala:
|         Fast   @ $041DB6 (38 B): fase += 0x80, delta = sin >> 8
|         Wide   @ $041DDC (38 B): fase += 0x10, delta = sin >> 6
|         Narrow @ $041E02 (34 B): fase += 0x10, delta = sin >> 7
|       Las tres confluyen en Squad_BobYApply_041E24 (Narrow por caida,
|       las otras dos por bra.w). Preservan pos Y en la pila para poder
|       revertir si la prueba de limites ($28D70) devuelve carry. */
|    bool Squad_BobY*(Entity *e /*a6*/);   // CCR X/N: carry del test
| ----------------------------------------------------------------------------
        .section .text.Squad_BobYFast_041DB6, "ax", @progbits
        .global Squad_BobYFast_041DB6
Squad_BobYFast_041DB6:
        move.w  0x24(a6), d4                    | +000  salva pos Y
        move.w  d4, -(sp)                       | +004
        move.b  0x7a(a6), d0                    | +006  fase actual
        addi.b  #0x80, 0x7a(a6)                 | +00a  fase += 128 (rapido)
        andi.w  #0xFF, d0                       | +010
        add.w   d0, d0                          | +014  indice de word
        lea     0x2c072c.l, a0                  | +016  tabla seno
        move.w  (a0,d0.w), d0                   | +01c
        asr.w   #8, d0                          | +020  amplitud /256
        bra.w   Squad_BobYApply_041E24          | +022

        .section .text.Squad_BobYWide_041DDC, "ax", @progbits
        .global Squad_BobYWide_041DDC
Squad_BobYWide_041DDC:
        move.w  0x24(a6), d4                    | +000  salva pos Y
        move.w  d4, -(sp)                       | +004
        move.b  0x7a(a6), d0                    | +006
        addi.b  #0x10, 0x7a(a6)                 | +00a  fase += 16 (lento)
        andi.w  #0xFF, d0                       | +010
        add.w   d0, d0                          | +014
        lea     0x2c072c.l, a0                  | +016
        move.w  (a0,d0.w), d0                   | +01c
        asr.w   #6, d0                          | +020  amplitud /64 (amplio)
        bra.w   Squad_BobYApply_041E24          | +022

        .section .text.Squad_BobYNarrow_041E02, "ax", @progbits
        .global Squad_BobYNarrow_041E02
Squad_BobYNarrow_041E02:
        move.w  0x24(a6), d4                    | +000  salva pos Y
        move.w  d4, -(sp)                       | +004
        move.b  0x7a(a6), d0                    | +006
        addi.b  #0x10, 0x7a(a6)                 | +00a  fase += 16
        andi.w  #0xFF, d0                       | +010
        add.w   d0, d0                          | +014
        lea     0x2c072c.l, a0                  | +016
        move.w  (a0,d0.w), d0                   | +01c
        asr.w   #7, d0                          | +020  amplitud /128 (estrecho)
        | --- cae en Squad_BobYApply_041E24 ----------------------------------

| ----------------------------------------------------------------------------
|  Squad_BobYApply_041E24  @ $041E24  (24 B)  - cola comun de los tres bobs
| ----------------------------------------------------------------------------
        .section .text.Squad_BobYApply_041E24, "ax", @progbits
        .global Squad_BobYApply_041E24
Squad_BobYApply_041E24:
        move.w  d0, 0x8e(a6)                    | +000  guarda delta de bob
        add.w   d0, 0x24(a6)                    | +004  aplica a pos Y
        jsr     0x28d70.l                       | +008  test de limites (T)
        bcs.w   Squad_BobYRestore_041E42        | +00e  fuera: revierte
        move.w  (sp)+, d4                       | +012  dentro: descarta copia
        move.w  d4, 0x24(a6)                    | +014  (reescribe la misma Y*)
        | --- cae en ClearXN_041e3c (isla): return false ---------------------
        | (*) el compilador reescribe pos Y con la copia salvada en ambas
        |     ramas; en esta la Y "vieja" == la de antes del bob, es decir
        |     la rama sin-carry DESHACE el bob y la rama con-carry lo
        |     conserva: el delta +0x8E queda para que el handler lo integre.

| ----------------------------------------------------------------------------
|  Squad_BobYRestore_041E42  @ $041E42  (6 B)  - rama carry del test
| ----------------------------------------------------------------------------
        .section .text.Squad_BobYRestore_041E42, "ax", @progbits
        .global Squad_BobYRestore_041E42
Squad_BobYRestore_041E42:
        move.w  (sp)+, d4                       | +000  recupera pos Y salvada
        move.w  d4, 0x24(a6)                    | +002
        | --- cae en SetXN_041e48 (isla): return true ------------------------

| ----------------------------------------------------------------------------
|  Squad_StateDispatch_041E4E  @ $041E4E  (34 B)
|
|    /* Despachador de estados del escuadron: ASSERT(estado < 6) [trap#15
|       nop-patched] y despues instala en (a6) (= PC de la tarea, primer
|       long del TCB/entity) el handler del jump table $28633C[estado].
|       Mismo idioma "2cb00000" de instalacion directa que el scheduler. */
|    void Squad_StateDispatch(Entity *e /*a6*/, u16 state /*d0*/);
| ----------------------------------------------------------------------------
        .section .text.Squad_StateDispatch_041E4E, "ax", @progbits
        .global Squad_StateDispatch_041E4E
Squad_StateDispatch_041E4E:
        | ---- ASSERT(state < 6) ----------------------------------------------
        cmpi.w  #6, d0                          | +000
        bcs.w   .Lstate_ok                      | +004
        nop                                     | +008
        nop                                     | +00a
        cmpi.w  #6, d0                          | +00c
        nop                                     | +010
        trap    #15                             | +012
.Lstate_ok:
        lsl.w   #2, d0                          | +014  indice de long
        lea     0x28633c.l, a0                  | +016  jump table (6 entradas)
        move.l  (a0,d0.w), (a6)                 | +01c  instala handler
        rts                                     | +020

| ----------------------------------------------------------------------------
|  SquadAnim_State4Select_041E70  @ $041E70  (58 B)
|
|    /* Handler de transicion: fija estado 4, recalcula target y si el
|       target Y no supera la Y del objetivo (a0 de $5E1EA) selecciona el
|       template de animacion $2BC62E (o $2BC6B0 si +0x7C == $FF, la
|       variante espejada) y cae en JsrAbsThunk_041eaa que lo aplica.
|       Si target Y > obj Y continua en SquadAnim_State5Select_041EB2. */
|    void SquadAnim_State4Select(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.SquadAnim_State4Select_041E70, "ax", @progbits
        .global SquadAnim_State4Select_041E70
SquadAnim_State4Select_041E70:
        jsr     0x5e1ea.l                       | +000  a0 = entity objetivo
        movem.l a0, -(sp)                       | +006
        move.b  #4, 0x89(a6)                    | +00a  estado = 4
        jsr     Squad_ComputeTargetPos_041C1A(pc) | +010
        movem.l (sp)+, a0                       | +014
        move.w  0x8c(a6), d0                    | +018  target Y
        cmp.w   0x24(a0), d0                    | +01c  vs pos Y objetivo
        bgt.w   SquadAnim_State5Select_041EB2   | +020  por debajo: estado 5
        lea     0x2bc62e.l, a0                  | +024  template estado 4
        cmpi.b  #0xFF, 0x7c(a6)                 | +02a  variante espejada ?
        bne.w   JsrAbsThunk_041eaa              | +030
        lea     0x2bc6b0.l, a0                  | +034  template espejado
        | --- cae en JsrAbsThunk_041eaa (isla): aplica template y rts --------

| ----------------------------------------------------------------------------
|  SquadAnim_State5Select_041EB2  @ $041EB2  (52 B)  - gemela de la anterior
| ----------------------------------------------------------------------------
        .section .text.SquadAnim_State5Select_041EB2, "ax", @progbits
        .global SquadAnim_State5Select_041EB2
SquadAnim_State5Select_041EB2:
        movem.l a0, -(sp)                       | +000
        move.b  #5, 0x89(a6)                    | +004  estado = 5
        jsr     Squad_ComputeTargetPos_041C1A(pc) | +00a
        movem.l (sp)+, a0                       | +00e
        move.w  0x8c(a6), d0                    | +012  target Y
        cmp.w   0x24(a0), d0                    | +016
        bgt.w   SquadAnim_ApproachSelect_041EEE | +01a  aun por debajo
        lea     0x2bc732.l, a0                  | +01e  template estado 5
        cmpi.b  #0xFF, 0x7c(a6)                 | +024
        bne.w   JsrAbsThunk_041ee6              | +02a
        lea     0x2bc7b4.l, a0                  | +02e  template espejado
        | --- cae en JsrAbsThunk_041ee6 (isla) -------------------------------

| ----------------------------------------------------------------------------
|  SquadAnim_ApproachSelect_041EEE  @ $041EEE  (38 B)
|
|    /* Si pos X - 0x20 >= obj X elige template de aproximacion $2BC836
|       (o $2BC8B8 espejado); si no, continua en ArriveSelect. */
| ----------------------------------------------------------------------------
        .section .text.SquadAnim_ApproachSelect_041EEE, "ax", @progbits
        .global SquadAnim_ApproachSelect_041EEE
SquadAnim_ApproachSelect_041EEE:
        move.w  0x22(a6), d0                    | +000  pos X propia
        subi.w  #0x20, d0                       | +004  margen 32 px
        cmp.w   0x22(a0), d0                    | +008  vs pos X objetivo
        blt.w   SquadAnim_ArriveSelect_041F1C   | +00c  ya encima: "arrive"
        lea     0x2bc836.l, a0                  | +010  template aproximacion
        cmpi.b  #0xFF, 0x7c(a6)                 | +016
        bne.w   JsrAbsThunk_041f14              | +01c
        lea     0x2bc8b8.l, a0                  | +020  template espejado
        | --- cae en JsrAbsThunk_041f14 (isla) -------------------------------

| ----------------------------------------------------------------------------
|  SquadAnim_ArriveSelect_041F1C  @ $041F1C  (22 B)
| ----------------------------------------------------------------------------
        .section .text.SquadAnim_ArriveSelect_041F1C, "ax", @progbits
        .global SquadAnim_ArriveSelect_041F1C
SquadAnim_ArriveSelect_041F1C:
        lea     0x2bc93a.l, a0                  | +000  template llegada
        cmpi.b  #0xFF, 0x7c(a6)                 | +006
        bne.w   JsrAbsThunk_041f32              | +00c
        lea     0x2bc9bc.l, a0                  | +010  template espejado
        | --- cae en JsrAbsThunk_041f32 (isla) -------------------------------

| ----------------------------------------------------------------------------
|  Squad_TagSharedBit_041F3A  @ $041F3A  (14 B)
|
|    /* Marca el bit propio (+0x85, sembrado por Squad_SpawnEight) en el
|       byte +0x21 de la estructura compartida del escuadron (+0xC):
|       "este miembro ha llegado/muerto". El lider testea la mascara. */
|    void Squad_TagSharedBit(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.Squad_TagSharedBit_041F3A, "ax", @progbits
        .global Squad_TagSharedBit_041F3A
Squad_TagSharedBit_041F3A:
        movea.l 0xc(a6), a0                     | +000  estructura compartida
        move.b  0x85(a6), d0                    | +004  id de bit propio
        bset    d0, 0x21(a0)                    | +008  marca en la mascara
        rts                                     | +00c

| ----------------------------------------------------------------------------
|  Squad_PhaseStepToTarget_041F48  @ $041F48  (48 B)  - DOS ENTRADAS
|
|    /* Acerca el heading +0x76 al objetivo +0x78 con paso +-4 (entrada
|       $041F48) o +-1 (entrada $041F50, .global interna: mismo idioma de
|       multiple entry point ya catalogado). CCR: SetXN_041f7e = alineado,
|       ClearXN_041f78 = todavia girando. */
|    bool Squad_PhaseStepToTarget(Entity *e /*a6*/);   // paso 4
|    bool Squad_PhaseStepByOne(Entity *e /*a6*/);      // paso 1
| ----------------------------------------------------------------------------
        .section .text.Squad_PhaseStepToTarget_041F48, "ax", @progbits
        .global Squad_PhaseStepToTarget_041F48
        .global Squad_PhaseStepByOne_041F50
Squad_PhaseStepToTarget_041F48:
        move.w  #4, d1                          | +000  paso rapido
        bra.w   .Lstep                          | +004
Squad_PhaseStepByOne_041F50:
        move.w  #1, d1                          | +008  paso fino
.Lstep:
        move.w  0x76(a6), d0                    | +00c  heading actual
        cmp.w   0x78(a6), d0                    | +010  ya en el objetivo ?
        beq.w   SetXN_041f7e                    | +014  si: true
        blt.w   .Lfwd                           | +018
        neg.w   d1                              | +01c  girar hacia atras
.Lfwd:
        add.w   d1, d0                          | +01e
        andi.w  #0xFF, d0                       | +020  modulo 256
        move.w  d0, 0x76(a6)                    | +024
        cmp.w   0x78(a6), d0                    | +028  llego ahora ?
        beq.w   SetXN_041f7e                    | +02c  si: true
        | --- cae en ClearXN_041f78 (isla): return false ---------------------

| ----------------------------------------------------------------------------
|  Squad_ApplyLeaderDelta_041F84  @ $041F84  (48 B)
|
|    /* Integra el movimiento relativo al lider: $5E506 devuelve en a0 la
|       entity lider; suma los deltas por-frame del registro de miembro
|       (+0x84 -> Z/+0x38, +0x82 -> X, +0x83 + bob del lider (+0x8E de a0)
|       -> Y) y copia el flag de template +0x7C del lider. */
|    void Squad_ApplyLeaderDelta(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.Squad_ApplyLeaderDelta_041F84, "ax", @progbits
        .global Squad_ApplyLeaderDelta_041F84
Squad_ApplyLeaderDelta_041F84:
        jsr     0x5e506.l                       | +000  a0 = entity lider
        move.b  0x84(a6), d0                    | +006  delta Z (s8)
        ext.w   d0                              | +00a
        add.w   d0, 0x38(a6)                    | +00c
        move.b  0x82(a6), d0                    | +010  delta X (s8)
        ext.w   d0                              | +014
        add.w   d0, 0x22(a6)                    | +016
        move.b  0x83(a6), d0                    | +01a  delta Y (s8)
        ext.w   d0                              | +01e
        add.w   0x8e(a0), d0                    | +020  + bob del lider
        add.w   d0, 0x24(a6)                    | +024
        move.b  0x7c(a0), 0x7c(a6)              | +028  hereda flag template
        rts                                     | +02e
