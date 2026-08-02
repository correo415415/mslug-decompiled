| ============================================================================
|  Metal Slug 1 - asm/charger_states_0429xx.s
|  ----------------------------------------------------------------------------
|  Wave VV (parte 2/3) - maquina de estados del "Charger" + "Skirmisher"
|  Cluster $04290C..$042E8A, 12 funciones, 1 374 B.
|
|  El Charger es el enemigo que corre hacia el jugador y ataca en melee
|  con 5 golpes distintos (dispatch por +0x9c). Su Init estrena un patron
|  NUEVO: dispatcher tejido a traves de CUATRO islas SetTaskHandler
|  consecutivas ($429FC/$42A0C/$42A1A/$42A22) - cada rama del if-else
|  CAE en la isla que instala su estado:
|      +0x72 == 0            -> isla $429FC -> IdleState_042ACC
|      +0x9d == 0            -> isla $42A0C -> WalkFast_042A6E
|      +0x9d == 1            -> isla $42A1A -> WalkB_042A56
|      +0x9d >= 2            -> isla $42A22 -> WalkA_042A44
|
|  Hallazgos forenses:
|    - El UNICO `bsr.b` de la wave (bytes 61 82) en $042AA6: obliga a
|      fusionar helper ColumnDeltaTest + los 3 walk-entries en UNA
|      seccion (el bsr.b corto no puede cruzar secciones re-linkeadas).
|    - `bra.w .Lat_anim` MUERTO en $042BCC (3er resto de plantilla).
|    - `moveq #0,d0` MUERTO en $042C86 (el valor se pisa sin leerse).
|    - Direccion de codigo como INMEDIATO: `move.l #.Lat_anim,0x78(a6)`
|      (reloc R_68K_32 a label local) y `move.l #$00059042,0x78(a6)`
|      (externa, hex) - dos plantillas del mismo idiom "continuacion
|      diferida en +0x78".
|    - Charger_Init usa `clr.w d1` donde MeleeGuard_Handler usa
|      `moveq #0,d1` para el MISMO calculo de escala (plantillas
|      gemelas, codegen distinto).
|
|  Campos (a6): +0x70 columna home / latch skirmisher  +0x72 radio(cols)
|    +0x74 timer ataque  +0x75 latch tracker  +0x76 flag "enfadado"
|    +0x78 continuacion diferida  +0x7c target ptr  +0x84... ver parte 1
|    +0x99 golpes restantes  +0x9a/+0x9b stats  +0x9c tipo de golpe
|    +0x9d variante de marcha
| ============================================================================

| ----------------------------------------------------------------------------
|  void Charger_Init_04290C(Task *t /*a6*/)  @ $04290C  (240 B)
|
|  Setup completo: lado (scc +0x75), limpia +0x7c/+0x76, sonido $E,
|  prioridad $8000, flags 38, sombra $776E2; columna home = x via
|  $440BC (+0x70) y radio = +0x98*8 (+0x72); DOS escalados de stats
|  con `clr.w d1` (timer +0x74 desde tabla $2B74EC * +0x9a, y re-escala
|  +0x9b con $2B746A), ambos con clamp a $FF; paso tri-estado SIN fase
|  $28364 (solo scan $27F08); facing por posicion (x < $A0 -> bset);
|  dispatch final: si +0x72 != 0 sigue en InitDispatch, si no CAE en
|  la isla SetTaskHandler_0429fc (instala IdleState_042ACC).
| ----------------------------------------------------------------------------
        .section .text.Charger_Init_04290C, "ax", @progbits
        .global Charger_Init_04290C
