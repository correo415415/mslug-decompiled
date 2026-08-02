| ============================================================================
|  Metal Slug 1 - asm/melee_guard_cluster_0422xx.s
|  ----------------------------------------------------------------------------
|  Wave VV (parte 1/3) - spawners de hijos del escuadron + guardia melee
|  Cluster $0422EA..$0428C6+$0428C6..$042904, 24 funciones, 1 516 B.
|
|  Continua el megabloque del escuadron ($040EF2..$0422EA de Waves TT/UU):
|  cuatro spawners cortos que crean hijos con los handlers de la Wave UU,
|  la familia "MeleeGuard" (enemigo que persigue y ataca cuerpo a cuerpo,
|  con doble variante A/B del handler) y el arranque del "Charger"
|  (tablas de animacion + tracker tri-estado).
|
|  Hallazgos forenses de esta wave:
|    - TRES restos muertos de plantilla `bra.w`: $0423BE y $0423E0 saltan
|      a Stub_000423EA (rts) pero NINGUN flujo los alcanza (el codigo
|      anterior siempre salta por encima); $042BCC idem en el archivo 2.
|    - NOP de alineacion en $0425DE entre las dos variantes A/B del
|      handler MeleeGuard (ambas terminan en `bra.w .Lconverge`).
|    - Nuevo patron: saltos a MITAD de isla C ya matcheada, directamente
|      al rts final (SetTaskWRts_04247a, SetHandlerRts_0424a8,
|      JsrAbsRts_04290a) - 3 simbolos nuevos en tools/symbols.py.
|    - `jsr X(pc)` (4EBA) vs `bsr.w` (6100) para llamadas a la MISMA
|      distancia: plantillas de compilacion distintas conviviendo.
|
|  Protocolo de campos del task (a6):
|    +0x22/+0x24 x/y   +0x28 dx   +0x38 sprite flags  +0x3a facing(bit0)
|    +0x70 template A  +0x74 template B / timer  +0x78 latch reaccion
|    +0x79 flag "usa tracker"  +0x7a fase engage  +0x7c timer cooldown
|    +0x7e timer de vida  +0x80 handler salida  +0x84 anim id
|    +0x85 variante A/B  +0x86 puntero target  +0x98..+0x9d stats spawn
|
|  Callees externos (sin simbolo, convencion hex):
|    $4AE   = spawn hijo (handler en a1, hijo en a0)
|    $5DD02 = enlaza hijo con a0        $2352/$236E = encolar sonido d0/d1
|    $2783A = fisica base               $27F08 = scan atacantes (cc)
|    $27F60 = test de lado              $28134 = prioridad de sprite
|    $28364 = test dano directo (cs)    $28292 = latch de reaccion
|    $28CD4 = aplicar template a0       $28D70 = avanzar anim (cc=no fin)
|    $49FD0 = probe de colision         $56B38 = localizar jugador
|    $5DD56 = test de script            $5E0D4 = adquirir target
|    $8F344 = chequeo dano/estado       $518   = despawn
|    $799DE = decode tabla 2D           $57044 = aplicar efecto a target
| ============================================================================

| ----------------------------------------------------------------------------
|  void SquadSpawn_DropPair_0422EA(Task *t /*a6*/)  @ $0422EA  (58 B)
|
|  Crea el PAR de hijos "drop": primero DropSpawnAtTop (cae desde arriba,
|  x -= (+0x76<<5)-0x40, es decir 32px por indice menos 64), despues
|  ZigzagFall 8px a la derecha del primero y le pasa el indice +0x76
|  en su +0x34 (el zigzag lo usa como fase).
| ----------------------------------------------------------------------------
        .section .text.SquadSpawn_DropPair_0422EA, "ax", @progbits
        .global SquadSpawn_DropPair_0422EA
SquadSpawn_DropPair_0422EA:
        lea     SquadChild_DropSpawnAtTop_0417AC(pc), a1 | +000
        jsr     0x4ae.l                         | +004  spawn hijo 1
        jsr     0x5dd02.l                       | +00a  enlazar con a0
        move.w  0x76(a6), d0                    | +010  indice de par
        lsl.w   #5, d0                          | +014  *32 px
        subi.w  #0x40, d0                       | +016  -64 px
        sub.w   d0, 0x22(a0)                    | +01a  x -= offset
        lea     SquadChild_ZigzagFall_041850(pc), a1 | +01e
        jsr     0x4ae.l                         | +022  spawn hijo 2
        jsr     0x5dd02.l                       | +028
        addq.w  #8, 0x22(a0)                    | +02e  x += 8 (junto al 1o)
        move.w  0x76(a6), 0x34(a0)              | +032  pasa indice como fase
        rts                                     | +038

        .size SquadSpawn_DropPair_0422EA, .-SquadSpawn_DropPair_0422EA

| ----------------------------------------------------------------------------
|  void SquadSpawn_FxChild_042324(Task *t /*a6*/)  @ $042324  (26 B)
|
|  Crea un hijo con handler EXTERNO $78042 (efecto visual generico) y
|  fuerza sus sprite-flags a $E000 (prioridad maxima, sin colision).
| ----------------------------------------------------------------------------
        .section .text.SquadSpawn_FxChild_042324, "ax", @progbits
        .global SquadSpawn_FxChild_042324
SquadSpawn_FxChild_042324:
        lea     0x78042.l, a1                   | +000  handler externo (abs.l!)
        jsr     0x4ae.l                         | +006
        jsr     0x5dd02.l                       | +00c
        move.w  #0xe000, 0x38(a0)               | +012  flags = prio max
        rts                                     | +018

        .size SquadSpawn_FxChild_042324, .-SquadSpawn_FxChild_042324

