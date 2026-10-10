| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave YYYY — hijos del prop guionizado, fragmentos, plataformas, FX de
|  cañón, explosiones/humo, destructibles, PathScript VM, autodemo y
|  tripulación de vehículos
|  Región: $076000..$07A000  (15,600 B, 251 entradas, 67 huecos)
| ============================================================================
|
|  A) QUÉ ES
|  ---------
|  Región de 15.6 KB de código de "efectos y utilidades de escena" que
|  cierra el hueco entre el prop guionizado de misión ($075100) y el
|  banner de misión ($07A970). Diez módulos independientes:
|   1. $076000..$076D16 — hijos de ScriptedProp (tmpl 151): muros/torres/
|      puertas (Wall/Tower/Door con variantes B) que copian +$14/+$16 del
|      padre, parpadean con Child_Flash ($75F44) y degradan el sprite
|      cuando HP padre +$7E < +$66; Window/Roof/Gate/Base (spawneados por
|      Tmpl97+$3A..+$66) caen al activarse +$76..+$7B del padre y lanzan
|      debris (Entity_InstallHandlerAndCopyXf + Sprites_07466e). Cb_Spawn*
|      son callbacks `0800 <ptr>` del sprite-script de $75654/$75B50.
|   2. $076D16..$076FCE — Frag_*: fragmento balístico genérico (música
|      $10EA, snd $1E1..$1E3, 3 sprites de $23A394) con velocidad polar
|      por VelFromScale (divu 65536/escala) hacia el jugador más cercano o
|      aleatoria; hijo de animación Child_Run sigue +$14 del padre.
|   3. $076FCE..$077224 — Platform_*: dos listas de 8 plataformas de 6 B
|      (x, y, w, id) en $10E27C/$10E2AE (una por jugador, bit0 $106F28);
|      Register añade (+$30 = cuenta×6), FindNearestX busca la más próxima
|      bajo los pies, ProbeUnderFeet/ProbeNearest devuelven id en d0 (−1
|      si ninguna) y SnapToId ajusta +$28 para aterrizar. Llamado por
|      Pow_Walk ($3FFBC), Soldier_ProbeWalkEdge ($57002) y scene VM.
|   4. $077224..$0775AC — MovingPlatform_Tmpl143 (+$70/+$72 = pos mundo,
|      +$74 ancho de +$98, +$75 id incremental $10E2E0) y BridgePlank:
|      7 spawners (tabla $96F20 de escena) que leen registros de 40 B en
|      $7730A.. (x,y en /16 → Coord_ScreenToLocal) y alinean tablones.
|   5. $0775AC..$077C98 — MuzzleFx_*: fogonazo/humo de cañón hijo de un
|      vehículo (slug, tanque, torreta, Proj_Shell): Init* eligen 4 tablas
|      de sprites $2DCA..; Attach/Track/Flash siguen +$22/+$24 del padre y
|      su ángulo (+$72 clase, +$88 ángulo, +$73 cambio) leído con $27FAC;
|      Shot* son el proyectil hijo; SceneGate solo vive en escenas 0/9 y
|      SndByScene usa $138 en escena 5.
|   6. $077C98..$077E10 — Spawner_Handler: handler literal instalado por
|      Entity_InstallHandlerAndCopyXf; interpreta registros (+0 tipo, +1
|      ciclos, +2 repeticiones, +4/+8 offset, +6/+A rango rand, +D flag
|      tabla, +E puntero/tabla de handlers) y spawnea hijos con $6FE.
|   7. $077E10..$078840 — Explosion_*: 17 variantes directas (sprite
|      $2DD..., anim $2DF1F2.., snd $4/$D/$6C/$82/$12D/$145/$16C) + 32
|      variantes en tabla $78218 (stride 30 B, usadas por scripts de
|      misión $1980xx/$1E9Bxx/$1EDxxx) + Debris_Arc (balística rand).
|      Todas comparten el núcleo FireFront+$30: RandFlipFacing, anim hasta
|      fin (C=1 → $518). Explosion_Std ($77EFE) es la más llamada.
|   8. $078840..$078F6E — Smoke_* (7 variantes) y Breakable_Tmpl23..26
|      (cajas/barriles E8000 23-26, snd $1E, +$86 variante, bit0 +$3A
|      espejo): al destruirse (Destroyed) recorren FallPath ($78BE0, 33
|      pares dx/dy) y Wreck_SpawnSoldier libera un soldado (Leap/JumpIn).
|   9. $078F6E..$079250 — PathScript VM: intérprete de 11 opcodes sobre
|      cursor +$90(a6) (op0 mover dist, 1 jump, 2 call, 3 música, 4/5/6
|      set byte/word/long en (a6,d1) con assert d1<$A0 → trap #15, 7 set
|      ángulo +$34, 8 repeat, 9 mover longitud acumulada +$94, A arco)
|      e IntegratePolar (sin/cos $2C072C/$2C07AC, subpíxel +$26/+$27).
|      Lo usan gunship, M5Tank y POW ($3FE5A).
|  10. $079250..$079A6C — AutoDemo_*: tarea del attract (instalada por
|      MissionDriver_Init) que inyecta entradas sintéticas en $10E2E3..
|      $10E2EC (pad P1/P2) según nivel $10E39C (1..4, cada script fuerza
|      dirección/salto); DebugWarpKeys (A+B+C en $10E200) teleporta al
|      jugador y RecordBest guarda el máximo de $106E92.
|  11. $079A6C..$07A000 — Crew_Tmpl125..128: tripulantes que saltan de un
|      vehículo destruido (Jump→Land→Flee→Free, variante B con snd $1D5 y
|      sprites $29C244), Hatch* marcan bit +$72 en +$70 del padre;
|      Tmpl127/128 (Gunner/Driver) spawnean 3 tripulantes y heredan +$20.
|
|  B) CÓMO FUNCIONA
|  ----------------
|  Todas las tareas usan el patrón `lea next(pc),a1; move.l a1,(a6)` para
|  avanzar de estado y `jsr $28D70` (anim) con C=1 al terminar; los
|  "RetGate" (`cmp.b $10(a1),d0 ; bcs`) comparan prioridad con el padre.
|  PathScript: d5 = +$36 (velocidad) se consume por op (OpMoveLen resta
|  distancia acumulada en +$94.l formato 8.8 ×256); OpArc suma +$1(a0) al
|  ángulo hasta igualar +$2(a0). Platform_FindNearestX usa smi/eor/sub
|  para |dx| sin salto (idiom de abs) y devuelve d5=−1 si nada.
|  Explosion_VarTbl es una tabla de código: cada entrada de 30 B carga
|  sprite+anim y hace `bra.w` al núcleo con d1 = snd (10/11).
|
|  C) INTERFAZ
|  -----------
|   Entradas E8000: 23..26 $78C98/$78C78/$78CA2/$78C88, 125 $79A6C,
|   126 $79C8C, 127 $79EB8, 128 $79EC2, 143 $77224.
|   Externas más llamadas: Explosion_Std $77EFE (9), Explosion_FlashLoud
|   $7808A (6), Explosion_FireLoud $78066, Smoke_Pair20 $78890, MuzzleFx_
|   InitE $7773E (slug/tanques), Platform_ProbeUnderFeet $770CC, Path-
|   Script_StepCtx $78F8A, Spawner_Handler $77C98, Crew_Hatch $79B6E
|   (desde $7EB84, hueco futuro) y Frag_Launch $76E10 (script de $749E8).
|   Forward: Crew_Hostage_Init_07a19e (hijo extra de Crew_Tmpl127/128).
|
|  D) EVIDENCIAS
|  -------------
|   - $78FC0: 16 punteros long a los 11 opcodes (+5 rellenos a $79000 =
|     trap #15); assert duplicado con `nop;nop;cmpi;nop;trap` = macro de
|     depuración SNK que sobrevive en ROM.
|   - $78BE0..$78C64: 33 pares (dx,dy) con dy creciente −8..−$79 y vuelta:
|     trayectoria de caída pre-calculada (cmpa.l #$78C64 marca el fin).
|   - $772A8: 24 punteros a Explosion_Var01..06 + 7 registros de 40 B con
|     $106F6C/$298244 (scroll y sprite base de tablones).
|   - $2DF442: tabla de 5 handlers por nivel de autodemo (−1 = ninguno).
|
|  E) DUDAS / HIPÓTESIS
|  --------------------
|   - El nombre "BridgePlank" asume que los 7 registros con offsets X
|     crecientes de $20 (puente de misión 1 a $96F20) son tablones; podría
|     ser otro decorado segmentado.
|   - Explosion_Var01..32 se nombran por índice; el mapeo semántico
|     (agua, barro, chispas...) requiere ver los sprites $2DDxxx.
|   - Platform_ClearListByScroll limpia por jugador según bit0 $106F28;
|     el significado exacto de ese bit (cámara dividida?) no está confirmado.
|
|  F) RELACIÓN CON OTRAS WAVES
|  ---------------------------
|   XXXX (ScriptedProp_Tmpl97/Sprites/SpriteTable), T (Entity_Install-
|   HandlerAndCopyXf $77C7E, dentro de esta región), V (Tbl_Decode2D
|   $799DE, Entity_CheckBoxOverlapWithSelector $798AC), CCC (Squad_Track-
|   Arc/LeaderDeadGate usados por Crew), late_props_turrets (Breakable_
|   Tmpl23/24 en $060050/$06013C comparten familia con Tmpl23..26 aquí).
|
|  Verificación: cada sección .text.<Sym> se coloca en su dirección CPU
|  absoluta y reensambla byte-exacta contra build/mslug_prom.bin
|  (MD5 816b3f74c76b3373993407615f1850fe).
| ============================================================================

        .text

