| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave RRR — Gun / Items / Score popups / Chute / Crate / Thrown / Flag
|  Región: $09A0BC..$09C608  (9,196 B, 96 entradas, 28 huecos)
| ============================================================================
|
|  A) RESUMEN
|  ----------
|  Objetos recogibles y sus efectos visuales, mas una torreta genérica:
|
|   1. Gun_* ($9A0BC..$9A5F4): cañón orientable con hijo de sprite.
|      Gun_Init (desde Airship_Wait_0893ac): snd $D1, prio $8000, registra
|      id de target ($8F3A6, invertido en +$72), crea el hijo Gun_Child
|      (+$73 = indice de pose, -1 = oculto; tabla de mapas +$74 y snd
|      $2F65EE), HP 100. Gun_Idle/Gun_Aim: +$70 = angulo 8.8 (wrap),
|      Gun_UpdateAngleSprite elige el mapa por octante en $2F6540 (bit 0 de
|      +$3A = espejo), Gun_RegisterTarget publica la boca en el anillo de
|      targets ($8F3BE con offset $2F6588), Gun_Aim persigue al jugador
|      ($5E136 => angulo) con inercia +$80, dispara rafagas de 3
|      (Gun_FireShell -> Gun_Shell $9A520 con tabla $2F66CA y seno $13C0E;
|      Gun_FireBullet -> $3093A). Gun_FollowParent pega el cañón al padre
|      (+$7B) o usa fisica simple $2783A. Gun_Destroyed: musica $1021,
|      cola $77FD6.
|
|   2. Item_* ($9A5F4..$9BDAA): items de la tabla E8000 284..294/316/317.
|      Item_SpawnTable_09a5f4: cabecera {hitbox, pickbox A/B} + 30 punteros
|      a handlers (Item_SpawnTable_Handlers) indexados por el byte de tipo
|      (0..29) por Item_SpawnFromParent_09a7cc (desde $3EFF6/$614A8; C=1 si
|      creo el item, lee +$98/+$99 del padre en (a6,d1)).
|        - Ammo 1..4 (tmpl 284-287, snd $17D) y AmmoSeq (288: secuencia de
|          mapas $2F79CE[+$98] con fade $2F7A28, da el tipo $2F7A38/$2F7A3C).
|        - Weapon (289, snd $17F/$1A7 -> $32C12), WeaponSwap (290: si el
|          jugador +$70==2 llama $2A28E y popup kind 1, si no $32C7E/kind 0),
|          Bombs (291: $2A2BA, popup kind 3).
|        - Food (294, 21 variantes de mapa con bra-tabla, snd $1AC/$1C6):
|          Wait -> Blink -> Rot/Thrown/Bounce (vel. $2F7A40/$2F7A58,
|          gravedad $27D50); Item_Food_Pickup -> popup Score_Popup_Value
|          con combo ($10E488 timer 45 f, $10E489 nivel 0..14, kind 2 = $F).
|        - Pow (293: $28998, dano $2870A, muere con musica $102E).
|        - ComboTimer (316): decrementa $10E488 y clampa $10E489 <= 14.
|        - Static (317): blit $43FAC con lista Item_Tmpl317_BlitList.
|      Pickup: Item_PickupTestA/B (+$45 cooldown 8 f, pickbox A/B via
|      Data+$DA/+$E2), Item_GiveAmmoKind ($32B58, musica $10FD si el
|      jugador esta en +$70==2). Item_Taken_Snd{A..D}: snd $1A6..$1A9,
|      prio $C000, 2 frames y Item_Taken_Rise (snd $180, mapa $2F674C).
|      Item_GroundTail: $5DD56/$5DD5C con hitbox y, si +$9F==0, cuenta
|      $10E48B.
|
|   3. Score_Popup_* ($9B48A..$9BAD2): digitos de puntuacion flotantes.
|      Score_Popup_Table (kind 0..3 -> lista de digitos $2F79A4) y
|      Score_Popup_Value (valor packed BCD en +$80 via $51A44, longitud
|      $2F798C/$2F7994, color por +$9C); cada digito Score_Digit (snd
|      $1B8/$1B9 o $1CD/$1CE segun jugador +$68, mapa $2F77A4[d], vel X
|      $5DCA4, gravedad $FEDD) con Lift/Hold/Blink (parpadeo con $106F28).
|      Score_Popup_ForPlayer ($32C6E): kind 2 para el slot $100440/$1004E0.
|
|   4. Chute_* ($9BDC6..$9BE9E): paracaidas A/B/C (mapas $2DD4BA/$2DD594/
|      $2DD37E, snd $163, prio $2000|$10) colocados con offset +$70/+$72.
|
|   5. Crate_* ($9BE9E..$9C218): caja lanzada (hitbox FFD0/0030).
|      Crate_Init: prio $D000, snd $17C; +$98&1 elige mapa vertical
|      ($2F7A70, hijo Crate_ChuteFollowX) o lateral ($2F7AC6/$2F7AAC, hijo
|      Crate_ChuteFollow); bit 1 de +$98^+$99 espeja. Cae con $27D50,
|      genera Crate_Debris cada 3 frames (seno $13C0E, amortiguacion /16),
|      al tocar suelo -> Crate_Opened (musica $1025, snd 4) o Crate_Destroyed
|      (cola $77EFE). Entity_IntegrateVelFrac_09c072 = integrador 8.8 de
|      +$28/+$2A sobre +$22/+$24 (+$26/+$27 fraccion) usado tambien por
|      $60618/$76F0A.
|
|   6. Thrown_* ($9C234..$9C4D4): objeto arrojado (snd $184, prio $D000):
|      mapas $2F83A4/$2F84AE, sombra Thrown_Shadow/ShadowB (siguen al padre
|      a 3/8 o 3/4), aterriza en Thrown_Landed.
|
|   7. Flag_Init_09c4d4 (desde $3C8BA): bandera/marcador (snd $19B, mapas
|      $2F86EC/$2F865C, pone $10A2D1=1, musica $1086 al activarse bit 1 de
|      +$5A).
|
|  B) EVIDENCIAS
|  -------------
|  - E8000: 284..291 -> Item_Tmpl*, 293 Pow, 294 Food, 316 ComboTimer,
|    317 Static. Jump-table $9A6E2 (30 x u32) apunta a los Item_Tmpl*.
|  - $8948A Airship_Wait -> Gun_Init; $3C9FC -> Thrown_Init; $3C8BA ->
|    Flag_Init; $25B68 -> Item_SpawnAtOffset; $53D3E -> Score_Popup_Kind4.
|
|  C) HIPOTESIS / DUDAS
|  --------------------
|  - "Gun" podria ser el cañon del Hairbuster Riberts / torretas de la
|    escena 5 (unico caller Airship_Wait).
|  - El significado exacto de kind 0..4 de los popups (arma, cambio,
|    comida, bombas, rescate) se infiere de los callers.
|  - Las 21 variantes de Food se eligen por entrada multiple (bra-tabla
|    de 12 B); los offsets intermedios no son entradas registradas.
|
|  D) DATOS EMBEBIDOS
|  ------------------
|  Gun_Hitbox_09a0d8 / Crate_Hitbox_09be9e / Thrown_Hitbox_09c234 (8 B,
|  FFD0 0030 FFD0 0030), Item_FadeRamp_09ab54 (8 B), Item_SpawnTable_09a5f4
|  (358 B), Item_Tmpl317_BlitList_09bb92 (536 B).
|
|  E) CALLEES EXTERNOS
|  -------------------
|  $4AE/$518/$5B6 tareas, $236E snd, $2352 musica, $28134 prio, $28CD4
|  mapa, $28D70 hijos, $2870A dano, $2783A/$27D50 fisica, $27C8C/$27CEE
|  probes, $27EBA efecto, $267E2 relink, $283CA/$283D8 flags, $5DD02 copia,
|  $5DD56/$5DD5C suelo, $5CA2A sprite, $13C0E seno, $5E9B6 RNG, $5DCA4 rand,
|  $5E45A fuera-de-mundo, $5E136 angulo, $799DE tabla 2D, $51A44 BCD,
|  $43FAC blit, $13600/$138FE slots, $32B58/$32C12/$32C7E/$2A28E/$2A2BA
|  inventario del jugador, $77FD6/$77EFE colas de muerte.
|
|  F) ESTADO
|  ---------
|  96/96 entradas byte-exactas (matcher 4397/4397, 9.37 % de la P ROM).
|
|  Verificación: cada sección .text.<Sym> se coloca en su dirección CPU
|  absoluta y reensambla byte-exacta contra build/mslug_prom.bin
|  (MD5 816b3f74c76b3373993407615f1850fe).
| ============================================================================

        .text

| ----------------------------------------------------------------------------
|  Entity_CmpPrioWithSibling_09a0bc  @ $09A0BC  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpPrioWithSibling_09a0bc, "ax", @progbits
        .global Entity_CmpPrioWithSibling_09a0bc
Entity_CmpPrioWithSibling_09a0bc:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_09a0d2                    | +00c

| ----------------------------------------------------------------------------
|  Gun_Hitbox_09a0d8  @ $09A0D8  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Gun_Hitbox_09a0d8, "ax", @progbits
        .global Gun_Hitbox_09a0d8
Gun_Hitbox_09a0d8:
        .dc.w   0xffd0                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +002  (dato / opcode no decodificado)
        .dc.w   0xffd0                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +006  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Gun_GroundTail_09a0e0  @ $09A0E0  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Gun_GroundTail_09a0e0, "ax", @progbits
        .global Gun_GroundTail_09a0e0
Gun_GroundTail_09a0e0:
        lea     Gun_Hitbox_09a0d8(pc),a0        | +000
        jsr     0x5dd5c.l                       | +004
        bcc.w   Jsr5B6Rts_09a0fa                | +00a

| ----------------------------------------------------------------------------
|  Gun_UpdateAngleSprite_09a0fc  @ $09A0FC  (138 B)
| ----------------------------------------------------------------------------
        .section .text.Gun_UpdateAngleSprite_09a0fc, "ax", @progbits
        .global Gun_UpdateAngleSprite_09a0fc
Gun_UpdateAngleSprite_09a0fc:
        move.w  0x70(a6),d0                     | +000
        asr.w   #0x8,d0                         | +004
        addq.w  #0x4,d0                         | +006
        andi.w  #0xf8,d0                        | +008
        cmpi.b  #0x88,d0                        | +00c
        bcs.w   .L09a112                        | +010
        neg.b   d0                              | +014
.L09a112:
        cmpi.w  #0x40,d0                        | +016
        bcs.w   .L09a12e                        | +01a
        bset    #0x0,0x3a(a6)                   | +01e
        subi.b  #0x40,d0                        | +024
        neg.b   d0                              | +028
        addi.b  #0x40,d0                        | +02a
        bra.w   .L09a134                        | +02e
.L09a12e:
        bclr    #0x0,0x3a(a6)                   | +032
.L09a134:
        andi.w  #0x78,d0                        | +038
        lea     0x2f6540.l,a0                   | +03c
        movea.l (a0,d0.w),a1                    | +042
        move.b  0x7(a0,d0.w),d0                 | +046
        btst    #0x0,0x3a(a6)                   | +04a
        beq.w   .L09a15a                        | +050
        subi.b  #0x40,d0                        | +054
        neg.b   d0                              | +058
        addi.b  #0x40,d0                        | +05a
.L09a15a:
        move.b  d0,0x82(a6)                     | +05e
        moveq   #0,d0                           | +062
        move.b  0x3b(a6),d0                     | +064
        beq.w   .L09a16e                        | +068
        subq.b  #0x1,d0                         | +06c
        move.b  d0,0x3b(a6)                     | +06e
.L09a16e:
        add.w   d0,d0                           | +072
        add.w   d0,d0                           | +074
        add.w   d0,d0                           | +076
        move.l  (a1,d0.w),0x74(a6)              | +078
        move.l  0x4(a1,d0.w),0x3c(a6)           | +07e
        jmp     0x5ca2a.l                       | +084

| ----------------------------------------------------------------------------
|  Gun_RegisterTarget_09a186  @ $09A186  (48 B)
| ----------------------------------------------------------------------------
        .section .text.Gun_RegisterTarget_09a186, "ax", @progbits
        .global Gun_RegisterTarget_09a186
Gun_RegisterTarget_09a186:
        move.b  0x70(a6),d0                     | +000
        andi.w  #0xf8,d0                        | +004
        cmpi.b  #0x88,d0                        | +008
        bcs.w   .L09a198                        | +00c
        neg.b   d0                              | +010
.L09a198:
        lsr.w   #0x2,d0                         | +012
        lea     0x2f6588.l,a0                   | +014
        move.w  (a0,d0.w),d0                    | +01a
        add.w   0x22(a6),d0                     | +01e
        move.w  0x24(a6),d1                     | +022
        move.b  0x72(a6),d2                     | +026
        jmp     0x8f3be.l                       | +02a

| ----------------------------------------------------------------------------
|  Gun_MuzzlePos_09a1b6  @ $09A1B6  (202 B)
| ----------------------------------------------------------------------------
        .section .text.Gun_MuzzlePos_09a1b6, "ax", @progbits
        .global Gun_MuzzlePos_09a1b6
Gun_MuzzlePos_09a1b6:
        move.b  0x82(a6),d7                     | +000
        andi.w  #0xf0,d7                        | +004
        lsr.w   #0x1,d7                         | +008
        lea     0x2f65aa.l,a1                   | +00a
        move.w  (a1,d7.w),d0                    | +010
        move.w  0x2(a1,d7.w),d1                 | +014
        add.w   0x22(a6),d0                     | +018
        add.w   0x24(a6),d1                     | +01c
        rts                                     | +020
        .global Gun_FollowParent_09a1d8
Gun_FollowParent_09a1d8:
        tst.b   0x7b(a6)                        | +022
        bne.w   .L09a1ea                        | +026
        jmp     0x2783a.l                       | +02a
        bra.w   .L09a20c                        | +030
.L09a1ea:
        movea.l 0xc(a6),a0                      | +034
        move.w  0x22(a0),d0                     | +038
        add.w   0x7c(a6),d0                     | +03c
        move.w  d0,0x22(a6)                     | +040
        move.w  0x24(a0),d0                     | +044
        add.w   0x7e(a6),d0                     | +048
        move.w  d0,0x24(a6)                     | +04c
        jmp     0x28108.l                       | +050
.L09a20c:
        rts                                     | +056
        .global Gun_FireShell_09a20e
Gun_FireShell_09a20e:
        move.b  #0x5,0x3b(a6)                   | +058
        move.l  a6,-(a7)                        | +05e
        lea     0x100800.l,a6                   | +060
        move.w  #0x10f4,d0                      | +066
        jsr     0x2352.l                        | +06a
        lea     Gun_Shell_09a520(pc),a1         | +070
        jsr     0x4ae.l                         | +074
        movea.l (a7)+,a6                        | +07a
        move.b  0x82(a6),0x98(a0)               | +07c
        bsr.w   Gun_MuzzlePos_09a1b6            | +082
        move.w  d0,0x22(a0)                     | +086
        move.w  d1,0x24(a0)                     | +08a
        rts                                     | +08e
        .global Gun_FireBullet_09a246
Gun_FireBullet_09a246:
        move.b  #0x5,0x3b(a6)                   | +090
        move.l  a6,-(a7)                        | +096
        lea     0x100800.l,a6                   | +098
        move.w  #0x10f4,d0                      | +09e
        jsr     0x2352.l                        | +0a2
        lea     0x3093a.l,a1                    | +0a8
        jsr     0x4ae.l                         | +0ae
        movea.l (a7)+,a6                        | +0b4
        move.b  0x82(a6),0x98(a0)               | +0b6
        bsr.w   Gun_MuzzlePos_09a1b6            | +0bc
        move.w  d0,0x22(a0)                     | +0c0
        move.w  d1,0x24(a0)                     | +0c4
        rts                                     | +0c8

| ----------------------------------------------------------------------------
|  Gun_Child_Init_09a280  @ $09A280  (48 B)
| ----------------------------------------------------------------------------
        .section .text.Gun_Child_Init_09a280, "ax", @progbits
        .global Gun_Child_Init_09a280
Gun_Child_Init_09a280:
        movea.l 0xc(a6),a0                      | +000
        move.b  0x73(a0),d0                     | +004
        bmi.w   SetHandlerRts_09a2b6            | +008
        jsr     0x13600.l                       | +00c
        movea.l 0xc(a6),a0                      | +012
        move.b  0x73(a0),d0                     | +016
        andi.w  #0xff,d0                        | +01a
        add.w   d0,d0                           | +01e
        lea     0x2f65ee.l,a0                   | +020
        move.w  (a0,d0.w),d1                    | +026
        jsr     0x236e.l                        | +02a

| ----------------------------------------------------------------------------
|  Gun_Child_Sync_09a2b8  @ $09A2B8  (64 B)
| ----------------------------------------------------------------------------
        .section .text.Gun_Child_Sync_09a2b8, "ax", @progbits
        .global Gun_Child_Sync_09a2b8
Gun_Child_Sync_09a2b8:
        movea.l 0xc(a6),a0                      | +000
        move.b  0x3a(a0),0x3a(a6)               | +004
        movea.l 0x74(a0),a1                     | +00a
        move.b  0x73(a0),d0                     | +00e
        bmi.w   SetTaskHandler_09a2f8           | +012
        andi.w  #0xff,d0                        | +016
        add.w   d0,d0                           | +01a
        add.w   d0,d0                           | +01c
        move.l  (a1,d0.w),0x3c(a6)              | +01e
        move.w  0x22(a0),0x22(a6)               | +024
        move.w  0x24(a0),0x24(a6)               | +02a
        move.w  0x38(a0),0x38(a6)               | +030
        jsr     0x5ca2a.l                       | +036
        bra.w   SetHandlerRts_09a2fe            | +03c

| ----------------------------------------------------------------------------
|  Gun_Init_09a300  @ $09A300  (116 B)
| ----------------------------------------------------------------------------
        .section .text.Gun_Init_09a300, "ax", @progbits
        .global Gun_Init_09a300
Gun_Init_09a300:
        move.b  #0xff,0x7b(a6)                  | +000
        movea.l 0xc(a6),a0                      | +006
        move.w  0x22(a6),0x7c(a6)               | +00a
        move.w  0x24(a6),0x7e(a6)               | +010
        bra.w   .L09a31e                        | +016
        clr.b   0x7b(a6)                        | +01a
