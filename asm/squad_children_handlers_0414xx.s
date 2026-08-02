| ============================================================================
|  Metal Slug 1 - asm/squad_children_handlers_0414xx.s
|  ----------------------------------------------------------------------------
|  Wave UU (parte 2/2) - handlers de los HIJOS derivados del escuadron:
|  fisica de picado/aterrizaje, hijos del par escoltado (PairChild),
|  spawner de caida, caida en zigzag, ataque en planeo y el trio con
|  seguimiento orbital. Cluster $041408..$041C12, 18 funciones, 1 970 B.
|
|  Mismo patron threaded-scheduler que la parte 1: setup una sola vez +
|  `lea cont(pc),a1; move.l a1,(a6)` autoinstalando la continuacion
|  per-frame. Las islas SetTaskHandler_XXXX intercaladas (ya matcheadas
|  en task_handlers.c) instalan el siguiente estado o el handler de
|  muerte $40EF2 (Jsr5B6ThenJmpScheduler); el IDIOMA "branch a mitad de
|  isla" (bcc al rts INTERNO de la isla, SetHandlerRts_XXXX) aparece
|  11 veces en este fichero.
|
|  Callees externos (hipotesis, abs.l literales):
|    $2352  = sonido inmediato (id d0)   $236E  = encolar sonido (id d1)
|    $28134 = flags de sprite (d0)       $28CD4 = aplicar template (a0)
|    $27D50 = fisica con gravedad        $2783A = fisica simple
|    $27BC8 / $27CEE = movimiento guiado (carry = evento)
|    $267E2 = relink de sprite           $28D70 = test de limites
|    $283CA / $283D8 = attach / detach de sub-sprites
|    $28758 = colision con jugador       $2870A = chequeo de dano
|    $4AAD2 / $4AAE0 = spawns de efecto  $5DD56 = release de tarea
|    $5E070 = angulo hacia entity        $5E1EA = adquirir objetivo
|    $5E9B6 = RNG                        $78F8A = paso orbital
|    $799DE = medir template             $138FE = init subsistema (+0x1C)
|    $440D0 = conversion de coordenada   $77FD6 / $77F6A = colas de muerte
|
|  HALLAZGO FORENSE: en $041A96 hay un DEAD STORE `movea.l #-1,a0`
|  inmediatamente pisado por `lea 0x28610A.l,a0` - los demas release
|  ($5DD56) pasan a0 = -1; aqui Nazca edito la plantilla a mano para
|  pasar un puntero real y olvido borrar la carga anterior. Tambien se
|  preservan dos `ori.w #0,0x38(a6)` muertos (capa 0 explicita) y el
|  encoding `move.w #0,0x88(a6)` (3d7c, no clr.w) en $041B18.
| ============================================================================