Charger_Init_04290C:
        jsr     0x27f60.l                       | +000  test de lado
        scc.b   0x75(a6)                        | +006  latch inicial
        clr.l   0x7c(a6)                        | +00a  sin target
        clr.b   0x76(a6)                        | +00e  no enfadado
        move.w  #0xe, d1                        | +012
        jsr     0x236e.l                        | +016  sonido
        move.w  #0x8000, d0                     | +01c
        jsr     0x28134.l                       | +020  prioridad
        andi.w  #0xffe3, 0x38(a6)               | +026
        ori.w   #0x18, 0x38(a6)                 | +02c
        lea     0x776e2.l, a1                   | +032  handler de sombra
        jsr     0x4ae.l                         | +038  spawn sombra
        move.w  0x22(a6), d0                    | +03e  x
        jsr     0x440bc.l                       | +042  x -> columna de mapa
        move.w  d0, 0x70(a6)                    | +048  columna home
        moveq   #0, d0                          | +04c
        move.b  0x98(a6), d0                    | +04e  stat de radio
        lsl.w   #3, d0                          | +052  *8
        move.w  d0, 0x72(a6)                    | +054  radio en columnas
        lea     0x2b74ec.l, a0                  | +058  tabla timer
        jsr     0x799de.l                       | +05e  decode 2D
        clr.w   d1                              | +064  (clr.w, no moveq!)
        move.b  0x9a(a6), d1                    | +066
        mulu.w  d0, d1                          | +06a
        lsr.l   #8, d1                          | +06c
        cmpi.w  #0x100, d1                      | +06e
        bcs.w   .Lci_t_ok                       | +072
        move.b  #0xff, d1                       | +076  clamp
.Lci_t_ok:
        move.b  d1, 0x74(a6)                    | +07a  timer de ataque
        lea     0x2b746a.l, a0                  | +07e  tabla de escala
        jsr     0x799de.l                       | +084
        clr.w   d1                              | +08a  (clr.w otra vez)
        move.b  0x9b(a6), d1                    | +08c
        mulu.w  d1, d0                          | +090
        lsr.l   #8, d0                          | +092
        cmpi.w  #0x100, d0                      | +094
        bcs.w   .Lci_s_ok                       | +098
        move.b  #0xff, d0                       | +09c  clamp
.Lci_s_ok:
        move.b  d0, 0x9b(a6)                    | +0a0  stat re-escalado
        jsr     0x27f08.l                       | +0a4  scan atacantes
        bcc.w   .Lci_cmp                        | +0aa
        move.b  d3, 0x75(a6)                    | +0ae  latch = atacante
        bra.w   .Lci_face                       | +0b2
.Lci_cmp:
        cmp.b   0x75(a6), d0                    | +0b6
        bne.w   .Lci_clear                      | +0ba
        move.b  d3, 0x75(a6)                    | +0be  refresca
        bra.w   .Lci_face                       | +0c2
.Lci_clear:
        move.b  #0xff, 0x75(a6)                 | +0c6  latch invalido
.Lci_face:
        move.w  0x22(a6), d1                    | +0cc  x
        cmpi.w  #0xa0, d1                       | +0d0  mitad de pantalla
        bge.w   .Lci_right                      | +0d4
        bset    #0, 0x3a(a6)                    | +0d8  mira derecha
        bra.w   .Lci_dispatch                   | +0de
.Lci_right:
        bclr    #0, 0x3a(a6)                    | +0e2  mira izquierda
.Lci_dispatch:
        tst.w   0x72(a6)                        | +0e8  radio 0?
        bne.w   Charger_InitDispatch_042A04     | +0ec  no -> elegir marcha
        | cae en SetTaskHandler_0429fc (instala IdleState_042ACC)

        .size Charger_Init_04290C, .-Charger_Init_04290C

| ----------------------------------------------------------------------------
|  Charger_InitDispatch_042A04  @ $042A04  (8 B)
|
|  Eslabon 2 del dispatcher entre islas: si +0x9d != 0 sigue al
|  siguiente eslabon; si es 0 CAE en SetTaskHandler_042a0c (WalkFast).
| ----------------------------------------------------------------------------
        .section .text.Charger_InitDispatch_042A04, "ax", @progbits
        .global Charger_InitDispatch_042A04
Charger_InitDispatch_042A04:
        move.b  0x9d(a6), d0                    | +000  variante de marcha
        bne.w   Charger_InitDispatch2_042A14    | +004
        | cae en SetTaskHandler_042a0c (instala WalkFast_042A6E)

        .size Charger_InitDispatch_042A04, .-Charger_InitDispatch_042A04

| ----------------------------------------------------------------------------
|  Charger_InitDispatch2_042A14  @ $042A14  (6 B)
|
|  Eslabon 3: si +0x9d != 1 salta a la isla SetTaskHandler_042a22
|  (WalkA); si == 1 CAE en SetTaskHandler_042a1a (WalkB).
| ----------------------------------------------------------------------------
        .section .text.Charger_InitDispatch2_042A14, "ax", @progbits
        .global Charger_InitDispatch2_042A14