| ----------------------------------------------------------------------------
|  ScriptedProp_Child_ParentDeadGate_076000  @ $076000  (10 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Child_ParentDeadGate_076000, "ax", @progbits
        .global ScriptedProp_Child_ParentDeadGate_076000
ScriptedProp_Child_ParentDeadGate_076000:
        btst    #0x0,0x13(a0)                   | +000
        beq.w   SetHandlerRts_076010            | +006

| ----------------------------------------------------------------------------
|  ScriptedProp_SpawnFlagAndInit_076012  @ $076012  (60 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_SpawnFlagAndInit_076012, "ax", @progbits
        .global ScriptedProp_SpawnFlagAndInit_076012
ScriptedProp_SpawnFlagAndInit_076012:
        lea     ScriptedProp_Flag_Follow_0764b0(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        move.w  #0x54,d1                        | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0x1d4,d1                       | +014
        jsr     0x236e.l                        | +018
        move.w  #0x12,0x1c(a6)                  | +01e
        jsr     0x138fe.l                       | +024
        move.w  #0x2000,0x38(a6)                | +02a
        clr.w   0x72(a6)                        | +030
        clr.w   0x70(a6)                        | +034
        clr.b   0x84(a6)                        | +038

| ----------------------------------------------------------------------------
|  ScriptedProp_Wall_Idle_076056  @ $076056  (74 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Wall_Idle_076056, "ax", @progbits
        .global ScriptedProp_Wall_Idle_076056
ScriptedProp_Wall_Idle_076056:
        move.w  0x16(a6),0x14(a6)               | +000
        lea     ScriptedProp_SpriteTable_075270__L07527c(pc),a0 | +006
        jsr     0x28cd4.l                       | +00a
        lea     .L07606c(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L07606c:
        bsr.w   ScriptedProp_Child_Flash_075f3c__L075f44 | +016
        jsr     0x28d70.l                       | +01a
        movea.l 0xc(a6),a0                      | +020
        move.b  0x11(a0),0x11(a6)               | +024
        tst.b   0x70(a0)                        | +02a
        beq.w   SetHandlerRts_0760a6            | +02e
        move.l  a6,-(a7)                        | +032
        movea.l 0xc(a6),a6                      | +034
        lea     ScriptedProp_Gate_Init_076ba6(pc),a1 | +038
        jsr     0x4ae.l                         | +03c
        movea.l (a7)+,a6                        | +042
        move.w  #0x3c,0x70(a6)                  | +044

| ----------------------------------------------------------------------------
|  ScriptedProp_Wall_WaitTimer_0760a8  @ $0760A8  (24 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Wall_WaitTimer_0760a8, "ax", @progbits
        .global ScriptedProp_Wall_WaitTimer_0760a8
ScriptedProp_Wall_WaitTimer_0760a8:
        move.w  0x16(a6),0x14(a6)               | +000
        bsr.w   ScriptedProp_Child_Flash_075f3c__L075f44 | +006
        jsr     0x28d70.l                       | +00a
        tst.w   0x70(a6)                        | +010
        bne.w   SetHandlerRts_0760c6            | +014

| ----------------------------------------------------------------------------
|  ScriptedProp_Wall_Alt_0760c8  @ $0760C8  (36 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Wall_Alt_0760c8, "ax", @progbits
        .global ScriptedProp_Wall_Alt_0760c8
ScriptedProp_Wall_Alt_0760c8:
        move.w  0x16(a6),0x14(a6)               | +000
        lea     ScriptedProp_SpriteTable_075270__L075300(pc),a0 | +006
        jsr     0x28cd4.l                       | +00a
        lea     .L0760de(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L0760de:
        bsr.w   ScriptedProp_Child_Flash_075f3c__L075f44 | +016
        jsr     0x28d70.l                       | +01a
        bcc.w   SetHandlerRts_0760f2            | +020

| ----------------------------------------------------------------------------
|  ScriptedProp_Child_RandDelay_0760f4  @ $0760F4  (12 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Child_RandDelay_0760f4, "ax", @progbits
        .global ScriptedProp_Child_RandDelay_0760f4
ScriptedProp_Child_RandDelay_0760f4:
        lea     0x2bf392.l,a0                   | +000
        jsr     0x799de.l                       | +006

| ----------------------------------------------------------------------------
|  ScriptedProp_Child_DelayThenResume_076100  @ $076100  (56 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Child_DelayThenResume_076100, "ax", @progbits
        .global ScriptedProp_Child_DelayThenResume_076100
ScriptedProp_Child_DelayThenResume_076100:
        move.w  d0,0x70(a6)                     | +000
        move.w  0x16(a6),0x14(a6)               | +004
        lea     .L076110(pc),a1                 | +00a
        move.l  a1,(a6)                         | +00e
.L076110:
        bsr.w   ScriptedProp_Child_Flash_075f3c__L075f44 | +010
        jsr     0x28d70.l                       | +014
        subq.w  #0x1,0x70(a6)                   | +01a
        cmpi.w  #0x0,0x70(a6)                   | +01e
        bgt.w   .L076132                        | +024
        move.b  #0x1,0x84(a6)                   | +028
        move.l  0x80(a6),(a6)                   | +02e
.L076132:
        bra.w   ScriptedProp_Child_Parent_075ffc | +032
        rts                                     | +036

| ----------------------------------------------------------------------------
|  ScriptedProp_Tower_Idle_076138  @ $076138  (98 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Tower_Idle_076138, "ax", @progbits
        .global ScriptedProp_Tower_Idle_076138
ScriptedProp_Tower_Idle_076138:
        move.w  0x16(a6),0x14(a6)               | +000
        movea.l 0xc(a6),a1                      | +006
        move.w  0x7e(a1),d0                     | +00a
        cmp.w   0x66(a1),d0                     | +00e
        bge.w   .L07615c                        | +012
        lea     ScriptedProp_SpriteTable_075270__L0753a2(pc),a0 | +016
        jsr     0x28cd4.l                       | +01a
        bra.w   .L076174                        | +020
.L07615c:
        move.l  #0x76138,0x80(a6)               | +024
        tst.b   0x84(a6)                        | +02c
        beq.b   ScriptedProp_Child_RandDelay_0760f4 | +030
        lea     ScriptedProp_SpriteTable_075270__L075466(pc),a0 | +032
        jsr     0x28cd4.l                       | +036
.L076174:
        lea     .L07617a(pc),a1                 | +03c
        move.l  a1,(a6)                         | +040
.L07617a:
        bsr.w   ScriptedProp_Child_Flash_075f3c__L075f44 | +042
        jsr     0x28d70.l                       | +046
        bcc.w   .L076196                        | +04c
        tst.w   0x70(a6)                        | +050
        bne.w   .L076196                        | +054
        lea     ScriptedProp_Tower_Aim_07619a(pc),a1 | +058
        move.l  a1,(a6)                         | +05c
.L076196:
        bra.w   ScriptedProp_Child_Parent_075ffc | +05e

| ----------------------------------------------------------------------------
|  ScriptedProp_Tower_Aim_07619a  @ $07619A  (76 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Tower_Aim_07619a, "ax", @progbits
        .global ScriptedProp_Tower_Aim_07619a
ScriptedProp_Tower_Aim_07619a:
        move.w  0x16(a6),0x14(a6)               | +000
        movea.l 0xc(a6),a0                      | +006
        move.w  0x7e(a0),d0                     | +00a
        cmp.w   0x66(a0),d0                     | +00e
        bge.w   .L0761b8                        | +012
        lea     ScriptedProp_Child_Flash_075f3c(pc),a0 | +016
        bra.w   .L0761cc                        | +01a
.L0761b8:
        move.l  #0x7619a,0x80(a6)               | +01e
        tst.b   0x84(a6)                        | +026
        beq.w   ScriptedProp_Child_RandDelay_0760f4 | +02a
        lea     ScriptedProp_Hitbox_075f34(pc),a0 | +02e
.L0761cc:
        jsr     0x5e086.l                       | +032
        bcc.w   ScriptedProp_Door_Close_07639a__L0763c8 | +038
        lea     0x2bf20c.l,a0                   | +03c
        jsr     0x799de.l                       | +042
        move.w  d0,0x72(a6)                     | +048

| ----------------------------------------------------------------------------
|  ScriptedProp_Tower_Fire_0761e6  @ $0761E6  (70 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Tower_Fire_0761e6, "ax", @progbits
        .global ScriptedProp_Tower_Fire_0761e6
ScriptedProp_Tower_Fire_0761e6:
        lea     ScriptedProp_SpriteTable_075270__L075a82(pc),a0 | +000
        jsr     0x28cd4.l                       | +004
        bsr.w   ScriptedProp_Child_Timer_075fc8 | +00a
        lea     .L0761fa(pc),a1                 | +00e
        move.l  a1,(a6)                         | +012
.L0761fa:
        bsr.w   ScriptedProp_Child_Flash_075f3c__L075f44 | +014
        jsr     0x28d70.l                       | +018
        bcc.w   .L076228                        | +01e
        tst.w   0x72(a6)                        | +022
        beq.w   .L076222                        | +026
        tst.w   0x70(a6)                        | +02a
        bne.w   .L07621e                        | +02e
        lea     ScriptedProp_Tower_Fire_0761e6(pc),a1 | +032
        move.l  a1,(a6)                         | +036
.L07621e:
        bra.w   .L076228                        | +038
.L076222:
        lea     ScriptedProp_TowerB_Idle_07622c(pc),a1 | +03c
        move.l  a1,(a6)                         | +040
.L076228:
        bra.w   ScriptedProp_Child_Parent_075ffc | +042

| ----------------------------------------------------------------------------
|  ScriptedProp_TowerB_Idle_07622c  @ $07622C  (100 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_TowerB_Idle_07622c, "ax", @progbits
        .global ScriptedProp_TowerB_Idle_07622c
ScriptedProp_TowerB_Idle_07622c:
        move.w  0x16(a6),0x14(a6)               | +000
        movea.l 0xc(a6),a1                      | +006
        move.w  0x7e(a1),d0                     | +00a
        cmp.w   0x66(a1),d0                     | +00e
        bge.w   .L076250                        | +012
        lea     ScriptedProp_SpriteTable_075270__L075288(pc),a0 | +016
        jsr     0x28cd4.l                       | +01a
        bra.w   .L07626a                        | +020
.L076250:
        move.l  #0x7622c,0x80(a6)               | +024
        tst.b   0x84(a6)                        | +02c
        beq.w   ScriptedProp_Child_RandDelay_0760f4 | +030
        lea     ScriptedProp_SpriteTable_075270__L07534c(pc),a0 | +034
        jsr     0x28cd4.l                       | +038
.L07626a:
        lea     .L076270(pc),a1                 | +03e
        move.l  a1,(a6)                         | +042
.L076270:
        bsr.w   ScriptedProp_Child_Flash_075f3c__L075f44 | +044
        jsr     0x28d70.l                       | +048
        bcc.w   .L07628c                        | +04e
        tst.w   0x70(a6)                        | +052
        bne.w   .L07628c                        | +056
        lea     ScriptedProp_TowerB_Aim_076290(pc),a1 | +05a
        move.l  a1,(a6)                         | +05e
.L07628c:
        bra.w   ScriptedProp_Child_Parent_075ffc | +060

| ----------------------------------------------------------------------------
|  ScriptedProp_TowerB_Aim_076290  @ $076290  (76 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_TowerB_Aim_076290, "ax", @progbits
        .global ScriptedProp_TowerB_Aim_076290
ScriptedProp_TowerB_Aim_076290:
        move.w  0x16(a6),0x14(a6)               | +000
        movea.l 0xc(a6),a0                      | +006
        move.w  0x7e(a0),d0                     | +00a
        cmp.w   0x66(a0),d0                     | +00e
        bge.w   .L0762ae                        | +012
        lea     ScriptedProp_Child_Flash_075f3c(pc),a0 | +016
        bra.w   .L0762c2                        | +01a
.L0762ae:
        move.l  #0x76290,0x80(a6)               | +01e
        tst.b   0x84(a6)                        | +026
        beq.w   ScriptedProp_Child_RandDelay_0760f4 | +02a
        lea     ScriptedProp_Hitbox_075f34(pc),a0 | +02e
.L0762c2:
        jsr     0x5e086.l                       | +032
        bcc.w   ScriptedProp_TowerB_Fire_0762dc__L076322 | +038
        lea     0x2bf18a.l,a0                   | +03c
        jsr     0x799de.l                       | +042
        move.w  d0,0x72(a6)                     | +048

| ----------------------------------------------------------------------------
|  ScriptedProp_TowerB_Fire_0762dc  @ $0762DC  (116 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_TowerB_Fire_0762dc, "ax", @progbits
        .global ScriptedProp_TowerB_Fire_0762dc
ScriptedProp_TowerB_Fire_0762dc:
        lea     ScriptedProp_SpriteTable_075270__L0754b2(pc),a0 | +000
        jsr     0x28cd4.l                       | +004
        bsr.w   ScriptedProp_Child_Timer_075fc8 | +00a
        lea     .L0762f0(pc),a1                 | +00e
        move.l  a1,(a6)                         | +012
.L0762f0:
        bsr.w   ScriptedProp_Child_Flash_075f3c__L075f44 | +014
        jsr     0x28d70.l                       | +018
        bcc.w   .L07631e                        | +01e
        tst.w   0x72(a6)                        | +022
        beq.w   .L076318                        | +026
        tst.w   0x70(a6)                        | +02a
        bne.w   .L076314                        | +02e
        lea     ScriptedProp_TowerB_Fire_0762dc(pc),a1 | +032
        move.l  a1,(a6)                         | +036
.L076314:
        bra.w   .L07631e                        | +038
.L076318:
        lea     ScriptedProp_Tower_Idle_076138(pc),a1 | +03c
        move.l  a1,(a6)                         | +040
.L07631e:
        bra.w   ScriptedProp_Child_Parent_075ffc | +042
        .global ScriptedProp_TowerB_Fire_0762dc__L076322
ScriptedProp_TowerB_Fire_0762dc__L076322:
.L076322:
        move.w  0x16(a6),0x14(a6)               | +046
        lea     ScriptedProp_SpriteTable_075270__L075bd8(pc),a0 | +04c
        jsr     0x28cd4.l                       | +050
        lea     .L076338(pc),a1                 | +056
        move.l  a1,(a6)                         | +05a
.L076338:
        bsr.w   ScriptedProp_Child_Flash_075f3c__L075f44 | +05c
        jsr     0x28d70.l                       | +060
        bcc.w   .L07634c                        | +066
        lea     ScriptedProp_Door_Open_076350(pc),a1 | +06a
        move.l  a1,(a6)                         | +06e
.L07634c:
        bra.w   ScriptedProp_Child_Parent_075ffc | +070

| ----------------------------------------------------------------------------
|  ScriptedProp_Door_Open_076350  @ $076350  (74 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Door_Open_076350, "ax", @progbits
        .global ScriptedProp_Door_Open_076350
ScriptedProp_Door_Open_076350:
        move.w  0x16(a6),0x14(a6)               | +000
        lea     ScriptedProp_SpriteTable_075270__L075c34(pc),a0 | +006
        jsr     0x28cd4.l                       | +00a
        move.w  #0x3,0x72(a6)                   | +010
        lea     .L07636c(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L07636c:
        bsr.w   ScriptedProp_Child_Flash_075f3c__L075f44 | +01c
        jsr     0x28d70.l                       | +020
        bcc.w   .L076396                        | +026
        subq.w  #0x1,0x72(a6)                   | +02a
        bne.w   .L07638c                        | +02e
        lea     ScriptedProp_Door_Close_07639a(pc),a1 | +032
        move.l  a1,(a6)                         | +036
        bra.w   .L076396                        | +038
.L07638c:
        lea     ScriptedProp_SpriteTable_075270__L075c34(pc),a0 | +03c
        jsr     0x28cd4.l                       | +040
.L076396:
        bra.w   ScriptedProp_Child_Parent_075ffc | +046

| ----------------------------------------------------------------------------
|  ScriptedProp_Door_Close_07639a  @ $07639A  (92 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Door_Close_07639a, "ax", @progbits
        .global ScriptedProp_Door_Close_07639a
ScriptedProp_Door_Close_07639a:
        move.w  0x16(a6),0x14(a6)               | +000
        lea     ScriptedProp_SpriteTable_075270__L075cd0(pc),a0 | +006
        jsr     0x28cd4.l                       | +00a
        lea     .L0763b0(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L0763b0:
        bsr.w   ScriptedProp_Child_Flash_075f3c__L075f44 | +016
        jsr     0x28d70.l                       | +01a
        bcc.w   .L0763c4                        | +020
        lea     ScriptedProp_Tower_Idle_076138(pc),a1 | +024
        move.l  a1,(a6)                         | +028
.L0763c4:
        bra.w   ScriptedProp_Child_Parent_075ffc | +02a
        .global ScriptedProp_Door_Close_07639a__L0763c8
ScriptedProp_Door_Close_07639a__L0763c8:
.L0763c8:
        move.w  0x16(a6),0x14(a6)               | +02e
        lea     ScriptedProp_SpriteTable_075270__L075d2c(pc),a0 | +034
        jsr     0x28cd4.l                       | +038
        lea     .L0763de(pc),a1                 | +03e
        move.l  a1,(a6)                         | +042
.L0763de:
        bsr.w   ScriptedProp_Child_Flash_075f3c__L075f44 | +044
        jsr     0x28d70.l                       | +048
        bcc.w   .L0763f2                        | +04e
        lea     ScriptedProp_DoorB_Open_0763f6(pc),a1 | +052
        move.l  a1,(a6)                         | +056
.L0763f2:
        bra.w   ScriptedProp_Child_Parent_075ffc | +058

| ----------------------------------------------------------------------------
|  ScriptedProp_DoorB_Open_0763f6  @ $0763F6  (10 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_DoorB_Open_0763f6, "ax", @progbits
        .global ScriptedProp_DoorB_Open_0763f6
ScriptedProp_DoorB_Open_0763f6:
        move.w  0x16(a6),0x14(a6)               | +000
        lea     ScriptedProp_SpriteTable_075270__L075dac(pc),a0 | +006

| ----------------------------------------------------------------------------
|  ScriptedProp_DoorB_OpenLoop_076400  @ $076400  (64 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_DoorB_OpenLoop_076400, "ax", @progbits
        .global ScriptedProp_DoorB_OpenLoop_076400
ScriptedProp_DoorB_OpenLoop_076400:
        jsr     0x28cd4.l                       | +000
        move.w  #0x3,0x72(a6)                   | +006
        lea     .L076412(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L076412:
        bsr.w   ScriptedProp_Child_Flash_075f3c__L075f44 | +012
        jsr     0x28d70.l                       | +016
        bcc.w   .L07643c                        | +01c
        subq.w  #0x1,0x72(a6)                   | +020
        bne.w   .L076432                        | +024
        lea     ScriptedProp_DoorB_Close_076440(pc),a1 | +028
        move.l  a1,(a6)                         | +02c
        bra.w   .L07643c                        | +02e
.L076432:
        lea     ScriptedProp_SpriteTable_075270__L075dac(pc),a0 | +032
        jsr     0x28cd4.l                       | +036
.L07643c:
        bra.w   ScriptedProp_Child_Parent_075ffc | +03c

| ----------------------------------------------------------------------------
|  ScriptedProp_DoorB_Close_076440  @ $076440  (46 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_DoorB_Close_076440, "ax", @progbits
        .global ScriptedProp_DoorB_Close_076440
ScriptedProp_DoorB_Close_076440:
        move.w  0x16(a6),0x14(a6)               | +000
        lea     ScriptedProp_SpriteTable_075270__L075e76(pc),a0 | +006
        jsr     0x28cd4.l                       | +00a
        lea     .L076456(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L076456:
        bsr.w   ScriptedProp_Child_Flash_075f3c__L075f44 | +016
        jsr     0x28d70.l                       | +01a
        bcc.w   .L07646a                        | +020
        lea     ScriptedProp_TowerB_Idle_07622c(pc),a1 | +024
        move.l  a1,(a6)                         | +028
.L07646a:
        bra.w   ScriptedProp_Child_Parent_075ffc | +02a

| ----------------------------------------------------------------------------
|  ScriptedProp_Child_Collapse_07646e  @ $07646E  (66 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Child_Collapse_07646e, "ax", @progbits
        .global ScriptedProp_Child_Collapse_07646e
ScriptedProp_Child_Collapse_07646e:
        move.w  0x16(a6),0x14(a6)               | +000
        bset    #0x0,0x13(a6)                   | +006
        lea     ScriptedProp_SpriteTable_075270__L075bcc(pc),a0 | +00c
        jsr     0x28cd4.l                       | +010
        lea     .L07648a(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L07648a:
        bsr.w   ScriptedProp_Child_Flash_075f3c__L075f44 | +01c
        jsr     0x28d70.l                       | +020
        bcc.w   .L0764ae                        | +026
        lea     ScriptedProp_Sprites_07466e__L0749d6(pc),a1 | +02a
        jsr     0x77c7e.l                       | +02e
        jsr     0x13600.l                       | +034
        jmp     0x77f6a.l                       | +03a
.L0764ae:
        rts                                     | +040

| ----------------------------------------------------------------------------
|  ScriptedProp_Flag_Follow_0764b0  @ $0764B0  (94 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Flag_Follow_0764b0, "ax", @progbits
        .global ScriptedProp_Flag_Follow_0764b0
ScriptedProp_Flag_Follow_0764b0:
        movea.l 0xc(a6),a0                      | +000
        btst    #0x0,0x13(a0)                   | +004
        beq.w   .L0764ca                        | +00a
        jsr     0x5b6.l                         | +00e
        jmp     0x518.l                         | +014
.L0764ca:
        move.w  0x22(a0),0x22(a6)               | +01a
        move.w  0x24(a0),d0                     | +020
        add.w   0x78(a0),d0                     | +024
        move.w  d0,0x24(a6)                     | +028
        move.w  0x38(a0),0x38(a6)               | +02c
        move.l  0x74(a0),0x3c(a6)               | +032
        move.w  0x7a(a0),d1                     | +038
        cmp.w   0x7a(a6),d1                     | +03c
        beq.w   .L076508                        | +040
        move.w  d1,0x7a(a6)                     | +044
        jsr     0x13600.l                       | +048
        move.w  0x7a(a6),d1                     | +04e
        jsr     0x236e.l                        | +052
.L076508:
        jmp     0x5ca2a.l                       | +058

| ----------------------------------------------------------------------------
|  ScriptedProp_Cb_SpawnFlame_07650e  @ $07650E  (40 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Cb_SpawnFlame_07650e, "ax", @progbits
        .global ScriptedProp_Cb_SpawnFlame_07650e
ScriptedProp_Cb_SpawnFlame_07650e:
        lea     ScriptedProp_Flame_Run_076766(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        move.l  #0x76576,0x4c(a6)               | +00a
        jsr     0x283ca.l                       | +012
        jsr     0x283d8.l                       | +018
        move.l  #0xffffffff,0x4c(a6)            | +01e
        rts                                     | +026

| ----------------------------------------------------------------------------
|  ScriptedProp_Cb_SpawnSmoke_076536  @ $076536  (64 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Cb_SpawnSmoke_076536, "ax", @progbits
        .global ScriptedProp_Cb_SpawnSmoke_076536
ScriptedProp_Cb_SpawnSmoke_076536:
        lea     ScriptedProp_Smoke_Rise_0767f4(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        move.w  0x22(a6),d0                     | +00a
        move.w  0x24(a6),d1                     | +00e
        subi.w  #0x0,d0                         | +012
        subi.w  #0x40,d1                        | +016
        move.w  d0,0x22(a0)                     | +01a
        move.w  d1,0x24(a0)                     | +01e
        move.l  #0x765ca,0x4c(a6)               | +022
        jsr     0x283ca.l                       | +02a
        jsr     0x283d8.l                       | +030
        move.l  #0xffffffff,0x4c(a6)            | +036
        rts                                     | +03e

| ----------------------------------------------------------------------------
|  ScriptedProp_FlameSprites_076576  @ $076576  (168 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_FlameSprites_076576, "ax", @progbits
        .global ScriptedProp_FlameSprites_076576
ScriptedProp_FlameSprites_076576:
        .dc.w   0x000f                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0204                        | +004  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xffd8                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xffd8                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +014  (dato / opcode no decodificado)
        .dc.w   0x363a                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +020  (dato / opcode no decodificado)
        .dc.w   0x3644                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +024  (dato / opcode no decodificado)
        .dc.w   0xffd8                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xffd8                        | +032  (dato / opcode no decodificado)
        .dc.w   0xffd8                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +04a  (dato / opcode no decodificado)
        .dc.w   0xffd8                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)
        .dc.w   0x000f                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0204                        | +058  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +05c  (dato / opcode no decodificado)
        .dc.w   0xffd8                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +060  (dato / opcode no decodificado)
        .dc.w   0xffc0                        | +062  (dato / opcode no decodificado)
        .dc.w   0xffd8                        | +064  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +066  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +068  (dato / opcode no decodificado)
        .dc.w   0x363a                        | +06a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +070  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +072  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +074  (dato / opcode no decodificado)
        .dc.w   0x3644                        | +076  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +078  (dato / opcode no decodificado)
        .dc.w   0xffd8                        | +07a  (dato / opcode no decodificado)
        .dc.w   0xffd8                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +07e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +080  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +082  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +084  (dato / opcode no decodificado)
        .dc.w   0xffd8                        | +086  (dato / opcode no decodificado)
        .dc.w   0xffc0                        | +088  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +08a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +08e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +090  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +092  (dato / opcode no decodificado)
        .dc.w   0xffd8                        | +094  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +096  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +098  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +09a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +09e  (dato / opcode no decodificado)
        .dc.w   0xffc0                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +0a2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a6  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  ScriptedProp_SmokeSpritesB_07661e  @ $07661E  (164 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_SmokeSpritesB_07661e, "ax", @progbits
        .global ScriptedProp_SmokeSpritesB_07661e
ScriptedProp_SmokeSpritesB_07661e:
        .dc.w   0x000f                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0204                        | +004  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xffbe                        | +00a  (dato / opcode no decodificado)
        .dc.w   0xffd0                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +014  (dato / opcode no decodificado)
        .dc.w   0x363a                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +020  (dato / opcode no decodificado)
        .dc.w   0x3644                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +024  (dato / opcode no decodificado)
        .dc.w   0xffbe                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xffbe                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0xffd0                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0xffd0                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x030f                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0204                        | +054  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +058  (dato / opcode no decodificado)
        .dc.w   0xffbe                        | +05a  (dato / opcode no decodificado)
        .dc.w   0xffd0                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +060  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +062  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +064  (dato / opcode no decodificado)
        .dc.w   0x363a                        | +066  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +068  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +070  (dato / opcode no decodificado)
        .dc.w   0x3644                        | +072  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +074  (dato / opcode no decodificado)
        .dc.w   0xffbe                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +07e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +080  (dato / opcode no decodificado)
        .dc.w   0xffbe                        | +082  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +084  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +088  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +08a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +08c  (dato / opcode no decodificado)
        .dc.w   0xffd0                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +090  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +094  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +096  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +098  (dato / opcode no decodificado)
        .dc.w   0xffd0                        | +09a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +09e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a2  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  ScriptedProp_SmokeSpritesA_0766c2  @ $0766C2  (164 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_SmokeSpritesA_0766c2, "ax", @progbits
        .global ScriptedProp_SmokeSpritesA_0766c2
ScriptedProp_SmokeSpritesA_0766c2:
        .dc.w   0x000f                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0404                        | +004  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfec0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xfff2                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +014  (dato / opcode no decodificado)
        .dc.w   0x363a                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +020  (dato / opcode no decodificado)
        .dc.w   0x3644                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +024  (dato / opcode no decodificado)
        .dc.w   0xfec0                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xfec0                        | +032  (dato / opcode no decodificado)
        .dc.w   0xfff2                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +04a  (dato / opcode no decodificado)
        .dc.w   0xfff2                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x030f                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0404                        | +054  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +058  (dato / opcode no decodificado)
        .dc.w   0xfec0                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +05c  (dato / opcode no decodificado)
        .dc.w   0xfff2                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +060  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +062  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +064  (dato / opcode no decodificado)
        .dc.w   0x363a                        | +066  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +068  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +070  (dato / opcode no decodificado)
        .dc.w   0x3644                        | +072  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +074  (dato / opcode no decodificado)
        .dc.w   0xfec0                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +07e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +080  (dato / opcode no decodificado)
        .dc.w   0xfec0                        | +082  (dato / opcode no decodificado)
        .dc.w   0xfff2                        | +084  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +088  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +08a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +090  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +094  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +096  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +098  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +09a  (dato / opcode no decodificado)
        .dc.w   0xfff2                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +09e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a2  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  ScriptedProp_Flame_Run_076766  @ $076766  (142 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Flame_Run_076766, "ax", @progbits
        .global ScriptedProp_Flame_Run_076766
ScriptedProp_Flame_Run_076766:
        move.w  #0x156,d1                       | +000
        jsr     0x236e.l                        | +004
        move.w  #0x0,0x38(a6)                   | +00a
        clr.b   0x20(a6)                        | +010
        lea     ScriptedProp_SpriteTable_075270__L075704(pc),a0 | +014
        jsr     0x28cd4.l                       | +018
        lea     ScriptedProp_SmokeSpritesA_0766c2(pc),a0 | +01e
        move.l  a0,0x4c(a6)                     | +022
        jsr     0x283ca.l                       | +026
        lea     .L076798(pc),a1                 | +02c
        move.l  a1,(a6)                         | +030
.L076798:
        movea.l 0xc(a6),a0                      | +032
        move.w  0x22(a0),d0                     | +036
        subi.w  #0x20,d0                        | +03a
        move.w  d0,0x22(a6)                     | +03e
        move.w  0x24(a0),d0                     | +042
        subi.w  #0x18,d0                        | +046
        move.w  d0,0x24(a6)                     | +04a
        btst    #0x0,0x13(a0)                   | +04e
        beq.w   .L0767c4                        | +054
        jmp     0x518.l                         | +058
.L0767c4:
        jsr     0x28d70.l                       | +05e
        bcc.w   .L0767d4                        | +064
        jmp     0x518.l                         | +068
.L0767d4:
        tst.b   0x20(a6)                        | +06e
        beq.w   .L0767f2                        | +072
        clr.b   0x20(a6)                        | +076
        move.l  a6,-(a7)                        | +07a
        movea.l 0xc(a6),a6                      | +07c
        lea     ScriptedProp_SpriteTable_075270__L075688(pc),a0 | +080
        jsr     0x28cd4.l                       | +084
        movea.l (a7)+,a6                        | +08a
.L0767f2:
        rts                                     | +08c

| ----------------------------------------------------------------------------
|  ScriptedProp_Smoke_Rise_0767f4  @ $0767F4  (102 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Smoke_Rise_0767f4, "ax", @progbits
        .global ScriptedProp_Smoke_Rise_0767f4
ScriptedProp_Smoke_Rise_0767f4:
        lea     ScriptedProp_SpriteTable_075270__L075b98(pc),a0 | +000
        jsr     0x28cd4.l                       | +004
        move.w  #0x55,d1                        | +00a
        jsr     0x236e.l                        | +00e
        bset    #0x6,0x12(a6)                   | +014
        lea     ScriptedProp_SmokeSpritesB_07661e(pc),a0 | +01a
        move.l  a0,0x4c(a6)                     | +01e
        jsr     0x283ca.l                       | +022
        lea     0x2bf28e.l,a0                   | +028
        jsr     0x799de.l                       | +02e
        neg.w   d0                              | +034
        move.w  d0,0x28(a6)                     | +036
        clr.w   0x2a(a6)                        | +03a
        lea     .L076838(pc),a1                 | +03e
        move.l  a1,(a6)                         | +042
.L076838:
        jsr     0x27cee.l                       | +044
        jsr     0x283d8.l                       | +04a
        move.w  0x22(a6),d0                     | +050
        addi.w  #0x30,d0                        | +054
        cmpi.w  #0x1a0,d0                       | +058
        bcs.w   JsrAbsThunk_07685a              | +05c
        jmp     0x518.l                         | +060

| ----------------------------------------------------------------------------
|  ScriptedProp_WindowSprites_076862  @ $076862  (168 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_WindowSprites_076862, "ax", @progbits
        .global ScriptedProp_WindowSprites_076862
ScriptedProp_WindowSprites_076862:
        .dc.w   0x0001                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +004  (dato / opcode no decodificado)
        .dc.w   0x3534                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +00a  (dato / opcode no decodificado)
        .global ScriptedProp_WindowSprites_076862__L07686e
ScriptedProp_WindowSprites_076862__L07686e:
.L07686e:
        .dc.w   0x0003                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +010  (dato / opcode no decodificado)
        .dc.w   0x5c0e                        | +012  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x5c58                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +024  (dato / opcode no decodificado)
        .dc.w   0x5ca0                        | +026  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x5cf0                        | +030  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x5d40                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +042  (dato / opcode no decodificado)
        .dc.w   0x5d8a                        | +044  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x5dd4                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +056  (dato / opcode no decodificado)
        .dc.w   0x5e12                        | +058  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +060  (dato / opcode no decodificado)
        .dc.w   0x5e54                        | +062  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +064  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +066  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +068  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x5e96                        | +06c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +070  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +072  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +074  (dato / opcode no decodificado)
        .dc.w   0x5ed8                        | +076  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0501                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +07e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +080  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +082  (dato / opcode no decodificado)
        .dc.w   0x5f1c                        | +084  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +088  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +08a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x5f62                        | +08e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +090  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +094  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +096  (dato / opcode no decodificado)
        .dc.w   0x5fb0                        | +098  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +09a  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +09e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0x5ff0                        | +0a2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a4  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +0a6  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  ScriptedProp_Window_Init_07690a  @ $07690A  (26 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Window_Init_07690a, "ax", @progbits
        .global ScriptedProp_Window_Init_07690a
ScriptedProp_Window_Init_07690a:
        move.w  #0xd000,0x38(a6)                | +000
        move.w  #0x51,d1                        | +006
        jsr     0x236e.l                        | +00a
        lea     ScriptedProp_WindowSprites_076862(pc),a0 | +010
        jsr     0x28cd4.l                       | +014

| ----------------------------------------------------------------------------
|  ScriptedProp_Window_Idle_07692c  @ $07692C  (38 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Window_Idle_07692c, "ax", @progbits
        .global ScriptedProp_Window_Idle_07692c
ScriptedProp_Window_Idle_07692c:
        movea.l 0xc(a6),a0                      | +000
        tst.b   0x70(a0)                        | +004
        beq.w   .L07693e                        | +008
        lea     ScriptedProp_Window_Fall_07695a(pc),a1 | +00c
        move.l  a1,(a6)                         | +010
.L07693e:
        move.w  #0xfe0,d0                       | +012
        move.w  #0xa0,d1                        | +016
        jsr     ScriptedProp_ClampLocalX_0750c0(pc) | +01a
        move.w  d0,0x22(a6)                     | +01e
        move.w  d1,0x24(a6)                     | +022

| ----------------------------------------------------------------------------
|  ScriptedProp_Window_Fall_07695a  @ $07695A  (72 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Window_Fall_07695a, "ax", @progbits
        .global ScriptedProp_Window_Fall_07695a
ScriptedProp_Window_Fall_07695a:
        lea     ScriptedProp_WindowSprites_076862__L07686e(pc),a0 | +000
        jsr     0x28cd4.l                       | +004
        clr.b   0x20(a6)                        | +00a
        clr.w   0x28(a6)                        | +00e
        clr.w   0x2a(a6)                        | +012
        lea     .L076976(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L076976:
        tst.b   0x20(a6)                        | +01c
        beq.w   .L076984                        | +020
        addi.w  #0x10,0x28(a6)                  | +024
.L076984:
        jsr     0x27cee.l                       | +02a
        jsr     0x28d70.l                       | +030
        cmpi.w  #0x180,0x22(a6)                 | +036
        bcs.w   .L0769a0                        | +03c
        jmp     0x518.l                         | +040
.L0769a0:
        rts                                     | +046

| ----------------------------------------------------------------------------
|  ScriptedProp_RoofSprites_0769a2  @ $0769A2  (108 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_RoofSprites_0769a2, "ax", @progbits
        .global ScriptedProp_RoofSprites_0769a2
ScriptedProp_RoofSprites_0769a2:
        .dc.w   0x0001                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +004  (dato / opcode no decodificado)
        .dc.w   0x357a                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +00a  (dato / opcode no decodificado)
        .global ScriptedProp_RoofSprites_0769a2__L0769ae
ScriptedProp_RoofSprites_0769a2__L0769ae:
.L0769ae:
        .dc.w   0x0001                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +010  (dato / opcode no decodificado)
        .dc.w   0x35ae                        | +012  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +014  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +018  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +01a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +020  (dato / opcode no decodificado)
        .dc.w   0xffe0                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +024  (dato / opcode no decodificado)
        .dc.w   0xffe0                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0xffe0                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0xffe0                        | +04a  (dato / opcode no decodificado)
        .dc.w   0xffe0                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +050  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +052  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +058  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +05e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +060  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +062  (dato / opcode no decodificado)
        .dc.w   0xffe0                        | +064  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +066  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +068  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +06a  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  ScriptedProp_Roof_SetSprite_076a0e  @ $076A0E  (28 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Roof_SetSprite_076a0e, "ax", @progbits
        .global ScriptedProp_Roof_SetSprite_076a0e
ScriptedProp_Roof_SetSprite_076a0e:
        move.w  #0x50,d1                        | +000
        jsr     0x236e.l                        | +004
        lea     ScriptedProp_RoofSprites_0769a2(pc),a0 | +00a
        jsr     0x28cd4.l                       | +00e
        move.w  #0xa0,0x66(a6)                  | +014
        rts                                     | +01a

| ----------------------------------------------------------------------------
|  ScriptedProp_RoofA_Run_076a2a  @ $076A2A  (94 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_RoofA_Run_076a2a, "ax", @progbits
        .global ScriptedProp_RoofA_Run_076a2a
ScriptedProp_RoofA_Run_076a2a:
        bsr.b   ScriptedProp_Roof_SetSprite_076a0e | +000
        lea     .L076a32(pc),a1                 | +002
        move.l  a1,(a6)                         | +006
.L076a32:
        move.w  #0xf20,d0                       | +008
        move.w  #0x30,d1                        | +00c
        jsr     ScriptedProp_ClampLocalX_0750c0(pc) | +010
        move.w  d0,0x22(a6)                     | +014
        move.w  d1,0x24(a6)                     | +018
        jsr     0x28d70.l                       | +01c
        movea.l 0xc(a6),a0                      | +022
        tst.b   0x76(a0)                        | +026
        bne.w   .L076a7e                        | +02a
        jsr     0x2870a.l                       | +02e
        bcc.w   .L076a74                        | +034
        lea     0x5e766.l,a0                    | +038
        jsr     0x5e798.l                       | +03e
        bclr    #0x3,0x13(a6)                   | +044
.L076a74:
        jsr     0x28758.l                       | +04a
        bcc.w   SetHandlerRts_076a8e            | +050
.L076a7e:
        lea     ScriptedProp_Sprites_07466e__L0749b2(pc),a1 | +054
        jsr     0x77c7e.l                       | +058

| ----------------------------------------------------------------------------
|  ScriptedProp_RoofA_Debris_076a90  @ $076A90  (64 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_RoofA_Debris_076a90, "ax", @progbits
        .global ScriptedProp_RoofA_Debris_076a90
ScriptedProp_RoofA_Debris_076a90:
        lea     ScriptedProp_RoofSprites_0769a2__L0769ae(pc),a0 | +000
        jsr     0x28cd4.l                       | +004
        lea     ScriptedProp_Sprites_07466e__L0747b2(pc),a2 | +00a
        jsr     0x5022a.l                       | +00e
        lea     .L076aaa(pc),a1                 | +014
        move.l  a1,(a6)                         | +018
.L076aaa:
        move.w  #0xf20,d0                       | +01a
        move.w  #0x30,d1                        | +01e
        jsr     ScriptedProp_ClampLocalX_0750c0(pc) | +022
        move.w  d0,0x22(a6)                     | +026
        move.w  d1,0x24(a6)                     | +02a
        jsr     0x28d70.l                       | +02e
        movea.l 0xc(a6),a0                      | +034
        tst.b   0x78(a0)                        | +038
        beq.w   Jsr5B6Rts_076adc                | +03c

| ----------------------------------------------------------------------------
|  ScriptedProp_RoofB_Run_076ade  @ $076ADE  (84 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_RoofB_Run_076ade, "ax", @progbits
        .global ScriptedProp_RoofB_Run_076ade
ScriptedProp_RoofB_Run_076ade:
        bsr.w   ScriptedProp_Roof_SetSprite_076a0e | +000
        lea     .L076ae8(pc),a1                 | +004
        move.l  a1,(a6)                         | +008
.L076ae8:
        move.w  #0xfc8,d0                       | +00a
        move.w  #0x30,d1                        | +00e
        jsr     ScriptedProp_ClampLocalX_0750c0(pc) | +012
        move.w  d0,0x22(a6)                     | +016
        move.w  d1,0x24(a6)                     | +01a
        jsr     0x28d70.l                       | +01e
        movea.l 0xc(a6),a0                      | +024
        tst.b   0x77(a0)                        | +028
        bne.w   .L076b28                        | +02c
        jsr     0x2870a.l                       | +030
        bcc.w   .L076b1e                        | +036
        bclr    #0x3,0x13(a6)                   | +03a
.L076b1e:
        jsr     0x28758.l                       | +040
        bcc.w   SetHandlerRts_076b38            | +046
.L076b28:
        lea     ScriptedProp_Sprites_07466e__L0749c4(pc),a1 | +04a
        jsr     0x77c7e.l                       | +04e

| ----------------------------------------------------------------------------
|  ScriptedProp_RoofB_Debris_076b3a  @ $076B3A  (100 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_RoofB_Debris_076b3a, "ax", @progbits
        .global ScriptedProp_RoofB_Debris_076b3a
ScriptedProp_RoofB_Debris_076b3a:
        lea     ScriptedProp_RoofSprites_0769a2__L0769ae(pc),a0 | +000
        jsr     0x28cd4.l                       | +004
        lea     ScriptedProp_Sprites_07466e__L0747c6(pc),a2 | +00a
        jsr     0x5022a.l                       | +00e
        lea     ScriptedProp_Sprites_07466e__L0747da(pc),a2 | +014
        jsr     0x5022a.l                       | +018
        lea     ScriptedProp_Sprites_07466e__L0747ee(pc),a2 | +01e
        jsr     0x5022a.l                       | +022
        lea     ScriptedProp_Sprites_07466e__L0748b6(pc),a2 | +028
        jsr     0x5022a.l                       | +02c
        lea     .L076b72(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L076b72:
        movea.l 0xc(a6),a0                      | +038
        tst.b   0x79(a0)                        | +03c
        beq.w   .L076b8a                        | +040
        jsr     0x5b6.l                         | +044
        lea     .L076b8a(pc),a1                 | +04a
        move.l  a1,(a6)                         | +04e
.L076b8a:
        move.w  #0xfc8,d0                       | +050
        move.w  #0x30,d1                        | +054
        jsr     ScriptedProp_ClampLocalX_0750c0(pc) | +058
        move.w  d0,0x22(a6)                     | +05c
        move.w  d1,0x24(a6)                     | +060

| ----------------------------------------------------------------------------
|  ScriptedProp_Gate_Init_076ba6  @ $076BA6  (34 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Gate_Init_076ba6, "ax", @progbits
        .global ScriptedProp_Gate_Init_076ba6
ScriptedProp_Gate_Init_076ba6:
        move.w  #0x52,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x167,d1                       | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0x0,0x38(a6)                   | +014
        move.l  #0x2435e6,0x3c(a6)              | +01a

| ----------------------------------------------------------------------------
|  ScriptedProp_Gate_Idle_076bd0  @ $076BD0  (24 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Gate_Idle_076bd0, "ax", @progbits
        .global ScriptedProp_Gate_Idle_076bd0
ScriptedProp_Gate_Idle_076bd0:
        move.w  #0xfe0,d0                       | +000
        move.w  #0xa0,d1                        | +004
        jsr     ScriptedProp_ClampLocalX_0750c0(pc) | +008
        move.w  d0,0x22(a6)                     | +00c
        move.w  d1,0x24(a6)                     | +010
        jmp     ScriptedProp_Base_Run_076be8__L076c50(pc) | +014

| ----------------------------------------------------------------------------
|  ScriptedProp_Base_Run_076be8  @ $076BE8  (132 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_Base_Run_076be8, "ax", @progbits
        .global ScriptedProp_Base_Run_076be8
ScriptedProp_Base_Run_076be8:
        move.w  #0xa0,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x168,d1                       | +00a
        jsr     0x236e.l                        | +00e
        move.l  #0x24369e,0x3c(a6)              | +014
        move.w  #0x2000,0x38(a6)                | +01c
        lea     ScriptedProp_BaseSprites_076c74__L076cd4(pc),a1 | +022
        jsr     0x4ae.l                         | +026
        lea     .L076c1a(pc),a1                 | +02c
        move.l  a1,(a6)                         | +030
.L076c1a:
        movea.l 0xc(a6),a0                      | +032
        tst.b   0x7a(a0)                        | +036
        beq.w   .L076c3c                        | +03a
        jsr     0x5b6.l                         | +03e
        jsr     0x13600.l                       | +044
        lea     0x77fd6.l,a1                    | +04a
        move.l  a1,(a6)                         | +050
        rts                                     | +052
.L076c3c:
        move.w  #0x1028,d0                      | +054
        move.w  #0xa0,d1                        | +058
        jsr     ScriptedProp_ClampLocalX_0750c0(pc) | +05c
        move.w  d0,0x22(a6)                     | +060
        move.w  d1,0x24(a6)                     | +064
        .global ScriptedProp_Base_Run_076be8__L076c50
ScriptedProp_Base_Run_076be8__L076c50:
.L076c50:
        movea.l 0xc(a6),a0                      | +068
        tst.b   0x7b(a0)                        | +06c
        bne.w   .L076c66                        | +070
        move.w  0x16(a6),0x14(a6)               | +074
        bra.w   JsrAbsThunk_076c6c              | +07a
.L076c66:
        move.w  0x18(a6),0x14(a6)               | +07e

| ----------------------------------------------------------------------------
|  ScriptedProp_BaseSprites_076c74  @ $076C74  (154 B)
| ----------------------------------------------------------------------------
        .section .text.ScriptedProp_BaseSprites_076c74, "ax", @progbits
        .global ScriptedProp_BaseSprites_076c74
ScriptedProp_BaseSprites_076c74:
        .dc.w   0x000f                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0b44                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0b84                        | +010  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0b74                        | +01a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0b64                        | +024  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x0b54                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0b44                        | +038  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0b2c                        | +042  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +044  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0b2c                        | +04c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0b44                        | +056  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +058  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x6c74                        | +05e  (dato / opcode no decodificado)
        .global ScriptedProp_BaseSprites_076c74__L076cd4
ScriptedProp_BaseSprites_076c74__L076cd4:
.L076cd4:
        move.w  #0xe,d1                         | +060
        jsr     0x236e.l                        | +064
        move.w  #0x0,0x38(a6)                   | +06a
        move.b  #0x4,0x5c(a6)                   | +070
        lea     ScriptedProp_BaseSprites_076c74(pc),a0 | +076
        jsr     0x28cd4.l                       | +07a
        lea     .L076cfa(pc),a1                 | +080
        move.l  a1,(a6)                         | +084
.L076cfa:
        move.w  #0x1028,d0                      | +086
        move.w  #0xa0,d1                        | +08a
        jsr     ScriptedProp_ClampLocalX_0750c0(pc) | +08e
        move.w  d0,0x22(a6)                     | +092
        move.w  d1,0x24(a6)                     | +096

| ----------------------------------------------------------------------------
|  Frag_Sprites_076d16  @ $076D16  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Frag_Sprites_076d16, "ax", @progbits
        .global Frag_Sprites_076d16
Frag_Sprites_076d16:
        .dc.w   0x000f                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0304                        | +004  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfffc                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xfffe                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +014  (dato / opcode no decodificado)
        .dc.w   0x363a                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +020  (dato / opcode no decodificado)
        .dc.w   0x3644                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +024  (dato / opcode no decodificado)
        .dc.w   0xfffc                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xfffc                        | +032  (dato / opcode no decodificado)
        .dc.w   0xfffe                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +04a  (dato / opcode no decodificado)
        .dc.w   0xfffe                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Frag_SpriteScript_076d6a  @ $076D6A  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Frag_SpriteScript_076d6a, "ax", @progbits
        .global Frag_SpriteScript_076d6a
Frag_SpriteScript_076d6a:
        .dc.w   0x0a00                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +002  (dato / opcode no decodificado)
        .dc.w   0x6d16                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0800                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +008  (dato / opcode no decodificado)
        .dc.w   0x83ca                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +010  (dato / opcode no decodificado)
        .dc.w   0xa394                        | +012  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0800                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +018  (dato / opcode no decodificado)
        .dc.w   0x6dd4                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +020  (dato / opcode no decodificado)
        .dc.w   0xa394                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +028  (dato / opcode no decodificado)
        .dc.w   0x6d80                        | +02a  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Frag_SpriteScriptB_076d96  @ $076D96  (62 B)
| ----------------------------------------------------------------------------
        .section .text.Frag_SpriteScriptB_076d96, "ax", @progbits
        .global Frag_SpriteScriptB_076d96
Frag_SpriteScriptB_076d96:
        .dc.w   0x0001                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +002  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +004  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +00e  (dato / opcode no decodificado)
        .dc.w   0xa394                        | +010  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +018  (dato / opcode no decodificado)
        .dc.w   0xa39e                        | +01a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +022  (dato / opcode no decodificado)
        .dc.w   0xa3a8                        | +024  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +02c  (dato / opcode no decodificado)
        .dc.w   0xa3b2                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +036  (dato / opcode no decodificado)
        .dc.w   0xa3bc                        | +038  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +03c  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Frag_SpawnChildAnim_076dd4  @ $076DD4  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Frag_SpawnChildAnim_076dd4, "ax", @progbits
        .global Frag_SpawnChildAnim_076dd4
Frag_SpawnChildAnim_076dd4:
        lea     Frag_Child_Run_076f42(pc),a1    | +000
        jsr     0x6fe.l                         | +004
        jsr     0x5dd02.l                       | +00a
        subq.w  #0x1,0x38(a0)                   | +010
        move.b  0x5c(a6),d0                     | +014
        addq.b  #0x1,d0                         | +018
        andi.w  #0x3,d0                         | +01a
        cmpi.w  #0x3,d0                         | +01e
        blt.w   .L076dfc                        | +022
        clr.w   d0                              | +026
.L076dfc:
        move.w  d0,d1                           | +028
        move.b  d0,0x5c(a6)                     | +02a
        add.w   d1,d1                           | +02e
        addi.w  #0x16,d1                        | +030
        move.w  (a6,d1.w),0x14(a6)              | +034
        rts                                     | +03a

| ----------------------------------------------------------------------------
|  Frag_Launch_076e10  @ $076E10  (240 B)
| ----------------------------------------------------------------------------
        .section .text.Frag_Launch_076e10, "ax", @progbits
        .global Frag_Launch_076e10
Frag_Launch_076e10:
        move.w  #0x10ea,d0                      | +000
        jsr     0x2352.l                        | +004
        bset    #0x4,0x6b(a6)                   | +00a
        jsr     Frag_PlayHitSounds_076f78(pc)   | +010
        movea.l 0x7c(a6),a0                     | +014
        jsr     0x5e086.l                       | +018
        bcc.w   .L076e54                        | +01e
        move.w  #0x100,d0                       | +022
        jsr     0x5e9e4.l                       | +026
        move.w  d0,-(a7)                        | +02c
        move.w  #0x10,d0                        | +02e
        jsr     0x5e9e4.l                       | +032
        addi.w  #0x10,d0                        | +038
        move.w  d0,d1                           | +03c
        move.w  (a7)+,d0                        | +03e
        bra.w   .L076eae                        | +040
.L076e54:
        move.w  0x22(a0),d0                     | +044
        sub.w   0x22(a6),d0                     | +048
        move.w  0x24(a0),d1                     | +04c
        addi.w  #0x10,d1                        | +050
        sub.w   0x24(a6),d1                     | +054
        movem.w d0-d1,-(a7)                     | +058
        jsr     0x5e9b6.l                       | +05c
        andi.w  #0x3,d0                         | +062
        bne.w   .L076e82                        | +066
        clr.w   d2                              | +06a
        clr.w   d3                              | +06c
        bra.w   .L076ea0                        | +06e
.L076e82:
        jsr     0x5e9b6.l                       | +072
        andi.w  #0x7f,d0                        | +078
        subi.w  #0x3f,d0                        | +07c
        move.w  d0,-(a7)                        | +080
        jsr     0x5e9b6.l                       | +082
        andi.w  #0xf,d0                         | +088
        move.w  d0,d3                           | +08c
        move.w  (a7)+,d2                        | +08e
.L076ea0:
        movem.w (a7)+,d0-d1                     | +090
        add.w   d2,d0                           | +094
        add.w   d3,d1                           | +096
        bpl.w   .L076eae                        | +098
        clr.w   d1                              | +09c
.L076eae:
        movem.w d0-d1,-(a7)                     | +09e
        lea     0x2bf310.l,a0                   | +0a2
        jsr     0x799de.l                       | +0a8
        move.w  d0,d2                           | +0ae
        movem.w (a7)+,d0-d1                     | +0b0
        bsr.w   Frag_VelFromScale_076f9e        | +0b4
        move.w  d0,0x28(a6)                     | +0b8
        move.w  d1,0x2a(a6)                     | +0bc
        move.w  d2,0x2e(a6)                     | +0c0
        move.w  #0xd000,d0                      | +0c4
        jsr     0x28134.l                       | +0c8
        andi.w  #0xffe3,0x38(a6)                | +0ce
        ori.w   #0x14,0x38(a6)                  | +0d4
        lea     Frag_SpriteScript_076d6a(pc),a0 | +0da
        jsr     0x28cd4.l                       | +0de
        lea     .L076efa(pc),a1                 | +0e4
        move.l  a1,(a6)                         | +0e8
.L076efa:
        jsr     0x2783a.l                       | +0ea

| ----------------------------------------------------------------------------
|  Frag_Fly_076f00  @ $076F00  (66 B)
| ----------------------------------------------------------------------------
        .section .text.Frag_Fly_076f00, "ax", @progbits
        .global Frag_Fly_076f00
Frag_Fly_076f00:
        move.w  0x2e(a6),d0                     | +000
        add.w   d0,0x2a(a6)                     | +004
        jsr     0x9c072.l                       | +008
        jsr     0x28d70.l                       | +00e
        jsr     0x283d8.l                       | +014
        cmpi.w  #0x140,0x22(a6)                 | +01a
        bcs.w   .L076f2a                        | +020
        jmp     0x518.l                         | +024
.L076f2a:
        move.w  0x24(a6),d0                     | +02a
        subi.w  #0x100,d0                       | +02e
        cmpi.w  #0x140,d0                       | +032
        bcs.w   .L076f40                        | +036
        jmp     0x518.l                         | +03a
.L076f40:
        rts                                     | +040

| ----------------------------------------------------------------------------
|  Frag_Child_Run_076f42  @ $076F42  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Frag_Child_Run_076f42, "ax", @progbits
        .global Frag_Child_Run_076f42
Frag_Child_Run_076f42:
        jsr     Frag_PlayHitSounds_076f78(pc)   | +000
        lea     Frag_SpriteScriptB_076d96(pc),a0 | +004
        jsr     0x28cd4.l                       | +008
        lea     .L076f56(pc),a1                 | +00e
        move.l  a1,(a6)                         | +012
.L076f56:
        jsr     0x2783a.l                       | +014
        movea.l 0xc(a6),a0                      | +01a
        move.w  0x14(a0),0x14(a6)               | +01e
        jsr     0x28d70.l                       | +024
        bcc.w   .L076f76                        | +02a
        jmp     0x518.l                         | +02e
.L076f76:
        rts                                     | +034

| ----------------------------------------------------------------------------
|  Frag_PlayHitSounds_076f78  @ $076F78  (38 B)
| ----------------------------------------------------------------------------
        .section .text.Frag_PlayHitSounds_076f78, "ax", @progbits
        .global Frag_PlayHitSounds_076f78
Frag_PlayHitSounds_076f78:
        move.w  #0x1e1,d1                       | +000
        jsr     0x236e.l                        | +004
        move.w  #0x1e2,d1                       | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0x1e3,d1                       | +014
        jsr     0x236e.l                        | +018
        move.w  0x16(a6),0x14(a6)               | +01e
        rts                                     | +024

| ----------------------------------------------------------------------------
|  Frag_VelFromScale_076f9e  @ $076F9E  (48 B)
| ----------------------------------------------------------------------------
        .section .text.Frag_VelFromScale_076f9e, "ax", @progbits
        .global Frag_VelFromScale_076f9e
Frag_VelFromScale_076f9e:
        cmpi.w  #0x2,d2                         | +000
        bcc.w   .L076faa                        | +004
        move.w  #0x2,d2                         | +008
.L076faa:
        move.l  #0x10000,d4                     | +00c
        divu.w  d2,d4                           | +012
        muls.w  d4,d0                           | +014
        asr.l   #0x8,d0                         | +016
        move.w  #0x200,d2                       | +018
        mulu.w  d4,d2                           | +01c
        lsr.l   #0x8,d2                         | +01e
        mulu.w  d4,d2                           | +020
        swap    d2                              | +022
        neg.w   d2                              | +024
        addi.w  #0x100,d1                       | +026
        mulu.w  d4,d1                           | +02a
        lsr.l   #0x8,d1                         | +02c
        rts                                     | +02e

| ----------------------------------------------------------------------------
|  Platform_RetGate_076fce  @ $076FCE  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Platform_RetGate_076fce, "ax", @progbits
        .global Platform_RetGate_076fce
Platform_RetGate_076fce:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_076fe4                    | +00c

| ----------------------------------------------------------------------------
|  Platform_Register_076fea  @ $076FEA  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Platform_Register_076fea, "ax", @progbits
        .global Platform_Register_076fea
Platform_Register_076fea:
        move.w  0x30(a0),d0                     | +000
        cmpi.w  #0x30,d0                        | +004
        bcc.w   Platform_AdvanceCount_077004__L07700a | +008
        lea     (a0,d0.w),a1                    | +00c
        lea     0x70(a6),a2                     | +010
        move.w  (a2)+,(a1)+                     | +014
        move.w  (a2)+,(a1)+                     | +016
        move.w  (a2)+,(a1)+                     | +018

| ----------------------------------------------------------------------------
|  Platform_AdvanceCount_077004  @ $077004  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Platform_AdvanceCount_077004, "ax", @progbits
        .global Platform_AdvanceCount_077004
Platform_AdvanceCount_077004:
        addq.w  #0x6,d0                         | +000
        move.w  d0,0x30(a0)                     | +002
        .global Platform_AdvanceCount_077004__L07700a
Platform_AdvanceCount_077004__L07700a:
.L07700a:
        rts                                     | +006

| ----------------------------------------------------------------------------
|  Platform_FindNearestX_07700c  @ $07700C  (112 B)
| ----------------------------------------------------------------------------
        .section .text.Platform_FindNearestX_07700c, "ax", @progbits
        .global Platform_FindNearestX_07700c
Platform_FindNearestX_07700c:
        clr.w   d7                              | +000
        moveq   #-1,d6                          | +002
        move.w  d6,d5                           | +004
        lea     (a0),a5                         | +006
.L077014:
        cmp.w   0x30(a0),d7                     | +008
        bcc.w   .L077054                        | +00c
        move.w  0x2(a5,d7.w),d2                 | +010
        sub.w   d1,d2                           | +014
        clr.w   d3                              | +016
        move.b  0x4(a5,d7.w),d3                 | +018
        cmp.w   d3,d2                           | +01c
        bhi.w   .L077050                        | +01e
        lsr.w   #0x3,d2                         | +022
        neg.w   d2                              | +024
        add.w   (a5,d7.w),d2                    | +026
        move.w  d2,d3                           | +02a
        sub.w   d0,d3                           | +02c
        smi.b   d4                              | +02e
        ext.w   d4                              | +030
        eor.w   d4,d3                           | +032
        sub.w   d4,d3                           | +034
        cmp.w   d3,d5                           | +036
        bls.w   .L077050                        | +038
        move.w  d3,d5                           | +03c
        move.w  d2,d6                           | +03e
        lea     (a5,d7.w),a1                    | +040
.L077050:
        addq.w  #0x6,d7                         | +044
        bra.b   .L077014                        | +046
.L077054:
        rts                                     | +048
        .global Platform_FindNearestX_07700c__L077056
Platform_FindNearestX_07700c__L077056:
.L077056:
        moveq   #0,d7                           | +04a
        movea.l d7,a1                           | +04c
        moveq   #-1,d6                          | +04e
        lea     (a0),a5                         | +050
.L07705e:
        cmp.w   0x30(a0),d7                     | +052
        bcc.w   .L07707a                        | +056
        cmp.b   0x5(a5,d7.w),d0                 | +05a
        bne.w   .L077076                        | +05e
        lea     (a5,d7.w),a1                    | +062
        bra.w   .L07707a                        | +066
.L077076:
        addq.w  #0x6,d7                         | +06a
        bra.b   .L07705e                        | +06c
.L07707a:
        rts                                     | +06e

| ----------------------------------------------------------------------------
|  Platform_ListsInit_07707c  @ $07707C  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Platform_ListsInit_07707c, "ax", @progbits
        .global Platform_ListsInit_07707c
Platform_ListsInit_07707c:
        lea     0x10e27c.l,a0                   | +000
        clr.w   0x30(a0)                        | +006
        lea     0x10e2ae.l,a0                   | +00a
        clr.w   0x30(a0)                        | +010
        rts                                     | +014

| ----------------------------------------------------------------------------
|  Platform_ClearListByScroll_077092  @ $077092  (58 B)
| ----------------------------------------------------------------------------
        .section .text.Platform_ClearListByScroll_077092, "ax", @progbits
        .global Platform_ClearListByScroll_077092
Platform_ClearListByScroll_077092:
        lea     0x10e27c.l,a0                   | +000
        btst    #0x0,0x106f28.l                 | +006
        beq.w   .L0770aa                        | +00e
        lea     0x10e2ae.l,a0                   | +012
.L0770aa:
        clr.w   0x30(a0)                        | +018
        rts                                     | +01c
        .global Platform_ClearListByScroll_077092__L0770b0
Platform_ClearListByScroll_077092__L0770b0:
.L0770b0:
        lea     0x10e27c.l,a0                   | +01e
        btst    #0x0,0x106f28.l                 | +024
        beq.w   .L0770c8                        | +02c
        lea     0x10e2ae.l,a0                   | +030
.L0770c8:
        bra.w   Platform_Register_076fea        | +036

| ----------------------------------------------------------------------------
|  Platform_ProbeUnderFeet_0770cc  @ $0770CC  (120 B)
| ----------------------------------------------------------------------------
        .section .text.Platform_ProbeUnderFeet_0770cc, "ax", @progbits
        .global Platform_ProbeUnderFeet_0770cc
Platform_ProbeUnderFeet_0770cc:
        lea     0x10e27c.l,a0                   | +000
        btst    #0x0,0x106f28.l                 | +006
        bne.w   .L0770e4                        | +00e
        lea     0x10e2ae.l,a0                   | +012
.L0770e4:
        move.w  0x22(a6),d0                     | +018
        move.w  0x24(a6),d1                     | +01c
        add.w   0x106f50.l,d0                   | +020
        subi.w  #0x200,d1                       | +026
        neg.w   d1                              | +02a
        add.w   0x106f54.l,d1                   | +02c
        bsr.w   Platform_FindNearestX_07700c    | +032
        addq.w  #0x1,d5                         | +036
        beq.w   RetMinus1_00077144              | +038
        clr.w   d2                              | +03c
        move.b  0x26(a6),d2                     | +03e
        add.w   0x28(a6),d2                     | +042
        asr.w   #0x8,d2                         | +046
        add.w   d0,d2                           | +048
        sub.w   d6,d2                           | +04a
        smi.b   d2                              | +04c
        sub.w   d6,d0                           | +04e
        smi.b   d0                              | +050
        eor.b   d2,d0                           | +052
        beq.w   RetMinus1_00077144              | +054
        clr.w   d4                              | +058
        clr.w   d2                              | +05a
        move.b  0x4(a1),d2                      | +05c
        lsr.w   #0x1,d2                         | +060
        neg.w   d2                              | +062
        add.w   0x2(a1),d2                      | +064
        cmp.w   d2,d1                           | +068
        spl.b   d1                              | +06a
        addi.b  #0x2,d1                         | +06c
        clr.w   d0                              | +070
        move.b  0x5(a1),d0                      | +072
        rts                                     | +076

| ----------------------------------------------------------------------------
|  Platform_ProbeNearest_077148  @ $077148  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Platform_ProbeNearest_077148, "ax", @progbits
        .global Platform_ProbeNearest_077148
Platform_ProbeNearest_077148:
        lea     0x10e27c.l,a0                   | +000
        btst    #0x0,0x106f28.l                 | +006
        bne.w   .L077160                        | +00e
        lea     0x10e2ae.l,a0                   | +012
.L077160:
        move.w  0x22(a6),d0                     | +018
        move.w  0x24(a6),d1                     | +01c
        add.w   0x106f50.l,d0                   | +020
        subi.w  #0x200,d1                       | +026
        neg.w   d1                              | +02a
        add.w   0x106f54.l,d1                   | +02c
        bsr.w   Platform_FindNearestX_07700c    | +032
        addq.w  #0x1,d5                         | +036
        beq.b   RetMinus1_00077144              | +038
        sub.w   0x106f50.l,d6                   | +03a
        clr.w   d0                              | +040
        move.b  0x5(a1),d0                      | +042
        rts                                     | +046

| ----------------------------------------------------------------------------
|  Platform_SnapToId_077190  @ $077190  (148 B)
| ----------------------------------------------------------------------------
        .section .text.Platform_SnapToId_077190, "ax", @progbits
        .global Platform_SnapToId_077190
Platform_SnapToId_077190:
        lea     0x10e27c.l,a0                   | +000
        btst    #0x0,0x106f28.l                 | +006
        bne.w   .L0771a8                        | +00e
        lea     0x10e2ae.l,a0                   | +012
.L0771a8:
        bsr.w   Platform_FindNearestX_07700c__L077056 | +018
        move.l  a1,d0                           | +01c
        beq.w   .L07721a                        | +01e
        clr.w   -(a7)                           | +022
        move.w  0x2(a1),d0                      | +024
        sub.w   0x106f54.l,d0                   | +028
        neg.w   d0                              | +02e
        addi.w  #0x200,d0                       | +030
        clr.w   d1                              | +034
        move.b  0x4(a1),d1                      | +036
        add.w   d0,d1                           | +03a
        addq.w  #0x1,d1                         | +03c
        move.w  d1,(a7)                         | +03e
        clr.w   d1                              | +040
        move.b  0x27(a6),d1                     | +042
        add.w   0x2a(a6),d1                     | +046
        add.w   0x2e(a6),d1                     | +04a
        asr.w   #0x8,d1                         | +04e
        add.w   0x24(a6),d1                     | +050
        sub.w   d0,d1                           | +054
        cmpi.w  #0xfff8,d1                      | +056
        bge.w   .L0771f2                        | +05a
        move.w  #0xffff,(a7)                    | +05e
.L0771f2:
        neg.w   d1                              | +062
        asr.w   #0x3,d1                         | +064
        add.w   (a1),d1                         | +066
        sub.w   0x106f50.l,d1                   | +068
        sub.w   0x22(a6),d1                     | +06e
        lsl.w   #0x8,d1                         | +072
        move.w  d1,0x28(a6)                     | +074
        clr.w   0x2c(a6)                        | +078
        jsr     0x27d50.l                       | +07c
        clr.w   0x28(a6)                        | +082
        move.w  (a7)+,d0                        | +086
        rts                                     | +088
.L07721a:
        jsr     0x27d50.l                       | +08a
        moveq   #-1,d0                          | +090
        rts                                     | +092

| ----------------------------------------------------------------------------
|  MovingPlatform_Tmpl143_077224  @ $077224  (4 B)
| ----------------------------------------------------------------------------
        .section .text.MovingPlatform_Tmpl143_077224, "ax", @progbits
        .global MovingPlatform_Tmpl143_077224
MovingPlatform_Tmpl143_077224:
        clr.b   0x99(a6)                        | +000

| ----------------------------------------------------------------------------
|  MovingPlatform_Init_077228  @ $077228  (58 B)
| ----------------------------------------------------------------------------
        .section .text.MovingPlatform_Init_077228, "ax", @progbits
        .global MovingPlatform_Init_077228
MovingPlatform_Init_077228:
        move.w  0x22(a6),d0                     | +000
        add.w   0x106f50.l,d0                   | +004
        move.w  d0,0x70(a6)                     | +00a
        move.w  0x24(a6),d0                     | +00e
        subi.w  #0x200,d0                       | +012
        neg.w   d0                              | +016
        add.w   0x106f54.l,d0                   | +018
        move.w  d0,0x72(a6)                     | +01e
        move.b  0x98(a6),d0                     | +022
        move.b  d0,0x74(a6)                     | +026
        lea     0x10e2e0.l,a0                   | +02a
        move.b  (a0),d0                         | +030
        addq.b  #0x1,d0                         | +032
        move.b  d0,(a0)                         | +034
        move.b  d0,0x75(a6)                     | +036

| ----------------------------------------------------------------------------
|  MovingPlatform_Run_07726a  @ $07726A  (62 B)
| ----------------------------------------------------------------------------
        .section .text.MovingPlatform_Run_07726a, "ax", @progbits
        .global MovingPlatform_Run_07726a
MovingPlatform_Run_07726a:
        move.w  0x70(a6),d0                     | +000
        sub.w   0x106f50.l,d0                   | +004
        addi.w  #0x40,d0                        | +00a
        subi.w  #0x380,d0                       | +00e
        bcs.w   .L077286                        | +012
        jmp     0x518.l                         | +016
.L077286:
        moveq   #0,d0                           | +01c
        move.b  0x99(a6),d0                     | +01e
        beq.w   .L0772a2                        | +022
        movea.l 0xc(a6),a0                      | +026
        tst.b   (a0,d0.w)                       | +02a
        beq.w   .L0772a2                        | +02e
        jmp     0x518.l                         | +032
.L0772a2:
        bsr.w   Platform_ClearListByScroll_077092__L0770b0 | +038
        rts                                     | +03c

| ----------------------------------------------------------------------------
|  BridgePlank_Table_0772a8  @ $0772A8  (502 B)
| ----------------------------------------------------------------------------
        .section .text.BridgePlank_Table_0772a8, "ax", @progbits
        .global BridgePlank_Table_0772a8
BridgePlank_Table_0772a8:
        .dc.w   0x0007                        | +000  (dato / opcode no decodificado)
        .dc.w   0x8218                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +004  (dato / opcode no decodificado)
        .dc.w   0x8236                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +008  (dato / opcode no decodificado)
        .dc.w   0x8254                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x8272                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +010  (dato / opcode no decodificado)
        .dc.w   0x8290                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +014  (dato / opcode no decodificado)
        .dc.w   0x82ae                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +018  (dato / opcode no decodificado)
        .dc.w   0x8218                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x8236                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +020  (dato / opcode no decodificado)
        .dc.w   0x8254                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +024  (dato / opcode no decodificado)
        .dc.w   0x8272                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +028  (dato / opcode no decodificado)
        .dc.w   0x8290                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x82ae                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +030  (dato / opcode no decodificado)
        .dc.w   0x8218                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +034  (dato / opcode no decodificado)
        .dc.w   0x8236                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +038  (dato / opcode no decodificado)
        .dc.w   0x8254                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x8272                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +040  (dato / opcode no decodificado)
        .dc.w   0x8290                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +044  (dato / opcode no decodificado)
        .dc.w   0x82ae                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +048  (dato / opcode no decodificado)
        .dc.w   0x8218                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x8236                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +056  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +058  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x72a8                        | +060  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +062  (dato / opcode no decodificado)
        .dc.w   0x6f6c                        | +064  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +066  (dato / opcode no decodificado)
        .dc.w   0x8244                        | +068  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +070  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +072  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +074  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +076  (dato / opcode no decodificado)
        .dc.w   0x6f6c                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x8244                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +07e  (dato / opcode no decodificado)
        .dc.w   0x0013                        | +080  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +082  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +084  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +088  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +08a  (dato / opcode no decodificado)
        .dc.w   0x6f6c                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x8244                        | +090  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0012                        | +094  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +096  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +098  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +09a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +09e  (dato / opcode no decodificado)
        .dc.w   0x6f6c                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +0a2  (dato / opcode no decodificado)
        .dc.w   0x8244                        | +0a4  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +0a6  (dato / opcode no decodificado)
        .dc.w   0x0012                        | +0a8  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +0aa  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +0ac  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +0ae  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0b0  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +0b2  (dato / opcode no decodificado)
        .dc.w   0x6f6c                        | +0b4  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +0b6  (dato / opcode no decodificado)
        .dc.w   0x8244                        | +0b8  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +0ba  (dato / opcode no decodificado)
        .dc.w   0x0011                        | +0bc  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +0be  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0c0  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +0c2  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0c4  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +0c6  (dato / opcode no decodificado)
        .dc.w   0x6f6c                        | +0c8  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +0ca  (dato / opcode no decodificado)
        .dc.w   0x8244                        | +0cc  (dato / opcode no decodificado)
        .dc.w   0x000e                        | +0ce  (dato / opcode no decodificado)
        .dc.w   0x0011                        | +0d0  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +0d2  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0d4  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +0d6  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0d8  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +0da  (dato / opcode no decodificado)
        .dc.w   0x6f6c                        | +0dc  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +0de  (dato / opcode no decodificado)
        .dc.w   0x8244                        | +0e0  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +0e2  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +0e4  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +0e6  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +0e8  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +0ea  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +0ec  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +0ee  (dato / opcode no decodificado)
        .dc.w   0x6f6c                        | +0f0  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +0f2  (dato / opcode no decodificado)
        .dc.w   0x8244                        | +0f4  (dato / opcode no decodificado)
        .dc.w   0x0012                        | +0f6  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +0f8  (dato / opcode no decodificado)
        .dc.w   0x000e                        | +0fa  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +0fc  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +0fe  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +100  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +102  (dato / opcode no decodificado)
        .dc.w   0x6f6c                        | +104  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +106  (dato / opcode no decodificado)
        .dc.w   0x8244                        | +108  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +10a  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +10c  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +10e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +110  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +112  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +114  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +116  (dato / opcode no decodificado)
        .dc.w   0x6f6c                        | +118  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +11a  (dato / opcode no decodificado)
        .dc.w   0x8244                        | +11c  (dato / opcode no decodificado)
        .dc.w   0x0016                        | +11e  (dato / opcode no decodificado)
        .dc.w   0x000f                        | +120  (dato / opcode no decodificado)
        .dc.w   0x0012                        | +122  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +124  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +126  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +128  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +12a  (dato / opcode no decodificado)
        .dc.w   0x6f6c                        | +12c  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +12e  (dato / opcode no decodificado)
        .dc.w   0x8244                        | +130  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +132  (dato / opcode no decodificado)
        .dc.w   0x000e                        | +134  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +136  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +138  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +13a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +13c  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +13e  (dato / opcode no decodificado)
        .dc.w   0x6f6c                        | +140  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +142  (dato / opcode no decodificado)
        .dc.w   0x8244                        | +144  (dato / opcode no decodificado)
        .dc.w   0x001a                        | +146  (dato / opcode no decodificado)
        .dc.w   0x000e                        | +148  (dato / opcode no decodificado)
        .dc.w   0x0016                        | +14a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +14c  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +14e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +150  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +152  (dato / opcode no decodificado)
        .dc.w   0x6f6c                        | +154  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +156  (dato / opcode no decodificado)
        .dc.w   0x8244                        | +158  (dato / opcode no decodificado)
        .dc.w   0x001c                        | +15a  (dato / opcode no decodificado)
        .dc.w   0x000d                        | +15c  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +15e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +160  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +162  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +164  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +166  (dato / opcode no decodificado)
        .dc.w   0x6f6c                        | +168  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +16a  (dato / opcode no decodificado)
        .dc.w   0x8244                        | +16c  (dato / opcode no decodificado)
        .dc.w   0x001e                        | +16e  (dato / opcode no decodificado)
        .dc.w   0x000d                        | +170  (dato / opcode no decodificado)
        .dc.w   0x001a                        | +172  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +174  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +176  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +178  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +17a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +17c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +17e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +180  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +182  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +184  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +186  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +188  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +18a  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +18c  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +18e  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +190  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +192  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +194  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +196  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +198  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +19a  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +19c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +19e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1a0  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1a2  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +1a4  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +1a6  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +1a8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1aa  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1ac  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +1ae  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +1b0  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +1b2  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +1b4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1b6  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +1b8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1ba  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +1bc  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +1be  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +1c0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1c2  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +1c4  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +1c6  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +1c8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1ca  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1cc  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1ce  (dato / opcode no decodificado)
        .dc.w   0x000f                        | +1d0  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +1d2  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +1d4  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1d6  (dato / opcode no decodificado)
        .dc.w   0x161e                        | +1d8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1da  (dato / opcode no decodificado)
        .dc.w   0x171e                        | +1dc  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1de  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1e0  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1e2  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1e4  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1e6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1e8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1ea  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +1ec  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +1ee  (dato / opcode no decodificado)
        .dc.w   0x1e00                        | +1f0  (dato / opcode no decodificado)
        .dc.w   0x0015                        | +1f2  (dato / opcode no decodificado)
        .dc.w   0x1e00                        | +1f4  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  BridgePlank_PosFromRecord_07749e  @ $07749E  (78 B)
| ----------------------------------------------------------------------------
        .section .text.BridgePlank_PosFromRecord_07749e, "ax", @progbits
        .global BridgePlank_PosFromRecord_07749e
BridgePlank_PosFromRecord_07749e:
        movea.l 0x70(a6),a0                     | +000
        move.w  0x8(a0),d0                      | +004
        move.w  0xa(a0),d1                      | +008
        lsl.w   #0x4,d0                         | +00c
        lsl.w   #0x4,d1                         | +00e
        jsr     0x44022.l                       | +010
        move.w  d0,0x22(a6)                     | +016
        move.w  d1,0x24(a6)                     | +01a
        rts                                     | +01e
        lea     MovingPlatform_Tmpl143_077224(pc),a1 | +020
        jsr     0x4ae.l                         | +024
        move.w  0x22(a6),d0                     | +02a
        move.w  0x24(a6),d1                     | +02e
        addi.w  #0x20,d0                        | +032
        addi.w  #0x10,d0                        | +036
        subi.w  #0x80,d1                        | +03a
        move.w  d0,0x22(a0)                     | +03e
        move.w  d1,0x24(a0)                     | +042
        move.b  #0x80,0x98(a0)                  | +046
        rts                                     | +04c

| ----------------------------------------------------------------------------
|  BridgePlank_Spawn0_0774ec  @ $0774EC  (20 B)
| ----------------------------------------------------------------------------
        .section .text.BridgePlank_Spawn0_0774ec, "ax", @progbits
        .global BridgePlank_Spawn0_0774ec
BridgePlank_Spawn0_0774ec:
        move.l  #0x7730a,0x70(a6)               | +000
        bsr.b   BridgePlank_PosFromRecord_07749e | +008
        subi.w  #0x10,0x24(a6)                  | +00a
        bra.w   BridgePlank_Spawn6_07756a__L07757c | +010

| ----------------------------------------------------------------------------
|  BridgePlank_Spawn1_077500  @ $077500  (20 B)
| ----------------------------------------------------------------------------
        .section .text.BridgePlank_Spawn1_077500, "ax", @progbits
        .global BridgePlank_Spawn1_077500
BridgePlank_Spawn1_077500:
        move.l  #0x77332,0x70(a6)               | +000
        bsr.b   BridgePlank_PosFromRecord_07749e | +008
        subi.w  #0x20,0x24(a6)                  | +00a
        bra.w   BridgePlank_Spawn6_07756a__L07757c | +010

| ----------------------------------------------------------------------------
|  BridgePlank_Spawn2_077514  @ $077514  (20 B)
| ----------------------------------------------------------------------------
        .section .text.BridgePlank_Spawn2_077514, "ax", @progbits
        .global BridgePlank_Spawn2_077514
BridgePlank_Spawn2_077514:
        move.l  #0x7735a,0x70(a6)               | +000
        bsr.b   BridgePlank_PosFromRecord_07749e | +008
        subi.w  #0x20,0x24(a6)                  | +00a
        bra.w   BridgePlank_Spawn6_07756a__L07757c | +010

| ----------------------------------------------------------------------------
|  BridgePlank_Spawn3_077528  @ $077528  (22 B)
| ----------------------------------------------------------------------------
        .section .text.BridgePlank_Spawn3_077528, "ax", @progbits
        .global BridgePlank_Spawn3_077528
BridgePlank_Spawn3_077528:
        move.l  #0x77382,0x70(a6)               | +000
        bsr.w   BridgePlank_PosFromRecord_07749e | +008
        subi.w  #0x20,0x24(a6)                  | +00c
        bra.w   BridgePlank_Spawn6_07756a__L07757c | +012

| ----------------------------------------------------------------------------
|  BridgePlank_Spawn4_07753e  @ $07753E  (22 B)
| ----------------------------------------------------------------------------
        .section .text.BridgePlank_Spawn4_07753e, "ax", @progbits
        .global BridgePlank_Spawn4_07753e
BridgePlank_Spawn4_07753e:
        move.l  #0x773aa,0x70(a6)               | +000
        bsr.w   BridgePlank_PosFromRecord_07749e | +008
        subi.w  #0x10,0x24(a6)                  | +00c
        bra.w   BridgePlank_Spawn6_07756a__L07757c | +012

| ----------------------------------------------------------------------------
|  BridgePlank_Spawn5_077554  @ $077554  (22 B)
| ----------------------------------------------------------------------------
        .section .text.BridgePlank_Spawn5_077554, "ax", @progbits
        .global BridgePlank_Spawn5_077554
BridgePlank_Spawn5_077554:
        move.l  #0x773d2,0x70(a6)               | +000
        bsr.w   BridgePlank_PosFromRecord_07749e | +008
        subi.w  #0x20,0x24(a6)                  | +00c
        bra.w   BridgePlank_Spawn6_07756a__L07757c | +012

| ----------------------------------------------------------------------------
|  BridgePlank_Spawn6_07756a  @ $07756A  (66 B)
| ----------------------------------------------------------------------------
        .section .text.BridgePlank_Spawn6_07756a, "ax", @progbits
        .global BridgePlank_Spawn6_07756a
BridgePlank_Spawn6_07756a:
        move.l  #0x773fa,0x70(a6)               | +000
        bsr.w   BridgePlank_PosFromRecord_07749e | +008
        subi.w  #0x20,0x24(a6)                  | +00c
        .global BridgePlank_Spawn6_07756a__L07757c
BridgePlank_Spawn6_07756a__L07757c:
.L07757c:
        lea     0xffff.w,a0                     | +012
        move.l  a0,0x48(a6)                     | +016
        move.w  #0x3c,0x66(a6)                  | +01a
        lea     .L077590(pc),a1                 | +020
        move.l  a1,(a6)                         | +024
.L077590:
        jsr     0x2783a.l                       | +026
        move.w  0x22(a6),d0                     | +02c
        bmi.w   .L0775a0                        | +030
        rts                                     | +034
.L0775a0:
        jsr     0x5b6.l                         | +036
        jmp     0x518.l                         | +03c

| ----------------------------------------------------------------------------
|  MuzzleFx_RetGate_0775ac  @ $0775AC  (16 B)
| ----------------------------------------------------------------------------
        .section .text.MuzzleFx_RetGate_0775ac, "ax", @progbits
        .global MuzzleFx_RetGate_0775ac
MuzzleFx_RetGate_0775ac:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_0775c2                    | +00c

| ----------------------------------------------------------------------------
|  MuzzleFx_SpawnC_0775c8  @ $0775C8  (10 B)
| ----------------------------------------------------------------------------
        .section .text.MuzzleFx_SpawnC_0775c8, "ax", @progbits
        .global MuzzleFx_SpawnC_0775c8
MuzzleFx_SpawnC_0775c8:
        lea     MuzzleFx_InitC_0776e2(pc),a1    | +000
        jsr     0x4ae.l                         | +004

| ----------------------------------------------------------------------------
|  MuzzleFx_SpawnB_0775da  @ $0775DA  (10 B)
| ----------------------------------------------------------------------------
        .section .text.MuzzleFx_SpawnB_0775da, "ax", @progbits
        .global MuzzleFx_SpawnB_0775da
MuzzleFx_SpawnB_0775da:
        lea     MuzzleFx_InitB_0776b4(pc),a1    | +000
        jsr     0x4ae.l                         | +004

| ----------------------------------------------------------------------------
|  MuzzleFx_Sprites_0775ec  @ $0775EC  (84 B)
| ----------------------------------------------------------------------------
        .section .text.MuzzleFx_Sprites_0775ec, "ax", @progbits
        .global MuzzleFx_Sprites_0775ec
MuzzleFx_Sprites_0775ec:
        .dc.w   0x0002                        | +000  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +002  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +014  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +020  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +024  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  MuzzleFx_SceneGate_077640  @ $077640  (48 B)
| ----------------------------------------------------------------------------
        .section .text.MuzzleFx_SceneGate_077640, "ax", @progbits
        .global MuzzleFx_SceneGate_077640
MuzzleFx_SceneGate_077640:
        cmpi.b  #0x0,0x106ece.l                 | +000
        beq.w   .L07765e                        | +008
        cmpi.b  #0x9,0x106ece.l                 | +00c
        beq.w   .L07765e                        | +014
        jmp     0x518.l                         | +018
.L07765e:
        jsr     MuzzleFx_SndByScene_077670(pc)  | +01e
        jsr     0x236e.l                        | +022
        move.w  #0xe000,0x38(a6)                | +028
        rts                                     | +02e

| ----------------------------------------------------------------------------
|  MuzzleFx_SndByScene_077670  @ $077670  (22 B)
| ----------------------------------------------------------------------------
        .section .text.MuzzleFx_SndByScene_077670, "ax", @progbits
        .global MuzzleFx_SndByScene_077670
MuzzleFx_SndByScene_077670:
        move.w  #0x10,d1                        | +000
        cmpi.b  #0x5,0x106ece.l                 | +004
        bne.w   .L077684                        | +00c
        move.w  #0x138,d1                       | +010
.L077684:
        rts                                     | +014

| ----------------------------------------------------------------------------
|  MuzzleFx_InitA_077686  @ $077686  (46 B)
| ----------------------------------------------------------------------------
        .section .text.MuzzleFx_InitA_077686, "ax", @progbits
        .global MuzzleFx_InitA_077686
MuzzleFx_InitA_077686:
        jsr     MuzzleFx_SceneGate_077640(pc)   | +000
        move.l  #0x2dcb84,0x74(a6)              | +004
        move.l  #0x2dca62,0x78(a6)              | +00c
        move.l  #0x2dca82,0x7c(a6)              | +014
        move.l  #0x2dca1a,0x80(a6)              | +01c
        lea     MuzzleFx_Track_0778a2(pc),a1    | +024
        move.l  a1,(a6)                         | +028
        bra.w   MuzzleFx_Track_0778a2           | +02a

| ----------------------------------------------------------------------------
|  MuzzleFx_InitB_0776b4  @ $0776B4  (46 B)
| ----------------------------------------------------------------------------
        .section .text.MuzzleFx_InitB_0776b4, "ax", @progbits
        .global MuzzleFx_InitB_0776b4
MuzzleFx_InitB_0776b4:
        jsr     MuzzleFx_SceneGate_077640(pc)   | +000
        move.l  #0x2dca8a,0x74(a6)              | +004
        move.l  #0x2dca4a,0x78(a6)              | +00c
        move.l  #0x2dca6a,0x7c(a6)              | +014
        move.l  #0x2dc98a,0x80(a6)              | +01c
        lea     MuzzleFx_Track_0778a2(pc),a1    | +024
        move.l  a1,(a6)                         | +028
        bra.w   MuzzleFx_Track_0778a2           | +02a

| ----------------------------------------------------------------------------
|  MuzzleFx_InitC_0776e2  @ $0776E2  (46 B)
| ----------------------------------------------------------------------------
        .section .text.MuzzleFx_InitC_0776e2, "ax", @progbits
        .global MuzzleFx_InitC_0776e2
MuzzleFx_InitC_0776e2:
        jsr     MuzzleFx_SceneGate_077640(pc)   | +000
        move.l  #0x2dcabc,0x74(a6)              | +004
        move.l  #0x2dca52,0x78(a6)              | +00c
        move.l  #0x2dca72,0x7c(a6)              | +014
        move.l  #0x2dc9ba,0x80(a6)              | +01c
        lea     MuzzleFx_Track_0778a2(pc),a1    | +024
        move.l  a1,(a6)                         | +028
        bra.w   MuzzleFx_Track_0778a2           | +02a

| ----------------------------------------------------------------------------
|  MuzzleFx_InitD_077710  @ $077710  (46 B)
| ----------------------------------------------------------------------------
        .section .text.MuzzleFx_InitD_077710, "ax", @progbits
        .global MuzzleFx_InitD_077710
MuzzleFx_InitD_077710:
        jsr     MuzzleFx_SceneGate_077640(pc)   | +000
        move.l  #0x2dcaee,0x74(a6)              | +004
        move.l  #0x2dca5a,0x78(a6)              | +00c
        move.l  #0x2dca7a,0x7c(a6)              | +014
        move.l  #0x2dc9ea,0x80(a6)              | +01c
        lea     MuzzleFx_Track_0778a2(pc),a1    | +024
        move.l  a1,(a6)                         | +028
        bra.w   MuzzleFx_Track_0778a2           | +02a

| ----------------------------------------------------------------------------
|  MuzzleFx_InitE_07773e  @ $07773E  (46 B)
| ----------------------------------------------------------------------------
        .section .text.MuzzleFx_InitE_07773e, "ax", @progbits
        .global MuzzleFx_InitE_07773e
MuzzleFx_InitE_07773e:
        jsr     MuzzleFx_SceneGate_077640(pc)   | +000
        move.l  #0x2dcb84,0x74(a6)              | +004
        move.l  #0x2dca62,0x78(a6)              | +00c
        move.l  #0x2dca82,0x7c(a6)              | +014
        move.l  #0x2dca1a,0x80(a6)              | +01c
        lea     MuzzleFx_Track_0778a2(pc),a1    | +024
        move.l  a1,(a6)                         | +028
        bra.w   MuzzleFx_Track_0778a2           | +02a

| ----------------------------------------------------------------------------
|  MuzzleFx_Attach_07776c  @ $07776C  (302 B)
| ----------------------------------------------------------------------------
        .section .text.MuzzleFx_Attach_07776c, "ax", @progbits
        .global MuzzleFx_Attach_07776c
MuzzleFx_Attach_07776c:
        move.w  #0xffff,0x70(a6)                | +000
        lea     .L077778(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L077778:
        jsr     MuzzleFx_SavePos_077c0c(pc)     | +00c
        jsr     MuzzleFx_ReadParentAim_077bc6(pc) | +010
        movea.l 0xc(a6),a2                      | +014
        move.w  0x22(a2),d1                     | +018
        move.w  0x24(a2),d2                     | +01c
        move.w  d1,0x22(a6)                     | +020
        move.w  d2,0x24(a6)                     | +024
        jsr     MuzzleFx_ClampGroundY_077c28(pc) | +028
        clr.w   d0                              | +02c
        tst.w   0x28(a2)                        | +02e
        beq.w   .L0777b2                        | +032
        move.w  #0x4,d0                         | +036
        tst.w   0x28(a2)                        | +03a
        bge.w   .L0777b2                        | +03e
        move.w  #0x8,d0                         | +042
.L0777b2:
        cmpi.b  #0x80,0x72(a6)                  | +046
        bne.w   .L0777fc                        | +04c
        cmpi.b  #0x82,0x88(a6)                  | +050
        beq.w   .L0777d0                        | +056
        cmpi.b  #0x8c,0x88(a6)                  | +05a
        bne.w   .L0777d2                        | +060
.L0777d0:
        addq.w  #0x2,d0                         | +064
.L0777d2:
        movem.w d0,-(a7)                        | +066
        move.w  0x22(a6),d0                     | +06a
        move.w  0x24(a6),d1                     | +06e
        jsr     0x440bc.l                       | +072
        move.w  d0,d2                           | +078
        movem.w (a7)+,d0                        | +07a
        cmpi.w  #0xd20,d2                       | +07e
        bcs.w   .L0777fc                        | +082
        cmpi.w  #0xe58,d2                       | +086
        bcc.w   .L0777fc                        | +08a
        addq.w  #0x1,d0                         | +08e
.L0777fc:
        cmp.w   0x70(a6),d0                     | +090
        beq.w   .L077822                        | +094
        move.w  d0,0x70(a6)                     | +098
        movea.l 0x80(a6),a0                     | +09c
        lsl.w   #0x2,d0                         | +0a0
        movea.l (a0,d0.w),a0                    | +0a2
        cmpa.l  #0xffffffff,a0                  | +0a6
        beq.w   .L077822                        | +0ac
        jsr     0x28cd4.l                       | +0b0
.L077822:
        cmpi.b  #0x0,0x72(a6)                   | +0b6
        beq.w   .L077890                        | +0bc
        cmpi.b  #0xc0,0x72(a6)                  | +0c0
        beq.w   .L077890                        | +0c6
        cmpi.b  #0x1,0x73(a6)                   | +0ca
        beq.w   .L077860                        | +0d0
        jsr     MuzzleFx_SetFlipByDir_077bae(pc) | +0d4
        move.w  0x70(a6),d0                     | +0d8
        lea     0x2dc946.l,a0                   | +0dc
        asl.w   #0x2,d0                         | +0e2
        move.l  (a0,d0.w),0x5c(a6)              | +0e4
        jsr     0x28d70.l                       | +0ea
        bra.w   .L077890                        | +0f0
.L077860:
        move.w  0x84(a6),d1                     | +0f4
        move.w  0x86(a6),d2                     | +0f8
        lea     MuzzleFx_Track_0778a2(pc),a1    | +0fc
        move.l  a1,(a6)                         | +100
        lea     MuzzleFx_Detached_077a16(pc),a1 | +102
        jsr     0x6fe.l                         | +106
        jsr     0x5dd02.l                       | +10c
        move.w  0x88(a6),0x88(a0)               | +112
        move.b  0x12(a6),d0                     | +118
        andi.b  #0x40,d0                        | +11c
        or.b    d0,0x12(a0)                     | +120
.L077890:
        jsr     0x7b2.l                         | +124
        bcc.w   SetHandlerRts_0778a0            | +12a

| ----------------------------------------------------------------------------
|  MuzzleFx_Track_0778a2  @ $0778A2  (82 B)
| ----------------------------------------------------------------------------
        .section .text.MuzzleFx_Track_0778a2, "ax", @progbits
        .global MuzzleFx_Track_0778a2
MuzzleFx_Track_0778a2:
        jsr     MuzzleFx_SavePos_077c0c(pc)     | +000
        movea.l 0xc(a6),a2                      | +004
        move.w  0x22(a2),d1                     | +008
        move.w  0x24(a2),d2                     | +00c
        move.w  d1,0x22(a6)                     | +010
        move.w  d2,0x24(a6)                     | +014
        jsr     MuzzleFx_ReadParentAim_077bc6(pc) | +018
        bcc.w   .L0778e0                        | +01c
        cmpi.b  #0x40,0x72(a6)                  | +020
        beq.w   .L0778ea                        | +026
        cmpi.b  #0x80,0x72(a6)                  | +02a
        beq.w   .L0778ea                        | +030
        lea     MuzzleFx_WaitAim_0778fc(pc),a1  | +034
        move.l  a1,(a6)                         | +038
        bra.w   .L0778ea                        | +03a
.L0778e0:
        jsr     MuzzleFx_SetFlipByDir_077bae(pc) | +03e
        lea     MuzzleFx_Flash_077912(pc),a1    | +042
        move.l  a1,(a6)                         | +046
        .global MuzzleFx_Track_0778a2__L0778ea
MuzzleFx_Track_0778a2__L0778ea:
.L0778ea:
        jsr     0x7b2.l                         | +048
        bcc.w   SetHandlerRts_0778fa            | +04e

| ----------------------------------------------------------------------------
|  MuzzleFx_WaitAim_0778fc  @ $0778FC  (22 B)
| ----------------------------------------------------------------------------
        .section .text.MuzzleFx_WaitAim_0778fc, "ax", @progbits
        .global MuzzleFx_WaitAim_0778fc
MuzzleFx_WaitAim_0778fc:
        jsr     MuzzleFx_ReadParentAim_077bc6(pc) | +000
        bcs.w   .L07790e                        | +004
        jsr     MuzzleFx_SetFlipByDir_077bae(pc) | +008
        lea     MuzzleFx_Flash_077912(pc),a1    | +00c
        move.l  a1,(a6)                         | +010
.L07790e:
        jmp     MuzzleFx_Track_0778a2__L0778ea(pc) | +012

| ----------------------------------------------------------------------------
|  MuzzleFx_Flash_077912  @ $077912  (252 B)
| ----------------------------------------------------------------------------
        .section .text.MuzzleFx_Flash_077912, "ax", @progbits
        .global MuzzleFx_Flash_077912
MuzzleFx_Flash_077912:
        jsr     MuzzleFx_ReadParentAim_077bc6(pc) | +000
        clr.w   d0                              | +004
        cmpi.b  #0x80,0x72(a6)                  | +006
        bne.w   .L077938                        | +00c
        cmpi.b  #0x82,0x88(a6)                  | +010
        beq.w   .L077936                        | +016
        cmpi.b  #0x8c,0x88(a6)                  | +01a
        bne.w   .L077938                        | +020
.L077936:
        addq.w  #0x1,d0                         | +024
.L077938:
        movea.l 0x7c(a6),a0                     | +026
        lsl.w   #0x2,d0                         | +02a
        movea.l (a0,d0.w),a0                    | +02c
        cmpa.l  #0xffffffff,a0                  | +030
        beq.w   .L077952                        | +036
        jsr     0x28cd4.l                       | +03a
.L077952:
        lea     .L077958(pc),a1                 | +040
        move.l  a1,(a6)                         | +044
.L077958:
        jsr     MuzzleFx_SavePos_077c0c(pc)     | +046
        movea.l 0xc(a6),a2                      | +04a
        move.w  0x22(a2),d1                     | +04e
        move.w  0x24(a2),d2                     | +052
        move.w  d1,0x22(a6)                     | +056
        move.w  d2,0x24(a6)                     | +05a
        jsr     MuzzleFx_ReadParentAim_077bc6(pc) | +05e
        bcc.w   .L0779d8                        | +062
        jsr     MuzzleFx_ClampGroundY_077c28(pc) | +066
        lea     MuzzleFx_Track_0778a2(pc),a1    | +06a
        move.l  a1,(a6)                         | +06e
        move.w  0x84(a6),d1                     | +070
        jsr     MuzzleFx_ReadParentAim_077bc6(pc) | +074
        cmpi.b  #0x0,0x72(a6)                   | +078
        beq.w   .L077a04                        | +07e
        cmpi.b  #0xc0,0x72(a6)                  | +082
        beq.w   .L077a04                        | +088
        lea     MuzzleFx_Detached_077a16(pc),a1 | +08c
        jsr     0x6fe.l                         | +090
        jsr     0x5dd02.l                       | +096
        movea.l 0xc(a6),a2                      | +09c
        move.w  0x34(a2),0x34(a0)               | +0a0
        move.w  0x88(a6),0x88(a0)               | +0a6
        bclr    #0x6,0x12(a0)                   | +0ac
        btst    #0x6,0x12(a6)                   | +0b2
        beq.w   .L077a04                        | +0b8
        bset    #0x6,0x12(a0)                   | +0bc
        bra.w   .L077a04                        | +0c2
.L0779d8:
        cmpi.b  #0x0,0x72(a6)                   | +0c6
        beq.w   .L077a04                        | +0cc
        cmpi.b  #0xc0,0x72(a6)                  | +0d0
        beq.w   .L077a04                        | +0d6
        jsr     MuzzleFx_ClampGroundY_077c28(pc) | +0da
        jsr     MuzzleFx_SetFlipByDir_077bae(pc) | +0de
        jsr     0x28d70.l                       | +0e2
        bcc.w   .L077a04                        | +0e8
        lea     MuzzleFx_Attach_07776c(pc),a1   | +0ec
        move.l  a1,(a6)                         | +0f0
.L077a04:
        jsr     0x7b2.l                         | +0f2
        bcc.w   SetHandlerRts_077a14            | +0f8

| ----------------------------------------------------------------------------
|  MuzzleFx_Detached_077a16  @ $077A16  (112 B)
| ----------------------------------------------------------------------------
        .section .text.MuzzleFx_Detached_077a16, "ax", @progbits
        .global MuzzleFx_Detached_077a16
MuzzleFx_Detached_077a16:
        clr.w   d0                              | +000
        cmpi.b  #0x82,0x88(a6)                  | +002
        beq.w   .L077a2e                        | +008
        cmpi.b  #0x8c,0x88(a6)                  | +00c
        bne.w   .L077a2e                        | +012
        addq.w  #0x1,d0                         | +016
.L077a2e:
        movea.l 0xc(a6),a2                      | +018
        movea.l 0x78(a2),a0                     | +01c
        .global MuzzleFx_Detached_077a16__L077a36
MuzzleFx_Detached_077a16__L077a36:
.L077a36:
        lsl.w   #0x2,d0                         | +020
        movea.l (a0,d0.w),a0                    | +022
        cmpa.l  #0xffffffff,a0                  | +026
        beq.w   .L077a4c                        | +02c
        jsr     0x28cd4.l                       | +030
.L077a4c:
        jsr     MuzzleFx_SndByScene_077670(pc)  | +036
        jsr     0x236e.l                        | +03a
        addq.w  #0x1,0x38(a6)                   | +040
        lea     .L077a60(pc),a1                 | +044
        move.l  a1,(a6)                         | +048
.L077a60:
        jsr     0x2783a.l                       | +04a
        jsr     0x28d70.l                       | +050
        bcc.w   .L077a76                        | +056
        lea     JmpToScheduler_077a8e(pc),a1    | +05a
        move.l  a1,(a6)                         | +05e
.L077a76:
        movea.l #0xffffffff,a0                  | +060
        jsr     0x5dd56.l                       | +066
        bcc.w   SetHandlerRts_077a8c            | +06c

| ----------------------------------------------------------------------------
|  MuzzleFx_ShotA_077a96  @ $077A96  (16 B)
| ----------------------------------------------------------------------------
        .section .text.MuzzleFx_ShotA_077a96, "ax", @progbits
        .global MuzzleFx_ShotA_077a96
MuzzleFx_ShotA_077a96:
        jsr     MuzzleFx_SceneGate_077640(pc)   | +000
        move.l  #0x2dca6a,0x78(a6)              | +004
        jmp     MuzzleFx_Shot_Run_077ad6(pc)    | +00c

| ----------------------------------------------------------------------------
|  MuzzleFx_ShotB_077aa6  @ $077AA6  (16 B)
| ----------------------------------------------------------------------------
        .section .text.MuzzleFx_ShotB_077aa6, "ax", @progbits
        .global MuzzleFx_ShotB_077aa6
MuzzleFx_ShotB_077aa6:
        jsr     MuzzleFx_SceneGate_077640(pc)   | +000
        move.l  #0x2dca72,0x78(a6)              | +004
        jmp     MuzzleFx_Shot_Run_077ad6(pc)    | +00c

| ----------------------------------------------------------------------------
|  MuzzleFx_ShotC_077ab6  @ $077AB6  (16 B)
| ----------------------------------------------------------------------------
        .section .text.MuzzleFx_ShotC_077ab6, "ax", @progbits
        .global MuzzleFx_ShotC_077ab6
MuzzleFx_ShotC_077ab6:
        jsr     MuzzleFx_SceneGate_077640(pc)   | +000
        move.l  #0x2dca7a,0x78(a6)              | +004
        jmp     MuzzleFx_Shot_Run_077ad6(pc)    | +00c

| ----------------------------------------------------------------------------
|  MuzzleFx_ShotD_077ac6  @ $077AC6  (16 B)
| ----------------------------------------------------------------------------
        .section .text.MuzzleFx_ShotD_077ac6, "ax", @progbits
        .global MuzzleFx_ShotD_077ac6
MuzzleFx_ShotD_077ac6:
        jsr     MuzzleFx_SceneGate_077640(pc)   | +000
        move.l  #0x2dca82,0x78(a6)              | +004
        jmp     MuzzleFx_Shot_Run_077ad6(pc)    | +00c

| ----------------------------------------------------------------------------
|  MuzzleFx_Shot_Run_077ad6  @ $077AD6  (138 B)
| ----------------------------------------------------------------------------
        .section .text.MuzzleFx_Shot_Run_077ad6, "ax", @progbits
        .global MuzzleFx_Shot_Run_077ad6
MuzzleFx_Shot_Run_077ad6:
        jsr     MuzzleFx_SavePos_077c0c(pc)     | +000
        movea.l 0xc(a6),a2                      | +004
        move.w  0x22(a2),d1                     | +008
        move.w  0x24(a2),d2                     | +00c
        move.w  d1,0x22(a6)                     | +010
        move.w  d2,0x24(a6)                     | +014
        jsr     MuzzleFx_ReadParentAim_077bc6(pc) | +018
        bcs.w   .L077b14                        | +01c
        cmpi.b  #0x40,0x72(a6)                  | +020
        beq.w   .L077b28                        | +026
        cmpi.b  #0x80,0x72(a6)                  | +02a
        beq.w   .L077b2c                        | +030
        lea     JmpToScheduler_077a8e(pc),a1    | +034
        move.l  a1,(a6)                         | +038
        bra.w   .L077b40                        | +03a
.L077b14:
        cmpi.b  #0x80,0x72(a6)                  | +03e
        beq.w   .L077b2c                        | +044
        cmpi.b  #0x40,0x72(a6)                  | +048
        bne.w   .L077b40                        | +04e
.L077b28:
        jsr     MuzzleFx_ClampGroundY_077c28(pc) | +052
.L077b2c:
        lea     MuzzleFx_Shot_Fire_077b68(pc),a1 | +056
        move.l  a1,(a6)                         | +05a
        movea.l 0xc(a6),a2                      | +05c
        move.w  0x34(a2),0x34(a6)               | +060
        jsr     MuzzleFx_SetFlipByDir_077bae(pc) | +066
.L077b40:
        movea.l #0xffffffff,a0                  | +06a
        jsr     0x5dd56.l                       | +070
        bcc.w   .L077b56                        | +076
        lea     JmpToScheduler_077a8e(pc),a1    | +07a
        move.l  a1,(a6)                         | +07e
.L077b56:
        jsr     0x7b2.l                         | +080
        bcc.w   SetHandlerRts_077b66            | +086

| ----------------------------------------------------------------------------
|  MuzzleFx_Shot_Fire_077b68  @ $077B68  (70 B)
| ----------------------------------------------------------------------------
        .section .text.MuzzleFx_Shot_Fire_077b68, "ax", @progbits
        .global MuzzleFx_Shot_Fire_077b68
MuzzleFx_Shot_Fire_077b68:
        cmpi.l  #0x2dca6a,0x78(a6)              | +000
        bne.w   .L077b8e                        | +008
        cmpi.b  #0x80,0x72(a6)                  | +00c
        beq.w   .L077b8e                        | +012
        jsr     0x5e9b6.l                       | +016
        andi.w  #0xf,d0                         | +01c
        subq.w  #0x8,d0                         | +020
        add.w   d0,0x22(a6)                     | +022
.L077b8e:
        clr.w   d0                              | +026
        cmpi.b  #0x82,0x88(a6)                  | +028
        beq.w   .L077ba4                        | +02e
        cmpi.b  #0x8c,0x88(a6)                  | +032
        bne.w   .L077ba6                        | +038
.L077ba4:
        addq.w  #0x1,d0                         | +03c
.L077ba6:
        movea.l 0x78(a6),a0                     | +03e
        bra.w   MuzzleFx_Detached_077a16__L077a36 | +042

| ----------------------------------------------------------------------------
|  MuzzleFx_SetFlipByDir_077bae  @ $077BAE  (24 B)
| ----------------------------------------------------------------------------
        .section .text.MuzzleFx_SetFlipByDir_077bae, "ax", @progbits
        .global MuzzleFx_SetFlipByDir_077bae
MuzzleFx_SetFlipByDir_077bae:
        bset    #0x6,0x12(a6)                   | +000
        cmpi.b  #0x80,0x72(a6)                  | +006
        bne.w   .L077bc4                        | +00c
        bclr    #0x6,0x12(a6)                   | +010
.L077bc4:
        rts                                     | +016

| ----------------------------------------------------------------------------
|  MuzzleFx_ReadParentAim_077bc6  @ $077BC6  (38 B)
| ----------------------------------------------------------------------------
        .section .text.MuzzleFx_ReadParentAim_077bc6, "ax", @progbits
        .global MuzzleFx_ReadParentAim_077bc6
MuzzleFx_ReadParentAim_077bc6:
        movem.l a6,-(a7)                        | +000
        movea.l 0xc(a6),a6                      | +004
        jsr     0x27fac.l                       | +008
        movem.l (a7)+,a6                        | +00e
        bcc.w   MuzzleFx_StoreAim_077bf2        | +012
        move.b  d7,0x72(a6)                     | +016
        andi.b  #0xc0,0x72(a6)                  | +01a
        move.b  #0x1,0x73(a6)                   | +020

| ----------------------------------------------------------------------------
|  MuzzleFx_StoreAim_077bf2  @ $077BF2  (20 B)
| ----------------------------------------------------------------------------
        .section .text.MuzzleFx_StoreAim_077bf2, "ax", @progbits
        .global MuzzleFx_StoreAim_077bf2
MuzzleFx_StoreAim_077bf2:
        move.b  d7,0x72(a6)                     | +000
        move.b  d7,0x88(a6)                     | +004
        andi.b  #0xc0,0x72(a6)                  | +008
        move.b  #0x0,0x73(a6)                   | +00e

| ----------------------------------------------------------------------------
|  MuzzleFx_SavePos_077c0c  @ $077C0C  (14 B)
| ----------------------------------------------------------------------------
        .section .text.MuzzleFx_SavePos_077c0c, "ax", @progbits
        .global MuzzleFx_SavePos_077c0c
MuzzleFx_SavePos_077c0c:
        move.w  0x22(a6),0x84(a6)               | +000
        move.w  0x24(a6),0x86(a6)               | +006
        rts                                     | +00c

| ----------------------------------------------------------------------------
|  MuzzleFx_IsDirUp_077c1a  @ $077C1A  (12 B)
| ----------------------------------------------------------------------------
        .section .text.MuzzleFx_IsDirUp_077c1a, "ax", @progbits
        .global MuzzleFx_IsDirUp_077c1a
MuzzleFx_IsDirUp_077c1a:
        cmpi.b  #0x80,0x72(a6)                  | +000
        beq.w   Stub_00077C26                   | +006
        rts                                     | +00a

| ----------------------------------------------------------------------------
|  MuzzleFx_ClampGroundY_077c28  @ $077C28  (58 B)
| ----------------------------------------------------------------------------
        .section .text.MuzzleFx_ClampGroundY_077c28, "ax", @progbits
        .global MuzzleFx_ClampGroundY_077c28
MuzzleFx_ClampGroundY_077c28:
        cmpi.b  #0x40,0x72(a6)                  | +000
        bne.w   .L077c60                        | +006
        move.w  #0x0,d0                         | +00a
        move.w  #0x167,d1                       | +00e
        cmpi.b  #0x6,0x106ece.l                 | +012
        beq.w   .L077c52                        | +01a
        cmpi.b  #0x9,0x106ece.l                 | +01e
        bne.w   .L077c56                        | +026
.L077c52:
        move.w  #0x1ef,d1                       | +02a
.L077c56:
        jsr     0x440d0.l                       | +02e
        move.w  d1,0x24(a6)                     | +034
.L077c60:
        rts                                     | +038

| ----------------------------------------------------------------------------
|  MuzzleFx_RetGateB_077c62  @ $077C62  (16 B)
| ----------------------------------------------------------------------------
        .section .text.MuzzleFx_RetGateB_077c62, "ax", @progbits
        .global MuzzleFx_RetGateB_077c62
MuzzleFx_RetGateB_077c62:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_077c78                    | +00c

| ----------------------------------------------------------------------------
|  Spawner_Handler_077c98  @ $077C98  (236 B)
| ----------------------------------------------------------------------------
        .section .text.Spawner_Handler_077c98, "ax", @progbits
        .global Spawner_Handler_077c98
Spawner_Handler_077c98:
        movea.l 0x70(a6),a0                     | +000
        move.b  0x1(a0),0x74(a6)                | +004
        move.w  0x2(a0),0x76(a6)                | +00a
        lea     .L077cae(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L077cae:
        jsr     0x44048.l                       | +016
        movea.l 0xc(a6),a0                      | +01c
        move.b  0x32(a0),0x32(a6)               | +020
        move.b  0x33(a0),0x33(a6)               | +026
        cmpi.b  #0xff,0x74(a6)                  | +02c
        beq.w   .L077ce0                        | +032
        subq.b  #0x1,0x74(a6)                   | +036
        bne.w   .L077d76                        | +03a
        movea.l 0x70(a6),a2                     | +03e
        move.b  0x1(a2),0x74(a6)                | +042
.L077ce0:
        movea.l 0x70(a6),a2                     | +048
        cmpi.b  #0x0,(a2)                       | +04c
        beq.w   .L077cf0                        | +050
        jsr     Spawner_OffsetByParent_077da6(pc) | +054
.L077cf0:
        movea.l 0x70(a6),a2                     | +058
        movea.l 0xe(a2),a1                      | +05c
        cmpi.b  #0x0,0xd(a2)                    | +060
        beq.w   .L077d18                        | +066
        move.w  #0x10,d0                        | +06a
        jsr     0x5e9e4.l                       | +06e
        asl.w   #0x2,d0                         | +074
        andi.l  #0xffff,d0                      | +076
        movea.l (a1,d0.w),a1                    | +07c
.L077d18:
        jsr     0x6fe.l                         | +080
        jsr     0x5dd02.l                       | +086
        move.w  0x6(a2),d0                      | +08c
        jsr     0x5e9e4.l                       | +090
        jsr     Spawner_NegIfLeft_077de4(pc)    | +096
        add.w   d0,0x22(a0)                     | +09a
        move.w  0xa(a2),d0                      | +09e
        jsr     0x5e9e4.l                       | +0a2
        jsr     Spawner_NegIfLeftB_077df2(pc)   | +0a8
        add.w   d0,0x24(a0)                     | +0ac
        move.b  0x12(a6),d0                     | +0b0
        andi.b  #0x40,d0                        | +0b4
        or.b    d0,0x12(a0)                     | +0b8
        move.b  0x32(a6),0x32(a0)               | +0bc
        move.b  0x33(a6),0x33(a0)               | +0c2
        cmpi.b  #0xff,0x74(a6)                  | +0c8
        bne.w   .L077d76                        | +0ce
        subq.w  #0x1,0x76(a6)                   | +0d2
        bne.w   .L077cf0                        | +0d6
        bra.w   .L077d7e                        | +0da
.L077d76:
        subq.w  #0x1,0x76(a6)                   | +0de
        bne.w   Stub_00077D86                   | +0e2
.L077d7e:
        jmp     0x518.l                         | +0e6

| ----------------------------------------------------------------------------
|  Spawner_Rts_077d84  @ $077D84  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Spawner_Rts_077d84, "ax", @progbits
        .global Spawner_Rts_077d84
Spawner_Rts_077d84:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  Spawner_SpawnAtGoal_077d88  @ $077D88  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Spawner_SpawnAtGoal_077d88, "ax", @progbits
        .global Spawner_SpawnAtGoal_077d88
Spawner_SpawnAtGoal_077d88:
        lea     Explosion_Small_077e10(pc),a1   | +000
        jsr     0x6fe.l                         | +004
        jsr     0x5dd02.l                       | +00a
        move.w  0x54(a6),0x22(a0)               | +010
        move.w  0x56(a6),0x24(a0)               | +016
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  Spawner_OffsetByParent_077da6  @ $077DA6  (56 B)
| ----------------------------------------------------------------------------
        .section .text.Spawner_OffsetByParent_077da6, "ax", @progbits
        .global Spawner_OffsetByParent_077da6
Spawner_OffsetByParent_077da6:
        movea.l 0x70(a6),a0                     | +000
        movea.l 0xc(a6),a1                      | +004
        move.w  0x22(a1),d0                     | +008
        move.w  0x4(a0),d1                      | +00c
        btst    #0x0,0x3a(a1)                   | +010
        beq.w   .L077dc2                        | +016
        neg.w   d1                              | +01a
.L077dc2:
        add.w   d1,d0                           | +01c
        move.w  d0,0x22(a6)                     | +01e
        move.w  0x24(a1),d0                     | +022
        move.w  0x8(a0),d1                      | +026
        btst    #0x0,0x3a(a1)                   | +02a
        beq.w   .L077ddc                        | +030
        neg.w   d1                              | +034
.L077ddc:
        add.w   d1,d0                           | +036

| ----------------------------------------------------------------------------
|  Spawner_NegIfLeft_077de4  @ $077DE4  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Spawner_NegIfLeft_077de4, "ax", @progbits
        .global Spawner_NegIfLeft_077de4
Spawner_NegIfLeft_077de4:
        btst    #0x0,0x3a(a6)                   | +000
        beq.w   .L077df0                        | +006
        neg.w   d0                              | +00a
.L077df0:
        rts                                     | +00c

| ----------------------------------------------------------------------------
|  Spawner_NegIfLeftB_077df2  @ $077DF2  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Spawner_NegIfLeftB_077df2, "ax", @progbits
        .global Spawner_NegIfLeftB_077df2
Spawner_NegIfLeftB_077df2:
        btst    #0x1,0x3a(a6)                   | +000
        beq.w   .L077dfe                        | +006
        neg.w   d0                              | +00a
.L077dfe:
        rts                                     | +00c

| ----------------------------------------------------------------------------
|  Fx_RandFlipFacing_077e00  @ $077E00  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Fx_RandFlipFacing_077e00, "ax", @progbits
        .global Fx_RandFlipFacing_077e00
Fx_RandFlipFacing_077e00:
        jsr     0x5e9b6.l                       | +000
        andi.b  #0x1,d0                         | +006
        or.b    d0,0x3a(a6)                     | +00a
        rts                                     | +00e

| ----------------------------------------------------------------------------
|  Explosion_Small_077e10  @ $077E10  (100 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Small_077e10, "ax", @progbits
        .global Explosion_Small_077e10
Explosion_Small_077e10:
        move.w  #0x4,d1                         | +000
        jsr     0x236e.l                        | +004
        move.b  #0xff,0x32(a6)                  | +00a
        move.b  #0xff,0x33(a6)                  | +010
        jsr     Fx_RandFlipFacing_077e00(pc)    | +016
        lea     0x2dd2d4.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        lea     0x2df1f2.l,a0                   | +026
        jsr     0x5dd56.l                       | +02c
        bcs.w   .L077e6c                        | +032
        lea     .L077e4c(pc),a1                 | +036
        move.l  a1,(a6)                         | +03a
.L077e4c:
        jsr     0x44048.l                       | +03c
        jsr     0x28d70.l                       | +042
        bcs.w   .L077e6c                        | +048
        lea     0x2df1f2.l,a0                   | +04c
        jsr     0x5dd56.l                       | +052
        bcc.w   .L077e72                        | +058
.L077e6c:
        jmp     0x518.l                         | +05c
.L077e72:
        rts                                     | +062

| ----------------------------------------------------------------------------
|  Explosion_Big_077e74  @ $077E74  (102 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Big_077e74, "ax", @progbits
        .global Explosion_Big_077e74
Explosion_Big_077e74:
        lea     0x2df066.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df22e.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0xd,d1                         | +016
        jsr     0x236e.l                        | +01a
        move.b  #0xff,0x32(a6)                  | +020
        move.b  #0xff,0x33(a6)                  | +026
        movea.l 0x70(a6),a0                     | +02c
        jsr     0x5dd56.l                       | +030
        bcs.w   Explosion_FireFront_0780ae__L078128 | +036
        lea     .L077eb4(pc),a1                 | +03a
        move.l  a1,(a6)                         | +03e
.L077eb4:
        jsr     0x44048.l                       | +040
        jsr     0x28d70.l                       | +046
        bcs.w   .L077ed2                        | +04c
        movea.l 0x70(a6),a0                     | +050
        jsr     0x5dd56.l                       | +054
        bcc.w   .L077ed8                        | +05a
.L077ed2:
        jmp     0x518.l                         | +05e
.L077ed8:
        rts                                     | +064

| ----------------------------------------------------------------------------
|  Explosion_Water_077eda  @ $077EDA  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Water_077eda, "ax", @progbits
        .global Explosion_Water_077eda
Explosion_Water_077eda:
        lea     0x2df180.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df1fc.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0x16c,d1                       | +016
        jsr     0x236e.l                        | +01a
        bra.w   Explosion_FireFront_0780ae__L0780e2 | +020

| ----------------------------------------------------------------------------
|  Explosion_Std_077efe  @ $077EFE  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Std_077efe, "ax", @progbits
        .global Explosion_Std_077efe
Explosion_Std_077efe:
        lea     0x2dd4ba.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df1fc.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0x4,d1                         | +016
        jsr     0x236e.l                        | +01a
        bra.w   Explosion_FireFront_0780ae__L0780de | +020

| ----------------------------------------------------------------------------
|  Explosion_StdSndB_077f22  @ $077F22  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_StdSndB_077f22, "ax", @progbits
        .global Explosion_StdSndB_077f22
Explosion_StdSndB_077f22:
        lea     0x2dd4ba.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df1fc.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0x12d,d1                       | +016
        jsr     0x236e.l                        | +01a
        bra.w   Explosion_FireFront_0780ae__L0780de | +020

| ----------------------------------------------------------------------------
|  Explosion_StdB_077f46  @ $077F46  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_StdB_077f46, "ax", @progbits
        .global Explosion_StdB_077f46
Explosion_StdB_077f46:
        lea     0x2dd4d6.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df1fc.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0x145,d1                       | +016
        jsr     0x236e.l                        | +01a
        bra.w   Explosion_FireFront_0780ae__L0780de | +020

| ----------------------------------------------------------------------------
|  Explosion_Fire_077f6a  @ $077F6A  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Fire_077f6a, "ax", @progbits
        .global Explosion_Fire_077f6a
Explosion_Fire_077f6a:
        lea     0x2dd594.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df206.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0x4,d1                         | +016
        jsr     0x236e.l                        | +01a
        bra.w   Explosion_FireFront_0780ae__L0780de | +020

| ----------------------------------------------------------------------------
|  Explosion_FireSndB_077f8e  @ $077F8E  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_FireSndB_077f8e, "ax", @progbits
        .global Explosion_FireSndB_077f8e
Explosion_FireSndB_077f8e:
        lea     0x2dd594.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df206.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0x82,d1                        | +016
        jsr     0x236e.l                        | +01a
        bra.w   Explosion_FireFront_0780ae__L0780de | +020

| ----------------------------------------------------------------------------
|  Explosion_Flash_077fb2  @ $077FB2  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Flash_077fb2, "ax", @progbits
        .global Explosion_Flash_077fb2
Explosion_Flash_077fb2:
        lea     0x2dd37e.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df210.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0x82,d1                        | +016
        jsr     0x236e.l                        | +01a
        bra.w   Explosion_FireFront_0780ae__L0780de | +020

| ----------------------------------------------------------------------------
|  Explosion_FlashSndB_077fd6  @ $077FD6  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_FlashSndB_077fd6, "ax", @progbits
        .global Explosion_FlashSndB_077fd6
Explosion_FlashSndB_077fd6:
        lea     0x2dd37e.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df210.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0x4,d1                         | +016
        jsr     0x236e.l                        | +01a
        bra.w   Explosion_FireFront_0780ae__L0780de | +020
        lea     0x2dd39a.l,a0                   | +024
        jsr     0x28cd4.l                       | +02a
        lea     0x2df210.l,a0                   | +030
        move.l  a0,0x70(a6)                     | +036
        move.w  #0x4,d1                         | +03a
        jsr     0x236e.l                        | +03e
        bra.w   Explosion_FireFront_0780ae__L0780de | +044

| ----------------------------------------------------------------------------
|  Explosion_Spark_07801e  @ $07801E  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Spark_07801e, "ax", @progbits
        .global Explosion_Spark_07801e
Explosion_Spark_07801e:
        lea     0x2dd6a6.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df210.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0x4,d1                         | +016
        jsr     0x236e.l                        | +01a
        bra.w   Explosion_FireFront_0780ae__L0780de | +020

| ----------------------------------------------------------------------------
|  Explosion_SmokePuff_078042  @ $078042  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_SmokePuff_078042, "ax", @progbits
        .global Explosion_SmokePuff_078042
Explosion_SmokePuff_078042:
        lea     0x2dd6d2.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df1fc.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0x4,d1                         | +016
        jsr     0x236e.l                        | +01a
        bra.w   Explosion_FireFront_0780ae__L0780de | +020

| ----------------------------------------------------------------------------
|  Explosion_FireLoud_078066  @ $078066  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_FireLoud_078066, "ax", @progbits
        .global Explosion_FireLoud_078066
Explosion_FireLoud_078066:
        lea     0x2dd594.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df206.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0x6c,d1                        | +016
        jsr     0x236e.l                        | +01a
        bra.w   Explosion_FireFront_0780ae__L0780de | +020

| ----------------------------------------------------------------------------
|  Explosion_FlashLoud_07808a  @ $07808A  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_FlashLoud_07808a, "ax", @progbits
        .global Explosion_FlashLoud_07808a
Explosion_FlashLoud_07808a:
        lea     0x2dd37e.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df210.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0x6c,d1                        | +016
        jsr     0x236e.l                        | +01a
        bra.w   Explosion_FireFront_0780ae__L0780de | +020

| ----------------------------------------------------------------------------
|  Explosion_FireFront_0780ae  @ $0780AE  (130 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_FireFront_0780ae, "ax", @progbits
        .global Explosion_FireFront_0780ae
Explosion_FireFront_0780ae:
        lea     0x2dd594.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df206.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0x4,d1                         | +016
        jsr     0x236e.l                        | +01a
        andi.w  #0xfff,0x38(a6)                 | +020
        ori.w   #0xc000,0x38(a6)                | +026
        bra.w   .L0780de                        | +02c
        .global Explosion_FireFront_0780ae__L0780de
Explosion_FireFront_0780ae__L0780de:
.L0780de:
        jsr     Fx_RandFlipFacing_077e00(pc)    | +030
        .global Explosion_FireFront_0780ae__L0780e2
Explosion_FireFront_0780ae__L0780e2:
.L0780e2:
        move.b  #0xff,0x32(a6)                  | +034
        move.b  #0xff,0x33(a6)                  | +03a
        movea.l 0x70(a6),a0                     | +040
        jsr     0x5dd56.l                       | +044
        bcs.w   .L078128                        | +04a
        lea     0xffff.w,a0                     | +04e
        move.l  a0,0x48(a6)                     | +052
        lea     .L07810a(pc),a1                 | +056
        move.l  a1,(a6)                         | +05a
.L07810a:
        jsr     0x44048.l                       | +05c
        jsr     0x28d70.l                       | +062
        bcs.w   .L078128                        | +068
        movea.l 0x70(a6),a0                     | +06c
        jsr     0x5dd56.l                       | +070
        bcc.w   .L07812e                        | +076
        .global Explosion_FireFront_0780ae__L078128
Explosion_FireFront_0780ae__L078128:
.L078128:
        jmp     0x518.l                         | +07a
.L07812e:
        rts                                     | +080

| ----------------------------------------------------------------------------
|  Explosion_StdAttack_078130  @ $078130  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_StdAttack_078130, "ax", @progbits
        .global Explosion_StdAttack_078130
Explosion_StdAttack_078130:
        lea     0x2dd4ba.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df1fc.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        lea     0x2df256.l,a0                   | +016
        move.l  a0,0x4c(a6)                     | +01c
        jsr     0x283ca.l                       | +020
        bra.w   Explosion_FireAttack_07815a__L0781aa | +026

| ----------------------------------------------------------------------------
|  Explosion_FireAttack_07815a  @ $07815A  (182 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_FireAttack_07815a, "ax", @progbits
        .global Explosion_FireAttack_07815a
Explosion_FireAttack_07815a:
        lea     0x2dd594.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df206.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        lea     0x2df2fa.l,a0                   | +016
        move.l  a0,0x4c(a6)                     | +01c
        jsr     0x283ca.l                       | +020
        bra.w   .L0781aa                        | +026
        lea     0x2dd37e.l,a0                   | +02a
        jsr     0x28cd4.l                       | +030
        lea     0x2df210.l,a0                   | +036
        move.l  a0,0x70(a6)                     | +03c
        lea     0x2df39e.l,a0                   | +040
        move.l  a0,0x4c(a6)                     | +046
        jsr     0x283ca.l                       | +04a
        .global Explosion_FireAttack_07815a__L0781aa
Explosion_FireAttack_07815a__L0781aa:
.L0781aa:
        move.w  #0x4,d1                         | +050
        jsr     0x236e.l                        | +054
        move.b  #0xff,0x32(a6)                  | +05a
        move.b  #0xff,0x33(a6)                  | +060
        jsr     Fx_RandFlipFacing_077e00(pc)    | +066
        movea.l 0x70(a6),a0                     | +06a
        jsr     0x5dd56.l                       | +06e
        bcs.w   .L07820a                        | +074
        jsr     0x283ca.l                       | +078
        lea     0xffff.w,a0                     | +07e
        move.l  a0,0x48(a6)                     | +082
        lea     .L0781e6(pc),a1                 | +086
        move.l  a1,(a6)                         | +08a
.L0781e6:
        jsr     0x44048.l                       | +08c
        jsr     0x283ca.l                       | +092
        jsr     0x28d70.l                       | +098
        bcs.w   .L07820a                        | +09e
        movea.l 0x70(a6),a0                     | +0a2
        jsr     0x5dd56.l                       | +0a6
        bcc.w   JsrAbsThunk_078210              | +0ac
        .global Explosion_FireAttack_07815a__L07820a
Explosion_FireAttack_07815a__L07820a:
.L07820a:
        jmp     0x518.l                         | +0b0

| ----------------------------------------------------------------------------
|  Explosion_VarTbl_078218  @ $078218  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_VarTbl_078218, "ax", @progbits
        .global Explosion_VarTbl_078218
Explosion_VarTbl_078218:
        lea     0x2ddad0.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df21a.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0xa,d1                         | +016
        bra.w   Debris_ArcB_07875e__L078778     | +01a

| ----------------------------------------------------------------------------
|  Explosion_Var01_078236  @ $078236  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Var01_078236, "ax", @progbits
        .global Explosion_Var01_078236
Explosion_Var01_078236:
        lea     0x2ddb62.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df21a.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0xa,d1                         | +016
        bra.w   Debris_ArcB_07875e__L078778     | +01a

| ----------------------------------------------------------------------------
|  Explosion_Var02_078254  @ $078254  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Var02_078254, "ax", @progbits
        .global Explosion_Var02_078254
Explosion_Var02_078254:
        lea     0x2ddbf4.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df21a.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0xa,d1                         | +016
        bra.w   Debris_ArcB_07875e__L078778     | +01a

| ----------------------------------------------------------------------------
|  Explosion_Var03_078272  @ $078272  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Var03_078272, "ax", @progbits
        .global Explosion_Var03_078272
Explosion_Var03_078272:
        lea     0x2ddc86.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df21a.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0xa,d1                         | +016
        bra.w   Debris_ArcB_07875e__L078778     | +01a

| ----------------------------------------------------------------------------
|  Explosion_Var04_078290  @ $078290  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Var04_078290, "ax", @progbits
        .global Explosion_Var04_078290
Explosion_Var04_078290:
        lea     0x2ddd18.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df224.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0xa,d1                         | +016
        bra.w   Debris_ArcB_07875e__L078778     | +01a

| ----------------------------------------------------------------------------
|  Explosion_Var05_0782ae  @ $0782AE  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Var05_0782ae, "ax", @progbits
        .global Explosion_Var05_0782ae
Explosion_Var05_0782ae:
        lea     0x2dddaa.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df21a.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0xa,d1                         | +016
        bra.w   Debris_ArcB_07875e__L078778     | +01a

| ----------------------------------------------------------------------------
|  Explosion_Var06_0782cc  @ $0782CC  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Var06_0782cc, "ax", @progbits
        .global Explosion_Var06_0782cc
Explosion_Var06_0782cc:
        lea     0x2dde3c.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df21a.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0xa,d1                         | +016
        bra.w   Debris_ArcB_07875e__L078778     | +01a
        lea     0x2ddeb2.l,a0                   | +01e
        jsr     0x28cd4.l                       | +024
        lea     0x2df21a.l,a0                   | +02a
        move.l  a0,0x70(a6)                     | +030
        move.w  #0xa,d1                         | +034
        bra.w   Debris_ArcB_07875e__L078778     | +038

| ----------------------------------------------------------------------------
|  Explosion_Var07_078308  @ $078308  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Var07_078308, "ax", @progbits
        .global Explosion_Var07_078308
Explosion_Var07_078308:
        lea     0x2ddf28.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df224.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0xa,d1                         | +016
        bra.w   Debris_ArcB_07875e__L078778     | +01a

| ----------------------------------------------------------------------------
|  Explosion_Var08_078326  @ $078326  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Var08_078326, "ax", @progbits
        .global Explosion_Var08_078326
Explosion_Var08_078326:
        lea     0x2ddf9e.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df224.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0xa,d1                         | +016
        bra.w   Debris_ArcB_07875e__L078778     | +01a

| ----------------------------------------------------------------------------
|  Explosion_Var09_078344  @ $078344  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Var09_078344, "ax", @progbits
        .global Explosion_Var09_078344
Explosion_Var09_078344:
        lea     0x2de014.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df21a.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0xa,d1                         | +016
        bra.w   Debris_ArcB_07875e__L078778     | +01a
        lea     0x2de08a.l,a0                   | +01e
        jsr     0x28cd4.l                       | +024
        lea     0x2df224.l,a0                   | +02a
        move.l  a0,0x70(a6)                     | +030
        move.w  #0xa,d1                         | +034
        bra.w   Debris_ArcB_07875e__L078778     | +038

| ----------------------------------------------------------------------------
|  Explosion_Var10_078380  @ $078380  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Var10_078380, "ax", @progbits
        .global Explosion_Var10_078380
Explosion_Var10_078380:
        lea     0x2de100.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df224.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0xa,d1                         | +016
        bra.w   Debris_ArcB_07875e__L078778     | +01a
        lea     0x2de176.l,a0                   | +01e
        jsr     0x28cd4.l                       | +024
        lea     0x2df21a.l,a0                   | +02a
        move.l  a0,0x70(a6)                     | +030
        move.w  #0xa,d1                         | +034
        bra.w   Debris_ArcB_07875e__L078778     | +038

| ----------------------------------------------------------------------------
|  Explosion_Var11_0783bc  @ $0783BC  (120 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Var11_0783bc, "ax", @progbits
        .global Explosion_Var11_0783bc
Explosion_Var11_0783bc:
        lea     0x2de1ec.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df21a.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0xa,d1                         | +016
        bra.w   Debris_ArcB_07875e__L078778     | +01a
        lea     0x2dde3c.l,a0                   | +01e
        jsr     0x28cd4.l                       | +024
        lea     0x2df21a.l,a0                   | +02a
        move.l  a0,0x70(a6)                     | +030
        move.w  #0x9e,d1                        | +034
        bra.w   Debris_ArcB_07875e__L078778     | +038
        lea     0x2ddeb2.l,a0                   | +03c
        jsr     0x28cd4.l                       | +042
        lea     0x2df21a.l,a0                   | +048
        move.l  a0,0x70(a6)                     | +04e
        move.w  #0x9e,d1                        | +052
        bra.w   Debris_ArcB_07875e__L078778     | +056
        lea     0x2ddf28.l,a0                   | +05a
        jsr     0x28cd4.l                       | +060
        lea     0x2df224.l,a0                   | +066
        move.l  a0,0x70(a6)                     | +06c
        move.w  #0x9e,d1                        | +070
        bra.w   Debris_ArcB_07875e__L078778     | +074

| ----------------------------------------------------------------------------
|  Explosion_Var12_078434  @ $078434  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Var12_078434, "ax", @progbits
        .global Explosion_Var12_078434
Explosion_Var12_078434:
        lea     0x2ddf9e.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df224.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0x9e,d1                        | +016
        bra.w   Debris_ArcB_07875e__L078778     | +01a

| ----------------------------------------------------------------------------
|  Explosion_Var13_078452  @ $078452  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Var13_078452, "ax", @progbits
        .global Explosion_Var13_078452
Explosion_Var13_078452:
        lea     0x2de014.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df21a.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0x9e,d1                        | +016
        bra.w   Debris_ArcB_07875e__L078778     | +01a
        lea     0x2de08a.l,a0                   | +01e
        jsr     0x28cd4.l                       | +024
        lea     0x2df224.l,a0                   | +02a
        move.l  a0,0x70(a6)                     | +030
        move.w  #0x9e,d1                        | +034
        bra.w   Debris_ArcB_07875e__L078778     | +038

| ----------------------------------------------------------------------------
|  Explosion_Var14_07848e  @ $07848E  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Var14_07848e, "ax", @progbits
        .global Explosion_Var14_07848e
Explosion_Var14_07848e:
        lea     0x2de100.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df224.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0x9e,d1                        | +016
        bra.w   Debris_ArcB_07875e__L078778     | +01a
        lea     0x2de176.l,a0                   | +01e
        jsr     0x28cd4.l                       | +024
        lea     0x2df21a.l,a0                   | +02a
        move.l  a0,0x70(a6)                     | +030
        move.w  #0x9e,d1                        | +034
        bra.w   Debris_ArcB_07875e__L078778     | +038

| ----------------------------------------------------------------------------
|  Explosion_Var15_0784ca  @ $0784CA  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Var15_0784ca, "ax", @progbits
        .global Explosion_Var15_0784ca
Explosion_Var15_0784ca:
        lea     0x2de1ec.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df21a.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0x9e,d1                        | +016
        bra.w   Debris_ArcB_07875e__L078778     | +01a
        lea     0x2de08a.l,a0                   | +01e
        jsr     0x28cd4.l                       | +024
        lea     0x2df224.l,a0                   | +02a
        move.l  a0,0x70(a6)                     | +030
        move.w  #0xad,d1                        | +034
        bra.w   Debris_ArcB_07875e__L078778     | +038

| ----------------------------------------------------------------------------
|  Explosion_Var16_078506  @ $078506  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Var16_078506, "ax", @progbits
        .global Explosion_Var16_078506
Explosion_Var16_078506:
        lea     0x2de100.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df224.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0xad,d1                        | +016
        bra.w   Debris_ArcB_07875e__L078778     | +01a
        lea     0x2de176.l,a0                   | +01e
        jsr     0x28cd4.l                       | +024
        lea     0x2df21a.l,a0                   | +02a
        move.l  a0,0x70(a6)                     | +030
        move.w  #0xad,d1                        | +034
        bra.w   Debris_ArcB_07875e__L078778     | +038

| ----------------------------------------------------------------------------
|  Explosion_Var17_078542  @ $078542  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Var17_078542, "ax", @progbits
        .global Explosion_Var17_078542
Explosion_Var17_078542:
        lea     0x2de1ec.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df21a.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0xad,d1                        | +016
        bra.w   Debris_ArcB_07875e__L078778     | +01a

| ----------------------------------------------------------------------------
|  Explosion_Var18_078560  @ $078560  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Var18_078560, "ax", @progbits
        .global Explosion_Var18_078560
Explosion_Var18_078560:
        lea     0x2de262.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df21a.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0xa,d1                         | +016
        bra.w   Debris_ArcB_07875e__L078778     | +01a

| ----------------------------------------------------------------------------
|  Explosion_Var19_07857e  @ $07857E  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Var19_07857e, "ax", @progbits
        .global Explosion_Var19_07857e
Explosion_Var19_07857e:
        lea     0x2de262.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df21a.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0x72,d1                        | +016
        bra.w   Debris_ArcB_07875e__L078778     | +01a

| ----------------------------------------------------------------------------
|  Explosion_Var20_07859c  @ $07859C  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Var20_07859c, "ax", @progbits
        .global Explosion_Var20_07859c
Explosion_Var20_07859c:
        lea     0x2de2d8.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df21a.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0xa,d1                         | +016
        bra.w   Debris_ArcB_07875e__L078778     | +01a

| ----------------------------------------------------------------------------
|  Explosion_Var21_0785ba  @ $0785BA  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Var21_0785ba, "ax", @progbits
        .global Explosion_Var21_0785ba
Explosion_Var21_0785ba:
        lea     0x2de2d8.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df21a.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0x72,d1                        | +016
        bra.w   Debris_ArcB_07875e__L078778     | +01a

| ----------------------------------------------------------------------------
|  Explosion_Var22_0785d8  @ $0785D8  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Var22_0785d8, "ax", @progbits
        .global Explosion_Var22_0785d8
Explosion_Var22_0785d8:
        lea     0x2de2d8.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df21a.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0x88,d1                        | +016
        bra.w   Debris_ArcB_07875e__L078778     | +01a

| ----------------------------------------------------------------------------
|  Explosion_Var23_0785f6  @ $0785F6  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Var23_0785f6, "ax", @progbits
        .global Explosion_Var23_0785f6
Explosion_Var23_0785f6:
        lea     0x2de34e.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df224.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0xa,d1                         | +016
        bra.w   Debris_ArcB_07875e__L078778     | +01a

| ----------------------------------------------------------------------------
|  Explosion_Var24_078614  @ $078614  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Var24_078614, "ax", @progbits
        .global Explosion_Var24_078614
Explosion_Var24_078614:
        lea     0x2de34e.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df224.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0x72,d1                        | +016
        bra.w   Debris_ArcB_07875e__L078778     | +01a

| ----------------------------------------------------------------------------
|  Explosion_Var25_078632  @ $078632  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Var25_078632, "ax", @progbits
        .global Explosion_Var25_078632
Explosion_Var25_078632:
        lea     0x2de34e.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df224.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0x77,d1                        | +016
        bra.w   Debris_ArcB_07875e__L078778     | +01a

| ----------------------------------------------------------------------------
|  Explosion_Var26_078650  @ $078650  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Var26_078650, "ax", @progbits
        .global Explosion_Var26_078650
Explosion_Var26_078650:
        lea     0x2de3c4.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df21a.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0xa,d1                         | +016
        bra.w   Debris_ArcB_07875e__L078778     | +01a

| ----------------------------------------------------------------------------
|  Explosion_Var27_07866e  @ $07866E  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Var27_07866e, "ax", @progbits
        .global Explosion_Var27_07866e
Explosion_Var27_07866e:
        lea     0x2de3c4.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df21a.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0x72,d1                        | +016
        bra.w   Debris_ArcB_07875e__L078778     | +01a

| ----------------------------------------------------------------------------
|  Explosion_Var28_07868c  @ $07868C  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Var28_07868c, "ax", @progbits
        .global Explosion_Var28_07868c
Explosion_Var28_07868c:
        lea     0x2de43a.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df21a.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0xa,d1                         | +016
        bra.w   Debris_ArcB_07875e__L078778     | +01a

| ----------------------------------------------------------------------------
|  Explosion_Var29_0786aa  @ $0786AA  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Var29_0786aa, "ax", @progbits
        .global Explosion_Var29_0786aa
Explosion_Var29_0786aa:
        lea     0x2de43a.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df21a.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0x72,d1                        | +016
        bra.w   Debris_ArcB_07875e__L078778     | +01a

| ----------------------------------------------------------------------------
|  Explosion_Var30_0786c8  @ $0786C8  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Var30_0786c8, "ax", @progbits
        .global Explosion_Var30_0786c8
Explosion_Var30_0786c8:
        lea     0x2de43a.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df21a.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0x77,d1                        | +016
        bra.w   Debris_ArcB_07875e__L078778     | +01a

| ----------------------------------------------------------------------------
|  Explosion_Var31_0786e6  @ $0786E6  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Var31_0786e6, "ax", @progbits
        .global Explosion_Var31_0786e6
Explosion_Var31_0786e6:
        lea     0x2de43a.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df21a.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0x88,d1                        | +016
        bra.w   Debris_ArcB_07875e__L078778     | +01a

| ----------------------------------------------------------------------------
|  Explosion_Var32_078704  @ $078704  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Explosion_Var32_078704, "ax", @progbits
        .global Explosion_Var32_078704
Explosion_Var32_078704:
        lea     0x2de43a.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df21a.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0x28,d1                        | +016
        bra.w   Debris_ArcB_07875e__L078778     | +01a
        lea     0x2de4b0.l,a0                   | +01e
        jsr     0x28cd4.l                       | +024
        lea     0x2df21a.l,a0                   | +02a
        move.l  a0,0x70(a6)                     | +030
        move.w  #0xb,d1                         | +034
        bra.w   Debris_ArcB_07875e__L078778     | +038

| ----------------------------------------------------------------------------
|  Debris_ArcA_078740  @ $078740  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Debris_ArcA_078740, "ax", @progbits
        .global Debris_ArcA_078740
Debris_ArcA_078740:
        lea     0x2de5f4.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df21a.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0xb,d1                         | +016
        bra.w   Debris_ArcB_07875e__L078778     | +01a

| ----------------------------------------------------------------------------
|  Debris_ArcB_07875e  @ $07875E  (226 B)
| ----------------------------------------------------------------------------
        .section .text.Debris_ArcB_07875e, "ax", @progbits
        .global Debris_ArcB_07875e
Debris_ArcB_07875e:
        lea     0x2de66a.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df224.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        move.w  #0xb,d1                         | +016
        .global Debris_ArcB_07875e__L078778
Debris_ArcB_07875e__L078778:
.L078778:
        jsr     0x236e.l                        | +01a
        move.b  #0xff,0x32(a6)                  | +020
        move.b  #0xff,0x33(a6)                  | +026
        jsr     Fx_RandFlipFacing_077e00(pc)    | +02c
        move.w  #0x0,d0                         | +030
        jsr     0x5dca4.l                       | +034
        move.w  d0,0x28(a6)                     | +03a
        move.w  #0x7f8,0x2a(a6)                 | +03e
        move.w  #0xffbc,0x2e(a6)                | +044
        move.w  #0x0,0x2c(a6)                   | +04a
        move.w  #0x3ff,d0                       | +050
        jsr     0x5e9e4.l                       | +054
        btst    #0x0,d0                         | +05a
        beq.w   .L0787c2                        | +05e
        neg.w   d0                              | +062
.L0787c2:
        move.w  d0,0x28(a6)                     | +064
        move.w  #0x7ff,d0                       | +068
        jsr     0x5e9e4.l                       | +06c
        sub.w   d0,0x2a(a6)                     | +072
        andi.b  #0x1,d0                         | +076
        or.b    d0,0x3a(a6)                     | +07a
        movea.l 0x70(a6),a0                     | +07e
        jsr     0x5dd56.l                       | +082
        bcs.w   .L078838                        | +088
        lea     0xffff.w,a0                     | +08c
        move.l  a0,0x48(a6)                     | +090
        lea     .L0787f8(pc),a1                 | +094
        move.l  a1,(a6)                         | +098
.L0787f8:
        cmpi.w  #0xfb00,0x2a(a6)                | +09a
        bgt.w   .L078808                        | +0a0
        move.w  #0xfb01,0x2a(a6)                | +0a4
.L078808:
        jsr     0x27cee.l                       | +0aa
        jsr     0x5e804.l                       | +0b0
        bcc.w   .L078820                        | +0b6
        andi.b  #0xee,ccr                       | +0ba
        bra.w   .L078826                        | +0be
.L078820:
        jsr     0x28d70.l                       | +0c2
.L078826:
        bcs.w   Explosion_FireAttack_07815a__L07820a | +0c8
        movea.l 0x70(a6),a0                     | +0cc
        jsr     0x5dd56.l                       | +0d0
        bcc.w   .L07883e                        | +0d6
.L078838:
        jmp     0x518.l                         | +0da
.L07883e:
        rts                                     | +0e0

| ----------------------------------------------------------------------------
|  Smoke_Small_078840  @ $078840  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Smoke_Small_078840, "ax", @progbits
        .global Smoke_Small_078840
Smoke_Small_078840:
        move.w  #0xc,d1                         | +000
        jsr     0x236e.l                        | +004
        lea     0x2ded26.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     0x2df238.l,a0                   | +016
        move.l  a0,0x70(a6)                     | +01c
        bra.w   Smoke_Big_078908__L0789ae       | +020

| ----------------------------------------------------------------------------
|  Smoke_Pair40_078864  @ $078864  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Smoke_Pair40_078864, "ax", @progbits
        .global Smoke_Pair40_078864
Smoke_Pair40_078864:
        lea     Smoke_Big_078908(pc),a1         | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        addi.w  #0x20,0x22(a0)                  | +010
        lea     Smoke_Big_078908(pc),a1         | +016
        jsr     0x4ae.l                         | +01a
        jsr     0x5dd02.l                       | +020
        addi.w  #0xffe0,0x22(a0)                | +026

| ----------------------------------------------------------------------------
|  Smoke_Pair20_078890  @ $078890  (48 B)
| ----------------------------------------------------------------------------
        .section .text.Smoke_Pair20_078890, "ax", @progbits
        .global Smoke_Pair20_078890
Smoke_Pair20_078890:
        lea     Smoke_Big_078908(pc),a1         | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        addi.w  #0x10,0x22(a0)                  | +010
        lea     Smoke_Big_078908(pc),a1         | +016
        jsr     0x4ae.l                         | +01a
        jsr     0x5dd02.l                       | +020
        addi.w  #0xfff0,0x22(a0)                | +026
        bra.w   Smoke_Big_078908                | +02c

| ----------------------------------------------------------------------------
|  Smoke_Std_0788c0  @ $0788C0  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Smoke_Std_0788c0, "ax", @progbits
        .global Smoke_Std_0788c0
Smoke_Std_0788c0:
        move.w  #0xc,d1                         | +000
        jsr     0x236e.l                        | +004
        lea     0x2de7c2.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     0x2df238.l,a0                   | +016
        move.l  a0,0x70(a6)                     | +01c
        bra.w   Smoke_Big_078908__L078998       | +020

| ----------------------------------------------------------------------------
|  Smoke_StdB_0788e4  @ $0788E4  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Smoke_StdB_0788e4, "ax", @progbits
        .global Smoke_StdB_0788e4
Smoke_StdB_0788e4:
        move.w  #0xc,d1                         | +000
        jsr     0x236e.l                        | +004
        lea     0x2de7c2.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     0x2df238.l,a0                   | +016
        move.l  a0,0x70(a6)                     | +01c
        bra.w   Smoke_Big_078908__L0789ae       | +020

| ----------------------------------------------------------------------------
|  Smoke_Big_078908  @ $078908  (232 B)
| ----------------------------------------------------------------------------
        .section .text.Smoke_Big_078908, "ax", @progbits
        .global Smoke_Big_078908
Smoke_Big_078908:
        move.w  #0x139,d1                       | +000
        jsr     0x236e.l                        | +004
        lea     0x2de7c2.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     0x2df238.l,a0                   | +016
        move.l  a0,0x70(a6)                     | +01c
        bra.w   .L078998                        | +020
        move.w  #0xc,d1                         | +024
        jsr     0x236e.l                        | +028
        lea     0x2de93e.l,a0                   | +02e
        jsr     0x28cd4.l                       | +034
        lea     0x2df242.l,a0                   | +03a
        move.l  a0,0x70(a6)                     | +040
        bra.w   .L078998                        | +044
        move.w  #0xc,d1                         | +048
        jsr     0x236e.l                        | +04c
        lea     0x2deaba.l,a0                   | +052
        jsr     0x28cd4.l                       | +058
        lea     0x2df238.l,a0                   | +05e
        move.l  a0,0x70(a6)                     | +064
        bra.w   .L078998                        | +068
        move.w  #0xc,d1                         | +06c
        jsr     0x236e.l                        | +070
        lea     0x2dec0c.l,a0                   | +076
        jsr     0x28cd4.l                       | +07c
        lea     0x2df242.l,a0                   | +082
        move.l  a0,0x70(a6)                     | +088
        bra.w   .L078998                        | +08c
        .global Smoke_Big_078908__L078998
Smoke_Big_078908__L078998:
.L078998:
        move.b  #0xff,0x32(a6)                  | +090
        move.b  #0xff,0x33(a6)                  | +096
        jsr     Fx_RandFlipFacing_077e00(pc)    | +09c
        bset    #0x6,0x12(a6)                   | +0a0
        .global Smoke_Big_078908__L0789ae
Smoke_Big_078908__L0789ae:
.L0789ae:
        movea.l 0x70(a6),a0                     | +0a6
        jsr     0x5dd56.l                       | +0aa
        bcs.w   .L0789e8                        | +0b0
        lea     0xffff.w,a0                     | +0b4
        move.l  a0,0x48(a6)                     | +0b8
        lea     .L0789ca(pc),a1                 | +0bc
        move.l  a1,(a6)                         | +0c0
.L0789ca:
        jsr     0x44048.l                       | +0c2
        jsr     0x28d70.l                       | +0c8
        bcs.w   .L0789e8                        | +0ce
        movea.l 0x70(a6),a0                     | +0d2
        jsr     0x5dd56.l                       | +0d6
        bcc.w   .L0789ee                        | +0dc
.L0789e8:
        jmp     0x518.l                         | +0e0
.L0789ee:
        rts                                     | +0e6

| ----------------------------------------------------------------------------
|  Smoke_Column_0789f0  @ $0789F0  (120 B)
| ----------------------------------------------------------------------------
        .section .text.Smoke_Column_0789f0, "ax", @progbits
        .global Smoke_Column_0789f0
Smoke_Column_0789f0:
        lea     0x2deea2.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2df24c.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        bra.w   .L078a0a                        | +016
.L078a0a:
        move.w  #0x10,d1                        | +01a
        jsr     0x236e.l                        | +01e
        bset    #0x6,0x12(a6)                   | +024
        move.b  #0xff,0x32(a6)                  | +02a
        move.b  #0xff,0x33(a6)                  | +030
        movea.l 0x70(a6),a0                     | +036
        jsr     0x5dd56.l                       | +03a
        bcs.w   .L078a60                        | +040
        lea     0xffff.w,a0                     | +044
        move.l  a0,0x48(a6)                     | +048
        lea     .L078a42(pc),a1                 | +04c
        move.l  a1,(a6)                         | +050
.L078a42:
        jsr     0x44048.l                       | +052
        jsr     0x28d70.l                       | +058
        bcs.w   .L078a60                        | +05e
        movea.l 0x70(a6),a0                     | +062
        jsr     0x5dd56.l                       | +066
        bcc.w   .L078a66                        | +06c
.L078a60:
        jmp     0x518.l                         | +070
.L078a66:
        rts                                     | +076

| ----------------------------------------------------------------------------
|  Breakable_RetGate_078a68  @ $078A68  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Breakable_RetGate_078a68, "ax", @progbits
        .global Breakable_RetGate_078a68
Breakable_RetGate_078a68:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_078a7e                    | +00c

| ----------------------------------------------------------------------------
|  Breakable_Sprites_078a84  @ $078A84  (268 B)
| ----------------------------------------------------------------------------
        .section .text.Breakable_Sprites_078a84, "ax", @progbits
        .global Breakable_Sprites_078a84
Breakable_Sprites_078a84:
        .dc.w   0x0400                        | +000  (dato / opcode no decodificado)
        .dc.w   0x10c0                        | +002  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +008  (dato / opcode no decodificado)
        .dc.w   0x6fc6                        | +00a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +012  (dato / opcode no decodificado)
        .dc.w   0x7014                        | +014  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x7066                        | +01e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0600                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0600                        | +028  (dato / opcode no decodificado)
        .dc.w   0xffdc                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0078                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x0800                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +030  (dato / opcode no decodificado)
        .dc.w   0x8ede                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +038  (dato / opcode no decodificado)
        .dc.w   0x70b2                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0600                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x001e                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0600                        | +044  (dato / opcode no decodificado)
        .dc.w   0xffcc                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0078                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0800                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x8f14                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +054  (dato / opcode no decodificado)
        .dc.w   0x7100                        | +056  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +058  (dato / opcode no decodificado)
        .dc.w   0x0600                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0600                        | +060  (dato / opcode no decodificado)
        .dc.w   0xffe1                        | +062  (dato / opcode no decodificado)
        .dc.w   0x0078                        | +064  (dato / opcode no decodificado)
        .dc.w   0x0800                        | +066  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +068  (dato / opcode no decodificado)
        .dc.w   0x8f14                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +070  (dato / opcode no decodificado)
        .dc.w   0x7156                        | +072  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +074  (dato / opcode no decodificado)
        .dc.w   0x0600                        | +076  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0074                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0600                        | +07c  (dato / opcode no decodificado)
        .dc.w   0xffb2                        | +07e  (dato / opcode no decodificado)
        .dc.w   0x0078                        | +080  (dato / opcode no decodificado)
        .dc.w   0x0800                        | +082  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +084  (dato / opcode no decodificado)
        .dc.w   0x8f14                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +088  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +08a  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x71a0                        | +08e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +090  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +094  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +096  (dato / opcode no decodificado)
        .dc.w   0x71ee                        | +098  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +09a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +09e  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0x7236                        | +0a2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a4  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +0a6  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +0a8  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +0aa  (dato / opcode no decodificado)
        .dc.w   0x728a                        | +0ac  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0ae  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +0b0  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +0b2  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +0b4  (dato / opcode no decodificado)
        .dc.w   0x72e2                        | +0b6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0b8  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +0ba  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +0bc  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +0be  (dato / opcode no decodificado)
        .dc.w   0x7330                        | +0c0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0c2  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +0c4  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +0c6  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +0c8  (dato / opcode no decodificado)
        .dc.w   0x7378                        | +0ca  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0cc  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0ce  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +0d0  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +0d2  (dato / opcode no decodificado)
        .dc.w   0x73c0                        | +0d4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0d6  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0d8  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +0da  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +0dc  (dato / opcode no decodificado)
        .dc.w   0x7406                        | +0de  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0e0  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0e2  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +0e4  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +0e6  (dato / opcode no decodificado)
        .dc.w   0x744c                        | +0e8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0ea  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0ec  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +0ee  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +0f0  (dato / opcode no decodificado)
        .dc.w   0x7492                        | +0f2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0f4  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0f6  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +0f8  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +0fa  (dato / opcode no decodificado)
        .dc.w   0x744c                        | +0fc  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0fe  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +100  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +102  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +104  (dato / opcode no decodificado)
        .dc.w   0x7492                        | +106  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +108  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +10a  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Breakable_WreckSprites_078b90  @ $078B90  (12 B)
| ----------------------------------------------------------------------------
        .section .text.Breakable_WreckSprites_078b90, "ax", @progbits
        .global Breakable_WreckSprites_078b90
Breakable_WreckSprites_078b90:
        .dc.w   0x0001                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +004  (dato / opcode no decodificado)
        .dc.w   0x6fc6                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +00a  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Breakable_WreckSpritesB_078b9c  @ $078B9C  (68 B)
| ----------------------------------------------------------------------------
        .section .text.Breakable_WreckSpritesB_078b9c, "ax", @progbits
        .global Breakable_WreckSpritesB_078b9c
Breakable_WreckSpritesB_078b9c:
        .dc.w   0x0900                        | +000  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +002  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +004  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x7526                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +014  (dato / opcode no decodificado)
        .dc.w   0x753e                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x7556                        | +020  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +028  (dato / opcode no decodificado)
        .dc.w   0x756a                        | +02a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +032  (dato / opcode no decodificado)
        .dc.w   0x757e                        | +034  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +038  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x7592                        | +03e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +040  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +042  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Breakable_FallPath_078be0  @ $078BE0  (152 B)
| ----------------------------------------------------------------------------
        .section .text.Breakable_FallPath_078be0, "ax", @progbits
        .global Breakable_FallPath_078be0
Breakable_FallPath_078be0:
        .dc.w   0x003c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +002  (dato / opcode no decodificado)
        .dc.w   0x003b                        | +004  (dato / opcode no decodificado)
        .dc.w   0xfff4                        | +006  (dato / opcode no decodificado)
        .dc.w   0x003a                        | +008  (dato / opcode no decodificado)
        .dc.w   0xffef                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0039                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xffeb                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0038                        | +010  (dato / opcode no decodificado)
        .dc.w   0xffe7                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0037                        | +014  (dato / opcode no decodificado)
        .dc.w   0xffe3                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0036                        | +018  (dato / opcode no decodificado)
        .dc.w   0xffde                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0035                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xffda                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0034                        | +020  (dato / opcode no decodificado)
        .dc.w   0xffd6                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0033                        | +024  (dato / opcode no decodificado)
        .dc.w   0xffd1                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0032                        | +028  (dato / opcode no decodificado)
        .dc.w   0xffcd                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0031                        | +02c  (dato / opcode no decodificado)
        .dc.w   0xffc9                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +030  (dato / opcode no decodificado)
        .dc.w   0xffc5                        | +032  (dato / opcode no decodificado)
        .dc.w   0x002f                        | +034  (dato / opcode no decodificado)
        .dc.w   0xffc1                        | +036  (dato / opcode no decodificado)
        .dc.w   0x002e                        | +038  (dato / opcode no decodificado)
        .dc.w   0xffbd                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x002d                        | +03c  (dato / opcode no decodificado)
        .dc.w   0xffb9                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +040  (dato / opcode no decodificado)
        .dc.w   0xffb5                        | +042  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +044  (dato / opcode no decodificado)
        .dc.w   0xffb1                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +048  (dato / opcode no decodificado)
        .dc.w   0xffad                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +04c  (dato / opcode no decodificado)
        .dc.w   0xffa9                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffa7                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +054  (dato / opcode no decodificado)
        .dc.w   0xffa3                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +058  (dato / opcode no decodificado)
        .dc.w   0xff9f                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x001c                        | +05c  (dato / opcode no decodificado)
        .dc.w   0xff9a                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +060  (dato / opcode no decodificado)
        .dc.w   0xff95                        | +062  (dato / opcode no decodificado)
        .dc.w   0x000b                        | +064  (dato / opcode no decodificado)
        .dc.w   0xff8f                        | +066  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +068  (dato / opcode no decodificado)
        .dc.w   0xff8d                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +06c  (dato / opcode no decodificado)
        .dc.w   0xff89                        | +06e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +070  (dato / opcode no decodificado)
        .dc.w   0xff87                        | +072  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +074  (dato / opcode no decodificado)
        .dc.w   0xff87                        | +076  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +078  (dato / opcode no decodificado)
        .dc.w   0xff88                        | +07a  (dato / opcode no decodificado)
        .dc.w   0xffe8                        | +07c  (dato / opcode no decodificado)
        .dc.w   0xff8b                        | +07e  (dato / opcode no decodificado)
        .dc.w   0xffe0                        | +080  (dato / opcode no decodificado)
        .dc.w   0xff8f                        | +082  (dato / opcode no decodificado)
        .global Breakable_FallPath_078be0__L078c64
Breakable_FallPath_078be0__L078c64:
.L078c64:
        move.w  0x22(a6),d0                     | +084
        addi.w  #0x40,d0                        | +088
        bpl.w   .L078c76                        | +08c
        jmp     0x518.l                         | +090
.L078c76:
        rts                                     | +096

| ----------------------------------------------------------------------------
|  Breakable_Tmpl24_078c78  @ $078C78  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Breakable_Tmpl24_078c78, "ax", @progbits
        .global Breakable_Tmpl24_078c78
Breakable_Tmpl24_078c78:
        move.w  #0x1,0x86(a6)                   | +000
        bset    #0x0,0x3a(a6)                   | +006
        bra.w   Breakable_Tmpl25_078ca2__L078ca8 | +00c

| ----------------------------------------------------------------------------
|  Breakable_Tmpl26_078c88  @ $078C88  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Breakable_Tmpl26_078c88, "ax", @progbits
        .global Breakable_Tmpl26_078c88
Breakable_Tmpl26_078c88:
        move.w  #0x0,0x86(a6)                   | +000
        bset    #0x0,0x3a(a6)                   | +006
        bra.w   Breakable_Tmpl25_078ca2__L078ca8 | +00c

| ----------------------------------------------------------------------------
|  Breakable_Tmpl23_078c98  @ $078C98  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Breakable_Tmpl23_078c98, "ax", @progbits
        .global Breakable_Tmpl23_078c98
Breakable_Tmpl23_078c98:
        move.w  #0x1,0x86(a6)                   | +000
        bra.w   Breakable_Tmpl25_078ca2__L078ca8 | +006

| ----------------------------------------------------------------------------
|  Breakable_Tmpl25_078ca2  @ $078CA2  (82 B)
| ----------------------------------------------------------------------------
        .section .text.Breakable_Tmpl25_078ca2, "ax", @progbits
        .global Breakable_Tmpl25_078ca2
Breakable_Tmpl25_078ca2:
        move.w  #0x0,0x86(a6)                   | +000
        .global Breakable_Tmpl25_078ca2__L078ca8
Breakable_Tmpl25_078ca2__L078ca8:
.L078ca8:
        bclr    #0x1,0x12(a6)                   | +006
        move.w  #0x1e,d1                        | +00c
        jsr     0x236e.l                        | +010
        lea     Breakable_WreckSprites_078b90(pc),a0 | +016
        jsr     0x28cd4.l                       | +01a
        lea     .L078cc8(pc),a1                 | +020
        move.l  a1,(a6)                         | +024
.L078cc8:
        jsr     0x2783a.l                       | +026
        jsr     0x28d70.l                       | +02c
        moveq   #0,d0                           | +032
        move.b  0x98(a6),d0                     | +034
        lsl.w   #0x3,d0                         | +038
        neg.w   d0                              | +03a
        addi.w  #0x140,d0                       | +03c
        cmp.w   0x22(a6),d0                     | +040
        bcs.w   .L078cf0                        | +044
        lea     Breakable_Destroyed_078cf4(pc),a1 | +048
        move.l  a1,(a6)                         | +04c
.L078cf0:
        bra.w   Breakable_FallPath_078be0__L078c64 | +04e

| ----------------------------------------------------------------------------
|  Breakable_Destroyed_078cf4  @ $078CF4  (58 B)
| ----------------------------------------------------------------------------
        .section .text.Breakable_Destroyed_078cf4, "ax", @progbits
        .global Breakable_Destroyed_078cf4
Breakable_Destroyed_078cf4:
        lea     Breakable_Sprites_078a84(pc),a0 | +000
        jsr     0x28cd4.l                       | +004
        lea     Breakable_Wreck_Fall_078d58(pc),a1 | +00a
        jsr     0x4ae.l                         | +00e
        move.w  0x86(a6),0x86(a0)               | +014
        lea     .L078d14(pc),a1                 | +01a
        move.l  a1,(a6)                         | +01e
.L078d14:
        jsr     0x2783a.l                       | +020
        jsr     0x28d70.l                       | +026
        bcc.w   .L078d2a                        | +02c
        lea     Breakable_FadeOut_078d2e(pc),a1 | +030
        move.l  a1,(a6)                         | +034
.L078d2a:
        bra.w   Breakable_FallPath_078be0__L078c64 | +036

| ----------------------------------------------------------------------------
|  Breakable_FadeOut_078d2e  @ $078D2E  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Breakable_FadeOut_078d2e, "ax", @progbits
        .global Breakable_FadeOut_078d2e
Breakable_FadeOut_078d2e:
        move.b  #0x1e,0x59(a6)                  | +000
        lea     .L078d3a(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L078d3a:
        jsr     0x2783a.l                       | +00c
        jsr     0x28d70.l                       | +012
        tst.b   0x59(a6)                        | +018
        bne.w   .L078d54                        | +01c
        jmp     0x518.l                         | +020
.L078d54:
        bra.w   Breakable_FallPath_078be0__L078c64 | +026

| ----------------------------------------------------------------------------
|  Breakable_Wreck_Fall_078d58  @ $078D58  (150 B)
| ----------------------------------------------------------------------------
        .section .text.Breakable_Wreck_Fall_078d58, "ax", @progbits
        .global Breakable_Wreck_Fall_078d58
Breakable_Wreck_Fall_078d58:
        move.w  #0x8000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x18,0x38(a6)                  | +010
        bset    #0x1,0x12(a6)                   | +016
        move.w  #0xe,d1                         | +01c
        jsr     0x236e.l                        | +020
        lea     Breakable_WreckSpritesB_078b9c(pc),a0 | +026
        jsr     0x28cd4.l                       | +02a
        movea.l 0xc(a6),a0                      | +030
        lea     0x98(a0),a1                     | +034
        lea     0x98(a6),a2                     | +038
        move.b  (a1)+,(a2)+                     | +03c
        move.b  (a1)+,(a2)+                     | +03e
        move.b  (a1)+,(a2)+                     | +040
        move.b  (a1)+,(a2)+                     | +042
        move.b  (a1)+,(a2)+                     | +044
        move.b  (a1)+,(a2)+                     | +046
        move.b  (a1)+,(a2)+                     | +048
        move.b  (a1)+,(a2)+                     | +04a
        move.w  0x3a(a0),0x3a(a6)               | +04c
        lea     Breakable_FallPath_078be0(pc),a0 | +052
        move.l  a0,0x70(a6)                     | +056
        lea     .L078db8(pc),a1                 | +05a
        move.l  a1,(a6)                         | +05e
.L078db8:
        movea.l 0x70(a6),a0                     | +060
        move.w  (a0)+,d0                        | +064
        move.w  (a0)+,d1                        | +066
        btst    #0x0,0x3a(a6)                   | +068
        beq.w   .L078dcc                        | +06e
        neg.w   d0                              | +072
.L078dcc:
        movea.l 0xc(a6),a1                      | +074
        add.w   0x22(a1),d0                     | +078
        add.w   0x24(a1),d1                     | +07c
        move.w  d0,0x22(a6)                     | +080
        move.w  d1,0x24(a6)                     | +084
        move.l  a0,0x70(a6)                     | +088
        cmpa.l  #0x78c64,a0                     | +08c
        bcc.w   Breakable_Wreck_SpawnSoldier_078df6 | +092

| ----------------------------------------------------------------------------
|  Breakable_Wreck_SpawnSoldier_078df6  @ $078DF6  (86 B)
| ----------------------------------------------------------------------------
        .section .text.Breakable_Wreck_SpawnSoldier_078df6, "ax", @progbits
        .global Breakable_Wreck_SpawnSoldier_078df6
Breakable_Wreck_SpawnSoldier_078df6:
        jsr     0x28d70.l                       | +000
        move.w  0x86(a6),d0                     | +006
        cmpi.w  #0x1,d0                         | +00a
        bne.w   .L078e1e                        | +00e
        lea     0x57880.l,a1                    | +012
        jsr     0x4498e.l                       | +018
        jsr     0x5dd02.l                       | +01e
        bra.w   .L078e30                        | +024
.L078e1e:
        lea     0x5783e.l,a1                    | +028
        jsr     0x4498e.l                       | +02e
        jsr     0x5dd02.l                       | +034
.L078e30:
        lea     0x99(a6),a1                     | +03a
        lea     0x99(a0),a2                     | +03e
        move.b  (a1)+,(a2)+                     | +042
        move.b  (a1)+,(a2)+                     | +044
        move.b  (a1)+,(a2)+                     | +046
        move.b  (a1)+,(a2)+                     | +048
        move.b  (a1)+,(a2)+                     | +04a
        move.b  (a1)+,(a2)+                     | +04c
        move.b  (a1)+,(a2)+                     | +04e
        jmp     0x518.l                         | +050

| ----------------------------------------------------------------------------
|  Breakable_WreckAnimTbl_078e4c  @ $078E4C  (146 B)
| ----------------------------------------------------------------------------
        .section .text.Breakable_WreckAnimTbl_078e4c, "ax", @progbits
        .global Breakable_WreckAnimTbl_078e4c
Breakable_WreckAnimTbl_078e4c:
        .dc.w   0x0003                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +004  (dato / opcode no decodificado)
        .dc.w   0x74d6                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x74e0                        | +010  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +018  (dato / opcode no decodificado)
        .dc.w   0x74ea                        | +01a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +022  (dato / opcode no decodificado)
        .dc.w   0x74f4                        | +024  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x74fe                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0x05ff                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0059                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +038  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x7508                        | +03c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +044  (dato / opcode no decodificado)
        .dc.w   0x7512                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x751c                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +058  (dato / opcode no decodificado)
        .dc.w   0x7512                        | +05a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +060  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +062  (dato / opcode no decodificado)
        .dc.w   0x7508                        | +064  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +066  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +068  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x74fe                        | +06e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +070  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +072  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +074  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +076  (dato / opcode no decodificado)
        .dc.w   0x74f4                        | +078  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +07e  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +080  (dato / opcode no decodificado)
        .dc.w   0x74ea                        | +082  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +084  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +088  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +08a  (dato / opcode no decodificado)
        .dc.w   0x74e0                        | +08c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +090  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Breakable_Cb_SpawnChip_078ede  @ $078EDE  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Breakable_Cb_SpawnChip_078ede, "ax", @progbits
        .global Breakable_Cb_SpawnChip_078ede
Breakable_Cb_SpawnChip_078ede:
        lea     Breakable_Chip_Run_078f2c(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        move.b  0x3a(a6),0x3a(a0)               | +00a
        .global Breakable_Cb_SpawnChip_078ede__L078eee
Breakable_Cb_SpawnChip_078ede__L078eee:
.L078eee:
        move.w  0x74(a6),d0                     | +010
        move.w  0x78(a6),d1                     | +014
        btst    #0x0,0x3a(a6)                   | +018
        beq.w   .L078f02                        | +01e
        neg.w   d0                              | +022
.L078f02:
        add.w   0x22(a6),d0                     | +024
        add.w   0x24(a6),d1                     | +028
        move.w  d0,0x22(a0)                     | +02c
        move.w  d1,0x24(a0)                     | +030
        rts                                     | +034

| ----------------------------------------------------------------------------
|  Breakable_Cb_SpawnChipFlip_078f14  @ $078F14  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Breakable_Cb_SpawnChipFlip_078f14, "ax", @progbits
        .global Breakable_Cb_SpawnChipFlip_078f14
Breakable_Cb_SpawnChipFlip_078f14:
        lea     Breakable_Chip_Run_078f2c(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        move.b  0x3a(a6),d0                     | +00a
        bchg    #0x0,d0                         | +00e
        move.b  d0,0x3a(a0)                     | +012
        bra.b   Breakable_Cb_SpawnChip_078ede__L078eee | +016

| ----------------------------------------------------------------------------
|  Breakable_Chip_Run_078f2c  @ $078F2C  (66 B)
| ----------------------------------------------------------------------------
        .section .text.Breakable_Chip_Run_078f2c, "ax", @progbits
        .global Breakable_Chip_Run_078f2c
Breakable_Chip_Run_078f2c:
        move.w  #0x1e,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0xfffc,0x2e(a6)                | +00a
        lea     Breakable_WreckAnimTbl_078e4c(pc),a0 | +010
        jsr     0x28cd4.l                       | +014
        lea     .L078f4c(pc),a1                 | +01a
        move.l  a1,(a6)                         | +01e
.L078f4c:
        jsr     0x27d50.l                       | +020
        bcc.w   .L078f5c                        | +026
        jmp     0x518.l                         | +02a
.L078f5c:
        jsr     0x28d70.l                       | +030
        bcc.w   .L078f6c                        | +036
        jmp     0x518.l                         | +03a
.L078f6c:
        rts                                     | +040

| ----------------------------------------------------------------------------
|  PathScript_RetGate_078f6e  @ $078F6E  (16 B)
| ----------------------------------------------------------------------------
        .section .text.PathScript_RetGate_078f6e, "ax", @progbits
        .global PathScript_RetGate_078f6e
PathScript_RetGate_078f6e:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_078f84                    | +00c

| ----------------------------------------------------------------------------
|  PathScript_StepCtx_078f8a  @ $078F8A  (6 B)
| ----------------------------------------------------------------------------
        .section .text.PathScript_StepCtx_078f8a, "ax", @progbits
        .global PathScript_StepCtx_078f8a
PathScript_StepCtx_078f8a:
        jsr     0x2783a.l                       | +000

| ----------------------------------------------------------------------------
|  PathScript_Step_078f90  @ $078F90  (48 B)
| ----------------------------------------------------------------------------
        .section .text.PathScript_Step_078f90, "ax", @progbits
        .global PathScript_Step_078f90
PathScript_Step_078f90:
        move.w  0x36(a6),d5                     | +000
        movea.l 0x90(a6),a0                     | +004
        .global PathScript_Step_078f90__L078f98
PathScript_Step_078f90__L078f98:
.L078f98:
        lea     PathScript_OpTbl_078fc0(pc),a1  | +008
        move.b  (a0),d0                         | +00c
        cmpi.b  #0xb,d0                         | +00e
        bcs.w   .L078fb2                        | +012
        nop                                     | +016
        nop                                     | +018
        cmpi.b  #0xb,d0                         | +01a
        nop                                     | +01e
        trap    #0xf                            | +020
.L078fb2:
        andi.l  #0xf,d0                         | +022
        lsl.l   #0x2,d0                         | +028
        movea.l (a1,d0.w),a2                    | +02a
        jmp     (a2)                            | +02e

| ----------------------------------------------------------------------------
|  PathScript_OpTbl_078fc0  @ $078FC0  (68 B)
| ----------------------------------------------------------------------------
        .section .text.PathScript_OpTbl_078fc0, "ax", @progbits
        .global PathScript_OpTbl_078fc0
PathScript_OpTbl_078fc0:
        .dc.w   0x0007                        | +000  (dato / opcode no decodificado)
        .dc.w   0x90d2                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +004  (dato / opcode no decodificado)
        .dc.w   0x9106                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +008  (dato / opcode no decodificado)
        .dc.w   0x917a                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x90e6                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +010  (dato / opcode no decodificado)
        .dc.w   0x9014                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +014  (dato / opcode no decodificado)
        .dc.w   0x90a8                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +018  (dato / opcode no decodificado)
        .dc.w   0x907e                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x9054                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +020  (dato / opcode no decodificado)
        .dc.w   0x9020                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +024  (dato / opcode no decodificado)
        .dc.w   0x9038                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +028  (dato / opcode no decodificado)
        .dc.w   0x9004                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x9000                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +030  (dato / opcode no decodificado)
        .dc.w   0x9000                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +034  (dato / opcode no decodificado)
        .dc.w   0x9000                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +038  (dato / opcode no decodificado)
        .dc.w   0x9000                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x9000                        | +03e  (dato / opcode no decodificado)
        trap    #0xf                            | +040
        rts                                     | +042

| ----------------------------------------------------------------------------
|  PathScript_OpMoveDist_079004  @ $079004  (10 B)
| ----------------------------------------------------------------------------
        .section .text.PathScript_OpMoveDist_079004, "ax", @progbits
        .global PathScript_OpMoveDist_079004
PathScript_OpMoveDist_079004:
        move.w  d5,d0                           | +000
        move.b  0x34(a6),d1                     | +002
        jsr     PathScript_IntegratePolar_0791e0(pc) | +006

| ----------------------------------------------------------------------------
|  PathScript_OpJump_079014  @ $079014  (12 B)
| ----------------------------------------------------------------------------
        .section .text.PathScript_OpJump_079014, "ax", @progbits
        .global PathScript_OpJump_079014
PathScript_OpJump_079014:
        movea.l 0x2(a0),a0                      | +000
        move.l  a0,0x90(a6)                     | +004
        bra.w   PathScript_Step_078f90__L078f98 | +008

| ----------------------------------------------------------------------------
|  PathScript_OpCall_079020  @ $079020  (24 B)
| ----------------------------------------------------------------------------
        .section .text.PathScript_OpCall_079020, "ax", @progbits
        .global PathScript_OpCall_079020
PathScript_OpCall_079020:
        movem.l d5/a0,-(a7)                     | +000
        movea.l 0x2(a0),a1                      | +004
        jsr     (a1)                            | +008
        movem.l (a7)+,d5/a0                     | +00a
        addq.l  #0x6,a0                         | +00e
        move.l  a0,0x90(a6)                     | +010
        bra.w   PathScript_Step_078f90__L078f98 | +014

| ----------------------------------------------------------------------------
|  PathScript_OpMusic_079038  @ $079038  (28 B)
| ----------------------------------------------------------------------------
        .section .text.PathScript_OpMusic_079038, "ax", @progbits
        .global PathScript_OpMusic_079038
PathScript_OpMusic_079038:
        movem.l d5/a0,-(a7)                     | +000
        move.w  0x2(a0),d0                      | +004
        jsr     0x2352.l                        | +008
        movem.l (a7)+,d5/a0                     | +00e
        addq.l  #0x4,a0                         | +012
        move.l  a0,0x90(a6)                     | +014
        bra.w   PathScript_Step_078f90__L078f98 | +018

| ----------------------------------------------------------------------------
|  PathScript_OpSetByte_079054  @ $079054  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PathScript_OpSetByte_079054, "ax", @progbits
        .global PathScript_OpSetByte_079054
PathScript_OpSetByte_079054:
        move.b  0x1(a0),d0                      | +000
        move.w  0x2(a0),d1                      | +004
        cmpi.w  #0xa0,d1                        | +008
        bcs.w   .L079070                        | +00c
        nop                                     | +010
        nop                                     | +012
        cmpi.w  #0xa0,d1                        | +014
        nop                                     | +018
        trap    #0xf                            | +01a
.L079070:
        move.b  d0,(a6,d1.w)                    | +01c
        addq.l  #0x4,a0                         | +020
        move.l  a0,0x90(a6)                     | +022
        bra.w   PathScript_Step_078f90__L078f98 | +026

| ----------------------------------------------------------------------------
|  PathScript_OpSetWord_07907e  @ $07907E  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PathScript_OpSetWord_07907e, "ax", @progbits
        .global PathScript_OpSetWord_07907e
PathScript_OpSetWord_07907e:
        move.w  0x2(a0),d0                      | +000
        move.w  0x4(a0),d1                      | +004
        cmpi.w  #0xa0,d1                        | +008
        bcs.w   .L07909a                        | +00c
        nop                                     | +010
        nop                                     | +012
        cmpi.w  #0xa0,d1                        | +014
        nop                                     | +018
        trap    #0xf                            | +01a
.L07909a:
        move.w  d0,(a6,d1.w)                    | +01c
        addq.l  #0x6,a0                         | +020
        move.l  a0,0x90(a6)                     | +022
        bra.w   PathScript_Step_078f90__L078f98 | +026

| ----------------------------------------------------------------------------
|  PathScript_OpSetLong_0790a8  @ $0790A8  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PathScript_OpSetLong_0790a8, "ax", @progbits
        .global PathScript_OpSetLong_0790a8
PathScript_OpSetLong_0790a8:
        move.l  0x4(a0),d0                      | +000
        move.w  0x2(a0),d1                      | +004
        cmpi.w  #0xa0,d1                        | +008
        bcs.w   .L0790c4                        | +00c
        nop                                     | +010
        nop                                     | +012
        cmpi.w  #0xa0,d1                        | +014
        nop                                     | +018
        trap    #0xf                            | +01a
.L0790c4:
        move.l  d0,(a6,d1.w)                    | +01c
        addq.l  #0x8,a0                         | +020
        move.l  a0,0x90(a6)                     | +022
        bra.w   PathScript_Step_078f90__L078f98 | +026

| ----------------------------------------------------------------------------
|  PathScript_OpSetAngle_0790d2  @ $0790D2  (20 B)
| ----------------------------------------------------------------------------
        .section .text.PathScript_OpSetAngle_0790d2, "ax", @progbits
        .global PathScript_OpSetAngle_0790d2
PathScript_OpSetAngle_0790d2:
        move.b  0x1(a0),d0                      | +000
        lsl.w   #0x8,d0                         | +004
        move.w  d0,0x34(a6)                     | +006
        addq.l  #0x2,a0                         | +00a
        move.l  a0,0x90(a6)                     | +00c
        bra.w   PathScript_Step_078f90__L078f98 | +010

| ----------------------------------------------------------------------------
|  PathScript_OpRepeat_0790e6  @ $0790E6  (26 B)
| ----------------------------------------------------------------------------
        .section .text.PathScript_OpRepeat_0790e6, "ax", @progbits
        .global PathScript_OpRepeat_0790e6
PathScript_OpRepeat_0790e6:
        addq.w  #0x1,0x94(a6)                   | +000
        move.w  0x2(a0),d0                      | +004
        cmp.w   0x94(a6),d0                     | +008
        bhi.w   ClearXN_079100                  | +00c
        clr.w   0x94(a6)                        | +010
        addq.l  #0x4,a0                         | +014
        move.l  a0,0x90(a6)                     | +016

| ----------------------------------------------------------------------------
|  PathScript_OpMoveLen_079106  @ $079106  (110 B)
| ----------------------------------------------------------------------------
        .section .text.PathScript_OpMoveLen_079106, "ax", @progbits
        .global PathScript_OpMoveLen_079106
PathScript_OpMoveLen_079106:
        cmpi.w  #0x0,d5                         | +000
        beq.w   ClearXN_079174                  | +004
        move.w  0x2(a0),d0                      | +008
        swap    d0                              | +00c
        andi.l  #0xffff0000,d0                  | +00e
        move.l  0x94(a6),d1                     | +014
        andi.l  #0xffffff00,d1                  | +018
        sub.l   d1,d0                           | +01e
        cmpi.l  #0x1000000,d0                   | +020
        bcc.w   .L07915c                        | +026
        lsr.l   #0x8,d0                         | +02a
        cmp.w   d0,d5                           | +02c
        bcs.w   .L07915c                        | +02e
        sub.w   d0,d5                           | +032
        clr.b   0x96(a6)                        | +034
        clr.w   0x94(a6)                        | +038
        move.b  0x34(a6),d1                     | +03c
        movem.l d5/a0,-(a7)                     | +040
        jsr     PathScript_IntegratePolar_0791e0(pc) | +044
        movem.l (a7)+,d5/a0                     | +048
        addq.l  #0x4,a0                         | +04c
        move.l  a0,0x90(a6)                     | +04e
        bra.w   PathScript_Step_078f90__L078f98 | +052
        .global PathScript_OpMoveLen_079106__L07915c
PathScript_OpMoveLen_079106__L07915c:
.L07915c:
        move.w  d5,d0                           | +056
        andi.l  #0xffff,d0                      | +058
        lsl.l   #0x8,d0                         | +05e
        add.l   d0,0x94(a6)                     | +060
        move.w  d5,d0                           | +064
        move.b  0x34(a6),d1                     | +066
        jsr     PathScript_IntegratePolar_0791e0(pc) | +06a

| ----------------------------------------------------------------------------
|  PathScript_OpArc_07917a  @ $07917A  (102 B)
| ----------------------------------------------------------------------------
        .section .text.PathScript_OpArc_07917a, "ax", @progbits
        .global PathScript_OpArc_07917a
PathScript_OpArc_07917a:
        cmpi.w  #0x0,d5                         | +000
        beq.b   ClearXN_079174                  | +004
        move.w  0x4(a0),d0                      | +006
        swap    d0                              | +00a
        andi.l  #0xffff0000,d0                  | +00c
        lsr.l   #0x8,d0                         | +012
        move.l  0x94(a6),d1                     | +014
        andi.l  #0xffffff00,d1                  | +018
        sub.l   d1,d0                           | +01e
        cmpi.l  #0x1000000,d0                   | +020
        bcc.b   PathScript_OpMoveLen_079106__L07915c | +026
        lsr.l   #0x8,d0                         | +028
        cmp.w   d0,d5                           | +02a
        bcs.b   PathScript_OpMoveLen_079106__L07915c | +02c
        sub.w   d0,d5                           | +02e
        clr.b   0x96(a6)                        | +030
        clr.w   0x94(a6)                        | +034
        move.b  0x34(a6),d1                     | +038
        movem.l d5/a0,-(a7)                     | +03c
        jsr     PathScript_IntegratePolar_0791e0(pc) | +040
        movem.l (a7)+,d5/a0                     | +044
        move.b  0x1(a0),d0                      | +048
        add.b   d0,0x34(a6)                     | +04c
        move.b  0x34(a6),d1                     | +050
        cmp.b   0x2(a0),d1                      | +054
        bne.w   .L0791dc                        | +058
        addq.l  #0x6,a0                         | +05c
        move.l  a0,0x90(a6)                     | +05e
.L0791dc:
        bra.w   PathScript_Step_078f90__L078f98 | +062

| ----------------------------------------------------------------------------
|  PathScript_IntegratePolar_0791e0  @ $0791E0  (106 B)
| ----------------------------------------------------------------------------
        .section .text.PathScript_IntegratePolar_0791e0, "ax", @progbits
        .global PathScript_IntegratePolar_0791e0
PathScript_IntegratePolar_0791e0:
        lea     0x2c072c.l,a2                   | +000
        lea     0x2c07ac.l,a3                   | +006
        andi.w  #0xff,d1                        | +00c
        add.w   d1,d1                           | +010
        move.w  (a2,d1.w),d2                    | +012
        move.w  (a3,d1.w),d3                    | +016
        muls.w  d0,d2                           | +01a
        muls.w  d0,d3                           | +01c
        moveq   #127,d1                         | +01e
        cmpi.l  #0x0,d2                         | +020
        bge.w   .L079210                        | +026
        move.l  #0x80,d1                        | +02a
.L079210:
        add.l   d1,d2                           | +030
        moveq   #127,d1                         | +032
        cmpi.l  #0x0,d3                         | +034
        bge.w   .L079224                        | +03a
        move.l  #0x80,d1                        | +03e
.L079224:
        add.l   d1,d3                           | +044
        move.l  d2,d1                           | +046
        swap    d2                              | +048
        asr.w   #0x8,d1                         | +04a
        move.w  0x24(a6),d0                     | +04c
        add.b   d1,0x27(a6)                     | +050
        addx.w  d2,d0                           | +054
        move.w  d0,0x24(a6)                     | +056
        move.l  d3,d1                           | +05a
        swap    d3                              | +05c
        asr.w   #0x8,d1                         | +05e
        move.w  0x22(a6),d0                     | +060
        add.b   d1,0x26(a6)                     | +064
        addx.w  d3,d0                           | +068

| ----------------------------------------------------------------------------
|  AutoDemo_RetGate_079250  @ $079250  (16 B)
| ----------------------------------------------------------------------------
        .section .text.AutoDemo_RetGate_079250, "ax", @progbits
        .global AutoDemo_RetGate_079250
AutoDemo_RetGate_079250:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_079266                    | +00c

| ----------------------------------------------------------------------------
|  AutoDemo_RecordBest_07926c  @ $07926C  (42 B)
| ----------------------------------------------------------------------------
        .section .text.AutoDemo_RecordBest_07926c, "ax", @progbits
        .global AutoDemo_RecordBest_07926c
AutoDemo_RecordBest_07926c:
        move.b  0x98(a6),d0                     | +000
        andi.w  #0xff,d0                        | +004
        cmpi.w  #0x63,d0                        | +008
        ble.w   .L079280                        | +00c
        move.w  #0x63,d0                        | +010
.L079280:
        cmp.w   0x106e92.l,d0                   | +014
        ble.w   .L079290                        | +01a
        move.w  d0,0x106e92.l                   | +01e
.L079290:
        jmp     0x518.l                         | +024

| ----------------------------------------------------------------------------
|  AutoDemo_Rts_079296  @ $079296  (2 B)
| ----------------------------------------------------------------------------
        .section .text.AutoDemo_Rts_079296, "ax", @progbits
        .global AutoDemo_Rts_079296
AutoDemo_Rts_079296:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  AutoDemo_Tmpl_Driver_079298  @ $079298  (142 B)
| ----------------------------------------------------------------------------
        .section .text.AutoDemo_Tmpl_Driver_079298, "ax", @progbits
        .global AutoDemo_Tmpl_Driver_079298
AutoDemo_Tmpl_Driver_079298:
        clr.b   d4                              | +000
        move.b  0x10e2e4.l,0x10e2e3.l           | +002
        move.b  0x10e2ea.l,0x10e2e9.l           | +00c
        move.b  d4,0x10e2e3.l                   | +016
        move.b  d4,0x10e2e9.l                   | +01c
        move.b  d4,0x10e2e4.l                   | +022
        move.b  d4,0x10e2ea.l                   | +028
        move.b  d4,0x10e2e5.l                   | +02e
        move.b  d4,0x10e2eb.l                   | +034
        move.b  d4,0x10e2e6.l                   | +03a
        move.b  d4,0x10e2ec.l                   | +040
        clr.b   0x10e39c.l                      | +046
        clr.b   0x10e39d.l                      | +04c
        lea     .L0792f0(pc),a1                 | +052
        move.l  a1,(a6)                         | +056
.L0792f0:
        jsr     AutoDemo_StartLevelScript_07962c(pc) | +058
        tst.b   0x10e39c.l                      | +05c
        beq.w   .L079324                        | +062
        clr.w   d0                              | +066
        jsr     0x5e3a2.l                       | +068
        bcc.w   .L079310                        | +06e
        move.b  #0x2,0x45(a0)                   | +072
.L079310:
        move.w  #0x1,d0                         | +078
        jsr     0x5e3a2.l                       | +07c
        bcc.w   .L079324                        | +082
        move.b  #0x2,0x45(a0)                   | +086
.L079324:
        rts                                     | +08c

| ----------------------------------------------------------------------------
|  AutoDemo_Stop_079326  @ $079326  (20 B)
| ----------------------------------------------------------------------------
        .section .text.AutoDemo_Stop_079326, "ax", @progbits
        .global AutoDemo_Stop_079326
AutoDemo_Stop_079326:
        clr.b   0x10e39d.l                      | +000
        clr.b   0x10e39c.l                      | +006
        jmp     0x518.l                         | +00c
        rts                                     | +012

| ----------------------------------------------------------------------------
|  AutoDemo_Level4Script_07933a  @ $07933A  (258 B)
| ----------------------------------------------------------------------------
        .section .text.AutoDemo_Level4Script_07933a, "ax", @progbits
        .global AutoDemo_Level4Script_07933a
AutoDemo_Level4Script_07933a:
        move.w  #0x14,0x70(a6)                  | +000
        clr.b   d4                              | +006
        move.b  0x10e2e4.l,0x10e2e3.l           | +008
        move.b  0x10e2ea.l,0x10e2e9.l           | +012
        move.b  d4,0x10e2e3.l                   | +01c
        move.b  d4,0x10e2e9.l                   | +022
        move.b  d4,0x10e2e4.l                   | +028
        move.b  d4,0x10e2ea.l                   | +02e
        move.b  d4,0x10e2e5.l                   | +034
        move.b  d4,0x10e2eb.l                   | +03a
        move.b  d4,0x10e2e6.l                   | +040
        move.b  d4,0x10e2ec.l                   | +046
        lea     .L07938c(pc),a1                 | +04c
        move.l  a1,(a6)                         | +050
.L07938c:
        cmpi.b  #0x4,0x10e39d.l                 | +052
        bne.b   AutoDemo_Stop_079326            | +05a
        cmpi.b  #0x0,0x10e39c.l                 | +05c
        beq.b   AutoDemo_Stop_079326            | +064
        clr.b   d4                              | +066
        move.b  0x10e2e4.l,0x10e2e3.l           | +068
        move.b  0x10e2ea.l,0x10e2e9.l           | +072
        move.b  d4,0x10e2e3.l                   | +07c
        move.b  d4,0x10e2e9.l                   | +082
        move.b  d4,0x10e2e4.l                   | +088
        move.b  d4,0x10e2ea.l                   | +08e
        move.b  d4,0x10e2e5.l                   | +094
        move.b  d4,0x10e2eb.l                   | +09a
        move.b  d4,0x10e2e6.l                   | +0a0
        move.b  d4,0x10e2ec.l                   | +0a6
        clr.w   d0                              | +0ac
        jsr     0x5e3a2.l                       | +0ae
        bcc.w   .L079404                        | +0b4
        cmpi.w  #0xc0,0x22(a0)                  | +0b8
        bgt.w   .L079404                        | +0be
        ori.b   #0x8,0x10e2e4.l                 | +0c2
.L079404:
        move.w  #0x1,d0                         | +0ca
        jsr     0x5e3a2.l                       | +0ce
        bcc.w   .L079424                        | +0d4
        cmpi.w  #0xc0,0x22(a0)                  | +0d8
        bgt.w   .L079424                        | +0de
        ori.b   #0x8,0x10e2ea.l                 | +0e2
.L079424:
        tst.w   0x106f5e.l                      | +0ea
        beq.w   SetHandlerRts_079442            | +0f0
        subq.w  #0x1,0x70(a6)                   | +0f4
        cmpi.w  #0x0,0x70(a6)                   | +0f8
        bgt.w   SetHandlerRts_079442            | +0fe

| ----------------------------------------------------------------------------
|  AutoDemo_Level3Script_079444  @ $079444  (102 B)
| ----------------------------------------------------------------------------
        .section .text.AutoDemo_Level3Script_079444, "ax", @progbits
        .global AutoDemo_Level3Script_079444
AutoDemo_Level3Script_079444:
        clr.b   d4                              | +000
        move.b  0x10e2e4.l,0x10e2e3.l           | +002
        move.b  0x10e2ea.l,0x10e2e9.l           | +00c
        move.b  d4,0x10e2e3.l                   | +016
        move.b  d4,0x10e2e9.l                   | +01c
        move.b  d4,0x10e2e4.l                   | +022
        move.b  d4,0x10e2ea.l                   | +028
        move.b  d4,0x10e2e5.l                   | +02e
        move.b  d4,0x10e2eb.l                   | +034
        move.b  d4,0x10e2e6.l                   | +03a
        move.b  d4,0x10e2ec.l                   | +040
        lea     .L079490(pc),a1                 | +046
        move.l  a1,(a6)                         | +04a
.L079490:
        cmpi.b  #0x3,0x10e39d.l                 | +04c
        bne.w   AutoDemo_Stop_079326            | +054
        cmpi.b  #0x0,0x10e39c.l                 | +058
        beq.w   AutoDemo_Stop_079326            | +060
        rts                                     | +064

| ----------------------------------------------------------------------------
|  AutoDemo_Level1Script_0794aa  @ $0794AA  (208 B)
| ----------------------------------------------------------------------------
        .section .text.AutoDemo_Level1Script_0794aa, "ax", @progbits
        .global AutoDemo_Level1Script_0794aa
AutoDemo_Level1Script_0794aa:
        lea     .L0794b0(pc),a1                 | +000
        move.l  a1,(a6)                         | +004
.L0794b0:
        clr.b   d4                              | +006
        move.b  0x10e2e4.l,0x10e2e3.l           | +008
        move.b  0x10e2ea.l,0x10e2e9.l           | +012
        move.b  d4,0x10e2e3.l                   | +01c
        move.b  d4,0x10e2e9.l                   | +022
        move.b  d4,0x10e2e4.l                   | +028
        move.b  d4,0x10e2ea.l                   | +02e
        move.b  d4,0x10e2e5.l                   | +034
        move.b  d4,0x10e2eb.l                   | +03a
        move.b  d4,0x10e2e6.l                   | +040
        move.b  d4,0x10e2ec.l                   | +046
        cmpi.b  #0x1,0x10e39d.l                 | +04c
        bne.w   AutoDemo_Stop_079326            | +054
        cmpi.b  #0x0,0x10e39c.l                 | +058
        beq.w   AutoDemo_Stop_079326            | +060
        clr.w   d0                              | +064
        jsr     0x5e3a2.l                       | +066
        bcc.w   .L079542                        | +06c
        cmpi.w  #0x78,0x22(a0)                  | +070
        blt.w   .L07953a                        | +076
        cmpi.w  #0x88,0x22(a0)                  | +07a
        ble.w   .L079542                        | +080
        ori.b   #0x4,0x10e2e4.l                 | +084
        bra.w   .L079542                        | +08c
.L07953a:
        ori.b   #0x8,0x10e2e4.l                 | +090
.L079542:
        move.w  #0x1,d0                         | +098
        jsr     0x5e3a2.l                       | +09c
        bcc.w   .L079578                        | +0a2
        cmpi.w  #0x40,0x22(a0)                  | +0a6
        blt.w   .L079570                        | +0ac
        cmpi.w  #0x50,0x22(a0)                  | +0b0
        ble.w   .L079578                        | +0b6
        ori.b   #0x4,0x10e2ea.l                 | +0ba
        bra.w   .L079578                        | +0c2
.L079570:
        ori.b   #0x8,0x10e2ea.l                 | +0c6
.L079578:
        rts                                     | +0ce

| ----------------------------------------------------------------------------
|  AutoDemo_Level2Script_07957a  @ $07957A  (176 B)
| ----------------------------------------------------------------------------
        .section .text.AutoDemo_Level2Script_07957a, "ax", @progbits
        .global AutoDemo_Level2Script_07957a
AutoDemo_Level2Script_07957a:
        clr.b   0x80(a6)                        | +000
        clr.b   0x81(a6)                        | +004
        move.w  #0x12c,0x70(a6)                 | +008
        move.w  #0x14,0x72(a6)                  | +00e
        lea     .L079594(pc),a1                 | +014
        move.l  a1,(a6)                         | +018
.L079594:
        cmpi.b  #0x2,0x10e39d.l                 | +01a
        bne.w   AutoDemo_Stop_079326            | +022
        cmpi.b  #0x0,0x10e39c.l                 | +026
        beq.w   AutoDemo_Stop_079326            | +02e
        clr.b   d4                              | +032
        move.b  0x10e2e4.l,0x10e2e3.l           | +034
        move.b  0x10e2ea.l,0x10e2e9.l           | +03e
        move.b  d4,0x10e2e3.l                   | +048
        move.b  d4,0x10e2e9.l                   | +04e
        move.b  d4,0x10e2e4.l                   | +054
        move.b  d4,0x10e2ea.l                   | +05a
        move.b  d4,0x10e2e5.l                   | +060
        move.b  d4,0x10e2eb.l                   | +066
        move.b  d4,0x10e2e6.l                   | +06c
        move.b  d4,0x10e2ec.l                   | +072
        cmpi.w  #0x0,0x72(a6)                   | +078
        bgt.w   .L079614                        | +07e
        ori.b   #0x8,0x10e2e4.l                 | +082
        ori.b   #0x8,0x10e2ea.l                 | +08a
        jsr     AutoDemo_JumpIfBlocked_0796f6(pc) | +092
        bra.w   .L079628                        | +096
.L079614:
        subq.w  #0x1,0x72(a6)                   | +09a
        ori.b   #0x4,0x10e2e4.l                 | +09e
        ori.b   #0x4,0x10e2ea.l                 | +0a6
.L079628:
        rts                                     | +0ae

| ----------------------------------------------------------------------------
|  AutoDemo_StartLevelScript_07962c  @ $07962C  (184 B)
| ----------------------------------------------------------------------------
        .section .text.AutoDemo_StartLevelScript_07962c, "ax", @progbits
        .global AutoDemo_StartLevelScript_07962c
AutoDemo_StartLevelScript_07962c:
        move.b  0x10e39c.l,d0                   | +000
        cmpi.b  #0x0,d0                         | +006
        beq.w   AutoDemo_ClearLevel_0796ea      | +00a
        cmpi.b  #0x5,d0                         | +00e
        bcs.w   .L07964e                        | +012
        nop                                     | +016
        nop                                     | +018
        cmpi.b  #0x5,d0                         | +01a
        nop                                     | +01e
        trap    #0xf                            | +020
.L07964e:
        cmp.b   0x10e39d.l,d0                   | +022
        beq.w   ClearXN_0796f0                  | +028
        move.b  d0,0x10e39d.l                   | +02c
        move.b  0x10e200.l,d1                   | +032
        move.b  d1,0x10e2e2.l                   | +038
        move.b  d1,0x10e2e8.l                   | +03e
        clr.b   d4                              | +044
        move.b  0x10e2e4.l,0x10e2e3.l           | +046
        move.b  0x10e2ea.l,0x10e2e9.l           | +050
        move.b  d4,0x10e2e3.l                   | +05a
        move.b  d4,0x10e2e9.l                   | +060
        move.b  d4,0x10e2e4.l                   | +066
        move.b  d4,0x10e2ea.l                   | +06c
        move.b  d4,0x10e2e5.l                   | +072
        move.b  d4,0x10e2eb.l                   | +078
        move.b  d4,0x10e2e6.l                   | +07e
        move.b  d4,0x10e2ec.l                   | +084
        lea     0x1001c0.l,a4                   | +08a
        move.b  #0x2,0x44(a4)                   | +090
        andi.w  #0xff,d0                        | +096
        lsl.w   #0x2,d0                         | +09a
        lea     0x2df442.l,a1                   | +09c
        move.l  (a1,d0.w),d0                    | +0a2
        cmpi.l  #0xffffffff,d0                  | +0a6
        beq.w   ClearXN_0796f0                  | +0ac
        movea.l d0,a1                           | +0b0
        jsr     0x4ae.l                         | +0b2

| ----------------------------------------------------------------------------
|  AutoDemo_ClearLevel_0796ea  @ $0796EA  (6 B)
| ----------------------------------------------------------------------------
        .section .text.AutoDemo_ClearLevel_0796ea, "ax", @progbits
        .global AutoDemo_ClearLevel_0796ea
AutoDemo_ClearLevel_0796ea:
        clr.b   0x10e39d.l                      | +000

| ----------------------------------------------------------------------------
|  AutoDemo_JumpIfBlocked_0796f6  @ $0796F6  (216 B)
| ----------------------------------------------------------------------------
        .section .text.AutoDemo_JumpIfBlocked_0796f6, "ax", @progbits
        .global AutoDemo_JumpIfBlocked_0796f6
AutoDemo_JumpIfBlocked_0796f6:
        tst.b   0x80(a6)                        | +000
        bne.w   .L079760                        | +004
        clr.w   d0                              | +008
        jsr     0x5e3a2.l                       | +00a
        bcc.w   .L079760                        | +010
        movem.l a6,-(a7)                        | +014
        move.l  a0,0x5c(a6)                     | +018
        movea.l a0,a6                           | +01c
        jsr     0x2a8c0.l                       | +01e
        movem.l (a7)+,a6                        | +024
        bcs.w   .L079760                        | +028
        movea.l 0x5c(a6),a0                     | +02c
        move.w  0x22(a0),d1                     | +030
        move.w  0x24(a0),d2                     | +034
        addi.w  #0x18,d1                        | +038
        subq.w  #0x8,d2                         | +03c
        jsr     0x27db2.l                       | +03e
        cmpi.b  #0x40,d7                        | +044
        beq.w   .L079760                        | +048
        move.b  #0x1,0x80(a6)                   | +04c
        ori.b   #0x1,0x10e2e4.l                 | +052
        ori.b   #0x1,0x10e2e5.l                 | +05a
        ori.b   #0x1,0x10e2e6.l                 | +062
.L079760:
        tst.b   0x81(a6)                        | +06a
        bne.w   .L0797cc                        | +06e
        move.w  #0x1,d0                         | +072
        jsr     0x5e3a2.l                       | +076
        bcc.w   .L0797cc                        | +07c
        movem.l a6,-(a7)                        | +080
        move.l  a0,0x5c(a6)                     | +084
        movea.l a0,a6                           | +088
        jsr     0x2a8c0.l                       | +08a
        movem.l (a7)+,a6                        | +090
        bcs.w   .L0797cc                        | +094
        movea.l 0x5c(a6),a0                     | +098
        move.w  0x22(a0),d1                     | +09c
        move.w  0x24(a0),d2                     | +0a0
        addi.w  #0x18,d1                        | +0a4
        subq.w  #0x8,d2                         | +0a8
        jsr     0x27db2.l                       | +0aa
        cmpi.b  #0x40,d7                        | +0b0
        beq.w   .L0797cc                        | +0b4
        move.b  #0x1,0x81(a6)                   | +0b8
        ori.b   #0x1,0x10e2ea.l                 | +0be
        ori.b   #0x1,0x10e2eb.l                 | +0c6
        ori.b   #0x1,0x10e2ec.l                 | +0ce
.L0797cc:
        rts                                     | +0d6

| ----------------------------------------------------------------------------
|  AutoDemo_DebugWarpKeys_0797ce  @ $0797CE  (90 B)
| ----------------------------------------------------------------------------
        .section .text.AutoDemo_DebugWarpKeys_0797ce, "ax", @progbits
        .global AutoDemo_DebugWarpKeys_0797ce
AutoDemo_DebugWarpKeys_0797ce:
        lea     0x10e200.l,a0                   | +000
        move.b  0x2(a0),d0                      | +006
        move.b  0x3(a0),d1                      | +00a
        move.b  d1,d2                           | +00e
        andi.b  #0x80,d0                        | +010
        andi.b  #0x40,d1                        | +014
        andi.b  #0x20,d2                        | +018
        tst.b   d0                              | +01c
        beq.w   .L079826                        | +01e
        tst.b   d1                              | +022
        beq.w   .L07981a                        | +024
        move.w  #0xa0,0x22(a6)                  | +028
        move.w  #0x180,0x24(a6)                 | +02e
        move.l  #0x2df456,0x4c(a6)              | +034
        jsr     0x283ca.l                       | +03c
        jsr     0x283d8.l                       | +042
        bra.w   .L079826                        | +048
.L07981a:
        tst.b   d2                              | +04c
        beq.w   .L079826                        | +04e
        clr.w   0x106e92.l                      | +052
.L079826:
        rts                                     | +058

| ----------------------------------------------------------------------------
|  AutoDemo_RetGateB_079828  @ $079828  (16 B)
| ----------------------------------------------------------------------------
        .section .text.AutoDemo_RetGateB_079828, "ax", @progbits
        .global AutoDemo_RetGateB_079828
AutoDemo_RetGateB_079828:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_07983e                    | +00c

| ----------------------------------------------------------------------------
|  Hit_TestPlayersAndWeapons_079844  @ $079844  (98 B)
| ----------------------------------------------------------------------------
        .section .text.Hit_TestPlayersAndWeapons_079844, "ax", @progbits
        .global Hit_TestPlayersAndWeapons_079844
Hit_TestPlayersAndWeapons_079844:
        move.l  a2,d0                           | +000
        cmpi.l  #0xffffffff,d0                  | +002
        beq.w   JsrPcRts_0798aa                 | +008
        movem.l a2,-(a7)                        | +00c
        clr.w   d0                              | +010
        jsr     0x5e3a2.l                       | +012
        movem.l (a7)+,a2                        | +018
        bcc.w   .L07987c                        | +01c
        movea.l 0x74(a0),a1                     | +020
        jsr     Entity_CheckBoxOverlapWithSelector_0798AC(pc) | +024
        movea.l 0x7c(a0),a1                     | +028
        jsr     Entity_CheckBoxOverlapWithSelector_0798AC(pc) | +02c
        movea.l 0x78(a0),a1                     | +030
        jsr     Entity_CheckBoxOverlapWithSelector_0798AC(pc) | +034
.L07987c:
        movem.l a2,-(a7)                        | +038
        move.w  #0x1,d0                         | +03c
        jsr     0x5e3a2.l                       | +040
        movem.l (a7)+,a2                        | +046
        bcc.w   JsrPcRts_0798aa                 | +04a
        movea.l 0x74(a0),a1                     | +04e
        jsr     Entity_CheckBoxOverlapWithSelector_0798AC(pc) | +052
        movea.l 0x7c(a0),a1                     | +056
        jsr     Entity_CheckBoxOverlapWithSelector_0798AC(pc) | +05a
        movea.l 0x78(a0),a1                     | +05e

| ----------------------------------------------------------------------------
|  Scale_X2_079952  @ $079952  (4 B)
| ----------------------------------------------------------------------------
        .section .text.Scale_X2_079952, "ax", @progbits
        .global Scale_X2_079952
Scale_X2_079952:
        add.w   d0,d0                           | +000
        rts                                     | +002

| ----------------------------------------------------------------------------
|  Scale_X1_5_079956  @ $079956  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Scale_X1_5_079956, "ax", @progbits
        .global Scale_X1_5_079956
Scale_X1_5_079956:
        move.w  d0,d1                           | +000
        asr.w   #0x1,d1                         | +002
        add.w   d1,d0                           | +004
        rts                                     | +006

| ----------------------------------------------------------------------------
|  Hit_TestPlayersDefault_07995e  @ $07995E  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Hit_TestPlayersDefault_07995e, "ax", @progbits
        .global Hit_TestPlayersDefault_07995e
Hit_TestPlayersDefault_07995e:
        lea     0x2df4ba.l,a2                   | +000
        jmp     Hit_TestPlayersAndWeapons_079844(pc) | +006

| ----------------------------------------------------------------------------
|  Rank_DelayTbl_079968  @ $079968  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Rank_DelayTbl_079968, "ax", @progbits
        .global Rank_DelayTbl_079968
Rank_DelayTbl_079968:
        ori.b   #0x46,ccr                       | +000
        ori.w   #0x5a,(a0)                      | +004

| ----------------------------------------------------------------------------
|  Rank_DelayByDifficulty_079970  @ $079970  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Rank_DelayByDifficulty_079970, "ax", @progbits
        .global Rank_DelayByDifficulty_079970
Rank_DelayByDifficulty_079970:
        lea     0x10fd84.l,a3                   | +000
        moveq   #0,d0                           | +006
        move.b  0x8(a3),d0                      | +008
        lea     Rank_DelayTbl_079968(pc),a0     | +00c
        lsl.w   #0x1,d0                         | +010
        move.w  (a0,d0.w),d0                    | +012
        rts                                     | +016

| ----------------------------------------------------------------------------
|  Rank_RetGate_079988  @ $079988  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Rank_RetGate_079988, "ax", @progbits
        .global Rank_RetGate_079988
Rank_RetGate_079988:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_07999e                    | +00c

| ----------------------------------------------------------------------------
|  Rank_SubIndex_0799a4  @ $0799A4  (58 B)
| ----------------------------------------------------------------------------
        .section .text.Rank_SubIndex_0799a4, "ax", @progbits
        .global Rank_SubIndex_0799a4
Rank_SubIndex_0799a4:
        move.b  0x10fd8b.l,d1                   | +000
        cmpi.b  #0x1,0x10fdaf.l                 | +006
        bne.w   .L0799ba                        | +00e
        move.b  #0x4,d1                         | +012
.L0799ba:
        move.l  a0,-(a7)                        | +016
        move.w  d0,-(a7)                        | +018
        move.b  d1,-(a7)                        | +01a
        jsr     0x2ac4c.l                       | +01c
        scs.b   d1                              | +022
        neg.b   d1                              | +024
        add.b   (a7)+,d1                        | +026
        move.w  (a7)+,d0                        | +028
        movea.l (a7)+,a0                        | +02a
        cmpi.b  #0x8,d1                         | +02c
        bcs.w   .L0799dc                        | +030
        move.b  #0x7,d1                         | +034
.L0799dc:
        rts                                     | +038

| ----------------------------------------------------------------------------
|  Tbl_DecodeShort_079A0E  @ $079A0E  (66 B)
| ----------------------------------------------------------------------------
        .section .text.Tbl_DecodeShort_079A0E, "ax", @progbits
        .global Tbl_DecodeShort_079A0E
Tbl_DecodeShort_079A0E:
        move.b  0x11(a6),d0                     | +000
        andi.w  #0x3,d0                         | +004
        add.w   d0,d0                           | +008
        move.b  0x106ed1.l,d1                   | +00a
        subq.b  #0x1,d1                         | +010
        andi.w  #0x1,d1                         | +012
        add.w   d1,d0                           | +016
        jsr     Rank_SubIndex_0799a4(pc)        | +018
        andi.w  #0x7,d1                         | +01c
        lsl.w   #0x3,d1                         | +020
        add.w   d1,d0                           | +022
        move.b  (a0,d0.w),d0                    | +024
        andi.w  #0xff,d0                        | +028
        lsl.w   #0x4,d0                         | +02c
        lea     0x40(a0,d0.w),a0                | +02e
        jsr     0x5e9b6.l                       | +032
        andi.w  #0xe,d0                         | +038
        move.w  (a0,d0.w),d0                    | +03c
        rts                                     | +040

| ----------------------------------------------------------------------------
|  Crew_RetGate_079a50  @ $079A50  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_RetGate_079a50, "ax", @progbits
        .global Crew_RetGate_079a50
Crew_RetGate_079a50:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_079a66                    | +00c

| ----------------------------------------------------------------------------
|  Crew_Tmpl125_079a6c  @ $079A6C  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_Tmpl125_079a6c, "ax", @progbits
        .global Crew_Tmpl125_079a6c
Crew_Tmpl125_079a6c:
        move.w  #0x19,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x1,0x66(a6)                   | +00a
        jsr     0x267e2.l                       | +010

| ----------------------------------------------------------------------------
|  Crew_Jump_Run_079a8a  @ $079A8A  (82 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_Jump_Run_079a8a, "ax", @progbits
        .global Crew_Jump_Run_079a8a
Crew_Jump_Run_079a8a:
        jsr     0x5e7c0.l                       | +000
        move.w  #0x8000,d0                      | +006
        jsr     0x28134.l                       | +00a
        andi.w  #0xffe3,0x38(a6)                | +010
        ori.w   #0xc,0x38(a6)                   | +016
        lea     0x29c0ae.l,a0                   | +01c
        jsr     0x28cd4.l                       | +022
        lea     .L079ab8(pc),a1                 | +028
        move.l  a1,(a6)                         | +02c
.L079ab8:
        jsr     0x2783a.l                       | +02e
        jsr     0x28d70.l                       | +034
        jsr     0x2870a.l                       | +03a
        bcc.w   .L079ad4                        | +040
        lea     Crew_Land_079ae4(pc),a1         | +044
        move.l  a1,(a6)                         | +048
.L079ad4:
        jsr     Crew_FreeIfOffWorld_079b4a(pc)  | +04a
        bcc.w   SetHandlerRts_079ae2            | +04e

| ----------------------------------------------------------------------------
|  Crew_Land_079ae4  @ $079AE4  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_Land_079ae4, "ax", @progbits
        .global Crew_Land_079ae4
Crew_Land_079ae4:
        bclr    #0x1,0x12(a6)                   | +000
        lea     0x29c1d4.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     .L079afc(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L079afc:
        jsr     0x2783a.l                       | +018
        jsr     0x28d70.l                       | +01e
        bcc.w   .L079b12                        | +024
        lea     Crew_Flee_079b22(pc),a1         | +028
        move.l  a1,(a6)                         | +02c
.L079b12:
        jsr     Crew_FreeIfOffWorld_079b4a(pc)  | +02e
        bcc.w   SetHandlerRts_079b20            | +032

| ----------------------------------------------------------------------------
|  Crew_Flee_079b22  @ $079B22  (32 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_Flee_079b22, "ax", @progbits
        .global Crew_Flee_079b22
Crew_Flee_079b22:
        move.b  #0x28,0x59(a6)                  | +000
        lea     .L079b2e(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L079b2e:
        jsr     0x2783a.l                       | +00c
        jsr     0x28d70.l                       | +012
        tst.b   0x59(a6)                        | +018
        bne.w   Crew_Free_079b42__L079b48       | +01c

| ----------------------------------------------------------------------------
|  Crew_Free_079b42  @ $079B42  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_Free_079b42, "ax", @progbits
        .global Crew_Free_079b42
Crew_Free_079b42:
        jmp     0x518.l                         | +000
        .global Crew_Free_079b42__L079b48
Crew_Free_079b42__L079b48:
.L079b48:
        rts                                     | +006

| ----------------------------------------------------------------------------
|  Crew_FreeIfOffWorld_079b4a  @ $079B4A  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_FreeIfOffWorld_079b4a, "ax", @progbits
        .global Crew_FreeIfOffWorld_079b4a
Crew_FreeIfOffWorld_079b4a:
        lea     Crew_OffWorldBox_079b64(pc),a0  | +000
        jsr     0x5dd5c.l                       | +004
        bcc.w   ClearC_079b5e                   | +00a

| ----------------------------------------------------------------------------
|  Crew_OffWorldBox_079b64  @ $079B64  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_OffWorldBox_079b64, "ax", @progbits
        .global Crew_OffWorldBox_079b64
Crew_OffWorldBox_079b64:
        .dc.w   0xfff0                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Crew_Hatch_079b6e  @ $079B6E  (156 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_Hatch_079b6e, "ax", @progbits
        .global Crew_Hatch_079b6e
Crew_Hatch_079b6e:
        jsr     0x5e9b6.l                       | +000
        btst    #0x0,d0                         | +006
        beq.w   .L079b82                        | +00a
        jmp     0x79d9a.l                       | +00e
.L079b82:
        movea.l 0xc(a6),a0                      | +014
        move.w  0x72(a6),d0                     | +018
        bset    d0,0x70(a0)                     | +01c
        move.w  #0x19,d1                        | +020
        jsr     0x236e.l                        | +024
        addq.w  #0x3,0x38(a6)                   | +02a
        move.w  0x38(a6),d0                     | +02e
        jsr     0x28134.l                       | +032
        move.w  #0x1,0x66(a6)                   | +038
        jsr     0x267e2.l                       | +03e
        lea     0x29c0ae.l,a0                   | +044
        jsr     0x28cd4.l                       | +04a
        lea     .L079bc4(pc),a1                 | +050
        move.l  a1,(a6)                         | +054
.L079bc4:
        jsr     0x7fdfa.l                       | +056
        jsr     0x28d70.l                       | +05c
        movea.l 0xc(a6),a0                      | +062
        cmpi.b  #0x3,0x21(a0)                   | +066
        beq.w   .L079be8                        | +06c
        cmpi.b  #0xff,0x20(a0)                  | +070
        bne.w   .L079bf0                        | +076
.L079be8:
        lea     0x77efe.l,a1                    | +07a
        move.l  a1,(a6)                         | +080
.L079bf0:
        jsr     0x2870a.l                       | +082
        bcc.w   .L079c00                        | +088
        lea     Crew_Hatch_Land_079c12(pc),a1   | +08c
        move.l  a1,(a6)                         | +090
.L079c00:
        jsr     0x7fde0.l                       | +092
        bcc.w   SetHandlerRts_079c10            | +098

| ----------------------------------------------------------------------------
|  Crew_Hatch_Land_079c12  @ $079C12  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_Hatch_Land_079c12, "ax", @progbits
        .global Crew_Hatch_Land_079c12
Crew_Hatch_Land_079c12:
        lea     0x29c1d4.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L079c24(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L079c24:
        jsr     0x7fdfa.l                       | +012
        jsr     0x28d70.l                       | +018
        bcc.w   SetHandlerRts_079c3a            | +01e

| ----------------------------------------------------------------------------
|  Crew_Hatch_Flee_079c3c  @ $079C3C  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_Hatch_Flee_079c3c, "ax", @progbits
        .global Crew_Hatch_Flee_079c3c
Crew_Hatch_Flee_079c3c:
        move.b  #0x28,0x59(a6)                  | +000
        lea     .L079c48(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L079c48:
        jsr     0x7fdfa.l                       | +00c
        jsr     0x28d70.l                       | +012
        tst.b   0x59(a6)                        | +018
        bne.w   Crew_Hatch_Free_079c68__L079c6e | +01c
        movea.l 0xc(a6),a0                      | +020
        move.w  0x72(a6),d0                     | +024
        bclr    d0,0x70(a0)                     | +028

| ----------------------------------------------------------------------------
|  Crew_Hatch_Free_079c68  @ $079C68  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_Hatch_Free_079c68, "ax", @progbits
        .global Crew_Hatch_Free_079c68
Crew_Hatch_Free_079c68:
        jmp     0x518.l                         | +000
        .global Crew_Hatch_Free_079c68__L079c6e
Crew_Hatch_Free_079c68__L079c6e:
.L079c6e:
        rts                                     | +006

| ----------------------------------------------------------------------------
|  Crew_RetGateB_079c70  @ $079C70  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_RetGateB_079c70, "ax", @progbits
        .global Crew_RetGateB_079c70
Crew_RetGateB_079c70:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_079c86                    | +00c

| ----------------------------------------------------------------------------
|  Crew_Tmpl126_079c8c  @ $079C8C  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_Tmpl126_079c8c, "ax", @progbits
        .global Crew_Tmpl126_079c8c
Crew_Tmpl126_079c8c:
        move.w  #0x19,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x1,0x66(a6)                   | +00a
        jsr     0x267e2.l                       | +010

| ----------------------------------------------------------------------------
|  Crew_JumpB_Run_079caa  @ $079CAA  (82 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_JumpB_Run_079caa, "ax", @progbits
        .global Crew_JumpB_Run_079caa
Crew_JumpB_Run_079caa:
        jsr     0x5e7c0.l                       | +000
        move.w  #0x8000,d0                      | +006
        jsr     0x28134.l                       | +00a
        andi.w  #0xffe3,0x38(a6)                | +010
        ori.w   #0xc,0x38(a6)                   | +016
        lea     0x29c244.l,a0                   | +01c
        jsr     0x28cd4.l                       | +022
        lea     .L079cd8(pc),a1                 | +028
        move.l  a1,(a6)                         | +02c
.L079cd8:
        jsr     0x2783a.l                       | +02e
        jsr     0x28d70.l                       | +034
        jsr     0x2870a.l                       | +03a
        bcc.w   .L079cf4                        | +040
        lea     Crew_LandB_079d04(pc),a1        | +044
        move.l  a1,(a6)                         | +048
.L079cf4:
        jsr     Crew_FreeIfOffWorldB_079d7e(pc) | +04a
        bcc.w   SetHandlerRts_079d02            | +04e

| ----------------------------------------------------------------------------
|  Crew_LandB_079d04  @ $079D04  (74 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_LandB_079d04, "ax", @progbits
        .global Crew_LandB_079d04
Crew_LandB_079d04:
        tst.b   0x10fd8f.l                      | +000
        bne.w   .L079d18                        | +006
        move.w  #0x1d5,d1                       | +00a
        jsr     0x236e.l                        | +00e
.L079d18:
        bclr    #0x1,0x12(a6)                   | +014
        lea     0x29c28c.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        lea     .L079d30(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L079d30:
        jsr     0x2783a.l                       | +02c
        jsr     0x28d70.l                       | +032
        bcc.w   .L079d46                        | +038
        lea     Crew_FleeB_079d56(pc),a1        | +03c
        move.l  a1,(a6)                         | +040
.L079d46:
        jsr     Crew_FreeIfOffWorldB_079d7e(pc) | +042
        bcc.w   SetHandlerRts_079d54            | +046

| ----------------------------------------------------------------------------
|  Crew_FleeB_079d56  @ $079D56  (32 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_FleeB_079d56, "ax", @progbits
        .global Crew_FleeB_079d56
Crew_FleeB_079d56:
        move.b  #0x28,0x59(a6)                  | +000
        lea     .L079d62(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L079d62:
        jsr     0x2783a.l                       | +00c
        jsr     0x28d70.l                       | +012
        tst.b   0x59(a6)                        | +018
        bne.w   Crew_FreeB_079d76__L079d7c      | +01c

| ----------------------------------------------------------------------------
|  Crew_FreeB_079d76  @ $079D76  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_FreeB_079d76, "ax", @progbits
        .global Crew_FreeB_079d76
Crew_FreeB_079d76:
        jmp     0x518.l                         | +000
        .global Crew_FreeB_079d76__L079d7c
Crew_FreeB_079d76__L079d7c:
.L079d7c:
        rts                                     | +006

| ----------------------------------------------------------------------------
|  Crew_FreeIfOffWorldB_079d7e  @ $079D7E  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_FreeIfOffWorldB_079d7e, "ax", @progbits
        .global Crew_FreeIfOffWorldB_079d7e
Crew_FreeIfOffWorldB_079d7e:
        lea     0x2df4e2.l,a0                   | +000
        jsr     0x5dd5c.l                       | +006
        bcc.w   ClearC_079d94                   | +00c

| ----------------------------------------------------------------------------
|  Crew_HatchB_079d9a  @ $079D9A  (136 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_HatchB_079d9a, "ax", @progbits
        .global Crew_HatchB_079d9a
Crew_HatchB_079d9a:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x72(a6),d0                     | +004
        bset    d0,0x70(a0)                     | +008
        move.w  #0x19,d1                        | +00c
        jsr     0x236e.l                        | +010
        addq.w  #0x3,0x38(a6)                   | +016
        move.w  0x38(a6),d0                     | +01a
        jsr     0x28134.l                       | +01e
        move.w  #0x1,0x66(a6)                   | +024
        jsr     0x267e2.l                       | +02a
        lea     0x29c244.l,a0                   | +030
        jsr     0x28cd4.l                       | +036
        lea     .L079ddc(pc),a1                 | +03c
        move.l  a1,(a6)                         | +040
.L079ddc:
        jsr     0x7fdfa.l                       | +042
        jsr     0x28d70.l                       | +048
        movea.l 0xc(a6),a0                      | +04e
        cmpi.b  #0x3,0x21(a0)                   | +052
        beq.w   .L079e00                        | +058
        cmpi.b  #0xff,0x20(a0)                  | +05c
        bne.w   .L079e08                        | +062
.L079e00:
        lea     0x77efe.l,a1                    | +066
        move.l  a1,(a6)                         | +06c
.L079e08:
        jsr     0x2870a.l                       | +06e
        bcc.w   .L079e18                        | +074
        lea     Crew_HatchB_Land_079e2a(pc),a1  | +078
        move.l  a1,(a6)                         | +07c
.L079e18:
        jsr     0x7fde0.l                       | +07e
        bcc.w   SetHandlerRts_079e28            | +084

| ----------------------------------------------------------------------------
|  Crew_HatchB_Land_079e2a  @ $079E2A  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_HatchB_Land_079e2a, "ax", @progbits
        .global Crew_HatchB_Land_079e2a
Crew_HatchB_Land_079e2a:
        tst.b   0x10fd8f.l                      | +000
        bne.w   .L079e3e                        | +006
        move.w  #0x1d5,d1                       | +00a
        jsr     0x236e.l                        | +00e
.L079e3e:
        lea     0x29c28c.l,a0                   | +014
        jsr     0x28cd4.l                       | +01a
        lea     .L079e50(pc),a1                 | +020
        move.l  a1,(a6)                         | +024
.L079e50:
        jsr     0x7fdfa.l                       | +026
        jsr     0x28d70.l                       | +02c
        bcc.w   SetHandlerRts_079e66            | +032

| ----------------------------------------------------------------------------
|  Crew_HatchB_Flee_079e68  @ $079E68  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_HatchB_Flee_079e68, "ax", @progbits
        .global Crew_HatchB_Flee_079e68
Crew_HatchB_Flee_079e68:
        move.b  #0x28,0x59(a6)                  | +000
        lea     .L079e74(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L079e74:
        jsr     0x7fdfa.l                       | +00c
        jsr     0x28d70.l                       | +012
        tst.b   0x59(a6)                        | +018
        bne.w   Crew_HatchB_Free_079e94__L079e9a | +01c
        movea.l 0xc(a6),a0                      | +020
        move.w  0x72(a6),d0                     | +024
        bclr    d0,0x70(a0)                     | +028

| ----------------------------------------------------------------------------
|  Crew_HatchB_Free_079e94  @ $079E94  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_HatchB_Free_079e94, "ax", @progbits
        .global Crew_HatchB_Free_079e94
Crew_HatchB_Free_079e94:
        jmp     0x518.l                         | +000
        .global Crew_HatchB_Free_079e94__L079e9a
Crew_HatchB_Free_079e94__L079e9a:
.L079e9a:
        rts                                     | +006

| ----------------------------------------------------------------------------
|  Crew_RetGateC_079e9c  @ $079E9C  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_RetGateC_079e9c, "ax", @progbits
        .global Crew_RetGateC_079e9c
Crew_RetGateC_079e9c:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_079eb2                    | +00c

| ----------------------------------------------------------------------------
|  Crew_Tmpl127_079eb8  @ $079EB8  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_Tmpl127_079eb8, "ax", @progbits
        .global Crew_Tmpl127_079eb8
Crew_Tmpl127_079eb8:
        move.w  #0xffff,0x70(a6)                | +000
        bra.w   Crew_Tmpl128_079ec2__L079ec8    | +006

| ----------------------------------------------------------------------------
|  Crew_Tmpl128_079ec2  @ $079EC2  (134 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_Tmpl128_079ec2, "ax", @progbits
        .global Crew_Tmpl128_079ec2
Crew_Tmpl128_079ec2:
        move.w  #0x0,0x70(a6)                   | +000
        .global Crew_Tmpl128_079ec2__L079ec8
Crew_Tmpl128_079ec2__L079ec8:
.L079ec8:
        moveq   #0,d0                           | +006
        move.b  d0,0x20(a6)                     | +008
        move.b  d0,0x21(a6)                     | +00c
        lea     Crew_Driver_079fe8(pc),a1       | +010
        jsr     0x4ae.l                         | +014
        jsr     0x5dd02.l                       | +01a
        subi.w  #0x40,0x22(a0)                  | +020
        lea     Crew_Hostage_Init_07a19e(pc),a1             | +026  -> $07A19E (hueco futuro, defsym forward)
        jsr     0x4ae.l                         | +02a
        jsr     0x5dd02.l                       | +030
        subi.w  #0x20,0x22(a0)                  | +036
        lea     Crew_Gunner_079f48(pc),a1       | +03c
        jsr     0x4ae.l                         | +040
        jsr     0x5dd02.l                       | +046
        bclr    #0x1,0x12(a6)                   | +04c
        lea     .L079f1a(pc),a1                 | +052
        move.l  a1,(a6)                         | +056
.L079f1a:
        tst.w   0x70(a6)                        | +058
        beq.w   .L079f30                        | +05c
        movea.l 0xc(a6),a0                      | +060
        movea.l 0xc(a0),a0                      | +064
        move.b  0x20(a0),0x20(a6)               | +068
.L079f30:
        cmpi.b  #0xff,0x21(a6)                  | +06e
        bne.w   .L079f40                        | +074
        jmp     0x518.l                         | +078
.L079f40:
        move.w  #0x0,0x72(a6)                   | +07e
        rts                                     | +084

| ----------------------------------------------------------------------------
|  Crew_Gunner_079f48  @ $079F48  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_Gunner_079f48, "ax", @progbits
        .global Crew_Gunner_079f48
Crew_Gunner_079f48:
        move.w  #0xe,d1                         | +000
        jsr     0x236e.l                        | +004
        moveq   #0,d0                           | +00a
        move.b  d0,0x20(a6)                     | +00c
        move.b  d0,0x21(a6)                     | +010
        move.w  #0x1,0x66(a6)                   | +014

| ----------------------------------------------------------------------------
|  Crew_Gunner_Run_079f6a  @ $079F6A  (118 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_Gunner_Run_079f6a, "ax", @progbits
        .global Crew_Gunner_Run_079f6a
Crew_Gunner_Run_079f6a:
        jsr     0x5e7c0.l                       | +000
        move.w  #0x8000,d0                      | +006
        jsr     0x28134.l                       | +00a
        andi.w  #0xffe3,0x38(a6)                | +010
        ori.w   #0xc,0x38(a6)                   | +016
        lea     0x2df4f2.l,a0                   | +01c
        jsr     0x28cd4.l                       | +022
        lea     .L079f98(pc),a1                 | +028
        move.l  a1,(a6)                         | +02c
.L079f98:
        jsr     0x2783a.l                       | +02e
        jsr     0x28d70.l                       | +034
        movea.l 0xc(a6),a0                      | +03a
        cmpi.b  #0xff,0x20(a0)                  | +03e
        beq.w   .L079fbc                        | +044
        cmpi.b  #0xff,0x21(a0)                  | +048
        bne.w   .L079fc4                        | +04e
.L079fbc:
        lea     0x58fc2.l,a1                    | +052
        move.l  a1,(a6)                         | +058
.L079fc4:
        jsr     0x49fd0.l                       | +05a
        movea.l #0xffffffff,a0                  | +060
        lea     0x2df594.l,a0                   | +066
        jsr     0x5dd5c.l                       | +06c
        bcc.w   SetHandlerRts_079fe6            | +072

| ----------------------------------------------------------------------------
|  Crew_Driver_079fe8  @ $079FE8  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Crew_Driver_079fe8, "ax", @progbits
        .global Crew_Driver_079fe8
Crew_Driver_079fe8:
        move.w  #0xe,d1                         | +000
        jsr     0x236e.l                        | +004
        move.w  #0x1,0x66(a6)                   | +00a
        moveq   #0,d0                           | +010
        move.b  d0,0x20(a6)                     | +012
        move.b  d0,0x21(a6)                     | +016