.L09a31e:
        clr.b   0x3b(a6)                        | +01e
        move.w  #0xd1,d1                        | +022
        jsr     0x236e.l                        | +026
        move.w  #0x8000,d0                      | +02c
        jsr     0x28134.l                       | +030
        andi.w  #0xffe3,0x38(a6)                | +036
        ori.w   #0x4,0x38(a6)                   | +03c
        jsr     0x8f3a6.l                       | +042
        not.b   d0                              | +048
        move.b  d0,0x72(a6)                     | +04a
        clr.w   0x70(a6)                        | +04e
        lea     Gun_Child_Init_09a280(pc),a1    | +052
        jsr     0x4ae.l                         | +056
        move.w  #0x64,0x66(a6)                  | +05c
        move.w  #0x2,0x1c(a6)                   | +062
        jsr     0x138fe.l                       | +068
        lea     Gun_Idle_09a374(pc),a1          | +06e
        move.l  a1,(a6)                         | +072

| ----------------------------------------------------------------------------
|  Gun_Idle_09a374  @ $09A374  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Gun_Idle_09a374, "ax", @progbits
        .global Gun_Idle_09a374
Gun_Idle_09a374:
        clr.b   0x7a(a6)                        | +000
        clr.b   0x3b(a6)                        | +004
        move.b  #0xff,0x73(a6)                  | +008
        move.l  #0xffffffff,0x48(a6)            | +00e
        lea     .L09a390(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L09a390:
        bsr.w   Gun_FollowParent_09a1d8   | +01c
        bsr.w   Gun_UpdateAngleSprite_09a0fc    | +020
        bsr.w   Gun_RegisterTarget_09a186       | +024
        tst.b   d4                              | +028
        bmi.w   .L09a3ac                        | +02a
        move.b  d4,0x73(a6)                     | +02e
        lea     Gun_Aim_09a3b0(pc),a1           | +032
        move.l  a1,(a6)                         | +036
.L09a3ac:
        bra.w   Gun_GroundTail_09a0e0           | +038

| ----------------------------------------------------------------------------
|  Gun_Aim_09a3b0  @ $09A3B0  (346 B)
| ----------------------------------------------------------------------------
        .section .text.Gun_Aim_09a3b0, "ax", @progbits
        .global Gun_Aim_09a3b0
Gun_Aim_09a3b0:
        lea     .L09a3b6(pc),a1                 | +000
        move.l  a1,(a6)                         | +004
.L09a3b6:
        cmpi.b  #0x2,0x73(a6)                   | +006
        bne.w   .L09a43a                        | +00c
        jsr     0x5e136.l                       | +010
        bcs.w   .L09a436                        | +016
        cmpi.b  #0x3,0x7a(a6)                   | +01a
        bne.w   .L09a3e0                        | +020
        lsl.w   #0x8,d0                         | +024
        sub.w   0x70(a6),d0                     | +026
        asr.w   #0x3,d0                         | +02a
        add.w   d0,0x70(a6)                     | +02c
.L09a3e0:
        tst.w   0x78(a6)                        | +030
        beq.w   .L09a3f0                        | +034
        subq.w  #0x1,0x78(a6)                   | +038
        bra.w   .L09a436                        | +03c
.L09a3f0:
        addi.w  #0x80,d0                        | +040
        andi.w  #0xff00,d0                      | +044
        bne.w   .L09a436                        | +048
        move.b  0x7a(a6),d1                     | +04c
        bne.w   .L09a41e                        | +050
        move.b  #0x3,0x7a(a6)                   | +054
        lea     0x2f6648.l,a0                   | +05a
        jsr     0x799de.l                       | +060
        move.w  d0,0x78(a6)                     | +066
        bra.w   .L09a436                        | +06a
.L09a41e:
        subq.w  #0x1,0x78(a6)                   | +06e
        bne.w   .L09a436                        | +072
        move.w  #0x2,0x78(a6)                   | +076
        subq.b  #0x1,d1                         | +07c
        move.b  d1,0x7a(a6)                     | +07e
        bsr.w   Gun_FireShell_09a20e   | +082
.L09a436:
        bra.w   .L09a4dc                        | +086
.L09a43a:
        cmpi.b  #0x0,0x73(a6)                   | +08a
        bne.w   .L09a44e                        | +090
        lea     0x10e200.l,a2                   | +094
        bra.w   .L09a454                        | +09a
.L09a44e:
        lea     0x10e206.l,a2                   | +09e
.L09a454:
        move.b  0x2(a2),d0                      | +0a4
        andi.w  #0xc,d0                         | +0a8
        beq.w   .L09a4a2                        | +0ac
        andi.b  #0x4,d0                         | +0b0
        bne.w   .L09a470                        | +0b4
        move.w  #0x8000,d0                      | +0b8
        bra.w   .L09a472                        | +0bc
.L09a470:
        clr.w   d0                              | +0c0
.L09a472:
        sub.w   0x70(a6),d0                     | +0c2
        asr.w   #0x2,d0                         | +0c6
        ext.l   d0                              | +0c8
        move.l  d0,d1                           | +0ca
        swap    d1                              | +0cc
        eor.w   d1,d0                           | +0ce
        sub.w   d1,d0                           | +0d0
        move.w  0x80(a6),d2                     | +0d2
        addi.w  #0x100,d2                       | +0d6
        cmp.w   d0,d2                           | +0da
        bcs.w   .L09a492                        | +0dc
        move.w  d0,d2                           | +0e0
.L09a492:
        move.w  d2,0x80(a6)                     | +0e2
        eor.w   d1,d2                           | +0e6
        sub.w   d1,d2                           | +0e8
        add.w   d2,0x70(a6)                     | +0ea
        bra.w   .L09a4a6                        | +0ee
.L09a4a2:
        clr.w   0x80(a6)                        | +0f2
.L09a4a6:
        btst    #0x4,0x3(a2)                    | +0f6
        beq.w   .L09a4bc                        | +0fc
        move.b  #0x3,0x7a(a6)                   | +100
        move.b  #0x1,0x78(a6)                   | +106
.L09a4bc:
        move.b  0x7a(a6),d0                     | +10c
        beq.w   .L09a4dc                        | +110
        subq.b  #0x1,0x78(a6)                   | +114
        bne.w   .L09a4dc                        | +118
        move.b  #0x2,0x78(a6)                   | +11c
        subq.b  #0x1,d0                         | +122
        move.b  d0,0x7a(a6)                     | +124
        bsr.w   Gun_FireBullet_09a246   | +128
.L09a4dc:
        bsr.w   Gun_FollowParent_09a1d8   | +12c
        bsr.w   Gun_UpdateAngleSprite_09a0fc    | +130
        jsr     0x28d70.l                       | +134
        move.w  0x22(a6),d0                     | +13a
        move.w  0x24(a6),d1                     | +13e
        move.b  0x72(a6),d2                     | +142
        bsr.w   Gun_RegisterTarget_09a186       | +146
        tst.b   d4                              | +14a
        bpl.w   .L09a506                        | +14c
        lea     Gun_Idle_09a374(pc),a1          | +150
        move.l  a1,(a6)                         | +154
.L09a506:
        bra.w   Gun_GroundTail_09a0e0           | +156

| ----------------------------------------------------------------------------
|  Gun_Destroyed_09a50a  @ $09A50A  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Gun_Destroyed_09a50a, "ax", @progbits
        .global Gun_Destroyed_09a50a
Gun_Destroyed_09a50a:
        move.w  #0x1021,d0                      | +000
        jsr     0x2352.l                        | +004
        jsr     0x13600.l                       | +00a
        jmp     0x77fd6.l                       | +010

| ----------------------------------------------------------------------------
|  Gun_Shell_09a520  @ $09A520  (184 B)
| ----------------------------------------------------------------------------
        .section .text.Gun_Shell_09a520, "ax", @progbits
        .global Gun_Shell_09a520
Gun_Shell_09a520:
        move.w  #0xd1,d1                        | +000
        jsr     0x236e.l                        | +004
        bset    #0x4,0x6b(a6)                   | +00a
        lea     0x2f65f4.l,a0                   | +010
        move.l  a0,0x4c(a6)                     | +016
        jsr     0x283ca.l                       | +01a
        move.w  #0xd000,0x38(a6)                | +020
        lea     0x2f66ca.l,a0                   | +026
        jsr     0x799de.l                       | +02c
        move.w  d0,d1                           | +032
        move.b  0x98(a6),d0                     | +034
        addq.b  #0x8,d0                         | +038
        andi.w  #0xf0,d0                        | +03a
        jsr     0x13c0e.l                       | +03e
        move.w  d1,0x28(a6)                     | +044
        move.w  d2,0x2a(a6)                     | +048
        move.b  0x98(a6),d0                     | +04c
        andi.w  #0xff,d0                        | +050
        asr.w   #0x4,d0                         | +054
        movea.l #0x29d452,a0                    | +056
        lsl.w   #0x2,d0                         | +05c
        movea.l (a0,d0.w),a0                    | +05e
        cmpa.l  #0xffffffff,a0                  | +062
        beq.w   .L09a592                        | +068
        jsr     0x28cd4.l                       | +06c
.L09a592:
        lea     .L09a598(pc),a1                 | +072
        move.l  a1,(a6)                         | +076
.L09a598:
        bset    #0x6,0x13(a6)                   | +078
        jsr     0x27cee.l                       | +07e
        jsr     0x28d70.l                       | +084
        jsr     0x283d8.l                       | +08a
        btst    #0x1,0x13(a6)                   | +090
        beq.w   .L09a5c0                        | +096
        jmp     0x518.l                         | +09a
.L09a5c0:
        movea.l #0xffffffff,a0                  | +0a0
        jsr     0x5dd56.l                       | +0a6
        bcc.w   .L09a5d6                        | +0ac
        jmp     0x518.l                         | +0b0
.L09a5d6:
        rts                                     | +0b6

| ----------------------------------------------------------------------------
|  Entity_CmpPrioWithSibling_09a5d8  @ $09A5D8  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpPrioWithSibling_09a5d8, "ax", @progbits
        .global Entity_CmpPrioWithSibling_09a5d8
Entity_CmpPrioWithSibling_09a5d8:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_09a5ee                    | +00c

| ----------------------------------------------------------------------------
|  Item_SpawnTable_09a5f4  @ $09A5F4  (358 B)
| ----------------------------------------------------------------------------
        .section .text.Item_SpawnTable_09a5f4, "ax", @progbits
        .global Item_SpawnTable_09a5f4
Item_SpawnTable_09a5f4:
        .dc.w   0xffc0                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +002  (dato / opcode no decodificado)
        .dc.w   0xffc0                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +02c  (dato / opcode no decodificado)
        .dc.w   0xa5fc                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +032  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +038  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +040  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +044  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +04c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +056  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +058  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +05a  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +060  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +062  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +064  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +066  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +068  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +070  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +072  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +074  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +076  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +078  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +07a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x8001                        | +07e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +080  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +082  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +084  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +088  (dato / opcode no decodificado)
        .dc.w   0x000e                        | +08a  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x3608                        | +090  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +094  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +096  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +098  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +09a  (dato / opcode no decodificado)
        .dc.w   0x3612                        | +09c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +09e  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0x000e                        | +0a2  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0a4  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0a6  (dato / opcode no decodificado)
        .dc.w   0x361c                        | +0a8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0aa  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +0ac  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0ae  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0b0  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0b2  (dato / opcode no decodificado)
        .dc.w   0x3626                        | +0b4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0b6  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +0b8  (dato / opcode no decodificado)
        .dc.w   0x000e                        | +0ba  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0bc  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0be  (dato / opcode no decodificado)
        .dc.w   0x3630                        | +0c0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0c2  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +0c4  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0c6  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +0c8  (dato / opcode no decodificado)
        .dc.w   0xffe0                        | +0ca  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +0cc  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0ce  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +0d0  (dato / opcode no decodificado)
        .global Item_SpawnTable_Hitbox_09a6c6
Item_SpawnTable_Hitbox_09a6c6:
        .dc.w   0xffd0                        | +0d2  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +0d4  (dato / opcode no decodificado)
        .dc.w   0xffd0                        | +0d6  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +0d8  (dato / opcode no decodificado)
        .global Item_SpawnTable_PickboxA_09a6ce
Item_SpawnTable_PickboxA_09a6ce:
        .dc.w   0x0014                        | +0da  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +0dc  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +0de  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +0e0  (dato / opcode no decodificado)
        .global Item_SpawnTable_PickboxB_09a6d6
Item_SpawnTable_PickboxB_09a6d6:
        .dc.w   0x0010                        | +0e2  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +0e4  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +0e6  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +0e8  (dato / opcode no decodificado)
        .global Item_SpawnTable_Handlers_09a6de
Item_SpawnTable_Handlers_09a6de:
        .dc.w   0xffff                        | +0ea  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0ec  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +0ee  (dato / opcode no decodificado)
        .dc.w   0xa918                        | +0f0  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +0f2  (dato / opcode no decodificado)
        .dc.w   0xaa32                        | +0f4  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +0f6  (dato / opcode no decodificado)
        .dc.w   0xa9d4                        | +0f8  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +0fa  (dato / opcode no decodificado)
        .dc.w   0xa976                        | +0fc  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +0fe  (dato / opcode no decodificado)
        .dc.w   0xabc2                        | +100  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +102  (dato / opcode no decodificado)
        .dc.w   0xac52                        | +104  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +106  (dato / opcode no decodificado)
        .dc.w   0xad54                        | +108  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +10a  (dato / opcode no decodificado)
        .dc.w   0xaec4                        | +10c  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +10e  (dato / opcode no decodificado)
        .dc.w   0xaed8                        | +110  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +112  (dato / opcode no decodificado)
        .dc.w   0xaeec                        | +114  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +116  (dato / opcode no decodificado)
        .dc.w   0xb11e                        | +118  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +11a  (dato / opcode no decodificado)
        .dc.w   0xaf02                        | +11c  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +11e  (dato / opcode no decodificado)
        .dc.w   0xaf16                        | +120  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +122  (dato / opcode no decodificado)
        .dc.w   0xaf2a                        | +124  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +126  (dato / opcode no decodificado)
        .dc.w   0xaf3e                        | +128  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +12a  (dato / opcode no decodificado)
        .dc.w   0xb1ca                        | +12c  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +12e  (dato / opcode no decodificado)
        .dc.w   0xaf52                        | +130  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +132  (dato / opcode no decodificado)
        .dc.w   0xaf68                        | +134  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +136  (dato / opcode no decodificado)
        .dc.w   0xaf7c                        | +138  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +13a  (dato / opcode no decodificado)
        .dc.w   0xaf90                        | +13c  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +13e  (dato / opcode no decodificado)
        .dc.w   0xafa4                        | +140  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +142  (dato / opcode no decodificado)
        .dc.w   0xafb8                        | +144  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +146  (dato / opcode no decodificado)
        .dc.w   0xafcc                        | +148  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +14a  (dato / opcode no decodificado)
        .dc.w   0xafe0                        | +14c  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +14e  (dato / opcode no decodificado)
        .dc.w   0xaff4                        | +150  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +152  (dato / opcode no decodificado)
        .dc.w   0xb034                        | +154  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +156  (dato / opcode no decodificado)
        .dc.w   0xb048                        | +158  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +15a  (dato / opcode no decodificado)
        .dc.w   0xb008                        | +15c  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +15e  (dato / opcode no decodificado)
        .dc.w   0xb01e                        | +160  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +162  (dato / opcode no decodificado)
        .dc.w   0xaa90                        | +164  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Item_GroundTail_09a75a  @ $09A75A  (80 B)
| ----------------------------------------------------------------------------
        .section .text.Item_GroundTail_09a75a, "ax", @progbits
        .global Item_GroundTail_09a75a
Item_GroundTail_09a75a:
        cmpi.b  #0x1,0x90(a6)                   | +000
        bne.w   Item_GroundTail_NoCount_09a76a | +006
        addq.b  #0x1,0x10e48b.l                 | +00a
        .global Item_GroundTail_NoCount_09a76a
Item_GroundTail_NoCount_09a76a:
        tst.b   0x9f(a6)                        | +010
        beq.w   .L09a78e                        | +014
        movea.l #0xffffffff,a0                  | +018
        lea     Item_SpawnTable_09a5f4(pc),a0   | +01e
        jsr     0x5dd56.l                       | +022
        bcc.w   .L09a78c                        | +028
        jmp     0x518.l                         | +02c
.L09a78c:
        rts                                     | +032
.L09a78e:
        movea.l #0xffffffff,a0                  | +034
        lea     Item_SpawnTable_09a5f4(pc),a0   | +03a
        jsr     0x5dd5c.l                       | +03e
        bcc.w   .L09a7a8                        | +044
        jmp     0x518.l                         | +048
.L09a7a8:
        rts                                     | +04e

| ----------------------------------------------------------------------------
|  Item_SpawnFromParent_09a7cc  @ $09A7CC  (106 B)
| ----------------------------------------------------------------------------
        .section .text.Item_SpawnFromParent_09a7cc, "ax", @progbits
        .global Item_SpawnFromParent_09a7cc
Item_SpawnFromParent_09a7cc:
        tst.b   d0                              | +000
        beq.w   .L09a830                        | +002
        move.w  d1,-(a7)                        | +006
        andi.w  #0x3f,d0                        | +008
        cmpi.w  #0x1e,d0                        | +00c
        bcs.w   .L09a7e4                        | +010
        move.w  #0xa,d0                         | +014
.L09a7e4:
        add.w   d0,d0                           | +018
        add.w   d0,d0                           | +01a
        lea     Item_SpawnTable_Handlers_09a6de(pc),a0 | +01c
        movea.l (a0,d0.w),a1                    | +020
        move.l  a6,-(a7)                        | +024
        lea     0x100800.l,a6                   | +026
        jsr     0x4ae.l                         | +02c
        movea.l (a7)+,a6                        | +032
        move.w  0x24(a6),0x24(a0)               | +034
        move.w  #0x10,d0                        | +03a
        btst    #0x0,0x3a(a6)                   | +03e
        bne.w   .L09a816                        | +044
        neg.w   d0                              | +048
.L09a816:
        add.w   0x22(a6),d0                     | +04a
        move.w  d0,0x22(a0)                     | +04e
        move.w  (a7)+,d1                        | +052
        move.b  (a6,d1.w),0x98(a0)              | +054
        move.b  0x1(a6,d1.w),0x99(a0)           | +05a
        ori.b   #0x11,ccr                       | +060
.L09a830:
        eori.b  #0x11,ccr                       | +064
        rts                                     | +068

| ----------------------------------------------------------------------------
|  Item_PickupTestA_09a836  @ $09A836  (12 B)
| ----------------------------------------------------------------------------
        .section .text.Item_PickupTestA_09a836, "ax", @progbits
        .global Item_PickupTestA_09a836
Item_PickupTestA_09a836:
        lea     Item_SpawnTable_PickboxA_09a6ce(pc),a1 | +000
        .global Item_PickupTest_Common_09a83a
Item_PickupTest_Common_09a83a:
        tst.b   0x45(a6)                        | +004
        beq.w   JmpAbsThunk_09a848              | +008

| ----------------------------------------------------------------------------
|  Item_PickupTestB_09a84e  @ $09A84E  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Item_PickupTestB_09a84e, "ax", @progbits
        .global Item_PickupTestB_09a84e
Item_PickupTestB_09a84e:
        lea     Item_SpawnTable_PickboxB_09a6d6(pc),a1 | +000
        bra.b   Item_PickupTest_Common_09a83a | +004

| ----------------------------------------------------------------------------
|  Item_GiveAmmoKind_09a854  @ $09A854  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Item_GiveAmmoKind_09a854, "ax", @progbits
        .global Item_GiveAmmoKind_09a854
Item_GiveAmmoKind_09a854:
        move.b  0x98(a6),d2                     | +000
        .global Item_GiveAmmo_09a858
Item_GiveAmmo_09a858:
        jsr     0x32b58.l                       | +004
        cmpi.b  #0x2,0x70(a1)                   | +00a
        bne.w   JsrAbsThunk_09a876              | +010
        move.l  a1,-(a7)                        | +014
        move.w  #0x10fd,d0                      | +016
        jsr     0x2352.l                        | +01a
        movea.l (a7)+,a1                        | +020

| ----------------------------------------------------------------------------
|  Item_SetMapFromSeq_09a87e  @ $09A87E  (56 B)
| ----------------------------------------------------------------------------
        .section .text.Item_SetMapFromSeq_09a87e, "ax", @progbits
        .global Item_SetMapFromSeq_09a87e
Item_SetMapFromSeq_09a87e:
        movea.l 0x76(a6),a0                     | +000
        move.w  0x7a(a6),d0                     | +004
        move.b  (a0,d0.w),d0                    | +008
        cmpi.b  #0xff,d0                        | +00c
        bne.w   .L09a89c                        | +010
        clr.w   0x7a(a6)                        | +014
        movea.l 0x76(a6),a0                     | +018
        move.b  (a0),d0                         | +01c
.L09a89c:
        andi.w  #0x3,d0                         | +01e
        movea.l #0x2f7a28,a0                    | +022
        lsl.w   #0x2,d0                         | +028
        movea.l (a0,d0.w),a0                    | +02a
        cmpa.l  #0xffffffff,a0                  | +02e
        beq.w   JsrAbsRts_09a8bc                | +034

| ----------------------------------------------------------------------------
|  Item_Tmpl_PowCounted_09a8be  @ $09A8BE  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Item_Tmpl_PowCounted_09a8be, "ax", @progbits
        .global Item_Tmpl_PowCounted_09a8be
Item_Tmpl_PowCounted_09a8be:
        move.b  #0x1,0x90(a6)                   | +000
        cmpi.b  #0x2,0x10e48a.l                 | +006
        blt.w   Item_FlyAway_09a8de             | +00e

| ----------------------------------------------------------------------------
|  Item_FlyAway_09a8de  @ $09A8DE  (58 B)
| ----------------------------------------------------------------------------
        .section .text.Item_FlyAway_09a8de, "ax", @progbits
        .global Item_FlyAway_09a8de
Item_FlyAway_09a8de:
        move.w  #0x180,d1                       | +000
        jsr     0x236e.l                        | +004
        lea     0x2f678a.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L09a8fa(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L09a8fa:
        jsr     0x2783a.l                       | +01c
        jsr     0x28d70.l                       | +022
        bcc.w   .L09a914                        | +028
        jsr     0x13600.l                       | +02c
        move.l  0x5c(a6),(a6)                   | +032
.L09a914:
        bra.w   Item_GroundTail_09a75a          | +036

| ----------------------------------------------------------------------------
|  Item_Tmpl284_Ammo4_09a918  @ $09A918  (94 B)
| ----------------------------------------------------------------------------
        .section .text.Item_Tmpl284_Ammo4_09a918, "ax", @progbits
        .global Item_Tmpl284_Ammo4_09a918
Item_Tmpl284_Ammo4_09a918:
        move.w  #0x17d,d1                       | +000
        jsr     0x236e.l                        | +004
        move.b  #0x8,0x45(a6)                   | +00a
        move.w  #0x8000,d0                      | +010
        jsr     0x28134.l                       | +014
        andi.w  #0xffe3,0x38(a6)                | +01a
        ori.w   #0x0,0x38(a6)                   | +020
        lea     0x2f67dc.l,a0                   | +026
        jsr     0x28cd4.l                       | +02c
        lea     .L09a950(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L09a950:
        jsr     0x27c8c.l                       | +038
        jsr     0x28d70.l                       | +03e
        jsr     Item_PickupTestA_09a836(pc)     | +044
        bcc.w   .L09a972                        | +048
        move.b  #0x4,d1                         | +04c
        jsr     Item_GiveAmmoKind_09a854(pc)    | +050
        lea     Item_Taken_SndA_09ae26(pc),a1   | +054
        move.l  a1,(a6)                         | +058
.L09a972:
        bra.w   Item_GroundTail_09a75a          | +05a

| ----------------------------------------------------------------------------
|  Item_Tmpl285_Ammo3_09a976  @ $09A976  (94 B)
| ----------------------------------------------------------------------------
        .section .text.Item_Tmpl285_Ammo3_09a976, "ax", @progbits
        .global Item_Tmpl285_Ammo3_09a976
Item_Tmpl285_Ammo3_09a976:
        move.w  #0x17d,d1                       | +000
        jsr     0x236e.l                        | +004
        move.b  #0x8,0x45(a6)                   | +00a
        move.w  #0x8000,d0                      | +010
        jsr     0x28134.l                       | +014
        andi.w  #0xffe3,0x38(a6)                | +01a
        ori.w   #0x0,0x38(a6)                   | +020
        lea     0x2f67e8.l,a0                   | +026
        jsr     0x28cd4.l                       | +02c
        lea     .L09a9ae(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L09a9ae:
        jsr     0x27c8c.l                       | +038
        jsr     0x28d70.l                       | +03e
        jsr     Item_PickupTestA_09a836(pc)     | +044
        bcc.w   .L09a9d0                        | +048
        move.b  #0x3,d1                         | +04c
        jsr     Item_GiveAmmoKind_09a854(pc)    | +050
        lea     Item_Taken_SndA_09ae26(pc),a1   | +054
        move.l  a1,(a6)                         | +058
.L09a9d0:
        bra.w   Item_GroundTail_09a75a          | +05a

| ----------------------------------------------------------------------------
|  Item_Tmpl286_Ammo1_09a9d4  @ $09A9D4  (94 B)
| ----------------------------------------------------------------------------
        .section .text.Item_Tmpl286_Ammo1_09a9d4, "ax", @progbits
        .global Item_Tmpl286_Ammo1_09a9d4
Item_Tmpl286_Ammo1_09a9d4:
        move.w  #0x17d,d1                       | +000
        jsr     0x236e.l                        | +004
        move.b  #0x8,0x45(a6)                   | +00a
        move.w  #0x8000,d0                      | +010
        jsr     0x28134.l                       | +014
        andi.w  #0xffe3,0x38(a6)                | +01a
        ori.w   #0x0,0x38(a6)                   | +020
        lea     0x2f67f4.l,a0                   | +026
        jsr     0x28cd4.l                       | +02c
        lea     .L09aa0c(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L09aa0c:
        jsr     0x27c8c.l                       | +038
        jsr     0x28d70.l                       | +03e
        jsr     Item_PickupTestA_09a836(pc)     | +044
        bcc.w   .L09aa2e                        | +048
        move.b  #0x1,d1                         | +04c
        jsr     Item_GiveAmmoKind_09a854(pc)    | +050
        lea     Item_Taken_SndA_09ae26(pc),a1   | +054
        move.l  a1,(a6)                         | +058
.L09aa2e:
        bra.w   Item_GroundTail_09a75a          | +05a

| ----------------------------------------------------------------------------
|  Item_Tmpl287_Ammo2_09aa32  @ $09AA32  (94 B)
| ----------------------------------------------------------------------------
        .section .text.Item_Tmpl287_Ammo2_09aa32, "ax", @progbits
        .global Item_Tmpl287_Ammo2_09aa32
Item_Tmpl287_Ammo2_09aa32:
        move.w  #0x17d,d1                       | +000
        jsr     0x236e.l                        | +004
        move.b  #0x8,0x45(a6)                   | +00a
        move.w  #0x8000,d0                      | +010
        jsr     0x28134.l                       | +014
        andi.w  #0xffe3,0x38(a6)                | +01a
        ori.w   #0x0,0x38(a6)                   | +020
        lea     0x2f6800.l,a0                   | +026
        jsr     0x28cd4.l                       | +02c
        lea     .L09aa6a(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L09aa6a:
        jsr     0x27c8c.l                       | +038
        jsr     0x28d70.l                       | +03e
        jsr     Item_PickupTestA_09a836(pc)     | +044
        bcc.w   .L09aa8c                        | +048
        move.b  #0x2,d1                         | +04c
        jsr     Item_GiveAmmoKind_09a854(pc)    | +050
        lea     Item_Taken_SndA_09ae26(pc),a1   | +054
        move.l  a1,(a6)                         | +058
.L09aa8c:
        bra.w   Item_GroundTail_09a75a          | +05a

| ----------------------------------------------------------------------------
|  Item_Tmpl288_AmmoSeq_09aa90  @ $09AA90  (82 B)
| ----------------------------------------------------------------------------
        .section .text.Item_Tmpl288_AmmoSeq_09aa90, "ax", @progbits
        .global Item_Tmpl288_AmmoSeq_09aa90
Item_Tmpl288_AmmoSeq_09aa90:
        move.w  #0x17d,d1                       | +000
        jsr     0x236e.l                        | +004
        lea     0x2f79ce.l,a0                   | +00a
        move.b  0x98(a6),d0                     | +010
        cmpi.b  #0xb,d0                         | +014
        bcs.w   .L09aab0                        | +018
        move.b  #0xa,d0                         | +01c
.L09aab0:
        andi.w  #0xf,d0                         | +020
        lsl.w   #0x2,d0                         | +024
        move.l  (a0,d0.w),d0                    | +026
        move.l  d0,0x76(a6)                     | +02a
        clr.w   0x7a(a6)                        | +02e
        jsr     Item_SetMapFromSeq_09a87e(pc)   | +032
        move.b  #0x8,0x45(a6)                   | +036
        move.w  #0x8000,d0                      | +03c
        jsr     0x28134.l                       | +040
        andi.w  #0xffe3,0x38(a6)                | +046
        ori.w   #0x0,0x38(a6)                   | +04c

| ----------------------------------------------------------------------------
|  Item_AmmoSeq_Show_09aae2  @ $09AAE2  (114 B)
| ----------------------------------------------------------------------------
        .section .text.Item_AmmoSeq_Show_09aae2, "ax", @progbits
        .global Item_AmmoSeq_Show_09aae2
Item_AmmoSeq_Show_09aae2:
        move.b  #0xff,0x32(a6)                  | +000
        move.w  #0x50,0x70(a6)                  | +006
        lea     .L09aaf4(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L09aaf4:
        subq.w  #0x1,0x70(a6)                   | +012
        cmpi.w  #0x0,0x70(a6)                   | +016
        bgt.w   Item_AmmoSeq_Common_09ab08 | +01c
        lea     Item_AmmoSeq_FadeOut_09ab5c(pc),a1 | +020
        move.l  a1,(a6)                         | +024
        .global Item_AmmoSeq_Common_09ab08
Item_AmmoSeq_Common_09ab08:
        jsr     0x27c8c.l                       | +026
        jsr     0x28d70.l                       | +02c
        jsr     Item_PickupTestA_09a836(pc)     | +032
        bcc.w   .L09ab50                        | +036
        movea.l 0x76(a6),a2                     | +03a
        move.w  0x7a(a6),d2                     | +03e
        move.b  (a2,d2.w),d2                    | +042
        andi.w  #0x3,d2                         | +046
        lea     0x2f7a38.l,a2                   | +04a
        move.b  (a2,d2.w),d1                    | +050
        lea     0x2f7a3c.l,a2                   | +054
        move.b  (a2,d2.w),d2                    | +05a
        jsr     Item_GiveAmmo_09a858(pc) | +05e
        move.b  #0xff,0x32(a6)                  | +062
        lea     Item_Taken_SndA_09ae26(pc),a1   | +068
        move.l  a1,(a6)                         | +06c
.L09ab50:
        bra.w   Item_GroundTail_09a75a          | +06e

| ----------------------------------------------------------------------------
|  Item_FadeRamp_09ab54  @ $09AB54  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Item_FadeRamp_09ab54, "ax", @progbits
        .global Item_FadeRamp_09ab54
Item_FadeRamp_09ab54:
        .dc.w   0xf0e0                        | +000  (dato / opcode no decodificado)
        .dc.w   0xd0b0                        | +002  (dato / opcode no decodificado)
        .dc.w   0x9060                        | +004  (dato / opcode no decodificado)
        .dc.w   0x3000                        | +006  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Item_AmmoSeq_FadeOut_09ab5c  @ $09AB5C  (50 B)
| ----------------------------------------------------------------------------
        .section .text.Item_AmmoSeq_FadeOut_09ab5c, "ax", @progbits
        .global Item_AmmoSeq_FadeOut_09ab5c
Item_AmmoSeq_FadeOut_09ab5c:
        clr.w   0x70(a6)                        | +000
        lea     .L09ab66(pc),a1                 | +004
        move.l  a1,(a6)                         | +008
.L09ab66:
        lea     Item_FadeRamp_09ab54(pc),a0     | +00a
        move.w  0x70(a6),d0                     | +00e
        move.b  (a0,d0.w),d0                    | +012
        move.b  d0,0x32(a6)                     | +016
        addq.w  #0x1,0x70(a6)                   | +01a
        cmpi.w  #0x7,0x70(a6)                   | +01e
        bne.w   .L09ab8a                        | +024
        lea     Item_AmmoSeq_FadeIn_09ab8e(pc),a1 | +028
        move.l  a1,(a6)                         | +02c
.L09ab8a:
        bra.w   Item_AmmoSeq_Common_09ab08 | +02e

| ----------------------------------------------------------------------------
|  Item_AmmoSeq_FadeIn_09ab8e  @ $09AB8E  (52 B)
| ----------------------------------------------------------------------------
        .section .text.Item_AmmoSeq_FadeIn_09ab8e, "ax", @progbits
        .global Item_AmmoSeq_FadeIn_09ab8e
Item_AmmoSeq_FadeIn_09ab8e:
        addq.w  #0x1,0x7a(a6)                   | +000
        jsr     Item_SetMapFromSeq_09a87e(pc)   | +004
        lea     .L09ab9c(pc),a1                 | +008
        move.l  a1,(a6)                         | +00c
.L09ab9c:
        lea     Item_FadeRamp_09ab54(pc),a0     | +00e
        move.w  0x70(a6),d0                     | +012
        move.b  (a0,d0.w),d0                    | +016
        move.b  d0,0x32(a6)                     | +01a
        subq.w  #0x1,0x70(a6)                   | +01e
        tst.w   0x70(a6)                        | +022
        bne.w   .L09abbe                        | +026
        lea     Item_AmmoSeq_Show_09aae2(pc),a1 | +02a
        move.l  a1,(a6)                         | +02e
.L09abbe:
        bra.w   Item_AmmoSeq_Common_09ab08 | +030

| ----------------------------------------------------------------------------
|  Item_Tmpl289_Weapon_09abc2  @ $09ABC2  (144 B)
| ----------------------------------------------------------------------------
        .section .text.Item_Tmpl289_Weapon_09abc2, "ax", @progbits
        .global Item_Tmpl289_Weapon_09abc2
Item_Tmpl289_Weapon_09abc2:
        move.w  #0x17f,d1                       | +000
        jsr     0x236e.l                        | +004
        move.w  #0x1a7,d1                       | +00a
        jsr     0x236e.l                        | +00e
        move.b  #0x8,0x45(a6)                   | +014
        move.w  #0x8000,d0                      | +01a
        jsr     0x28134.l                       | +01e
        andi.w  #0xffe3,0x38(a6)                | +024
        ori.w   #0x0,0x38(a6)                   | +02a
        lea     0x2f680c.l,a0                   | +030
        jsr     0x28cd4.l                       | +036
        tst.b   0x98(a6)                        | +03c
        bne.w   .L09ac0c                        | +040
        move.b  #0xa,0x98(a6)                   | +044
.L09ac0c:
        tst.b   0x99(a6)                        | +04a
        bne.w   .L09ac1a                        | +04e
        move.b  #0x64,0x99(a6)                  | +052
.L09ac1a:
        lea     .L09ac20(pc),a1                 | +058
        move.l  a1,(a6)                         | +05c
.L09ac20:
        jsr     0x27c8c.l                       | +05e
        jsr     0x28d70.l                       | +064
        jsr     Item_PickupTestA_09a836(pc)     | +06a
        bcc.w   .L09ac4e                        | +06e
        move.b  d0,0x68(a6)                     | +072
        jsr     0x32c12.l                       | +076
        lea     Item_Taken_SndC_09ae36(pc),a1   | +07c
        move.l  a1,(a6)                         | +080
        move.w  #0x1124,d0                      | +082
        jsr     0x2352.l                        | +086
.L09ac4e:
        bra.w   Item_GroundTail_09a75a          | +08c

| ----------------------------------------------------------------------------
|  Item_Tmpl290_WeaponSwap_09ac52  @ $09AC52  (258 B)
| ----------------------------------------------------------------------------
        .section .text.Item_Tmpl290_WeaponSwap_09ac52, "ax", @progbits
        .global Item_Tmpl290_WeaponSwap_09ac52
Item_Tmpl290_WeaponSwap_09ac52:
        move.w  #0x17e,d1                       | +000
        jsr     0x236e.l                        | +004
        move.w  #0x1a8,d1                       | +00a
        jsr     0x236e.l                        | +00e
        move.b  #0x8,0x45(a6)                   | +014
        move.w  #0x8000,d0                      | +01a
        jsr     0x28134.l                       | +01e
        andi.w  #0xffe3,0x38(a6)                | +024
        ori.w   #0x0,0x38(a6)                   | +02a
        lea     0x2f685c.l,a0                   | +030
        jsr     0x28cd4.l                       | +036
        tst.b   0x98(a6)                        | +03c
        bne.w   .L09ac9c                        | +040
        move.b  #0xa,0x98(a6)                   | +044
.L09ac9c:
        tst.b   0x99(a6)                        | +04a
        bne.w   .L09acaa                        | +04e
        move.b  #0xa,0x99(a6)                   | +052
.L09acaa:
        lea     .L09acb0(pc),a1                 | +058
        move.l  a1,(a6)                         | +05c
.L09acb0:
        jsr     0x27c8c.l                       | +05e
        jsr     0x28d70.l                       | +064
        jsr     Item_PickupTestA_09a836(pc)     | +06a
        bcc.w   .L09ad50                        | +06e
        cmpi.b  #0x0,d0                         | +072
        bne.w   .L09acd6                        | +076
        lea     0x100440.l,a1                   | +07a
        bra.w   .L09acdc                        | +080
.L09acd6:
        lea     0x1004e0.l,a1                   | +084
.L09acdc:
        cmpi.b  #0x2,0x70(a1)                   | +08a
        bne.w   .L09ad16                        | +090
        move.b  d0,0x68(a6)                     | +094
        move.b  0x99(a6),0x98(a6)               | +098
        jsr     0x2a28e.l                       | +09e
        lea     Score_Popup_Table_09b48a(pc),a1 | +0a4
        jsr     0x4ae.l                         | +0a8
        jsr     0x5dd02.l                       | +0ae
        move.b  0x68(a6),0x68(a0)               | +0b4
        move.b  #0x1,0x98(a0)                   | +0ba
        bra.w   .L09ad40                        | +0c0
.L09ad16:
        move.b  d0,0x68(a6)                     | +0c4
        move.b  0x98(a6),0x98(a6)               | +0c8
        jsr     0x32c7e.l                       | +0ce
        lea     Score_Popup_Table_09b48a(pc),a1 | +0d4
        jsr     0x4ae.l                         | +0d8
        jsr     0x5dd02.l                       | +0de
        move.b  0x68(a6),0x68(a0)               | +0e4
        clr.b   0x98(a0)                        | +0ea
.L09ad40:
        lea     Item_Taken_SndB_09ae2e(pc),a1   | +0ee
        move.l  a1,(a6)                         | +0f2
        move.w  #0x1124,d0                      | +0f4
        jsr     0x2352.l                        | +0f8
.L09ad50:
        bra.w   Item_GroundTail_09a75a          | +0fe

| ----------------------------------------------------------------------------
|  Item_Tmpl291_Bombs_09ad54  @ $09AD54  (210 B)
| ----------------------------------------------------------------------------
        .section .text.Item_Tmpl291_Bombs_09ad54, "ax", @progbits
        .global Item_Tmpl291_Bombs_09ad54
Item_Tmpl291_Bombs_09ad54:
        move.w  #0x180,d1                       | +000
        jsr     0x236e.l                        | +004
        move.w  #0x1a9,d1                       | +00a
        jsr     0x236e.l                        | +00e
        move.b  #0x8,0x45(a6)                   | +014
        move.w  #0x8000,d0                      | +01a
        jsr     0x28134.l                       | +01e
        andi.w  #0xffe3,0x38(a6)                | +024
        ori.w   #0x0,0x38(a6)                   | +02a
        lea     0x2f68ac.l,a0                   | +030
        jsr     0x28cd4.l                       | +036
        lea     .L09ad96(pc),a1                 | +03c
        move.l  a1,(a6)                         | +040
.L09ad96:
        jsr     0x27c8c.l                       | +042
        jsr     0x28d70.l                       | +048
        jsr     Item_PickupTestA_09a836(pc)     | +04e
        bcc.w   .L09ae22                        | +052
        cmpi.b  #0x0,d0                         | +056
        bne.w   .L09adbc                        | +05a
        lea     0x100440.l,a1                   | +05e
        bra.w   .L09adc2                        | +064
.L09adbc:
        lea     0x1004e0.l,a1                   | +068
.L09adc2:
        move.b  d0,0x68(a6)                     | +06e
        cmpi.b  #0x2,0x70(a1)                   | +072
        bne.w   .L09ae0e                        | +078
        move.b  d0,0x68(a6)                     | +07c
        move.b  0x98(a6),d0                     | +080
        andi.w  #0xff,d0                        | +084
        jsr     0x2a2ba.l                       | +088
        bcc.w   .L09adee                        | +08e
        jsr     Score_Popup_Kind3_09ba34(pc)    | +092
        bra.w   .L09ae0a                        | +096
.L09adee:
        lea     Score_Popup_Table_09b48a(pc),a1 | +09a
        jsr     0x4ae.l                         | +09e
        jsr     0x5dd02.l                       | +0a4
        move.b  0x68(a6),0x68(a0)               | +0aa
        move.b  #0x3,0x98(a0)                   | +0b0
.L09ae0a:
        bra.w   .L09ae12                        | +0b6
.L09ae0e:
        jsr     Score_Popup_Kind3_09ba34(pc)    | +0ba
.L09ae12:
        lea     Item_Taken_SndD_09ae3e(pc),a1   | +0be
        move.l  a1,(a6)                         | +0c2
        move.w  #0x1124,d0                      | +0c4
        jsr     0x2352.l                        | +0c8
.L09ae22:
        bra.w   Item_GroundTail_09a75a          | +0ce

| ----------------------------------------------------------------------------
|  Item_Taken_SndA_09ae26  @ $09AE26  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Item_Taken_SndA_09ae26, "ax", @progbits
        .global Item_Taken_SndA_09ae26
Item_Taken_SndA_09ae26:
        move.w  #0x1a6,d1                       | +000
        bra.w   Item_Taken_Common_09ae42 | +004

| ----------------------------------------------------------------------------
|  Item_Taken_SndB_09ae2e  @ $09AE2E  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Item_Taken_SndB_09ae2e, "ax", @progbits
        .global Item_Taken_SndB_09ae2e
Item_Taken_SndB_09ae2e:
        move.w  #0x1a7,d1                       | +000
        bra.w   Item_Taken_Common_09ae42 | +004

| ----------------------------------------------------------------------------
|  Item_Taken_SndC_09ae36  @ $09AE36  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Item_Taken_SndC_09ae36, "ax", @progbits
        .global Item_Taken_SndC_09ae36
Item_Taken_SndC_09ae36:
        move.w  #0x1a8,d1                       | +000
        bra.w   Item_Taken_Common_09ae42 | +004

| ----------------------------------------------------------------------------
|  Item_Taken_SndD_09ae3e  @ $09AE3E  (80 B)
| ----------------------------------------------------------------------------
        .section .text.Item_Taken_SndD_09ae3e, "ax", @progbits
        .global Item_Taken_SndD_09ae3e
Item_Taken_SndD_09ae3e:
        move.w  #0x1a9,d1                       | +000
        .global Item_Taken_Common_09ae42
Item_Taken_Common_09ae42:
        jsr     0x236e.l                        | +004
        move.w  #0x2,0x70(a6)                   | +00a
        move.w  #0xc000,d0                      | +010
        jsr     0x28134.l                       | +014
        andi.w  #0xffe3,0x38(a6)                | +01a
        ori.w   #0x0,0x38(a6)                   | +020
        lea     .L09ae6a(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L09ae6a:
        jsr     0x2783a.l                       | +02c
        jsr     0x28d70.l                       | +032
        subq.w  #0x1,0x70(a6)                   | +038
        cmpi.w  #0x0,0x70(a6)                   | +03c
        bgt.w   .L09ae8a                        | +042
        lea     Item_Taken_Rise_09ae8e(pc),a1   | +046
        move.l  a1,(a6)                         | +04a
.L09ae8a:
        bra.w   Item_GroundTail_NoCount_09a76a | +04c

| ----------------------------------------------------------------------------
|  Item_Taken_Rise_09ae8e  @ $09AE8E  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Item_Taken_Rise_09ae8e, "ax", @progbits
        .global Item_Taken_Rise_09ae8e
Item_Taken_Rise_09ae8e:
        move.w  #0x180,d1                       | +000
        jsr     0x236e.l                        | +004
        lea     0x2f674c.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L09aeaa(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L09aeaa:
        jsr     0x2783a.l                       | +01c
        jsr     0x28d70.l                       | +022
        bcc.w   .L09aec0                        | +028
        jmp     0x518.l                         | +02c
.L09aec0:
        bra.w   Item_GroundTail_NoCount_09a76a | +032

| ----------------------------------------------------------------------------
|  Item_Tmpl294_Food_09aec4  @ $09AEC4  (484 B)
| ----------------------------------------------------------------------------
        .section .text.Item_Tmpl294_Food_09aec4, "ax", @progbits
        .global Item_Tmpl294_Food_09aec4
Item_Tmpl294_Food_09aec4:
        clr.b   0x9c(a6)                        | +000
        lea     0x2f6b78.l,a0                   | +004
        jsr     0x28cd4.l                       | +00a
        bra.w   .L09b05c                        | +010
        clr.b   0x9c(a6)                        | +014
        lea     0x2f6b22.l,a0                   | +018
        jsr     0x28cd4.l                       | +01e
        bra.w   .L09b05c                        | +024
        move.b  #0x1,0x9c(a6)                   | +028
        lea     0x2f6ae0.l,a0                   | +02e
        jsr     0x28cd4.l                       | +034
        bra.w   .L09b05c                        | +03a
        clr.b   0x9c(a6)                        | +03e
        lea     0x2f68fc.l,a0                   | +042
        jsr     0x28cd4.l                       | +048
        bra.w   .L09b05c                        | +04e
        clr.b   0x9c(a6)                        | +052
        lea     0x2f6bce.l,a0                   | +056
        jsr     0x28cd4.l                       | +05c
        bra.w   .L09b064                        | +062
        clr.b   0x9c(a6)                        | +066
        lea     0x2f6cca.l,a0                   | +06a
        jsr     0x28cd4.l                       | +070
        bra.w   .L09b064                        | +076
        clr.b   0x9c(a6)                        | +07a
        lea     0x2f6e4c.l,a0                   | +07e
        jsr     0x28cd4.l                       | +084
        bra.w   .L09b064                        | +08a
        move.b  #0x1,0x9c(a6)                   | +08e
        lea     0x2f7002.l,a0                   | +094
        jsr     0x28cd4.l                       | +09a
        bra.w   .L09b05c                        | +0a0
        clr.b   0x9c(a6)                        | +0a4
        lea     0x2f706c.l,a0                   | +0a8
        jsr     0x28cd4.l                       | +0ae
        bra.w   .L09b05c                        | +0b4
        clr.b   0x9c(a6)                        | +0b8
        lea     0x2f70b8.l,a0                   | +0bc
        jsr     0x28cd4.l                       | +0c2
        bra.w   .L09b064                        | +0c8
        clr.b   0x9c(a6)                        | +0cc
        lea     0x2f7186.l,a0                   | +0d0
        jsr     0x28cd4.l                       | +0d6
        bra.w   .L09b05c                        | +0dc
        clr.b   0x9c(a6)                        | +0e0
        lea     0x2f7204.l,a0                   | +0e4
        jsr     0x28cd4.l                       | +0ea
        bra.w   .L09b05c                        | +0f0
        clr.b   0x9c(a6)                        | +0f4
        lea     0x2f7290.l,a0                   | +0f8
        jsr     0x28cd4.l                       | +0fe
        bra.w   .L09b064                        | +104
        clr.b   0x9c(a6)                        | +108
        lea     0x2f73b8.l,a0                   | +10c
        jsr     0x28cd4.l                       | +112
        bra.w   .L09b064                        | +118
        clr.b   0x9c(a6)                        | +11c
        lea     0x2f745e.l,a0                   | +120
        jsr     0x28cd4.l                       | +126
        bra.w   .L09b064                        | +12c
        clr.b   0x9c(a6)                        | +130
        lea     0x2f74a0.l,a0                   | +134
        jsr     0x28cd4.l                       | +13a
        bra.w   .L09b05c                        | +140
        move.b  #0x2,0x9c(a6)                   | +144
        lea     0x2f75f2.l,a0                   | +14a
        jsr     0x28cd4.l                       | +150
        bra.w   .L09b064                        | +156
        move.b  #0x1,0x9c(a6)                   | +15a
        lea     0x2f76e8.l,a0                   | +160
        jsr     0x28cd4.l                       | +166
        bra.w   .L09b05c                        | +16c
        clr.b   0x9c(a6)                        | +170
        lea     0x2f755a.l,a0                   | +174
        jsr     0x28cd4.l                       | +17a
        bra.w   .L09b05c                        | +180
        clr.b   0x9c(a6)                        | +184
        lea     0x2f75b0.l,a0                   | +188
        jsr     0x28cd4.l                       | +18e
        bra.w   .L09b064                        | +194
.L09b05c:
        move.w  #0x1ac,d1                       | +198
        bra.w   .L09b068                        | +19c
.L09b064:
        move.w  #0x1c6,d1                       | +1a0
.L09b068:
        cmpi.b  #0xff,0x9a(a6)                  | +1a4
        beq.w   .L09b078                        | +1aa
        eori.b  #0x1,0x3a(a6)                   | +1ae
.L09b078:
        jsr     0x236e.l                        | +1b4
        move.b  #0x8,0x45(a6)                   | +1ba
        move.w  #0x8000,d0                      | +1c0
        jsr     0x28134.l                       | +1c4
        andi.w  #0xffe3,0x38(a6)                | +1ca
        ori.w   #0x0,0x38(a6)                   | +1d0
        jsr     Item_LifetimeFromParam_09bb3c(pc) | +1d6
        move.w  d0,0x70(a6)                     | +1da
        lea     Item_Food_Wait_09b0a8(pc),a1    | +1de
        move.l  a1,(a6)                         | +1e2

| ----------------------------------------------------------------------------
|  Item_Food_Wait_09b0a8  @ $09B0A8  (48 B)
| ----------------------------------------------------------------------------
        .section .text.Item_Food_Wait_09b0a8, "ax", @progbits
        .global Item_Food_Wait_09b0a8
Item_Food_Wait_09b0a8:
        jsr     Item_PhysAndProbe_09b8c0(pc)    | +000
        jsr     0x28d70.l                       | +004
        subq.w  #0x1,0x70(a6)                   | +00a
        cmpi.w  #0x0,0x70(a6)                   | +00e
        bgt.w   Item_Food_PickupCheck_09b0c6  | +014
        lea     Item_Food_Blink_09b0d8(pc),a1   | +018
        move.l  a1,(a6)                         | +01c
        .global Item_Food_PickupCheck_09b0c6
Item_Food_PickupCheck_09b0c6:
        jsr     Item_Food_Pickup_09b920(pc)     | +01e
        bcc.w   .L09b0d4                        | +022
        lea     Item_Free_09b474(pc),a1         | +026
        move.l  a1,(a6)                         | +02a
.L09b0d4:
        bra.w   Item_GroundTail_09a75a          | +02c

| ----------------------------------------------------------------------------
|  Item_Food_Blink_09b0d8  @ $09B0D8  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Item_Food_Blink_09b0d8, "ax", @progbits
        .global Item_Food_Blink_09b0d8
Item_Food_Blink_09b0d8:
        move.b  #0x1e,0x70(a6)                  | +000

| ----------------------------------------------------------------------------
|  Item_Food_BlinkLoop_09b0de  @ $09B0DE  (368 B)
| ----------------------------------------------------------------------------
        .section .text.Item_Food_BlinkLoop_09b0de, "ax", @progbits
        .global Item_Food_BlinkLoop_09b0de
Item_Food_BlinkLoop_09b0de:
        move.w  #0x8000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x0,0x38(a6)                   | +010
        lea     .L09b0fa(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L09b0fa:
        jsr     Item_PhysAndProbe_09b8c0(pc)    | +01c
        btst    #0x0,0x70(a6)                   | +020
        beq.w   .L09b10e                        | +026
        jsr     0x28d70.l                       | +02a
.L09b10e:
        subq.b  #0x1,0x70(a6)                   | +030
        bcc.w   .L09b11c                        | +034
        lea     Item_Free_09b474(pc),a1         | +038
        move.l  a1,(a6)                         | +03c
.L09b11c:
        bra.b   Item_Food_PickupCheck_09b0c6  | +03e
        clr.b   0x9c(a6)                        | +040
        cmpi.b  #0xff,0x9a(a6)                  | +044
        beq.w   .L09b132                        | +04a
        eori.b  #0x1,0x3a(a6)                   | +04e
.L09b132:
        move.b  0x99(a6),d0                     | +054
        andi.w  #0xff,d0                        | +058
        lsl.w   #0x1,d0                         | +05c
        tst.w   d0                              | +05e
        bne.w   .L09b146                        | +060
        move.w  #0x7fff,d0                      | +064
.L09b146:
        move.w  d0,0x70(a6)                     | +068
        clr.w   0x72(a6)                        | +06c
        move.w  #0x1ac,d1                       | +070
        jsr     0x236e.l                        | +074
        move.w  #0x8000,d0                      | +07a
        jsr     0x28134.l                       | +07e
        andi.w  #0xffe3,0x38(a6)                | +084
        ori.w   #0x0,0x38(a6)                   | +08a
        lea     0x2f6a1a.l,a0                   | +090
        jsr     0x28cd4.l                       | +096
        lea     .L09b180(pc),a1                 | +09c
        move.l  a1,(a6)                         | +0a0
.L09b180:
        jsr     Item_PhysAndProbe_09b8c0(pc)    | +0a2
        jsr     0x28d70.l                       | +0a6
        bcc.w   .L09b1a6                        | +0ac
        lea     0x2f6aa8.l,a0                   | +0b0
        jsr     0x28cd4.l                       | +0b6
        move.b  #0x14,0x70(a6)                  | +0bc
        lea     Item_Food_BlinkLoop_09b0de(pc),a1 | +0c2
        move.l  a1,(a6)                         | +0c6
.L09b1a6:
        subq.w  #0x1,0x70(a6)                   | +0c8
        cmpi.w  #0x0,0x70(a6)                   | +0cc
        bgt.w   .L09b1c6                        | +0d2
        lea     0x2f6a5c.l,a0                   | +0d6
        jsr     0x28cd4.l                       | +0dc
        move.w  #0x7fff,0x70(a6)                | +0e2
.L09b1c6:
        bra.w   Item_Food_PickupCheck_09b0c6  | +0e8
        clr.b   0x9c(a6)                        | +0ec
        cmpi.b  #0xff,0x9a(a6)                  | +0f0
        beq.w   .L09b1de                        | +0f6
        eori.b  #0x1,0x3a(a6)                   | +0fa
.L09b1de:
        jsr     Item_LifetimeFromParam_09bb3c(pc) | +100
        cmpi.w  #0x46,d0                        | +104
        bgt.w   .L09b1ee                        | +108
        move.w  #0x46,d0                        | +10c
.L09b1ee:
        subi.w  #0x46,d0                        | +110
        move.w  d0,0x70(a6)                     | +114
        clr.w   0x72(a6)                        | +118
        move.w  #0x1c6,d1                       | +11c
        jsr     0x236e.l                        | +120
        move.w  #0x8000,d0                      | +126
        jsr     0x28134.l                       | +12a
        andi.w  #0xffe3,0x38(a6)                | +130
        ori.w   #0x0,0x38(a6)                   | +136
        lea     0x2f6f06.l,a0                   | +13c
        jsr     0x28cd4.l                       | +142
        lea     .L09b22c(pc),a1                 | +148
        move.l  a1,(a6)                         | +14c
.L09b22c:
        jsr     Item_PhysAndProbe_09b8c0(pc)    | +14e
        jsr     0x28d70.l                       | +152
        subq.w  #0x1,0x70(a6)                   | +158
        cmpi.w  #0x0,0x70(a6)                   | +15c
        bgt.w   .L09b24a                        | +162
        lea     Item_Food_Rot_09b24e(pc),a1     | +166
        move.l  a1,(a6)                         | +16a
.L09b24a:
        bra.w   Item_Food_PickupCheck_09b0c6  | +16c

| ----------------------------------------------------------------------------
|  Item_Food_Rot_09b24e  @ $09B24E  (64 B)
| ----------------------------------------------------------------------------
        .section .text.Item_Food_Rot_09b24e, "ax", @progbits
        .global Item_Food_Rot_09b24e
Item_Food_Rot_09b24e:
        lea     0x2f6f34.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        move.w  #0x8000,d0                      | +00c
        jsr     0x28134.l                       | +010
        andi.w  #0xffe3,0x38(a6)                | +016
        ori.w   #0x0,0x38(a6)                   | +01c
        lea     .L09b276(pc),a1                 | +022
        move.l  a1,(a6)                         | +026
.L09b276:
        jsr     Item_PhysAndProbe_09b8c0(pc)    | +028
        jsr     0x28d70.l                       | +02c
        bcc.w   .L09b28a                        | +032
        lea     Item_Free_09b474(pc),a1         | +036
        move.l  a1,(a6)                         | +03a
.L09b28a:
        bra.w   Item_Food_PickupCheck_09b0c6  | +03c

| ----------------------------------------------------------------------------
|  Item_Food_Thrown_09b28e  @ $09B28E  (200 B)
| ----------------------------------------------------------------------------
        .section .text.Item_Food_Thrown_09b28e, "ax", @progbits
        .global Item_Food_Thrown_09b28e
Item_Food_Thrown_09b28e:
        move.w  #0x1ac,d1                       | +000
        jsr     0x236e.l                        | +004
        jsr     0x5e9b6.l                       | +00a
        andi.b  #0x1,d0                         | +010
        move.b  d0,0x3a(a6)                     | +014
        jsr     0x5e9b6.l                       | +018
        andi.w  #0x3,d0                         | +01e
        move.w  d0,d1                           | +022
        add.w   d0,d0                           | +024
        add.w   d1,d0                           | +026
        add.w   d0,d0                           | +028
        move.w  d0,0x5c(a6)                     | +02a
        movea.l #0x2f7a40,a0                    | +02e
        move.w  (a0,d0.w),d1                    | +034
        move.w  0x2(a0,d0.w),d2                 | +038
        move.w  0x4(a0,d0.w),d3                 | +03c
        btst    #0x0,0x3a(a6)                   | +040
        bne.w   .L09b2da                        | +046
        neg.w   d1                              | +04a
.L09b2da:
        move.w  d1,0x28(a6)                     | +04c
        move.w  d2,0x2e(a6)                     | +050
        move.w  d3,0x2a(a6)                     | +054
        move.b  #0x1,0x9c(a6)                   | +058
        move.b  #0xa,0x99(a6)                   | +05e
        jsr     Item_LifetimeFromParam_09bb3c(pc) | +064
        move.w  d0,0x70(a6)                     | +068
        move.b  #0x8,0x45(a6)                   | +06c
        move.w  #0x8000,d0                      | +072
        jsr     0x28134.l                       | +076
        andi.w  #0xffe3,0x38(a6)                | +07c
        ori.w   #0x0,0x38(a6)                   | +082
        lea     0x2f7002.l,a0                   | +088
        jsr     0x28cd4.l                       | +08e
        lea     .L09b328(pc),a1                 | +094
        move.l  a1,(a6)                         | +098
.L09b328:
        jsr     0x27d50.l                       | +09a
        bcc.w   .L09b338                        | +0a0
        lea     Item_Food_Bounce_09b356(pc),a1  | +0a4
        move.l  a1,(a6)                         | +0a8
.L09b338:
        jsr     0x28d70.l                       | +0aa
        subq.w  #0x1,0x70(a6)                   | +0b0
        cmpi.w  #0x0,0x70(a6)                   | +0b4
        bgt.w   .L09b352                        | +0ba
        lea     Item_Food_Blink_09b0d8(pc),a1   | +0be
        move.l  a1,(a6)                         | +0c2
.L09b352:
        bra.w   Item_GroundTail_09a75a          | +0c4

| ----------------------------------------------------------------------------
|  Item_Food_Bounce_09b356  @ $09B356  (112 B)
| ----------------------------------------------------------------------------
        .section .text.Item_Food_Bounce_09b356, "ax", @progbits
        .global Item_Food_Bounce_09b356
Item_Food_Bounce_09b356:
        movea.l #0x2f7a58,a0                    | +000
        move.w  0x5c(a6),d0                     | +006
        move.w  (a0,d0.w),d1                    | +00a
        move.w  0x2(a0,d0.w),d2                 | +00e
        move.w  0x4(a0,d0.w),d3                 | +012
        btst    #0x0,0x3a(a6)                   | +016
        bne.w   .L09b378                        | +01c
        neg.w   d1                              | +020
.L09b378:
        move.w  d1,0x28(a6)                     | +022
        move.w  d2,0x2e(a6)                     | +026
        move.w  d3,0x2a(a6)                     | +02a
        lea     .L09b38a(pc),a1                 | +02e
        move.l  a1,(a6)                         | +032
.L09b38a:
        jsr     0x27d50.l                       | +034
        bcc.w   .L09b39a                        | +03a
        lea     Item_Food_Wait_09b0a8(pc),a1    | +03e
        move.l  a1,(a6)                         | +042
.L09b39a:
        jsr     0x28d70.l                       | +044
        subq.w  #0x1,0x70(a6)                   | +04a
        cmpi.w  #0x0,0x70(a6)                   | +04e
        bgt.w   .L09b3b4                        | +054
        lea     Item_Food_Blink_09b0d8(pc),a1   | +058
        move.l  a1,(a6)                         | +05c
.L09b3b4:
        jsr     Item_Food_Pickup_09b920(pc)     | +05e
        bcc.w   .L09b3c2                        | +062
        lea     Item_Free_09b474(pc),a1         | +066
        move.l  a1,(a6)                         | +06a
.L09b3c2:
        bra.w   Item_GroundTail_09a75a          | +06c

| ----------------------------------------------------------------------------
|  Item_Tmpl293_Pow_09b3c6  @ $09B3C6  (120 B)
| ----------------------------------------------------------------------------
        .section .text.Item_Tmpl293_Pow_09b3c6, "ax", @progbits
        .global Item_Tmpl293_Pow_09b3c6
Item_Tmpl293_Pow_09b3c6:
        move.w  #0x1ab,d1                       | +000
        jsr     0x236e.l                        | +004
        move.l  #0x9a672,0x60(a6)               | +00a
        move.w  #0x8000,d0                      | +012
        jsr     0x28134.l                       | +016
        andi.w  #0xffe3,0x38(a6)                | +01c
        ori.w   #0x0,0x38(a6)                   | +022
        lea     0x2f772a.l,a0                   | +028
        jsr     0x28cd4.l                       | +02e
        lea     .L09b400(pc),a1                 | +034
        move.l  a1,(a6)                         | +038
.L09b400:
        jsr     Item_PhysAndProbe_09b8c0(pc)    | +03a
        jsr     Item_TrailRecord_09b8ee(pc)     | +03e
        jsr     0x28998.l                       | +042
        move.w  #0x8000,0x38(a6)                | +048
        jsr     0x28d70.l                       | +04e
        jsr     0x2870a.l                       | +054
        bcc.w   .L09b42a                        | +05a
        bclr    #0x3,0x13(a6)                   | +05e
.L09b42a:
        jsr     0x28758.l                       | +064
        bcc.w   .L09b43a                        | +06a
        lea     Item_Pow_Die_09b43e(pc),a1      | +06e
        move.l  a1,(a6)                         | +072
.L09b43a:
        bra.w   Item_GroundTail_NoCount_09a76a | +074

| ----------------------------------------------------------------------------
|  Item_Pow_Die_09b43e  @ $09B43E  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Item_Pow_Die_09b43e, "ax", @progbits
        .global Item_Pow_Die_09b43e
Item_Pow_Die_09b43e:
        move.w  #0x102e,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x2f773c.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L09b45a(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L09b45a:
        jsr     0x2783a.l                       | +01c
        jsr     0x28d70.l                       | +022
        bcc.w   .L09b470                        | +028
        lea     Item_Free_09b474(pc),a1         | +02c
        move.l  a1,(a6)                         | +030
.L09b470:
        bra.w   Item_GroundTail_NoCount_09a76a | +032

| ----------------------------------------------------------------------------
|  Item_Free_09b474  @ $09B474  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Item_Free_09b474, "ax", @progbits
        .global Item_Free_09b474
Item_Free_09b474:
        jmp     0x518.l                         | +000

| ----------------------------------------------------------------------------
|  Item_Rts_09b47a  @ $09B47A  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Item_Rts_09b47a, "ax", @progbits
        .global Item_Rts_09b47a
Item_Rts_09b47a:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  Score_Popup_Table_09b48a  @ $09B48A  (148 B)
| ----------------------------------------------------------------------------
        .section .text.Score_Popup_Table_09b48a, "ax", @progbits
        .global Score_Popup_Table_09b48a
Score_Popup_Table_09b48a:
        move.b  0x68(a6),0x6e(a6)               | +000
        move.b  0x98(a6),d1                     | +006
        lea     0x2f79a4.l,a1                   | +00a
        andi.w  #0x3,d1                         | +010
        lsl.w   #0x2,d1                         | +014
        movea.l (a1,d1.w),a1                    | +016
        move.b  (a1),d1                         | +01a
        addq.l  #0x1,a1                         | +01c
        move.l  a1,0x5c(a6)                     | +01e
        move.b  d1,0x72(a6)                     | +022
        move.b  d1,0x21(a6)                     | +026
        andi.w  #0xf,d1                         | +02a
        move.w  #0x8,d0                         | +02e
        muls.w  d1,d0                           | +032
        add.w   d0,0x22(a6)                     | +034
        clr.b   0x3a(a6)                        | +038
        jsr     0x5e9b6.l                       | +03c
        andi.w  #0xe,d0                         | +042
        addi.w  #0x9,d0                         | +046
        add.w   d0,0x24(a6)                     | +04a
.L09b4d8:
        lea     Score_Digit_09b66a(pc),a1       | +04e
        jsr     0x4ae.l                         | +052
        jsr     0x5dd02.l                       | +058
        move.b  0x68(a6),0x68(a0)               | +05e
        subq.w  #0x8,0x22(a6)                   | +064
        movea.l 0x5c(a6),a1                     | +068
        addq.l  #0x1,0x5c(a6)                   | +06c
        move.b  (a1),d0                         | +070
        move.w  d0,0x74(a0)                     | +072
        move.b  0x72(a6),d0                     | +076
        andi.w  #0x7,d0                         | +07a
        lsl.w   #0x1,d0                         | +07e
        move.w  d0,0x70(a0)                     | +080
        subq.b  #0x1,0x72(a6)                   | +084
        cmpi.b  #0x0,0x72(a6)                   | +088
        bgt.b   .L09b4d8                        | +08e
        bra.w   Score_Popup_Run_09b5e4 | +090

| ----------------------------------------------------------------------------
|  Score_Popup_Value_09b51e  @ $09B51E  (324 B)
| ----------------------------------------------------------------------------
        .section .text.Score_Popup_Value_09b51e, "ax", @progbits
        .global Score_Popup_Value_09b51e
Score_Popup_Value_09b51e:
        move.b  0x68(a6),0x6e(a6)               | +000
        move.b  0x98(a6),d1                     | +006
        lea     0x2f792c.l,a1                   | +00a
        tst.b   0x9c(a6)                        | +010
        beq.w   .L09b53c                        | +014
        lea     0x2f794c.l,a1                   | +018
.L09b53c:
        andi.w  #0xf,d1                         | +01e
        lsl.w   #0x2,d1                         | +022
        move.l  (a1,d1.w),d0                    | +024
        move.l  d0,0x80(a6)                     | +028
        jsr     0x51a44.l                       | +02c
        move.b  0x98(a6),d1                     | +032
        lea     0x2f798c.l,a1                   | +036
        tst.b   0x9c(a6)                        | +03c
        beq.w   .L09b568                        | +040
        lea     0x2f7994.l,a1                   | +044
.L09b568:
        andi.w  #0xf,d1                         | +04a
        move.b  (a1,d1.w),d1                    | +04e
        move.b  d1,0x72(a6)                     | +052
        move.b  d1,0x21(a6)                     | +056
        andi.w  #0xf,d1                         | +05a
        move.w  #0x8,d0                         | +05e
        muls.w  d1,d0                           | +062
        add.w   d0,0x22(a6)                     | +064
        .global Score_Popup_Value_Spawn_09b586
Score_Popup_Value_Spawn_09b586:
        jsr     0x5e9b6.l                       | +068
        andi.w  #0xe,d0                         | +06e
        addi.w  #0x9,d0                         | +072
        add.w   d0,0x24(a6)                     | +076
        clr.b   0x3a(a6)                        | +07a
.L09b59c:
        lea     Score_Digit_09b66a(pc),a1       | +07e
        jsr     0x4ae.l                         | +082
        jsr     0x5dd02.l                       | +088
        move.b  0x68(a6),0x68(a0)               | +08e
        subq.w  #0x8,0x22(a6)                   | +094
        move.l  0x80(a6),d0                     | +098
        move.l  d0,d1                           | +09c
        lsr.l   #0x4,d1                         | +09e
        move.l  d1,0x80(a6)                     | +0a0
        andi.w  #0xf,d0                         | +0a4
        move.w  d0,0x74(a0)                     | +0a8
        move.b  0x72(a6),d0                     | +0ac
        andi.w  #0x7,d0                         | +0b0
        lsl.w   #0x1,d0                         | +0b4
        move.w  d0,0x70(a0)                     | +0b6
        subq.b  #0x1,0x72(a6)                   | +0ba
        cmpi.b  #0x0,0x72(a6)                   | +0be
        bgt.b   .L09b59c                        | +0c4
        .global Score_Popup_Run_09b5e4
Score_Popup_Run_09b5e4:
        move.w  #0x1e,0x70(a6)                  | +0c6
        move.w  #0x96,0x84(a6)                  | +0cc
        lea     .L09b5f6(pc),a1                 | +0d2
        move.l  a1,(a6)                         | +0d6
.L09b5f6:
        jsr     0x2783a.l                       | +0d8
        cmpi.b  #0xff,0x21(a6)                  | +0de
        beq.w   .L09b60e                        | +0e4
        tst.b   0x21(a6)                        | +0e8
        bne.w   .L09b63a                        | +0ec
.L09b60e:
        subq.w  #0x1,0x70(a6)                   | +0f0
        cmpi.w  #0x0,0x70(a6)                   | +0f4
        bgt.w   .L09b63a                        | +0fa
        tst.b   0x21(a6)                        | +0fe
        bne.w   .L09b634                        | +102
        move.b  #0xff,0x21(a6)                  | +106
        move.w  #0xa,0x70(a6)                   | +10c
        bra.w   .L09b63a                        | +112
.L09b634:
        lea     Jsr5B6ThenJmpScheduler_09b47c(pc),a1 | +116
        move.l  a1,(a6)                         | +11a
.L09b63a:
        subq.w  #0x1,0x84(a6)                   | +11c
        cmpi.w  #0x0,0x84(a6)                   | +120
        bgt.w   .L09b64e                        | +126
        lea     Jsr5B6ThenJmpScheduler_09b47c(pc),a1 | +12a
        move.l  a1,(a6)                         | +12e
.L09b64e:
        movea.l #0xffffffff,a0                  | +130
        lea     Item_SpawnTable_Hitbox_09a6c6(pc),a0 | +136
        jsr     0x5dd56.l                       | +13a
        bcc.w   SetHandlerRts_09b668            | +140

| ----------------------------------------------------------------------------
|  Score_Digit_09b66a  @ $09B66A  (232 B)
| ----------------------------------------------------------------------------
        .section .text.Score_Digit_09b66a, "ax", @progbits
        .global Score_Digit_09b66a
Score_Digit_09b66a:
        move.b  0x68(a6),d0                     | +000
        cmpi.b  #0x0,d0                         | +004
        bne.w   .L09b68e                        | +008
        move.w  #0x1b8,d1                       | +00c
        jsr     0x236e.l                        | +010
        move.w  #0x1b9,d1                       | +016
        jsr     0x236e.l                        | +01a
        bra.w   .L09b6a2                        | +020
.L09b68e:
        move.w  #0x1cd,d1                       | +024
        jsr     0x236e.l                        | +028
        move.w  #0x1ce,d1                       | +02e
        jsr     0x236e.l                        | +032
.L09b6a2:
        move.w  #0xc000,d0                      | +038
        jsr     0x28134.l                       | +03c
        andi.w  #0xffe3,0x38(a6)                | +042
        ori.w   #0x1c,0x38(a6)                  | +048
        move.w  #0x0,d0                         | +04e
        jsr     0x5dca4.l                       | +052
        move.w  d0,0x28(a6)                     | +058
        move.w  #0x886,0x2a(a6)                 | +05c
        move.w  #0xfedd,0x2e(a6)                | +062
        move.w  #0x0,0x2c(a6)                   | +068
        move.w  0x74(a6),d0                     | +06e
        andi.w  #0x1f,d0                        | +072
        movea.l #0x2f77a4,a0                    | +076
        lsl.w   #0x2,d0                         | +07c
        movea.l (a0,d0.w),a0                    | +07e
        cmpa.l  #0xffffffff,a0                  | +082
        beq.w   .L09b6fc                        | +088
        jsr     0x28cd4.l                       | +08c
.L09b6fc:
        lea     .L09b702(pc),a1                 | +092
        move.l  a1,(a6)                         | +096
.L09b702:
        jsr     0x2783a.l                       | +098
        subq.w  #0x1,0x70(a6)                   | +09e
        cmpi.w  #0x0,0x70(a6)                   | +0a2
        bgt.w   .L09b748                        | +0a8
        jsr     0x27cee.l                       | +0ac
        movea.l 0xc(a6),a0                      | +0b2
        move.w  0x24(a0),d0                     | +0b6
        cmp.w   0x24(a6),d0                     | +0ba
        ble.w   .L09b73e                        | +0be
        addi.w  #0x10,d0                        | +0c2
        move.w  d0,0x24(a6)                     | +0c6
        subq.b  #0x1,0x21(a0)                   | +0ca
        lea     Score_Digit_Lift_09b75a(pc),a1  | +0ce
        move.l  a1,(a6)                         | +0d2
.L09b73e:
        jsr     Item_SelectAnimParity_09bb22(pc) | +0d4
        jsr     0x28d70.l                       | +0d8
.L09b748:
        jsr     0x5e45a.l                       | +0de
        bcc.w   SetHandlerRts_09b758            | +0e4

| ----------------------------------------------------------------------------
|  Score_Digit_Lift_09b75a  @ $09B75A  (90 B)
| ----------------------------------------------------------------------------
        .section .text.Score_Digit_Lift_09b75a, "ax", @progbits
        .global Score_Digit_Lift_09b75a
Score_Digit_Lift_09b75a:
        jsr     0x267e2.l                       | +000
        move.w  #0x400,0x2a(a6)                 | +006
        move.w  #0x2,0x70(a6)                   | +00c
        lea     .L09b772(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L09b772:
        jsr     0x27cee.l                       | +018
        jsr     Item_SelectAnimParity_09bb22(pc) | +01e
        jsr     0x28d70.l                       | +022
        subq.w  #0x1,0x70(a6)                   | +028
        cmpi.w  #0x0,0x70(a6)                   | +02c
        bgt.w   .L09b796                        | +032
        lea     Score_Digit_Hold_09b7bc(pc),a1  | +036
        move.l  a1,(a6)                         | +03a
.L09b796:
        movea.l 0xc(a6),a0                      | +03c
        cmpi.b  #0xff,0x21(a0)                  | +040
        bne.w   .L09b7aa                        | +046
        lea     Score_Digit_Blink_09b7f8(pc),a1 | +04a
        move.l  a1,(a6)                         | +04e
.L09b7aa:
        jsr     0x5e45a.l                       | +050
        bcc.w   SetHandlerRts_09b7ba            | +056

| ----------------------------------------------------------------------------
|  Score_Digit_Hold_09b7bc  @ $09B7BC  (52 B)
| ----------------------------------------------------------------------------
        .section .text.Score_Digit_Hold_09b7bc, "ax", @progbits
        .global Score_Digit_Hold_09b7bc
Score_Digit_Hold_09b7bc:
        lea     .L09b7c2(pc),a1                 | +000
        move.l  a1,(a6)                         | +004
.L09b7c2:
        jsr     0x2783a.l                       | +006
        jsr     Item_SelectAnimParity_09bb22(pc) | +00c
        jsr     0x28d70.l                       | +010
        movea.l 0xc(a6),a0                      | +016
        cmpi.b  #0xff,0x21(a0)                  | +01a
        bne.w   .L09b7e6                        | +020
        lea     Score_Digit_Blink_09b7f8(pc),a1 | +024
        move.l  a1,(a6)                         | +028
.L09b7e6:
        jsr     0x5e45a.l                       | +02a
        bcc.w   SetHandlerRts_09b7f6            | +030

| ----------------------------------------------------------------------------
|  Score_Digit_Blink_09b7f8  @ $09B7F8  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Score_Digit_Blink_09b7f8, "ax", @progbits
        .global Score_Digit_Blink_09b7f8
Score_Digit_Blink_09b7f8:
        lea     .L09b7fe(pc),a1                 | +000
        move.l  a1,(a6)                         | +004
.L09b7fe:
        jsr     0x2783a.l                       | +006
        btst    #0x0,0x106f28.l                 | +00c
        beq.w   .L09b81a                        | +014
        jsr     Item_SelectAnimParity_09bb22(pc) | +018
        jsr     0x28d70.l                       | +01c
.L09b81a:
        jsr     0x5e45a.l                       | +022
        bcc.w   SetHandlerRts_09b82a            | +028

| ----------------------------------------------------------------------------
|  Item_Tmpl316_ComboTimer_09b82c  @ $09B82C  (70 B)
| ----------------------------------------------------------------------------
        .section .text.Item_Tmpl316_ComboTimer_09b82c, "ax", @progbits
        .global Item_Tmpl316_ComboTimer_09b82c
Item_Tmpl316_ComboTimer_09b82c:
        clr.b   0x10e488.l                      | +000
        clr.b   0x10e489.l                      | +006
        lea     .L09b83e(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L09b83e:
        subq.b  #0x1,0x10e488.l                 | +012
        cmpi.b  #0x0,0x10e488.l                 | +018
        bgt.w   .L09b85c                        | +020
        clr.b   0x10e488.l                      | +024
        clr.b   0x10e489.l                      | +02a
.L09b85c:
        cmpi.b  #0xe,0x10e489.l                 | +030
        ble.w   .L09b870                        | +038
        move.b  #0xe,0x10e489.l                 | +03c
.L09b870:
        rts                                     | +044

| ----------------------------------------------------------------------------
|  Item_SpawnAtOffset_09b872  @ $09B872  (76 B)
| ----------------------------------------------------------------------------
        .section .text.Item_SpawnAtOffset_09b872, "ax", @progbits
        .global Item_SpawnAtOffset_09b872
Item_SpawnAtOffset_09b872:
        jsr     0x4cac4.l                       | +000
        tst.w   0x98(a6)                        | +006
        bne.w   .L09b886                        | +00a
        add.w   d3,d0                           | +00e
        bra.w   .L09b888                        | +010
.L09b886:
        sub.w   d3,d0                           | +014
.L09b888:
        addq.w  #0x8,d1                         | +016
        move.w  d0,0x22(a6)                     | +018
        move.w  d1,0x24(a6)                     | +01c
        move.b  d4,0x98(a6)                     | +020
        move.b  d5,0x99(a6)                     | +024
        move.b  d6,0x9a(a6)                     | +028
        move.b  0x98(a6),d0                     | +02c
        move.w  #0x99,d1                        | +030
        jsr     Entity_ProbeMoveX_09A7AA(pc)    | +034
        bcs.w   .L09b8b8                        | +038
        move.l  (a0),0x5c(a0)                   | +03c
        move.l  #0x9a8be,(a0)                   | +040
.L09b8b8:
        jmp     0x518.l                         | +046

| ----------------------------------------------------------------------------
|  Item_Rts_09b8be  @ $09B8BE  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Item_Rts_09b8be, "ax", @progbits
        .global Item_Rts_09b8be
Item_Rts_09b8be:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  Item_PhysAndProbe_09b8c0  @ $09B8C0  (40 B)
| ----------------------------------------------------------------------------
        .section .text.Item_PhysAndProbe_09b8c0, "ax", @progbits
        .global Item_PhysAndProbe_09b8c0
Item_PhysAndProbe_09b8c0:
        jsr     0x2783a.l                       | +000
        tst.b   0x9b(a6)                        | +006
        beq.w   .L09b8d8                        | +00a
        cmpi.b  #0xff,0x9b(a6)                  | +00e
        bne.w   ClearXN_09b8e8                  | +014
.L09b8d8:
        jsr     0x27eba.l                       | +018
        bcc.w   ClearXN_09b8e8                  | +01e
        jsr     0x27c8c.l                       | +022

| ----------------------------------------------------------------------------
|  Item_TrailRecord_09b8ee  @ $09B8EE  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Item_TrailRecord_09b8ee, "ax", @progbits
        .global Item_TrailRecord_09b8ee
Item_TrailRecord_09b8ee:
        move.w  0x22(a6),d0                     | +000
        move.w  0x24(a6),d1                     | +004
        subi.w  #0x10,d0                        | +008
        addi.w  #0x10,d1                        | +00c
        move.w  #0x20,d2                        | +010

| ----------------------------------------------------------------------------
|  Item_RespawnBelow_09b90a  @ $09B90A  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Item_RespawnBelow_09b90a, "ax", @progbits
        .global Item_RespawnBelow_09b90a
Item_RespawnBelow_09b90a:
        move.b  0x98(a6),d0                     | +000
        move.w  #0x99,d1                        | +004
        jsr     Entity_ProbeMoveX_09A7AA(pc)    | +008
        bcs.w   .L09b91e                        | +00c
        addq.w  #0x4,0x24(a0)                   | +010
.L09b91e:
        rts                                     | +014

| ----------------------------------------------------------------------------
|  Item_Food_Pickup_09b920  @ $09B920  (142 B)
| ----------------------------------------------------------------------------
        .section .text.Item_Food_Pickup_09b920, "ax", @progbits
        .global Item_Food_Pickup_09b920
Item_Food_Pickup_09b920:
        jsr     Item_PickupTestB_09a84e(pc)     | +000
        bcc.w   ClearXN_09b9b4                  | +004
        move.b  d0,0x68(a6)                     | +008
        move.w  #0x1121,d0                      | +00c
        jsr     0x2352.l                        | +010
        lea     Score_Popup_Value_09b51e(pc),a1 | +016
        jsr     0x4ae.l                         | +01a
        jsr     0x5dd02.l                       | +020
        move.b  0x68(a6),0x68(a0)               | +026
        move.b  0x98(a6),d1                     | +02c
        move.b  0x9c(a6),d0                     | +030
        move.b  d0,0x9c(a0)                     | +034
        tst.b   d0                              | +038
        beq.w   .L09b99e                        | +03a
        cmpi.b  #0x2,d0                         | +03e
        bne.w   .L09b97e                        | +042
        move.b  #0xf,d1                         | +046
        move.b  #0xe,0x10e489.l                 | +04a
        move.b  #0x2d,0x10e488.l                | +052
        bra.w   .L09b9aa                        | +05a
.L09b97e:
        move.b  0x10e489.l,d2                   | +05e
        cmp.b   d1,d2                           | +064
        ble.w   .L09b98c                        | +066
        move.b  d2,d1                           | +06a
.L09b98c:
        move.w  d1,d2                           | +06c
        addq.b  #0x1,d2                         | +06e
        move.b  d2,0x10e489.l                   | +070
        move.b  #0x2d,0x10e488.l                | +076
.L09b99e:
        cmpi.b  #0xe,d1                         | +07e
        ble.w   .L09b9aa                        | +082
        move.b  #0xe,d1                         | +086
.L09b9aa:
        move.b  d1,0x98(a0)                     | +08a

| ----------------------------------------------------------------------------
|  Score_Popup_ForPlayer_09b9ba  @ $09B9BA  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Score_Popup_ForPlayer_09b9ba, "ax", @progbits
        .global Score_Popup_ForPlayer_09b9ba
Score_Popup_ForPlayer_09b9ba:
        move.l  a1,d1                           | +000
        clr.b   d0                              | +002
        cmpi.l  #0x100440,d1                    | +004
        beq.w   .L09b9d6                        | +00a
        addq.b  #0x1,d0                         | +00e
        cmpi.l  #0x1004e0,d1                    | +010
        beq.w   .L09b9d6                        | +016
        rts                                     | +01a
.L09b9d6:
        move.l  d0,-(a7)                        | +01c
        lea     Score_Popup_Table_09b48a(pc),a1 | +01e
        jsr     0x4ae.l                         | +022
        jsr     0x5dd02.l                       | +028
        move.l  (a7)+,d0                        | +02e
        move.b  d0,0x68(a0)                     | +030
        move.b  #0x2,0x98(a0)                   | +034
        rts                                     | +03a

| ----------------------------------------------------------------------------
|  Score_Popup_Kind3_09ba34  @ $09BA34  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Score_Popup_Kind3_09ba34, "ax", @progbits
        .global Score_Popup_Kind3_09ba34
Score_Popup_Kind3_09ba34:
        lea     Score_Popup_Value_09b51e(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        move.b  0x68(a6),0x68(a0)               | +010
        move.b  #0x3,0x98(a0)                   | +016
        clr.b   0x9c(a0)                        | +01c
        rts                                     | +020

| ----------------------------------------------------------------------------
|  Score_Popup_Kind4_09ba56  @ $09BA56  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Score_Popup_Kind4_09ba56, "ax", @progbits
        .global Score_Popup_Kind4_09ba56
Score_Popup_Kind4_09ba56:
        lea     Score_Popup_Value_09b51e(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        move.b  0x6e(a6),0x68(a0)               | +010
        move.b  #0x4,0x98(a0)                   | +016
        clr.b   0x9c(a0)                        | +01c
        rts                                     | +020

| ----------------------------------------------------------------------------
|  Item_Food_Spawn_09ba78  @ $09BA78  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Item_Food_Spawn_09ba78, "ax", @progbits
        .global Item_Food_Spawn_09ba78
Item_Food_Spawn_09ba78:
        lea     Item_Food_Thrown_09b28e(pc),a1  | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        move.b  0x68(a6),0x68(a0)               | +010
        move.b  0x6e(a6),0x6e(a0)               | +016
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  Score_Popup_Value_Alloc_09ba96  @ $09BA96  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Score_Popup_Value_Alloc_09ba96, "ax", @progbits
        .global Score_Popup_Value_Alloc_09ba96
Score_Popup_Value_Alloc_09ba96:
        movem.l d0-d2,-(a7)                     | +000
        lea     Score_Popup_Value_Spawn_09b586(pc),a1 | +004
        jsr     0x4ae.l                         | +008
        jsr     0x5dd02.l                       | +00e
        movem.l (a7)+,d0-d2                     | +014
        move.l  d0,0x80(a6)                     | +018
        move.b  d2,0x68(a6)                     | +01c
        move.b  d2,0x6e(a6)                     | +020
        move.b  d1,0x72(a0)                     | +024
        move.b  d1,0x21(a0)                     | +028
        andi.w  #0xf,d1                         | +02c
        move.w  #0x8,d0                         | +030
        muls.w  d1,d0                           | +034
        add.w   d0,0x22(a0)                     | +036
        rts                                     | +03a

| ----------------------------------------------------------------------------
|  Score_Popup_Value_AllocAt_09bad2  @ $09BAD2  (80 B)
| ----------------------------------------------------------------------------
        .section .text.Score_Popup_Value_AllocAt_09bad2, "ax", @progbits
        .global Score_Popup_Value_AllocAt_09bad2
Score_Popup_Value_AllocAt_09bad2:
        movem.l d0-d2,-(a7)                     | +000
        lea     Score_Popup_Value_Spawn_09b586(pc),a1 | +004
        jsr     0x4ae.l                         | +008
        move.w  d3,0x22(a0)                     | +00e
        move.w  d4,0x24(a0)                     | +012
        move.w  0x38(a6),0x38(a0)               | +016
        move.b  0x3a(a6),0x3a(a0)               | +01c
        move.b  0x11(a6),0x11(a0)               | +022
        movem.l (a7)+,d0-d2                     | +028
        move.l  d0,0x80(a6)                     | +02c
        move.b  d2,0x68(a6)                     | +030
        move.b  d2,0x6e(a6)                     | +034
        move.b  d1,0x72(a0)                     | +038
        move.b  d1,0x21(a0)                     | +03c
        andi.w  #0xf,d1                         | +040
        move.w  #0x8,d0                         | +044
        muls.w  d1,d0                           | +048
        move.w  d0,0x5c(a0)                     | +04a
        rts                                     | +04e

| ----------------------------------------------------------------------------
|  Item_SelectAnimParity_09bb22  @ $09BB22  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Item_SelectAnimParity_09bb22, "ax", @progbits
        .global Item_SelectAnimParity_09bb22
Item_SelectAnimParity_09bb22:
        move.w  0x16(a6),d0                     | +000
        btst    #0x0,0x106f28.l                 | +004
        beq.w   SetTaskW_09bb36                 | +00c
        move.w  0x18(a6),d0                     | +010

| ----------------------------------------------------------------------------
|  Item_LifetimeFromParam_09bb3c  @ $09BB3C  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Item_LifetimeFromParam_09bb3c, "ax", @progbits
        .global Item_LifetimeFromParam_09bb3c
Item_LifetimeFromParam_09bb3c:
        move.b  0x99(a6),d0                     | +000
        move.b  0x99(a6),d0                     | +004
        andi.w  #0xff,d0                        | +008
        lsl.w   #0x4,d0                         | +00c
        tst.w   d0                              | +00e
        bne.w   .L09bb54                        | +010
        move.w  #0x7fff,d0                      | +014
.L09bb54:
        rts                                     | +018

| ----------------------------------------------------------------------------
|  Item_Tmpl317_Static_09bb56  @ $09BB56  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Item_Tmpl317_Static_09bb56, "ax", @progbits
        .global Item_Tmpl317_Static_09bb56
Item_Tmpl317_Static_09bb56:
        jsr     0x5e7c0.l                       | +000
        lea     Item_Tmpl317_BlitList_09bb92(pc),a1 | +006
        jsr     0x43fac.l                       | +00a
        move.w  #0x1c6,d1                       | +010
        jsr     0x236e.l                        | +014
        lea     0x2f73b8.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        lea     .L09bb82(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L09bb82:
        jsr     0x2783a.l                       | +02c
        jsr     0x28d70.l                       | +032
        bra.w   Item_GroundTail_NoCount_09a76a | +038

| ----------------------------------------------------------------------------
|  Item_Tmpl317_BlitList_09bb92  @ $09BB92  (536 B)
| ----------------------------------------------------------------------------
        .section .text.Item_Tmpl317_BlitList_09bb92, "ax", @progbits
        .global Item_Tmpl317_BlitList_09bb92
Item_Tmpl317_BlitList_09bb92:
        .dc.w   0xff80                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0057                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +010  (dato / opcode no decodificado)
        .dc.w   0x141e                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x151e                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +028  (dato / opcode no decodificado)
        .dc.w   0x161e                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +034  (dato / opcode no decodificado)
        .dc.w   0x171e                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +038  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +040  (dato / opcode no decodificado)
        .dc.w   0x1e01                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +044  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0015                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1e01                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0016                        | +058  (dato / opcode no decodificado)
        .dc.w   0x1e01                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +060  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +062  (dato / opcode no decodificado)
        .dc.w   0x0017                        | +064  (dato / opcode no decodificado)
        .dc.w   0x1e01                        | +066  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +068  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x141e                        | +070  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +072  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +074  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x151e                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +07e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +080  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +082  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +084  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +086  (dato / opcode no decodificado)
        .dc.w   0x161e                        | +088  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +08a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +090  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +092  (dato / opcode no decodificado)
        .dc.w   0x171e                        | +094  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +096  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +098  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +09a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +09e  (dato / opcode no decodificado)
        .dc.w   0x1e00                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +0a2  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0a4  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0a6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0a8  (dato / opcode no decodificado)
        .dc.w   0x0015                        | +0aa  (dato / opcode no decodificado)
        .dc.w   0x1e00                        | +0ac  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +0ae  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0b0  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0b2  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0b4  (dato / opcode no decodificado)
        .dc.w   0x0016                        | +0b6  (dato / opcode no decodificado)
        .dc.w   0x1e00                        | +0b8  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +0ba  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0bc  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0be  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0c0  (dato / opcode no decodificado)
        .dc.w   0x0017                        | +0c2  (dato / opcode no decodificado)
        .dc.w   0x1e00                        | +0c4  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +0c6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0c8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0ca  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0cc  (dato / opcode no decodificado)
        .dc.w   0x141e                        | +0ce  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0d0  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +0d2  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0d4  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0d6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0d8  (dato / opcode no decodificado)
        .dc.w   0x151e                        | +0da  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0dc  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +0de  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0e0  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0e2  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0e4  (dato / opcode no decodificado)
        .dc.w   0x161e                        | +0e6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0e8  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +0ea  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0ec  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0ee  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0f0  (dato / opcode no decodificado)
        .dc.w   0x171e                        | +0f2  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0f4  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +0f6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0f8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0fa  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +0fc  (dato / opcode no decodificado)
        .dc.w   0x1e00                        | +0fe  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +100  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +102  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +104  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +106  (dato / opcode no decodificado)
        .dc.w   0x0015                        | +108  (dato / opcode no decodificado)
        .dc.w   0x1e00                        | +10a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +10c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +10e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +110  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +112  (dato / opcode no decodificado)
        .dc.w   0x0016                        | +114  (dato / opcode no decodificado)
        .dc.w   0x1e00                        | +116  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +118  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +11a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +11c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +11e  (dato / opcode no decodificado)
        .dc.w   0x0017                        | +120  (dato / opcode no decodificado)
        .dc.w   0x1e00                        | +122  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +124  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +126  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +128  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +12a  (dato / opcode no decodificado)
        .dc.w   0x141e                        | +12c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +12e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +130  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +132  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +134  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +136  (dato / opcode no decodificado)
        .dc.w   0x151e                        | +138  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +13a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +13c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +13e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +140  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +142  (dato / opcode no decodificado)
        .dc.w   0x161e                        | +144  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +146  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +148  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +14a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +14c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +14e  (dato / opcode no decodificado)
        .dc.w   0x171e                        | +150  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +152  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +154  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +156  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +158  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +15a  (dato / opcode no decodificado)
        .dc.w   0x1e00                        | +15c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +15e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +160  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +162  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +164  (dato / opcode no decodificado)
        .dc.w   0x0015                        | +166  (dato / opcode no decodificado)
        .dc.w   0x1e00                        | +168  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +16a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +16c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +16e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +170  (dato / opcode no decodificado)
        .dc.w   0x0016                        | +172  (dato / opcode no decodificado)
        .dc.w   0x1e00                        | +174  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +176  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +178  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +17a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +17c  (dato / opcode no decodificado)
        .dc.w   0x0017                        | +17e  (dato / opcode no decodificado)
        .dc.w   0x1e00                        | +180  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +182  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +184  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +186  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +188  (dato / opcode no decodificado)
        .dc.w   0x141e                        | +18a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +18c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +18e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +190  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +192  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +194  (dato / opcode no decodificado)
        .dc.w   0x151e                        | +196  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +198  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +19a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +19c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +19e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1a0  (dato / opcode no decodificado)
        .dc.w   0x161e                        | +1a2  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1a4  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1a6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1a8  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1aa  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1ac  (dato / opcode no decodificado)
        .dc.w   0x171e                        | +1ae  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1b0  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1b2  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1b4  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1b6  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +1b8  (dato / opcode no decodificado)
        .dc.w   0x1e00                        | +1ba  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1bc  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1be  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1c0  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1c2  (dato / opcode no decodificado)
        .dc.w   0x0015                        | +1c4  (dato / opcode no decodificado)
        .dc.w   0x1e00                        | +1c6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1c8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1ca  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1cc  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1ce  (dato / opcode no decodificado)
        .dc.w   0x0016                        | +1d0  (dato / opcode no decodificado)
        .dc.w   0x1e00                        | +1d2  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1d4  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1d6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1d8  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1da  (dato / opcode no decodificado)
        .dc.w   0x0017                        | +1dc  (dato / opcode no decodificado)
        .dc.w   0x1e00                        | +1de  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1e0  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1e2  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1e4  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1e6  (dato / opcode no decodificado)
        .dc.w   0x141e                        | +1e8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1ea  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1ec  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1ee  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1f0  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1f2  (dato / opcode no decodificado)
        .dc.w   0x151e                        | +1f4  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1f6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1f8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1fa  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1fc  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1fe  (dato / opcode no decodificado)
        .dc.w   0x161e                        | +200  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +202  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +204  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +206  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +208  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +20a  (dato / opcode no decodificado)
        .dc.w   0x171e                        | +20c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +20e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +210  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +212  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +214  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +216  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Entity_CmpPrioWithSibling_09bdaa  @ $09BDAA  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpPrioWithSibling_09bdaa, "ax", @progbits
        .global Entity_CmpPrioWithSibling_09bdaa
Entity_CmpPrioWithSibling_09bdaa:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_09bdc0                    | +00c

| ----------------------------------------------------------------------------
|  Chute_SpawnByKind_09bdc6  @ $09BDC6  (88 B)
| ----------------------------------------------------------------------------
        .section .text.Chute_SpawnByKind_09bdc6, "ax", @progbits
        .global Chute_SpawnByKind_09bdc6
Chute_SpawnByKind_09bdc6:
        lea     Chute_A_09be1e(pc),a1           | +000
        jsr     0x4ae.l                         | +004
        bra.w   .L09bdec                        | +00a
        lea     Chute_B_09be2e(pc),a1           | +00e
        jsr     0x4ae.l                         | +012
        bra.w   .L09bdec                        | +018
        lea     Chute_C_09be3e(pc),a1           | +01c
        jsr     0x4ae.l                         | +020
.L09bdec:
        move.w  0x70(a6),d0                     | +026
        btst    #0x0,0x3a(a6)                   | +02a
        beq.w   .L09bdfc                        | +030
        neg.w   d0                              | +034
.L09bdfc:
        add.w   0x22(a6),d0                     | +036
        move.w  d0,0x22(a0)                     | +03a
        move.w  0x72(a6),d0                     | +03e
        btst    #0x1,0x3a(a6)                   | +042
        beq.w   .L09be14                        | +048
        neg.w   d0                              | +04c
.L09be14:
        add.w   0x24(a6),d0                     | +04e
        move.w  d0,0x24(a0)                     | +052
        rts                                     | +056

| ----------------------------------------------------------------------------
|  Chute_A_09be1e  @ $09BE1E  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Chute_A_09be1e, "ax", @progbits
        .global Chute_A_09be1e
Chute_A_09be1e:
        lea     0x2dd4ba.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        bra.w   Chute_Common_09be4a         | +00c

| ----------------------------------------------------------------------------
|  Chute_B_09be2e  @ $09BE2E  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Chute_B_09be2e, "ax", @progbits
        .global Chute_B_09be2e
Chute_B_09be2e:
        lea     0x2dd594.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        bra.w   Chute_Common_09be4a         | +00c

| ----------------------------------------------------------------------------
|  Chute_C_09be3e  @ $09BE3E  (96 B)
| ----------------------------------------------------------------------------
        .section .text.Chute_C_09be3e, "ax", @progbits
        .global Chute_C_09be3e
Chute_C_09be3e:
        lea     0x2dd37e.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        .global Chute_Common_09be4a
Chute_Common_09be4a:
        move.w  #0x163,d1                       | +00c
        jsr     0x236e.l                        | +010
        move.w  #0x2000,d0                      | +016
        jsr     0x28134.l                       | +01a
        andi.w  #0xffe3,0x38(a6)                | +020
        ori.w   #0x10,0x38(a6)                  | +026
        lea     .L09be70(pc),a1                 | +02c
        move.l  a1,(a6)                         | +030
.L09be70:
        jsr     0x2783a.l                       | +032
        jsr     0x28d70.l                       | +038
        bcc.w   .L09be86                        | +03e
        jmp     0x518.l                         | +042
.L09be86:
        movea.l #0xffffffff,a0                  | +048
        jsr     0x5dd56.l                       | +04e
        bcc.w   .L09be9c                        | +054
        jmp     0x518.l                         | +058
.L09be9c:
        rts                                     | +05e

| ----------------------------------------------------------------------------
|  Crate_Hitbox_09be9e  @ $09BE9E  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Crate_Hitbox_09be9e, "ax", @progbits
        .global Crate_Hitbox_09be9e
Crate_Hitbox_09be9e:
        .dc.w   0xffd0                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +002  (dato / opcode no decodificado)
        .dc.w   0xffd0                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +006  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Crate_Init_09bea6  @ $09BEA6  (344 B)
| ----------------------------------------------------------------------------
        .section .text.Crate_Init_09bea6, "ax", @progbits
        .global Crate_Init_09bea6
Crate_Init_09bea6:
        jsr     0x32b36.l                       | +000
        lea     Crate_Hitbox_09be9e(pc),a0      | +006
        jsr     0x5dd56.l                       | +00a
        bcc.w   .L09bec0                        | +010
        jmp     0x518.l                         | +014
.L09bec0:
        rts                                     | +01a
        move.w  #0xd000,d0                      | +01c
        jsr     0x28134.l                       | +020
        andi.w  #0xffe3,0x38(a6)                | +026
        ori.w   #0x10,0x38(a6)                  | +02c
        move.w  #0x17c,d1                       | +032
        jsr     0x236e.l                        | +036
        move.b  0x98(a6),d0                     | +03c
        andi.b  #0x1,d0                         | +040
        beq.w   .L09bf22                        | +044
        lea     0x2f7a70.l,a0                   | +048
        jsr     0x28cd4.l                       | +04e
        move.w  #0x20,0x2e(a6)                  | +054
        clr.w   0x2c(a6)                        | +05a
        lea     0x2f7ed4.l,a0                   | +05e
        move.l  a0,0x4c(a6)                     | +064
        jsr     0x283ca.l                       | +068
        lea     Crate_ChuteFollowX_09c1ac(pc),a1 | +06e
        jsr     0x4ae.l                         | +072
        bra.w   .L09bf6a                        | +078
.L09bf22:
        tst.b   0x9a(a6)                        | +07c
        bne.w   .L09bf3a                        | +080
        lea     0x2f7ac6.l,a0                   | +084
        jsr     0x28cd4.l                       | +08a
        bra.w   .L09bf46                        | +090
.L09bf3a:
        lea     0x2f7aac.l,a0                   | +094
        jsr     0x28cd4.l                       | +09a
.L09bf46:
        move.w  #0x20,0x2c(a6)                  | +0a0
        clr.w   0x2e(a6)                        | +0a6
        lea     0x2f7ddc.l,a0                   | +0aa
        move.l  a0,0x4c(a6)                     | +0b0
        jsr     0x283ca.l                       | +0b4
        lea     Crate_ChuteFollow_09c13c(pc),a1 | +0ba
        jsr     0x4ae.l                         | +0be
.L09bf6a:
        move.b  0x98(a6),d0                     | +0c4
        cmpi.b  #0x3,d0                         | +0c8
        bne.w   .L09bf84                        | +0cc
        bset    #0x1,0x3a(a6)                   | +0d0
        neg.w   0x2e(a6)                        | +0d6
        bra.w   .L09bf9c                        | +0da
.L09bf84:
        move.b  0x99(a6),d1                     | +0de
        eor.b   d1,d0                           | +0e2
        andi.b  #0x2,d0                         | +0e4
        beq.w   .L09bf9c                        | +0e8
        bset    #0x0,0x3a(a6)                   | +0ec
        neg.w   0x2c(a6)                        | +0f2
.L09bf9c:
        clr.w   0x76(a6)                        | +0f6
        lea     .L09bfa6(pc),a1                 | +0fa
        move.l  a1,(a6)                         | +0fe
.L09bfa6:
        jsr     0x27d50.l                       | +100
        bcs.w   .L09bfb6                        | +106
        jsr     0x27d50.l                       | +10a
.L09bfb6:
        bcc.w   .L09bfc0                        | +110
        lea     Crate_Destroyed_09bffe(pc),a1   | +114
        move.l  a1,(a6)                         | +118
.L09bfc0:
        subq.w  #0x1,0x76(a6)                   | +11a
        bcc.w   .L09bfde                        | +11e
        move.w  #0x2,0x76(a6)                   | +122
        lea     Crate_Debris_09c0a4(pc),a1      | +128
        jsr     0x4ae.l                         | +12c
        jsr     0x5dd02.l                       | +132
.L09bfde:
        jsr     0x28d70.l                       | +138
        jsr     0x283d8.l                       | +13e
        btst    #0x1,0x13(a6)                   | +144
        beq.w   .L09bffa                        | +14a
        lea     Crate_Opened_09c00a(pc),a1      | +14e
        move.l  a1,(a6)                         | +152
.L09bffa:
        bra.w   Crate_Init_09bea6               | +154

| ----------------------------------------------------------------------------
|  Crate_Destroyed_09bffe  @ $09BFFE  (12 B)
| ----------------------------------------------------------------------------
        .section .text.Crate_Destroyed_09bffe, "ax", @progbits
        .global Crate_Destroyed_09bffe
Crate_Destroyed_09bffe:
        jsr     0x13600.l                       | +000
        jmp     0x77efe.l                       | +006

| ----------------------------------------------------------------------------
|  Crate_Opened_09c00a  @ $09C00A  (104 B)
| ----------------------------------------------------------------------------
        .section .text.Crate_Opened_09c00a, "ax", @progbits
        .global Crate_Opened_09c00a
Crate_Opened_09c00a:
        move.w  #0x1025,d0                      | +000
        jsr     0x2352.l                        | +004
        jsr     0x13600.l                       | +00a
        move.w  #0x4,d1                         | +010
        jsr     0x236e.l                        | +014
        move.b  0x98(a6),d0                     | +01a
        andi.b  #0x1,d0                         | +01e
        beq.w   .L09c040                        | +022
        lea     0x2f7cc0.l,a0                   | +026
        jsr     0x28cd4.l                       | +02c
        bra.w   .L09c04c                        | +032
.L09c040:
        lea     0x2f7c46.l,a0                   | +036
        jsr     0x28cd4.l                       | +03c
.L09c04c:
        lea     .L09c052(pc),a1                 | +042
        move.l  a1,(a6)                         | +046
.L09c052:
        jsr     0x283ca.l                       | +048
        jsr     0x2783a.l                       | +04e
        jsr     0x28d70.l                       | +054
        bcc.w   .L09c06e                        | +05a
        jmp     0x518.l                         | +05e
.L09c06e:
        bra.w   Crate_Init_09bea6               | +064

| ----------------------------------------------------------------------------
|  Entity_IntegrateVelFrac_09c072  @ $09C072  (50 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_IntegrateVelFrac_09c072, "ax", @progbits
        .global Entity_IntegrateVelFrac_09c072
Entity_IntegrateVelFrac_09c072:
        move.b  0x29(a6),d0                     | +000
        move.b  0x28(a6),d1                     | +004
        ext.w   d1                              | +008
        move.w  0x22(a6),d2                     | +00a
        add.b   d0,0x26(a6)                     | +00e
        addx.w  d1,d2                           | +012
        move.w  d2,0x22(a6)                     | +014
        move.b  0x2b(a6),d0                     | +018
        move.b  0x2a(a6),d1                     | +01c
        ext.w   d1                              | +020
        move.w  0x24(a6),d2                     | +022
        add.b   d0,0x27(a6)                     | +026
        addx.w  d1,d2                           | +02a
        move.w  d2,0x24(a6)                     | +02c
        rts                                     | +030

| ----------------------------------------------------------------------------
|  Crate_Debris_09c0a4  @ $09C0A4  (152 B)
| ----------------------------------------------------------------------------
        .section .text.Crate_Debris_09c0a4, "ax", @progbits
        .global Crate_Debris_09c0a4
Crate_Debris_09c0a4:
        move.w  #0x17c,d1                       | +000
        jsr     0x236e.l                        | +004
        move.w  #0x2000,d0                      | +00a
        jsr     0x28134.l                       | +00e
        andi.w  #0xffe3,0x38(a6)                | +014
        ori.w   #0x10,0x38(a6)                  | +01a
        lea     0x2f7d3a.l,a0                   | +020
        jsr     0x28cd4.l                       | +026
        movea.l 0xc(a6),a0                      | +02c
        move.w  0x74(a0),d0                     | +030
        move.w  #0x400,d1                       | +034
        jsr     0x13c0e.l                       | +038
        btst    #0x0,0x3a(a0)                   | +03e
        beq.w   .L09c0ee                        | +044
        neg.w   d1                              | +048
.L09c0ee:
        move.w  0x28(a0),d3                     | +04a
        add.w   d3,d3                           | +04e
        add.w   d3,d1                           | +050
        move.w  0x2a(a6),d3                     | +052
        add.w   d3,d3                           | +056
        add.w   d3,d2                           | +058
        move.w  d1,0x28(a6)                     | +05a
        move.w  d2,0x2a(a6)                     | +05e
        lea     .L09c10c(pc),a1                 | +062
        move.l  a1,(a6)                         | +066
.L09c10c:
        jsr     0x2783a.l                       | +068
        jsr     Entity_IntegrateVelFrac_09c072(pc) | +06e
        move.w  0x28(a6),d0                     | +072
        asr.w   #0x4,d0                         | +076
        sub.w   d0,0x28(a6)                     | +078
        move.w  0x2a(a6),d0                     | +07c
        asr.w   #0x4,d0                         | +080
        sub.w   d0,0x2a(a6)                     | +082
        jsr     0x28d70.l                       | +086
        bcc.w   .L09c13a                        | +08c
        jmp     0x518.l                         | +090
.L09c13a:
        rts                                     | +096

| ----------------------------------------------------------------------------
|  Crate_ChuteFollow_09c13c  @ $09C13C  (112 B)
| ----------------------------------------------------------------------------
        .section .text.Crate_ChuteFollow_09c13c, "ax", @progbits
        .global Crate_ChuteFollow_09c13c
Crate_ChuteFollow_09c13c:
        lea     0x2f7e80.l,a0                   | +000
        move.l  a0,0x4c(a6)                     | +006
        jsr     0x283ca.l                       | +00a
        lea     .L09c152(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L09c152:
        jsr     0x7b2.l                         | +016
        bcc.w   .L09c162                        | +01c
        jmp     0x518.l                         | +020
.L09c162:
        movea.l 0xc(a6),a0                      | +026
        move.w  0x22(a0),0x22(a6)               | +02a
        move.w  0x24(a0),0x24(a6)               | +030
        move.b  0x3a(a0),0x3a(a6)               | +036
        jsr     0x283ca.l                       | +03c
        jsr     0x283d8.l                       | +042
        btst    #0x1,0x13(a6)                   | +048
        beq.w   .L09c1aa                        | +04e
        movea.l 0xc(a6),a0                      | +052
        move.w  0x24(a1),d0                     | +056
        addi.w  #0x1e,d0                        | +05a
        sub.w   0x24(a0),d0                     | +05e
        asl.w   #0x3,d0                         | +062
        sub.w   0x2a(a0),d0                     | +064
        asr.w   #0x3,d0                         | +068
        move.w  d0,0x2e(a0)                     | +06a
.L09c1aa:
        rts                                     | +06e

| ----------------------------------------------------------------------------
|  Crate_ChuteFollowX_09c1ac  @ $09C1AC  (108 B)
| ----------------------------------------------------------------------------
        .section .text.Crate_ChuteFollowX_09c1ac, "ax", @progbits
        .global Crate_ChuteFollowX_09c1ac
Crate_ChuteFollowX_09c1ac:
        lea     0x2f7f78.l,a0                   | +000
        move.l  a0,0x4c(a6)                     | +006
        jsr     0x283ca.l                       | +00a
        lea     .L09c1c2(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L09c1c2:
        jsr     0x7b2.l                         | +016
        bcc.w   .L09c1d2                        | +01c
        jmp     0x518.l                         | +020
.L09c1d2:
        movea.l 0xc(a6),a0                      | +026
        move.w  0x22(a0),0x22(a6)               | +02a
        move.w  0x24(a0),0x24(a6)               | +030
        move.b  0x3a(a0),0x3a(a6)               | +036
        jsr     0x283ca.l                       | +03c
        jsr     0x283d8.l                       | +042
        btst    #0x1,0x13(a6)                   | +048
        beq.w   .L09c216                        | +04e
        movea.l 0xc(a6),a0                      | +052
        move.w  0x22(a1),d0                     | +056
        sub.w   0x22(a0),d0                     | +05a
        asl.w   #0x3,d0                         | +05e
        sub.w   0x28(a0),d0                     | +060
        asr.w   #0x3,d0                         | +064
        move.w  d0,0x2c(a0)                     | +066
.L09c216:
        rts                                     | +06a

| ----------------------------------------------------------------------------
|  Entity_CmpPrioWithSibling_09c218  @ $09C218  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpPrioWithSibling_09c218, "ax", @progbits
        .global Entity_CmpPrioWithSibling_09c218
Entity_CmpPrioWithSibling_09c218:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_09c22e                    | +00c

| ----------------------------------------------------------------------------
|  Thrown_Hitbox_09c234  @ $09C234  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Thrown_Hitbox_09c234, "ax", @progbits
        .global Thrown_Hitbox_09c234
Thrown_Hitbox_09c234:
        .dc.w   0xffd0                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +002  (dato / opcode no decodificado)
        .dc.w   0xffd0                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +006  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Thrown_GroundTail_09c23c  @ $09C23C  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Thrown_GroundTail_09c23c, "ax", @progbits
        .global Thrown_GroundTail_09c23c
Thrown_GroundTail_09c23c:
        jsr     0x32b36.l                       | +000
        lea     Thrown_Hitbox_09c234(pc),a0     | +006
        jsr     0x5dd56.l                       | +00a
        bcc.w   Jsr5B6Rts_09c25c                | +010

| ----------------------------------------------------------------------------
|  Thrown_Init_09c25e  @ $09C25E  (316 B)
| ----------------------------------------------------------------------------
        .section .text.Thrown_Init_09c25e, "ax", @progbits
        .global Thrown_Init_09c25e
Thrown_Init_09c25e:
        move.w  #0xd000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x10,0x38(a6)                  | +010
        move.w  #0x184,d1                       | +016
        jsr     0x236e.l                        | +01a
        move.b  0x98(a6),d0                     | +020
        andi.b  #0x1,d0                         | +024
        beq.w   .L09c2a0                        | +028
        lea     0x2f83a4.l,a0                   | +02c
        jsr     0x28cd4.l                       | +032
        move.w  #0x800,0x2a(a6)                 | +038
        bra.w   .L09c2b8                        | +03e
.L09c2a0:
        lea     0x2f84ae.l,a0                   | +042
        jsr     0x28cd4.l                       | +048
        move.w  #0x800,0x28(a6)                 | +04e
        move.w  #0xfe00,0x2a(a6)                | +054
.L09c2b8:
        move.b  0x98(a6),d0                     | +05a
        cmpi.b  #0x3,d0                         | +05e
        bne.w   .L09c2d2                        | +062
        bset    #0x1,0x3a(a6)                   | +066
        neg.w   0x2a(a6)                        | +06c
        bra.w   .L09c2ea                        | +070
.L09c2d2:
        move.b  0x99(a6),d1                     | +074
        eor.b   d1,d0                           | +078
        andi.b  #0x2,d0                         | +07a
        beq.w   .L09c2ea                        | +07e
        bset    #0x0,0x3a(a6)                   | +082
        neg.w   0x28(a6)                        | +088
.L09c2ea:
        lea     0x2f85b8.l,a0                   | +08c
        move.l  a0,0x4c(a6)                     | +092
        jsr     0x283ca.l                       | +096
        lea     Thrown_Shadow_09c3f0(pc),a1     | +09c
        jsr     0x4ae.l                         | +0a0
        lea     .L09c30a(pc),a1                 | +0a6
        move.l  a1,(a6)                         | +0aa
.L09c30a:
        move.b  0x98(a6),d0                     | +0ac
        andi.b  #0x1,d0                         | +0b0
        beq.w   .L09c332                        | +0b4
        move.w  0x28(a6),d0                     | +0b8
        asr.w   #0x4,d0                         | +0bc
        sub.w   d0,0x28(a6)                     | +0be
        move.w  #0x200,d0                       | +0c2
        sub.w   0x2a(a6),d0                     | +0c6
        asr.w   #0x6,d0                         | +0ca
        add.w   d0,0x2a(a6)                     | +0cc
        bra.w   .L09c34a                        | +0d0
.L09c332:
        move.w  0x28(a6),d0                     | +0d4
        asr.w   #0x5,d0                         | +0d8
        sub.w   d0,0x28(a6)                     | +0da
        move.w  #0x200,d0                       | +0de
        sub.w   0x2a(a6),d0                     | +0e2
        asr.w   #0x3,d0                         | +0e6
        add.w   d0,0x2a(a6)                     | +0e8
.L09c34a:
        jsr     0x283ca.l                       | +0ec
        jsr     0x27cee.l                       | +0f2
        bcs.w   .L09c360                        | +0f8
        jsr     0x27cee.l                       | +0fc
.L09c360:
        bcc.w   .L09c36a                        | +102
        lea     Thrown_Landed_09c39a(pc),a1     | +106
        move.l  a1,(a6)                         | +10a
.L09c36a:
        jsr     0x28d70.l                       | +10c
        bcc.w   .L09c380                        | +112
        jsr     0x5b6.l                         | +116
        jmp     0x518.l                         | +11c
.L09c380:
        jsr     0x283d8.l                       | +122
        btst    #0x1,0x13(a6)                   | +128
        beq.w   .L09c396                        | +12e
        lea     Thrown_Landed_09c39a(pc),a1     | +132
        move.l  a1,(a6)                         | +136
.L09c396:
        bra.w   Thrown_GroundTail_09c23c        | +138

| ----------------------------------------------------------------------------
|  Thrown_Landed_09c39a  @ $09C39A  (86 B)
| ----------------------------------------------------------------------------
        .section .text.Thrown_Landed_09c39a, "ax", @progbits
        .global Thrown_Landed_09c39a
Thrown_Landed_09c39a:
        move.b  0x98(a6),d0                     | +000
        andi.b  #0x1,d0                         | +004
        beq.w   .L09c3ba                        | +008
        clr.w   0x2a(a6)                        | +00c
        lea     0x2f8416.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        bra.w   .L09c3ca                        | +01c
.L09c3ba:
        clr.w   0x28(a6)                        | +020
        lea     0x2f8520.l,a0                   | +024
        jsr     0x28cd4.l                       | +02a
.L09c3ca:
        lea     .L09c3d0(pc),a1                 | +030
        move.l  a1,(a6)                         | +034
.L09c3d0:
        jsr     0x27cee.l                       | +036
        jsr     0x28d70.l                       | +03c
        bcc.w   .L09c3ec                        | +042
        jsr     0x5b6.l                         | +046
        jmp     0x518.l                         | +04c
.L09c3ec:
        bra.w   Thrown_GroundTail_09c23c        | +052

| ----------------------------------------------------------------------------
|  Thrown_Shadow_09c3f0  @ $09C3F0  (92 B)
| ----------------------------------------------------------------------------
        .section .text.Thrown_Shadow_09c3f0, "ax", @progbits
        .global Thrown_Shadow_09c3f0
Thrown_Shadow_09c3f0:
        move.w  #0x184,d1                       | +000
        jsr     0x236e.l                        | +004
        movea.l 0xc(a6),a0                      | +00a
        move.w  0x22(a0),0x22(a6)               | +00e
        move.w  0x24(a0),0x24(a6)               | +014
        lea     .L09c410(pc),a1                 | +01a
        move.l  a1,(a6)                         | +01e
.L09c410:
        movea.l 0xc(a6),a0                      | +020
        move.w  0x38(a0),d0                     | +024
        subq.w  #0x1,d0                         | +028
        move.w  d0,0x38(a6)                     | +02a
        move.w  0x22(a0),d0                     | +02e
        sub.w   0x22(a6),d0                     | +032
        asr.w   #0x3,d0                         | +036
        move.w  d0,d1                           | +038
        add.w   d1,d0                           | +03a
        add.w   d1,d0                           | +03c
        add.w   d0,0x22(a6)                     | +03e
        move.w  0x24(a0),d0                     | +042
        sub.w   0x24(a6),d0                     | +046
        asr.w   #0x3,d0                         | +04a
        move.w  d0,d1                           | +04c
        add.w   d1,d0                           | +04e
        add.w   d1,d0                           | +050
        add.w   d0,0x24(a6)                     | +052
        move.l  0x70(a0),0x3c(a6)               | +056

| ----------------------------------------------------------------------------
|  Thrown_ShadowB_09c454  @ $09C454  (92 B)
| ----------------------------------------------------------------------------
        .section .text.Thrown_ShadowB_09c454, "ax", @progbits
        .global Thrown_ShadowB_09c454
Thrown_ShadowB_09c454:
        move.w  #0x184,d1                       | +000
        jsr     0x236e.l                        | +004
        movea.l 0xc(a6),a0                      | +00a
        move.w  0x22(a0),0x22(a6)               | +00e
        move.w  0x24(a0),0x24(a6)               | +014
        lea     .L09c474(pc),a1                 | +01a
        move.l  a1,(a6)                         | +01e
.L09c474:
        movea.l 0xc(a6),a0                      | +020
        move.w  0x38(a0),d0                     | +024
        subq.w  #0x1,d0                         | +028
        move.w  d0,0x38(a6)                     | +02a
        move.w  0x22(a0),d0                     | +02e
        sub.w   0x22(a6),d0                     | +032
        asr.w   #0x2,d0                         | +036
        move.w  d0,d1                           | +038
        add.w   d1,d0                           | +03a
        add.w   d1,d0                           | +03c
        add.w   d0,0x22(a6)                     | +03e
        move.w  0x24(a0),d0                     | +042
        sub.w   0x24(a6),d0                     | +046
        asr.w   #0x2,d0                         | +04a
        move.w  d0,d1                           | +04c
        add.w   d1,d0                           | +04e
        add.w   d1,d0                           | +050
        add.w   d0,0x24(a6)                     | +052
        move.l  0x70(a0),0x3c(a6)               | +056

| ----------------------------------------------------------------------------
|  Entity_CmpPrioWithSibling_09c4b8  @ $09C4B8  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpPrioWithSibling_09c4b8, "ax", @progbits
        .global Entity_CmpPrioWithSibling_09c4b8
Entity_CmpPrioWithSibling_09c4b8:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_09c4ce                    | +00c

| ----------------------------------------------------------------------------
|  Flag_Init_09c4d4  @ $09C4D4  (300 B)
| ----------------------------------------------------------------------------
        .section .text.Flag_Init_09c4d4, "ax", @progbits
        .global Flag_Init_09c4d4
Flag_Init_09c4d4:
        move.b  #0x1,0x10a2d1.l                 | +000
        move.w  #0xd000,d0                      | +008
        jsr     0x28134.l                       | +00c
        andi.w  #0xffe3,0x38(a6)                | +012
        ori.w   #0x10,0x38(a6)                  | +018
        move.w  #0x19b,d1                       | +01e
        jsr     0x236e.l                        | +022
        move.b  0x98(a6),d0                     | +028
        andi.b  #0x1,d0                         | +02c
        beq.w   .L09c524                        | +030
        lea     0x2f86ec.l,a0                   | +034
        jsr     0x28cd4.l                       | +03a
        move.w  #0xffff,0x5c(a6)                | +040
        move.w  #0x0,0x5e(a6)                   | +046
        bra.w   .L09c53c                        | +04c
.L09c524:
        lea     0x2f865c.l,a0                   | +050
        jsr     0x28cd4.l                       | +056
        move.w  #0x0,0x5c(a6)                   | +05c
        move.w  #0xffff,0x5e(a6)                | +062
.L09c53c:
        move.b  0x98(a6),d0                     | +068
        cmpi.b  #0x3,d0                         | +06c
        bne.w   .L09c562                        | +070
        bset    #0x1,0x3a(a6)                   | +074
        cmpi.w  #0x0,0x5e(a6)                   | +07a
        bne.w   .L09c55e                        | +080
        move.w  #0x1,0x5e(a6)                   | +084
.L09c55e:
        bra.w   .L09c586                        | +08a
.L09c562:
        move.b  0x99(a6),d1                     | +08e
        eor.b   d1,d0                           | +092
        andi.b  #0x2,d0                         | +094
        beq.w   .L09c586                        | +098
        bset    #0x0,0x3a(a6)                   | +09c
        cmpi.w  #0x0,0x5c(a6)                   | +0a2
        bne.w   .L09c586                        | +0a8
        move.w  #0x1,0x5c(a6)                   | +0ac
.L09c586:
        move.b  0x98(a6),d0                     | +0b2
        cmpi.b  #0x1,d0                         | +0b6
        bne.w   .L09c5a0                        | +0ba
        addq.w  #0x2,0x22(a6)                   | +0be
        addi.w  #0x3e,0x24(a6)                  | +0c2
        bra.w   .L09c5ca                        | +0c8
.L09c5a0:
        cmpi.b  #0x3,d0                         | +0cc
        bne.w   .L09c5ac                        | +0d0
        bra.w   .L09c5ca                        | +0d4
.L09c5ac:
        move.w  #0x19,d0                        | +0d8
        btst    #0x0,0x3a(a6)                   | +0dc
        beq.w   .L09c5bc                        | +0e2
        neg.w   d0                              | +0e6
.L09c5bc:
        add.w   d0,0x22(a6)                     | +0e8
        move.w  d0,0x28(a6)                     | +0ec
        addi.w  #0x14,0x24(a6)                  | +0f0
.L09c5ca:
        lea     .L09c5d0(pc),a1                 | +0f6
        move.l  a1,(a6)                         | +0fa
.L09c5d0:
        jsr     0x2783a.l                       | +0fc
        jsr     0x28d70.l                       | +102
        bcc.w   .L09c5e6                        | +108
        jmp     0x518.l                         | +10c
.L09c5e6:
        jsr     0x283d8.l                       | +112
        btst    #0x1,0x5a(a6)                   | +118
        beq.w   JsrAbsThunk_09c600              | +11e
        move.w  #0x1086,d0                      | +122
        jsr     0x2352.l                        | +126