Charger_InitDispatch2_042A14:
        subq.b  #1, d0                          | +000
        bne.w   SetTaskHandler_042a22           | +002  >=2 -> WalkA
        | cae en SetTaskHandler_042a1a (instala WalkB_042A56)

        .size Charger_InitDispatch2_042A14, .-Charger_InitDispatch2_042A14

| ----------------------------------------------------------------------------
|  ccr Charger_ColumnDeltaTest_042A2A / estados Walk  @ $042A2A  (162 B)
|
|  UNA SOLA seccion con 4 entradas (helper + 3 estados de marcha):
|  el `bsr.b` de $042AA6 (bytes 61 82, el UNICO de la wave) apunta
|  hacia atras al helper y no puede cruzar limites de seccion.
|
|    Charger_ColumnDeltaTest_042A2A: |columna(x) - home| vs radio,
|      resultado en carry (cs = dentro del radio).
|    Charger_WalkA_042A44: tabla propia $427FA (pc-rel!), dx = $100.
|    Charger_WalkB_042A56: +0x76 = $FF (enfadado), tabla run $4287E,
|      dx = $100.
|    Charger_WalkFast_042A6E: template $29B744, dx MEDIDO de la tabla
|      $2B75D0 via $799DE.
|  Convergen: signo de dx segun facing, instala bucle .Lwk_run:
|  tracker (jsr pc-rel!) + anim + bsr.b helper; si salio del radio
|  instala IdleState; si fue golpeado ($574E8 cs) instala HurtState;
|  colision + cola comun.
| ----------------------------------------------------------------------------
        .section .text.Charger_ColumnDeltaTest_042A2A, "ax", @progbits
        .global Charger_ColumnDeltaTest_042A2A
        .global Charger_WalkA_042A44
        .global Charger_WalkB_042A56
        .global Charger_WalkFast_042A6E
Charger_ColumnDeltaTest_042A2A:
        move.w  0x22(a6), d0                    | +000  x
        jsr     0x440bc.l                       | +004  -> columna
        sub.w   0x70(a6), d0                    | +00a  - home
        bpl.w   .Lcd_abs                        | +00e
        neg.w   d0                              | +012  valor absoluto
.Lcd_abs:
        cmp.w   0x72(a6), d0                    | +014  vs radio (carry)
        rts                                     | +018
Charger_WalkA_042A44:
        lea     Charger_WalkAnimTables_0427FA(pc), a0 | +01a  tabla propia!
        jsr     0x28cd4.l                       | +01e
        move.w  #0x100, d0                      | +024  dx fijo
        bra.w   .Lwk_sign                       | +028
Charger_WalkB_042A56:
        move.b  #0xff, 0x76(a6)                 | +02c  ENFADADO
        lea     Charger_RunAnimTable_04287E(pc), a0 | +032  tabla run
        jsr     0x28cd4.l                       | +036
        move.w  #0x100, d0                      | +03c  dx fijo
        bra.w   .Lwk_sign                       | +040
Charger_WalkFast_042A6E:
        lea     0x29b744.l, a0                  | +044  template externo
        jsr     0x28cd4.l                       | +04a
        lea     0x2b75d0.l, a0                  | +050  tabla de velocidad
        jsr     0x799de.l                       | +056  dx medido
.Lwk_sign:
        btst    #0, 0x3a(a6)                    | +05c  facing
        bne.w   .Lwk_store                      | +062
        neg.w   d0                              | +066  hacia la izquierda
.Lwk_store:
        move.w  d0, 0x28(a6)                    | +068  dx
        lea     .Lwk_run(pc), a1                | +06c
        move.l  a1, (a6)                        | +070  instala bucle
.Lwk_run:
        jsr     Charger_TrackTarget_0428C6(pc)  | +072  4EBA!
        jsr     0x28d70.l                       | +076  anim
        bsr.b   Charger_ColumnDeltaTest_042A2A  | +07c  UNICO bsr.b (61 82)
        bcs.w   .Lwk_hit                        | +07e  dentro del radio
        lea     Charger_IdleState_042ACC(pc), a1 | +082
        move.l  a1, (a6)                        | +086  -> idle
.Lwk_hit:
        jsr     0x574e8.l                       | +088  fue golpeado?
        bcc.w   .Lwk_tail                       | +08e
        lea     Charger_HurtState_042B5C(pc), a1 | +092
        move.l  a1, (a6)                        | +096  -> hurt
