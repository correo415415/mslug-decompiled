| =============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave KKK — Fortaleza con casamatas, trampillas, blindado y helicóptero
|  Región: $0865BE..$088A56  (9,334 B, 84 entradas, 6 huecos cerrados)
| =============================================================================
|
|  Cluster completo de entidades de la escena 4 del dispatcher de spawn
|  (lista $0972CC = JumpTable_096B9C[4], registros de 20 B {$0100, x, y,
|  handler.l, 0, 0, FFFF x3} con centinela $FFFF). Todas las entradas
|  ($0865DC/$086624/$086666/$0866AE/$0866F0/$086F1C/$086F40/$08716A/
|  $0871FA/$087280/$0872FA/$087366/$0877D4/$087B1C/$087FBC/$088114) están
|  referenciadas desde esa lista o desde la lista 7 ($0975A2, x>=$1670).
|  Además `Heli_InitTmpl_087b26` es la plantilla $E8000[269] (op $00 del
|  Mission VM).
|
|  Convenciones comunes a casi todos los handlers del cluster:
|    * `movea.l +$3C(a6),a1; jsr $2942A` — lee x/y del registro de spawn
|      (ajustando x & $FFF en las misiones $B/$C) y los convierte con $440D0.
|    * `jsr $2783A` física; `jsr $28D70` slots enlazados (C=1 => sin hijos
|      vivos); `jsr $2870A` daño (C=1 => golpe recibido: flash $5E766 vía
|      $5E770 + bclr bit3 de +$13); `jsr $28758` colisión con jugador.
|    * `jsr $4FA70` — cull por scroll: C=1 si x < -(+$70) => `jmp $518`.
|    * `move.w #$82,d0; move.b #0,+$82(a6); jsr $4429E` — MissionWatch_Spawn
|      sobre la lista aux en +$7C (habilita la siguiente oleada de la misión).
|    * +$66 = HP, +$70 = margen de cull, +$72 = timer/índice, +$74 = tabla de
|      blit (StateMachineRun $5022A), +$78/+$7C/+$88/+$8C = listas de
|      sprites/aux, +$48 = puntero de hitbox ($FFFF = sin hitbox).
|
|  A) $0865BE..$086868 — FORTALEZA (contenedor)
|     Boss_Shadow_Clear_0865be: $10E39E=0, hitbox off, jsr $283CA, $518
|       (cola del thunk SetTaskHandler_0865b6; complementa Boss_Shadow_Init).
|     Fort_Init_V0..V4 ($865DC/$86624/$86666/$866AE/$866F0): 5 variantes
|       (+$20=0..4) que llaman a su Fort_SpawnChildren_V<n> (casamatas +
|       cajas), cargan las listas +$78 ($2EDCB0..$2EDD20) / +$7C
|       ($EDC8C..$EDE22) y crean la trampilla Hatch_Init_V<n> (+$50,+$10 en
|       las pares). Convergen en Fort_Init_Common_086738: +$70=$C0, HP
|       +$66=$140, hitbox $2EC41E (se apaga si x<=-$40), re-arma HP si
|       x>=$C0, y al llegar a 0 pasa a Fort_Destroyed_0867d4 (+$21=$FF);
|       se autodestruye por cull (+$21=$FF).
|     Fort_Destroyed_0867d4: snd $1028, tres pares de escombros
|       ($2ECF68/$2ECF7A/$2ED04A vía $77C7E), Fort_BlitWreck_088a28, blit de
|       fila $43FAC con +$78, caja $2ECC88 con $283CA x2 + $283D8, y
|       MissionWatch_Spawn con +$7C. Cae en el thunk SetTaskHandler_08684c ->
|       Fort_Idle_086854 (hitbox off, $283CA, $518).
|
|  B) $08686A..$086DB4 — CASAMATAS Y CAJAS (hijos de la fortaleza)
|     Pillbox_Init_08686a (+6 variantes por `bra.w`; las globales
|       Pillbox_Init_Lower/Right/RightUpper son targets `lea pc` de los
|       Fort_SpawnChildren): snd $E3/$E4, +$21 = $00/$30/$60/$01/$31/$61
|       (nibble alto = fila de sprite en $2EB2C0[(+$21>>4)<<2], bit0 =
|       orientación), HP $28 (+$66 y copia +$80), +$38=$8000, $267E2 limpia
|       velocidades, probe $27CEE. Bucle: muere con el padre (+$21 del padre
|       = $FF => blit $2EDD48 + $518); daño: snd $108D +
|       Entity_PropagateDamageToParent_08848c; HP<=$1E => Damaged1, <=$1A =>
|       Damaged2 (sprite fila+1/+2), <=$D => Pillbox_Critical_086abc.
|     Pillbox_Critical_086abc: al contacto con el jugador ($28758) apaga la
|       hitbox; si +$21 bit0 => Pillbox_Collapse_086ba4 (spawnea
|       Pillbox_Fragment_086c62: snd $6C, sprite $2EB2E4, marca `ori.b #$F`
|       en +$21 del padre cuando el abuelo muere) y espera a que el nibble
|       bajo de +$21 sea $F; si no => score $100 ($51A28), snd $102F/$102C,
|       lista $78066 vía $4AE/$5DD22, blit +$74 y par $2ED05C, y
|       MissionWatch_Spawn con +$7C si != -1.
|     Crate_Init_086cd0 (V0..V4 = snd $E7..$EB, +$21=0..4): HP $14,
|       +$38=0; muere con el padre; al contacto del jugador apaga la hitbox y
|       pasa a Crate_Burst_086db4: score $100, snd $1039, sprite
|       $2EB6BA[+$21<<2], y cuando no quedan hijos snd $102F + lista $7808A +
|       blit +$74 + par $2ED05C. El `bcc.w` colgante de Crate_Init apunta al
|       thunk JsrPcThunk_086dae (-> Entity_HitboxPulseTable).
|
|  C) $086E4A..$08716A — TRAMPILLA DE TROPAS
|     Hatch_Init_086e4a (V0..V4 + 2 variantes sin padre en +$D2/+$F6 con
|       +$20=5/6 y lectura del registro de spawn): +$70=$30/$20, +$20=n,
|       +$21=1, listas +$7C ($EDE9E..$EDFAA) y +$8C ($EDCB2..$EDE7E).
|     Ciclo Hatch_WaitClosed_086f64 (90 frames) -> Hatch_Opening_086fb4
|       (sprite $2EBFAE[+$20<<2], espera a que no haya hijos) ->
|       Hatch_SpawnTroops_087004 (timer $28, MissionWatch_Spawn con +$7C;
|       avanza cuando Entity_CmpPrioWithSibling_086552 devuelve C=0) ->
|       Hatch_WaitOpen_087068 (80 frames) -> Hatch_Closing_0870b8 (sprite
|       $2EBFCA) -> vuelve a WaitClosed. Si +$21==1 y el padre ha muerto
|       (+$21=$FF): Hatch_FinalOpen_087108 -> Hatch_FinalSpawn_087130
|       (MissionWatch_Spawn con +$8C y $518).
|
|  D) $08716A..$087366 — ATREZZO DESTRUIBLE
|     Prop_Breakable_A/B/C ($8716A/$871FA/$87280): +$70=$20, hitbox
|       $2EC726, daño con flash; al contacto del jugador snd $10A6 (sólo A),
|       par $2ED0AE y 1-2 blits ($2ED878/$2ED864, $2ED968/$2ED97C, $2EDA6C).
|     Prop_Signboard_0872fa: sprite $2EBFE6, al recibir daño snd $10A9 y
|       sprite roto $2EBFFC; HP=$7FFF (indestructible, sólo cambia sprite).
|
|  E) $087366..$0877D4 — MURO DE 13 CAJAS
|     CrateWall_Spawn13_087366: crea 13 piezas con offsets fijos
|       (dx -$10..+$A0, dy 0..$A0) y termina en $518.
|     CrateWall_Piece00..12: cada pieza fija su tabla de blit +$74
|       ($2ED990..$2EDA94), lista aux +$7C ($EDFE2/$EE002/$EE022 ó -1), índice
|       +$72 (0..12), hitbox +$48 ($2EC822/$2EC876/$2EC8CA/$2EC91E/$2EC972/
|       $2EC9C6) y par +$88 ($2ECFDE..$2ED038), y salta a
|       CrateWall_Piece_Common_087730: +$70=$30, HP $14, guarda la hitbox en
|       +$84; al contacto del jugador hitbox off, snd $1027, blit +$74, par
|       +$88 y MissionWatch_Spawn con +$7C si != -1. El `bcc.w` colgante va
|       al thunk JsrPcThunk_0877ce (-> Entity_HitboxPulseSaved).
|
|  F) $0877D4..$087B1C — BLINDADO
|     ArmoredCar_Init_0877d4: snd $E5, +$70=$F0, sprite $2EC180, torreta
|       hija ArmoredCar_Turret_087aa0; espera a que x<$100 (limpiando
|       $10E39A) y cae en ArmoredCar_Engage_087846: HP aleatorio ($2C0628 vía
|       $799DE) + $2E4, +$60=$2EDE46, $28998 (dispatcher de jugadores);
|       HP<=$29A => par $2ED0C0 + ArmoredCar_Damaged1_0878d8 (snd $102B,
|       sprite $2EC19A); HP<=$F6 => par $2ED0D2 + ArmoredCar_Damaged2_087956
|       (sprite $2EC1AE); al contacto: snd $1038, pares $2ED0E4/$2ECFCC,
|       blits de fila $2EDD56/$2EDD68, spawn de ArmoredCar_BlastBox_087a62
|       (16 frames de caja $2ECC34) y ArmoredCar_Wreck_087a18 (score $5000,
|       sprite $2EC1C2, hitbox off).
|     ArmoredCar_Turret_087aa0: snd $E5, hitbox $8000/$80 ($2813C), sigue
|       x/y del padre y toma su sprite de $2EC21A[+$20 del padre<<2]; muere
|       cuando el padre pasa a +$20==3.
|
|  G) $087B1C..$087F4C — HELICÓPTERO
|     Heli_Init_087b1c (Heli_InitTmpl_087b26 = entrada de plantilla sin
|       lectura del registro de spawn): snd $86+$87, +$70=$50, HP aleatorio
|       ($2C05A6) + $17C, +$38=$2000, sprite $2EC226, hijo
|       Heli_Dropper_087eae, blit de fila $2EDD7E, +$60=$2EDDFA. Bucle con
|       $28998, Heli_RotorAnim_0883ec, daño (+$72=$F => 15 frames de
|       parpadeo); cuando x<=$100 => Heli_Approach_087c22 (sprite $2EC23C;
|       cuando no quedan hijos => Heli_Hover_087c9a: MissionWatch_Spawn con
|       $EE042; HP<=$FC => Heli_Damaged1_087d1c (snd $1027, $2EC2CE);
|       HP<=$7E => Heli_Damaged2_087d9e ($2EC2DE); al contacto =>
|       Heli_Crash_087e20: score $5000, snd $1023, lista $77FD6, blit de fila
|       $2EDD92, pares $2ED176/$2ED188, sprite $2EC2EE).
|     Heli_Dropper_087eae: snd $88, sprite $2EC2FE, lanza soldado $77228
|       (+$58, +$98=$30, +$99=$83) y, cuando el padre muere (+$20=$FF),
|       $77F6A.
|     Heli_RotorAnim_0883ec: alterna +$16/+$18 (par de tiles del rotor) cada
|       4 frames mientras +$72>0 (parpadeo de daño, decrementa +$72) o cada
|       16 frames en reposo; cae en SetTaskW_088432 (rts en +4 = defsym
|       SetTaskWRts_088436).
|
|  H) $087F4C..$088114 — ESCOMBRO Y ATREZZO CUÁDRUPLE
|     Debris_Scatter_087f4c: snd $F3, velocidad X aleatoria ($5EA1C & $7F,
|       signo por bit0), vel Y -= |dx|, sprite $2DE4B0, probe $27CEE; cuando
|       no quedan hijos y $5DD56 (wait-anim de $298736) devuelve C=1 => $518.
|     Prop_Quad_Spawn_087fbc: crea Prop_Quad_A..D ($8800E..$8805C, sprites
|       $2EC30E..$2EC350, pares $2ED19A..$2ED1D0) que convergen en
|       Prop_Quad_Common_088076: snd $E4, +$70=$40, HP $A, +$38=$8000; al
|       contacto snd $102E, hitbox off y par +$88.
|
|  I) $088114..$0883EC — BARRICADA
|     Barricade_Init_088114: snd $E6, +$70=$40, HP $32 (re-armado cada
|       frame hasta x<=$50) -> Barricade_Arm_0881a8 (sprite $2EC366; sin
|       hijos => Barricade_Stage1_08820a: MissionWatch_Spawn $EE062; HP<=$28
|       => Stage2 $2EC3DE; <=$1E => Stage3 $2EC3EE; <=$14 => Stage4 $2EC3FE;
|       <=$A => par $2ED1E2 + Barricade_Stage5_088390 ($2EC40E, espera
|       Entity_CmpPrioWithSibling C=0, contacto => $518 incondicional:
|       el `jsr $4FA70; bcc.w` que sigue es código muerto).
|
|  J) $088438..$088A56 — HELPERS
|     Entity_HitboxPulseTable_088438: cada 4 frames (+$72) carga +$48 desde
|       $2EC712[+$21<<2], el resto apaga la hitbox (pulso de colisión).
|     Entity_HitboxPulseSaved_08846a: idem con la hitbox guardada en +$84.
|     Entity_PropagateDamageToParent_08848c: resta al padre (+$66) la
|       diferencia de HP perdida desde la última llamada (+$80).
|     Fort_SpawnChildren_V0..V4 ($884A6/$885B4/$886C2/$887D0/$8891A): crean
|       3-4 casamatas (Pillbox_Init_*, +$38=$60/$70, tablas de blit
|       $2ED738..$2ED954, listas aux $EE066..$EE19E) y 2 cajas
|       (Crate_Init_V<n>) con offsets fijos; devuelven con rts.
|     Fort_BlitWreck_088a28: si +$20==0 blits $2EDAA8/$2EDABC/$2EDAD0; si no
|       salta al hueco futuro Sub_00088A64 (defsym forward).
|
|  Rarezas de matching: `movea.l #-1,a0` pisado por `lea` en Debris_Scatter
|  (+$52), `addi.w #0,+$24(a0)` (x4) en los Fort_SpawnChildren, `jmp $518`
|  seguido de `rts` inalcanzable (x7), `bra.w` a la instrucción siguiente en
|  los Fort_Init (+$158) y `jsr $434DC` (rts puro) en Fort_Destroyed y
|  ArmoredCar_Damaged2.
|
|  Verificación: cada sección .text.<Sym> se coloca en su dirección CPU
|  absoluta y reensambla byte-exacta contra build/mslug_prom.bin
|  (MD5 816b3f74c76b3373993407615f1850fe).
| =============================================================================

        .text

| ----------------------------------------------------------------------------
|  Boss_Shadow_Clear_0865be  @ $0865BE  (28 B)
| ----------------------------------------------------------------------------
        .section .text.Boss_Shadow_Clear_0865be, "ax", @progbits
        .global Boss_Shadow_Clear_0865be
Boss_Shadow_Clear_0865be:
        move.b  #0x0,0x10e39e.l                 | +000
        lea     0xffff.w,a0                     | +008
        move.l  a0,0x4c(a6)                     | +00c
        jsr     0x283ca.l                       | +010
        jmp     0x518.l                         | +016

| ----------------------------------------------------------------------------
|  Rts_0865da  @ $0865DA  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Rts_0865da, "ax", @progbits
        .global Rts_0865da
Rts_0865da:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  Fort_Init_V0_0865dc  @ $0865DC  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Fort_Init_V0_0865dc, "ax", @progbits
        .global Fort_Init_V0_0865dc
Fort_Init_V0_0865dc:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.b  #0x0,0x20(a6)                   | +00a
        jsr     Fort_SpawnChildren_V0_0884a6(pc) | +010
        lea     0x2edcb0.l,a1                   | +014
        move.l  a1,0x78(a6)                     | +01a
        lea     0xedc8c.l,a1                    | +01e
        move.l  a1,0x7c(a6)                     | +024
        lea     Hatch_Init_086e4a(pc),a1        | +028
        jsr     0x4ae.l                         | +02c
        jsr     0x5dd22.l                       | +032
        addi.w  #0x50,0x22(a0)                  | +038
        addi.w  #0x10,0x24(a0)                  | +03e
        bra.w   Fort_Init_Common_086738    | +044

| ----------------------------------------------------------------------------
|  Fort_Init_V1_086624  @ $086624  (66 B)
| ----------------------------------------------------------------------------
        .section .text.Fort_Init_V1_086624, "ax", @progbits
        .global Fort_Init_V1_086624
