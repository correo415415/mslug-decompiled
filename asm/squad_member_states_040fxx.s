| ============================================================================
|  Metal Slug 1 - asm/squad_member_states_040fxx.s
|  ----------------------------------------------------------------------------
|  Wave UU (parte 1/2) - maquina de estados de los MIEMBROS del escuadron
|  (los 8 entities creados por Squad_SpawnEight_041FB4 con handler $40F00).
|  Cluster $040F00..$041408, 13 funciones, 1 268 B.
|
|  Todos los handlers siguen el mismo patron threaded-scheduler del
|  proyecto: bloque de setup una sola vez + `lea cont(pc),a1; move.l
|  a1,(a6)` para autoinstalar la continuacion per-frame, y cola comun
|  que retorna al scheduler (las islas SetTaskHandler_XXXX intercaladas
|  instalan el handler de muerte $40EF2 = Jsr5B6ThenJmpScheduler).
|
|  Protocolo con el lider (estructura +0xC, ver Wave TT):
|    - Squad_PollSharedState_041FF6 / Squad_WriteBackState_042040 mueven
|      ordenes por el array +0x80; Squad_TagSharedBit_041F3A marca la
|      mascara de "llegada" (+0x21) una sola vez por estado usando el
|      latch local +0x86 (0 = aun no marcado, $FF = ya marcado).
|    - Squad_ApplyLeaderDelta_041F84 integra el movimiento relativo.
|
|  Callees externos (hipotesis, abs.l literales):
|    $236E  = encolar sonido (id en d1)     $138FE = init de subsistema (+0x1C)
|    $28CD4 = aplicar template de anim (a0) $28D70 = test de limites (carry)
|    $2870A = chequeo de dano recibido      $5E45A = chequeo fuera-de-mundo
|    $5E018 = atan2 de tabla                $5E0D4 = adquirir entity objetivo
|    $799DE = medir template (longitud->d0)
|
|  Tablas de datos:
|    $28617C = 8 words: id de sonido por bit-id de miembro
|    $28635C = 8 punteros a template por bit-id ($FFFFFFFF = ninguno)
|    $28637C = 16 punteros a template por orden compartida (+0x80 & 15)
|    $28618C / $2861A4 = 6 punteros a template de pose por heading/2
|    $286738/$286E24/$286F74/$286F7A/$2872A2 = templates de anim individuales
|    $2BC426/$2BC4A8, $2BC52A/$2BC5AC = pares normal/espejado (+0x7C=$FF)
|
|  IDIOMA "branch a mitad de isla": la cola comun sale por bcc al rts
|  INTERNO de SetTaskHandler_041320 (SetHandlerRts_041326) - mismo
|  patron que JsrAbsRts_09a0ba en Wave SS.
|
|  ENCODINGS verificados contra bytes de ROM: las tablas de punteros se
|  cargan con `movea.l #imm,a0` (207c) y NO con `lea abs.l` (41f9); los
|  templates individuales al reves. Se preserva tal cual.
| ============================================================================

