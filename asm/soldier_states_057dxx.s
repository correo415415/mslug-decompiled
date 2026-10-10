| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave EEEE — soldado rebelde (infantería): agarre al player, carrera,
|              pasos, huida, rendición, granadas, burla y entrada en escena
|  Región: $057D04..$059342  (5,694 B, 54 entradas, 1 huecos)
| ============================================================================
|
|  A. RESUMEN
|  ----------
|  Región de 5,694 B (54 entradas): máquina de estados del soldado rebelde
|  estándar (infantería de la misión 1 en adelante). Cada estado es una
|  rutina a la que apunta (a6) y que se re-engancha a sí misma o a la
|  siguiente con `lea X(pc),a1 / move.l a1,(a6)`. Depende del clúster de
|  helpers inmediatamente anterior ($056ACC..$057D04: Soldier_PhysicsStep_056acc/56B92/
|  56E36/56F64/56F8A/56FA0/56FEC/5740E/574E8/57CA8, Soldier_DespawnIfOffscreen_056e1e),
|  pendiente de la Wave FFFF.
|
|   1. Agarre al player ($57D04..$5804C): Soldier_GrabPlayer toma el ancla
|      del player enlazado en +$7A (PlayerSlot_ClaimAnchor $8F85C, modo 0);
|      GrabStruggle/GrabStruggleNext alternan animación mientras el player
|      forcejea; GrabBreakA/B/C y GrabBreak sueltan al player (snd $20 si el
|      enlazado es $100440, $14B en otro caso) y GrabThrownA/B lanzan al
|      soldado por los aires (velocidad +$28/+$2A, gravedad +$2E) hasta
|      tocar suelo.
|   2. Locomoción ($5804C..$58144): RunToward gira hacia el objetivo
|      (facing +$3A) y entra en Run_Loop; RunByTable toma la velocidad de
|      una tabla indexada por +$98.
|   3. Idle/IdleFidget ($58144..$58412): espera con cambios aleatorios
|      (RNG $5E9B6 contra umbrales en +$80..+$8A) entre fidget, pasos,
|      ataque cuerpo a cuerpo, retirada y burla.
|   4. Hurt/Hurt_Loop/Land ($58412..$584C4): impacto recibido, caída y
|      aterrizaje (test de suelo vía Soldier_Think_056b92).
|   5. StepRight/StepLeft/Step_Loop ($584C4..$585AE): pasos laterales
|      cortos; MeleeAttack ($585AE) golpe de cuchillo con tabla de ataque
|      +$4C (Attack_* $283CA); Brake ($585F6) frenado.
|   6. Flee/FleeStop/RetreatJmp/Retreat/Retreat_Loop/Stand/Jump
|      ($58658..$58968): huida corriendo, retirada andando hacia atrás,
|      quedarse de pie y salto con parábola.
|   7. Surrender/SurrenderFlee ($58968..$58B1E): rendición (brazos arriba,
|      flag +$73) y posterior huida corriendo sin colisión.
|   8. Granadas ($58B1E..$58DF8): ThrowGrenadeA/B (dos posturas) con
|      Loop/Recover y ThrowGrenadeAim que ajusta ángulo +$80 según la
|      distancia al player; HopBack salto corto hacia atrás.
|   9. Burla ($58DF8..$58F1E): Soldier_TauntAnimPtrTbl (4 punteros a
|      animación), TauntInit/Taunt/TauntEnd.
|  10. Entrada en escena ($58F1E..$59332): Soldier_SpawnVariantTbl es la
|      tabla de variantes referida desde MeleeGuard_DeathToExtern_0427CA;
|      SpawnFaceTarget/SpawnEnter/SpawnWait/SpawnStand(+Loop)/SpawnBrake/
|      SpawnLeap/SpawnHurt/SpawnRecover cubren la aparición (caer desde
|      arriba, saltar desde un lateral, esperar al scroll).
|  11. Entity_CmpField10WithLink8_059332: helper hoja; compara +$10 del
|      objeto con +$8 del enlazado (orden/profundidad).
|
|  B. EVIDENCIAS
|  -------------
|  - +$7A se compara con #$100440 (slot del player 1) antes de elegir el
|    sonido $20/$14B: +$7A es el puntero al player agarrado.
|  - jsr $8F85C (PlayerSlot_ClaimAnchor) con a0 = +$7A y d0 = 0 en
|    GrabPlayer/GrabStruggle*: reclama/valida el ancla del player; `bcc`
|    => el player se soltó -> GrabThrownB.
|  - 23 llamadas a $5E9B6 (RNG) seguidas de `cmp.b +$80..+$8A(a6)`: los
|    umbrales de comportamiento por dificultad viven en +$80..+$8A.
|  - 52 x $28CD4 (sprite) y 40 x $28D70 (anim): cada estado fija sprite y
|    avanza animación por frame.
|  - 32 x $49FD0 (Entity_ProbeAndInstallHandler): transición por sondeo
|    del entorno (colisión con el player / fuera de pantalla).
|  - $58DF8 contiene 4 longs apuntando dentro de $2Bxxxx (banco de
|    animaciones) -> tabla de punteros, excluida del código con --data.
|
|  C. CAMPOS DEL OBJETO (a6)
|  -------------------------
|  +$00 handler  +$20 contador  +$22/+$24 x/y  +$28/+$2A vel  +$2E grav
|  +$3A facing (bit 0)  +$4C tabla de ataque  +$60 hitbox  +$66 HP
|  +$72/+$73/+$74 flags  +$75 subestado  +$7A ptr player enlazado
|  +$80 ángulo/umbral RNG  +$82 groundY  +$84..+$8A umbrales RNG
|  +$94 índice anim  +$98 parámetro de spawn (variante)
|
|  D. HELPERS EXTERNOS
|  -------------------
|  $236E snd  $2352 música  $13600  $27EBA/$27F60 (ptr objeto / lista)
|  $2783A  $282D8/$2831E/$28364  $2870A  $283CA attack  $28CD4 sprite
|  $28D70 anim  $49FD0/$49FF2 probe+install  $5DCA4 rand escalado
|  $5E9B6 RNG  $77190  $799DE  $8F308  $8F85C/$8F8C2 PlayerSlot_*
|  $56ACC..$57CA8 helpers del soldado (Wave FFFF).
|
|  E. HIPÓTESIS
|  ------------
|  - "Soldier" = rebelde de infantería genérico; las variantes de spawn
|    y la tabla $58F1E sugieren que el mismo código sirve a varios tipos
|    (cuchillo, granada, escudo) seleccionados por +$98.
|  - Taunt (burla) se elige desde Idle con baja probabilidad; nombre por
|    la tabla de 4 animaciones y el retorno a Idle sin efectos.
|
|  F. SIGUIENTE
|  ------------
|  $056ACC..$057D04 (helpers del soldado), $0527BA..$0539E2,
|  $0478FC..$048A3C.
|
|  Verificación: cada sección .text.<Sym> se coloca en su dirección CPU
|  absoluta y reensambla byte-exacta contra build/mslug_prom.bin
|  (MD5 816b3f74c76b3373993407615f1850fe).
| ============================================================================

        .text

