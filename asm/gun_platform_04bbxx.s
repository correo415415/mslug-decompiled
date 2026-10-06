| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave NNNN — emplazamiento de cañón enemigo (plantillas $E8214..$E8220):
|              base con 4 variantes, escotilla, escudo, dos tiradores,
|              cañón hijo, destrucción y piezas volantes; lector del
|              stream de spawn por scroll
|  Región: $04BB9A..$04CBD4  (3,814 B, 53 entradas, 34 huecos)
| ============================================================================
|
|  A. RESUMEN
|  ----------
|  Región de 3,814 B (53 entradas; 24 B de tabla de punteros $4CB44) justo
|  después del módulo de muerte de humanos. Es una entidad compuesta: una
|  BASE ($4BB9A, variantes 0..3 en +$70 por las 4 plantillas de spawn
|  $E8214/$E8218/$E821C/$E8220) que crea hijos según tablas de variante en
|  $2902E8/$2902EC/$2902F0/$2902F4 y un CAÑÓN ($4C776) que dispara; todo
|  comparte el protocolo padre/hijo +$20 (0 idle, 1 abriendo, 2 fuego, 3
|  fuego alt, $FF muerto) y +$21 (variante de sprite / "dañado").
|   1. GunPlatform_Spawn_04bb9a (4 entradas -> +$70 = 0..3): snd $84,
|      temporizador +$1C = $19 ($138FE), prio $4000 ($28134) con bits
|      +$38 = ...01000, +$20/+$21 = 0, HP +$66 por dificultad
|      $799DE[$2BD0DA], relink $267E2, $5E7C0, registro en MissionDriver
|      $43FAC con $2902D8[variante], hijo GunPlatform_Gun_04c776 que
|      hereda +$70. GunPlatform_SpawnCrewAndIdle_04bc48: según las tablas
|      por variante crea RiderA_Init ($2902F0 == 1, +$18 px abajo) o
|      RiderB_Init (== 2), Hatch ($2902E8 != 0, con +$72 = offset Y de
|      $2902F4) y Shield ($2902EC != 0); luego cola __L04bcf8: LoadIdleTimer
|      (+$80 por dificultad [$2BD260]), +$88 = 0, bucle: scroll, sprite
|      SpriteByStateVariant($290304[+$21][+$70]), anim, TickTimer (+$80--
|      llega a 0 -> +$20 = 1 y Rearm__L04bd6c), HitCheckUnlessDead, fuera de
|      mundo $5DD5C[$290912].
|   2. Ciclo de fuego: GunPlatform_Rearm_04bd5e (LoadBurstParams: +$82 =
|      nº ráfagas [$2BD364], +$84 = intervalo [$2BD2E2], +$86 = 0; 30
|      frames con +$20 = 1) -> Aim_04bd90 (TickFireInterval: +$86++ hasta
|      +$84 -> alterna FireA/FireB por bit0 de +$8A, +$20 = 2/3) ->
|      FireA_04be04 / FireB_04be72 ($1B frames, +$8A++, +$82--; quedan
|      ráfagas -> Rearm, si no -> __L04bcf8 idle).
|   3. Hijos: GunPlatform_Hatch_04bee0 (snd $84, prio $8000; espera +$20
|      del padre == 1 -> HatchOpen_04bf58 (sprite $290344[+$21], sigue al
|      padre con offset +$72, hereda bit7 de +$5A como bit0, prio por Y
|      $28108; padre == 2 -> HatchFire_04bff0 ($29033C) -> HatchHold_04c088
|      ($290334; padre 0 -> HatchClose_04c128 ($29034C) -> vuelve a
|      esperar)). GunPlatform_Shield_04c1b0 (snd $84, +$12 bit1, prio 0,
|      HP por dificultad [$2BD15C], sprite SpriteByParentState($29030C
|      [+$21 del padre][+$70]); padre == 2 -> ShieldFire_04c276; recibe
|      daño con PartHitCheck: flash $5E770 y, al agotar HP, marca al padre
|      +$20 = $FF). GunPlatform_RiderA/RiderB_Init_04c44a/04c2e4 (HP 1,
|      snd $38, colisión +$48 = $290C20/$290C74, prio $2000; RiderB a
|      (+$A,-$B)): Idle (sprite $28E6B0/$28E312, el del POW/soldado) ->
|      Alert ($28E6BC/$28E8F4) -> Anim -> Fire ($28E6C8/$28EC54 mientras
|      el padre está en 3) -> Return (RiderB $28E934); si $2870A les
|      golpea saltan a HumanDeath $4A154/$4A146 (variantes de caída).
|   4. GunPlatform_Gun_04c776: snd $84, +$1C = $19, offset (+$74,+$76) =
|      $290EFE[variante], tabla de ataque +$4C = $290D70, sprite $290876;
|      bucle: $5E506 (sigue al padre; si el padre tiene +$78 = "golpeado"
|      hereda bit0 de +$5A), $283CA/$283D8 ataque, padre +$21 == 1 ->
|      GunDamaged_04c832 (sprite $290882), padre muerto -> Jsr5B6Then
|      JmpScheduler, offworld $5E45A.
|   5. Destrucción: GunPlatform_HitCheck_04c6ea (+$78 = 1 al recibir golpe,
|      bclr +$13 bit3; HP < $32 y aún no dañado -> +$21 = 1, música $1027,
|      escombros $77C7E ($290EEC o $290EC8 si daño tipo 1, + $290EDA);
|      $28758 HP agotado -> Destroyed_04c5a4), HitCheckUnlessDead_04c6d4
|      (si +$20 == $FF -> RegisterKill $43FAC[$290E6C]). Destroyed_04c5a4
|      (SndByVariant, música $1023, escombros $290EA4/$290EB6, 10 frames,
|      +$20 = $FF, RegisterKill, bclr +$12 bit1, SetHandler ->
|      MarkDead/FreeClearBit1). DestroyedWithWreck_04c606 (sprite de
|      restos $290374[variante] o Destroyed si -1; hijo Wreck_04c68a con
|      prio -8 que muestra $290864). FlyingPart_04c846 / Land_04c910
|      (pieza lanzada: sprite $2908C8/$29088E, vel X por dificultad
|      [$2BD1DE] con signo del facing, snd $1CF, prio $D000, cae con
|      $27CEE hasta el suelo o bit1 de +$13 -> prio $4000, sin colisión,
|      jmp $77F6A AnimSeq). SpawnFlyingPart_04ca7c, SpawnExplosionMusic
|      _04ca58 (música $1067 + hijo $620DA 32 px arriba).
|   6. Helpers: FreeIfParentDead_04c942, SlotPrioCheck_04caa8/04cbb8,
|      SpriteByStateVariant/ByParentState (tabla[+$21][+$70]).
|   7. SpawnStream_ReadNext_04cac4 + SpawnStream_Dispatch_04cb88 (módulo
|      aparte, lo llaman $032ACA/$09B874 y $043CDA): lee el registro de 16
|      B apuntado por $1081B2 (X, Y, flags relativos a scroll $106F50/
|      $106F54, d2..d6 params); avanza cuando la cámara $106F5C alcanza
|      +$10; si $1081B2 < 0 devuelve un registro por defecto ($50,$1F8,
|      0,$10). Dispatch: si bit0 de $100001 (DIP/debug) monta con
|      $243608 vía JsrAbsThunk_04cbb0 ($5A9D6). SpriteSetPtrTbl6_04cb44 =
|      6 punteros $288D54/$28845E/$288EE6/$288980/$2890D8/$288C22 leídos
|      por Table_LoadPtrByIdxClamp6_04CB5C.
|
|  B. EVIDENCIAS
|  -------------
|  - $E8214..$E8220 (índice de plantillas de spawn) apuntan a las 4
|    entradas de GunPlatform_Spawn; $0620D6 referencia FlyingPart_04c846.
|  - Las tablas $2902E8/$EC/$F0 son bytes por variante (0/1/2) y deciden
|    qué hijos existen; $2902D8 son 4 punteros de registro MissionDriver.
|  - Los Riders usan los sprites del soldado/POW ($28E312/$28E6B0) y al
|    ser golpeados saltan a las entradas __L04a146/__L04a154 de
|    HumanDeath_PlayCryByKind (caída + grito): son humanos montados.
|  - $1081B2 lo inicializa init_entity_spawn_0018da.s; el formato de 16 B
|    coincide con las listas de spawn de misión.
|
|  C. HIPÓTESIS / DUDAS
|  --------------------
|  - "Emplazamiento de cañón" (GunPlatform) es la lectura funcional: base
|    estática con cañón hijo que dispara en ráfagas, escotilla que se
|    abre al disparar, escudo con HP propio y tiradores humanos. Podría
|    ser el búnker/torreta de la misión 2-3 o el cañón del fuerte; se
|    confirmará por los tiles ($290304..).
|  - Las variantes 0..3 difieren en registro de misión, offset del cañón
|    y qué hijos se crean; no se ha identificado cuál es cada misión.
|  - SpawnStream_* no pertenece a la entidad (lo llaman el bucle de misión
|    y el modo debug); se deja aquí por contigüidad.
|
|  D. CAMPOS DE LA ENTIDAD (a6) USADOS
|  ----------------------------------
|  +$00 handler  +$0C padre  +$12 bit1  +$13 bits 1/3  +$1C timer  +$20
|  estado (protocolo)  +$21 variante sprite/dañado  +$22/+$24 X/Y  +$28 vel
|  +$36  +$38 prio+bits  +$3A facing  +$48 colisión  +$4C ataque  +$5A
|  bit0/bit7  +$58 tipo daño  +$66 HP  +$6B bit4  +$70 variante  +$72
|  offset Y  +$74/+$76 offset cañón  +$78 golpeado  +$80 timer idle  +$82
|  ráfagas  +$84 intervalo  +$86 contador  +$88 frames  +$8A alternancia
|  +$98 param snd
|
|  E. CALLEES EXTERNOS
|  -------------------
|  $4AE alloc  $518 free  $5B6  $236E snd  $2352 música  $138FE timer
|  $267E2 relink  $2783A scroll  $27CEE  $28108 prio por Y  $28134 prio
|  $283CA/$283D8 ataque  $28758 HP agotado  $2870A impacto  $28CD4 sprite
|  $28D70 anim  $43FAC registro  $4A146/$4A154 HumanDeath  $5DD02 copia
|  pos  $5DD56/$5DD5C fuera de mundo  $5E45A  $5E506 seguir padre  $5E766/
|  $5E770 flash  $5E7C0  $620DA explosión  $77C7E escombros  $77F6A
|  AnimSeq  $799DE dificultad  $5A9D6  $100001 DIP  $1081B2 stream
|  $106F50/$106F54/$106F5C scroll  $10E39A.
|
|  Verificación: cada sección .text.<Sym> se coloca en su dirección CPU
|  absoluta y reensambla byte-exacta contra build/mslug_prom.bin
|  (MD5 816b3f74c76b3373993407615f1850fe).
| ============================================================================

        .text

| ----------------------------------------------------------------------------
|  GunPlatform_Spawn_04bb9a  @ $04BB9A  (166 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_Spawn_04bb9a, "ax", @progbits
        .global GunPlatform_Spawn_04bb9a
GunPlatform_Spawn_04bb9a:
        move.w  #0x0,0x70(a6)                   | +000
        bra.w   .L04bbbe                        | +006
        move.w  #0x1,0x70(a6)                   | +00a
        bra.w   .L04bbbe                        | +010
        move.w  #0x2,0x70(a6)                   | +014
        bra.w   .L04bbbe                        | +01a
        move.w  #0x3,0x70(a6)                   | +01e
.L04bbbe:
        move.w  #0x84,d1                        | +024
        jsr     0x236e.l                        | +028
        move.w  #0x19,0x1c(a6)                  | +02e
        jsr     0x138fe.l                       | +034
        move.w  #0x4000,d0                      | +03a
        jsr     0x28134.l                       | +03e
        andi.w  #0xffe3,0x38(a6)                | +044
        ori.w   #0x8,0x38(a6)                   | +04a
        moveq   #0,d0                           | +050
        move.b  d0,0x20(a6)                     | +052
        move.b  d0,0x21(a6)                     | +056
        lea     0x2bd0da.l,a0                   | +05a
        jsr     0x799de.l                       | +060
        move.w  d0,0x66(a6)                     | +066
        jsr     0x267e2.l                       | +06a
        jsr     0x5e7c0.l                       | +070
        lea     0x2902d8.l,a0                   | +076
        move.w  0x70(a6),d0                     | +07c
        andi.w  #0x3,d0                         | +080
        lsl.w   #0x2,d0                         | +084
        movea.l (a0,d0.w),a1                    | +086
        jsr     0x43fac.l                       | +08a
        lea     GunPlatform_Gun_04c776(pc),a1   | +090
        jsr     0x4ae.l                         | +094
        jsr     0x5dd02.l                       | +09a
        move.w  0x70(a6),0x70(a0)               | +0a0

| ----------------------------------------------------------------------------
|  GunPlatform_SpawnCrewAndIdle_04bc48  @ $04BC48  (270 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_SpawnCrewAndIdle_04bc48, "ax", @progbits
        .global GunPlatform_SpawnCrewAndIdle_04bc48
GunPlatform_SpawnCrewAndIdle_04bc48:
        lea     0x2902f0.l,a0                   | +000
        move.w  0x70(a6),d0                     | +006
        cmpi.b  #0x1,(a0,d0.w)                  | +00a
        bne.w   .L04bc76                        | +010
        lea     GunPlatform_RiderA_Init_04c44a(pc),a1 | +014
        jsr     0x4ae.l                         | +018
        jsr     0x5dd02.l                       | +01e
        addi.w  #0x18,0x24(a0)                  | +024
        bra.w   .L04bc96                        | +02a
.L04bc76:
        cmpi.b  #0x2,(a0,d0.w)                  | +02e
        bne.w   .L04bc96                        | +034
        lea     GunPlatform_RiderB_Init_04c2e4(pc),a1 | +038
        jsr     0x4ae.l                         | +03c
        jsr     0x5dd02.l                       | +042
        addi.w  #0x18,0x24(a0)                  | +048
.L04bc96:
        lea     0x2902e8.l,a0                   | +04e
        move.w  0x70(a6),d0                     | +054
        tst.b   (a0,d0.w)                       | +058
        beq.w   .L04bcd0                        | +05c
        lea     GunPlatform_Hatch_04bee0(pc),a1 | +060
        jsr     0x4ae.l                         | +064
        jsr     0x5dd02.l                       | +06a
        lea     0x2902f4.l,a1                   | +070
        move.w  0x70(a6),d1                     | +076
        move.w  d1,0x70(a0)                     | +07a
        add.w   d1,d1                           | +07e
        add.w   d1,d1                           | +080
        move.w  0x2(a1,d1.w),0x72(a0)           | +082
.L04bcd0:
        lea     0x2902ec.l,a0                   | +088
        move.w  0x70(a6),d0                     | +08e
        tst.b   (a0,d0.w)                       | +092
        beq.w   .L04bcf8                        | +096
        lea     GunPlatform_Shield_04c1b0(pc),a1 | +09a
        jsr     0x4ae.l                         | +09e
        jsr     0x5dd02.l                       | +0a4
        move.w  0x70(a6),0x70(a0)               | +0aa
        .global GunPlatform_SpawnCrewAndIdle_04bc48__L04bcf8
GunPlatform_SpawnCrewAndIdle_04bc48__L04bcf8:
.L04bcf8:
        jsr     GunPlatform_LoadIdleTimer_04ca0e(pc) | +0b0
        moveq   #0,d0                           | +0b4
        move.b  d0,0x20(a6)                     | +0b6
        move.w  d0,0x88(a6)                     | +0ba
        lea     .L04bd0c(pc),a1                 | +0be
        move.l  a1,(a6)                         | +0c2
.L04bd0c:
        jsr     0x2783a.l                       | +0c4
        lea     0x290304.l,a1                   | +0ca
        jsr     GunPlatform_SpriteByStateVariant_04c98a(pc) | +0d0
        jsr     0x28d70.l                       | +0d4
        jsr     GunPlatform_TickTimer_04c9d6(pc) | +0da
        bcc.w   .L04bd36                        | +0de
        move.b  #0x1,0x20(a6)                   | +0e2
        lea     GunPlatform_Rearm_04bd5e__L04bd6c(pc),a1 | +0e8
        move.l  a1,(a6)                         | +0ec
.L04bd36:
        clr.b   0x10e39a.l                      | +0ee
        jsr     GunPlatform_HitCheckUnlessDead_04c6d4(pc) | +0f4
        movea.l #0xffffffff,a0                  | +0f8
        lea     0x290912.l,a0                   | +0fe
        jsr     0x5dd5c.l                       | +104
        bcc.w   SetHandlerRts_04bd5c            | +10a

| ----------------------------------------------------------------------------
|  GunPlatform_Rearm_04bd5e  @ $04BD5E  (50 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_Rearm_04bd5e, "ax", @progbits
        .global GunPlatform_Rearm_04bd5e
GunPlatform_Rearm_04bd5e:
        clr.w   0x88(a6)                        | +000
        lea     GunPlatform_Aim_04bd90(pc),a1   | +004
        move.l  a1,(a6)                         | +008
        bra.w   GunPlatform_Aim_04bd90          | +00a
        .global GunPlatform_Rearm_04bd5e__L04bd6c
GunPlatform_Rearm_04bd5e__L04bd6c:
.L04bd6c:
        jsr     GunPlatform_LoadBurstParams_04ca24(pc) | +00e
        move.w  #0x1e,0x88(a6)                  | +012
        move.b  #0x1,0x20(a6)                   | +018
        lea     .L04bd82(pc),a1                 | +01e
        move.l  a1,(a6)                         | +022
.L04bd82:
        subq.w  #0x1,0x88(a6)                   | +024
        bne.w   GunPlatform_Aim_04bd90          | +028
        lea     GunPlatform_Aim_04bd90(pc),a1   | +02c
        move.l  a1,(a6)                         | +030

| ----------------------------------------------------------------------------
|  GunPlatform_Aim_04bd90  @ $04BD90  (108 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_Aim_04bd90, "ax", @progbits
        .global GunPlatform_Aim_04bd90
GunPlatform_Aim_04bd90:
        jsr     0x2783a.l                       | +000
        lea     0x290304.l,a1                   | +006
        jsr     GunPlatform_SpriteByStateVariant_04c98a(pc) | +00c
        jsr     0x28d70.l                       | +010
        tst.w   0x88(a6)                        | +016
        bne.w   .L04bddc                        | +01a
        jsr     GunPlatform_TickFireInterval_04c9ee(pc) | +01e
        bcc.w   .L04bddc                        | +022
        btst    #0x0,0x8a(a6)                   | +026
        bne.w   .L04bdd0                        | +02c
        move.b  #0x2,0x20(a6)                   | +030
        lea     GunPlatform_FireA_04be04(pc),a1 | +036
        move.l  a1,(a6)                         | +03a
        bra.w   .L04bddc                        | +03c
.L04bdd0:
        move.b  #0x3,0x20(a6)                   | +040
        lea     GunPlatform_FireB_04be72(pc),a1 | +046
        move.l  a1,(a6)                         | +04a
.L04bddc:
        clr.b   0x10e39a.l                      | +04c
        jsr     GunPlatform_HitCheckUnlessDead_04c6d4(pc) | +052
        movea.l #0xffffffff,a0                  | +056
        lea     0x290912.l,a0                   | +05c
        jsr     0x5dd5c.l                       | +062
        bcc.w   SetHandlerRts_04be02            | +068

| ----------------------------------------------------------------------------
|  GunPlatform_FireA_04be04  @ $04BE04  (102 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_FireA_04be04, "ax", @progbits
        .global GunPlatform_FireA_04be04
GunPlatform_FireA_04be04:
        move.w  #0x1b,0x88(a6)                  | +000
        lea     .L04be10(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L04be10:
        jsr     0x2783a.l                       | +00c
        lea     0x290304.l,a1                   | +012
        jsr     GunPlatform_SpriteByStateVariant_04c98a(pc) | +018
        jsr     0x28d70.l                       | +01c
        subq.w  #0x1,0x88(a6)                   | +022
        bne.w   .L04be4a                        | +026
        addq.b  #0x1,0x8a(a6)                   | +02a
        subq.b  #0x1,0x82(a6)                   | +02e
        bne.w   .L04be44                        | +032
        lea     GunPlatform_SpawnCrewAndIdle_04bc48__L04bcf8(pc),a1 | +036
        move.l  a1,(a6)                         | +03a
        bra.w   .L04be4a                        | +03c
.L04be44:
        lea     GunPlatform_Rearm_04bd5e(pc),a1 | +040
        move.l  a1,(a6)                         | +044
.L04be4a:
        clr.b   0x10e39a.l                      | +046
        jsr     GunPlatform_HitCheckUnlessDead_04c6d4(pc) | +04c
        movea.l #0xffffffff,a0                  | +050
        lea     0x290912.l,a0                   | +056
        jsr     0x5dd5c.l                       | +05c
        bcc.w   SetHandlerRts_04be70            | +062

| ----------------------------------------------------------------------------
|  GunPlatform_FireB_04be72  @ $04BE72  (102 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_FireB_04be72, "ax", @progbits
        .global GunPlatform_FireB_04be72
GunPlatform_FireB_04be72:
        move.w  #0x1b,0x88(a6)                  | +000
        lea     .L04be7e(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L04be7e:
        jsr     0x2783a.l                       | +00c
        lea     0x290304.l,a1                   | +012
        jsr     GunPlatform_SpriteByStateVariant_04c98a(pc) | +018
        jsr     0x28d70.l                       | +01c
        subq.w  #0x1,0x88(a6)                   | +022
        bne.w   .L04beb8                        | +026
        addq.b  #0x1,0x8a(a6)                   | +02a
        subq.b  #0x1,0x82(a6)                   | +02e
        bne.w   .L04beb2                        | +032
        lea     GunPlatform_SpawnCrewAndIdle_04bc48__L04bcf8(pc),a1 | +036
        move.l  a1,(a6)                         | +03a
        bra.w   .L04beb8                        | +03c
.L04beb2:
        lea     GunPlatform_Rearm_04bd5e(pc),a1 | +040
        move.l  a1,(a6)                         | +044
.L04beb8:
        clr.b   0x10e39a.l                      | +046
        jsr     GunPlatform_HitCheckUnlessDead_04c6d4(pc) | +04c
        movea.l #0xffffffff,a0                  | +050
        lea     0x290912.l,a0                   | +056
        jsr     0x5dd5c.l                       | +05c
        bcc.w   SetHandlerRts_04bede            | +062

| ----------------------------------------------------------------------------
|  GunPlatform_Hatch_04bee0  @ $04BEE0  (112 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_Hatch_04bee0, "ax", @progbits
        .global GunPlatform_Hatch_04bee0
GunPlatform_Hatch_04bee0:
        move.w  #0x84,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x19,0x1c(a6)                  | +00a
        jsr     0x138fe.l                       | +010
        move.w  #0x8000,d0                      | +016
        jsr     0x28134.l                       | +01a
        andi.w  #0xffe3,0x38(a6)                | +020
        ori.w   #0x8,0x38(a6)                   | +026
        moveq   #0,d0                           | +02c
        move.b  d0,0x20(a6)                     | +02e
        move.b  d0,0x21(a6)                     | +032
        jsr     0x267e2.l                       | +036
        .global GunPlatform_Hatch_04bee0__L04bf1c
GunPlatform_Hatch_04bee0__L04bf1c:
.L04bf1c:
        lea     .L04bf22(pc),a1                 | +03c
        move.l  a1,(a6)                         | +040
.L04bf22:
        movea.l 0xc(a6),a0                      | +042
        cmpi.b  #0x1,0x20(a0)                   | +046
        bne.w   .L04bf36                        | +04c
        lea     GunPlatform_HatchOpen_04bf58(pc),a1 | +050
        move.l  a1,(a6)                         | +054
.L04bf36:
        jsr     GunPlatform_FreeIfParentDead_04c942(pc) | +056
        movea.l #0xffffffff,a0                  | +05a
        lea     0x290912.l,a0                   | +060
        jsr     0x5dd5c.l                       | +066
        bcc.w   SetHandlerRts_04bf56            | +06c

| ----------------------------------------------------------------------------
|  GunPlatform_HatchOpen_04bf58  @ $04BF58  (144 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_HatchOpen_04bf58, "ax", @progbits
        .global GunPlatform_HatchOpen_04bf58
GunPlatform_HatchOpen_04bf58:
        move.b  0x21(a6),d0                     | +000
        andi.w  #0xff,d0                        | +004
        movea.l #0x290344,a0                    | +008
        lsl.w   #0x2,d0                         | +00e
        movea.l (a0,d0.w),a0                    | +010
        cmpa.l  #0xffffffff,a0                  | +014
        beq.w   .L04bf7c                        | +01a
        jsr     0x28cd4.l                       | +01e
.L04bf7c:
        lea     .L04bf82(pc),a1                 | +024
        move.l  a1,(a6)                         | +028
.L04bf82:
        movea.l 0xc(a6),a0                      | +02a
        move.w  0x22(a0),0x22(a6)               | +02e
        move.w  0x24(a0),0x24(a6)               | +034
        move.w  0x72(a6),d0                     | +03a
        add.w   d0,0x24(a6)                     | +03e
        jsr     0x28108.l                       | +042
        movea.l 0xc(a6),a0                      | +048
        btst    #0x7,0x5a(a0)                   | +04c
        beq.w   .L04bfb4                        | +052
        bset    #0x0,0x5a(a6)                   | +056
.L04bfb4:
        jsr     0x28d70.l                       | +05c
        movea.l 0xc(a6),a0                      | +062
        cmpi.b  #0x2,0x20(a0)                   | +066
        bne.w   .L04bfce                        | +06c
        lea     GunPlatform_HatchFire_04bff0(pc),a1 | +070
        move.l  a1,(a6)                         | +074
.L04bfce:
        jsr     GunPlatform_FreeIfParentDead_04c942(pc) | +076
        movea.l #0xffffffff,a0                  | +07a
        lea     0x290912.l,a0                   | +080
        jsr     0x5dd5c.l                       | +086
        bcc.w   SetHandlerRts_04bfee            | +08c

| ----------------------------------------------------------------------------
|  GunPlatform_HatchFire_04bff0  @ $04BFF0  (144 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_HatchFire_04bff0, "ax", @progbits
        .global GunPlatform_HatchFire_04bff0
GunPlatform_HatchFire_04bff0:
        move.b  0x21(a6),d0                     | +000
        andi.w  #0xff,d0                        | +004
        movea.l #0x29033c,a0                    | +008
        lsl.w   #0x2,d0                         | +00e
        movea.l (a0,d0.w),a0                    | +010
        cmpa.l  #0xffffffff,a0                  | +014
        beq.w   .L04c014                        | +01a
        jsr     0x28cd4.l                       | +01e
.L04c014:
        lea     .L04c01a(pc),a1                 | +024
        move.l  a1,(a6)                         | +028
.L04c01a:
        movea.l 0xc(a6),a0                      | +02a
        move.w  0x22(a0),0x22(a6)               | +02e
        move.w  0x24(a0),0x24(a6)               | +034
        move.w  0x72(a6),d0                     | +03a
        add.w   d0,0x24(a6)                     | +03e
        jsr     0x28108.l                       | +042
        movea.l 0xc(a6),a0                      | +048
        btst    #0x7,0x5a(a0)                   | +04c
        beq.w   .L04c04c                        | +052
        bset    #0x0,0x5a(a6)                   | +056
.L04c04c:
        jsr     0x28d70.l                       | +05c
        movea.l 0xc(a6),a0                      | +062
        cmpi.b  #0x2,0x20(a0)                   | +066
        beq.w   .L04c066                        | +06c
        lea     GunPlatform_HatchHold_04c088(pc),a1 | +070
        move.l  a1,(a6)                         | +074
.L04c066:
        jsr     GunPlatform_FreeIfParentDead_04c942(pc) | +076
        movea.l #0xffffffff,a0                  | +07a
        lea     0x290912.l,a0                   | +080
        jsr     0x5dd5c.l                       | +086
        bcc.w   SetHandlerRts_04c086            | +08c

| ----------------------------------------------------------------------------
|  GunPlatform_HatchHold_04c088  @ $04C088  (152 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_HatchHold_04c088, "ax", @progbits
        .global GunPlatform_HatchHold_04c088
GunPlatform_HatchHold_04c088:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x22(a0),0x22(a6)               | +004
        move.w  0x24(a0),0x24(a6)               | +00a
        move.w  0x72(a6),d0                     | +010
        add.w   d0,0x24(a6)                     | +014
        jsr     0x28108.l                       | +018
        move.b  0x21(a6),d0                     | +01e
        andi.w  #0xff,d0                        | +022
        movea.l #0x290334,a0                    | +026
        lsl.w   #0x2,d0                         | +02c
        movea.l (a0,d0.w),a0                    | +02e
        cmpa.l  #0xffffffff,a0                  | +032
        beq.w   .L04c0ca                        | +038
        jsr     0x28cd4.l                       | +03c
.L04c0ca:
        movea.l 0xc(a6),a0                      | +042
        btst    #0x7,0x5a(a0)                   | +046
        beq.w   .L04c0de                        | +04c
        bset    #0x0,0x5a(a6)                   | +050
.L04c0de:
        jsr     0x28d70.l                       | +056
        movea.l 0xc(a6),a0                      | +05c
        tst.b   0x20(a0)                        | +060
        bne.w   .L04c0f6                        | +064
        lea     GunPlatform_HatchClose_04c128(pc),a1 | +068
        move.l  a1,(a6)                         | +06c
.L04c0f6:
        cmpi.b  #0x2,0x20(a0)                   | +06e
        bne.w   .L04c106                        | +074
        lea     GunPlatform_HatchFire_04bff0(pc),a1 | +078
        move.l  a1,(a6)                         | +07c
.L04c106:
        jsr     GunPlatform_FreeIfParentDead_04c942(pc) | +07e
        movea.l #0xffffffff,a0                  | +082
        lea     0x290912.l,a0                   | +088
        jsr     0x5dd5c.l                       | +08e
        bcc.w   SetHandlerRts_04c126            | +094

| ----------------------------------------------------------------------------
|  GunPlatform_HatchClose_04c128  @ $04C128  (128 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_HatchClose_04c128, "ax", @progbits
        .global GunPlatform_HatchClose_04c128
GunPlatform_HatchClose_04c128:
        move.b  0x21(a6),d0                     | +000
        andi.w  #0xff,d0                        | +004
        movea.l #0x29034c,a0                    | +008
        lsl.w   #0x2,d0                         | +00e
        movea.l (a0,d0.w),a0                    | +010
        cmpa.l  #0xffffffff,a0                  | +014
        beq.w   .L04c14c                        | +01a
        jsr     0x28cd4.l                       | +01e
.L04c14c:
        lea     .L04c152(pc),a1                 | +024
        move.l  a1,(a6)                         | +028
.L04c152:
        movea.l 0xc(a6),a0                      | +02a
        move.w  0x22(a0),0x22(a6)               | +02e
        move.w  0x24(a0),0x24(a6)               | +034
        move.w  0x72(a6),d0                     | +03a
        add.w   d0,0x24(a6)                     | +03e
        movea.l 0xc(a6),a0                      | +042
        btst    #0x7,0x5a(a0)                   | +046
        beq.w   .L04c17e                        | +04c
        bset    #0x0,0x5a(a6)                   | +050
.L04c17e:
        jsr     0x28d70.l                       | +056
        bcc.w   .L04c18e                        | +05c
        lea     GunPlatform_Hatch_04bee0__L04bf1c(pc),a1 | +060
        move.l  a1,(a6)                         | +064
.L04c18e:
        jsr     GunPlatform_FreeIfParentDead_04c942(pc) | +066
        movea.l #0xffffffff,a0                  | +06a
        lea     0x290912.l,a0                   | +070
        jsr     0x5dd5c.l                       | +076
        bcc.w   SetHandlerRts_04c1ae            | +07c

| ----------------------------------------------------------------------------
|  GunPlatform_Shield_04c1b0  @ $04C1B0  (190 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_Shield_04c1b0, "ax", @progbits
        .global GunPlatform_Shield_04c1b0
GunPlatform_Shield_04c1b0:
        move.w  #0x84,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x19,0x1c(a6)                  | +00a
        jsr     0x138fe.l                       | +010
        bset    #0x1,0x12(a6)                   | +016
        move.w  #0x0,d0                         | +01c
        jsr     0x28134.l                       | +020
        andi.w  #0xffe3,0x38(a6)                | +026
        ori.w   #0x8,0x38(a6)                   | +02c
        moveq   #0,d0                           | +032
        move.b  d0,0x20(a6)                     | +034
        move.b  d0,0x21(a6)                     | +038
        lea     0x2bd15c.l,a0                   | +03c
        jsr     0x799de.l                       | +042
        move.w  d0,0x66(a6)                     | +048
        jsr     0x267e2.l                       | +04c
        lea     .L04c208(pc),a1                 | +052
        move.l  a1,(a6)                         | +056
        .global GunPlatform_Shield_04c1b0__L04c208
GunPlatform_Shield_04c1b0__L04c208:
.L04c208:
        movea.l 0xc(a6),a0                      | +058
        move.w  0x22(a0),0x22(a6)               | +05c
        move.w  0x24(a0),0x24(a6)               | +062
        lea     0x29030c.l,a1                   | +068
        jsr     GunPlatform_SpriteByParentState_04c9ae(pc) | +06e
        movea.l 0xc(a6),a0                      | +072
        btst    #0x7,0x5a(a0)                   | +076
        beq.w   .L04c236                        | +07c
        bset    #0x0,0x5a(a6)                   | +080
.L04c236:
        jsr     0x28d70.l                       | +086
        movea.l 0xc(a6),a0                      | +08c
        cmpi.b  #0x2,0x20(a0)                   | +090
        bne.w   .L04c250                        | +096
        lea     GunPlatform_ShieldFire_04c276(pc),a1 | +09a
        move.l  a1,(a6)                         | +09e
.L04c250:
        jsr     GunPlatform_FreeIfParentDead_04c942(pc) | +0a0
        jsr     GunPlatform_PartHitCheck_04c958(pc) | +0a4
        movea.l #0xffffffff,a0                  | +0a8
        lea     0x290912.l,a0                   | +0ae
        jsr     0x5dd5c.l                       | +0b4
        bcc.w   SetHandlerRts_04c274            | +0ba

| ----------------------------------------------------------------------------
|  GunPlatform_ShieldFire_04c276  @ $04C276  (102 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_ShieldFire_04c276, "ax", @progbits
        .global GunPlatform_ShieldFire_04c276
GunPlatform_ShieldFire_04c276:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x22(a0),0x22(a6)               | +004
        move.w  0x24(a0),0x24(a6)               | +00a
        movea.l 0xc(a6),a0                      | +010
        btst    #0x7,0x5a(a0)                   | +014
        beq.w   .L04c29a                        | +01a
        bset    #0x0,0x5a(a6)                   | +01e
.L04c29a:
        lea     0x29030c.l,a1                   | +024
        jsr     GunPlatform_SpriteByParentState_04c9ae(pc) | +02a
        jsr     0x28d70.l                       | +02e
        movea.l 0xc(a6),a0                      | +034
        cmpi.b  #0x2,0x20(a0)                   | +038
        beq.w   .L04c2be                        | +03e
        lea     GunPlatform_Shield_04c1b0__L04c208(pc),a1 | +042
        move.l  a1,(a6)                         | +046
.L04c2be:
        jsr     GunPlatform_FreeIfParentDead_04c942(pc) | +048
        jsr     GunPlatform_PartHitCheck_04c958(pc) | +04c
        movea.l #0xffffffff,a0                  | +050
        lea     0x290912.l,a0                   | +056
        jsr     0x5dd5c.l                       | +05c
        bcc.w   SetHandlerRts_04c2e2            | +062

| ----------------------------------------------------------------------------
|  GunPlatform_RiderB_Init_04c2e4  @ $04C2E4  (80 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_RiderB_Init_04c2e4, "ax", @progbits
        .global GunPlatform_RiderB_Init_04c2e4
GunPlatform_RiderB_Init_04c2e4:
        clr.b   0x20(a6)                        | +000
        clr.b   0x21(a6)                        | +004
        move.w  #0x1,0x66(a6)                   | +008
        move.w  #0x38,d1                        | +00e
        jsr     0x236e.l                        | +012
        jsr     0x5e7c0.l                       | +018
        jsr     0x267e2.l                       | +01e
        addi.w  #0xa,0x22(a6)                   | +024
        subi.w  #0xb,0x24(a6)                   | +02a
        lea     0x290c74.l,a0                   | +030
        move.l  a0,0x48(a6)                     | +036
        move.w  #0x2000,d0                      | +03a
        jsr     0x28134.l                       | +03e
        andi.w  #0xffe3,0x38(a6)                | +044
        ori.w   #0x8,0x38(a6)                   | +04a

| ----------------------------------------------------------------------------
|  GunPlatform_RiderB_Idle_04c334  @ $04C334  (52 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_RiderB_Idle_04c334, "ax", @progbits
        .global GunPlatform_RiderB_Idle_04c334
GunPlatform_RiderB_Idle_04c334:
        lea     0x28e312.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L04c346(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L04c346:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        movea.l 0xc(a6),a0                      | +01e
        tst.b   0x20(a0)                        | +022
        beq.w   .L04c364                        | +026
        lea     GunPlatform_RiderB_Alert_04c368(pc),a1 | +02a
        move.l  a1,(a6)                         | +02e
.L04c364:
        bra.w   GunPlatform_RiderB_Return_04c3ea__L04c416 | +030

| ----------------------------------------------------------------------------
|  GunPlatform_RiderB_Alert_04c368  @ $04C368  (12 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_RiderB_Alert_04c368, "ax", @progbits
        .global GunPlatform_RiderB_Alert_04c368
GunPlatform_RiderB_Alert_04c368:
        lea     0x28e8f4.l,a0                   | +000
        jsr     0x28cd4.l                       | +006

| ----------------------------------------------------------------------------
|  GunPlatform_RiderB_Anim_04c374  @ $04C374  (64 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_RiderB_Anim_04c374, "ax", @progbits
        .global GunPlatform_RiderB_Anim_04c374
GunPlatform_RiderB_Anim_04c374:
        lea     .L04c37a(pc),a1                 | +000
        move.l  a1,(a6)                         | +004
.L04c37a:
        jsr     0x2783a.l                       | +006
        jsr     0x28d70.l                       | +00c
        bcc.w   .L04c3b0                        | +012
        movea.l 0xc(a6),a0                      | +016
        tst.b   0x20(a0)                        | +01a
        bne.w   .L04c39c                        | +01e
        lea     GunPlatform_RiderB_Return_04c3ea(pc),a1 | +022
        move.l  a1,(a6)                         | +026
.L04c39c:
        movea.l 0xc(a6),a0                      | +028
        cmpi.b  #0x3,0x20(a0)                   | +02c
        bne.w   .L04c3b0                        | +032
        lea     GunPlatform_RiderB_Fire_04c3b4(pc),a1 | +036
        move.l  a1,(a6)                         | +03a
.L04c3b0:
        bra.w   GunPlatform_RiderB_Return_04c3ea__L04c416 | +03c

| ----------------------------------------------------------------------------
|  GunPlatform_RiderB_Fire_04c3b4  @ $04C3B4  (54 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_RiderB_Fire_04c3b4, "ax", @progbits
        .global GunPlatform_RiderB_Fire_04c3b4
GunPlatform_RiderB_Fire_04c3b4:
        lea     0x28ec54.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L04c3c6(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L04c3c6:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        movea.l 0xc(a6),a0                      | +01e
        cmpi.b  #0x3,0x20(a0)                   | +022
        beq.w   .L04c3e6                        | +028
        lea     GunPlatform_RiderB_Anim_04c374(pc),a1 | +02c
        move.l  a1,(a6)                         | +030
.L04c3e6:
        bra.w   GunPlatform_RiderB_Return_04c3ea__L04c416 | +032

| ----------------------------------------------------------------------------
|  GunPlatform_RiderB_Return_04c3ea  @ $04C3EA  (88 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_RiderB_Return_04c3ea, "ax", @progbits
        .global GunPlatform_RiderB_Return_04c3ea
GunPlatform_RiderB_Return_04c3ea:
        lea     0x28e934.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L04c3fc(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L04c3fc:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        bcc.w   .L04c412                        | +01e
        lea     GunPlatform_RiderB_Idle_04c334(pc),a1 | +022
        move.l  a1,(a6)                         | +026
.L04c412:
        bra.w   .L04c416                        | +028
        .global GunPlatform_RiderB_Return_04c3ea__L04c416
GunPlatform_RiderB_Return_04c3ea__L04c416:
.L04c416:
        jsr     0x2870a.l                       | +02c
        bcc.w   .L04c428                        | +032
        lea     0x4a146.l,a1                    | +036
        move.l  a1,(a6)                         | +03c
.L04c428:
        jsr     GunPlatform_FreeIfParentDead_04c942(pc) | +03e
        movea.l #0xffffffff,a0                  | +042
        lea     0x290912.l,a0                   | +048
        jsr     0x5dd5c.l                       | +04e
        bcc.w   SetHandlerRts_04c448            | +054

| ----------------------------------------------------------------------------
|  GunPlatform_RiderA_Init_04c44a  @ $04C44A  (68 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_RiderA_Init_04c44a, "ax", @progbits
        .global GunPlatform_RiderA_Init_04c44a
GunPlatform_RiderA_Init_04c44a:
        clr.b   0x20(a6)                        | +000
        clr.b   0x21(a6)                        | +004
        move.w  #0x1,0x66(a6)                   | +008
        move.w  #0x38,d1                        | +00e
        jsr     0x236e.l                        | +012
        jsr     0x5e7c0.l                       | +018
        jsr     0x267e2.l                       | +01e
        lea     0x290c20.l,a0                   | +024
        move.l  a0,0x48(a6)                     | +02a
        move.w  #0x2000,d0                      | +02e
        jsr     0x28134.l                       | +032
        andi.w  #0xffe3,0x38(a6)                | +038
        ori.w   #0x8,0x38(a6)                   | +03e

| ----------------------------------------------------------------------------
|  GunPlatform_RiderA_Idle_04c48e  @ $04C48E  (52 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_RiderA_Idle_04c48e, "ax", @progbits
        .global GunPlatform_RiderA_Idle_04c48e
GunPlatform_RiderA_Idle_04c48e:
        lea     0x28e6b0.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L04c4a0(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L04c4a0:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        movea.l 0xc(a6),a0                      | +01e
        tst.b   0x20(a0)                        | +022
        beq.w   .L04c4be                        | +026
        lea     GunPlatform_RiderA_Alert_04c4c2(pc),a1 | +02a
        move.l  a1,(a6)                         | +02e
.L04c4be:
        bra.w   GunPlatform_RiderA_Fire_04c50e__L04c544 | +030

| ----------------------------------------------------------------------------
|  GunPlatform_RiderA_Alert_04c4c2  @ $04C4C2  (12 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_RiderA_Alert_04c4c2, "ax", @progbits
        .global GunPlatform_RiderA_Alert_04c4c2
GunPlatform_RiderA_Alert_04c4c2:
        lea     0x28e6bc.l,a0                   | +000
        jsr     0x28cd4.l                       | +006

| ----------------------------------------------------------------------------
|  GunPlatform_RiderA_Anim_04c4ce  @ $04C4CE  (64 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_RiderA_Anim_04c4ce, "ax", @progbits
        .global GunPlatform_RiderA_Anim_04c4ce
GunPlatform_RiderA_Anim_04c4ce:
        lea     .L04c4d4(pc),a1                 | +000
        move.l  a1,(a6)                         | +004
.L04c4d4:
        jsr     0x2783a.l                       | +006
        jsr     0x28d70.l                       | +00c
        bcc.w   .L04c50a                        | +012
        movea.l 0xc(a6),a0                      | +016
        tst.b   0x20(a0)                        | +01a
        bne.w   .L04c4f6                        | +01e
        lea     GunPlatform_RiderA_Idle_04c48e(pc),a1 | +022
        move.l  a1,(a6)                         | +026
.L04c4f6:
        movea.l 0xc(a6),a0                      | +028
        cmpi.b  #0x3,0x20(a0)                   | +02c
        bne.w   .L04c50a                        | +032
        lea     GunPlatform_RiderA_Fire_04c50e(pc),a1 | +036
        move.l  a1,(a6)                         | +03a
.L04c50a:
        bra.w   GunPlatform_RiderA_Fire_04c50e__L04c544 | +03c

| ----------------------------------------------------------------------------
|  GunPlatform_RiderA_Fire_04c50e  @ $04C50E  (98 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_RiderA_Fire_04c50e, "ax", @progbits
        .global GunPlatform_RiderA_Fire_04c50e
GunPlatform_RiderA_Fire_04c50e:
        lea     0x28e6c8.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L04c520(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L04c520:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        movea.l 0xc(a6),a0                      | +01e
        cmpi.b  #0x3,0x20(a0)                   | +022
        beq.w   .L04c540                        | +028
        lea     GunPlatform_RiderA_Anim_04c4ce(pc),a1 | +02c
        move.l  a1,(a6)                         | +030
.L04c540:
        bra.w   .L04c544                        | +032
        .global GunPlatform_RiderA_Fire_04c50e__L04c544
GunPlatform_RiderA_Fire_04c50e__L04c544:
.L04c544:
        jsr     0x2870a.l                       | +036
        bcc.w   .L04c556                        | +03c
        lea     0x4a154.l,a1                    | +040
        move.l  a1,(a6)                         | +046
.L04c556:
        jsr     GunPlatform_FreeIfParentDead_04c942(pc) | +048
        movea.l #0xffffffff,a0                  | +04c
        lea     0x290912.l,a0                   | +052
        jsr     0x5dd5c.l                       | +058
        bcc.w   SetHandlerRts_04c576            | +05e

| ----------------------------------------------------------------------------
|  GunPlatform_MarkDead_04c578  @ $04C578  (6 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_MarkDead_04c578, "ax", @progbits
        .global GunPlatform_MarkDead_04c578
GunPlatform_MarkDead_04c578:
        move.b  #0xff,0x20(a6)                  | +000

| ----------------------------------------------------------------------------
|  GunPlatform_FreeClearBit1_04c58c  @ $04C58C  (22 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_FreeClearBit1_04c58c, "ax", @progbits
        .global GunPlatform_FreeClearBit1_04c58c
GunPlatform_FreeClearBit1_04c58c:
        btst    #0x1,0x12(a6)                   | +000
        beq.w   .L04c59c                        | +006
        bclr    #0x1,0x12(a6)                   | +00a
.L04c59c:
        jmp     0x518.l                         | +010

| ----------------------------------------------------------------------------
|  GunPlatform_Nop_04c5a2  @ $04C5A2  (2 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_Nop_04c5a2, "ax", @progbits
        .global GunPlatform_Nop_04c5a2
GunPlatform_Nop_04c5a2:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  GunPlatform_Destroyed_04c5a4  @ $04C5A4  (90 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_Destroyed_04c5a4, "ax", @progbits
        .global GunPlatform_Destroyed_04c5a4
GunPlatform_Destroyed_04c5a4:
        clr.b   0x78(a6)                        | +000
        jsr     GunPlatform_SndByVariant_04ca98(pc) | +004
        move.w  #0x1023,d0                      | +008
        jsr     0x2352.l                        | +00c
        lea     0x290ea4.l,a1                   | +012
        jsr     0x77c7e.l                       | +018
        lea     0x290eb6.l,a1                   | +01e
        jsr     0x77c7e.l                       | +024
        move.w  #0xa,0x88(a6)                   | +02a
        lea     .L04c5da(pc),a1                 | +030
        move.l  a1,(a6)                         | +034
.L04c5da:
        jsr     0x2783a.l                       | +036
        jsr     0x28d70.l                       | +03c
        subq.w  #0x1,0x88(a6)                   | +042
        bne.w   SetHandlerRts_04c604            | +046
        move.b  #0xff,0x20(a6)                  | +04a
        jsr     GunPlatform_RegisterKill_04ca4a(pc) | +050
        bclr    #0x1,0x12(a6)                   | +054

| ----------------------------------------------------------------------------
|  GunPlatform_DestroyedWithWreck_04c606  @ $04C606  (124 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_DestroyedWithWreck_04c606, "ax", @progbits
        .global GunPlatform_DestroyedWithWreck_04c606
GunPlatform_DestroyedWithWreck_04c606:
        clr.b   0x78(a6)                        | +000
        jsr     GunPlatform_SndByVariant_04ca98(pc) | +004
        bclr    #0x1,0x12(a6)                   | +008
        lea     0x290374.l,a1                   | +00e
        move.w  0x70(a6),d1                     | +014
        add.w   d1,d1                           | +018
        add.w   d1,d1                           | +01a
        movea.l (a1,d1.w),a0                    | +01c
        cmpa.l  #0xffffffff,a0                  | +020
        beq.w   GunPlatform_Destroyed_04c5a4    | +026
        jsr     0x28cd4.l                       | +02a
        lea     GunPlatform_Wreck_04c68a(pc),a1 | +030
        jsr     0x4ae.l                         | +034
        jsr     0x5dd02.l                       | +03a
        move.w  0x38(a6),d0                     | +040
        subq.w  #0x8,d0                         | +044
        move.w  d0,0x38(a0)                     | +046
        lea     .L04c656(pc),a1                 | +04a
        move.l  a1,(a6)                         | +04e
.L04c656:
        jsr     0x2783a.l                       | +050
        jsr     0x28d70.l                       | +056
        bcc.w   .L04c66c                        | +05c
        lea     GunPlatform_Wreck_04c68a(pc),a1 | +060
        move.l  a1,(a6)                         | +064
.L04c66c:
        movea.l #0xffffffff,a0                  | +066
        lea     0x290912.l,a0                   | +06c
        jsr     0x5dd5c.l                       | +072
        bcc.w   SetHandlerRts_04c688            | +078

| ----------------------------------------------------------------------------
|  GunPlatform_Wreck_04c68a  @ $04C68A  (66 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_Wreck_04c68a, "ax", @progbits
        .global GunPlatform_Wreck_04c68a
GunPlatform_Wreck_04c68a:
        clr.b   0x78(a6)                        | +000
        move.w  #0x84,d1                        | +004
        jsr     0x236e.l                        | +008
        lea     0x290864.l,a0                   | +00e
        jsr     0x28cd4.l                       | +014
        lea     .L04c6aa(pc),a1                 | +01a
        move.l  a1,(a6)                         | +01e
.L04c6aa:
        jsr     0x2783a.l                       | +020
        jsr     0x28d70.l                       | +026
        movea.l #0xffffffff,a0                  | +02c
        lea     0x290912.l,a0                   | +032
        jsr     0x5dd5c.l                       | +038
        bcc.w   SetHandlerRts_04c6d2            | +03e

| ----------------------------------------------------------------------------
|  GunPlatform_HitCheckUnlessDead_04c6d4  @ $04C6D4  (14 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_HitCheckUnlessDead_04c6d4, "ax", @progbits
        .global GunPlatform_HitCheckUnlessDead_04c6d4
GunPlatform_HitCheckUnlessDead_04c6d4:
        cmpi.b  #0xff,0x20(a6)                  | +000
        bne.w   GunPlatform_HitCheck_04c6ea     | +006
        jsr     GunPlatform_RegisterKill_04ca4a(pc) | +00a

| ----------------------------------------------------------------------------
|  GunPlatform_HitCheck_04c6ea  @ $04C6EA  (128 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_HitCheck_04c6ea, "ax", @progbits
        .global GunPlatform_HitCheck_04c6ea
GunPlatform_HitCheck_04c6ea:
        clr.b   0x78(a6)                        | +000
        jsr     0x2870a.l                       | +004
        bcc.w   .L04c75a                        | +00a
        move.b  #0x1,0x78(a6)                   | +00e
        bclr    #0x3,0x13(a6)                   | +014
        cmpi.b  #0x1,0x21(a6)                   | +01a
        beq.w   .L04c75a                        | +020
        cmpi.w  #0x32,0x66(a6)                  | +024
        bge.w   .L04c75a                        | +02a
        move.b  #0x1,0x21(a6)                   | +02e
        move.w  #0x1027,d0                      | +034
        jsr     0x2352.l                        | +038
        cmpi.b  #0x1,0x58(a6)                   | +03e
        beq.w   .L04c742                        | +044
        lea     0x290eec.l,a1                   | +048
        jsr     0x77c7e.l                       | +04e
        bra.w   .L04c74e                        | +054
.L04c742:
        lea     0x290ec8.l,a1                   | +058
        jsr     0x77c7e.l                       | +05e
.L04c74e:
        lea     0x290eda.l,a1                   | +064
        jsr     0x77c7e.l                       | +06a
.L04c75a:
        jsr     0x28758.l                       | +070
        bcc.w   ClearXN_04c770                  | +076
        lea     GunPlatform_Destroyed_04c5a4(pc),a1 | +07a
        move.l  a1,(a6)                         | +07e

| ----------------------------------------------------------------------------
|  GunPlatform_Gun_04c776  @ $04C776  (180 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_Gun_04c776, "ax", @progbits
        .global GunPlatform_Gun_04c776
GunPlatform_Gun_04c776:
        move.w  #0x84,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x19,0x1c(a6)                  | +00a
        jsr     0x138fe.l                       | +010
        move.w  0x70(a6),d0                     | +016
        andi.w  #0x3,d0                         | +01a
        lsl.w   #0x2,d0                         | +01e
        lea     0x290efe.l,a1                   | +020
        move.w  (a1,d0.w),0x74(a6)              | +026
        move.w  0x2(a1,d0.w),0x76(a6)           | +02c
        move.l  #0x290d70,0x4c(a6)              | +032
        lea     0x290876.l,a0                   | +03a
        jsr     0x28cd4.l                       | +040
        lea     .L04c7c2(pc),a1                 | +046
        move.l  a1,(a6)                         | +04a
.L04c7c2:
        movea.l 0xc(a6),a0                      | +04c
        cmpi.b  #0x1,0x21(a0)                   | +050
        bne.w   .L04c7d6                        | +056
        lea     GunPlatform_GunDamaged_04c832(pc),a1 | +05a
        move.l  a1,(a6)                         | +05e
        .global GunPlatform_Gun_04c776__L04c7d6
GunPlatform_Gun_04c776__L04c7d6:
.L04c7d6:
        jsr     0x5e506.l                       | +060
        tst.b   0x78(a0)                        | +066
        beq.w   .L04c7ea                        | +06a
        bset    #0x0,0x5a(a6)                   | +06e
.L04c7ea:
        move.w  0x74(a6),d0                     | +074
        add.w   d0,0x22(a6)                     | +078
        move.w  0x76(a6),d0                     | +07c
        add.w   d0,0x24(a6)                     | +080
        jsr     0x28d70.l                       | +084
        jsr     0x283ca.l                       | +08a
        jsr     0x283d8.l                       | +090
        movea.l 0xc(a6),a0                      | +096
        cmpi.b  #0xff,0x20(a0)                  | +09a
        bne.w   .L04c820                        | +0a0
        lea     Jsr5B6ThenJmpScheduler_04c934(pc),a1 | +0a4
        move.l  a1,(a6)                         | +0a8
.L04c820:
        jsr     0x5e45a.l                       | +0aa
        bcc.w   SetHandlerRts_04c830            | +0b0

| ----------------------------------------------------------------------------
|  GunPlatform_GunDamaged_04c832  @ $04C832  (20 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_GunDamaged_04c832, "ax", @progbits
        .global GunPlatform_GunDamaged_04c832
GunPlatform_GunDamaged_04c832:
        lea     0x290882.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L04c844(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L04c844:
        bra.b   GunPlatform_Gun_04c776__L04c7d6 | +012

| ----------------------------------------------------------------------------
|  GunPlatform_FlyingPart_04c846  @ $04C846  (194 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_FlyingPart_04c846, "ax", @progbits
        .global GunPlatform_FlyingPart_04c846
GunPlatform_FlyingPart_04c846:
        lea     0x2908c8.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        subq.w  #0x8,0x24(a6)                   | +00c
        move.w  0x36(a6),d0                     | +010
        cmpi.w  #0x0,d0                         | +014
        bgt.w   .L04c886                        | +018
        move.w  0x36(a6),d0                     | +01c
        cmpi.w  #0x0,d0                         | +020
        bgt.w   .L04c87a                        | +024
        .global GunPlatform_FlyingPart_04c846__L04c86e
GunPlatform_FlyingPart_04c846__L04c86e:
.L04c86e:
        lea     0x2bd1de.l,a0                   | +028
        jsr     0x799de.l                       | +02e
.L04c87a:
        lea     0x29088e.l,a0                   | +034
        jsr     0x28cd4.l                       | +03a
.L04c886:
        btst    #0x0,0x3a(a6)                   | +040
        bne.w   .L04c892                        | +046
        neg.w   d0                              | +04a
.L04c892:
        move.w  d0,0x28(a6)                     | +04c
        move.w  #0x1cf,d1                       | +050
        jsr     0x236e.l                        | +054
        bset    #0x4,0x6b(a6)                   | +05a
        move.w  #0xd000,d0                      | +060
        jsr     0x28134.l                       | +064
        andi.w  #0xffe3,0x38(a6)                | +06a
        ori.w   #0x14,0x38(a6)                  | +070
        lea     .L04c8c2(pc),a1                 | +076
        move.l  a1,(a6)                         | +07a
.L04c8c2:
        jsr     0x27cee.l                       | +07c
        bcs.w   .L04c8d6                        | +082
        jsr     0x27cee.l                       | +086
        bcc.w   .L04c8dc                        | +08c
.L04c8d6:
        lea     GunPlatform_FlyingPartLand_04c910(pc),a1 | +090
        move.l  a1,(a6)                         | +094
.L04c8dc:
        jsr     0x28d70.l                       | +096
        jsr     0x283d8.l                       | +09c
        btst    #0x1,0x13(a6)                   | +0a2
        beq.w   .L04c8f8                        | +0a8
        lea     GunPlatform_FlyingPartLand_04c910(pc),a1 | +0ac
        move.l  a1,(a6)                         | +0b0
.L04c8f8:
        movea.l #0xffffffff,a0                  | +0b2
        jsr     0x5dd56.l                       | +0b8
        bcc.w   SetHandlerRts_04c90e            | +0be

| ----------------------------------------------------------------------------
|  GunPlatform_FlyingPartLand_04c910  @ $04C910  (36 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_FlyingPartLand_04c910, "ax", @progbits
        .global GunPlatform_FlyingPartLand_04c910
GunPlatform_FlyingPartLand_04c910:
        move.w  #0x4000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x0,0x38(a6)                   | +010
        move.l  #0xffffffff,0x48(a6)            | +016
        jmp     0x77f6a.l                       | +01e

| ----------------------------------------------------------------------------
|  GunPlatform_FreeIfParentDead_04c942  @ $04C942  (14 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_FreeIfParentDead_04c942, "ax", @progbits
        .global GunPlatform_FreeIfParentDead_04c942
GunPlatform_FreeIfParentDead_04c942:
        movea.l 0xc(a6),a0                      | +000
        cmpi.b  #0xff,0x20(a0)                  | +004
        bne.w   SetHandlerRts_04c956            | +00a

| ----------------------------------------------------------------------------
|  GunPlatform_PartHitCheck_04c958  @ $04C958  (50 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_PartHitCheck_04c958, "ax", @progbits
        .global GunPlatform_PartHitCheck_04c958
GunPlatform_PartHitCheck_04c958:
        jsr     0x2870a.l                       | +000
        bcc.w   .L04c974                        | +006
        lea     0x5e766.l,a0                    | +00a
        jsr     0x5e770.l                       | +010
        bclr    #0x3,0x13(a6)                   | +016
.L04c974:
        jsr     0x28758.l                       | +01c
        bcc.w   .L04c988                        | +022
        movea.l 0xc(a6),a0                      | +026
        move.b  #0xff,0x20(a0)                  | +02a
.L04c988:
        rts                                     | +030

| ----------------------------------------------------------------------------
|  GunPlatform_SpriteByStateVariant_04c98a  @ $04C98A  (28 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_SpriteByStateVariant_04c98a, "ax", @progbits
        .global GunPlatform_SpriteByStateVariant_04c98a
GunPlatform_SpriteByStateVariant_04c98a:
        move.b  0x21(a6),d1                     | +000
        andi.w  #0xff,d1                        | +004
        add.w   d1,d1                           | +008
        add.w   d1,d1                           | +00a
        movea.l (a1,d1.w),a0                    | +00c
        move.w  0x70(a6),d0                     | +010
        add.w   d0,d0                           | +014
        add.w   d0,d0                           | +016
        movea.l (a0,d0.w),a0                    | +018

| ----------------------------------------------------------------------------
|  GunPlatform_SpriteByParentState_04c9ae  @ $04C9AE  (32 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_SpriteByParentState_04c9ae, "ax", @progbits
        .global GunPlatform_SpriteByParentState_04c9ae
GunPlatform_SpriteByParentState_04c9ae:
        movea.l 0xc(a6),a0                      | +000
        move.b  0x21(a0),d1                     | +004
        andi.w  #0xff,d1                        | +008
        add.w   d1,d1                           | +00c
        add.w   d1,d1                           | +00e
        movea.l (a1,d1.w),a0                    | +010
        move.w  0x70(a6),d0                     | +014
        add.w   d0,d0                           | +018
        add.w   d0,d0                           | +01a
        movea.l (a0,d0.w),a0                    | +01c

| ----------------------------------------------------------------------------
|  GunPlatform_TickTimer_04c9d6  @ $04C9D6  (12 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_TickTimer_04c9d6, "ax", @progbits
        .global GunPlatform_TickTimer_04c9d6
GunPlatform_TickTimer_04c9d6:
        subq.w  #0x1,0x80(a6)                   | +000
        bgt.w   ClearXN_04c9e8                  | +004
        clr.w   0x80(a6)                        | +008

| ----------------------------------------------------------------------------
|  GunPlatform_TickFireInterval_04c9ee  @ $04C9EE  (20 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_TickFireInterval_04c9ee, "ax", @progbits
        .global GunPlatform_TickFireInterval_04c9ee
GunPlatform_TickFireInterval_04c9ee:
        addq.w  #0x1,0x86(a6)                   | +000
        move.w  0x86(a6),d0                     | +004
        cmp.w   0x84(a6),d0                     | +008
        blt.w   ClearXN_04ca08                  | +00c
        clr.w   0x86(a6)                        | +010

| ----------------------------------------------------------------------------
|  GunPlatform_LoadIdleTimer_04ca0e  @ $04CA0E  (22 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_LoadIdleTimer_04ca0e, "ax", @progbits
        .global GunPlatform_LoadIdleTimer_04ca0e
GunPlatform_LoadIdleTimer_04ca0e:
        lea     0x2bd260.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,0x80(a6)                     | +00c
        clr.b   0x8a(a6)                        | +010
        rts                                     | +014

| ----------------------------------------------------------------------------
|  GunPlatform_LoadBurstParams_04ca24  @ $04CA24  (38 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_LoadBurstParams_04ca24, "ax", @progbits
        .global GunPlatform_LoadBurstParams_04ca24
GunPlatform_LoadBurstParams_04ca24:
        lea     0x2bd364.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.b  d0,0x82(a6)                     | +00c
        lea     0x2bd2e2.l,a0                   | +010
        jsr     0x799de.l                       | +016
        move.w  d0,0x84(a6)                     | +01c
        clr.w   0x86(a6)                        | +020
        rts                                     | +024

| ----------------------------------------------------------------------------
|  GunPlatform_RegisterKill_04ca4a  @ $04CA4A  (6 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_RegisterKill_04ca4a, "ax", @progbits
        .global GunPlatform_RegisterKill_04ca4a
GunPlatform_RegisterKill_04ca4a:
        lea     0x290e6c.l,a1                   | +000

| ----------------------------------------------------------------------------
|  GunPlatform_SpawnExplosionMusic_04ca58  @ $04CA58  (36 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_SpawnExplosionMusic_04ca58, "ax", @progbits
        .global GunPlatform_SpawnExplosionMusic_04ca58
GunPlatform_SpawnExplosionMusic_04ca58:
        move.w  #0x1067,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x620da.l,a1                    | +00a
        jsr     0x4ae.l                         | +010
        jsr     0x5dd02.l                       | +016
        subi.w  #0x20,0x24(a0)                  | +01c
        rts                                     | +022

| ----------------------------------------------------------------------------
|  GunPlatform_SpawnFlyingPart_04ca7c  @ $04CA7C  (28 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_SpawnFlyingPart_04ca7c, "ax", @progbits
        .global GunPlatform_SpawnFlyingPart_04ca7c
GunPlatform_SpawnFlyingPart_04ca7c:
        lea     GunPlatform_FlyingPart_04c846__L04c86e(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        subi.w  #0x20,0x22(a0)                  | +010
        addq.w  #0x4,0x24(a0)                   | +016
        rts                                     | +01a

| ----------------------------------------------------------------------------
|  GunPlatform_SndByVariant_04ca98  @ $04CA98  (8 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_SndByVariant_04ca98, "ax", @progbits
        .global GunPlatform_SndByVariant_04ca98
GunPlatform_SndByVariant_04ca98:
        move.b  0x98(a6),d0                     | +000
        move.w  #0x99,d1                        | +004

| ----------------------------------------------------------------------------
|  GunPlatform_SlotPrioCheck_04caa8  @ $04CAA8  (16 B)
| ----------------------------------------------------------------------------
        .section .text.GunPlatform_SlotPrioCheck_04caa8, "ax", @progbits
        .global GunPlatform_SlotPrioCheck_04caa8
GunPlatform_SlotPrioCheck_04caa8:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_04cabe                    | +00c

| ----------------------------------------------------------------------------
|  SpawnStream_ReadNext_04cac4  @ $04CAC4  (128 B)
| ----------------------------------------------------------------------------
        .section .text.SpawnStream_ReadNext_04cac4, "ax", @progbits
        .global SpawnStream_ReadNext_04cac4
SpawnStream_ReadNext_04cac4:
        movea.l 0x1081b2.l,a0                   | +000
        cmpa.w  #0x0,a0                         | +006
        bpl.w   .L04caec                        | +00a
        move.w  #0x50,d0                        | +00e
        move.w  #0x1f8,d1                       | +012
        move.b  #0x0,d2                         | +016
        move.w  #0x10,d3                        | +01a
        clr.b   d4                              | +01e
        clr.b   d5                              | +020
        clr.b   d6                              | +022
        bra.w   .L04cb42                        | +024
.L04caec:
        move.w  0x106f5c.l,d0                   | +028
        cmp.w   0x10(a0),d0                     | +02e
        bcs.w   .L04cb04                        | +032
        adda.w  #0x10,a0                        | +036
        move.l  a0,0x1081b2.l                   | +03a
.L04cb04:
        move.w  0x4(a0),d0                      | +040
        tst.b   0x2(a0)                         | +044
        beq.w   .L04cb16                        | +048
        sub.w   0x106f50.l,d0                   | +04c
.L04cb16:
        move.w  0x6(a0),d1                      | +052
        tst.b   0x3(a0)                         | +056
        beq.w   .L04cb28                        | +05a
        sub.w   0x106f54.l,d1                   | +05e
.L04cb28:
        neg.w   d1                              | +064
        addi.w  #0x200,d1                       | +066
        move.w  0x8(a0),d2                      | +06a
        move.w  0xa(a0),d3                      | +06e
        move.b  0xc(a0),d4                      | +072
        move.b  0xd(a0),d5                      | +076
        move.b  0xe(a0),d6                      | +07a
.L04cb42:
        rts                                     | +07e

| ----------------------------------------------------------------------------
|  SpriteSetPtrTbl6_04cb44  @ $04CB44  (24 B)
| ----------------------------------------------------------------------------
        .section .text.SpriteSetPtrTbl6_04cb44, "ax", @progbits
        .global SpriteSetPtrTbl6_04cb44
SpriteSetPtrTbl6_04cb44:
        .dc.w   0x0028                        | +000  (dato / opcode no decodificado)
        .dc.w   0x8d54                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x845e                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x8ee6                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x8980                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x90d8                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +014  (dato / opcode no decodificado)
        .dc.w   0x8c22                        | +016  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  SpawnStream_Dispatch_04cb88  @ $04CB88  (40 B)
| ----------------------------------------------------------------------------
        .section .text.SpawnStream_Dispatch_04cb88, "ax", @progbits
        .global SpawnStream_Dispatch_04cb88
SpawnStream_Dispatch_04cb88:
        jsr     SpawnStream_ReadNext_04cac4(pc) | +000
        btst    #0x0,0x100001.l                 | +004
        beq.w   JsrAbsRts_04cbb6                | +00c
        swap    d0                              | +010
        move.w  d1,d0                           | +012
        move.w  #0xffff,d1                      | +014
        move.w  #0x0,d2                         | +018
        moveq   #-1,d3                          | +01c
        moveq   #-1,d4                          | +01e
        moveq   #0,d5                           | +020
        lea     0x243608.l,a0                   | +022

| ----------------------------------------------------------------------------
|  SlotPrioCheck_04cbb8  @ $04CBB8  (16 B)
| ----------------------------------------------------------------------------
        .section .text.SlotPrioCheck_04cbb8, "ax", @progbits
        .global SlotPrioCheck_04cbb8
SlotPrioCheck_04cbb8:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_04cbce                    | +00c
