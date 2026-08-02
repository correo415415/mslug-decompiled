| ============================================================================
|  Metal Slug 1 - asm/squad_spawn_states_041fxx.s
|  ----------------------------------------------------------------------------
|  Wave TT (parte 2/2) - spawner y gestion de estado compartido del
|  subsistema "escuadron" de 8 entities con vuelo senoidal, mas el par
|  de moduladores de brillo por seno del final del cluster.
|  Cluster $041FB4..$0422E4, 12 funciones (14 entradas), 810 B.
|
|  Mapa (islas ya matcheadas intercaladas, se conservan):
|    $041FB4 Squad_SpawnEight_041FB4          (66 B)
|    $041FF6 Squad_PollSharedState_041FF6     (74 B) <- promueve PcThunkTarget_041ff6
|    $042040 Squad_WriteBackState_042040      (42 B) <- promueve PcThunkTarget_042040
|    $04206A Squad_DepthToScaleIdx_04206A     (58 B)
|    $0420A4 Squad_SinCosVelocity_0420A4      (64 B)
|    $0420E4 SquadPair_SpawnJoinTail_0420E4    (4 B)  bra a $042188
|    $0420E8 SquadPair_SpawnFlagged_0420E8    (22 B)  2 entradas ($0420F6)
|    $0420FE SquadPair_SpawnCore_0420FE      (132 B)  -> cae en JsrPcThunk_042182
|    $042188 SquadPair_SpawnPlain_042188     (126 B)
|    $042206 TrioSpawner_PatternedPair_042206 (124 B)
|    $042282 Entity_ShadeBySine_042282        (72 B)
|    $0422CA Entity_SineToStep_0422CA         (26 B)  -> cae en SetTaskW_0422e4
|
|  Callees externos (abs.l literales, aun sin matchear):
|    $4AE   = ThunkTarget_0004ae   Task_AllocFromFreeList(handler en a1)
|    $5DD02 = ThunkTarget_05dd02   Entity_CopyTransform (copia a6 -> a0)
|    $2352  = InputGuardCall219c   (aqui: encolar peticion de sonido d0)
|    $5E506 / $5E1EA = getters de entity (lider / objetivo)
|  Handlers instalados (pc-rel, dentro del segmento $04xxxx):
|    $40F00 = handler de miembro de escuadron (SquadMember_Handler_040F00)
|    $40F82 = handler de cambio de estado    (SquadMember_OnStateChange_040F82)
|    $415C6 / $41626 = handlers del par escoltado (PairChild_*)
|    $419CC / $419FC = handlers del trio patron  (TrioChild_*)
|
|  Tablas de datos (segmento de datos, sin matchear):
|    $28615C = 8 registros de 4 bytes (dx,dy,dz,bit-id) de los miembros
|    $2861D4 = 6 pares de words (offset X,Y) por distancia del par
|    $28631C = 16 bytes de patron ciclico (indices 0..3) del trio
|    $286310 = 4 registros (X,Y) usados por el patron del trio
|    $2863BC = jump table de handlers por estado compartido (bit 7 + 0..127)
|    $2C072C / $2C07AC = tablas seno / coseno (256 words, coseno = seno+64)
| ============================================================================