.Lwk_tail:
        jsr     0x49fd0.l                       | +098  colision
        bra.w   MeleeGuard_TailDespawn_042576   | +09e  cola comun

        .size Charger_ColumnDeltaTest_042A2A, .-Charger_ColumnDeltaTest_042A2A

| ----------------------------------------------------------------------------
|  void Charger_IdleState_042ACC(Task *t /*a6*/)  @ $042ACC  (144 B)
|
|  Guardia parado dentro de su radio. Template segun humor (+0x76:
|  normal $29B816 / enfadado $2B5C10), doble tick de anim $283CA,
|  dx = 0. Bucle: facing test (jsr pc-rel, gira si de espaldas, guarda
|  target en +0x7c); tracker (bsr.w!); anim; si el timer +0x74 llego a
|  0 Y hay target -> AttackState, si no decrementa; si no esta enfadado
|  y $574E8 da cs -> HurtState.
| ----------------------------------------------------------------------------
        .section .text.Charger_IdleState_042ACC, "ax", @progbits
        .global Charger_IdleState_042ACC
Charger_IdleState_042ACC:
        tst.b   0x76(a6)                        | +000  enfadado?
        bne.w   .Lcid_mad                       | +004
        lea     0x29b816.l, a0                  | +008  template normal
        jsr     0x28cd4.l                       | +00e
        bra.w   .Lcid_tick                      | +014
.Lcid_mad:
        lea     0x2b5c10.l, a0                  | +018  template enfadado
        jsr     0x28cd4.l                       | +01e
.Lcid_tick:
        jsr     0x283ca.l                       | +024  doble tick de anim
        clr.w   0x28(a6)                        | +02a  parado
        lea     .Lcid_run(pc), a1               | +02e
        move.l  a1, (a6)                        | +032
.Lcid_run:
        jsr     TargetFacingTest_0423AC(pc)     | +034  4EBA
        bcc.w   .Lcid_store                     | +038
        bchg    #0, 0x3a(a6)                    | +03c  girar
.Lcid_store:
        move.l  a0, 0x7c(a6)                    | +042  guarda target
        bsr.w   Charger_TrackTarget_0428C6      | +046  6100! (no jsr pc)
        jsr     0x28d70.l                       | +04a  anim
        tst.b   0x74(a6)                        | +050  timer de ataque
        bne.w   .Lcid_tick_down                 | +054
        move.l  0x7c(a6), d0                    | +058  hay target?
        beq.w   .Lcid_hit                       | +05c
        lea     Charger_AttackState_042BA6(pc), a1 | +060
        move.l  a1, (a6)                        | +064  -> ataque!
.Lcid_hit:
        bra.w   .Lcid_dmg                       | +066
.Lcid_tick_down:
        subq.b  #1, 0x74(a6)                    | +06a
.Lcid_dmg:
        tst.b   0x76(a6)                        | +06e  enfadado no siente
        bne.w   .Lcid_tail                      | +072
        jsr     0x574e8.l                       | +076  fue golpeado?
        bcc.w   .Lcid_tail                      | +07c
        lea     Charger_HurtState_042B5C(pc), a1 | +080
        move.l  a1, (a6)                        | +084  -> hurt
.Lcid_tail:
        jsr     0x49fd0.l                       | +086  colision
        bra.w   MeleeGuard_TailDespawn_042576   | +08c

        .size Charger_IdleState_042ACC, .-Charger_IdleState_042ACC

| ----------------------------------------------------------------------------
|  void Charger_HurtState_042B5C(Task *t /*a6*/)  @ $042B5C  (74 B)
|
|  Reaccion al golpe: para (dx=0), template $2B70D2, engancha el
|  handler externo $57494 en +0x4c (respuesta a hits durante el stun)
|  y DOBLE doble-tick $283CA. Al terminar la anim vuelve a IdleState.
| ----------------------------------------------------------------------------
        .section .text.Charger_HurtState_042B5C, "ax", @progbits
        .global Charger_HurtState_042B5C
