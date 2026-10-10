| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave VVVV — bazooka, vehículo lanzacohetes, walker y fragmentos
|  Región: $06A000..$06DFE8  (15,616 B, 179 entradas, 71 huecos)
| ============================================================================
|
|  A) QUÉ ES
|  ---------
|  Cuatro familias de entidades enemigas/aliadas de la mitad tardía del
|  runtime, todas siguiendo el protocolo estándar (a6 = entidad, +$00
|  handler, +$0C padre, +$72 estado, +$98..+$9F parámetros de la VM de
|  misión) y 19 plantillas del índice $E8000:
|   1. $06A000..$06A07E — Tank_*: cola del tanque de la Wave UUUU (ataque
|      por variante +$7D/+$7E, sonidos $E/$1A, comparación de slot).
|   2. $06A07E..$06ACB2 — Bazooka_* / BazookaB_* / BazookaCrew_* /
|      AllyBazooka_*: soldado bazooka (tmpl 86/87 y 90/91), dotación bazooka
|      de mortero/emplazamiento (tmpl 92/93, entrada externa $6A7D6 desde
|      Mortar_ToJump6A7D6_063960) y el bazooka aliado del escuadrón de
|      rescate (tmpl 64, spawner tmpl 65; lo crea TaskHandler_0849ba).
|   3. $06ACB2..$06BA2C — BazookaWeapon_* y helpers Bazooka_*: hijo "arma"
|      que sigue al padre (+$0C) con offsets $2CA470 y despacha por estado
|      del padre (tabla $2CA644/$2CA664 → jmp (a1)); fogonazo, cohete
|      ($2CA4B0: vel/grav por octante, snd $173, música $1025 al impactar),
|      puntería por tabla $2CA5F0/$2CA612/$2CA634, muerte (debris $77C7E,
|      explosiones $77FD6, músicas $1033/$1021).
|   4. $06BA2C..$06D654 — RocketVehicle_*: vehículo con ruedas (tmpl 88/89)
|      sobre tres eslabones Chain3 (+$74/+$78/+$7C, $30696/$30704/$3076A,
|      físicas de suelo $2785C/$2A8C0/$27A18), fase de rueda +$8A → +$97
|      que selecciona entre 29 tablas inline de 4 punteros de sprite
|      (RocketVehicle_Sprites*/RiderSprites*; 4º puntero siempre
|      $2CD62A), conductor hijo (RocketVehicle_Rider, tmpl vía $4AE),
|      lanzacohetes (MuzzleFlash/Rocket, tablas $2CD10A/$2CD11A por
|      +$97/+$98), muerte con Wreck/Corpse y DetachLinks (eslabones →
|      $6C554).
|   5. $06D654..$06DA9E — Walker_*: enemigo de aproximación (tmpl 66..74:
|      nueve variantes = 3 modos +$70 × 3 combinaciones +$7E/+$7F), HP de
|      $2B8D6C, X límite +$86 = +$9A<<4 (o $150), hijo $723D2, ataque por
|      ráfagas (tablas $2B8DEE/$2B8E70/$2B8EF2), explosión final con
|      Entity_SpawnLoop16 (16 Frag_Scatter) y $518.
|   6. $06DA9E..$06DFE8 — Frag_* / FireBurst_*: fragmentos (Shell, Debris
|      $6DBD4 ← Tank_Debris_069af8, Smoke, Spark, Scatter/ScatterSlow con
|      velocidad sin/cos $2C072C/$2C07AC × RNG $5E9B6) y chorro de fuego
|      ($6DF32: música $108E, snd 2, sprites $2D4666, FX $31D26; continúa
|      en $6E15E, Wave WWWW).
|
|  B) CÓMO FUNCIONA
|  ----------------
|  Bazooka (tmpl 87): máquina +$72 = 0 Idle/Walk, 1 Alert, 2→4→3 Attack
|  (timer +$70 de Tbl_Decode2D $2BD4EA/$2BD56C/$2BD5EE, +$1E tras disparo),
|  5 Hurt, 6 Die/Corpse ($3C frames), 7 HitTest. Facing +$75 ≠ +$3A dispara
|  Turn ($2CAA94/$2CABAA). El arma es una entidad hija (BazookaWeapon_Child)
|  que copia el estado del padre cada frame (StateLookup) y dibuja su propio
|  sprite ($2CB082, o $2CB1AC[+$34&$F] al apuntar); al disparar, Bazooka_Fire
|  crea MuzzleFlash ($2DD6D2, snd 8) y Rocket (vel $2CA4B0[+$98&7][+$34&$F],
|  ángulo con Atan2 $5E018, hit $283D8, impacto → $1025 + $77F6A).
|  BazookaB (tmpl 90/91): igual pero con +$9C/+$9D/+$9E (modo, lado,
|  reacquire), objetivo +$90 vía Player_GetEntity $5E3A2.
|  AllyBazooka (tmpl 64): avanza hasta +$98=$110 y dispara ráfagas +$74
|  ($2BD5EE); muere si el padre tiene +$20=$FF; HitTest con clases +$58 2/3.
|  RocketVehicle: Drive/Stop/Turn/Idle/Alert/Attack/Hurt/Die/Wreck/Corpse
|  con sprites por fase de rueda; GroundPhysics usa Chain3 cuando +$13 bit6
|  está limpio; SlopeAngle (Atan2 entre eslabones +$74/+$78) → SlopeToSpeed.
|  Walker: Init (snap $5E7C0, scroll $267E2, Walker_PlayCry_06e31e snd por +$9B,
|  prio $14; si +$9D: prio $8000 + Walker_SetSpriteByFlags7E7F_06e484 + hijo $723D2) → Approach
|  (sprites $2D428C/$2D42B0/$2D4298, hasta +$86 o a $60 del jugador) →
|  Attack (ráfagas $2B8E70 / pausas $2B8DEE / cuenta $2B8EF2, FX $5E086
|  con $2D4036) → Die ($2D4394, debris) → Explode ($1033, $77FD6, 16 Frag).
|  Frag_Scatter: sprite aleatorio $2D4106/$2D4146[rnd&$F], velocidad
|  (sin,cos)[rnd&$3E+$20] × (rnd&7+1)/4 × +$36/256, rebote con +$80/+$82,
|  Scroll_IsPastQuarter $5E804 al terminar.
|
|  C) INTERFAZ
|  -----------
|   Entradas E8000: 64 $6AA14, 65 $6AC5C, 66..74 $6D654..$6D6D4, 86 $6A08A,
|   87 $6A07E, 88 $6BA34, 89 $6BA2C, 90 $6A476, 91 $6A486, 92 $6A7BC,
|   93 $6A7CA. Entradas externas: $6A7D6 (mortero), $6DBD4 (Tank_Debris),
|   $6DCE0/$6DD16 (props/para_squad/sniper), $6DD5C (Entity_SpawnLoop16).
|   Campos: +$34 índice de puntería (0..$F), +$36 velocidad base, +$70 timer,
|   +$72 estado, +$73/+$74 contadores, +$75 facing lógico, +$7A kind de
|   grito, +$80/+$82 rebotes, +$86 X límite, +$88 ráfagas, +$89 flag,
|   +$8A fase rueda, +$8D facing vehículo, +$8E/+$8F offset jinete,
|   +$90 objetivo, +$96/+$97 fase, +$98..+$9F parámetros VM.
|
|  D) EVIDENCIAS
|  -------------
|   - Índice $E8000 leído directamente del P ROM (19 plantillas en rango).
|   - Mortar_ToJump6A7D6_063960 prepara +$98..+$9E y salta a $6A7D6.
|   - rescue_squad TaskHandler_0849ba hace `lea $6AA14; jsr $4AE` (aliado).
|   - Tank_Debris_069af8 → jmp $6DBD4; props/para_squad → $6DCE0/$6DD16.
|   - Chain3_* ($30696/$30704/$3076A) ya identificados en slug_helpers
|     como eslabones de suspensión → vehículo con ruedas.
|   - 29 tablas de 4 punteros ($2CDxxx..$2D3xxx, 4º = $2CD62A) tras bra.w,
|     indexadas por +$97&3: poses por fase de rueda.
|   - Entity_SpawnLoop16_06E412 ya documentado: 16 × Frag_Scatter.
|
|  E) HIPÓTESIS / DUDAS
|  --------------------
|   - "Bazooka": soldado con lanzacohetes (cohete con gravedad $663 y snd
|     $173). Podría ser el soldado con mortero portátil; nombre provisional.
|   - "RocketVehicle": vehículo enemigo con ruedas y lanzacohetes (¿Girida-O
|     de la misión 2?). "Walker": enemigo que avanza hasta un X y ataca en
|     ráfagas (¿Mission 4 enemigo mecánico?). Verificar con sprites.
|   - "Frag"/"FireBurst" nombrados por comportamiento (dispersión sin/cos,
|     música $108E); su atribución exacta a un arma queda pendiente.
|   - Helpers $6E15E/$6E176/$6E20C/$6E2FE/$6E31E/$6E34A/$6E356/$6E394/
|     $6E484 pertenecen a la Wave WWWW (Sub_0006Exxx provisionales).
|
|  F) ESTADO
|  ---------
|   179/179 entradas byte-exactas (gen_asm_region --registry); lint OK.
|
|  Verificación: cada sección .text.<Sym> se coloca en su dirección CPU
|  absoluta y reensambla byte-exacta contra build/mslug_prom.bin
|  (MD5 816b3f74c76b3373993407615f1850fe).
| ============================================================================

        .text

| ----------------------------------------------------------------------------
|  Tank_SetAttackByVariant_06a000  @ $06A000  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_SetAttackByVariant_06a000, "ax", @progbits
        .global Tank_SetAttackByVariant_06a000
Tank_SetAttackByVariant_06a000:
        andi.w  #0xffe3,0x38(a6)                | +000
        ori.w   #0x14,0x38(a6)                  | +006
        tst.b   0x7d(a6)                        | +00c
        bne.w   JsrAbsRts_06a042                | +010
        move.l  #0x2c8a74,0x4c(a6)              | +014
        jsr     0x283ca.l                       | +01c
        jsr     0x283d8.l                       | +022
        lea     0x2c8c7e.l,a1                   | +028
        tst.b   0x7e(a6)                        | +02e
        beq.w   JsrAbsThunk_06a03c              | +032
        lea     0x2c8ca6.l,a1                   | +036