| ----------------------------------------------------------------------------
|  Squad_SpawnEight_041FB4  @ $041FB4  (66 B)
|
|    /* Crea los 8 miembros del escuadron: para cada registro de 4 bytes
|       de la tabla $28615C aloja una task con handler $40F00 (pc-rel),
|       copia el transform del padre (a6 -> a0 nuevo) via $5DD02 y
|       siembra los campos +0x82..+0x85 (dx,dy,dz,bit-id) del miembro
|       desde el registro. Bucle con contador en d0 preservado junto a
|       a1 con movem.l (idioma "PUSH multiple" ya catalogado). */
|    void Squad_SpawnEight(Entity *parent /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.Squad_SpawnEight_041FB4, "ax", @progbits
        .global Squad_SpawnEight_041FB4
Squad_SpawnEight_041FB4:
        clr.w   d0                              | +000  contador = 0
        lea     0x28615c.l, a1                  | +002  tabla de miembros
.Lspawn_loop:
        movem.l d0/a1, -(sp)                    | +008  preserva contador+cursor
        lea     SquadMember_Handler_040F00(pc), a1 | +00c  handler del miembro
        jsr     0x4ae.l                         | +010  Task_Alloc -> a0
        jsr     0x5dd02.l                       | +016  Entity_CopyTransform
        movem.l (sp)+, d0/a1                    | +01c
        move.b  (a1), 0x82(a0)                  | +020  dx por frame (s8)
        move.b  1(a1), 0x83(a0)                 | +024  dy por frame (s8)
        move.b  2(a1), 0x84(a0)                 | +02a  dz por frame (s8)
        move.b  3(a1), 0x85(a0)                 | +030  bit-id del miembro
        addq.l  #4, a1                          | +036  siguiente registro
        addq.w  #1, d0                          | +038
        cmpi.w  #8, d0                          | +03a  8 miembros
        bcs.b   .Lspawn_loop                    | +03e
        rts                                     | +040

| ----------------------------------------------------------------------------
|  Squad_PollSharedState_041FF6  @ $041FF6  (74 B)
|
|    /* Detecta cambios del estado compartido: lee el byte del array de
|       estados de la estructura compartida (+0xC), indexado por el
|       bit-id propio (+0x85 & 7) + 0x80. Si difiere del cacheado +0x80
|       lo cachea (+0x80 y +0x81) e instala el handler de transicion:
|       $40F82 por defecto, o el del jump table $2863BC[estado & 0x7F]
|       si el estado tiene el bit 7 (comando directo). Mismo idioma de
|       instalacion "move.l x,(a6)" que Squad_StateDispatch. */
|    void Squad_PollSharedState(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.Squad_PollSharedState_041FF6, "ax", @progbits
        .global Squad_PollSharedState_041FF6
Squad_PollSharedState_041FF6:
        movea.l 0xc(a6), a0                     | +000  estructura compartida
        move.b  0x85(a6), d0                    | +004  bit-id propio
        andi.w  #7, d0                          | +008
        addi.w  #0x80, d0                       | +00c  array en +0x80
        move.b  (a0,d0.w), d0                   | +010  estado publicado
        cmp.b   0x80(a6), d0                    | +014  cambio ?
        beq.w   .Lno_change                     | +018  no: nada que hacer
        move.b  d0, 0x80(a6)                    | +01c  cachea estado
        move.b  d0, 0x81(a6)                    | +020  copia de eco (writeback)
        lea     SquadMember_OnStateChange_040F82(pc), a1 | +024
        move.l  a1, (a6)                        | +028  handler por defecto
        btst    #7, 0x80(a6)                    | +02a  comando directo ?
        beq.w   .Lno_change                     | +030  no: listo
        move.b  0x80(a6), d0                    | +034
        andi.w  #0x7F, d0                       | +038  indice de comando
        lsl.w   #2, d0                          | +03c
        lea     0x2863bc.l, a0                  | +03e  jump table de comandos
        move.l  (a0,d0.w), (a6)                 | +044  instala handler
.Lno_change:
        rts                                     | +048

| ----------------------------------------------------------------------------
|  Squad_WriteBackState_042040  @ $042040  (42 B)
|
|    /* Contraparte de PollSharedState: publica el estado propio +0x80 en
|       el array compartido, pero SOLO si el valor publicado sigue siendo
|       el eco +0x81 (nadie mas lo ha tocado desde la ultima lectura:
|       compare-and-swap cooperativo sin atomicidad, valido porque las
|       tasks corren secuenciadas por el scheduler). */
|    void Squad_WriteBackState(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.Squad_WriteBackState_042040, "ax", @progbits
        .global Squad_WriteBackState_042040
Squad_WriteBackState_042040:
        move.b  0x80(a6), d1                    | +000  estado propio
        movea.l 0xc(a6), a0                     | +004  estructura compartida
        move.b  0x85(a6), d0                    | +008  bit-id propio
        andi.w  #7, d0                          | +00c
        addi.w  #0x80, d0                       | +010
        move.b  (a0,d0.w), d2                   | +014  valor publicado
        cmp.b   0x81(a6), d2                    | +018  sigue siendo mi eco ?
        bne.w   .Lskip                          | +01c  no: otro escribio
        move.b  d1, (a0,d0.w)                   | +020  publica
        move.b  d1, 0x81(a6)                    | +024  actualiza eco
.Lskip:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  Squad_DepthToScaleIdx_04206A  @ $04206A  (58 B)
|
|    /* Cuantiza una coordenada (d0, tipicamente Y de profundidad) a un
|       indice de escala PAR 0..10: clamp [0x70..0xC0], -0x68, /8,
|       clamp [0..0xA], andi #$FE (los indices impares se reservan a
|       la variante espejada de las tablas de sprite-scale). */
|    u16 Squad_DepthToScaleIdx(u16 y /*d0*/);
| ----------------------------------------------------------------------------
        .section .text.Squad_DepthToScaleIdx_04206A, "ax", @progbits
        .global Squad_DepthToScaleIdx_04206A
Squad_DepthToScaleIdx_04206A:
        cmpi.w  #0x70, d0                       | +000
        bgt.w   .Llo_ok                         | +004
        move.w  #0x70, d0                       | +008  clamp inferior
.Llo_ok:
        cmpi.w  #0xC0, d0                       | +00c
        blt.w   .Lhi_ok                         | +010
        move.w  #0xC0, d0                       | +014  clamp superior
.Lhi_ok:
        subi.w  #0x68, d0                       | +018  base
        lsr.w   #3, d0                          | +01c  /8
        cmpi.w  #0, d0                          | +01e
        bge.w   .Lpos                           | +022
        clr.w   d0                              | +026  (defensivo, inalcanzable)
.Lpos:
        cmpi.w  #0xA, d0                        | +028
        blt.w   .Lidx_ok                        | +02c
        move.w  #0xA, d0                        | +030  tope de indice
.Lidx_ok:
        andi.w  #0xFE, d0                       | +034  fuerza indice par
        rts                                     | +038

| ----------------------------------------------------------------------------
|  Squad_SinCosVelocity_0420A4  @ $0420A4  (64 B)
|
|    /* Convierte heading (d0, 0..255) + amplitud (+0x36, minimo 0x100)
|       en el vector velocidad: velY (+0x2A) = sin(h)*amp >> 8,
|       velX (+0x28) = cos(h)*amp >> 8. Coseno = tabla seno desfasada
|       64 entradas ($2C07AC = $2C072C + 0x80). Callee pc-rel de
|       Squad_SteerTowardTarget (cruza al archivo parte 1). */
|    void Squad_SinCosVelocity(Entity *e /*a6*/, u16 heading /*d0*/);
| ----------------------------------------------------------------------------
        .section .text.Squad_SinCosVelocity_0420A4, "ax", @progbits
        .global Squad_SinCosVelocity_0420A4
Squad_SinCosVelocity_0420A4:
        andi.w  #0xFF, d0                       | +000  angulo 0..255
        add.w   d0, d0                          | +004  indice de word
        lea     0x2c072c.l, a1                  | +006  tabla seno
        lea     0x2c07ac.l, a2                  | +00c  tabla coseno (seno+64)
        move.w  (a1,d0.w), d1                   | +012  sin(h)
        move.w  (a2,d0.w), d2                   | +016  cos(h)
        move.w  0x36(a6), d0                    | +01a  amplitud
        andi.w  #0x7FFF, d0                     | +01e  descarta bit de signo
        cmpi.w  #0x100, d0                      | +022  minimo 1.0 (8.8 fixed)
        bgt.w   .Lamp_ok                        | +026
        move.w  #0x100, d0                      | +02a
.Lamp_ok:
        muls.w  d0, d1                          | +02e  sin * amp
        muls.w  d0, d2                          | +030  cos * amp
        asr.l   #8, d1                          | +032  a 8.8
        asr.l   #8, d2                          | +034
        move.w  d1, 0x2a(a6)                    | +036  vel Y
        move.w  d2, 0x28(a6)                    | +03a  vel X
        rts                                     | +03e

| ----------------------------------------------------------------------------
|  SquadPair_SpawnJoinTail_0420E4  @ $0420E4  (4 B)
|
|    /* Trampolin: bra.w a SquadPair_SpawnPlain_042188 (la variante que
|       limpia +0x7F y fija +0x88=1 en los hijos). Colocado justo antes
|       de las otras dos entradas para compartir el rango pc-rel. */
| ----------------------------------------------------------------------------
        .section .text.SquadPair_SpawnJoinTail_0420E4, "ax", @progbits
        .global SquadPair_SpawnJoinTail_0420E4
SquadPair_SpawnJoinTail_0420E4:
        bra.w   SquadPair_SpawnPlain_042188     | +000

| ----------------------------------------------------------------------------
|  SquadPair_SpawnFlagged_0420E8  @ $0420E8  (22 B)  - DOS ENTRADAS
|
|    /* Preambulos del spawner de par: fijan los flags heredables antes
|       de caer en SquadPair_SpawnCore_0420FE.
|         $0420E8: +0x88 = 0, +0x7F = 1 (variante "flagged")
|         $0420F6: +0x88 = 0, +0x7F = 0 (variante limpia)          */
|    void SquadPair_SpawnFlagged(Entity *e /*a6*/);
|    void SquadPair_SpawnClean(Entity *e /*a6*/);    // entrada $0420F6
| ----------------------------------------------------------------------------
        .section .text.SquadPair_SpawnFlagged_0420E8, "ax", @progbits
        .global SquadPair_SpawnFlagged_0420E8
        .global SquadPair_SpawnClean_0420F6
SquadPair_SpawnFlagged_0420E8:
        clr.b   0x88(a6)                        | +000
        move.b  #1, 0x7f(a6)                    | +004
        bra.w   SquadPair_SpawnCore_0420FE      | +00a
SquadPair_SpawnClean_0420F6:
        clr.b   0x88(a6)                        | +00e
        clr.b   0x7f(a6)                        | +012
        | --- cae en SquadPair_SpawnCore_0420FE ------------------------------

| ----------------------------------------------------------------------------
|  SquadPair_SpawnCore_0420FE  @ $0420FE  (132 B)
|
|    /* Nucleo del spawner de par escoltado: encola el sonido $1064,
|       cuantiza la "distancia" heading/2 a 0..5 e indexa la tabla de
|       offsets $2861D4 (pares X,Y). Crea DOS hijos con handlers
|       $415C6 y $41626 (pc-rel), cada uno con copy-transform y offset
|       aplicado; el segundo hereda ademas el heading d0 en +0x76. Los
|       hijos heredan +0x7F/+0x88 del padre y el padre pasa a estado
|       compartido $87 (bit7 = comando directo 7). Cae en
|       JsrPcThunk_042182 (isla, post-hook) y de ahi al codigo comun. */
|    void SquadPair_SpawnCore(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.SquadPair_SpawnCore_0420FE, "ax", @progbits
        .global SquadPair_SpawnCore_0420FE
SquadPair_SpawnCore_0420FE:
        move.w  #0x1064, d0                     | +000  id de sonido
        jsr     0x2352.l                        | +004  InputGuardCall219c
        move.w  0x76(a6), d0                    | +00a  heading propio
        lsr.w   #1, d0                          | +00e  /2
        cmpi.w  #6, d0                          | +010  tope 5
        bcs.w   .Lidx_ok1                       | +014
        move.w  #5, d0                          | +018
.Lidx_ok1:
        move.w  d0, d3                          | +01c
        lsl.w   #2, d3                          | +01e  *4 (pares de words)
        lea     0x2861d4.l, a0                  | +020  tabla de offsets
        move.w  (a0,d3.w), d1                   | +026  offset X
        move.w  2(a0,d3.w), d2                  | +02a  offset Y
        movem.w d0-d2, -(sp)                    | +02e  preserva idx+offsets
        lea     PairChild_HandlerA_0415C6(pc), a1 | +032
        jsr     0x4ae.l                         | +036  Task_Alloc -> a0
        jsr     0x5dd02.l                       | +03c  Entity_CopyTransform
        movem.w (sp)+, d0-d2                    | +042
        add.w   d1, 0x22(a0)                    | +046  hijo A: pos += offset
        add.w   d2, 0x24(a0)                    | +04a
        movem.w d0-d2, -(sp)                    | +04e
        lea     PairChild_HandlerB_041626(pc), a1 | +052
        jsr     0x4ae.l                         | +056  Task_Alloc -> a0
        jsr     0x5dd02.l                       | +05c  Entity_CopyTransform
        movem.w (sp)+, d0-d2                    | +062
        move.w  d0, 0x76(a0)                    | +066  hijo B hereda heading
        add.w   d1, 0x22(a0)                    | +06a  pos += offset
        add.w   d2, 0x24(a0)                    | +06e
        move.b  0x7f(a6), 0x7f(a0)              | +072  hereda flag variante
        move.b  0x88(a6), 0x88(a0)              | +078  hereda flag extra
        move.b  #0x87, 0x80(a6)                 | +07e  padre: comando 7|bit7
        | --- cae en JsrPcThunk_042182 (isla, post-hook) ---------------------

| ----------------------------------------------------------------------------
|  SquadPair_SpawnPlain_042188  @ $042188  (126 B)
|
|    /* Clon NO factorizado de SpawnCore (mismo cuerpo, par #11 del
|       catalogo de clones): unica diferencia el epilogo, que en vez de
|       heredar flags fija en el hijo B +0x7F = 0 y +0x88 = 1, no toca
|       el estado compartido del padre y retorna con rts propio. */
|    void SquadPair_SpawnPlain(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.SquadPair_SpawnPlain_042188, "ax", @progbits
        .global SquadPair_SpawnPlain_042188
SquadPair_SpawnPlain_042188:
        move.w  #0x1064, d0                     | +000  id de sonido
        jsr     0x2352.l                        | +004
        move.w  0x76(a6), d0                    | +00a
        lsr.w   #1, d0                          | +00e
        cmpi.w  #6, d0                          | +010
        bcs.w   .Lidx_ok2                       | +014
        move.w  #5, d0                          | +018
.Lidx_ok2:
        move.w  d0, d3                          | +01c
        lsl.w   #2, d3                          | +01e
        lea     0x2861d4.l, a0                  | +020
        move.w  (a0,d3.w), d1                   | +026
        move.w  2(a0,d3.w), d2                  | +02a
        movem.w d0-d2, -(sp)                    | +02e
        lea     PairChild_HandlerA_0415C6(pc), a1 | +032
        jsr     0x4ae.l                         | +036
        jsr     0x5dd02.l                       | +03c
        movem.w (sp)+, d0-d2                    | +042
        add.w   d1, 0x22(a0)                    | +046
        add.w   d2, 0x24(a0)                    | +04a
        movem.w d0-d2, -(sp)                    | +04e
        lea     PairChild_HandlerB_041626(pc), a1 | +052
        jsr     0x4ae.l                         | +056
        jsr     0x5dd02.l                       | +05c
        movem.w (sp)+, d0-d2                    | +062
        move.w  d0, 0x76(a0)                    | +066
        add.w   d1, 0x22(a0)                    | +06a
        add.w   d2, 0x24(a0)                    | +06e
        clr.b   0x7f(a0)                        | +072  hijo B: variante limpia
        move.b  #1, 0x88(a0)                    | +076  flag extra forzado
        rts                                     | +07c

| ----------------------------------------------------------------------------
|  TrioSpawner_PatternedPair_042206  @ $042206  (124 B)
|
|    /* Spawner del trio con patron: encola el sonido $10A3, avanza el
|       contador ciclico +0x9A y lo usa (mod 16) para leer el byte de
|       patron de $28631C; ese byte (mod 4) indexa la tabla de offsets
|       $286310 (pares X,Y) y se pasa a los hijos como d3. Crea DOS
|       hijos con handlers $419CC y $419FC (pc-rel), copy-transform y
|       offset; el hijo B recibe ademas d3 en +0x9A (fase del patron). */
|    void TrioSpawner_PatternedPair(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.TrioSpawner_PatternedPair_042206, "ax", @progbits
        .global TrioSpawner_PatternedPair_042206
TrioSpawner_PatternedPair_042206:
        move.w  #0x10A3, d0                     | +000  id de sonido
        jsr     0x2352.l                        | +004
        move.w  0x9a(a6), d0                    | +00a  contador ciclico
        addq.w  #1, 0x9a(a6)                    | +00e
        andi.w  #0xF, d0                        | +012  mod 16
        lea     0x28631c.l, a0                  | +016  tabla de patron
        move.b  (a0,d0.w), d0                   | +01c  byte de patron
        andi.w  #3, d0                          | +020  mod 4
        move.w  d0, d3                          | +024  d3 = fase (a los hijos)
        lea     0x286310.l, a0                  | +026  tabla de offsets
        lsl.w   #2, d0                          | +02c
        move.w  (a0,d0.w), d1                   | +02e  offset X
        move.w  2(a0,d0.w), d2                  | +032  offset Y
        movem.w d1-d3, -(sp)                    | +036
        lea     TrioChild_HandlerA_0419CC(pc), a1 | +03a
        jsr     0x4ae.l                         | +03e  Task_Alloc -> a0
        jsr     0x5dd02.l                       | +044  Entity_CopyTransform
        movem.w (sp)+, d1-d3                    | +04a
        add.w   d1, 0x22(a0)                    | +04e  hijo A: pos += offset
        add.w   d2, 0x24(a0)                    | +052
        movem.w d1-d3, -(sp)                    | +056
        lea     TrioChild_HandlerB_0419FC(pc), a1 | +05a
        jsr     0x4ae.l                         | +05e
        jsr     0x5dd02.l                       | +064
        movem.w (sp)+, d1-d3                    | +06a
        add.w   d1, 0x22(a0)                    | +06e  hijo B: pos += offset
        add.w   d2, 0x24(a0)                    | +072
        move.w  d3, 0x9a(a0)                    | +076  hereda fase del patron
        rts                                     | +07a

| ----------------------------------------------------------------------------
|  Entity_ShadeBySine_042282  @ $042282  (72 B)
|
|    /* Modulacion de brillo/escala por onda triangular derivada del
|       contador +0x34: pliega 0..255 -> 0..127 -> 0..63, aplica zona
|       muerta (<0x18 -> 0) y escribe 0x80 + 2*v en los bytes de shade
|       +0x32/+0x33; ademas integra v en +0x38 (profundidad/altura).
|       Da el efecto de "palpitacion" de los objetos brillantes. */
|    void Entity_ShadeBySine(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.Entity_ShadeBySine_042282, "ax", @progbits
        .global Entity_ShadeBySine_042282
Entity_ShadeBySine_042282:
        move.b  0x34(a6), d0                    | +000  contador de fase
        andi.w  #0xFF, d0                       | +004
        cmpi.w  #0x80, d0                       | +008  pliegue 255->0
        bcs.w   .Lfold1                         | +00c
        move.w  d0, d1                          | +010
        move.w  #0xFF, d0                       | +012
        sub.w   d1, d0                          | +016
.Lfold1:
        cmpi.w  #0x40, d0                       | +018  pliegue 127->0
        bcs.w   .Lfold2                         | +01c
        move.w  d0, d1                          | +020
        move.w  #0x7F, d0                       | +022
        sub.w   d1, d0                          | +026
.Lfold2:
        cmpi.w  #0x18, d0                       | +028  zona muerta
        bcc.w   .Lshade                         | +02c
        clr.w   d0                              | +030
.Lshade:
        move.w  d0, d1                          | +032
        lsl.w   #1, d0                          | +034  *2
        addi.w  #0x80, d0                       | +036  base de shade
        move.b  d0, 0x32(a6)                    | +03a  shade A
        move.b  d0, 0x33(a6)                    | +03e  shade B
        add.w   d1, 0x38(a6)                    | +042  integra en profundidad
        rts                                     | +046

| ----------------------------------------------------------------------------
|  Entity_SineToStep_0422CA  @ $0422CA  (26 B)
|
|    /* Variante corta del plegado: 0..255 -> 0..127, /8, y cae en
|       SetTaskW_0422e4 (isla) que almacena la word resultante en la
|       entity. Paso discreto 0..15 derivado de la fase +0x34. */
|    void Entity_SineToStep(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.Entity_SineToStep_0422CA, "ax", @progbits
        .global Entity_SineToStep_0422CA
Entity_SineToStep_0422CA:
        move.b  0x34(a6), d0                    | +000  contador de fase
        andi.w  #0xFF, d0                       | +004
        cmpi.w  #0x80, d0                       | +008  pliegue 255->0
        bcs.w   .Lfold                          | +00c
        move.w  d0, d1                          | +010
        move.w  #0xFF, d0                       | +012
        sub.w   d1, d0                          | +016
.Lfold:
        lsr.w   #3, d0                          | +018  /8 -> 0..15
        | --- cae en SetTaskW_0422e4 (isla): almacena y rts ------------------