Charger_HurtState_042B5C:
        clr.w   0x28(a6)                        | +000  parado
        lea     0x2b70d2.l, a0                  | +004  template hurt
        jsr     0x28cd4.l                       | +00a
        lea     0x57494.l, a0                   | +010  handler +0x4c
        move.l  a0, 0x4c(a6)                    | +016
        jsr     0x283ca.l                       | +01a  doble tick x2
        jsr     0x283ca.l                       | +020
        lea     .Lch_run(pc), a1                | +026
        move.l  a1, (a6)                        | +02a
.Lch_run:
        bsr.w   Charger_TrackTarget_0428C6      | +02c
        jsr     0x28d70.l                       | +030
        bcc.w   .Lch_tail                       | +036
        lea     Charger_IdleState_042ACC(pc), a1 | +03a
        move.l  a1, (a6)                        | +03e  -> idle
.Lch_tail:
        jsr     0x49fd0.l                       | +040
        bra.w   MeleeGuard_TailDespawn_042576   | +046

        .size Charger_HurtState_042B5C, .-Charger_HurtState_042B5C

| ----------------------------------------------------------------------------
|  void Charger_AttackState_042BA6(Task *t /*a6*/)  @ $042BA6  (292 B)
|
|  Estado de ataque con 5 golpes (+0x9c) y sistema de CONTINUACION
|  DIFERIDA: si el tipo de golpe requiere windup, guarda la direccion
|  de .Lat_anim en +0x78 (como INMEDIATO de 32 bits, reloc a label
|  local!) y desvia a WindupB (normal) o WindupA (enfadado); el windup
|  saltara a +0x78 al terminar. Sin golpes (+0x99=0) -> .Lat_exit:
|  enfadado difiere $00059042 via +0x78 + WindupA, normal salta
|  directo (`jmp $59042.l`).
|
|  RESTOS MUERTOS: `bra.w .Lat_anim` en +026 ($042BCC, jamas alcanzado:
|  ambas ramas anteriores saltan) y `moveq #0,d0` en +0e0 ($042C86,
|  el registro se pisa sin leerse).
|
|  Dispatch de golpe: 0 -> $29BA70 + anim slot 5; 1 -> ENFADA (+0x76=$FF)
|  + $29B9CE; 2 -> $29BBEC; 3 -> $29BC64; resto -> $29BB4C. Tras elegir:
|  golpea al target ($57044 con d0 = +0x7c), dx=0, bucle .Lat_run:
|  timer, tracker, anim; al terminar recarga timer con +0x9b, decrementa
|  +0x99: quedan golpes -> IdleState, si no -> .Lat_exit.
| ----------------------------------------------------------------------------
        .section .text.Charger_AttackState_042BA6, "ax", @progbits
        .global Charger_AttackState_042BA6
Charger_AttackState_042BA6:
        tst.b   0x99(a6)                        | +000  quedan golpes?
        beq.w   .Lat_exit                       | +004  no -> salida
        tst.b   0x76(a6)                        | +008  enfadado?
        bne.w   .Lat_mad                        | +00c
        cmpi.b  #4, 0x9c(a6)                    | +010  golpe con windup?
        bcs.w   .Lat_anim                       | +016  0..3 -> directo
        move.l  #.Lat_anim, 0x78(a6)            | +01a  continuacion (reloc!)
        bra.w   Charger_WindupB_042D02          | +022  windup normal
        bra.w   .Lat_anim                       | +026  MUERTO (plantilla)
.Lat_mad:
        cmpi.b  #4, 0x9c(a6)                    | +02a
        bcc.w   .Lat_anim                       | +030  4+ -> directo
        move.l  #.Lat_anim, 0x78(a6)            | +034  continuacion (reloc!)
        bra.w   Charger_WindupA_042CCA          | +03c  windup enfadado
.Lat_anim:
        move.b  0x9c(a6), d0                    | +040  tipo de golpe
        bne.w   .Lat_g1                         | +044
        lea     0x29ba70.l, a0                  | +048  golpe 0
        jsr     0x28cd4.l                       | +04e
        move.b  #5, 0x5c(a6)                    | +054  anim slot 5
        bra.w   .Lat_strike                     | +05a
.Lat_g1:
        subq.b  #1, d0                          | +05e
        bne.w   .Lat_g2                         | +060
        move.b  #0xff, 0x76(a6)                 | +064  golpe 1 ENFADA
        lea     0x29b9ce.l, a0                  | +06a
        jsr     0x28cd4.l                       | +070
        bra.w   .Lat_strike                     | +076