| ----------------------------------------------------------------------------
|  Tank_SetSndE_06a044  @ $06A044  (4 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_SetSndE_06a044, "ax", @progbits
        .global Tank_SetSndE_06a044
Tank_SetSndE_06a044:
        move.w  #0xe,d1                         | +000

| ----------------------------------------------------------------------------
|  Tank_SetSnd1A_06a050  @ $06A050  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_SetSnd1A_06a050, "ax", @progbits
        .global Tank_SetSnd1A_06a050
Tank_SetSnd1A_06a050:
        addi.w  #0x0,0x38(a6)                   | +000
        move.w  #0x1a,d1                        | +006

| ----------------------------------------------------------------------------
|  Tank_CmpSlotWithParent_06a062  @ $06A062  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_CmpSlotWithParent_06a062, "ax", @progbits
        .global Tank_CmpSlotWithParent_06a062
Tank_CmpSlotWithParent_06a062:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_06a078                    | +00c

| ----------------------------------------------------------------------------
|  Bazooka_Tmpl57_06a07e  @ $06A07E  (86 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_Tmpl57_06a07e, "ax", @progbits
        .global Bazooka_Tmpl57_06a07e
Bazooka_Tmpl57_06a07e:
        move.l  #0x2ca238,0x4c(a6)              | +000
        bra.w   .L06a092                        | +008
        move.l  #0x2ca2d8,0x4c(a6)              | +00c
.L06a092:
        jsr     0x283ca.l                       | +014
        clr.b   0x9d(a6)                        | +01a
        clr.b   0x9e(a6)                        | +01e
        jsr     Bazooka_SpawnWeaponGround_06b78a(pc) | +022
        lea     0x2bd468.l,a0                   | +026
        jsr     0x799de.l                       | +02c
        move.w  d0,0x36(a6)                     | +032
        lea     0x2bd4ea.l,a0                   | +036
        jsr     0x799de.l                       | +03c
        move.w  d0,0x70(a6)                     | +042
        lea     0x2bd3e6.l,a0                   | +046
        jsr     0x799de.l                       | +04c
        move.w  d0,0x66(a6)                     | +052

| ----------------------------------------------------------------------------
|  Bazooka_Idle_06a0d4  @ $06A0D4  (86 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_Idle_06a0d4, "ax", @progbits
        .global Bazooka_Idle_06a0d4
Bazooka_Idle_06a0d4:
        move.b  #0x0,0x72(a6)                   | +000
        lea     0x2bd670.l,a0                   | +006
        jsr     0x799de.l                       | +00c
        move.b  d0,0x74(a6)                     | +012
        clr.w   0x28(a6)                        | +016
        lea     0x2ca84a.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        lea     .L06a100(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L06a100:
        jsr     Bazooka_ScrollGroundIfBit6_06b232(pc) | +02c
        jsr     0x28d70.l                       | +030
        subq.b  #0x1,0x74(a6)                   | +036
        bgt.w   .L06a118                        | +03a
        lea     Bazooka_Turn_06a20c(pc),a1      | +03e
        move.l  a1,(a6)                         | +042
.L06a118:
        jsr     Bazooka_AttackTimerInRange_06b47e(pc) | +044
        bcc.w   .L06a126                        | +048
        lea     Bazooka_Alert_06a270(pc),a1     | +04c
        move.l  a1,(a6)                         | +050
.L06a126:
        bra.w   Bazooka_Attack_06a2d0__L06a390  | +052

| ----------------------------------------------------------------------------
|  Bazooka_Walk_06a12a  @ $06A12A  (138 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_Walk_06a12a, "ax", @progbits
        .global Bazooka_Walk_06a12a
Bazooka_Walk_06a12a:
        move.b  #0x0,0x72(a6)                   | +000
        move.w  0x36(a6),d0                     | +006
        btst    #0x0,0x75(a6)                   | +00a
        bne.w   .L06a140                        | +010
        neg.w   d0                              | +014
.L06a140:
        move.w  d0,0x28(a6)                     | +016
        jsr     Bazooka_FacingMismatch_06b368(pc) | +01a
        bcs.w   .L06a15c                        | +01e
        lea     0x2ca690.l,a0                   | +022
        jsr     0x28cd4.l                       | +028
        bra.w   .L06a168                        | +02e
.L06a15c:
        lea     0x2ca74a.l,a0                   | +032
        jsr     0x28cd4.l                       | +038
.L06a168:
        lea     .L06a16e(pc),a1                 | +03e
        move.l  a1,(a6)                         | +042
.L06a16e:
        jsr     Bazooka_ScrollGroundIfBit6_06b232(pc) | +044
        bcc.w   .L06a182                        | +048
        eori.b  #0x1,0x75(a6)                   | +04c
        lea     Bazooka_Stop_06a1b4(pc),a1      | +052
        move.l  a1,(a6)                         | +056
.L06a182:
        jsr     0x28d70.l                       | +058
        jsr     Bazooka_FaceTarget_06b27a(pc)   | +05e
        bcc.w   .L06a196                        | +062
        lea     Bazooka_Stop_06a1b4(pc),a1      | +066
        move.l  a1,(a6)                         | +06a
.L06a196:
        jsr     Bazooka_AttackTimerInRange_06b47e(pc) | +06c
        bcc.w   .L06a1a4                        | +070
        lea     Bazooka_Stop_06a1b4(pc),a1      | +074
        move.l  a1,(a6)                         | +078
.L06a1a4:
        jsr     0x283ca.l                       | +07a
        jsr     0x283d8.l                       | +080
        bra.w   Bazooka_Attack_06a2d0__L06a390  | +086

| ----------------------------------------------------------------------------
|  Bazooka_Stop_06a1b4  @ $06A1B4  (88 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_Stop_06a1b4, "ax", @progbits
        .global Bazooka_Stop_06a1b4
Bazooka_Stop_06a1b4:
        clr.w   0x28(a6)                        | +000
        move.b  #0x0,0x72(a6)                   | +004
        jsr     Bazooka_FacingMismatch_06b368(pc) | +00a
        bcs.w   .L06a1d6                        | +00e
        lea     0x2ca996.l,a0                   | +012
        jsr     0x28cd4.l                       | +018
        bra.w   .L06a1e2                        | +01e
.L06a1d6:
        lea     0x2ca898.l,a0                   | +022
        jsr     0x28cd4.l                       | +028
.L06a1e2:
        lea     .L06a1e8(pc),a1                 | +02e
        move.l  a1,(a6)                         | +032
.L06a1e8:
        jsr     Bazooka_ScrollGroundIfBit6_06b232(pc) | +034
        jsr     0x28d70.l                       | +038
        bcc.w   .L06a1fc                        | +03e
        lea     Bazooka_Idle_06a0d4(pc),a1      | +042
        move.l  a1,(a6)                         | +046
.L06a1fc:
        jsr     0x283ca.l                       | +048
        jsr     0x283d8.l                       | +04e
        bra.w   Bazooka_Attack_06a2d0__L06a390  | +054

| ----------------------------------------------------------------------------
|  Bazooka_Turn_06a20c  @ $06A20C  (100 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_Turn_06a20c, "ax", @progbits
        .global Bazooka_Turn_06a20c
Bazooka_Turn_06a20c:
        move.b  #0x0,0x72(a6)                   | +000
        jsr     Bazooka_FacingMismatch_06b368(pc) | +006
        bcs.w   .L06a22a                        | +00a
        lea     0x2caa94.l,a0                   | +00e
        jsr     0x28cd4.l                       | +014
        bra.w   .L06a236                        | +01a
.L06a22a:
        lea     0x2cabaa.l,a0                   | +01e
        jsr     0x28cd4.l                       | +024
.L06a236:
        lea     .L06a23c(pc),a1                 | +02a
        move.l  a1,(a6)                         | +02e
.L06a23c:
        jsr     Bazooka_ScrollGroundIfBit6_06b232(pc) | +030
        bcc.w   .L06a250                        | +034
        eori.b  #0x1,0x75(a6)                   | +038
        lea     Bazooka_Stop_06a1b4(pc),a1      | +03e
        move.l  a1,(a6)                         | +042
.L06a250:
        jsr     0x28d70.l                       | +044
        bcc.w   .L06a260                        | +04a
        lea     Bazooka_Walk_06a12a(pc),a1      | +04e
        move.l  a1,(a6)                         | +052
.L06a260:
        jsr     0x283ca.l                       | +054
        jsr     0x283d8.l                       | +05a
        bra.w   Bazooka_Attack_06a2d0__L06a390  | +060

| ----------------------------------------------------------------------------
|  Bazooka_Alert_06a270  @ $06A270  (76 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_Alert_06a270, "ax", @progbits
        .global Bazooka_Alert_06a270
Bazooka_Alert_06a270:
        jsr     0x267e2.l                       | +000
        jsr     0x5e0d4.l                       | +006
        move.l  a0,0x90(a6)                     | +00c
        move.b  #0x1,0x72(a6)                   | +010
        clr.b   0x74(a6)                        | +016
        clr.b   0x73(a6)                        | +01a
        lea     0x2ca84a.l,a0                   | +01e
        jsr     0x28cd4.l                       | +024
        lea     .L06a2a0(pc),a1                 | +02a
        move.l  a1,(a6)                         | +02e
.L06a2a0:
        jsr     Bazooka_ScrollGroundIfBit6_06b232(pc) | +030
        jsr     0x28d70.l                       | +034
        tst.b   0x73(a6)                        | +03a
        beq.w   .L06a2b8                        | +03e
        lea     Bazooka_RngTimer_06a2bc(pc),a1  | +042
        move.l  a1,(a6)                         | +046
.L06a2b8:
        bra.w   Bazooka_Attack_06a2d0__L06a390  | +048

| ----------------------------------------------------------------------------
|  Bazooka_RngTimer_06a2bc  @ $06A2BC  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_RngTimer_06a2bc, "ax", @progbits
        .global Bazooka_RngTimer_06a2bc
Bazooka_RngTimer_06a2bc:
        lea     0x2bd5ee.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.b  d0,0x74(a6)                     | +00c
        bra.w   Bazooka_Attack_06a2d0__L06a2de  | +010

| ----------------------------------------------------------------------------
|  Bazooka_Attack_06a2d0  @ $06A2D0  (254 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_Attack_06a2d0, "ax", @progbits
        .global Bazooka_Attack_06a2d0
Bazooka_Attack_06a2d0:
        subq.b  #0x1,0x74(a6)                   | +000
        cmpi.b  #0x0,0x74(a6)                   | +004
        ble.w   .L06a33e                        | +00a
        .global Bazooka_Attack_06a2d0__L06a2de
Bazooka_Attack_06a2d0__L06a2de:
.L06a2de:
        clr.b   0x73(a6)                        | +00e
        move.b  #0x2,0x72(a6)                   | +012
        lea     0x2bd56c.l,a0                   | +018
        jsr     0x799de.l                       | +01e
        move.w  d0,0x70(a6)                     | +024
        lea     0x2cacc0.l,a0                   | +028
        jsr     0x28cd4.l                       | +02e
        lea     .L06a30a(pc),a1                 | +034
        move.l  a1,(a6)                         | +038
.L06a30a:
        jsr     Bazooka_ScrollGroundIfBit6_06b232(pc) | +03a
        jsr     0x28d70.l                       | +03e
        tst.b   0x73(a6)                        | +044
        beq.w   .L06a326                        | +048
        clr.b   0x73(a6)                        | +04c
        move.b  #0x4,0x72(a6)                   | +050
.L06a326:
        subq.w  #0x1,0x70(a6)                   | +056
        cmpi.w  #0x0,0x70(a6)                   | +05a
        bgt.w   .L06a33a                        | +060
        lea     Bazooka_Attack_06a2d0(pc),a1    | +064
        move.l  a1,(a6)                         | +068
.L06a33a:
        bra.w   .L06a390                        | +06a
.L06a33e:
        lea     0x2bd4ea.l,a0                   | +06e
        jsr     0x799de.l                       | +074
        move.w  d0,0x70(a6)                     | +07a
        addi.w  #0x1e,0x70(a6)                  | +07e
        clr.b   0x74(a6)                        | +084
        clr.b   0x73(a6)                        | +088
        move.b  #0x3,0x72(a6)                   | +08c
        lea     0x2ca84a.l,a0                   | +092
        jsr     0x28cd4.l                       | +098
        lea     .L06a374(pc),a1                 | +09e
        move.l  a1,(a6)                         | +0a2
.L06a374:
        jsr     Bazooka_ScrollGroundIfBit6_06b232(pc) | +0a4
        jsr     0x28d70.l                       | +0a8
        tst.b   0x73(a6)                        | +0ae
        beq.w   .L06a38c                        | +0b2
        lea     Bazooka_Turn_06a20c(pc),a1      | +0b6
        move.l  a1,(a6)                         | +0ba
.L06a38c:
        bra.w   .L06a390                        | +0bc
        .global Bazooka_Attack_06a2d0__L06a390
Bazooka_Attack_06a2d0__L06a390:
.L06a390:
        clr.b   0x78(a6)                        | +0c0
        jsr     0x2870a.l                       | +0c4
        bcc.w   .L06a3b6                        | +0ca
        lea     0x5e766.l,a0                    | +0ce
        jsr     0x5e770.l                       | +0d4
        bclr    #0x3,0x13(a6)                   | +0da
        move.b  #0x1,0x78(a6)                   | +0e0
.L06a3b6:
        jsr     0x28758.l                       | +0e6
        bcc.w   .L06a3c6                        | +0ec
        lea     Bazooka_Die_06a3d6(pc),a1       | +0f0
        move.l  a1,(a6)                         | +0f4
.L06a3c6:
        jsr     Bazooka_OffWorldOrLeft_06b1e2(pc) | +0f6
        bcc.w   SetHandlerRts_06a3d4            | +0fa

| ----------------------------------------------------------------------------
|  Bazooka_Die_06a3d6  @ $06A3D6  (62 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_Die_06a3d6, "ax", @progbits
        .global Bazooka_Die_06a3d6
Bazooka_Die_06a3d6:
        clr.w   0x28(a6)                        | +000
        bclr    #0x1,0x12(a6)                   | +004
        move.b  #0x6,0x72(a6)                   | +00a
        lea     0x2caf58.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        lea     .L06a3f8(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L06a3f8:
        jsr     Bazooka_ScrollGround_06b20e(pc) | +022
        jsr     0x28d70.l                       | +026
        bcc.w   .L06a40c                        | +02c
        lea     Bazooka_Corpse_06a41c(pc),a1    | +030
        move.l  a1,(a6)                         | +034
.L06a40c:
        jsr     Bazooka_OffWorldOrLeft_06b1e2(pc) | +036
        bcc.w   SetHandlerRts_06a41a            | +03a

| ----------------------------------------------------------------------------
|  Bazooka_Corpse_06a41c  @ $06A41C  (46 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_Corpse_06a41c, "ax", @progbits
        .global Bazooka_Corpse_06a41c
Bazooka_Corpse_06a41c:
        move.w  #0x3c,0x70(a6)                  | +000
        move.l  #0xffffffff,0x48(a6)            | +006
        bclr    #0x1,0x12(a6)                   | +00e
        move.b  #0x6,0x72(a6)                   | +014
        lea     .L06a43c(pc),a1                 | +01a
        move.l  a1,(a6)                         | +01e
.L06a43c:
        subq.w  #0x1,0x70(a6)                   | +020
        cmpi.w  #0x0,0x70(a6)                   | +024
        bgt.w   SetHandlerRts_06a450            | +02a

| ----------------------------------------------------------------------------
|  BazookaB_Tmpl5A_06a476  @ $06A476  (96 B)
| ----------------------------------------------------------------------------
        .section .text.BazookaB_Tmpl5A_06a476, "ax", @progbits
        .global BazookaB_Tmpl5A_06a476
BazookaB_Tmpl5A_06a476:
        move.b  #0x1,0x9c(a6)                   | +000
        move.b  #0x1,0x9e(a6)                   | +006
        bra.w   .L06a490                        | +00c
        clr.b   0x9c(a6)                        | +010
        move.b  #0x1,0x9e(a6)                   | +014
        .global BazookaB_Tmpl5A_06a476__L06a490
BazookaB_Tmpl5A_06a476__L06a490:
.L06a490:
        move.b  #0x1,0x9d(a6)                   | +01a
        jsr     Bazooka_SpawnWeaponGround_06b78a(pc) | +020
        lea     0x2bd774.l,a0                   | +024
        jsr     0x799de.l                       | +02a
        move.w  d0,0x36(a6)                     | +030
        lea     0x2bd7f6.l,a0                   | +034
        jsr     0x799de.l                       | +03a
        move.w  d0,0x70(a6)                     | +040
        lea     0x2bd6f2.l,a0                   | +044
        jsr     0x799de.l                       | +04a
        move.w  d0,0x66(a6)                     | +050
        jsr     Bazooka_AcquireTarget_06b3a4(pc) | +054
        move.l  #0x2ca2d8,0x4c(a6)              | +058

| ----------------------------------------------------------------------------
|  BazookaB_Walk_06a4d6  @ $06A4D6  (146 B)
| ----------------------------------------------------------------------------
        .section .text.BazookaB_Walk_06a4d6, "ax", @progbits
        .global BazookaB_Walk_06a4d6
BazookaB_Walk_06a4d6:
        jsr     Bazooka_TargetBehind_06b2dc(pc) | +000
        move.b  #0x0,0x72(a6)                   | +004
        move.w  0x36(a6),d0                     | +00a
        btst    #0x0,0x75(a6)                   | +00e
        bne.w   .L06a4f0                        | +014
        neg.w   d0                              | +018
.L06a4f0:
        move.w  d0,0x28(a6)                     | +01a
        jsr     Bazooka_FacingMismatch_06b368(pc) | +01e
        bcs.w   .L06a50c                        | +022
        lea     0x2ca690.l,a0                   | +026
        jsr     0x28cd4.l                       | +02c
        bra.w   .L06a518                        | +032
.L06a50c:
        lea     0x2ca74a.l,a0                   | +036
        jsr     0x28cd4.l                       | +03c
.L06a518:
        lea     .L06a51e(pc),a1                 | +042
        move.l  a1,(a6)                         | +046
.L06a51e:
        jsr     Bazooka_ScrollGroundIfBit6_06b232(pc) | +048
        bcc.w   .L06a532                        | +04c
        eori.b  #0x1,0x75(a6)                   | +050
        lea     BazookaB_Idle_06a568(pc),a1     | +056
        move.l  a1,(a6)                         | +05a
.L06a532:
        jsr     0x28d70.l                       | +05c
        jsr     Bazooka_FaceTarget_06b27a(pc)   | +062
        bcc.w   .L06a546                        | +066
        lea     BazookaB_Idle_06a568(pc),a1     | +06a
        move.l  a1,(a6)                         | +06e
.L06a546:
        subq.w  #0x1,0x70(a6)                   | +070
        jsr     Bazooka_TargetOverhead_06b4f2(pc) | +074
        bcc.w   .L06a558                        | +078
        lea     BazookaB_Alert_06a5cc(pc),a1    | +07c
        move.l  a1,(a6)                         | +080
.L06a558:
        jsr     0x283ca.l                       | +082
        jsr     0x283d8.l                       | +088
        bra.w   BazookaB_Attack_06a6e0__L06a772 | +08e

| ----------------------------------------------------------------------------
|  BazookaB_Idle_06a568  @ $06A568  (100 B)
| ----------------------------------------------------------------------------
        .section .text.BazookaB_Idle_06a568, "ax", @progbits
        .global BazookaB_Idle_06a568
BazookaB_Idle_06a568:
        jsr     Bazooka_PickTargetEvery4_06b38a(pc) | +000
        move.b  #0x0,0x72(a6)                   | +004
        lea     0x2bd9fe.l,a0                   | +00a
        jsr     0x799de.l                       | +010
        move.b  d0,0x74(a6)                     | +016
        clr.w   0x28(a6)                        | +01a
        lea     0x2ca84a.l,a0                   | +01e
        jsr     0x28cd4.l                       | +024
        lea     .L06a598(pc),a1                 | +02a
        move.l  a1,(a6)                         | +02e
.L06a598:
        jsr     Bazooka_ScrollGroundIfBit6_06b232(pc) | +030
        jsr     0x28d70.l                       | +034
        subq.b  #0x1,0x74(a6)                   | +03a
        cmpi.b  #0x0,0x74(a6)                   | +03e
        bgt.w   .L06a5b6                        | +044
        lea     BazookaB_Walk_06a4d6(pc),a1     | +048
        move.l  a1,(a6)                         | +04c
.L06a5b6:
        subq.w  #0x1,0x70(a6)                   | +04e
        jsr     Bazooka_TargetOverhead_06b4f2(pc) | +052
        bcc.w   .L06a5c8                        | +056
        lea     BazookaB_Alert_06a5cc(pc),a1    | +05a
        move.l  a1,(a6)                         | +05e
.L06a5c8:
        bra.w   BazookaB_Attack_06a6e0__L06a772 | +060

| ----------------------------------------------------------------------------
|  BazookaB_Alert_06a5cc  @ $06A5CC  (106 B)
| ----------------------------------------------------------------------------
        .section .text.BazookaB_Alert_06a5cc, "ax", @progbits
        .global BazookaB_Alert_06a5cc
BazookaB_Alert_06a5cc:
        tst.b   0x9e(a6)                        | +000
        beq.w   BazookaB_Reacquire_06a6c2       | +004
        jsr     0x267e2.l                       | +008
        lea     0x2bd8fa.l,a0                   | +00e
        jsr     0x799de.l                       | +014
        move.w  d0,0x70(a6)                     | +01a
        lea     0x2ca84a.l,a0                   | +01e
        jsr     0x28cd4.l                       | +024
        lea     .L06a5fc(pc),a1                 | +02a
        move.l  a1,(a6)                         | +02e
.L06a5fc:
        jsr     Bazooka_ScrollGroundIfBit6_06b232(pc) | +030
        jsr     0x28d70.l                       | +034
        subq.w  #0x1,0x70(a6)                   | +03a
        cmpi.w  #0x0,0x70(a6)                   | +03e
        bgt.w   .L06a61a                        | +044
        lea     BazookaB_Reacquire_06a6c2(pc),a1 | +048
        move.l  a1,(a6)                         | +04c
.L06a61a:
        bra.w   BazookaB_Attack_06a6e0__L06a772 | +04e
        clr.b   0x73(a6)                        | +052
        lea     0x2bd97c.l,a0                   | +056
        jsr     0x799de.l                       | +05c
        move.b  d0,0x74(a6)                     | +062
        bra.w   BazookaB_AttackLoop_06a636__L06a644 | +066

| ----------------------------------------------------------------------------
|  BazookaB_AttackLoop_06a636  @ $06A636  (140 B)
| ----------------------------------------------------------------------------
        .section .text.BazookaB_AttackLoop_06a636, "ax", @progbits
        .global BazookaB_AttackLoop_06a636
BazookaB_AttackLoop_06a636:
        subq.b  #0x1,0x74(a6)                   | +000
        cmpi.b  #0x0,0x74(a6)                   | +004
        ble.w   BazookaB_Walk_06a4d6            | +00a
        .global BazookaB_AttackLoop_06a636__L06a644
BazookaB_AttackLoop_06a636__L06a644:
.L06a644:
        clr.b   0x73(a6)                        | +00e
        move.b  #0x2,0x72(a6)                   | +012
        lea     0x2bd878.l,a0                   | +018
        jsr     0x799de.l                       | +01e
        move.w  d0,0x70(a6)                     | +024
        lea     0x2cad10.l,a0                   | +028
        jsr     0x28cd4.l                       | +02e
        lea     .L06a670(pc),a1                 | +034
        move.l  a1,(a6)                         | +038
.L06a670:
        subq.w  #0x1,0x70(a6)                   | +03a
        cmpi.w  #0x0,0x70(a6)                   | +03e
        bgt.w   .L06a684                        | +044
        lea     BazookaB_AttackLoop_06a636(pc),a1 | +048
        move.l  a1,(a6)                         | +04c
.L06a684:
        jsr     Bazooka_ScrollGroundIfBit6_06b232(pc) | +04e
        bcc.w   .L06a698                        | +052
        eori.b  #0x1,0x75(a6)                   | +056
        lea     BazookaB_Idle_06a568(pc),a1     | +05c
        move.l  a1,(a6)                         | +060
.L06a698:
        jsr     0x28d70.l                       | +062
        tst.b   0x73(a6)                        | +068
        beq.w   .L06a6b0                        | +06c
        clr.b   0x73(a6)                        | +070
        move.b  #0x0,0x72(a6)                   | +074
.L06a6b0:
        jsr     Bazooka_FaceTarget_06b27a(pc)   | +07a
        bcc.w   .L06a6be                        | +07e
        lea     BazookaB_Idle_06a568(pc),a1     | +082
        move.l  a1,(a6)                         | +086
.L06a6be:
        bra.w   BazookaB_Attack_06a6e0__L06a772 | +088

| ----------------------------------------------------------------------------
|  BazookaB_Reacquire_06a6c2  @ $06A6C2  (30 B)
| ----------------------------------------------------------------------------
        .section .text.BazookaB_Reacquire_06a6c2, "ax", @progbits
        .global BazookaB_Reacquire_06a6c2
BazookaB_Reacquire_06a6c2:
        jsr     0x267e2.l                       | +000
        clr.b   0x73(a6)                        | +006
        lea     0x2bd97c.l,a0                   | +00a
        jsr     0x799de.l                       | +010
        move.b  d0,0x74(a6)                     | +016
        bra.w   BazookaB_Attack_06a6e0__L06a6ee | +01a

| ----------------------------------------------------------------------------
|  BazookaB_Attack_06a6e0  @ $06A6E0  (212 B)
| ----------------------------------------------------------------------------
        .section .text.BazookaB_Attack_06a6e0, "ax", @progbits
        .global BazookaB_Attack_06a6e0
BazookaB_Attack_06a6e0:
        subq.b  #0x1,0x74(a6)                   | +000
        cmpi.b  #0x0,0x74(a6)                   | +004
        ble.w   .L06a752                        | +00a
        .global BazookaB_Attack_06a6e0__L06a6ee
BazookaB_Attack_06a6e0__L06a6ee:
.L06a6ee:
        clr.b   0x73(a6)                        | +00e
        move.b  #0x2,0x72(a6)                   | +012
        lea     0x2bd878.l,a0                   | +018
        jsr     0x799de.l                       | +01e
        move.w  d0,0x70(a6)                     | +024
        lea     0x2cacc0.l,a0                   | +028
        jsr     0x28cd4.l                       | +02e
        lea     .L06a71a(pc),a1                 | +034
        move.l  a1,(a6)                         | +038
.L06a71a:
        jsr     Bazooka_ScrollGroundIfBit6_06b232(pc) | +03a
        jsr     0x28d70.l                       | +03e
        bcc.w   .L06a73a                        | +044
        tst.b   0x73(a6)                        | +048
        beq.w   .L06a73a                        | +04c
        clr.b   0x73(a6)                        | +050
        move.b  #0x0,0x72(a6)                   | +054
.L06a73a:
        subq.w  #0x1,0x70(a6)                   | +05a
        cmpi.w  #0x0,0x70(a6)                   | +05e
        bgt.w   .L06a74e                        | +064
        lea     BazookaB_Attack_06a6e0(pc),a1   | +068
        move.l  a1,(a6)                         | +06c
.L06a74e:
        bra.w   .L06a772                        | +06e
.L06a752:
        lea     0x2bd7f6.l,a0                   | +072
        jsr     0x799de.l                       | +078
        move.w  d0,0x70(a6)                     | +07e
        move.l  #0xffffffff,0x90(a6)            | +082
        jsr     Bazooka_AcquireTarget_06b3a4(pc) | +08a
        bra.w   BazookaB_Idle_06a568            | +08e
        .global BazookaB_Attack_06a6e0__L06a772
BazookaB_Attack_06a6e0__L06a772:
.L06a772:
        jsr     Bazooka_AcquireTarget_06b3a4(pc) | +092
        clr.b   0x78(a6)                        | +096
        jsr     0x2870a.l                       | +09a
        bcc.w   .L06a79c                        | +0a0
        lea     0x5e766.l,a0                    | +0a4
        jsr     0x5e770.l                       | +0aa
        bclr    #0x3,0x13(a6)                   | +0b0
        move.b  #0x1,0x78(a6)                   | +0b6
.L06a79c:
        jsr     0x28758.l                       | +0bc
        bcc.w   .L06a7ac                        | +0c2
        lea     Bazooka_Die_06a3d6(pc),a1       | +0c6
        move.l  a1,(a6)                         | +0ca
.L06a7ac:
        jsr     Bazooka_OffWorldOrLeft_06b1e2(pc) | +0cc
        bcc.w   SetHandlerRts_06a7ba            | +0d0

| ----------------------------------------------------------------------------
|  BazookaCrew_Tmpl5C_06a7bc  @ $06A7BC  (14 B)
| ----------------------------------------------------------------------------
        .section .text.BazookaCrew_Tmpl5C_06a7bc, "ax", @progbits
        .global BazookaCrew_Tmpl5C_06a7bc
BazookaCrew_Tmpl5C_06a7bc:
        move.b  #0x1,0x9c(a6)                   | +000
        clr.b   0x9e(a6)                        | +006
        bra.w   BazookaB_Tmpl5A_06a476__L06a490 | +00a

| ----------------------------------------------------------------------------
|  BazookaCrew_Tmpl5D_06a7ca  @ $06A7CA  (12 B)
| ----------------------------------------------------------------------------
        .section .text.BazookaCrew_Tmpl5D_06a7ca, "ax", @progbits
        .global BazookaCrew_Tmpl5D_06a7ca
BazookaCrew_Tmpl5D_06a7ca:
        clr.b   0x9c(a6)                        | +000
        clr.b   0x9e(a6)                        | +004
        bra.w   BazookaB_Tmpl5A_06a476__L06a490 | +008

| ----------------------------------------------------------------------------
|  BazookaCrew_Init_06a7d6  @ $06A7D6  (38 B)
| ----------------------------------------------------------------------------
        .section .text.BazookaCrew_Init_06a7d6, "ax", @progbits
        .global BazookaCrew_Init_06a7d6
BazookaCrew_Init_06a7d6:
        clr.b   0x7a(a6)                        | +000
        bra.w   .L06a7e4                        | +004
        move.b  #0x1,0x7a(a6)                   | +008
.L06a7e4:
        jsr     Bazooka_SpawnWeaponTable_06b810(pc) | +00e
        clr.b   0x74(a6)                        | +012
        lea     0x2bd4ea.l,a0                   | +016
        jsr     0x799de.l                       | +01c
        move.w  d0,0x70(a6)                     | +022

| ----------------------------------------------------------------------------
|  BazookaCrew_Idle_06a7fc  @ $06A7FC  (108 B)
| ----------------------------------------------------------------------------
        .section .text.BazookaCrew_Idle_06a7fc, "ax", @progbits
        .global BazookaCrew_Idle_06a7fc
BazookaCrew_Idle_06a7fc:
        jsr     0x5e0d4.l                       | +000
        move.l  a0,0x90(a6)                     | +006
        move.b  #0x4,0x72(a6)                   | +00a
        clr.b   0x73(a6)                        | +010
        lea     0x2ca804.l,a0                   | +014
        jsr     0x28cd4.l                       | +01a
        lea     .L06a822(pc),a1                 | +020
        move.l  a1,(a6)                         | +024
.L06a822:
        jsr     Bazooka_FollowParentOffs_06b87a(pc) | +026
        jsr     0x28d70.l                       | +02a
        jsr     Bazooka_PastTargetX_06b8bc(pc)  | +030
        bcc.w   .L06a864                        | +034
        tst.b   0x74(a6)                        | +038
        beq.w   .L06a842                        | +03c
        lea     BazookaCrew_Attack_06a878(pc),a1 | +040
        move.l  a1,(a6)                         | +044
.L06a842:
        move.b  #0x1,0x72(a6)                   | +046
        subq.w  #0x1,0x70(a6)                   | +04c
        tst.b   0x73(a6)                        | +050
        beq.w   .L06a864                        | +054
        cmpi.w  #0x0,0x70(a6)                   | +058
        bgt.w   .L06a864                        | +05e
        lea     BazookaCrew_RngTimer_06a868(pc),a1 | +062
        move.l  a1,(a6)                         | +066
.L06a864:
        bra.w   BazookaCrew_HitTest_06a96c      | +068

| ----------------------------------------------------------------------------
|  BazookaCrew_RngTimer_06a868  @ $06A868  (16 B)
| ----------------------------------------------------------------------------
        .section .text.BazookaCrew_RngTimer_06a868, "ax", @progbits
        .global BazookaCrew_RngTimer_06a868
BazookaCrew_RngTimer_06a868:
        lea     0x2bd5ee.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.b  d0,0x74(a6)                     | +00c

| ----------------------------------------------------------------------------
|  BazookaCrew_Attack_06a878  @ $06A878  (144 B)
| ----------------------------------------------------------------------------
        .section .text.BazookaCrew_Attack_06a878, "ax", @progbits
        .global BazookaCrew_Attack_06a878
BazookaCrew_Attack_06a878:
        lea     0x2bd56c.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,0x70(a6)                     | +00c
        clr.b   0x73(a6)                        | +010
        move.b  #0x2,0x72(a6)                   | +014
        lea     0x2cacc0.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        lea     .L06a8a4(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L06a8a4:
        jsr     Bazooka_FollowParentOffs_06b87a(pc) | +02c
        jsr     0x28d70.l                       | +030
        bcc.w   .L06a8be                        | +036
        lea     0x2ca804.l,a0                   | +03a
        jsr     0x28cd4.l                       | +040
.L06a8be:
        subq.w  #0x1,0x70(a6)                   | +046
        cmpi.w  #0x0,0x70(a6)                   | +04a
        bgt.w   .L06a904                        | +050
        tst.b   0x73(a6)                        | +054
        beq.w   .L06a904                        | +058
        move.b  #0x4,0x72(a6)                   | +05c
        lea     BazookaCrew_Attack_06a878(pc),a1 | +062
        move.l  a1,(a6)                         | +066
        subq.b  #0x1,0x74(a6)                   | +068
        cmpi.b  #0x0,0x74(a6)                   | +06c
        bgt.w   .L06a904                        | +072
        lea     0x2bd4ea.l,a0                   | +076
        jsr     0x799de.l                       | +07c
        move.w  d0,0x70(a6)                     | +082
        lea     BazookaCrew_Idle_06a7fc(pc),a1  | +086
        move.l  a1,(a6)                         | +08a
.L06a904:
        bra.w   BazookaCrew_HitTest_06a96c      | +08c

| ----------------------------------------------------------------------------
|  BazookaCrew_Hurt_06a908  @ $06A908  (54 B)
| ----------------------------------------------------------------------------
        .section .text.BazookaCrew_Hurt_06a908, "ax", @progbits
        .global BazookaCrew_Hurt_06a908
BazookaCrew_Hurt_06a908:
        move.b  #0x5,0x72(a6)                   | +000
        lea     0x2cae00.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     .L06a920(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L06a920:
        jsr     Bazooka_FollowParentOffs_06b87a(pc) | +018
        jsr     0x28d70.l                       | +01c
        bcc.w   .L06a93a                        | +022
        bclr    #0x3,0x13(a6)                   | +026
        lea     BazookaCrew_Idle_06a7fc(pc),a1  | +02c
        move.l  a1,(a6)                         | +030
.L06a93a:
        bra.w   BazookaCrew_HitTest_06a96c__L06a9e2 | +032

| ----------------------------------------------------------------------------
|  BazookaCrew_Die_06a93e  @ $06A93E  (38 B)
| ----------------------------------------------------------------------------
        .section .text.BazookaCrew_Die_06a93e, "ax", @progbits
        .global BazookaCrew_Die_06a93e
BazookaCrew_Die_06a93e:
        move.b  #0x6,0x72(a6)                   | +000
        lea     0x2caf58.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     .L06a956(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L06a956:
        jsr     Bazooka_FollowParentOffs_06b87a(pc) | +018
        jsr     0x28d70.l                       | +01c
        bcc.w   SetHandlerRts_06a96a            | +022

| ----------------------------------------------------------------------------
|  BazookaCrew_HitTest_06a96c  @ $06A96C  (144 B)
| ----------------------------------------------------------------------------
        .section .text.BazookaCrew_HitTest_06a96c, "ax", @progbits
        .global BazookaCrew_HitTest_06a96c
BazookaCrew_HitTest_06a96c:
        clr.b   0x78(a6)                        | +000
        movea.l 0xc(a6),a0                      | +004
        cmpi.b  #0xff,0x20(a0)                  | +008
        bne.w   .L06a9a0                        | +00e
        btst    #0x0,0x13(a6)                   | +012
        bne.w   .L06a9a0                        | +018
        addi.w  #0x20,0x24(a6)                  | +01c
        lea     0x6ba34.l,a1                    | +022
        move.l  a1,(a6)                         | +028
        move.b  #0x7,0x72(a6)                   | +02a
        bra.w   .L06a9f2                        | +030
.L06a9a0:
        lea     0x5e766.l,a0                    | +034
        jsr     0x5e770.l                       | +03a
        jsr     0x2870a.l                       | +040
        bcc.w   .L06a9e2                        | +046
        bclr    #0x3,0x13(a6)                   | +04a
        cmpi.b  #0x2,0x58(a6)                   | +050
        beq.w   .L06a9d0                        | +056
        cmpi.b  #0x3,0x58(a6)                   | +05a
        bne.w   .L06a9e2                        | +060
.L06a9d0:
        bset    #0x3,0x13(a6)                   | +064
        lea     BazookaCrew_Hurt_06a908(pc),a1  | +06a
        move.l  a1,(a6)                         | +06e
        move.b  #0x1,0x78(a6)                   | +070
        .global BazookaCrew_HitTest_06a96c__L06a9e2
BazookaCrew_HitTest_06a96c__L06a9e2:
.L06a9e2:
        jsr     0x28758.l                       | +076
        bcc.w   .L06a9f2                        | +07c
        lea     BazookaCrew_Die_06a93e(pc),a1   | +080
        move.l  a1,(a6)                         | +084
.L06a9f2:
        jsr     0x5e45a.l                       | +086
        bcc.w   SetHandlerRts_06aa02            | +08c

| ----------------------------------------------------------------------------
|  AllyBazooka_Tmpl40_06aa04  @ $06AA04  (88 B)
| ----------------------------------------------------------------------------
        .section .text.AllyBazooka_Tmpl40_06aa04, "ax", @progbits
        .global AllyBazooka_Tmpl40_06aa04
AllyBazooka_Tmpl40_06aa04:
        clr.b   0x9e(a6)                        | +000
        clr.b   0x7a(a6)                        | +004
        jsr     Bazooka_SpawnWeaponTable_06b810(pc) | +008
        bra.w   .L06aa28                        | +00c
        clr.b   0x9e(a6)                        | +010
        clr.b   0x7a(a6)                        | +014
        jsr     Bazooka_SpawnWeaponTable_06b810(pc) | +018
        move.l  #0xffffffff,0x48(a6)            | +01c
.L06aa28:
        move.b  #0x1,0x98(a0)                   | +024
        clr.b   0x9f(a0)                        | +02a
        move.b  0x98(a6),d0                     | +02e
        andi.w  #0xff,d0                        | +032
        lsl.w   #0x4,d0                         | +036
        move.w  #0x110,d0                       | +038
        move.w  d0,0x98(a6)                     | +03c
        clr.b   0x75(a6)                        | +040
        clr.b   0x74(a6)                        | +044
        lea     0x2bd4ea.l,a0                   | +048
        jsr     0x799de.l                       | +04e
        move.w  d0,0x70(a6)                     | +054

| ----------------------------------------------------------------------------
|  AllyBazooka_Advance_06aa5c  @ $06AA5C  (110 B)
| ----------------------------------------------------------------------------
        .section .text.AllyBazooka_Advance_06aa5c, "ax", @progbits
        .global AllyBazooka_Advance_06aa5c
AllyBazooka_Advance_06aa5c:
        jsr     0x5e0d4.l                       | +000
        move.l  a0,0x90(a6)                     | +006
        move.b  #0x4,0x72(a6)                   | +00a
        clr.b   0x73(a6)                        | +010
        lea     0x2ca832.l,a0                   | +014
        jsr     0x28cd4.l                       | +01a
        lea     .L06aa82(pc),a1                 | +020
        move.l  a1,(a6)                         | +024
.L06aa82:
        jsr     0x2783a.l                       | +026
        jsr     0x28d70.l                       | +02c
        tst.b   0x74(a6)                        | +032
        bne.w   .L06aa9c                        | +036
        lea     AllyBazooka_Attack_06aada(pc),a1 | +03a
        move.l  a1,(a6)                         | +03e
.L06aa9c:
        jsr     Bazooka_PastTargetX_06b8bc(pc)  | +040
        bcc.w   .L06aac6                        | +044
        move.b  #0x1,0x72(a6)                   | +048
        subq.w  #0x1,0x70(a6)                   | +04e
        tst.b   0x73(a6)                        | +052
        beq.w   .L06aac6                        | +056
        cmpi.w  #0x0,0x70(a6)                   | +05a
        bgt.w   .L06aac6                        | +060
        lea     AllyBazooka_PickBurst_06aaca(pc),a1 | +064
        move.l  a1,(a6)                         | +068
.L06aac6:
        bra.w   AllyBazooka_HitTest_06abe2      | +06a

| ----------------------------------------------------------------------------
|  AllyBazooka_PickBurst_06aaca  @ $06AACA  (16 B)
| ----------------------------------------------------------------------------
        .section .text.AllyBazooka_PickBurst_06aaca, "ax", @progbits
        .global AllyBazooka_PickBurst_06aaca
AllyBazooka_PickBurst_06aaca:
        lea     0x2bd5ee.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.b  d0,0x74(a6)                     | +00c

| ----------------------------------------------------------------------------
|  AllyBazooka_Attack_06aada  @ $06AADA  (146 B)
| ----------------------------------------------------------------------------
        .section .text.AllyBazooka_Attack_06aada, "ax", @progbits
        .global AllyBazooka_Attack_06aada
AllyBazooka_Attack_06aada:
        lea     0x2bd56c.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,0x70(a6)                     | +00c
        clr.b   0x73(a6)                        | +010
        move.b  #0x2,0x72(a6)                   | +014
        lea     0x2cacc0.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        lea     .L06ab06(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L06ab06:
        jsr     0x2783a.l                       | +02c
        jsr     0x28d70.l                       | +032
        bcc.w   .L06ab22                        | +038
        lea     0x2ca832.l,a0                   | +03c
        jsr     0x28cd4.l                       | +042
.L06ab22:
        subq.w  #0x1,0x70(a6)                   | +048
        cmpi.w  #0x0,0x70(a6)                   | +04c
        bgt.w   .L06ab68                        | +052
        tst.b   0x73(a6)                        | +056
        beq.w   .L06ab68                        | +05a
        move.b  #0x4,0x72(a6)                   | +05e
        lea     AllyBazooka_Attack_06aada(pc),a1 | +064
        move.l  a1,(a6)                         | +068
        subq.b  #0x1,0x74(a6)                   | +06a
        cmpi.b  #0x0,0x74(a6)                   | +06e
        bgt.w   .L06ab68                        | +074
        lea     0x2bd4ea.l,a0                   | +078
        jsr     0x799de.l                       | +07e
        move.w  d0,0x70(a6)                     | +084
        lea     AllyBazooka_Advance_06aa5c(pc),a1 | +088
        move.l  a1,(a6)                         | +08c
.L06ab68:
        bra.w   AllyBazooka_HitTest_06abe2      | +08e

| ----------------------------------------------------------------------------
|  AllyBazooka_Hurt_06ab6c  @ $06AB6C  (56 B)
| ----------------------------------------------------------------------------
        .section .text.AllyBazooka_Hurt_06ab6c, "ax", @progbits
        .global AllyBazooka_Hurt_06ab6c
AllyBazooka_Hurt_06ab6c:
        move.b  #0x5,0x72(a6)                   | +000
        lea     0x2cae00.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     .L06ab84(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L06ab84:
        jsr     0x2783a.l                       | +018
        jsr     0x28d70.l                       | +01e
        bcc.w   .L06aba0                        | +024
        bclr    #0x3,0x13(a6)                   | +028
        lea     AllyBazooka_Advance_06aa5c(pc),a1 | +02e
        move.l  a1,(a6)                         | +032
.L06aba0:
        bra.w   AllyBazooka_HitTest_06abe2__L06ac28 | +034

| ----------------------------------------------------------------------------
|  AllyBazooka_Die_06aba4  @ $06ABA4  (54 B)
| ----------------------------------------------------------------------------
        .section .text.AllyBazooka_Die_06aba4, "ax", @progbits
        .global AllyBazooka_Die_06aba4
AllyBazooka_Die_06aba4:
        move.b  #0x6,0x72(a6)                   | +000
        lea     0x2caf58.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     .L06abbc(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L06abbc:
        jsr     0x2783a.l                       | +018
        jsr     0x28d70.l                       | +01e
        bcc.w   .L06abd2                        | +024
        lea     Bazooka_Corpse_06a41c(pc),a1    | +028
        move.l  a1,(a6)                         | +02c
.L06abd2:
        jsr     Bazooka_OffWorldOrLeft_06b1e2(pc) | +02e
        bcc.w   SetHandlerRts_06abe0            | +032

| ----------------------------------------------------------------------------
|  AllyBazooka_HitTest_06abe2  @ $06ABE2  (114 B)
| ----------------------------------------------------------------------------
        .section .text.AllyBazooka_HitTest_06abe2, "ax", @progbits
        .global AllyBazooka_HitTest_06abe2
AllyBazooka_HitTest_06abe2:
        clr.b   0x78(a6)                        | +000
        lea     0x5e766.l,a0                    | +004
        jsr     0x5e770.l                       | +00a
        jsr     0x2870a.l                       | +010
        bcc.w   .L06ac28                        | +016
        bclr    #0x3,0x13(a6)                   | +01a
        cmpi.b  #0x2,0x58(a6)                   | +020
        beq.w   .L06ac16                        | +026
        cmpi.b  #0x3,0x58(a6)                   | +02a
        bne.w   .L06ac28                        | +030
.L06ac16:
        bset    #0x3,0x13(a6)                   | +034
        lea     AllyBazooka_Hurt_06ab6c(pc),a1  | +03a
        move.l  a1,(a6)                         | +03e
        move.b  #0x1,0x78(a6)                   | +040
        .global AllyBazooka_HitTest_06abe2__L06ac28
AllyBazooka_HitTest_06abe2__L06ac28:
.L06ac28:
        jsr     0x28758.l                       | +046
        bcc.w   .L06ac38                        | +04c
        lea     AllyBazooka_Die_06aba4(pc),a1   | +050
        move.l  a1,(a6)                         | +054
.L06ac38:
        movea.l 0xc(a6),a0                      | +056
        cmpi.b  #0xff,0x20(a0)                  | +05a
        bne.w   .L06ac4c                        | +060
        lea     AllyBazooka_Die_06aba4(pc),a1   | +064
        move.l  a1,(a6)                         | +068
.L06ac4c:
        jsr     Bazooka_OffWorldOrLeft_06b1e2(pc) | +06a
        bcc.w   SetHandlerRts_06ac5a            | +06e

| ----------------------------------------------------------------------------
|  AllyBazooka_Spawner_Tmpl41_06ac5c  @ $06AC5C  (70 B)
| ----------------------------------------------------------------------------
        .section .text.AllyBazooka_Spawner_Tmpl41_06ac5c, "ax", @progbits
        .global AllyBazooka_Spawner_Tmpl41_06ac5c
AllyBazooka_Spawner_Tmpl41_06ac5c:
        clr.b   0x20(a6)                        | +000
        lea     AllyBazooka_Tmpl40_06aa04(pc),a1 | +004
        jsr     0x4ae.l                         | +008
        jsr     0x5dd02.l                       | +00e
        move.b  #0x12,0x98(a0)                  | +014
        move.l  a0,0x5c(a6)                     | +01a
        lea     .L06ac80(pc),a1                 | +01e
        move.l  a1,(a6)                         | +022
.L06ac80:
        jsr     0x2783a.l                       | +024
        movea.l 0x5c(a6),a0                     | +02a
        btst    #0x0,0x13(a0)                   | +02e
        beq.w   .L06ac9a                        | +034
        lea     JmpToScheduler_06acaa(pc),a1    | +038
        move.l  a1,(a6)                         | +03c
.L06ac9a:
        jsr     Bazooka_OffWorldOrLeft_06b1e2(pc) | +03e
        bcc.w   SetHandlerRts_06aca8            | +042

| ----------------------------------------------------------------------------
|  BazookaWeapon_Child_06acb2  @ $06ACB2  (614 B)
| ----------------------------------------------------------------------------
        .section .text.BazookaWeapon_Child_06acb2, "ax", @progbits
        .global BazookaWeapon_Child_06acb2
BazookaWeapon_Child_06acb2:
        jsr     Bazooka_PlayCry_06b94c(pc)      | +000
        andi.b  #0x3,0x98(a6)                   | +004
        move.b  #0x0,0x72(a6)                   | +00a
        tst.b   0x9d(a6)                        | +010
        bne.w   .L06aebc                        | +014
        move.w  #0x8,0x34(a6)                   | +018
        lea     0x2cb082.l,a0                   | +01e
        jsr     0x28cd4.l                       | +024
        lea     .L06ace2(pc),a1                 | +02a
        move.l  a1,(a6)                         | +02e
.L06ace2:
        cmpi.b  #0x6,0x72(a6)                   | +030
        beq.w   BazookaWeapon_FlyOff_06af68     | +036
        jsr     BazookaWeapon_StateLookup_06b8ea(pc) | +03a
        bcc.w   .L06acf6                        | +03e
        jmp     (a1)                            | +042
.L06acf6:
        jsr     BazookaWeapon_FollowParent_06b550(pc) | +044
        jsr     0x28d70.l                       | +048
        bcc.w   .L06ad10                        | +04e
        lea     0x2cb082.l,a0                   | +052
        jsr     0x28cd4.l                       | +058
.L06ad10:
        bra.w   BazookaWeapon_SpriteByParent78_06af4e | +05e
        clr.b   0x74(a6)                        | +062
        movea.l 0xc(a6),a0                      | +066
        move.l  0x90(a0),0x90(a6)               | +06a
        lea     0x2cb082.l,a0                   | +070
        jsr     0x28cd4.l                       | +076
        lea     .L06ad34(pc),a1                 | +07c
        move.l  a1,(a6)                         | +080
.L06ad34:
        addq.b  #0x1,0x74(a6)                   | +082
        andi.b  #0x3,0x74(a6)                   | +086
        bne.w   .L06ad68                        | +08c
        jsr     Bazooka_RangeClass_06b5a4(pc)   | +090
        movea.l 0xc(a6),a0                      | +094
        clr.b   0x73(a0)                        | +098
        jsr     Bazooka_AimStep_06b5ea(pc)      | +09c
        bcc.w   .L06ad68                        | +0a0
        jsr     Bazooka_AimDone_06b6cc(pc)      | +0a4
        bcs.w   .L06ad68                        | +0a8
        movea.l 0xc(a6),a0                      | +0ac
        move.b  #0x1,0x73(a0)                   | +0b0
.L06ad68:
        cmpi.b  #0x6,0x72(a6)                   | +0b6
        beq.w   BazookaWeapon_FlyOff_06af68     | +0bc
        jsr     BazookaWeapon_StateLookup_06b8ea(pc) | +0c0
        bcc.w   .L06ad7c                        | +0c4
        jmp     (a1)                            | +0c8
.L06ad7c:
        jsr     BazookaWeapon_FollowParent_06b550(pc) | +0ca
        jsr     0x28d70.l                       | +0ce
        bcc.w   .L06ad96                        | +0d4
        lea     0x2cb082.l,a0                   | +0d8
        jsr     0x28cd4.l                       | +0de
.L06ad96:
        bra.w   BazookaWeapon_SpriteByParent78_06af4e | +0e4
        lea     0x2cb1ac.l,a0                   | +0e8
        move.w  0x34(a6),d0                     | +0ee
        andi.w  #0xf,d0                         | +0f2
        lsl.w   #0x2,d0                         | +0f6
        movea.l (a0,d0.w),a0                    | +0f8
        cmpa.l  #0xffffffff,a0                  | +0fc
        beq.w   .L06adbe                        | +102
        jsr     0x28cd4.l                       | +106
.L06adbe:
        jsr     Bazooka_Fire_06b6f2(pc)         | +10c
        lea     .L06adc8(pc),a1                 | +110
        move.l  a1,(a6)                         | +114
.L06adc8:
        cmpi.b  #0x6,0x72(a6)                   | +116
        beq.w   BazookaWeapon_FlyOff_06af68     | +11c
        jsr     BazookaWeapon_StateLookup_06b8ea(pc) | +120
        bcc.w   .L06addc                        | +124
        jmp     (a1)                            | +128
.L06addc:
        jsr     BazookaWeapon_FollowParent_06b550(pc) | +12a
        jsr     0x28d70.l                       | +12e
        bcc.w   .L06adf4                        | +134
        movea.l 0xc(a6),a0                      | +138
        move.b  #0x1,0x73(a0)                   | +13c
.L06adf4:
        rts                                     | +142
        clr.b   0x74(a6)                        | +144
        lea     0x2cb082.l,a0                   | +148
        jsr     0x28cd4.l                       | +14e
        lea     .L06ae0c(pc),a1                 | +154
        move.l  a1,(a6)                         | +158
.L06ae0c:
        addq.b  #0x1,0x74(a6)                   | +15a
        andi.b  #0x3,0x74(a6)                   | +15e
        bne.w   .L06ae42                        | +164
        move.w  #0x1,d1                         | +168
        cmpi.w  #0x8,0x34(a6)                   | +16c
        beq.w   .L06ae38                        | +172
        blt.w   .L06ae30                        | +176
        move.w  #0xffff,d1                      | +17a
.L06ae30:
        add.w   d1,0x34(a6)                     | +17e
        bra.w   .L06ae42                        | +182
.L06ae38:
        movea.l 0xc(a6),a0                      | +186
        move.b  #0x1,0x73(a0)                   | +18a
.L06ae42:
        cmpi.b  #0x6,0x72(a6)                   | +190
        beq.w   BazookaWeapon_FlyOff_06af68     | +196
        jsr     BazookaWeapon_StateLookup_06b8ea(pc) | +19a
        bcc.w   .L06ae56                        | +19e
        jmp     (a1)                            | +1a2
.L06ae56:
        jsr     BazookaWeapon_FollowParent_06b550(pc) | +1a4
        jsr     0x28d70.l                       | +1a8
        bcc.w   .L06ae70                        | +1ae
        lea     0x2cb082.l,a0                   | +1b2
        jsr     0x28cd4.l                       | +1b8
.L06ae70:
        bra.w   BazookaWeapon_SpriteByParent78_06af4e | +1be
        clr.b   0x74(a6)                        | +1c2
        lea     0x2cb082.l,a0                   | +1c6
        jsr     0x28cd4.l                       | +1cc
        lea     .L06ae8a(pc),a1                 | +1d2
        move.l  a1,(a6)                         | +1d6
.L06ae8a:
        cmpi.b  #0x6,0x72(a6)                   | +1d8
        beq.w   BazookaWeapon_FlyOff_06af68     | +1de
        jsr     BazookaWeapon_StateLookup_06b8ea(pc) | +1e2
        bcc.w   .L06ae9e                        | +1e6
        jmp     (a1)                            | +1ea
.L06ae9e:
        jsr     BazookaWeapon_FollowParent_06b550(pc) | +1ec
        jsr     0x28d70.l                       | +1f0
        bcc.w   .L06aeb8                        | +1f6
        lea     0x2cb082.l,a0                   | +1fa
        jsr     0x28cd4.l                       | +200
.L06aeb8:
        bra.w   BazookaWeapon_SpriteByParent78_06af4e | +206
.L06aebc:
        clr.b   0x74(a6)                        | +20a
        move.w  #0xc,d0                         | +20e
        tst.b   0x9c(a6)                        | +212
        beq.w   .L06aed0                        | +216
        move.w  #0x4,d0                         | +21a
.L06aed0:
        move.w  d0,0x34(a6)                     | +21e
        lea     0x2cb082.l,a0                   | +222
        jsr     0x28cd4.l                       | +228
        lea     .L06aee6(pc),a1                 | +22e
        move.l  a1,(a6)                         | +232
.L06aee6:
        cmpi.b  #0x6,0x72(a6)                   | +234
        beq.w   BazookaWeapon_FlyOff_06af68     | +23a
        jsr     BazookaWeapon_StateLookup_06b8ea(pc) | +23e
        bcc.w   .L06aefa                        | +242
        jmp     (a1)                            | +246
.L06aefa:
        jsr     BazookaWeapon_FollowParent_06b550(pc) | +248
        jsr     0x28d70.l                       | +24c
        bcc.w   .L06af14                        | +252
        lea     0x2cb082.l,a0                   | +256
        jsr     0x28cd4.l                       | +25c
.L06af14:
        bra.w   BazookaWeapon_SpriteByParent78_06af4e | +262

| ----------------------------------------------------------------------------
|  BazookaWeapon_State5_06af18  @ $06AF18  (46 B)
| ----------------------------------------------------------------------------
        .section .text.BazookaWeapon_State5_06af18, "ax", @progbits
        .global BazookaWeapon_State5_06af18
BazookaWeapon_State5_06af18:
        lea     0x2cc1f0.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L06af2a(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L06af2a:
        cmpi.b  #0x6,0x72(a6)                   | +012
        beq.w   BazookaWeapon_FlyOff_06af68     | +018
        jsr     BazookaWeapon_StateLookup_06b8ea(pc) | +01c
        bcc.w   .L06af3e                        | +020
        jmp     (a1)                            | +024
.L06af3e:
        jsr     BazookaWeapon_FollowParent_06b550(pc) | +026
        subq.w  #0x1,0x38(a6)                   | +02a

| ----------------------------------------------------------------------------
|  BazookaWeapon_SpriteByParent78_06af4e  @ $06AF4E  (18 B)
| ----------------------------------------------------------------------------
        .section .text.BazookaWeapon_SpriteByParent78_06af4e, "ax", @progbits
        .global BazookaWeapon_SpriteByParent78_06af4e
BazookaWeapon_SpriteByParent78_06af4e:
        movea.l 0xc(a6),a0                      | +000
        tst.b   0x78(a0)                        | +004
        beq.w   JsrAbsRts_06af66                | +008
        lea     0x2cb8ce.l,a0                   | +00c

| ----------------------------------------------------------------------------
|  BazookaWeapon_FlyOff_06af68  @ $06AF68  (134 B)
| ----------------------------------------------------------------------------
        .section .text.BazookaWeapon_FlyOff_06af68, "ax", @progbits
        .global BazookaWeapon_FlyOff_06af68
BazookaWeapon_FlyOff_06af68:
        move.w  #0xffde,d0                      | +000
        jsr     0x5dca4.l                       | +004
        move.w  d0,0x28(a6)                     | +00a
        move.w  #0x663,0x2a(a6)                 | +00e
        move.w  #0xff93,0x2e(a6)                | +014
        move.w  #0x0,0x2c(a6)                   | +01a
        move.w  #0x4000,d0                      | +020
        jsr     0x28134.l                       | +024
        andi.w  #0xffe3,0x38(a6)                | +02a
        ori.w   #0x0,0x38(a6)                   | +030
        lea     .L06afa4(pc),a1                 | +036
        move.l  a1,(a6)                         | +03a
.L06afa4:
        move.w  0x34(a6),0x5c(a6)               | +03c
        jsr     0x27bc8.l                       | +042
        bcc.w   .L06afba                        | +048
        lea     BazookaWeapon_Explode_06b014(pc),a1 | +04c
        move.l  a1,(a6)                         | +050
.L06afba:
        move.w  0x5c(a6),0x34(a6)               | +052
        jsr     0x28d70.l                       | +058
        cmpi.w  #0xc0,0x24(a6)                  | +05e
        bgt.w   .L06afd6                        | +064
        lea     JmpToScheduler_06a460(pc),a1    | +068
        move.l  a1,(a6)                         | +06c
.L06afd6:
        jsr     0x5e45a.l                       | +06e
        bcc.w   .L06afe6                        | +074
        lea     BazookaWeapon_Explode_06b014(pc),a1 | +078
        move.l  a1,(a6)                         | +07c
.L06afe6:
        jsr     Bazooka_OffWorldOrLeft_06b1e2(pc) | +07e
        bcc.w   SetHandlerRts_06aff4            | +082

| ----------------------------------------------------------------------------
|  BazookaWeapon_Fall_06aff6  @ $06AFF6  (22 B)
| ----------------------------------------------------------------------------
        .section .text.BazookaWeapon_Fall_06aff6, "ax", @progbits
        .global BazookaWeapon_Fall_06aff6
BazookaWeapon_Fall_06aff6:
        lea     .L06affc(pc),a1                 | +000
        move.l  a1,(a6)                         | +004
.L06affc:
        jsr     0x2783a.l                       | +006
        jsr     0x28d70.l                       | +00c
        bcc.w   SetHandlerRts_06b012            | +012

| ----------------------------------------------------------------------------
|  BazookaWeapon_Explode_06b014  @ $06B014  (36 B)
| ----------------------------------------------------------------------------
        .section .text.BazookaWeapon_Explode_06b014, "ax", @progbits
        .global BazookaWeapon_Explode_06b014
BazookaWeapon_Explode_06b014:
        move.w  #0x4000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x0,0x38(a6)                   | +010
        move.l  #0xffffffff,0x48(a6)            | +016
        jmp     0x77f6a.l                       | +01e

| ----------------------------------------------------------------------------
|  Bazooka_MuzzleFlash_06b038  @ $06B038  (114 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_MuzzleFlash_06b038, "ax", @progbits
        .global Bazooka_MuzzleFlash_06b038
Bazooka_MuzzleFlash_06b038:
        move.w  #0x4000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x1c,0x38(a6)                  | +010
        move.w  #0x8,d1                         | +016
        jsr     0x236e.l                        | +01a
        move.w  0x34(a6),d0                     | +020
        andi.w  #0x7,d0                         | +024
        bra.w   .L06b082                        | +028
        move.w  0x34(a6),d0                     | +02c
        lsr.w   #0x3,d0                         | +030
        andi.w  #0x1,d0                         | +032
        eor.b   d0,0x3a(a6)                     | +036
        lea     0x2dd6d2.l,a0                   | +03a
        jsr     0x28cd4.l                       | +040
        bra.w   .L06b0a6                        | +046
.L06b082:
        move.b  0x76(a6),d0                     | +04a
        move.b  0x77(a6),d1                     | +04e
        ext.w   d0                              | +052
        ext.w   d1                              | +054
        add.w   d0,0x22(a6)                     | +056
        add.w   d1,0x24(a6)                     | +05a
        subq.w  #0x8,0x24(a6)                   | +05e
        lea     0x2dd6d2.l,a0                   | +062
        jsr     0x28cd4.l                       | +068
.L06b0a6:
        bra.w   BazookaWeapon_Fall_06aff6       | +06e

| ----------------------------------------------------------------------------
|  Bazooka_Rocket_06b0aa  @ $06B0AA  (290 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_Rocket_06b0aa, "ax", @progbits
        .global Bazooka_Rocket_06b0aa
Bazooka_Rocket_06b0aa:
        lea     0x2ca4b0.l,a1                   | +000
        move.b  0x98(a6),d0                     | +006
        andi.w  #0x7,d0                         | +00a
        lsl.w   #0x2,d0                         | +00e
        movea.l (a1,d0.w),a1                    | +010
        move.w  0x34(a6),d0                     | +014
        andi.w  #0xf,d0                         | +018
        move.w  d0,d1                           | +01c
        add.w   d0,d0                           | +01e
        add.w   d1,d0                           | +020
        add.w   d0,d0                           | +022
        ext.l   d0                              | +024
        adda.l  d0,a1                           | +026
        move.w  (a1),0x28(a6)                   | +028
        move.w  0x2(a1),0x2e(a6)                | +02c
        move.w  0x4(a1),0x2a(a6)                | +032
        andi.b  #0xfe,0x3a(a6)                  | +038
        move.w  #0x173,d1                       | +03e
        jsr     0x236e.l                        | +042
        bset    #0x4,0x6b(a6)                   | +048
        lea     0x2cc686.l,a0                   | +04e
        jsr     0x28cd4.l                       | +054
        move.w  #0xd000,d0                      | +05a
        jsr     0x28134.l                       | +05e
        andi.w  #0xffe3,0x38(a6)                | +064
        ori.w   #0x1c,0x38(a6)                  | +06a
        clr.b   0x79(a6)                        | +070
        move.w  0x34(a6),d0                     | +074
        andi.w  #0xf,d0                         | +078
        cmpi.w  #0x0,d0                         | +07c
        beq.w   .L06b16c                        | +080
        cmpi.w  #0x8,d0                         | +084
        bge.w   .L06b16c                        | +088
        move.w  #0xd000,d0                      | +08c
        jsr     0x28134.l                       | +090
        andi.w  #0xffe3,0x38(a6)                | +096
        ori.w   #0x8,0x38(a6)                   | +09c
        move.b  #0x1,0x79(a6)                   | +0a2
        lea     .L06b158(pc),a1                 | +0a8
        move.l  a1,(a6)                         | +0ac
.L06b158:
        jsr     0x27bc8.l                       | +0ae
        bcc.w   .L06b168                        | +0b4
        lea     Bazooka_RocketHit_06b1d4(pc),a1 | +0b8
        move.l  a1,(a6)                         | +0bc
.L06b168:
        bra.w   .L06b182                        | +0be
.L06b16c:
        lea     .L06b172(pc),a1                 | +0c2
        move.l  a1,(a6)                         | +0c6
.L06b172:
        jsr     0x27d50.l                       | +0c8
        bcc.w   .L06b182                        | +0ce
        lea     Bazooka_RocketHit_06b1d4(pc),a1 | +0d2
        move.l  a1,(a6)                         | +0d6
.L06b182:
        move.w  0x28(a6),d0                     | +0d8
        move.w  0x2a(a6),d1                     | +0dc
        asr.w   #0x4,d0                         | +0e0
        asr.w   #0x4,d1                         | +0e2
        jsr     0x5e018.l                       | +0e4
        lsr.w   #0x3,d0                         | +0ea
        move.w  d0,0x34(a6)                     | +0ec
        jsr     0x28d70.l                       | +0f0
        jsr     0x283d8.l                       | +0f6
        btst    #0x1,0x13(a6)                   | +0fc
        beq.w   .L06b1b6                        | +102
        lea     BazookaWeapon_Explode_06b014(pc),a1 | +106
        move.l  a1,(a6)                         | +10a
.L06b1b6:
        movea.l #0xffffffff,a0                  | +10c
        lea     0x2ca3d8.l,a0                   | +112
        jsr     0x5dd56.l                       | +118
        bcc.w   SetHandlerRts_06b1d2            | +11e

| ----------------------------------------------------------------------------
|  Bazooka_RocketHit_06b1d4  @ $06B1D4  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_RocketHit_06b1d4, "ax", @progbits
        .global Bazooka_RocketHit_06b1d4
Bazooka_RocketHit_06b1d4:
        move.w  #0x1025,d0                      | +000
        jsr     0x2352.l                        | +004
        bra.w   BazookaWeapon_Explode_06b014    | +00a

| ----------------------------------------------------------------------------
|  Bazooka_OffWorldOrLeft_06b1e2  @ $06B1E2  (32 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_OffWorldOrLeft_06b1e2, "ax", @progbits
        .global Bazooka_OffWorldOrLeft_06b1e2
Bazooka_OffWorldOrLeft_06b1e2:
        movea.l #0xffffffff,a0                  | +000
        lea     0x2ca3d0.l,a0                   | +006
        jsr     0x5dd5c.l                       | +00c
        bcs.w   SetXN_06b208                    | +012
        cmpi.w  #0xff80,0x22(a6)                | +016
        blt.w   SetXN_06b208                    | +01c

| ----------------------------------------------------------------------------
|  Bazooka_ScrollGround_06b20e  @ $06B20E  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_ScrollGround_06b20e, "ax", @progbits
        .global Bazooka_ScrollGround_06b20e
Bazooka_ScrollGround_06b20e:
        jsr     0x2783a.l                       | +000
        jsr     0x27eba.l                       | +006
        bcc.w   .L06b230                        | +00c
        jsr     0x27c8c.l                       | +010
        bcc.w   .L06b230                        | +016
        clr.w   0x2a(a6)                        | +01a
        clr.w   0x2e(a6)                        | +01e
.L06b230:
        rts                                     | +022

| ----------------------------------------------------------------------------
|  Bazooka_ScrollGroundIfBit6_06b232  @ $06B232  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_ScrollGroundIfBit6_06b232, "ax", @progbits
        .global Bazooka_ScrollGroundIfBit6_06b232
Bazooka_ScrollGroundIfBit6_06b232:
        btst    #0x6,0x13(a6)                   | +000
        beq.w   Bazooka_DispatchScrollGround_06b242 | +006

| ----------------------------------------------------------------------------
|  Bazooka_DispatchScrollGround_06b242  @ $06B242  (48 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_DispatchScrollGround_06b242, "ax", @progbits
        .global Bazooka_DispatchScrollGround_06b242
Bazooka_DispatchScrollGround_06b242:
        jsr     0x28998.l                       | +000
        jsr     0x2783a.l                       | +006
        jsr     0x27eba.l                       | +00c
        bcc.w   JsrAbsThunk_06b272              | +012
        jsr     0x27c8c.l                       | +016
        bcc.w   JsrAbsRts_06b278                | +01c
        clr.w   0x2a(a6)                        | +020
        clr.w   0x2e(a6)                        | +024
        andi.b  #0xee,ccr                       | +028
        bra.w   JsrAbsRts_06b278                | +02c

| ----------------------------------------------------------------------------
|  Bazooka_FaceTarget_06b27a  @ $06B27A  (86 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_FaceTarget_06b27a, "ax", @progbits
        .global Bazooka_FaceTarget_06b27a
Bazooka_FaceTarget_06b27a:
        jsr     Bazooka_WallAhead_06b336(pc)    | +000
        bcc.w   .L06b28c                        | +004
        eori.b  #0x1,0x75(a6)                   | +008
        bra.w   SetXN_06b2d6                    | +00e
.L06b28c:
        cmpi.w  #0x30,0x22(a6)                  | +012
        bgt.w   .L06b2aa                        | +018
        btst    #0x0,0x75(a6)                   | +01c
        bne.w   .L06b2aa                        | +022
        ori.b   #0x1,0x75(a6)                   | +026
        bra.w   SetXN_06b2d6                    | +02c
.L06b2aa:
        cmpi.w  #0x110,0x22(a6)                 | +030
        blt.w   .L06b2c8                        | +036
        btst    #0x0,0x75(a6)                   | +03a
        beq.w   .L06b2c8                        | +040
        andi.b  #0xfe,0x75(a6)                  | +044
        bra.w   SetXN_06b2d6                    | +04a
.L06b2c8:
        jsr     Bazooka_TargetBehind_06b2dc(pc) | +04e
        bcs.w   SetXN_06b2d6                    | +052

| ----------------------------------------------------------------------------
|  Bazooka_TargetBehind_06b2dc  @ $06B2DC  (78 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_TargetBehind_06b2dc, "ax", @progbits
        .global Bazooka_TargetBehind_06b2dc
Bazooka_TargetBehind_06b2dc:
        tst.b   0x9e(a6)                        | +000
        beq.w   ClearXN_06b32a                  | +004
        move.l  0x90(a6),d0                     | +008
        cmpi.l  #0xffffffff,d0                  | +00c
        beq.w   ClearXN_06b32a                  | +012
        movea.l d0,a0                           | +016
        move.w  0x22(a0),d0                     | +018
        btst    #0x0,0x75(a6)                   | +01c
        bne.w   .L06b316                        | +022
        addq.w  #0x8,d0                         | +026
        cmp.w   0x22(a6),d0                     | +028
        blt.w   ClearXN_06b32a                  | +02c
        eori.b  #0x1,0x75(a6)                   | +030
        bra.w   SetXN_06b330                    | +036
.L06b316:
        subq.w  #0x8,d0                         | +03a
        cmp.w   0x22(a6),d0                     | +03c
        bgt.w   ClearXN_06b32a                  | +040
        eori.b  #0x1,0x75(a6)                   | +044
        bra.w   SetXN_06b330                    | +04a

| ----------------------------------------------------------------------------
|  Bazooka_WallAhead_06b336  @ $06B336  (38 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_WallAhead_06b336, "ax", @progbits
        .global Bazooka_WallAhead_06b336
Bazooka_WallAhead_06b336:
        cmpi.w  #0x0,0x28(a6)                   | +000
        beq.w   ClearXN_06b35c                  | +006
        bgt.w   .L06b352                        | +00a
        btst    #0x1,0x69(a6)                   | +00e
        bne.w   SetXN_06b362                    | +014
        bra.w   ClearXN_06b35c                  | +018
.L06b352:
        btst    #0x0,0x69(a6)                   | +01c
        bne.w   SetXN_06b362                    | +022

| ----------------------------------------------------------------------------
|  Bazooka_FacingMismatch_06b368  @ $06B368  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_FacingMismatch_06b368, "ax", @progbits
        .global Bazooka_FacingMismatch_06b368
Bazooka_FacingMismatch_06b368:
        move.b  0x75(a6),d0                     | +000
        move.b  0x3a(a6),d1                     | +004
        andi.b  #0x1,d0                         | +008
        andi.b  #0x1,d1                         | +00c
        cmp.b   d0,d1                           | +010
        bne.w   SetXN_06b384                    | +012

| ----------------------------------------------------------------------------
|  Bazooka_PickTargetEvery4_06b38a  @ $06B38A  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_PickTargetEvery4_06b38a, "ax", @progbits
        .global Bazooka_PickTargetEvery4_06b38a
Bazooka_PickTargetEvery4_06b38a:
        addq.b  #0x1,0x7b(a6)                   | +000
        andi.b  #0x3,0x7b(a6)                   | +004
        bne.w   .L06b3a2                        | +00a
        jsr     0x5e0d4.l                       | +00e
        move.l  a0,0x90(a6)                     | +014
.L06b3a2:
        rts                                     | +018

| ----------------------------------------------------------------------------
|  Bazooka_AcquireTarget_06b3a4  @ $06B3A4  (170 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_AcquireTarget_06b3a4, "ax", @progbits
        .global Bazooka_AcquireTarget_06b3a4
Bazooka_AcquireTarget_06b3a4:
        move.l  0x90(a6),d0                     | +000
        cmpi.l  #0xffffffff,d0                  | +004
        beq.w   .L06b3e0                        | +00a
        clr.w   d0                              | +00e
        jsr     0x5e3a2.l                       | +010
        bcc.w   .L06b3c8                        | +016
        move.l  a0,d0                           | +01a
        cmp.l   0x90(a6),d0                     | +01c
        beq.w   ClearXN_06b454                  | +020
.L06b3c8:
        move.w  #0x1,d0                         | +024
        jsr     0x5e3a2.l                       | +028
        bcc.w   .L06b3e0                        | +02e
        move.l  a0,d0                           | +032
        cmp.l   0x90(a6),d0                     | +034
        beq.w   ClearXN_06b454                  | +038
.L06b3e0:
        jsr     0x5e0d4.l                       | +03c
        move.l  a0,0x90(a6)                     | +042
        clr.w   d0                              | +046
        jsr     0x5e3a2.l                       | +048
        bcc.w   .L06b426                        | +04e
        jsr     Bazooka_TargetWithin32X_06b45a(pc) | +052
        bcc.w   .L06b426                        | +056
        move.l  a0,0x90(a6)                     | +05a
        move.w  #0x1,d0                         | +05e
        jsr     0x5e3a2.l                       | +062
        bcc.w   SetXN_06b44e                    | +068
        jsr     Bazooka_TargetWithin32X_06b45a(pc) | +06c
        bcc.w   SetXN_06b44e                    | +070
        jsr     0x5e0d4.l                       | +074
        move.l  a0,0x90(a6)                     | +07a
        bra.w   SetXN_06b44e                    | +07e
.L06b426:
        move.w  #0x1,d0                         | +082
        jsr     0x5e3a2.l                       | +086
        bcc.w   .L06b444                        | +08c
        jsr     Bazooka_TargetWithin32X_06b45a(pc) | +090
        bcc.w   .L06b444                        | +094
        move.l  a0,0x90(a6)                     | +098
        bra.w   SetXN_06b44e                    | +09c
.L06b444:
        jsr     0x5e0d4.l                       | +0a0
        move.l  a0,0x90(a6)                     | +0a6

| ----------------------------------------------------------------------------
|  Bazooka_TargetWithin32X_06b45a  @ $06B45A  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_TargetWithin32X_06b45a, "ax", @progbits
        .global Bazooka_TargetWithin32X_06b45a
Bazooka_TargetWithin32X_06b45a:
        move.w  0x22(a6),d0                     | +000
        sub.w   0x22(a0),d0                     | +004
        cmpi.w  #0xffe0,d0                      | +008
        blt.w   ClearXN_06b478                  | +00c
        cmpi.w  #0x20,d0                        | +010
        bgt.w   ClearXN_06b478                  | +014

| ----------------------------------------------------------------------------
|  Bazooka_AttackTimerInRange_06b47e  @ $06B47E  (104 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_AttackTimerInRange_06b47e, "ax", @progbits
        .global Bazooka_AttackTimerInRange_06b47e
Bazooka_AttackTimerInRange_06b47e:
        subq.w  #0x1,0x70(a6)                   | +000
        cmpi.w  #0x0,0x70(a6)                   | +004
        bgt.w   ClearXN_06b4ec                  | +00a
        jsr     0x5e0d4.l                       | +00e
        move.w  0x24(a0),d0                     | +014
        sub.w   0x24(a6),d0                     | +018
        cmpi.w  #0x0,d0                         | +01c
        bgt.w   .L06b4a4                        | +020
        neg.w   d0                              | +024
.L06b4a4:
        cmpi.w  #0x80,d0                        | +026
        ble.w   .L06b4b0                        | +02a
        move.w  #0x80,d0                        | +02e
.L06b4b0:
        lsr.w   #0x5,d0                         | +032
        lea     0x2ca44c.l,a1                   | +034
        move.b  0x98(a6),d1                     | +03a
        andi.w  #0x3,d1                         | +03e
        lsl.w   #0x2,d1                         | +042
        movea.l (a1,d1.w),a2                    | +044
        move.b  (a2,d0.w),d1                    | +048
        andi.w  #0xff,d1                        | +04c
        move.w  d1,d2                           | +050
        neg.w   d1                              | +052
        move.w  0x22(a0),d0                     | +054
        sub.w   0x22(a6),d0                     | +058
        cmp.w   d1,d0                           | +05c
        blt.w   ClearXN_06b4ec                  | +05e
        cmp.w   d2,d0                           | +062
        bgt.w   ClearXN_06b4ec                  | +064

| ----------------------------------------------------------------------------
|  Bazooka_TargetOverhead_06b4f2  @ $06B4F2  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_TargetOverhead_06b4f2, "ax", @progbits
        .global Bazooka_TargetOverhead_06b4f2
Bazooka_TargetOverhead_06b4f2:
        cmpi.w  #0x0,0x70(a6)                   | +000
        bgt.w   ClearXN_06b534                  | +006
        tst.b   0x9e(a6)                        | +00a
        beq.w   SetXN_06b52e                    | +00e
        move.l  0x90(a6),d0                     | +012
        cmpi.l  #0xffffffff,d0                  | +016
        beq.w   ClearXN_06b534                  | +01c
        movea.l d0,a0                           | +020
        move.w  0x22(a0),d0                     | +022
        move.w  d0,d1                           | +026
        subq.w  #0x8,d0                         | +028
        addq.w  #0x8,d1                         | +02a
        cmp.w   0x22(a6),d0                     | +02c
        bgt.w   ClearXN_06b534                  | +030
        cmp.w   0x22(a6),d1                     | +034
        blt.w   ClearXN_06b534                  | +038

| ----------------------------------------------------------------------------
|  Bazooka_TimerExpired_06b53a  @ $06B53A  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_TimerExpired_06b53a, "ax", @progbits
        .global Bazooka_TimerExpired_06b53a
Bazooka_TimerExpired_06b53a:
        cmpi.w  #0x0,0x70(a6)                   | +000
        bgt.w   ClearXN_06b54a                  | +006

| ----------------------------------------------------------------------------
|  BazookaWeapon_FollowParent_06b550  @ $06B550  (84 B)
| ----------------------------------------------------------------------------
        .section .text.BazookaWeapon_FollowParent_06b550, "ax", @progbits
        .global BazookaWeapon_FollowParent_06b550
BazookaWeapon_FollowParent_06b550:
        jsr     0x5e506.l                       | +000
        move.b  0x76(a0),d0                     | +006
        ext.w   d0                              | +00a
        btst    #0x0,0x3a(a6)                   | +00c
        beq.w   .L06b568                        | +012
        neg.w   d0                              | +016
.L06b568:
        add.w   d0,0x22(a6)                     | +018
        move.b  0x77(a0),d0                     | +01c
        ext.w   d0                              | +020
        add.w   d0,0x24(a6)                     | +022
        move.w  0x34(a6),d0                     | +026
        andi.w  #0xf,d0                         | +02a
        beq.w   .L06b58a                        | +02e
        cmpi.w  #0x8,d0                         | +032
        bcs.w   .L06b58e                        | +036
.L06b58a:
        subq.w  #0x1,0x38(a6)                   | +03a
.L06b58e:
        movea.l 0xc(a6),a0                      | +03e
        btst    #0x7,0x5a(a0)                   | +042
        beq.w   .L06b5a2                        | +048
        bset    #0x0,0x5a(a6)                   | +04c
.L06b5a2:
        rts                                     | +052

| ----------------------------------------------------------------------------
|  Bazooka_RangeClass_06b5a4  @ $06B5A4  (70 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_RangeClass_06b5a4, "ax", @progbits
        .global Bazooka_RangeClass_06b5a4
Bazooka_RangeClass_06b5a4:
        tst.b   0x9f(a6)                        | +000
        beq.w   .L06b5e8                        | +004
        movea.l 0x90(a6),a0                     | +008
        move.w  0x22(a6),d0                     | +00c
        move.w  0x24(a6),d1                     | +010
        move.w  0x22(a0),d2                     | +014
        move.w  0x24(a0),d3                     | +018
        jsr     0x5e23a.l                       | +01c
        clr.b   d1                              | +022
        cmpi.w  #0x70,d0                        | +024
        blt.w   .L06b5e0                        | +028
        move.b  #0x1,d1                         | +02c
        cmpi.w  #0xb0,d0                        | +030
        blt.w   .L06b5e0                        | +034
        move.b  #0x2,d1                         | +038
.L06b5e0:
        andi.b  #0x3,d1                         | +03c
        move.b  d1,0x98(a6)                     | +040
.L06b5e8:
        rts                                     | +044

| ----------------------------------------------------------------------------
|  Bazooka_AimStep_06b5ea  @ $06B5EA  (130 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_AimStep_06b5ea, "ax", @progbits
        .global Bazooka_AimStep_06b5ea
Bazooka_AimStep_06b5ea:
        movea.l 0x90(a6),a0                     | +000
        lea     0x2ca5f0.l,a1                   | +004
        move.w  0x24(a0),d0                     | +00a
        cmp.w   0x24(a6),d0                     | +00e
        bgt.w   .L06b606                        | +012
        lea     0x2ca612.l,a1                   | +016
.L06b606:
        move.w  0x22(a0),d0                     | +01c
        sub.w   0x22(a6),d0                     | +020
        cmpi.w  #0xff00,d0                      | +024
        bgt.w   .L06b61a                        | +028
        move.w  #0xff00,d0                      | +02c
.L06b61a:
        cmpi.w  #0x100,d0                       | +030
        blt.w   .L06b626                        | +034
        move.w  #0x100,d0                       | +038
.L06b626:
        addi.w  #0x100,d0                       | +03c
        lsr.w   #0x4,d0                         | +040
        move.b  (a1,d0.w),d0                    | +042
        btst    #0x0,0x3a(a6)                   | +046
        beq.w   .L06b644                        | +04c
        lea     0x2ca634.l,a1                   | +050
        move.b  (a1,d0.w),d0                    | +056
.L06b644:
        jsr     Bazooka_AimClampNear_06b678(pc) | +05a
        andi.w  #0xf,d0                         | +05e
        cmp.w   0x34(a6),d0                     | +062
        beq.w   SetXN_06b672                    | +066
        sub.w   0x34(a6),d0                     | +06a
        move.w  #0x1,d1                         | +06e
        cmpi.w  #0x0,d0                         | +072
        bgt.w   .L06b668                        | +076
        move.w  #0xffff,d1                      | +07a
.L06b668:
        add.w   d1,0x34(a6)                     | +07e

| ----------------------------------------------------------------------------
|  Bazooka_AimClampNear_06b678  @ $06B678  (68 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_AimClampNear_06b678, "ax", @progbits
        .global Bazooka_AimClampNear_06b678
Bazooka_AimClampNear_06b678:
        move.b  d0,0x5c(a6)                     | +000
        movea.l 0x90(a6),a0                     | +004
        move.w  0x24(a0),d0                     | +008
        sub.w   0x24(a6),d0                     | +00c
        cmpi.w  #0xfff8,d0                      | +010
        blt.w   Bazooka_AimRestore_06b6c2       | +014
        cmpi.w  #0x28,d0                        | +018
        bgt.w   Bazooka_AimRestore_06b6c2       | +01c
        move.w  0x22(a0),d0                     | +020
        sub.w   0x22(a6),d0                     | +024
        btst    #0x0,0x3a(a6)                   | +028
        beq.w   .L06b6ac                        | +02e
        neg.w   d0                              | +032
.L06b6ac:
        clr.b   d1                              | +034
        cmpi.w  #0x0,d0                         | +036
        bgt.w   .L06b6ba                        | +03a
        move.b  #0x8,d1                         | +03e
.L06b6ba:
        move.b  d1,d0                           | +042

| ----------------------------------------------------------------------------
|  Bazooka_AimRestore_06b6c2  @ $06B6C2  (4 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_AimRestore_06b6c2, "ax", @progbits
        .global Bazooka_AimRestore_06b6c2
Bazooka_AimRestore_06b6c2:
        move.b  0x5c(a6),d0                     | +000

| ----------------------------------------------------------------------------
|  Bazooka_AimDone_06b6cc  @ $06B6CC  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_AimDone_06b6cc, "ax", @progbits
        .global Bazooka_AimDone_06b6cc
Bazooka_AimDone_06b6cc:
        tst.b   0x9e(a6)                        | +000
        beq.w   ClearXN_06b6ec                  | +004
        tst.w   0x34(a6)                        | +008
        beq.w   ClearXN_06b6ec                  | +00c
        cmpi.w  #0xd,0x34(a6)                   | +010
        bcc.w   ClearXN_06b6ec                  | +016

| ----------------------------------------------------------------------------
|  Bazooka_Fire_06b6f2  @ $06B6F2  (152 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_Fire_06b6f2, "ax", @progbits
        .global Bazooka_Fire_06b6f2
Bazooka_Fire_06b6f2:
        move.w  #0x1064,d0                      | +000
        jsr     0x2352.l                        | +004
        move.w  0x34(a6),d0                     | +00a
        btst    #0x0,0x3a(a6)                   | +00e
        beq.w   .L06b714                        | +014
        lea     0x2ca634.l,a1                   | +018
        move.b  (a1,d0.w),d0                    | +01e
.L06b714:
        andi.w  #0xf,d0                         | +022
        move.w  d0,0x5c(a6)                     | +026
        lea     Bazooka_MuzzleFlash_06b038(pc),a1 | +02a
        jsr     0x4ae.l                         | +02e
        jsr     0x5dd02.l                       | +034
        lea     0x2ca470.l,a1                   | +03a
        move.w  0x34(a6),d0                     | +040
        move.w  d0,0x34(a0)                     | +044
        lsl.w   #0x2,d0                         | +048
        move.w  (a1,d0.w),d1                    | +04a
        move.w  0x2(a1,d0.w),d2                 | +04e
        btst    #0x0,0x3a(a6)                   | +052
        beq.w   .L06b750                        | +058
        neg.w   d1                              | +05c
.L06b750:
        move.w  d1,d3                           | +05e
        move.w  d2,d4                           | +060
        move.b  d3,0x76(a0)                     | +062
        move.b  d4,0x77(a0)                     | +066
        movem.w d1-d2,-(a7)                     | +06a
        lea     Bazooka_Rocket_06b0aa(pc),a1    | +06e
        jsr     0x4ae.l                         | +072
        jsr     0x5dd02.l                       | +078
        movem.w (a7)+,d1-d2                     | +07e
        move.w  0x5c(a6),0x34(a0)               | +082
        add.w   d1,0x22(a0)                     | +088
        add.w   d2,0x24(a0)                     | +08c
        move.b  0x98(a6),0x98(a0)               | +090
        rts                                     | +096

| ----------------------------------------------------------------------------
|  Bazooka_SpawnWeaponGround_06b78a  @ $06B78A  (134 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_SpawnWeaponGround_06b78a, "ax", @progbits
        .global Bazooka_SpawnWeaponGround_06b78a
Bazooka_SpawnWeaponGround_06b78a:
        jsr     0x5e7c0.l                       | +000
        move.b  0x99(a6),d0                     | +006
        andi.b  #0x1,d0                         | +00a
        move.b  d0,0x3a(a6)                     | +00e
        move.b  d0,0x75(a6)                     | +012
        move.b  0x9a(a6),0x7a(a6)               | +016
        jsr     Bazooka_PlayCry_06b94c(pc)      | +01c
        move.w  #0x8000,d0                      | +020
        jsr     0x28134.l                       | +024
        andi.w  #0xffe3,0x38(a6)                | +02a
        ori.w   #0x14,0x38(a6)                  | +030
        move.l  #0x2ca198,0x60(a6)              | +036
        move.l  #0x2ca1e4,0x48(a6)              | +03e
        lea     BazookaWeapon_Child_06acb2(pc),a1 | +046
        jsr     0x4ae.l                         | +04a
        jsr     0x5dd02.l                       | +050
        clr.b   0x9e(a0)                        | +056
        move.b  0x9f(a6),0x9f(a0)               | +05a
        move.b  0x98(a6),0x98(a0)               | +060
        move.b  0x9a(a6),0x7a(a0)               | +066
        move.b  0x9c(a6),0x9c(a0)               | +06c
        move.b  0x9d(a6),0x9d(a0)               | +072
        move.b  0x9e(a6),0x9e(a0)               | +078
        bset    #0x2,0x6b(a6)                   | +07e
        rts                                     | +084

| ----------------------------------------------------------------------------
|  Bazooka_SpawnWeaponTable_06b810  @ $06B810  (106 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_SpawnWeaponTable_06b810, "ax", @progbits
        .global Bazooka_SpawnWeaponTable_06b810
Bazooka_SpawnWeaponTable_06b810:
        jsr     Bazooka_PlayCry_06b94c(pc)      | +000
        move.w  #0x8000,d0                      | +004
        jsr     0x28134.l                       | +008
        andi.w  #0xffe3,0x38(a6)                | +00e
        ori.w   #0x8,0x38(a6)                   | +014
        lea     0x2bd3e6.l,a0                   | +01a
        jsr     0x799de.l                       | +020
        move.w  d0,0x66(a6)                     | +026
        move.l  #0x2ca1e4,0x48(a6)              | +02a
        move.b  0x9d(a6),0x75(a6)               | +032
        lea     BazookaWeapon_Child_06acb2(pc),a1 | +038
        jsr     0x4ae.l                         | +03c
        jsr     0x5dd02.l                       | +042
        clr.b   0x98(a0)                        | +048
        clr.b   0x9f(a0)                        | +04c
        clr.b   0x9c(a0)                        | +050
        clr.b   0x9d(a0)                        | +054
        clr.b   0x9e(a0)                        | +058
        move.b  0x9e(a6),0x9e(a0)               | +05c
        move.b  0x7a(a6),0x7a(a0)               | +062
        rts                                     | +068

| ----------------------------------------------------------------------------
|  Bazooka_FollowParentOffs_06b87a  @ $06B87A  (66 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_FollowParentOffs_06b87a, "ax", @progbits
        .global Bazooka_FollowParentOffs_06b87a
Bazooka_FollowParentOffs_06b87a:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x22(a0),d0                     | +004
        move.w  0x24(a0),d1                     | +008
        move.w  0x38(a0),d2                     | +00c
        move.b  0x9a(a6),d3                     | +010
        move.b  0x9b(a6),d4                     | +014
        move.b  0x9c(a6),d5                     | +018
        ext.w   d3                              | +01c
        ext.w   d4                              | +01e
        ext.w   d5                              | +020
        btst    #0x0,0x3a(a0)                   | +022
        bne.w   .L06b8a8                        | +028
        neg.w   d3                              | +02c
.L06b8a8:
        add.w   d3,d0                           | +02e
        add.w   d4,d1                           | +030
        add.w   d5,d2                           | +032
        move.w  d0,0x22(a6)                     | +034
        move.w  d1,0x24(a6)                     | +038
        move.w  d2,0x38(a6)                     | +03c
        rts                                     | +040

| ----------------------------------------------------------------------------
|  Bazooka_PastTargetX_06b8bc  @ $06B8BC  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_PastTargetX_06b8bc, "ax", @progbits
        .global Bazooka_PastTargetX_06b8bc
Bazooka_PastTargetX_06b8bc:
        move.w  0x22(a6),d0                     | +000
        move.w  0x98(a6),d1                     | +004
        btst    #0x0,0x75(a6)                   | +008
        beq.w   .L06b8d8                        | +00e
        cmp.w   d0,d1                           | +012
        bgt.w   ClearXN_06b8e4                  | +014
        bra.w   SetXN_06b8de                    | +018
.L06b8d8:
        cmp.w   d0,d1                           | +01c
        blt.w   ClearXN_06b8e4                  | +01e

| ----------------------------------------------------------------------------
|  BazookaWeapon_StateLookup_06b8ea  @ $06B8EA  (82 B)
| ----------------------------------------------------------------------------
        .section .text.BazookaWeapon_StateLookup_06b8ea, "ax", @progbits
        .global BazookaWeapon_StateLookup_06b8ea
BazookaWeapon_StateLookup_06b8ea:
        lea     0x2ca644.l,a1                   | +000
        tst.b   0x9d(a6)                        | +006
        beq.w   .L06b8fe                        | +00a
        lea     0x2ca664.l,a1                   | +00e
.L06b8fe:
        movea.l 0xc(a6),a0                      | +014
        move.b  0x72(a0),d0                     | +018
        cmp.b   0x72(a6),d0                     | +01c
        beq.w   BazookaWeapon_SetState_06b942   | +020
        move.b  d0,0x72(a6)                     | +024
        cmpi.b  #0x8,d0                         | +028
        bcs.w   .L06b926                        | +02c
        nop                                     | +030
        nop                                     | +032
        cmpi.b  #0x8,d0                         | +034
        nop                                     | +038
        trap    #0xf                            | +03a
.L06b926:
        andi.w  #0x7,d0                         | +03c
        lsl.w   #0x2,d0                         | +040
        move.l  (a1,d0.w),d0                    | +042
        cmpi.l  #0xffffffff,d0                  | +046
        beq.w   ClearXN_06b946                  | +04c
        movea.l d0,a1                           | +050

| ----------------------------------------------------------------------------
|  BazookaWeapon_SetState_06b942  @ $06B942  (4 B)
| ----------------------------------------------------------------------------
        .section .text.BazookaWeapon_SetState_06b942, "ax", @progbits
        .global BazookaWeapon_SetState_06b942
BazookaWeapon_SetState_06b942:
        move.b  d0,0x72(a6)                     | +000

| ----------------------------------------------------------------------------
|  Bazooka_PlayCry_06b94c  @ $06B94C  (32 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_PlayCry_06b94c, "ax", @progbits
        .global Bazooka_PlayCry_06b94c
Bazooka_PlayCry_06b94c:
        lea     0x2ca404.l,a1                   | +000
        move.b  0x7a(a6),d0                     | +006
        andi.w  #0x3,d0                         | +00a
        lsl.w   #0x1,d0                         | +00e
        move.w  (a1,d0.w),d1                    | +010
        jsr     0x236e.l                        | +014
        move.w  #0x7,0x1c(a6)                   | +01a

| ----------------------------------------------------------------------------
|  Bazooka_DeathDebris_06b974  @ $06B974  (48 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_DeathDebris_06b974, "ax", @progbits
        .global Bazooka_DeathDebris_06b974
Bazooka_DeathDebris_06b974:
        move.w  #0x1033,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x2ca3e0.l,a1                   | +00a
        jsr     0x77c7e.l                       | +010
        move.w  #0x4000,0x38(a0)                | +016
        lea     0x2ca3f2.l,a1                   | +01c
        jsr     0x77c7e.l                       | +022
        move.w  #0x4000,0x38(a0)                | +028
        rts                                     | +02e

| ----------------------------------------------------------------------------
|  Bazooka_DeathExplodeA_06b9a4  @ $06B9A4  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_DeathExplodeA_06b9a4, "ax", @progbits
        .global Bazooka_DeathExplodeA_06b9a4
Bazooka_DeathExplodeA_06b9a4:
        move.w  #0x1021,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x2ca3f2.l,a1                   | +00a
        jsr     0x77c7e.l                       | +010
        move.w  #0x4000,0x38(a0)                | +016
        lea     0x77fd6.l,a1                    | +01c
        jsr     0x4ae.l                         | +022
        jsr     0x5dd02.l                       | +028
        move.w  #0x4000,0x38(a0)                | +02e
        rts                                     | +034

| ----------------------------------------------------------------------------
|  Bazooka_DeathExplodeB_06b9da  @ $06B9DA  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_DeathExplodeB_06b9da, "ax", @progbits
        .global Bazooka_DeathExplodeB_06b9da
Bazooka_DeathExplodeB_06b9da:
        move.w  #0x1021,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x2ca3f2.l,a1                   | +00a
        jsr     0x77c7e.l                       | +010
        move.w  #0x4000,0x38(a0)                | +016
        lea     0x77fd6.l,a1                    | +01c
        jsr     0x4ae.l                         | +022
        jsr     0x5dd02.l                       | +028
        move.w  #0x4000,0x38(a0)                | +02e
        rts                                     | +034

| ----------------------------------------------------------------------------
|  Bazooka_CmpSlotWithParent_06ba10  @ $06BA10  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Bazooka_CmpSlotWithParent_06ba10, "ax", @progbits
        .global Bazooka_CmpSlotWithParent_06ba10
Bazooka_CmpSlotWithParent_06ba10:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_06ba26                    | +00c

| ----------------------------------------------------------------------------
|  RocketVehicle_Tmpl59_06ba2c  @ $06BA2C  (214 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_Tmpl59_06ba2c, "ax", @progbits
        .global RocketVehicle_Tmpl59_06ba2c
RocketVehicle_Tmpl59_06ba2c:
        clr.b   0x9e(a6)                        | +000
        bra.w   .L06ba3a                        | +004
        move.b  #0x1,0x9e(a6)                   | +008
.L06ba3a:
        jsr     0x283ca.l                       | +00e
        jsr     0x5e7c0.l                       | +014
        move.b  0x99(a6),d0                     | +01a
        andi.b  #0x1,d0                         | +01e
        move.b  d0,0x3a(a6)                     | +022
        move.b  d0,0x8d(a6)                     | +026
        jsr     RocketVehicle_SetFlags45_06d446(pc) | +02a
        move.b  0x9a(a6),0x96(a6)               | +02e
        jsr     RocketVehicle_PlayCry_06d574(pc) | +034
        lea     0x2bd468.l,a0                   | +038
        jsr     0x799de.l                       | +03e
        move.w  d0,0x36(a6)                     | +044
        lea     0x2bd4ea.l,a0                   | +048
        jsr     0x799de.l                       | +04e
        move.w  d0,0x70(a6)                     | +054
        lea     0x2bd3e6.l,a0                   | +058
        jsr     0x799de.l                       | +05e
        move.w  d0,0x66(a6)                     | +064
        move.w  #0x8000,d0                      | +068
        jsr     0x28134.l                       | +06c
        andi.w  #0xffe3,0x38(a6)                | +072
        ori.w   #0x14,0x38(a6)                  | +078
        move.l  #0x2cc7a6,0x60(a6)              | +07e
        move.l  #0x2cc8d6,0x48(a6)              | +086
        clr.b   0x89(a6)                        | +08e
        move.w  #0x8,0x8a(a6)                   | +092
        move.b  #0xff,0x97(a6)                  | +098
        move.w  0x24(a6),0x82(a6)               | +09e
        jsr     0x30696.l                       | +0a4
        lea     RocketVehicle_Rider_06c562(pc),a1 | +0aa
        jsr     0x4ae.l                         | +0ae
        jsr     0x5dd02.l                       | +0b4
        move.b  0x9f(a6),0x9f(a0)               | +0ba
        move.b  0x98(a6),0x98(a0)               | +0c0
        move.b  0x9a(a6),0x96(a0)               | +0c6
        bset    #0x2,0x6b(a6)                   | +0cc
        bra.w   RocketVehicle_Idle_06bee0       | +0d2

| ----------------------------------------------------------------------------
|  RocketVehicle_Drive_06bb02  @ $06BB02  (86 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_Drive_06bb02, "ax", @progbits
        .global RocketVehicle_Drive_06bb02
RocketVehicle_Drive_06bb02:
        move.b  #0x0,0x72(a6)                   | +000
        move.w  0x36(a6),d0                     | +006
        btst    #0x0,0x8d(a6)                   | +00a
        bne.w   .L06bb18                        | +010
        neg.w   d0                              | +014
.L06bb18:
        move.w  d0,0x28(a6)                     | +016
        jsr     RocketVehicle_FacingMismatch_06d206(pc) | +01a
        bcs.w   RocketVehicle_SpritesA_06bb58__L06bb6c | +01e
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +022
        bcs.w   RocketVehicle_Die_06c272        | +026
        move.b  d0,0x97(a6)                     | +02a
        andi.w  #0x3,d0                         | +02e
        movea.l #0x6bb58,a0                     | +032
        lsl.w   #0x2,d0                         | +038
        movea.l (a0,d0.w),a0                    | +03a
        cmpa.l  #0xffffffff,a0                  | +03e
        beq.w   .L06bb50                        | +044
        jsr     0x28cd4.l                       | +048
.L06bb50:
        jsr     RocketVehicle_Fire_06d484(pc)   | +04e
        bra.w   RocketVehicle_SpritesA_06bb58__L06bb68 | +052

| ----------------------------------------------------------------------------
|  RocketVehicle_SpritesA_06bb58  @ $06BB58  (72 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_SpritesA_06bb58, "ax", @progbits
        .global RocketVehicle_SpritesA_06bb58
RocketVehicle_SpritesA_06bb58:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xda24                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xd8b0                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xd96a                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global RocketVehicle_SpritesA_06bb58__L06bb68
RocketVehicle_SpritesA_06bb58__L06bb68:
.L06bb68:
        bra.w   RocketVehicle_SpritesB_06bba0__L06bbb0 | +010
        .global RocketVehicle_SpritesA_06bb58__L06bb6c
RocketVehicle_SpritesA_06bb58__L06bb6c:
.L06bb6c:
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +014
        bcs.w   RocketVehicle_Die_06c272        | +018
        move.b  d0,0x97(a6)                     | +01c
        andi.w  #0x3,d0                         | +020
        movea.l #0x6bba0,a0                     | +024
        lsl.w   #0x2,d0                         | +02a
        movea.l (a0,d0.w),a0                    | +02c
        cmpa.l  #0xffffffff,a0                  | +030
        beq.w   .L06bb98                        | +036
        jsr     0x28cd4.l                       | +03a
.L06bb98:
        jsr     RocketVehicle_Fire_06d484(pc)   | +040
        bra.w   RocketVehicle_SpritesB_06bba0__L06bbb0 | +044

| ----------------------------------------------------------------------------
|  RocketVehicle_SpritesB_06bba0  @ $06BBA0  (124 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_SpritesB_06bba0, "ax", @progbits
        .global RocketVehicle_SpritesB_06bba0
RocketVehicle_SpritesB_06bba0:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xdc52                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xdade                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xdb98                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global RocketVehicle_SpritesB_06bba0__L06bbb0
RocketVehicle_SpritesB_06bba0__L06bbb0:
.L06bbb0:
        lea     .L06bbb6(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L06bbb6:
        jsr     0x28998.l                       | +016
        jsr     RocketVehicle_GroundPhysics_06d0f6(pc) | +01c
        bcc.w   .L06bbd0                        | +020
        eori.b  #0x1,0x8d(a6)                   | +024
        lea     RocketVehicle_Stop_06bcfc(pc),a1 | +02a
        move.l  a1,(a6)                         | +02e
.L06bbd0:
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +030
        bcs.w   RocketVehicle_Die_06c272        | +034
        cmp.b   0x97(a6),d0                     | +038
        beq.w   RocketVehicle_SpritesD_06bc64__L06bc74 | +03c
        jsr     RocketVehicle_FacingMismatch_06d206(pc) | +040
        bcs.w   RocketVehicle_SpritesC_06bc1c__L06bc30 | +044
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +048
        bcs.w   RocketVehicle_Die_06c272        | +04c
        move.b  d0,0x97(a6)                     | +050
        andi.w  #0x3,d0                         | +054
        movea.l #0x6bc1c,a0                     | +058
        lsl.w   #0x2,d0                         | +05e
        movea.l (a0,d0.w),a0                    | +060
        cmpa.l  #0xffffffff,a0                  | +064
        beq.w   .L06bc14                        | +06a
        jsr     0x28cd4.l                       | +06e
.L06bc14:
        jsr     RocketVehicle_Fire_06d484(pc)   | +074
        bra.w   RocketVehicle_SpritesC_06bc1c__L06bc2c | +078

| ----------------------------------------------------------------------------
|  RocketVehicle_SpritesC_06bc1c  @ $06BC1C  (72 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_SpritesC_06bc1c, "ax", @progbits
        .global RocketVehicle_SpritesC_06bc1c
RocketVehicle_SpritesC_06bc1c:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xda24                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xd8b0                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xd96a                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global RocketVehicle_SpritesC_06bc1c__L06bc2c
RocketVehicle_SpritesC_06bc1c__L06bc2c:
.L06bc2c:
        bra.w   RocketVehicle_SpritesD_06bc64__L06bc74 | +010
        .global RocketVehicle_SpritesC_06bc1c__L06bc30
RocketVehicle_SpritesC_06bc1c__L06bc30:
.L06bc30:
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +014
        bcs.w   RocketVehicle_Die_06c272        | +018
        move.b  d0,0x97(a6)                     | +01c
        andi.w  #0x3,d0                         | +020
        movea.l #0x6bc64,a0                     | +024
        lsl.w   #0x2,d0                         | +02a
        movea.l (a0,d0.w),a0                    | +02c
        cmpa.l  #0xffffffff,a0                  | +030
        beq.w   .L06bc5c                        | +036
        jsr     0x28cd4.l                       | +03a
.L06bc5c:
        jsr     RocketVehicle_Fire_06d484(pc)   | +040
        bra.w   RocketVehicle_SpritesD_06bc64__L06bc74 | +044

| ----------------------------------------------------------------------------
|  RocketVehicle_SpritesD_06bc64  @ $06BC64  (152 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_SpritesD_06bc64, "ax", @progbits
        .global RocketVehicle_SpritesD_06bc64
RocketVehicle_SpritesD_06bc64:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xdc52                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xdade                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xdb98                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global RocketVehicle_SpritesD_06bc64__L06bc74
RocketVehicle_SpritesD_06bc64__L06bc74:
.L06bc74:
        jsr     0x28d70.l                       | +010
        jsr     RocketVehicle_WallAhead_06d172(pc) | +016
        bcc.w   .L06bc92                        | +01a
        eori.b  #0x1,0x8d(a6)                   | +01e
        lea     RocketVehicle_Stop_06bcfc(pc),a1 | +024
        move.l  a1,(a6)                         | +028
        bra.w   .L06bcda                        | +02a
.L06bc92:
        cmpi.w  #0x30,0x22(a6)                  | +02e
        bgt.w   .L06bcb6                        | +034
        btst    #0x0,0x8d(a6)                   | +038
        bne.w   .L06bcb6                        | +03e
        ori.b   #0x1,0x8d(a6)                   | +042
        clr.w   0x28(a6)                        | +048
        lea     RocketVehicle_Stop_06bcfc(pc),a1 | +04c
        move.l  a1,(a6)                         | +050
.L06bcb6:
        cmpi.w  #0x110,0x22(a6)                 | +052
        blt.w   .L06bcda                        | +058
        btst    #0x0,0x8d(a6)                   | +05c
        beq.w   .L06bcda                        | +062
        andi.b  #0xfe,0x8d(a6)                  | +066
        clr.w   0x28(a6)                        | +06c
        lea     RocketVehicle_Stop_06bcfc(pc),a1 | +070
        move.l  a1,(a6)                         | +074
.L06bcda:
        jsr     RocketVehicle_AttackTimerInRange_06d228(pc) | +076
        bcc.w   .L06bcec                        | +07a
        clr.w   0x28(a6)                        | +07e
        lea     RocketVehicle_Stop_06bcfc(pc),a1 | +082
        move.l  a1,(a6)                         | +086
.L06bcec:
        jsr     0x283ca.l                       | +088
        jsr     0x283d8.l                       | +08e
        bra.w   RocketVehicle_Wreck_06c37c__L06c418 | +094

| ----------------------------------------------------------------------------
|  RocketVehicle_Stop_06bcfc  @ $06BCFC  (74 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_Stop_06bcfc, "ax", @progbits
        .global RocketVehicle_Stop_06bcfc
RocketVehicle_Stop_06bcfc:
        move.b  #0x0,0x72(a6)                   | +000
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +006
        bcs.w   RocketVehicle_Die_06c272        | +00a
        jsr     RocketVehicle_FacingMismatch_06d206(pc) | +00e
        bcc.w   RocketVehicle_SpritesE_06bd46__L06bd5a | +012
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +016
        bcs.w   RocketVehicle_Die_06c272        | +01a
        move.b  d0,0x97(a6)                     | +01e
        andi.w  #0x3,d0                         | +022
        movea.l #0x6bd46,a0                     | +026
        lsl.w   #0x2,d0                         | +02c
        movea.l (a0,d0.w),a0                    | +02e
        cmpa.l  #0xffffffff,a0                  | +032
        beq.w   .L06bd3e                        | +038
        jsr     0x28cd4.l                       | +03c
.L06bd3e:
        jsr     RocketVehicle_Fire_06d484(pc)   | +042
        bra.w   RocketVehicle_SpritesE_06bd46__L06bd56 | +046

| ----------------------------------------------------------------------------
|  RocketVehicle_SpritesE_06bd46  @ $06BD46  (72 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_SpritesE_06bd46, "ax", @progbits
        .global RocketVehicle_SpritesE_06bd46
RocketVehicle_SpritesE_06bd46:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xe038                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xde3c                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xdf3a                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global RocketVehicle_SpritesE_06bd46__L06bd56
RocketVehicle_SpritesE_06bd46__L06bd56:
.L06bd56:
        bra.w   RocketVehicle_SpritesF_06bd8e__L06bd9e | +010
        .global RocketVehicle_SpritesE_06bd46__L06bd5a
RocketVehicle_SpritesE_06bd46__L06bd5a:
.L06bd5a:
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +014
        bcs.w   RocketVehicle_Die_06c272        | +018
        move.b  d0,0x97(a6)                     | +01c
        andi.w  #0x3,d0                         | +020
        movea.l #0x6bd8e,a0                     | +024
        lsl.w   #0x2,d0                         | +02a
        movea.l (a0,d0.w),a0                    | +02c
        cmpa.l  #0xffffffff,a0                  | +030
        beq.w   .L06bd86                        | +036
        jsr     0x28cd4.l                       | +03a
.L06bd86:
        jsr     RocketVehicle_Fire_06d484(pc)   | +040
        bra.w   RocketVehicle_SpritesF_06bd8e__L06bd9e | +044

| ----------------------------------------------------------------------------
|  RocketVehicle_SpritesF_06bd8e  @ $06BD8E  (90 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_SpritesF_06bd8e, "ax", @progbits
        .global RocketVehicle_SpritesF_06bd8e
RocketVehicle_SpritesF_06bd8e:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xe332                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xe136                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xe234                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global RocketVehicle_SpritesF_06bd8e__L06bd9e
RocketVehicle_SpritesF_06bd8e__L06bd9e:
.L06bd9e:
        clr.w   0x28(a6)                        | +010
        lea     .L06bda8(pc),a1                 | +014
        move.l  a1,(a6)                         | +018
.L06bda8:
        jsr     0x28998.l                       | +01a
        jsr     RocketVehicle_GroundPhysics_06d0f6(pc) | +020
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +024
        bcs.w   .L06bdc2                        | +028
        cmp.b   0x97(a6),d0                     | +02c
        beq.w   .L06bdc8                        | +030
.L06bdc2:
        lea     RocketVehicle_Idle_06bee0(pc),a1 | +034
        move.l  a1,(a6)                         | +038
.L06bdc8:
        jsr     0x28d70.l                       | +03a
        bcc.w   .L06bdd8                        | +040
        lea     RocketVehicle_Idle_06bee0(pc),a1 | +044
        move.l  a1,(a6)                         | +048
.L06bdd8:
        jsr     0x283ca.l                       | +04a
        jsr     0x283d8.l                       | +050
        bra.w   RocketVehicle_Wreck_06c37c__L06c418 | +056

| ----------------------------------------------------------------------------
|  RocketVehicle_Turn_06bde8  @ $06BDE8  (74 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_Turn_06bde8, "ax", @progbits
        .global RocketVehicle_Turn_06bde8
RocketVehicle_Turn_06bde8:
        move.b  #0x0,0x72(a6)                   | +000
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +006
        bcs.w   RocketVehicle_Die_06c272        | +00a
        jsr     RocketVehicle_FacingMismatch_06d206(pc) | +00e
        bcs.w   RocketVehicle_SpritesG_06be32__L06be46 | +012
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +016
        bcs.w   RocketVehicle_Die_06c272        | +01a
        move.b  d0,0x97(a6)                     | +01e
        andi.w  #0x3,d0                         | +022
        movea.l #0x6be32,a0                     | +026
        lsl.w   #0x2,d0                         | +02c
        movea.l (a0,d0.w),a0                    | +02e
        cmpa.l  #0xffffffff,a0                  | +032
        beq.w   .L06be2a                        | +038
        jsr     0x28cd4.l                       | +03c
.L06be2a:
        jsr     RocketVehicle_Fire_06d484(pc)   | +042
        bra.w   RocketVehicle_SpritesG_06be32__L06be42 | +046

| ----------------------------------------------------------------------------
|  RocketVehicle_SpritesG_06be32  @ $06BE32  (72 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_SpritesG_06be32, "ax", @progbits
        .global RocketVehicle_SpritesG_06be32
RocketVehicle_SpritesG_06be32:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xe65c                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xe430                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xe546                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global RocketVehicle_SpritesG_06be32__L06be42
RocketVehicle_SpritesG_06be32__L06be42:
.L06be42:
        bra.w   RocketVehicle_SpritesH_06be7a__L06be8a | +010
        .global RocketVehicle_SpritesG_06be32__L06be46
RocketVehicle_SpritesG_06be32__L06be46:
.L06be46:
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +014
        bcs.w   RocketVehicle_Die_06c272        | +018
        move.b  d0,0x97(a6)                     | +01c
        andi.w  #0x3,d0                         | +020
        movea.l #0x6be7a,a0                     | +024
        lsl.w   #0x2,d0                         | +02a
        movea.l (a0,d0.w),a0                    | +02c
        cmpa.l  #0xffffffff,a0                  | +030
        beq.w   .L06be72                        | +036
        jsr     0x28cd4.l                       | +03a
.L06be72:
        jsr     RocketVehicle_Fire_06d484(pc)   | +040
        bra.w   RocketVehicle_SpritesH_06be7a__L06be8a | +044

| ----------------------------------------------------------------------------
|  RocketVehicle_SpritesH_06be7a  @ $06BE7A  (102 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_SpritesH_06be7a, "ax", @progbits
        .global RocketVehicle_SpritesH_06be7a
RocketVehicle_SpritesH_06be7a:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xe99e                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xe772                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xe888                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global RocketVehicle_SpritesH_06be7a__L06be8a
RocketVehicle_SpritesH_06be7a__L06be8a:
.L06be8a:
        lea     .L06be90(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L06be90:
        jsr     0x28998.l                       | +016
        jsr     RocketVehicle_GroundPhysics_06d0f6(pc) | +01c
        bcc.w   .L06beaa                        | +020
        eori.b  #0x1,0x8d(a6)                   | +024
        lea     RocketVehicle_Stop_06bcfc(pc),a1 | +02a
        move.l  a1,(a6)                         | +02e
.L06beaa:
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +030
        bcs.w   .L06beba                        | +034
        cmp.b   0x97(a6),d0                     | +038
        beq.w   .L06bec0                        | +03c
.L06beba:
        lea     RocketVehicle_Idle_06bee0(pc),a1 | +040
        move.l  a1,(a6)                         | +044
.L06bec0:
        jsr     0x28d70.l                       | +046
        bcc.w   .L06bed0                        | +04c
        lea     RocketVehicle_Drive_06bb02(pc),a1 | +050
        move.l  a1,(a6)                         | +054
.L06bed0:
        jsr     0x283ca.l                       | +056
        jsr     0x283d8.l                       | +05c
        bra.w   RocketVehicle_Wreck_06c37c__L06c418 | +062

| ----------------------------------------------------------------------------
|  RocketVehicle_Idle_06bee0  @ $06BEE0  (86 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_Idle_06bee0, "ax", @progbits
        .global RocketVehicle_Idle_06bee0
RocketVehicle_Idle_06bee0:
        move.b  #0x0,0x72(a6)                   | +000
        lea     0x2bd670.l,a0                   | +006
        jsr     0x799de.l                       | +00c
        move.b  d0,0x88(a6)                     | +012
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +016
        bcs.w   RocketVehicle_Die_06c272        | +01a
        clr.w   0x28(a6)                        | +01e
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +022
        bcs.w   RocketVehicle_Die_06c272        | +026
        move.b  d0,0x97(a6)                     | +02a
        andi.w  #0x3,d0                         | +02e
        movea.l #0x6bf36,a0                     | +032
        lsl.w   #0x2,d0                         | +038
        movea.l (a0,d0.w),a0                    | +03a
        cmpa.l  #0xffffffff,a0                  | +03e
        beq.w   .L06bf2e                        | +044
        jsr     0x28cd4.l                       | +048
.L06bf2e:
        jsr     RocketVehicle_Fire_06d484(pc)   | +04e
        bra.w   RocketVehicle_SpritesI_06bf36__L06bf46 | +052

| ----------------------------------------------------------------------------
|  RocketVehicle_SpritesI_06bf36  @ $06BF36  (100 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_SpritesI_06bf36, "ax", @progbits
        .global RocketVehicle_SpritesI_06bf36
RocketVehicle_SpritesI_06bf36:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xddee                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xdd52                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xdda0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global RocketVehicle_SpritesI_06bf36__L06bf46
RocketVehicle_SpritesI_06bf36__L06bf46:
.L06bf46:
        lea     .L06bf4c(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L06bf4c:
        jsr     0x28998.l                       | +016
        jsr     RocketVehicle_GroundPhysics_06d0f6(pc) | +01c
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +020
        bcs.w   .L06bf66                        | +024
        cmp.b   0x97(a6),d0                     | +028
        beq.w   RocketVehicle_SpritesJ_06bf9a__L06bfaa | +02c
.L06bf66:
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +030
        bcs.w   RocketVehicle_Die_06c272        | +034
        move.b  d0,0x97(a6)                     | +038
        andi.w  #0x3,d0                         | +03c
        movea.l #0x6bf9a,a0                     | +040
        lsl.w   #0x2,d0                         | +046
        movea.l (a0,d0.w),a0                    | +048
        cmpa.l  #0xffffffff,a0                  | +04c
        beq.w   .L06bf92                        | +052
        jsr     0x28cd4.l                       | +056
.L06bf92:
        jsr     RocketVehicle_Fire_06d484(pc)   | +05c
        bra.w   RocketVehicle_SpritesJ_06bf9a__L06bfaa | +060

| ----------------------------------------------------------------------------
|  RocketVehicle_SpritesJ_06bf9a  @ $06BF9A  (54 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_SpritesJ_06bf9a, "ax", @progbits
        .global RocketVehicle_SpritesJ_06bf9a
RocketVehicle_SpritesJ_06bf9a:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xddee                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xdd52                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xdda0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global RocketVehicle_SpritesJ_06bf9a__L06bfaa
RocketVehicle_SpritesJ_06bf9a__L06bfaa:
.L06bfaa:
        jsr     0x28d70.l                       | +010
        subq.b  #0x1,0x88(a6)                   | +016
        bgt.w   .L06bfbe                        | +01a
        lea     RocketVehicle_Turn_06bde8(pc),a1 | +01e
        move.l  a1,(a6)                         | +022
.L06bfbe:
        jsr     RocketVehicle_AttackTimerInRange_06d228(pc) | +024
        bcc.w   .L06bfcc                        | +028
        lea     RocketVehicle_Alert_06bfd0(pc),a1 | +02c
        move.l  a1,(a6)                         | +030
.L06bfcc:
        bra.w   RocketVehicle_Wreck_06c37c__L06c418 | +032

| ----------------------------------------------------------------------------
|  RocketVehicle_Alert_06bfd0  @ $06BFD0  (90 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_Alert_06bfd0, "ax", @progbits
        .global RocketVehicle_Alert_06bfd0
RocketVehicle_Alert_06bfd0:
        jsr     0x267e2.l                       | +000
        jsr     0x5e0d4.l                       | +006
        move.l  a0,0x90(a6)                     | +00c
        move.b  #0x2,0x72(a6)                   | +010
        clr.b   0x88(a6)                        | +016
        clr.b   0x73(a6)                        | +01a
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +01e
        bcs.w   RocketVehicle_Die_06c272        | +022
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +026
        bcs.w   RocketVehicle_Die_06c272        | +02a
        move.b  d0,0x97(a6)                     | +02e
        andi.w  #0x3,d0                         | +032
        movea.l #0x6c02a,a0                     | +036
        lsl.w   #0x2,d0                         | +03c
        movea.l (a0,d0.w),a0                    | +03e
        cmpa.l  #0xffffffff,a0                  | +042
        beq.w   .L06c022                        | +048
        jsr     0x28cd4.l                       | +04c
.L06c022:
        jsr     RocketVehicle_Fire_06d484(pc)   | +052
        bra.w   RocketVehicle_SpritesK_06c02a__L06c03a | +056

| ----------------------------------------------------------------------------
|  RocketVehicle_SpritesK_06c02a  @ $06C02A  (56 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_SpritesK_06c02a, "ax", @progbits
        .global RocketVehicle_SpritesK_06c02a
RocketVehicle_SpritesK_06c02a:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xddee                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xdd52                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xdda0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global RocketVehicle_SpritesK_06c02a__L06c03a
RocketVehicle_SpritesK_06c02a__L06c03a:
.L06c03a:
        lea     .L06c040(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L06c040:
        jsr     0x28998.l                       | +016
        jsr     RocketVehicle_GroundPhysics_06d0f6(pc) | +01c
        jsr     0x28d70.l                       | +020
        tst.b   0x73(a6)                        | +026
        beq.w   .L06c05e                        | +02a
        lea     RocketVehicle_BurstCount_06c062(pc),a1 | +02e
        move.l  a1,(a6)                         | +032
.L06c05e:
        bra.w   RocketVehicle_Wreck_06c37c__L06c418 | +034

| ----------------------------------------------------------------------------
|  RocketVehicle_BurstCount_06c062  @ $06C062  (20 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_BurstCount_06c062, "ax", @progbits
        .global RocketVehicle_BurstCount_06c062
RocketVehicle_BurstCount_06c062:
        lea     0x2bd5ee.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.b  d0,0x88(a6)                     | +00c
        bra.w   RocketVehicle_Attack_06c076__L06c084 | +010

| ----------------------------------------------------------------------------
|  RocketVehicle_Attack_06c076  @ $06C076  (92 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_Attack_06c076, "ax", @progbits
        .global RocketVehicle_Attack_06c076
RocketVehicle_Attack_06c076:
        subq.b  #0x1,0x88(a6)                   | +000
        cmpi.b  #0x0,0x88(a6)                   | +004
        ble.w   RocketVehicle_SpritesL_06c0d2__L06c122 | +00a
        .global RocketVehicle_Attack_06c076__L06c084
RocketVehicle_Attack_06c076__L06c084:
.L06c084:
        clr.b   0x73(a6)                        | +00e
        move.b  #0x3,0x72(a6)                   | +012
        lea     0x2bd56c.l,a0                   | +018
        jsr     0x799de.l                       | +01e
        move.w  d0,0x70(a6)                     | +024
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +028
        bcs.w   RocketVehicle_Die_06c272        | +02c
        move.b  d0,0x97(a6)                     | +030
        andi.w  #0x3,d0                         | +034
        movea.l #0x6c0d2,a0                     | +038
        lsl.w   #0x2,d0                         | +03e
        movea.l (a0,d0.w),a0                    | +040
        cmpa.l  #0xffffffff,a0                  | +044
        beq.w   .L06c0ca                        | +04a
        jsr     0x28cd4.l                       | +04e
.L06c0ca:
        jsr     RocketVehicle_Fire_06d484(pc)   | +054
        bra.w   RocketVehicle_SpritesL_06c0d2__L06c0e2 | +058

| ----------------------------------------------------------------------------
|  RocketVehicle_SpritesL_06c0d2  @ $06C0D2  (176 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_SpritesL_06c0d2, "ax", @progbits
        .global RocketVehicle_SpritesL_06c0d2
RocketVehicle_SpritesL_06c0d2:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xeb54                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xeab4                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xeb04                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global RocketVehicle_SpritesL_06c0d2__L06c0e2
RocketVehicle_SpritesL_06c0d2__L06c0e2:
.L06c0e2:
        lea     .L06c0e8(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L06c0e8:
        jsr     0x28998.l                       | +016
        jsr     RocketVehicle_GroundPhysics_06d0f6(pc) | +01c
        jsr     0x28d70.l                       | +020
        tst.b   0x73(a6)                        | +026
        beq.w   .L06c10a                        | +02a
        clr.b   0x73(a6)                        | +02e
        move.b  #0x5,0x72(a6)                   | +032
.L06c10a:
        subq.w  #0x1,0x70(a6)                   | +038
        cmpi.w  #0x0,0x70(a6)                   | +03c
        bgt.w   .L06c11e                        | +042
        lea     RocketVehicle_Attack_06c076(pc),a1 | +046
        move.l  a1,(a6)                         | +04a
.L06c11e:
        bra.w   RocketVehicle_Wreck_06c37c__L06c418 | +04c
        .global RocketVehicle_SpritesL_06c0d2__L06c122
RocketVehicle_SpritesL_06c0d2__L06c122:
.L06c122:
        lea     0x2bd4ea.l,a0                   | +050
        jsr     0x799de.l                       | +056
        move.w  d0,0x70(a6)                     | +05c
        addi.w  #0x1e,0x70(a6)                  | +060
        clr.b   0x88(a6)                        | +066
        clr.b   0x73(a6)                        | +06a
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +06e
        bcs.w   RocketVehicle_Die_06c272        | +072
        move.b  #0x4,0x72(a6)                   | +076
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +07c
        bcs.w   RocketVehicle_Die_06c272        | +080
        move.b  d0,0x97(a6)                     | +084
        andi.w  #0x3,d0                         | +088
        movea.l #0x6c182,a0                     | +08c
        lsl.w   #0x2,d0                         | +092
        movea.l (a0,d0.w),a0                    | +094
        cmpa.l  #0xffffffff,a0                  | +098
        beq.w   .L06c17a                        | +09e
        jsr     0x28cd4.l                       | +0a2
.L06c17a:
        jsr     RocketVehicle_Fire_06d484(pc)   | +0a8
        bra.w   RocketVehicle_SpritesM_06c182__L06c192 | +0ac

| ----------------------------------------------------------------------------
|  RocketVehicle_SpritesM_06c182  @ $06C182  (56 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_SpritesM_06c182, "ax", @progbits
        .global RocketVehicle_SpritesM_06c182
RocketVehicle_SpritesM_06c182:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xddee                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xdd52                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xdda0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global RocketVehicle_SpritesM_06c182__L06c192
RocketVehicle_SpritesM_06c182__L06c192:
.L06c192:
        lea     .L06c198(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L06c198:
        jsr     0x28998.l                       | +016
        jsr     RocketVehicle_GroundPhysics_06d0f6(pc) | +01c
        jsr     0x28d70.l                       | +020
        tst.b   0x73(a6)                        | +026
        beq.w   .L06c1b6                        | +02a
        lea     RocketVehicle_Turn_06bde8(pc),a1 | +02e
        move.l  a1,(a6)                         | +032
.L06c1b6:
        bra.w   RocketVehicle_Wreck_06c37c__L06c418 | +034

| ----------------------------------------------------------------------------
|  RocketVehicle_Hurt_06c1ba  @ $06C1BA  (76 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_Hurt_06c1ba, "ax", @progbits
        .global RocketVehicle_Hurt_06c1ba
RocketVehicle_Hurt_06c1ba:
        bclr    #0x3,0x13(a6)                   | +000
        clr.w   0x28(a6)                        | +006
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +00a
        bcs.w   RocketVehicle_Die_06c272        | +00e
        move.b  #0x6,0x72(a6)                   | +012
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +018
        bcs.w   RocketVehicle_Die_06c272        | +01c
        move.b  d0,0x97(a6)                     | +020
        andi.w  #0x3,d0                         | +024
        movea.l #0x6c206,a0                     | +028
        lsl.w   #0x2,d0                         | +02e
        movea.l (a0,d0.w),a0                    | +030
        cmpa.l  #0xffffffff,a0                  | +034
        beq.w   .L06c1fe                        | +03a
        jsr     0x28cd4.l                       | +03e
.L06c1fe:
        jsr     RocketVehicle_Fire_06d484(pc)   | +044
        bra.w   RocketVehicle_SpritesN_06c206__L06c216 | +048

| ----------------------------------------------------------------------------
|  RocketVehicle_SpritesN_06c206  @ $06C206  (108 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_SpritesN_06c206, "ax", @progbits
        .global RocketVehicle_SpritesN_06c206
RocketVehicle_SpritesN_06c206:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xf124                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xee74                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xefcc                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global RocketVehicle_SpritesN_06c206__L06c216
RocketVehicle_SpritesN_06c206__L06c216:
.L06c216:
        lea     .L06c21c(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L06c21c:
        jsr     0x28998.l                       | +016
        jsr     RocketVehicle_GroundPhysics_06d0f6(pc) | +01c
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +020
        bcs.w   .L06c236                        | +024
        cmp.b   0x97(a6),d0                     | +028
        beq.w   .L06c23c                        | +02c
.L06c236:
        lea     RocketVehicle_Idle_06bee0(pc),a1 | +030
        move.l  a1,(a6)                         | +034
.L06c23c:
        jsr     0x28d70.l                       | +036
        bcc.w   .L06c252                        | +03c
        bclr    #0x3,0x13(a6)                   | +040
        lea     RocketVehicle_Turn_06bde8(pc),a1 | +046
        move.l  a1,(a6)                         | +04a
.L06c252:
        jsr     0x2870a.l                       | +04c
        bcc.w   .L06c26e                        | +052
        lea     0x5e766.l,a0                    | +056
        jsr     0x5e770.l                       | +05c
        bclr    #0x3,0x13(a6)                   | +062
.L06c26e:
        bra.w   RocketVehicle_Wreck_06c37c__L06c448 | +068

| ----------------------------------------------------------------------------
|  RocketVehicle_Die_06c272  @ $06C272  (266 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_Die_06c272, "ax", @progbits
        .global RocketVehicle_Die_06c272
RocketVehicle_Die_06c272:
        move.w  0x36(a6),d0                     | +000
        btst    #0x0,0x8d(a6)                   | +004
        bne.w   .L06c282                        | +00a
        neg.w   d0                              | +00e
.L06c282:
        move.w  d0,0x28(a6)                     | +010
        move.b  #0x1,0x72(a6)                   | +014
        move.l  #0x2cc9d2,0x48(a6)              | +01a
        move.l  #0x2cc88a,0x60(a6)              | +022
        move.l  #0x2ccda2,d0                    | +02a
        tst.b   0x9e(a6)                        | +030
        beq.w   .L06c2b0                        | +034
        move.l  #0x2cce42,d0                    | +038
.L06c2b0:
        move.l  d0,0x4c(a6)                     | +03e
        lea     0x2cd636.l,a0                   | +042
        jsr     0x28cd4.l                       | +048
        lea     .L06c2c6(pc),a1                 | +04e
        move.l  a1,(a6)                         | +052
.L06c2c6:
        jsr     0x28998.l                       | +054
        jsr     RocketVehicle_GroundPhysics_06d0f6(pc) | +05a
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +05e
        bcs.w   .L06c2de                        | +062
        lea     RocketVehicle_Drive_06bb02(pc),a1 | +066
        move.l  a1,(a6)                         | +06a
.L06c2de:
        jsr     0x28d70.l                       | +06c
        jsr     0x2870a.l                       | +072
        bcc.w   .L06c2f4                        | +078
        bclr    #0x3,0x13(a6)                   | +07c
.L06c2f4:
        subq.w  #0x1,0x70(a6)                   | +082
        jsr     RocketVehicle_LinkBlocked_06d13c(pc) | +086
        bcc.w   .L06c310                        | +08a
        eori.b  #0x1,0x8d(a6)                   | +08e
        lea     RocketVehicle_Wreck_06c37c(pc),a1 | +094
        move.l  a1,(a6)                         | +098
        bra.w   .L06c36c                        | +09a
.L06c310:
        jsr     RocketVehicle_WallAhead_06d172(pc) | +09e
        bcc.w   .L06c328                        | +0a2
        eori.b  #0x1,0x8d(a6)                   | +0a6
        lea     RocketVehicle_Wreck_06c37c(pc),a1 | +0ac
        move.l  a1,(a6)                         | +0b0
        bra.w   .L06c36c                        | +0b2
.L06c328:
        cmpi.w  #0x10,0x22(a6)                  | +0b6
        bgt.w   .L06c34c                        | +0bc
        btst    #0x0,0x8d(a6)                   | +0c0
        bne.w   .L06c34c                        | +0c6
        ori.b   #0x1,0x8d(a6)                   | +0ca
        lea     RocketVehicle_Wreck_06c37c(pc),a1 | +0d0
        move.l  a1,(a6)                         | +0d4
        bra.w   .L06c36c                        | +0d6
.L06c34c:
        cmpi.w  #0x130,0x22(a6)                 | +0da
        blt.w   .L06c36c                        | +0e0
        btst    #0x0,0x8d(a6)                   | +0e4
        beq.w   .L06c36c                        | +0ea
        andi.b  #0xfe,0x8d(a6)                  | +0ee
        lea     RocketVehicle_Wreck_06c37c(pc),a1 | +0f4
        move.l  a1,(a6)                         | +0f8
.L06c36c:
        jsr     0x283ca.l                       | +0fa
        jsr     0x283d8.l                       | +100
        bra.w   RocketVehicle_Wreck_06c37c__L06c448 | +106

| ----------------------------------------------------------------------------
|  RocketVehicle_Wreck_06c37c  @ $06C37C  (228 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_Wreck_06c37c, "ax", @progbits
        .global RocketVehicle_Wreck_06c37c
RocketVehicle_Wreck_06c37c:
        jsr     0x267e2.l                       | +000
        move.b  #0x1,0x72(a6)                   | +006
        move.l  #0x2cc9d2,0x48(a6)              | +00c
        move.l  #0x2cc88a,0x60(a6)              | +014
        move.l  #0x2ccda2,d0                    | +01c
        tst.b   0x9e(a6)                        | +022
        beq.w   .L06c3ac                        | +026
        move.l  #0x2cce42,d0                    | +02a
.L06c3ac:
        move.l  d0,0x4c(a6)                     | +030
        lea     0x2bd670.l,a0                   | +034
        jsr     0x799de.l                       | +03a
        lsr.b   #0x1,d0                         | +040
        move.b  d0,0x88(a6)                     | +042
        lea     0x2cd636.l,a0                   | +046
        jsr     0x28cd4.l                       | +04c
        lea     .L06c3d4(pc),a1                 | +052
        move.l  a1,(a6)                         | +056
.L06c3d4:
        jsr     0x28998.l                       | +058
        jsr     RocketVehicle_GroundPhysics_06d0f6(pc) | +05e
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +062
        bcs.w   .L06c3ec                        | +066
        lea     RocketVehicle_Idle_06bee0(pc),a1 | +06a
        move.l  a1,(a6)                         | +06e
.L06c3ec:
        jsr     0x28d70.l                       | +070
        jsr     0x2870a.l                       | +076
        bcc.w   .L06c402                        | +07c
        bclr    #0x3,0x13(a6)                   | +080
.L06c402:
        subq.w  #0x1,0x70(a6)                   | +086
        subq.b  #0x1,0x88(a6)                   | +08a
        bgt.w   .L06c414                        | +08e
        lea     RocketVehicle_Die_06c272(pc),a1 | +092
        move.l  a1,(a6)                         | +096
.L06c414:
        bra.w   .L06c448                        | +098
        .global RocketVehicle_Wreck_06c37c__L06c418
RocketVehicle_Wreck_06c37c__L06c418:
.L06c418:
        clr.b   0x94(a6)                        | +09c
        jsr     0x2870a.l                       | +0a0
        bcc.w   .L06c448                        | +0a6
        lea     0x5e766.l,a0                    | +0aa
        jsr     0x5e770.l                       | +0b0
        bclr    #0x3,0x13(a6)                   | +0b6
        move.b  #0x1,0x94(a6)                   | +0bc
        bra.w   .L06c448                        | +0c2
        lea     RocketVehicle_Hurt_06c1ba(pc),a1 | +0c6
        move.l  a1,(a6)                         | +0ca
        .global RocketVehicle_Wreck_06c37c__L06c448
RocketVehicle_Wreck_06c37c__L06c448:
.L06c448:
        jsr     0x28758.l                       | +0cc
        bcc.w   .L06c458                        | +0d2
        lea     RocketVehicle_Corpse_06c468(pc),a1 | +0d6
        move.l  a1,(a6)                         | +0da
.L06c458:
        jsr     RocketVehicle_OffWorldOrLeft_06d05c(pc) | +0dc
        bcc.w   SetHandlerRts_06c466            | +0e0

| ----------------------------------------------------------------------------
|  RocketVehicle_Corpse_06c468  @ $06C468  (86 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_Corpse_06c468, "ax", @progbits
        .global RocketVehicle_Corpse_06c468
RocketVehicle_Corpse_06c468:
        bclr    #0x1,0x12(a6)                   | +000
        clr.w   0x28(a6)                        | +006
        move.b  #0x7,0x72(a6)                   | +00a
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +010
        andi.w  #0x3,d0                         | +014
        movea.l #0x2ccfb6,a0                    | +018
        lsl.w   #0x2,d0                         | +01e
        movea.l (a0,d0.w),a0                    | +020
        cmpa.l  #0xffffffff,a0                  | +024
        beq.w   .L06c49c                        | +02a
        jsr     0x28cd4.l                       | +02e
.L06c49c:
        lea     .L06c4a2(pc),a1                 | +034
        move.l  a1,(a6)                         | +038
.L06c4a2:
        jsr     RocketVehicle_GroundPhysics_06d0f6(pc) | +03a
        jsr     0x28d70.l                       | +03e
        bcc.w   .L06c4b6                        | +044
        lea     RocketVehicle_DetachLinks_06c4c6(pc),a1 | +048
        move.l  a1,(a6)                         | +04c
.L06c4b6:
        jsr     RocketVehicle_OffWorldOrLeft_06d05c(pc) | +04e
        bcc.w   SetHandlerRts_06c4c4            | +052

| ----------------------------------------------------------------------------
|  RocketVehicle_DetachLinks_06c4c6  @ $06C4C6  (112 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_DetachLinks_06c4c6, "ax", @progbits
        .global RocketVehicle_DetachLinks_06c4c6
RocketVehicle_DetachLinks_06c4c6:
        move.l  0x74(a6),d0                     | +000
        cmpi.l  #0xffffffff,d0                  | +004
        beq.w   .L06c4dc                        | +00a
        movea.l d0,a0                           | +00e
        move.l  #0x6c554,(a0)                   | +010
.L06c4dc:
        move.l  0x78(a6),d0                     | +016
        cmpi.l  #0xffffffff,d0                  | +01a
        beq.w   .L06c4f2                        | +020
        movea.l d0,a0                           | +024
        move.l  #0x6c554,(a0)                   | +026
.L06c4f2:
        move.l  0x7c(a6),d0                     | +02c
        cmpi.l  #0xffffffff,d0                  | +030
        beq.w   .L06c508                        | +036
        movea.l d0,a0                           | +03a
        move.l  #0x6c554,(a0)                   | +03c
.L06c508:
        move.w  #0x3c,0x70(a6)                  | +042
        move.l  #0xffffffff,0x48(a6)            | +048
        bclr    #0x1,0x12(a6)                   | +050
        move.b  #0x7,0x72(a6)                   | +056
        lea     .L06c528(pc),a1                 | +05c
        move.l  a1,(a6)                         | +060
.L06c528:
        subq.w  #0x1,0x70(a6)                   | +062
        cmpi.w  #0x0,0x70(a6)                   | +066
        bgt.w   SetHandlerRts_06c53c            | +06c

| ----------------------------------------------------------------------------
|  RocketVehicle_Rider_06c562  @ $06C562  (80 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_Rider_06c562, "ax", @progbits
        .global RocketVehicle_Rider_06c562
RocketVehicle_Rider_06c562:
        jsr     RocketVehicle_PlayCry_06d574(pc) | +000
        andi.b  #0x3,0x98(a6)                   | +004
        move.b  #0x0,0x72(a6)                   | +00a
        .global RocketVehicle_Rider_06c562__L06c572
RocketVehicle_Rider_06c562__L06c572:
.L06c572:
        move.w  #0x8,0x34(a6)                   | +010
        movea.l 0xc(a6),a0                      | +016
        move.w  0x8a(a0),0x8a(a6)               | +01a
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +020
        bcs.w   RocketVehicle_RiderSpritesC_06c6b6__L06c6ca | +024
        move.b  d0,0x97(a6)                     | +028
        andi.w  #0x3,d0                         | +02c
        movea.l #0x6c5b2,a0                     | +030
        lsl.w   #0x2,d0                         | +036
        movea.l (a0,d0.w),a0                    | +038
        cmpa.l  #0xffffffff,a0                  | +03c
        beq.w   .L06c5ae                        | +042
        jsr     0x28cd4.l                       | +046
.L06c5ae:
        bra.w   RocketVehicle_RiderSpritesA_06c5b2__L06c5c2 | +04c

| ----------------------------------------------------------------------------
|  RocketVehicle_RiderSpritesA_06c5b2  @ $06C5B2  (176 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_RiderSpritesA_06c5b2, "ax", @progbits
        .global RocketVehicle_RiderSpritesA_06c5b2
RocketVehicle_RiderSpritesA_06c5b2:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xfada                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xf886                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xf9b0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global RocketVehicle_RiderSpritesA_06c5b2__L06c5c2
RocketVehicle_RiderSpritesA_06c5b2__L06c5c2:
.L06c5c2:
        lea     .L06c5c8(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L06c5c8:
        movea.l 0xc(a6),a0                      | +016
        move.b  0x72(a0),d0                     | +01a
        cmp.b   0x72(a6),d0                     | +01e
        beq.w   .L06c60c                        | +022
        cmpi.b  #0x7,0x72(a6)                   | +026
        beq.w   RocketVehicle_WeaponFlyOff_06cdba | +02c
        move.b  d0,0x72(a6)                     | +030
        cmpi.b  #0x9,d0                         | +034
        bcs.w   .L06c5fa                        | +038
        nop                                     | +03c
        nop                                     | +03e
        cmpi.b  #0x9,d0                         | +040
        nop                                     | +044
        trap    #0xf                            | +046
.L06c5fa:
        andi.w  #0xf,d0                         | +048
        lea     0x2cd606.l,a0                   | +04c
        lsl.w   #0x2,d0                         | +052
        movea.l (a0,d0.w),a1                    | +054
        jmp     (a1)                            | +058
.L06c60c:
        move.b  d0,0x72(a6)                     | +05a
        jsr     RocketVehicle_RiderFollowParent_06d29c(pc) | +05e
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +062
        bcs.w   RocketVehicle_RiderSpritesC_06c6b6__L06c6ca | +066
        cmp.b   0x97(a6),d0                     | +06a
        beq.w   RocketVehicle_RiderSpritesB_06c662__L06c672 | +06e
        move.b  d0,0x97(a6)                     | +072
        movea.l 0xc(a6),a0                      | +076
        move.w  0x8a(a0),0x8a(a6)               | +07a
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +080
        bcs.w   RocketVehicle_RiderSpritesC_06c6b6__L06c6ca | +084
        move.b  d0,0x97(a6)                     | +088
        andi.w  #0x3,d0                         | +08c
        movea.l #0x6c662,a0                     | +090
        lsl.w   #0x2,d0                         | +096
        movea.l (a0,d0.w),a0                    | +098
        cmpa.l  #0xffffffff,a0                  | +09c
        beq.w   .L06c65e                        | +0a2
        jsr     0x28cd4.l                       | +0a6
.L06c65e:
        bra.w   RocketVehicle_RiderSpritesB_06c662__L06c672 | +0ac

| ----------------------------------------------------------------------------
|  RocketVehicle_RiderSpritesB_06c662  @ $06C662  (84 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_RiderSpritesB_06c662, "ax", @progbits
        .global RocketVehicle_RiderSpritesB_06c662
RocketVehicle_RiderSpritesB_06c662:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xfada                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xf886                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xf9b0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global RocketVehicle_RiderSpritesB_06c662__L06c672
RocketVehicle_RiderSpritesB_06c662__L06c672:
.L06c672:
        jsr     0x28d70.l                       | +010
        bcc.w   RocketVehicle_RiderSpritesC_06c6b6__L06c6c6 | +016
        movea.l 0xc(a6),a0                      | +01a
        move.w  0x8a(a0),0x8a(a6)               | +01e
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +024
        bcs.w   RocketVehicle_RiderSpritesC_06c6b6__L06c6ca | +028
        move.b  d0,0x97(a6)                     | +02c
        andi.w  #0x3,d0                         | +030
        movea.l #0x6c6b6,a0                     | +034
        lsl.w   #0x2,d0                         | +03a
        movea.l (a0,d0.w),a0                    | +03c
        cmpa.l  #0xffffffff,a0                  | +040
        beq.w   .L06c6b2                        | +046
        jsr     0x28cd4.l                       | +04a
.L06c6b2:
        bra.w   RocketVehicle_RiderSpritesC_06c6b6__L06c6c6 | +050

| ----------------------------------------------------------------------------
|  RocketVehicle_RiderSpritesC_06c6b6  @ $06C6B6  (128 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_RiderSpritesC_06c6b6, "ax", @progbits
        .global RocketVehicle_RiderSpritesC_06c6b6
RocketVehicle_RiderSpritesC_06c6b6:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xfada                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xf886                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xf9b0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global RocketVehicle_RiderSpritesC_06c6b6__L06c6c6
RocketVehicle_RiderSpritesC_06c6b6__L06c6c6:
.L06c6c6:
        bra.w   RocketVehicle_RiderCorpse_06cd62 | +010
        .global RocketVehicle_RiderSpritesC_06c6b6__L06c6ca
RocketVehicle_RiderSpritesC_06c6b6__L06c6ca:
.L06c6ca:
        move.w  #0x8,0x34(a6)                   | +014
        lea     0x2cf614.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        lea     .L06c6e2(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L06c6e2:
        movea.l 0xc(a6),a0                      | +02c
        move.b  0x72(a0),d0                     | +030
        cmp.b   0x72(a6),d0                     | +034
        beq.w   .L06c726                        | +038
        cmpi.b  #0x7,0x72(a6)                   | +03c
        beq.w   RocketVehicle_WeaponFlyOff_06cdba | +042
        move.b  d0,0x72(a6)                     | +046
        cmpi.b  #0x9,d0                         | +04a
        bcs.w   .L06c714                        | +04e
        nop                                     | +052
        nop                                     | +054
        cmpi.b  #0x9,d0                         | +056
        nop                                     | +05a
        trap    #0xf                            | +05c
.L06c714:
        andi.w  #0xf,d0                         | +05e
        lea     0x2cd606.l,a0                   | +062
        lsl.w   #0x2,d0                         | +068
        movea.l (a0,d0.w),a1                    | +06a
        jmp     (a1)                            | +06e
.L06c726:
        move.b  d0,0x72(a6)                     | +070
        jsr     RocketVehicle_RiderFollowParent_06d29c(pc) | +074
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +078
        bcc.w   RocketVehicle_Rider_06c562__L06c572 | +07c

| ----------------------------------------------------------------------------
|  RocketVehicle_RiderIdle_06c73e  @ $06C73E  (72 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_RiderIdle_06c73e, "ax", @progbits
        .global RocketVehicle_RiderIdle_06c73e
RocketVehicle_RiderIdle_06c73e:
        clr.b   0x88(a6)                        | +000
        movea.l 0xc(a6),a0                      | +004
        move.l  0x90(a0),0x90(a6)               | +008
        movea.l 0xc(a6),a0                      | +00e
        move.w  0x8a(a0),0x8a(a6)               | +012
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +018
        bcs.w   RocketVehicle_RiderSpritesC_06c6b6__L06c6ca | +01c
        move.b  d0,0x97(a6)                     | +020
        andi.w  #0x3,d0                         | +024
        movea.l #0x6c786,a0                     | +028
        lsl.w   #0x2,d0                         | +02e
        movea.l (a0,d0.w),a0                    | +030
        cmpa.l  #0xffffffff,a0                  | +034
        beq.w   .L06c782                        | +03a
        jsr     0x28cd4.l                       | +03e
.L06c782:
        bra.w   RocketVehicle_RiderSpritesD_06c786__L06c796 | +044

| ----------------------------------------------------------------------------
|  RocketVehicle_RiderSpritesD_06c786  @ $06C786  (220 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_RiderSpritesD_06c786, "ax", @progbits
        .global RocketVehicle_RiderSpritesD_06c786
RocketVehicle_RiderSpritesD_06c786:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xfada                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xf886                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xf9b0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global RocketVehicle_RiderSpritesD_06c786__L06c796
RocketVehicle_RiderSpritesD_06c786__L06c796:
.L06c796:
        lea     .L06c79c(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L06c79c:
        addq.b  #0x1,0x88(a6)                   | +016
        andi.b  #0x3,0x88(a6)                   | +01a
        bne.w   .L06c7c8                        | +020
        jsr     RocketVehicle_RangeClass_06d2fa(pc) | +024
        movea.l 0xc(a6),a0                      | +028
        clr.b   0x73(a0)                        | +02c
        jsr     RocketVehicle_AimStep_06d340(pc) | +030
        bcc.w   .L06c7c8                        | +034
        movea.l 0xc(a6),a0                      | +038
        move.b  #0x1,0x73(a0)                   | +03c
.L06c7c8:
        movea.l 0xc(a6),a0                      | +042
        move.b  0x72(a0),d0                     | +046
        cmp.b   0x72(a6),d0                     | +04a
        beq.w   .L06c80c                        | +04e
        cmpi.b  #0x7,0x72(a6)                   | +052
        beq.w   RocketVehicle_WeaponFlyOff_06cdba | +058
        move.b  d0,0x72(a6)                     | +05c
        cmpi.b  #0x9,d0                         | +060
        bcs.w   .L06c7fa                        | +064
        nop                                     | +068
        nop                                     | +06a
        cmpi.b  #0x9,d0                         | +06c
        nop                                     | +070
        trap    #0xf                            | +072
.L06c7fa:
        andi.w  #0xf,d0                         | +074
        lea     0x2cd606.l,a0                   | +078
        lsl.w   #0x2,d0                         | +07e
        movea.l (a0,d0.w),a1                    | +080
        jmp     (a1)                            | +084
.L06c80c:
        move.b  d0,0x72(a6)                     | +086
        jsr     RocketVehicle_RiderFollowParent_06d29c(pc) | +08a
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +08e
        bcs.w   RocketVehicle_RiderSpritesC_06c6b6__L06c6ca | +092
        cmp.b   0x97(a6),d0                     | +096
        beq.w   RocketVehicle_RiderSpritesE_06c862__L06c872 | +09a
        move.b  d0,0x97(a6)                     | +09e
        movea.l 0xc(a6),a0                      | +0a2
        move.w  0x8a(a0),0x8a(a6)               | +0a6
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +0ac
        bcs.w   RocketVehicle_RiderSpritesC_06c6b6__L06c6ca | +0b0
        move.b  d0,0x97(a6)                     | +0b4
        andi.w  #0x3,d0                         | +0b8
        movea.l #0x6c862,a0                     | +0bc
        lsl.w   #0x2,d0                         | +0c2
        movea.l (a0,d0.w),a0                    | +0c4
        cmpa.l  #0xffffffff,a0                  | +0c8
        beq.w   .L06c85e                        | +0ce
        jsr     0x28cd4.l                       | +0d2
.L06c85e:
        bra.w   RocketVehicle_RiderSpritesE_06c862__L06c872 | +0d8

| ----------------------------------------------------------------------------
|  RocketVehicle_RiderSpritesE_06c862  @ $06C862  (84 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_RiderSpritesE_06c862, "ax", @progbits
        .global RocketVehicle_RiderSpritesE_06c862
RocketVehicle_RiderSpritesE_06c862:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xfada                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xf886                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xf9b0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global RocketVehicle_RiderSpritesE_06c862__L06c872
RocketVehicle_RiderSpritesE_06c862__L06c872:
.L06c872:
        jsr     0x28d70.l                       | +010
        bcc.w   RocketVehicle_RiderSpritesF_06c8b6__L06c8c6 | +016
        movea.l 0xc(a6),a0                      | +01a
        move.w  0x8a(a0),0x8a(a6)               | +01e
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +024
        bcs.w   RocketVehicle_RiderSpritesC_06c6b6__L06c6ca | +028
        move.b  d0,0x97(a6)                     | +02c
        andi.w  #0x3,d0                         | +030
        movea.l #0x6c8b6,a0                     | +034
        lsl.w   #0x2,d0                         | +03a
        movea.l (a0,d0.w),a0                    | +03c
        cmpa.l  #0xffffffff,a0                  | +040
        beq.w   .L06c8b2                        | +046
        jsr     0x28cd4.l                       | +04a
.L06c8b2:
        bra.w   RocketVehicle_RiderSpritesF_06c8b6__L06c8c6 | +050

| ----------------------------------------------------------------------------
|  RocketVehicle_RiderSpritesF_06c8b6  @ $06C8B6  (20 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_RiderSpritesF_06c8b6, "ax", @progbits
        .global RocketVehicle_RiderSpritesF_06c8b6
RocketVehicle_RiderSpritesF_06c8b6:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xfada                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xf886                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xf9b0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global RocketVehicle_RiderSpritesF_06c8b6__L06c8c6
RocketVehicle_RiderSpritesF_06c8b6__L06c8c6:
.L06c8c6:
        bra.w   RocketVehicle_RiderCorpse_06cd62 | +010

| ----------------------------------------------------------------------------
|  RocketVehicle_RiderAim_06c8ca  @ $06C8CA  (176 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_RiderAim_06c8ca, "ax", @progbits
        .global RocketVehicle_RiderAim_06c8ca
RocketVehicle_RiderAim_06c8ca:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x8a(a0),0x8a(a6)               | +004
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +00a
        bcs.w   RocketVehicle_RiderSpritesC_06c6b6__L06c6ca | +00e
        move.b  d0,0x97(a6)                     | +012
        andi.w  #0x3,d0                         | +016
        lea     0x2cd02a.l,a0                   | +01a
        lsl.w   #0x2,d0                         | +020
        movea.l (a0,d0.w),a0                    | +022
        move.w  0x34(a6),d0                     | +026
        andi.w  #0xf,d0                         | +02a
        lsl.w   #0x2,d0                         | +02e
        movea.l (a0,d0.w),a0                    | +030
        cmpa.l  #0xffffffff,a0                  | +034
        beq.w   .L06c90e                        | +03a
        jsr     0x28cd4.l                       | +03e
.L06c90e:
        jsr     RocketVehicle_FireRocket_06d4c2(pc) | +044
        lea     .L06c918(pc),a1                 | +048
        move.l  a1,(a6)                         | +04c
.L06c918:
        movea.l 0xc(a6),a0                      | +04e
        move.b  0x72(a0),d0                     | +052
        cmp.b   0x72(a6),d0                     | +056
        beq.w   .L06c95c                        | +05a
        cmpi.b  #0x7,0x72(a6)                   | +05e
        beq.w   RocketVehicle_WeaponFlyOff_06cdba | +064
        move.b  d0,0x72(a6)                     | +068
        cmpi.b  #0x9,d0                         | +06c
        bcs.w   .L06c94a                        | +070
        nop                                     | +074
        nop                                     | +076
        cmpi.b  #0x9,d0                         | +078
        nop                                     | +07c
        trap    #0xf                            | +07e
.L06c94a:
        andi.w  #0xf,d0                         | +080
        lea     0x2cd606.l,a0                   | +084
        lsl.w   #0x2,d0                         | +08a
        movea.l (a0,d0.w),a1                    | +08c
        jmp     (a1)                            | +090
.L06c95c:
        move.b  d0,0x72(a6)                     | +092
        jsr     RocketVehicle_RiderFollowParent_06d29c(pc) | +096
        jsr     0x28d70.l                       | +09a
        bcc.w   .L06c978                        | +0a0
        movea.l 0xc(a6),a0                      | +0a4
        move.b  #0x1,0x73(a0)                   | +0a8
.L06c978:
        rts                                     | +0ae

| ----------------------------------------------------------------------------
|  RocketVehicle_RiderFire_06c97a  @ $06C97A  (62 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_RiderFire_06c97a, "ax", @progbits
        .global RocketVehicle_RiderFire_06c97a
RocketVehicle_RiderFire_06c97a:
        clr.b   0x88(a6)                        | +000
        movea.l 0xc(a6),a0                      | +004
        move.w  0x8a(a0),0x8a(a6)               | +008
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +00e
        bcs.w   RocketVehicle_RiderSpritesC_06c6b6__L06c6ca | +012
        move.b  d0,0x97(a6)                     | +016
        andi.w  #0x3,d0                         | +01a
        movea.l #0x6c9b8,a0                     | +01e
        lsl.w   #0x2,d0                         | +024
        movea.l (a0,d0.w),a0                    | +026
        cmpa.l  #0xffffffff,a0                  | +02a
        beq.w   .L06c9b4                        | +030
        jsr     0x28cd4.l                       | +034
.L06c9b4:
        bra.w   RocketVehicle_RiderSpritesG_06c9b8__L06c9c8 | +03a

| ----------------------------------------------------------------------------
|  RocketVehicle_RiderSpritesG_06c9b8  @ $06C9B8  (230 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_RiderSpritesG_06c9b8, "ax", @progbits
        .global RocketVehicle_RiderSpritesG_06c9b8
RocketVehicle_RiderSpritesG_06c9b8:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xfada                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xf886                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xf9b0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global RocketVehicle_RiderSpritesG_06c9b8__L06c9c8
RocketVehicle_RiderSpritesG_06c9b8__L06c9c8:
.L06c9c8:
        lea     .L06c9ce(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L06c9ce:
        addq.b  #0x1,0x88(a6)                   | +016
        andi.b  #0x3,0x88(a6)                   | +01a
        bne.w   .L06ca04                        | +020
        move.w  #0x1,d1                         | +024
        cmpi.w  #0x8,0x34(a6)                   | +028
        beq.w   .L06c9fa                        | +02e
        blt.w   .L06c9f2                        | +032
        move.w  #0xffff,d1                      | +036
.L06c9f2:
        add.w   d1,0x34(a6)                     | +03a
        bra.w   .L06ca04                        | +03e
.L06c9fa:
        movea.l 0xc(a6),a0                      | +042
        move.b  #0x1,0x73(a0)                   | +046
.L06ca04:
        movea.l 0xc(a6),a0                      | +04c
        move.b  0x72(a0),d0                     | +050
        cmp.b   0x72(a6),d0                     | +054
        beq.w   .L06ca48                        | +058
        cmpi.b  #0x7,0x72(a6)                   | +05c
        beq.w   RocketVehicle_WeaponFlyOff_06cdba | +062
        move.b  d0,0x72(a6)                     | +066
        cmpi.b  #0x9,d0                         | +06a
        bcs.w   .L06ca36                        | +06e
        nop                                     | +072
        nop                                     | +074
        cmpi.b  #0x9,d0                         | +076
        nop                                     | +07a
        trap    #0xf                            | +07c
.L06ca36:
        andi.w  #0xf,d0                         | +07e
        lea     0x2cd606.l,a0                   | +082
        lsl.w   #0x2,d0                         | +088
        movea.l (a0,d0.w),a1                    | +08a
        jmp     (a1)                            | +08e
.L06ca48:
        move.b  d0,0x72(a6)                     | +090
        jsr     RocketVehicle_RiderFollowParent_06d29c(pc) | +094
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +098
        bcs.w   RocketVehicle_RiderSpritesC_06c6b6__L06c6ca | +09c
        cmp.b   0x97(a6),d0                     | +0a0
        beq.w   RocketVehicle_RiderSpritesH_06ca9e__L06caae | +0a4
        move.b  d0,0x97(a6)                     | +0a8
        movea.l 0xc(a6),a0                      | +0ac
        move.w  0x8a(a0),0x8a(a6)               | +0b0
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +0b6
        bcs.w   RocketVehicle_RiderSpritesC_06c6b6__L06c6ca | +0ba
        move.b  d0,0x97(a6)                     | +0be
        andi.w  #0x3,d0                         | +0c2
        movea.l #0x6ca9e,a0                     | +0c6
        lsl.w   #0x2,d0                         | +0cc
        movea.l (a0,d0.w),a0                    | +0ce
        cmpa.l  #0xffffffff,a0                  | +0d2
        beq.w   .L06ca9a                        | +0d8
        jsr     0x28cd4.l                       | +0dc
.L06ca9a:
        bra.w   RocketVehicle_RiderSpritesH_06ca9e__L06caae | +0e2

| ----------------------------------------------------------------------------
|  RocketVehicle_RiderSpritesH_06ca9e  @ $06CA9E  (84 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_RiderSpritesH_06ca9e, "ax", @progbits
        .global RocketVehicle_RiderSpritesH_06ca9e
RocketVehicle_RiderSpritesH_06ca9e:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xfada                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xf886                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xf9b0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global RocketVehicle_RiderSpritesH_06ca9e__L06caae
RocketVehicle_RiderSpritesH_06ca9e__L06caae:
.L06caae:
        jsr     0x28d70.l                       | +010
        bcc.w   RocketVehicle_RiderSpritesI_06caf2__L06cb02 | +016
        movea.l 0xc(a6),a0                      | +01a
        move.w  0x8a(a0),0x8a(a6)               | +01e
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +024
        bcs.w   RocketVehicle_RiderSpritesC_06c6b6__L06c6ca | +028
        move.b  d0,0x97(a6)                     | +02c
        andi.w  #0x3,d0                         | +030
        movea.l #0x6caf2,a0                     | +034
        lsl.w   #0x2,d0                         | +03a
        movea.l (a0,d0.w),a0                    | +03c
        cmpa.l  #0xffffffff,a0                  | +040
        beq.w   .L06caee                        | +046
        jsr     0x28cd4.l                       | +04a
.L06caee:
        bra.w   RocketVehicle_RiderSpritesI_06caf2__L06cb02 | +050

| ----------------------------------------------------------------------------
|  RocketVehicle_RiderSpritesI_06caf2  @ $06CAF2  (20 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_RiderSpritesI_06caf2, "ax", @progbits
        .global RocketVehicle_RiderSpritesI_06caf2
RocketVehicle_RiderSpritesI_06caf2:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xfada                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xf886                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xf9b0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global RocketVehicle_RiderSpritesI_06caf2__L06cb02
RocketVehicle_RiderSpritesI_06caf2__L06cb02:
.L06cb02:
        bra.w   RocketVehicle_RiderCorpse_06cd62 | +010

| ----------------------------------------------------------------------------
|  RocketVehicle_RiderHurt_06cb06  @ $06CB06  (62 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_RiderHurt_06cb06, "ax", @progbits
        .global RocketVehicle_RiderHurt_06cb06
RocketVehicle_RiderHurt_06cb06:
        clr.b   0x88(a6)                        | +000
        movea.l 0xc(a6),a0                      | +004
        move.w  0x8a(a0),0x8a(a6)               | +008
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +00e
        bcs.w   RocketVehicle_RiderSpritesC_06c6b6__L06c6ca | +012
        move.b  d0,0x97(a6)                     | +016
        andi.w  #0x3,d0                         | +01a
        movea.l #0x6cb44,a0                     | +01e
        lsl.w   #0x2,d0                         | +024
        movea.l (a0,d0.w),a0                    | +026
        cmpa.l  #0xffffffff,a0                  | +02a
        beq.w   .L06cb40                        | +030
        jsr     0x28cd4.l                       | +034
.L06cb40:
        bra.w   RocketVehicle_RiderSpritesJ_06cb44__L06cb54 | +03a

| ----------------------------------------------------------------------------
|  RocketVehicle_RiderSpritesJ_06cb44  @ $06CB44  (176 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_RiderSpritesJ_06cb44, "ax", @progbits
        .global RocketVehicle_RiderSpritesJ_06cb44
RocketVehicle_RiderSpritesJ_06cb44:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xfada                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xf886                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xf9b0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global RocketVehicle_RiderSpritesJ_06cb44__L06cb54
RocketVehicle_RiderSpritesJ_06cb44__L06cb54:
.L06cb54:
        lea     .L06cb5a(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L06cb5a:
        movea.l 0xc(a6),a0                      | +016
        move.b  0x72(a0),d0                     | +01a
        cmp.b   0x72(a6),d0                     | +01e
        beq.w   .L06cb9e                        | +022
        cmpi.b  #0x7,0x72(a6)                   | +026
        beq.w   RocketVehicle_WeaponFlyOff_06cdba | +02c
        move.b  d0,0x72(a6)                     | +030
        cmpi.b  #0x9,d0                         | +034
        bcs.w   .L06cb8c                        | +038
        nop                                     | +03c
        nop                                     | +03e
        cmpi.b  #0x9,d0                         | +040
        nop                                     | +044
        trap    #0xf                            | +046
.L06cb8c:
        andi.w  #0xf,d0                         | +048
        lea     0x2cd606.l,a0                   | +04c
        lsl.w   #0x2,d0                         | +052
        movea.l (a0,d0.w),a1                    | +054
        jmp     (a1)                            | +058
.L06cb9e:
        move.b  d0,0x72(a6)                     | +05a
        jsr     RocketVehicle_RiderFollowParent_06d29c(pc) | +05e
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +062
        bcs.w   RocketVehicle_RiderSpritesC_06c6b6__L06c6ca | +066
        cmp.b   0x97(a6),d0                     | +06a
        beq.w   RocketVehicle_RiderSpritesK_06cbf4__L06cc04 | +06e
        move.b  d0,0x97(a6)                     | +072
        movea.l 0xc(a6),a0                      | +076
        move.w  0x8a(a0),0x8a(a6)               | +07a
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +080
        bcs.w   RocketVehicle_RiderSpritesC_06c6b6__L06c6ca | +084
        move.b  d0,0x97(a6)                     | +088
        andi.w  #0x3,d0                         | +08c
        movea.l #0x6cbf4,a0                     | +090
        lsl.w   #0x2,d0                         | +096
        movea.l (a0,d0.w),a0                    | +098
        cmpa.l  #0xffffffff,a0                  | +09c
        beq.w   .L06cbf0                        | +0a2
        jsr     0x28cd4.l                       | +0a6
.L06cbf0:
        bra.w   RocketVehicle_RiderSpritesK_06cbf4__L06cc04 | +0ac

| ----------------------------------------------------------------------------
|  RocketVehicle_RiderSpritesK_06cbf4  @ $06CBF4  (84 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_RiderSpritesK_06cbf4, "ax", @progbits
        .global RocketVehicle_RiderSpritesK_06cbf4
RocketVehicle_RiderSpritesK_06cbf4:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xfada                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xf886                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xf9b0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global RocketVehicle_RiderSpritesK_06cbf4__L06cc04
RocketVehicle_RiderSpritesK_06cbf4__L06cc04:
.L06cc04:
        jsr     0x28d70.l                       | +010
        bcc.w   RocketVehicle_RiderSpritesL_06cc48__L06cc58 | +016
        movea.l 0xc(a6),a0                      | +01a
        move.w  0x8a(a0),0x8a(a6)               | +01e
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +024
        bcs.w   RocketVehicle_RiderSpritesC_06c6b6__L06c6ca | +028
        move.b  d0,0x97(a6)                     | +02c
        andi.w  #0x3,d0                         | +030
        movea.l #0x6cc48,a0                     | +034
        lsl.w   #0x2,d0                         | +03a
        movea.l (a0,d0.w),a0                    | +03c
        cmpa.l  #0xffffffff,a0                  | +040
        beq.w   .L06cc44                        | +046
        jsr     0x28cd4.l                       | +04a
.L06cc44:
        bra.w   RocketVehicle_RiderSpritesL_06cc48__L06cc58 | +050

| ----------------------------------------------------------------------------
|  RocketVehicle_RiderSpritesL_06cc48  @ $06CC48  (20 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_RiderSpritesL_06cc48, "ax", @progbits
        .global RocketVehicle_RiderSpritesL_06cc48
RocketVehicle_RiderSpritesL_06cc48:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xfada                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xf886                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xf9b0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global RocketVehicle_RiderSpritesL_06cc48__L06cc58
RocketVehicle_RiderSpritesL_06cc48__L06cc58:
.L06cc58:
        bra.w   RocketVehicle_RiderCorpse_06cd62 | +010

| ----------------------------------------------------------------------------
|  RocketVehicle_RiderDie_06cc5c  @ $06CC5C  (58 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_RiderDie_06cc5c, "ax", @progbits
        .global RocketVehicle_RiderDie_06cc5c
RocketVehicle_RiderDie_06cc5c:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x8a(a0),0x8a(a6)               | +004
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +00a
        bcs.w   RocketVehicle_RiderSpritesC_06c6b6__L06c6ca | +00e
        move.b  d0,0x97(a6)                     | +012
        andi.w  #0x3,d0                         | +016
        movea.l #0x6cc96,a0                     | +01a
        lsl.w   #0x2,d0                         | +020
        movea.l (a0,d0.w),a0                    | +022
        cmpa.l  #0xffffffff,a0                  | +026
        beq.w   .L06cc92                        | +02c
        jsr     0x28cd4.l                       | +030
.L06cc92:
        bra.w   RocketVehicle_RiderSpritesM_06cc96__L06cca6 | +036

| ----------------------------------------------------------------------------
|  RocketVehicle_RiderSpritesM_06cc96  @ $06CC96  (176 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_RiderSpritesM_06cc96, "ax", @progbits
        .global RocketVehicle_RiderSpritesM_06cc96
RocketVehicle_RiderSpritesM_06cc96:
        .dc.w   0x002d                        | +000  (dato / opcode no decodificado)
        .dc.w   0x35f8                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002d                        | +004  (dato / opcode no decodificado)
        .dc.w   0x2ccc                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002d                        | +008  (dato / opcode no decodificado)
        .dc.w   0x3162                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global RocketVehicle_RiderSpritesM_06cc96__L06cca6
RocketVehicle_RiderSpritesM_06cc96__L06cca6:
.L06cca6:
        lea     .L06ccac(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L06ccac:
        movea.l 0xc(a6),a0                      | +016
        move.b  0x72(a0),d0                     | +01a
        cmp.b   0x72(a6),d0                     | +01e
        beq.w   .L06ccf0                        | +022
        cmpi.b  #0x7,0x72(a6)                   | +026
        beq.w   RocketVehicle_WeaponFlyOff_06cdba | +02c
        move.b  d0,0x72(a6)                     | +030
        cmpi.b  #0x9,d0                         | +034
        bcs.w   .L06ccde                        | +038
        nop                                     | +03c
        nop                                     | +03e
        cmpi.b  #0x9,d0                         | +040
        nop                                     | +044
        trap    #0xf                            | +046
.L06ccde:
        andi.w  #0xf,d0                         | +048
        lea     0x2cd606.l,a0                   | +04c
        lsl.w   #0x2,d0                         | +052
        movea.l (a0,d0.w),a1                    | +054
        jmp     (a1)                            | +058
.L06ccf0:
        move.b  d0,0x72(a6)                     | +05a
        jsr     RocketVehicle_RiderFollowParent_06d29c(pc) | +05e
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +062
        bcs.w   RocketVehicle_RiderSpritesC_06c6b6__L06c6ca | +066
        cmp.b   0x97(a6),d0                     | +06a
        beq.w   RocketVehicle_RiderSpritesN_06cd46__L06cd56 | +06e
        move.b  d0,0x97(a6)                     | +072
        movea.l 0xc(a6),a0                      | +076
        move.w  0x8a(a0),0x8a(a6)               | +07a
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +080
        bcs.w   RocketVehicle_RiderSpritesC_06c6b6__L06c6ca | +084
        move.b  d0,0x97(a6)                     | +088
        andi.w  #0x3,d0                         | +08c
        movea.l #0x6cd46,a0                     | +090
        lsl.w   #0x2,d0                         | +096
        movea.l (a0,d0.w),a0                    | +098
        cmpa.l  #0xffffffff,a0                  | +09c
        beq.w   .L06cd42                        | +0a2
        jsr     0x28cd4.l                       | +0a6
.L06cd42:
        bra.w   RocketVehicle_RiderSpritesN_06cd46__L06cd56 | +0ac

| ----------------------------------------------------------------------------
|  RocketVehicle_RiderSpritesN_06cd46  @ $06CD46  (20 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_RiderSpritesN_06cd46, "ax", @progbits
        .global RocketVehicle_RiderSpritesN_06cd46
RocketVehicle_RiderSpritesN_06cd46:
        .dc.w   0x002d                        | +000  (dato / opcode no decodificado)
        .dc.w   0x35f8                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002d                        | +004  (dato / opcode no decodificado)
        .dc.w   0x2ccc                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002d                        | +008  (dato / opcode no decodificado)
        .dc.w   0x3162                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global RocketVehicle_RiderSpritesN_06cd46__L06cd56
RocketVehicle_RiderSpritesN_06cd46__L06cd56:
.L06cd56:
        subq.w  #0x1,0x38(a6)                   | +010

| ----------------------------------------------------------------------------
|  RocketVehicle_RiderCorpse_06cd62  @ $06CD62  (70 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_RiderCorpse_06cd62, "ax", @progbits
        .global RocketVehicle_RiderCorpse_06cd62
RocketVehicle_RiderCorpse_06cd62:
        movea.l 0xc(a6),a0                      | +000
        tst.b   0x94(a0)                        | +004
        beq.w   RocketVehicle_RiderSpritesO_06cda8__L06cdb8 | +008
        movea.l 0xc(a6),a0                      | +00c
        move.w  0x8a(a0),0x8a(a6)               | +010
        jsr     RocketVehicle_WheelPhase_06d1dc(pc) | +016
        bcs.w   RocketVehicle_RiderSpritesC_06c6b6__L06c6ca | +01a
        move.b  d0,0x97(a6)                     | +01e
        andi.w  #0x3,d0                         | +022
        movea.l #0x6cda8,a0                     | +026
        lsl.w   #0x2,d0                         | +02c
        movea.l (a0,d0.w),a0                    | +02e
        cmpa.l  #0xffffffff,a0                  | +032
        beq.w   .L06cda4                        | +038
        jsr     0x28cd4.l                       | +03c
.L06cda4:
        bra.w   RocketVehicle_RiderSpritesO_06cda8__L06cdb8 | +042

| ----------------------------------------------------------------------------
|  RocketVehicle_RiderSpritesO_06cda8  @ $06CDA8  (18 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_RiderSpritesO_06cda8, "ax", @progbits
        .global RocketVehicle_RiderSpritesO_06cda8
RocketVehicle_RiderSpritesO_06cda8:
        .dc.w   0x002d                        | +000  (dato / opcode no decodificado)
        .dc.w   0x23aa                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002d                        | +004  (dato / opcode no decodificado)
        .dc.w   0x1166                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002d                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1a88                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global RocketVehicle_RiderSpritesO_06cda8__L06cdb8
RocketVehicle_RiderSpritesO_06cda8__L06cdb8:
.L06cdb8:
        rts                                     | +010

| ----------------------------------------------------------------------------
|  RocketVehicle_WeaponFlyOff_06cdba  @ $06CDBA  (134 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_WeaponFlyOff_06cdba, "ax", @progbits
        .global RocketVehicle_WeaponFlyOff_06cdba
RocketVehicle_WeaponFlyOff_06cdba:
        move.w  #0xffde,d0                      | +000
        jsr     0x5dca4.l                       | +004
        move.w  d0,0x28(a6)                     | +00a
        move.w  #0x663,0x2a(a6)                 | +00e
        move.w  #0xff93,0x2e(a6)                | +014
        move.w  #0x0,0x2c(a6)                   | +01a
        move.w  #0x4000,d0                      | +020
        jsr     0x28134.l                       | +024
        andi.w  #0xffe3,0x38(a6)                | +02a
        ori.w   #0x0,0x38(a6)                   | +030
        lea     .L06cdf6(pc),a1                 | +036
        move.l  a1,(a6)                         | +03a
.L06cdf6:
        move.w  0x34(a6),0x5c(a6)               | +03c
        jsr     0x27bc8.l                       | +042
        bcc.w   .L06ce0c                        | +048
        lea     RocketVehicle_WeaponExplode_06ce66(pc),a1 | +04c
        move.l  a1,(a6)                         | +050
.L06ce0c:
        move.w  0x5c(a6),0x34(a6)               | +052
        jsr     0x28d70.l                       | +058
        cmpi.w  #0xc0,0x24(a6)                  | +05e
        bgt.w   .L06ce28                        | +064
        lea     JmpToScheduler_06c54c(pc),a1    | +068
        move.l  a1,(a6)                         | +06c
.L06ce28:
        jsr     0x5e45a.l                       | +06e
        bcc.w   .L06ce38                        | +074
        lea     RocketVehicle_WeaponExplode_06ce66(pc),a1 | +078
        move.l  a1,(a6)                         | +07c
.L06ce38:
        jsr     RocketVehicle_OffWorldOrLeft_06d05c(pc) | +07e
        bcc.w   SetHandlerRts_06ce46            | +082

| ----------------------------------------------------------------------------
|  RocketVehicle_WeaponFall_06ce48  @ $06CE48  (22 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_WeaponFall_06ce48, "ax", @progbits
        .global RocketVehicle_WeaponFall_06ce48
RocketVehicle_WeaponFall_06ce48:
        lea     .L06ce4e(pc),a1                 | +000
        move.l  a1,(a6)                         | +004
.L06ce4e:
        jsr     0x2783a.l                       | +006
        jsr     0x28d70.l                       | +00c
        bcc.w   SetHandlerRts_06ce64            | +012

| ----------------------------------------------------------------------------
|  RocketVehicle_WeaponExplode_06ce66  @ $06CE66  (36 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_WeaponExplode_06ce66, "ax", @progbits
        .global RocketVehicle_WeaponExplode_06ce66
RocketVehicle_WeaponExplode_06ce66:
        move.w  #0x4000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x0,0x38(a6)                   | +010
        move.l  #0xffffffff,0x48(a6)            | +016
        jmp     0x77f6a.l                       | +01e

| ----------------------------------------------------------------------------
|  RocketVehicle_MuzzleFlash_06ce8a  @ $06CE8A  (124 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_MuzzleFlash_06ce8a, "ax", @progbits
        .global RocketVehicle_MuzzleFlash_06ce8a
RocketVehicle_MuzzleFlash_06ce8a:
        move.w  #0x4000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x1c,0x38(a6)                  | +010
        move.w  #0x8,d1                         | +016
        jsr     0x236e.l                        | +01a
        cmpi.b  #0x1,0x97(a6)                   | +020
        bne.w   .L06cede                        | +026
        move.w  0x34(a6),d0                     | +02a
        andi.w  #0x7,d0                         | +02e
        bra.w   .L06cede                        | +032
        move.w  0x34(a6),d0                     | +036
        lsr.w   #0x3,d0                         | +03a
        andi.w  #0x1,d0                         | +03c
        eor.b   d0,0x3a(a6)                     | +040
        lea     0x2dd6d2.l,a0                   | +044
        jsr     0x28cd4.l                       | +04a
        bra.w   .L06cf02                        | +050
.L06cede:
        move.b  0x8e(a6),d0                     | +054
        move.b  0x8f(a6),d1                     | +058
        ext.w   d0                              | +05c
        ext.w   d1                              | +05e
        add.w   d0,0x22(a6)                     | +060
        add.w   d1,0x24(a6)                     | +064
        subq.w  #0x8,0x24(a6)                   | +068
        lea     0x2dd6d2.l,a0                   | +06c
        jsr     0x28cd4.l                       | +072
.L06cf02:
        bra.w   RocketVehicle_WeaponFall_06ce48 | +078

| ----------------------------------------------------------------------------
|  RocketVehicle_Rocket_06cf06  @ $06CF06  (320 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_Rocket_06cf06, "ax", @progbits
        .global RocketVehicle_Rocket_06cf06
RocketVehicle_Rocket_06cf06:
        lea     0x2cd10a.l,a1                   | +000
        btst    #0x0,0x3a(a6)                   | +006
        beq.w   .L06cf1c                        | +00c
        lea     0x2cd11a.l,a1                   | +010
.L06cf1c:
        move.b  0x97(a6),d0                     | +016
        andi.w  #0x3,d0                         | +01a
        lsl.w   #0x2,d0                         | +01e
        movea.l (a1,d0.w),a1                    | +020
        move.b  0x98(a6),d0                     | +024
        andi.w  #0x7,d0                         | +028
        lsl.w   #0x2,d0                         | +02c
        movea.l (a1,d0.w),a1                    | +02e
        move.w  0x34(a6),d0                     | +032
        andi.w  #0xf,d0                         | +036
        move.w  d0,d1                           | +03a
        add.w   d0,d0                           | +03c
        add.w   d1,d0                           | +03e
        add.w   d0,d0                           | +040
        ext.l   d0                              | +042
        adda.l  d0,a1                           | +044
        move.w  (a1),0x28(a6)                   | +046
        move.w  0x2(a1),0x2e(a6)                | +04a
        move.w  0x4(a1),0x2a(a6)                | +050
        andi.b  #0xfe,0x3a(a6)                  | +056
        move.w  #0x173,d1                       | +05c
        jsr     0x236e.l                        | +060
        bset    #0x4,0x6b(a6)                   | +066
        lea     0x2d3a8e.l,a0                   | +06c
        jsr     0x28cd4.l                       | +072
        move.w  #0xd000,d0                      | +078
        jsr     0x28134.l                       | +07c
        andi.w  #0xffe3,0x38(a6)                | +082
        ori.w   #0x1c,0x38(a6)                  | +088
        clr.b   0x95(a6)                        | +08e
        move.w  0x34(a6),d0                     | +092
        andi.w  #0xf,d0                         | +096
        cmpi.w  #0x0,d0                         | +09a
        beq.w   .L06cfe6                        | +09e
        cmpi.w  #0x8,d0                         | +0a2
        bge.w   .L06cfe6                        | +0a6
        move.w  #0xd000,d0                      | +0aa
        jsr     0x28134.l                       | +0ae
        andi.w  #0xffe3,0x38(a6)                | +0b4
        ori.w   #0x8,0x38(a6)                   | +0ba
        move.b  #0x1,0x95(a6)                   | +0c0
        lea     .L06cfd2(pc),a1                 | +0c6
        move.l  a1,(a6)                         | +0ca
.L06cfd2:
        jsr     0x27bc8.l                       | +0cc
        bcc.w   .L06cfe2                        | +0d2
        lea     RocketVehicle_RocketHit_06d04e(pc),a1 | +0d6
        move.l  a1,(a6)                         | +0da
.L06cfe2:
        bra.w   .L06cffc                        | +0dc
.L06cfe6:
        lea     .L06cfec(pc),a1                 | +0e0
        move.l  a1,(a6)                         | +0e4
.L06cfec:
        jsr     0x27d50.l                       | +0e6
        bcc.w   .L06cffc                        | +0ec
        lea     RocketVehicle_RocketHit_06d04e(pc),a1 | +0f0
        move.l  a1,(a6)                         | +0f4
.L06cffc:
        move.w  0x28(a6),d0                     | +0f6
        move.w  0x2a(a6),d1                     | +0fa
        asr.w   #0x4,d0                         | +0fe
        asr.w   #0x4,d1                         | +100
        jsr     0x5e018.l                       | +102
        lsr.w   #0x3,d0                         | +108
        move.w  d0,0x34(a6)                     | +10a
        jsr     0x28d70.l                       | +10e
        jsr     0x283d8.l                       | +114
        btst    #0x1,0x13(a6)                   | +11a
        beq.w   .L06d030                        | +120
        lea     RocketVehicle_WeaponExplode_06ce66(pc),a1 | +124
        move.l  a1,(a6)                         | +128
.L06d030:
        movea.l #0xffffffff,a0                  | +12a
        lea     0x2ccf42.l,a0                   | +130
        jsr     0x5dd56.l                       | +136
        bcc.w   SetHandlerRts_06d04c            | +13c

| ----------------------------------------------------------------------------
|  RocketVehicle_RocketHit_06d04e  @ $06D04E  (14 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_RocketHit_06d04e, "ax", @progbits
        .global RocketVehicle_RocketHit_06d04e
RocketVehicle_RocketHit_06d04e:
        move.w  #0x1025,d0                      | +000
        jsr     0x2352.l                        | +004
        bra.w   RocketVehicle_WeaponExplode_06ce66 | +00a

| ----------------------------------------------------------------------------
|  RocketVehicle_OffWorldOrLeft_06d05c  @ $06D05C  (32 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_OffWorldOrLeft_06d05c, "ax", @progbits
        .global RocketVehicle_OffWorldOrLeft_06d05c
RocketVehicle_OffWorldOrLeft_06d05c:
        movea.l #0xffffffff,a0                  | +000
        lea     0x2ccf3a.l,a0                   | +006
        jsr     0x5dd5c.l                       | +00c
        bcs.w   SetXN_06d082                    | +012
        cmpi.w  #0xffdc,0x22(a6)                | +016
        blt.w   SetXN_06d082                    | +01c

| ----------------------------------------------------------------------------
|  RocketVehicle_Chain3Step_06d088  @ $06D088  (32 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_Chain3Step_06d088, "ax", @progbits
        .global RocketVehicle_Chain3Step_06d088
RocketVehicle_Chain3Step_06d088:
        jsr     0x30704.l                       | +000
        beq.w   .L06d098                        | +006
        jsr     0x281b0.l                       | +00a
.L06d098:
        jsr     0x2788c.l                       | +010
        jsr     0x3076a.l                       | +016
        beq.w   ClearXN_06d0ae                  | +01c

| ----------------------------------------------------------------------------
|  RocketVehicle_SlopeAngle_06d0b4  @ $06D0B4  (66 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_SlopeAngle_06d0b4, "ax", @progbits
        .global RocketVehicle_SlopeAngle_06d0b4
RocketVehicle_SlopeAngle_06d0b4:
        movea.l 0x74(a6),a1                     | +000
        movea.l 0x78(a6),a2                     | +004
        move.w  0x22(a1),d0                     | +008
        sub.w   0x22(a2),d0                     | +00c
        move.w  0x24(a1),d1                     | +010
        sub.w   0x24(a2),d1                     | +014
        move.w  0x24(a1),d2                     | +018
        add.w   0x24(a2),d2                     | +01c
        lsr.w   #0x1,d2                         | +020
        move.w  d2,0x24(a6)                     | +022
        jsr     0x5e018.l                       | +026
        move.w  d0,d1                           | +02c
        addq.w  #0x8,d1                         | +02e
        lsr.w   #0x4,d1                         | +030
        btst    #0x0,d1                         | +032
        bne.w   .L06d0f4                        | +036
        addq.w  #0x4,d0                         | +03a
        andi.w  #0xf8,d0                        | +03c
.L06d0f4:
        rts                                     | +040

| ----------------------------------------------------------------------------
|  RocketVehicle_GroundPhysics_06d0f6  @ $06D0F6  (58 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_GroundPhysics_06d0f6, "ax", @progbits
        .global RocketVehicle_GroundPhysics_06d0f6
RocketVehicle_GroundPhysics_06d0f6:
        btst    #0x6,0x13(a6)                   | +000
        bne.w   ClearXN_06d136                  | +006
        jsr     0x2785c.l                       | +00a
        jsr     0x2a8c0.l                       | +010
        bcc.w   .L06d120                        | +016
        move.w  #0xffc0,0x2e(a6)                | +01a
        jsr     0x27a18.l                       | +020
        bra.w   ClearXN_06d136                  | +026
.L06d120:
        jsr     RocketVehicle_Chain3Step_06d088(pc) | +02a
        jsr     RocketVehicle_SlopeAngle_06d0b4(pc) | +02e
        move.b  d0,0x89(a6)                     | +032
        jsr     RocketVehicle_SlopeToSpeed_06d1a4(pc) | +036

| ----------------------------------------------------------------------------
|  RocketVehicle_LinkBlocked_06d13c  @ $06D13C  (42 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_LinkBlocked_06d13c, "ax", @progbits
        .global RocketVehicle_LinkBlocked_06d13c
RocketVehicle_LinkBlocked_06d13c:
        btst    #0x0,0x8d(a6)                   | +000
        beq.w   .L06d158                        | +006
        movea.l 0x74(a6),a1                     | +00a
        btst    #0x5,0x5a(a1)                   | +00e
        bne.w   SetXN_06d16c                    | +014
        bra.w   ClearXN_06d166                  | +018
.L06d158:
        movea.l 0x78(a6),a1                     | +01c
        btst    #0x5,0x5a(a1)                   | +020
        bne.w   SetXN_06d16c                    | +026

| ----------------------------------------------------------------------------
|  RocketVehicle_WallAhead_06d172  @ $06D172  (38 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_WallAhead_06d172, "ax", @progbits
        .global RocketVehicle_WallAhead_06d172
RocketVehicle_WallAhead_06d172:
        cmpi.w  #0x0,0x28(a6)                   | +000
        beq.w   ClearXN_06d198                  | +006
        bgt.w   .L06d18e                        | +00a
        btst    #0x1,0x69(a6)                   | +00e
        bne.w   SetXN_06d19e                    | +014
        bra.w   ClearXN_06d198                  | +018
.L06d18e:
        btst    #0x0,0x69(a6)                   | +01c
        bne.w   SetXN_06d19e                    | +022

| ----------------------------------------------------------------------------
|  RocketVehicle_SlopeToSpeed_06d1a4  @ $06D1A4  (50 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_SlopeToSpeed_06d1a4, "ax", @progbits
        .global RocketVehicle_SlopeToSpeed_06d1a4
RocketVehicle_SlopeToSpeed_06d1a4:
        addi.b  #0x20,d0                        | +000
        cmpi.b  #0x0,d0                         | +004
        bge.w   .L06d1b2                        | +008
        clr.b   d0                              | +00c
.L06d1b2:
        andi.w  #0xff,d0                        | +00e
        cmpi.w  #0x40,d0                        | +012
        blt.w   .L06d1c2                        | +016
        move.w  #0x40,d0                        | +01a
.L06d1c2:
        btst    #0x0,0x3a(a6)                   | +01e
        bne.w   .L06d1d4                        | +024
        move.w  #0x40,d2                        | +028
        sub.w   d0,d2                           | +02c
        move.w  d2,d0                           | +02e
.L06d1d4:
        lsr.w   #0x2,d0                         | +030

| ----------------------------------------------------------------------------
|  RocketVehicle_WheelPhase_06d1dc  @ $06D1DC  (26 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_WheelPhase_06d1dc, "ax", @progbits
        .global RocketVehicle_WheelPhase_06d1dc
RocketVehicle_WheelPhase_06d1dc:
        move.w  0x8a(a6),d0                     | +000
        andi.w  #0x7,d0                         | +004
        bne.w   RocketVehicle_WheelPhase3_06d1fc | +008
        move.w  0x8a(a6),d0                     | +00c
        andi.w  #0x1f,d0                        | +010
        lsr.w   #0x3,d0                         | +014
        andi.w  #0x3,d0                         | +016

| ----------------------------------------------------------------------------
|  RocketVehicle_WheelPhase3_06d1fc  @ $06D1FC  (4 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_WheelPhase3_06d1fc, "ax", @progbits
        .global RocketVehicle_WheelPhase3_06d1fc
RocketVehicle_WheelPhase3_06d1fc:
        move.w  #0x3,d0                         | +000

| ----------------------------------------------------------------------------
|  RocketVehicle_FacingMismatch_06d206  @ $06D206  (22 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_FacingMismatch_06d206, "ax", @progbits
        .global RocketVehicle_FacingMismatch_06d206
RocketVehicle_FacingMismatch_06d206:
        move.b  0x8d(a6),d0                     | +000
        move.b  0x3a(a6),d1                     | +004
        andi.b  #0x1,d0                         | +008
        andi.b  #0x1,d1                         | +00c
        cmp.b   d0,d1                           | +010
        bne.w   SetXN_06d222                    | +012

| ----------------------------------------------------------------------------
|  RocketVehicle_AttackTimerInRange_06d228  @ $06D228  (104 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_AttackTimerInRange_06d228, "ax", @progbits
        .global RocketVehicle_AttackTimerInRange_06d228
RocketVehicle_AttackTimerInRange_06d228:
        subq.w  #0x1,0x70(a6)                   | +000
        cmpi.w  #0x0,0x70(a6)                   | +004
        bgt.w   ClearXN_06d296                  | +00a
        jsr     0x5e0d4.l                       | +00e
        move.w  0x24(a0),d0                     | +014
        sub.w   0x24(a6),d0                     | +018
        cmpi.w  #0x0,d0                         | +01c
        bgt.w   .L06d24e                        | +020
        neg.w   d0                              | +024
.L06d24e:
        cmpi.w  #0x80,d0                        | +026
        ble.w   .L06d25a                        | +02a
        move.w  #0x80,d0                        | +02e
.L06d25a:
        lsr.w   #0x5,d0                         | +032
        lea     0x2ccfc6.l,a1                   | +034
        move.b  0x98(a6),d1                     | +03a
        andi.w  #0x3,d1                         | +03e
        lsl.w   #0x2,d1                         | +042
        movea.l (a1,d1.w),a2                    | +044
        move.b  (a2,d0.w),d1                    | +048
        andi.w  #0xff,d1                        | +04c
        move.w  d1,d2                           | +050
        neg.w   d1                              | +052
        move.w  0x22(a0),d0                     | +054
        sub.w   0x22(a6),d0                     | +058
        cmp.w   d1,d0                           | +05c
        blt.w   ClearXN_06d296                  | +05e
        cmp.w   d2,d0                           | +062
        bgt.w   ClearXN_06d296                  | +064

| ----------------------------------------------------------------------------
|  RocketVehicle_RiderFollowParent_06d29c  @ $06D29C  (94 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_RiderFollowParent_06d29c, "ax", @progbits
        .global RocketVehicle_RiderFollowParent_06d29c
RocketVehicle_RiderFollowParent_06d29c:
        jsr     0x5e506.l                       | +000
        movea.l 0xc(a6),a0                      | +006
        move.w  0x8a(a0),0x8a(a6)               | +00a
        move.b  0x8e(a0),d0                     | +010
        ext.w   d0                              | +014
        btst    #0x0,0x3a(a6)                   | +016
        beq.w   .L06d2be                        | +01c
        neg.w   d0                              | +020
.L06d2be:
        add.w   d0,0x22(a6)                     | +022
        move.b  0x8f(a0),d0                     | +026
        ext.w   d0                              | +02a
        add.w   d0,0x24(a6)                     | +02c
        move.w  0x34(a6),d0                     | +030
        andi.w  #0xf,d0                         | +034
        beq.w   .L06d2e0                        | +038
        cmpi.w  #0x8,d0                         | +03c
        bcs.w   .L06d2e4                        | +040
.L06d2e0:
        subq.w  #0x1,0x38(a6)                   | +044
.L06d2e4:
        movea.l 0xc(a6),a0                      | +048
        btst    #0x7,0x5a(a0)                   | +04c
        beq.w   .L06d2f8                        | +052
        bset    #0x0,0x5a(a6)                   | +056
.L06d2f8:
        rts                                     | +05c

| ----------------------------------------------------------------------------
|  RocketVehicle_RangeClass_06d2fa  @ $06D2FA  (70 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_RangeClass_06d2fa, "ax", @progbits
        .global RocketVehicle_RangeClass_06d2fa
RocketVehicle_RangeClass_06d2fa:
        tst.b   0x9f(a6)                        | +000
        beq.w   .L06d33e                        | +004
        movea.l 0x90(a6),a0                     | +008
        move.w  0x22(a6),d0                     | +00c
        move.w  0x24(a6),d1                     | +010
        move.w  0x22(a0),d2                     | +014
        move.w  0x24(a0),d3                     | +018
        jsr     0x5e23a.l                       | +01c
        clr.b   d1                              | +022
        cmpi.w  #0x70,d0                        | +024
        blt.w   .L06d336                        | +028
        move.b  #0x1,d1                         | +02c
        cmpi.w  #0xb0,d0                        | +030
        blt.w   .L06d336                        | +034
        move.b  #0x2,d1                         | +038
.L06d336:
        andi.b  #0x3,d1                         | +03c
        move.b  d1,0x98(a6)                     | +040
.L06d33e:
        rts                                     | +044

| ----------------------------------------------------------------------------
|  RocketVehicle_AimStep_06d340  @ $06D340  (156 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_AimStep_06d340, "ax", @progbits
        .global RocketVehicle_AimStep_06d340
RocketVehicle_AimStep_06d340:
        movea.l 0x90(a6),a0                     | +000
        lea     0x2cd4ea.l,a1                   | +004
        btst    #0x0,0x3a(a6)                   | +00a
        beq.w   .L06d35a                        | +010
        lea     0x2cd50a.l,a1                   | +014
.L06d35a:
        move.b  0x97(a6),d1                     | +01a
        andi.w  #0x3,d1                         | +01e
        move.w  0x24(a0),d0                     | +022
        cmp.w   0x24(a6),d0                     | +026
        bgt.w   .L06d370                        | +02a
        addq.w  #0x4,d1                         | +02e
.L06d370:
        lsl.w   #0x2,d1                         | +030
        movea.l (a1,d1.w),a1                    | +032
        move.w  0x22(a0),d0                     | +036
        sub.w   0x22(a6),d0                     | +03a
        cmpi.w  #0xff00,d0                      | +03e
        bgt.w   .L06d38a                        | +042
        move.w  #0xff00,d0                      | +046
.L06d38a:
        cmpi.w  #0x100,d0                       | +04a
        blt.w   .L06d396                        | +04e
        move.w  #0x100,d0                       | +052
.L06d396:
        addi.w  #0x100,d0                       | +056
        lsr.w   #0x4,d0                         | +05a
        move.b  (a1,d0.w),d0                    | +05c
        btst    #0x0,0x3a(a6)                   | +060
        beq.w   .L06d3b4                        | +066
        lea     0x2cd5f6.l,a1                   | +06a
        move.b  (a1,d0.w),d0                    | +070
.L06d3b4:
        jsr     RocketVehicle_AimClampNear_06d3e8(pc) | +074
        andi.w  #0xf,d0                         | +078
        cmp.w   0x34(a6),d0                     | +07c
        beq.w   SetXN_06d3e2                    | +080
        sub.w   0x34(a6),d0                     | +084
        move.w  #0x1,d1                         | +088
        cmpi.w  #0x0,d0                         | +08c
        bgt.w   .L06d3d8                        | +090
        move.w  #0xffff,d1                      | +094
.L06d3d8:
        add.w   d1,0x34(a6)                     | +098

| ----------------------------------------------------------------------------
|  RocketVehicle_AimClampNear_06d3e8  @ $06D3E8  (78 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_AimClampNear_06d3e8, "ax", @progbits
        .global RocketVehicle_AimClampNear_06d3e8
RocketVehicle_AimClampNear_06d3e8:
        cmpi.b  #0x1,0x97(a6)                   | +000
        bne.w   ClearXN_06d440                  | +006
        move.b  d0,0x5c(a6)                     | +00a
        movea.l 0x90(a6),a0                     | +00e
        move.w  0x24(a0),d0                     | +012
        sub.w   0x24(a6),d0                     | +016
        cmpi.w  #0xfff8,d0                      | +01a
        blt.w   RocketVehicle_AimRestore_06d43c | +01e
        cmpi.w  #0x28,d0                        | +022
        bgt.w   RocketVehicle_AimRestore_06d43c | +026
        move.w  0x22(a0),d0                     | +02a
        sub.w   0x22(a6),d0                     | +02e
        btst    #0x0,0x3a(a6)                   | +032
        beq.w   .L06d426                        | +038
        neg.w   d0                              | +03c
.L06d426:
        clr.b   d1                              | +03e
        cmpi.w  #0x0,d0                         | +040
        bgt.w   .L06d434                        | +044
        move.b  #0x8,d1                         | +048
.L06d434:
        move.b  d1,d0                           | +04c

| ----------------------------------------------------------------------------
|  RocketVehicle_AimRestore_06d43c  @ $06D43C  (4 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_AimRestore_06d43c, "ax", @progbits
        .global RocketVehicle_AimRestore_06d43c
RocketVehicle_AimRestore_06d43c:
        move.b  0x5c(a6),d0                     | +000

| ----------------------------------------------------------------------------
|  RocketVehicle_SetFlags45_06d446  @ $06D446  (14 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_SetFlags45_06d446, "ax", @progbits
        .global RocketVehicle_SetFlags45_06d446
RocketVehicle_SetFlags45_06d446:
        bset    #0x5,0x13(a6)                   | +000
        bset    #0x4,0x13(a6)                   | +006
        rts                                     | +00c

| ----------------------------------------------------------------------------
|  RocketVehicle_RandFlags45_06d454  @ $06D454  (48 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_RandFlags45_06d454, "ax", @progbits
        .global RocketVehicle_RandFlags45_06d454
RocketVehicle_RandFlags45_06d454:
        jsr     0x5e9b6.l                       | +000
        bset    #0x4,0x13(a6)                   | +006
        btst    #0x0,d0                         | +00c
        beq.w   .L06d46e                        | +010
        bclr    #0x4,0x13(a6)                   | +014
.L06d46e:
        bset    #0x5,0x13(a6)                   | +01a
        btst    #0x1,d0                         | +020
        beq.w   .L06d482                        | +024
        bclr    #0x5,0x13(a6)                   | +028
.L06d482:
        rts                                     | +02e

| ----------------------------------------------------------------------------
|  RocketVehicle_Fire_06d484  @ $06D484  (62 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_Fire_06d484, "ax", @progbits
        .global RocketVehicle_Fire_06d484
RocketVehicle_Fire_06d484:
        lea     0x2ccfea.l,a1                   | +000
        lea     0x2ccffa.l,a2                   | +006
        lea     0x2cd00a.l,a3                   | +00c
        tst.b   0x9e(a6)                        | +012
        beq.w   .L06d4a4                        | +016
        lea     0x2cd01a.l,a3                   | +01a
.L06d4a4:
        move.b  0x97(a6),d0                     | +020
        andi.w  #0x3,d0                         | +024
        lsl.w   #0x2,d0                         | +028
        move.l  (a1,d0.w),0x48(a6)              | +02a
        move.l  (a2,d0.w),0x60(a6)              | +030
        move.l  (a3,d0.w),0x4c(a6)              | +036
        rts                                     | +03c

| ----------------------------------------------------------------------------
|  RocketVehicle_FireRocket_06d4c2  @ $06D4C2  (178 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_FireRocket_06d4c2, "ax", @progbits
        .global RocketVehicle_FireRocket_06d4c2
RocketVehicle_FireRocket_06d4c2:
        move.w  #0x1064,d0                      | +000
        jsr     0x2352.l                        | +004
        move.w  0x34(a6),d0                     | +00a
        btst    #0x0,0x3a(a6)                   | +00e
        beq.w   .L06d4e4                        | +014
        lea     0x2cd5f6.l,a1                   | +018
        move.b  (a1,d0.w),d0                    | +01e
.L06d4e4:
        andi.w  #0xf,d0                         | +022
        move.w  d0,0x5c(a6)                     | +026
        lea     RocketVehicle_MuzzleFlash_06ce8a(pc),a1 | +02a
        jsr     0x4ae.l                         | +02e
        jsr     0x5dd02.l                       | +034
        lea     0x2cd03a.l,a1                   | +03a
        move.b  0x97(a6),d0                     | +040
        andi.w  #0x3,d0                         | +044
        lsl.w   #0x2,d0                         | +048
        movea.l (a1,d0.w),a1                    | +04a
        move.w  0x34(a6),d0                     | +04e
        move.w  d0,0x34(a0)                     | +052
        lsl.w   #0x2,d0                         | +056
        move.w  (a1,d0.w),d1                    | +058
        move.w  0x2(a1,d0.w),d2                 | +05c
        btst    #0x0,0x3a(a6)                   | +060
        beq.w   .L06d52e                        | +066
        neg.w   d1                              | +06a
.L06d52e:
        move.w  d1,d3                           | +06c
        move.w  d2,d4                           | +06e
        move.b  d3,0x8e(a0)                     | +070
        move.b  d4,0x8f(a0)                     | +074
        move.b  0x97(a6),0x97(a0)               | +078
        movem.w d1-d2,-(a7)                     | +07e
        lea     RocketVehicle_Rocket_06cf06(pc),a1 | +082
        jsr     0x4ae.l                         | +086
        jsr     0x5dd02.l                       | +08c
        movem.w (a7)+,d1-d2                     | +092
        move.w  0x5c(a6),0x34(a0)               | +096
        add.w   d1,0x22(a0)                     | +09c
        add.w   d2,0x24(a0)                     | +0a0
        move.b  0x98(a6),0x98(a0)               | +0a4
        move.b  0x97(a6),0x97(a0)               | +0aa
        rts                                     | +0b0

| ----------------------------------------------------------------------------
|  RocketVehicle_PlayCry_06d574  @ $06D574  (32 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_PlayCry_06d574, "ax", @progbits
        .global RocketVehicle_PlayCry_06d574
RocketVehicle_PlayCry_06d574:
        lea     0x2ccf6e.l,a1                   | +000
        move.b  0x96(a6),d0                     | +006
        andi.w  #0x3,d0                         | +00a
        lsl.w   #0x1,d0                         | +00e
        move.w  (a1,d0.w),d1                    | +010
        jsr     0x236e.l                        | +014
        move.w  #0x7,0x1c(a6)                   | +01a

| ----------------------------------------------------------------------------
|  RocketVehicle_DeathDebris_06d59c  @ $06D59C  (48 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_DeathDebris_06d59c, "ax", @progbits
        .global RocketVehicle_DeathDebris_06d59c
RocketVehicle_DeathDebris_06d59c:
        move.w  #0x1033,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x2ccf4a.l,a1                   | +00a
        jsr     0x77c7e.l                       | +010
        move.w  #0x4000,0x38(a0)                | +016
        lea     0x2ccf5c.l,a1                   | +01c
        jsr     0x77c7e.l                       | +022
        move.w  #0x4000,0x38(a0)                | +028
        rts                                     | +02e

| ----------------------------------------------------------------------------
|  RocketVehicle_DeathExplodeA_06d5cc  @ $06D5CC  (54 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_DeathExplodeA_06d5cc, "ax", @progbits
        .global RocketVehicle_DeathExplodeA_06d5cc
RocketVehicle_DeathExplodeA_06d5cc:
        move.w  #0x1021,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x2ccf5c.l,a1                   | +00a
        jsr     0x77c7e.l                       | +010
        move.w  #0x4000,0x38(a0)                | +016
        lea     0x77fd6.l,a1                    | +01c
        jsr     0x4ae.l                         | +022
        jsr     0x5dd02.l                       | +028
        move.w  #0x4000,0x38(a0)                | +02e
        rts                                     | +034

| ----------------------------------------------------------------------------
|  RocketVehicle_DeathExplodeB_06d602  @ $06D602  (54 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_DeathExplodeB_06d602, "ax", @progbits
        .global RocketVehicle_DeathExplodeB_06d602
RocketVehicle_DeathExplodeB_06d602:
        move.w  #0x1021,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x2ccf5c.l,a1                   | +00a
        jsr     0x77c7e.l                       | +010
        move.w  #0x4000,0x38(a0)                | +016
        lea     0x77fd6.l,a1                    | +01c
        jsr     0x4ae.l                         | +022
        jsr     0x5dd02.l                       | +028
        move.w  #0x4000,0x38(a0)                | +02e
        rts                                     | +034

| ----------------------------------------------------------------------------
|  RocketVehicle_CmpSlotWithParent_06d638  @ $06D638  (16 B)
| ----------------------------------------------------------------------------
        .section .text.RocketVehicle_CmpSlotWithParent_06d638, "ax", @progbits
        .global RocketVehicle_CmpSlotWithParent_06d638
RocketVehicle_CmpSlotWithParent_06d638:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_06d64e                    | +00c

| ----------------------------------------------------------------------------
|  Walker_Tmpl42_06d654  @ $06D654  (154 B)
| ----------------------------------------------------------------------------
        .section .text.Walker_Tmpl42_06d654, "ax", @progbits
        .global Walker_Tmpl42_06d654
Walker_Tmpl42_06d654:
        clr.b   0x7e(a6)                        | +000
        clr.b   0x7f(a6)                        | +004
        bra.w   .L06d67a                        | +008
        move.b  #0x1,0x7e(a6)                   | +00c
        clr.b   0x7f(a6)                        | +012
        bra.w   .L06d67a                        | +016
        move.b  #0x1,0x7e(a6)                   | +01a
        move.b  #0x1,0x7f(a6)                   | +020
.L06d67a:
        jsr     Walker_Init_06d6ee(pc)          | +026
        clr.w   0x70(a6)                        | +02a
        bra.w   Walker_Init_06d6ee__L06d7a2     | +02e
        clr.b   0x7e(a6)                        | +032
        clr.b   0x7f(a6)                        | +036
        bra.w   .L06d6ac                        | +03a
        move.b  #0x1,0x7e(a6)                   | +03e
        clr.b   0x7f(a6)                        | +044
        bra.w   .L06d6ac                        | +048
        move.b  #0x1,0x7e(a6)                   | +04c
        move.b  #0x1,0x7f(a6)                   | +052
.L06d6ac:
        jsr     Walker_Init_06d6ee(pc)          | +058
        move.w  #0x1,0x70(a6)                   | +05c
        bra.w   Walker_Approach_06d808__L06d83a | +062
        clr.b   0x7e(a6)                        | +066
        clr.b   0x7f(a6)                        | +06a
        bra.w   .L06d6e0                        | +06e
        move.b  #0x1,0x7e(a6)                   | +072
        clr.b   0x7f(a6)                        | +078
        bra.w   .L06d6e0                        | +07c
        move.b  #0x1,0x7e(a6)                   | +080
        move.b  #0x1,0x7f(a6)                   | +086
.L06d6e0:
        jsr     Walker_Init_06d6ee(pc)          | +08c
        move.w  #0x2,0x70(a6)                   | +090
        bra.w   Walker_Approach_06d808__L06d83a | +096

| ----------------------------------------------------------------------------
|  Walker_Init_06d6ee  @ $06D6EE  (282 B)
| ----------------------------------------------------------------------------
        .section .text.Walker_Init_06d6ee, "ax", @progbits
        .global Walker_Init_06d6ee
Walker_Init_06d6ee:
        jsr     0x5e7c0.l                       | +000
        jsr     0x267e2.l                       | +006
        jsr     Walker_PlayCry_06e31e(pc)                | +00c  -> $06E31E (hueco futuro, defsym forward)
        move.w  #0x8,0x1c(a6)                   | +010
        jsr     0x138fe.l                       | +016
        lea     0x2b8d6c.l,a0                   | +01c
        jsr     0x799de.l                       | +022
        move.w  d0,0x66(a6)                     | +028
        move.w  0x66(a6),0x7a(a6)               | +02c
        move.l  #0x2d3bae,0x48(a6)              | +032
        move.l  #0x2d4056,0x60(a6)              | +03a
        move.b  #0x0,0x20(a6)                   | +042
        clr.w   0x76(a6)                        | +048
        move.b  0x9a(a6),d0                     | +04c
        tst.b   d0                              | +050
        bne.w   .L06d748                        | +052
        move.b  #0x15,d0                        | +056
.L06d748:
        andi.w  #0xff,d0                        | +05a
        lsl.w   #0x4,d0                         | +05e
        move.w  d0,0x86(a6)                     | +060
        move.w  #0x4000,d0                      | +064
        jsr     0x28134.l                       | +068
        andi.w  #0xffe3,0x38(a6)                | +06e
        ori.w   #0x14,0x38(a6)                  | +074
        tst.b   0x9d(a6)                        | +07a
        beq.w   .L06d786                        | +07e
        move.w  #0x8000,d0                      | +082
        jsr     0x28134.l                       | +086
        andi.w  #0xffe3,0x38(a6)                | +08c
        ori.w   #0x14,0x38(a6)                  | +092
.L06d786:
        jsr     Walker_SetSpriteByFlags7E7F_06e484(pc)                | +098  -> $06E484 (hueco futuro, defsym forward)
        lea     0x723d2.l,a1                    | +09c
        jsr     0x4ae.l                         | +0a2
        jsr     0x5dd02.l                       | +0a8
        clr.w   0x98(a0)                        | +0ae
        rts                                     | +0b2
        .global Walker_Init_06d6ee__L06d7a2
Walker_Init_06d6ee__L06d7a2:
.L06d7a2:
        clr.w   0x76(a6)                        | +0b4
        lea     0x2d428c.l,a0                   | +0b8
        jsr     0x28cd4.l                       | +0be
        lea     .L06d7b8(pc),a1                 | +0c4
        move.l  a1,(a6)                         | +0c8
.L06d7b8:
        jsr     0x28998.l                       | +0ca
        jsr     0x2783a.l                       | +0d0
        jsr     0x28d70.l                       | +0d6
        move.w  0x22(a6),d0                     | +0dc
        cmp.w   0x86(a6),d0                     | +0e0
        bge.w   .L06d7f6                        | +0e4
        jsr     0x5e0d4.l                       | +0e8
        bcs.w   .L06d7f6                        | +0ee
        move.w  0x22(a6),d0                     | +0f2
        sub.w   0x22(a0),d0                     | +0f6
        cmpi.w  #0x60,d0                        | +0fa
        bgt.w   .L06d7f6                        | +0fe
        lea     Walker_Approach_06d808(pc),a1   | +102
        move.l  a1,(a6)                         | +106
.L06d7f6:
        tst.w   0x76(a6)                        | +108
        beq.w   .L06d804                        | +10c
        lea     Walker_Approach_06d808(pc),a1   | +110
        move.l  a1,(a6)                         | +114
.L06d804:
        bra.w   Walker_Attack_06d926__L06d986   | +116

| ----------------------------------------------------------------------------
|  Walker_Approach_06d808  @ $06D808  (124 B)
| ----------------------------------------------------------------------------
        .section .text.Walker_Approach_06d808, "ax", @progbits
        .global Walker_Approach_06d808
Walker_Approach_06d808:
        lea     0x2d42b0.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L06d81a(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L06d81a:
        jsr     0x28998.l                       | +012
        jsr     0x2783a.l                       | +018
        jsr     0x28d70.l                       | +01e
        bcc.w   .L06d836                        | +024
        lea     Walker_AttackOnce_06d900(pc),a1 | +028
        move.l  a1,(a6)                         | +02c
.L06d836:
        bra.w   Walker_Attack_06d926__L06d986   | +02e
        .global Walker_Approach_06d808__L06d83a
Walker_Approach_06d808__L06d83a:
.L06d83a:
        lea     0x2d4298.l,a0                   | +032
        jsr     0x28cd4.l                       | +038
        lea     .L06d84c(pc),a1                 | +03e
        move.l  a1,(a6)                         | +042
.L06d84c:
        jsr     0x28998.l                       | +044
        jsr     0x2783a.l                       | +04a
        jsr     0x28d70.l                       | +050
        move.w  0x22(a6),d0                     | +056
        cmp.w   0x86(a6),d0                     | +05a
        bge.w   .L06d880                        | +05e
        lea     0x2d4036.l,a0                   | +062
        jsr     0x5e086.l                       | +068
        bcs.w   .L06d880                        | +06e
        lea     Walker_AttackOnce_06d900(pc),a1 | +072
        move.l  a1,(a6)                         | +076
.L06d880:
        bra.w   Walker_Attack_06d926__L06d986   | +078

| ----------------------------------------------------------------------------
|  Walker_AttackPause_06d884  @ $06D884  (124 B)
| ----------------------------------------------------------------------------
        .section .text.Walker_AttackPause_06d884, "ax", @progbits
        .global Walker_AttackPause_06d884
Walker_AttackPause_06d884:
        lea     0x2b8dee.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,0x72(a6)                     | +00c
        lea     0x2d42a4.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        lea     .L06d8a6(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L06d8a6:
        jsr     0x28998.l                       | +022
        jsr     0x2783a.l                       | +028
        jsr     0x28d70.l                       | +02e
        subq.w  #0x1,0x72(a6)                   | +034
        cmpi.w  #0x0,0x72(a6)                   | +038
        bgt.w   .L06d8fc                        | +03e
        cmpi.w  #0x0,0x70(a6)                   | +042
        bne.w   .L06d8d6                        | +048
        lea     Walker_AttackBurst_06d916(pc),a1 | +04c
        move.l  a1,(a6)                         | +050
.L06d8d6:
        cmpi.w  #0x2,0x70(a6)                   | +052
        bne.w   .L06d8e6                        | +058
        lea     Walker_AttackBurst_06d916(pc),a1 | +05c
        move.l  a1,(a6)                         | +060
.L06d8e6:
        lea     0x2d4036.l,a0                   | +062
        jsr     0x5e086.l                       | +068
        bcs.w   .L06d8fc                        | +06e
        lea     Walker_AttackBurst_06d916(pc),a1 | +072
        move.l  a1,(a6)                         | +076
.L06d8fc:
        bra.w   Walker_Attack_06d926__L06d986   | +078

| ----------------------------------------------------------------------------
|  Walker_AttackOnce_06d900  @ $06D900  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Walker_AttackOnce_06d900, "ax", @progbits
        .global Walker_AttackOnce_06d900
Walker_AttackOnce_06d900:
        move.w  #0x1,0x78(a6)                   | +000
        lea     0x2d42e8.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        bra.w   Walker_Attack_06d926__L06d932   | +012

| ----------------------------------------------------------------------------
|  Walker_AttackBurst_06d916  @ $06D916  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Walker_AttackBurst_06d916, "ax", @progbits
        .global Walker_AttackBurst_06d916
Walker_AttackBurst_06d916:
        lea     0x2b8ef2.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,0x78(a6)                     | +00c

| ----------------------------------------------------------------------------
|  Walker_Attack_06d926  @ $06D926  (190 B)
| ----------------------------------------------------------------------------
        .section .text.Walker_Attack_06d926, "ax", @progbits
        .global Walker_Attack_06d926
Walker_Attack_06d926:
        lea     0x2d431e.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        .global Walker_Attack_06d926__L06d932
Walker_Attack_06d926__L06d932:
.L06d932:
        lea     0x2b8e70.l,a0                   | +00c
        jsr     0x799de.l                       | +012
        move.w  d0,0x72(a6)                     | +018
        lea     .L06d948(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L06d948:
        jsr     0x28998.l                       | +022
        jsr     0x2783a.l                       | +028
        jsr     0x28d70.l                       | +02e
        bcc.w   .L06d986                        | +034
        subq.w  #0x1,0x72(a6)                   | +038
        cmpi.w  #0x0,0x72(a6)                   | +03c
        bgt.w   .L06d986                        | +042
        lea     Walker_Attack_06d926(pc),a1     | +046
        move.l  a1,(a6)                         | +04a
        subq.w  #0x1,0x78(a6)                   | +04c
        cmpi.w  #0x0,0x78(a6)                   | +050
        bgt.w   .L06d986                        | +056
        lea     Walker_AttackPause_06d884(pc),a1 | +05a
        move.l  a1,(a6)                         | +05e
        .global Walker_Attack_06d926__L06d986
Walker_Attack_06d926__L06d986:
.L06d986:
        jsr     Walker_ClearFlag10E39A_06e394(pc)                | +060  -> $06E394 (hueco futuro, defsym forward)
        jsr     0x2870a.l                       | +064
        bcc.w   .L06d9a6                        | +06a
        bclr    #0x3,0x13(a6)                   | +06e
        lea     0x5e766.l,a0                    | +074
        jsr     0x5e770.l                       | +07a
.L06d9a6:
        jsr     0x28758.l                       | +080
        bcc.w   .L06d9b6                        | +086
        lea     Walker_Die_06d9ec(pc),a1        | +08a
        move.l  a1,(a6)                         | +08e
.L06d9b6:
        tst.b   0x99(a6)                        | +090
        beq.w   .L06d9ce                        | +094
        jsr     0x27eba.l                       | +098
        bcc.w   .L06d9ce                        | +09e
        lea     Walker_Explode_06da4c(pc),a1    | +0a2
        move.l  a1,(a6)                         | +0a6
.L06d9ce:
        movea.l #0xffffffff,a0                  | +0a8
        lea     0x2d402e.l,a0                   | +0ae
        jsr     0x5dd5c.l                       | +0b4
        bcc.w   SetHandlerRts_06d9ea            | +0ba

| ----------------------------------------------------------------------------
|  Walker_Die_06d9ec  @ $06D9EC  (88 B)
| ----------------------------------------------------------------------------
        .section .text.Walker_Die_06d9ec, "ax", @progbits
        .global Walker_Die_06d9ec
Walker_Die_06d9ec:
        bclr    #0x1,0x12(a6)                   | +000
        jsr     0x267e2.l                       | +006
        lea     0x2d4394.l,a0                   | +00c
        jsr     0x28cd4.l                       | +012
        lea     .L06da0a(pc),a1                 | +018
        move.l  a1,(a6)                         | +01c
.L06da0a:
        jsr     0x2783a.l                       | +01e
        jsr     0x28d70.l                       | +024
        tst.b   0x99(a6)                        | +02a
        beq.w   .L06da2e                        | +02e
        jsr     0x27eba.l                       | +032
        bcc.w   .L06da2e                        | +038
        lea     Walker_Explode_06da4c(pc),a1    | +03c
        move.l  a1,(a6)                         | +040
.L06da2e:
        movea.l #0xffffffff,a0                  | +042
        lea     0x2d402e.l,a0                   | +048
        jsr     0x5dd5c.l                       | +04e
        bcc.w   SetHandlerRts_06da4a            | +054

| ----------------------------------------------------------------------------
|  Walker_Explode_06da4c  @ $06DA4C  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Walker_Explode_06da4c, "ax", @progbits
        .global Walker_Explode_06da4c
Walker_Explode_06da4c:
        move.w  #0x1033,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x77fd6.l,a1                    | +00a
        jsr     0x4ae.l                         | +010
        jsr     0x5dd02.l                       | +016
        move.w  #0x8000,0x38(a0)                | +01c
        jsr     Entity_SpawnLoop16_06E412(pc)   | +022
        jmp     0x518.l                         | +026

| ----------------------------------------------------------------------------
|  Walker_Nop_06da78  @ $06DA78  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Walker_Nop_06da78, "ax", @progbits
        .global Walker_Nop_06da78
Walker_Nop_06da78:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  Frag_ExplodeB_06da9e  @ $06DA9E  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Frag_ExplodeB_06da9e, "ax", @progbits
        .global Frag_ExplodeB_06da9e
Frag_ExplodeB_06da9e:
        move.w  #0x4000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x1c,0x38(a6)                  | +010
        move.l  #0xffffffff,0x48(a6)            | +016
        jmp     0x77f6a.l                       | +01e

| ----------------------------------------------------------------------------
|  Frag_ExplodeC_06dac2  @ $06DAC2  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Frag_ExplodeC_06dac2, "ax", @progbits
        .global Frag_ExplodeC_06dac2
Frag_ExplodeC_06dac2:
        jsr     0x13600.l                       | +000
        move.w  #0x4000,d0                      | +006
        jsr     0x28134.l                       | +00a
        andi.w  #0xffe3,0x38(a6)                | +010
        ori.w   #0x1c,0x38(a6)                  | +016
        move.l  #0xffffffff,0x48(a6)            | +01c
        jmp     0x77fd6.l                       | +024

| ----------------------------------------------------------------------------
|  Frag_Shell_06daec  @ $06DAEC  (224 B)
| ----------------------------------------------------------------------------
        .section .text.Frag_Shell_06daec, "ax", @progbits
        .global Frag_Shell_06daec
Frag_Shell_06daec:
        jsr     0x13600.l                       | +000
        move.w  #0x4000,d0                      | +006
        jsr     0x28134.l                       | +00a
        andi.w  #0xffe3,0x38(a6)                | +010
        ori.w   #0x0,0x38(a6)                   | +016
        move.l  #0xffffffff,0x48(a6)            | +01c
        jmp     0x77fd6.l                       | +024
        .global Frag_Shell_06daec__L06db16
Frag_Shell_06daec__L06db16:
.L06db16:
        tst.b   0x89(a6)                        | +02a
        bne.b   Frag_Shell_06daec               | +02e
        jsr     0x13600.l                       | +030
        move.w  #0x4000,d0                      | +036
        jsr     0x28134.l                       | +03a
        andi.w  #0xffe3,0x38(a6)                | +040
        ori.w   #0x1c,0x38(a6)                  | +046
        move.w  #0x1021,d0                      | +04c
        jsr     0x2352.l                        | +050
        lea     0x2d40b4.l,a1                   | +056
        jsr     0x77c7e.l                       | +05c
        move.w  #0xc000,0x38(a0)                | +062
        move.w  #0x4,d1                         | +068
        jsr     0x236e.l                        | +06c
        move.w  #0x2,0x72(a6)                   | +072
        move.l  #0x2d3f8a,0x4c(a6)              | +078
        jsr     0x283ca.l                       | +080
        lea     0x2dd37e.l,a0                   | +086
        jsr     0x28cd4.l                       | +08c
        lea     .L06db84(pc),a1                 | +092
        move.l  a1,(a6)                         | +096
.L06db84:
        jsr     0x2783a.l                       | +098
        jsr     0x28d70.l                       | +09e
        bcc.w   .L06db9a                        | +0a4
        lea     Jsr5B6ThenJmpScheduler_06da90(pc),a1 | +0a8
        move.l  a1,(a6)                         | +0ac
.L06db9a:
        jsr     0x283d8.l                       | +0ae
        btst    #0x1,0x13(a6)                   | +0b4
        subq.w  #0x1,0x72(a6)                   | +0ba
        cmpi.w  #0x0,0x72(a6)                   | +0be
        bgt.w   .L06dbbc                        | +0c4
        move.l  #0xffffffff,0x4c(a6)            | +0c8
.L06dbbc:
        movea.l #0xffffffff,a0                  | +0d0
        jsr     0x5dd56.l                       | +0d6
        bcc.w   SetHandlerRts_06dbd2            | +0dc

| ----------------------------------------------------------------------------
|  Frag_Debris_06dbd4  @ $06DBD4  (130 B)
| ----------------------------------------------------------------------------
        .section .text.Frag_Debris_06dbd4, "ax", @progbits
        .global Frag_Debris_06dbd4
Frag_Debris_06dbd4:
        move.b  #0x1,0x89(a6)                   | +000
        bra.w   .L06dbe2                        | +006
        .global Frag_Debris_06dbd4__L06dbde
Frag_Debris_06dbd4__L06dbde:
        jsr     Walker_PickBurstVel_06e176(pc)                | +00a  -> $06E176 (hueco futuro, defsym forward)
.L06dbe2:
        bset    #0x4,0x6b(a6)                   | +00e
        jsr     Frag_PlaySnd157To159_06e356(pc)                | +014  -> $06E356 (hueco futuro, defsym forward)
        move.w  #0xd000,d0                      | +018
        jsr     0x28134.l                       | +01c
        andi.w  #0xffe3,0x38(a6)                | +022
        ori.w   #0x14,0x38(a6)                  | +028
        lea     0x2d45fc.l,a0                   | +02e
        jsr     0x28cd4.l                       | +034
        lea     .L06dc14(pc),a1                 | +03a
        move.l  a1,(a6)                         | +03e
.L06dc14:
        jsr     0x27d50.l                       | +040
        bcc.w   .L06dc24                        | +046
        lea     Frag_Shell_06daec__L06db16(pc),a1 | +04a
        move.l  a1,(a6)                         | +04e
.L06dc24:
        jsr     0x28d70.l                       | +050
        jsr     0x283d8.l                       | +056
        btst    #0x1,0x13(a6)                   | +05c
        beq.w   .L06dc40                        | +062
        lea     Frag_ExplodeC_06dac2(pc),a1     | +066
        move.l  a1,(a6)                         | +06a
.L06dc40:
        movea.l #0xffffffff,a0                  | +06c
        lea     0x2d404e.l,a0                   | +072
        jsr     0x5dd56.l                       | +078
        bcc.w   SetHandlerRts_06dc5c            | +07e

| ----------------------------------------------------------------------------
|  Frag_Smoke_06dc5e  @ $06DC5E  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Frag_Smoke_06dc5e, "ax", @progbits
        .global Frag_Smoke_06dc5e
Frag_Smoke_06dc5e:
        jsr     Frag_PlaySnd157To159_06e356(pc)                | +000  -> $06E356 (hueco futuro, defsym forward)
        lea     0x2d4628.l,a0                   | +004
        jsr     0x28cd4.l                       | +00a
        lea     .L06dc74(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L06dc74:
        jsr     Entity_CopyParentAnimTimer_06e34a(pc)                | +016  -> $06E34A (hueco futuro, defsym forward)
        jsr     0x2783a.l                       | +01a
        jsr     0x28d70.l                       | +020
        bcc.w   SetHandlerRts_06dc8e            | +026

| ----------------------------------------------------------------------------
|  Frag_Spark_06dc90  @ $06DC90  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Frag_Spark_06dc90, "ax", @progbits
        .global Frag_Spark_06dc90
Frag_Spark_06dc90:
        eori.b  #0x1,0x3a(a6)                   | +000
        move.w  #0x8,d1                         | +006
        jsr     0x236e.l                        | +00a
        move.w  #0x4000,d0                      | +010
        jsr     0x28134.l                       | +014
        andi.w  #0xffe3,0x38(a6)                | +01a
        ori.w   #0x0,0x38(a6)                   | +020
        lea     0x2d455a.l,a0                   | +026
        jsr     0x28cd4.l                       | +02c
        lea     .L06dcc8(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L06dcc8:
        jsr     0x2783a.l                       | +038
        jsr     0x28d70.l                       | +03e
        bcc.w   SetHandlerRts_06dcde            | +044

| ----------------------------------------------------------------------------
|  Frag_ScatterSlow_06dce0  @ $06DCE0  (124 B)
| ----------------------------------------------------------------------------
        .section .text.Frag_ScatterSlow_06dce0, "ax", @progbits
        .global Frag_ScatterSlow_06dce0
Frag_ScatterSlow_06dce0:
        cmpi.w  #0x0,0x36(a6)                   | +000
        bgt.w   .L06dcf0                        | +006
        move.w  #0x200,0x36(a6)                 | +00a
.L06dcf0:
        move.w  #0x1,0x80(a6)                   | +010
        move.w  #0x1,0x82(a6)                   | +016
        move.w  #0x8000,d0                      | +01c
        jsr     0x28134.l                       | +020
        andi.w  #0xffe3,0x38(a6)                | +026
        ori.w   #0x14,0x38(a6)                  | +02c
        bra.w   Frag_Scatter_06dd5c__L06dde4    | +032
        cmpi.w  #0x0,0x36(a6)                   | +036
        bgt.w   .L06dd26                        | +03c
        move.w  #0x200,0x36(a6)                 | +040
.L06dd26:
        move.w  #0x8000,d0                      | +046
        jsr     0x28134.l                       | +04a
        andi.w  #0xffe3,0x38(a6)                | +050
        ori.w   #0x14,0x38(a6)                  | +056
        move.w  #0x1,0x80(a6)                   | +05c
        move.w  #0x1,0x82(a6)                   | +062
        jsr     0x5e9b6.l                       | +068
        move.w  d0,d3                           | +06e
        andi.w  #0x3e,d0                        | +070
        addi.w  #0x50,d0                        | +074
        bra.w   Frag_Scatter_06dd5c__L06ddf4    | +078

| ----------------------------------------------------------------------------
|  Frag_Scatter_06dd5c  @ $06DD5C  (418 B)
| ----------------------------------------------------------------------------
        .section .text.Frag_Scatter_06dd5c, "ax", @progbits
        .global Frag_Scatter_06dd5c
Frag_Scatter_06dd5c:
        jsr     0x5e9b6.l                       | +000
        andi.w  #0xf,d0                         | +006
        movea.l #0x2d4106,a0                    | +00a
        lsl.w   #0x2,d0                         | +010
        movea.l (a0,d0.w),a0                    | +012
        cmpa.l  #0xffffffff,a0                  | +016
        beq.w   .L06dd82                        | +01c
        jsr     0x28cd4.l                       | +020
.L06dd82:
        bra.w   .L06ddac                        | +026
        .global Frag_Scatter_06dd5c__L06dd86
Frag_Scatter_06dd5c__L06dd86:
        jsr     0x5e9b6.l                       | +02a
        andi.w  #0xf,d0                         | +030
        movea.l #0x2d4146,a0                    | +034
        lsl.w   #0x2,d0                         | +03a
        movea.l (a0,d0.w),a0                    | +03c
        cmpa.l  #0xffffffff,a0                  | +040
        beq.w   .L06ddac                        | +046
        jsr     0x28cd4.l                       | +04a
.L06ddac:
        move.w  #0x4000,d0                      | +050
        jsr     0x28134.l                       | +054
        andi.w  #0xffe3,0x38(a6)                | +05a
        ori.w   #0x0,0x38(a6)                   | +060
        jsr     0x5e9b6.l                       | +066
        andi.w  #0x700,d0                       | +06c
        addi.w  #0x200,d0                       | +070
        move.w  d0,0x36(a6)                     | +074
        move.w  #0x3,0x80(a6)                   | +078
        move.w  #0x3,0x82(a6)                   | +07e
        jsr     Walker_PlayCry_06e31e(pc)                | +084  -> $06E31E (hueco futuro, defsym forward)
        .global Frag_Scatter_06dd5c__L06dde4
Frag_Scatter_06dd5c__L06dde4:
.L06dde4:
        jsr     0x5e9b6.l                       | +088
        move.w  d0,d3                           | +08e
        andi.w  #0x3e,d0                        | +090
        addi.w  #0x20,d0                        | +094
        .global Frag_Scatter_06dd5c__L06ddf4
Frag_Scatter_06dd5c__L06ddf4:
.L06ddf4:
        add.w   d0,d0                           | +098
        lea     0x2c072c.l,a1                   | +09a
        move.w  (a1,d0.w),d1                    | +0a0
        lea     0x2c07ac.l,a1                   | +0a4
        move.w  (a1,d0.w),d2                    | +0aa
        andi.w  #0x7,d3                         | +0ae
        addq.w  #0x1,d3                         | +0b2
        muls.w  d3,d1                           | +0b4
        asr.w   #0x2,d1                         | +0b6
        move.w  0x36(a6),d0                     | +0b8
        muls.w  d0,d1                           | +0bc
        muls.w  d0,d2                           | +0be
        asr.l   #0x8,d1                         | +0c0
        asr.l   #0x8,d2                         | +0c2
        move.w  d1,0x2a(a6)                     | +0c4
        move.w  d2,0x28(a6)                     | +0c8
        jsr     0x5e9b6.l                       | +0cc
        cmpi.b  #0x0,d0                         | +0d2
        bgt.w   .L06de66                        | +0d6
        jsr     0x5e9b6.l                       | +0da
        andi.w  #0x3,d0                         | +0e0
        addq.w  #0x2,d0                         | +0e4
        move.w  0x28(a6),d1                     | +0e6
        muls.w  d1,d0                           | +0ea
        asr.w   #0x1,d0                         | +0ec
        move.w  d0,0x28(a6)                     | +0ee
        jsr     0x5e9b6.l                       | +0f2
        andi.w  #0x3,d0                         | +0f8
        addq.w  #0x2,d0                         | +0fc
        move.w  0x2a(a6),d1                     | +0fe
        muls.w  d1,d0                           | +102
        asr.w   #0x1,d0                         | +104
        move.w  d0,0x2a(a6)                     | +106
.L06de66:
        move.w  0x2a(a6),d0                     | +10a
        move.w  d0,0x84(a6)                     | +10e
        move.w  #0xffd0,0x2e(a6)                | +112
        lea     .L06de7a(pc),a1                 | +118
        move.l  a1,(a6)                         | +11c
.L06de7a:
        jsr     0x27d50.l                       | +11e
        bcc.w   .L06deb6                        | +124
        move.w  #0xffd0,0x2e(a6)                | +128
        move.w  0x28(a6),d0                     | +12e
        asr.w   #0x2,d0                         | +132
        move.w  d0,0x28(a6)                     | +134
        move.w  0x84(a6),d0                     | +138
        asr.w   #0x1,d0                         | +13c
        move.w  d0,0x2a(a6)                     | +13e
        move.w  d0,0x84(a6)                     | +142
        subq.w  #0x1,0x80(a6)                   | +146
        cmpi.w  #0x0,0x80(a6)                   | +14a
        bgt.w   .L06deb6                        | +150
        lea     Jsr5B6ThenJmpScheduler_06da90(pc),a1 | +154
        move.l  a1,(a6)                         | +158
.L06deb6:
        move.w  0x82(a6),d0                     | +15a
        cmp.w   0x80(a6),d0                     | +15e
        beq.w   .L06ded0                        | +162
        addq.b  #0x1,0x72(a6)                   | +166
        btst    #0x0,0x72(a6)                   | +16a
        beq.w   .L06dee8                        | +170
.L06ded0:
        jsr     0x5e804.l                       | +174
        bcc.w   .L06dee2                        | +17a
        andi.b  #0xee,ccr                       | +17e
        bra.w   .L06dee8                        | +182
.L06dee2:
        jsr     0x28d70.l                       | +186
.L06dee8:
        movea.l #0xffffffff,a0                  | +18c
        lea     0x2d403e.l,a0                   | +192
        jsr     0x5dd56.l                       | +198
        bcc.w   SetHandlerRts_06df04            | +19e

| ----------------------------------------------------------------------------
|  FireBurst_Wide_06df06  @ $06DF06  (44 B)
| ----------------------------------------------------------------------------
        .section .text.FireBurst_Wide_06df06, "ax", @progbits
        .global FireBurst_Wide_06df06
FireBurst_Wide_06df06:
        jsr     0x5e9b6.l                       | +000
        andi.w  #0xf00,d0                       | +006
        addi.w  #0x800,d0                       | +00a
        jsr     Facing_NegIfLeft_06e20c(pc)                | +00e  -> $06E20C (hueco futuro, defsym forward)
        move.w  d0,0x28(a6)                     | +012
        jsr     0x5e9b6.l                       | +016
        andi.w  #0xf00,d0                       | +01c
        addi.w  #0x800,d0                       | +020
        move.w  d0,0x2a(a6)                     | +024
        bra.w   FireBurst_Tmpl_06df32__L06df5a  | +028

| ----------------------------------------------------------------------------
|  FireBurst_Tmpl_06df32  @ $06DF32  (182 B)
| ----------------------------------------------------------------------------
        .section .text.FireBurst_Tmpl_06df32, "ax", @progbits
        .global FireBurst_Tmpl_06df32
FireBurst_Tmpl_06df32:
        jsr     0x5e9b6.l                       | +000
        andi.w  #0x700,d0                       | +006
        addi.w  #0x300,d0                       | +00a
        jsr     Facing_NegIfLeft_06e20c(pc)                | +00e  -> $06E20C (hueco futuro, defsym forward)
        move.w  d0,0x28(a6)                     | +012
        jsr     0x5e9b6.l                       | +016
        andi.w  #0x700,d0                       | +01c
        addi.w  #0x500,d0                       | +020
        move.w  d0,0x2a(a6)                     | +024
        .global FireBurst_Tmpl_06df32__L06df5a
FireBurst_Tmpl_06df32__L06df5a:
.L06df5a:
        bset    #0x4,0x6b(a6)                   | +028
        move.w  #0x108e,d0                      | +02e
        jsr     0x2352.l                        | +032
        lea     0x2d4666.l,a0                   | +038
        jsr     0x28cd4.l                       | +03e
        move.l  #0xffffffff,0x48(a6)            | +044
        move.l  #0x2d3e42,0x4c(a6)              | +04c
        move.w  #0x2,d1                         | +054
        jsr     0x236e.l                        | +058
        clr.w   0x72(a6)                        | +05e
        jsr     0x283ca.l                       | +062
        move.w  #0xd000,d0                      | +068
        jsr     0x28134.l                       | +06c
        andi.w  #0xffe3,0x38(a6)                | +072
        ori.w   #0x14,0x38(a6)                  | +078
        lea     .L06dfb6(pc),a1                 | +07e
        move.l  a1,(a6)                         | +082
.L06dfb6:
        jsr     0x27cee.l                       | +084
        bcc.w   .L06dfc8                        | +08a
        lea     0x31d26.l,a1                    | +08e
        move.l  a1,(a6)                         | +094
.L06dfc8:
        jsr     0x28d70.l                       | +096
        jsr     FireBurst_TickHit_06e2fe(pc)                | +09c  -> $06E2FE (hueco futuro, defsym forward)
        btst    #0x1,0x13(a6)                   | +0a0
        beq.w   .L06dfe4                        | +0a6
        lea     0x31d26.l,a1                    | +0aa
        move.l  a1,(a6)                         | +0b0
.L06dfe4:
        bra.w   FireBurst_FreeIfOffWorld_06e15e                    | +0b2  -> $06E15E (hueco futuro, defsym forward)
