| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave OOOO — vehículo-torreta enemigo (base, cañón giratorio con
|              histórico de ángulos, conductor, proyectil) y 3ª tanda de
|              props destructibles de misión (farola, choza, torre,
|              búnker, puente, nido, cobertizo, barrera)
|  Región: $04CBD4..$04E580  (6,384 B, 64 entradas, 22 huecos)
| ============================================================================
|
|  A. RESUMEN
|  ----------
|  Región de 6,384 B (64 entradas, todo código) con dos módulos:
|  I. VEHÍCULO-TORRETA ($4CBD4..$4D6EC, TurretCar_*): entidad compuesta
|  cuyo estado se comparte con los hijos por +$20 ($44 = base viva, $88 =
|  variante "jefe"), +$21 ($FF = destruida), +$34 = ángulo 0..31 de la
|  torreta, +$75 (hijo conductor: $FF = ha huido, $80 = en marcha), +$7B
|  ($FF = retroceso), +$7C ($FF = cuerpo destruido), +$7D ($FF = cañón
|  vivo), +$80 (fin de idle).
|   1. TurretCar_Spawn_04cbd4 (2 entradas: la 1ª pone +$20 = $44, +$80 =
|      $FF, +$98 = 1; la 2ª espera a X <= $140): snd $E, prio $8000,
|      +$7D = $FF, relink, hijo TurretCar_Body_04cf2e; cae en
|      TurretCar_Idle_04cc46: ángulo objetivo +$78 = $1F o $11 según bit3
|      de +$34, sprite $290F6E, cada 8 (o 2 si +$98) frames gira 1 paso
|      hacia +$78 (AngleStep16), anim; si +$21 == $FF -> destruida (si
|      +$20 == $88 incrementa +$21 del padre, +$7C = $FF, handler
|      Soldier $58F82 "salida B", sombra $776E2, offsets B, limpia padre);
|      al llegar al ángulo: si +$75 == $FF -> Track, si no crea el
|      conductor TurretCar_Driver_04d048 (con $6FE) y +$75 = $80;
|      IsBaseIdleDone / OffworldA -> free.
|   2. TurretCar_Track_04cd80: +$72 = temporizador por dificultad
|      [$2B791E]; gira hacia el ángulo al objetivo (AngleToTargetMirror ->
|      SpriteIdxByAngle $2941C4) y al expirar con el cañón vivo (+$7D)
|      -> +$7B = $FF y TurretCar_Recoil_04ce62 (50 frames de sprite
|      $29BFC4 con offsets B, facing por bit3 del ángulo) -> Idle.
|   3. TurretCar_Body_04cf2e (hijo): snd $22, HP por dificultad [$2B7798],
|      sprite $2912D6 ($291526 en retroceso), sigue pos/prio del padre,
|      copia su ángulo; $2870A impacto -> flash; HP agotado -> música
|      $1023, padre +$21 = $FF, escombros $293EB4/$293EC6 y, si el padre
|      es "jefe" ($88), StateMachineRun $5022A[$295682]; free.
|   4. TurretCar_Driver_04d048 (hijo): snd $23, HP [$2B789C], ataque +$4C
|      = $293ED8, sprite $29276A; al acabar la anim marca al padre +$75 =
|      $FF -> DriverPanic_04d0e2 (sprite $292A48, 300 frames; si el padre
|      retrocede -> DriverFlee_04d13a: música $10A3, sprite $292C92 ->
|      TurretCar_Cannon_04d186).
|   5. TurretCar_Cannon_04d186: prio +2, vel base +$36 [$2B781A], sprite
|      $2927F8, histórico de 16 ángulos en +$88..+$97 (ShiftAngleHistory
|      desplaza y añade +$34); cada 16 frames recalcula el ángulo objetivo
|      (AngleToTarget $5E136; si Y >= $1C0 fija +$7A y sube; GroundProbeUp
|      $280C6 7 pasos de 8 px -> ángulo 8 si hay suelo), cada 32 fija el
|      sentido +$76, cada 4 gira; velocidad desde el ángulo de hace 16
|      frames ($13C0E, grav -$60 si bit4), ProbeGuided $27BC8; cada 8
|      frames dispara TurretCar_Shell_04d340 (hijo con $4AE o $6FE según
|      el ángulo, AddMuzzleOffset $294044, ángulo del histórico);
|      $283D8, impacto/HP agotado o 600 frames -> explosión $77F6A prio
|      $4000, escombros $293FF2, música $1022, padre +$7D = $FF, free.
|      TurretCar_Shell_04d340: sprite $2911BC, colisión $293FE8, snd $D,
|      vel +$36 = $80 con jitter RNG $5E9E4 +-2 del ángulo, VelFromAngle,
|      $27CEE; offworld/anim -> free.
|   6. Helpers: SyncPosIfBase/ToParent (offset -$23,+$2D), AngleStep16/8
|      (+-1 según bit4/bit3 de la diferencia), Angle16, ClampAngle (evita
|      $11/$1F), AddOffsetA/B ($2940C4/$294144 por ángulo), OffworldA/B
|      ($293FD4/$293FDE), ParentOffworld (a6 = padre, $5E45A),
|      SlotPrioCheck.
|  II. PROPS DE MISIÓN, 3ª tanda ($4D6EC..$4E580, Prop_*), misma plantilla
|  que Waves GGGG/MMMM ($2942A template, HP +$66, $2870A -> flash $5E770,
|  $28758 -> música + escombros $77C7E + puntos $51A28 + blits $5022A +
|  registro $43FAC/$4429E, $4FA70 offscreen -> free), referenciados por los
|  registros de spawn $096BFE..$096C8A y $0975F8..$097670 (y $096FFE..
|  $09709E para Prop_Static):
|   7. Prop_Static_04d6ec (2 entradas, sin HP, solo scroll+anim),
|      Prop_Lamp_04d74a (snd $41, sprite $294280; LampIdle: golpe ->
|      música $10A9, 1/4 de crear LampSpark_04d81a (chispa con vel RNG,
|      240+ frames, parpadeo final), LampHit $294296 -> Idle),
|      Prop_Hut_04d8f2 (snd $3F, hijos $5FA00 y HutRoof_04d9d6 a +$28;
|      techo HP $14, al caer HutRoofFall_04da52 puntos $1000, ataque
|      $295FBA, música $1028, padre +$21 = $FF -> HutWreck_04d996 registro
|      $295D38), Prop_Tower_04dad2 (hijos $4F2C2, TowerTop_04db72 y
|      TowerBase_04dce6; cuenta las partes destruidas en +$21 ($10 por la
|      cima, 1 por la base; $12 = todo -> free; $4FA8A al cambiar);
|      TowerTop: MissionWatch $4429E[$E92B2] slot $75, HP $3C, música
|      $1030, blits $29556A/$2955E2; TowerBase: hijo $5F384, HP $3C, a
|      HP <= $1E música $1028 + escombro + blit $2955F6),
|      Prop_TowerFlag_04dc96 (snd $40, muere con el padre),
|      Prop_Bunker_04de40 (MissionWatch [$E92EA], HP $3C, música $1030,
|      blit $29561E/$295632 según signo de X; espera a que +$74 == +$21),
|      Prop_Bridge_04df98 (2 hijos BridgePillar_04e12c a 0 y +$D0 px con
|      facing; MissionWatch [$E9310]; cuando +$21 == $11 (ambos pilares)
|      -> música $1030, escombros, blits $2956AA/$2956E6 ->
|      BridgeCollapse_04e05e: HP $3C, puntos $5000, música $1028, 4 blits;
|      Pillar: hijo $5F38A, HP $3C, música $102F, $4FB3C, suma $10 o 1 al
|      padre según facing), Prop_Nest_04e248 (snd $40, HP $12C, hijo Nest
|      $8E738 a (+$98,+$38); activo con X < $110 -> NestActive_04e2b0:
|      música $102F, escombros -> NestWreck_04e32c puntos $3000 registro
|      $295D78 música $1030), Prop_Shed_04e38a (hijo $4F2C2, HP $1E; ->
|      ShedDamaged_04e450 música $1028 blit $295786, HP $1E; -> puntos
|      $1000 música $1030 blit $29579A free), Prop_Barrier_04e512 (snd
|      $40, +$12 bit6, hitbox $296272, hijo $4ED90; se libera con el
|      scroll $106F50 > $A10 vía $4E580 (hueco siguiente)).
|
|  B. EVIDENCIAS
|  -------------
|  - $070EC0 (lea $4CBD4 + jsr $4AE + $5DD02) crea el TurretCar como hijo
|    desde el módulo de vehículos $070xxx; $049A74 (PowHang) llama a
|    AngleStep8_04d4d0; $08F25A llama a GroundProbeUp_04d668.
|  - Los props aparecen en las listas de spawn de misión $096BFE..$096C8A
|    (misión A) y $0975F8..$097670 (misión B), 20 B por registro.
|  - $58F82 = "handler salida B" del soldado (MeleeGuard lo pone en +$80):
|    la base destruida se convierte en un soldado que huye.
|
|  C. HIPÓTESIS / DUDAS
|  --------------------
|  - "TurretCar" = vehículo con torreta giratoria (el blindado/jeep con
|    cañón de la misión 2-3); el cañón hijo con histórico de ángulos es
|    el proyectil guiado que sigue la trayectoria de la torreta.
|  - Los nombres de props (Lamp, Hut, Tower, Bunker, Bridge, Nest, Shed,
|    Barrier) son funcionales; se confirmarán por tiles.
|  - $4F2C2/$4ED90/$4FA70/$4FA8A/$4FB3C/$4E580 están en el hueco siguiente
|    ($04E580..$04FA50).
|
|  D. CAMPOS DE LA ENTIDAD (a6) USADOS
|  ----------------------------------
|  +$00 handler  +$0C padre  +$12 bit6  +$13 bits 1/3/6  +$20/+$21 estado
|  +$22/+$24 X/Y  +$28/+$2A vel  +$2E grav  +$32/+$33 zoom  +$34 ángulo
|  +$36 vel base  +$38 prio  +$3A facing  +$3C template  +$46  +$48
|  colisión  +$4C ataque  +$60 hitbox  +$66 HP  +$70 contador/ptr  +$72
|  timer  +$74  +$75 conductor  +$76 sentido  +$78 ángulo objetivo  +$7A
|  +$7B retroceso  +$7C/+$7D destruido/cañón  +$7E  +$80  +$88..+$97
|  histórico de ángulos  +$98 rápido
|
|  E. CALLEES EXTERNOS
|  -------------------
|  $4AE/$6FE alloc  $518 free  $236E snd  $2352 música  $13C0E sin/cos
|  $267E2 relink  $2783A scroll  $27BC8 guiado  $27CEE  $280C6 probe
|  $283CA/$283D8 ataque  $28758 HP agotado  $2870A impacto  $28998
|  $28CD4 sprite  $28D70 anim  $2942A template  $43FAC registro  $4429E
|  MissionWatch  $51A28 puntos  $5022A blits  $58F82 soldado  $5DD02/
|  $5DD22 copia pos  $5DD56/$5DD5C offworld  $5E136 ángulo  $5E45A  $5E4DC
|  $5E766/$5E770 flash  $5E9B6/$5E9E4 RNG  $5F384/$5F38A/$5FA00 hijos
|  $776E2 sombra  $77C7E escombros  $77F6A explosión  $799DE dificultad
|  $8E738 nido  $106F50 scroll.
|
|  Verificación: cada sección .text.<Sym> se coloca en su dirección CPU
|  absoluta y reensambla byte-exacta contra build/mslug_prom.bin
|  (MD5 816b3f74c76b3373993407615f1850fe).
| ============================================================================

        .text

| ----------------------------------------------------------------------------
|  TurretCar_Spawn_04cbd4  @ $04CBD4  (114 B)
| ----------------------------------------------------------------------------
        .section .text.TurretCar_Spawn_04cbd4, "ax", @progbits
        .global TurretCar_Spawn_04cbd4
TurretCar_Spawn_04cbd4:
        move.b  #0x44,0x20(a6)                  | +000
        move.b  #0xff,0x80(a6)                  | +006
        move.b  #0x1,0x98(a6)                   | +00c
        bra.w   .L04cbfc                        | +012
        jsr     0x2783a.l                       | +016
        cmpi.w  #0x140,0x22(a6)                 | +01c
        bgt.w   .L04cbfc                        | +022
        rts                                     | +026