| ----------------------------------------------------------------------------
|  SquadMember_Handler_040F00  @ $040F00  (130 B)  - handler inicial
|
|    /* Instalado por Squad_SpawnEight_041FB4. Setup una sola vez:
|       sonido por bit-id (tabla $28617C), init $138FE con +0x1C=$13,
|       estado compartido local = eco = $80 (bit 7: "sin orden"),
|       template opcional por bit-id ($28635C, $FFFFFFFF = saltar).
|       Continuacion per-frame (.Lrun_a): integrar delta del lider,
|       espejar bit 7 de +0x5A del lider en bit 0 propio, test de
|       limites, poll de ordenes, cola comun. */
|    void SquadMember_Handler(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.SquadMember_Handler_040F00, "ax", @progbits
        .global SquadMember_Handler_040F00
SquadMember_Handler_040F00:
        move.b  0x85(a6), d0                    | +000  bit-id del miembro
        andi.w  #7, d0                          | +004
        add.w   d0, d0                          | +008
        lea     0x28617c.l, a0                  | +00a  tabla de sonidos
        move.w  (a0,d0.w), d1                   | +010
        jsr     0x236e.l                        | +014  encola sonido d1
        move.w  #0x13, 0x1c(a6)                 | +01a
        jsr     0x138fe.l                       | +020  init subsistema
        move.b  #0x80, 0x80(a6)                 | +026  estado local = "sin orden"
        move.b  #0x80, 0x81(a6)                 | +02c  eco = idem
        move.b  0x85(a6), d0                    | +032  bit-id otra vez
        andi.w  #7, d0                          | +036
        movea.l #0x28635c, a0                   | +03a  tabla de templates
        lsl.w   #2, d0                          | +040
        movea.l (a0,d0.w), a0                   | +042
        cmpa.l  #-1, a0                         | +046  $FFFFFFFF = sin template
        beq.w   .Lno_tmpl_a                     | +04c
        jsr     0x28cd4.l                       | +050  aplica template
.Lno_tmpl_a:
        lea     .Lrun_a(pc), a1                 | +056  autoinstala continuacion
        move.l  a1, (a6)                        | +05a
.Lrun_a:                                        |       (= $040F5C, per-frame)
        jsr     Squad_ApplyLeaderDelta_041F84(pc) | +05c
        movea.l 0xc(a6), a0                     | +060  lider
        btst    #7, 0x5a(a0)                    | +064  lider en fase especial?
        beq.w   .Lno_mirror_a                   | +06a
        bset    #0, 0x5a(a6)                    | +06e  espeja en bit 0 propio
.Lno_mirror_a:
        jsr     0x28d70.l                       | +074  test de limites
        jsr     Squad_PollSharedState_041FF6(pc) | +07a
        bra.w   SquadMember_FrameTailFull_041316 | +07e  cola comun (sin dano)

| ----------------------------------------------------------------------------
|  SquadMember_OnStateChange_040F82  @ $040F82  (108 B)
|
|    /* Handler de transicion instalado por Squad_PollSharedState al
|       detectar una orden nueva: resetea el latch +0x86, aplica el
|       template de la orden ($28637C[orden & 15], $FFFFFFFF = saltar)
|       y autoinstala el bucle B. Bucle B (RunStateB, entrada global
|       secundaria, referenciada tambien desde AnimCycleIdle): delta
|       del lider, espejo del bit del lider, limites; si el test da
|       carry hace poll y marca la llegada UNA vez (latch +0x86) via
|       Squad_TagSharedBit. */
|    void SquadMember_OnStateChange(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.SquadMember_OnStateChange_040F82, "ax", @progbits
        .global SquadMember_OnStateChange_040F82
        .global SquadMember_RunStateB_040FB0
SquadMember_OnStateChange_040F82:
        clr.b   0x86(a6)                        | +000  latch de llegada = 0
        move.b  0x80(a6), d0                    | +004  orden actual
        andi.w  #0xF, d0                        | +008
        movea.l #0x28637c, a0                   | +00c  tabla por orden
        lsl.w   #2, d0                          | +012
        movea.l (a0,d0.w), a0                   | +014
        cmpa.l  #-1, a0                         | +018
        beq.w   .Lno_tmpl_b                     | +01e
        jsr     0x28cd4.l                       | +022  aplica template
.Lno_tmpl_b:
        lea     SquadMember_RunStateB_040FB0(pc), a1 | +028
        move.l  a1, (a6)                        | +02c
SquadMember_RunStateB_040FB0:                   |       (= $040FB0, per-frame)
        jsr     Squad_ApplyLeaderDelta_041F84(pc) | +02e
        movea.l 0xc(a6), a0                     | +032
        btst    #7, 0x5a(a0)                    | +036
        beq.w   .Lno_mirror_b                   | +03c
        bset    #0, 0x5a(a6)                    | +040
.Lno_mirror_b:
        jsr     0x28d70.l                       | +046  limites (carry = dentro)
        bcc.w   .Lstateb_tail                   | +04c
        jsr     Squad_PollSharedState_041FF6(pc) | +050
        cmpi.b  #0, 0x86(a6)                    | +054  ya marcado ?
        bne.w   .Lstateb_tail                   | +05a
        move.b  #0xFF, 0x86(a6)                 | +05e  marca latch
        jsr     Squad_TagSharedBit_041F3A(pc)   | +064  publica llegada
.Lstateb_tail:
        bra.w   SquadMember_FrameTailFull_041316 | +068  cola comun (sin dano)

| ----------------------------------------------------------------------------
|  SquadMember_AnimCycleIdle_040FEE  @ $040FEE  (204 B)  - la mas grande
|
|    /* Estado de "descanso en formacion": animacion ciclica temporizada.
|       Setup: mide DOS templates con $799DE - el par $2BC426/$2BC4A8
|       (normal/espejado segun +0x7C=$FF) da el sub-contador +0x7B/+0x7E,
|       y el par $2BC52A/$2BC5AC da el periodo +0x72 (copiado al contador
|       +0x70). Aplica el template base $286738. Bucle: si el lider tiene
|       el bit 0 de +0x13 encendido -> reinstala RunStateB directamente;
|       si el periodo expira, reaplica $286738 y decrementa el ciclo
|       exterior +0x7E; tras +0x76 > 10 ciclos pasa al template $286E24
|       (variante "cansado"). Cae SIEMPRE en RunStateB (bra). */
|    void SquadMember_AnimCycleIdle(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.SquadMember_AnimCycleIdle_040FEE, "ax", @progbits
        .global SquadMember_AnimCycleIdle_040FEE
SquadMember_AnimCycleIdle_040FEE:
        clr.w   0x76(a6)                        | +000  ciclos completados = 0
        clr.b   0x86(a6)                        | +004  latch = 0
        lea     0x2bc426.l, a0                  | +008  template A (normal)
        cmpi.b  #0xFF, 0x7c(a6)                 | +00e  espejado ?
        bne.w   .Lidle_meas1                    | +014
        lea     0x2bc4a8.l, a0                  | +018  template A (espejo)
.Lidle_meas1:
        jsr     0x799de.l                       | +01e  mide -> d0
        move.b  d0, 0x7b(a6)                    | +024  sub-contador base
        lea     0x2bc52a.l, a0                  | +028  template B (normal)
        cmpi.b  #0xFF, 0x7c(a6)                 | +02e
        bne.w   .Lidle_meas2                    | +034
        lea     0x2bc5ac.l, a0                  | +038  template B (espejo)
.Lidle_meas2:
        jsr     0x799de.l                       | +03e  mide -> d0
        move.w  d0, 0x72(a6)                    | +044  periodo
        move.w  0x72(a6), 0x70(a6)              | +048  contador = periodo
        move.b  0x7b(a6), 0x7e(a6)              | +04e  ciclo exterior
        lea     0x286738.l, a0                  | +054  template base
        jsr     0x28cd4.l                       | +05a
        lea     .Lidle_loop(pc), a1             | +060
        move.l  a1, (a6)                        | +064
.Lidle_loop:                                    |       (= $041054, per-frame)
        movea.l 0xc(a6), a0                     | +066  lider
        btst    #0, 0x13(a0)                    | +06a  lider abortando ?
        bne.w   .Lidle_to_b                     | +070  si: instala RunStateB
        subq.w  #1, 0x70(a6)                    | +074  contador de periodo
        cmpi.w  #0, 0x70(a6)                    | +078
        bgt.w   .Lidle_tail                     | +07e
        move.w  0x72(a6), 0x70(a6)              | +082  recarga periodo
        lea     0x286738.l, a0                  | +088  reaplica base
        jsr     0x28cd4.l                       | +08e
        subq.b  #1, 0x7e(a6)                    | +094  ciclo exterior--
        cmpi.b  #0, 0x7e(a6)                    | +098
        bgt.w   .Lidle_tail                     | +09e
        move.b  0x7b(a6), 0x7e(a6)              | +0a2  recarga ciclo
        addq.w  #1, 0x76(a6)                    | +0a8  ciclos completados++
        cmpi.w  #0xA, 0x76(a6)                  | +0ac
        ble.w   .Lidle_tail                     | +0b2
        lea     0x286e24.l, a0                  | +0b6  variante "cansado"
        jsr     0x28cd4.l                       | +0bc
.Lidle_to_b:
        lea     SquadMember_RunStateB_040FB0(pc), a1 | +0c2
        move.l  a1, (a6)                        | +0c6
.Lidle_tail:
        bra.w   SquadMember_RunStateB_040FB0    | +0c8  ejecuta bucle B ya

| ----------------------------------------------------------------------------
|  SquadMember_SetPose2_0410BA  @ $0410BA  (10 B)
|
|    /* Prologo minusculo: fija pose actual 2 y salta a AimTrackTarget
|       (que fija la pose objetivo 10). */
|    void SquadMember_SetPose2(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.SquadMember_SetPose2_0410BA, "ax", @progbits
        .global SquadMember_SetPose2_0410BA
SquadMember_SetPose2_0410BA:
        move.w  #2, 0x76(a6)                    | +000  pose actual = 2
        bra.w   SquadMember_AimTrackTarget_0410EE | +006

| ----------------------------------------------------------------------------
|  SquadMember_HoldPose_0410C4  @ $0410C4  (42 B)
|
|    /* Estado de espera tras publicar la orden $83 (ver AckAndHold /
|       PoseFromHeading): limpia bit 3 de +0x13, aplica template $286F74
|       y queda en un bucle minimo delta+limites+poll con chequeo de
|       dano (FrameTail con dano). */
|    void SquadMember_HoldPose(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.SquadMember_HoldPose_0410C4, "ax", @progbits
        .global SquadMember_HoldPose_0410C4
SquadMember_HoldPose_0410C4:
        bclr    #3, 0x13(a6)                    | +000
        lea     0x286f74.l, a0                  | +006  template de espera
        jsr     0x28cd4.l                       | +00c
        lea     .Lhold_loop(pc), a1             | +012
        move.l  a1, (a6)                        | +016
.Lhold_loop:                                    |       (= $0410DC, per-frame)
        jsr     Squad_ApplyLeaderDelta_041F84(pc) | +018
        jsr     0x28d70.l                       | +01c
        jsr     Squad_PollSharedState_041FF6(pc) | +022
        bra.w   SquadMember_FrameTail_0412FC    | +026  cola comun (con dano)

| ----------------------------------------------------------------------------
|  SquadMember_AimTrackTarget_0410EE  @ $0410EE  (168 B)
|
|    /* Estado de punteria: pose objetivo 10, adquiere el entity
|       objetivo con $5E0D4 y lo publica en +0x9C propio Y del lider.
|       Bucle (frames alternos via toggle +0x70 & 1): calcula atan2
|       ($5E018) del vector al objetivo (con sesgo +$20 en X), lo
|       cuantiza con Squad_DepthToScaleIdx y gira la pose actual +0x76
|       un paso hacia la objetivo (+-1 via d1 = -1 negada); cuando
|       coinciden pone el flag +0x7D del lider a 1 ("en punteria").
|       Cada frame limpia +0x7D a 0 antes (protocolo de watchdog). */
|    void SquadMember_AimTrackTarget(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.SquadMember_AimTrackTarget_0410EE, "ax", @progbits
        .global SquadMember_AimTrackTarget_0410EE
SquadMember_AimTrackTarget_0410EE:
        move.w  #0xA, 0x78(a6)                  | +000  pose objetivo = 10
        lea     0x286f74.l, a0                  | +006  template de espera
        jsr     0x28cd4.l                       | +00c
        jsr     0x5e0d4.l                       | +012  adquiere objetivo -> a0
        movea.l 0xc(a6), a1                     | +018  lider
        move.l  a0, 0x9c(a1)                    | +01c  publica en el lider
        move.l  a0, 0x9c(a6)                    | +020  y en si mismo
        bclr    #3, 0x13(a6)                    | +024
        lea     .Laim_loop(pc), a1              | +02a
        move.l  a1, (a6)                        | +02e
.Laim_loop:                                     |       (= $04111E, per-frame)
        jsr     Squad_ApplyLeaderDelta_041F84(pc) | +030
        movea.l 0xc(a6), a0                     | +034  lider
        move.b  #0, 0x7d(a0)                    | +038  watchdog: "no apuntado"
        addq.w  #1, 0x70(a6)                    | +03e  toggle de frame
        andi.w  #1, 0x70(a6)                    | +042
        bne.w   .Laim_tail                      | +048  solo frames pares
        movea.l 0x9c(a6), a0                    | +04c  objetivo
        move.w  0x22(a0), d0                    | +050  x objetivo
        move.w  0x24(a0), d1                    | +054  y objetivo
        sub.w   0x22(a6), d0                    | +058  dx
        sub.w   0x24(a6), d1                    | +05c  dy
        addi.w  #0x20, d0                       | +060  sesgo de cañon
        jsr     0x5e018.l                       | +064  atan2 -> d0
        jsr     Squad_DepthToScaleIdx_04206A(pc) | +06a  cuantiza
        move.w  d0, 0x78(a6)                    | +06e  pose objetivo
        move.w  #-1, d1                         | +072  paso = -1
        move.w  0x78(a6), d0                    | +076
        cmp.w   0x76(a6), d0                    | +07a  vs pose actual
        beq.w   .Laim_locked                    | +07e  ya alineado
        bcs.w   .Laim_step                      | +082  objetivo < actual: -1
        neg.w   d1                              | +086  si no: +1
.Laim_step:
        add.w   d1, 0x76(a6)                    | +088  gira un paso
        bra.w   .Laim_tail                      | +08c
.Laim_locked:
        movea.l 0xc(a6), a0                     | +090  lider
        move.b  #1, 0x7d(a0)                    | +094  "en punteria"
.Laim_tail:
        jsr     0x28d70.l                       | +09a  limites
        jsr     Squad_PollSharedState_041FF6(pc) | +0a0
        bra.w   SquadMember_FrameTail_0412FC    | +0a4  cola comun (con dano)

| ----------------------------------------------------------------------------
|  SquadMember_AlignHeading_041196  @ $041196  (98 B)
|
|    /* Girar hacia la pose objetivo 2 a media velocidad: en frames
|       alternos llama Squad_PhaseStepByOne (avanza +0x76 un paso hacia
|       +0x78 y devuelve carry al llegar); al llegar marca la llegada
|       UNA vez (latch +0x86 + Squad_TagSharedBit). Template $286F74.
|       Gemelo de AlignHeadingB_041328 (pose 8, template $286F7A). */
|    void SquadMember_AlignHeading(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.SquadMember_AlignHeading_041196, "ax", @progbits
        .global SquadMember_AlignHeading_041196
SquadMember_AlignHeading_041196:
        bclr    #3, 0x13(a6)                    | +000
        clr.w   0x70(a6)                        | +006  toggle = 0
        clr.b   0x86(a6)                        | +00a  latch = 0
        move.w  #2, 0x78(a6)                    | +00e  pose objetivo = 2
        lea     0x286f74.l, a0                  | +014  template
        jsr     0x28cd4.l                       | +01a
        lea     .Lalign_loop(pc), a1            | +020
        move.l  a1, (a6)                        | +024
.Lalign_loop:                                   |       (= $0411BC, per-frame)
        jsr     Squad_ApplyLeaderDelta_041F84(pc) | +026
        jsr     0x28d70.l                       | +02a
        addq.w  #1, 0x70(a6)                    | +030  toggle
        andi.w  #1, 0x70(a6)                    | +034
        bne.w   .Lalign_tail                    | +03a  solo frames pares
        jsr     Squad_PhaseStepByOne_041F50(pc) | +03e  paso hacia objetivo
        bcc.w   .Lalign_tail                    | +042  aun no llego
        cmpi.b  #0, 0x86(a6)                    | +046  ya marcado ?
        bne.w   .Lalign_tail                    | +04c
        move.b  #0xFF, 0x86(a6)                 | +050
        jsr     Squad_TagSharedBit_041F3A(pc)   | +056  publica llegada
.Lalign_tail:
        jsr     Squad_PollSharedState_041FF6(pc) | +05a
        bra.w   SquadMember_FrameTail_0412FC    | +05e  cola comun (con dano)

| ----------------------------------------------------------------------------
|  SquadMember_PoseFromHeading_0411F8  @ $0411F8  (134 B)
|
|    /* Suelta el vinculo (+0x48 = -1) y aplica el template de pose
|       segun heading: indice = min(+0x76 >> 1, 5) en la tabla $28618C
|       ($FFFFFFFF = saltar). Bucle: al dar carry el test de limites,
|       marca la llegada UNA vez, publica la orden $83 (WriteBackState)
|       e instala HoldPose. Ademas fija +0x45 = 2 (i-frames minimos)
|       cada frame. Gemelo de PoseFromHeadingB_041390 (tabla $2861A4,
|       destino AlignHeadingB_KeepPose) = PAR DE CLONES #12. */
|    void SquadMember_PoseFromHeading(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.SquadMember_PoseFromHeading_0411F8, "ax", @progbits
        .global SquadMember_PoseFromHeading_0411F8
SquadMember_PoseFromHeading_0411F8:
        bclr    #3, 0x13(a6)                    | +000
        clr.b   0x86(a6)                        | +006  latch = 0
        move.l  #-1, 0x48(a6)                   | +00a  suelta vinculo
        move.w  0x76(a6), d0                    | +012  heading actual
        lsr.w   #1, d0                          | +016
        cmpi.w  #6, d0                          | +018
        bcs.w   .Lpose_idx_ok                   | +01c
        move.w  #5, d0                          | +020  clamp a 5
.Lpose_idx_ok:
        movea.l #0x28618c, a0                   | +024  tabla de poses
        lsl.w   #2, d0                          | +02a
        movea.l (a0,d0.w), a0                   | +02c
        cmpa.l  #-1, a0                         | +030
        beq.w   .Lpose_no_tmpl                  | +036
        jsr     0x28cd4.l                       | +03a
.Lpose_no_tmpl:
        lea     .Lpose_loop(pc), a1             | +040
        move.l  a1, (a6)                        | +044
.Lpose_loop:                                    |       (= $04123E, per-frame)
        jsr     Squad_ApplyLeaderDelta_041F84(pc) | +046
        move.b  #2, 0x45(a6)                    | +04a  i-frames minimos
        jsr     0x28d70.l                       | +050  limites (carry = evento)
        bcc.w   .Lpose_tail                     | +056
        cmpi.b  #0, 0x86(a6)                    | +05a  ya marcado ?
        bne.w   .Lpose_tail                     | +060
        move.b  #0xFF, 0x86(a6)                 | +064
        jsr     Squad_TagSharedBit_041F3A(pc)   | +06a  publica llegada
        move.b  #0x83, 0x80(a6)                 | +06e  orden $83
        jsr     Squad_WriteBackState_042040(pc) | +074  publica
        lea     SquadMember_HoldPose_0410C4(pc), a1 | +078
        move.l  a1, (a6)                        | +07c
.Lpose_tail:
        jsr     Squad_PollSharedState_041FF6(pc) | +07e
        bra.w   SquadMember_FrameTail_0412FC    | +082  cola comun (con dano)

| ----------------------------------------------------------------------------
|  SquadMember_AckAndHold_04127E  @ $04127E  (62 B)
|
|    /* Version incondicional de la transicion a HoldPose: aplica
|       $286F74, y cada frame marca la llegada (TagSharedBit SIN latch),
|       publica la orden $83 e instala HoldPose. */
|    void SquadMember_AckAndHold(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.SquadMember_AckAndHold_04127E, "ax", @progbits
        .global SquadMember_AckAndHold_04127E
SquadMember_AckAndHold_04127E:
        bclr    #3, 0x13(a6)                    | +000
        lea     0x286f74.l, a0                  | +006  template de espera
        jsr     0x28cd4.l                       | +00c
        lea     .Lack_loop(pc), a1              | +012
        move.l  a1, (a6)                        | +016
.Lack_loop:                                     |       (= $041296, per-frame)
        jsr     Squad_ApplyLeaderDelta_041F84(pc) | +018
        jsr     0x28d70.l                       | +01c
        jsr     Squad_TagSharedBit_041F3A(pc)   | +022  marca (sin latch)
        move.b  #0x83, 0x80(a6)                 | +026  orden $83
        jsr     Squad_WriteBackState_042040(pc) | +02c  publica
        lea     SquadMember_HoldPose_0410C4(pc), a1 | +030
        move.l  a1, (a6)                        | +034
        jsr     Squad_PollSharedState_041FF6(pc) | +036
        bra.w   SquadMember_FrameTail_0412FC    | +03a  cola comun (con dano)

| ----------------------------------------------------------------------------
|  SquadMember_HitRecoil_0412BC  @ $0412BC  (64 B)
|
|    /* Instalado por FrameTail al recibir dano: vida visual +0x66 = 1,
|       template de retroceso $2872A2. Bucle: cuando el test de limites
|       da carry (retroceso terminado) limpia bit 3 de +0x13, restaura
|       +0x66 = 1 y resetea la orden compartida a 0 (+0x80 = 0, el
|       lider re-emitira). Cola comun SIN chequeo de dano (i-frames). */
|    void SquadMember_HitRecoil(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.SquadMember_HitRecoil_0412BC, "ax", @progbits
        .global SquadMember_HitRecoil_0412BC
SquadMember_HitRecoil_0412BC:
        move.w  #1, 0x66(a6)                    | +000  vida visual = 1
        lea     0x2872a2.l, a0                  | +006  template de retroceso
        jsr     0x28cd4.l                       | +00c
        lea     .Lrecoil_loop(pc), a1           | +012
        move.l  a1, (a6)                        | +016
.Lrecoil_loop:                                  |       (= $0412D4, per-frame)
        jsr     Squad_ApplyLeaderDelta_041F84(pc) | +018
        jsr     0x28d70.l                       | +01c
        bcc.w   .Lrecoil_tail                   | +022  aun retrocediendo
        bclr    #3, 0x13(a6)                    | +026
        move.w  #1, 0x66(a6)                    | +02c
        move.b  #0, 0x80(a6)                    | +032  orden = 0 (reset)
.Lrecoil_tail:
        jsr     Squad_PollSharedState_041FF6(pc) | +038
        bra.w   SquadMember_FrameTailFull_041316 | +03c  cola comun (sin dano)

| ----------------------------------------------------------------------------
|  SquadMember_FrameTail_0412FC  @ $0412FC  (36 B)  - cola comun
|
|    /* Cola comun de todos los per-frame de miembro. Entrada $0412FC
|       (con chequeo de dano): si $2870A da carry, +0x87++ (contador de
|       impactos), +0x45 = $1E (i-frames) e instala HitRecoil. Entrada
|       $041316 (FrameTailFull, .global, sin chequeo de dano): $5E45A
|       fuera-de-mundo; con carry cae en la isla SetTaskHandler_041320
|       que instala el handler de muerte $40EF2; sin carry salta al rts
|       INTERNO de esa isla (SetHandlerRts_041326, idioma Wave SS). */
| ----------------------------------------------------------------------------
        .section .text.SquadMember_FrameTail_0412FC, "ax", @progbits
        .global SquadMember_FrameTail_0412FC
        .global SquadMember_FrameTailFull_041316
SquadMember_FrameTail_0412FC:
        jsr     0x2870a.l                       | +000  dano recibido ?
        bcc.w   SquadMember_FrameTailFull_041316 | +006  no
        addq.b  #1, 0x87(a6)                    | +00a  contador de impactos
        move.b  #0x1E, 0x45(a6)                 | +00e  i-frames
        lea     SquadMember_HitRecoil_0412BC(pc), a1 | +014
        move.l  a1, (a6)                        | +018
SquadMember_FrameTailFull_041316:               |       (= $041316)
        jsr     0x5e45a.l                       | +01a  fuera de mundo ?
        bcc.w   SetHandlerRts_041326            | +020  no: rts de la isla
        | --- cae en SetTaskHandler_041320 (isla): instala $40EF2 y rts ------

| ----------------------------------------------------------------------------
|  SquadMember_AlignHeadingB_041328  @ $041328  (98 B)
|
|    /* Clon del AlignHeading con pose objetivo 8 y template $286F7A;
|       ademas resetea pose actual 2 y suelta el vinculo +0x48 = -1.
|       La entrada secundaria $04132E (global) omite el reset de pose
|       (la instala PoseFromHeadingB tras publicar $83). Sale por la
|       isla JsrPcThunk_04138a (jsr Squad_PollSharedState; rts). */
| ----------------------------------------------------------------------------
        .section .text.SquadMember_AlignHeadingB_041328, "ax", @progbits
        .global SquadMember_AlignHeadingB_041328
        .global SquadMember_AlignHeadingB_KeepPose_04132E
SquadMember_AlignHeadingB_041328:
        move.w  #2, 0x76(a6)                    | +000  pose actual = 2
SquadMember_AlignHeadingB_KeepPose_04132E:      |       (= $04132E)
        move.l  #-1, 0x48(a6)                   | +006  suelta vinculo
        clr.w   0x70(a6)                        | +00e  toggle = 0
        clr.b   0x86(a6)                        | +012  latch = 0
        move.w  #8, 0x78(a6)                    | +016  pose objetivo 8
        lea     0x286f7a.l, a0                  | +01c  template
        jsr     0x28cd4.l                       | +022
        lea     .Lalignb_loop(pc), a1           | +028
        move.l  a1, (a6)                        | +02c
.Lalignb_loop:                                  |       (= $041356)
        jsr     Squad_ApplyLeaderDelta_041F84(pc) | +02e
        jsr     0x28d70.l                       | +032
        addq.w  #1, 0x70(a6)                    | +038
        andi.w  #1, 0x70(a6)                    | +03c
        bne.w   .Lalignb_tail                   | +042
        jsr     Squad_PhaseStepByOne_041F50(pc) | +046
        bcc.w   .Lalignb_tail                   | +04a
        cmpi.b  #0, 0x86(a6)                    | +04e
        bne.w   .Lalignb_tail                   | +054
        move.b  #0xFF, 0x86(a6)                 | +058
        jsr     Squad_TagSharedBit_041F3A(pc)   | +05e
.Lalignb_tail:
        | --- cae en JsrPcThunk_04138a (isla): jsr PollSharedState; rts ------

| ----------------------------------------------------------------------------
|  SquadMember_PoseFromHeadingB_041390  @ $041390  (114 B)
|
|    /* Clon del PoseFromHeading con la tabla de poses $2861A4 y salto
|       a AlignHeadingB_KeepPose tras publicar la orden $83 (sin el
|       bclr inicial ni el +0x45 = 2 del gemelo). Sale por la isla
|       JsrPcThunk_041402 (jsr PollSharedState; rts). Junto a su
|       gemelo forman el PAR DE CLONES #12 del catalogo. */
| ----------------------------------------------------------------------------
        .section .text.SquadMember_PoseFromHeadingB_041390, "ax", @progbits
        .global SquadMember_PoseFromHeadingB_041390
SquadMember_PoseFromHeadingB_041390:
        clr.b   0x86(a6)                        | +000  latch = 0
        move.l  #-1, 0x48(a6)                   | +004  suelta vinculo
        move.w  0x76(a6), d0                    | +00c  heading actual
        lsr.w   #1, d0                          | +010
        cmpi.w  #6, d0                          | +012
        bcs.w   .Lposeb_idx_ok                  | +016
        move.w  #5, d0                          | +01a
.Lposeb_idx_ok:
        movea.l #0x2861a4, a0                   | +01e  tabla de poses B
        lsl.w   #2, d0                          | +024
        movea.l (a0,d0.w), a0                   | +026
        cmpa.l  #-1, a0                         | +02a
        beq.w   .Lposeb_no_tmpl                 | +030
        jsr     0x28cd4.l                       | +034
.Lposeb_no_tmpl:
        lea     .Lposeb_loop(pc), a1            | +03a
        move.l  a1, (a6)                        | +03e
.Lposeb_loop:                                   |       (= $0413D0)
        jsr     Squad_ApplyLeaderDelta_041F84(pc) | +040
        jsr     0x28d70.l                       | +044
        bcc.w   .Lposeb_tail                    | +04a
        cmpi.b  #0, 0x86(a6)                    | +04e
        bne.w   .Lposeb_tail                    | +054
        move.b  #0xFF, 0x86(a6)                 | +058
        jsr     Squad_TagSharedBit_041F3A(pc)   | +05e
        move.b  #0x83, 0x80(a6)                 | +062  orden $83
        jsr     Squad_WriteBackState_042040(pc) | +068  publica
        lea     SquadMember_AlignHeadingB_KeepPose_04132E(pc), a1 | +06c
        move.l  a1, (a6)                        | +070
.Lposeb_tail:
        | --- cae en JsrPcThunk_041402 (isla): jsr PollSharedState; rts ------