| ----------------------------------------------------------------------------
|  SquadChild_SwoopPhysics_041408  @ $041408  (122 B)
|
|    /* Picado hacia la izquierda con gravedad: capa $18, velocidad
|       (-$400, $100) con drag +0x2C = $20 y gravedad -$28, sonido $98,
|       template $2872E4. Bucle $27D50: al tocar suelo instala
|       FlipTouchdown; si velX >= -$80 corta el drag; cuando la senal
|       +0x21 se activa cae en SetTaskHandler_041482 (instala
|       TouchdownIdle), si no sale por SetHandlerRts_041488. */
|    void SquadChild_SwoopPhysics(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.SquadChild_SwoopPhysics_041408, "ax", @progbits
        .global SquadChild_SwoopPhysics_041408
SquadChild_SwoopPhysics_041408:
        move.w  #0xd000, d0                     | +000
        jsr     0x28134.l                       | +004  flags de sprite
        andi.w  #0xffe3, 0x38(a6)               | +00a
        ori.w   #0x18, 0x38(a6)                 | +010  capa $18
        move.w  #0xfc00, 0x28(a6)               | +016  velX = -$400
        move.w  #0x20, 0x2c(a6)                 | +01c  drag
        move.w  #0x100, 0x2a(a6)                | +022  velY = $100
        move.w  #0xffd8, 0x2e(a6)               | +028  gravedad = -$28
        move.w  #0x98, d1                       | +02e  sonido
        clr.b   0x21(a6)                        | +032  senal = 0
        jsr     0x236e.l                        | +036
        lea     0x2872e4.l, a0                  | +03c  template de picado
        jsr     0x28cd4.l                       | +042
        lea     .Lswoop_loop(pc), a1            | +048
        move.l  a1, (a6)                        | +04c
.Lswoop_loop:                                   |       (= $041456, per-frame)
        jsr     0x27d50.l                       | +04e  fisica con gravedad
        bcc.w   .Lswoop_air                     | +054  sin contacto
        lea     SquadChild_FlipTouchdown_04148A(pc), a1 | +058
        move.l  a1, (a6)                        | +05c
.Lswoop_air:
        jsr     0x28d70.l                       | +05e  limites
        cmpi.w  #0xff80, 0x28(a6)               | +064  velX < -$80 ?
        blt.w   .Lswoop_keep                    | +06a  si: mantiene drag
        clr.w   0x2c(a6)                        | +06e  no: corta el drag
.Lswoop_keep:
        tst.b   0x21(a6)                        | +072  senal de llegada ?
        beq.w   SetHandlerRts_041488            | +076  no: rts de la isla
        | --- cae en SetTaskHandler_041482 (isla): instala $4155A y rts ------

| ----------------------------------------------------------------------------
|  SquadChild_FlipTouchdown_04148A  @ $04148A  (200 B)
|
|    /* Aterrizaje con voltereta: dos efectos $4AAD2 en (x-$20, y-8)
|       espejados via eori del flip +0x3A (offsets sumados con
|       addi #$FFE0/#$FFF8 y restaurados con subi), relink de sprite,
|       rebote velY = $21C con gravedad -$24 y template $28732C.
|       Bucle $27D50: al tocar suelo pone el bit "apoyado" (+0x12),
|       fija el vinculo +0x48 = $285DBE, template $28735E y pasa a
|       fisica simple $2783A. Si $2870A detecta dano: sonido $103F,
|       efecto $4AAE0 (mismo truco de offsets) y cae en la isla
|       JsrAbsThunk_041552 con a0 = template $287372 (jsr $28CD4;rts);
|       sin dano sale por JsrAbsRts_041558 (rts interno de la isla). */
|    void SquadChild_FlipTouchdown(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.SquadChild_FlipTouchdown_04148A, "ax", @progbits
        .global SquadChild_FlipTouchdown_04148A
SquadChild_FlipTouchdown_04148A:
        addi.w  #0xffe0, 0x22(a6)               | +000  x -= $20
        addi.w  #0xfff8, 0x24(a6)               | +006  y -= 8
        eori.b  #1, 0x3a(a6)                    | +00c  invierte flip
        jsr     0x4aad2.l                       | +012  efecto 1
        eori.b  #1, 0x3a(a6)                    | +018  restaura flip
        jsr     0x4aad2.l                       | +01e  efecto 2
        subi.w  #0xffe0, 0x22(a6)               | +024  restaura x
        subi.w  #0xfff8, 0x24(a6)               | +02a  restaura y
        jsr     0x267e2.l                       | +030  relink de sprite
        move.w  #0x21c, 0x2a(a6)                | +036  rebote velY
        move.w  #0xffdc, 0x2e(a6)               | +03c  gravedad = -$24
        lea     0x28732c.l, a0                  | +042  template de voltereta
        jsr     0x28cd4.l                       | +048
        lea     .Lflip_loop(pc), a1             | +04e
        move.l  a1, (a6)                        | +052
.Lflip_loop:                                    |       (= $0414DE, per-frame)
        jsr     0x27d50.l                       | +054  fisica con gravedad
        bcc.w   .Lflip_air                      | +05a  sin contacto
        bset    #1, 0x12(a6)                    | +05e  "apoyado"
        move.l  #0x285dbe, 0x48(a6)             | +064  vinculo
        lea     0x28735e.l, a0                  | +06c  template en suelo
        jsr     0x28cd4.l                       | +072
        lea     .Lflip_ground(pc), a1           | +078
        move.l  a1, (a6)                        | +07c
.Lflip_ground:                                  |       (= $041508, per-frame)
        jsr     0x2783a.l                       | +07e  fisica simple
.Lflip_air:
        jsr     0x28d70.l                       | +084  limites
        jsr     0x2870a.l                       | +08a  dano recibido ?
        bcc.w   JsrAbsRts_041558                | +090  no: rts de la isla
        bclr    #3, 0x13(a6)                    | +094
        move.w  #0x103f, d0                     | +09a  sonido de impacto
        jsr     0x2352.l                        | +09e
        addi.w  #0xffe0, 0x22(a6)               | +0a4  mismo truco de offsets
        addi.w  #0xfff8, 0x24(a6)               | +0aa
        jsr     0x4aae0.l                       | +0b0  efecto de dano
        subi.w  #0xffe0, 0x22(a6)               | +0b6
        subi.w  #0xfff8, 0x24(a6)               | +0bc
        lea     0x287372.l, a0                  | +0c2  template final
        | --- cae en JsrAbsThunk_041552 (isla): jsr $28CD4; rts --------------

| ----------------------------------------------------------------------------
|  SquadChild_TouchdownIdle_04155A  @ $04155A  (28 B)
|
|    /* Espera en el suelo: relink + fisica simple; MIENTRAS la senal
|       +0x21 valga 1 sale por SetHandlerRts_04157c; en cuanto cambia
|       cae en SetTaskHandler_041576 que instala JmpToScheduler_04157E
|       (jmp $518 = re-encolar en el scheduler). */
|    void SquadChild_TouchdownIdle(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.SquadChild_TouchdownIdle_04155A, "ax", @progbits
        .global SquadChild_TouchdownIdle_04155A
SquadChild_TouchdownIdle_04155A:
        jsr     0x267e2.l                       | +000  relink de sprite
        lea     .Ltidle_loop(pc), a1            | +006
        move.l  a1, (a6)                        | +00a
.Ltidle_loop:                                   |       (= $041566, per-frame)
        jsr     0x2783a.l                       | +00c  fisica simple
        cmpi.b  #1, 0x21(a6)                    | +012  senal aun activa ?
        beq.w   SetHandlerRts_04157c            | +018  si: rts de la isla
        | --- cae en SetTaskHandler_041576 (isla): instala $4157E y rts ------

| ----------------------------------------------------------------------------
|  SquadChild_DieToScheduler_041586  @ $041586  (28 B)
|
|    /* Muerte "ruidosa": flags $D000, capa $1C y salto a la cola de
|       muerte $77FD6 (con animacion de baja). */
|    void SquadChild_DieToScheduler(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.SquadChild_DieToScheduler_041586, "ax", @progbits
        .global SquadChild_DieToScheduler_041586
SquadChild_DieToScheduler_041586:
        move.w  #0xd000, d0                     | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3, 0x38(a6)               | +00a
        ori.w   #0x1c, 0x38(a6)                 | +010  capa $1C
        jmp     0x77fd6.l                       | +016  cola de muerte A

| ----------------------------------------------------------------------------
|  SquadChild_DespawnNoLink_0415A2  @ $0415A2  (36 B)
|
|    /* Despawn silencioso: flags $4000, `ori.w #0,0x38(a6)` MUERTO
|       (capa 0 explicita - plantilla editada), suelta el vinculo
|       +0x48 = -1 y salta a la cola $77F6A (sin animacion). */
|    void SquadChild_DespawnNoLink(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.SquadChild_DespawnNoLink_0415A2, "ax", @progbits
        .global SquadChild_DespawnNoLink_0415A2
SquadChild_DespawnNoLink_0415A2:
        move.w  #0x4000, d0                     | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3, 0x38(a6)               | +00a
        ori.w   #0, 0x38(a6)                    | +010  OP MUERTO (capa 0)
        move.l  #-1, 0x48(a6)                   | +016  suelta vinculo
        jmp     0x77f6a.l                       | +01e  cola de muerte B

| ----------------------------------------------------------------------------
|  PairChild_HandlerA_0415C6  @ $0415C6  (88 B)
|
|    /* Hijo A del par escoltado (SquadPair_Spawn*): capa $14, sonido
|       $B3, template $288200. La entrada secundaria GLOBAL
|       PairChildA_InstallRun_0415F2 (usada por TrioChild_HandlerA via
|       bra) instala el bucle: fisica simple + limites (fuera ->
|       instala el handler de muerte $40EF2) + release $5DD56 con
|       a0 = -1; sin release sale por SetHandlerRts_041624. */
|    void PairChild_HandlerA(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.PairChild_HandlerA_0415C6, "ax", @progbits
        .global PairChild_HandlerA_0415C6
        .global PairChildA_InstallRun_0415F2
PairChild_HandlerA_0415C6:
        move.w  #0xd000, d0                     | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3, 0x38(a6)               | +00a
        ori.w   #0x14, 0x38(a6)                 | +010  capa $14
        move.w  #0xb3, d1                       | +016  sonido
        jsr     0x236e.l                        | +01a
        lea     0x288200.l, a0                  | +020  template
        jsr     0x28cd4.l                       | +026
PairChildA_InstallRun_0415F2:                   |       (= $0415F2, global)
        lea     .Lpaira_loop(pc), a1            | +02c
        move.l  a1, (a6)                        | +030
.Lpaira_loop:                                   |       (= $0415F8, per-frame)
        jsr     0x2783a.l                       | +032  fisica simple
        jsr     0x28d70.l                       | +038  limites
        bcc.w   .Lpaira_in                      | +03e  dentro
        lea     Jsr5B6ThenJmpScheduler_040ef2(pc), a1 | +042  handler de muerte
        move.l  a1, (a6)                        | +046
.Lpaira_in:
        movea.l #-1, a0                         | +048
        jsr     0x5dd56.l                       | +04e  release de tarea ?
        bcc.w   SetHandlerRts_041624            | +054  no: rts de la isla
        | --- cae en SetTaskHandler_04161e (isla): instala $40EF2 y rts ------

| ----------------------------------------------------------------------------
|  PairChild_HandlerB_041626  @ $041626  (308 B)  - la mas grande del wave
|
|    /* Hijo B del par escoltado. Setup: vida +0x36 medida con $799DE
|       sobre el template elegido - si +0x88 == 0 usa el par espejado
|       $2BBBC6/$2BBC48 (segun el +0x7C DEL LIDER, doble consulta via
|       `movea.l 0xc(a6),a1`), si no $2BA052. Velocidad por heading:
|       tabla $2861EC[+0x76 * 2] -> Squad_SinCosVelocity. Sub-sprites
|       +0x4C = $285FB6 y +0x48 = $285E12, vida visual +0x66 = $32,
|       sonido $B4, attach $283CA, template de pose opcional por la
|       tabla $2861BC[+0x76 * 4] ($FFFFFFFF = saltar, cargada con
|       `movea.l #imm` como en la parte 1), capa $14.
|       Bucle: si +0x88 != 0 -> bset 6 de +0x13 + guiado $27BC8, si no
|       guiado $27CEE (carry en ambos -> DeathCry); limites; si el
|       contador de escolta +0x7F llega a 0: detach $283D8, bit 1 de
|       +0x13 -> DeathPlain, bclr 3, colision con jugador $28758
|       (carry -> DeathCry); release $5DD56 con a0 = -1; sin release
|       sale por SetHandlerRts_041760. */
|    void PairChild_HandlerB(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.PairChild_HandlerB_041626, "ax", @progbits
        .global PairChild_HandlerB_041626
PairChild_HandlerB_041626:
        bset    #4, 0x6b(a6)                    | +000
        tst.b   0x88(a6)                        | +006  variante ?
        bne.w   .Lpairb_alt                     | +00a
        lea     0x2bbbc6.l, a0                  | +00e  template (normal)
        movea.l 0xc(a6), a1                     | +014  lider
        cmpi.b  #0xff, 0x7c(a1)                 | +018  lider espejado ?
        bne.w   .Lpairb_meas                    | +01e
        lea     0x2bbc48.l, a0                  | +022  template (espejo)
.Lpairb_meas:
        jsr     0x799de.l                       | +028  mide -> d0
        bra.w   .Lpairb_store                   | +02e
.Lpairb_alt:
        lea     0x2ba052.l, a0                  | +032  template alternativo
        jsr     0x799de.l                       | +038
.Lpairb_store:
        move.w  d0, 0x36(a6)                    | +03e  vida
        lea     0x2861ec.l, a0                  | +042  tabla de headings
        move.w  0x76(a6), d1                    | +048
        add.w   d1, d1                          | +04c
        move.w  (a0,d1.w), d0                   | +04e  angulo
        jsr     Squad_SinCosVelocity_0420A4(pc) | +052  velocidad sin/cos
        move.l  #0x285fb6, 0x4c(a6)             | +056  sub-sprite alto
        move.l  #0x285e12, 0x48(a6)             | +05e  sub-sprite bajo
        move.w  #0x32, 0x66(a6)                 | +066  vida visual
        move.w  #0xb4, d1                       | +06c  sonido
        jsr     0x236e.l                        | +070
        jsr     0x283ca.l                       | +076  attach
        move.w  0x76(a6), d0                    | +07c  heading
        movea.l #0x2861bc, a0                   | +080  tabla de poses
        lsl.w   #2, d0                          | +086
        movea.l (a0,d0.w), a0                   | +088
        cmpa.l  #-1, a0                         | +08c  $FFFFFFFF = sin pose
        beq.w   .Lpairb_notmpl                  | +092
        jsr     0x28cd4.l                       | +096
.Lpairb_notmpl:
        move.w  #0xd000, d0                     | +09c
        jsr     0x28134.l                       | +0a0
        andi.w  #0xffe3, 0x38(a6)               | +0a6
        ori.w   #0x14, 0x38(a6)                 | +0ac  capa $14
        lea     .Lpairb_loop(pc), a1            | +0b2
        move.l  a1, (a6)                        | +0b6
.Lpairb_loop:                                   |       (= $0416DE, per-frame)
        tst.b   0x88(a6)                        | +0b8  variante ?
        beq.w   .Lpairb_free                    | +0bc
        bset    #6, 0x13(a6)                    | +0c0
        jsr     0x27bc8.l                       | +0c6  guiado A
        bcc.w   .Lpairb_alive1                  | +0cc
        lea     PairChild_DeathCry_041762(pc), a1 | +0d0
        move.l  a1, (a6)                        | +0d4
.Lpairb_alive1:
        bra.w   .Lpairb_post                    | +0d6
.Lpairb_free:
        jsr     0x27cee.l                       | +0da  guiado B
        bcc.w   .Lpairb_post                    | +0e0
        lea     PairChild_DeathCry_041762(pc), a1 | +0e4
        move.l  a1, (a6)                        | +0e8
.Lpairb_post:
        jsr     0x28d70.l                       | +0ea  limites
        tst.b   0x7f(a6)                        | +0f0  escolta activa ?
        bne.w   .Lpairb_release                 | +0f4
        jsr     0x283d8.l                       | +0f8  detach
        btst    #1, 0x13(a6)                    | +0fe  matado por script ?
        beq.w   .Lpairb_nokill                  | +104
        lea     PairChild_DeathPlain_041788(pc), a1 | +108
        move.l  a1, (a6)                        | +10c
.Lpairb_nokill:
        bclr    #3, 0x13(a6)                    | +10e
        jsr     0x28758.l                       | +114  colision con jugador
        bcc.w   .Lpairb_release                 | +11a
        lea     PairChild_DeathCry_041762(pc), a1 | +11e
        move.l  a1, (a6)                        | +122
.Lpairb_release:
        movea.l #-1, a0                         | +124
        jsr     0x5dd56.l                       | +12a  release de tarea ?
        bcc.w   SetHandlerRts_041760            | +130  no: rts de la isla
        | --- cae en SetTaskHandler_04175a (isla): instala $40EF2 y rts ------

| ----------------------------------------------------------------------------
|  PairChild_DeathCry_041762  @ $041762  (74 B)
|
|    /* Muerte con grito: sonido $1022; si la escolta sigue activa
|       (+0x7F != 0) fija el sub-sprite +0x4C = $28605E y hace
|       attach + detach (refresco). Entrada secundaria GLOBAL
|       PairChild_DeathPlain_041788: suelta el vinculo, flags $4000,
|       `ori.w #0` MUERTO (capa 0) y salta a la cola $77FD6. */
|    void PairChild_DeathCry(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.PairChild_DeathCry_041762, "ax", @progbits
        .global PairChild_DeathCry_041762
        .global PairChild_DeathPlain_041788
PairChild_DeathCry_041762:
        move.w  #0x1022, d0                     | +000  grito
        jsr     0x2352.l                        | +004
        tst.b   0x7f(a6)                        | +00a  escolta activa ?
        beq.w   PairChild_DeathPlain_041788     | +00e
        move.l  #0x28605e, 0x4c(a6)             | +012  sub-sprite de muerte
        jsr     0x283ca.l                       | +01a  attach
        jsr     0x283d8.l                       | +020  detach
PairChild_DeathPlain_041788:                    |       (= $041788, global)
        move.l  #-1, 0x48(a6)                   | +026  suelta vinculo
        move.w  #0x4000, d0                     | +02e
        jsr     0x28134.l                       | +032
        andi.w  #0xffe3, 0x38(a6)               | +038
        ori.w   #0, 0x38(a6)                    | +03e  OP MUERTO (capa 0)
        jmp     0x77fd6.l                       | +044  cola de muerte A

| ----------------------------------------------------------------------------
|  SquadChild_DropSpawnAtTop_0417AC  @ $0417AC  (104 B)
|
|    /* Spawner de caida desde el borde superior: template $28760C,
|       jitter X aleatorio (RNG & 7 - 4), coordenada Y de aparicion
|       calculada con $440D0 sobre ($F80, $560 - $280). El release
|       $5DD56 usa aqui un `bcs.w Jsr5B6ThenJmpScheduler_040ef2`
|       DIRECTO (branch externo a la funcion C ya matcheada, unico en
|       el cluster). Despues capa $14, sonido $193 y cae en
|       SetTaskHandler_041814 que instala DropRun. */
|    void SquadChild_DropSpawnAtTop(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.SquadChild_DropSpawnAtTop_0417AC, "ax", @progbits
        .global SquadChild_DropSpawnAtTop_0417AC
SquadChild_DropSpawnAtTop_0417AC:
        bset    #4, 0x6b(a6)                    | +000
        lea     0x28760c.l, a0                  | +006  template de caida
        jsr     0x28cd4.l                       | +00c
        jsr     0x5e9b6.l                       | +012  RNG -> d0
        andi.w  #7, d0                          | +018
        subq.w  #4, d0                          | +01c  jitter -4..+3
        add.w   d0, 0x22(a6)                    | +01e  aplica a X
        move.w  #0xf80, d0                      | +022
        move.w  #0x560, d1                      | +026
        subi.w  #0x280, d1                      | +02a
        jsr     0x440d0.l                       | +02e  conversion -> d1
        move.w  d1, 0x24(a6)                    | +034  Y de aparicion
        movea.l #-1, a0                         | +038
        jsr     0x5dd56.l                       | +03e  release de tarea ?
        bcs.w   Jsr5B6ThenJmpScheduler_040ef2   | +044  si: BRANCH DIRECTO
        move.w  #0xd000, d0                     | +048
        jsr     0x28134.l                       | +04c
        andi.w  #0xffe3, 0x38(a6)               | +052
        ori.w   #0x14, 0x38(a6)                 | +058  capa $14
        move.w  #0x193, d1                      | +05e  sonido
        jsr     0x236e.l                        | +062
        | --- cae en SetTaskHandler_041814 (isla): instala $4181C y rts ------

| ----------------------------------------------------------------------------
|  SquadChild_DropRun_04181C  @ $04181C  (44 B)
|
|    /* Per-frame de la caida (instalado por la isla $041814): fisica
|       simple, limites (fuera -> handler de muerte $40EF2), detach
|       $283D8 y release $5DD56; sin release sale por
|       SetHandlerRts_04184e. */
|    void SquadChild_DropRun(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.SquadChild_DropRun_04181C, "ax", @progbits
        .global SquadChild_DropRun_04181C
SquadChild_DropRun_04181C:
        jsr     0x2783a.l                       | +000  fisica simple
        jsr     0x28d70.l                       | +006  limites
        bcc.w   .Ldrop_in                       | +00c  dentro
        lea     Jsr5B6ThenJmpScheduler_040ef2(pc), a1 | +010  handler de muerte
        move.l  a1, (a6)                        | +014
.Ldrop_in:
        jsr     0x283d8.l                       | +016  detach
        movea.l #-1, a0                         | +01c
        jsr     0x5dd56.l                       | +022  release de tarea ?
        bcc.w   SetHandlerRts_04184e            | +028  no: rts de la isla
        | --- cae en SetTaskHandler_041848 (isla): instala $40EF2 y rts ------

| ----------------------------------------------------------------------------
|  SquadChild_ZigzagFall_041850  @ $041850  (194 B)
|
|    /* Caida en zigzag por tabla: RNG & 3 elige un puntero en
|       $2861F8; el registro de 6 bytes ((+0x34 & 15) * 6, calculado
|       con la secuencia d0=d0*2+d1; d0*=2; ext.l; adda.l) da velX
|       (+0x28) y amplitud*2 (+0x2E); velY = 0 y contador +0x70 = 0.
|       Sonido $9B, flags $4000 con capa $1C, template $287684.
|       Bucle $27D50 (contacto -> handler de muerte $40EF2); el
|       contador +0x70 regula el test de limites: < 8 siempre, 8..14
|       solo en frames impares (btst #0), >= 15 -> muerte. Release
|       $5DD56; sin release sale por SetHandlerRts_041918. */
|    void SquadChild_ZigzagFall(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.SquadChild_ZigzagFall_041850, "ax", @progbits
        .global SquadChild_ZigzagFall_041850
SquadChild_ZigzagFall_041850:
        jsr     0x5e9b6.l                       | +000  RNG -> d0
        andi.w  #3, d0                          | +006
        lsl.w   #2, d0                          | +00a
        lea     0x2861f8.l, a0                  | +00c  tabla de patrones
        movea.l (a0,d0.w), a0                   | +012
        move.w  0x34(a6), d0                    | +016  indice de registro
        andi.w  #0xf, d0                        | +01a
        move.w  d0, d1                          | +01e
        add.w   d0, d0                          | +020  d0 = 2n
        add.w   d1, d0                          | +022  d0 = 3n
        add.w   d0, d0                          | +024  d0 = 6n
        ext.l   d0                              | +026
        adda.l  d0, a0                          | +028
        move.w  (a0), 0x28(a6)                  | +02a  velX del registro
        move.w  0x2(a0), d0                     | +02e  amplitud
        add.w   d0, d0                          | +032
        move.w  d0, 0x2e(a6)                    | +034  amplitud * 2
        clr.w   0x2a(a6)                        | +038  velY = 0
        clr.w   0x70(a6)                        | +03c  contador = 0
        move.w  #0x9b, d1                       | +040  sonido
        jsr     0x236e.l                        | +044
        move.w  #0x4000, d0                     | +04a
        jsr     0x28134.l                       | +04e
        andi.w  #0xffe3, 0x38(a6)               | +054
        ori.w   #0x1c, 0x38(a6)                 | +05a  capa $1C
        lea     0x287684.l, a0                  | +060  template de zigzag
        jsr     0x28cd4.l                       | +066
        lea     .Lzig_loop(pc), a1              | +06c
        move.l  a1, (a6)                        | +070
.Lzig_loop:                                     |       (= $0418C2, per-frame)
        jsr     0x27d50.l                       | +072  fisica con gravedad
        bcc.w   .Lzig_air                       | +078  sin contacto
        lea     Jsr5B6ThenJmpScheduler_040ef2(pc), a1 | +07c
        move.l  a1, (a6)                        | +080
.Lzig_air:
        addq.w  #1, 0x70(a6)                    | +082  contador++
        cmpi.w  #8, 0x70(a6)                    | +086
        blt.w   .Lzig_limits                    | +08c  < 8: siempre testea
        cmpi.w  #0xf, 0x70(a6)                  | +090
        blt.w   .Lzig_odd                       | +096  8..14: frames impares
        lea     Jsr5B6ThenJmpScheduler_040ef2(pc), a1 | +09a  >= 15: muerte
        move.l  a1, (a6)                        | +09e
.Lzig_odd:
        move.w  0x70(a6), d0                    | +0a0
        btst    #0, d0                          | +0a4  frame impar ?
        beq.w   .Lzig_rel                       | +0a8  no: sin test
.Lzig_limits:
        jsr     0x28d70.l                       | +0ac  test de limites
.Lzig_rel:
        movea.l #-1, a0                         | +0b2
        jsr     0x5dd56.l                       | +0b8  release de tarea ?
        bcc.w   SetHandlerRts_041918            | +0be  no: rts de la isla
        | --- cae en SetTaskHandler_041912 (isla): instala $40EF2 y rts ------

| ----------------------------------------------------------------------------
|  SquadChild_GlideAttack_04191A  @ $04191A  (156 B)
|
|    /* Ataque en planeo: sonido $99, init $138FE con +0x1C = $15,
|       velY = -$200 con gravedad -$40, vida +0x66 medida sobre el
|       template $2BBFF6, template de anim $2876AE. Bucle $27D50
|       (contacto -> GlideDeath), limites, detach; bit 1 de +0x13 ->
|       DespawnNoLink; bclr 3 + colision $28758 (carry -> GlideDeath);
|       release $5DD56; sin release sale por SetHandlerRts_0419bc. */
|    void SquadChild_GlideAttack(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.SquadChild_GlideAttack_04191A, "ax", @progbits
        .global SquadChild_GlideAttack_04191A
SquadChild_GlideAttack_04191A:
        bset    #4, 0x6b(a6)                    | +000
        move.w  #0x99, d1                       | +006  sonido
        jsr     0x236e.l                        | +00a
        move.w  #0x15, 0x1c(a6)                 | +010
        jsr     0x138fe.l                       | +016  init subsistema
        move.w  #0xfe00, 0x2a(a6)               | +01c  velY = -$200
        move.w  #0xffc0, 0x2e(a6)               | +022  gravedad = -$40
        lea     0x2bbff6.l, a0                  | +028  template a medir
        jsr     0x799de.l                       | +02e  mide -> d0
        move.w  d0, 0x66(a6)                    | +034  vida
        lea     0x2876ae.l, a0                  | +038  template de planeo
        jsr     0x28cd4.l                       | +03e
        lea     .Lglide_loop(pc), a1            | +044
        move.l  a1, (a6)                        | +048
.Lglide_loop:                                   |       (= $041964, per-frame)
        jsr     0x27d50.l                       | +04a  fisica con gravedad
        bcc.w   .Lglide_air                     | +050  sin contacto
        lea     SquadChild_GlideDeath_0419BE(pc), a1 | +054
        move.l  a1, (a6)                        | +058
.Lglide_air:
        jsr     0x28d70.l                       | +05a  limites
        jsr     0x283d8.l                       | +060  detach
        btst    #1, 0x13(a6)                    | +066  matado por script ?
        beq.w   .Lglide_nokill                  | +06c
        lea     SquadChild_DespawnNoLink_0415A2(pc), a1 | +070
        move.l  a1, (a6)                        | +074
.Lglide_nokill:
        bclr    #3, 0x13(a6)                    | +076
        jsr     0x28758.l                       | +07c  colision con jugador
        bcc.w   .Lglide_rel                     | +082
        lea     SquadChild_GlideDeath_0419BE(pc), a1 | +086
        move.l  a1, (a6)                        | +08a
.Lglide_rel:
        movea.l #-1, a0                         | +08c
        jsr     0x5dd56.l                       | +092  release de tarea ?
        bcc.w   SetHandlerRts_0419bc            | +098  no: rts de la isla
        | --- cae en SetTaskHandler_0419b6 (isla): instala $40EF2 y rts ------

| ----------------------------------------------------------------------------
|  SquadChild_GlideDeath_0419BE  @ $0419BE  (14 B)
|
|    /* Grito $1022 y despawn silencioso. */
|    void SquadChild_GlideDeath(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.SquadChild_GlideDeath_0419BE, "ax", @progbits
        .global SquadChild_GlideDeath_0419BE
SquadChild_GlideDeath_0419BE:
        move.w  #0x1022, d0                     | +000  grito
        jsr     0x2352.l                        | +004
        bra.w   SquadChild_DespawnNoLink_0415A2 | +00a

| ----------------------------------------------------------------------------
|  TrioChild_HandlerA_0419CC  @ $0419CC  (48 B)
|
|    /* Hijo A del trio (TrioSpawner_PatternedPair): capa $1C, sonido
|       $B3, template $287708 y salto a PairChildA_InstallRun_0415F2 -
|       comparte TODO el bucle per-frame con PairChild_HandlerA. */
|    void TrioChild_HandlerA(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.TrioChild_HandlerA_0419CC, "ax", @progbits
        .global TrioChild_HandlerA_0419CC
TrioChild_HandlerA_0419CC:
        move.w  #0xd000, d0                     | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3, 0x38(a6)               | +00a
        ori.w   #0x1c, 0x38(a6)                 | +010  capa $1C
        move.w  #0xb3, d1                       | +016  sonido
        jsr     0x236e.l                        | +01a
        lea     0x287708.l, a0                  | +020  template
        jsr     0x28cd4.l                       | +026
        bra.w   PairChildA_InstallRun_0415F2    | +02c  comparte el bucle

| ----------------------------------------------------------------------------
|  TrioChild_HandlerB_0419FC  @ $0419FC  (176 B)
|
|    /* Hijo B del trio con vuelo orbital: +0x34 = 0, +0x36 = $200,
|       capa $14, sonido $9C, template $2877AA; parametro orbital
|       +0x90 (long) desde la tabla $28632C[(+0x9A) & 3], fase
|       +0x94/+0x96 a cero. Bucle: paso orbital $78F8A (carry ->
|       OrbitTracker), sombreado Entity_ShadeBySine + Entity_SineToStep
|       (Wave TT, pc-rel), limites. Cola comun TrioChildB_RunTail_041A7A
|       (entrada GLOBAL, tambien destino del bra de OrbitTracker):
|       doble deref del lider (`movea.l 0xc(a6),a0; movea.l 0xc(a0),a0`)
|       para incrementar el latido +0x92 del ABUELO y testear su bit 0
|       de +0x13 (-> DespawnNoLink); despues el DEAD STORE
|       `movea.l #-1,a0` pisado por `lea $28610A.l,a0` y el release
|       $5DD56; sin release sale por SetHandlerRts_041ab2. */
|    void TrioChild_HandlerB(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.TrioChild_HandlerB_0419FC, "ax", @progbits
        .global TrioChild_HandlerB_0419FC
        .global TrioChildB_RunTail_041A7A
TrioChild_HandlerB_0419FC:
        bset    #4, 0x6b(a6)                    | +000
        clr.w   0x34(a6)                        | +006
        move.w  #0x200, 0x36(a6)                | +00a  vida
        move.w  #0xd000, d0                     | +010
        jsr     0x28134.l                       | +014
        andi.w  #0xffe3, 0x38(a6)               | +01a
        ori.w   #0x14, 0x38(a6)                 | +020  capa $14
        move.w  #0x9c, d1                       | +026  sonido
        jsr     0x236e.l                        | +02a
        lea     0x2877aa.l, a0                  | +030  template
        jsr     0x28cd4.l                       | +036
        move.w  0x9a(a6), d0                    | +03c  selector orbital
        andi.w  #3, d0                          | +040
        lea     0x28632c.l, a0                  | +044  tabla de parametros
        lsl.w   #2, d0                          | +04a
        move.l  (a0,d0.w), 0x90(a6)             | +04c  parametro orbital
        clr.w   0x94(a6)                        | +052  fase = 0
        clr.b   0x96(a6)                        | +056
        lea     .Ltriob_loop(pc), a1            | +05a
        move.l  a1, (a6)                        | +05e
.Ltriob_loop:                                   |       (= $041A5C, per-frame)
        jsr     0x78f8a.l                       | +060  paso orbital
        bcc.w   .Ltriob_fly                     | +066  sigue en vuelo
        lea     TrioChild_OrbitTracker_041AB4(pc), a1 | +06a
        move.l  a1, (a6)                        | +06e
.Ltriob_fly:
        jsr     Entity_ShadeBySine_042282(pc)   | +070  sombreado (Wave TT)
        jsr     Entity_SineToStep_0422CA(pc)    | +074  paso de anim
        jsr     0x28d70.l                       | +078  limites
TrioChildB_RunTail_041A7A:                      |       (= $041A7A, global)
        movea.l 0xc(a6), a0                     | +07e  lider
        movea.l 0xc(a0), a0                     | +082  ABUELO (doble deref)
        addq.b  #1, 0x92(a0)                    | +086  latido del abuelo
        btst    #0, 0x13(a0)                    | +08a  abuelo abortando ?
        beq.w   .Ltriob_alive                   | +090
        lea     SquadChild_DespawnNoLink_0415A2(pc), a1 | +094
        move.l  a1, (a6)                        | +098
.Ltriob_alive:
        movea.l #-1, a0                         | +09a  DEAD STORE (pisado)
        lea     0x28610a.l, a0                  | +0a0  puntero real
        jsr     0x5dd56.l                       | +0a6  release de tarea ?
        bcc.w   SetHandlerRts_041ab2            | +0ac  no: rts de la isla
        | --- cae en SetTaskHandler_041aac (isla): instala $40EF2 y rts ------

| ----------------------------------------------------------------------------
|  TrioChild_OrbitTracker_041AB4  @ $041AB4  (260 B)
|
|    /* Fase de seguimiento orbital: fase +0x76 = $80, sombras
|       +0x32/+0x33 = $FF, vida +0x66 medida sobre $2BC078; indice de
|       orbita medido sobre el par espejado $2BC1FE/$2BC280 (por el
|       +0x7C del LIDER) e indexado *4 en $2BC302 -> amplitud +0x36 y
|       periodo +0x72; objetivo $5E1EA -> +0x9C; timeout +0x88
|       INICIALIZADO CON `move.w #0,0x88(a6)` (encoding 3d7c, NO
|       clr.w - verificado en ROM); template $287B1C.
|       Bucle: +0x88++ contra el timeout $B4; al vencer el periodo
|       +0x70: angulo hacia el objetivo ($5E070) -> +0x78, giro
|       Squad_PhaseStepToTarget + velocidad Squad_SinCosVelocity
|       (Wave TT, pc-rel). Guiado $27CEE (carry -> OrbitDeath);
|       +0x34 = +0x76 >> 3 (profundidad desde la fase); limites,
|       detach, bit 1 -> DespawnNoLink, bclr 3, colision $28758
|       (carry -> OrbitDeath) y salto a TrioChildB_RunTail_041A7A
|       (cola comun compartida, con su dead store). */
|    void TrioChild_OrbitTracker(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.TrioChild_OrbitTracker_041AB4, "ax", @progbits
        .global TrioChild_OrbitTracker_041AB4
TrioChild_OrbitTracker_041AB4:
        clr.w   0x70(a6)                        | +000  contador de periodo
        move.w  #0x80, 0x76(a6)                 | +004  fase inicial
        move.b  #0xff, 0x32(a6)                 | +00a  sombra A
        move.b  #0xff, 0x33(a6)                 | +010  sombra B
        lea     0x2bc078.l, a0                  | +016  template de vida
        jsr     0x799de.l                       | +01c  mide -> d0
        move.w  d0, 0x66(a6)                    | +022  vida
        lea     0x2bc1fe.l, a0                  | +026  template (normal)
        movea.l 0xc(a6), a1                     | +02c  lider
        cmpi.b  #0xff, 0x7c(a1)                 | +030  lider espejado ?
        bne.w   .Lorbit_meas                    | +036
        lea     0x2bc280.l, a0                  | +03a  template (espejo)
.Lorbit_meas:
        jsr     0x799de.l                       | +040  mide -> indice
        lea     0x2bc302.l, a0                  | +046  tabla de orbitas
        lsl.w   #2, d0                          | +04c
        move.w  (a0,d0.w), 0x36(a6)             | +04e  amplitud
        move.w  0x2(a0,d0.w), 0x72(a6)          | +054  periodo
        jsr     0x5e1ea.l                       | +05a  adquiere objetivo
        move.l  a0, 0x9c(a6)                    | +060
        move.w  #0, 0x88(a6)                    | +064  timeout (3d7c, NO clr)
        lea     0x287b1c.l, a0                  | +06a  template orbital
        jsr     0x28cd4.l                       | +070
        lea     .Lorbit_loop(pc), a1            | +076
        move.l  a1, (a6)                        | +07a
.Lorbit_loop:                                   |       (= $041B30, per-frame)
        addq.w  #1, 0x88(a6)                    | +07c  timeout++
        cmpi.w  #0xb4, 0x88(a6)                 | +080  $B4 frames ?
        bgt.w   .Lorbit_move                    | +086  vencido: solo guiado
        subq.w  #1, 0x70(a6)                    | +08a  periodo--
        cmpi.w  #0, 0x70(a6)                    | +08e
        bgt.w   .Lorbit_move                    | +094
        move.w  0x72(a6), 0x70(a6)              | +098  recarga periodo
        movea.l 0x9c(a6), a0                    | +09e  objetivo
        jsr     0x5e070.l                       | +0a2  angulo -> d0
        move.w  d0, 0x78(a6)                    | +0a8  fase objetivo
        jsr     Squad_PhaseStepToTarget_041F48(pc) | +0ac  gira (Wave TT)
        jsr     Squad_SinCosVelocity_0420A4(pc) | +0b0  velocidad (Wave TT)
.Lorbit_move:
        jsr     0x27cee.l                       | +0b4  guiado
        bcc.w   .Lorbit_alive1                  | +0ba
        lea     TrioChild_OrbitDeath_041BB8(pc), a1 | +0be
        move.l  a1, (a6)                        | +0c2
.Lorbit_alive1:
        move.w  0x76(a6), d0                    | +0c4  fase
        lsr.w   #3, d0                          | +0c8
        move.w  d0, 0x34(a6)                    | +0ca  profundidad
        jsr     0x28d70.l                       | +0ce  limites
        jsr     0x283d8.l                       | +0d4  detach
        btst    #1, 0x13(a6)                    | +0da  matado por script ?
        beq.w   .Lorbit_nokill                  | +0e0
        lea     SquadChild_DespawnNoLink_0415A2(pc), a1 | +0e4
        move.l  a1, (a6)                        | +0e8
.Lorbit_nokill:
        bclr    #3, 0x13(a6)                    | +0ea
        jsr     0x28758.l                       | +0f0  colision con jugador
        bcc.w   .Lorbit_tail                    | +0f6
        lea     TrioChild_OrbitDeath_041BB8(pc), a1 | +0fa
        move.l  a1, (a6)                        | +0fe
.Lorbit_tail:
        bra.w   TrioChildB_RunTail_041A7A       | +100  cola comun compartida

| ----------------------------------------------------------------------------
|  TrioChild_OrbitDeath_041BB8  @ $041BB8  (14 B)
|
|    /* Grito $1022 y despawn silencioso (clon de GlideDeath). */
|    void TrioChild_OrbitDeath(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.TrioChild_OrbitDeath_041BB8, "ax", @progbits
        .global TrioChild_OrbitDeath_041BB8
TrioChild_OrbitDeath_041BB8:
        move.w  #0x1022, d0                     | +000  grito
        jsr     0x2352.l                        | +004
        bra.w   SquadChild_DespawnNoLink_0415A2 | +00a

| ----------------------------------------------------------------------------
|  SquadChild_FinalPose_041BC6  @ $041BC6  (76 B)
|
|    /* Pose final antes del despawn: sonido $98, template $288320.
|       Bucle $27D50 (contacto -> DespawnNoLink), limites (fuera ->
|       DespawnNoLink tambien) y release $5DD56; sin release sale por
|       SetHandlerRts_041c18 - el ultimo "branch a mitad de isla" del
|       cluster. */
|    void SquadChild_FinalPose(Entity *e /*a6*/);
| ----------------------------------------------------------------------------
        .section .text.SquadChild_FinalPose_041BC6, "ax", @progbits
        .global SquadChild_FinalPose_041BC6
SquadChild_FinalPose_041BC6:
        move.w  #0x98, d1                       | +000  sonido
        jsr     0x236e.l                        | +004
        lea     0x288320.l, a0                  | +00a  template final
        jsr     0x28cd4.l                       | +010
        lea     .Lfinal_loop(pc), a1            | +016
        move.l  a1, (a6)                        | +01a
.Lfinal_loop:                                   |       (= $041BE2, per-frame)
        jsr     0x27d50.l                       | +01c  fisica con gravedad
        bcc.w   .Lfinal_air                     | +022  sin contacto
        lea     SquadChild_DespawnNoLink_0415A2(pc), a1 | +026
        move.l  a1, (a6)                        | +02a
.Lfinal_air:
        jsr     0x28d70.l                       | +02c  limites
        bcc.w   .Lfinal_in                      | +032  dentro
        lea     SquadChild_DespawnNoLink_0415A2(pc), a1 | +036
        move.l  a1, (a6)                        | +03a
.Lfinal_in:
        movea.l #-1, a0                         | +03c
        jsr     0x5dd56.l                       | +042  release de tarea ?
        bcc.w   SetHandlerRts_041c18            | +048  no: rts de la isla
        | --- cae en SetTaskHandler_041c12 (isla): instala $40EF2 y rts ------