.L04cbfc:
        move.w  #0xe,d1                         | +028
        jsr     0x236e.l                        | +02c
        move.b  #0xff,0x32(a6)                  | +032
        move.b  #0xff,0x33(a6)                  | +038
        move.w  #0x8000,0x38(a6)                | +03e
        move.b  #0xff,0x7d(a6)                  | +044
        move.b  #0x0,0x7c(a6)                   | +04a
        move.b  #0x0,0x21(a6)                   | +050
        move.w  #0x0,0x34(a6)                   | +056
        jsr     0x267e2.l                       | +05c
        lea     TurretCar_Body_04cf2e(pc),a1    | +062
        jsr     0x4ae.l                         | +066
        jsr     0x5dd02.l                       | +06c

| ----------------------------------------------------------------------------
|  TurretCar_Idle_04cc46  @ $04CC46  (314 B)
| ----------------------------------------------------------------------------
        .section .text.TurretCar_Idle_04cc46, "ax", @progbits
        .global TurretCar_Idle_04cc46
TurretCar_Idle_04cc46:
        move.w  #0x1f,d0                        | +000
        move.w  0x34(a6),d1                     | +004
        btst    #0x3,d1                         | +008
        bne.w   .L04cc5a                        | +00c
        move.w  #0x11,d0                        | +010
.L04cc5a:
        move.w  d0,0x78(a6)                     | +014
        move.b  #0x0,0x7b(a6)                   | +018
        move.b  #0x0,0x75(a6)                   | +01e
        move.w  #0x0,0x70(a6)                   | +024
        lea     0x290f6e.l,a0                   | +02a
        jsr     0x28cd4.l                       | +030
        lea     .L04cc82(pc),a1                 | +036
        move.l  a1,(a6)                         | +03a
.L04cc82:
        jsr     TurretCar_SyncPosIfBase_04d3ce(pc) | +03c
        addq.w  #0x1,0x70(a6)                   | +040
        move.w  #0x7,d0                         | +044
        cmpi.b  #0x0,0x98(a6)                   | +048
        beq.w   .L04cc9c                        | +04e
        move.w  #0x1,d0                         | +052
.L04cc9c:
        and.w   d0,0x70(a6)                     | +056
        bne.w   .L04ccba                        | +05a
        move.w  0x78(a6),d0                     | +05e
        jsr     TurretCar_AngleStep16_04d4b0(pc) | +062
        move.w  0x34(a6),d1                     | +066
        add.w   d0,0x34(a6)                     | +06a
        andi.w  #0x1f,0x34(a6)                  | +06e
.L04ccba:
        jsr     0x28d70.l                       | +074
        cmpi.b  #0xff,0x21(a6)                  | +07a
        bne.w   .L04cd04                        | +080
        cmpi.b  #0x88,0x20(a6)                  | +084
        bne.w   .L04ccdc                        | +08a
        movea.l 0xc(a6),a0                      | +08e
        addq.b  #0x1,0x21(a0)                   | +092
.L04ccdc:
        move.b  #0xff,0x7c(a6)                  | +096
        lea     0x58f82.l,a1                    | +09c
        move.l  a1,(a6)                         | +0a2
        lea     0x776e2.l,a1                    | +0a4
        jsr     0x4ae.l                         | +0aa
        jsr     0x5dd02.l                       | +0b0
        jsr     TurretCar_AddOffsetB_04d646(pc) | +0b6
        jsr     TurretCar_ClearParentStateIfBase_04d474(pc) | +0ba
.L04cd04:
        move.w  0x78(a6),d0                     | +0be
        cmp.w   0x34(a6),d0                     | +0c2
        bne.w   .L04cd5e                        | +0c6
        cmpi.b  #0xff,0x75(a6)                  | +0ca
        bne.w   .L04cd24                        | +0d0
        lea     TurretCar_Track_04cd80(pc),a1   | +0d4
        move.l  a1,(a6)                         | +0d8
        bra.w   .L04cd5e                        | +0da
.L04cd24:
        cmpi.b  #0x80,0x75(a6)                  | +0de
        beq.w   .L04cd5e                        | +0e4
        lea     0x290f0e.l,a0                   | +0e8
        jsr     0x28cd4.l                       | +0ee
        move.w  0x34(a6),d0                     | +0f4
        asr.b   #0x3,d0                         | +0f8
        andi.b  #0x1,d0                         | +0fa
        or.b    d0,0x3a(a6)                     | +0fe
        lea     TurretCar_Driver_04d048(pc),a1  | +102
        jsr     0x6fe.l                         | +106
        jsr     0x5dd02.l                       | +10c
        move.b  #0x80,0x75(a6)                  | +112
.L04cd5e:
        jsr     TurretCar_IsBaseIdleDone_04d454(pc) | +118
        bcs.w   .L04cd6e                        | +11c
        jsr     TurretCar_OffworldA_04d5ac(pc)  | +120
        bcc.w   .L04cd7e                        | +124
.L04cd6e:
        move.b  #0xff,0x7c(a6)                  | +128
        jsr     TurretCar_ClearParentStateIfBase_04d474(pc) | +12e
        jmp     0x518.l                         | +132
.L04cd7e:
        rts                                     | +138

| ----------------------------------------------------------------------------
|  TurretCar_Track_04cd80  @ $04CD80  (226 B)
| ----------------------------------------------------------------------------
        .section .text.TurretCar_Track_04cd80, "ax", @progbits
        .global TurretCar_Track_04cd80
