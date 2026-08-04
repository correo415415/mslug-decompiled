| ============================================================================
|  Metal Slug 1 - asm/squad_death_handlers_0807xx.s
|  ----------------------------------------------------------------------------
|  Wave DDD - handlers de muerte y escape del modulo "Squad Deploy".
|  Region $080736..$08180E, 29 entradas, 4 102 B (cierra los 23 huecos entre
|  las islas SetTaskHandler_*/SetXN_*/ClearXN_*/SetC_*/ClearC_*/JsrAbsThunk_*
|  ya matcheadas en C - con esto la region $07FBD2..$08180E queda 100%
|  decompilada junto con Wave CCC).
|
|  ARQUITECTURA
|  ------------
|  1. Tabla de despacho de muerte a 2 niveles en $2E3DC0 (8 punteros a
|     arrays de 8 handlers) elige, segun arma y tipo de soldado, entre:
|       - Squad_DeathTumble_080bd6   voltereta con rebote (curva $2E22FA)
|       - Squad_DeathSkid_080cdc     derrape horizontal con freno
|       - Squad_DeathSkidBrake_080d92 derrape + frenada por friccion
|       - Squad_DeathArcJump_080e48  arco parabolico (blast)
|       - Squad_DeathHopBack_080b1a  saltito hacia atras
|       - Squad_DeathPieces_080f32   despiece en fragmentos (gibs)
|       - TaskHandler_081018         variante de despiece con gate
|  2. Fragmentos/esquirlas (Squad_ShardSprites_08134c y plantillas
|     Template_08121C/08123C/081260/081284): cada pieza recibe sprite
|     propio, velocidad radial y gravedad; Squad_GibGate_081332 corta el
|     ciclo cuando la pieza sale de pantalla (Squad_BoundsCheck_080efa).
|  3. Escape del comandante (Squad_CmdrExitHop_08163a /
|     Squad_CmdrEscapeInit_08166e / Sub_0008169A): al vaciarse el
|     escuadron el comandante grita ($171), da saltitos con la curva
|     de campana SquadCurve_BellTable_08175e (32 words: rampa
|     0100,00F0..0180..0110 - tabla de datos embebida en codigo) y
|     abandona la pantalla; Squad_SpeedTailFrags_0817d0 suelta
|     fragmentos de estela y las Squad_LeaSprite_0817xx cargan los
|     sprites de salida ($2E3946/$2E3952/$2E395E).
|  4. Probes de proximidad Squad_ProbeLeaderNear/Far ($8095E/$80982)
|     comparan la x del lider con margenes +-$30/+-$60 y devuelven
|     el resultado en X/N via las islas SetXN_*/ClearXN_*.
|
|  Notas de matching:
|  - Los bcc.w "colgantes" saltan al RTS interno (+6) de la isla
|    SetTaskHandler_* siguiente -> defsyms SetHandlerRts_* en symbols.py.
|  - $8098E: bgt.b hacia ClearXN_08097c es cross-section y GAS no puede
|    emitir un branch byte con reloc -> se emite crudo como .dc.w 0x6eec.
|  - Verificado byte a byte contra build/mslug_prom.bin (pre-link:
|    11 exactas + 18 solo-reloc de 29).
| ============================================================================

        .text