.Lat_g2:
        subq.b  #1, d0                          | +07a
        bne.w   .Lat_g3                         | +07c
        lea     0x29bbec.l, a0                  | +080  golpe 2
        jsr     0x28cd4.l                       | +086
        bra.w   .Lat_strike                     | +08c
.Lat_g3:
        subq.b  #1, d0                          | +090
        bne.w   .Lat_gdef                       | +092
        lea     0x29bc64.l, a0                  | +096  golpe 3
        jsr     0x28cd4.l                       | +09c
        bra.w   .Lat_strike                     | +0a2
.Lat_gdef:
        lea     0x29bb4c.l, a0                  | +0a6  golpe por defecto
        jsr     0x28cd4.l                       | +0ac
.Lat_strike:
        move.l  0x7c(a6), d0                    | +0b2  d0 = target
        jsr     0x57044.l                       | +0b6  aplicar efecto
        clr.w   0x28(a6)                        | +0bc  parado
        lea     .Lat_run(pc), a1                | +0c0
        move.l  a1, (a6)                        | +0c4
.Lat_run:
        tst.b   0x74(a6)                        | +0c6  timer
        beq.w   .Lat_track                      | +0ca
        subq.b  #1, 0x74(a6)                    | +0ce
.Lat_track:
        bsr.w   Charger_TrackTarget_0428C6      | +0d2
        jsr     0x28d70.l                       | +0d6  anim
        bcc.w   .Lat_tail                       | +0dc  aun no termina
        moveq   #0, d0                          | +0e0  MUERTO (se pisa)
        move.b  0x9b(a6), 0x74(a6)              | +0e2  recarga timer
        subq.b  #1, 0x99(a6)                    | +0e8  un golpe menos
        bne.w   .Lat_more                       | +0ec
        lea     .Lat_exit(pc), a1               | +0f0  sin golpes
        move.l  a1, (a6)                        | +0f4
        bra.w   .Lat_tail                       | +0f6
.Lat_more:
        lea     Charger_IdleState_042ACC(pc), a1 | +0fa
        move.l  a1, (a6)                        | +0fe  -> idle entre golpes
.Lat_tail:
        jsr     0x49fd0.l                       | +100  colision
        bra.w   MeleeGuard_TailDespawn_042576   | +106
.Lat_exit:
        tst.b   0x76(a6)                        | +10a  enfadado?
        beq.w   .Lat_direct                     | +10e
        move.l  #0x00059042, 0x78(a6)           | +112  difiere salida (hex!)
        bra.w   Charger_WindupA_042CCA          | +11a  via windup
.Lat_direct:
        jmp     0x59042.l                       | +11e  salida directa

        .size Charger_AttackState_042BA6, .-Charger_AttackState_042BA6

| ----------------------------------------------------------------------------
|  void Charger_WindupA_042CCA(Task *t /*a6*/)  @ $042CCA  (56 B)
|
|  Windup "enfadado" (template $29BD84): al terminar la anim salta a
|  la CONTINUACION DIFERIDA +0x78 (via d0) y CALMA al bicho
|  (clr.b +0x76). Tracker con `bsr.w` (contrastar con el 4EBA de B).
| ----------------------------------------------------------------------------
        .section .text.Charger_WindupA_042CCA, "ax", @progbits
        .global Charger_WindupA_042CCA
Charger_WindupA_042CCA:
        lea     0x29bd84.l, a0                  | +000  template windup A
        jsr     0x28cd4.l                       | +006
        clr.w   0x28(a6)                        | +00c  parado
        lea     .Lwa_run(pc), a1                | +010
        move.l  a1, (a6)                        | +014
.Lwa_run:
        bsr.w   Charger_TrackTarget_0428C6      | +016  6100
        jsr     0x28d70.l                       | +01a
        bcc.w   .Lwa_tail                       | +020
        move.l  0x78(a6), d0                    | +024  continuacion
        move.l  d0, (a6)                        | +028  instalar
        clr.b   0x76(a6)                        | +02a  CALMA
.Lwa_tail:
        jsr     0x49fd0.l                       | +02e
        bra.w   MeleeGuard_TailDespawn_042576   | +034

        .size Charger_WindupA_042CCA, .-Charger_WindupA_042CCA