Fort_Init_V1_086624:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.b  #0x1,0x20(a6)                   | +00a
        jsr     Fort_SpawnChildren_V1_0885b4(pc) | +010
        lea     0x2edcba.l,a1                   | +014
        move.l  a1,0x78(a6)                     | +01a
        lea     0xedcd2.l,a1                    | +01e
        move.l  a1,0x7c(a6)                     | +024
        lea     Hatch_Init_V1_086e74(pc),a1 | +028
        jsr     0x4ae.l                         | +02c
        jsr     0x5dd22.l                       | +032
        addi.w  #0x50,0x22(a0)                  | +038
        bra.w   Fort_Init_Common_086738    | +03e

| ----------------------------------------------------------------------------
|  Fort_Init_V2_086666  @ $086666  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Fort_Init_V2_086666, "ax", @progbits
        .global Fort_Init_V2_086666
Fort_Init_V2_086666:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.b  #0x2,0x20(a6)                   | +00a
        jsr     Fort_SpawnChildren_V2_0886c2(pc) | +010
        lea     0x2edcfe.l,a1                   | +014
        move.l  a1,0x78(a6)                     | +01a
        lea     0xedd2a.l,a1                    | +01e
        move.l  a1,0x7c(a6)                     | +024
        lea     Hatch_Init_V2_086e9e(pc),a1 | +028
        jsr     0x4ae.l                         | +02c
        jsr     0x5dd22.l                       | +032
        addi.w  #0x50,0x22(a0)                  | +038
        addi.w  #0x10,0x24(a0)                  | +03e
        bra.w   Fort_Init_Common_086738    | +044

| ----------------------------------------------------------------------------
|  Fort_Init_V3_0866ae  @ $0866AE  (66 B)
| ----------------------------------------------------------------------------
        .section .text.Fort_Init_V3_0866ae, "ax", @progbits
        .global Fort_Init_V3_0866ae
Fort_Init_V3_0866ae:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.b  #0x3,0x20(a6)                   | +00a
        jsr     Fort_SpawnChildren_V3_0887d0(pc) | +010
        lea     0x2edd0c.l,a1                   | +014
        move.l  a1,0x78(a6)                     | +01a
        lea     0xedda6.l,a1                    | +01e
        move.l  a1,0x7c(a6)                     | +024
        lea     Hatch_Init_V3_086ec8(pc),a1 | +028
        jsr     0x4ae.l                         | +02c
        jsr     0x5dd22.l                       | +032
        addi.w  #0x50,0x22(a0)                  | +038
        bra.w   Fort_Init_Common_086738    | +03e

| ----------------------------------------------------------------------------
|  Fort_Init_V4_0866f0  @ $0866F0  (228 B)
| ----------------------------------------------------------------------------
        .section .text.Fort_Init_V4_0866f0, "ax", @progbits
        .global Fort_Init_V4_0866f0
Fort_Init_V4_0866f0:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.b  #0x4,0x20(a6)                   | +00a
        jsr     Fort_SpawnChildren_V4_08891a(pc) | +010
        lea     0x2edd20.l,a1                   | +014
        move.l  a1,0x78(a6)                     | +01a
        lea     0xede22.l,a1                    | +01e
        move.l  a1,0x7c(a6)                     | +024
        lea     Hatch_Init_V4_086ef2(pc),a1 | +028
        jsr     0x4ae.l                         | +02c
        jsr     0x5dd22.l                       | +032
        addi.w  #0x50,0x22(a0)                  | +038
        addi.w  #0x10,0x24(a0)                  | +03e
        bra.w   Fort_Init_Common_086738    | +044
        .global Fort_Init_Common_086738
Fort_Init_Common_086738:
        move.w  #0xc0,0x70(a6)                  | +048
        move.w  #0x140,0x66(a6)                 | +04e
        move.b  #0x0,0x21(a6)                   | +054
        lea     0x2ec41e.l,a0                   | +05a
        move.l  a0,0x48(a6)                     | +060
        lea     .L08675a(pc),a1                 | +064
        move.l  a1,(a6)                         | +068
.L08675a:
        cmpi.w  #0xffc0,0x22(a6)                | +06a
        bgt.w   .L08676c                        | +070
        lea     0xffff.w,a0                     | +074
        move.l  a0,0x48(a6)                     | +078
.L08676c:
        jsr     0x2783a.l                       | +07c
        jsr     0x2870a.l                       | +082
        bcc.w   .L08678e                        | +088
        lea     0x5e766.l,a0                    | +08c
        jsr     0x5e770.l                       | +092
        bclr    #0x3,0x13(a6)                   | +098
.L08678e:
        cmpi.w  #0xc0,0x22(a6)                  | +09e
        blt.w   .L08679e                        | +0a4
        move.w  #0x140,0x66(a6)                 | +0a8
.L08679e:
        cmpi.w  #0x0,0x66(a6)                   | +0ae
        bgt.w   .L0867bc                        | +0b4
        lea     0xffff.w,a0                     | +0b8
        move.l  a0,0x48(a6)                     | +0bc
        move.b  #0xff,0x21(a6)                  | +0c0
        lea     Fort_Destroyed_0867d4(pc),a1    | +0c6
        move.l  a1,(a6)                         | +0ca
.L0867bc:
        jsr     0x4fa70.l                       | +0cc
        bcc.w   .L0867d2                        | +0d2
        move.b  #0xff,0x21(a6)                  | +0d6
        jmp     0x518.l                         | +0dc
.L0867d2:
        rts                                     | +0e2

| ----------------------------------------------------------------------------
|  Fort_Destroyed_0867d4  @ $0867D4  (120 B)
| ----------------------------------------------------------------------------
        .section .text.Fort_Destroyed_0867d4, "ax", @progbits
        .global Fort_Destroyed_0867d4
Fort_Destroyed_0867d4:
        jsr     0x2783a.l                       | +000
        move.w  #0x1028,d0                      | +006
        jsr     0x2352.l                        | +00a
        lea     0x2ecf68.l,a1                   | +010
        jsr     0x77c7e.l                       | +016
        lea     0x2ecf7a.l,a1                   | +01c
        jsr     0x77c7e.l                       | +022
        lea     0x2ed04a.l,a1                   | +028
        jsr     0x77c7e.l                       | +02e
        jsr     Fort_BlitWreck_088a28(pc)       | +034
        movea.l 0x78(a6),a1                     | +038
        jsr     0x43fac.l                       | +03c
        jsr     0x434dc.l                       | +042
        lea     0x2ecc88.l,a0                   | +048
        move.l  a0,0x4c(a6)                     | +04e
        jsr     0x283ca.l                       | +052
        jsr     0x283ca.l                       | +058
        jsr     0x283d8.l                       | +05e
        movea.l 0x7c(a6),a1                     | +064
        move.w  #0x82,d0                        | +068
        move.b  #0x0,0x82(a6)                   | +06c
        jsr     0x4429e.l                       | +072

| ----------------------------------------------------------------------------
|  Fort_Idle_086854  @ $086854  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Fort_Idle_086854, "ax", @progbits
        .global Fort_Idle_086854
Fort_Idle_086854:
        lea     0xffff.w,a0                     | +000
        move.l  a0,0x4c(a6)                     | +004
        jsr     0x283ca.l                       | +008
        jmp     0x518.l                         | +00e

| ----------------------------------------------------------------------------
|  Rts_086868  @ $086868  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Rts_086868, "ax", @progbits
        .global Rts_086868
Rts_086868:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  Pillbox_Init_08686a  @ $08686A  (314 B)
| ----------------------------------------------------------------------------
        .section .text.Pillbox_Init_08686a, "ax", @progbits
        .global Pillbox_Init_08686a
Pillbox_Init_08686a:
        move.w  #0xe3,d1                        | +000
        jsr     0x236e.l                        | +004
        move.b  #0x0,0x21(a6)                   | +00a
        bra.w   .L0868e2                        | +010
        move.w  #0xe4,d1                        | +014
        jsr     0x236e.l                        | +018
        move.b  #0x30,0x21(a6)                  | +01e
        bra.w   .L0868e2                        | +024
        .global Pillbox_Init_Lower_086892
Pillbox_Init_Lower_086892:
        move.w  #0xe3,d1                        | +028
        jsr     0x236e.l                        | +02c
        move.b  #0x60,0x21(a6)                  | +032
        bra.w   .L0868e2                        | +038
        .global Pillbox_Init_Right_0868a6
Pillbox_Init_Right_0868a6:
        move.w  #0xe3,d1                        | +03c
        jsr     0x236e.l                        | +040
        move.b  #0x1,0x21(a6)                   | +046
        bra.w   .L0868e2                        | +04c
        .global Pillbox_Init_RightUpper_0868ba
Pillbox_Init_RightUpper_0868ba:
        move.w  #0xe4,d1                        | +050
        jsr     0x236e.l                        | +054
        move.b  #0x31,0x21(a6)                  | +05a
        bra.w   .L0868e2                        | +060
        move.w  #0xe3,d1                        | +064
        jsr     0x236e.l                        | +068
        move.b  #0x61,0x21(a6)                  | +06e
        bra.w   .L0868e2                        | +074
.L0868e2:
        move.b  #0xff,0x32(a6)                  | +078
        move.b  #0xff,0x33(a6)                  | +07e
        move.b  #0x0,0x3a(a6)                   | +084
        move.w  #0x28,d0                        | +08a
        move.w  d0,0x66(a6)                     | +08e
        move.w  d0,0x80(a6)                     | +092
        move.w  #0x8000,0x38(a6)                | +096
        jsr     0x267e2.l                       | +09c
        jsr     0x27cee.l                       | +0a2
        clr.l   d0                              | +0a8
        move.b  0x21(a6),d0                     | +0aa
        asr.l   #0x4,d0                         | +0ae
        movea.l #0x2eb2c0,a0                    | +0b0
        lsl.w   #0x2,d0                         | +0b6
        movea.l (a0,d0.w),a0                    | +0b8
        cmpa.l  #0xffffffff,a0                  | +0bc
        beq.w   .L086936                        | +0c2
        jsr     0x28cd4.l                       | +0c6
.L086936:
        move.w  0x72(a6),d0                     | +0cc
        move.b  d0,0x44(a6)                     | +0d0
        lea     .L086944(pc),a1                 | +0d4
        move.l  a1,(a6)                         | +0d8
.L086944:
        movea.l 0xc(a6),a0                      | +0da
        cmpi.b  #0xff,0x21(a0)                  | +0de
        beq.w   .L086990                        | +0e4
        jsr     0x2783a.l                       | +0e8
        jsr     0x28d70.l                       | +0ee
        jsr     0x2870a.l                       | +0f4
        bcc.w   .L08697c                        | +0fa
        move.w  #0x108d,d0                      | +0fe
        jsr     0x2352.l                        | +102
        bclr    #0x3,0x13(a6)                   | +108
        jsr     Entity_PropagateDamageToParent_08848c(pc) | +10e
.L08697c:
        cmpi.w  #0x1e,0x66(a6)                  | +112
        bgt.w   .L0869a2                        | +118
        lea     Pillbox_Damaged1_0869a4(pc),a1  | +11c
        move.l  a1,(a6)                         | +120
        bra.w   .L0869a2                        | +122
.L086990:
        lea     0x2edd48.l,a1                   | +126
        jsr     0x43fac.l                       | +12c
        jmp     0x518.l                         | +132
.L0869a2:
        rts                                     | +138

| ----------------------------------------------------------------------------
|  Pillbox_Damaged1_0869a4  @ $0869A4  (140 B)
| ----------------------------------------------------------------------------
        .section .text.Pillbox_Damaged1_0869a4, "ax", @progbits
        .global Pillbox_Damaged1_0869a4
Pillbox_Damaged1_0869a4:
        clr.l   d0                              | +000
        move.b  0x21(a6),d0                     | +002
        asr.l   #0x4,d0                         | +006
        addq.l  #0x1,d0                         | +008
        movea.l #0x2eb2c0,a0                    | +00a
        lsl.w   #0x2,d0                         | +010
        movea.l (a0,d0.w),a0                    | +012
        cmpa.l  #0xffffffff,a0                  | +016
        beq.w   .L0869ca                        | +01c
        jsr     0x28cd4.l                       | +020