TurretCar_Track_04cd80:
        lea     0x2b791e.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,0x72(a6)                     | +00c
        lea     .L04cd96(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L04cd96:
        jsr     TurretCar_SyncPosIfBase_04d3ce(pc) | +016
        addq.w  #0x1,0x70(a6)                   | +01a
        move.w  #0x7,d0                         | +01e
        cmpi.b  #0x0,0x98(a6)                   | +022
        beq.w   .L04cdb0                        | +028
        move.w  #0x1,d0                         | +02c
.L04cdb0:
        and.w   d0,0x70(a6)                     | +030
        bne.w   .L04cdd2                        | +034
        jsr     TurretCar_AngleToTargetMirror_04d58a(pc) | +038
        jsr     TurretCar_SpriteIdxByAngle_04d612(pc) | +03c
        jsr     TurretCar_AngleStep16_04d4b0(pc) | +040
        move.w  0x34(a6),d1                     | +044
        add.w   d0,0x34(a6)                     | +048
        andi.w  #0x1f,0x34(a6)                  | +04c
.L04cdd2:
        jsr     0x28d70.l                       | +052
        cmpi.b  #0xff,0x21(a6)                  | +058
        bne.w   .L04ce1c                        | +05e
        cmpi.b  #0x88,0x20(a6)                  | +062
        bne.w   .L04cdf4                        | +068
        movea.l 0xc(a6),a0                      | +06c
        addq.b  #0x1,0x21(a0)                   | +070
.L04cdf4:
        move.b  #0xff,0x7c(a6)                  | +074
        lea     0x58f82.l,a1                    | +07a
        move.l  a1,(a6)                         | +080
        lea     0x776e2.l,a1                    | +082
        jsr     0x4ae.l                         | +088
        jsr     0x5dd02.l                       | +08e
        jsr     TurretCar_AddOffsetB_04d646(pc) | +094
        jsr     TurretCar_ClearParentStateIfBase_04d474(pc) | +098
.L04ce1c:
        cmpi.b  #0xff,0x7d(a6)                  | +09c
        bne.w   .L04ce40                        | +0a2
        subq.w  #0x1,0x72(a6)                   | +0a6
        bne.w   .L04ce40                        | +0aa
        move.b  #0x0,0x7d(a6)                   | +0ae
        move.b  #0xff,0x7b(a6)                  | +0b4
        lea     TurretCar_Recoil_04ce62(pc),a1  | +0ba
        move.l  a1,(a6)                         | +0be
.L04ce40:
        jsr     TurretCar_IsBaseIdleDone_04d454(pc) | +0c0
        bcs.w   .L04ce50                        | +0c4
        jsr     TurretCar_OffworldA_04d5ac(pc)  | +0c8
        bcc.w   .L04ce60                        | +0cc
.L04ce50:
        move.b  #0xff,0x7c(a6)                  | +0d0
        jsr     TurretCar_ClearParentStateIfBase_04d474(pc) | +0d6
        jmp     0x518.l                         | +0da
.L04ce60:
        rts                                     | +0e0

| ----------------------------------------------------------------------------
|  TurretCar_Recoil_04ce62  @ $04CE62  (204 B)
| ----------------------------------------------------------------------------
        .section .text.TurretCar_Recoil_04ce62, "ax", @progbits
        .global TurretCar_Recoil_04ce62
TurretCar_Recoil_04ce62:
        move.b  #0x0,0x7b(a6)                   | +000
        move.w  0x34(a6),d0                     | +006
        asr.b   #0x3,d0                         | +00a
        andi.b  #0x1,d0                         | +00c
        or.b    d0,0x3a(a6)                     | +010
        move.w  #0x32,0x7e(a6)                  | +014
        lea     0x29bfc4.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        lea     .L04ce8e(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L04ce8e:
        jsr     TurretCar_SyncPosIfBase_04d3ce(pc) | +02c
        move.w  0x22(a6),d0                     | +030
        move.w  0x24(a6),d1                     | +034
        movem.w d0-d1,-(a7)                     | +038
        jsr     TurretCar_AddOffsetB_04d646(pc) | +03c
        jsr     0x28d70.l                       | +040
        subq.w  #0x1,0x7e(a6)                   | +046
        bne.w   .L04cebc                        | +04a
        move.b  #0x0,0x3a(a6)                   | +04e
        lea     TurretCar_Idle_04cc46(pc),a1    | +054
        move.l  a1,(a6)                         | +058
.L04cebc:
        movem.w (a7)+,d0-d1                     | +05a
        move.w  d0,0x22(a6)                     | +05e
        move.w  d1,0x24(a6)                     | +062
        cmpi.b  #0xff,0x21(a6)                  | +066
        bne.w   .L04cf0c                        | +06c
        cmpi.b  #0x88,0x20(a6)                  | +070
        bne.w   .L04cee4                        | +076
        movea.l 0xc(a6),a0                      | +07a
        addq.b  #0x1,0x21(a0)                   | +07e
.L04cee4:
        move.b  #0xff,0x7c(a6)                  | +082
        lea     0x58f82.l,a1                    | +088
        move.l  a1,(a6)                         | +08e
        lea     0x776e2.l,a1                    | +090
        jsr     0x4ae.l                         | +096
        jsr     0x5dd02.l                       | +09c
        jsr     TurretCar_AddOffsetB_04d646(pc) | +0a2
        jsr     TurretCar_ClearParentStateIfBase_04d474(pc) | +0a6
.L04cf0c:
        jsr     TurretCar_IsBaseIdleDone_04d454(pc) | +0aa
        bcs.w   .L04cf1c                        | +0ae
        jsr     TurretCar_OffworldA_04d5ac(pc)  | +0b2
        bcc.w   .L04cf2c                        | +0b6
.L04cf1c:
        move.b  #0xff,0x7c(a6)                  | +0ba
        jsr     TurretCar_ClearParentStateIfBase_04d474(pc) | +0c0
        jmp     0x518.l                         | +0c4
.L04cf2c:
        rts                                     | +0ca

| ----------------------------------------------------------------------------
|  TurretCar_Body_04cf2e  @ $04CF2E  (282 B)
| ----------------------------------------------------------------------------
        .section .text.TurretCar_Body_04cf2e, "ax", @progbits
        .global TurretCar_Body_04cf2e
TurretCar_Body_04cf2e:
        move.w  #0x22,d1                        | +000
        jsr     0x236e.l                        | +004
        move.b  #0xff,0x32(a6)                  | +00a
        move.b  #0xff,0x33(a6)                  | +010
        move.w  #0xc,0x66(a6)                   | +016
        lea     0x2b7798.l,a0                   | +01c
        jsr     0x799de.l                       | +022
        move.w  d0,0x66(a6)                     | +028
        jsr     0x267e2.l                       | +02c
        jsr     TurretCar_CopyParentAngle_04d5e4(pc) | +032
        lea     0x2912d6.l,a0                   | +036
        jsr     0x28cd4.l                       | +03c
        lea     .L04cf76(pc),a1                 | +042
        move.l  a1,(a6)                         | +046
.L04cf76:
        movea.l 0xc(a6),a0                      | +048
        move.w  0x22(a0),0x22(a6)               | +04c
        move.w  0x24(a0),0x24(a6)               | +052
        move.w  0x38(a0),0x38(a6)               | +058
        jsr     TurretCar_CopyParentAngle_04d5e4(pc) | +05e
        movea.l 0xc(a6),a0                      | +062
        cmpi.b  #0xff,0x7b(a0)                  | +066
        bne.w   .L04cfaa                        | +06c
        lea     0x291526.l,a0                   | +070
        jsr     0x28cd4.l                       | +076
.L04cfaa:
        jsr     0x28d70.l                       | +07c
        movea.l 0xc(a6),a0                      | +082
        cmpi.b  #0x44,0x20(a0)                  | +086
        bne.w   .L04cfc6                        | +08c
        jsr     TurretCar_ParentOffworld_04d48a(pc) | +090
        bcs.w   .L04cfde                        | +094
.L04cfc6:
        movea.l 0xc(a6),a0                      | +098
        cmpi.b  #0xff,0x7c(a0)                  | +09c
        beq.w   .L04d040                        | +0a2
        jsr     0x2870a.l                       | +0a6
        bcc.w   .L04cff0                        | +0ac
.L04cfde:
        lea     0x5e766.l,a0                    | +0b0
        jsr     0x5e770.l                       | +0b6
        bclr    #0x3,0x13(a6)                   | +0bc
.L04cff0:
        jsr     0x28758.l                       | +0c2
        bcc.w   .L04d046                        | +0c8
        move.w  #0x1023,d0                      | +0cc
        jsr     0x2352.l                        | +0d0
        movea.l 0xc(a6),a0                      | +0d6
        move.b  #0xff,0x21(a0)                  | +0da
        lea     0x293eb4.l,a1                   | +0e0
        jsr     0x77c7e.l                       | +0e6
        lea     0x293ec6.l,a1                   | +0ec
        jsr     0x77c7e.l                       | +0f2
        movea.l 0xc(a6),a0                      | +0f8
        cmpi.b  #0x88,0x20(a0)                  | +0fc
        bne.w   .L04d040                        | +102
        lea     0x295682.l,a2                   | +106
        jsr     0x5022a.l                       | +10c
.L04d040:
        jmp     0x518.l                         | +112
.L04d046:
        rts                                     | +118

| ----------------------------------------------------------------------------
|  TurretCar_Driver_04d048  @ $04D048  (154 B)
| ----------------------------------------------------------------------------
        .section .text.TurretCar_Driver_04d048, "ax", @progbits
        .global TurretCar_Driver_04d048
TurretCar_Driver_04d048:
        move.w  #0x23,d1                        | +000
        jsr     0x236e.l                        | +004
        movea.l 0xc(a6),a0                      | +00a
        move.w  0x34(a0),d0                     | +00e
        asr.b   #0x3,d0                         | +012
        andi.b  #0x1,d0                         | +014
        or.b    d0,0x3a(a6)                     | +018
        move.b  #0xff,0x32(a6)                  | +01c
        move.b  #0xff,0x33(a6)                  | +022
        lea     0x2b789c.l,a0                   | +028
        jsr     0x799de.l                       | +02e
        move.w  d0,0x66(a6)                     | +034
        lea     0x293ed8.l,a0                   | +038
        move.l  a0,0x4c(a6)                     | +03e
        jsr     0x283ca.l                       | +042
        jsr     0x267e2.l                       | +048
        lea     0x29276a.l,a0                   | +04e
        jsr     0x28cd4.l                       | +054
        lea     .L04d0a8(pc),a1                 | +05a
        move.l  a1,(a6)                         | +05e
.L04d0a8:
        jsr     0x5e4dc.l                       | +060
        jsr     TurretCar_CopyParentAngle_04d5e4(pc) | +066
        jsr     0x28d70.l                       | +06a
        bcc.w   .L04d0cc                        | +070
        movea.l 0xc(a6),a0                      | +074
        move.b  #0xff,0x75(a0)                  | +078
        lea     TurretCar_DriverPanic_04d0e2(pc),a1 | +07e
        move.l  a1,(a6)                         | +082
.L04d0cc:
        movea.l 0xc(a6),a0                      | +084
        cmpi.b  #0xff,0x7c(a0)                  | +088
        bne.w   .L04d0e0                        | +08e
        jmp     0x518.l                         | +092
.L04d0e0:
        rts                                     | +098

| ----------------------------------------------------------------------------
|  TurretCar_DriverPanic_04d0e2  @ $04D0E2  (88 B)
| ----------------------------------------------------------------------------
        .section .text.TurretCar_DriverPanic_04d0e2, "ax", @progbits
        .global TurretCar_DriverPanic_04d0e2
TurretCar_DriverPanic_04d0e2:
        move.b  #0x0,0x3a(a6)                   | +000
        move.w  #0x12c,0x70(a6)                 | +006
        lea     0x292a48.l,a0                   | +00c
        jsr     0x28cd4.l                       | +012
        lea     .L04d100(pc),a1                 | +018
        move.l  a1,(a6)                         | +01c
.L04d100:
        jsr     0x5e4dc.l                       | +01e
        jsr     TurretCar_CopyParentAngle_04d5e4(pc) | +024
        jsr     0x28d70.l                       | +028
        movea.l 0xc(a6),a0                      | +02e
        cmpi.b  #0xff,0x7b(a0)                  | +032
        bne.w   .L04d124                        | +038
        lea     TurretCar_DriverFlee_04d13a(pc),a1 | +03c
        move.l  a1,(a6)                         | +040
.L04d124:
        movea.l 0xc(a6),a0                      | +042
        cmpi.b  #0xff,0x7c(a0)                  | +046
        bne.w   .L04d138                        | +04c
        jmp     0x518.l                         | +050
.L04d138:
        rts                                     | +056

| ----------------------------------------------------------------------------
|  TurretCar_DriverFlee_04d13a  @ $04D13A  (76 B)
| ----------------------------------------------------------------------------
        .section .text.TurretCar_DriverFlee_04d13a, "ax", @progbits
        .global TurretCar_DriverFlee_04d13a
TurretCar_DriverFlee_04d13a:
        move.w  #0x10a3,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x292c92.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L04d156(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L04d156:
        jsr     0x5e4dc.l                       | +01c
        jsr     TurretCar_CopyParentAngle_04d5e4(pc) | +022
        jsr     0x28d70.l                       | +026
        bcc.w   .L04d170                        | +02c
        lea     TurretCar_Cannon_04d186(pc),a1  | +030
        move.l  a1,(a6)                         | +034
.L04d170:
        movea.l 0xc(a6),a0                      | +036
        cmpi.b  #0xff,0x7c(a0)                  | +03a
        bne.w   .L04d184                        | +040
        jmp     0x518.l                         | +044
.L04d184:
        rts                                     | +04a

| ----------------------------------------------------------------------------
|  TurretCar_Cannon_04d186  @ $04D186  (442 B)
| ----------------------------------------------------------------------------
        .section .text.TurretCar_Cannon_04d186, "ax", @progbits
        .global TurretCar_Cannon_04d186
TurretCar_Cannon_04d186:
        jsr     TurretCar_AddOffsetA_04d624(pc) | +000
        addq.w  #0x2,0x38(a6)                   | +004
        move.b  #0x0,0x7a(a6)                   | +008
        move.w  #0xffff,0x70(a6)                | +00e
        lea     0x2b781a.l,a0                   | +014
        jsr     0x799de.l                       | +01a
        move.w  d0,0x36(a6)                     | +020
        jsr     TurretCar_CopyParentAngle_04d5e4(pc) | +024
        lea     0x2927f8.l,a0                   | +028
        jsr     0x28cd4.l                       | +02e
        move.w  0x34(a6),d0                     | +034
        move.w  #0x97,d1                        | +038
.L04d1c2:
        move.b  d0,(a6,d1.w)                    | +03c
        subq.w  #0x1,d1                         | +040
        cmpi.w  #0x88,d1                        | +042
        bgt.b   .L04d1c2                        | +046
        jsr     TurretCar_ClampAngle_04d6ac(pc) | +048
        lea     .L04d1d8(pc),a1                 | +04c
        move.l  a1,(a6)                         | +050
.L04d1d8:
        addq.w  #0x1,0x70(a6)                   | +052
        move.w  0x78(a6),d0                     | +056
        cmpi.w  #0x258,0x70(a6)                 | +05a
        bgt.w   .L04d228                        | +060
        move.w  0x70(a6),d0                     | +064
        andi.w  #0xf,d0                         | +068
        bne.w   .L04d240                        | +06c
        jsr     TurretCar_AngleToTarget_04d4f0(pc) | +070
        cmpi.b  #0xff,0x7a(a6)                  | +074
        beq.w   .L04d218                        | +07a
        jsr     TurretCar_Angle16_04d57e(pc)    | +07e
        cmpi.w  #0x1c0,0x24(a6)                 | +082
        blt.w   .L04d218                        | +088
        move.b  #0xff,0x7a(a6)                  | +08c
.L04d218:
        jsr     TurretCar_GroundProbeUp_04d668(pc) | +092
        bcc.w   .L04d224                        | +096
        move.w  #0x8,d0                         | +09a
.L04d224:
        move.w  d0,0x78(a6)                     | +09e
.L04d228:
        jsr     TurretCar_AngleStep16_04d4b0(pc) | +0a2
        ori.w   #0x1,d0                         | +0a6
        move.w  0x70(a6),d1                     | +0aa
        andi.w  #0x1f,d1                        | +0ae
        bne.w   .L04d240                        | +0b2
        move.w  d0,0x76(a6)                     | +0b6
.L04d240:
        move.w  0x70(a6),d0                     | +0ba
        andi.w  #0x3,d0                         | +0be
        bne.w   .L04d25e                        | +0c2
        move.w  0x34(a6),d1                     | +0c6
        move.w  0x76(a6),d0                     | +0ca
        add.w   d0,0x34(a6)                     | +0ce
        andi.w  #0x1f,0x34(a6)                  | +0d2
.L04d25e:
        jsr     TurretCar_ShiftAngleHistory_04d51c(pc) | +0d8
        jsr     TurretCar_ProbeGuided_04d3fe(pc) | +0dc
        bcs.w   .L04d2f4                        | +0e0
        jsr     0x28d70.l                       | +0e4
        move.w  0x70(a6),d0                     | +0ea
        andi.w  #0x7,d0                         | +0ee
        bne.w   .L04d2be                        | +0f2
        cmpi.w  #0x10,0x34(a6)                  | +0f6
        bgt.w   .L04d29a                        | +0fc
        lea     TurretCar_Shell_04d340(pc),a1   | +100
        jsr     0x4ae.l                         | +104
        jsr     0x5dd02.l                       | +10a
        bra.w   .L04d2ae                        | +110
.L04d29a:
        lea     TurretCar_Shell_04d340(pc),a1   | +114
        jsr     0x6fe.l                         | +118
        jsr     0x5dd02.l                       | +11e
        subq.w  #0x1,0x38(a0)                   | +124
.L04d2ae:
        jsr     TurretCar_AddMuzzleOffset_04d5f0(pc) | +128
        move.b  0x97(a6),d1                     | +12c
        andi.w  #0x1f,d1                        | +130
        move.w  d1,0x34(a0)                     | +134
.L04d2be:
        jsr     0x283d8.l                       | +138
        btst    #0x1,0x13(a6)                   | +13e
        bne.w   .L04d2f4                        | +144
        jsr     0x2870a.l                       | +148
        bcc.w   .L04d2ea                        | +14e
        lea     0x5e766.l,a0                    | +152
        jsr     0x5e770.l                       | +158
        bclr    #0x3,0x13(a6)                   | +15e
.L04d2ea:
        jsr     0x28758.l                       | +164
        bcc.w   .L04d326                        | +16a
.L04d2f4:
        lea     0x77f6a.l,a1                    | +16e
        jsr     0x4ae.l                         | +174
        jsr     0x5dd02.l                       | +17a
        move.w  #0x4000,0x38(a0)                | +180
        lea     0x293ff2.l,a1                   | +186
        jsr     0x77c7e.l                       | +18c
        move.w  #0x1022,d0                      | +192
        jsr     0x2352.l                        | +196
        bra.w   .L04d32e                        | +19c
.L04d326:
        jsr     TurretCar_OffworldB_04d5c8(pc)  | +1a0
        bcc.w   .L04d33e                        | +1a4
.L04d32e:
        movea.l 0xc(a6),a0                      | +1a8
        move.b  #0xff,0x7d(a0)                  | +1ac
        jmp     0x518.l                         | +1b2
.L04d33e:
        rts                                     | +1b8

| ----------------------------------------------------------------------------
|  TurretCar_Shell_04d340  @ $04D340  (142 B)
| ----------------------------------------------------------------------------
        .section .text.TurretCar_Shell_04d340, "ax", @progbits
        .global TurretCar_Shell_04d340
TurretCar_Shell_04d340:
        lea     0x2911bc.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x293fe8.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0xd,d1                         | +016
        jsr     0x236e.l                        | +01a
        move.b  #0xff,0x32(a6)                  | +020
        move.b  #0xff,0x33(a6)                  | +026
        move.w  #0x80,0x36(a6)                  | +02c
        move.w  #0x2,d0                         | +032
        jsr     0x5e9e4.l                       | +036
        btst    #0x0,d0                         | +03c
        bne.w   .L04d386                        | +040
        neg.w   d0                              | +044
.L04d386:
        add.w   d0,0x34(a6)                     | +046
        andi.w  #0x1f,0x34(a6)                  | +04a
        jsr     TurretCar_VelFromAngle_04d502(pc) | +050
        movea.l 0x70(a6),a0                     | +054
        jsr     0x5dd56.l                       | +058
        bcs.w   .L04d3c6                        | +05e
        lea     .L04d3a8(pc),a1                 | +062
        move.l  a1,(a6)                         | +066
.L04d3a8:
        jsr     0x27cee.l                       | +068
        jsr     0x28d70.l                       | +06e
        bcs.w   .L04d3c6                        | +074
        movea.l 0x70(a6),a0                     | +078
        jsr     0x5dd56.l                       | +07c
        bcc.w   .L04d3cc                        | +082
.L04d3c6:
        jmp     0x518.l                         | +086
.L04d3cc:
        rts                                     | +08c

| ----------------------------------------------------------------------------
|  TurretCar_SyncPosIfBase_04d3ce  @ $04D3CE  (10 B)
| ----------------------------------------------------------------------------
        .section .text.TurretCar_SyncPosIfBase_04d3ce, "ax", @progbits
        .global TurretCar_SyncPosIfBase_04d3ce
TurretCar_SyncPosIfBase_04d3ce:
        cmpi.b  #0x44,0x20(a6)                  | +000
        beq.w   TurretCar_SyncPosToParent_04d3e0 | +006

| ----------------------------------------------------------------------------
|  TurretCar_SyncPosToParent_04d3e0  @ $04D3E0  (24 B)
| ----------------------------------------------------------------------------
        .section .text.TurretCar_SyncPosToParent_04d3e0, "ax", @progbits
        .global TurretCar_SyncPosToParent_04d3e0
TurretCar_SyncPosToParent_04d3e0:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x22(a0),d0                     | +004
        subi.w  #0x23,d0                        | +008
        move.w  d0,0x22(a6)                     | +00c
        move.w  0x24(a0),d0                     | +010
        addi.w  #0x2d,d0                        | +014

| ----------------------------------------------------------------------------
|  TurretCar_ProbeGuided_04d3fe  @ $04D3FE  (66 B)
| ----------------------------------------------------------------------------
        .section .text.TurretCar_ProbeGuided_04d3fe, "ax", @progbits
        .global TurretCar_ProbeGuided_04d3fe
TurretCar_ProbeGuided_04d3fe:
        movea.l 0xc(a6),a0                      | +000
        cmpi.b  #0x44,0x20(a0)                  | +004
        bne.w   JsrAbsThunk_04d44c              | +00a
        bset    #0x6,0x13(a6)                   | +00e
        move.w  0x34(a6),d0                     | +014
        movem.w d0,-(a7)                        | +018
        jsr     0x27bc8.l                       | +01c
        bcc.w   .L04d42c                        | +022
        move.b  #0x1,d1                         | +026
        bra.w   .L04d430                        | +02a
.L04d42c:
        move.b  #0x0,d1                         | +02e
.L04d430:
        movem.w (a7)+,d0                        | +032
        move.w  d0,0x34(a6)                     | +036
        cmpi.b  #0x0,d1                         | +03a
        beq.w   ClearXN_04d446                  | +03e

| ----------------------------------------------------------------------------
|  TurretCar_IsBaseIdleDone_04d454  @ $04D454  (20 B)
| ----------------------------------------------------------------------------
        .section .text.TurretCar_IsBaseIdleDone_04d454, "ax", @progbits
        .global TurretCar_IsBaseIdleDone_04d454
TurretCar_IsBaseIdleDone_04d454:
        cmpi.b  #0x44,0x20(a6)                  | +000
        bne.w   ClearXN_04d46e                  | +006
        cmpi.b  #0x0,0x80(a6)                   | +00a
        bne.w   ClearXN_04d46e                  | +010

| ----------------------------------------------------------------------------
|  TurretCar_ClearParentStateIfBase_04d474  @ $04D474  (22 B)
| ----------------------------------------------------------------------------
        .section .text.TurretCar_ClearParentStateIfBase_04d474, "ax", @progbits
        .global TurretCar_ClearParentStateIfBase_04d474
TurretCar_ClearParentStateIfBase_04d474:
        cmpi.b  #0x44,0x20(a6)                  | +000
        bne.w   .L04d488                        | +006
        movea.l 0xc(a6),a0                      | +00a
        move.b  #0x0,0x20(a0)                   | +00e
.L04d488:
        rts                                     | +014

| ----------------------------------------------------------------------------
|  TurretCar_ParentOffworld_04d48a  @ $04D48A  (22 B)
| ----------------------------------------------------------------------------
        .section .text.TurretCar_ParentOffworld_04d48a, "ax", @progbits
        .global TurretCar_ParentOffworld_04d48a
TurretCar_ParentOffworld_04d48a:
        movem.l a6,-(a7)                        | +000
        movea.l 0xc(a6),a6                      | +004
        jsr     0x5e45a.l                       | +008
        bcs.w   TurretCar_ParentOffworldRestore_04d4a6 | +00e
        movem.l (a7)+,a6                        | +012

| ----------------------------------------------------------------------------
|  TurretCar_ParentOffworldRestore_04d4a6  @ $04D4A6  (4 B)
| ----------------------------------------------------------------------------
        .section .text.TurretCar_ParentOffworldRestore_04d4a6, "ax", @progbits
        .global TurretCar_ParentOffworldRestore_04d4a6
TurretCar_ParentOffworldRestore_04d4a6:
        movem.l (a7)+,a6                        | +000

| ----------------------------------------------------------------------------
|  TurretCar_AngleStep16_04d4b0  @ $04D4B0  (32 B)
| ----------------------------------------------------------------------------
        .section .text.TurretCar_AngleStep16_04d4b0, "ax", @progbits
        .global TurretCar_AngleStep16_04d4b0
TurretCar_AngleStep16_04d4b0:
        move.w  0x34(a6),d1                     | +000
        sub.w   d1,d0                           | +004
        beq.w   .L04d4ce                        | +006
        btst    #0x4,d0                         | +00a
        bne.w   .L04d4ca                        | +00e
        move.w  #0x1,d0                         | +012
        bra.w   .L04d4ce                        | +016
.L04d4ca:
        move.w  #0xff,d0                        | +01a
.L04d4ce:
        rts                                     | +01e

| ----------------------------------------------------------------------------
|  TurretCar_AngleStep8_04d4d0  @ $04D4D0  (32 B)
| ----------------------------------------------------------------------------
        .section .text.TurretCar_AngleStep8_04d4d0, "ax", @progbits
        .global TurretCar_AngleStep8_04d4d0
TurretCar_AngleStep8_04d4d0:
        move.w  0x34(a6),d1                     | +000
        sub.w   d1,d0                           | +004
        beq.w   .L04d4ee                        | +006
        btst    #0x3,d0                         | +00a
        bne.w   .L04d4ea                        | +00e
        move.w  #0x1,d0                         | +012
        bra.w   .L04d4ee                        | +016
.L04d4ea:
        move.w  #0xff,d0                        | +01a
.L04d4ee:
        rts                                     | +01e

| ----------------------------------------------------------------------------
|  TurretCar_AngleToTarget_04d4f0  @ $04D4F0  (18 B)
| ----------------------------------------------------------------------------
        .section .text.TurretCar_AngleToTarget_04d4f0, "ax", @progbits
        .global TurretCar_AngleToTarget_04d4f0
TurretCar_AngleToTarget_04d4f0:
        jsr     0x5e136.l                       | +000
        addi.w  #0x8,d0                         | +006
        asr.w   #0x3,d0                         | +00a
        andi.w  #0x1f,d0                        | +00c
        rts                                     | +010

| ----------------------------------------------------------------------------
|  TurretCar_VelFromAngle_04d502  @ $04D502  (26 B)
| ----------------------------------------------------------------------------
        .section .text.TurretCar_VelFromAngle_04d502, "ax", @progbits
        .global TurretCar_VelFromAngle_04d502
TurretCar_VelFromAngle_04d502:
        move.w  0x34(a6),d0                     | +000
        asl.w   #0x3,d0                         | +004
        move.w  0x36(a6),d1                     | +006
        jsr     0x13c0e.l                       | +00a
        move.w  d1,0x28(a6)                     | +010
        move.w  d2,0x2a(a6)                     | +014
        rts                                     | +018

| ----------------------------------------------------------------------------
|  TurretCar_ShiftAngleHistory_04d51c  @ $04D51C  (98 B)
| ----------------------------------------------------------------------------
        .section .text.TurretCar_ShiftAngleHistory_04d51c, "ax", @progbits
        .global TurretCar_ShiftAngleHistory_04d51c
TurretCar_ShiftAngleHistory_04d51c:
        move.w  #0x97,d1                        | +000
.L04d520:
        move.b  -0x1(a6,d1.w),(a6,d1.w)         | +004
        subq.w  #0x1,d1                         | +00a
        cmpi.w  #0x88,d1                        | +00c
        bgt.b   .L04d520                        | +010
        move.w  0x34(a6),d0                     | +012
        move.b  d0,0x88(a6)                     | +016
        move.b  0x97(a6),d0                     | +01a
        andi.w  #0x1f,d0                        | +01e
        asl.w   #0x3,d0                         | +022
        move.w  0x36(a6),d1                     | +024
        jsr     TurretCar_GroundProbeUp_04d668(pc) | +028
        bcc.w   .L04d558                        | +02c
        btst    #0x4,0x97(a6)                   | +030
        beq.w   .L04d558                        | +036
        asr.w   #0x1,d1                         | +03a
.L04d558:
        jsr     0x13c0e.l                       | +03c
        move.w  d1,0x28(a6)                     | +042
        move.w  d2,0x2a(a6)                     | +046
        move.w  #0x0,0x2e(a6)                   | +04a
        btst    #0x4,0x97(a6)                   | +050
        beq.w   .L04d57c                        | +056
        move.w  #0xffa0,0x2e(a6)                | +05a
.L04d57c:
        rts                                     | +060

| ----------------------------------------------------------------------------
|  TurretCar_Angle16_04d57e  @ $04D57E  (4 B)
| ----------------------------------------------------------------------------
        .section .text.TurretCar_Angle16_04d57e, "ax", @progbits
        .global TurretCar_Angle16_04d57e
TurretCar_Angle16_04d57e:
        move.w  #0x10,d0                        | +000

| ----------------------------------------------------------------------------
|  TurretCar_AngleToTargetMirror_04d58a  @ $04D58A  (34 B)
| ----------------------------------------------------------------------------
        .section .text.TurretCar_AngleToTargetMirror_04d58a, "ax", @progbits
        .global TurretCar_AngleToTargetMirror_04d58a
TurretCar_AngleToTargetMirror_04d58a:
        jsr     0x5e136.l                       | +000
        addi.w  #0x8,d0                         | +006
        asr.w   #0x3,d0                         | +00a
        btst    #0x4,d0                         | +00c
        bne.w   .L04d5a6                        | +010
        neg.w   d0                              | +014
        clr.w   d1                              | +016
        add.w   d0,d1                           | +018
        move.w  d1,d0                           | +01a
.L04d5a6:
        andi.w  #0x1f,d0                        | +01c
        rts                                     | +020

| ----------------------------------------------------------------------------
|  TurretCar_OffworldA_04d5ac  @ $04D5AC  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TurretCar_OffworldA_04d5ac, "ax", @progbits
        .global TurretCar_OffworldA_04d5ac
TurretCar_OffworldA_04d5ac:
        lea     0x293fd4.l,a0                   | +000
        jsr     0x5dd5c.l                       | +006
        bcc.w   ClearC_04d5c2                   | +00c

| ----------------------------------------------------------------------------
|  TurretCar_OffworldB_04d5c8  @ $04D5C8  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TurretCar_OffworldB_04d5c8, "ax", @progbits
        .global TurretCar_OffworldB_04d5c8
TurretCar_OffworldB_04d5c8:
        lea     0x293fde.l,a0                   | +000
        jsr     0x5dd56.l                       | +006
        bcc.w   ClearC_04d5de                   | +00c

| ----------------------------------------------------------------------------
|  TurretCar_CopyParentAngle_04d5e4  @ $04D5E4  (12 B)
| ----------------------------------------------------------------------------
        .section .text.TurretCar_CopyParentAngle_04d5e4, "ax", @progbits
        .global TurretCar_CopyParentAngle_04d5e4
TurretCar_CopyParentAngle_04d5e4:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x34(a0),0x34(a6)               | +004
        rts                                     | +00a

| ----------------------------------------------------------------------------
|  TurretCar_AddMuzzleOffset_04d5f0  @ $04D5F0  (34 B)
| ----------------------------------------------------------------------------
        .section .text.TurretCar_AddMuzzleOffset_04d5f0, "ax", @progbits
        .global TurretCar_AddMuzzleOffset_04d5f0
TurretCar_AddMuzzleOffset_04d5f0:
        lea     0x294044.l,a1                   | +000
        move.w  0x34(a6),d0                     | +006
        andi.w  #0x1f,d0                        | +00a
        asl.w   #0x2,d0                         | +00e
        move.w  (a1,d0.w),d1                    | +010
        add.w   d1,0x22(a0)                     | +014
        move.w  0x2(a1,d0.w),d1                 | +018
        add.w   d1,0x24(a0)                     | +01c
        rts                                     | +020

| ----------------------------------------------------------------------------
|  TurretCar_SpriteIdxByAngle_04d612  @ $04D612  (18 B)
| ----------------------------------------------------------------------------
        .section .text.TurretCar_SpriteIdxByAngle_04d612, "ax", @progbits
        .global TurretCar_SpriteIdxByAngle_04d612
TurretCar_SpriteIdxByAngle_04d612:
        andi.w  #0x1f,d0                        | +000
        lea     0x2941c4.l,a0                   | +004
        asl.w   #0x1,d0                         | +00a
        move.w  (a0,d0.w),d0                    | +00c
        rts                                     | +010

| ----------------------------------------------------------------------------
|  TurretCar_AddOffsetA_04d624  @ $04D624  (34 B)
| ----------------------------------------------------------------------------
        .section .text.TurretCar_AddOffsetA_04d624, "ax", @progbits
        .global TurretCar_AddOffsetA_04d624
TurretCar_AddOffsetA_04d624:
        lea     0x2940c4.l,a1                   | +000
        move.w  0x34(a6),d0                     | +006
        andi.w  #0x1f,d0                        | +00a
        asl.w   #0x2,d0                         | +00e
        move.w  (a1,d0.w),d1                    | +010
        add.w   d1,0x22(a6)                     | +014
        move.w  0x2(a1,d0.w),d1                 | +018
        add.w   d1,0x24(a6)                     | +01c
        rts                                     | +020

| ----------------------------------------------------------------------------
|  TurretCar_AddOffsetB_04d646  @ $04D646  (34 B)
| ----------------------------------------------------------------------------
        .section .text.TurretCar_AddOffsetB_04d646, "ax", @progbits
        .global TurretCar_AddOffsetB_04d646
TurretCar_AddOffsetB_04d646:
        lea     0x294144.l,a1                   | +000
        move.w  0x34(a6),d0                     | +006
        andi.w  #0x1f,d0                        | +00a
        asl.w   #0x2,d0                         | +00e
        move.w  (a1,d0.w),d1                    | +010
        add.w   d1,0x22(a6)                     | +014
        move.w  0x2(a1,d0.w),d1                 | +018
        add.w   d1,0x24(a6)                     | +01c
        rts                                     | +020

| ----------------------------------------------------------------------------
|  TurretCar_GroundProbeUp_04d668  @ $04D668  (52 B)
| ----------------------------------------------------------------------------
        .section .text.TurretCar_GroundProbeUp_04d668, "ax", @progbits
        .global TurretCar_GroundProbeUp_04d668
TurretCar_GroundProbeUp_04d668:
        movem.w d0-d1,-(a7)                     | +000
        move.w  0x22(a6),d1                     | +004
        move.w  0x24(a6),d2                     | +008
        move.w  #0x7,d5                         | +00c
.L04d678:
        subi.w  #0x8,d2                         | +010
        subi.w  #0x1,d5                         | +014
        beq.w   TurretCar_GroundProbeRestore_04d6a2 | +018
        movem.w d5,-(a7)                        | +01c
        jsr     0x280c6.l                       | +020
        movem.w (a7)+,d5                        | +026
        cmpi.b  #0x1,d0                         | +02a
        bne.b   .L04d678                        | +02e
        movem.w (a7)+,d0-d1                     | +030

| ----------------------------------------------------------------------------
|  TurretCar_GroundProbeRestore_04d6a2  @ $04D6A2  (4 B)
| ----------------------------------------------------------------------------
        .section .text.TurretCar_GroundProbeRestore_04d6a2, "ax", @progbits
        .global TurretCar_GroundProbeRestore_04d6a2
TurretCar_GroundProbeRestore_04d6a2:
        movem.w (a7)+,d0-d1                     | +000

| ----------------------------------------------------------------------------
|  TurretCar_ClampAngle_04d6ac  @ $04D6AC  (36 B)
| ----------------------------------------------------------------------------
        .section .text.TurretCar_ClampAngle_04d6ac, "ax", @progbits
        .global TurretCar_ClampAngle_04d6ac
TurretCar_ClampAngle_04d6ac:
        cmpi.w  #0x11,0x34(a6)                  | +000
        bne.w   .L04d6be                        | +006
        addi.w  #0x2,0x34(a6)                   | +00a
        rts                                     | +010
.L04d6be:
        cmpi.w  #0x1f,0x34(a6)                  | +012
        bne.w   .L04d6ce                        | +018
        subi.w  #0x2,0x34(a6)                   | +01c
.L04d6ce:
        rts                                     | +022

| ----------------------------------------------------------------------------
|  TurretCar_SlotPrioCheck_04d6d0  @ $04D6D0  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TurretCar_SlotPrioCheck_04d6d0, "ax", @progbits
        .global TurretCar_SlotPrioCheck_04d6d0
TurretCar_SlotPrioCheck_04d6d0:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_04d6e6                    | +00c

| ----------------------------------------------------------------------------
|  Prop_Static_04d6ec  @ $04D6EC  (94 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_Static_04d6ec, "ax", @progbits
        .global Prop_Static_04d6ec
Prop_Static_04d6ec:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x8000,0x38(a6)                | +00a
        jsr     0x267e2.l                       | +010
        jsr     0x27cee.l                       | +016
        bra.w   .L04d716                        | +01c
        movea.l 0x3c(a6),a1                     | +020
        jsr     0x2942a.l                       | +024
.L04d716:
        move.b  #0xff,0x32(a6)                  | +02a
        move.b  #0xff,0x33(a6)                  | +030
        move.b  #0x0,0x3a(a6)                   | +036
        lea     .L04d72e(pc),a1                 | +03c
        move.l  a1,(a6)                         | +040
.L04d72e:
        jsr     0x2783a.l                       | +042
        jsr     0x28d70.l                       | +048
        jsr     Sub_0004FA70(pc)                | +04e
        bcc.w   .L04d748                        | +052
        jmp     0x518.l                         | +056
.L04d748:
        rts                                     | +05c

| ----------------------------------------------------------------------------
|  Prop_Lamp_04d74a  @ $04D74A  (56 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_Lamp_04d74a, "ax", @progbits
        .global Prop_Lamp_04d74a
Prop_Lamp_04d74a:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x41,d1                        | +00a
        jsr     0x236e.l                        | +00e
        move.b  #0xff,0x32(a6)                  | +014
        move.b  #0xff,0x33(a6)                  | +01a
        move.b  #0x0,0x3a(a6)                   | +020
        lea     0x294280.l,a0                   | +026
        jsr     0x28cd4.l                       | +02c
        lea     Prop_LampIdle_04d782(pc),a1     | +032
        move.l  a1,(a6)                         | +036

| ----------------------------------------------------------------------------
|  Prop_LampIdle_04d782  @ $04D782  (90 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_LampIdle_04d782, "ax", @progbits
        .global Prop_LampIdle_04d782
Prop_LampIdle_04d782:
        jsr     0x2783a.l                       | +000
        jsr     0x28d70.l                       | +006
        jsr     0x2870a.l                       | +00c
        bcc.w   .L04d7cc                        | +012
        move.w  #0x10a9,d0                      | +016
        jsr     0x2352.l                        | +01a
        jsr     0x5e9b6.l                       | +020
        andi.w  #0x3,d0                         | +026
        bne.w   .L04d7c6                        | +02a
        lea     Prop_LampSpark_04d81a(pc),a1    | +02e
        jsr     0x6fe.l                         | +032
        jsr     0x5dd02.l                       | +038
        addi.w  #0x18,0x24(a0)                  | +03e
.L04d7c6:
        lea     Prop_LampHit_04d7dc(pc),a1      | +044
        move.l  a1,(a6)                         | +048
.L04d7cc:
        jsr     Sub_0004FA70(pc)                | +04a
        bcc.w   .L04d7da                        | +04e
        jmp     0x518.l                         | +052
.L04d7da:
        rts                                     | +058

| ----------------------------------------------------------------------------
|  Prop_LampHit_04d7dc  @ $04D7DC  (62 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_LampHit_04d7dc, "ax", @progbits
        .global Prop_LampHit_04d7dc
Prop_LampHit_04d7dc:
        lea     0x294296.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L04d7ee(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L04d7ee:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        bcc.w   .L04d80a                        | +01e
        bclr    #0x3,0x13(a6)                   | +022
        lea     Prop_LampIdle_04d782(pc),a1     | +028
        move.l  a1,(a6)                         | +02c
.L04d80a:
        jsr     Sub_0004FA70(pc)                | +02e
        bcc.w   .L04d818                        | +032
        jmp     0x518.l                         | +036
.L04d818:
        rts                                     | +03c

| ----------------------------------------------------------------------------
|  Prop_LampSpark_04d81a  @ $04D81A  (216 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_LampSpark_04d81a, "ax", @progbits
        .global Prop_LampSpark_04d81a
Prop_LampSpark_04d81a:
        move.w  #0x41,d1                        | +000
        jsr     0x236e.l                        | +004
        bset    #0x6,0x12(a6)                   | +00a
        move.b  #0xff,0x32(a6)                  | +010
        move.b  #0xff,0x33(a6)                  | +016
        move.b  #0x0,0x3a(a6)                   | +01c
        jsr     0x267e2.l                       | +022
        move.w  #0xf0,0x72(a6)                  | +028
        jsr     0x5e9b6.l                       | +02e
        andi.w  #0x1f,d0                        | +034
        add.w   d0,0x72(a6)                     | +038
        jsr     0x5e9b6.l                       | +03c
        andi.w  #0x1f,d0                        | +042
        btst    #0x0,d0                         | +046
        beq.w   .L04d86a                        | +04a
        neg.w   d0                              | +04e
.L04d86a:
        move.w  d0,0x28(a6)                     | +050
        jsr     0x5e9b6.l                       | +054
        andi.w  #0x3f,d0                        | +05a
        btst    #0x0,d0                         | +05e
        beq.w   .L04d882                        | +062
        neg.w   d0                              | +066
.L04d882:
        move.w  d0,0x2a(a6)                     | +068
        lea     0x294378.l,a0                   | +06c
        jsr     0x28cd4.l                       | +072
        lea     .L04d898(pc),a1                 | +078
        move.l  a1,(a6)                         | +07c
.L04d898:
        jsr     0x27cee.l                       | +07e
        cmpi.w  #0x14,0x72(a6)                  | +084
        bgt.w   .L04d8be                        | +08a
        move.w  0x72(a6),d0                     | +08e
        btst    #0x0,d0                         | +092
        beq.w   .L04d8be                        | +096
        subi.b  #0x1,0x46(a6)                   | +09a
        bra.w   .L04d8c4                        | +0a0
.L04d8be:
        jsr     0x28d70.l                       | +0a4
.L04d8c4:
        subi.w  #0x1,0x72(a6)                   | +0aa
        cmpi.w  #0x0,0x72(a6)                   | +0b0
        bgt.w   .L04d8da                        | +0b6
        jmp     0x518.l                         | +0ba
.L04d8da:
        lea     0x296254.l,a0                   | +0c0
        jsr     0x5dd5c.l                       | +0c6
        bcc.w   .L04d8f0                        | +0cc
        jmp     0x518.l                         | +0d0
.L04d8f0:
        rts                                     | +0d6

| ----------------------------------------------------------------------------
|  Prop_Hut_04d8f2  @ $04D8F2  (164 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_Hut_04d8f2, "ax", @progbits
        .global Prop_Hut_04d8f2
Prop_Hut_04d8f2:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x3f,d1                        | +00a
        jsr     0x236e.l                        | +00e
        move.b  #0xff,0x32(a6)                  | +014
        move.b  #0xff,0x33(a6)                  | +01a
        move.b  #0x0,0x3a(a6)                   | +020
        move.b  #0x0,0x21(a6)                   | +026
        lea     0x5fa00.l,a1                    | +02c
        jsr     0x4ae.l                         | +032
        jsr     0x5dd22.l                       | +038
        addi.w  #0x28,0x22(a0)                  | +03e
        addi.w  #0x1,0x24(a0)                   | +044
        lea     Prop_HutRoof_04d9d6(pc),a1      | +04a
        jsr     0x4ae.l                         | +04e
        jsr     0x5dd22.l                       | +054
        addi.w  #0x28,0x22(a0)                  | +05a
        addi.w  #0x1,0x24(a0)                   | +060
        lea     0x294234.l,a0                   | +066
        jsr     0x28cd4.l                       | +06c
        lea     .L04d96a(pc),a1                 | +072
        move.l  a1,(a6)                         | +076
.L04d96a:
        jsr     0x2783a.l                       | +078
        jsr     0x28d70.l                       | +07e
        cmpi.b  #0xff,0x21(a6)                  | +084
        bne.w   .L04d986                        | +08a
        lea     Prop_HutWreck_04d996(pc),a1     | +08e
        move.l  a1,(a6)                         | +092
.L04d986:
        jsr     Sub_0004FA70(pc)                | +094
        bcc.w   .L04d994                        | +098
        jmp     0x518.l                         | +09c
.L04d994:
        rts                                     | +0a2

| ----------------------------------------------------------------------------
|  Prop_HutWreck_04d996  @ $04D996  (64 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_HutWreck_04d996, "ax", @progbits
        .global Prop_HutWreck_04d996
Prop_HutWreck_04d996:
        jsr     0x2783a.l                       | +000
        lea     0x295d38.l,a1                   | +006
        jsr     0x43fac.l                       | +00c
        lea     0x294244.l,a0                   | +012
        jsr     0x28cd4.l                       | +018
        lea     .L04d9ba(pc),a1                 | +01e
        move.l  a1,(a6)                         | +022
.L04d9ba:
        jsr     0x2783a.l                       | +024
        jsr     0x28d70.l                       | +02a
        jsr     Sub_0004FA70(pc)                | +030
        bcc.w   .L04d9d4                        | +034
        jmp     0x518.l                         | +038
.L04d9d4:
        rts                                     | +03e

| ----------------------------------------------------------------------------
|  Prop_HutRoof_04d9d6  @ $04D9D6  (124 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_HutRoof_04d9d6, "ax", @progbits
        .global Prop_HutRoof_04d9d6
Prop_HutRoof_04d9d6:
        move.w  #0x3d,d1                        | +000
        jsr     0x236e.l                        | +004
        move.b  #0xff,0x32(a6)                  | +00a
        move.b  #0xff,0x33(a6)                  | +010
        move.b  #0x0,0x3a(a6)                   | +016
        move.w  #0x14,0x66(a6)                  | +01c
        lea     0x294254.l,a0                   | +022
        jsr     0x28cd4.l                       | +028
        lea     .L04da0a(pc),a1                 | +02e
        move.l  a1,(a6)                         | +032
.L04da0a:
        jsr     0x2783a.l                       | +034
        jsr     0x28d70.l                       | +03a
        jsr     0x2870a.l                       | +040
        bcc.w   .L04da32                        | +046
        lea     0x5e766.l,a0                    | +04a
        jsr     0x5e770.l                       | +050
        bclr    #0x3,0x13(a6)                   | +056
.L04da32:
        jsr     0x28758.l                       | +05c
        bcc.w   .L04da42                        | +062
        lea     Prop_HutRoofFall_04da52(pc),a1  | +066
        move.l  a1,(a6)                         | +06a
.L04da42:
        jsr     Sub_0004FA70(pc)                | +06c
        bcc.w   .L04da50                        | +070
        jmp     0x518.l                         | +074
.L04da50:
        rts                                     | +07a

| ----------------------------------------------------------------------------
|  Prop_HutRoofFall_04da52  @ $04DA52  (120 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_HutRoofFall_04da52, "ax", @progbits
        .global Prop_HutRoofFall_04da52
Prop_HutRoofFall_04da52:
        move.l  #0x1000,d0                      | +000
        jsr     0x51a28.l                       | +006
        lea     0x295fba.l,a0                   | +00c
        move.l  a0,0x4c(a6)                     | +012
        jsr     0x283ca.l                       | +016
        jsr     0x283ca.l                       | +01c
        jsr     0x283d8.l                       | +022
        lea     0x29426a.l,a0                   | +028
        jsr     0x28cd4.l                       | +02e
        lea     .L04da8c(pc),a1                 | +034
        move.l  a1,(a6)                         | +038
.L04da8c:
        jsr     0x2783a.l                       | +03a
        jsr     0x28d70.l                       | +040
        bcc.w   SetHandlerRts_04dad0            | +046
        lea     0xffff.w,a0                     | +04a
        move.l  a0,0x4c(a6)                     | +04e
        jsr     0x283ca.l                       | +052
        move.w  #0x1028,d0                      | +058
        jsr     0x2352.l                        | +05c
        movea.l 0xc(a6),a0                      | +062
        move.b  #0xff,0x21(a0)                  | +066
        lea     0x295bd8.l,a1                   | +06c
        jsr     0x77c7e.l                       | +072

| ----------------------------------------------------------------------------
|  Prop_Tower_04dad2  @ $04DAD2  (154 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_Tower_04dad2, "ax", @progbits
        .global Prop_Tower_04dad2
Prop_Tower_04dad2:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        lea     Prop_Roof_04f2c2(pc),a1             | +00a
        jsr     0x4ae.l                         | +00e
        jsr     0x5dd22.l                       | +014
        addi.w  #0x5c,0x22(a0)                  | +01a
        addi.w  #0x22,0x24(a0)                  | +020
        lea     Prop_TowerTop_04db72(pc),a1     | +026
        jsr     0x4ae.l                         | +02a
        jsr     0x5dd22.l                       | +030
        lea     Prop_TowerBase_04dce6(pc),a1    | +036
        jsr     0x4ae.l                         | +03a
        jsr     0x5dd22.l                       | +040
        addi.w  #0x80,0x22(a0)                  | +046
        addi.w  #0x30,0x24(a0)                  | +04c
        move.b  #0x0,0x21(a6)                   | +052
        move.b  #0x0,0x20(a6)                   | +058
        lea     .L04db36(pc),a1                 | +05e
        move.l  a1,(a6)                         | +062
.L04db36:
        cmpi.b  #0xff,0x21(a6)                  | +064
        bne.w   .L04db48                        | +06a
        jmp     0x518.l                         | +06e
        rts                                     | +074
.L04db48:
        move.b  0x20(a6),d0                     | +076
        cmp.b   0x21(a6),d0                     | +07a
        beq.w   .L04db58                        | +07e
        jsr     Sub_0004FA8A(pc)                | +082
.L04db58:
        cmpi.b  #0x12,0x21(a6)                  | +086
        bne.w   .L04db68                        | +08c
        jmp     0x518.l                         | +090
.L04db68:
        move.b  0x21(a6),d0                     | +096

| ----------------------------------------------------------------------------
|  Prop_TowerTop_04db72  @ $04DB72  (208 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_TowerTop_04db72, "ax", @progbits
        .global Prop_TowerTop_04db72
Prop_TowerTop_04db72:
        lea     0xe92b2.l,a1                    | +000
        move.w  #0x75,d0                        | +006
        move.b  #0x0,0x75(a6)                   | +00a
        jsr     0x4429e.l                       | +010
        move.w  #0x50,0x70(a6)                  | +016
        move.w  #0x3c,0x66(a6)                  | +01c
        move.b  #0x0,0x20(a6)                   | +022
        lea     0x294446.l,a0                   | +028
        jsr     0x28cd4.l                       | +02e
        lea     .L04dbac(pc),a1                 | +034
        move.l  a1,(a6)                         | +038
.L04dbac:
        jsr     0x2783a.l                       | +03a
        jsr     0x28d70.l                       | +040
        jsr     0x2870a.l                       | +046
        bcc.w   .L04dbd4                        | +04c
        lea     0x5e766.l,a0                    | +050
        jsr     0x5e770.l                       | +056
        bclr    #0x3,0x13(a6)                   | +05c
.L04dbd4:
        jsr     0x28758.l                       | +062
        bcc.w   .L04dc2a                        | +068
        addi.w  #0x10,0x22(a6)                  | +06c
        lea     0x295b5a.l,a1                   | +072
        jsr     0x77c7e.l                       | +078
        lea     0x295b6c.l,a1                   | +07e
        jsr     0x77c7e.l                       | +084
        lea     0x295cb0.l,a1                   | +08a
        jsr     0x77c7e.l                       | +090
        addi.w  #0x20,0x22(a0)                  | +096
        addi.w  #0x30,0x24(a0)                  | +09c
        movea.l 0xc(a6),a0                      | +0a2
        addi.b  #0x10,0x21(a0)                  | +0a6
        move.b  #0xff,0x20(a6)                  | +0ac
        lea     Prop_TowerTopWreck_04dc4a(pc),a1 | +0b2
        move.l  a1,(a6)                         | +0b6
.L04dc2a:
        jsr     Sub_0004FA70(pc)                | +0b8
        bcc.w   SetHandlerRts_04dc48            | +0bc
        move.b  #0xff,0x20(a6)                  | +0c0
        movea.l 0xc(a6),a0                      | +0c6
        ori.b   #0xf0,0x21(a0)                  | +0ca

| ----------------------------------------------------------------------------
|  Prop_TowerTopWreck_04dc4a  @ $04DC4A  (62 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_TowerTopWreck_04dc4a, "ax", @progbits
        .global Prop_TowerTopWreck_04dc4a
Prop_TowerTopWreck_04dc4a:
        move.l  #0x1000,d0                      | +000
        jsr     0x51a28.l                       | +006
        lea     0xffff.w,a0                     | +00c
        move.l  a0,0x48(a6)                     | +010
        move.b  #0xff,0x75(a6)                  | +014
        move.w  #0x1030,d0                      | +01a
        jsr     0x2352.l                        | +01e
        lea     0x29556a.l,a2                   | +024
        jsr     Sprite_InvokeBlit8Params(pc)    | +02a
        lea     0x2955e2.l,a2                   | +02e
        jsr     Sprite_InvokeBlit8Params(pc)    | +034
        lea     Prop_TowerTopWreckLoop_04dc88(pc),a1 | +038
        move.l  a1,(a6)                         | +03c

| ----------------------------------------------------------------------------
|  Prop_TowerTopWreckLoop_04dc88  @ $04DC88  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_TowerTopWreckLoop_04dc88, "ax", @progbits
        .global Prop_TowerTopWreckLoop_04dc88
Prop_TowerTopWreckLoop_04dc88:
        jsr     0x2783a.l                       | +000

| ----------------------------------------------------------------------------
|  Prop_TowerFlag_04dc96  @ $04DC96  (80 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_TowerFlag_04dc96, "ax", @progbits
        .global Prop_TowerFlag_04dc96
Prop_TowerFlag_04dc96:
        move.w  #0x40,d1                        | +000
        jsr     0x236e.l                        | +004
        move.b  #0xff,0x32(a6)                  | +00a
        move.b  #0xff,0x33(a6)                  | +010
        move.b  #0x0,0x3a(a6)                   | +016
        lea     0x29440a.l,a0                   | +01c
        jsr     0x28cd4.l                       | +022
        lea     .L04dcc4(pc),a1                 | +028
        move.l  a1,(a6)                         | +02c
.L04dcc4:
        jsr     0x2783a.l                       | +02e
        jsr     0x28d70.l                       | +034
        movea.l 0xc(a6),a0                      | +03a
        cmpi.b  #0xff,0x20(a0)                  | +03e
        bne.w   .L04dce4                        | +044
        jmp     0x518.l                         | +048
.L04dce4:
        rts                                     | +04e

| ----------------------------------------------------------------------------
|  Prop_TowerBase_04dce6  @ $04DCE6  (254 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_TowerBase_04dce6, "ax", @progbits
        .global Prop_TowerBase_04dce6
Prop_TowerBase_04dce6:
        lea     0x5f384.l,a1                    | +000
        jsr     0x4ae.l                         | +006
        jsr     0x5dd22.l                       | +00c
        addi.w  #0x29,0x22(a0)                  | +012
        addi.w  #0x9,0x24(a0)                   | +018
        move.w  #0x50,0x70(a6)                  | +01e
        move.w  #0x3c,0x66(a6)                  | +024
        move.b  #0x0,0x21(a6)                   | +02a
        move.b  #0x0,0x20(a6)                   | +030
        lea     0x29445c.l,a0                   | +036
        jsr     0x28cd4.l                       | +03c
        lea     .L04dd2e(pc),a1                 | +042
        move.l  a1,(a6)                         | +046
.L04dd2e:
        jsr     0x2783a.l                       | +048
        jsr     0x28d70.l                       | +04e
        cmpi.b  #0x1,0x20(a6)                   | +054
        bge.w   .L04dd84                        | +05a
        cmpi.w  #0x1e,0x66(a6)                  | +05e
        bgt.w   .L04dd84                        | +064
        move.w  #0x1028,d0                      | +068
        jsr     0x2352.l                        | +06c
        lea     0x295ba2.l,a1                   | +072
        jsr     0x77c7e.l                       | +078
        lea     0x2955f6.l,a2                   | +07e
        jsr     Sprite_InvokeBlit8Params(pc)    | +084
        move.b  #0xff,0x21(a6)                  | +088
        movea.l 0xc(a6),a0                      | +08e
        addi.b  #0x1,0x21(a0)                   | +092
        move.b  #0x1,0x20(a6)                   | +098
.L04dd84:
        jsr     0x2870a.l                       | +09e
        bcc.w   .L04dda0                        | +0a4
        lea     0x5e766.l,a0                    | +0a8
        jsr     0x5e770.l                       | +0ae
        bclr    #0x3,0x13(a6)                   | +0b4
.L04dda0:
        jsr     0x28758.l                       | +0ba
        bcc.w   .L04ddd2                        | +0c0
        lea     0x295ba2.l,a1                   | +0c4
        jsr     0x77c7e.l                       | +0ca
        lea     0x295bb4.l,a1                   | +0d0
        jsr     0x77c7e.l                       | +0d6
        movea.l 0xc(a6),a0                      | +0dc
        addi.b  #0x1,0x21(a0)                   | +0e0
        lea     Prop_TowerBaseWreck_04ddec(pc),a1 | +0e6
        move.l  a1,(a6)                         | +0ea
.L04ddd2:
        jsr     Sub_0004FA70(pc)                | +0ec
        bcc.w   SetHandlerRts_04ddea            | +0f0
        movea.l 0xc(a6),a0                      | +0f4
        ori.b   #0xf,0x21(a0)                   | +0f8

| ----------------------------------------------------------------------------
|  Prop_TowerBaseWreck_04ddec  @ $04DDEC  (70 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_TowerBaseWreck_04ddec, "ax", @progbits
        .global Prop_TowerBaseWreck_04ddec
Prop_TowerBaseWreck_04ddec:
        move.l  #0x1000,d0                      | +000
        jsr     0x51a28.l                       | +006
        lea     0xffff.w,a0                     | +00c
        move.l  a0,0x48(a6)                     | +010
        move.w  #0x1028,d0                      | +014
        jsr     0x2352.l                        | +018
        lea     0x29560a.l,a2                   | +01e
        jsr     Sprite_InvokeBlit8Params(pc)    | +024
        lea     0x295cb0.l,a1                   | +028
        jsr     0x77c7e.l                       | +02e
        addi.w  #0xffe0,0x22(a0)                | +034
        addi.w  #0x30,0x24(a0)                  | +03a
        lea     Prop_TowerBaseWreckLoop_04de32(pc),a1 | +040
        move.l  a1,(a6)                         | +044

| ----------------------------------------------------------------------------
|  Prop_TowerBaseWreckLoop_04de32  @ $04DE32  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_TowerBaseWreckLoop_04de32, "ax", @progbits
        .global Prop_TowerBaseWreckLoop_04de32
Prop_TowerBaseWreckLoop_04de32:
        jsr     0x2783a.l                       | +000

| ----------------------------------------------------------------------------
|  Prop_Bunker_04de40  @ $04DE40  (234 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_Bunker_04de40, "ax", @progbits
        .global Prop_Bunker_04de40
Prop_Bunker_04de40:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        lea     0xe92ea.l,a1                    | +00a
        move.w  #0x75,d0                        | +010
        move.b  #0x0,0x75(a6)                   | +014
        jsr     0x4429e.l                       | +01a
        move.w  #0x70,0x70(a6)                  | +020
        move.w  #0x3c,0x66(a6)                  | +026
        move.b  #0x0,0x20(a6)                   | +02c
        move.b  #0x0,0x21(a6)                   | +032
        lea     0x294472.l,a0                   | +038
        jsr     0x28cd4.l                       | +03e
        lea     .L04de8a(pc),a1                 | +044
        move.l  a1,(a6)                         | +048
.L04de8a:
        jsr     0x2783a.l                       | +04a
        jsr     0x28d70.l                       | +050
        jsr     0x2870a.l                       | +056
        bcc.w   .L04deb2                        | +05c
        lea     0x5e766.l,a0                    | +060
        jsr     0x5e770.l                       | +066
        bclr    #0x3,0x13(a6)                   | +06c
.L04deb2:
        jsr     0x28758.l                       | +072
        bcc.w   .L04df0a                        | +078
        move.b  #0xff,0x75(a6)                  | +07c
        addi.b  #0x10,0x21(a6)                  | +082
        move.b  #0xff,0x20(a6)                  | +088
        addi.w  #0x10,0x22(a6)                  | +08e
        lea     0x295b5a.l,a1                   | +094
        jsr     0x77c7e.l                       | +09a
        lea     0x295b6c.l,a1                   | +0a0
        jsr     0x77c7e.l                       | +0a6
        lea     0x295cb0.l,a1                   | +0ac
        jsr     0x77c7e.l                       | +0b2
        addi.w  #0x20,0x22(a0)                  | +0b8
        addi.w  #0x20,0x24(a0)                  | +0be
        lea     Prop_BunkerWreck_04df30(pc),a1  | +0c4
        move.l  a1,(a6)                         | +0c8
.L04df0a:
        jsr     Sub_0004FA70(pc)                | +0ca
        bcc.w   .L04df1e                        | +0ce
        move.b  #0xff,0x20(a6)                  | +0d2
        lea     Prop_BunkerWreck_04df30__L04df72(pc),a1 | +0d8
        move.l  a1,(a6)                         | +0dc
.L04df1e:
        move.b  0x74(a6),d0                     | +0de
        cmp.b   0x21(a6),d0                     | +0e2
        beq.w   JsrPcRts_04df2e                 | +0e6

| ----------------------------------------------------------------------------
|  Prop_BunkerWreck_04df30  @ $04DF30  (98 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_BunkerWreck_04df30, "ax", @progbits
        .global Prop_BunkerWreck_04df30
Prop_BunkerWreck_04df30:
        move.l  #0x1000,d0                      | +000
        jsr     0x51a28.l                       | +006
        lea     0xffff.w,a0                     | +00c
        move.l  a0,0x48(a6)                     | +010
        move.w  #0x1030,d0                      | +014
        jsr     0x2352.l                        | +018
        cmpi.w  #0x0,0x22(a6)                   | +01e
        bmi.w   .L04df62                        | +024
        lea     0x29561e.l,a2                   | +028
        bra.w   .L04df68                        | +02e
.L04df62:
        lea     0x295632.l,a2                   | +032
.L04df68:
        jsr     Sprite_InvokeBlit8Params(pc)    | +038
        lea     .L04df72(pc),a1                 | +03c
        move.l  a1,(a6)                         | +040
        .global Prop_BunkerWreck_04df30__L04df72
Prop_BunkerWreck_04df30__L04df72:
.L04df72:
        jsr     0x2783a.l                       | +042
        jsr     Sub_0004FA70(pc)                | +048
        bcc.w   .L04df86                        | +04c
        lea     Prop_SlotPrioCheckRts_04f2a4(pc),a1       | +050
        move.l  a1,(a6)                         | +054
.L04df86:
        move.b  0x74(a6),d0                     | +056
        cmp.b   0x21(a6),d0                     | +05a
        beq.w   JsrPcRts_04df96                 | +05e

| ----------------------------------------------------------------------------
|  Prop_Bridge_04df98  @ $04DF98  (198 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_Bridge_04df98, "ax", @progbits
        .global Prop_Bridge_04df98
Prop_Bridge_04df98:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        lea     Prop_BridgePillar_04e12c(pc),a1 | +00a
        jsr     0x4ae.l                         | +00e
        jsr     0x5dd22.l                       | +014
        lea     Prop_BridgePillar_04e12c(pc),a1 | +01a
        jsr     0x4ae.l                         | +01e
        jsr     0x5dd22.l                       | +024
        addi.w  #0xd0,0x22(a0)                  | +02a
        ori.b   #0x1,0x3a(a0)                   | +030
        move.w  #0x60,0x70(a6)                  | +036
        move.b  #0x0,0x21(a6)                   | +03c
        lea     0xe9310.l,a1                    | +042
        move.w  #0x75,d0                        | +048
        move.b  #0x0,0x75(a6)                   | +04c
        jsr     0x4429e.l                       | +052
        lea     .L04dff6(pc),a1                 | +058
        move.l  a1,(a6)                         | +05c
.L04dff6:
        jsr     0x2783a.l                       | +05e
        cmpi.b  #0x11,0x21(a6)                  | +064
        bne.w   .L04e04e                        | +06a
        move.w  #0x1030,d0                      | +06e
        jsr     0x2352.l                        | +072
        lea     0x295bea.l,a1                   | +078
        jsr     0x77c7e.l                       | +07e
        lea     0x295bfc.l,a1                   | +084
        jsr     0x77c7e.l                       | +08a
        lea     0x295cc2.l,a1                   | +090
        jsr     0x77c7e.l                       | +096
        lea     0x2956aa.l,a2                   | +09c
        jsr     Sprite_InvokeBlit8Params(pc)    | +0a2
        lea     0x2956e6.l,a2                   | +0a6
        jsr     Sprite_InvokeBlit8Params(pc)    | +0ac
        lea     Prop_BridgeCollapse_04e05e(pc),a1 | +0b0
        move.l  a1,(a6)                         | +0b4
.L04e04e:
        jsr     Sub_0004FA70(pc)                | +0b6
        bcc.w   .L04e05c                        | +0ba
        jmp     0x518.l                         | +0be
.L04e05c:
        rts                                     | +0c4

| ----------------------------------------------------------------------------
|  Prop_BridgeCollapse_04e05e  @ $04E05E  (206 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_BridgeCollapse_04e05e, "ax", @progbits
        .global Prop_BridgeCollapse_04e05e
Prop_BridgeCollapse_04e05e:
        move.w  #0x3c,0x66(a6)                  | +000
        lea     0x294488.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     .L04e076(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L04e076:
        jsr     0x2783a.l                       | +018
        jsr     0x28d70.l                       | +01e
        jsr     0x2870a.l                       | +024
        bcc.w   .L04e09e                        | +02a
        lea     0x5e766.l,a0                    | +02e
        jsr     0x5e770.l                       | +034
        bclr    #0x3,0x13(a6)                   | +03a
.L04e09e:
        jsr     0x28758.l                       | +040
        bcc.w   .L04e116                        | +046
        lea     0xffff.w,a0                     | +04a
        move.l  a0,0x48(a6)                     | +04e
        move.l  #0x5000,d0                      | +052
        jsr     0x51a28.l                       | +058
        move.w  #0x1028,d0                      | +05e
        jsr     0x2352.l                        | +062
        lea     0x295c0e.l,a1                   | +068
        jsr     0x77c7e.l                       | +06e
        lea     0x295c20.l,a1                   | +074
        jsr     0x77c7e.l                       | +07a
        lea     0x295cc2.l,a1                   | +080
        jsr     0x77c7e.l                       | +086
        lea     0x2956be.l,a2                   | +08c
        jsr     Sprite_InvokeBlit8Params(pc)    | +092
        lea     0x2956fa.l,a2                   | +096
        jsr     Sprite_InvokeBlit8Params(pc)    | +09c
        lea     0x295722.l,a2                   | +0a0
        jsr     Sprite_InvokeBlit8Params(pc)    | +0a6
        lea     0x29574a.l,a2                   | +0aa
        jsr     Sprite_InvokeBlit8Params(pc)    | +0b0
        bra.w   .L04e11e                        | +0b4
.L04e116:
        jsr     Sub_0004FA70(pc)                | +0b8
        bcc.w   .L04e12a                        | +0bc
.L04e11e:
        move.b  #0xff,0x75(a6)                  | +0c0
        jmp     0x518.l                         | +0c6
.L04e12a:
        rts                                     | +0cc

| ----------------------------------------------------------------------------
|  Prop_BridgePillar_04e12c  @ $04E12C  (284 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_BridgePillar_04e12c, "ax", @progbits
        .global Prop_BridgePillar_04e12c
Prop_BridgePillar_04e12c:
        lea     0x5f38a.l,a1                    | +000
        jsr     0x4ae.l                         | +006
        jsr     0x5dd22.l                       | +00c
        btst    #0x0,0x3a(a6)                   | +012
        bne.w   .L04e158                        | +018
        addi.w  #0x2b,0x22(a0)                  | +01c
        subi.w  #0x17,0x24(a0)                  | +022
        bra.w   .L04e164                        | +028
.L04e158:
        subi.w  #0x14,0x22(a0)                  | +02c
        subi.w  #0x17,0x24(a0)                  | +032
.L04e164:
        move.b  #0x0,0x21(a6)                   | +038
        move.w  #0x3c,0x66(a6)                  | +03e
        move.w  #0x70,0x70(a6)                  | +044
        lea     0x29449e.l,a0                   | +04a
        jsr     0x28cd4.l                       | +050
        lea     .L04e188(pc),a1                 | +056
        move.l  a1,(a6)                         | +05a
.L04e188:
        jsr     0x2783a.l                       | +05c
        jsr     0x28d70.l                       | +062
        jsr     0x2870a.l                       | +068
        bcc.w   .L04e1b0                        | +06e
        lea     0x5e766.l,a0                    | +072
        jsr     0x5e770.l                       | +078
        bclr    #0x3,0x13(a6)                   | +07e
.L04e1b0:
        jsr     0x28758.l                       | +084
        bcc.w   .L04e238                        | +08a
        move.w  #0x102f,d0                      | +08e
        jsr     0x2352.l                        | +092
        lea     0xffff.w,a0                     | +098
        move.l  a0,0x48(a6)                     | +09c
        lea     0x295b7e.l,a1                   | +0a0
        jsr     0x77c7e.l                       | +0a6
        subi.w  #0x20,0x24(a0)                  | +0ac
        lea     0x295b90.l,a1                   | +0b2
        jsr     0x77c7e.l                       | +0b8
        subi.w  #0x20,0x24(a0)                  | +0be
        lea     0x295cb0.l,a1                   | +0c4
        jsr     0x77c7e.l                       | +0ca
        move.w  #0x20,d0                        | +0d0
        btst    #0x0,0x3a(a6)                   | +0d4
        beq.w   .L04e20c                        | +0da
        neg.w   d0                              | +0de
.L04e20c:
        add.w   d0,0x22(a0)                     | +0e0
        jsr     Sub_0004FB3C(pc)                | +0e4
        movea.l 0xc(a6),a0                      | +0e8
        move.b  #0x10,d0                        | +0ec
        btst    #0x0,0x3a(a6)                   | +0f0
        beq.w   .L04e22a                        | +0f6
        move.b  #0x1,d0                         | +0fa
.L04e22a:
        or.b    d0,0x21(a0)                     | +0fe
        move.b  #0xff,0x21(a6)                  | +102
        bra.w   .L04e240                        | +108
.L04e238:
        jsr     Sub_0004FA70(pc)                | +10c
        bcc.w   .L04e246                        | +110
.L04e240:
        jmp     0x518.l                         | +114
.L04e246:
        rts                                     | +11a

| ----------------------------------------------------------------------------
|  Prop_Nest_04e248  @ $04E248  (96 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_Nest_04e248, "ax", @progbits
        .global Prop_Nest_04e248
Prop_Nest_04e248:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x40,d1                        | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0x70,0x70(a6)                  | +014
        move.w  #0x12c,0x66(a6)                 | +01a
        lea     0x29441a.l,a0                   | +020
        jsr     0x28cd4.l                       | +026
        lea     0x8e738.l,a1                    | +02c
        jsr     0x4ae.l                         | +032
        jsr     0x5dd22.l                       | +038
        addi.w  #0x98,0x22(a0)                  | +03e
        addi.w  #0x38,0x24(a0)                  | +044
        lea     .L04e298(pc),a1                 | +04a
        move.l  a1,(a6)                         | +04e
.L04e298:
        cmpi.w  #0x110,0x22(a6)                 | +050
        blt.w   Prop_NestActive_04e2b0          | +056
        jsr     0x2783a.l                       | +05a

| ----------------------------------------------------------------------------
|  Prop_NestActive_04e2b0  @ $04E2B0  (124 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_NestActive_04e2b0, "ax", @progbits
        .global Prop_NestActive_04e2b0
Prop_NestActive_04e2b0:
        lea     .L04e2b6(pc),a1                 | +000
        move.l  a1,(a6)                         | +004
.L04e2b6:
        jsr     0x2783a.l                       | +006
        jsr     0x28d70.l                       | +00c
        jsr     0x2870a.l                       | +012
        bcc.w   .L04e2de                        | +018
        lea     0x5e766.l,a0                    | +01c
        jsr     0x5e770.l                       | +022
        bclr    #0x3,0x13(a6)                   | +028
.L04e2de:
        jsr     0x28758.l                       | +02e
        bcc.w   .L04e31c                        | +034
        move.w  #0x102f,d0                      | +038
        jsr     0x2352.l                        | +03c
        lea     0x295c56.l,a1                   | +042
        jsr     0x77c7e.l                       | +048
        lea     0x295c68.l,a1                   | +04e
        jsr     0x77c7e.l                       | +054
        lea     0x295ce6.l,a1                   | +05a
        jsr     0x77c7e.l                       | +060
        lea     Prop_NestWreck_04e32c(pc),a1    | +066
        move.l  a1,(a6)                         | +06a
.L04e31c:
        jsr     Sub_0004FA70(pc)                | +06c
        bcc.w   .L04e32a                        | +070
        jmp     0x518.l                         | +074
.L04e32a:
        rts                                     | +07a

| ----------------------------------------------------------------------------
|  Prop_NestWreck_04e32c  @ $04E32C  (94 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_NestWreck_04e32c, "ax", @progbits
        .global Prop_NestWreck_04e32c
Prop_NestWreck_04e32c:
        move.l  #0x3000,d0                      | +000
        jsr     0x51a28.l                       | +006
        lea     0xffff.w,a0                     | +00c
        move.l  a0,0x48(a6)                     | +010
        jsr     0x2783a.l                       | +014
        lea     0x295d78.l,a1                   | +01a
        jsr     0x43fac.l                       | +020
        move.w  #0x1030,d0                      | +026
        jsr     0x2352.l                        | +02a
        lea     0x294430.l,a0                   | +030
        jsr     0x28cd4.l                       | +036
        lea     .L04e36e(pc),a1                 | +03c
        move.l  a1,(a6)                         | +040
.L04e36e:
        jsr     0x2783a.l                       | +042
        jsr     0x28d70.l                       | +048
        jsr     Sub_0004FA70(pc)                | +04e
        bcc.w   .L04e388                        | +052
        jmp     0x518.l                         | +056
.L04e388:
        rts                                     | +05c

| ----------------------------------------------------------------------------
|  Prop_Shed_04e38a  @ $04E38A  (198 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_Shed_04e38a, "ax", @progbits
        .global Prop_Shed_04e38a
Prop_Shed_04e38a:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        lea     Prop_Roof_04f2c2(pc),a1             | +00a
        jsr     0x4ae.l                         | +00e
        jsr     0x5dd22.l                       | +014
        addi.w  #0x40,0x22(a0)                  | +01a
        addi.w  #0x49,0x24(a0)                  | +020
        move.w  #0x70,0x70(a6)                  | +026
        move.w  #0x3c,d0                        | +02c
        asr.w   #0x1,d0                         | +030
        move.w  d0,0x66(a6)                     | +032
        lea     0x2944b4.l,a0                   | +036
        jsr     0x28cd4.l                       | +03c
        lea     .L04e3d2(pc),a1                 | +042
        move.l  a1,(a6)                         | +046
.L04e3d2:
        jsr     0x2783a.l                       | +048
        jsr     0x28d70.l                       | +04e
        jsr     0x2870a.l                       | +054
        bcc.w   .L04e3fa                        | +05a
        lea     0x5e766.l,a0                    | +05e
        jsr     0x5e770.l                       | +064
        bclr    #0x3,0x13(a6)                   | +06a
.L04e3fa:
        jsr     0x28758.l                       | +070
        bcc.w   .L04e440                        | +076
        bclr    #0x0,0x13(a6)                   | +07a
        lea     0x295ba2.l,a1                   | +080
        jsr     0x77c7e.l                       | +086
        addi.w  #0x20,0x22(a0)                  | +08c
        addi.w  #0x30,0x24(a0)                  | +092
        lea     0x295cb0.l,a1                   | +098
        jsr     0x77c7e.l                       | +09e
        addi.w  #0x20,0x22(a0)                  | +0a4
        addi.w  #0x30,0x24(a0)                  | +0aa
        lea     Prop_ShedDamaged_04e450(pc),a1  | +0b0
        move.l  a1,(a6)                         | +0b4
.L04e440:
        jsr     Sub_0004FA70(pc)                | +0b6
        bcc.w   .L04e44e                        | +0ba
        jmp     0x518.l                         | +0be
.L04e44e:
        rts                                     | +0c4

| ----------------------------------------------------------------------------
|  Prop_ShedDamaged_04e450  @ $04E450  (186 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_ShedDamaged_04e450, "ax", @progbits
        .global Prop_ShedDamaged_04e450
Prop_ShedDamaged_04e450:
        move.w  #0x1028,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x295786.l,a2                   | +00a
        jsr     Sprite_InvokeBlit8Params(pc)    | +010
        move.w  #0x3c,d0                        | +014
        asr.w   #0x1,d0                         | +018
        move.w  d0,0x66(a6)                     | +01a
        lea     .L04e474(pc),a1                 | +01e
        move.l  a1,(a6)                         | +022
.L04e474:
        jsr     0x2783a.l                       | +024
        jsr     0x28d70.l                       | +02a
        jsr     0x2870a.l                       | +030
        bcc.w   .L04e49c                        | +036
        lea     0x5e766.l,a0                    | +03a
        jsr     0x5e770.l                       | +040
        bclr    #0x3,0x13(a6)                   | +046
.L04e49c:
        jsr     0x28758.l                       | +04c
        bcc.w   .L04e502                        | +052
        move.l  #0x1000,d0                      | +056
        jsr     0x51a28.l                       | +05c
        move.w  #0x1030,d0                      | +062
        jsr     0x2352.l                        | +066
        lea     0xffff.w,a0                     | +06c
        move.l  a0,0x48(a6)                     | +070
        lea     0x295ba2.l,a1                   | +074
        jsr     0x77c7e.l                       | +07a
        addi.w  #0x20,0x22(a0)                  | +080
        addi.w  #0x30,0x24(a0)                  | +086
        lea     0x295cb0.l,a1                   | +08c
        jsr     0x77c7e.l                       | +092
        addi.w  #0x20,0x22(a0)                  | +098
        addi.w  #0x30,0x24(a0)                  | +09e
        lea     0x29579a.l,a2                   | +0a4
        jsr     Sprite_InvokeBlit8Params(pc)    | +0aa
        jsr     Prop_FreeOrRts_04e50a(pc)       | +0ae
.L04e502:
        jsr     Sub_0004FA70(pc)                | +0b2
        bcc.w   Prop_FreeOrRts_04e50a__L04e510  | +0b6

| ----------------------------------------------------------------------------
|  Prop_FreeOrRts_04e50a  @ $04E50A  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_FreeOrRts_04e50a, "ax", @progbits
        .global Prop_FreeOrRts_04e50a
Prop_FreeOrRts_04e50a:
        jmp     0x518.l                         | +000
        .global Prop_FreeOrRts_04e50a__L04e510
Prop_FreeOrRts_04e50a__L04e510:
.L04e510:
        rts                                     | +006

| ----------------------------------------------------------------------------
|  Prop_Barrier_04e512  @ $04E512  (102 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_Barrier_04e512, "ax", @progbits
        .global Prop_Barrier_04e512
Prop_Barrier_04e512:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x40,d1                        | +00a
        jsr     0x236e.l                        | +00e
        lea     0x2944ca.l,a0                   | +014
        jsr     0x28cd4.l                       | +01a
        bset    #0x6,0x12(a6)                   | +020
        move.l  #0x296272,0x60(a6)              | +026
        lea     Prop_BarrierPost_04ed90(pc),a1             | +02e
        jsr     0x4ae.l                         | +032
        jsr     0x5dd22.l                       | +038
        addi.w  #0x10,0x24(a0)                  | +03e
        lea     .L04e55c(pc),a1                 | +044
        move.l  a1,(a6)                         | +048
.L04e55c:
        jsr     0x2783a.l                       | +04a
        move.l  0x106f50.l,d0                   | +050
        swap    d0                              | +056
        cmpi.w  #0xa10,d0                       | +058
        bgt.w   Prop_BarrierActive_04e580                    | +05c
        jsr     0x28998.l                       | +060