| ----------------------------------------------------------------------------
|  Soldier_GrabPlayer_057d04  @ $057D04  (290 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_GrabPlayer_057d04, "ax", @progbits
        .global Soldier_GrabPlayer_057d04
Soldier_GrabPlayer_057d04:
        bclr    #0x0,0x3a(a6)                   | +000
        movea.l 0x7a(a6),a0                     | +006
        move.w  #0x0,d0                         | +00a
        jsr     0x8f85c.l                       | +00e
        bcc.w   Soldier_GrabThrownB_057fc6      | +014
        lea     0x2b6d64.l,a0                   | +018
        jsr     0x28cd4.l                       | +01e
        clr.b   0x20(a6)                        | +024
        lea     .L057d32(pc),a1                 | +028
        move.l  a1,(a6)                         | +02c
.L057d32:
        bsr.w   Soldier_GrabHoldFlag_057ca8                    | +02e
        jsr     Soldier_TestGrabBreak_056f64(pc)                | +032
        bcc.w   .L057d44                        | +036
        lea     Soldier_GrabBreakA_057ece(pc),a1 | +03a
        move.l  a1,(a6)                         | +03e
.L057d44:
        jsr     0x28d70.l                       | +040
        bcc.w   .L057d54                        | +046
        lea     Soldier_GrabThrownB_057fc6(pc),a1 | +04a
        move.l  a1,(a6)                         | +04e
.L057d54:
        tst.b   0x20(a6)                        | +050
        beq.w   .L057d6a                        | +054
        movea.l 0x7a(a6),a0                     | +058
        move.w  #0x0,d0                         | +05c
        jsr     0x8f8c2.l                       | +060
.L057d6a:
        jmp     Soldier_HitCheckTail_057ae4(pc)                | +066
        .global Soldier_GrabPlayer_057d04__L057d6e
Soldier_GrabPlayer_057d04__L057d6e:
        bclr    #0x0,0x3a(a6)                   | +06a
        movea.l 0x7a(a6),a0                     | +070
        move.w  #0x1,d0                         | +074
        jsr     0x8f85c.l                       | +078
        bcc.w   Soldier_GrabThrownB_057fc6      | +07e
        jsr     0x13600.l                       | +082
        lea     0x2b6aa4.l,a0                   | +088
        jsr     0x28cd4.l                       | +08e
        move.l  0x7a(a6),d0                     | +094
        cmpi.l  #0x100440,d0                    | +098
        bne.w   .L057dc4                        | +09e
        move.w  #0x20,d1                        | +0a2
        jsr     0x236e.l                        | +0a6
        lea     0x2b7284.l,a0                   | +0ac
        move.l  a0,0x4c(a6)                     | +0b2
        jsr     0x283ca.l                       | +0b6
        bra.w   .L057dde                        | +0bc
.L057dc4:
        move.w  #0x14b,d1                       | +0c0
        jsr     0x236e.l                        | +0c4
        lea     0x2b72d8.l,a0                   | +0ca
        move.l  a0,0x4c(a6)                     | +0d0
        jsr     0x283ca.l                       | +0d4
.L057dde:
        lea     .L057de4(pc),a1                 | +0da
        move.l  a1,(a6)                         | +0de
.L057de4:
        bsr.w   Soldier_GrabHoldFlag_057ca8                    | +0e0
        jsr     Soldier_TestGrabBreak_056f64(pc)                | +0e4
        bcc.w   .L057df6                        | +0e8
        lea     Soldier_GrabBreakB_057ee2(pc),a1 | +0ec
        move.l  a1,(a6)                         | +0f0
.L057df6:
        jsr     0x28d70.l                       | +0f2
        bcc.w   .L057e06                        | +0f8
        lea     Soldier_GrabThrownB_057fc6(pc),a1 | +0fc
        move.l  a1,(a6)                         | +100
.L057e06:
        jmp     Soldier_HitCheckTail_057ae4(pc)                | +102
        .global Soldier_GrabPlayer_057d04__L057e0a
Soldier_GrabPlayer_057d04__L057e0a:
        bclr    #0x0,0x3a(a6)                   | +106
        movea.l 0x7a(a6),a0                     | +10c
        move.w  #0x2,d0                         | +110
        jsr     0x8f85c.l                       | +114
        bcc.w   Soldier_GrabThrownB_057fc6      | +11a
        clr.b   0x30(a6)                        | +11e

| ----------------------------------------------------------------------------
|  Soldier_GrabStruggle_057e26  @ $057E26  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_GrabStruggle_057e26, "ax", @progbits
        .global Soldier_GrabStruggle_057e26
Soldier_GrabStruggle_057e26:
        lea     0x2b6832.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        bclr    #0x3,0x13(a6)                   | +00c
        bclr    #0x0,0x13(a6)                   | +012
        lea     .L057e44(pc),a1                 | +018
        move.l  a1,(a6)                         | +01c
.L057e44:
        bsr.w   Soldier_GrabHoldFlag_057ca8                    | +01e
        jsr     0x28d70.l                       | +022
        jsr     Soldier_TestGrabBreak_056f64(pc)                | +028
        bcc.w   .L057e5c                        | +02c
        lea     Soldier_GrabBreakC_057ef6(pc),a1 | +030
        move.l  a1,(a6)                         | +034
.L057e5c:
        jsr     0x2870a.l                       | +036
        bcc.w   .L057e76                        | +03c
        cmpi.b  #0x2,0x58(a6)                   | +040
        bne.w   .L057e76                        | +046
        lea     Soldier_GrabStruggleNext_057e7a(pc),a1 | +04a
        move.l  a1,(a6)                         | +04e
.L057e76:
        jmp     Soldier_HitCheckTail_057ae4(pc)                | +050

| ----------------------------------------------------------------------------
|  Soldier_GrabStruggleNext_057e7a  @ $057E7A  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_GrabStruggleNext_057e7a, "ax", @progbits
        .global Soldier_GrabStruggleNext_057e7a
Soldier_GrabStruggleNext_057e7a:
        move.b  0x30(a6),d0                     | +000
        addi.b  #0x1,d0                         | +004
        move.b  d0,0x30(a6)                     | +008
        cmpi.b  #0x3,d0                         | +00c
        bcs.w   .L057e98                        | +010
        bset    #0x3,0x13(a6)                   | +014
        bra.w   Soldier_GrabThrownA_057f4e      | +01a
.L057e98:
        bclr    #0x3,0x13(a6)                   | +01e
        bclr    #0x0,0x13(a6)                   | +024
        lea     0x2b6892.l,a0                   | +02a
        jsr     0x28cd4.l                       | +030
        lea     .L057eb6(pc),a1                 | +036
        move.l  a1,(a6)                         | +03a
.L057eb6:
        bsr.w   Soldier_GrabHoldFlag_057ca8                    | +03c
        jsr     0x28d70.l                       | +040
        bcc.w   .L057eca                        | +046
        lea     Soldier_GrabStruggle_057e26(pc),a1 | +04a
        move.l  a1,(a6)                         | +04e
.L057eca:
        jmp     Soldier_HitCheckTail_057ae4(pc)                | +050

| ----------------------------------------------------------------------------
|  Soldier_GrabBreakA_057ece  @ $057ECE  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_GrabBreakA_057ece, "ax", @progbits
        .global Soldier_GrabBreakA_057ece
Soldier_GrabBreakA_057ece:
        move.l  #0x57d1c,0x5c(a6)               | +000
        move.l  #0x2b6c32,0x92(a6)              | +008
        bra.w   Soldier_GrabBreak_057f06        | +010

| ----------------------------------------------------------------------------
|  Soldier_GrabBreakB_057ee2  @ $057EE2  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_GrabBreakB_057ee2, "ax", @progbits
        .global Soldier_GrabBreakB_057ee2
Soldier_GrabBreakB_057ee2:
        move.l  #0x57d86,0x5c(a6)               | +000
        move.l  #0x2b6c32,0x92(a6)              | +008
        bra.w   Soldier_GrabBreak_057f06        | +010

| ----------------------------------------------------------------------------
|  Soldier_GrabBreakC_057ef6  @ $057EF6  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_GrabBreakC_057ef6, "ax", @progbits
        .global Soldier_GrabBreakC_057ef6
Soldier_GrabBreakC_057ef6:
        move.l  #0x57e26,0x5c(a6)               | +000
        move.l  #0x2b6bc0,0x92(a6)              | +008

| ----------------------------------------------------------------------------
|  Soldier_GrabBreak_057f06  @ $057F06  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_GrabBreak_057f06, "ax", @progbits
        .global Soldier_GrabBreak_057f06
Soldier_GrabBreak_057f06:
        movea.l 0x92(a6),a0                     | +000
        jsr     0x28cd4.l                       | +004
        lea     .L057f16(pc),a1                 | +00a
        move.l  a1,(a6)                         | +00e
.L057f16:
        bsr.w   Soldier_GrabHoldFlag_057ca8                    | +010
        jsr     0x28d70.l                       | +014
        bcc.w   .L057f3a                        | +01a
        bsr.w   Soldier_TestGrabBreak_056f64                    | +01e
        bcc.w   .L057f36                        | +022
        lea     Soldier_GrabBreak_057f06(pc),a1 | +026
        move.l  a1,(a6)                         | +02a
        bra.w   .L057f3a                        | +02c
.L057f36:
        move.l  0x5c(a6),(a6)                   | +030
.L057f3a:
        jsr     0x2870a.l                       | +034
        bcc.w   .L057f4a                        | +03a
        lea     Soldier_GrabStruggleNext_057e7a(pc),a1 | +03e
        move.l  a1,(a6)                         | +042
.L057f4a:
        jmp     Soldier_HitCheckTail_057ae4(pc)                | +044

| ----------------------------------------------------------------------------
|  Soldier_GrabThrownA_057f4e  @ $057F4E  (120 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_GrabThrownA_057f4e, "ax", @progbits
        .global Soldier_GrabThrownA_057f4e
Soldier_GrabThrownA_057f4e:
        lea     0x2b6ca4.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        movea.l 0x7a(a6),a0                     | +00c
        move.w  0x22(a0),d0                     | +010
        sub.w   0x22(a6),d0                     | +014
        bpl.w   .L057f74                        | +018
        bclr    #0x0,0x3a(a6)                   | +01c
        bra.w   .L057f7a                        | +022
.L057f74:
        bset    #0x0,0x3a(a6)                   | +026
.L057f7a:
        move.w  #0x333,d0                       | +02c
        jsr     0x5dca4.l                       | +030
        move.w  d0,0x28(a6)                     | +036
        move.w  #0x32a,0x2a(a6)                 | +03a
        move.w  #0xffaf,0x2e(a6)                | +040
        move.w  #0x0,0x2c(a6)                   | +046
        lea     .L057fa0(pc),a1                 | +04c
        move.l  a1,(a6)                         | +050
.L057fa0:
        jsr     0x2831e.l                       | +052
        scs.b   0x78(a6)                        | +058
        bcc.w   .L057fb4                        | +05c
        lea     Soldier_Stand_0588ae(pc),a1     | +060
        move.l  a1,(a6)                         | +064
.L057fb4:
        jsr     0x28d70.l                       | +066
        jsr     0x49fd0.l                       | +06c
        bsr.w   Soldier_DespawnIfOffscreen_056e1e            | +072
        rts                                     | +076

| ----------------------------------------------------------------------------
|  Soldier_GrabThrownB_057fc6  @ $057FC6  (134 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_GrabThrownB_057fc6, "ax", @progbits
        .global Soldier_GrabThrownB_057fc6
Soldier_GrabThrownB_057fc6:
        lea     0xffff.w,a0                     | +000
        move.l  a0,0x4c(a6)                     | +004
        jsr     0x283ca.l                       | +008
        movea.l 0x7a(a6),a0                     | +00e
        move.w  0x22(a0),d0                     | +012
        sub.w   0x22(a6),d0                     | +016
        bpl.w   .L057fee                        | +01a
        bclr    #0x0,0x3a(a6)                   | +01e
        bra.w   .L057ff4                        | +024
.L057fee:
        bset    #0x0,0x3a(a6)                   | +028
.L057ff4:
        lea     0x2b6d00.l,a0                   | +02e
        jsr     0x28cd4.l                       | +034
        move.w  #0x266,d0                       | +03a
        jsr     0x5dca4.l                       | +03e
        move.w  d0,0x28(a6)                     | +044
        move.w  #0x65e,0x2a(a6)                 | +048
        move.w  #0xff5d,0x2e(a6)                | +04e
        move.w  #0x0,0x2c(a6)                   | +054
        lea     .L058026(pc),a1                 | +05a
        move.l  a1,(a6)                         | +05e
.L058026:
        jsr     0x2831e.l                       | +060
        scs.b   0x78(a6)                        | +066
        bcc.w   .L05803a                        | +06a
        lea     Soldier_Land_058464(pc),a1      | +06e
        move.l  a1,(a6)                         | +072
.L05803a:
        jsr     0x28d70.l                       | +074
        jsr     0x49fd0.l                       | +07a
        bsr.w   Soldier_DespawnIfOffscreen_056e1e            | +080
        rts                                     | +084

| ----------------------------------------------------------------------------
|  Soldier_RunToward_05804c  @ $05804C  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_RunToward_05804c, "ax", @progbits
        .global Soldier_RunToward_05804c
Soldier_RunToward_05804c:
        btst    #0x4,0x74(a6)                   | +000
        bne.w   Soldier_SurrenderFlee_0589f8__L058aa8 | +006
        lea     0x29b7c8.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        move.w  #0x100,0x36(a6)                 | +016
        bsr.w   Soldier_SetVelXByFacing_056f8a                    | +01c
        lea     Soldier_Run_Loop_05809e(pc),a1  | +020
        move.l  a1,(a6)                         | +024
        bra.w   Soldier_Run_Loop_05809e         | +026

| ----------------------------------------------------------------------------
|  Soldier_RunByTable_058076  @ $058076  (40 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_RunByTable_058076, "ax", @progbits
        .global Soldier_RunByTable_058076
Soldier_RunByTable_058076:
        lea     0x2b5d14.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2b756e.l,a0                   | +00c
        jsr     0x799de.l                       | +012
        neg.w   d0                              | +018
        move.w  d0,0x36(a6)                     | +01a
        bsr.w   Soldier_SetVelXByFacing_056f8a                    | +01e
        lea     Soldier_Run_Loop_05809e(pc),a1  | +022
        move.l  a1,(a6)                         | +026

| ----------------------------------------------------------------------------
|  Soldier_Run_Loop_05809e  @ $05809E  (166 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_Run_Loop_05809e, "ax", @progbits
        .global Soldier_Run_Loop_05809e
Soldier_Run_Loop_05809e:
        bsr.w   Soldier_PhysicsStep_056acc                    | +000
        bsr.w   Soldier_Think_056b92                    | +004
        move.w  0x28(a6),d0                     | +008
        asr.w   #0x4,d0                         | +00c
        sub.w   d0,0x28(a6)                     | +00e
        jsr     0x28d70.l                       | +012
        bcc.w   .L0580c0                        | +018
        lea     Soldier_Idle_058144(pc),a1      | +01c
        move.l  a1,(a6)                         | +020
.L0580c0:
        bsr.w   Soldier_ProbeWalkEdge_056fec                    | +022
        jsr     0x5e9b6.l                       | +026
        move.w  d0,-(a7)                        | +02c
        andi.w  #0xff,d0                        | +02e
        cmp.w   0x8a(a6),d0                     | +032
        bcc.w   .L0580de                        | +036
        move.b  (a7),d0                         | +03a
        cmp.w   0x8a(a6),d0                     | +03c
.L0580de:
        addq.w  #0x2,a7                         | +040
        bcc.w   .L0580ea                        | +042
        lea     Soldier_Flee_058658(pc),a1      | +046
        move.l  a1,(a6)                         | +04a
.L0580ea:
        jsr     Soldier_LeaveTimerExpired_056e36(pc)                | +04c
        bcc.w   .L0580f8                        | +050
        lea     Soldier_SpawnFaceTarget_059062(pc),a1 | +054
        move.l  a1,(a6)                         | +058
.L0580f8:
        tst.b   0x78(a6)                        | +05a
        bne.w   .L058106                        | +05e
        lea     Soldier_Hurt_058412(pc),a1      | +062
        move.l  a1,(a6)                         | +066
.L058106:
        jsr     0x5e9b6.l                       | +068
        move.w  d0,-(a7)                        | +06e
        andi.w  #0xff,d0                        | +070
        cmp.w   0x88(a6),d0                     | +074
        bcc.w   .L058120                        | +078
        move.b  (a7),d0                         | +07c
        cmp.w   0x88(a6),d0                     | +07e
.L058120:
        addq.w  #0x2,a7                         | +082
        bcc.w   .L05812c                        | +084
        lea     Soldier_Jump_0588f6(pc),a1      | +088
        move.l  a1,(a6)                         | +08c
.L05812c:
        jsr     Soldier_TestMeleeRange_0574e8(pc)                | +08e
        bcc.w   .L05813a                        | +092
        lea     Soldier_MeleeAttack_0585ae(pc),a1 | +096
        move.l  a1,(a6)                         | +09a
.L05813a:
        jsr     0x49fd0.l                       | +09c
        bra.w   Soldier_DespawnIfOffscreen_056e1e            | +0a2

| ----------------------------------------------------------------------------
|  Soldier_Idle_058144  @ $058144  (648 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_Idle_058144, "ax", @progbits
        .global Soldier_Idle_058144
Soldier_Idle_058144:
        tst.w   0x34(a6)                        | +000
        bne.w   .L05815c                        | +004
        lea     0x29b816.l,a0                   | +008
        jsr     0x28cd4.l                       | +00e
        bra.w   .L058188                        | +014
.L05815c:
        spl.b   d0                              | +018
        btst    #0x0,0x3a(a6)                   | +01a
        spl.b   d1                              | +020
        eor.w   d0,d1                           | +022
        bne.w   .L05817c                        | +024
        lea     0x2b5f28.l,a0                   | +028
        jsr     0x28cd4.l                       | +02e
        bra.w   .L058188                        | +034
.L05817c:
        lea     0x2b5f28.l,a0                   | +038
        jsr     0x28cd4.l                       | +03e
.L058188:
        clr.w   0x28(a6)                        | +044
        clr.w   0x2a(a6)                        | +048
        lea     .L058196(pc),a1                 | +04c
        move.l  a1,(a6)                         | +050
.L058196:
        bsr.w   Soldier_PhysicsStep_056acc                    | +052
        bsr.w   Soldier_Think_056b92                    | +056
        jsr     0x28d70.l                       | +05a
        bsr.w   Soldier_ProbeWalkEdge_056fec                    | +060
        btst    #0x0,0x72(a6)                   | +064
        beq.w   .L05828c                        | +06a
        move.w  0x80(a6),d1                     | +06e
        cmp.w   0x86(a6),d1                     | +072
        bcc.w   .L0581c2                        | +076
        move.w  0x86(a6),d1                     | +07a
.L0581c2:
        jsr     0x5e9b6.l                       | +07e
        move.w  d0,-(a7)                        | +084
        andi.w  #0xff,d0                        | +086
        cmp.w   d1,d0                           | +08a
        bcc.w   .L0581d8                        | +08c
        move.b  (a7),d0                         | +090
        cmp.w   d1,d0                           | +092
.L0581d8:
        addq.w  #0x2,a7                         | +094
        bcc.w   .L0581e4                        | +096
        lea     Soldier_StepLeft_058500(pc),a1  | +09a
        move.l  a1,(a6)                         | +09e
.L0581e4:
        mulu.w  d1,d1                           | +0a0
        lsr.l   #0x8,d1                         | +0a2
        jsr     0x5e9b6.l                       | +0a4
        move.w  d0,-(a7)                        | +0aa
        andi.w  #0xff,d0                        | +0ac
        cmp.w   d1,d0                           | +0b0
        bcc.w   .L0581fe                        | +0b2
        move.b  (a7),d0                         | +0b6
        cmp.w   d1,d0                           | +0b8
.L0581fe:
        addq.w  #0x2,a7                         | +0ba
        bcc.w   .L05820a                        | +0bc
        lea     Soldier_RunByTable_058076(pc),a1 | +0c0
        move.l  a1,(a6)                         | +0c4
.L05820a:
        mulu.w  d1,d1                           | +0c6
        lsr.l   #0x8,d1                         | +0c8
        jsr     0x5e9b6.l                       | +0ca
        move.w  d0,-(a7)                        | +0d0
        andi.w  #0xff,d0                        | +0d2
        cmp.w   d1,d0                           | +0d6
        bcc.w   .L058224                        | +0d8
        move.b  (a7),d0                         | +0dc
        cmp.w   d1,d0                           | +0de
.L058224:
        addq.w  #0x2,a7                         | +0e0
        bcc.w   .L058230                        | +0e2
        lea     Soldier_Brake_0585f6(pc),a1     | +0e6
        move.l  a1,(a6)                         | +0ea
.L058230:
        move.w  0x82(a6),d1                     | +0ec
        cmp.w   0x84(a6),d1                     | +0f0
        bcc.w   .L058240                        | +0f4
        move.w  0x84(a6),d1                     | +0f8
.L058240:
        jsr     0x5e9b6.l                       | +0fc
        move.w  d0,-(a7)                        | +102
        andi.w  #0xff,d0                        | +104
        cmp.w   d1,d0                           | +108
        bcc.w   .L058256                        | +10a
        move.b  (a7),d0                         | +10e
        cmp.w   d1,d0                           | +110
.L058256:
        addq.w  #0x2,a7                         | +112
        bcc.w   .L058262                        | +114
        lea     Soldier_StepRight_0584c4(pc),a1 | +118
        move.l  a1,(a6)                         | +11c
.L058262:
        mulu.w  d1,d1                           | +11e
        lsr.l   #0x8,d1                         | +120
        jsr     0x5e9b6.l                       | +122
        move.w  d0,-(a7)                        | +128
        andi.w  #0xff,d0                        | +12a
        cmp.w   d1,d0                           | +12e
        bcc.w   .L05827c                        | +130
        move.b  (a7),d0                         | +134
        cmp.w   d1,d0                           | +136
.L05827c:
        addq.w  #0x2,a7                         | +138
        bcc.w   .L058288                        | +13a
        lea     Soldier_WalkStart_057558(pc),a1             | +13e
        move.l  a1,(a6)                         | +142
.L058288:
        bra.w   .L0582d8                        | +144
.L05828c:
        jsr     0x5e9b6.l                       | +148
        move.w  d0,-(a7)                        | +14e
        andi.w  #0xff,d0                        | +150
        cmp.w   0x80(a6),d0                     | +154
        bcc.w   .L0582a6                        | +158
        move.b  (a7),d0                         | +15c
        cmp.w   0x80(a6),d0                     | +15e
.L0582a6:
        addq.w  #0x2,a7                         | +162
        bcc.w   .L0582b2                        | +164
        lea     Soldier_WalkStart_057558(pc),a1             | +168
        move.l  a1,(a6)                         | +16c
.L0582b2:
        jsr     0x5e9b6.l                       | +16e
        move.w  d0,-(a7)                        | +174
        andi.w  #0xff,d0                        | +176
        cmp.w   0x82(a6),d0                     | +17a
        bcc.w   .L0582cc                        | +17e
        move.b  (a7),d0                         | +182
        cmp.w   0x82(a6),d0                     | +184
.L0582cc:
        addq.w  #0x2,a7                         | +188
        bcc.w   .L0582d8                        | +18a
        lea     Soldier_IdleFidget_0583cc(pc),a1 | +18e
        move.l  a1,(a6)                         | +192
.L0582d8:
        jsr     0x5e9b6.l                       | +194
        move.w  d0,-(a7)                        | +19a
        andi.w  #0xff,d0                        | +19c
        cmp.w   0x86(a6),d0                     | +1a0
        bcc.w   .L0582f2                        | +1a4
        move.b  (a7),d0                         | +1a8
        cmp.w   0x86(a6),d0                     | +1aa
.L0582f2:
        addq.w  #0x2,a7                         | +1ae
        bcc.w   .L0582fe                        | +1b0
        lea     Soldier_Brake_0585f6(pc),a1     | +1b4
        move.l  a1,(a6)                         | +1b8
.L0582fe:
        jsr     0x5e9b6.l                       | +1ba
        move.w  d0,-(a7)                        | +1c0
        andi.w  #0xff,d0                        | +1c2
        cmp.w   0x84(a6),d0                     | +1c6
        bcc.w   .L058318                        | +1ca
        move.b  (a7),d0                         | +1ce
        cmp.w   0x84(a6),d0                     | +1d0
.L058318:
        addq.w  #0x2,a7                         | +1d4
        bcc.w   .L058324                        | +1d6
        lea     Soldier_WalkStart_057558(pc),a1             | +1da
        move.l  a1,(a6)                         | +1de
.L058324:
        move.w  0x34(a6),d0                     | +1e0
        bne.w   .L058352                        | +1e4
        jsr     0x5e9b6.l                       | +1e8
        move.w  d0,-(a7)                        | +1ee
        andi.w  #0xff,d0                        | +1f0
        cmp.w   0x8a(a6),d0                     | +1f4
        bcc.w   .L058346                        | +1f8
        move.b  (a7),d0                         | +1fc
        cmp.w   0x8a(a6),d0                     | +1fe
.L058346:
        addq.w  #0x2,a7                         | +202
        bcc.w   .L058352                        | +204
        lea     Soldier_Flee_058658(pc),a1      | +208
        move.l  a1,(a6)                         | +20c
.L058352:
        jsr     Soldier_LeaveTimerExpired_056e36(pc)                | +20e
        bcc.w   .L058360                        | +212
        lea     Soldier_SpawnFaceTarget_059062(pc),a1 | +216
        move.l  a1,(a6)                         | +21a
.L058360:
        jsr     Soldier_TestSurrender_056fa0(pc)                | +21c
        bcc.w   .L05836e                        | +220
        lea     Soldier_Surrender_058968(pc),a1 | +224
        move.l  a1,(a6)                         | +228
.L05836e:
        btst    #0x0,0x73(a6)                   | +22a
        beq.w   .L05837e                        | +230
        lea     Soldier_TauntInit_058e08(pc),a1 | +234
        move.l  a1,(a6)                         | +238
.L05837e:
        jsr     0x5e9b6.l                       | +23a
        move.w  d0,-(a7)                        | +240
        andi.w  #0xff,d0                        | +242
        cmp.w   0x88(a6),d0                     | +246
        bcc.w   .L058398                        | +24a
        move.b  (a7),d0                         | +24e
        cmp.w   0x88(a6),d0                     | +250
.L058398:
        addq.w  #0x2,a7                         | +254
        bcc.w   .L0583a4                        | +256
        lea     Soldier_Jump_0588f6(pc),a1      | +25a
        move.l  a1,(a6)                         | +25e
.L0583a4:
        jsr     Soldier_TestMeleeRange_0574e8(pc)                | +260
        bcc.w   .L0583b2                        | +264
        lea     Soldier_MeleeAttack_0585ae(pc),a1 | +268
        move.l  a1,(a6)                         | +26c
.L0583b2:
        tst.b   0x78(a6)                        | +26e
        bne.w   .L0583c0                        | +272
        lea     Soldier_Hurt_058412(pc),a1      | +276
        move.l  a1,(a6)                         | +27a
.L0583c0:
        jsr     0x49fd0.l                       | +27c
        bsr.w   Soldier_DespawnIfOffscreen_056e1e            | +282
        rts                                     | +286

| ----------------------------------------------------------------------------
|  Soldier_IdleFidget_0583cc  @ $0583CC  (70 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_IdleFidget_0583cc, "ax", @progbits
        .global Soldier_IdleFidget_0583cc
Soldier_IdleFidget_0583cc:
        jsr     0x5e9b6.l                       | +000
        andi.b  #0xf,d0                         | +006
        bne.w   Soldier_Brake_0585f6            | +00a
        lea     0x29bf34.l,a0                   | +00e
        jsr     0x28cd4.l                       | +014
        lea     .L0583ec(pc),a1                 | +01a
        move.l  a1,(a6)                         | +01e
.L0583ec:
        jsr     0x2783a.l                       | +020
        bsr.w   Soldier_Think_056b92                    | +026
        jsr     0x28d70.l                       | +02a
        bcc.w   .L058406                        | +030
        lea     Soldier_Idle_058144(pc),a1      | +034
        move.l  a1,(a6)                         | +038
.L058406:
        jsr     0x49fd0.l                       | +03a
        bsr.w   Soldier_DespawnIfOffscreen_056e1e            | +040
        rts                                     | +044

| ----------------------------------------------------------------------------
|  Soldier_Hurt_058412  @ $058412  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_Hurt_058412, "ax", @progbits
        .global Soldier_Hurt_058412
Soldier_Hurt_058412:
        lea     0x2b5b92.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     Soldier_Hurt_Loop_058424(pc),a1 | +00c
        move.l  a1,(a6)                         | +010

| ----------------------------------------------------------------------------
|  Soldier_Hurt_Loop_058424  @ $058424  (64 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_Hurt_Loop_058424, "ax", @progbits
        .global Soldier_Hurt_Loop_058424
Soldier_Hurt_Loop_058424:
        move.w  0x28(a6),d0                     | +000
        asr.w   #0x6,d0                         | +004
        sub.w   d0,0x28(a6)                     | +006
        jsr     0x28364.l                       | +00a
        scs.b   0x78(a6)                        | +010
        bcc.w   .L058442                        | +014
        lea     Soldier_Land_058464(pc),a1      | +018
        move.l  a1,(a6)                         | +01c
.L058442:
        jsr     0x28d70.l                       | +01e
        jsr     0x49fd0.l                       | +024
        bsr.w   Soldier_DespawnIfOffscreen_056e1e            | +02a
        rts                                     | +02e
        .global Soldier_Hurt_Loop_058424__L058454
Soldier_Hurt_Loop_058424__L058454:
        lea     0x2b7030.l,a0                   | +030
        jsr     0x28cd4.l                       | +036
        bra.w   Soldier_Land_058464__L05848a    | +03c

| ----------------------------------------------------------------------------
|  Soldier_Land_058464  @ $058464  (96 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_Land_058464, "ax", @progbits
        .global Soldier_Land_058464
Soldier_Land_058464:
        move.w  0x28(a6),d0                     | +000
        bpl.w   .L05846e                        | +004
        neg.w   d0                              | +008
.L05846e:
        move.w  0x2a(a6),d1                     | +00a
        bpl.w   .L058478                        | +00e
        neg.w   d1                              | +012
.L058478:
        cmp.w   d0,d1                           | +014
        bcs.w   Soldier_RunToward_05804c        | +016
        lea     0x2b5c22.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        .global Soldier_Land_058464__L05848a
Soldier_Land_058464__L05848a:
        clr.w   0x28(a6)                        | +026
        lea     .L058494(pc),a1                 | +02a
        move.l  a1,(a6)                         | +02e
.L058494:
        bsr.w   Soldier_PhysicsStep_056acc                    | +030
        jsr     0x28d70.l                       | +034
        bcc.w   .L0584ba                        | +03a
        tst.b   0x78(a6)                        | +03e
        beq.w   .L0584b4                        | +042
        lea     Soldier_WalkStart_057558(pc),a1             | +046
        move.l  a1,(a6)                         | +04a
        bra.w   .L0584ba                        | +04c
.L0584b4:
        lea     Soldier_Hurt_058412(pc),a1      | +050
        move.l  a1,(a6)                         | +054
.L0584ba:
        jsr     0x49fd0.l                       | +056
        bra.w   Soldier_DespawnIfOffscreen_056e1e            | +05c

| ----------------------------------------------------------------------------
|  Soldier_StepRight_0584c4  @ $0584C4  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_StepRight_0584c4, "ax", @progbits
        .global Soldier_StepRight_0584c4
Soldier_StepRight_0584c4:
        tst.w   0x34(a6)                        | +000
        bne.w   .L0584dc                        | +004
        lea     0x2b5c56.l,a0                   | +008
        jsr     0x28cd4.l                       | +00e
        bra.w   .L0584e8                        | +014
.L0584dc:
        lea     0x2b5f56.l,a0                   | +018
        jsr     0x28cd4.l                       | +01e
.L0584e8:
        move.w  #0x80,0x36(a6)                  | +024
        bsr.w   Soldier_SetVelXByFacing_056f8a                    | +02a
        clr.w   0x2a(a6)                        | +02e
        lea     Soldier_Step_Loop_058538(pc),a1 | +032
        move.l  a1,(a6)                         | +036
        bra.w   Soldier_Step_Loop_058538        | +038

| ----------------------------------------------------------------------------
|  Soldier_StepLeft_058500  @ $058500  (56 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_StepLeft_058500, "ax", @progbits
        .global Soldier_StepLeft_058500
Soldier_StepLeft_058500:
        tst.w   0x34(a6)                        | +000
        bne.w   .L058518                        | +004
        lea     0x2b5ca8.l,a0                   | +008
        jsr     0x28cd4.l                       | +00e
        bra.w   .L058524                        | +014
.L058518:
        lea     0x2b5fd0.l,a0                   | +018
        jsr     0x28cd4.l                       | +01e
.L058524:
        move.w  #0xff80,0x36(a6)                | +024
        bsr.w   Soldier_SetVelXByFacing_056f8a                    | +02a
        clr.w   0x2a(a6)                        | +02e
        lea     Soldier_Step_Loop_058538(pc),a1 | +032
        move.l  a1,(a6)                         | +036

| ----------------------------------------------------------------------------
|  Soldier_Step_Loop_058538  @ $058538  (118 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_Step_Loop_058538, "ax", @progbits
        .global Soldier_Step_Loop_058538
Soldier_Step_Loop_058538:
        bsr.w   Soldier_PhysicsStep_056acc                    | +000
        bsr.w   Soldier_Think_056b92                    | +004
        jsr     0x28d70.l                       | +008
        bcc.w   .L058550                        | +00e
        lea     Soldier_Idle_058144(pc),a1      | +012
        move.l  a1,(a6)                         | +016
.L058550:
        bsr.w   Soldier_ProbeWalkEdge_056fec                    | +018
        jsr     0x5e9b6.l                       | +01c
        move.w  d0,-(a7)                        | +022
        andi.w  #0xff,d0                        | +024
        cmp.w   0x88(a6),d0                     | +028
        bcc.w   .L05856e                        | +02c
        move.b  (a7),d0                         | +030
        cmp.w   0x88(a6),d0                     | +032
.L05856e:
        addq.w  #0x2,a7                         | +036
        bcc.w   .L05857a                        | +038
        lea     Soldier_Jump_0588f6(pc),a1      | +03c
        move.l  a1,(a6)                         | +040
.L05857a:
        tst.b   0x78(a6)                        | +042
        bne.w   .L058588                        | +046
        lea     Soldier_Hurt_058412(pc),a1      | +04a
        move.l  a1,(a6)                         | +04e
.L058588:
        jsr     Soldier_TestSurrender_056fa0(pc)                | +050
        bcc.w   .L058596                        | +054
        lea     Soldier_Surrender_058968(pc),a1 | +058
        move.l  a1,(a6)                         | +05c
.L058596:
        jsr     Soldier_TestMeleeRange_0574e8(pc)                | +05e
        bcc.w   .L0585a4                        | +062
        lea     Soldier_MeleeAttack_0585ae(pc),a1 | +066
        move.l  a1,(a6)                         | +06a
.L0585a4:
        jsr     0x49fd0.l                       | +06c
        bra.w   Soldier_DespawnIfOffscreen_056e1e            | +072

| ----------------------------------------------------------------------------
|  Soldier_MeleeAttack_0585ae  @ $0585AE  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_MeleeAttack_0585ae, "ax", @progbits
        .global Soldier_MeleeAttack_0585ae
Soldier_MeleeAttack_0585ae:
        clr.w   0x28(a6)                        | +000
        lea     0x2b70d2.l,a0                   | +004
        jsr     0x28cd4.l                       | +00a
        lea     Soldier_AttackTblMelee_057494(pc),a0             | +010
        move.l  a0,0x4c(a6)                     | +014
        jsr     0x283ca.l                       | +018
        jsr     0x283ca.l                       | +01e
        lea     .L0585d8(pc),a1                 | +024
        move.l  a1,(a6)                         | +028
.L0585d8:
        bsr.w   Soldier_PhysicsStep_056acc                    | +02a
        jsr     0x28d70.l                       | +02e
        bcc.w   .L0585ec                        | +034
        lea     Soldier_Idle_058144(pc),a1      | +038
        move.l  a1,(a6)                         | +03c
.L0585ec:
        jsr     0x49fd0.l                       | +03e
        bra.w   Soldier_DespawnIfOffscreen_056e1e            | +044

| ----------------------------------------------------------------------------
|  Soldier_Brake_0585f6  @ $0585F6  (98 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_Brake_0585f6, "ax", @progbits
        .global Soldier_Brake_0585f6
Soldier_Brake_0585f6:
        lea     0x2b5cfa.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L058608(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L058608:
        bsr.w   Soldier_Think_056b92                    | +012
        move.w  0x28(a6),d0                     | +016
        asr.w   #0x4,d0                         | +01a
        sub.w   d0,0x28(a6)                     | +01c
        bsr.w   Soldier_PhysicsStep_056acc                    | +020
        jsr     0x28d70.l                       | +024
        bcc.w   .L05863e                        | +02a
        btst    #0x0,0x72(a6)                   | +02e
        beq.w   .L058638                        | +034
        lea     Soldier_RunToward_05804c(pc),a1 | +038
        move.l  a1,(a6)                         | +03c
        bra.w   .L05863e                        | +03e
.L058638:
        lea     Soldier_WalkStart_057558(pc),a1             | +042
        move.l  a1,(a6)                         | +046
.L05863e:
        tst.b   0x78(a6)                        | +048
        bne.w   .L05864c                        | +04c
        lea     Soldier_Hurt_058412(pc),a1      | +050
        move.l  a1,(a6)                         | +054
.L05864c:
        jsr     0x49fd0.l                       | +056
        bsr.w   Soldier_DespawnIfOffscreen_056e1e            | +05c
        rts                                     | +060

| ----------------------------------------------------------------------------
|  Soldier_Flee_058658  @ $058658  (96 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_Flee_058658, "ax", @progbits
        .global Soldier_Flee_058658
Soldier_Flee_058658:
        btst    #0x4,0x74(a6)                   | +000
        bne.w   Soldier_SurrenderFlee_0589f8__L058aa8 | +006
        lea     0x29b956.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        move.w  0x28(a6),d0                     | +016
        add.w   d0,d0                           | +01a
        move.w  d0,0x28(a6)                     | +01c
        clr.w   0x2a(a6)                        | +020
        lea     .L058682(pc),a1                 | +024
        move.l  a1,(a6)                         | +028
.L058682:
        move.w  0x28(a6),d0                     | +02a
        asr.w   #0x5,d0                         | +02e
        sub.w   d0,0x28(a6)                     | +030
        bsr.w   Soldier_PhysicsStep_056acc                    | +034
        bsr.w   Soldier_Think_056b92                    | +038
        jsr     0x28d70.l                       | +03c
        bcc.w   .L0586ac                        | +042
        tst.b   0x78(a6)                        | +046
        beq.w   .L0586ac                        | +04a
        lea     Soldier_FleeStop_0586b8(pc),a1  | +04e
        move.l  a1,(a6)                         | +052
.L0586ac:
        jsr     0x49fd0.l                       | +054
        bsr.w   Soldier_DespawnIfOffscreen_056e1e            | +05a
        rts                                     | +05e

| ----------------------------------------------------------------------------
|  Soldier_FleeStop_0586b8  @ $0586B8  (230 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_FleeStop_0586b8, "ax", @progbits
        .global Soldier_FleeStop_0586b8
Soldier_FleeStop_0586b8:
        lea     0x2b5c10.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        clr.w   0x28(a6)                        | +00c
        lea     .L0586ce(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L0586ce:
        bsr.w   Soldier_PhysicsStep_056acc                    | +016
        bsr.w   Soldier_Think_056b92                    | +01a
        jsr     0x28d70.l                       | +01e
        move.w  0x8a(a6),d1                     | +024
        neg.w   d1                              | +028
        addi.w  #0x100,d1                       | +02a
        jsr     0x5e9b6.l                       | +02e
        move.w  d0,-(a7)                        | +034
        andi.w  #0xff,d0                        | +036
        cmp.w   d1,d0                           | +03a
        bcc.w   .L0586fc                        | +03c
        move.b  (a7),d0                         | +040
        cmp.w   d1,d0                           | +042
.L0586fc:
        addq.w  #0x2,a7                         | +044
        bcc.w   .L058708                        | +046
        lea     Soldier_Stand_0588ae(pc),a1     | +04a
        move.l  a1,(a6)                         | +04e
.L058708:
        btst    #0x0,0x72(a6)                   | +050
        beq.w   .L058762                        | +056
        jsr     0x5e9b6.l                       | +05a
        move.w  d0,-(a7)                        | +060
        andi.w  #0xff,d0                        | +062
        cmp.w   0x82(a6),d0                     | +066
        bcc.w   .L05872c                        | +06a
        move.b  (a7),d0                         | +06e
        cmp.w   0x82(a6),d0                     | +070
.L05872c:
        addq.w  #0x2,a7                         | +074
        bcc.w   .L058738                        | +076
        lea     Soldier_RetreatJmp_05879e(pc),a1 | +07a
        move.l  a1,(a6)                         | +07e
.L058738:
        move.w  0x80(a6),d1                     | +080
        jsr     0x5e9b6.l                       | +084
        move.w  d0,-(a7)                        | +08a
        andi.w  #0xff,d0                        | +08c
        cmp.w   d1,d0                           | +090
        bcc.w   .L058752                        | +092
        move.b  (a7),d0                         | +096
        cmp.w   d1,d0                           | +098
.L058752:
        addq.w  #0x2,a7                         | +09a
        bcc.w   .L05875e                        | +09c
        lea     Soldier_Retreat_0587ac(pc),a1   | +0a0
        move.l  a1,(a6)                         | +0a4
.L05875e:
        bra.w   .L058768                        | +0a6
.L058762:
        lea     Soldier_Retreat_0587ac(pc),a1   | +0aa
        move.l  a1,(a6)                         | +0ae
.L058768:
        jsr     Soldier_LeaveTimerExpired_056e36(pc)                | +0b0
        bcc.w   .L058776                        | +0b4
        lea     Soldier_SpawnStand_0591c8(pc),a1 | +0b8
        move.l  a1,(a6)                         | +0bc
.L058776:
        jsr     Soldier_TestSurrender_056fa0(pc)                | +0be
        bcc.w   .L058784                        | +0c2
        lea     Soldier_SurrenderFlee_0589f8(pc),a1 | +0c6
        move.l  a1,(a6)                         | +0ca
.L058784:
        tst.b   0x78(a6)                        | +0cc
        bne.w   .L058792                        | +0d0
        lea     Soldier_Hurt_058412(pc),a1      | +0d4
        move.l  a1,(a6)                         | +0d8
.L058792:
        jsr     0x49ff2.l                       | +0da
        bsr.w   Soldier_DespawnIfOffscreen_056e1e            | +0e0
        rts                                     | +0e4

| ----------------------------------------------------------------------------
|  Soldier_RetreatJmp_05879e  @ $05879E  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_RetreatJmp_05879e, "ax", @progbits
        .global Soldier_RetreatJmp_05879e
Soldier_RetreatJmp_05879e:
        btst    #0x0,0x72(a6)                   | +000
        bne.w   Soldier_Retreat_0587ac__L0587b6 | +006
        bra.w   Soldier_Retreat_0587ac__L0587e4 | +00a

| ----------------------------------------------------------------------------
|  Soldier_Retreat_0587ac  @ $0587AC  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_Retreat_0587ac, "ax", @progbits
        .global Soldier_Retreat_0587ac
Soldier_Retreat_0587ac:
        btst    #0x0,0x72(a6)                   | +000
        bne.w   Soldier_Retreat_0587ac__L0587e4 | +006
        .global Soldier_Retreat_0587ac__L0587b6
Soldier_Retreat_0587ac__L0587b6:
        lea     0x2b5bcc.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        move.w  #0x100,d0                       | +016
        btst    #0x0,0x72(a6)                   | +01a
        bne.w   .L0587d2                        | +020
        add.w   d0,d0                           | +024
.L0587d2:
        move.w  d0,0x36(a6)                     | +026
        bsr.w   Soldier_SetVelXByFacing_056f8a                    | +02a
        lea     Soldier_Retreat_Loop_058800(pc),a1 | +02e
        move.l  a1,(a6)                         | +032
        bra.w   Soldier_Retreat_Loop_058800     | +034
        .global Soldier_Retreat_0587ac__L0587e4
Soldier_Retreat_0587ac__L0587e4:
        lea     0x2b5bcc.l,a0                   | +038
        jsr     0x28cd4.l                       | +03e
        move.w  #0xff00,0x36(a6)                | +044
        bsr.w   Soldier_SetVelXByFacing_056f8a                    | +04a
        lea     Soldier_Retreat_Loop_058800(pc),a1 | +04e
        move.l  a1,(a6)                         | +052

| ----------------------------------------------------------------------------
|  Soldier_Retreat_Loop_058800  @ $058800  (174 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_Retreat_Loop_058800, "ax", @progbits
        .global Soldier_Retreat_Loop_058800
Soldier_Retreat_Loop_058800:
        jsr     0x282d8.l                       | +000
        bcc.w   .L058810                        | +006
        lea     Soldier_Stand_0588ae(pc),a1     | +00a
        move.l  a1,(a6)                         | +00e
.L058810:
        jsr     0x27eba.l                       | +010
        scc.b   0x78(a6)                        | +016
        bsr.w   Soldier_Think_056b92                    | +01a
        move.w  0x34(a6),d0                     | +01e
        beq.w   .L05882c                        | +022
        lea     Soldier_Stand_0588ae(pc),a1     | +026
        move.l  a1,(a6)                         | +02a
.L05882c:
        jsr     0x28d70.l                       | +02c
        bcc.w   .L05883c                        | +032
        lea     Soldier_FleeStop_0586b8(pc),a1  | +036
        move.l  a1,(a6)                         | +03a
.L05883c:
        move.w  0x8a(a6),d1                     | +03c
        neg.w   d1                              | +040
        addi.w  #0x100,d1                       | +042
        jsr     0x5e9b6.l                       | +046
        move.w  d0,-(a7)                        | +04c
        andi.w  #0xff,d0                        | +04e
        cmp.w   d1,d0                           | +052
        bcc.w   .L05885c                        | +054
        move.b  (a7),d0                         | +058
        cmp.w   d1,d0                           | +05a
.L05885c:
        addq.w  #0x2,a7                         | +05c
        bcc.w   .L058868                        | +05e
        lea     Soldier_Stand_0588ae(pc),a1     | +062
        move.l  a1,(a6)                         | +066
.L058868:
        btst    #0x5,0x5a(a6)                   | +068
        beq.w   .L058878                        | +06e
        lea     Soldier_Stand_0588ae(pc),a1     | +072
        move.l  a1,(a6)                         | +076
.L058878:
        jsr     Soldier_LeaveTimerExpired_056e36(pc)                | +078
        bcc.w   .L058886                        | +07c
        lea     Soldier_SpawnStand_0591c8(pc),a1 | +080
        move.l  a1,(a6)                         | +084
.L058886:
        jsr     Soldier_TestSurrender_056fa0(pc)                | +086
        bcc.w   .L058894                        | +08a
        lea     Soldier_SurrenderFlee_0589f8(pc),a1 | +08e
        move.l  a1,(a6)                         | +092
.L058894:
        tst.b   0x78(a6)                        | +094
        bne.w   .L0588a2                        | +098
        lea     Soldier_Hurt_058412(pc),a1      | +09c
        move.l  a1,(a6)                         | +0a0
.L0588a2:
        jsr     0x49ff2.l                       | +0a2
        bsr.w   Soldier_DespawnIfOffscreen_056e1e            | +0a8
        rts                                     | +0ac

| ----------------------------------------------------------------------------
|  Soldier_Stand_0588ae  @ $0588AE  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_Stand_0588ae, "ax", @progbits
        .global Soldier_Stand_0588ae
Soldier_Stand_0588ae:
        lea     0x29bd84.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        clr.w   0x28(a6)                        | +00c
        lea     .L0588c4(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L0588c4:
        jsr     Soldier_PhysicsStep_056acc(pc)                | +016
        bsr.w   Soldier_Think_056b92                    | +01a
        jsr     0x28d70.l                       | +01e
        bcc.w   .L0588dc                        | +024
        lea     Soldier_WalkStart_057558(pc),a1             | +028
        move.l  a1,(a6)                         | +02c
.L0588dc:
        tst.b   0x78(a6)                        | +02e
        bne.w   .L0588ea                        | +032
        lea     Soldier_Hurt_058412(pc),a1      | +036
        move.l  a1,(a6)                         | +03a
.L0588ea:
        jsr     0x49fd0.l                       | +03c
        bsr.w   Soldier_DespawnIfOffscreen_056e1e            | +042
        rts                                     | +046

| ----------------------------------------------------------------------------
|  Soldier_Jump_0588f6  @ $0588F6  (114 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_Jump_0588f6, "ax", @progbits
        .global Soldier_Jump_0588f6
Soldier_Jump_0588f6:
        move.w  0x84(a6),d0                     | +000
        add.w   0x82(a6),d0                     | +004
        sub.w   0x86(a6),d0                     | +008
        sub.w   0x80(a6),d0                     | +00c
        asr.w   #0x1,d0                         | +010
        btst    #0x0,0x3a(a6)                   | +012
        seq.b   d1                              | +018
        ext.w   d1                              | +01a
        eor.w   d0,d1                           | +01c
        move.w  d1,0x28(a6)                     | +01e
        tst.w   d0                              | +022
        bmi.w   .L05893e                        | +024
        lea     0x2b5d58.l,a0                   | +028
        jsr     0x28cd4.l                       | +02e
        move.w  0x88(a6),d0                     | +034
        lsl.w   #0x3,d0                         | +038
        move.w  d0,0x2a(a6)                     | +03a
        lea     Soldier_Hurt_Loop_058424(pc),a1 | +03e
        move.l  a1,(a6)                         | +042
        bra.w   Soldier_Hurt_Loop_058424        | +044
.L05893e:
        lea     0x2b5e68.l,a0                   | +048
        jsr     0x28cd4.l                       | +04e
        move.w  0x88(a6),d0                     | +054
        lsl.w   #0x3,d0                         | +058
        move.w  d0,0x2a(a6)                     | +05a
        move.w  #0xff00,0x36(a6)                | +05e
        bsr.w   Soldier_SetVelXByFacing_056f8a                    | +064
        lea     Soldier_Hurt_Loop_058424(pc),a1 | +068
        move.l  a1,(a6)                         | +06c
        bra.w   Soldier_Hurt_Loop_058424        | +06e

| ----------------------------------------------------------------------------
|  Soldier_Surrender_058968  @ $058968  (144 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_Surrender_058968, "ax", @progbits
        .global Soldier_Surrender_058968
Soldier_Surrender_058968:
        move.b  0x75(a6),d0                     | +000
        lea     0x29ba70.l,a0                   | +004
        cmpi.b  #0x3,d0                         | +00a
        bne.w   .L058994                        | +00e
        lea     0x29bbec.l,a0                   | +012
        jsr     0x5e9b6.l                       | +018
        bmi.w   .L058990                        | +01e
        lea     0x29bc64.l,a0                   | +022
.L058990:
        bra.w   .L0589b0                        | +028
.L058994:
        cmpi.b  #0x1,d0                         | +02c
        bne.w   .L0589a2                        | +030
        lea     0x29bbec.l,a0                   | +034
.L0589a2:
        cmpi.b  #0x2,d0                         | +03a
        bne.w   .L0589b0                        | +03e
        lea     0x29bc64.l,a0                   | +042
.L0589b0:
        jsr     0x28cd4.l                       | +048
        move.l  0x7a(a6),d0                     | +04e
        bsr.w   EntityFrame_FrameSelectByGate3A_057044 | +052
        clr.w   0x28(a6)                        | +056
        lea     .L0589c8(pc),a1                 | +05a
        move.l  a1,(a6)                         | +05e
.L0589c8:
        bsr.w   Soldier_TestSurrender_056fa0                    | +060
        bsr.w   Soldier_PhysicsStep_056acc                    | +064
        jsr     0x28d70.l                       | +068
        bcc.w   .L0589e0                        | +06e
        lea     Soldier_Idle_058144(pc),a1      | +072
        move.l  a1,(a6)                         | +076
.L0589e0:
        tst.b   0x78(a6)                        | +078
        bne.w   .L0589ee                        | +07c
        lea     Soldier_Hurt_058412(pc),a1      | +080
        move.l  a1,(a6)                         | +084
.L0589ee:
        jsr     0x49fd0.l                       | +086
        bra.w   Soldier_DespawnIfOffscreen_056e1e            | +08c

| ----------------------------------------------------------------------------
|  Soldier_SurrenderFlee_0589f8  @ $0589F8  (294 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_SurrenderFlee_0589f8, "ax", @progbits
        .global Soldier_SurrenderFlee_0589f8
Soldier_SurrenderFlee_0589f8:
        move.b  0x75(a6),d0                     | +000
        cmpi.b  #0x0,d0                         | +004
        bne.w   Soldier_Retreat_0587ac          | +008
        lea     0x29bb4c.l,a0                   | +00c
        jsr     0x28cd4.l                       | +012
        move.b  #0x6,0x5c(a6)                   | +018
        clr.w   0x28(a6)                        | +01e
        lea     .L058a20(pc),a1                 | +022
        move.l  a1,(a6)                         | +026
.L058a20:
        bsr.w   Soldier_TestSurrender_056fa0                    | +028
        bsr.w   Soldier_PhysicsStep_056acc                    | +02c
        jsr     0x28d70.l                       | +030
        bcc.w   .L058a38                        | +036
        lea     Soldier_FleeStop_0586b8(pc),a1  | +03a
        move.l  a1,(a6)                         | +03e
.L058a38:
        tst.b   0x78(a6)                        | +040
        bne.w   .L058a46                        | +044
        lea     Soldier_Hurt_058412(pc),a1      | +048
        move.l  a1,(a6)                         | +04c
.L058a46:
        jsr     0x49ff2.l                       | +04e
        bra.w   Soldier_DespawnIfOffscreen_056e1e            | +054
        .global Soldier_SurrenderFlee_0589f8__L058a50
Soldier_SurrenderFlee_0589f8__L058a50:
        lea     0x2b5d58.l,a0                   | +058
        jsr     0x28cd4.l                       | +05e
        move.w  #0xfe67,d0                      | +064
        jsr     0x5dca4.l                       | +068
        move.w  d0,0x28(a6)                     | +06e
        move.w  #0x65e,0x2a(a6)                 | +072
        move.w  #0xff5d,0x2e(a6)                | +078
        move.w  #0x0,0x2c(a6)                   | +07e
        lea     .L058a82(pc),a1                 | +084
        move.l  a1,(a6)                         | +088
.L058a82:
        .global Soldier_SurrenderFlee_0589f8__L058a82
Soldier_SurrenderFlee_0589f8__L058a82:
        jsr     0x2831e.l                       | +08a
        scs.b   0x78(a6)                        | +090
        bcc.w   .L058a96                        | +094
        lea     Soldier_Land_058464(pc),a1      | +098
        move.l  a1,(a6)                         | +09c
.L058a96:
        jsr     0x28d70.l                       | +09e
        jsr     0x49fd0.l                       | +0a4
        bsr.w   Soldier_DespawnIfOffscreen_056e1e            | +0aa
        rts                                     | +0ae
        .global Soldier_SurrenderFlee_0589f8__L058aa8
Soldier_SurrenderFlee_0589f8__L058aa8:
        bsr.w   Soldier_Think_056b92                    | +0b0
        btst    #0x0,0x72(a6)                   | +0b4
        beq.w   Soldier_Brake_0585f6            | +0ba
        move.w  #0x190,0x36(a6)                 | +0be
        bsr.w   Soldier_SetVelXByFacing_056f8a                    | +0c4
        lea     0x29b9ce.l,a0                   | +0c8
        jsr     0x28cd4.l                       | +0ce
        lea     .L058ad2(pc),a1                 | +0d4
        move.l  a1,(a6)                         | +0d8
.L058ad2:
        move.w  0x28(a6),d0                     | +0da
        beq.w   .L058ae8                        | +0de
        asr.w   #0x4,d0                         | +0e2
        bne.w   .L058ae4                        | +0e4
        move.w  #0x1,d0                         | +0e8
.L058ae4:
        sub.w   d0,0x28(a6)                     | +0ec
.L058ae8:
        bsr.w   Soldier_PhysicsStep_056acc                    | +0f0
        jsr     0x28d70.l                       | +0f4
        bcc.w   .L058b12                        | +0fa
        move.b  0x74(a6),d0                     | +0fe
        andi.b  #0x7,d0                         | +102
        beq.w   .L058b0c                        | +106
        lea     Soldier_Stand_0588ae(pc),a1     | +10a
        move.l  a1,(a6)                         | +10e
        bra.w   .L058b12                        | +110
.L058b0c:
        lea     Soldier_SpawnStand_0591c8(pc),a1 | +114
        move.l  a1,(a6)                         | +118
.L058b12:
        jsr     0x49fd0.l                       | +11a
        bsr.w   Soldier_DespawnIfOffscreen_056e1e            | +120
        rts                                     | +124

| ----------------------------------------------------------------------------
|  Soldier_ThrowGrenadeA_058b1e  @ $058B1E  (64 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_ThrowGrenadeA_058b1e, "ax", @progbits
        .global Soldier_ThrowGrenadeA_058b1e
Soldier_ThrowGrenadeA_058b1e:
        lea     0x2b60ce.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L058b30(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L058b30:
        clr.w   0x2e(a6)                        | +012
        clr.w   0x2a(a6)                        | +016
        move.b  0x77(a6),d0                     | +01a
        jsr     0x77190.l                       | +01e
        jsr     0x28d70.l                       | +024
        bcc.w   .L058b52                        | +02a
        lea     Soldier_ThrowGrenadeA_Loop_058b5e(pc),a1 | +02e
        move.l  a1,(a6)                         | +032
.L058b52:
        jsr     0x49fd0.l                       | +034
        bsr.w   Soldier_DespawnIfOffscreen_056e1e            | +03a
        rts                                     | +03e

| ----------------------------------------------------------------------------
|  Soldier_ThrowGrenadeA_Loop_058b5e  @ $058B5E  (120 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_ThrowGrenadeA_Loop_058b5e, "ax", @progbits
        .global Soldier_ThrowGrenadeA_Loop_058b5e
Soldier_ThrowGrenadeA_Loop_058b5e:
        lea     0x2b63f6.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        clr.w   0x2e(a6)                        | +00c
        move.w  #0x154,0x2a(a6)                 | +010
        lea     .L058b7a(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L058b7a:
        jsr     Soldier_Think_056b92(pc)                | +01c
        move.b  0x77(a6),d0                     | +020
        jsr     0x77190.l                       | +024
        move.w  d0,-(a7)                        | +02a
        jsr     0x28d70.l                       | +02c
        bcc.w   .L058ba8                        | +032
        lea     Soldier_ThrowGrenadeA_Loop_058b5e(pc),a1 | +036
        move.l  a1,(a6)                         | +03a
        jsr     Soldier_TestSurrender_056fa0(pc)                | +03c
        bcc.w   .L058ba8                        | +040
        lea     Soldier_ThrowGrenadeAim_058d78(pc),a1 | +044
        move.l  a1,(a6)                         | +048
.L058ba8:
        move.w  (a7)+,d0                        | +04a
        bpl.w   .L058bb8                        | +04c
        lea     Soldier_Hurt_058412(pc),a1      | +050
        move.l  a1,(a6)                         | +054
        bra.w   .L058bca                        | +056
.L058bb8:
        subi.w  #0x24,d0                        | +05a
        sub.w   0x24(a6),d0                     | +05e
        bcc.w   .L058bca                        | +062
        lea     Soldier_ThrowGrenadeRecover_058bd6(pc),a1 | +066
        move.l  a1,(a6)                         | +06a
.L058bca:
        jsr     0x49fd0.l                       | +06c
        bsr.w   Soldier_DespawnIfOffscreen_056e1e            | +072
        rts                                     | +076

| ----------------------------------------------------------------------------
|  Soldier_ThrowGrenadeRecover_058bd6  @ $058BD6  (56 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_ThrowGrenadeRecover_058bd6, "ax", @progbits
        .global Soldier_ThrowGrenadeRecover_058bd6
Soldier_ThrowGrenadeRecover_058bd6:
        lea     0x2b6124.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L058be8(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L058be8:
        jsr     0x2783a.l                       | +012
        bsr.w   Soldier_Think_056b92                    | +018
        jsr     0x28d70.l                       | +01c
        bcc.w   .L058c02                        | +022
        lea     Soldier_HopBack_058c0e(pc),a1   | +026
        move.l  a1,(a6)                         | +02a
.L058c02:
        jsr     0x49fd0.l                       | +02c
        bsr.w   Soldier_DespawnIfOffscreen_056e1e            | +032
        rts                                     | +036

| ----------------------------------------------------------------------------
|  Soldier_HopBack_058c0e  @ $058C0E  (128 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_HopBack_058c0e, "ax", @progbits
        .global Soldier_HopBack_058c0e
Soldier_HopBack_058c0e:
        clr.b   0x78(a6)                        | +000
        lea     0x2b6204.l,a0                   | +004
        jsr     0x28cd4.l                       | +00a
        move.w  #0xfe67,d0                      | +010
        jsr     0x5dca4.l                       | +014
        move.w  d0,0x28(a6)                     | +01a
        move.w  #0x32a,0x2a(a6)                 | +01e
        move.w  #0xffaf,0x2e(a6)                | +024
        move.w  #0x0,0x2c(a6)                   | +02a
        lea     .L058c44(pc),a1                 | +030
        move.l  a1,(a6)                         | +034
.L058c44:
        move.w  0x28(a6),d0                     | +036
        beq.w   .L058c58                        | +03a
        asr.w   #0x4,d0                         | +03e
        seq.b   d1                              | +040
        ext.w   d1                              | +042
        sub.w   d1,d0                           | +044
        sub.w   d0,0x28(a6)                     | +046
.L058c58:
        jsr     Soldier_PhysicsStep_056acc(pc)                | +04a
        bsr.w   Soldier_Think_056b92                    | +04e
        jsr     0x28d70.l                       | +052
        bcc.w   .L058c82                        | +058
        tst.b   0x78(a6)                        | +05c
        bne.w   .L058c7c                        | +060
        lea     Soldier_Hurt_058412(pc),a1      | +064
        move.l  a1,(a6)                         | +068
        bra.w   .L058c82                        | +06a
.L058c7c:
        lea     Soldier_Idle_058144(pc),a1      | +06e
        move.l  a1,(a6)                         | +072
.L058c82:
        jsr     0x49fd0.l                       | +074
        bsr.w   Soldier_DespawnIfOffscreen_056e1e            | +07a
        rts                                     | +07e

| ----------------------------------------------------------------------------
|  Soldier_ThrowGrenadeB_058c8e  @ $058C8E  (64 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_ThrowGrenadeB_058c8e, "ax", @progbits
        .global Soldier_ThrowGrenadeB_058c8e
Soldier_ThrowGrenadeB_058c8e:
        lea     0x2b6284.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        clr.w   0x2a(a6)                        | +00c
        clr.w   0x2e(a6)                        | +010
        lea     .L058ca8(pc),a1                 | +014
        move.l  a1,(a6)                         | +018
.L058ca8:
        move.b  0x77(a6),d0                     | +01a
        jsr     0x77190.l                       | +01e
        jsr     0x28d70.l                       | +024
        bcc.w   .L058cc2                        | +02a
        lea     Soldier_ThrowGrenadeB_Loop_058cce(pc),a1 | +02e
        move.l  a1,(a6)                         | +032
.L058cc2:
        jsr     0x49fd0.l                       | +034
        bsr.w   Soldier_DespawnIfOffscreen_056e1e            | +03a
        rts                                     | +03e

| ----------------------------------------------------------------------------
|  Soldier_ThrowGrenadeB_Loop_058cce  @ $058CCE  (118 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_ThrowGrenadeB_Loop_058cce, "ax", @progbits
        .global Soldier_ThrowGrenadeB_Loop_058cce
Soldier_ThrowGrenadeB_Loop_058cce:
        lea     0x2b648a.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        clr.w   0x2e(a6)                        | +00c
        move.w  #0xfeac,0x2a(a6)                | +010
        lea     .L058cea(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L058cea:
        jsr     Soldier_Think_056b92(pc)                | +01c
        move.b  0x77(a6),d0                     | +020
        jsr     0x77190.l                       | +024
        move.w  d0,-(a7)                        | +02a
        jsr     0x28d70.l                       | +02c
        bcc.w   .L058d18                        | +032
        lea     Soldier_ThrowGrenadeB_Loop_058cce(pc),a1 | +036
        move.l  a1,(a6)                         | +03a
        jsr     Soldier_TestSurrender_056fa0(pc)                | +03c
        bcc.w   .L058d18                        | +040
        lea     Soldier_ThrowGrenadeAim_058d78(pc),a1 | +044
        move.l  a1,(a6)                         | +048
.L058d18:
        move.w  (a7)+,d0                        | +04a
        bpl.w   .L058d24                        | +04c
        lea     Soldier_Hurt_058412(pc),a1      | +050
        move.l  a1,(a6)                         | +054
.L058d24:
        jsr     0x27eba.l                       | +056
        scc.b   0x78(a6)                        | +05c
        bcs.w   .L058d38                        | +060
        lea     Soldier_ThrowGrenadeBRecover_058d44(pc),a1 | +064
        move.l  a1,(a6)                         | +068
.L058d38:
        jsr     0x49fd0.l                       | +06a
        bsr.w   Soldier_DespawnIfOffscreen_056e1e            | +070
        rts                                     | +074

| ----------------------------------------------------------------------------
|  Soldier_ThrowGrenadeBRecover_058d44  @ $058D44  (52 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_ThrowGrenadeBRecover_058d44, "ax", @progbits
        .global Soldier_ThrowGrenadeBRecover_058d44
Soldier_ThrowGrenadeBRecover_058d44:
        lea     0x2b63b4.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L058d56(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L058d56:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        bcc.w   .L058d6c                        | +01e
        lea     Soldier_Idle_058144(pc),a1      | +022
        move.l  a1,(a6)                         | +026
.L058d6c:
        jsr     0x49fd0.l                       | +028
        bsr.w   Soldier_DespawnIfOffscreen_056e1e            | +02e
        rts                                     | +032

| ----------------------------------------------------------------------------
|  Soldier_ThrowGrenadeAim_058d78  @ $058D78  (128 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_ThrowGrenadeAim_058d78, "ax", @progbits
        .global Soldier_ThrowGrenadeAim_058d78
Soldier_ThrowGrenadeAim_058d78:
        btst    #0x0,0x72(a6)                   | +000
        beq.w   .L058d98                        | +006
        lea     0x2b6596.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        move.b  #0x7,0x5c(a6)                   | +016
        bra.w   .L058daa                        | +01c
.L058d98:
        lea     0x2b6644.l,a0                   | +020
        jsr     0x28cd4.l                       | +026
        move.b  #0x8,0x5c(a6)                   | +02c
.L058daa:
        lea     .L058db0(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L058db0:
        move.b  0x77(a6),d0                     | +038
        move.w  0x2a(a6),-(a7)                  | +03c
        clr.w   0x2a(a6)                        | +040
        clr.w   0x2e(a6)                        | +044
        jsr     0x77190.l                       | +048
        move.w  (a7)+,0x2a(a6)                  | +04e
        jsr     0x28d70.l                       | +052
        bcc.w   .L058dec                        | +058
        tst.w   0x2a(a6)                        | +05c
        bmi.w   .L058de6                        | +060
        lea     Soldier_ThrowGrenadeA_Loop_058b5e(pc),a1 | +064
        move.l  a1,(a6)                         | +068
        bra.w   .L058dec                        | +06a
.L058de6:
        lea     Soldier_ThrowGrenadeB_Loop_058cce(pc),a1 | +06e
        move.l  a1,(a6)                         | +072
.L058dec:
        jsr     0x49fd0.l                       | +074
        bsr.w   Soldier_DespawnIfOffscreen_056e1e            | +07a
        rts                                     | +07e

| ----------------------------------------------------------------------------
|  Soldier_TauntAnimPtrTbl_058df8  @ $058DF8  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_TauntAnimPtrTbl_058df8, "ax", @progbits
        .global Soldier_TauntAnimPtrTbl_058df8
Soldier_TauntAnimPtrTbl_058df8:
        .dc.w   0x0029                        | +000  (dato / opcode no decodificado)
        .dc.w   0xbde8                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +004  (dato / opcode no decodificado)
        .dc.w   0xbe2c                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +008  (dato / opcode no decodificado)
        .dc.w   0xbe70                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xbf04                        | +00e  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Soldier_TauntInit_058e08  @ $058E08  (12 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_TauntInit_058e08, "ax", @progbits
        .global Soldier_TauntInit_058e08
Soldier_TauntInit_058e08:
        clr.w   0x30(a6)                        | +000
        clr.w   0x28(a6)                        | +004
        clr.w   0x2a(a6)                        | +008

| ----------------------------------------------------------------------------
|  Soldier_Taunt_058e14  @ $058E14  (190 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_Taunt_058e14, "ax", @progbits
        .global Soldier_Taunt_058e14
Soldier_Taunt_058e14:
        jsr     0x5e9b6.l                       | +000
        andi.w  #0xc,d0                         | +006
        lea     Soldier_TauntAnimPtrTbl_058df8(pc),a0 | +00a
        movea.l (a0,d0.w),a0                    | +00e
        jsr     0x28cd4.l                       | +012
        lea     .L058e32(pc),a1                 | +018
        move.l  a1,(a6)                         | +01c
.L058e32:
        jsr     Soldier_PhysicsStep_056acc(pc)                | +01e
        jsr     Soldier_Think_056b92(pc)                | +022
        jsr     0x28d70.l                       | +026
        bcc.w   .L058e4a                        | +02c
        lea     Soldier_Taunt_058e14(pc),a1     | +030
        move.l  a1,(a6)                         | +034
.L058e4a:
        btst    #0x0,0x73(a6)                   | +036
        beq.w   .L058e5e                        | +03c
        move.b  #0x1,0x30(a6)                   | +040
        bra.w   .L058ec6                        | +046
.L058e5e:
        tst.b   0x30(a6)                        | +04a
        beq.w   .L058e6c                        | +04e
        lea     Soldier_TauntEnd_058ed2(pc),a1  | +052
        move.l  a1,(a6)                         | +056
.L058e6c:
        move.w  0x80(a6),d1                     | +058
        jsr     0x5e9b6.l                       | +05c
        move.w  d0,-(a7)                        | +062
        andi.w  #0xff,d0                        | +064
        cmp.w   d1,d0                           | +068
        bcc.w   .L058e86                        | +06a
        move.b  (a7),d0                         | +06e
        cmp.w   d1,d0                           | +070
.L058e86:
        addq.w  #0x2,a7                         | +072
        bcc.w   .L058ea6                        | +074
        btst    #0x0,0x72(a6)                   | +078
        beq.w   .L058ea0                        | +07e
        lea     Soldier_TauntEnd_058ed2(pc),a1  | +082
        move.l  a1,(a6)                         | +086
        bra.w   .L058ea6                        | +088
.L058ea0:
        lea     Soldier_Brake_0585f6(pc),a1     | +08c
        move.l  a1,(a6)                         | +090
.L058ea6:
        btst    #0x3,0x73(a6)                   | +092
        beq.w   .L058eb6                        | +098
        lea     Soldier_TauntEnd_058ed2(pc),a1  | +09c
        move.l  a1,(a6)                         | +0a0
.L058eb6:
        btst    #0x2,0x73(a6)                   | +0a2
        beq.w   .L058ec6                        | +0a8
        lea     Soldier_Brake_0585f6(pc),a1     | +0ac
        move.l  a1,(a6)                         | +0b0
.L058ec6:
        jsr     0x49fd0.l                       | +0b2
        bsr.w   Soldier_DespawnIfOffscreen_056e1e            | +0b8
        rts                                     | +0bc

| ----------------------------------------------------------------------------
|  Soldier_TauntEnd_058ed2  @ $058ED2  (76 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_TauntEnd_058ed2, "ax", @progbits
        .global Soldier_TauntEnd_058ed2
Soldier_TauntEnd_058ed2:
        lea     0x29bfc4.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L058ee4(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L058ee4:
        jsr     0x2783a.l                       | +012
        bsr.w   Soldier_Think_056b92                    | +018
        jsr     0x28d70.l                       | +01c
        bcc.w   .L058f12                        | +022
        btst    #0x0,0x72(a6)                   | +026
        beq.w   .L058f0c                        | +02c
        lea     Soldier_Brake_0585f6(pc),a1     | +030
        move.l  a1,(a6)                         | +034
        bra.w   .L058f12                        | +036
.L058f0c:
        lea     Soldier_WalkStart_057558(pc),a1             | +03a
        move.l  a1,(a6)                         | +03e
.L058f12:
        jsr     0x49fd0.l                       | +040
        bsr.w   Soldier_DespawnIfOffscreen_056e1e            | +046
        rts                                     | +04a

| ----------------------------------------------------------------------------
|  Soldier_SpawnVariantTbl_058f1e  @ $058F1E  (324 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_SpawnVariantTbl_058f1e, "ax", @progbits
        .global Soldier_SpawnVariantTbl_058f1e
Soldier_SpawnVariantTbl_058f1e:
        bset    #0x1,0x12(a6)                   | +000
        bset    #0x4,0x72(a6)                   | +006
        move.b  #0x1,0x98(a6)                   | +00c
        jsr     0x27f60.l                       | +012
        scc.b   0x78(a6)                        | +018
        cmpi.b  #0xa0,0x22(a6)                  | +01c
        bmi.w   .L058f4e                        | +022
        bclr    #0x0,0x3a(a6)                   | +026
        bra.w   .L058f54                        | +02c
.L058f4e:
        bset    #0x0,0x3a(a6)                   | +030
.L058f54:
        bra.w   Soldier_SpawnEnter_059086       | +036
        bset    #0x1,0x12(a6)                   | +03a
        bset    #0x4,0x72(a6)                   | +040
        move.b  #0x1,0x98(a6)                   | +046
        jsr     0x27f60.l                       | +04c
        scc.b   0x78(a6)                        | +052
        move.w  #0xe,d1                         | +056
        jsr     0x236e.l                        | +05a
        bra.w   Soldier_SpawnFaceTarget_059062  | +060
        bset    #0x1,0x12(a6)                   | +064
        bset    #0x4,0x72(a6)                   | +06a
        move.b  #0x1,0x98(a6)                   | +070
        jsr     0x27f60.l                       | +076
        scc.b   0x78(a6)                        | +07c
        bra.w   Soldier_SpawnFaceTarget_059062  | +080
        bset    #0x1,0x12(a6)                   | +084
        bset    #0x4,0x72(a6)                   | +08a
        move.b  #0x1,0x98(a6)                   | +090
        jsr     0x27f60.l                       | +096
        scc.b   0x78(a6)                        | +09c
        bra.w   Soldier_SpawnStand_0591c8       | +0a0
        bset    #0x1,0x12(a6)                   | +0a4
        bset    #0x4,0x72(a6)                   | +0aa
        move.b  #0x1,0x98(a6)                   | +0b0
        jsr     0x27f60.l                       | +0b6
        scc.b   0x78(a6)                        | +0bc
        bra.w   Soldier_SpawnStand_0591c8__L0591de | +0c0
        bset    #0x1,0x12(a6)                   | +0c4
        bclr    #0x4,0x72(a6)                   | +0ca
        move.b  #0x1,0x98(a6)                   | +0d0
        jsr     0x27f60.l                       | +0d6
        scc.b   0x78(a6)                        | +0dc
        bra.w   Soldier_SpawnFaceTarget_059062  | +0e0
        bset    #0x1,0x12(a6)                   | +0e4
        bclr    #0x4,0x72(a6)                   | +0ea
        move.b  #0x1,0x98(a6)                   | +0f0
        jsr     0x27f60.l                       | +0f6
        scc.b   0x78(a6)                        | +0fc
        bra.w   Soldier_SpawnStand_0591c8       | +100
        bset    #0x1,0x12(a6)                   | +104
        bclr    #0x4,0x72(a6)                   | +10a
        move.b  #0x1,0x98(a6)                   | +110
        jsr     0x27f60.l                       | +116
        scc.b   0x78(a6)                        | +11c
        bra.w   Soldier_SpawnStand_0591c8__L0591de | +120
        bset    #0x1,0x12(a6)                   | +124
        bclr    #0x4,0x72(a6)                   | +12a
        move.b  #0x1,0x98(a6)                   | +130
        jsr     0x27f60.l                       | +136
        scc.b   0x78(a6)                        | +13c
        bra.w   Soldier_SpawnBrake_059228       | +140

| ----------------------------------------------------------------------------
|  Soldier_SpawnFaceTarget_059062  @ $059062  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_SpawnFaceTarget_059062, "ax", @progbits
        .global Soldier_SpawnFaceTarget_059062
Soldier_SpawnFaceTarget_059062:
        move.w  #0xffff,d0                      | +000
        jsr     0x2352.l                        | +004
        cmpi.w  #0xa0,0x22(a6)                  | +00a
        bmi.w   .L059080                        | +010
        bset    #0x0,0x3a(a6)                   | +014
        bra.w   Soldier_SpawnEnter_059086       | +01a
.L059080:
        bclr    #0x0,0x3a(a6)                   | +01e

| ----------------------------------------------------------------------------
|  Soldier_SpawnEnter_059086  @ $059086  (192 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_SpawnEnter_059086, "ax", @progbits
        .global Soldier_SpawnEnter_059086
Soldier_SpawnEnter_059086:
        btst    #0x4,0x72(a6)                   | +000
        bne.w   .L0590a0                        | +006
        lea     0x29b744.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        bra.w   .L0590ac                        | +016
.L0590a0:
        lea     0x2b604a.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
.L0590ac:
        lea     0x2b75d0.l,a0                   | +026
        jsr     0x799de.l                       | +02c
        move.w  d0,0x36(a6)                     | +032
        bsr.w   Soldier_SetVelXByFacing_056f8a                    | +036
        clr.w   0x2a(a6)                        | +03a
        lea     .L0590ca(pc),a1                 | +03e
        move.l  a1,(a6)                         | +042
.L0590ca:
        jsr     Soldier_PhysicsStep_056acc(pc)                | +044
        bsr.w   Soldier_Think_056b92                    | +048
        jsr     0x28d70.l                       | +04c
        btst    #0x0,0x72(a6)                   | +052
        beq.w   .L05910c                        | +058
        jsr     0x5e9b6.l                       | +05c
        move.w  d0,-(a7)                        | +062
        andi.w  #0xff,d0                        | +064
        cmp.w   0x80(a6),d0                     | +068
        bcc.w   .L0590fc                        | +06c
        move.b  (a7),d0                         | +070
        cmp.w   0x80(a6),d0                     | +072
.L0590fc:
        addq.w  #0x2,a7                         | +076
        bcc.w   .L059108                        | +078
        lea     Soldier_SpawnBrake_059228(pc),a1 | +07c
        move.l  a1,(a6)                         | +080
.L059108:
        bra.w   .L05911c                        | +082
.L05910c:
        btst    #0x6,0x72(a6)                   | +086
        beq.w   .L05911c                        | +08c
        lea     Soldier_SpawnLeap_05925a(pc),a1 | +090
        move.l  a1,(a6)                         | +094
.L05911c:
        btst    #0x5,0x72(a6)                   | +096
        beq.w   .L05912c                        | +09c
        lea     Soldier_SpawnWait_059146(pc),a1 | +0a0
        move.l  a1,(a6)                         | +0a4
.L05912c:
        tst.b   0x78(a6)                        | +0a6
        bne.w   .L05913a                        | +0aa
        lea     Soldier_SpawnLeap_05925a(pc),a1 | +0ae
        move.l  a1,(a6)                         | +0b2
.L05913a:
        jsr     0x49fd0.l                       | +0b4
        bsr.w   Soldier_DespawnIfOffscreen_056e1e            | +0ba
        rts                                     | +0be

| ----------------------------------------------------------------------------
|  Soldier_SpawnWait_059146  @ $059146  (130 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_SpawnWait_059146, "ax", @progbits
        .global Soldier_SpawnWait_059146
Soldier_SpawnWait_059146:
        btst    #0x0,0x72(a6)                   | +000
        bne.w   Soldier_SpawnBrake_059228       | +006
        move.b  #0xb4,0x5c(a6)                  | +00a
        lea     0x2b6ef8.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        lea     .L059168(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L059168:
        jsr     Soldier_PhysicsStep_056acc(pc)                | +022
        bsr.w   Soldier_Think_056b92                    | +026
        jsr     0x28d70.l                       | +02a
        btst    #0x0,0x72(a6)                   | +030
        beq.w   .L059190                        | +036
        btst    #0x0,0x73(a6)                   | +03a
        bne.w   .L059190                        | +040
        lea     Soldier_SpawnBrake_059228(pc),a1 | +044
        move.l  a1,(a6)                         | +048
.L059190:
        btst    #0x5,0x72(a6)                   | +04a
        bne.w   .L0591a0                        | +050
        lea     Soldier_SpawnEnter_059086(pc),a1 | +054
        move.l  a1,(a6)                         | +058
.L0591a0:
        tst.b   0x78(a6)                        | +05a
        bne.w   .L0591ae                        | +05e
        lea     Soldier_SpawnHurt_0592b2(pc),a1 | +062
        move.l  a1,(a6)                         | +066
.L0591ae:
        subq.b  #0x1,0x5c(a6)                   | +068
        bne.w   .L0591bc                        | +06c
        lea     Soldier_SpawnBrake_059228(pc),a1 | +070
        move.l  a1,(a6)                         | +074
.L0591bc:
        jsr     0x49fd0.l                       | +076
        bsr.w   Soldier_DespawnIfOffscreen_056e1e            | +07c
        rts                                     | +080

| ----------------------------------------------------------------------------
|  Soldier_SpawnStand_0591c8  @ $0591C8  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_SpawnStand_0591c8, "ax", @progbits
        .global Soldier_SpawnStand_0591c8
Soldier_SpawnStand_0591c8:
        lea     0x29bd84.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     Soldier_SpawnStand_Loop_0591f4(pc),a1 | +00c
        move.l  a1,(a6)                         | +010
        bra.w   Soldier_SpawnStand_Loop_0591f4  | +012
        .global Soldier_SpawnStand_0591c8__L0591de
Soldier_SpawnStand_0591c8__L0591de:
        lea     0x29bd40.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
        clr.w   0x28(a6)                        | +022
        lea     Soldier_SpawnStand_Loop_0591f4(pc),a1 | +026
        move.l  a1,(a6)                         | +02a

| ----------------------------------------------------------------------------
|  Soldier_SpawnStand_Loop_0591f4  @ $0591F4  (52 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_SpawnStand_Loop_0591f4, "ax", @progbits
        .global Soldier_SpawnStand_Loop_0591f4
Soldier_SpawnStand_Loop_0591f4:
        jsr     Soldier_PhysicsStep_056acc(pc)                | +000
        jsr     0x28d70.l                       | +004
        bcc.w   .L05920e                        | +00a
        lea     Soldier_SpawnBrake_059228(pc),a1 | +00e
        move.l  a1,(a6)                         | +012
        jsr     0x8f308.l                       | +014
.L05920e:
        tst.b   0x78(a6)                        | +01a
        bne.w   .L05921c                        | +01e
        lea     Soldier_SpawnHurt_0592b2(pc),a1 | +022
        move.l  a1,(a6)                         | +026
.L05921c:
        jsr     0x49fd0.l                       | +028
        bsr.w   Soldier_DespawnIfOffscreen_056e1e            | +02e
        rts                                     | +032

| ----------------------------------------------------------------------------
|  Soldier_SpawnBrake_059228  @ $059228  (50 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_SpawnBrake_059228, "ax", @progbits
        .global Soldier_SpawnBrake_059228
Soldier_SpawnBrake_059228:
        lea     0x2b5cfa.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L05923a(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L05923a:
        jsr     Soldier_PhysicsStep_056acc(pc)                | +012
        jsr     0x28d70.l                       | +016
        bcc.w   .L05924e                        | +01c
        lea     Soldier_SpawnEnter_059086(pc),a1 | +020
        move.l  a1,(a6)                         | +024
.L05924e:
        jsr     0x49fd0.l                       | +026
        bsr.w   Soldier_DespawnIfOffscreen_056e1e            | +02c
        rts                                     | +030

| ----------------------------------------------------------------------------
|  Soldier_SpawnLeap_05925a  @ $05925A  (88 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_SpawnLeap_05925a, "ax", @progbits
        .global Soldier_SpawnLeap_05925a
Soldier_SpawnLeap_05925a:
        lea     0x2b5d58.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        move.w  #0xfe15,d0                      | +00c
        jsr     0x5dca4.l                       | +010
        move.w  d0,0x28(a6)                     | +016
        move.w  #0x7aa,0x2a(a6)                 | +01a
        move.w  #0xff63,0x2e(a6)                | +020
        move.w  #0x0,0x2c(a6)                   | +026
        lea     .L05928c(pc),a1                 | +02c
        move.l  a1,(a6)                         | +030
.L05928c:
        jsr     0x2831e.l                       | +032
        scs.b   0x78(a6)                        | +038
        bcc.w   .L0592a0                        | +03c
        lea     Soldier_SpawnRecover_0592f6(pc),a1 | +040
        move.l  a1,(a6)                         | +044
.L0592a0:
        jsr     0x28d70.l                       | +046
        jsr     0x49fd0.l                       | +04c
        bsr.w   Soldier_DespawnIfOffscreen_056e1e            | +052
        rts                                     | +056

| ----------------------------------------------------------------------------
|  Soldier_SpawnHurt_0592b2  @ $0592B2  (68 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_SpawnHurt_0592b2, "ax", @progbits
        .global Soldier_SpawnHurt_0592b2
Soldier_SpawnHurt_0592b2:
        lea     0x2b5b92.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        clr.w   0x2a(a6)                        | +00c
        lea     .L0592c8(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L0592c8:
        move.w  0x28(a6),d0                     | +016
        asr.w   #0x6,d0                         | +01a
        sub.w   d0,0x28(a6)                     | +01c
        bsr.w   Soldier_PhysicsStep_056acc                    | +020
        tst.b   0x78(a6)                        | +024
        beq.w   .L0592e4                        | +028
        lea     Soldier_SpawnRecover_0592f6(pc),a1 | +02c
        move.l  a1,(a6)                         | +030
.L0592e4:
        jsr     0x28d70.l                       | +032
        jsr     0x49fd0.l                       | +038
        bsr.w   Soldier_DespawnIfOffscreen_056e1e            | +03e
        rts                                     | +042

| ----------------------------------------------------------------------------
|  Soldier_SpawnRecover_0592f6  @ $0592F6  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_SpawnRecover_0592f6, "ax", @progbits
        .global Soldier_SpawnRecover_0592f6
Soldier_SpawnRecover_0592f6:
        bsr.w   Soldier_PickFallAnim_05740e                    | +000
        clr.w   0x28(a6)                        | +004
        lea     .L059304(pc),a1                 | +008
        move.l  a1,(a6)                         | +00c
.L059304:
        jsr     Soldier_PhysicsStep_056acc(pc)                | +00e
        jsr     0x28d70.l                       | +012
        bcc.w   .L059318                        | +018
        lea     Soldier_SpawnEnter_059086(pc),a1 | +01c
        move.l  a1,(a6)                         | +020
.L059318:
        tst.b   0x78(a6)                        | +022
        bne.w   .L059326                        | +026
        lea     Soldier_SpawnHurt_0592b2(pc),a1 | +02a
        move.l  a1,(a6)                         | +02e
.L059326:
        jsr     0x49fd0.l                       | +030
        bsr.w   Soldier_DespawnIfOffscreen_056e1e            | +036
        rts                                     | +03a

| ----------------------------------------------------------------------------
|  Entity_CmpField10WithLink8_059332  @ $059332  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpField10WithLink8_059332, "ax", @progbits
        .global Entity_CmpField10WithLink8_059332
Entity_CmpField10WithLink8_059332:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_059348                    | +00c