| ----------------------------------------------------------------------------
|  void SquadSpawn_GlideAttacker_04233E(Task *t /*a6*/)  @ $04233E  (40 B)
|
|  Suena $109B y crea el hijo GlideAttack desplazado (-40,+10) respecto
|  al padre. Nota: los inmediatos negativos se emiten como hex word
|  (#0xffd8 = -40), fiel a la plantilla original.
| ----------------------------------------------------------------------------
        .section .text.SquadSpawn_GlideAttacker_04233E, "ax", @progbits
        .global SquadSpawn_GlideAttacker_04233E
SquadSpawn_GlideAttacker_04233E:
        move.w  #0x109b, d0                     | +000  sfx id
        jsr     0x2352.l                        | +004  encolar sonido d0
        lea     SquadChild_GlideAttack_04191A(pc), a1 | +00a
        jsr     0x4ae.l                         | +00e
        jsr     0x5dd02.l                       | +014
        addi.w  #0xffd8, 0x24(a0)               | +01a  y -= 40
        addi.w  #0xa, 0x22(a0)                  | +020  x += 10
        rts                                     | +026

        .size SquadSpawn_GlideAttacker_04233E, .-SquadSpawn_GlideAttacker_04233E

| ----------------------------------------------------------------------------
|  void SquadSpawn_FinalPoseChild_042366(Task *t /*a6*/)  @ $042366  (42 B)
|
|  Crea el hijo FinalPose desplazado (-56,+20) y le COPIA la velocidad
|  (+0x28) y el parametro +0x2E del padre (hereda el movimiento).
| ----------------------------------------------------------------------------
        .section .text.SquadSpawn_FinalPoseChild_042366, "ax", @progbits
        .global SquadSpawn_FinalPoseChild_042366
SquadSpawn_FinalPoseChild_042366:
        lea     SquadChild_FinalPose_041BC6(pc), a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        addi.w  #0xffc8, 0x22(a0)               | +010  x -= 56
        addi.w  #0x14, 0x24(a0)                 | +016  y += 20
        move.w  0x28(a6), 0x28(a0)              | +01c  hereda dx
        move.w  0x2e(a6), 0x2e(a0)              | +022  hereda +0x2E
        rts                                     | +028

        .size SquadSpawn_FinalPoseChild_042366, .-SquadSpawn_FinalPoseChild_042366

| ----------------------------------------------------------------------------
|  ccr Entity_CmpDepthToParent_042390(Task *t /*a6*/)  @ $042390  (16 B)
|
|  Compara la "profundidad" +0x10 propia contra la del padre (+0x8 -> a1)
|  y devuelve el resultado via CCR usando las islas: si es menor salta a
|  SetXN_0423a6, si no CAE en ClearXN_0423a0 (isla C matcheada justo
|  despues). Clon identico en $0434B2 (archivo 3).
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpDepthToParent_042390, "ax", @progbits
        .global Entity_CmpDepthToParent_042390
Entity_CmpDepthToParent_042390:
        movea.l 0x8(a6), a1                     | +000  a1 = padre
        move.b  0x10(a6), d0                    | +004  profundidad propia
        cmp.b   0x10(a1), d0                    | +008
        bcs.w   SetXN_0423a6                    | +00c  menor -> X/N set
        | cae en ClearXN_0423a0 (isla C)

        .size Entity_CmpDepthToParent_042390, .-Entity_CmpDepthToParent_042390

| ----------------------------------------------------------------------------
|  ccr TargetFacingTest_0423AC(Task *t /*a6*/)  @ $0423AC  (12 B)
|
|  Localiza al jugador ($56B38 -> a0); si no hay, CAE en ClearC_0423b8
|  (C=0, "sin target"). Si hay, salta a la comparacion de facing en
|  TargetFacingTest_Cmp_0423C2 (entrada secundaria del bloque $0423BE).
|  Devuelve ademas a0 = jugador.
| ----------------------------------------------------------------------------
        .section .text.TargetFacingTest_0423AC, "ax", @progbits
        .global TargetFacingTest_0423AC
TargetFacingTest_0423AC:
        jsr     0x56b38.l                       | +000  a0 = jugador
        move.l  a0, d0                          | +006
        bne.w   TargetFacingTest_Cmp_0423C2     | +008  hay target -> comparar
        | cae en ClearC_0423b8 (isla C)

        .size TargetFacingTest_0423AC, .-TargetFacingTest_0423AC

| ----------------------------------------------------------------------------
|  TargetFacingTest_Cont_0423BE  @ $0423BE  (28 B)
|
|  ARRANCA con un `bra.w Stub_000423EA` MUERTO (resto de plantilla: ningun
|  flujo llega a $0423BE; TargetFacingTest salta directo a +4). La entrada
|  real es TargetFacingTest_Cmp_0423C2: dx = x_target - x_propia;
|  smi captura el signo, se XOR-ea con el facing (+0x3a bit0) y si NO
|  coinciden salta a ClearC_0423e4 (C=0 "de espaldas"); si coinciden CAE
|  en SetC_0423da (C=1 "encarado").
| ----------------------------------------------------------------------------
        .section .text.TargetFacingTest_Cont_0423BE, "ax", @progbits
        .global TargetFacingTest_Cont_0423BE
        .global TargetFacingTest_Cmp_0423C2
TargetFacingTest_Cont_0423BE:
        bra.w   Stub_000423EA                   | +000  MUERTO (plantilla)
TargetFacingTest_Cmp_0423C2:
        move.w  0x22(a0), d0                    | +004  x target
        sub.w   0x22(a6), d0                    | +008  dx
        smi.b   d0                              | +00c  d0 = signo(dx)
        move.b  0x3a(a6), d1                    | +00e  facing propio
        eor.b   d0, d1                          | +012
        btst    #0, d1                          | +014  coinciden?
        bne.w   ClearC_0423e4                   | +018  no -> C=0
        | cae en SetC_0423da (isla C)

        .size TargetFacingTest_Cont_0423BE, .-TargetFacingTest_Cont_0423BE

| ----------------------------------------------------------------------------
|  TargetFacingTest_Pad_0423E0  @ $0423E0  (4 B)
|
|  Segundo `bra.w Stub_000423EA` MUERTO, identico al de $0423BE: relleno
|  de la misma plantilla de compilacion entre las islas SetC/ClearC.
| ----------------------------------------------------------------------------
        .section .text.TargetFacingTest_Pad_0423E0, "ax", @progbits
        .global TargetFacingTest_Pad_0423E0
TargetFacingTest_Pad_0423E0:
        bra.w   Stub_000423EA                   | +000  MUERTO (plantilla)

        .size TargetFacingTest_Pad_0423E0, .-TargetFacingTest_Pad_0423E0

| ----------------------------------------------------------------------------
|  void MeleeGuard_TrackTarget_0423EC(Task *t /*a6*/)  @ $0423EC  (74 B)
|
|  Fisica base + tracker tri-estado del latch +0x78 (gating por +0x79):
|    scan $27F08: cc=0 -> latch = d3 (id atacante);
|                 cc=1 -> si d0 == latch lo refresca, si no latch = $FF.
|  Si el latch quedo a 0, prueba dano directo $28364 y captura el carry
|  (scs). Es la MISMA plantilla tri-estado que se repite (con offsets
|  distintos) en $0428C6, $042D90, $042DFA, $04345E y $043488.
| ----------------------------------------------------------------------------
        .section .text.MeleeGuard_TrackTarget_0423EC, "ax", @progbits
        .global MeleeGuard_TrackTarget_0423EC
MeleeGuard_TrackTarget_0423EC:
        jsr     0x2783a.l                       | +000  fisica base
        tst.b   0x79(a6)                        | +006  tracker activo?
        beq.w   .Ltt_rts                        | +00a  no -> rts
        jsr     0x27f08.l                       | +00e  scan atacantes
        bcc.w   .Ltt_cmp                        | +014
        move.b  d3, 0x78(a6)                    | +018  latch = atacante
        bra.w   .Ltt_check                      | +01c
.Ltt_cmp:
        cmp.b   0x78(a6), d0                    | +020  mismo atacante?
        bne.w   .Ltt_clear                      | +024
        move.b  d3, 0x78(a6)                    | +028  refresca
        bra.w   .Ltt_check                      | +02c
.Ltt_clear:
        move.b  #0xff, 0x78(a6)                 | +030  latch invalido
.Ltt_check:
        tst.b   0x78(a6)                        | +036
        bne.w   .Ltt_rts                        | +03a  latch ocupado -> fuera
        jsr     0x28364.l                       | +03e  test dano directo
        scs.b   0x78(a6)                        | +044  latch = carry
.Ltt_rts:
        rts                                     | +048

        .size MeleeGuard_TrackTarget_0423EC, .-MeleeGuard_TrackTarget_0423EC

| ----------------------------------------------------------------------------
|  void MeleeGuard_EngageAndTimers_042436(Task *t /*a6*/)  @ $042436  (64 B)
|
|  Fase 1 (+0x7a == 0): calcula el umbral de engage $140 - (+0x98<<3);
|  si x propia lo supera (bmi tras cmp) arma +0x7a = 1. Fase 2: decrementa
|  el cooldown +0x7c y el timer de vida +0x7e; ambos convergen saltando
|  al RTS DE MITAD DE ISLA SetTaskWRts_04247a, y el decremento de +0x7e
|  CAE en la isla SetTaskW_042476 (move.w d0,0x7e; rts) para escribirlo.
| ----------------------------------------------------------------------------
        .section .text.MeleeGuard_EngageAndTimers_042436, "ax", @progbits
        .global MeleeGuard_EngageAndTimers_042436
MeleeGuard_EngageAndTimers_042436:
        tst.b   0x7a(a6)                        | +000  ya en fase 2?
        bne.w   .Legt_timers                    | +004
        moveq   #0, d0                          | +008
        move.b  0x98(a6), d0                    | +00a  stat de rango
        lsl.w   #3, d0                          | +00e  *8
        neg.w   d0                              | +010
        addi.w  #0x140, d0                      | +012  umbral = $140 - stat*8
        cmp.w   0x22(a6), d0                    | +016
        bmi.w   .Legt_out                       | +01a  aun lejos -> rts isla
        move.b  #1, 0x7a(a6)                    | +01e  arma fase 2
.Legt_out:
        bra.w   SetTaskWRts_04247a              | +024  rts a mitad de isla!
.Legt_timers:
        move.w  0x7c(a6), d0                    | +028  cooldown
        beq.w   .Legt_life                      | +02c
        subq.w  #1, d0                          | +030
        move.w  d0, 0x7c(a6)                    | +032
.Legt_life:
        move.w  0x7e(a6), d0                    | +036  timer de vida
        beq.w   SetTaskWRts_04247a              | +03a  0 -> rts a mitad isla
        subq.w  #1, d0                          | +03e
        | cae en SetTaskW_042476 (isla: move.w d0,0x7e(a6); rts)

        .size MeleeGuard_EngageAndTimers_042436, .-MeleeGuard_EngageAndTimers_042436

| ----------------------------------------------------------------------------
|  void MeleeGuard_MeleeGate_04247C(Task *t /*a6*/)  @ $04247C  (38 B)
|
|  Puerta de entrada al ataque: tres salidas tempranas al RTS de mitad
|  de isla SetHandlerRts_0424a8 (cooldown activo / fuera de pantalla
|  [x-$30 >= $F0] / sin target). Si pasa, copia el stat +0x9a al timer
|  +0x7d y CAE en SetTaskHandler_0424a2 (instala ApproachState $42740).
| ----------------------------------------------------------------------------
        .section .text.MeleeGuard_MeleeGate_04247C, "ax", @progbits
        .global MeleeGuard_MeleeGate_04247C
MeleeGuard_MeleeGate_04247C:
        tst.w   0x7c(a6)                        | +000  cooldown?
        bne.w   SetHandlerRts_0424a8            | +004  si -> rts mitad isla
        move.w  0x22(a6), d0                    | +008
        subi.w  #0x30, d0                       | +00c
        cmpi.w  #0xf0, d0                       | +010  en pantalla?
        bcc.w   SetHandlerRts_0424a8            | +014  no -> rts mitad isla
        move.l  0x86(a6), d0                    | +018  hay target?
        beq.w   SetHandlerRts_0424a8            | +01c  no -> rts mitad isla
        move.b  0x9a(a6), 0x7d(a6)              | +020  arma timer ataque
        | cae en SetTaskHandler_0424a2 (instala ApproachState_042740)

        .size MeleeGuard_MeleeGate_04247C, .-MeleeGuard_MeleeGate_04247C

| ----------------------------------------------------------------------------
|  void MeleeGuard_Think_0424AA(Task *t /*a6*/)  @ $0424AA  (104 B)
|
|  IA por frame de la variante A. Cada 32 frames (throttle con el
|  contador global $106F28 & $1F): re-adquiere target con
|  TargetFacingTest (jsr PC-REL 4EBA, no bsr!), gira si esta de espaldas
|  (bchg facing), guarda a0 en +0x86 y elige anim por distancia vertical
|  (dy < $40 -> anim 6 cercana, si no anim $10 lejana). Cada frame:
|  si MeleeRange da cc=0 instala RetreatState; si $8F344 da cc=0 mete
|  el handler EXTERNO $58FC2 (muerte/dano).
| ----------------------------------------------------------------------------
        .section .text.MeleeGuard_Think_0424AA, "ax", @progbits
        .global MeleeGuard_Think_0424AA
MeleeGuard_Think_0424AA:
        move.b  0x106f28.l, d0                  | +000  frame counter global
        andi.b  #0x1f, d0                       | +006  throttle 1/32
        bne.w   .Lth_every                      | +00a
        jsr     TargetFacingTest_0423AC(pc)     | +00e  4EBA! (no bsr)
        bcc.w   .Lth_store                      | +012  encarado -> sigue
        bchg    #0, 0x3a(a6)                    | +016  girar facing
.Lth_store:
        move.l  a0, 0x86(a6)                    | +01c  guarda target
        move.l  a0, d0                          | +020
        beq.w   .Lth_every                      | +022  sin target
        move.w  0x24(a6), d0                    | +026  dy = y propia
        sub.w   0x24(a0), d0                    | +02a       - y target
        cmpi.w  #0x40, d0                       | +02e
        bge.w   .Lth_far                        | +032
        move.b  #6, 0x84(a6)                    | +036  anim cercana
        bra.w   .Lth_every                      | +03c
.Lth_far:
        move.b  #0x10, 0x84(a6)                 | +040  anim lejana
.Lth_every:
        bsr.w   MeleeGuard_MeleeRange_042512    | +046  en rango melee?
        bcc.w   .Lth_dmg                        | +04a
        lea     MeleeGuard_RetreatState_042794(pc), a1 | +04e
        move.l  a1, (a6)                        | +052  -> retirada
.Lth_dmg:
        jsr     0x8f344.l                       | +054  chequeo dano/estado
        bcc.w   .Lth_rts                        | +05a
        lea     0x58fc2.l, a1                   | +05e  handler externo
        move.l  a1, (a6)                        | +064
.Lth_rts:
        rts                                     | +066

        .size MeleeGuard_Think_0424AA, .-MeleeGuard_Think_0424AA

| ----------------------------------------------------------------------------
|  ccr MeleeGuard_MeleeRange_042512(Task *t /*a6*/)  @ $042512  (40 B)
|
|  Adquiere target ($5E0D4 -> a0) y devuelve via carry si esta en la
|  caja de melee: |dy| < $40 y dx en [-$20,+$20) (el addi #$20 centra
|  el rango antes del cmpi #$40). cs = EN RANGO.
| ----------------------------------------------------------------------------
        .section .text.MeleeGuard_MeleeRange_042512, "ax", @progbits
        .global MeleeGuard_MeleeRange_042512
MeleeGuard_MeleeRange_042512:
        jsr     0x5e0d4.l                       | +000  a0 = target
        move.w  0x22(a0), d0                    | +006  dx
        move.w  0x24(a0), d1                    | +00a  dy
        sub.w   0x22(a6), d0                    | +00e
        sub.w   0x24(a6), d1                    | +012
        cmpi.w  #0x40, d1                       | +016
        bcc.w   .Lmr_rts                        | +01a  dy fuera -> cc=0
        addi.w  #0x20, d0                       | +01e  centra dx
        cmpi.w  #0x40, d0                       | +022  carry = en rango
.Lmr_rts:
        rts                                     | +026

        .size MeleeGuard_MeleeRange_042512, .-MeleeGuard_MeleeRange_042512

| ----------------------------------------------------------------------------
|  void MeleeGuard_FaceSelect_04253A(Task *t /*a6*/)  @ $04253A  (46 B)
|
|  IA de la variante B: segun el facing test elige el template de anim
|  (+0x74) y la anim id (+0x84): encarado -> $2B6644/anim 8; de espaldas
|  -> $2B6596/anim 7. Guarda el target en +0x86.
| ----------------------------------------------------------------------------
        .section .text.MeleeGuard_FaceSelect_04253A, "ax", @progbits
        .global MeleeGuard_FaceSelect_04253A
MeleeGuard_FaceSelect_04253A:
        jsr     TargetFacingTest_0423AC(pc)     | +000  4EBA
        bcs.w   .Lfs_facing                     | +004
        move.l  #0x2b6596, 0x74(a6)             | +008  template "espaldas"
        move.b  #7, 0x84(a6)                    | +010
        bra.w   .Lfs_store                      | +016
.Lfs_facing:
        move.l  #0x2b6644, 0x74(a6)             | +01a  template "encarado"
        move.b  #8, 0x84(a6)                    | +022
.Lfs_store:
        move.l  a0, 0x86(a6)                    | +028  guarda target
        rts                                     | +02c

        .size MeleeGuard_FaceSelect_04253A, .-MeleeGuard_FaceSelect_04253A

| ----------------------------------------------------------------------------
|  void MeleeGuard_TimerHandoff_042568(Task *t /*a6*/)  @ $042568  (14 B)
|
|  Cuando el timer de vida +0x7e llega a 0, instala el handler de salida
|  guardado en +0x80 (distinto por variante: $58FC2 en A, $58F82 en B).
| ----------------------------------------------------------------------------
        .section .text.MeleeGuard_TimerHandoff_042568, "ax", @progbits
        .global MeleeGuard_TimerHandoff_042568
MeleeGuard_TimerHandoff_042568:
        tst.w   0x7e(a6)                        | +000
        bne.w   .Lho_rts                        | +004
        move.l  0x80(a6), (a6)                  | +008  handler de salida
.Lho_rts:
        rts                                     | +00c

        .size MeleeGuard_TimerHandoff_042568, .-MeleeGuard_TimerHandoff_042568

| ----------------------------------------------------------------------------
|  void MeleeGuard_TailDespawn_042576(Task *t /*a6*/)  @ $042576  (24 B)
|
|  Cola comun de TODOS los estados MeleeGuard/Charger/Skirmisher: test
|  de script $5DD56 con la tabla $28838C; si devuelve carry salta al
|  despawn $518 (jmp, no jsr: no vuelve). Todos los `bra.w $42576`
|  del cluster convergen aqui.
| ----------------------------------------------------------------------------
        .section .text.MeleeGuard_TailDespawn_042576, "ax", @progbits
        .global MeleeGuard_TailDespawn_042576
MeleeGuard_TailDespawn_042576:
        lea     0x28838c.l, a0                  | +000  tabla de script
        jsr     0x5dd56.l                       | +006
        bcc.w   .Ltd_rts                        | +00c
        jmp     0x518.l                         | +010  despawn (no vuelve)
.Ltd_rts:
        rts                                     | +016

        .size MeleeGuard_TailDespawn_042576, .-MeleeGuard_TailDespawn_042576

| ----------------------------------------------------------------------------
|  void MeleeGuard_Handler_04258E(Task *t /*a6*/)  @ $04258E  (330 B)
|
|  Handler DOBLE del guardia melee. Dos entradas de setup:
|    A ($4258E): +0x85=0, template base $29B8EE, template alt $29BB4C,
|      tracker ON (+0x79=1), salida $58FC2.
|    B ($425B6 = MeleeGuard_HandlerB_0425B6, +0x28 bytes): +0x85=1,
|      base $288394, alt $2B6596, tracker OFF, salida $58F82.
|  Ambas terminan en `bra.w .Lconverge`; entre medias hay un NOP de
|  ALINEACION en $0425DE (la variante B mide 2 B menos que la A).
|
|  Convergencia: spawn de sombra ($776E2), sonido $E, test de lado
|  (scc -> +0x78), anim 6; cooldown +0x7c = tabla $2B74EC decodificada
|  * stat +0x99 >>8 (o $FFFF si stat=0, via moveq #-1); re-escala el
|  stat +0x9a con la tabla $2B746A (moveq #0,d1! - contrastar con el
|  clr.w d1 de Charger_Init); vida +0x7e = +0x9b<<6 (o $FFFF);
|  prioridad de sprite $8000 y flags 38 &= ~$1C | $18.
|
|  MeleeGuard_ReapplyBase_042682 (3a entrada global): reaplica el
|  template base +0x70 e instala el bucle .Lrun, que despacha por
|  variante: A -> Think + posible IdleState; B -> FaceSelect; ambos
|  pasan por MeleeGate + TimerHandoff + colision + TailDespawn.
| ----------------------------------------------------------------------------
        .section .text.MeleeGuard_Handler_04258E, "ax", @progbits
        .global MeleeGuard_Handler_04258E
        .global MeleeGuard_HandlerB_0425B6
        .global MeleeGuard_ReapplyBase_042682
MeleeGuard_Handler_04258E:
        move.b  #0, 0x85(a6)                    | +000  variante A (1D7C, no clr!)
        move.l  #0x29b8ee, 0x70(a6)             | +006  template base A
        move.l  #0x29bb4c, 0x74(a6)             | +00e  template alt A
        move.b  #1, 0x79(a6)                    | +016  tracker ON
        move.l  #0x58fc2, 0x80(a6)              | +01c  handler salida A
        bra.w   .Lmg_converge                   | +024
MeleeGuard_HandlerB_0425B6:
        move.b  #1, 0x85(a6)                    | +028  variante B
        move.l  #0x288394, 0x70(a6)             | +02e  template base B
        move.l  #0x2b6596, 0x74(a6)             | +036  template alt B
        move.b  #0, 0x79(a6)                    | +03e  tracker OFF
        move.l  #0x58f82, 0x80(a6)              | +044  handler salida B
        bra.w   .Lmg_converge                   | +04c
        nop                                     | +050  ALINEACION entre variantes
.Lmg_converge:
        lea     0x776e2.l, a1                   | +052  handler de sombra
        jsr     0x4ae.l                         | +058  spawn sombra
        move.w  #0xe, d1                        | +05e
        jsr     0x236e.l                        | +062  sonido $E
        jsr     0x27f60.l                       | +068  test de lado
        scc.b   0x78(a6)                        | +06e  latch inicial
        clr.b   0x7a(a6)                        | +072
        move.b  #6, 0x84(a6)                    | +076  anim inicial
        moveq   #0, d0                          | +07c
        move.b  0x99(a6), d0                    | +07e  stat cooldown
        bne.w   .Lmg_cool_calc                  | +082
        moveq   #-1, d0                         | +086  sin stat -> $FFFF
        bra.w   .Lmg_cool_set                   | +088
.Lmg_cool_calc:
        move.w  d0, -(a7)                       | +08c  salva stat
        lea     0x2b74ec.l, a0                  | +08e  tabla cooldown
        jsr     0x799de.l                       | +094  decode 2D
        mulu.w  (a7)+, d0                       | +09a  * stat
        lsr.l   #8, d0                          | +09c  >>8
.Lmg_cool_set:
        move.w  d0, 0x7c(a6)                    | +09e  cooldown
        lea     0x2b746a.l, a0                  | +0a2  tabla de escala
        jsr     0x799de.l                       | +0a8
        moveq   #0, d1                          | +0ae  (moveq, no clr.w!)
        move.b  0x9a(a6), d1                    | +0b0
        mulu.w  d1, d0                          | +0b4
        lsr.l   #8, d0                          | +0b6
        cmpi.w  #0x100, d0                      | +0b8
        bcs.w   .Lmg_scale_ok                   | +0bc
        move.b  #0xff, d0                       | +0c0  clamp a $FF
.Lmg_scale_ok:
        move.b  d0, 0x9a(a6)                    | +0c4  stat re-escalado
        moveq   #0, d0                          | +0c8
        move.b  0x9b(a6), d0                    | +0ca  stat vida
        bne.w   .Lmg_life_calc                  | +0ce
        moveq   #-1, d0                         | +0d2  sin stat -> $FFFF
        bra.w   .Lmg_life_set                   | +0d4
.Lmg_life_calc:
        lsl.w   #6, d0                          | +0d8  *64 frames
.Lmg_life_set:
        move.w  d0, 0x7e(a6)                    | +0da  timer de vida
        move.w  #0x8000, d0                     | +0de
        jsr     0x28134.l                       | +0e2  prioridad de sprite
        andi.w  #0xffe3, 0x38(a6)               | +0e8  flags &= ~$1C
        ori.w   #0x18, 0x38(a6)                 | +0ee  flags |= $18
MeleeGuard_ReapplyBase_042682:
        movea.l 0x70(a6), a0                    | +0f4  template base
        jsr     0x28cd4.l                       | +0f8  aplicar
        lea     .Lmg_run(pc), a1                | +0fe
        move.l  a1, (a6)                        | +102  instala bucle
.Lmg_run:
        bsr.w   MeleeGuard_EngageAndTimers_042436 | +104
        bsr.w   MeleeGuard_TrackTarget_0423EC   | +108
        jsr     0x28d70.l                       | +10c  avanzar anim
        move.b  0x85(a6), d0                    | +112  variante?
        cmpi.b  #0, d0                          | +116  (cmpi, no tst!)
        bne.w   .Lmg_run_b                      | +11a
        bsr.w   MeleeGuard_Think_0424AA         | +11e  IA variante A
        tst.b   0x78(a6)                        | +122
        bne.w   .Lmg_skip_idle                  | +126
        lea     MeleeGuard_IdleState_0426D8(pc), a1 | +12a
        move.l  a1, (a6)                        | +12e  -> idle
.Lmg_skip_idle:
        bra.w   .Lmg_gate                       | +130
.Lmg_run_b:
        bsr.w   MeleeGuard_FaceSelect_04253A    | +134  IA variante B
.Lmg_gate:
        bsr.w   MeleeGuard_MeleeGate_04247C     | +138
        bsr.w   MeleeGuard_TimerHandoff_042568  | +13c
        jsr     0x49fd0.l                       | +140  colision
        bra.w   MeleeGuard_TailDespawn_042576   | +146  cola comun

        .size MeleeGuard_Handler_04258E, .-MeleeGuard_Handler_04258E

| ----------------------------------------------------------------------------
|  void MeleeGuard_IdleState_0426D8(Task *t /*a6*/)  @ $0426D8  (56 B)
|
|  Estado de espera (template $29B956): avanza anim; cuando termina Y el
|  latch +0x78 esta activo, pasa a RecoverState. Colision + cola comun.
| ----------------------------------------------------------------------------
        .section .text.MeleeGuard_IdleState_0426D8, "ax", @progbits
        .global MeleeGuard_IdleState_0426D8
MeleeGuard_IdleState_0426D8:
        lea     0x29b956.l, a0                  | +000  template idle
        jsr     0x28cd4.l                       | +006
        lea     .Lid_run(pc), a1                | +00c
        move.l  a1, (a6)                        | +010
.Lid_run:
        bsr.w   MeleeGuard_TrackTarget_0423EC   | +012
        jsr     0x28d70.l                       | +016  anim
        bcc.w   .Lid_tail                       | +01c  aun no termina
        tst.b   0x78(a6)                        | +020  latch activo?
        beq.w   .Lid_tail                       | +024
        lea     MeleeGuard_RecoverState_042710(pc), a1 | +028
        move.l  a1, (a6)                        | +02c  -> recover
.Lid_tail:
        jsr     0x49fd0.l                       | +02e  colision
        bra.w   MeleeGuard_TailDespawn_042576   | +034

        .size MeleeGuard_IdleState_0426D8, .-MeleeGuard_IdleState_0426D8

| ----------------------------------------------------------------------------
|  void MeleeGuard_RecoverState_042710(Task *t /*a6*/)  @ $042710  (48 B)
|
|  Recuperacion (template $288414): al terminar la anim vuelve a
|  MeleeGuard_ReapplyBase_042682 (reaplica el template base).
| ----------------------------------------------------------------------------
        .section .text.MeleeGuard_RecoverState_042710, "ax", @progbits
        .global MeleeGuard_RecoverState_042710
MeleeGuard_RecoverState_042710:
        lea     0x288414.l, a0                  | +000  template recover
        jsr     0x28cd4.l                       | +006
        lea     .Lrc_run(pc), a1                | +00c
        move.l  a1, (a6)                        | +010
.Lrc_run:
        bsr.w   MeleeGuard_TrackTarget_0423EC   | +012
        jsr     0x28d70.l                       | +016
        bcc.w   .Lrc_tail                       | +01c
        lea     MeleeGuard_ReapplyBase_042682(pc), a1 | +020
        move.l  a1, (a6)                        | +024  reaplicar base
.Lrc_tail:
        jsr     0x49fd0.l                       | +026
        bra.w   MeleeGuard_TailDespawn_042576   | +02c

        .size MeleeGuard_RecoverState_042710, .-MeleeGuard_RecoverState_042710

| ----------------------------------------------------------------------------
|  void MeleeGuard_ApproachState_042740(Task *t /*a6*/)  @ $042740  (84 B)
|
|  Aproximacion al target (instalado por la isla SetTaskHandler_0424a2):
|  copia la anim elegida +0x84 al slot de sprite +0x5c y aplica el
|  template alterno +0x74. Bucle: timers + tracker + anim; al terminar
|  vuelve a ReapplyBase; en variante A ademas chequea MeleeRange y si
|  se salio del rango pasa a Retreat. OJO cola: `bsr.w TailDespawn` +
|  `rts` (no bra.w!) - plantilla distinta a los otros estados.
| ----------------------------------------------------------------------------
        .section .text.MeleeGuard_ApproachState_042740, "ax", @progbits
        .global MeleeGuard_ApproachState_042740
MeleeGuard_ApproachState_042740:
        move.b  0x84(a6), 0x5c(a6)              | +000  anim -> sprite slot
        movea.l 0x74(a6), a0                    | +006  template alterno
        jsr     0x28cd4.l                       | +00a
        lea     .Lap_run(pc), a1                | +010
        move.l  a1, (a6)                        | +014
.Lap_run:
        bsr.w   MeleeGuard_EngageAndTimers_042436 | +016
        bsr.w   MeleeGuard_TrackTarget_0423EC   | +01a
        jsr     0x28d70.l                       | +01e
        bcc.w   .Lap_var                        | +024
        lea     MeleeGuard_ReapplyBase_042682(pc), a1 | +028
        move.l  a1, (a6)                        | +02c  anim done -> base
.Lap_var:
        move.b  0x85(a6), d0                    | +02e
        cmpi.w  #0, d0                          | +032  (cmpi.W tras load .b!)
        bne.w   .Lap_tail                       | +036  variante B -> fuera
        bsr.w   MeleeGuard_MeleeRange_042512    | +03a
        bcc.w   .Lap_tail                       | +03e  fuera de rango
        lea     MeleeGuard_RetreatState_042794(pc), a1 | +042
        move.l  a1, (a6)                        | +046  -> retirada
.Lap_tail:
        jsr     0x49fd0.l                       | +048  colision
        bsr.w   MeleeGuard_TailDespawn_042576   | +04e  BSR (no bra)!
        rts                                     | +052

        .size MeleeGuard_ApproachState_042740, .-MeleeGuard_ApproachState_042740

| ----------------------------------------------------------------------------
|  void MeleeGuard_RetreatState_042794(Task *t /*a6*/)  @ $042794  (54 B)
|
|  Retirada (template $2883F4): mientras MeleeRange de cs (aun en rango)
|  sigue; cuando sale del rango vuelve a ReapplyBase. Misma cola
|  `bsr.w TailDespawn + rts` que ApproachState.
| ----------------------------------------------------------------------------
        .section .text.MeleeGuard_RetreatState_042794, "ax", @progbits
        .global MeleeGuard_RetreatState_042794
MeleeGuard_RetreatState_042794:
        lea     0x2883f4.l, a0                  | +000  template retreat
        jsr     0x28cd4.l                       | +006
        lea     .Lrt_run(pc), a1                | +00c
        move.l  a1, (a6)                        | +010
.Lrt_run:
        bsr.w   MeleeGuard_TrackTarget_0423EC   | +012
        jsr     0x28d70.l                       | +016
        bsr.w   MeleeGuard_MeleeRange_042512    | +01c
        bcs.w   .Lrt_tail                       | +020  aun en rango
        lea     MeleeGuard_ReapplyBase_042682(pc), a1 | +024
        move.l  a1, (a6)                        | +028  -> base
.Lrt_tail:
        jsr     0x49fd0.l                       | +02a
        bsr.w   MeleeGuard_TailDespawn_042576   | +030  BSR (no bra)!
        rts                                     | +034

        .size MeleeGuard_RetreatState_042794, .-MeleeGuard_RetreatState_042794

| ----------------------------------------------------------------------------
|  void MeleeGuard_DeathToExtern_0427CA(Task *t /*a6*/)  @ $0427CA  (48 B)
|
|  Transicion de muerte: sonido $E, prioridad $8000, flags 38, fisica,
|  e instala el handler EXTERNO $58F1E. rts normal (vuelve al scheduler
|  una ultima vez antes de que el handler externo tome el control).
| ----------------------------------------------------------------------------
        .section .text.MeleeGuard_DeathToExtern_0427CA, "ax", @progbits
        .global MeleeGuard_DeathToExtern_0427CA
MeleeGuard_DeathToExtern_0427CA:
        move.w  #0xe, d1                        | +000
        jsr     0x236e.l                        | +004  sonido
        move.w  #0x8000, d0                     | +00a
        jsr     0x28134.l                       | +00e  prioridad
        andi.w  #0xffe3, 0x38(a6)               | +014
        ori.w   #0x18, 0x38(a6)                 | +01a
        jsr     0x2783a.l                       | +020  fisica
        lea     0x58f1e.l, a1                   | +026  handler externo
        move.l  a1, (a6)                        | +02c
        rts                                     | +02e

        .size MeleeGuard_DeathToExtern_0427CA, .-MeleeGuard_DeathToExtern_0427CA

| ----------------------------------------------------------------------------
|  Charger_WalkAnimTables_0427FA  @ $0427FA  (204 B, DATOS)
|
|  DOS tablas de animacion consecutivas para el interprete $28CD4/$28D70:
|    A ($427FA, "walk"): header $0900 + metadata $0029B4A4; 12 frames
|      {dur=4, attr=$0208, tile, $FFFF} con tiles $233694..$23367A;
|      terminador $0100 + puntero ABSOLUTO de loop a .Lwalk_frames
|      ($042800) - emitido como `.long .Lwalk_frames` para que el
|      reloc R_68K_32 resuelva byte-exacto.
|    B ($4287E = Charger_RunAnimTable_04287E, "run"): header $0900 +
|      metadata $0029B5A0; 6 frames {dur=2} con tiles $232AF2..$232B00
|      (ciclo palindromo 0-1-2-3-2-1); loop a .Lrun_frames ($042884).
| ----------------------------------------------------------------------------
        .section .text.Charger_WalkAnimTables_0427FA, "ax", @progbits
        .global Charger_WalkAnimTables_0427FA
        .global Charger_RunAnimTable_04287E
Charger_WalkAnimTables_0427FA:
        .short  0x0900                          | +000  header
        .long   0x0029b4a4                      | +002  sprite metadata
.Lwalk_frames:
        .short  0x0004, 0x0208                  | +006  frame 0: dur 4
        .long   0x00233694
        .short  0xffff
        .short  0x0004, 0x0208                  | +010  frame 1
        .long   0x002336ae
        .short  0xffff
        .short  0x0004, 0x0208                  | +01a  frame 2
        .long   0x002336c2
        .short  0xffff
        .short  0x0004, 0x0208                  | +024  frame 3
        .long   0x002336d6
        .short  0xffff
        .short  0x0004, 0x0208                  | +02e  frame 4
        .long   0x002336ea
        .short  0xffff
        .short  0x0004, 0x0208                  | +038  frame 5
        .long   0x002336fe
        .short  0xffff
        .short  0x0004, 0x0208                  | +042  frame 6
        .long   0x00233712
        .short  0xffff
        .short  0x0004, 0x0208                  | +04c  frame 7
        .long   0x00233726
        .short  0xffff
        .short  0x0004, 0x0208                  | +056  frame 8
        .long   0x00233638
        .short  0xffff
        .short  0x0004, 0x0208                  | +060  frame 9
        .long   0x0023364c
        .short  0xffff
        .short  0x0004, 0x0208                  | +06a  frame 10
        .long   0x00233660
        .short  0xffff
        .short  0x0004, 0x0208                  | +074  frame 11
        .long   0x0023367a
        .short  0xffff
        .short  0x0100                          | +07e  terminador
        .long   .Lwalk_frames                   | +080  loop ptr ABSOLUTO
Charger_RunAnimTable_04287E:
        .short  0x0900                          | +084  header tabla B
        .long   0x0029b5a0                      | +086  sprite metadata
.Lrun_frames:
        .short  0x0002, 0x0208                  | +08a  frame 0: dur 2
        .long   0x00232af2
        .short  0xffff
        .short  0x0002, 0x0208                  | +094  frame 1
        .long   0x00232b00
        .short  0xffff
        .short  0x0002, 0x0208                  | +09e  frame 2
        .long   0x00232b0e
        .short  0xffff
        .short  0x0002, 0x0208                  | +0a8  frame 3
        .long   0x00232b1c
        .short  0xffff
        .short  0x0002, 0x0208                  | +0b2  frame 4 (=2, palindromo)
        .long   0x00232b0e
        .short  0xffff
        .short  0x0002, 0x0208                  | +0bc  frame 5 (=1)
        .long   0x00232b00
        .short  0xffff
        .short  0x0100                          | +0c6  terminador
        .long   .Lrun_frames                    | +0c8  loop ptr ABSOLUTO

        .size Charger_WalkAnimTables_0427FA, .-Charger_WalkAnimTables_0427FA

| ----------------------------------------------------------------------------
|  void Charger_TrackTarget_0428C6(Task *t /*a6*/)  @ $0428C6  (62 B)
|
|  Clon del tracker tri-estado sobre el latch +0x75 (sin gate +0x79).
|  Diferencias con MeleeGuard_TrackTarget: si el latch queda ocupado
|  salta a la isla JsrAbsThunk_042904 (jsr $28292 = latch de reaccion,
|  y rts); si prueba dano directo, sale por el RTS DE MITAD DE ISLA
|  JsrAbsRts_04290a (el rts final del thunk).
| ----------------------------------------------------------------------------
        .section .text.Charger_TrackTarget_0428C6, "ax", @progbits
        .global Charger_TrackTarget_0428C6
Charger_TrackTarget_0428C6:
        jsr     0x27f08.l                       | +000  scan atacantes
        bcc.w   .Lct_cmp                        | +006
        move.b  d3, 0x75(a6)                    | +00a  latch = atacante
        bra.w   .Lct_check                      | +00e
.Lct_cmp:
        cmp.b   0x75(a6), d0                    | +012
        bne.w   .Lct_clear                      | +016
        move.b  d3, 0x75(a6)                    | +01a  refresca
        bra.w   .Lct_check                      | +01e
.Lct_clear:
        move.b  #0xff, 0x75(a6)                 | +022  latch invalido
.Lct_check:
        tst.b   0x75(a6)                        | +028
        bne.w   JsrAbsThunk_042904              | +02c  ocupado -> jsr $28292
        jsr     0x28364.l                       | +030  test dano directo
        scs.b   0x75(a6)                        | +036
        bra.w   JsrAbsRts_04290a                | +03a  rts a mitad de isla!

        .size Charger_TrackTarget_0428C6, .-Charger_TrackTarget_0428C6