.L0869ca:
        lea     .L0869d0(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L0869d0:
        movea.l 0xc(a6),a0                      | +02c
        cmpi.b  #0xff,0x21(a0)                  | +030
        beq.w   .L086a1c                        | +036
        jsr     0x2783a.l                       | +03a
        jsr     0x28d70.l                       | +040
        jsr     0x2870a.l                       | +046
        bcc.w   .L086a08                        | +04c
        move.w  #0x108d,d0                      | +050
        jsr     0x2352.l                        | +054
        bclr    #0x3,0x13(a6)                   | +05a
        jsr     Entity_PropagateDamageToParent_08848c(pc) | +060
.L086a08:
        cmpi.w  #0x1a,0x66(a6)                  | +064
        bgt.w   .L086a2e                        | +06a
        lea     Pillbox_Damaged2_086a30(pc),a1  | +06e
        move.l  a1,(a6)                         | +072
        bra.w   .L086a2e                        | +074
.L086a1c:
        lea     0x2edd48.l,a1                   | +078
        jsr     0x43fac.l                       | +07e
        jmp     0x518.l                         | +084
.L086a2e:
        rts                                     | +08a

| ----------------------------------------------------------------------------
|  Pillbox_Damaged2_086a30  @ $086A30  (140 B)
| ----------------------------------------------------------------------------
        .section .text.Pillbox_Damaged2_086a30, "ax", @progbits
        .global Pillbox_Damaged2_086a30
Pillbox_Damaged2_086a30:
        clr.l   d0                              | +000
        move.b  0x21(a6),d0                     | +002
        asr.l   #0x4,d0                         | +006
        addq.l  #0x2,d0                         | +008
        movea.l #0x2eb2c0,a0                    | +00a
        lsl.w   #0x2,d0                         | +010
        movea.l (a0,d0.w),a0                    | +012
        cmpa.l  #0xffffffff,a0                  | +016
        beq.w   .L086a56                        | +01c
        jsr     0x28cd4.l                       | +020
.L086a56:
        lea     .L086a5c(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L086a5c:
        movea.l 0xc(a6),a0                      | +02c
        cmpi.b  #0xff,0x21(a0)                  | +030
        beq.w   .L086aa8                        | +036
        jsr     0x2783a.l                       | +03a
        jsr     0x28d70.l                       | +040
        jsr     0x2870a.l                       | +046
        bcc.w   .L086a94                        | +04c
        move.w  #0x108d,d0                      | +050
        jsr     0x2352.l                        | +054
        bclr    #0x3,0x13(a6)                   | +05a
        jsr     Entity_PropagateDamageToParent_08848c(pc) | +060
.L086a94:
        cmpi.w  #0xd,0x66(a6)                   | +064
        bgt.w   .L086aba                        | +06a
        lea     Pillbox_Critical_086abc(pc),a1  | +06e
        move.l  a1,(a6)                         | +072
        bra.w   .L086aba                        | +074
.L086aa8:
        lea     0x2edd48.l,a1                   | +078
        jsr     0x43fac.l                       | +07e
        jmp     0x518.l                         | +084
.L086aba:
        rts                                     | +08a

| ----------------------------------------------------------------------------
|  Pillbox_Critical_086abc  @ $086ABC  (232 B)
| ----------------------------------------------------------------------------
        .section .text.Pillbox_Critical_086abc, "ax", @progbits
        .global Pillbox_Critical_086abc
Pillbox_Critical_086abc:
        lea     .L086ac2(pc),a1                 | +000
        move.l  a1,(a6)                         | +004
.L086ac2:
        movea.l 0xc(a6),a0                      | +006
        cmpi.b  #0xff,0x21(a0)                  | +00a
        beq.w   .L086b90                        | +010
        jsr     0x2783a.l                       | +014
        jsr     0x28d70.l                       | +01a
        jsr     0x2870a.l                       | +020
        bcc.w   .L086afa                        | +026
        move.w  #0x108d,d0                      | +02a
        jsr     0x2352.l                        | +02e
        bclr    #0x3,0x13(a6)                   | +034
        jsr     Entity_PropagateDamageToParent_08848c(pc) | +03a
.L086afa:
        jsr     0x28758.l                       | +03e
        bcc.w   .L086ba2                        | +044
        bclr    #0x0,0x13(a6)                   | +048
        lea     0xffff.w,a0                     | +04e
        move.l  a0,0x48(a6)                     | +052
        btst    #0x0,0x21(a6)                   | +056
        beq.w   .L086b26                        | +05c
        lea     Pillbox_Collapse_086ba4(pc),a1  | +060
        move.l  a1,(a6)                         | +064
        bra.w   .L086ba2                        | +066
.L086b26:
        move.l  #0x100,d0                       | +06a
        jsr     0x51a28.l                       | +070
        move.w  #0x102f,d0                      | +076
        jsr     0x2352.l                        | +07a
        lea     0x78066.l,a1                    | +080
        jsr     0x4ae.l                         | +086
        jsr     0x5dd22.l                       | +08c
        addi.w  #0x10,0x24(a0)                  | +092
        movea.l 0x74(a6),a2                     | +098
        jsr     0x5022a.l                       | +09c
        lea     0x2ed05c.l,a1                   | +0a2
        jsr     0x77c7e.l                       | +0a8
        addi.w  #0x10,0x24(a0)                  | +0ae
        movea.l 0x7c(a6),a1                     | +0b4
        move.l  a1,d0                           | +0b8
        cmpi.l  #0xffffffff,d0                  | +0ba
        beq.w   .L086b90                        | +0c0
        move.w  #0x82,d0                        | +0c4
        move.b  #0x0,0x82(a6)                   | +0c8
        jsr     0x4429e.l                       | +0ce
.L086b90:
        lea     0x2edd48.l,a1                   | +0d4
        jsr     0x43fac.l                       | +0da
        jmp     0x518.l                         | +0e0
.L086ba2:
        rts                                     | +0e6

| ----------------------------------------------------------------------------
|  Pillbox_Collapse_086ba4  @ $086BA4  (190 B)
| ----------------------------------------------------------------------------
        .section .text.Pillbox_Collapse_086ba4, "ax", @progbits
        .global Pillbox_Collapse_086ba4
Pillbox_Collapse_086ba4:
        lea     Pillbox_Fragment_086c62(pc),a1  | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd22.l                       | +00a
        lea     .L086bba(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L086bba:
        movea.l 0xc(a6),a0                      | +016
        cmpi.b  #0xff,0x21(a0)                  | +01a
        beq.w   .L086c4e                        | +020
        jsr     0x2783a.l                       | +024
        jsr     0x28d70.l                       | +02a
        move.b  0x21(a6),d0                     | +030
        andi.b  #0xf,d0                         | +034
        cmpi.b  #0xf,d0                         | +038
        bne.w   .L086c60                        | +03c
        move.l  #0x100,d0                       | +040
        jsr     0x51a28.l                       | +046
        move.w  #0x102c,d0                      | +04c
        jsr     0x2352.l                        | +050
        lea     0x78066.l,a1                    | +056
        jsr     0x4ae.l                         | +05c
        jsr     0x5dd22.l                       | +062
        addi.w  #0x10,0x24(a0)                  | +068
        movea.l 0x74(a6),a2                     | +06e
        jsr     0x5022a.l                       | +072
        lea     0x2ed05c.l,a1                   | +078
        jsr     0x77c7e.l                       | +07e
        addi.w  #0x10,0x24(a0)                  | +084
        movea.l 0x7c(a6),a1                     | +08a
        move.l  a1,d0                           | +08e
        cmpi.l  #0xffffffff,d0                  | +090
        beq.w   .L086c4e                        | +096
        move.w  #0x82,d0                        | +09a
        move.b  #0x0,0x82(a6)                   | +09e
        jsr     0x4429e.l                       | +0a4
.L086c4e:
        lea     0x2edd48.l,a1                   | +0aa
        jsr     0x43fac.l                       | +0b0
        jmp     0x518.l                         | +0b6
.L086c60:
        rts                                     | +0bc

| ----------------------------------------------------------------------------
|  Pillbox_Fragment_086c62  @ $086C62  (110 B)
| ----------------------------------------------------------------------------
        .section .text.Pillbox_Fragment_086c62, "ax", @progbits
        .global Pillbox_Fragment_086c62
Pillbox_Fragment_086c62:
        move.w  #0x6c,d1                        | +000
        jsr     0x236e.l                        | +004
        move.b  #0xff,0x32(a6)                  | +00a
        move.b  #0xff,0x33(a6)                  | +010
        move.b  #0x0,0x3a(a6)                   | +016
        move.w  #0x0,0x38(a6)                   | +01c
        addi.w  #0x8,0x24(a6)                   | +022
        lea     0x2eb2e4.l,a0                   | +028
        jsr     0x28cd4.l                       | +02e
        lea     .L086c9c(pc),a1                 | +034
        move.l  a1,(a6)                         | +038
.L086c9c:
        movea.l 0xc(a6),a0                      | +03a
        movea.l 0xc(a0),a0                      | +03e
        cmpi.b  #0xff,0x21(a0)                  | +042
        beq.w   .L086cc8                        | +048
        jsr     0x2783a.l                       | +04c
        jsr     0x28d70.l                       | +052
        bcc.w   .L086cce                        | +058
        movea.l 0xc(a6),a0                      | +05c
        ori.b   #0xf,0x21(a0)                   | +060
.L086cc8:
        jmp     0x518.l                         | +066
.L086cce:
        rts                                     | +06c

| ----------------------------------------------------------------------------
|  Crate_Init_086cd0  @ $086CD0  (222 B)
| ----------------------------------------------------------------------------
        .section .text.Crate_Init_086cd0, "ax", @progbits
        .global Crate_Init_086cd0
Crate_Init_086cd0:
        move.w  #0xe7,d1                        | +000
        jsr     0x236e.l                        | +004
        move.b  #0x0,0x21(a6)                   | +00a
        bra.w   .L086d34                        | +010
        .global Crate_Init_V1_086ce4
Crate_Init_V1_086ce4:
        move.w  #0xe8,d1                        | +014
        jsr     0x236e.l                        | +018
        move.b  #0x1,0x21(a6)                   | +01e
        bra.w   .L086d34                        | +024
        .global Crate_Init_V2_086cf8
Crate_Init_V2_086cf8:
        move.w  #0xe9,d1                        | +028
        jsr     0x236e.l                        | +02c
        move.b  #0x2,0x21(a6)                   | +032
        bra.w   .L086d34                        | +038
        .global Crate_Init_V3_086d0c
Crate_Init_V3_086d0c:
        move.w  #0xea,d1                        | +03c
        jsr     0x236e.l                        | +040
        move.b  #0x3,0x21(a6)                   | +046
        bra.w   .L086d34                        | +04c
        .global Crate_Init_V4_086d20
Crate_Init_V4_086d20:
        move.w  #0xeb,d1                        | +050
        jsr     0x236e.l                        | +054
        move.b  #0x4,0x21(a6)                   | +05a
        bra.w   .L086d34                        | +060
.L086d34:
        move.b  #0xff,0x32(a6)                  | +064
        move.b  #0xff,0x33(a6)                  | +06a
        move.b  #0x0,0x3a(a6)                   | +070
        move.w  #0x14,0x66(a6)                  | +076
        move.w  #0x0,0x38(a6)                   | +07c
        lea     .L086d58(pc),a1                 | +082
        move.l  a1,(a6)                         | +086
.L086d58:
        movea.l 0xc(a6),a0                      | +088
        cmpi.b  #0xff,0x21(a0)                  | +08c
        beq.w   .L086da8                        | +092
        jsr     0x2783a.l                       | +096
        jsr     0x2870a.l                       | +09c
        bcc.w   .L086d8c                        | +0a2
        lea     0x5e766.l,a0                    | +0a6
        jsr     0x5e770.l                       | +0ac
        bclr    #0x3,0x13(a6)                   | +0b2
        jsr     Entity_PropagateDamageToParent_08848c(pc) | +0b8
.L086d8c:
        jsr     0x28758.l                       | +0bc
        bcc.w   JsrPcThunk_086dae               | +0c2
        lea     0xffff.w,a0                     | +0c6
        move.l  a0,0x48(a6)                     | +0ca
        lea     Crate_Burst_086db4(pc),a1       | +0ce
        move.l  a1,(a6)                         | +0d2
        bra.w   JsrPcThunk_086dae               | +0d4
.L086da8:
        jmp     0x518.l                         | +0d8

| ----------------------------------------------------------------------------
|  Crate_Burst_086db4  @ $086DB4  (150 B)
| ----------------------------------------------------------------------------
        .section .text.Crate_Burst_086db4, "ax", @progbits
        .global Crate_Burst_086db4
Crate_Burst_086db4:
        move.l  #0x100,d0                       | +000
        jsr     0x51a28.l                       | +006
        move.w  #0x1039,d0                      | +00c
        jsr     0x2352.l                        | +010
        clr.l   d0                              | +016
        move.b  0x21(a6),d0                     | +018
        movea.l #0x2eb6ba,a0                    | +01c
        lsl.w   #0x2,d0                         | +022
        movea.l (a0,d0.w),a0                    | +024
        cmpa.l  #0xffffffff,a0                  | +028
        beq.w   .L086dec                        | +02e
        jsr     0x28cd4.l                       | +032
.L086dec:
        lea     .L086df2(pc),a1                 | +038
        move.l  a1,(a6)                         | +03c
.L086df2:
        movea.l 0xc(a6),a0                      | +03e
        cmpi.b  #0xff,0x21(a0)                  | +042
        beq.w   .L086e42                        | +048
        jsr     0x2783a.l                       | +04c
        jsr     0x28d70.l                       | +052
        bcc.w   .L086e48                        | +058
        move.w  #0x102f,d0                      | +05c
        jsr     0x2352.l                        | +060
        lea     0x7808a.l,a1                    | +066
        jsr     0x4ae.l                         | +06c
        jsr     0x5dd22.l                       | +072
        movea.l 0x74(a6),a2                     | +078
        jsr     0x5022a.l                       | +07c
        lea     0x2ed05c.l,a1                   | +082
        jsr     0x77c7e.l                       | +088
.L086e42:
        jmp     0x518.l                         | +08e
.L086e48:
        rts                                     | +094

| ----------------------------------------------------------------------------
|  Hatch_Init_086e4a  @ $086E4A  (282 B)
| ----------------------------------------------------------------------------
        .section .text.Hatch_Init_086e4a, "ax", @progbits
        .global Hatch_Init_086e4a
Hatch_Init_086e4a:
        move.w  #0x30,0x70(a6)                  | +000
        move.b  #0x0,0x20(a6)                   | +006
        move.b  #0x1,0x21(a6)                   | +00c
        lea     0xede9e.l,a1                    | +012
        move.l  a1,0x7c(a6)                     | +018
        lea     0xedcb2.l,a1                    | +01c
        move.l  a1,0x8c(a6)                     | +022
        bra.w   Hatch_WaitClosed_086f64         | +026
        .global Hatch_Init_V1_086e74
Hatch_Init_V1_086e74:
        move.w  #0x30,0x70(a6)                  | +02a
        move.b  #0x1,0x20(a6)                   | +030
        move.b  #0x1,0x21(a6)                   | +036
        lea     0xededa.l,a1                    | +03c
        move.l  a1,0x7c(a6)                     | +042
        lea     0xedd0a.l,a1                    | +046
        move.l  a1,0x8c(a6)                     | +04c
        bra.w   Hatch_WaitClosed_086f64         | +050
        .global Hatch_Init_V2_086e9e
Hatch_Init_V2_086e9e:
        move.w  #0x30,0x70(a6)                  | +054
        move.b  #0x2,0x20(a6)                   | +05a
        move.b  #0x1,0x21(a6)                   | +060
        lea     0xedefa.l,a1                    | +066
        move.l  a1,0x7c(a6)                     | +06c
        lea     0xedd86.l,a1                    | +070
        move.l  a1,0x8c(a6)                     | +076
        bra.w   Hatch_WaitClosed_086f64         | +07a
        .global Hatch_Init_V3_086ec8
Hatch_Init_V3_086ec8:
        move.w  #0x20,0x70(a6)                  | +07e
        move.b  #0x3,0x20(a6)                   | +084
        move.b  #0x1,0x21(a6)                   | +08a
        lea     0xedf1a.l,a1                    | +090
        move.l  a1,0x7c(a6)                     | +096
        lea     0xede02.l,a1                    | +09a
        move.l  a1,0x8c(a6)                     | +0a0
        bra.w   Hatch_WaitClosed_086f64         | +0a4
        .global Hatch_Init_V4_086ef2
Hatch_Init_V4_086ef2:
        move.w  #0x20,0x70(a6)                  | +0a8
        move.b  #0x4,0x20(a6)                   | +0ae
        move.b  #0x1,0x21(a6)                   | +0b4
        lea     0xedf52.l,a1                    | +0ba
        move.l  a1,0x7c(a6)                     | +0c0
        lea     0xede7e.l,a1                    | +0c4
        move.l  a1,0x8c(a6)                     | +0ca
        bra.w   Hatch_WaitClosed_086f64         | +0ce
        movea.l 0x3c(a6),a1                     | +0d2
        jsr     0x2942a.l                       | +0d6
        move.w  #0x30,0x70(a6)                  | +0dc
        move.b  #0x5,0x20(a6)                   | +0e2
        lea     0xedf72.l,a1                    | +0e8
        move.l  a1,0x7c(a6)                     | +0ee
        bra.w   Hatch_WaitClosed_086f64         | +0f2
        movea.l 0x3c(a6),a1                     | +0f6
        jsr     0x2942a.l                       | +0fa
        move.w  #0x30,0x70(a6)                  | +100
        move.b  #0x6,0x20(a6)                   | +106
        lea     0xedfaa.l,a1                    | +10c
        move.l  a1,0x7c(a6)                     | +112
        bra.w   Hatch_WaitClosed_086f64         | +116

| ----------------------------------------------------------------------------
|  Hatch_WaitClosed_086f64  @ $086F64  (80 B)
| ----------------------------------------------------------------------------
        .section .text.Hatch_WaitClosed_086f64, "ax", @progbits
        .global Hatch_WaitClosed_086f64
Hatch_WaitClosed_086f64:
        move.w  #0x5a,0x72(a6)                  | +000
        lea     .L086f70(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L086f70:
        jsr     0x2783a.l                       | +00c
        subq.w  #0x1,0x72(a6)                   | +012
        bne.w   .L086f84                        | +016
        lea     Hatch_Opening_086fb4(pc),a1     | +01a
        move.l  a1,(a6)                         | +01e
.L086f84:
        cmpi.b  #0x1,0x21(a6)                   | +020
        bne.w   .L086fa2                        | +026
        movea.l 0xc(a6),a0                      | +02a
        cmpi.b  #0xff,0x21(a0)                  | +02e
        bne.w   .L086fa2                        | +034
        lea     Hatch_FinalOpen_087108(pc),a1   | +038
        move.l  a1,(a6)                         | +03c
.L086fa2:
        jsr     0x4fa70.l                       | +03e
        bcc.w   .L086fb2                        | +044
        jmp     0x518.l                         | +048
.L086fb2:
        rts                                     | +04e

| ----------------------------------------------------------------------------
|  Hatch_Opening_086fb4  @ $086FB4  (80 B)
| ----------------------------------------------------------------------------
        .section .text.Hatch_Opening_086fb4, "ax", @progbits
        .global Hatch_Opening_086fb4
Hatch_Opening_086fb4:
        clr.l   d0                              | +000
        move.b  0x20(a6),d0                     | +002
        movea.l #0x2ebfae,a0                    | +006
        lsl.w   #0x2,d0                         | +00c
        movea.l (a0,d0.w),a0                    | +00e
        cmpa.l  #0xffffffff,a0                  | +012
        beq.w   .L086fd6                        | +018
        jsr     0x28cd4.l                       | +01c
.L086fd6:
        lea     .L086fdc(pc),a1                 | +022
        move.l  a1,(a6)                         | +026
.L086fdc:
        jsr     0x2783a.l                       | +028
        jsr     0x28d70.l                       | +02e
        bcc.w   .L086ff2                        | +034
        lea     Hatch_SpawnTroops_087004(pc),a1 | +038
        move.l  a1,(a6)                         | +03c
.L086ff2:
        jsr     0x4fa70.l                       | +03e
        bcc.w   .L087002                        | +044
        jmp     0x518.l                         | +048
.L087002:
        rts                                     | +04e

| ----------------------------------------------------------------------------
|  Hatch_SpawnTroops_087004  @ $087004  (100 B)
| ----------------------------------------------------------------------------
        .section .text.Hatch_SpawnTroops_087004, "ax", @progbits
        .global Hatch_SpawnTroops_087004
Hatch_SpawnTroops_087004:
        move.w  #0x28,0x72(a6)                  | +000
        movea.l 0x7c(a6),a1                     | +006
        move.w  #0x82,d0                        | +00a
        move.b  #0x0,0x82(a6)                   | +00e
        jsr     0x4429e.l                       | +014
        lea     .L087024(pc),a1                 | +01a
        move.l  a1,(a6)                         | +01e
.L087024:
        jsr     0x2783a.l                       | +020
        jsr     Entity_CmpPrioWithSibling_086552(pc) | +026
        bcs.w   .L087038                        | +02a
        lea     Hatch_WaitOpen_087068(pc),a1    | +02e
        move.l  a1,(a6)                         | +032
.L087038:
        cmpi.b  #0x1,0x21(a6)                   | +034
        bne.w   .L087056                        | +03a
        movea.l 0xc(a6),a0                      | +03e
        cmpi.b  #0xff,0x21(a0)                  | +042
        bne.w   .L087056                        | +048
        lea     Hatch_FinalSpawn_087130(pc),a1  | +04c
        move.l  a1,(a6)                         | +050
.L087056:
        jsr     0x4fa70.l                       | +052
        bcc.w   .L087066                        | +058
        jmp     0x518.l                         | +05c
.L087066:
        rts                                     | +062

| ----------------------------------------------------------------------------
|  Hatch_WaitOpen_087068  @ $087068  (80 B)
| ----------------------------------------------------------------------------
        .section .text.Hatch_WaitOpen_087068, "ax", @progbits
        .global Hatch_WaitOpen_087068
Hatch_WaitOpen_087068:
        move.w  #0x50,0x72(a6)                  | +000
        lea     .L087074(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L087074:
        jsr     0x2783a.l                       | +00c
        subq.w  #0x1,0x72(a6)                   | +012
        bne.w   .L087088                        | +016
        lea     Hatch_Closing_0870b8(pc),a1     | +01a
        move.l  a1,(a6)                         | +01e
.L087088:
        cmpi.b  #0x1,0x21(a6)                   | +020
        bne.w   .L0870a6                        | +026
        movea.l 0xc(a6),a0                      | +02a
        cmpi.b  #0xff,0x21(a0)                  | +02e
        bne.w   .L0870a6                        | +034
        lea     Hatch_FinalSpawn_087130(pc),a1  | +038
        move.l  a1,(a6)                         | +03c
.L0870a6:
        jsr     0x4fa70.l                       | +03e
        bcc.w   .L0870b6                        | +044
        jmp     0x518.l                         | +048
.L0870b6:
        rts                                     | +04e

| ----------------------------------------------------------------------------
|  Hatch_Closing_0870b8  @ $0870B8  (80 B)
| ----------------------------------------------------------------------------
        .section .text.Hatch_Closing_0870b8, "ax", @progbits
        .global Hatch_Closing_0870b8
Hatch_Closing_0870b8:
        clr.l   d0                              | +000
        move.b  0x20(a6),d0                     | +002
        movea.l #0x2ebfca,a0                    | +006
        lsl.w   #0x2,d0                         | +00c
        movea.l (a0,d0.w),a0                    | +00e
        cmpa.l  #0xffffffff,a0                  | +012
        beq.w   .L0870da                        | +018
        jsr     0x28cd4.l                       | +01c
.L0870da:
        lea     .L0870e0(pc),a1                 | +022
        move.l  a1,(a6)                         | +026
.L0870e0:
        jsr     0x2783a.l                       | +028
        jsr     0x28d70.l                       | +02e
        bcc.w   .L0870f6                        | +034
        lea     Hatch_WaitClosed_086f64(pc),a1  | +038
        move.l  a1,(a6)                         | +03c
.L0870f6:
        jsr     0x4fa70.l                       | +03e
        bcc.w   .L087106                        | +044
        jmp     0x518.l                         | +048
.L087106:
        rts                                     | +04e

| ----------------------------------------------------------------------------
|  Hatch_FinalOpen_087108  @ $087108  (40 B)
| ----------------------------------------------------------------------------
        .section .text.Hatch_FinalOpen_087108, "ax", @progbits
        .global Hatch_FinalOpen_087108
Hatch_FinalOpen_087108:
        clr.l   d0                              | +000
        move.b  0x20(a6),d0                     | +002
        movea.l #0x2ebfae,a0                    | +006
        lsl.w   #0x2,d0                         | +00c
        movea.l (a0,d0.w),a0                    | +00e
        cmpa.l  #0xffffffff,a0                  | +012
        beq.w   .L08712a                        | +018
        jsr     0x28cd4.l                       | +01c
.L08712a:
        lea     Hatch_FinalSpawn_087130(pc),a1  | +022
        move.l  a1,(a6)                         | +026

| ----------------------------------------------------------------------------
|  Hatch_FinalSpawn_087130  @ $087130  (58 B)
| ----------------------------------------------------------------------------
        .section .text.Hatch_FinalSpawn_087130, "ax", @progbits
        .global Hatch_FinalSpawn_087130
Hatch_FinalSpawn_087130:
        jsr     0x2783a.l                       | +000
        jsr     0x28d70.l                       | +006
        bcc.w   .L087158                        | +00c
        movea.l 0x8c(a6),a1                     | +010
        move.w  #0x82,d0                        | +014
        move.b  #0x0,0x82(a6)                   | +018
        jsr     0x4429e.l                       | +01e
        bra.w   .L087162                        | +024
.L087158:
        jsr     0x4fa70.l                       | +028
        bcc.w   .L087168                        | +02e
.L087162:
        jmp     0x518.l                         | +032
.L087168:
        rts                                     | +038

| ----------------------------------------------------------------------------
|  Prop_Breakable_A_08716a  @ $08716A  (144 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_Breakable_A_08716a, "ax", @progbits
        .global Prop_Breakable_A_08716a
Prop_Breakable_A_08716a:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x20,0x70(a6)                  | +00a
        lea     0x2ec726.l,a0                   | +010
        move.l  a0,0x48(a6)                     | +016
        lea     .L08718a(pc),a1                 | +01a
        move.l  a1,(a6)                         | +01e
.L08718a:
        jsr     0x2783a.l                       | +020
        jsr     0x2870a.l                       | +026
        bcc.w   .L0871ac                        | +02c
        lea     0x5e766.l,a0                    | +030
        jsr     0x5e770.l                       | +036
        bclr    #0x3,0x13(a6)                   | +03c
.L0871ac:
        jsr     0x28758.l                       | +042
        bcc.w   .L0871e8                        | +048
        move.w  #0x10a6,d0                      | +04c
        jsr     0x2352.l                        | +050
        lea     0x2ed0ae.l,a1                   | +056
        jsr     0x77c7e.l                       | +05c
        lea     0x2ed878.l,a2                   | +062
        jsr     0x5022a.l                       | +068
        lea     0x2ed864.l,a2                   | +06e
        jsr     0x5022a.l                       | +074
        bra.w   .L0871f2                        | +07a
.L0871e8:
        jsr     0x4fa70.l                       | +07e
        bcc.w   .L0871f8                        | +084
.L0871f2:
        jmp     0x518.l                         | +088
.L0871f8:
        rts                                     | +08e

| ----------------------------------------------------------------------------
|  Prop_Breakable_B_0871fa  @ $0871FA  (134 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_Breakable_B_0871fa, "ax", @progbits
        .global Prop_Breakable_B_0871fa
Prop_Breakable_B_0871fa:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x20,0x70(a6)                  | +00a
        lea     0x2ec726.l,a0                   | +010
        move.l  a0,0x48(a6)                     | +016
        lea     .L08721a(pc),a1                 | +01a
        move.l  a1,(a6)                         | +01e
.L08721a:
        jsr     0x2783a.l                       | +020
        jsr     0x2870a.l                       | +026
        bcc.w   .L08723c                        | +02c
        lea     0x5e766.l,a0                    | +030
        jsr     0x5e770.l                       | +036
        bclr    #0x3,0x13(a6)                   | +03c
.L08723c:
        jsr     0x28758.l                       | +042
        bcc.w   .L08726e                        | +048
        lea     0x2ed0ae.l,a1                   | +04c
        jsr     0x77c7e.l                       | +052
        lea     0x2ed968.l,a2                   | +058
        jsr     0x5022a.l                       | +05e
        lea     0x2ed97c.l,a2                   | +064
        jsr     0x5022a.l                       | +06a
        bra.w   .L087278                        | +070
.L08726e:
        jsr     0x4fa70.l                       | +074
        bcc.w   .L08727e                        | +07a
.L087278:
        jmp     0x518.l                         | +07e
.L08727e:
        rts                                     | +084

| ----------------------------------------------------------------------------
|  Prop_Breakable_C_087280  @ $087280  (122 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_Breakable_C_087280, "ax", @progbits
        .global Prop_Breakable_C_087280
Prop_Breakable_C_087280:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x20,0x70(a6)                  | +00a
        lea     0x2ec726.l,a0                   | +010
        move.l  a0,0x48(a6)                     | +016
        lea     .L0872a0(pc),a1                 | +01a
        move.l  a1,(a6)                         | +01e
.L0872a0:
        jsr     0x2783a.l                       | +020
        jsr     0x2870a.l                       | +026
        bcc.w   .L0872c2                        | +02c
        lea     0x5e766.l,a0                    | +030
        jsr     0x5e770.l                       | +036
        bclr    #0x3,0x13(a6)                   | +03c
.L0872c2:
        jsr     0x28758.l                       | +042
        bcc.w   .L0872e8                        | +048
        lea     0x2ed0ae.l,a1                   | +04c
        jsr     0x77c7e.l                       | +052
        lea     0x2eda6c.l,a2                   | +058
        jsr     0x5022a.l                       | +05e
        bra.w   .L0872f2                        | +064
.L0872e8:
        jsr     0x4fa70.l                       | +068
        bcc.w   .L0872f8                        | +06e
.L0872f2:
        jmp     0x518.l                         | +072
.L0872f8:
        rts                                     | +078

| ----------------------------------------------------------------------------
|  Prop_Signboard_0872fa  @ $0872FA  (108 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_Signboard_0872fa, "ax", @progbits
        .global Prop_Signboard_0872fa
Prop_Signboard_0872fa:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x20,0x70(a6)                  | +00a
        lea     0x2ebfe6.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        lea     .L08731c(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L08731c:
        jsr     0x2783a.l                       | +022
        jsr     0x28d70.l                       | +028
        jsr     0x2870a.l                       | +02e
        bcc.w   .L08734e                        | +034
        move.w  #0x10a9,d0                      | +038
        jsr     0x2352.l                        | +03c
        lea     0x2ebffc.l,a0                   | +042
        jsr     0x28cd4.l                       | +048
        bclr    #0x3,0x13(a6)                   | +04e
.L08734e:
        move.w  #0x7fff,0x66(a6)                | +054
        jsr     0x4fa70.l                       | +05a
        bcc.w   .L087364                        | +060
        jmp     0x518.l                         | +064
.L087364:
        rts                                     | +06a

| ----------------------------------------------------------------------------
|  CrateWall_Spawn13_087366  @ $087366  (338 B)
| ----------------------------------------------------------------------------
        .section .text.CrateWall_Spawn13_087366, "ax", @progbits
        .global CrateWall_Spawn13_087366
CrateWall_Spawn13_087366:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        lea     CrateWall_Piece00_0874ba(pc),a1 | +00a
        jsr     0x4ae.l                         | +00e
        jsr     0x5dd22.l                       | +014
        subi.w  #0x10,0x22(a0)                  | +01a
        addi.w  #0x40,0x24(a0)                  | +020
        lea     CrateWall_Piece01_0874ea(pc),a1 | +026
        jsr     0x4ae.l                         | +02a
        jsr     0x5dd22.l                       | +030
        addi.w  #0x10,0x22(a0)                  | +036
        addi.w  #0xa0,0x24(a0)                  | +03c
        lea     CrateWall_Piece02_08751c(pc),a1 | +042
        jsr     0x4ae.l                         | +046
        jsr     0x5dd22.l                       | +04c
        addi.w  #0x50,0x22(a0)                  | +052
        addi.w  #0xa0,0x24(a0)                  | +058
        lea     CrateWall_Piece03_08754e(pc),a1 | +05e
        jsr     0x4ae.l                         | +062
        jsr     0x5dd22.l                       | +068
        addi.w  #0x80,0x22(a0)                  | +06e
        addi.w  #0xa0,0x24(a0)                  | +074
        lea     CrateWall_Piece04_087580(pc),a1 | +07a
        jsr     0x4ae.l                         | +07e
        jsr     0x5dd22.l                       | +084
        addi.w  #0x10,0x22(a0)                  | +08a
        addi.w  #0x80,0x24(a0)                  | +090
        lea     CrateWall_Piece05_0875b0(pc),a1 | +096
        jsr     0x4ae.l                         | +09a
        jsr     0x5dd22.l                       | +0a0
        addi.w  #0x60,0x22(a0)                  | +0a6
        addi.w  #0x80,0x24(a0)                  | +0ac
        lea     CrateWall_Piece06_0875e0(pc),a1 | +0b2
        jsr     0x4ae.l                         | +0b6
        jsr     0x5dd22.l                       | +0bc
        addi.w  #0x50,0x22(a0)                  | +0c2
        addi.w  #0x50,0x24(a0)                  | +0c8
        lea     CrateWall_Piece07_087610(pc),a1 | +0ce
        jsr     0x4ae.l                         | +0d2
        jsr     0x5dd22.l                       | +0d8
        lea     CrateWall_Piece08_087640(pc),a1 | +0de
        jsr     0x4ae.l                         | +0e2
        jsr     0x5dd22.l                       | +0e8
        addi.w  #0x20,0x22(a0)                  | +0ee
        lea     CrateWall_Piece09_087670(pc),a1 | +0f4
        jsr     0x4ae.l                         | +0f8
        jsr     0x5dd22.l                       | +0fe
        addi.w  #0x40,0x22(a0)                  | +104
        lea     CrateWall_Piece10_0876a0(pc),a1 | +10a
        jsr     0x4ae.l                         | +10e
        jsr     0x5dd22.l                       | +114
        addi.w  #0x60,0x22(a0)                  | +11a
        lea     CrateWall_Piece11_0876d0(pc),a1 | +120
        jsr     0x4ae.l                         | +124
        jsr     0x5dd22.l                       | +12a
        addi.w  #0x80,0x22(a0)                  | +130
        lea     CrateWall_Piece12_087700(pc),a1 | +136
        jsr     0x4ae.l                         | +13a
        jsr     0x5dd22.l                       | +140
        addi.w  #0xa0,0x22(a0)                  | +146
        jmp     0x518.l                         | +14c

| ----------------------------------------------------------------------------
|  Rts_0874b8  @ $0874B8  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Rts_0874b8, "ax", @progbits
        .global Rts_0874b8
Rts_0874b8:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  CrateWall_Piece00_0874ba  @ $0874BA  (48 B)
| ----------------------------------------------------------------------------
        .section .text.CrateWall_Piece00_0874ba, "ax", @progbits
        .global CrateWall_Piece00_0874ba
CrateWall_Piece00_0874ba:
        lea     0x2ed990.l,a1                   | +000
        move.l  a1,0x74(a6)                     | +006
        move.l  #0xffffffff,0x7c(a6)            | +00a
        move.w  #0x0,0x72(a6)                   | +012
        lea     0x2ec822.l,a0                   | +018
        move.l  a0,0x48(a6)                     | +01e
        lea     0x2ecfde.l,a1                   | +022
        move.l  a1,0x88(a6)                     | +028
        bra.w   CrateWall_Piece_Common_087730 | +02c

| ----------------------------------------------------------------------------
|  CrateWall_Piece01_0874ea  @ $0874EA  (50 B)
| ----------------------------------------------------------------------------
        .section .text.CrateWall_Piece01_0874ea, "ax", @progbits
        .global CrateWall_Piece01_0874ea
CrateWall_Piece01_0874ea:
        lea     0x2ed9a4.l,a1                   | +000
        move.l  a1,0x74(a6)                     | +006
        lea     0xedfe2.l,a1                    | +00a
        move.l  a1,0x7c(a6)                     | +010
        move.w  #0x1,0x72(a6)                   | +014
        lea     0x2ec876.l,a0                   | +01a
        move.l  a0,0x48(a6)                     | +020
        lea     0x2ecff0.l,a1                   | +024
        move.l  a1,0x88(a6)                     | +02a
        bra.w   CrateWall_Piece_Common_087730 | +02e

| ----------------------------------------------------------------------------
|  CrateWall_Piece02_08751c  @ $08751C  (50 B)
| ----------------------------------------------------------------------------
        .section .text.CrateWall_Piece02_08751c, "ax", @progbits
        .global CrateWall_Piece02_08751c
CrateWall_Piece02_08751c:
        lea     0x2ed9b8.l,a1                   | +000
        move.l  a1,0x74(a6)                     | +006
        lea     0xee002.l,a1                    | +00a
        move.l  a1,0x7c(a6)                     | +010
        move.w  #0x2,0x72(a6)                   | +014
        lea     0x2ec822.l,a0                   | +01a
        move.l  a0,0x48(a6)                     | +020
        lea     0x2ecfde.l,a1                   | +024
        move.l  a1,0x88(a6)                     | +02a
        bra.w   CrateWall_Piece_Common_087730 | +02e

| ----------------------------------------------------------------------------
|  CrateWall_Piece03_08754e  @ $08754E  (50 B)
| ----------------------------------------------------------------------------
        .section .text.CrateWall_Piece03_08754e, "ax", @progbits
        .global CrateWall_Piece03_08754e
CrateWall_Piece03_08754e:
        lea     0x2ed9cc.l,a1                   | +000
        move.l  a1,0x74(a6)                     | +006
        lea     0xee022.l,a1                    | +00a
        move.l  a1,0x7c(a6)                     | +010
        move.w  #0x3,0x72(a6)                   | +014
        lea     0x2ec822.l,a0                   | +01a
        move.l  a0,0x48(a6)                     | +020
        lea     0x2ecfde.l,a1                   | +024
        move.l  a1,0x88(a6)                     | +02a
        bra.w   CrateWall_Piece_Common_087730 | +02e

| ----------------------------------------------------------------------------
|  CrateWall_Piece04_087580  @ $087580  (48 B)
| ----------------------------------------------------------------------------
        .section .text.CrateWall_Piece04_087580, "ax", @progbits
        .global CrateWall_Piece04_087580
CrateWall_Piece04_087580:
        lea     0x2eda80.l,a1                   | +000
        move.l  a1,0x74(a6)                     | +006
        move.l  #0xffffffff,0x7c(a6)            | +00a
        move.w  #0x4,0x72(a6)                   | +012
        lea     0x2ec8ca.l,a0                   | +018
        move.l  a0,0x48(a6)                     | +01e
        lea     0x2ed002.l,a1                   | +022
        move.l  a1,0x88(a6)                     | +028
        bra.w   CrateWall_Piece_Common_087730 | +02c

| ----------------------------------------------------------------------------
|  CrateWall_Piece05_0875b0  @ $0875B0  (48 B)
| ----------------------------------------------------------------------------
        .section .text.CrateWall_Piece05_0875b0, "ax", @progbits
        .global CrateWall_Piece05_0875b0
CrateWall_Piece05_0875b0:
        lea     0x2eda94.l,a1                   | +000
        move.l  a1,0x74(a6)                     | +006
        move.l  #0xffffffff,0x7c(a6)            | +00a
        move.w  #0x5,0x72(a6)                   | +012
        lea     0x2ec8ca.l,a0                   | +018
        move.l  a0,0x48(a6)                     | +01e
        lea     0x2ed002.l,a1                   | +022
        move.l  a1,0x88(a6)                     | +028
        bra.w   CrateWall_Piece_Common_087730 | +02c

| ----------------------------------------------------------------------------
|  CrateWall_Piece06_0875e0  @ $0875E0  (48 B)
| ----------------------------------------------------------------------------
        .section .text.CrateWall_Piece06_0875e0, "ax", @progbits
        .global CrateWall_Piece06_0875e0
CrateWall_Piece06_0875e0:
        lea     0x2ed9e0.l,a1                   | +000
        move.l  a1,0x74(a6)                     | +006
        move.l  #0xffffffff,0x7c(a6)            | +00a
        move.w  #0x6,0x72(a6)                   | +012
        lea     0x2ec91e.l,a0                   | +018
        move.l  a0,0x48(a6)                     | +01e
        lea     0x2ed014.l,a1                   | +022
        move.l  a1,0x88(a6)                     | +028
        bra.w   CrateWall_Piece_Common_087730 | +02c

| ----------------------------------------------------------------------------
|  CrateWall_Piece07_087610  @ $087610  (48 B)
| ----------------------------------------------------------------------------
        .section .text.CrateWall_Piece07_087610, "ax", @progbits
        .global CrateWall_Piece07_087610
CrateWall_Piece07_087610:
        lea     0x2ed9f4.l,a1                   | +000
        move.l  a1,0x74(a6)                     | +006
        move.l  #0xffffffff,0x7c(a6)            | +00a
        move.w  #0x7,0x72(a6)                   | +012
        lea     0x2ec972.l,a0                   | +018
        move.l  a0,0x48(a6)                     | +01e
        lea     0x2ed026.l,a1                   | +022
        move.l  a1,0x88(a6)                     | +028
        bra.w   CrateWall_Piece_Common_087730 | +02c

| ----------------------------------------------------------------------------
|  CrateWall_Piece08_087640  @ $087640  (48 B)
| ----------------------------------------------------------------------------
        .section .text.CrateWall_Piece08_087640, "ax", @progbits
        .global CrateWall_Piece08_087640
CrateWall_Piece08_087640:
        lea     0x2eda08.l,a1                   | +000
        move.l  a1,0x74(a6)                     | +006
        move.l  #0xffffffff,0x7c(a6)            | +00a
        move.w  #0x8,0x72(a6)                   | +012
        lea     0x2ec9c6.l,a0                   | +018
        move.l  a0,0x48(a6)                     | +01e
        lea     0x2ed038.l,a1                   | +022
        move.l  a1,0x88(a6)                     | +028
        bra.w   CrateWall_Piece_Common_087730 | +02c

| ----------------------------------------------------------------------------
|  CrateWall_Piece09_087670  @ $087670  (48 B)
| ----------------------------------------------------------------------------
        .section .text.CrateWall_Piece09_087670, "ax", @progbits
        .global CrateWall_Piece09_087670
CrateWall_Piece09_087670:
        lea     0x2eda1c.l,a1                   | +000
        move.l  a1,0x74(a6)                     | +006
        move.l  #0xffffffff,0x7c(a6)            | +00a
        move.w  #0x9,0x72(a6)                   | +012
        lea     0x2ec9c6.l,a0                   | +018
        move.l  a0,0x48(a6)                     | +01e
        lea     0x2ed038.l,a1                   | +022
        move.l  a1,0x88(a6)                     | +028
        bra.w   CrateWall_Piece_Common_087730 | +02c

| ----------------------------------------------------------------------------
|  CrateWall_Piece10_0876a0  @ $0876A0  (48 B)
| ----------------------------------------------------------------------------
        .section .text.CrateWall_Piece10_0876a0, "ax", @progbits
        .global CrateWall_Piece10_0876a0
CrateWall_Piece10_0876a0:
        lea     0x2eda30.l,a1                   | +000
        move.l  a1,0x74(a6)                     | +006
        move.l  #0xffffffff,0x7c(a6)            | +00a
        move.w  #0xa,0x72(a6)                   | +012
        lea     0x2ec9c6.l,a0                   | +018
        move.l  a0,0x48(a6)                     | +01e
        lea     0x2ed038.l,a1                   | +022
        move.l  a1,0x88(a6)                     | +028
        bra.w   CrateWall_Piece_Common_087730 | +02c

| ----------------------------------------------------------------------------
|  CrateWall_Piece11_0876d0  @ $0876D0  (48 B)
| ----------------------------------------------------------------------------
        .section .text.CrateWall_Piece11_0876d0, "ax", @progbits
        .global CrateWall_Piece11_0876d0
CrateWall_Piece11_0876d0:
        lea     0x2eda44.l,a1                   | +000
        move.l  a1,0x74(a6)                     | +006
        move.l  #0xffffffff,0x7c(a6)            | +00a
        move.w  #0xb,0x72(a6)                   | +012
        lea     0x2ec9c6.l,a0                   | +018
        move.l  a0,0x48(a6)                     | +01e
        lea     0x2ed038.l,a1                   | +022
        move.l  a1,0x88(a6)                     | +028
        bra.w   CrateWall_Piece_Common_087730 | +02c

| ----------------------------------------------------------------------------
|  CrateWall_Piece12_087700  @ $087700  (206 B)
| ----------------------------------------------------------------------------
        .section .text.CrateWall_Piece12_087700, "ax", @progbits
        .global CrateWall_Piece12_087700
CrateWall_Piece12_087700:
        lea     0x2eda58.l,a1                   | +000
        move.l  a1,0x74(a6)                     | +006
        move.l  #0xffffffff,0x7c(a6)            | +00a
        move.w  #0xc,0x72(a6)                   | +012
        lea     0x2ec9c6.l,a0                   | +018
        move.l  a0,0x48(a6)                     | +01e
        lea     0x2ed038.l,a1                   | +022
        move.l  a1,0x88(a6)                     | +028
        bra.w   CrateWall_Piece_Common_087730 | +02c
        .global CrateWall_Piece_Common_087730
CrateWall_Piece_Common_087730:
        move.w  #0x30,0x70(a6)                  | +030
        move.w  #0x14,0x66(a6)                  | +036
        move.l  0x48(a6),0x84(a6)               | +03c
        lea     .L087748(pc),a1                 | +042
        move.l  a1,(a6)                         | +046
.L087748:
        jsr     0x2783a.l                       | +048
        jsr     0x2870a.l                       | +04e
        bcc.w   .L08776a                        | +054
        lea     0x5e766.l,a0                    | +058
        jsr     0x5e770.l                       | +05e
        bclr    #0x3,0x13(a6)                   | +064
.L08776a:
        jsr     0x28758.l                       | +06a
        bcc.w   .L0877be                        | +070
        lea     0xffff.w,a0                     | +074
        move.l  a0,0x48(a6)                     | +078
        move.w  #0x1027,d0                      | +07c
        jsr     0x2352.l                        | +080
        movea.l 0x74(a6),a2                     | +086
        jsr     0x5022a.l                       | +08a
        movea.l 0x88(a6),a1                     | +090
        jsr     0x77c7e.l                       | +094
        cmpi.l  #0xffffffff,0x7c(a6)            | +09a
        beq.w   .L0877c8                        | +0a2
        movea.l 0x7c(a6),a1                     | +0a6
        move.w  #0x82,d0                        | +0aa
        move.b  #0x0,0x82(a6)                   | +0ae
        jsr     0x4429e.l                       | +0b4
        bra.w   .L0877c8                        | +0ba
.L0877be:
        jsr     0x4fa70.l                       | +0be
        bcc.w   JsrPcThunk_0877ce               | +0c4
.L0877c8:
        jmp     0x518.l                         | +0c8

| ----------------------------------------------------------------------------
|  ArmoredCar_Init_0877d4  @ $0877D4  (106 B)
| ----------------------------------------------------------------------------
        .section .text.ArmoredCar_Init_0877d4, "ax", @progbits
        .global ArmoredCar_Init_0877d4
ArmoredCar_Init_0877d4:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0xe5,d1                        | +00a
        jsr     0x236e.l                        | +00e
        move.b  #0xff,0x32(a6)                  | +014
        move.b  #0xff,0x33(a6)                  | +01a
        move.b  #0x0,0x3a(a6)                   | +020
        move.w  #0xf0,0x70(a6)                  | +026
        move.w  #0x0,0x38(a6)                   | +02c
        lea     0x2ec180.l,a0                   | +032
        jsr     0x28cd4.l                       | +038
        lea     ArmoredCar_Turret_087aa0(pc),a1 | +03e
        jsr     0x4ae.l                         | +042
        jsr     0x5dd22.l                       | +048
        lea     .L087828(pc),a1                 | +04e
        move.l  a1,(a6)                         | +052
.L087828:
        cmpi.w  #0x100,0x22(a6)                 | +054
        blt.w   ArmoredCar_Engage_087846        | +05a
        clr.b   0x10e39a.l                      | +05e
        jsr     0x2783a.l                       | +064

| ----------------------------------------------------------------------------
|  ArmoredCar_Engage_087846  @ $087846  (146 B)
| ----------------------------------------------------------------------------
        .section .text.ArmoredCar_Engage_087846, "ax", @progbits
        .global ArmoredCar_Engage_087846
ArmoredCar_Engage_087846:
        bclr    #0x3,0x13(a6)                   | +000
        bclr    #0x0,0x13(a6)                   | +006
        lea     0x2c0628.l,a0                   | +00c
        jsr     0x799de.l                       | +012
        move.w  d0,0x66(a6)                     | +018
        addi.w  #0x2e4,0x66(a6)                 | +01c
        move.l  #0x2ede46,0x60(a6)              | +022
        lea     .L087876(pc),a1                 | +02a
        move.l  a1,(a6)                         | +02e
.L087876:
        clr.b   0x10e39a.l                      | +030
        jsr     0x2783a.l                       | +036
        jsr     0x28998.l                       | +03c
        jsr     0x28d70.l                       | +042
        jsr     0x2870a.l                       | +048
        bcc.w   .L0878aa                        | +04e
        lea     0x5e766.l,a0                    | +052
        jsr     0x5e770.l                       | +058
        bclr    #0x3,0x13(a6)                   | +05e
.L0878aa:
        cmpi.w  #0x29a,0x66(a6)                 | +064
        bgt.w   .L0878c6                        | +06a
        lea     0x2ed0c0.l,a1                   | +06e
        jsr     0x77c7e.l                       | +074
        lea     ArmoredCar_Damaged1_0878d8(pc),a1 | +07a
        move.l  a1,(a6)                         | +07e
.L0878c6:
        jsr     0x4fa70.l                       | +080
        bcc.w   .L0878d6                        | +086
        jmp     0x518.l                         | +08a
.L0878d6:
        rts                                     | +090

| ----------------------------------------------------------------------------
|  ArmoredCar_Damaged1_0878d8  @ $0878D8  (126 B)
| ----------------------------------------------------------------------------
        .section .text.ArmoredCar_Damaged1_0878d8, "ax", @progbits
        .global ArmoredCar_Damaged1_0878d8
ArmoredCar_Damaged1_0878d8:
        move.w  #0x102b,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x2ec19a.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L0878f4(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L0878f4:
        clr.b   0x10e39a.l                      | +01c
        jsr     0x2783a.l                       | +022
        jsr     0x28998.l                       | +028
        jsr     0x28d70.l                       | +02e
        jsr     0x2870a.l                       | +034
        bcc.w   .L087928                        | +03a
        lea     0x5e766.l,a0                    | +03e
        jsr     0x5e770.l                       | +044
        bclr    #0x3,0x13(a6)                   | +04a
.L087928:
        cmpi.w  #0xf6,0x66(a6)                  | +050
        bgt.w   .L087944                        | +056
        lea     0x2ed0d2.l,a1                   | +05a
        jsr     0x77c7e.l                       | +060
        lea     ArmoredCar_Damaged2_087956(pc),a1 | +066
        move.l  a1,(a6)                         | +06a
.L087944:
        jsr     0x4fa70.l                       | +06c
        bcc.w   .L087954                        | +072
        jmp     0x518.l                         | +076
.L087954:
        rts                                     | +07c

| ----------------------------------------------------------------------------
|  ArmoredCar_Damaged2_087956  @ $087956  (194 B)
| ----------------------------------------------------------------------------
        .section .text.ArmoredCar_Damaged2_087956, "ax", @progbits
        .global ArmoredCar_Damaged2_087956
ArmoredCar_Damaged2_087956:
        move.w  #0x102b,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x2ec1ae.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L087972(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L087972:
        clr.b   0x10e39a.l                      | +01c
        jsr     0x2783a.l                       | +022
        jsr     0x28998.l                       | +028
        jsr     0x28d70.l                       | +02e
        jsr     0x2870a.l                       | +034
        bcc.w   .L0879a6                        | +03a
        lea     0x5e766.l,a0                    | +03e
        jsr     0x5e770.l                       | +044
        bclr    #0x3,0x13(a6)                   | +04a
.L0879a6:
        jsr     0x28758.l                       | +050
        bcc.w   .L087a06                        | +056
        move.w  #0x1038,d0                      | +05a
        jsr     0x2352.l                        | +05e
        lea     0x2ed0e4.l,a1                   | +064
        jsr     0x77c7e.l                       | +06a
        lea     0x2ecfcc.l,a1                   | +070
        jsr     0x77c7e.l                       | +076
        lea     0x2edd56.l,a1                   | +07c
        jsr     0x43fac.l                       | +082
        lea     0x2edd68.l,a1                   | +088
        jsr     0x43fac.l                       | +08e
        jsr     0x434dc.l                       | +094
        lea     ArmoredCar_BlastBox_087a62(pc),a1 | +09a
        jsr     0x4ae.l                         | +09e
        jsr     0x5dd02.l                       | +0a4
        lea     ArmoredCar_Wreck_087a18(pc),a1  | +0aa
        move.l  a1,(a6)                         | +0ae
.L087a06:
        jsr     0x4fa70.l                       | +0b0
        bcc.w   .L087a16                        | +0b6
        jmp     0x518.l                         | +0ba
.L087a16:
        rts                                     | +0c0

| ----------------------------------------------------------------------------
|  ArmoredCar_Wreck_087a18  @ $087A18  (74 B)
| ----------------------------------------------------------------------------
        .section .text.ArmoredCar_Wreck_087a18, "ax", @progbits
        .global ArmoredCar_Wreck_087a18
ArmoredCar_Wreck_087a18:
        move.l  #0x5000,d0                      | +000
        jsr     0x51a28.l                       | +006
        lea     0xffff.w,a0                     | +00c
        move.l  a0,0x48(a6)                     | +010
        lea     0x2ec1c2.l,a0                   | +014
        jsr     0x28cd4.l                       | +01a
        lea     .L087a3e(pc),a1                 | +020
        move.l  a1,(a6)                         | +024
.L087a3e:
        jsr     0x2783a.l                       | +026
        jsr     0x28d70.l                       | +02c
        jsr     0x283d8.l                       | +032
        jsr     0x4fa70.l                       | +038
        bcc.w   .L087a60                        | +03e
        jmp     0x518.l                         | +042
.L087a60:
        rts                                     | +048

| ----------------------------------------------------------------------------
|  ArmoredCar_BlastBox_087a62  @ $087A62  (62 B)
| ----------------------------------------------------------------------------
        .section .text.ArmoredCar_BlastBox_087a62, "ax", @progbits
        .global ArmoredCar_BlastBox_087a62
ArmoredCar_BlastBox_087a62:
        move.w  #0x10,0x72(a6)                  | +000
        lea     .L087a6e(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L087a6e:
        jsr     0x2783a.l                       | +00c
        lea     0x2ecc34.l,a0                   | +012
        move.l  a0,0x4c(a6)                     | +018
        jsr     0x283ca.l                       | +01c
        jsr     0x283ca.l                       | +022
        jsr     0x283d8.l                       | +028
        subq.w  #0x1,0x72(a6)                   | +02e
        bpl.w   .L087a9e                        | +032
        jmp     0x518.l                         | +036
.L087a9e:
        rts                                     | +03c

| ----------------------------------------------------------------------------
|  ArmoredCar_Turret_087aa0  @ $087AA0  (124 B)
| ----------------------------------------------------------------------------
        .section .text.ArmoredCar_Turret_087aa0, "ax", @progbits
        .global ArmoredCar_Turret_087aa0
ArmoredCar_Turret_087aa0:
        move.w  #0xe5,d1                        | +000
        jsr     0x236e.l                        | +004
        move.b  #0xff,0x32(a6)                  | +00a
        move.b  #0xff,0x33(a6)                  | +010
        move.b  #0x0,0x3a(a6)                   | +016
        move.w  #0x8000,d0                      | +01c
        move.w  #0x80,d1                        | +020
        jsr     0x2813c.l                       | +024
        lea     .L087ad0(pc),a1                 | +02a
        move.l  a1,(a6)                         | +02e
.L087ad0:
        movea.l 0xc(a6),a0                      | +030
        move.w  0x22(a0),0x22(a6)               | +034
        move.w  0x24(a0),0x24(a6)               | +03a
        clr.l   d0                              | +040
        move.b  0x20(a0),d0                     | +042
        cmpi.b  #0x3,d0                         | +046
        beq.w   .L087b14                        | +04a
        movea.l #0x2ec21a,a0                    | +04e
        lsl.w   #0x2,d0                         | +054
        movea.l (a0,d0.w),a0                    | +056
        cmpa.l  #0xffffffff,a0                  | +05a
        beq.w   .L087b0a                        | +060
        jsr     0x28cd4.l                       | +064
.L087b0a:
        jsr     0x28d70.l                       | +06a
        bra.w   .L087b1a                        | +070
.L087b14:
        jmp     0x518.l                         | +074
.L087b1a:
        rts                                     | +07a

| ----------------------------------------------------------------------------
|  Heli_Init_087b1c  @ $087B1C  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Heli_Init_087b1c, "ax", @progbits
        .global Heli_Init_087b1c
Heli_Init_087b1c:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004

| ----------------------------------------------------------------------------
|  Heli_InitTmpl_087b26  @ $087B26  (252 B)
| ----------------------------------------------------------------------------
        .section .text.Heli_InitTmpl_087b26, "ax", @progbits
        .global Heli_InitTmpl_087b26
Heli_InitTmpl_087b26:
        jsr     0x2783a.l                       | +000
        move.w  #0x86,d1                        | +006
        jsr     0x236e.l                        | +00a
        move.w  #0x87,d1                        | +010
        jsr     0x236e.l                        | +014
        move.b  #0xff,0x32(a6)                  | +01a
        move.b  #0xff,0x33(a6)                  | +020
        move.b  #0x0,0x3a(a6)                   | +026
        move.w  #0x50,0x70(a6)                  | +02c
        lea     0x2c05a6.l,a0                   | +032
        jsr     0x799de.l                       | +038
        move.w  d0,0x66(a6)                     | +03e
        addi.w  #0x17c,0x66(a6)                 | +042
        move.w  #0x0,0x72(a6)                   | +048
        move.b  #0x0,0x20(a6)                   | +04e
        move.w  #0x2000,0x38(a6)                | +054
        lea     0x2ec226.l,a0                   | +05a
        jsr     0x28cd4.l                       | +060
        lea     Heli_Dropper_087eae(pc),a1      | +066
        jsr     0x4ae.l                         | +06a
        jsr     0x5dd22.l                       | +070
        lea     0x2edd7e.l,a1                   | +076
        jsr     0x43fac.l                       | +07c
        move.l  #0x2eddfa,0x60(a6)              | +082
        lea     .L087bb6(pc),a1                 | +08a
        move.l  a1,(a6)                         | +08e
.L087bb6:
        jsr     0x28998.l                       | +090
        jsr     0x2783a.l                       | +096
        jsr     Heli_RotorAnim_0883ec(pc)       | +09c
        jsr     0x28d70.l                       | +0a0
        jsr     0x2870a.l                       | +0a6
        bcc.w   .L087bee                        | +0ac
        lea     0x5e766.l,a0                    | +0b0
        jsr     0x5e770.l                       | +0b6
        bclr    #0x3,0x13(a6)                   | +0bc
        move.w  #0xf,0x72(a6)                   | +0c2
.L087bee:
        bclr    #0x0,0x13(a6)                   | +0c8
        move.w  #0x17c,0x66(a6)                 | +0ce
        cmpi.w  #0x100,0x22(a6)                 | +0d4
        bgt.w   .L087c0a                        | +0da
        lea     Heli_Approach_087c22(pc),a1     | +0de
        move.l  a1,(a6)                         | +0e2
.L087c0a:
        jsr     0x4fa70.l                       | +0e4
        bcc.w   .L087c20                        | +0ea
        move.b  #0xff,0x20(a6)                  | +0ee
        jmp     0x518.l                         | +0f4
.L087c20:
        rts                                     | +0fa

| ----------------------------------------------------------------------------
|  Heli_Approach_087c22  @ $087C22  (120 B)
| ----------------------------------------------------------------------------
        .section .text.Heli_Approach_087c22, "ax", @progbits
        .global Heli_Approach_087c22
Heli_Approach_087c22:
        lea     0x2ec23c.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L087c34(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L087c34:
        jsr     0x28998.l                       | +012
        jsr     0x2783a.l                       | +018
        jsr     Heli_RotorAnim_0883ec(pc)       | +01e
        jsr     0x28d70.l                       | +022
        bcc.w   .L087c54                        | +028
        lea     Heli_Hover_087c9a(pc),a1        | +02c
        move.l  a1,(a6)                         | +030
.L087c54:
        jsr     0x2870a.l                       | +032
        bcc.w   .L087c76                        | +038
        lea     0x5e766.l,a0                    | +03c
        jsr     0x5e770.l                       | +042
        bclr    #0x3,0x13(a6)                   | +048
        move.w  #0xf,0x72(a6)                   | +04e
.L087c76:
        bclr    #0x0,0x13(a6)                   | +054
        move.w  #0x17c,0x66(a6)                 | +05a
        jsr     0x4fa70.l                       | +060
        bcc.w   .L087c98                        | +066
        move.b  #0xff,0x20(a6)                  | +06a
        jmp     0x518.l                         | +070
.L087c98:
        rts                                     | +076

| ----------------------------------------------------------------------------
|  Heli_Hover_087c9a  @ $087C9A  (130 B)
| ----------------------------------------------------------------------------
        .section .text.Heli_Hover_087c9a, "ax", @progbits
        .global Heli_Hover_087c9a
Heli_Hover_087c9a:
        lea     0xee042.l,a1                    | +000
        move.w  #0x82,d0                        | +006
        move.b  #0x0,0x82(a6)                   | +00a
        jsr     0x4429e.l                       | +010
        lea     .L087cb6(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L087cb6:
        jsr     0x28998.l                       | +01c
        jsr     0x2783a.l                       | +022
        jsr     Heli_RotorAnim_0883ec(pc)       | +028
        jsr     0x28d70.l                       | +02c
        jsr     0x2870a.l                       | +032
        bcc.w   .L087cee                        | +038
        lea     0x5e766.l,a0                    | +03c
        jsr     0x5e770.l                       | +042
        bclr    #0x3,0x13(a6)                   | +048
        move.w  #0xf,0x72(a6)                   | +04e
.L087cee:
        cmpi.w  #0xfc,0x66(a6)                  | +054
        bgt.w   .L087cfe                        | +05a
        lea     Heli_Damaged1_087d1c(pc),a1     | +05e
        move.l  a1,(a6)                         | +062
.L087cfe:
        jsr     0x4fa70.l                       | +064
        bcc.w   .L087d1a                        | +06a
        move.b  #0xff,0x82(a6)                  | +06e
        move.b  #0xff,0x20(a6)                  | +074
        jmp     0x518.l                         | +07a
.L087d1a:
        rts                                     | +080

| ----------------------------------------------------------------------------
|  Heli_Damaged1_087d1c  @ $087D1C  (130 B)
| ----------------------------------------------------------------------------
        .section .text.Heli_Damaged1_087d1c, "ax", @progbits
        .global Heli_Damaged1_087d1c
Heli_Damaged1_087d1c:
        move.w  #0x1027,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x2ec2ce.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L087d38(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L087d38:
        jsr     0x28998.l                       | +01c
        jsr     0x2783a.l                       | +022
        jsr     Heli_RotorAnim_0883ec(pc)       | +028
        jsr     0x28d70.l                       | +02c
        jsr     0x2870a.l                       | +032
        bcc.w   .L087d70                        | +038
        lea     0x5e766.l,a0                    | +03c
        jsr     0x5e770.l                       | +042
        bclr    #0x3,0x13(a6)                   | +048
        move.w  #0xf,0x72(a6)                   | +04e
.L087d70:
        cmpi.w  #0x7e,0x66(a6)                  | +054
        bgt.w   .L087d80                        | +05a
        lea     Heli_Damaged2_087d9e(pc),a1     | +05e
        move.l  a1,(a6)                         | +062
.L087d80:
        jsr     0x4fa70.l                       | +064
        bcc.w   .L087d9c                        | +06a
        move.b  #0xff,0x82(a6)                  | +06e
        move.b  #0xff,0x20(a6)                  | +074
        jmp     0x518.l                         | +07a
.L087d9c:
        rts                                     | +080

| ----------------------------------------------------------------------------
|  Heli_Damaged2_087d9e  @ $087D9E  (130 B)
| ----------------------------------------------------------------------------
        .section .text.Heli_Damaged2_087d9e, "ax", @progbits
        .global Heli_Damaged2_087d9e
Heli_Damaged2_087d9e:
        move.w  #0x1027,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x2ec2de.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L087dba(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L087dba:
        jsr     0x28998.l                       | +01c
        jsr     0x2783a.l                       | +022
        jsr     Heli_RotorAnim_0883ec(pc)       | +028
        jsr     0x28d70.l                       | +02c
        jsr     0x2870a.l                       | +032
        bcc.w   .L087df2                        | +038
        lea     0x5e766.l,a0                    | +03c
        jsr     0x5e770.l                       | +042
        bclr    #0x3,0x13(a6)                   | +048
        move.w  #0xf,0x72(a6)                   | +04e
.L087df2:
        jsr     0x28758.l                       | +054
        bcc.w   .L087e02                        | +05a
        lea     Heli_Crash_087e20(pc),a1        | +05e
        move.l  a1,(a6)                         | +062
.L087e02:
        jsr     0x4fa70.l                       | +064
        bcc.w   .L087e1e                        | +06a
        move.b  #0xff,0x82(a6)                  | +06e
        move.b  #0xff,0x20(a6)                  | +074
        jmp     0x518.l                         | +07a
.L087e1e:
        rts                                     | +080

| ----------------------------------------------------------------------------
|  Heli_Crash_087e20  @ $087E20  (142 B)
| ----------------------------------------------------------------------------
        .section .text.Heli_Crash_087e20, "ax", @progbits
        .global Heli_Crash_087e20
Heli_Crash_087e20:
        move.l  #0x5000,d0                      | +000
        jsr     0x51a28.l                       | +006
        move.w  #0x1023,d0                      | +00c
        jsr     0x2352.l                        | +010
        jsr     0x2783a.l                       | +016
        move.b  #0xff,0x82(a6)                  | +01c
        move.b  #0xff,0x20(a6)                  | +022
        lea     0x77fd6.l,a1                    | +028
        jsr     0x4ae.l                         | +02e
        jsr     0x5dd22.l                       | +034
        lea     0x2edd92.l,a1                   | +03a
        jsr     0x43fac.l                       | +040
        lea     0x2ed176.l,a1                   | +046
        jsr     0x77c7e.l                       | +04c
        lea     0x2ed188.l,a1                   | +052
        jsr     0x77c7e.l                       | +058
        lea     0x2ec2ee.l,a0                   | +05e
        jsr     0x28cd4.l                       | +064
        lea     .L087e90(pc),a1                 | +06a
        move.l  a1,(a6)                         | +06e
.L087e90:
        jsr     0x2783a.l                       | +070
        jsr     0x28d70.l                       | +076
        jsr     0x4fa70.l                       | +07c
        bcc.w   .L087eac                        | +082
        jmp     0x518.l                         | +086
.L087eac:
        rts                                     | +08c

| ----------------------------------------------------------------------------
|  Heli_Dropper_087eae  @ $087EAE  (158 B)
| ----------------------------------------------------------------------------
        .section .text.Heli_Dropper_087eae, "ax", @progbits
        .global Heli_Dropper_087eae
Heli_Dropper_087eae:
        move.w  #0x88,d1                        | +000
        jsr     0x236e.l                        | +004
        move.b  #0xff,0x32(a6)                  | +00a
        move.b  #0xff,0x33(a6)                  | +010
        move.b  #0x0,0x3a(a6)                   | +016
        move.w  #0x0,0x38(a6)                   | +01c
        lea     0x2ec2fe.l,a0                   | +022
        jsr     0x28cd4.l                       | +028
        move.b  #0x0,0x83(a6)                   | +02e
        lea     0x77228.l,a1                    | +034
        jsr     0x4ae.l                         | +03a
        jsr     0x5dd22.l                       | +040
        addi.w  #0x58,0x22(a0)                  | +046
        move.b  #0x30,0x98(a0)                  | +04c
        move.b  #0x83,0x99(a0)                  | +052
        lea     .L087f0c(pc),a1                 | +058
        move.l  a1,(a6)                         | +05c
.L087f0c:
        jsr     0x2783a.l                       | +05e
        jsr     0x28d70.l                       | +064
        movea.l 0xc(a6),a0                      | +06a
        cmpi.b  #0xff,0x20(a0)                  | +06e
        bne.w   .L087f4a                        | +074
        move.b  #0xff,0x83(a6)                  | +078
        lea     0x77f6a.l,a1                    | +07e
        jsr     0x4ae.l                         | +084
        jsr     0x5dd22.l                       | +08a
        addi.w  #0x58,0x22(a0)                  | +090
        jmp     0x518.l                         | +096
.L087f4a:
        rts                                     | +09c

| ----------------------------------------------------------------------------
|  Debris_Scatter_087f4c  @ $087F4C  (112 B)
| ----------------------------------------------------------------------------
        .section .text.Debris_Scatter_087f4c, "ax", @progbits
        .global Debris_Scatter_087f4c
Debris_Scatter_087f4c:
        move.w  #0xf3,d1                        | +000
        jsr     0x236e.l                        | +004
        jsr     0x267e2.l                       | +00a
        move.w  #0x7f,d0                        | +010
        jsr     0x5ea1c.l                       | +014
        btst    #0x0,d0                         | +01a
        beq.w   .L087f70                        | +01e
        neg.w   d0                              | +022
.L087f70:
        move.w  d0,0x28(a6)                     | +024
        andi.w  #0x7f,d0                        | +028
        sub.w   d0,0x2a(a6)                     | +02c
        lea     0x2de4b0.l,a0                   | +030
        jsr     0x28cd4.l                       | +036
        lea     .L087f8e(pc),a1                 | +03c
        move.l  a1,(a6)                         | +040
.L087f8e:
        jsr     0x27cee.l                       | +042
        jsr     0x28d70.l                       | +048
        bcs.w   .L087fb4                        | +04e
        movea.l #0xffffffff,a0                  | +052
        lea     0x298736.l,a0                   | +058
        jsr     0x5dd56.l                       | +05e
        bcc.w   .L087fba                        | +064
.L087fb4:
        jmp     0x518.l                         | +068
.L087fba:
        rts                                     | +06e

| ----------------------------------------------------------------------------
|  Prop_Quad_Spawn_087fbc  @ $087FBC  (80 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_Quad_Spawn_087fbc, "ax", @progbits
        .global Prop_Quad_Spawn_087fbc
Prop_Quad_Spawn_087fbc:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        lea     Prop_Quad_A_08800e(pc),a1       | +00a
        jsr     0x4ae.l                         | +00e
        jsr     0x5dd22.l                       | +014
        lea     Prop_Quad_B_088028(pc),a1       | +01a
        jsr     0x4ae.l                         | +01e
        jsr     0x5dd22.l                       | +024
        lea     Prop_Quad_C_088042(pc),a1       | +02a
        jsr     0x4ae.l                         | +02e
        jsr     0x5dd22.l                       | +034
        lea     Prop_Quad_D_08805c(pc),a1       | +03a
        jsr     0x4ae.l                         | +03e
        jsr     0x5dd22.l                       | +044
        jmp     0x518.l                         | +04a

| ----------------------------------------------------------------------------
|  Rts_08800c  @ $08800C  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Rts_08800c, "ax", @progbits
        .global Rts_08800c
Rts_08800c:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  Prop_Quad_A_08800e  @ $08800E  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_Quad_A_08800e, "ax", @progbits
        .global Prop_Quad_A_08800e
Prop_Quad_A_08800e:
        lea     0x2ec30e.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2ed19a.l,a1                   | +00c
        move.l  a1,0x88(a6)                     | +012
        bra.w   Prop_Quad_Common_088076     | +016

| ----------------------------------------------------------------------------
|  Prop_Quad_B_088028  @ $088028  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_Quad_B_088028, "ax", @progbits
        .global Prop_Quad_B_088028
Prop_Quad_B_088028:
        lea     0x2ec324.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2ed1ac.l,a1                   | +00c
        move.l  a1,0x88(a6)                     | +012
        bra.w   Prop_Quad_Common_088076     | +016

| ----------------------------------------------------------------------------
|  Prop_Quad_C_088042  @ $088042  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_Quad_C_088042, "ax", @progbits
        .global Prop_Quad_C_088042
Prop_Quad_C_088042:
        lea     0x2ec33a.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2ed1be.l,a1                   | +00c
        move.l  a1,0x88(a6)                     | +012
        bra.w   Prop_Quad_Common_088076     | +016

| ----------------------------------------------------------------------------
|  Prop_Quad_D_08805c  @ $08805C  (184 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_Quad_D_08805c, "ax", @progbits
        .global Prop_Quad_D_08805c
Prop_Quad_D_08805c:
        lea     0x2ec350.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2ed1d0.l,a1                   | +00c
        move.l  a1,0x88(a6)                     | +012
        bra.w   Prop_Quad_Common_088076     | +016
        .global Prop_Quad_Common_088076
Prop_Quad_Common_088076:
        move.w  #0xe4,d1                        | +01a
        jsr     0x236e.l                        | +01e
        move.b  #0xff,0x32(a6)                  | +024
        move.b  #0xff,0x33(a6)                  | +02a
        move.w  #0x40,0x70(a6)                  | +030
        move.w  #0xa,0x66(a6)                   | +036
        move.w  #0x8000,0x38(a6)                | +03c
        jsr     0x267e2.l                       | +042
        jsr     0x27cee.l                       | +048
        lea     .L0880b0(pc),a1                 | +04e
        move.l  a1,(a6)                         | +052
.L0880b0:
        jsr     0x2783a.l                       | +054
        jsr     0x28d70.l                       | +05a
        jsr     0x2870a.l                       | +060
        bcc.w   .L0880d8                        | +066
        lea     0x5e766.l,a0                    | +06a
        jsr     0x5e770.l                       | +070
        bclr    #0x3,0x13(a6)                   | +076
.L0880d8:
        jsr     0x28758.l                       | +07c
        bcc.w   .L088102                        | +082
        move.w  #0x102e,d0                      | +086
        jsr     0x2352.l                        | +08a
        lea     0xffff.w,a0                     | +090
        move.l  a0,0x48(a6)                     | +094
        movea.l 0x88(a6),a1                     | +098
        jsr     0x77c7e.l                       | +09c
        bra.w   .L08810c                        | +0a2
.L088102:
        jsr     0x4fa70.l                       | +0a6
        bcc.w   .L088112                        | +0ac
.L08810c:
        jmp     0x518.l                         | +0b0
.L088112:
        rts                                     | +0b6

| ----------------------------------------------------------------------------
|  Barricade_Init_088114  @ $088114  (148 B)
| ----------------------------------------------------------------------------
        .section .text.Barricade_Init_088114, "ax", @progbits
        .global Barricade_Init_088114
Barricade_Init_088114:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0xe6,d1                        | +00a
        jsr     0x236e.l                        | +00e
        move.b  #0xff,0x32(a6)                  | +014
        move.b  #0xff,0x33(a6)                  | +01a
        move.w  #0x40,0x70(a6)                  | +020
        move.w  #0x32,0x66(a6)                  | +026
        move.w  #0x8000,0x38(a6)                | +02c
        jsr     0x267e2.l                       | +032
        jsr     0x27cee.l                       | +038
        lea     .L088158(pc),a1                 | +03e
        move.l  a1,(a6)                         | +042
.L088158:
        jsr     0x2783a.l                       | +044
        jsr     0x2870a.l                       | +04a
        bcc.w   .L08817a                        | +050
        lea     0x5e766.l,a0                    | +054
        jsr     0x5e770.l                       | +05a
        bclr    #0x3,0x13(a6)                   | +060
.L08817a:
        move.w  #0x32,0x66(a6)                  | +066
        bclr    #0x0,0x13(a6)                   | +06c
        cmpi.w  #0x50,0x22(a6)                  | +072
        bgt.w   .L088196                        | +078
        lea     Barricade_Arm_0881a8(pc),a1     | +07c
        move.l  a1,(a6)                         | +080
.L088196:
        jsr     0x4fa70.l                       | +082
        bcc.w   .L0881a6                        | +088
        jmp     0x518.l                         | +08c
.L0881a6:
        rts                                     | +092

| ----------------------------------------------------------------------------
|  Barricade_Arm_0881a8  @ $0881A8  (98 B)
| ----------------------------------------------------------------------------
        .section .text.Barricade_Arm_0881a8, "ax", @progbits
        .global Barricade_Arm_0881a8
Barricade_Arm_0881a8:
        lea     0x2ec366.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L0881ba(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L0881ba:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        bcc.w   .L0881d0                        | +01e
        lea     Barricade_Stage1_08820a(pc),a1  | +022
        move.l  a1,(a6)                         | +026
.L0881d0:
        jsr     0x2870a.l                       | +028
        bcc.w   .L0881ec                        | +02e
        lea     0x5e766.l,a0                    | +032
        jsr     0x5e770.l                       | +038
        bclr    #0x3,0x13(a6)                   | +03e
.L0881ec:
        move.w  #0x32,0x66(a6)                  | +044
        bclr    #0x0,0x13(a6)                   | +04a
        jsr     0x4fa70.l                       | +050
        bcc.w   .L088208                        | +056
        jmp     0x518.l                         | +05a
.L088208:
        rts                                     | +060

| ----------------------------------------------------------------------------
|  Barricade_Stage1_08820a  @ $08820A  (102 B)
| ----------------------------------------------------------------------------
        .section .text.Barricade_Stage1_08820a, "ax", @progbits
        .global Barricade_Stage1_08820a
Barricade_Stage1_08820a:
        lea     0xee062.l,a1                    | +000
        move.w  #0x82,d0                        | +006
        move.b  #0x0,0x82(a6)                   | +00a
        jsr     0x4429e.l                       | +010
        lea     .L088226(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L088226:
        jsr     0x2783a.l                       | +01c
        jsr     0x28d70.l                       | +022
        jsr     0x2870a.l                       | +028
        bcc.w   .L08824e                        | +02e
        lea     0x5e766.l,a0                    | +032
        jsr     0x5e770.l                       | +038
        bclr    #0x3,0x13(a6)                   | +03e
.L08824e:
        cmpi.w  #0x28,0x66(a6)                  | +044
        bgt.w   .L08825e                        | +04a
        lea     Barricade_Stage2_088270(pc),a1  | +04e
        move.l  a1,(a6)                         | +052
.L08825e:
        jsr     0x4fa70.l                       | +054
        bcc.w   .L08826e                        | +05a
        jmp     0x518.l                         | +05e
.L08826e:
        rts                                     | +064

| ----------------------------------------------------------------------------
|  Barricade_Stage2_088270  @ $088270  (92 B)
| ----------------------------------------------------------------------------
        .section .text.Barricade_Stage2_088270, "ax", @progbits
        .global Barricade_Stage2_088270
Barricade_Stage2_088270:
        lea     0x2ec3de.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L088282(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L088282:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        jsr     0x2870a.l                       | +01e
        bcc.w   .L0882aa                        | +024
        lea     0x5e766.l,a0                    | +028
        jsr     0x5e770.l                       | +02e
        bclr    #0x3,0x13(a6)                   | +034
.L0882aa:
        cmpi.w  #0x1e,0x66(a6)                  | +03a
        bgt.w   .L0882ba                        | +040
        lea     Barricade_Stage3_0882cc(pc),a1  | +044
        move.l  a1,(a6)                         | +048
.L0882ba:
        jsr     0x4fa70.l                       | +04a
        bcc.w   .L0882ca                        | +050
        jmp     0x518.l                         | +054
.L0882ca:
        rts                                     | +05a

| ----------------------------------------------------------------------------
|  Barricade_Stage3_0882cc  @ $0882CC  (92 B)
| ----------------------------------------------------------------------------
        .section .text.Barricade_Stage3_0882cc, "ax", @progbits
        .global Barricade_Stage3_0882cc
Barricade_Stage3_0882cc:
        lea     0x2ec3ee.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L0882de(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L0882de:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        jsr     0x2870a.l                       | +01e
        bcc.w   .L088306                        | +024
        lea     0x5e766.l,a0                    | +028
        jsr     0x5e770.l                       | +02e
        bclr    #0x3,0x13(a6)                   | +034
.L088306:
        cmpi.w  #0x14,0x66(a6)                  | +03a
        bgt.w   .L088316                        | +040
        lea     Barricade_Stage4_088328(pc),a1  | +044
        move.l  a1,(a6)                         | +048
.L088316:
        jsr     0x4fa70.l                       | +04a
        bcc.w   .L088326                        | +050
        jmp     0x518.l                         | +054
.L088326:
        rts                                     | +05a

| ----------------------------------------------------------------------------
|  Barricade_Stage4_088328  @ $088328  (104 B)
| ----------------------------------------------------------------------------
        .section .text.Barricade_Stage4_088328, "ax", @progbits
        .global Barricade_Stage4_088328
Barricade_Stage4_088328:
        lea     0x2ec3fe.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L08833a(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L08833a:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        jsr     0x2870a.l                       | +01e
        bcc.w   .L088362                        | +024
        lea     0x5e766.l,a0                    | +028
        jsr     0x5e770.l                       | +02e
        bclr    #0x3,0x13(a6)                   | +034
.L088362:
        cmpi.w  #0xa,0x66(a6)                   | +03a
        bgt.w   Barricade_Stage4_Tail_08837e | +040
        lea     0x2ed1e2.l,a1                   | +044
        jsr     0x77c7e.l                       | +04a
        lea     Barricade_Stage5_088390(pc),a1  | +050
        move.l  a1,(a6)                         | +054
        .global Barricade_Stage4_Tail_08837e
Barricade_Stage4_Tail_08837e:
        jsr     0x4fa70.l                       | +056
        bcc.w   .L08838e                        | +05c
        jmp     0x518.l                         | +060
.L08838e:
        rts                                     | +066

| ----------------------------------------------------------------------------
|  Barricade_Stage5_088390  @ $088390  (92 B)
| ----------------------------------------------------------------------------
        .section .text.Barricade_Stage5_088390, "ax", @progbits
        .global Barricade_Stage5_088390
Barricade_Stage5_088390:
        lea     0x2ec40e.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L0883a2(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L0883a2:
        jsr     Entity_CmpPrioWithSibling_086552(pc) | +012
        bcs.b   Barricade_Stage4_Tail_08837e | +016
        jsr     0x2783a.l                       | +018
        jsr     0x28d70.l                       | +01e
        jsr     0x2870a.l                       | +024
        bcc.w   .L0883d0                        | +02a
        lea     0x5e766.l,a0                    | +02e
        jsr     0x5e770.l                       | +034
        bclr    #0x3,0x13(a6)                   | +03a
.L0883d0:
        jsr     0x28758.l                       | +040
        bra.w   .L0883e4                        | +046
        jsr     0x4fa70.l                       | +04a
        bcc.w   .L0883ea                        | +050
.L0883e4:
        jmp     0x518.l                         | +054
.L0883ea:
        rts                                     | +05a

| ----------------------------------------------------------------------------
|  Heli_RotorAnim_0883ec  @ $0883EC  (70 B)
| ----------------------------------------------------------------------------
        .section .text.Heli_RotorAnim_0883ec, "ax", @progbits
        .global Heli_RotorAnim_0883ec
Heli_RotorAnim_0883ec:
        move.b  0x106f28.l,d0                   | +000
        cmpi.w  #0x0,0x72(a6)                   | +006
        ble.w   .L08841c                        | +00c
        andi.b  #0x3,d0                         | +010
        bne.w   SetTaskWRts_088436              | +014
        move.w  0x18(a6),d0                     | +018
        move.w  0x16(a6),0x18(a6)               | +01c
        move.w  d0,0x16(a6)                     | +022
        move.w  d0,0x14(a6)                     | +026
        subq.w  #0x1,0x72(a6)                   | +02a
        rts                                     | +02e
.L08841c:
        andi.b  #0xf,d0                         | +030
        bne.w   SetTaskWRts_088436              | +034
        move.w  0x18(a6),d0                     | +038
        move.w  0x16(a6),0x18(a6)               | +03c
        move.w  d0,0x16(a6)                     | +042

| ----------------------------------------------------------------------------
|  Entity_HitboxPulseTable_088438  @ $088438  (50 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_HitboxPulseTable_088438, "ax", @progbits
        .global Entity_HitboxPulseTable_088438
Entity_HitboxPulseTable_088438:
        addq.w  #0x1,0x72(a6)                   | +000
        move.w  0x72(a6),d0                     | +004
        andi.b  #0x3,d0                         | +008
        bne.w   .L088460                        | +00c
        clr.l   d0                              | +010
        move.b  0x21(a6),d0                     | +012
        asl.l   #0x2,d0                         | +016
        lea     0x2ec712.l,a0                   | +018
        movea.l (a0,d0.w),a0                    | +01e
        move.l  a0,0x48(a6)                     | +022
        rts                                     | +026
.L088460:
        lea     0xffff.w,a0                     | +028
        move.l  a0,0x48(a6)                     | +02c
        rts                                     | +030

| ----------------------------------------------------------------------------
|  Entity_HitboxPulseSaved_08846a  @ $08846A  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_HitboxPulseSaved_08846a, "ax", @progbits
        .global Entity_HitboxPulseSaved_08846a
Entity_HitboxPulseSaved_08846a:
        addq.w  #0x1,0x72(a6)                   | +000
        move.w  0x72(a6),d0                     | +004
        andi.b  #0x3,d0                         | +008
        bne.w   .L088482                        | +00c
        move.l  0x84(a6),0x48(a6)               | +010
        rts                                     | +016
.L088482:
        lea     0xffff.w,a0                     | +018
        move.l  a0,0x48(a6)                     | +01c
        rts                                     | +020

| ----------------------------------------------------------------------------
|  Entity_PropagateDamageToParent_08848c  @ $08848C  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_PropagateDamageToParent_08848c, "ax", @progbits
        .global Entity_PropagateDamageToParent_08848c
Entity_PropagateDamageToParent_08848c:
        move.w  0x80(a6),d0                     | +000
        move.w  0x66(a6),d1                     | +004
        sub.w   d1,d0                           | +008
        movea.l 0xc(a6),a0                      | +00a
        sub.w   d0,0x66(a0)                     | +00e
        move.w  0x66(a6),0x80(a6)               | +012
        rts                                     | +018

| ----------------------------------------------------------------------------
|  Fort_SpawnChildren_V0_0884a6  @ $0884A6  (270 B)
| ----------------------------------------------------------------------------
        .section .text.Fort_SpawnChildren_V0_0884a6, "ax", @progbits
        .global Fort_SpawnChildren_V0_0884a6
Fort_SpawnChildren_V0_0884a6:
        lea     Pillbox_Init_Right_0868a6(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd22.l                       | +00a
        addi.w  #0x28,0x22(a0)                  | +010
        addi.w  #0x50,0x24(a0)                  | +016
        move.w  #0x60,0x38(a0)                  | +01c
        lea     0x2ed738.l,a1                   | +022
        move.l  a1,0x74(a0)                     | +028
        move.w  #0x0,0x72(a0)                   | +02c
        lea     0xee066.l,a1                    | +032
        move.l  a1,0x7c(a0)                     | +038
        lea     Pillbox_Init_08686a(pc),a1      | +03c
        jsr     0x4ae.l                         | +040
        jsr     0x5dd22.l                       | +046
        addi.w  #0x68,0x22(a0)                  | +04c
        addi.w  #0x50,0x24(a0)                  | +052
        move.w  #0x60,0x38(a0)                  | +058
        lea     0x2ed74c.l,a1                   | +05e
        move.l  a1,0x74(a0)                     | +064
        move.w  #0x1,0x72(a0)                   | +068
        lea     0xee07c.l,a1                    | +06e
        move.l  a1,0x7c(a0)                     | +074
        lea     Pillbox_Init_Right_0868a6(pc),a1 | +078
        jsr     0x4ae.l                         | +07c
        jsr     0x5dd22.l                       | +082
        addi.w  #0xa8,0x22(a0)                  | +088
        addi.w  #0x50,0x24(a0)                  | +08e
        move.w  #0x60,0x38(a0)                  | +094
        lea     0x2ed760.l,a1                   | +09a
        move.l  a1,0x74(a0)                     | +0a0
        move.w  #0x2,0x72(a0)                   | +0a4
        lea     0xee092.l,a1                    | +0aa
        move.l  a1,0x7c(a0)                     | +0b0
        lea     Crate_Init_086cd0(pc),a1        | +0b4
        jsr     0x4ae.l                         | +0b8
        jsr     0x5dd22.l                       | +0be
        addi.w  #0x30,0x22(a0)                  | +0c4
        addi.w  #0x8,0x24(a0)                   | +0ca
        lea     0x2ed774.l,a1                   | +0d0
        move.l  a1,0x74(a0)                     | +0d6
        move.w  #0x3,0x72(a0)                   | +0da
        lea     Crate_Init_086cd0(pc),a1        | +0e0
        jsr     0x4ae.l                         | +0e4
        jsr     0x5dd22.l                       | +0ea
        addi.w  #0xa8,0x22(a0)                  | +0f0
        addi.w  #0x8,0x24(a0)                   | +0f6
        lea     0x2ed788.l,a1                   | +0fc
        move.l  a1,0x74(a0)                     | +102
        move.w  #0x4,0x72(a0)                   | +106
        rts                                     | +10c

| ----------------------------------------------------------------------------
|  Fort_SpawnChildren_V1_0885b4  @ $0885B4  (270 B)
| ----------------------------------------------------------------------------
        .section .text.Fort_SpawnChildren_V1_0885b4, "ax", @progbits
        .global Fort_SpawnChildren_V1_0885b4
Fort_SpawnChildren_V1_0885b4:
        lea     Pillbox_Init_Right_0868a6(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd22.l                       | +00a
        addi.w  #0x28,0x22(a0)                  | +010
        addi.w  #0x40,0x24(a0)                  | +016
        move.w  #0x70,0x38(a0)                  | +01c
        lea     0x2ed79c.l,a1                   | +022
        move.l  a1,0x74(a0)                     | +028
        move.w  #0x0,0x72(a0)                   | +02c
        lea     0xee0a8.l,a1                    | +032
        move.l  a1,0x7c(a0)                     | +038
        lea     Pillbox_Init_08686a(pc),a1      | +03c
        jsr     0x4ae.l                         | +040
        jsr     0x5dd22.l                       | +046
        addi.w  #0x68,0x22(a0)                  | +04c
        addi.w  #0x40,0x24(a0)                  | +052
        move.w  #0x70,0x38(a0)                  | +058
        lea     0x2ed7b0.l,a1                   | +05e
        move.l  a1,0x74(a0)                     | +064
        move.w  #0x1,0x72(a0)                   | +068
        lea     0xee0be.l,a1                    | +06e
        move.l  a1,0x7c(a0)                     | +074
        lea     Pillbox_Init_Right_0868a6(pc),a1 | +078
        jsr     0x4ae.l                         | +07c
        jsr     0x5dd22.l                       | +082
        addi.w  #0xa8,0x22(a0)                  | +088
        addi.w  #0x40,0x24(a0)                  | +08e
        move.w  #0x70,0x38(a0)                  | +094
        lea     0x2ed7c4.l,a1                   | +09a
        move.l  a1,0x74(a0)                     | +0a0
        move.w  #0x2,0x72(a0)                   | +0a4
        lea     0xee0d4.l,a1                    | +0aa
        move.l  a1,0x7c(a0)                     | +0b0
        lea     Crate_Init_V1_086ce4(pc),a1 | +0b4
        jsr     0x4ae.l                         | +0b8
        jsr     0x5dd22.l                       | +0be
        addi.w  #0x28,0x22(a0)                  | +0c4
        addi.w  #0x0,0x24(a0)                   | +0ca
        lea     0x2ed7d8.l,a1                   | +0d0
        move.l  a1,0x74(a0)                     | +0d6
        move.w  #0x3,0x72(a0)                   | +0da
        lea     Crate_Init_V1_086ce4(pc),a1 | +0e0
        jsr     0x4ae.l                         | +0e4
        jsr     0x5dd22.l                       | +0ea
        addi.w  #0xa0,0x22(a0)                  | +0f0
        addi.w  #0x0,0x24(a0)                   | +0f6
        lea     0x2ed7ec.l,a1                   | +0fc
        move.l  a1,0x74(a0)                     | +102
        move.w  #0x4,0x72(a0)                   | +106
        rts                                     | +10c

| ----------------------------------------------------------------------------
|  Fort_SpawnChildren_V2_0886c2  @ $0886C2  (270 B)
| ----------------------------------------------------------------------------
        .section .text.Fort_SpawnChildren_V2_0886c2, "ax", @progbits
        .global Fort_SpawnChildren_V2_0886c2
Fort_SpawnChildren_V2_0886c2:
        lea     Pillbox_Init_RightUpper_0868ba(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd22.l                       | +00a
        addi.w  #0x28,0x22(a0)                  | +010
        addi.w  #0x50,0x24(a0)                  | +016
        move.w  #0x60,0x38(a0)                  | +01c
        lea     0x2ed800.l,a1                   | +022
        move.l  a1,0x74(a0)                     | +028
        move.w  #0x0,0x72(a0)                   | +02c
        lea     0xee0ea.l,a1                    | +032
        move.l  a1,0x7c(a0)                     | +038
        lea     Pillbox_Init_RightUpper_0868ba(pc),a1 | +03c
        jsr     0x4ae.l                         | +040
        jsr     0x5dd22.l                       | +046
        addi.w  #0x68,0x22(a0)                  | +04c
        addi.w  #0x50,0x24(a0)                  | +052
        move.w  #0x60,0x38(a0)                  | +058
        lea     0x2ed814.l,a1                   | +05e
        move.l  a1,0x74(a0)                     | +064
        move.w  #0x1,0x72(a0)                   | +068
        lea     0xee100.l,a1                    | +06e
        move.l  a1,0x7c(a0)                     | +074
        lea     Pillbox_Init_RightUpper_0868ba(pc),a1 | +078
        jsr     0x4ae.l                         | +07c
        jsr     0x5dd22.l                       | +082
        addi.w  #0xa8,0x22(a0)                  | +088
        addi.w  #0x50,0x24(a0)                  | +08e
        move.w  #0x60,0x38(a0)                  | +094
        lea     0x2ed828.l,a1                   | +09a
        move.l  a1,0x74(a0)                     | +0a0
        move.w  #0x2,0x72(a0)                   | +0a4
        lea     0xee116.l,a1                    | +0aa
        move.l  a1,0x7c(a0)                     | +0b0
        lea     Crate_Init_V2_086cf8(pc),a1 | +0b4
        jsr     0x4ae.l                         | +0b8
        jsr     0x5dd22.l                       | +0be
        addi.w  #0x28,0x22(a0)                  | +0c4
        addi.w  #0x8,0x24(a0)                   | +0ca
        lea     0x2ed83c.l,a1                   | +0d0
        move.l  a1,0x74(a0)                     | +0d6
        move.w  #0x3,0x72(a0)                   | +0da
        lea     Crate_Init_V2_086cf8(pc),a1 | +0e0
        jsr     0x4ae.l                         | +0e4
        jsr     0x5dd22.l                       | +0ea
        addi.w  #0xa0,0x22(a0)                  | +0f0
        addi.w  #0x8,0x24(a0)                   | +0f6
        lea     0x2ed850.l,a1                   | +0fc
        move.l  a1,0x74(a0)                     | +102
        move.w  #0x4,0x72(a0)                   | +106
        rts                                     | +10c

| ----------------------------------------------------------------------------
|  Fort_SpawnChildren_V3_0887d0  @ $0887D0  (330 B)
| ----------------------------------------------------------------------------
        .section .text.Fort_SpawnChildren_V3_0887d0, "ax", @progbits
        .global Fort_SpawnChildren_V3_0887d0
Fort_SpawnChildren_V3_0887d0:
        lea     Pillbox_Init_Lower_086892(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd22.l                       | +00a
        addi.w  #0x18,0x22(a0)                  | +010
        addi.w  #0x40,0x24(a0)                  | +016
        move.w  #0x70,0x38(a0)                  | +01c
        lea     0x2ed88c.l,a1                   | +022
        move.l  a1,0x74(a0)                     | +028
        move.w  #0x0,0x72(a0)                   | +02c
        lea     0xee12c.l,a1                    | +032
        move.l  a1,0x7c(a0)                     | +038
        lea     Pillbox_Init_Right_0868a6(pc),a1 | +03c
        jsr     0x4ae.l                         | +040
        jsr     0x5dd22.l                       | +046
        addi.w  #0x48,0x22(a0)                  | +04c
        addi.w  #0x40,0x24(a0)                  | +052
        move.w  #0x70,0x38(a0)                  | +058
        lea     0x2ed8a0.l,a1                   | +05e
        move.l  a1,0x74(a0)                     | +064
        move.w  #0x1,0x72(a0)                   | +068
        lea     0xee130.l,a1                    | +06e
        move.l  a1,0x7c(a0)                     | +074
        lea     Pillbox_Init_Right_0868a6(pc),a1 | +078
        jsr     0x4ae.l                         | +07c
        jsr     0x5dd22.l                       | +082
        addi.w  #0x78,0x22(a0)                  | +088
        addi.w  #0x40,0x24(a0)                  | +08e
        move.w  #0x70,0x38(a0)                  | +094
        lea     0x2ed8b4.l,a1                   | +09a
        move.l  a1,0x74(a0)                     | +0a0
        move.w  #0x2,0x72(a0)                   | +0a4
        lea     0xee146.l,a1                    | +0aa
        move.l  a1,0x7c(a0)                     | +0b0
        lea     Pillbox_Init_Right_0868a6(pc),a1 | +0b4
        jsr     0x4ae.l                         | +0b8
        jsr     0x5dd22.l                       | +0be
        addi.w  #0xa8,0x22(a0)                  | +0c4
        addi.w  #0x40,0x24(a0)                  | +0ca
        move.w  #0x70,0x38(a0)                  | +0d0
        lea     0x2ed8c8.l,a1                   | +0d6
        move.l  a1,0x74(a0)                     | +0dc
        move.w  #0x3,0x72(a0)                   | +0e0
        lea     0xee15c.l,a1                    | +0e6
        move.l  a1,0x7c(a0)                     | +0ec
        lea     Crate_Init_V3_086d0c(pc),a1 | +0f0
        jsr     0x4ae.l                         | +0f4
        jsr     0x5dd22.l                       | +0fa
        addi.w  #0x28,0x22(a0)                  | +100
        addi.w  #0x8,0x24(a0)                   | +106
        lea     0x2ed8dc.l,a1                   | +10c
        move.l  a1,0x74(a0)                     | +112
        move.w  #0x4,0x72(a0)                   | +116
        lea     Crate_Init_V3_086d0c(pc),a1 | +11c
        jsr     0x4ae.l                         | +120
        jsr     0x5dd22.l                       | +126
        addi.w  #0xa0,0x22(a0)                  | +12c
        addi.w  #0x8,0x24(a0)                   | +132
        lea     0x2ed8f0.l,a1                   | +138
        move.l  a1,0x74(a0)                     | +13e
        move.w  #0x5,0x72(a0)                   | +142
        rts                                     | +148

| ----------------------------------------------------------------------------
|  Fort_SpawnChildren_V4_08891a  @ $08891A  (270 B)
| ----------------------------------------------------------------------------
        .section .text.Fort_SpawnChildren_V4_08891a, "ax", @progbits
        .global Fort_SpawnChildren_V4_08891a
Fort_SpawnChildren_V4_08891a:
        lea     Pillbox_Init_RightUpper_0868ba(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd22.l                       | +00a
        addi.w  #0x28,0x22(a0)                  | +010
        addi.w  #0x50,0x24(a0)                  | +016
        move.w  #0x60,0x38(a0)                  | +01c
        lea     0x2ed904.l,a1                   | +022
        move.l  a1,0x74(a0)                     | +028
        move.w  #0x0,0x72(a0)                   | +02c
        lea     0xee172.l,a1                    | +032
        move.l  a1,0x7c(a0)                     | +038
        lea     Pillbox_Init_RightUpper_0868ba(pc),a1 | +03c
        jsr     0x4ae.l                         | +040
        jsr     0x5dd22.l                       | +046
        addi.w  #0x68,0x22(a0)                  | +04c
        addi.w  #0x50,0x24(a0)                  | +052
        move.w  #0x60,0x38(a0)                  | +058
        lea     0x2ed918.l,a1                   | +05e
        move.l  a1,0x74(a0)                     | +064
        move.w  #0x1,0x72(a0)                   | +068
        lea     0xee188.l,a1                    | +06e
        move.l  a1,0x7c(a0)                     | +074
        lea     Pillbox_Init_RightUpper_0868ba(pc),a1 | +078
        jsr     0x4ae.l                         | +07c
        jsr     0x5dd22.l                       | +082
        addi.w  #0xa8,0x22(a0)                  | +088
        addi.w  #0x50,0x24(a0)                  | +08e
        move.w  #0x60,0x38(a0)                  | +094
        lea     0x2ed92c.l,a1                   | +09a
        move.l  a1,0x74(a0)                     | +0a0
        move.w  #0x2,0x72(a0)                   | +0a4
        lea     0xee19e.l,a1                    | +0aa
        move.l  a1,0x7c(a0)                     | +0b0
        lea     Crate_Init_V4_086d20(pc),a1 | +0b4
        jsr     0x4ae.l                         | +0b8
        jsr     0x5dd22.l                       | +0be
        addi.w  #0x28,0x22(a0)                  | +0c4
        addi.w  #0x0,0x24(a0)                   | +0ca
        lea     0x2ed940.l,a1                   | +0d0
        move.l  a1,0x74(a0)                     | +0d6
        move.w  #0x3,0x72(a0)                   | +0da
        lea     Crate_Init_V4_086d20(pc),a1 | +0e0
        jsr     0x4ae.l                         | +0e4
        jsr     0x5dd22.l                       | +0ea
        addi.w  #0x98,0x22(a0)                  | +0f0
        addi.w  #0x0,0x24(a0)                   | +0f6
        lea     0x2ed954.l,a1                   | +0fc
        move.l  a1,0x74(a0)                     | +102
        move.w  #0x4,0x72(a0)                   | +106
        rts                                     | +10c

| ----------------------------------------------------------------------------
|  Fort_BlitWreck_088a28  @ $088A28  (46 B)
| ----------------------------------------------------------------------------
        .section .text.Fort_BlitWreck_088a28, "ax", @progbits
        .global Fort_BlitWreck_088a28
Fort_BlitWreck_088a28:
        cmpi.b  #0x0,0x20(a6)                   | +000
        bne.w   Sub_00088A64                    | +006  -> $088A64 (hueco futuro, defsym forward)
        lea     0x2edaa8.l,a2                   | +00a
        jsr     0x5022a.l                       | +010
        lea     0x2edabc.l,a2                   | +016
        jsr     0x5022a.l                       | +01c
        lea     0x2edad0.l,a2                   | +022
        jsr     0x5022a.l                       | +028