| ----------------------------------------------------------------------------
|  void Charger_WindupB_042D02(Task *t /*a6*/)  @ $042D02  (58 B)
|
|  Windup "normal" (template $29B956): espejo de WindupA pero ENFADA
|  (+0x76 = $FF) al saltar a la continuacion, y su tracker usa
|  `jsr X(pc)` (4EBA) donde A usa `bsr.w` (6100) A LA MISMA DISTANCIA:
|  dos plantillas de codegen conviviendo (+2 B de diferencia total).
| ----------------------------------------------------------------------------
        .section .text.Charger_WindupB_042D02, "ax", @progbits
        .global Charger_WindupB_042D02
Charger_WindupB_042D02:
        lea     0x29b956.l, a0                  | +000  template windup B
        jsr     0x28cd4.l                       | +006
        clr.w   0x28(a6)                        | +00c  parado
        lea     .Lwb_run(pc), a1                | +010
        move.l  a1, (a6)                        | +014
.Lwb_run:
        jsr     Charger_TrackTarget_0428C6(pc)  | +016  4EBA!
        jsr     0x28d70.l                       | +01a
        bcc.w   .Lwb_tail                       | +020
        move.l  0x78(a6), d0                    | +024  continuacion
        move.l  d0, (a6)                        | +028
        move.b  #0xff, 0x76(a6)                 | +02a  ENFADA
.Lwb_tail:
        jsr     0x49fd0.l                       | +030
        bra.w   MeleeGuard_TailDespawn_042576   | +036

        .size Charger_WindupB_042D02, .-Charger_WindupB_042D02

| ----------------------------------------------------------------------------
|  void Skirmisher_Init_042D3C(Task *t /*a6*/)  @ $042D3C  (84 B)
|
|  Enemigo "escaramuzador": variante minimalista del Charger sin radio
|  ni golpes multiples. Init: fuerza facing derecha (bset), sonido $E,
|  test de lado (scc -> latch +0x70, NO +0x75!) e instala Run.
|  Skirmisher_Run_042D5C (entrada global, tambien back-ref de Hurt):
|  dx MEDIDO de la tabla $2B76B4, signo por facing, template $29B744
|  (el mismo que WalkFast), doble tick, e instala Main CAYENDO en el.
| ----------------------------------------------------------------------------
        .section .text.Skirmisher_Init_042D3C, "ax", @progbits
        .global Skirmisher_Init_042D3C
        .global Skirmisher_Run_042D5C
Skirmisher_Init_042D3C:
        bset    #0, 0x3a(a6)                    | +000  facing derecha
        move.w  #0xe, d1                        | +006
        jsr     0x236e.l                        | +00a  sonido
        jsr     0x27f60.l                       | +010  test de lado
        scc.b   0x70(a6)                        | +016  latch en +0x70!
        lea     Skirmisher_Run_042D5C(pc), a1   | +01a
        move.l  a1, (a6)                        | +01e
Skirmisher_Run_042D5C:
        lea     0x2b76b4.l, a0                  | +020  tabla de velocidad
        jsr     0x799de.l                       | +026  dx medido
        btst    #0, 0x3a(a6)                    | +02c  facing
        bne.w   .Lsk_store                      | +032
        neg.w   d0                              | +036
.Lsk_store:
        move.w  d0, 0x28(a6)                    | +038  dx
        lea     0x29b744.l, a0                  | +03c  template marcha
        jsr     0x28cd4.l                       | +042
        jsr     0x283ca.l                       | +048  doble tick
        lea     Skirmisher_Main_042D90(pc), a1  | +04e
        move.l  a1, (a6)                        | +052
        | cae en Skirmisher_Main_042D90

        .size Skirmisher_Init_042D3C, .-Skirmisher_Init_042D3C

| ----------------------------------------------------------------------------
|  void Skirmisher_Main_042D90(Task *t /*a6*/)  @ $042D90  (106 B)
|
|  Bucle principal: fisica + tracker tri-estado INLINE sobre +0x70
|  (no llama a helper: la rama "ocupado" hace jsr $28292 y la libre
|  $28364 + scs, con convergencia propia); si $574E8 da cs -> Hurt;
|  anim + colision + cola comun.
| ----------------------------------------------------------------------------
        .section .text.Skirmisher_Main_042D90, "ax", @progbits
        .global Skirmisher_Main_042D90
