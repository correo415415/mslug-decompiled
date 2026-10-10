| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave PPPP — props de la misión del fuerte: barrera, caseta/portón,
|  fortaleza (cadena de spawns por scroll), tejado, sensor, caja,
|  escombros rebotantes, carteles, barca; parpadeo del fix
|  Región: $04E580..$04FA50  (5,296 B, 40 entradas, 5 huecos)
| ============================================================================
|
|  A) QUÉ ES
|  40 handlers de tarea (entidad en a6) para los props de la misión del
|  fuerte/portón y utilidades afines. Tres familias enlazadas: la BARRERA
|  ($04E580..$04E7FE: activa → dañada → restos, con poste $04ED90 y luz
|  $04EE0A/$04EECA), la CASETA/PORTÓN ($04E7FE..$04EB18: intacta → dañada
|  → restos, bloqueador de paso $04EAE8, tejado $04EF0A/$04EFA2, puerta
|  $04EFE6) y la FORTALEZA ($04EB18..$04ED90: espera de scroll → activa →
|  restos, bloqueador $04EDDA, soporte de torreta $04F030/$04F0B0 y lateral
|  $04F138). Además: secuencia de blits $04F1F4, chequeo slot/prio $04F2A4,
|  tejado genérico $04F2C2/$04F344 (plantilla $E840C), parpadeo del fix en
|  3 fases $04F3AE/$04F40C, sensor de disparo $04F46A..$04F4EE, caja
|  destructible $04F4EE, escombros rebotantes $04F5BE..$04F70E, carteles
|  A/B $04F70E/$04F76C (plantillas $E8414/$E8418), barca $04F7CA..$04FA40
|  (plantilla $E8410, salpicadura $99812) y FixTile_Set11C2 $04FA40.
|
|  B) CÓMO FUNCIONA
|  Protocolo de estado +$20/+$21 habitual; cada etapa instala el siguiente
|  handler en +$00. Los props del fuerte se inicializan con la batería de
|  subrutinas $4FB8A..$4FD2C (hueco siguiente) que fijan hitbox (+$48),
|  HP (+$66), tipo de daño (+$58) y prioridad; usan $44022
|  (Coord_ScreenToLocal) para colocar hijos en posiciones absolutas del
|  mapa y $4429E (MissionWatch_Spawn) con listas $E9348/$E93B0/$E9442
|  para encadenar spawns según umbrales de scroll $A10/$A20/$A70 leídos
|  de $106F50. Al agotarse HP ($28758) saltan a la etapa "Damaged"/"Wreck"
|  que lanza explosiones ($77F6A), escombros ($77C7E) y puntos ($51A28).
|  El sensor consulta $5E086/$31FC2 y dispara la etapa Triggered. Los
|  escombros rebotantes aplican gravedad +$2E con rebote (Hop) y rodadura
|  (Roll) hasta quedar fuera de pantalla. FixBlink3 alterna el tile fijo
|  ($2C26) en dos fases con contador. FixTile_Set11C2 escribe el tile
|  $11C2 referenciando $1948A8/$1948D0.
|
|  C) INTERFAZ
|  Entrada: a6 = entidad; los padres dejan en +$0C el enlace; posición
|  +$22/+$24, velocidad +$28/+$2A, gravedad +$2E, prio +$38, facing
|  +$3A, plantilla +$3C, colisión +$48, HP +$66, +$70 flags de daño.
|  Salida: nuevas entidades vía $4AE/$6FE (copia de pos con $5DD22),
|  sonidos $236E, música $2352, registro de misión $43FAC.
|
|  D) EVIDENCIAS
|  - Plantillas $E840C (Roof), $E8410 (Boat), $E8414/$E8418 (SignA/B)
|    coinciden con el índice de spawn $E8000 usado en waves anteriores.
|  - Umbrales $A10/$A20/$A70 comparados contra $106F50 (scroll X).
|  - Llamadas a $28758/$2870A (HP agotado / golpe recibido) delimitan
|    las transiciones Active→Damaged→Wreck.
|  - $77F6A (AnimSeq explosión) y $77C7E (debris) en todas las etapas Wreck.
|
|  E) HIPÓTESIS / DUDAS
|  - Los nombres Barrier/Gatehouse/Fortress son por morfología del código
|    y plantillas; falta confirmar con el mapa de la misión in-game.
|  - Las subs $4FB8A..$5017A (setup de props) quedan para la wave QQQQ;
|    aquí se referencian como Sub_* provisionales.
|  - Prop_SlotPrioCheckRts_04f2a4 podría ser un helper compartido con
|    otras misiones (varios call-sites externos).
|
|  F) ESTADO
|  40/40 entradas byte-exactas; 5 huecos internos (datos/alineación) que
|  siguen en el pool de pendientes. Sin C: todo ASM a mano.
|
|  Verificación: cada sección .text.<Sym> se coloca en su dirección CPU
|  absoluta y reensambla byte-exacta contra build/mslug_prom.bin
|  (MD5 816b3f74c76b3373993407615f1850fe).
| ============================================================================

        .text