| ----------------------------------------------------------------------------
|  TaskHandler_080736  @ $080736  (354 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_080736, "ax", @progbits
        .global TaskHandler_080736
TaskHandler_080736:
        move.b  #0x1,0x86(a6)                   | +000
        movea.l 0xc(a6),a0                      | +006
        move.w  0x36(a0),d0                     | +00a
        asr.w   #0x1,d0                         | +00e
        cmpi.b  #0x40,0x34(a0)                  | +010
        bls.w   .L80760                         | +016
        cmpi.b  #0xc0,0x34(a0)                  | +01a
        bhi.w   .L80760                         | +020
        clr.b   0x86(a6)                        | +024
        neg.w   d0                              | +028
.L80760:
        move.w  d0,0x28(a6)                     | +02a
        bset    #0x4,0x6b(a6)                   | +02e
        movea.l 0xc(a6),a0                      | +034
        clr.b   0x7f(a0)                        | +038
        lea     0x2e3922.l,a0                   | +03c
        jsr     0x28cd4.l                       | +042
        lea     .L80784(pc),a1                  | +048
        move.l  a1,(a6)                         | +04c
.L80784:
        jsr     0x5e506.l                       | +04e
        jsr     Sub_000808E8(pc)                | +054
        jsr     Sub_000808A0(pc)                | +058
        jsr     0x5e826.l                       | +05c
        jsr     0x28d70.l                       | +062
        bcc.w   .L807a8                         | +068
        lea     .L807ac(pc),a1                  | +06c
        move.l  a1,(a6)                         | +070
.L807a8:
        bra.w   Squad_CmdrTick_080704           | +072
.L807ac:
        move.b  0x86(a6),0x3a(a6)               | +076
        move.b  #0x0,0x20(a6)                   | +07c
        move.w  #0xffb0,0x2e(a6)                | +082
        move.w  #0xd000,d0                      | +088
        jsr     0x28134.l                       | +08c
        andi.w  #0xffe3,0x38(a6)                | +092
        ori.w   #0x0,0x38(a6)                   | +098
        move.w  #0x1,0x66(a6)                   | +09e
        move.l  #0x2e3af8,0x48(a6)              | +0a4
        bclr    #0x3,0x13(a6)                   | +0ac
        lea     0x2e394c.l,a0                   | +0b2
        jsr     0x28cd4.l                       | +0b8
        lea     .L807fa(pc),a1                  | +0be
        move.l  a1,(a6)                         | +0c2
.L807fa:
        jsr     0x27d50.l                       | +0c4
        bcc.w   .L8080a                         | +0ca
        lea     Squad_CmdrEscapeInit_08166e(pc),a1 | +0ce
        move.l  a1,(a6)                         | +0d2
.L8080a:
        tst.b   0x20(a6)                        | +0d4
        bne.w   .L80850                         | +0d8
        jsr     Squad_BoundsCheckY_080f1c(pc)   | +0dc
        bcc.w   .L80850                         | +0e0
        move.b  #0xff,0x20(a6)                  | +0e4
        movea.l 0xc(a6),a0                      | +0ea
        movea.l 0xc(a0),a0                      | +0ee
        cmpi.b  #0x2,0x9f(a0)                   | +0f2
        blt.w   .L80850                         | +0f8
        lea     0x78908.l,a1                    | +0fc
        jsr     0x4ae.l                         | +102
        jsr     0x5dd02.l                       | +108
        move.w  #0xffff,0x38(a0)                | +10e
        move.w  #0x120,0x24(a0)                 | +114
.L80850:
        jsr     0x5e826.l                       | +11a
        jsr     0x28d70.l                       | +120
        jsr     0x283d8.l                       | +126
        btst    #0x1,0x13(a6)                   | +12c
        beq.w   .L80872                         | +132
        lea     Squad_CmdrEscapeNoCry_081678(pc),a1 | +136
        move.l  a1,(a6)                         | +13a
.L80872:
        jsr     0x2870a.l                       | +13c
        bcc.w   .L80882                         | +142
        lea     Squad_CmdrEscapeInit_08166e(pc),a1 | +146
        move.l  a1,(a6)                         | +14a
.L80882:
        movea.l #0xffffffff,a0                  | +14c
        lea     0x2e3c44.l,a0                   | +152
        jsr     0x5dd56.l                       | +158
        bcc.w   SetHandlerRts_08089e            | +15e

| ----------------------------------------------------------------------------
|  Sub_000808A0  @ $0808A0  (134 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_000808A0, "ax", @progbits
        .global Sub_000808A0
Sub_000808A0:
        movea.l 0xc(a6),a0                      | +000
        move.b  0x7e(a0),d0                     | +004
        andi.l  #0x7,d0                         | +008
        lsl.l   #0x3,d0                         | +00e
        lea     0x2e3d80.l,a1                   | +010
        adda.l  d0,a1                           | +016
        move.w  0x4(a1),d0                      | +018
        add.w   d0,0x38(a6)                     | +01c
        move.w  0x2(a1),d0                      | +020
        btst    #0x1,0x3a(a6)                   | +024
        beq.w   .L808d0                         | +02a
        neg.w   d0                              | +02e
.L808d0:
        add.w   d0,0x24(a6)                     | +030
        move.w  (a1),d0                         | +034
        btst    #0x0,0x3a(a6)                   | +036
        beq.w   .L808e2                         | +03c
        neg.w   d0                              | +040
.L808e2:
        add.w   d0,0x22(a6)                     | +042
        rts                                     | +046
        .global Sub_000808E8
Sub_000808E8:
        movea.l 0xc(a6),a0                      | +048
        tst.b   0x82(a0)                        | +04c
        beq.w   ClearXN_080926                  | +050
        clr.b   0x7e(a0)                        | +054
        move.w  0x76(a0),d0                     | +058
        beq.w   .L80916                         | +05c
        cmpi.w  #0x8,d0                         | +060
        bne.w   SetXN_08092c                    | +064
        cmpi.b  #0x1,0x82(a0)                   | +068
        beq.w   ClearXN_080926                  | +06e
        bra.w   .L80920                         | +072
.L80916:
        cmpi.b  #0x2,0x82(a0)                   | +076
        beq.w   ClearXN_080926                  | +07c
.L80920:
        eori.b  #0x3,0x3a(a6)                   | +080

| ----------------------------------------------------------------------------
|  Sub_00080932  @ $080932  (12 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00080932, "ax", @progbits
        .global Sub_00080932
Sub_00080932:
        movea.l 0xc(a6),a0                      | +000
        tst.b   0x7f(a0)                        | +004
        beq.w   SetXN_080944                    | +008

| ----------------------------------------------------------------------------
|  Sub_0008094A  @ $08094A  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0008094A, "ax", @progbits
        .global Sub_0008094A
Sub_0008094A:
        movea.l 0xc(a6),a0                      | +000
        cmpi.b  #0x1,0x7f(a0)                   | +004
        beq.w   Squad_ProbeLeaderNear_08095e    | +00a

| ----------------------------------------------------------------------------
|  Squad_ProbeLeaderNear_08095e  @ $08095E  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_ProbeLeaderNear_08095e, "ax", @progbits
        .global Squad_ProbeLeaderNear_08095e
Squad_ProbeLeaderNear_08095e:
        movea.l 0x7a(a0),a1                     | +000
        cmpi.w  #0x3,0x78(a0)                   | +004
        bge.w   Squad_ProbeLeaderFar_080982     | +00a
        move.w  0x22(a6),d0                     | +00e
        sub.w   0x22(a1),d0                     | +012
        cmpi.w  #0x60,d0                        | +016
        ble.w   SetXN_080990                    | +01a

| ----------------------------------------------------------------------------
|  Squad_ProbeLeaderFar_080982  @ $080982  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_ProbeLeaderFar_080982, "ax", @progbits
        .global Squad_ProbeLeaderFar_080982
Squad_ProbeLeaderFar_080982:
        move.w  0x22(a1),d0                     | +000
        sub.w   0x22(a6),d0                     | +004
        cmpi.w  #0x60,d0                        | +008
        .dc.w   0x6eec                          | +00c  bgt.b ClearXN_08097c  (branch corto cross-section, byte-exacto)

| ----------------------------------------------------------------------------
|  TaskHandler_080996  @ $080996  (46 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_080996, "ax", @progbits
        .global TaskHandler_080996
TaskHandler_080996:
        move.w  #0x48,d1                        | +000
        jsr     0x236e.l                        | +004
        lea     0x2e35b8.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L809b2(pc),a1                  | +016
        move.l  a1,(a6)                         | +01a
.L809b2:
        jsr     0x5e506.l                       | +01c
        move.b  0x32(a0),0x32(a6)               | +022
        move.b  0x33(a0),0x33(a6)               | +028

| ----------------------------------------------------------------------------
|  TaskHandler_0809cc  @ $0809CC  (108 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0809cc, "ax", @progbits
        .global TaskHandler_0809cc
TaskHandler_0809cc:
        move.w  #0x48,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x9,0x1c(a6)                   | +00a
        jsr     0x138fe.l                       | +010
        lea     0x2e35b8.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
        lea     .L809f4(pc),a1                  | +022
        move.l  a1,(a6)                         | +026
.L809f4:
        jsr     0x5e506.l                       | +028
        movea.l 0xc(a6),a0                      | +02e
        move.w  0x76(a0),0x76(a6)               | +032
        move.b  0x32(a0),0x32(a6)               | +038
        move.b  0x33(a0),0x33(a6)               | +03e
        cmpi.b  #0x1,0x21(a0)                   | +044
        beq.w   Squad_ChildInitVar2_080a40      | +04a
        cmpi.b  #0xff,0x20(a0)                  | +04e
        beq.w   JmpToScheduler_081214           | +054
        movea.l 0xc(a6),a0                      | +058
        btst    #0x0,0x5a(a0)                   | +05c
        beq.w   JsrAbsThunk_080a38              | +062
        bset    #0x0,0x5a(a6)                   | +066

| ----------------------------------------------------------------------------
|  Squad_ChildInitVar2_080a40  @ $080A40  (98 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_ChildInitVar2_080a40, "ax", @progbits
        .global Squad_ChildInitVar2_080a40
Squad_ChildInitVar2_080a40:
        move.w  #0x48,d1                        | +000
        jsr     0x236e.l                        | +004
        lea     0x2e36b4.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L80a5c(pc),a1                  | +016
        move.l  a1,(a6)                         | +01a
.L80a5c:
        jsr     0x5e506.l                       | +01c
        clr.b   0x3a(a6)                        | +022
        movea.l 0xc(a6),a0                      | +026
        move.w  0x76(a0),0x76(a6)               | +02a
        move.b  0x32(a0),0x32(a6)               | +030
        move.b  0x33(a0),0x33(a6)               | +036
        tst.b   0x21(a0)                        | +03c
        beq.w   TaskHandler_0809cc              | +040
        cmpi.b  #0xff,0x20(a0)                  | +044
        beq.w   JmpToScheduler_081214           | +04a
        movea.l 0xc(a6),a0                      | +04e
        btst    #0x0,0x5a(a0)                   | +052
        beq.w   JsrAbsThunk_080aa2              | +058
        bset    #0x0,0x5a(a6)                   | +05c

| ----------------------------------------------------------------------------
|  Sub_00080AAA  @ $080AAA  (292 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00080AAA, "ax", @progbits
        .global Sub_00080AAA
Sub_00080AAA:
        lea     0xffff.w,a0                     | +000
        move.l  a0,0x48(a6)                     | +004
        bclr    #0x1,0x12(a6)                   | +008
        cmpi.b  #0x1,0x7f(a6)                   | +00e
        bne.w   .L80ac8                         | +014
        move.b  #0xff,0x7f(a6)                  | +018
.L80ac8:
        movea.l 0xc(a6),a0                      | +01e
        subq.b  #0x1,0x99(a0)                   | +022
        move.w  0x84(a6),d0                     | +026
        bclr    d0,0x77(a0)                     | +02a
        bclr    d0,0x78(a0)                     | +02e
        add.w   d0,d0                           | +032
        addi.w  #0x80,d0                        | +034
        move.w  #0xffff,(a0,d0.w)               | +038
        subq.b  #0x1,0x76(a0)                   | +03e
        clr.w   d0                              | +042
        move.b  0x20(a6),d0                     | +044
        lea     0x2e3dc0.l,a0                   | +048
        add.w   d0,d0                           | +04e
        add.w   d0,d0                           | +050
        movea.l (a0,d0.w),a0                    | +052
        move.l  a0,-(a7)                        | +056
        jsr     0x5e9b6.l                       | +058
        andi.w  #0x7,d0                         | +05e
        add.w   d0,d0                           | +062
        add.w   d0,d0                           | +064
        movea.l (a7)+,a0                        | +066
        movea.l (a0,d0.w),a0                    | +068
        move.l  a0,(a6)                         | +06c
        rts                                     | +06e
        .global Squad_DeathHopBack_080b1a
Squad_DeathHopBack_080b1a:
        move.w  #0xfe00,0x2a(a6)                | +070
        move.w  #0xffb0,0x2e(a6)                | +076
        move.w  0x36(a6),d0                     | +07c
        btst    #0x0,0x3a(a6)                   | +080
        bne.w   .L80b36                         | +086
        neg.w   d0                              | +08a
.L80b36:
        asr.w   #0x1,d0                         | +08c
        move.w  d0,0x28(a6)                     | +08e
        jsr     0x5e9b6.l                       | +092
        andi.w  #0x3f,d0                        | +098
        addi.w  #0x3c,d0                        | +09c
        move.w  d0,0x70(a6)                     | +0a0
        lea     0x2e305c.l,a0                   | +0a4
        jsr     0x28cd4.l                       | +0aa
        lea     .L80b60(pc),a1                  | +0b0
        move.l  a1,(a6)                         | +0b4
.L80b60:
        subq.w  #0x1,0x70(a6)                   | +0b6
        bne.w   .L80b86                         | +0ba
        lea     .L80b86(pc),a1                  | +0be
        move.l  a1,(a6)                         | +0c2
        jsr     0x5e9b6.l                       | +0c4
        andi.w  #0xf,d0                         | +0ca
        cmpi.w  #0x5,d0                         | +0ce
        bgt.w   .L80b86                         | +0d2
        lea     TaskHandler_081018(pc),a1       | +0d6
        move.l  a1,(a6)                         | +0da
.L80b86:
        jsr     0x27d50.l                       | +0dc
        bcc.w   .L80b96                         | +0e2
        lea     Squad_DeathPieces_080f32(pc),a1 | +0e6
        move.l  a1,(a6)                         | +0ea
.L80b96:
        jsr     0x28d70.l                       | +0ec
        jsr     Squad_BoundsCheck_080efa(pc)    | +0f2
        bcc.w   .L80baa                         | +0f6
        lea     TaskHandler_081018(pc),a1       | +0fa
        move.l  a1,(a6)                         | +0fe
.L80baa:
        jsr     Squad_BoundsCheckY_080f1c(pc)   | +100
        bcc.w   .L80bb8                         | +104
        lea     Squad_DeathPiecesAlt_080f3c(pc),a1 | +108
        move.l  a1,(a6)                         | +10c
.L80bb8:
        movea.l #0xffffffff,a0                  | +10e
        lea     0x2e3c44.l,a0                   | +114
        jsr     0x5dd5c.l                       | +11a
        bcc.w   SetHandlerRts_080bd4            | +120

| ----------------------------------------------------------------------------
|  Squad_DeathTumble_080bd6  @ $080BD6  (254 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_DeathTumble_080bd6, "ax", @progbits
        .global Squad_DeathTumble_080bd6
Squad_DeathTumble_080bd6:
        move.l  #0x2e2d42,0x90(a6)              | +000
        btst    #0x0,0x3a(a6)                   | +008
        beq.w   .L80bf0                         | +00e
        move.l  #0x2e2d78,0x90(a6)              | +012
.L80bf0:
        moveq   #0x0,d0                         | +01a
        move.b  d0,0x20(a6)                     | +01c
        move.w  d0,0x94(a6)                     | +020
        move.b  d0,0x96(a6)                     | +024
        lea     .L80c68(pc),a1                  | +028
        move.l  a1,(a6)                         | +02c
        bra.w   .L80c68                         | +02e
        .global Squad_DeathTumbleMid_080c08
Squad_DeathTumbleMid_080c08:
        cmpi.b  #0x18,0x34(a6)                  | +032
        blt.w   .L80c1c                         | +038
        cmpi.b  #0x68,0x34(a6)                  | +03c
        ble.w   .L80c3a                         | +042
.L80c1c:
        move.l  #0x2e2e02,0x90(a6)              | +046
        btst    #0x0,0x3a(a6)                   | +04e
        beq.w   .L80c54                         | +054
        move.l  #0x2e2e74,0x90(a6)              | +058
        bra.w   .L80c54                         | +060
.L80c3a:
        move.l  #0x2e2e3e,0x90(a6)              | +064
        btst    #0x0,0x3a(a6)                   | +06c
        beq.w   .L80c54                         | +072
        move.l  #0x2e2eb0,0x90(a6)              | +076
.L80c54:
        moveq   #0x0,d0                         | +07e
        move.b  d0,0x20(a6)                     | +080
        move.w  d0,0x94(a6)                     | +084
        move.b  d0,0x96(a6)                     | +088
        lea     .L80c68(pc),a1                  | +08c
        move.l  a1,(a6)                         | +090
.L80c68:
        jsr     Sub_0008179E(pc)                | +092
        jsr     0x78f8a.l                       | +096
        bcc.w   .L80c7c                         | +09c
        lea     TaskHandler_0811cc(pc),a1       | +0a0
        move.l  a1,(a6)                         | +0a4
.L80c7c:
        jsr     0x28d70.l                       | +0a6
        move.w  0x22(a6),d1                     | +0ac
        move.w  0x24(a6),d2                     | +0b0
        jsr     0x280c6.l                       | +0b4
        bcc.w   .L80ca2                         | +0ba
        andi.b  #0xf0,d0                        | +0be
        bne.w   .L80ca2                         | +0c2
        lea     Squad_DeathPieces_080f32(pc),a1 | +0c6
        move.l  a1,(a6)                         | +0ca
.L80ca2:
        jsr     Squad_BoundsCheck_080efa(pc)    | +0cc
        bcc.w   .L80cb0                         | +0d0
        lea     TaskHandler_081018(pc),a1       | +0d4
        move.l  a1,(a6)                         | +0d8
.L80cb0:
        jsr     Squad_BoundsCheckY_080f1c(pc)   | +0da
        bcc.w   .L80cbe                         | +0de
        lea     Squad_DeathPiecesAlt_080f3c(pc),a1 | +0e2
        move.l  a1,(a6)                         | +0e6
.L80cbe:
        movea.l #0xffffffff,a0                  | +0e8
        lea     0x2e3c44.l,a0                   | +0ee
        jsr     0x5dd5c.l                       | +0f4
        bcc.w   SetHandlerRts_080cda            | +0fa

| ----------------------------------------------------------------------------
|  Squad_DeathSkid_080cdc  @ $080CDC  (174 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_DeathSkid_080cdc, "ax", @progbits
        .global Squad_DeathSkid_080cdc
Squad_DeathSkid_080cdc:
        move.l  #0x2e2dae,0x90(a6)              | +000
        btst    #0x0,0x3a(a6)                   | +008
        beq.w   .L80cf6                         | +00e
        move.l  #0x2e2dd8,0x90(a6)              | +012
.L80cf6:
        moveq   #0x0,d0                         | +01a
        move.b  d0,0x20(a6)                     | +01c
        move.w  d0,0x94(a6)                     | +020
        move.b  d0,0x96(a6)                     | +024
        asl.w   0x72(a6)                        | +028
        lea     .L80d0e(pc),a1                  | +02c
        move.l  a1,(a6)                         | +030
.L80d0e:
        move.w  0x72(a6),d0                     | +032
        sub.w   0x2e(a6),d0                     | +036
        bpl.w   .L80d28                         | +03a
        clr.w   d0                              | +03e
        move.w  #0xffe0,0x2e(a6)                | +040
        lea     Squad_DeathSkidBrake_080d92(pc),a1 | +046
        move.l  a1,(a6)                         | +04a
.L80d28:
        move.w  d0,0x72(a6)                     | +04c
        jsr     Sub_0008179E(pc)                | +050
        .global Squad_DeathSkidTick_080d30
Squad_DeathSkidTick_080d30:
        jsr     0x78f8a.l                       | +054
        bcc.w   .L80d40                         | +05a
        lea     TaskHandler_0811cc(pc),a1       | +05e
        move.l  a1,(a6)                         | +062
.L80d40:
        jsr     0x28d70.l                       | +064
        move.w  0x22(a6),d1                     | +06a
        move.w  0x24(a6),d2                     | +06e
        jsr     0x280c6.l                       | +072
        bcc.w   .L80d66                         | +078
        andi.b  #0xf0,d0                        | +07c
        bne.w   .L80d66                         | +080
        lea     Squad_DeathPieces_080f32(pc),a1 | +084
        move.l  a1,(a6)                         | +088
.L80d66:
        jsr     Squad_BoundsCheckY_080f1c(pc)   | +08a
        bcc.w   .L80d74                         | +08e
        lea     Squad_DeathPiecesAlt_080f3c(pc),a1 | +092
        move.l  a1,(a6)                         | +096
.L80d74:
        movea.l #0xffffffff,a0                  | +098
        lea     0x2e3c44.l,a0                   | +09e
        jsr     0x5dd5c.l                       | +0a4
        bcc.w   SetHandlerRts_080d90            | +0aa

| ----------------------------------------------------------------------------
|  Squad_DeathSkidBrake_080d92  @ $080D92  (174 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_DeathSkidBrake_080d92, "ax", @progbits
        .global Squad_DeathSkidBrake_080d92
Squad_DeathSkidBrake_080d92:
        jsr     0x27d50.l                       | +000
        bcc.w   .L80da2                         | +006
        lea     Squad_DeathPieces_080f32(pc),a1 | +00a
        move.l  a1,(a6)                         | +00e
.L80da2:
        jmp     Squad_DeathSkidTick_080d30(pc)  | +010
        .global Squad_DeathBlownAway_080da6
Squad_DeathBlownAway_080da6:
        move.l  #0x2e2ee6,0x90(a6)              | +014
        btst    #0x0,0x3a(a6)                   | +01c
        beq.w   .L80dc0                         | +022
        move.l  #0x2e2f06,0x90(a6)              | +026
.L80dc0:
        moveq   #0x0,d0                         | +02e
        move.b  d0,0x20(a6)                     | +030
        move.w  d0,0x94(a6)                     | +034
        move.b  d0,0x96(a6)                     | +038
        move.w  #0x2000,d0                      | +03c
        jsr     0x28134.l                       | +040
        andi.w  #0xffe3,0x38(a6)                | +046
        ori.w   #0x14,0x38(a6)                  | +04c
        move.w  #0x1,0x70(a6)                   | +052
        lea     .L80df0(pc),a1                  | +058
        move.l  a1,(a6)                         | +05c
.L80df0:
        jsr     Sub_0008179E(pc)                | +05e
        cmpi.b  #0x10,0x32(a6)                  | +062
        bls.w   .L80e14                         | +068
        subq.w  #0x1,0x70(a6)                   | +06c
        bne.w   .L80e14                         | +070
        move.w  #0x1,0x70(a6)                   | +074
        subq.b  #0x1,0x32(a6)                   | +07a
        subq.b  #0x1,0x33(a6)                   | +07e
.L80e14:
        jsr     0x78f8a.l                       | +082
        bcc.w   .L80e24                         | +088
        lea     Squad_DeathPieces_080f32(pc),a1 | +08c
        move.l  a1,(a6)                         | +090
.L80e24:
        jsr     0x28d70.l                       | +092
        movea.l #0xffffffff,a0                  | +098
        lea     0x2e3c44.l,a0                   | +09e
        jsr     0x5dd5c.l                       | +0a4
        bcc.w   SetHandlerRts_080e46            | +0aa

| ----------------------------------------------------------------------------
|  Squad_DeathArcJump_080e48  @ $080E48  (170 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_DeathArcJump_080e48, "ax", @progbits
        .global Squad_DeathArcJump_080e48
Squad_DeathArcJump_080e48:
        lea     0x2e30ae.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        btst    #0x0,0x3a(a6)                   | +00c
        beq.w   .L80e6a                         | +012
        lea     0x2e330a.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
.L80e6a:
        moveq   #0x0,d0                         | +022
        move.b  d0,0x20(a6)                     | +024
        move.b  #0x1,0x21(a6)                   | +028
        move.b  0x34(a6),d0                     | +02e
        move.w  d0,d1                           | +032
        cmpi.w  #0xf8,d1                        | +034
        bcs.w   .L80e86                         | +038
        moveq   #0x0,d1                         | +03c
.L80e86:
        asr.w   #0x4,d1                         | +03e
        move.w  d1,0x76(a6)                     | +040
        move.w  0x36(a6),d1                     | +044
        move.w  d1,d2                           | +048
        asr.w   #0x1,d2                         | +04a
        add.w   d2,d1                           | +04c
        jsr     0x13c0e.l                       | +04e
        move.w  d1,0x28(a6)                     | +054
        move.w  d2,0x2a(a6)                     | +058
        lea     .L80eaa(pc),a1                  | +05c
        move.l  a1,(a6)                         | +060
.L80eaa:
        jsr     0x27cee.l                       | +062
        bcc.w   .L80eba                         | +068
        lea     Squad_DeathPieces_080f32(pc),a1 | +06c
        move.l  a1,(a6)                         | +070
.L80eba:
        jsr     0x28d70.l                       | +072
        jsr     Squad_BoundsCheckY_080f1c(pc)   | +078
        bcc.w   .L80ece                         | +07c
        lea     Squad_DeathPiecesAlt_080f3c(pc),a1 | +080
        move.l  a1,(a6)                         | +084
.L80ece:
        jsr     Squad_BoundsCheck_080efa(pc)    | +086
        bcc.w   .L80edc                         | +08a
        lea     TaskHandler_081018(pc),a1       | +08e
        move.l  a1,(a6)                         | +092
.L80edc:
        movea.l #0xffffffff,a0                  | +094
        lea     0x2e3c44.l,a0                   | +09a
        jsr     0x5dd5c.l                       | +0a0
        bcc.w   SetHandlerRts_080ef8            | +0a6

| ----------------------------------------------------------------------------
|  Squad_BoundsCheck_080efa  @ $080EFA  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_BoundsCheck_080efa, "ax", @progbits
        .global Squad_BoundsCheck_080efa
Squad_BoundsCheck_080efa:
        cmpi.w  #0x20,0x22(a6)                  | +000
        ble.w   SetC_080f2c                     | +006
        cmpi.w  #0x120,0x22(a6)                 | +00a
        bge.w   SetC_080f2c                     | +010
        cmpi.w  #0x200,0x24(a6)                 | +014
        bge.w   SetC_080f2c                     | +01a
        bra.w   ClearC_080f26                   | +01e
        .global Squad_BoundsCheckY_080f1c
Squad_BoundsCheckY_080f1c:
        cmpi.w  #0x110,0x24(a6)                 | +022
        ble.w   SetC_080f2c                     | +028

| ----------------------------------------------------------------------------
|  Squad_DeathPieces_080f32  @ $080F32  (652 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_DeathPieces_080f32, "ax", @progbits
        .global Squad_DeathPieces_080f32
Squad_DeathPieces_080f32:
        jsr     0x434dc.l                       | +000
        bra.w   .L80f68                         | +006
        .global Squad_DeathPiecesAlt_080f3c
Squad_DeathPiecesAlt_080f3c:
        movea.l 0xc(a6),a0                      | +00a
        cmpi.b  #0x2,0x9f(a0)                   | +00e
        blt.w   .L80f68                         | +014
        lea     0x78890.l,a1                    | +018
        jsr     0x4ae.l                         | +01e
        jsr     0x5dd02.l                       | +024
        move.w  #0xffff,0x38(a0)                | +02a
        move.w  #0x120,0x24(a0)                 | +030
.L80f68:
        lea     0x77fd6.l,a1                    | +036
        jsr     0x4ae.l                         | +03c
        jsr     0x5dd02.l                       | +042
        move.b  0x32(a6),0x32(a0)               | +048
        move.b  0x33(a6),0x33(a0)               | +04e
        lea     0x2e3c54.l,a1                   | +054
        jsr     0x77c7e.l                       | +05a
        move.b  0x32(a6),0x32(a0)               | +060
        move.b  0x33(a6),0x33(a0)               | +066
        lea     Squad_ShardSprites_08134c(pc),a1 | +06c
        jsr     0x4ae.l                         | +070
        jsr     0x5dd02.l                       | +076
        move.b  0x32(a6),0x32(a0)               | +07c
        move.b  0x33(a6),0x33(a0)               | +082
        lea     Squad_ShardSprites_08134c(pc),a1 | +088
        jsr     0x4ae.l                         | +08c
        jsr     0x5dd02.l                       | +092
        eori.b  #0x1,0x3a(a0)                   | +098
        move.b  0x32(a6),0x32(a0)               | +09e
        move.b  0x33(a6),0x33(a0)               | +0a4
        lea     Squad_ShardSpriteB_08135c(pc),a1 | +0aa
        jsr     0x4ae.l                         | +0ae
        jsr     0x5dd02.l                       | +0b4
        move.b  0x32(a6),0x32(a0)               | +0ba
        move.b  0x33(a6),0x33(a0)               | +0c0
        lea     Squad_ShardSpriteC_08136c(pc),a1 | +0c6
        jsr     0x4ae.l                         | +0ca
        jsr     0x5dd02.l                       | +0d0
        move.b  0x32(a6),0x32(a0)               | +0d6
        move.b  0x33(a6),0x33(a0)               | +0dc
        bra.w   TaskHandler_0811cc              | +0e2
        .global TaskHandler_081018
TaskHandler_081018:
        movea.l 0xc(a6),a0                      | +0e6
        btst    #0x0,0x9f(a0)                   | +0ea
        bne.w   .L810a0                         | +0f0
        jsr     0x5e804.l                       | +0f4
        bcs.w   .L810a0                         | +0fa
        lea     Template_08121C(pc),a1          | +0fe
        jsr     0x4ae.l                         | +102
        jsr     0x5dd02.l                       | +108
        move.b  0x32(a6),0x32(a0)               | +10e
        move.b  0x33(a6),0x33(a0)               | +114
        lea     Template_08123C(pc),a1          | +11a
        jsr     0x4ae.l                         | +11e
        jsr     0x5dd02.l                       | +124
        move.b  0x32(a6),0x32(a0)               | +12a
        move.b  0x33(a6),0x33(a0)               | +130
        lea     Template_081260(pc),a1          | +136
        jsr     0x4ae.l                         | +13a
        jsr     0x5dd02.l                       | +140
        move.b  0x32(a6),0x32(a0)               | +146
        move.b  0x33(a6),0x33(a0)               | +14c
        lea     Template_081284(pc),a1          | +152
        jsr     0x4ae.l                         | +156
        jsr     0x5dd02.l                       | +15c
        move.b  0x32(a6),0x32(a0)               | +162
        move.b  0x33(a6),0x33(a0)               | +168
.L810a0:
        move.w  #0x10ba,d0                      | +16e
        jsr     0x2222.l                        | +172
        lea     0x77fd6.l,a1                    | +178
        jsr     0x4ae.l                         | +17e
        jsr     0x5dd02.l                       | +184
        move.b  0x32(a6),0x32(a0)               | +18a
        move.b  0x33(a6),0x33(a0)               | +190
        lea     0x2e3c66.l,a1                   | +196
        jsr     0x77c7e.l                       | +19c
        move.b  0x32(a6),0x32(a0)               | +1a2
        move.b  0x33(a6),0x33(a0)               | +1a8
        move.b  0x34(a6),0x34(a0)               | +1ae
        move.w  0x36(a6),0x36(a0)               | +1b4
        lea     Squad_ShardSpriteB2_08139c(pc),a1 | +1ba
        jsr     0x4ae.l                         | +1be
        jsr     0x5dd02.l                       | +1c4
        move.b  0x32(a6),0x32(a0)               | +1ca
        move.b  0x33(a6),0x33(a0)               | +1d0
        move.b  0x34(a6),0x34(a0)               | +1d6
        move.w  0x36(a6),0x36(a0)               | +1dc
        lea     Squad_ShardSpriteB2_08139c(pc),a1 | +1e2
        jsr     0x4ae.l                         | +1e6
        jsr     0x5dd02.l                       | +1ec
        move.b  0x32(a6),0x32(a0)               | +1f2
        move.b  0x33(a6),0x33(a0)               | +1f8
        move.b  0x34(a6),0x34(a0)               | +1fe
        move.w  0x36(a6),0x36(a0)               | +204
        eori.b  #0x1,0x3a(a0)                   | +20a
        lea     Squad_ShardSpriteC2_0813ac(pc),a1 | +210
        jsr     0x4ae.l                         | +214
        jsr     0x5dd02.l                       | +21a
        move.b  0x32(a6),0x32(a0)               | +220
        move.b  0x33(a6),0x33(a0)               | +226
        move.b  0x34(a6),0x34(a0)               | +22c
        move.w  0x36(a6),0x36(a0)               | +232
        lea     Squad_ShardSpriteD2_0813dc(pc),a1 | +238
        jsr     0x4ae.l                         | +23c
        jsr     0x5dd02.l                       | +242
        move.b  0x32(a6),0x32(a0)               | +248
        move.b  0x33(a6),0x33(a0)               | +24e
        move.b  0x34(a6),0x34(a0)               | +254
        move.w  0x36(a6),0x36(a0)               | +25a
        bra.w   TaskHandler_0811cc              | +260
        .global TaskHandler_081196
TaskHandler_081196:
        bclr    #0x1,0x12(a6)                   | +264
        movea.l 0xc(a6),a0                      | +26a
        move.w  0x84(a6),d0                     | +26e
        bclr    d0,0x78(a0)                     | +272
        add.w   d0,d0                           | +276
        addi.w  #0x80,d0                        | +278
        move.w  0x66(a6),(a0,d0.w)              | +27c
        subq.b  #0x1,0x76(a0)                   | +282
        move.b  #0xff,0x20(a6)                  | +286

| ----------------------------------------------------------------------------
|  TaskHandler_0811cc  @ $0811CC  (72 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0811cc, "ax", @progbits
        .global TaskHandler_0811cc
TaskHandler_0811cc:
        move.w  #0x1023,d0                      | +000
        jsr     0x2352.l                        | +004
        .global Sub_000811D6
Sub_000811D6:
        move.b  #0xff,0x20(a6)                  | +00a
        jmp     0x518.l                         | +010
        rts                                     | +016
        .global TaskHandler_0811e4
TaskHandler_0811e4:
        bclr    #0x1,0x12(a6)                   | +018
        movea.l 0xc(a6),a0                      | +01e
        move.w  0x84(a6),d0                     | +022
        bclr    d0,0x78(a0)                     | +026
        add.w   d0,d0                           | +02a
        addi.w  #0x80,d0                        | +02c
        move.w  0x66(a6),(a0,d0.w)              | +030
        subq.b  #0x1,0x76(a0)                   | +036
        move.b  #0xff,0x20(a6)                  | +03a
        jmp     0x518.l                         | +040
        rts                                     | +046

| ----------------------------------------------------------------------------
|  Template_08121C  @ $08121C  (154 B)
| ----------------------------------------------------------------------------
        .section .text.Template_08121C, "ax", @progbits
        .global Template_08121C
Template_08121C:
        jsr     0x5e9b6.l                       | +000
        andi.w  #0x3f,d0                        | +006
        move.w  #0x2000,d1                      | +00a
        jsr     0x13c0e.l                       | +00e
        move.w  d1,0x28(a6)                     | +014
        move.w  d2,0x2a(a6)                     | +018
        bra.w   .L812a4                         | +01c
        .global Template_08123C
Template_08123C:
        jsr     0x5e9b6.l                       | +020
        andi.w  #0x3f,d0                        | +026
        addi.w  #0x40,d0                        | +02a
        move.w  #0x2000,d1                      | +02e
        jsr     0x13c0e.l                       | +032
        move.w  d1,0x28(a6)                     | +038
        move.w  d2,0x2a(a6)                     | +03c
        bra.w   .L812a4                         | +040
        .global Template_081260
Template_081260:
        jsr     0x5e9b6.l                       | +044
        andi.w  #0x3f,d0                        | +04a
        addi.w  #0x80,d0                        | +04e
        move.w  #0x2000,d1                      | +052
        jsr     0x13c0e.l                       | +056
        move.w  d1,0x28(a6)                     | +05c
        move.w  d2,0x2a(a6)                     | +060
        bra.w   .L812a4                         | +064
        .global Template_081284
Template_081284:
        jsr     0x5e9b6.l                       | +068
        andi.w  #0x3f,d0                        | +06e
        addi.w  #0xc0,d0                        | +072
        move.w  #0x2000,d1                      | +076
        jsr     0x13c0e.l                       | +07a
        move.w  d1,0x28(a6)                     | +080
        move.w  d2,0x2a(a6)                     | +084
.L812a4:
        move.w  #0x3,0x70(a6)                   | +088
        move.w  #0x3,0x72(a6)                   | +08e
        move.w  #0x4,0x74(a6)                   | +094

| ----------------------------------------------------------------------------
|  TaskHandler_0812be  @ $0812BE  (134 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0812be, "ax", @progbits
        .global TaskHandler_0812be
TaskHandler_0812be:
        jsr     0x2783a.l                       | +000
        subq.w  #0x1,0x70(a6)                   | +006
        bne.w   .L81330                         | +00a
        move.w  0x72(a6),0x70(a6)               | +00e
        move.w  0x28(a6),d0                     | +014
        move.w  d0,d1                           | +018
        asr.w   #0x8,d1                         | +01a
        add.b   d0,0x26(a6)                     | +01c
        moveq   #0x0,d0                         | +020
        addx.w  d0,d1                           | +022
        add.w   d1,0x22(a6)                     | +024
        move.w  0x2a(a6),d0                     | +028
        move.w  d0,d1                           | +02c
        asr.w   #0x8,d1                         | +02e
        add.b   d0,0x27(a6)                     | +030
        moveq   #0x0,d0                         | +034
        addx.w  d0,d1                           | +036
        add.w   d1,0x24(a6)                     | +038
        jsr     0x5e804.l                       | +03c
        bcs.w   .L8132a                         | +042
        lea     0x77efe.l,a1                    | +046
        jsr     0x4ae.l                         | +04c
        jsr     0x5dd02.l                       | +052
        move.b  0x32(a6),0x32(a0)               | +058
        move.b  0x33(a6),0x33(a0)               | +05e
        subq.w  #0x1,0x74(a6)                   | +064
        bne.w   .L81330                         | +068
.L8132a:
        jmp     0x518.l                         | +06c
.L81330:
        rts                                     | +072
        .global Squad_GibGate_081332
Squad_GibGate_081332:
        jsr     0x5e9b6.l                       | +074
        andi.w  #0xf,d0                         | +07a
        cmpi.w  #0x6,d0                         | +07e
        bge.w   SetHandlerRts_08134a            | +082

| ----------------------------------------------------------------------------
|  Squad_ShardSprites_08134c  @ $08134C  (742 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_ShardSprites_08134c, "ax", @progbits
        .global Squad_ShardSprites_08134c
Squad_ShardSprites_08134c:
        lea     0x2e3990.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        bra.w   .L81378                         | +00c
        .global Squad_ShardSpriteB_08135c
Squad_ShardSpriteB_08135c:
        lea     0x2e39ec.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        bra.w   .L81378                         | +01c
        .global Squad_ShardSpriteC_08136c
Squad_ShardSpriteC_08136c:
        lea     0x2e3a48.l,a0                   | +020
        jsr     0x28cd4.l                       | +026
.L81378:
        move.w  #0xc000,d0                      | +02c
        jsr     0x28134.l                       | +030
        andi.w  #0xffe3,0x38(a6)                | +036
        ori.w   #0x1c,0x38(a6)                  | +03c
        move.w  #0x48,d1                        | +042
        jsr     0x236e.l                        | +046
        bra.w   .L81492                         | +04c
        .global Squad_ShardSpriteB2_08139c
Squad_ShardSpriteB2_08139c:
        lea     0x2e3990.l,a0                   | +050
        jsr     0x28cd4.l                       | +056
        bra.w   .L813e8                         | +05c
        .global Squad_ShardSpriteC2_0813ac
Squad_ShardSpriteC2_0813ac:
        lea     0x2e39ec.l,a0                   | +060
        jsr     0x28cd4.l                       | +066
        move.b  #0x1,0x3a(a6)                   | +06c
        cmpi.b  #0x40,0x34(a6)                  | +072
        bls.w   .L813e8                         | +078
        cmpi.b  #0xc0,0x34(a6)                  | +07c
        bhi.w   .L813e8                         | +082
        move.b  #0x0,0x3a(a6)                   | +086
        bra.w   .L813e8                         | +08c
        .global Squad_ShardSpriteD2_0813dc
Squad_ShardSpriteD2_0813dc:
        lea     0x2e3a48.l,a0                   | +090
        jsr     0x28cd4.l                       | +096
.L813e8:
        move.w  #0xc000,d0                      | +09c
        jsr     0x28134.l                       | +0a0
        andi.w  #0xffe3,0x38(a6)                | +0a6
        ori.w   #0x1c,0x38(a6)                  | +0ac
        move.w  #0x48,d1                        | +0b2
        jsr     0x236e.l                        | +0b6
        bra.w   .L8154a                         | +0bc
        lea     0x2de3c4.l,a0                   | +0c0
        jsr     0x28cd4.l                       | +0c6
        moveq   #0xa,d1                         | +0cc
        bra.w   .L81462                         | +0ce
        lea     0x2de34e.l,a0                   | +0d2
        jsr     0x28cd4.l                       | +0d8
        moveq   #0xa,d1                         | +0de
        bra.w   .L81462                         | +0e0
        lea     0x2de43a.l,a0                   | +0e4
        jsr     0x28cd4.l                       | +0ea
        moveq   #0xa,d1                         | +0f0
        bra.w   .L81462                         | +0f2
        lea     0x2de5f4.l,a0                   | +0f6
        jsr     0x28cd4.l                       | +0fc
        moveq   #0xb,d1                         | +102
        bra.w   .L81462                         | +104
        lea     0x2de66a.l,a0                   | +108
        jsr     0x28cd4.l                       | +10e
        moveq   #0xb,d1                         | +114
.L81462:
        jsr     0x236e.l                        | +116
        movea.l 0xc(a6),a0                      | +11c
        move.w  0x36(a0),0x36(a6)               | +120
        move.w  0x34(a0),0x34(a6)               | +126
        move.w  #0xc000,d0                      | +12c
        jsr     0x28134.l                       | +130
        andi.w  #0xffe3,0x38(a6)                | +136
        ori.w   #0x1c,0x38(a6)                  | +13c
        bra.w   .L8154a                         | +142
.L81492:
        cmpi.w  #0x0,0x36(a6)                   | +146
        bgt.w   .L814a2                         | +14c
        move.w  #0x400,0x36(a6)                 | +150
.L814a2:
        jsr     0x5e9b6.l                       | +156
        move.w  d0,d3                           | +15c
        andi.l  #0x1e,d0                        | +15e
        addi.w  #0x30,d0                        | +164
        add.w   d0,d0                           | +168
        lea     0x2c072c.l,a1                   | +16a
        move.w  (a1,d0.w),d1                    | +170
        lea     0x2c07ac.l,a1                   | +174
        move.w  (a1,d0.w),d2                    | +17a
        andi.w  #0x7,d3                         | +17e
        addq.w  #0x1,d3                         | +182
        muls.w  d3,d1                           | +184
        asr.w   #0x2,d1                         | +186
        move.w  0x36(a6),d0                     | +188
        muls.w  d0,d1                           | +18c
        muls.w  d0,d2                           | +18e
        asr.l   #0x8,d1                         | +190
        asr.l   #0x8,d2                         | +192
        move.w  d1,0x2a(a6)                     | +194
        move.w  d2,0x28(a6)                     | +198
        jsr     0x5e9b6.l                       | +19c
        cmpi.b  #0x0,d0                         | +1a2
        bgt.w   .L81526                         | +1a6
        jsr     0x5e9b6.l                       | +1aa
        andi.w  #0x3,d0                         | +1b0
        addq.w  #0x2,d0                         | +1b4
        move.w  0x28(a6),d1                     | +1b6
        muls.w  d1,d0                           | +1ba
        asr.w   #0x1,d0                         | +1bc
        move.w  d0,0x28(a6)                     | +1be
        jsr     0x5e9b6.l                       | +1c2
        andi.w  #0x3,d0                         | +1c8
        addq.w  #0x2,d0                         | +1cc
        move.w  0x2a(a6),d1                     | +1ce
        muls.w  d1,d0                           | +1d2
        asr.w   #0x1,d0                         | +1d4
        move.w  d0,0x2a(a6)                     | +1d6
.L81526:
        move.w  0x2a(a6),d0                     | +1da
        cmpi.w  #0xc00,d0                       | +1de
        ble.w   .L81536                         | +1e2
        move.w  #0xc00,d0                       | +1e6
.L81536:
        move.w  d0,0x74(a6)                     | +1ea
        move.w  #0xffe0,0x2e(a6)                | +1ee
        lea     .L815e8(pc),a1                  | +1f4
        move.l  a1,(a6)                         | +1f8
        bra.w   .L815e8                         | +1fa
.L8154a:
        cmpi.w  #0x0,0x36(a6)                   | +1fe
        bgt.w   .L8155a                         | +204
        move.w  #0x400,0x36(a6)                 | +208
.L8155a:
        jsr     0x5e9b6.l                       | +20e
        andi.w  #0x1f,d0                        | +214
        subi.w  #0x10,d0                        | +218
        move.b  0x34(a6),d1                     | +21c
        andi.w  #0xff,d1                        | +220
        add.w   d1,d0                           | +224
        andi.w  #0xff,d0                        | +226
        move.w  0x36(a6),d1                     | +22a
        asl.w   #0x1,d1                         | +22e
        jsr     0x13c0e.l                       | +230
        move.w  d1,0x28(a6)                     | +236
        move.w  d2,0x2a(a6)                     | +23a
        jsr     0x5e9b6.l                       | +23e
        cmpi.b  #0x0,d0                         | +244
        bgt.w   .L815c8                         | +248
        jsr     0x5e9b6.l                       | +24c
        andi.w  #0x3,d0                         | +252
        addq.w  #0x2,d0                         | +256
        move.w  0x28(a6),d1                     | +258
        muls.w  d1,d0                           | +25c
        asr.w   #0x1,d0                         | +25e
        move.w  d0,0x28(a6)                     | +260
        jsr     0x5e9b6.l                       | +264
        andi.w  #0x3,d0                         | +26a
        addq.w  #0x2,d0                         | +26e
        move.w  0x2a(a6),d1                     | +270
        muls.w  d1,d0                           | +274
        asr.w   #0x1,d0                         | +276
        move.w  d0,0x2a(a6)                     | +278
.L815c8:
        move.w  0x2a(a6),d0                     | +27c
        cmpi.w  #0xc00,d0                       | +280
        ble.w   .L815d8                         | +284
        move.w  #0xc00,d0                       | +288
.L815d8:
        move.w  d0,0x74(a6)                     | +28c
        move.w  #0xffe0,0x2e(a6)                | +290
        lea     .L815e8(pc),a1                  | +296
        move.l  a1,(a6)                         | +29a
.L815e8:
        jsr     0x27d50.l                       | +29c
        bcc.w   .L81616                         | +2a2
        move.w  #0xffe0,0x2e(a6)                | +2a6
        move.w  0x28(a6),d0                     | +2ac
        asr.w   #0x2,d0                         | +2b0
        move.w  d0,0x28(a6)                     | +2b2
        move.w  0x74(a6),d0                     | +2b6
        asr.w   #0x1,d0                         | +2ba
        move.w  d0,0x2a(a6)                     | +2bc
        move.w  d0,0x74(a6)                     | +2c0
        lea     JmpToScheduler_081214(pc),a1    | +2c4
        move.l  a1,(a6)                         | +2c8
.L81616:
        jsr     0x28d70.l                       | +2ca
        movea.l #0xffffffff,a0                  | +2d0
        lea     0x2e3c4c.l,a0                   | +2d6
        jsr     0x5dd56.l                       | +2dc
        bcc.w   SetHandlerRts_081638            | +2e2

| ----------------------------------------------------------------------------
|  Squad_CmdrExitHop_08163a  @ $08163A  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_CmdrExitHop_08163a, "ax", @progbits
        .global Squad_CmdrExitHop_08163a
Squad_CmdrExitHop_08163a:
        move.w  #0x4,d1                         | +000
        jsr     0x236e.l                        | +004
        lea     0x2dd8b6.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L81656(pc),a1                  | +016
        move.l  a1,(a6)                         | +01a
.L81656:
        jsr     0x2783a.l                       | +01c
        jsr     0x28d70.l                       | +022
        bcc.w   SetHandlerRts_08166c            | +028

| ----------------------------------------------------------------------------
|  Squad_CmdrEscapeInit_08166e  @ $08166E  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_CmdrEscapeInit_08166e, "ax", @progbits
        .global Squad_CmdrEscapeInit_08166e
Squad_CmdrEscapeInit_08166e:
        move.w  #0x1022,d0                      | +000
        jsr     0x2352.l                        | +004
        .global Squad_CmdrEscapeNoCry_081678
Squad_CmdrEscapeNoCry_081678:
        move.w  #0x2000,d0                      | +00a
        jsr     0x28134.l                       | +00e
        andi.w  #0xffe3,0x38(a6)                | +014
        ori.w   #0x14,0x38(a6)                  | +01a
        jsr     0x13600.l                       | +020
        jmp     0x77f6a.l                       | +026

| ----------------------------------------------------------------------------
|  Sub_0008169A  @ $08169A  (196 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0008169A, "ax", @progbits
        .global Sub_0008169A
Sub_0008169A:
        jsr     0x5e1ea.l                       | +000
        move.l  a0,0x7a(a6)                     | +006
        clr.w   d2                              | +00a
        lea     0x2e3d18.l,a1                   | +00c
        cmpi.w  #0xd0,0x22(a0)                  | +012
        bgt.w   .L816d4                         | +018
        lea     0x2e3cf8.l,a1                   | +01c
        cmpi.w  #0x70,0x22(a0)                  | +022
        ble.w   .L816d4                         | +028
        jsr     0x5e9b6.l                       | +02c
        andi.w  #0x1,d0                         | +032
        lsl.w   #0x5,d0                         | +036
        move.w  d0,d2                           | +038
.L816d4:
        movea.l 0xc(a6),a0                      | +03a
        move.b  0x9b(a0),d0                     | +03e
        cmpi.b  #0x3,d0                         | +042
        bcs.w   .L816ee                         | +046
        move.l  d2,-(a7)                        | +04a
        jsr     0x5e9b6.l                       | +04c
        move.l  (a7)+,d2                        | +052
.L816ee:
        andi.w  #0x3,d0                         | +054
        move.w  d0,d3                           | +058
        addi.w  #0x9c,d0                        | +05a
        move.b  (a0,d0.w),d1                    | +05e
        ext.w   d1                              | +062
        move.w  d1,0x24(a6)                     | +064
        lsl.l   #0x3,d3                         | +068
        add.w   d3,d2                           | +06a
        move.w  (a1,d2.w),0x22(a6)              | +06c
        move.w  0x2(a1,d2.w),d1                 | +072
        add.w   d1,0x24(a6)                     | +076
        move.w  0x4(a1,d2.w),d1                 | +07a
        move.b  d1,0x3a(a6)                     | +07e
        move.w  0x6(a1,d2.w),d0                 | +082
        move.w  d0,0x78(a6)                     | +086
        add.w   d0,d0                           | +08a
        add.w   d0,d0                           | +08c
        lea     0x2e3d38.l,a0                   | +08e
        move.l  (a0,d0.w),0x90(a6)              | +094
        moveq   #0x0,d1                         | +09a
        move.w  d1,0x94(a6)                     | +09c
        move.b  d1,0x96(a6)                     | +0a0
        lea     0x2e3d50.l,a0                   | +0a4
        movea.l (a0,d0.w),a0                    | +0aa
        jsr     0x799de.l                       | +0ae
        move.w  d0,0x72(a6)                     | +0b4
        clr.b   0x7e(a6)                        | +0b8
        move.b  #0xff,0x7f(a6)                  | +0bc
        rts                                     | +0c2

| ----------------------------------------------------------------------------
|  SquadCurve_BellTable_08175e  @ $08175E  (64 B)
| ----------------------------------------------------------------------------
        .section .text.SquadCurve_BellTable_08175e, "ax", @progbits
        .global SquadCurve_BellTable_08175e
SquadCurve_BellTable_08175e:
|       tabla de datos $08175E..$08179E (curva bell velocidad-vs-angulo)
        .dc.w   0x0100,0x00f0,0x00e0,0x00d0,0x00c0,0x00b0,0x00a0,0x0090
        .dc.w   0x0080,0x0090,0x00a0,0x00b0,0x00c0,0x00d0,0x00e0,0x00f0
        .dc.w   0x0100,0x0110,0x0120,0x0130,0x0140,0x0150,0x0160,0x0170
        .dc.w   0x0180,0x0170,0x0160,0x0150,0x0140,0x0130,0x0120,0x0110

| ----------------------------------------------------------------------------
|  Sub_0008179E  @ $08179E  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0008179E, "ax", @progbits
        .global Sub_0008179E
Sub_0008179E:
        lea     SquadCurve_BellTable_08175e(pc),a0 | +000
        move.b  0x34(a6),d0                     | +004
        andi.l  #0xff,d0                        | +008
        lsr.l   #0x3,d0                         | +00e
        adda.l  d0,a0                           | +010
        adda.l  d0,a0                           | +012
        move.w  (a0),d0                         | +014
        move.w  0x72(a6),d1                     | +016
        muls.w  d1,d0                           | +01a
        asr.l   #0x8,d0                         | +01c
        move.w  d0,0x36(a6)                     | +01e
        move.b  0x34(a6),d0                     | +022
        andi.w  #0xff,d0                        | +026
        lsr.w   #0x4,d0                         | +02a

| ----------------------------------------------------------------------------
|  Squad_SpeedTailFrags_0817d0  @ $0817D0  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_SpeedTailFrags_0817d0, "ax", @progbits
        .global Squad_SpeedTailFrags_0817d0
Squad_SpeedTailFrags_0817d0:
        asl.w   0x36(a6)                        | +000
        rts                                     | +004
        .global Squad_SetLift40_0817d6
Squad_SetLift40_0817d6:
        move.w  #0x40,0x2e(a6)                  | +006
        rts                                     | +00c
        .global Squad_LeaSprite_0817de
Squad_LeaSprite_0817de:
        lea     0x2e2ff6.l,a0                   | +00e

| ----------------------------------------------------------------------------
|  Squad_LeaSprite_0817ec  @ $0817EC  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_LeaSprite_0817ec, "ax", @progbits
        .global Squad_LeaSprite_0817ec
Squad_LeaSprite_0817ec:
        lea     0x2e300a.l,a0                   | +000

| ----------------------------------------------------------------------------
|  Squad_LeaSprite_0817fa  @ $0817FA  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_LeaSprite_0817fa, "ax", @progbits
        .global Squad_LeaSprite_0817fa
Squad_LeaSprite_0817fa:
        lea     0x2e303a.l,a0                   | +000

| ----------------------------------------------------------------------------
|  Squad_LeaSprite_081808  @ $081808  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_LeaSprite_081808, "ax", @progbits
        .global Squad_LeaSprite_081808
Squad_LeaSprite_081808:
        lea     0x2e305c.l,a0                   | +000
