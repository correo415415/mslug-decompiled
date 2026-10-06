| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave GGGG — props destructibles del escenario: carteles, muros, casas,
|              chozas, torres, portones; escombros, drops y parpadeo del fix
|  Región: $0527BA..$0539E2  (4,648 B, 27 entradas, 1 huecos)
| ============================================================================
|
|  A. RESUMEN
|  ----------
|  Región de 4,648 B (27 entradas): handlers de los objetos de escenario
|  destructibles (templates de spawn de misión referenciados desde las
|  listas de $096Cxx..$0975xx y $198070) y sus efectos secundarios. Todos
|  siguen el mismo patrón:
|
|    init:  a1 = +$3C (template) -> $2942A (copia pos/params), snd $236E,
|           +$70 puntos, +$66 HP, +$32/+$33 = $FF, +$3A = 0, sprite $28CD4
|    loop:  $2783A (scroll) ; $28D70 (anim) ; $2870A (impacto recibido?)
|           -> flash $5E770 con tabla $5E766, bclr +$13 bit3 ;
|           $28758 (HP agotada?) -> música $2352 $10xx, escombros
|           $77C7E con listas $297Fxx/$2980xx, puntos $51A28, siguiente
|           estado ; $4FA70 (fuera de pantalla?) -> $518 (liberar).
|
|   1. Prop_Breakable2Stage_0527ba: dos fases (+$21 0/1/2); al romperse la
|      1a suelta 1..4 PropDrop_Item ($5EA1C) y re-arma con HP $1E; la 2a
|      llama a $53EBA/$53E78 (hueco futuro) y libera.
|   2. Prop_OneShotAnim_0528bc: anim única hasta que $28D70 devuelve C=0.
|   3. Prop_Sign_052902 -> Prop_SignBroken_052986 -> Prop_SignWreck_0529ca
|      (música $1029 al caer, puntos $500).
|   4. Prop_Wall_052a26 -> Prop_WallStage2_052af4: cada impacto suelta
|      PropDebris_Chunk ($6FE + $5DD02), rota +$47 (0..3), música $10BB,
|      puntos $10 (`move.l #$10,d0` sin optimizar a moveq).
|   5. Prop_Large_052b8a: HP $64, hijo $77228 (+8 px, +$98=$81, +$99=$75),
|      al romperse spawnea $77E10 (humo) y, si $106F28 bit0,
|      PropDebris_Flying; música $1024 -> Prop_LargeWreck_052ca0 (puntos
|      $1000).
|   6. Prop_Explosive_052cea: HP $14, al romperse música $1076, escombros,
|      instala tabla de ataque $29874A (daña al jugador) y pasa a
|      Prop_ExplosiveWreck_052dce (puntos $500).
|   7. Prop_HouseVariants_052e20 (684 B): 7 entradas (cada $24 B) que
|      eligen la lista de escombros +$80 ($2983F0..$298454) y el sprite;
|      dos variantes spawnean DuckTrigger_SpawnPairLeft_038f48/$38F14
|      (pareja de patos) y ejecutan la StateMachine $5022A directamente.
|      Loop común en +$1A0: al romperse música $1035, puntos $300, patos,
|      $434DC, 3 listas de escombros, StateMachineRun.
|   8. Prop_HutVariants_0530cc (570 B): 10 entradas (+$21 = 0..4 por
|      pares, listas $298468..$29851C); al romperse $53E0C (hueco), puntos
|      $500, 2 listas de escombros.
|   9. Prop_Tower_053306 -> Prop_TowerStage2_0533da -> Prop_TowerWreck_053482:
|      hitbox +$60 = $2988EA, registra en $4429E (MissionDriver, slot $74)
|      y limpia $10E39A; música $1023/$1030; puntos $2000.
|  10. Prop_Gate_0534d8 -> Prop_GateStage2_05359e -> Prop_GateWreck_053652:
|      HP $28, puntos $C0; solo activo con X en -$20..$90; variantes por
|      +$38 == 0 ($53F2E/$53EE2/$53F54/$53F08, hueco futuro).
|  11. FixBlink_PhaseA_0536ac / PhaseB_05370a: escriben tiles del fix
|      layer ($2C26: col $19/$1B, fila $C..$F) alternando cada $B4/$78
|      frames hasta que la cámara $106F5C >= $140.
|  12. PropDrop_Item_053768: ítem soltado (snd $A, $53E9C, pos del padre
|      +$54/+$56, vel X aleatoria) -> handler $6DCE0.
|  13. PropDebris_Chunk_0537ea / ChunkAlt_05379c: cascote con gravedad
|      -$10, vel aleatoria, sprite $2DE4B0, 40 frames, snd $29/$B.
|  14. PropDebris_Flying_053894 / FlyingAlt_053886: trozo lanzado (snd
|      $9E/$AD) que hereda 2x la velocidad del atacante (+$50 -> +$28/+$2A)
|      salvo si +$58 == 4; rebota a mitad de vel Y y spawnea humo $77E10.
|  15. Prop_Indestructible_053964: HP $7FFF, hijo $539F0 (hueco), snd
|      $1AE; solo reacciona con flash ($53D80).
|
|  B. EVIDENCIAS
|  -------------
|  - Las direcciones $528BC/$52902/$52A26/$52B8A/$52CEA/$52E20/$530CC/
|    $53306/$53964 aparecen como longs en $096CB4..$096F0C y $09751A..
|    $097556 (listas de spawn de misión, zona DATA) -> templates.
|  - Patrón idéntico de init/loop/ruptura en los 10 props; los sprites
|    $2971xx..$2977xx son consecutivos (banco de escenario).
|  - $2C26 con (col, fila, ancho, alto) y la cámara $106F5C: fix layer.
|  - `move.l #$10,d0` codificado como $203C: SNK no usaba moveq aquí.
|
|  C. CAMPOS DEL OBJETO (a6)
|  -------------------------
|  +$00 handler  +$0C padre  +$13 bit0 roto, bit3 impacto  +$21 fase
|  +$22/+$24 x/y  +$28/+$2A vel  +$2E grav  +$32/+$33  +$36 vel X base
|  +$38 variante/prio  +$3A facing  +$3C template  +$47 índice de cascote
|  +$48 cb colisión  +$4C tabla de ataque  +$54/+$56 pos de spawn
|  +$58 id de impacto  +$60 hitbox  +$66 HP  +$70 puntos  +$72 contador
|  +$74 slot de misión  +$75  +$80 lista de escombros  +$98/+$99 params hijo
|
|  D. HELPERS EXTERNOS
|  -------------------
|  $4AE/$6FE alloc  $518 free  $236E snd  $2352 música  $2C26 fix tile
|  $267E2  $2783A scroll  $27CEE  $27D50  $2870A impacto  $28758 HP
|  $283CA/$283D8 attack  $28998  $28CD4 sprite  $28D70 anim  $2942A
|  copiar template  $38F14/$38F48 patos  $434DC  $43FAC  $4429E
|  MissionDriver slot  $4FA70 fuera de pantalla  $5022A StateMachineRun
|  $51A28 puntos  $5DD02/$5DD22 copiar pos  $5DD56  $5E722  $5E766/$5E770
|  flash  $5EA1C rand acotado  $6DCE0 ítem  $77228/$77E10 hijos
|  $77C7E escombros  $53D80..$53F54 (hueco $0539F0..$053F96, siguiente)
|
|  E. HIPÓTESIS
|  ------------
|  - Casa/choza/torre/portón son nombres por tamaño de HP, puntos y
|    número de listas de escombros; confirmar con los sprites de la
|    misión 1 (aldea) y misión 3 (fuerte).
|  - $5E770 = parpadeo blanco al recibir daño (tabla de paletas $5E766).
|
|  F. SIGUIENTE
|  ------------
|  $0539F0..$053F96 (helpers de props: $53D80/$53E0C/$53E78/$53E9C/$53EBA/
|  $53EE2/$53F08/$53F2E/$53F54 + hijo $539F0), $0478FC..$048A3C,
|  $04AC3A..$04BB8E.
|
|  Verificación: cada sección .text.<Sym> se coloca en su dirección CPU
|  absoluta y reensambla byte-exacta contra build/mslug_prom.bin
|  (MD5 816b3f74c76b3373993407615f1850fe).
| ============================================================================

        .text