| ----------------------------------------------------------------------------
|  Prop_BarrierActive_04e580  @ $04E580  (266 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_BarrierActive_04e580, "ax", @progbits
        .global Prop_BarrierActive_04e580
Prop_BarrierActive_04e580:
        lea     0x294e7c.l,a0                   | +000
        move.l  a0,0x48(a6)                     | +006
        move.w  #0x80,0x70(a6)                  | +00a
        move.w  #0x64,0x66(a6)                  | +010
        move.b  #0x0,0x21(a6)                   | +016
        move.b  #0x0,0x20(a6)                   | +01c
        jsr     Barrier_SpawnPiece01_04fb8a(pc)                | +022
        jsr     Barrier_SpawnPiece02_04fbe0(pc)                | +026
        jsr     Barrier_SpawnPiece04_04fc36(pc)                | +02a
        jsr     Barrier_SpawnPiece08_04fc8c(pc)                | +02e
        jsr     Barrier_SpawnPieceLeft_04fcd8(pc)                | +032
        jsr     Barrier_SpawnPieceRight_04fd2c(pc)                | +036
        lea     Prop_BarrierLight_04ee0a(pc),a1 | +03a
        jsr     0x4ae.l                         | +03e
        jsr     0x5dd22.l                       | +044
        addi.w  #0x20,0x22(a0)                  | +04a
        addi.w  #0x20,0x24(a0)                  | +050
        lea     0xe9348.l,a1                    | +056
        move.w  #0x75,d0                        | +05c
        move.b  #0x0,0x75(a6)                   | +060
        jsr     0x4429e.l                       | +066
        lea     .L04e5f2(pc),a1                 | +06c
        move.l  a1,(a6)                         | +070
.L04e5f2:
        clr.b   0x10e39a.l                      | +072
        jsr     0x2783a.l                       | +078
        jsr     0x28998.l                       | +07e
        jsr     0x28d70.l                       | +084
        jsr     0x2870a.l                       | +08a
        bcc.w   .L04e626                        | +090
        lea     0x5e766.l,a0                    | +094
        jsr     0x5e770.l                       | +09a
        bclr    #0x3,0x13(a6)                   | +0a0
.L04e626:
        jsr     0x28758.l                       | +0a6
        bcc.w   .L04e640                        | +0ac
        move.b  #0xff,0x20(a6)                  | +0b0
        bclr    #0x0,0x13(a6)                   | +0b6
        bra.w   .L04e64a                        | +0bc
.L04e640:
        cmpi.b  #0xf,0x21(a6)                   | +0c0
        bne.w   .L04e67a                        | +0c6
.L04e64a:
        lea     0x295c32.l,a1                   | +0ca
        jsr     0x77c7e.l                       | +0d0
        lea     0x295c44.l,a1                   | +0d6
        jsr     0x77c7e.l                       | +0dc
        lea     0x295cd4.l,a1                   | +0e2
        jsr     0x77c7e.l                       | +0e8
        move.b  #0xff,0x20(a6)                  | +0ee
        lea     Prop_BarrierDamaged_04e68a(pc),a1 | +0f4
        move.l  a1,(a6)                         | +0f8
.L04e67a:
        jsr     Prop_OffscreenLeftCheck_04fa70(pc)                | +0fa
        bcc.w   .L04e688                        | +0fe
        jmp     0x518.l                         | +102
.L04e688:
        rts                                     | +108

| ----------------------------------------------------------------------------
|  Prop_BarrierDamaged_04e68a  @ $04E68A  (194 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_BarrierDamaged_04e68a, "ax", @progbits
        .global Prop_BarrierDamaged_04e68a
Prop_BarrierDamaged_04e68a:
        move.w  #0x102f,d0                      | +000
        jsr     0x2352.l                        | +004
        move.w  #0x64,0x66(a6)                  | +00a
        bclr    #0x0,0x13(a6)                   | +010
        lea     0x2957c2.l,a2                   | +016
        jsr     Sprite_InvokeBlit8Params(pc)    | +01c
        lea     0x2957ea.l,a2                   | +020
        jsr     Sprite_InvokeBlit8Params(pc)    | +026
        lea     .L04e6ba(pc),a1                 | +02a
        move.l  a1,(a6)                         | +02e
.L04e6ba:
        clr.b   0x10e39a.l                      | +030
        jsr     0x2783a.l                       | +036
        jsr     0x28998.l                       | +03c
        jsr     0x28d70.l                       | +042
        jsr     0x2870a.l                       | +048
        bcc.w   .L04e6ee                        | +04e
        lea     0x5e766.l,a0                    | +052
        jsr     0x5e770.l                       | +058
        bclr    #0x3,0x13(a6)                   | +05e
.L04e6ee:
        jsr     0x28758.l                       | +064
        bcc.w   .L04e73c                        | +06a
        lea     0x295c32.l,a1                   | +06e
        jsr     0x77c7e.l                       | +074
        lea     0x295c44.l,a1                   | +07a
        jsr     0x77c7e.l                       | +080
        lea     0x295cd4.l,a1                   | +086
        jsr     0x77c7e.l                       | +08c
        lea     0x295e4e.l,a1                   | +092
        jsr     0x43fac.l                       | +098
        move.b  #0x88,0x20(a6)                  | +09e
        move.b  #0x1,0x10e39e.l                 | +0a4
        lea     Prop_BarrierWreck_04e74c(pc),a1 | +0ac
        move.l  a1,(a6)                         | +0b0
.L04e73c:
        jsr     Prop_OffscreenLeftCheck_04fa70(pc)                | +0b2
        bcc.w   .L04e74a                        | +0b6
        jmp     0x518.l                         | +0ba
.L04e74a:
        rts                                     | +0c0

| ----------------------------------------------------------------------------
|  Prop_BarrierWreck_04e74c  @ $04E74C  (178 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_BarrierWreck_04e74c, "ax", @progbits
        .global Prop_BarrierWreck_04e74c
Prop_BarrierWreck_04e74c:
        move.l  #0x1000,d0                      | +000
        jsr     0x51a28.l                       | +006
        move.b  #0x0,0x10e39e.l                 | +00c
        lea     0x296102.l,a0                   | +014
        move.l  a0,0x4c(a6)                     | +01a
        jsr     0x283ca.l                       | +01e
        jsr     0x283ca.l                       | +024
        jsr     0x283d8.l                       | +02a
        jsr     0x2783a.l                       | +030
        lea     0x295dee.l,a1                   | +036
        jsr     0x43fac.l                       | +03c
        move.w  #0x1030,d0                      | +042
        jsr     0x2352.l                        | +046
        lea     0x2957d6.l,a2                   | +04c
        jsr     Sprite_InvokeBlit8Params(pc)    | +052
        lea     0x2957fe.l,a2                   | +056
        jsr     Sprite_InvokeBlit8Params(pc)    | +05c
        lea     Prop_Gatehouse_04e7fe(pc),a1    | +060
        jsr     0x4ae.l                         | +064
        jsr     0x5dd22.l                       | +06a
        addi.w  #0x40,0x22(a0)                  | +070
        lea     0x2944da.l,a0                   | +076
        jsr     0x28cd4.l                       | +07c
        move.b  #0xff,0x75(a6)                  | +082
        lea     0xffff.w,a0                     | +088
        move.l  a0,0x48(a6)                     | +08c
        lea     .L04e7e2(pc),a1                 | +090
        move.l  a1,(a6)                         | +094
.L04e7e2:
        jsr     0x2783a.l                       | +096
        jsr     0x28d70.l                       | +09c
        jsr     Prop_OffscreenLeftCheck_04fa70(pc)                | +0a2
        bcc.w   .L04e7fc                        | +0a6
        jmp     0x518.l                         | +0aa
.L04e7fc:
        rts                                     | +0b0

| ----------------------------------------------------------------------------
|  Prop_Gatehouse_04e7fe  @ $04E7FE  (410 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_Gatehouse_04e7fe, "ax", @progbits
        .global Prop_Gatehouse_04e7fe
Prop_Gatehouse_04e7fe:
        jsr     0x2783a.l                       | +000
        lea     Prop_GatehouseBlocker_04eae8(pc),a1 | +006
        jsr     0x4ae.l                         | +00a
        jsr     0x5dd22.l                       | +010
        addi.w  #0x34,0x22(a0)                  | +016
        lea     .L04e820(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L04e820:
        jsr     0x2783a.l                       | +022
        move.l  0x106f50.l,d0                   | +028
        swap    d0                              | +02e
        cmpi.w  #0xa20,d0                       | +030
        bgt.w   .L04e838                        | +034
        rts                                     | +038
.L04e838:
        move.w  #0x3a0,d0                       | +03a
        move.w  #0xe0,d1                        | +03e
        jsr     0x44022.l                       | +042
        move.w  d0,0x22(a6)                     | +048
        move.w  d1,0x24(a6)                     | +04c
        move.b  #0x0,0x21(a6)                   | +050
        move.b  #0x0,0x20(a6)                   | +056
        lea     Debris_Bouncer_04f5be(pc),a1    | +05c
        jsr     0x4ae.l                         | +060
        jsr     0x5dd22.l                       | +066
        addi.w  #0x58,0x22(a0)                  | +06c
        addi.w  #0x15,0x24(a0)                  | +072
        move.w  #0x40,d1                        | +078
        jsr     0x236e.l                        | +07c
        move.w  #0x70,0x70(a6)                  | +082
        move.w  #0xb4,0x66(a6)                  | +088
        bset    #0x6,0x12(a6)                   | +08e
        move.w  #0xf000,0x38(a6)                | +094
        jsr     Gatehouse_SpawnPiece01_04fd8a(pc)                | +09a
        jsr     Gatehouse_SpawnPiece02_04fdde(pc)                | +09e
        jsr     Gatehouse_SpawnPiece04_04fe32(pc)                | +0a2
        jsr     Gatehouse_SpawnPiece08_04fe86(pc)                | +0a6
        jsr     Gatehouse_SpawnPiece10_04feda(pc)                | +0aa
        jsr     Gatehouse_SpawnPiece20_04ff2e(pc)                | +0ae
        jsr     Gatehouse_SpawnPiece40_04ff82(pc)                | +0b2
        jsr     Gatehouse_SpawnPiece80_04ffd6(pc)                | +0b6
        lea     Prop_GatehouseRoof_04ef0a(pc),a1 | +0ba
        jsr     0x4ae.l                         | +0be
        jsr     0x5dd22.l                       | +0c4
        lea     Prop_GatehouseDoor_04efe6(pc),a1 | +0ca
        jsr     0x4ae.l                         | +0ce
        jsr     0x5dd22.l                       | +0d4
        lea     0x2944fa.l,a0                   | +0da
        jsr     0x28cd4.l                       | +0e0
        lea     0xe93b0.l,a1                    | +0e6
        move.w  #0x75,d0                        | +0ec
        move.b  #0x0,0x75(a6)                   | +0f0
        jsr     0x4429e.l                       | +0f6
        jsr     0x2783a.l                       | +0fc
        lea     0x295f12.l,a1                   | +102
        jsr     0x43fac.l                       | +108
        lea     .L04e912(pc),a1                 | +10e
        move.l  a1,(a6)                         | +112
.L04e912:
        jsr     0x2783a.l                       | +114
        jsr     0x28d70.l                       | +11a
        jsr     0x2870a.l                       | +120
        bcc.w   .L04e93a                        | +126
        lea     0x5e766.l,a0                    | +12a
        jsr     0x5e770.l                       | +130
        bclr    #0x3,0x13(a6)                   | +136
.L04e93a:
        jsr     0x28758.l                       | +13c
        bcc.w   .L04e94e                        | +142
        bclr    #0x0,0x13(a6)                   | +146
        bra.w   .L04e958                        | +14c
.L04e94e:
        cmpi.b  #0xff,0x21(a6)                  | +150
        bne.w   .L04e988                        | +156
.L04e958:
        move.b  #0xff,0x20(a6)                  | +15a
        lea     0x295c32.l,a1                   | +160
        jsr     0x77c7e.l                       | +166
        lea     0x295c44.l,a1                   | +16c
        jsr     0x77c7e.l                       | +172
        lea     0x295cd4.l,a1                   | +178
        jsr     0x77c7e.l                       | +17e
        lea     Prop_GatehouseDamaged_04e998(pc),a1 | +184
        move.l  a1,(a6)                         | +188
.L04e988:
        jsr     Prop_OffscreenLeftCheck_04fa70(pc)                | +18a
        bcc.w   .L04e996                        | +18e
        jmp     0x518.l                         | +192
.L04e996:
        rts                                     | +198

| ----------------------------------------------------------------------------
|  Prop_GatehouseDamaged_04e998  @ $04E998  (166 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_GatehouseDamaged_04e998, "ax", @progbits
        .global Prop_GatehouseDamaged_04e998
Prop_GatehouseDamaged_04e998:
        move.w  #0x102f,d0                      | +000
        jsr     0x2352.l                        | +004
        move.w  #0xb4,0x66(a6)                  | +00a
        bclr    #0x0,0x13(a6)                   | +010
        jsr     Gatehouse_BlitDamaged_0501d8(pc)                | +016
        lea     0x29451c.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        lea     .L04e9c4(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L04e9c4:
        jsr     0x2783a.l                       | +02c
        jsr     0x28d70.l                       | +032
        jsr     0x2870a.l                       | +038
        bcc.w   .L04e9ec                        | +03e
        lea     0x5e766.l,a0                    | +042
        jsr     0x5e770.l                       | +048
        bclr    #0x3,0x13(a6)                   | +04e
.L04e9ec:
        jsr     0x28758.l                       | +054
        bcc.w   .L04ea2e                        | +05a
        lea     0x295c32.l,a1                   | +05e
        jsr     0x77c7e.l                       | +064
        lea     0x295c44.l,a1                   | +06a
        jsr     0x77c7e.l                       | +070
        lea     0x295cd4.l,a1                   | +076
        jsr     0x77c7e.l                       | +07c
        move.b  #0x88,0x20(a6)                  | +082
        move.b  #0x1,0x10e39e.l                 | +088
        lea     Prop_GatehouseWreck_04ea3e(pc),a1 | +090
        move.l  a1,(a6)                         | +094
.L04ea2e:
        jsr     Prop_OffscreenLeftCheck_04fa70(pc)                | +096
        bcc.w   .L04ea3c                        | +09a
        jmp     0x518.l                         | +09e
.L04ea3c:
        rts                                     | +0a4

| ----------------------------------------------------------------------------
|  Prop_GatehouseWreck_04ea3e  @ $04EA3E  (170 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_GatehouseWreck_04ea3e, "ax", @progbits
        .global Prop_GatehouseWreck_04ea3e
Prop_GatehouseWreck_04ea3e:
        move.l  #0x2000,d0                      | +000
        jsr     0x51a28.l                       | +006
        move.b  #0x2,0x10e39a.l                 | +00c
        move.b  #0x0,0x10e39e.l                 | +014
        jsr     0x2783a.l                       | +01c
        lea     0x295e04.l,a1                   | +022
        jsr     0x43fac.l                       | +028
        lea     0xffff.w,a0                     | +02e
        move.l  a0,0x48(a6)                     | +032
        move.w  #0x1030,d0                      | +036
        jsr     0x2352.l                        | +03a
        lea     0x295f24.l,a1                   | +040
        jsr     0x43fac.l                       | +046
        lea     0x295812.l,a2                   | +04c
        jsr     Sprite_InvokeBlit8Params(pc)    | +052
        lea     0x2958da.l,a2                   | +056
        jsr     Sprite_InvokeBlit8Params(pc)    | +05c
        lea     Prop_Fortress_04eb18(pc),a1     | +060
        jsr     0x4ae.l                         | +064
        jsr     0x5dd22.l                       | +06a
        addi.w  #0xa0,0x22(a0)                  | +070
        lea     0x29453e.l,a0                   | +076
        jsr     0x28cd4.l                       | +07c
        move.b  #0xff,0x75(a6)                  | +082
        lea     .L04eacc(pc),a1                 | +088
        move.l  a1,(a6)                         | +08c
.L04eacc:
        jsr     0x2783a.l                       | +08e
        jsr     0x28d70.l                       | +094
        jsr     Prop_OffscreenLeftCheck_04fa70(pc)                | +09a
        bcc.w   .L04eae6                        | +09e
        jmp     0x518.l                         | +0a2
.L04eae6:
        rts                                     | +0a8

| ----------------------------------------------------------------------------
|  Prop_GatehouseBlocker_04eae8  @ $04EAE8  (48 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_GatehouseBlocker_04eae8, "ax", @progbits
        .global Prop_GatehouseBlocker_04eae8
Prop_GatehouseBlocker_04eae8:
        move.l  #0x2962be,0x60(a6)              | +000
        lea     .L04eaf6(pc),a1                 | +008
        move.l  a1,(a6)                         | +00c
.L04eaf6:
        jsr     0x28998.l                       | +00e
        jsr     0x2783a.l                       | +014
        movea.l 0xc(a6),a0                      | +01a
        cmpi.b  #0x88,0x20(a0)                  | +01e
        bne.w   .L04eb16                        | +024
        jmp     0x518.l                         | +028
.L04eb16:
        rts                                     | +02e

| ----------------------------------------------------------------------------
|  Prop_Fortress_04eb18  @ $04EB18  (116 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_Fortress_04eb18, "ax", @progbits
        .global Prop_Fortress_04eb18
Prop_Fortress_04eb18:
        move.w  #0x440,d0                       | +000
        move.w  #0xe0,d1                        | +004
        jsr     0x44022.l                       | +008
        move.w  d0,0x22(a6)                     | +00e
        move.w  d1,0x24(a6)                     | +012
        move.b  #0x0,0x21(a6)                   | +016
        move.b  #0x0,0x20(a6)                   | +01c
        lea     Prop_FortressTurretMount_04f030(pc),a1 | +022
        jsr     0x4ae.l                         | +026
        jsr     0x5dd22.l                       | +02c
        subi.w  #0x10,0x22(a0)                  | +032
        addi.w  #0x51,0x24(a0)                  | +038
        move.w  #0x40,d1                        | +03e
        jsr     0x236e.l                        | +042
        move.w  #0x0,0x72(a6)                   | +048
        move.w  0x72(a6),d0                     | +04e
        andi.l  #0x3,d0                         | +052
        movea.l #0x2945c4,a0                    | +058
        lsl.w   #0x2,d0                         | +05e
        movea.l (a0,d0.w),a0                    | +060
        cmpa.l  #0xffffffff,a0                  | +064
        beq.w   SetTaskHandler_04eb8c           | +06a
        jsr     0x28cd4.l                       | +06e

| ----------------------------------------------------------------------------
|  Prop_FortressWaitScroll_04eb94  @ $04EB94  (32 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_FortressWaitScroll_04eb94, "ax", @progbits
        .global Prop_FortressWaitScroll_04eb94
Prop_FortressWaitScroll_04eb94:
        cmpi.w  #0x130,0x22(a6)                 | +000
        ble.w   Prop_FortressActive_04ebbc      | +006
        clr.b   0x10e39a.l                      | +00a
        jsr     0x2783a.l                       | +010
        cmpi.w  #0xd0,0x22(a6)                  | +016
        bgt.w   JsrAbsRts_04ebba                | +01c

| ----------------------------------------------------------------------------
|  Prop_FortressActive_04ebbc  @ $04EBBC  (324 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_FortressActive_04ebbc, "ax", @progbits
        .global Prop_FortressActive_04ebbc
Prop_FortressActive_04ebbc:
        move.w  #0x120,0x70(a6)                 | +000
        lea     0x2c0010.l,a0                   | +006
        jsr     0x799de.l                       | +00c
        move.w  d0,0x66(a6)                     | +012
        bset    #0x6,0x12(a6)                   | +016
        move.w  #0xf000,0x38(a6)                | +01c
        jsr     Fortress_SpawnPiece01_05002a(pc)                | +022
        jsr     Fortress_SpawnPiece02_05007e(pc)                | +026
        jsr     Fortress_SpawnPiece04_0500d2(pc)                | +02a
        jsr     Fortress_SpawnPiece08_050126(pc)                | +02e
        jsr     Fortress_SpawnPiece10_05017a(pc)                | +032
        lea     0xe9442.l,a1                    | +036
        move.w  #0x75,d0                        | +03c
        move.b  #0x0,0x75(a6)                   | +040
        jsr     0x4429e.l                       | +046
        lea     Prop_FortressBlocker_04edda(pc),a1 | +04c
        jsr     0x4ae.l                         | +050
        jsr     0x5dd22.l                       | +056
        addi.w  #0x78,0x22(a0)                  | +05c
        lea     .L04ec24(pc),a1                 | +062
        move.l  a1,(a6)                         | +066
.L04ec24:
        clr.b   0x10e39a.l                      | +068
        jsr     0x2783a.l                       | +06e
        jsr     0x28d70.l                       | +074
        jsr     0x2870a.l                       | +07a
        bcc.w   .L04ec52                        | +080
        lea     0x5e766.l,a0                    | +084
        jsr     0x5e770.l                       | +08a
        bclr    #0x3,0x13(a6)                   | +090
.L04ec52:
        jsr     0x28758.l                       | +096
        bcc.w   .L04ecb0                        | +09c
        bclr    #0x0,0x13(a6)                   | +0a0
        addq.w  #0x1,0x72(a6)                   | +0a6
        move.w  0x72(a6),d0                     | +0aa
        andi.l  #0x3,d0                         | +0ae
        movea.l #0x2945c4,a0                    | +0b4
        lsl.w   #0x2,d0                         | +0ba
        movea.l (a0,d0.w),a0                    | +0bc
        cmpa.l  #0xffffffff,a0                  | +0c0
        beq.w   .L04ec8c                        | +0c6
        jsr     0x28cd4.l                       | +0ca
.L04ec8c:
        lea     0x2c0010.l,a0                   | +0d0
        jsr     0x799de.l                       | +0d6
        move.w  d0,0x66(a6)                     | +0dc
        cmpi.w  #0x3,0x72(a6)                   | +0e0
        blt.w   .L04ecb0                        | +0e6
        move.b  #0xff,0x20(a6)                  | +0ea
        bra.w   .L04ecc0                        | +0f0
.L04ecb0:
        cmpi.b  #0x1f,0x21(a6)                  | +0f4
        bne.w   .L04ecf0                        | +0fa
        move.b  #0xff,0x20(a6)                  | +0fe
.L04ecc0:
        lea     0x295c32.l,a1                   | +104
        jsr     0x77c7e.l                       | +10a
        lea     0x295c44.l,a1                   | +110
        jsr     0x77c7e.l                       | +116
        lea     0x295cd4.l,a1                   | +11c
        jsr     0x77c7e.l                       | +122
        jsr     0x434dc.l                       | +128
        lea     Prop_FortressWreck_04ed00(pc),a1 | +12e
        move.l  a1,(a6)                         | +132
.L04ecf0:
        jsr     Prop_OffscreenLeftCheck_04fa70(pc)                | +134
        bcc.w   .L04ecfe                        | +138
        jmp     0x518.l                         | +13c
.L04ecfe:
        rts                                     | +142

| ----------------------------------------------------------------------------
|  Prop_FortressWreck_04ed00  @ $04ED00  (144 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_FortressWreck_04ed00, "ax", @progbits
        .global Prop_FortressWreck_04ed00
Prop_FortressWreck_04ed00:
        move.l  #0x5000,d0                      | +000
        jsr     0x51a28.l                       | +006
        lea     0x86586.l,a1                    | +00c
        jsr     0x4ae.l                         | +012
        jsr     0x2783a.l                       | +018
        lea     0x295e14.l,a1                   | +01e
        jsr     0x43fac.l                       | +024
        lea     0x295e38.l,a1                   | +02a
        jsr     0x43fac.l                       | +030
        lea     0xffff.w,a0                     | +036
        move.l  a0,0x48(a6)                     | +03a
        move.w  #0x1023,d0                      | +03e
        jsr     0x2352.l                        | +042
        lea     0x295a42.l,a2                   | +048
        jsr     Sprite_InvokeBlit8Params(pc)    | +04e
        lea     0x295a56.l,a2                   | +052
        jsr     Sprite_InvokeBlit8Params(pc)    | +058
        lea     0x2945b4.l,a0                   | +05c
        jsr     0x28cd4.l                       | +062
        move.b  #0xff,0x75(a6)                  | +068
        lea     .L04ed74(pc),a1                 | +06e
        move.l  a1,(a6)                         | +072
.L04ed74:
        jsr     0x2783a.l                       | +074
        jsr     0x28d70.l                       | +07a
        jsr     Prop_OffscreenLeftCheck_04fa70(pc)                | +080
        bcc.w   .L04ed8e                        | +084
        jmp     0x518.l                         | +088
.L04ed8e:
        rts                                     | +08e

| ----------------------------------------------------------------------------
|  Prop_BarrierPost_04ed90  @ $04ED90  (74 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_BarrierPost_04ed90, "ax", @progbits
        .global Prop_BarrierPost_04ed90
Prop_BarrierPost_04ed90:
        move.w  #0x3f,d1                        | +000
        jsr     0x236e.l                        | +004
        jsr     0x267e2.l                       | +00a
        move.w  #0x0,0x38(a6)                   | +010
        lea     0x2944ea.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
        lea     .L04edb8(pc),a1                 | +022
        move.l  a1,(a6)                         | +026
.L04edb8:
        jsr     0x2783a.l                       | +028
        jsr     0x28d70.l                       | +02e
        movea.l 0xc(a6),a0                      | +034
        cmpi.b  #0x88,0x20(a0)                  | +038
        bne.w   .L04edd8                        | +03e
        jmp     0x518.l                         | +042
.L04edd8:
        rts                                     | +048

| ----------------------------------------------------------------------------
|  Prop_FortressBlocker_04edda  @ $04EDDA  (48 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_FortressBlocker_04edda, "ax", @progbits
        .global Prop_FortressBlocker_04edda
Prop_FortressBlocker_04edda:
        move.l  #0x29630a,0x60(a6)              | +000
        lea     .L04ede8(pc),a1                 | +008
        move.l  a1,(a6)                         | +00c
.L04ede8:
        jsr     0x28998.l                       | +00e
        jsr     0x2783a.l                       | +014
        movea.l 0xc(a6),a0                      | +01a
        cmpi.b  #0xff,0x20(a0)                  | +01e
        bne.w   .L04ee08                        | +024
        jmp     0x518.l                         | +028
.L04ee08:
        rts                                     | +02e

| ----------------------------------------------------------------------------
|  Prop_BarrierLight_04ee0a  @ $04EE0A  (192 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_BarrierLight_04ee0a, "ax", @progbits
        .global Prop_BarrierLight_04ee0a
Prop_BarrierLight_04ee0a:
        move.w  #0x80,0x70(a6)                  | +000
        move.w  #0x3c,0x66(a6)                  | +006
        lea     0x294620.l,a0                   | +00c
        jsr     0x28cd4.l                       | +012
        bset    #0x6,0x12(a6)                   | +018
        lea     .L04ee2e(pc),a1                 | +01e
        move.l  a1,(a6)                         | +022
.L04ee2e:
        jsr     0x2783a.l                       | +024
        jsr     0x28d70.l                       | +02a
        movea.l 0xc(a6),a0                      | +030
        cmpi.b  #0x88,0x20(a0)                  | +034
        bne.w   .L04ee56                        | +03a
        lea     0x295b46.l,a2                   | +03e
        jsr     Sprite_InvokeBlit8Params(pc)    | +044
        bra.w   .L04eec2                        | +048
.L04ee56:
        cmpi.b  #0x5,0x58(a6)                   | +04c
        beq.w   .L04ee86                        | +052
        jsr     0x2870a.l                       | +056
        bcc.w   .L04ee7c                        | +05c
        lea     0x5e766.l,a0                    | +060
        jsr     0x5e770.l                       | +066
        bclr    #0x3,0x13(a6)                   | +06c
.L04ee7c:
        jsr     0x28758.l                       | +072
        bcc.w   .L04eeba                        | +078
.L04ee86:
        bclr    #0x3,0x13(a6)                   | +07c
        lea     0x295ae2.l,a2                   | +082
        jsr     Sprite_InvokeBlit8Params(pc)    | +088
        lea     0x295b0a.l,a2                   | +08c
        jsr     Sprite_InvokeBlit8Params(pc)    | +092
        lea     0x29584e.l,a2                   | +096
        jsr     Sprite_InvokeBlit8Params(pc)    | +09c
        lea     0x2958c6.l,a2                   | +0a0
        jsr     Sprite_InvokeBlit8Params(pc)    | +0a6
        lea     Prop_BarrierLightWreck_04eeca(pc),a1 | +0aa
        move.l  a1,(a6)                         | +0ae
.L04eeba:
        jsr     Prop_OffscreenLeftCheck_04fa70(pc)                | +0b0
        bcc.w   .L04eec8                        | +0b4
.L04eec2:
        jmp     0x518.l                         | +0b8
.L04eec8:
        rts                                     | +0be

| ----------------------------------------------------------------------------
|  Prop_BarrierLightWreck_04eeca  @ $04EECA  (64 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_BarrierLightWreck_04eeca, "ax", @progbits
        .global Prop_BarrierLightWreck_04eeca
Prop_BarrierLightWreck_04eeca:
        lea     0xffff.w,a0                     | +000
        move.l  a0,0x48(a6)                     | +004
        lea     .L04eed8(pc),a1                 | +008
        move.l  a1,(a6)                         | +00c
.L04eed8:
        jsr     0x2783a.l                       | +00e
        movea.l 0xc(a6),a0                      | +014
        cmpi.b  #0x88,0x20(a0)                  | +018
        bne.w   .L04eefa                        | +01e
        lea     0x295b46.l,a2                   | +022
        jsr     Sprite_InvokeBlit8Params(pc)    | +028
        bra.w   .L04ef02                        | +02c
.L04eefa:
        jsr     Prop_OffscreenLeftCheck_04fa70(pc)                | +030
        bcc.w   .L04ef08                        | +034
.L04ef02:
        jmp     0x518.l                         | +038
.L04ef08:
        rts                                     | +03e

| ----------------------------------------------------------------------------
|  Prop_GatehouseRoof_04ef0a  @ $04EF0A  (152 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_GatehouseRoof_04ef0a, "ax", @progbits
        .global Prop_GatehouseRoof_04ef0a
Prop_GatehouseRoof_04ef0a:
        move.w  #0x3f,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x40,0x70(a6)                  | +00a
        move.w  #0x78,0x66(a6)                  | +010
        lea     0x294636.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
        move.w  #0xbfff,0x38(a6)                | +022
        lea     .L04ef38(pc),a1                 | +028
        move.l  a1,(a6)                         | +02c
.L04ef38:
        jsr     0x2783a.l                       | +02e
        jsr     0x28d70.l                       | +034
        movea.l 0xc(a6),a0                      | +03a
        cmpi.b  #0xff,0x20(a0)                  | +03e
        bne.w   .L04ef56                        | +044
        bra.w   .L04ef82                        | +048
.L04ef56:
        jsr     0x2870a.l                       | +04c
        bcc.w   .L04ef72                        | +052
        lea     0x5e766.l,a0                    | +056
        jsr     0x5e770.l                       | +05c
        bclr    #0x3,0x13(a6)                   | +062
.L04ef72:
        jsr     0x28758.l                       | +068
        bcc.w   .L04ef92                        | +06e
        bclr    #0x0,0x13(a6)                   | +072
.L04ef82:
        lea     0x295a2e.l,a2                   | +078
        jsr     Sprite_InvokeBlit8Params(pc)    | +07e
        lea     Prop_GatehouseRoofWreck_04efa2(pc),a1 | +082
        move.l  a1,(a6)                         | +086
.L04ef92:
        jsr     Prop_OffscreenLeftCheck_04fa70(pc)                | +088
        bcc.w   .L04efa0                        | +08c
        jmp     0x518.l                         | +090
.L04efa0:
        rts                                     | +096

| ----------------------------------------------------------------------------
|  Prop_GatehouseRoofWreck_04efa2  @ $04EFA2  (68 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_GatehouseRoofWreck_04efa2, "ax", @progbits
        .global Prop_GatehouseRoofWreck_04efa2
Prop_GatehouseRoofWreck_04efa2:
        lea     0x29464c.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0xffff.w,a0                     | +00c
        move.l  a0,0x48(a6)                     | +010
        lea     .L04efbc(pc),a1                 | +014
        move.l  a1,(a6)                         | +018
.L04efbc:
        jsr     0x2783a.l                       | +01a
        jsr     0x28d70.l                       | +020
        movea.l 0xc(a6),a0                      | +026
        cmpi.b  #0x88,0x20(a0)                  | +02a
        beq.w   .L04efde                        | +030
        jsr     Prop_OffscreenLeftCheck_04fa70(pc)                | +034
        bcc.w   .L04efe4                        | +038
.L04efde:
        jmp     0x518.l                         | +03c
.L04efe4:
        rts                                     | +042

| ----------------------------------------------------------------------------
|  Prop_GatehouseDoor_04efe6  @ $04EFE6  (74 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_GatehouseDoor_04efe6, "ax", @progbits
        .global Prop_GatehouseDoor_04efe6
Prop_GatehouseDoor_04efe6:
        move.w  #0x40,d1                        | +000
        jsr     0x236e.l                        | +004
        lea     0x29465c.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        bset    #0x6,0x12(a6)                   | +016
        move.w  #0xbfff,0x38(a6)                | +01c
        lea     .L04f00e(pc),a1                 | +022
        move.l  a1,(a6)                         | +026
.L04f00e:
        jsr     0x2783a.l                       | +028
        jsr     0x28d70.l                       | +02e
        movea.l 0xc(a6),a0                      | +034
        cmpi.b  #0x88,0x20(a0)                  | +038
        bne.w   .L04f02e                        | +03e
        jmp     0x518.l                         | +042
.L04f02e:
        rts                                     | +048

| ----------------------------------------------------------------------------
|  Prop_FortressTurretMount_04f030  @ $04F030  (128 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_FortressTurretMount_04f030, "ax", @progbits
        .global Prop_FortressTurretMount_04f030
Prop_FortressTurretMount_04f030:
        move.w  #0x40,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x60,0x70(a6)                  | +00a
        lea     0x29466c.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        move.w  #0xbfff,0x38(a6)                | +01c
        lea     .L04f058(pc),a1                 | +022
        move.l  a1,(a6)                         | +026
.L04f058:
        jsr     0x2783a.l                       | +028
        move.l  0x106f50.l,d0                   | +02e
        swap    d0                              | +034
        cmpi.w  #0xa70,d0                       | +036
        blt.w   .L04f074                        | +03a
        jsr     0x28d70.l                       | +03e
.L04f074:
        movea.l 0xc(a6),a0                      | +044
        cmpi.b  #0xff,0x20(a0)                  | +048
        bne.w   .L04f0a0                        | +04e
        lea     0x295c32.l,a1                   | +052
        jsr     0x77c7e.l                       | +058
        lea     0x295cd4.l,a1                   | +05e
        jsr     0x77c7e.l                       | +064
        lea     Prop_FortressTurretMountFall_04f0b0(pc),a1 | +06a
        move.l  a1,(a6)                         | +06e
.L04f0a0:
        jsr     Prop_OffscreenLeftCheck_04fa70(pc)                | +070
        bcc.w   .L04f0ae                        | +074
        jmp     0x518.l                         | +078
.L04f0ae:
        rts                                     | +07e

| ----------------------------------------------------------------------------
|  Prop_FortressTurretMountFall_04f0b0  @ $04F0B0  (136 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_FortressTurretMountFall_04f0b0, "ax", @progbits
        .global Prop_FortressTurretMountFall_04f0b0
Prop_FortressTurretMountFall_04f0b0:
        lea     0x29467c.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        jsr     0x267e2.l                       | +00c
        move.w  #0xfff0,0x2e(a6)                | +012
        lea     .L04f0ce(pc),a1                 | +018
        move.l  a1,(a6)                         | +01c
.L04f0ce:
        jsr     0x27cee.l                       | +01e
        jsr     0x28d70.l                       | +024
        cmpi.w  #0x160,0x24(a6)                 | +02a
        bgt.w   .L04f128                        | +030
        move.w  #0x1030,d0                      | +034
        jsr     0x2352.l                        | +038
        subi.w  #0x20,0x24(a6)                  | +03e
        lea     0x295c32.l,a1                   | +044
        jsr     0x77c7e.l                       | +04a
        addi.w  #0x20,0x24(a0)                  | +050
        lea     0x295c44.l,a1                   | +056
        jsr     0x77c7e.l                       | +05c
        addi.w  #0x20,0x24(a0)                  | +062
        lea     0x295cd4.l,a1                   | +068
        jsr     0x77c7e.l                       | +06e
        bra.w   .L04f130                        | +074
.L04f128:
        jsr     Prop_OffscreenLeftCheck_04fa70(pc)                | +078
        bcc.w   .L04f136                        | +07c
.L04f130:
        jmp     0x518.l                         | +080
.L04f136:
        rts                                     | +086

| ----------------------------------------------------------------------------
|  Prop_FortressSide_04f138  @ $04F138  (188 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_FortressSide_04f138, "ax", @progbits
        .global Prop_FortressSide_04f138
Prop_FortressSide_04f138:
        move.w  #0x24,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x60,0x70(a6)                  | +00a
        addi.w  #0xa8,0x22(a6)                  | +010
        addi.w  #0x0,0x24(a6)                   | +016
        move.w  #0x0,0x38(a6)                   | +01c
        lea     .L04f160(pc),a1                 | +022
        move.l  a1,(a6)                         | +026
.L04f160:
        jsr     0x2783a.l                       | +028
        movea.l 0xc(a6),a0                      | +02e
        cmpi.b  #0xff,0x20(a0)                  | +032
        bne.w   .L04f1ae                        | +038
        lea     0x295f4a.l,a1                   | +03c
        jsr     0x43fac.l                       | +042
        subi.w  #0x50,0x22(a6)                  | +048
        lea     0x295b5a.l,a1                   | +04e
        jsr     0x77c7e.l                       | +054
        lea     0x295b6c.l,a1                   | +05a
        jsr     0x77c7e.l                       | +060
        lea     0x295cb0.l,a1                   | +066
        jsr     0x77c7e.l                       | +06c
        bra.w   .L04f1ec                        | +072
.L04f1ae:
        move.w  0x72(a0),d0                     | +076
        andi.l  #0x3,d0                         | +07a
        movea.l #0x294610,a0                    | +080
        lsl.w   #0x2,d0                         | +086
        movea.l (a0,d0.w),a0                    | +088
        cmpa.l  #0xffffffff,a0                  | +08c
        beq.w   .L04f1d4                        | +092
        jsr     0x28cd4.l                       | +096
.L04f1d4:
        cmpi.w  #0x190,0x22(a6)                 | +09c
        bgt.w   .L04f1e4                        | +0a2
        jsr     0x28d70.l                       | +0a6
.L04f1e4:
        jsr     Prop_OffscreenLeftCheck_04fa70(pc)                | +0ac
        bcc.w   .L04f1f2                        | +0b0
.L04f1ec:
        jmp     0x518.l                         | +0b4
.L04f1f2:
        rts                                     | +0ba

| ----------------------------------------------------------------------------
|  Prop_BlitSequence_04f1f4  @ $04F1F4  (176 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_BlitSequence_04f1f4, "ax", @progbits
        .global Prop_BlitSequence_04f1f4
Prop_BlitSequence_04f1f4:
        move.b  #0x0,0x20(a6)                   | +000
        move.w  #0x1e,0x66(a6)                  | +006
        lea     .L04f206(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L04f206:
        movea.l 0xc(a6),a0                      | +012
        cmpi.b  #0xff,0x20(a0)                  | +016
        beq.w   .L04f284                        | +01c
        jsr     0x2783a.l                       | +020
        jsr     0x28d70.l                       | +026
        jsr     0x2870a.l                       | +02c
        bcc.w   .L04f23c                        | +032
        lea     0x5e766.l,a0                    | +036
        jsr     0x5e770.l                       | +03c
        bclr    #0x3,0x13(a6)                   | +042
.L04f23c:
        jsr     0x28758.l                       | +048
        bcc.w   .L04f294                        | +04e
        move.w  #0x1e,0x66(a6)                  | +052
        bclr    #0x0,0x13(a6)                   | +058
        move.b  0x20(a6),d0                     | +05e
        andi.l  #0xff,d0                        | +062
        asl.l   #0x2,d0                         | +068
        movea.l 0x70(a6,d0.w),a2                | +06a
        jsr     Sprite_InvokeBlit8Params(pc)    | +06e
        addq.b  #0x1,0x20(a6)                   | +072
        move.b  0x20(a6),d0                     | +076
        andi.l  #0xff,d0                        | +07a
        asl.l   #0x2,d0                         | +080
        move.l  0x70(a6,d0.w),d0                | +082
        cmpi.l  #0xffffffff,d0                  | +086
        bne.w   .L04f294                        | +08c
.L04f284:
        move.b  0x21(a6),d0                     | +090
        movea.l 0xc(a6),a0                      | +094
        or.b    d0,0x21(a0)                     | +098
        bra.w   .L04f29c                        | +09c
.L04f294:
        jsr     Prop_OffscreenLeftCheck_04fa70(pc)                | +0a0
        bcc.w   .L04f2a2                        | +0a4
.L04f29c:
        jmp     0x518.l                         | +0a8
.L04f2a2:
        rts                                     | +0ae

| ----------------------------------------------------------------------------
|  Prop_SlotPrioCheckRts_04f2a4  @ $04F2A4  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_SlotPrioCheckRts_04f2a4, "ax", @progbits
        .global Prop_SlotPrioCheckRts_04f2a4
Prop_SlotPrioCheckRts_04f2a4:
        jsr     0x2783a.l                       | +000
        movea.l 0x8(a6),a1                      | +006
        move.b  0x10(a6),d0                     | +00a
        cmp.b   0x10(a1),d0                     | +00e
        bcs.w   .L04f2c0                        | +012
        jmp     0x518.l                         | +016
.L04f2c0:
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  Prop_Roof_04f2c2  @ $04F2C2  (130 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_Roof_04f2c2, "ax", @progbits
        .global Prop_Roof_04f2c2
Prop_Roof_04f2c2:
        move.w  #0x3d,d1                        | +000
        jsr     0x236e.l                        | +004
        move.b  #0xff,0x32(a6)                  | +00a
        move.b  #0xff,0x33(a6)                  | +010
        move.w  #0x28,0x70(a6)                  | +016
        move.b  #0x0,0x3a(a6)                   | +01c
        move.w  #0x14,0x66(a6)                  | +022
        lea     0x294254.l,a0                   | +028
        jsr     0x28cd4.l                       | +02e
        lea     .L04f2fc(pc),a1                 | +034
        move.l  a1,(a6)                         | +038
.L04f2fc:
        jsr     0x2783a.l                       | +03a
        jsr     0x28d70.l                       | +040
        jsr     0x2870a.l                       | +046
        bcc.w   .L04f324                        | +04c
        lea     0x5e766.l,a0                    | +050
        jsr     0x5e770.l                       | +056
        bclr    #0x3,0x13(a6)                   | +05c
.L04f324:
        jsr     0x28758.l                       | +062
        bcc.w   .L04f334                        | +068
        lea     Prop_RoofFall_04f344(pc),a1     | +06c
        move.l  a1,(a6)                         | +070
.L04f334:
        jsr     Prop_OffscreenLeftCheck_04fa70(pc)                | +072
        bcc.w   .L04f342                        | +076
        jmp     0x518.l                         | +07a
.L04f342:
        rts                                     | +080

| ----------------------------------------------------------------------------
|  Prop_RoofFall_04f344  @ $04F344  (98 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_RoofFall_04f344, "ax", @progbits
        .global Prop_RoofFall_04f344
Prop_RoofFall_04f344:
        lea     0x295fba.l,a0                   | +000
        move.l  a0,0x4c(a6)                     | +006
        jsr     0x283ca.l                       | +00a
        jsr     0x283ca.l                       | +010
        jsr     0x283d8.l                       | +016
        lea     0x29426a.l,a0                   | +01c
        jsr     0x28cd4.l                       | +022
        lea     .L04f372(pc),a1                 | +028
        move.l  a1,(a6)                         | +02c
.L04f372:
        jsr     0x2783a.l                       | +02e
        jsr     0x28d70.l                       | +034
        bcc.w   SetHandlerRts_04f3ac            | +03a
        lea     0xffff.w,a0                     | +03e
        move.l  a0,0x4c(a6)                     | +042
        jsr     0x283ca.l                       | +046
        move.w  #0x1028,d0                      | +04c
        jsr     0x2352.l                        | +050
        lea     0x295bd8.l,a1                   | +056
        jsr     0x77c7e.l                       | +05c

| ----------------------------------------------------------------------------
|  FixBlink3_PhaseA_04f3ae  @ $04F3AE  (94 B)
| ----------------------------------------------------------------------------
        .section .text.FixBlink3_PhaseA_04f3ae, "ax", @progbits
        .global FixBlink3_PhaseA_04f3ae
FixBlink3_PhaseA_04f3ae:
        move.w  #0x1e,d1                        | +000
        move.w  #0x111,d2                       | +004
        move.w  #0xffff,d3                      | +008
        move.w  #0x1,d4                         | +00c
        jsr     0x2c26.l                        | +010
        move.w  #0x1,0x72(a6)                   | +016
        lea     .L04f3d0(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L04f3d0:
        subq.w  #0x1,0x72(a6)                   | +022
        bne.w   .L04f3de                        | +026
        lea     FixBlink3_PhaseB_04f40c(pc),a1  | +02a
        move.l  a1,(a6)                         | +02e
.L04f3de:
        move.l  0x106f5c.l,d0                   | +030
        swap    d0                              | +036
        cmpi.w  #0x280,d0                       | +038
        blt.w   .L04f40a                        | +03c
        move.w  #0x1e,d1                        | +040
        move.w  #0xce,d2                        | +044
        move.w  #0xffff,d3                      | +048
        move.w  #0x1,d4                         | +04c
        jsr     0x2c26.l                        | +050
        jmp     0x518.l                         | +056
.L04f40a:
        rts                                     | +05c

| ----------------------------------------------------------------------------
|  FixBlink3_PhaseB_04f40c  @ $04F40C  (94 B)
| ----------------------------------------------------------------------------
        .section .text.FixBlink3_PhaseB_04f40c, "ax", @progbits
        .global FixBlink3_PhaseB_04f40c
FixBlink3_PhaseB_04f40c:
        move.w  #0x1e,d1                        | +000
        move.w  #0xce,d2                        | +004
        move.w  #0xffff,d3                      | +008
        move.w  #0x1,d4                         | +00c
        jsr     0x2c26.l                        | +010
        move.w  #0x1,0x72(a6)                   | +016
        lea     .L04f42e(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L04f42e:
        subq.w  #0x1,0x72(a6)                   | +022
        bne.w   .L04f43c                        | +026
        lea     FixBlink3_PhaseA_04f3ae(pc),a1  | +02a
        move.l  a1,(a6)                         | +02e
.L04f43c:
        move.l  0x106f5c.l,d0                   | +030
        swap    d0                              | +036
        cmpi.w  #0x280,d0                       | +038
        blt.w   .L04f468                        | +03c
        move.w  #0x1e,d1                        | +040
        move.w  #0xce,d2                        | +044
        move.w  #0xffff,d3                      | +048
        move.w  #0x1,d4                         | +04c
        jsr     0x2c26.l                        | +050
        jmp     0x518.l                         | +056
.L04f468:
        rts                                     | +05c

| ----------------------------------------------------------------------------
|  Prop_Sensor_04f46a  @ $04F46A  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_Sensor_04f46a, "ax", @progbits
        .global Prop_Sensor_04f46a
Prop_Sensor_04f46a:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x90,0x70(a6)                  | +00a

| ----------------------------------------------------------------------------
|  Prop_SensorWait_04f47a  @ $04F47A  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_SensorWait_04f47a, "ax", @progbits
        .global Prop_SensorWait_04f47a
Prop_SensorWait_04f47a:
        lea     .L04f480(pc),a1                 | +000
        move.l  a1,(a6)                         | +004
.L04f480:
        jsr     0x2783a.l                       | +006
        lea     0x295fb0.l,a0                   | +00c
        jsr     0x5e086.l                       | +012
        bcs.w   .L04f4a6                        | +018
        jsr     0x31fc2.l                       | +01c
        beq.w   .L04f4a6                        | +022
        lea     Prop_SensorTriggered_04f4b6(pc),a1 | +026
        move.l  a1,(a6)                         | +02a
.L04f4a6:
        jsr     Prop_OffscreenLeftCheck_04fa70(pc)                | +02c
        bcc.w   .L04f4b4                        | +030
        jmp     0x518.l                         | +034
.L04f4b4:
        rts                                     | +03a

| ----------------------------------------------------------------------------
|  Prop_SensorTriggered_04f4b6  @ $04F4B6  (56 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_SensorTriggered_04f4b6, "ax", @progbits
        .global Prop_SensorTriggered_04f4b6
Prop_SensorTriggered_04f4b6:
        lea     0x2948a6.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L04f4c8(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L04f4c8:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        bcc.w   .L04f4de                        | +01e
        lea     Prop_SensorWait_04f47a(pc),a1   | +022
        move.l  a1,(a6)                         | +026
.L04f4de:
        jsr     Prop_OffscreenLeftCheck_04fa70(pc)                | +028
        bcc.w   .L04f4ec                        | +02c
        jmp     0x518.l                         | +030
.L04f4ec:
        rts                                     | +036

| ----------------------------------------------------------------------------
|  Prop_Crate_04f4ee  @ $04F4EE  (200 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_Crate_04f4ee, "ax", @progbits
        .global Prop_Crate_04f4ee
Prop_Crate_04f4ee:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x20,0x70(a6)                  | +00a
        move.b  #0xff,0x32(a6)                  | +010
        move.b  #0xff,0x33(a6)                  | +016
        move.b  #0x0,0x3a(a6)                   | +01c
        move.w  #0x14,0x66(a6)                  | +022
        lea     0x295160.l,a0                   | +028
        move.l  a0,0x48(a6)                     | +02e
        lea     .L04f526(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L04f526:
        jsr     0x2783a.l                       | +038
        jsr     0x2870a.l                       | +03e
        bcc.w   .L04f548                        | +044
        lea     0x5e766.l,a0                    | +048
        jsr     0x5e770.l                       | +04e
        bclr    #0x3,0x13(a6)                   | +054
.L04f548:
        jsr     0x28758.l                       | +05a
        bcc.w   .L04f5a6                        | +060
        lea     0x78066.l,a1                    | +064
        jsr     0x4ae.l                         | +06a
        jsr     0x5dd22.l                       | +070
        addi.w  #0x28,0x22(a0)                  | +076
        lea     0x2957ae.l,a2                   | +07c
        jsr     Sprite_InvokeBlit8Params(pc)    | +082
        lea     0x295f5e.l,a1                   | +086
        jsr     0x43fac.l                       | +08c
        lea     0x29605e.l,a0                   | +092
        move.l  a0,0x4c(a6)                     | +098
        jsr     0x283ca.l                       | +09c
        jsr     0x283ca.l                       | +0a2
        jsr     0x283d8.l                       | +0a8
        lea     JmpToScheduler_04f5b6(pc),a1    | +0ae
        move.l  a1,(a6)                         | +0b2
        bra.w   .L04f5b4                        | +0b4
.L04f5a6:
        jsr     Prop_OffscreenLeftCheck_04fa70(pc)                | +0b8
        bcc.w   .L04f5b4                        | +0bc
        jmp     0x518.l                         | +0c0
.L04f5b4:
        rts                                     | +0c6

| ----------------------------------------------------------------------------
|  Debris_Bouncer_04f5be  @ $04F5BE  (142 B)
| ----------------------------------------------------------------------------
        .section .text.Debris_Bouncer_04f5be, "ax", @progbits
        .global Debris_Bouncer_04f5be
Debris_Bouncer_04f5be:
        jsr     0x5e7c0.l                       | +000
        move.w  #0x126,d1                       | +006
        jsr     0x236e.l                        | +00a
        move.b  #0xff,0x32(a6)                  | +010
        move.b  #0xff,0x33(a6)                  | +016
        move.b  #0x0,0x3a(a6)                   | +01c
        jsr     0x267e2.l                       | +022
        move.w  #0x0,0x38(a6)                   | +028
        lea     0x2948f8.l,a0                   | +02e
        jsr     0x28cd4.l                       | +034
        lea     .L04f5fe(pc),a1                 | +03a
        move.l  a1,(a6)                         | +03e
.L04f5fe:
        jsr     0x2783a.l                       | +040
        jsr     0x28d70.l                       | +046
        jsr     0x27eba.l                       | +04c
        bcc.w   .L04f61a                        | +052
        lea     Debris_BouncerHop_04f64c(pc),a1 | +056
        move.l  a1,(a6)                         | +05a
.L04f61a:
        jsr     0x5e9b6.l                       | +05c
        andi.w  #0x7,d0                         | +062
        bne.w   .L04f634                        | +066
        lea     0x294936.l,a0                   | +06a
        jsr     0x28cd4.l                       | +070
.L04f634:
        lea     0x29624a.l,a0                   | +076
        jsr     0x5dd5c.l                       | +07c
        bcc.w   .L04f64a                        | +082
        jmp     0x518.l                         | +086
.L04f64a:
        rts                                     | +08c

| ----------------------------------------------------------------------------
|  Debris_BouncerHop_04f64c  @ $04F64C  (70 B)
| ----------------------------------------------------------------------------
        .section .text.Debris_BouncerHop_04f64c, "ax", @progbits
        .global Debris_BouncerHop_04f64c
Debris_BouncerHop_04f64c:
        move.w  #0x400,0x2a(a6)                 | +000
        lea     0x29494e.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     .L04f664(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L04f664:
        jsr     0x27c8c.l                       | +018
        bcc.w   .L04f674                        | +01e
        lea     Debris_BouncerRoll_04f692(pc),a1 | +022
        move.l  a1,(a6)                         | +026
.L04f674:
        jsr     0x28d70.l                       | +028
        lea     0x29624a.l,a0                   | +02e
        jsr     0x5dd5c.l                       | +034
        bcc.w   .L04f690                        | +03a
        jmp     0x518.l                         | +03e
.L04f690:
        rts                                     | +044

| ----------------------------------------------------------------------------
|  Debris_BouncerRoll_04f692  @ $04F692  (124 B)
| ----------------------------------------------------------------------------
        .section .text.Debris_BouncerRoll_04f692, "ax", @progbits
        .global Debris_BouncerRoll_04f692
Debris_BouncerRoll_04f692:
        jsr     0x267e2.l                       | +000
        move.w  #0x80,0x28(a6)                  | +006
        lea     0x2948f8.l,a0                   | +00c
        jsr     0x28cd4.l                       | +012
        btst    #0x0,0x3a(a6)                   | +018
        bne.w   .L04f6c0                        | +01e
        lea     0x294936.l,a0                   | +022
        jsr     0x28cd4.l                       | +028
.L04f6c0:
        lea     .L04f6c6(pc),a1                 | +02e
        move.l  a1,(a6)                         | +032
.L04f6c6:
        jsr     0x27a92.l                       | +034
        jsr     0x28d70.l                       | +03a
        jsr     0x27eba.l                       | +040
        bcc.w   .L04f6e2                        | +046
        lea     Debris_BouncerHop_04f64c(pc),a1 | +04a
        move.l  a1,(a6)                         | +04e
.L04f6e2:
        jsr     0x5e9b6.l                       | +050
        andi.w  #0xf,d0                         | +056
        bne.w   .L04f6f6                        | +05a
        lea     Debris_BouncerHop_04f64c(pc),a1 | +05e
        move.l  a1,(a6)                         | +062
.L04f6f6:
        lea     0x29624a.l,a0                   | +064
        jsr     0x5dd5c.l                       | +06a
        bcc.w   .L04f70c                        | +070
        jmp     0x518.l                         | +074
.L04f70c:
        rts                                     | +07a

| ----------------------------------------------------------------------------
|  Prop_SignA_04f70e  @ $04F70E  (94 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_SignA_04f70e, "ax", @progbits
        .global Prop_SignA_04f70e
Prop_SignA_04f70e:
        move.w  #0x186,d1                       | +000
        jsr     0x236e.l                        | +004
        lea     0x2949ae.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        move.w  #0x0,0x38(a6)                   | +016
        jsr     0x2783a.l                       | +01c
        lea     0x295f6e.l,a1                   | +022
        jsr     0x43fac.l                       | +028
        lea     .L04f742(pc),a1                 | +02e
        move.l  a1,(a6)                         | +032
.L04f742:
        jsr     0x2783a.l                       | +034
        jsr     0x28d70.l                       | +03a
        movea.l #0xffffffff,a0                  | +040
        lea     0x29625e.l,a0                   | +046
        jsr     0x5dd5c.l                       | +04c
        bcc.w   .L04f76a                        | +052
        jmp     0x518.l                         | +056
.L04f76a:
        rts                                     | +05c

| ----------------------------------------------------------------------------
|  Prop_SignB_04f76c  @ $04F76C  (94 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_SignB_04f76c, "ax", @progbits
        .global Prop_SignB_04f76c
Prop_SignB_04f76c:
        move.w  #0x1b6,d1                       | +000
        jsr     0x236e.l                        | +004
        lea     0x2949be.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        move.w  #0x0,0x38(a6)                   | +016
        jsr     0x2783a.l                       | +01c
        lea     0x295f94.l,a1                   | +022
        jsr     0x43fac.l                       | +028
        lea     .L04f7a0(pc),a1                 | +02e
        move.l  a1,(a6)                         | +032
.L04f7a0:
        jsr     0x2783a.l                       | +034
        jsr     0x28d70.l                       | +03a
        movea.l #0xffffffff,a0                  | +040
        lea     0x29625e.l,a0                   | +046
        jsr     0x5dd5c.l                       | +04c
        bcc.w   .L04f7c8                        | +052
        jmp     0x518.l                         | +056
.L04f7c8:
        rts                                     | +05c

| ----------------------------------------------------------------------------
|  Prop_Boat_04f7ca  @ $04F7CA  (394 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_Boat_04f7ca, "ax", @progbits
        .global Prop_Boat_04f7ca
Prop_Boat_04f7ca:
        move.w  #0x1ca,d1                       | +000
        jsr     0x236e.l                        | +004
        move.w  #0x1d8,d1                       | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0x1cb,d1                       | +014
        jsr     0x236e.l                        | +018
        lea     0x2c0092.l,a0                   | +01e
        jsr     0x799de.l                       | +024
        move.w  d0,0x66(a6)                     | +02a
        lea     0x2949ce.l,a0                   | +02e
        jsr     0x28cd4.l                       | +034
        cmpi.b  #0x1,0x98(a6)                   | +03a
        bne.w   .L04f81c                        | +040
        lea     0x2954a8.l,a0                   | +044
        move.l  a0,0x48(a6)                     | +04a
        bra.w   .L04f826                        | +04e
.L04f81c:
        lea     0x295454.l,a0                   | +052
        move.l  a0,0x48(a6)                     | +058
.L04f826:
        jsr     0x267e2.l                       | +05c
        jsr     0x5e7c0.l                       | +062
        move.w  #0x2000,0x38(a6)                | +068
        move.l  #0x296356,0x60(a6)              | +06e
        move.w  0x16(a6),0x14(a6)               | +076
        lea     .L04f84c(pc),a1                 | +07c
        move.l  a1,(a6)                         | +080
.L04f84c:
        jsr     0x2783a.l                       | +082
        cmpi.w  #0x148,0x22(a6)                 | +088
        bmi.w   .L04f862                        | +08e
        move.b  #0x2,0x45(a6)                   | +092
.L04f862:
        move.b  0x98(a6),d0                     | +098
        cmpi.b  #0x1,d0                         | +09c
        beq.w   .L04f88e                        | +0a0
        jsr     0x28998.l                       | +0a4
        move.w  0x22(a6),d0                     | +0aa
        subi.w  #0x12,d0                        | +0ae
        move.w  0x24(a6),d1                     | +0b2
        addi.w  #0x24,d1                        | +0b6
        move.w  #0x24,d2                        | +0ba
        jsr     0x99812.l                       | +0be
.L04f88e:
        jsr     0x28d70.l                       | +0c4
        move.w  0x16(a6),d0                     | +0ca
        btst    #0x3,0x106f28.l                 | +0ce
        beq.w   .L04f8a8                        | +0d6
        move.w  0x18(a6),d0                     | +0da
.L04f8a8:
        move.w  d0,0x14(a6)                     | +0de
        jsr     0x2870a.l                       | +0e2
        bcc.w   .L04f8cc                        | +0e8
        move.w  0x1a(a6),0x14(a6)               | +0ec
        bclr    #0x3,0x13(a6)                   | +0f2
        move.w  #0x108d,d0                      | +0f8
        jsr     0x2352.l                        | +0fc
.L04f8cc:
        jsr     0x28758.l                       | +102
        bcc.w   .L04f936                        | +108
        move.w  #0x1030,d0                      | +10c
        jsr     0x2352.l                        | +110
        lea     Prop_BoatBlast_04f954(pc),a1    | +116
        jsr     0x4ae.l                         | +11a
        jsr     0x5dd02.l                       | +120
        lea     0x295c7a.l,a1                   | +126
        jsr     0x77c7e.l                       | +12c
        lea     0x295c8c.l,a1                   | +132
        jsr     0x77c7e.l                       | +138
        lea     0x295c9e.l,a1                   | +13e
        jsr     0x77c7e.l                       | +144
        jsr     0x5e9b6.l                       | +14a
        andi.b  #0x3f,d0                        | +150
        bne.w   .L04f94c                        | +154
        lea     Prop_BoatDebris_04f992(pc),a1   | +158
        jsr     0x4ae.l                         | +15c
        jsr     0x5dd02.l                       | +162
        bra.w   .L04f94c                        | +168
.L04f936:
        movea.l #0xffffffff,a0                  | +16c
        lea     0x296268.l,a0                   | +172
        jsr     0x5dd5c.l                       | +178
        bcc.w   .L04f952                        | +17e
.L04f94c:
        jmp     0x518.l                         | +182
.L04f952:
        rts                                     | +188

| ----------------------------------------------------------------------------
|  Prop_BoatBlast_04f954  @ $04F954  (62 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_BoatBlast_04f954, "ax", @progbits
        .global Prop_BoatBlast_04f954
Prop_BoatBlast_04f954:
        move.w  #0x8,0x72(a6)                   | +000
        lea     .L04f960(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L04f960:
        jsr     0x2783a.l                       | +00c
        lea     0x2961a6.l,a0                   | +012
        move.l  a0,0x4c(a6)                     | +018
        jsr     0x283ca.l                       | +01c
        jsr     0x283ca.l                       | +022
        jsr     0x283d8.l                       | +028
        subq.w  #0x1,0x72(a6)                   | +02e
        bpl.w   .L04f990                        | +032
        jmp     0x518.l                         | +036
.L04f990:
        rts                                     | +03c

| ----------------------------------------------------------------------------
|  Prop_BoatDebris_04f992  @ $04F992  (174 B)
| ----------------------------------------------------------------------------
        .section .text.Prop_BoatDebris_04f992, "ax", @progbits
        .global Prop_BoatDebris_04f992
Prop_BoatDebris_04f992:
        move.w  #0x1c6,d1                       | +000
        jsr     0x236e.l                        | +004
        lea     0x2949de.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        move.b  #0x0,0x3a(a6)                   | +016
        jsr     0x267e2.l                       | +01c
        jsr     0x5e9b6.l                       | +022
        move.w  d0,d1                           | +028
        andi.w  #0x7ff,d0                       | +02a
        addi.w  #0x1000,d0                      | +02e
        move.w  d1,d0                           | +032
        andi.w  #0xff,d0                        | +034
        btst    #0x0,d0                         | +038
        bne.w   .L04f9d4                        | +03c
        neg.w   d0                              | +040
.L04f9d4:
        move.w  d0,0x28(a6)                     | +042
        move.w  d1,d0                           | +046
        andi.w  #0x7f,d0                        | +048
        addi.w  #0x7f,d0                        | +04c
        move.w  d0,0x2e(a6)                     | +050
        subi.w  #0x10,0x38(a6)                  | +054
        lea     .L04f9f2(pc),a1                 | +05a
        move.l  a1,(a6)                         | +05e
.L04f9f2:
        jsr     0x27cee.l                       | +060
        jsr     0x28d70.l                       | +066
        move.b  0x106f28.l,d0                   | +06c
        andi.b  #0x7,d0                         | +072
        bne.w   .L04fa22                        | +076
        lea     0x77efe.l,a1                    | +07a
        jsr     0x4ae.l                         | +080
        jsr     0x5dd02.l                       | +086
        subq.w  #0x1,0x38(a6)                   | +08c
.L04fa22:
        movea.l #0xffffffff,a0                  | +090
        lea     0x296268.l,a0                   | +096
        jsr     0x5dd5c.l                       | +09c
        bcc.w   .L04fa3e                        | +0a2
        jmp     0x518.l                         | +0a6
.L04fa3e:
        rts                                     | +0ac

| ----------------------------------------------------------------------------
|  FixTile_Set11C2_04fa40  @ $04FA40  (16 B)
| ----------------------------------------------------------------------------
        .section .text.FixTile_Set11C2_04fa40, "ax", @progbits
        .global FixTile_Set11C2_04fa40
FixTile_Set11C2_04fa40:
        move.w  #0x11,d1                        | +000
        move.w  #0xc2,d2                        | +004
        move.w  #0xffff,d3                      | +008
        move.w  #0x1,d4                         | +00c