Skirmisher_Main_042D90:
        jsr     0x2783a.l                       | +000  fisica
        jsr     0x27f08.l                       | +006  scan atacantes
        bcc.w   .Lsm_cmp                        | +00c
        move.b  d3, 0x70(a6)                    | +010  latch = atacante
        bra.w   .Lsm_check                      | +014
.Lsm_cmp:
        cmp.b   0x70(a6), d0                    | +018
        bne.w   .Lsm_clear                      | +01c
        move.b  d3, 0x70(a6)                    | +020  refresca
        bra.w   .Lsm_check                      | +024
.Lsm_clear:
        move.b  #0xff, 0x70(a6)                 | +028  latch invalido
.Lsm_check:
        tst.b   0x70(a6)                        | +02e
        bne.w   .Lsm_latched                    | +032
        jsr     0x28364.l                       | +036  test dano directo
        scs.b   0x70(a6)                        | +03c
        bra.w   .Lsm_hit                        | +040
.Lsm_latched:
        jsr     0x28292.l                       | +044  latch de reaccion
.Lsm_hit:
        jsr     0x574e8.l                       | +04a  fue golpeado?
        bcc.w   .Lsm_anim                       | +050
        lea     Skirmisher_Hurt_042DFA(pc), a1  | +054
        move.l  a1, (a6)                        | +058  -> hurt
.Lsm_anim:
        jsr     0x28d70.l                       | +05a  anim
        jsr     0x49fd0.l                       | +060  colision
        bra.w   MeleeGuard_TailDespawn_042576   | +066

        .size Skirmisher_Main_042D90, .-Skirmisher_Main_042D90

| ----------------------------------------------------------------------------
|  void Skirmisher_Hurt_042DFA(Task *t /*a6*/)  @ $042DFA  (144 B)
|
|  Stun del escaramuzador: para, template $2B70D2, handler $57494 en
|  +0x4c, doble tick x2 (mismo prologo que Charger_HurtState). Bucle:
|  fisica + tracker tri-estado INLINE (+0x70) + anim; al terminar
|  vuelve a Skirmisher_Run_042D5C (back-ref pc-rel a la seccion Init).
| ----------------------------------------------------------------------------
        .section .text.Skirmisher_Hurt_042DFA, "ax", @progbits
        .global Skirmisher_Hurt_042DFA
Skirmisher_Hurt_042DFA:
        clr.w   0x28(a6)                        | +000  parado
        lea     0x2b70d2.l, a0                  | +004  template hurt
        jsr     0x28cd4.l                       | +00a
        lea     0x57494.l, a0                   | +010  handler +0x4c
        move.l  a0, 0x4c(a6)                    | +016
        jsr     0x283ca.l                       | +01a  doble tick x2
        jsr     0x283ca.l                       | +020
        lea     .Lsh_run(pc), a1                | +026
        move.l  a1, (a6)                        | +02a
.Lsh_run:
        jsr     0x2783a.l                       | +02c  fisica
        jsr     0x27f08.l                       | +032  scan atacantes
        bcc.w   .Lsh_cmp                        | +038
        move.b  d3, 0x70(a6)                    | +03c
        bra.w   .Lsh_check                      | +040
.Lsh_cmp:
        cmp.b   0x70(a6), d0                    | +044
        bne.w   .Lsh_clear                      | +048
        move.b  d3, 0x70(a6)                    | +04c
        bra.w   .Lsh_check                      | +050
.Lsh_clear:
        move.b  #0xff, 0x70(a6)                 | +054
.Lsh_check:
        tst.b   0x70(a6)                        | +05a
        bne.w   .Lsh_latched                    | +05e
        jsr     0x28364.l                       | +062
        scs.b   0x70(a6)                        | +068
        bra.w   .Lsh_anim                       | +06c
.Lsh_latched:
        jsr     0x28292.l                       | +070
.Lsh_anim:
        jsr     0x28d70.l                       | +076  anim
        bcc.w   .Lsh_tail                       | +07c
        lea     Skirmisher_Run_042D5C(pc), a1   | +080  back-ref!
        move.l  a1, (a6)                        | +084  -> run
.Lsh_tail:
        jsr     0x49fd0.l                       | +086
        bra.w   MeleeGuard_TailDespawn_042576   | +08c

        .size Skirmisher_Hurt_042DFA, .-Skirmisher_Hurt_042DFA