| ----------------------------------------------------------------------------
|  Prop_Breakable2Stage_0527ba  @ $0527BA  (258 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_Breakable2Stage_0527ba, "ax", @progbits
        .global Prop_Breakable2Stage_0527ba
Prop_Breakable2Stage_0527ba:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x40,0x70(a6)                  | +00a
        move.w  #0xa,0x66(a6)                   | +010
        move.b  #0x0,0x21(a6)                   | +016
        lea     0x297100.l,a0                   | +01c
        jsr     0x28cd4.l                       | +022
        cmpi.w  #0x0,0x38(a6)                   | +028
        beq.w   .L052802                        | +02e
        cmpi.w  #0x6,0x38(a6)                   | +032
        beq.w   .L052802                        | +038
        lea     0x297116.l,a0                   | +03c
        jsr     0x28cd4.l                       | +042
.L052802:
        lea     .L052808(pc),a1                 | +048
        move.l  a1,(a6)                         | +04c
.L052808:
        jsr     0x2783a.l                       | +04e
        jsr     0x28d70.l                       | +054
        jsr     0x2870a.l                       | +05a
        bcc.w   .L052844                        | +060
        lea     0x5e766.l,a0                    | +064
        jsr     0x5e770.l                       | +06a
        bclr    #0x3,0x13(a6)                   | +070
        cmpi.b  #0x2,0x58(a6)                   | +076
        bne.w   .L052844                        | +07c
        move.b  #0x1,0x21(a6)                   | +080
        bra.w   .L05284e                        | +086
.L052844:
        jsr     0x28758.l                       | +08a
        bcc.w   .L0528aa                        | +090
.L05284e:
        bclr    #0x0,0x13(a6)                   | +094
        move.w  #0x1e,0x66(a6)                  | +09a
        cmpi.b  #0x1,0x21(a6)                   | +0a0
        bne.w   .L052894                        | +0a6
        move.w  #0x1027,d0                      | +0aa
        jsr     0x2352.l                        | +0ae
        move.w  #0x3,d0                         | +0b4
        jsr     0x5ea1c.l                       | +0b8
        addq.w  #0x1,d0                         | +0be
        move.w  d0,0x72(a6)                     | +0c0
.L05287e:
        lea     PropDrop_Item_053768(pc),a1     | +0c4
        jsr     0x4ae.l                         | +0c8
        jsr     0x5dd22.l                       | +0ce
        subq.w  #0x1,0x72(a6)                   | +0d4
        bne.b   .L05287e                        | +0d8
.L052894:
        jsr     Sub_00053EBA(pc)                | +0da
        jsr     Sub_00053E78(pc)                | +0de
        addq.b  #0x1,0x21(a6)                   | +0e2
        cmpi.b  #0x2,0x21(a6)                   | +0e6
        beq.w   .L0528b4                        | +0ec
.L0528aa:
        jsr     0x4fa70.l                       | +0f0
        bcc.w   .L0528ba                        | +0f6
.L0528b4:
        jmp     0x518.l                         | +0fa
.L0528ba:
        rts                                     | +100

| ----------------------------------------------------------------------------
|  Prop_OneShotAnim_0528bc  @ $0528BC  (70 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_OneShotAnim_0528bc, "ax", @progbits
        .global Prop_OneShotAnim_0528bc
Prop_OneShotAnim_0528bc:
        move.w  #0x6d,d1                        | +000
        jsr     0x236e.l                        | +004
        move.b  #0xff,0x32(a6)                  | +00a
        move.b  #0xff,0x33(a6)                  | +010
        move.b  #0x0,0x3a(a6)                   | +016
        lea     0x29712c.l,a0                   | +01c
        jsr     0x28cd4.l                       | +022
        lea     .L0528ea(pc),a1                 | +028
        move.l  a1,(a6)                         | +02c
.L0528ea:
        jsr     0x2783a.l                       | +02e
        jsr     0x28d70.l                       | +034
        bcc.w   .L052900                        | +03a
        jmp     0x518.l                         | +03e
.L052900:
        rts                                     | +044

| ----------------------------------------------------------------------------
|  Prop_Sign_052902  @ $052902  (132 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_Sign_052902, "ax", @progbits
        .global Prop_Sign_052902
Prop_Sign_052902:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x33,d1                        | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0x20,0x70(a6)                  | +014
        move.b  #0xff,0x32(a6)                  | +01a
        move.b  #0xff,0x33(a6)                  | +020
        move.b  #0x0,0x3a(a6)                   | +026
        move.w  #0x1,0x66(a6)                   | +02c
        lea     0x297174.l,a0                   | +032
        jsr     0x28cd4.l                       | +038
        lea     .L052946(pc),a1                 | +03e
        move.l  a1,(a6)                         | +042
.L052946:
        jsr     0x2783a.l                       | +044
        jsr     0x28d70.l                       | +04a
        jsr     0x2870a.l                       | +050
        bcc.w   .L052974                        | +056
        bclr    #0x3,0x13(a6)                   | +05a
        move.l  #0x500,d0                       | +060
        jsr     0x51a28.l                       | +066
        lea     Prop_SignBroken_052986(pc),a1   | +06c
        move.l  a1,(a6)                         | +070
.L052974:
        jsr     0x4fa70.l                       | +072
        bcc.w   .L052984                        | +078
        jmp     0x518.l                         | +07c
.L052984:
        rts                                     | +082

| ----------------------------------------------------------------------------
|  Prop_SignBroken_052986  @ $052986  (68 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_SignBroken_052986, "ax", @progbits
        .global Prop_SignBroken_052986
Prop_SignBroken_052986:
        move.w  #0x1029,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x29718a.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L0529a2(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L0529a2:
        jsr     0x2783a.l                       | +01c
        jsr     0x28d70.l                       | +022
        bcc.w   .L0529b8                        | +028
        lea     Prop_SignWreck_0529ca(pc),a1    | +02c
        move.l  a1,(a6)                         | +030
.L0529b8:
        jsr     0x4fa70.l                       | +032
        bcc.w   .L0529c8                        | +038
        jmp     0x518.l                         | +03c
.L0529c8:
        rts                                     | +042

| ----------------------------------------------------------------------------
|  Prop_SignWreck_0529ca  @ $0529CA  (92 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_SignWreck_0529ca, "ax", @progbits
        .global Prop_SignWreck_0529ca
Prop_SignWreck_0529ca:
        bclr    #0x3,0x13(a6)                   | +000
        lea     0x2971ee.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     .L0529e2(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L0529e2:
        jsr     0x2783a.l                       | +018
        jsr     0x28d70.l                       | +01e
        jsr     0x2870a.l                       | +024
        bcc.w   .L052a0e                        | +02a
        move.w  #0x1029,d0                      | +02e
        jsr     0x2352.l                        | +032
        lea     0x2971ee.l,a0                   | +038
        jsr     0x28cd4.l                       | +03e
.L052a0e:
        bclr    #0x3,0x13(a6)                   | +044
        jsr     0x4fa70.l                       | +04a
        bcc.w   .L052a24                        | +050
        jmp     0x518.l                         | +054
.L052a24:
        rts                                     | +05a

| ----------------------------------------------------------------------------
|  Prop_Wall_052a26  @ $052A26  (206 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_Wall_052a26, "ax", @progbits
        .global Prop_Wall_052a26
Prop_Wall_052a26:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x32,d1                        | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0x20,0x70(a6)                  | +014
        move.b  #0xff,0x32(a6)                  | +01a
        move.b  #0xff,0x33(a6)                  | +020
        move.b  #0x0,0x3a(a6)                   | +026
        move.w  #0x3c,0x66(a6)                  | +02c
        lea     0x2973b6.l,a0                   | +032
        jsr     0x28cd4.l                       | +038
        lea     .L052a6a(pc),a1                 | +03e
        move.l  a1,(a6)                         | +042
.L052a6a:
        jsr     0x2783a.l                       | +044
        jsr     0x28d70.l                       | +04a
        jsr     0x2870a.l                       | +050
        bcc.w   .L052acc                        | +056
        bclr    #0x3,0x13(a6)                   | +05a
        move.w  #0x10bb,d0                      | +060
        jsr     0x2352.l                        | +064
        addq.b  #0x1,0x47(a6)                   | +06a
        andi.b  #0x3,0x47(a6)                   | +06e
        lea     PropDebris_Chunk_0537ea(pc),a1  | +074
        jsr     0x6fe.l                         | +078
        jsr     0x5dd02.l                       | +07e
        .dc.w   0x203c,0x0000,0x0010         | +084  move.l #$10, d0 (sin moveq)
        jsr     0x51a28.l                       | +08a
        btst    #0x3,0x13(a6)                   | +090
        bne.w   .L052acc                        | +096
        lea     0x2973cc.l,a0                   | +09a
        jsr     0x28cd4.l                       | +0a0
.L052acc:
        jsr     0x28758.l                       | +0a6
        bcc.w   .L052ae2                        | +0ac
        bclr    #0x0,0x13(a6)                   | +0b0
        lea     Prop_WallStage2_052af4(pc),a1   | +0b6
        move.l  a1,(a6)                         | +0ba
.L052ae2:
        jsr     0x4fa70.l                       | +0bc
        bcc.w   .L052af2                        | +0c2
        jmp     0x518.l                         | +0c6
.L052af2:
        rts                                     | +0cc

| ----------------------------------------------------------------------------
|  Prop_WallStage2_052af4  @ $052AF4  (150 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_WallStage2_052af4, "ax", @progbits
        .global Prop_WallStage2_052af4
Prop_WallStage2_052af4:
        move.w  #0x3c,0x66(a6)                  | +000
        lea     0x297458.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     .L052b0c(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L052b0c:
        jsr     0x2783a.l                       | +018
        jsr     0x28d70.l                       | +01e
        btst    #0x0,0x13(a6)                   | +024
        bne.w   .L052b78                        | +02a
        jsr     0x2870a.l                       | +02e
        bcc.w   .L052b78                        | +034
        bclr    #0x3,0x13(a6)                   | +038
        move.w  #0x10bb,d0                      | +03e
        jsr     0x2352.l                        | +042
        addq.b  #0x1,0x47(a6)                   | +048
        andi.b  #0x3,0x47(a6)                   | +04c
        lea     PropDebris_Chunk_0537ea(pc),a1  | +052
        jsr     0x6fe.l                         | +056
        jsr     0x5dd02.l                       | +05c
        .dc.w   0x203c,0x0000,0x0010         | +062  move.l #$10, d0 (sin moveq)
        jsr     0x51a28.l                       | +068
        btst    #0x3,0x13(a6)                   | +06e
        bne.w   .L052b78                        | +074
        lea     0x297474.l,a0                   | +078
        jsr     0x28cd4.l                       | +07e
.L052b78:
        jsr     0x4fa70.l                       | +084
        bcc.w   .L052b88                        | +08a
        jmp     0x518.l                         | +08e
.L052b88:
        rts                                     | +094

| ----------------------------------------------------------------------------
|  Prop_Large_052b8a  @ $052B8A  (278 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_Large_052b8a, "ax", @progbits
        .global Prop_Large_052b8a
Prop_Large_052b8a:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x31,d1                        | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0x10,0x70(a6)                  | +014
        move.b  #0xff,0x32(a6)                  | +01a
        move.b  #0xff,0x33(a6)                  | +020
        move.b  #0x0,0x3a(a6)                   | +026
        move.w  #0x64,0x66(a6)                  | +02c
        move.b  #0x0,0x75(a6)                   | +032
        lea     0x77228.l,a1                    | +038
        jsr     0x4ae.l                         | +03e
        jsr     0x5dd22.l                       | +044
        addi.w  #0x8,0x22(a0)                   | +04a
        move.b  #0x81,0x98(a0)                  | +050
        move.b  #0x75,0x99(a0)                  | +056
        lea     0x297500.l,a0                   | +05c
        jsr     0x28cd4.l                       | +062
        lea     .L052bf8(pc),a1                 | +068
        move.l  a1,(a6)                         | +06c
.L052bf8:
        jsr     0x2783a.l                       | +06e
        jsr     0x28d70.l                       | +074
        jsr     0x2870a.l                       | +07a
        bcc.w   .L052c5c                        | +080
        lea     0x5e766.l,a0                    | +084
        jsr     0x5e770.l                       | +08a
        bclr    #0x3,0x13(a6)                   | +090
        lea     0x77e10.l,a1                    | +096
        jsr     0x4ae.l                         | +09c
        jsr     0x5dd22.l                       | +0a2
        move.w  0x54(a6),0x22(a0)               | +0a8
        move.w  0x56(a6),0x24(a0)               | +0ae
        move.b  0x106f28.l,d0                   | +0b4
        btst    #0x0,d0                         | +0ba
        beq.w   .L052c5c                        | +0be
        lea     PropDebris_Flying_053894(pc),a1 | +0c2
        jsr     0x4ae.l                         | +0c6
        jsr     0x5dd22.l                       | +0cc
.L052c5c:
        jsr     0x28758.l                       | +0d2
        bcc.w   .L052c88                        | +0d8
        bclr    #0x0,0x13(a6)                   | +0dc
        move.w  #0x1024,d0                      | +0e2
        jsr     0x2352.l                        | +0e6
        lea     0x297fe4.l,a1                   | +0ec
        jsr     0x77c7e.l                       | +0f2
        lea     Prop_LargeWreck_052ca0(pc),a1   | +0f8
        move.l  a1,(a6)                         | +0fc
.L052c88:
        jsr     0x4fa70.l                       | +0fe
        bcc.w   .L052c9e                        | +104
        move.b  #0xff,0x75(a6)                  | +108
        jmp     0x518.l                         | +10e
.L052c9e:
        rts                                     | +114

| ----------------------------------------------------------------------------
|  Prop_LargeWreck_052ca0  @ $052CA0  (74 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_LargeWreck_052ca0, "ax", @progbits
        .global Prop_LargeWreck_052ca0
Prop_LargeWreck_052ca0:
        move.l  #0x1000,d0                      | +000
        jsr     0x51a28.l                       | +006
        lea     0xffff.w,a0                     | +00c
        move.l  a0,0x48(a6)                     | +010
        move.b  #0xff,0x75(a6)                  | +014
        lea     0x297516.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        lea     .L052ccc(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L052ccc:
        jsr     0x2783a.l                       | +02c
        jsr     0x28d70.l                       | +032
        jsr     0x4fa70.l                       | +038
        bcc.w   .L052ce8                        | +03e
        jmp     0x518.l                         | +042
.L052ce8:
        rts                                     | +048

| ----------------------------------------------------------------------------
|  Prop_Explosive_052cea  @ $052CEA  (228 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_Explosive_052cea, "ax", @progbits
        .global Prop_Explosive_052cea
Prop_Explosive_052cea:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x30,d1                        | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0x10,0x70(a6)                  | +014
        move.b  #0xff,0x32(a6)                  | +01a
        move.b  #0xff,0x33(a6)                  | +020
        move.b  #0x0,0x3a(a6)                   | +026
        move.w  #0x14,0x66(a6)                  | +02c
        lea     0x297526.l,a0                   | +032
        jsr     0x28cd4.l                       | +038
        lea     .L052d2e(pc),a1                 | +03e
        move.l  a1,(a6)                         | +042
.L052d2e:
        jsr     0x2783a.l                       | +044
        jsr     0x28d70.l                       | +04a
        jsr     0x2870a.l                       | +050
        bcc.w   .L052d74                        | +056
        lea     0x5e766.l,a0                    | +05a
        jsr     0x5e770.l                       | +060
        bclr    #0x3,0x13(a6)                   | +066
        lea     0x77e10.l,a1                    | +06c
        jsr     0x4ae.l                         | +072
        jsr     0x5dd22.l                       | +078
        move.w  0x54(a6),0x22(a0)               | +07e
        move.w  0x56(a6),0x24(a0)               | +084
.L052d74:
        jsr     0x28758.l                       | +08a
        bcc.w   .L052dbc                        | +090
        bclr    #0x0,0x13(a6)                   | +094
        move.w  #0x1076,d0                      | +09a
        jsr     0x2352.l                        | +09e
        lea     0x297fe4.l,a1                   | +0a4
        jsr     0x77c7e.l                       | +0aa
        lea     0x29874a.l,a0                   | +0b0
        move.l  a0,0x4c(a6)                     | +0b6
        jsr     0x283ca.l                       | +0ba
        jsr     0x283ca.l                       | +0c0
        jsr     0x283d8.l                       | +0c6
        lea     Prop_ExplosiveWreck_052dce(pc),a1 | +0cc
        move.l  a1,(a6)                         | +0d0
.L052dbc:
        jsr     0x4fa70.l                       | +0d2
        bcc.w   .L052dcc                        | +0d8
        jmp     0x518.l                         | +0dc
.L052dcc:
        rts                                     | +0e2

| ----------------------------------------------------------------------------
|  Prop_ExplosiveWreck_052dce  @ $052DCE  (82 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_ExplosiveWreck_052dce, "ax", @progbits
        .global Prop_ExplosiveWreck_052dce
Prop_ExplosiveWreck_052dce:
        move.l  #0x500,d0                       | +000
        jsr     0x51a28.l                       | +006
        lea     0xffff.w,a0                     | +00c
        move.l  a0,0x4c(a6)                     | +010
        jsr     0x283ca.l                       | +014
        lea     0xffff.w,a0                     | +01a
        move.l  a0,0x48(a6)                     | +01e
        lea     0x29753c.l,a0                   | +022
        jsr     0x28cd4.l                       | +028
        lea     .L052e02(pc),a1                 | +02e
        move.l  a1,(a6)                         | +032
.L052e02:
        jsr     0x2783a.l                       | +034
        jsr     0x28d70.l                       | +03a
        jsr     0x4fa70.l                       | +040
        bcc.w   .L052e1e                        | +046
        jmp     0x518.l                         | +04a
.L052e1e:
        rts                                     | +050

| ----------------------------------------------------------------------------
|  Prop_HouseVariants_052e20  @ $052E20  (684 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_HouseVariants_052e20, "ax", @progbits
        .global Prop_HouseVariants_052e20
Prop_HouseVariants_052e20:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        lea     0x2983f0.l,a2                   | +00a
        move.l  a2,0x80(a6)                     | +010
        lea     0x29754c.l,a0                   | +014
        jsr     0x28cd4.l                       | +01a
        bra.w   .L052fc0                        | +020
        movea.l 0x3c(a6),a1                     | +024
        jsr     0x2942a.l                       | +028
        lea     0x298404.l,a2                   | +02e
        move.l  a2,0x80(a6)                     | +034
        lea     0x29754c.l,a0                   | +038
        jsr     0x28cd4.l                       | +03e
        bra.w   .L052fc0                        | +044
        movea.l 0x3c(a6),a1                     | +048
        jsr     0x2942a.l                       | +04c
        lea     0x298418.l,a2                   | +052
        move.l  a2,0x80(a6)                     | +058
        lea     0x297562.l,a0                   | +05c
        jsr     0x28cd4.l                       | +062
        bra.w   .L052fc0                        | +068
        movea.l 0x3c(a6),a1                     | +06c
        jsr     0x2942a.l                       | +070
        lea     0x29842c.l,a2                   | +076
        move.l  a2,0x80(a6)                     | +07c
        lea     0x38f14.l,a1                    | +080
        jsr     0x4ae.l                         | +086
        jsr     0x5dd02.l                       | +08c
        addi.w  #0x7,0x22(a0)                   | +092
        addi.w  #0x20,0x24(a0)                  | +098
        move.b  #0x1,0x98(a0)                   | +09e
        lea     0x38f48.l,a1                    | +0a4
        jsr     0x4ae.l                         | +0aa
        jsr     0x5dd02.l                       | +0b0
        addi.w  #0x30,0x22(a0)                  | +0b6
        addi.w  #0x20,0x24(a0)                  | +0bc
        move.b  #0x1,0x98(a0)                   | +0c2
        lea     0x2986fc.l,a1                   | +0c8
        jsr     0x43fac.l                       | +0ce
        movea.l 0x80(a6),a2                     | +0d4
        jsr     0x5022a.l                       | +0d8
        jmp     0x518.l                         | +0de
        rts                                     | +0e4
        lea     0x297562.l,a0                   | +0e6
        jsr     0x28cd4.l                       | +0ec
        bra.w   .L052fc0                        | +0f2
        movea.l 0x3c(a6),a1                     | +0f6
        jsr     0x2942a.l                       | +0fa
        lea     0x298440.l,a2                   | +100
        move.l  a2,0x80(a6)                     | +106
        lea     0x297578.l,a0                   | +10a
        jsr     0x28cd4.l                       | +110
        bra.w   .L052fc0                        | +116
        movea.l 0x3c(a6),a1                     | +11a
        jsr     0x2942a.l                       | +11e
        lea     0x298454.l,a2                   | +124
        move.l  a2,0x80(a6)                     | +12a
        lea     0x38f14.l,a1                    | +12e
        jsr     0x4ae.l                         | +134
        jsr     0x5dd02.l                       | +13a
        addi.w  #0x7,0x22(a0)                   | +140
        addi.w  #0x20,0x24(a0)                  | +146
        move.b  #0x1,0x98(a0)                   | +14c
        lea     0x38f48.l,a1                    | +152
        jsr     0x4ae.l                         | +158
        jsr     0x5dd02.l                       | +15e
        addi.w  #0x30,0x22(a0)                  | +164
        addi.w  #0x20,0x24(a0)                  | +16a
        move.b  #0x1,0x98(a0)                   | +170
        lea     0x2986fc.l,a1                   | +176
        jsr     0x43fac.l                       | +17c
        movea.l 0x80(a6),a2                     | +182
        jsr     0x5022a.l                       | +186
        jmp     0x518.l                         | +18c
        rts                                     | +192
        lea     0x297578.l,a0                   | +194
        jsr     0x28cd4.l                       | +19a
.L052fc0:
        move.w  #0x40,0x70(a6)                  | +1a0
        move.w  #0x28,0x66(a6)                  | +1a6
        move.b  #0x0,0x21(a6)                   | +1ac
        lea     .L052fd8(pc),a1                 | +1b2
        move.l  a1,(a6)                         | +1b6
.L052fd8:
        jsr     0x2783a.l                       | +1b8
        jsr     0x28d70.l                       | +1be
        jsr     0x2870a.l                       | +1c4
        bcc.w   .L053000                        | +1ca
        lea     0x5e766.l,a0                    | +1ce
        jsr     0x5e770.l                       | +1d4
        bclr    #0x3,0x13(a6)                   | +1da
.L053000:
        jsr     0x28758.l                       | +1e0
        bcc.w   .L0530ba                        | +1e6
        move.w  #0x1035,d0                      | +1ea
        jsr     0x2352.l                        | +1ee
        lea     0xffff.w,a0                     | +1f4
        move.l  a0,0x48(a6)                     | +1f8
        move.l  #0x300,d0                       | +1fc
        jsr     0x51a28.l                       | +202
        lea     0x38f14.l,a1                    | +208
        jsr     0x4ae.l                         | +20e
        jsr     0x5dd02.l                       | +214
        addi.w  #0x7,0x22(a0)                   | +21a
        addi.w  #0x20,0x24(a0)                  | +220
        move.b  #0x1,0x98(a0)                   | +226
        lea     0x38f48.l,a1                    | +22c
        jsr     0x4ae.l                         | +232
        jsr     0x5dd02.l                       | +238
        addi.w  #0x30,0x22(a0)                  | +23e
        addi.w  #0x20,0x24(a0)                  | +244
        move.b  #0x1,0x98(a0)                   | +24a
        jsr     0x434dc.l                       | +250
        lea     0x2986fc.l,a1                   | +256
        jsr     0x43fac.l                       | +25c
        subi.w  #0x10,0x24(a6)                  | +262
        lea     0x297fc0.l,a1                   | +268
        jsr     0x77c7e.l                       | +26e
        lea     0x297fd2.l,a1                   | +274
        jsr     0x77c7e.l                       | +27a
        lea     0x298086.l,a1                   | +280
        jsr     0x77c7e.l                       | +286
        movea.l 0x80(a6),a2                     | +28c
        jsr     0x5022a.l                       | +290
        bra.w   .L0530c4                        | +296
.L0530ba:
        jsr     0x4fa70.l                       | +29a
        bcc.w   .L0530ca                        | +2a0
.L0530c4:
        jmp     0x518.l                         | +2a4
.L0530ca:
        rts                                     | +2aa

| ----------------------------------------------------------------------------
|  Prop_HutVariants_0530cc  @ $0530CC  (570 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_HutVariants_0530cc, "ax", @progbits
        .global Prop_HutVariants_0530cc
Prop_HutVariants_0530cc:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.b  #0x0,0x21(a6)                   | +00a
        lea     0x298468.l,a2                   | +010
        move.l  a2,0x80(a6)                     | +016
        lea     0x29758e.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        bra.w   .L05326c                        | +026
        movea.l 0x3c(a6),a1                     | +02a
        jsr     0x2942a.l                       | +02e
        move.b  #0x0,0x21(a6)                   | +034
        lea     0x29847c.l,a2                   | +03a
        move.l  a2,0x80(a6)                     | +040
        lea     0x29758e.l,a0                   | +044
        jsr     0x28cd4.l                       | +04a
        bra.w   .L05326c                        | +050
        movea.l 0x3c(a6),a1                     | +054
        jsr     0x2942a.l                       | +058
        move.b  #0x1,0x21(a6)                   | +05e
        lea     0x298490.l,a2                   | +064
        move.l  a2,0x80(a6)                     | +06a
        lea     0x2975a4.l,a0                   | +06e
        jsr     0x28cd4.l                       | +074
        bra.w   .L05326c                        | +07a
        movea.l 0x3c(a6),a1                     | +07e
        jsr     0x2942a.l                       | +082
        move.b  #0x1,0x21(a6)                   | +088
        lea     0x2984a4.l,a2                   | +08e
        move.l  a2,0x80(a6)                     | +094
        lea     0x2975a4.l,a0                   | +098
        jsr     0x28cd4.l                       | +09e
        bra.w   .L05326c                        | +0a4
        movea.l 0x3c(a6),a1                     | +0a8
        jsr     0x2942a.l                       | +0ac
        move.b  #0x2,0x21(a6)                   | +0b2
        lea     0x2984b8.l,a2                   | +0b8
        move.l  a2,0x80(a6)                     | +0be
        lea     0x2975ba.l,a0                   | +0c2
        jsr     0x28cd4.l                       | +0c8
        bra.w   .L05326c                        | +0ce
        movea.l 0x3c(a6),a1                     | +0d2
        jsr     0x2942a.l                       | +0d6
        move.b  #0x2,0x21(a6)                   | +0dc
        lea     0x2984cc.l,a2                   | +0e2
        move.l  a2,0x80(a6)                     | +0e8
        lea     0x2975ba.l,a0                   | +0ec
        jsr     0x28cd4.l                       | +0f2
        bra.w   .L05326c                        | +0f8
        movea.l 0x3c(a6),a1                     | +0fc
        jsr     0x2942a.l                       | +100
        move.b  #0x3,0x21(a6)                   | +106
        lea     0x2984e0.l,a2                   | +10c
        move.l  a2,0x80(a6)                     | +112
        lea     0x2975d0.l,a0                   | +116
        jsr     0x28cd4.l                       | +11c
        bra.w   .L05326c                        | +122
        movea.l 0x3c(a6),a1                     | +126
        jsr     0x2942a.l                       | +12a
        move.b  #0x3,0x21(a6)                   | +130
        lea     0x2984f4.l,a2                   | +136
        move.l  a2,0x80(a6)                     | +13c
        lea     0x2975d0.l,a0                   | +140
        jsr     0x28cd4.l                       | +146
        bra.w   .L05326c                        | +14c
        movea.l 0x3c(a6),a1                     | +150
        jsr     0x2942a.l                       | +154
        move.b  #0x4,0x21(a6)                   | +15a
        lea     0x298508.l,a2                   | +160
        move.l  a2,0x80(a6)                     | +166
        lea     0x2975e6.l,a0                   | +16a
        jsr     0x28cd4.l                       | +170
        bra.w   .L05326c                        | +176
        movea.l 0x3c(a6),a1                     | +17a
        jsr     0x2942a.l                       | +17e
        move.b  #0x4,0x21(a6)                   | +184
        lea     0x29851c.l,a2                   | +18a
        move.l  a2,0x80(a6)                     | +190
        lea     0x2975e6.l,a0                   | +194
        jsr     0x28cd4.l                       | +19a
.L05326c:
        move.w  #0x40,0x70(a6)                  | +1a0
        move.w  #0x28,0x66(a6)                  | +1a6
        lea     .L05327e(pc),a1                 | +1ac
        move.l  a1,(a6)                         | +1b0
.L05327e:
        jsr     0x2783a.l                       | +1b2
        jsr     0x28d70.l                       | +1b8
        jsr     0x2870a.l                       | +1be
        bcc.w   .L0532a6                        | +1c4
        lea     0x5e766.l,a0                    | +1c8
        jsr     0x5e770.l                       | +1ce
        bclr    #0x3,0x13(a6)                   | +1d4
.L0532a6:
        jsr     0x28758.l                       | +1da
        bcc.w   .L0532f4                        | +1e0
        jsr     Sub_00053E0C(pc)                | +1e4
        lea     0xffff.w,a0                     | +1e8
        move.l  a0,0x48(a6)                     | +1ec
        move.l  #0x500,d0                       | +1f0
        jsr     0x51a28.l                       | +1f6
        addi.w  #0x10,0x22(a6)                  | +1fc
        lea     0x297ff6.l,a1                   | +202
        jsr     0x77c7e.l                       | +208
        lea     0x298008.l,a1                   | +20e
        jsr     0x77c7e.l                       | +214
        movea.l 0x80(a6),a2                     | +21a
        jsr     0x5022a.l                       | +21e
        bra.w   .L0532fe                        | +224
.L0532f4:
        jsr     0x4fa70.l                       | +228
        bcc.w   .L053304                        | +22e
.L0532fe:
        jmp     0x518.l                         | +232
.L053304:
        rts                                     | +238

| ----------------------------------------------------------------------------
|  Prop_Tower_053306  @ $053306  (212 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_Tower_053306, "ax", @progbits
        .global Prop_Tower_053306
Prop_Tower_053306:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x36,d1                        | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0x40,0x70(a6)                  | +014
        move.w  #0x64,0x66(a6)                  | +01a
        lea     0x2975fc.l,a0                   | +020
        jsr     0x28cd4.l                       | +026
        jsr     0x2783a.l                       | +02c
        move.l  #0x2988ea,0x60(a6)              | +032
        lea     0xea3fa.l,a1                    | +03a
        move.w  #0x74,d0                        | +040
        move.b  #0x0,0x74(a6)                   | +044
        jsr     0x4429e.l                       | +04a
        lea     .L05335c(pc),a1                 | +050
        move.l  a1,(a6)                         | +054
.L05335c:
        clr.b   0x10e39a.l                      | +056
        jsr     0x2783a.l                       | +05c
        jsr     0x28998.l                       | +062
        jsr     0x28d70.l                       | +068
        jsr     0x2870a.l                       | +06e
        bcc.w   .L053390                        | +074
        lea     0x5e766.l,a0                    | +078
        jsr     0x5e770.l                       | +07e
        bclr    #0x3,0x13(a6)                   | +084
.L053390:
        jsr     0x28758.l                       | +08a
        bcc.w   .L0533c8                        | +090
        bclr    #0x0,0x13(a6)                   | +094
        move.w  #0x1023,d0                      | +09a
        jsr     0x2352.l                        | +09e
        lea     0x29801a.l,a1                   | +0a4
        jsr     0x77c7e.l                       | +0aa
        lea     0x29802c.l,a1                   | +0b0
        jsr     0x77c7e.l                       | +0b6
        lea     Prop_TowerStage2_0533da(pc),a1  | +0bc
        move.l  a1,(a6)                         | +0c0
.L0533c8:
        jsr     0x4fa70.l                       | +0c2
        bcc.w   .L0533d8                        | +0c8
        jmp     0x518.l                         | +0cc
.L0533d8:
        rts                                     | +0d2

| ----------------------------------------------------------------------------
|  Prop_TowerStage2_0533da  @ $0533DA  (168 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_TowerStage2_0533da, "ax", @progbits
        .global Prop_TowerStage2_0533da
Prop_TowerStage2_0533da:
        lea     0x298530.l,a2                   | +000
        jsr     0x5022a.l                       | +006
        move.w  #0x64,0x66(a6)                  | +00c
        lea     0x297612.l,a0                   | +012
        jsr     0x28cd4.l                       | +018
        lea     .L0533fe(pc),a1                 | +01e
        move.l  a1,(a6)                         | +022
.L0533fe:
        clr.b   0x10e39a.l                      | +024
        jsr     0x2783a.l                       | +02a
        jsr     0x28998.l                       | +030
        jsr     0x28d70.l                       | +036
        jsr     0x2870a.l                       | +03c
        bcc.w   .L053432                        | +042
        lea     0x5e766.l,a0                    | +046
        jsr     0x5e770.l                       | +04c
        bclr    #0x3,0x13(a6)                   | +052
.L053432:
        jsr     0x28758.l                       | +058
        bcc.w   .L053470                        | +05e
        move.w  #0x1030,d0                      | +062
        jsr     0x2352.l                        | +066
        lea     0x29801a.l,a1                   | +06c
        jsr     0x77c7e.l                       | +072
        lea     0x29802c.l,a1                   | +078
        jsr     0x77c7e.l                       | +07e
        lea     0x2980d8.l,a1                   | +084
        jsr     0x77c7e.l                       | +08a
        lea     Prop_TowerWreck_053482(pc),a1   | +090
        move.l  a1,(a6)                         | +094
.L053470:
        jsr     0x4fa70.l                       | +096
        bcc.w   .L053480                        | +09c
        jmp     0x518.l                         | +0a0
.L053480:
        rts                                     | +0a6

| ----------------------------------------------------------------------------
|  Prop_TowerWreck_053482  @ $053482  (86 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_TowerWreck_053482, "ax", @progbits
        .global Prop_TowerWreck_053482
Prop_TowerWreck_053482:
        move.l  #0x2000,d0                      | +000
        jsr     0x51a28.l                       | +006
        move.b  #0xff,0x74(a6)                  | +00c
        lea     0x298544.l,a2                   | +012
        jsr     0x5022a.l                       | +018
        lea     0xffff.w,a0                     | +01e
        move.l  a0,0x48(a6)                     | +022
        lea     0x297622.l,a0                   | +026
        jsr     0x28cd4.l                       | +02c
        lea     .L0534ba(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L0534ba:
        jsr     0x2783a.l                       | +038
        jsr     0x28d70.l                       | +03e
        jsr     0x4fa70.l                       | +044
        bcc.w   .L0534d6                        | +04a
        jmp     0x518.l                         | +04e
.L0534d6:
        rts                                     | +054

| ----------------------------------------------------------------------------
|  Prop_Gate_0534d8  @ $0534D8  (198 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_Gate_0534d8, "ax", @progbits
        .global Prop_Gate_0534d8
Prop_Gate_0534d8:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x36,d1                        | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0xc0,0x70(a6)                  | +014
        move.w  #0x28,0x66(a6)                  | +01a
        lea     0x297632.l,a0                   | +020
        jsr     0x28cd4.l                       | +026
        lea     .L05350a(pc),a1                 | +02c
        move.l  a1,(a6)                         | +030
.L05350a:
        jsr     0x2783a.l                       | +032
        jsr     0x28d70.l                       | +038
        cmpi.w  #0x90,0x22(a6)                  | +03e
        bgt.w   .L053590                        | +044
        cmpi.w  #0xffe0,0x22(a6)                | +048
        blt.w   Prop_Gate_0534d8__L05357e       | +04e
        jsr     0x2870a.l                       | +052
        bcc.w   .L053546                        | +058
        lea     0x5e766.l,a0                    | +05c
        jsr     0x5e770.l                       | +062
        bclr    #0x3,0x13(a6)                   | +068
.L053546:
        jsr     0x28758.l                       | +06e
        bcc.w   Prop_Gate_0534d8__L05357e       | +074
        bclr    #0x0,0x13(a6)                   | +078
        move.w  #0x1023,d0                      | +07e
        jsr     0x2352.l                        | +082
        lea     0x29803e.l,a1                   | +088
        jsr     0x77c7e.l                       | +08e
        lea     0x298050.l,a1                   | +094
        jsr     0x77c7e.l                       | +09a
        lea     Prop_GateStage2_05359e(pc),a1   | +0a0
        move.l  a1,(a6)                         | +0a4
        .global Prop_Gate_0534d8__L05357e
Prop_Gate_0534d8__L05357e:
        jsr     0x4fa70.l                       | +0a6
        bcc.w   .L05358e                        | +0ac
        jmp     0x518.l                         | +0b0
.L05358e:
        rts                                     | +0b6
.L053590:
        bclr    #0x3,0x13(a6)                   | +0b8
        bclr    #0x0,0x13(a6)                   | +0be
        rts                                     | +0c4

| ----------------------------------------------------------------------------
|  Prop_GateStage2_05359e  @ $05359E  (180 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_GateStage2_05359e, "ax", @progbits
        .global Prop_GateStage2_05359e
Prop_GateStage2_05359e:
        cmpi.w  #0x0,0x38(a6)                   | +000
        beq.w   .L0535b0                        | +006
        jsr     Sub_00053F2E(pc)                | +00a
        bra.w   .L0535b4                        | +00e
.L0535b0:
        jsr     Sub_00053EE2(pc)                | +012
.L0535b4:
        move.w  #0x28,0x66(a6)                  | +016
        lea     0x297648.l,a0                   | +01c
        jsr     0x28cd4.l                       | +022
        lea     .L0535cc(pc),a1                 | +028
        move.l  a1,(a6)                         | +02c
.L0535cc:
        jsr     0x2783a.l                       | +02e
        jsr     0x28d70.l                       | +034
        cmpi.w  #0xffe0,0x22(a6)                | +03a
        blt.b   Prop_Gate_0534d8__L05357e       | +040
        jsr     0x2870a.l                       | +042
        bcc.w   .L0535fc                        | +048
        lea     0x5e766.l,a0                    | +04c
        jsr     0x5e770.l                       | +052
        bclr    #0x3,0x13(a6)                   | +058
.L0535fc:
        jsr     0x28758.l                       | +05e
        bcc.w   .L053640                        | +064
        bclr    #0x0,0x13(a6)                   | +068
        move.w  #0x1030,d0                      | +06e
        jsr     0x2352.l                        | +072
        lea     0x29803e.l,a1                   | +078
        jsr     0x77c7e.l                       | +07e
        lea     0x298050.l,a1                   | +084
        jsr     0x77c7e.l                       | +08a
        lea     0x2980ea.l,a1                   | +090
        jsr     0x77c7e.l                       | +096
        lea     Prop_GateWreck_053652(pc),a1    | +09c
        move.l  a1,(a6)                         | +0a0
.L053640:
        jsr     0x4fa70.l                       | +0a2
        bcc.w   .L053650                        | +0a8
        jmp     0x518.l                         | +0ac
.L053650:
        rts                                     | +0b2

| ----------------------------------------------------------------------------
|  Prop_GateWreck_053652  @ $053652  (90 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_GateWreck_053652, "ax", @progbits
        .global Prop_GateWreck_053652
Prop_GateWreck_053652:
        move.l  #0x2000,d0                      | +000
        jsr     0x51a28.l                       | +006
        cmpi.w  #0x0,0x38(a6)                   | +00c
        beq.w   .L053670                        | +012
        jsr     Sub_00053F54(pc)                | +016
        bra.w   .L053674                        | +01a
.L053670:
        jsr     Sub_00053F08(pc)                | +01e
.L053674:
        lea     0xffff.w,a0                     | +022
        move.l  a0,0x48(a6)                     | +026
        lea     0x297658.l,a0                   | +02a
        jsr     0x28cd4.l                       | +030
        lea     .L05368e(pc),a1                 | +036
        move.l  a1,(a6)                         | +03a
.L05368e:
        jsr     0x2783a.l                       | +03c
        jsr     0x28d70.l                       | +042
        jsr     0x4fa70.l                       | +048
        bcc.w   .L0536aa                        | +04e
        jmp     0x518.l                         | +052
.L0536aa:
        rts                                     | +058

| ----------------------------------------------------------------------------
|  FixBlink_PhaseA_0536ac  @ $0536AC  (94 B)
| ----------------------------------------------------------------------------
        .section .text.FixBlink_PhaseA_0536ac, "ax", @progbits
        .global FixBlink_PhaseA_0536ac
FixBlink_PhaseA_0536ac:
        move.w  #0x19,d1                        | +000
        move.w  #0xc,d2                         | +004
        move.w  #0x2,d3                         | +008
        move.w  #0x1,d4                         | +00c
        jsr     0x2c26.l                        | +010
        move.w  #0x1b,d1                        | +016
        move.w  #0xe,d2                         | +01a
        move.w  #0x2,d3                         | +01e
        move.w  #0x1,d4                         | +022
        jsr     0x2c26.l                        | +026
        move.w  #0xb4,0x72(a6)                  | +02c
        lea     .L0536e4(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L0536e4:
        subq.w  #0x1,0x72(a6)                   | +038
        bne.w   .L0536f2                        | +03c
        lea     FixBlink_PhaseB_05370a(pc),a1   | +040
        move.l  a1,(a6)                         | +044
.L0536f2:
        move.l  0x106f5c.l,d0                   | +046
        swap    d0                              | +04c
        cmpi.w  #0x140,d0                       | +04e
        blt.w   .L053708                        | +052
        jmp     0x518.l                         | +056
.L053708:
        rts                                     | +05c

| ----------------------------------------------------------------------------
|  FixBlink_PhaseB_05370a  @ $05370A  (94 B)
| ----------------------------------------------------------------------------
        .section .text.FixBlink_PhaseB_05370a, "ax", @progbits
        .global FixBlink_PhaseB_05370a
FixBlink_PhaseB_05370a:
        move.w  #0x19,d1                        | +000
        move.w  #0xd,d2                         | +004
        move.w  #0x1,d3                         | +008
        move.w  #0x1,d4                         | +00c
        jsr     0x2c26.l                        | +010
        move.w  #0x1b,d1                        | +016
        move.w  #0xf,d2                         | +01a
        move.w  #0x1,d3                         | +01e
        move.w  #0x1,d4                         | +022
        jsr     0x2c26.l                        | +026
        move.w  #0x78,0x72(a6)                  | +02c
        lea     .L053742(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L053742:
        subq.w  #0x1,0x72(a6)                   | +038
        bne.w   .L053750                        | +03c
        lea     FixBlink_PhaseA_0536ac(pc),a1   | +040
        move.l  a1,(a6)                         | +044
.L053750:
        move.l  0x106f5c.l,d0                   | +046
        swap    d0                              | +04c
        cmpi.w  #0x140,d0                       | +04e
        blt.w   .L053766                        | +052
        jmp     0x518.l                         | +056
.L053766:
        rts                                     | +05c

| ----------------------------------------------------------------------------
|  PropDrop_Item_053768  @ $053768  (52 B)
| ----------------------------------------------------------------------------
        .section .text.PropDrop_Item_053768, "ax", @progbits
        .global PropDrop_Item_053768
PropDrop_Item_053768:
        move.w  #0xa,d1                         | +000
        jsr     0x236e.l                        | +004
        jsr     Sub_00053E9C(pc)                | +00a
        movea.l 0xc(a6),a0                      | +00e
        move.w  0x54(a0),0x22(a6)               | +012
        move.w  0x56(a0),0x24(a6)               | +018
        move.w  #0x3ff,d0                       | +01e
        jsr     0x5ea1c.l                       | +022
        addq.w  #0x1,d0                         | +028
        move.w  d0,0x36(a6)                     | +02a
        jmp     0x6dce0.l                       | +02e

| ----------------------------------------------------------------------------
|  PropDebris_ChunkAlt_05379c  @ $05379C  (78 B)
| ----------------------------------------------------------------------------
        .section .text.PropDebris_ChunkAlt_05379c, "ax", @progbits
        .global PropDebris_ChunkAlt_05379c
PropDebris_ChunkAlt_05379c:
        move.w  #0xb,d1                         | +000
        jsr     0x236e.l                        | +004
        bra.w   PropDebris_Chunk_0537ea__L053804 | +00a
        move.w  #0xb,d1                         | +00e
        jsr     0x236e.l                        | +012
        jsr     0x267e2.l                       | +018
        move.w  #0x1ff,d0                       | +01e
        jsr     0x5ea1c.l                       | +022
        move.w  d0,0x2a(a6)                     | +028
        move.w  #0xfff0,0x2e(a6)                | +02c
        move.w  #0xff,d0                        | +032
        jsr     0x5ea1c.l                       | +036
        btst    #0x0,d0                         | +03c
        beq.w   .L0537e2                        | +040
        neg.w   d0                              | +044
.L0537e2:
        move.w  d0,0x28(a6)                     | +046
        bra.w   PropDebris_Chunk_0537ea__L053828 | +04a

| ----------------------------------------------------------------------------
|  PropDebris_Chunk_0537ea  @ $0537EA  (156 B)
| ----------------------------------------------------------------------------
        .section .text.PropDebris_Chunk_0537ea, "ax", @progbits
        .global PropDebris_Chunk_0537ea
PropDebris_Chunk_0537ea:
        move.w  #0x29,d1                        | +000
        jsr     0x236e.l                        | +004
        movea.l 0xc(a6),a0                      | +00a
        move.w  0x54(a0),0x22(a6)               | +00e
        move.w  0x56(a0),0x24(a6)               | +014
        .global PropDebris_Chunk_0537ea__L053804
PropDebris_Chunk_0537ea__L053804:
        jsr     0x267e2.l                       | +01a
        move.w  #0xfff0,0x2e(a6)                | +020
        move.w  #0xf,d0                         | +026
        jsr     0x5ea1c.l                       | +02a
        btst    #0x0,d0                         | +030
        beq.w   .L053824                        | +034
        neg.w   d0                              | +038
.L053824:
        move.w  d0,0x28(a6)                     | +03a
        .global PropDebris_Chunk_0537ea__L053828
PropDebris_Chunk_0537ea__L053828:
        move.w  #0x28,0x72(a6)                  | +03e
        lea     0x2de4b0.l,a0                   | +044
        jsr     0x28cd4.l                       | +04a
        lea     .L053840(pc),a1                 | +050
        move.l  a1,(a6)                         | +054
.L053840:
        jsr     0x27cee.l                       | +056
        cmpi.w  #0xa,0x72(a6)                   | +05c
        bne.w   .L05385a                        | +062
        move.b  #0x14,d0                        | +066
        jsr     0x5e722.l                       | +06a
.L05385a:
        jsr     0x28d70.l                       | +070
        movea.l #0xffffffff,a0                  | +076
        lea     0x298736.l,a0                   | +07c
        jsr     0x5dd56.l                       | +082
        bcs.w   .L05387e                        | +088
        subq.w  #0x1,0x72(a6)                   | +08c
        bne.w   .L053884                        | +090
.L05387e:
        jmp     0x518.l                         | +094
.L053884:
        rts                                     | +09a

| ----------------------------------------------------------------------------
|  PropDebris_FlyingAlt_053886  @ $053886  (14 B)
| ----------------------------------------------------------------------------
        .section .text.PropDebris_FlyingAlt_053886, "ax", @progbits
        .global PropDebris_FlyingAlt_053886
PropDebris_FlyingAlt_053886:
        move.w  #0xad,d1                        | +000
        jsr     0x236e.l                        | +004
        bra.w   PropDebris_Flying_053894__L05389e | +00a

| ----------------------------------------------------------------------------
|  PropDebris_Flying_053894  @ $053894  (208 B)
| ----------------------------------------------------------------------------
        .section .text.PropDebris_Flying_053894, "ax", @progbits
        .global PropDebris_Flying_053894
PropDebris_Flying_053894:
        move.w  #0x9e,d1                        | +000
        jsr     0x236e.l                        | +004
        .global PropDebris_Flying_053894__L05389e
PropDebris_Flying_053894__L05389e:
        jsr     0x267e2.l                       | +00a
        movea.l 0xc(a6),a0                      | +010
        move.w  0x54(a0),0x22(a6)               | +014
        move.w  0x56(a0),0x24(a6)               | +01a
        move.w  #0xc00,0x2a(a6)                 | +020
        move.w  #0x100,d0                       | +026
        move.b  0x106f28.l,d1                   | +02a
        btst    #0x0,d1                         | +030
        beq.w   .L0538ce                        | +034
        neg.w   d0                              | +038
.L0538ce:
        move.w  d0,0x28(a6)                     | +03a
        cmpi.b  #0x4,0x58(a0)                   | +03e
        beq.w   .L0538f4                        | +044
        movea.l 0x50(a0),a0                     | +048
        move.w  0x28(a0),d0                     | +04c
        move.w  0x2a(a0),d1                     | +050
        asl.w   #0x1,d0                         | +054
        asl.w   #0x1,d1                         | +056
        move.w  d0,0x28(a6)                     | +058
        move.w  d1,0x2a(a6)                     | +05c
.L0538f4:
        move.w  #0x1ff,d0                       | +060
        jsr     0x5ea1c.l                       | +064
        ori.w   #0x100,d0                       | +06a
        andi.w  #0x1ff,d0                       | +06e
        sub.w   d0,0x2e(a6)                     | +072
        jsr     Sub_00053E9C(pc)                | +076
        lea     .L053914(pc),a1                 | +07a
        move.l  a1,(a6)                         | +07e
.L053914:
        jsr     0x27d50.l                       | +080
        bcc.w   .L053946                        | +086
        move.w  0x2a(a6),d0                     | +08a
        neg.w   d0                              | +08e
        asr.w   #0x1,d0                         | +090
        move.w  d0,0x2a(a6)                     | +092
        lea     0x77e10.l,a1                    | +096
        jsr     0x4ae.l                         | +09c
        jsr     0x5dd22.l                       | +0a2
        cmpi.w  #0x100,0x2a(a6)                 | +0a8
        blt.w   .L05395c                        | +0ae
.L053946:
        jsr     0x28d70.l                       | +0b2
        movea.l #0xffffffff,a0                  | +0b8
        jsr     0x5dd56.l                       | +0be
        bcc.w   .L053962                        | +0c4
.L05395c:
        jmp     0x518.l                         | +0c8
.L053962:
        rts                                     | +0ce

| ----------------------------------------------------------------------------
|  Prop_Indestructible_053964  @ $053964  (126 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_Indestructible_053964, "ax", @progbits
        .global Prop_Indestructible_053964
Prop_Indestructible_053964:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x1ae,d1                       | +00a
        jsr     0x236e.l                        | +00e
        jsr     0x267e2.l                       | +014
        move.w  #0x0,0x38(a6)                   | +01a
        lea     0x297728.l,a0                   | +020
        jsr     0x28cd4.l                       | +026
        move.b  #0x0,0x21(a6)                   | +02c
        lea     Sub_000539F0(pc),a1             | +032
        jsr     0x4ae.l                         | +036
        jsr     0x5dd22.l                       | +03c
        addi.w  #0x60,0x22(a0)                  | +042
        lea     .L0539b2(pc),a1                 | +048
        move.l  a1,(a6)                         | +04c
.L0539b2:
        jsr     0x2783a.l                       | +04e
        jsr     0x28d70.l                       | +054
        move.w  #0x7fff,0x66(a6)                | +05a
        jsr     0x2870a.l                       | +060
        bcc.w   .L0539d8                        | +066
        jsr     Sub_00053D80(pc)                | +06a
        bclr    #0x3,0x13(a6)                   | +06e
.L0539d8:
        jsr     0x4fa70.l                       | +074
        bcc.w   Jsr5B6Rts_0539ee                | +07a
