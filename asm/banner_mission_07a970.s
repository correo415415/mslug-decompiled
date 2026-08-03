| =====================================================================
| banner_mission_07a970.s — Wave ZZ
| Region: $07A970..$07BA28 (17 entradas, 4.216 B, byte-exacto)
| =====================================================================
|
| BANNER DE MISION: "MISSION START" / "MISSION COMPLETE"
| Las letras del rotulo caen una a una desde arriba y aterrizan en su
| posicion final; al cerrar, salen volando. Incluye las 4 rutinas de
| layout mas grandes decompiladas hasta ahora (640..968 B cada una).
|
| * BannerStart_Boot_07A970 / BannerComplete_Boot_07A9F0: tareas raiz de
|   cada rotulo. Inicializan el contenedor, eligen el layout segun la
|   escena ($106ECE==5 usa la variante "Final") y esperan a que el
|   contador de letras aterrizadas (+$21) llegue al total. COMPLETE
|   ademas dispara el sonido $20 al abrir y $1137 al completarse.
| * BannerLayout_Start_07ACD0 (640 B) / _StartFinal_07AF50 (830 B) /
|   _Complete_07B28E (778 B) / _CompleteFinal_07B598 (968 B): secuencias
|   lineales de spawn — una llamada al scheduler ($4AE) por LETRA, con
|   su indice de glifo (+$20), fila (+$21), orden de caida (+$74) y
|   posicion final X/Y (+$70/+$72). Los glifos indexan la tabla de
|   sprites $2DF684 (letras) / $2DF71C (digitos); las variantes "Final"
|   anaden la fila extra del numero de mision.
| * BannerLetter_Drop_07AAE0 / BannerDigit_Drop_07AB5C: cuenta atras por
|   letra (+$74) y caida con sonido $1BE al soltarse; el digito lee el
|   numero de escena de $106ECE en vez del indice de glifo.
| * BannerLetter_Tick_07ABD0 (cola compartida): seek + tick + settle de
|   cada letra en vuelo.
| * BannerLetter_Seek_07B960: dirige la letra hacia su posicion final
|   (delta<<6 como velocidad); _CheckSettle/_Approach: frenado cuando la
|   distancia ($5E23A) baja de $30; _Blink: parpadeo del sprite y sonido
|   de aterrizaje $1AA una sola vez (+$7A como latch).
| * Banner_Wait26_07AA78: espera 38 frames; Banner_Close_07AA94: marca
|   +$20=$FF, resetea $106ED2 y mata la tarea tras 40 frames.
| * Banner_Attach_07ABF4: engancha la letra al contenedor padre e
|   incrementa su contador de aterrizajes (+$21 del padre).
| * BannerLetter_FlyOff_07AC4A: salida — rampa un contador a $7FFF,
|   calcula el angulo hacia ($A0,$190) fuera de pantalla via $5E018,
|   lo invierte (+$7F) y convierte a velocidad con $13C0E.
|
| Mismo armazon de tareas del resto del juego: handlers auto-
| reemplazantes `lea next(pc),a1 ; move.l a1,(a6)` sobre el scheduler
| ($4AE alta / $518 baja), islas C SetTaskHandler/SetXN/ClearXN entre
| medias (5 defsyms mid-isla nuevos SetHandlerRts_07xxxx).
| =====================================================================

        .globl  BannerStart_Boot_07A970
        .type   BannerStart_Boot_07A970, @function
        .section .text.BannerStart_Boot_07A970, "ax", @progbits
BannerStart_Boot_07A970:
        jsr     0x4707e.l                              | +000
        move.b  #0x0, 0x7b(a6)                         | +006
        move.w  #0x0, 0x22(a6)                         | +00c
        move.w  #0x1ff, 0x24(a6)                       | +012
        move.w  #0xffff, 0x38(a6)                      | +018
        move.w  #0x0, 0x74(a6)                         | +01e
        move.b  0x106ece.l, d0                         | +024
        cmpi.b  #0x5, d0                               | +02a
        bne.w   .L7a9aa                                | +02e
        jsr     BannerLayout_StartFinal_07AF50(pc)     | +032
        bra.w   .L7a9ae                                | +036
.L7a9aa:
        jsr     BannerLayout_Start_07ACD0(pc)          | +03a
.L7a9ae:
        move.b  #0x0, 0x21(a6)                         | +03e
        move.b  #0x0, 0x20(a6)                         | +044
        lea     .L7a9c0(pc), a1                        | +04a
        move.l  a1, (a6)                               | +04e
.L7a9c0:
        move.b  0x106ece.l, d0                         | +050
        cmpi.b  #0x5, d0                               | +056
        bne.w   .L7a9d6                                | +05a
        move.b  #0x12, d0                              | +05e
        bra.w   .L7a9da                                | +062
.L7a9d6:
        move.b  #0xe, d0                               | +066
.L7a9da:
        cmp.b   0x21(a6), d0                           | +06a
        bne.w   SetHandlerRts_07a9ee                   | +06e
        jsr     0x4709e.l                              | +072
        .size   BannerStart_Boot_07A970, .-BannerStart_Boot_07A970

        .globl  BannerComplete_Boot_07A9F0
        .type   BannerComplete_Boot_07A9F0, @function
        .section .text.BannerComplete_Boot_07A9F0, "ax", @progbits
BannerComplete_Boot_07A9F0:
        move.w  #0x20, d0                              | +000
        jsr     0x2352.l                               | +004
        move.b  #0x1, 0x7b(a6)                         | +00a
        move.w  #0x0, 0x22(a6)                         | +010
        move.w  #0x1ff, 0x24(a6)                       | +016
        move.w  #0xffff, 0x38(a6)                      | +01c
        move.w  #0x0, 0x74(a6)                         | +022
        move.b  0x106ece.l, d0                         | +028
        cmpi.b  #0x5, d0                               | +02e
        bne.w   .L7aa2e                                | +032
        jsr     BannerLayout_CompleteFinal_07B598(pc)  | +036
        bra.w   .L7aa32                                | +03a
.L7aa2e:
        jsr     BannerLayout_Complete_07B28E(pc)       | +03e
.L7aa32:
        move.b  #0x0, 0x21(a6)                         | +042
        move.b  #0x0, 0x20(a6)                         | +048
        lea     .L7aa44(pc), a1                        | +04e
        move.l  a1, (a6)                               | +052
.L7aa44:
        move.b  0x106ece.l, d0                         | +054
        cmpi.b  #0x5, d0                               | +05a
        bne.w   .L7aa5a                                | +05e
        move.b  #0x15, d0                              | +062
        bra.w   .L7aa5e                                | +066
.L7aa5a:
        move.b  #0x11, d0                              | +06a
.L7aa5e:
        cmp.b   0x21(a6), d0                           | +06e
        bne.w   SetHandlerRts_07aa76                   | +072
        move.w  #0x1137, d0                            | +076
        jsr     0x2352.l                               | +07a
        .size   BannerComplete_Boot_07A9F0, .-BannerComplete_Boot_07A9F0

        .globl  Banner_Wait26_07AA78
        .type   Banner_Wait26_07AA78, @function
        .section .text.Banner_Wait26_07AA78, "ax", @progbits
Banner_Wait26_07AA78:
        lea     .L7aa7e(pc), a1                        | +000
        move.l  a1, (a6)                               | +004
.L7aa7e:
        addq.w  #0x1, 0x74(a6)                         | +006
        cmpi.w  #0x26, 0x74(a6)                        | +00a
        blt.w   SetHandlerRts_07aa92                   | +010
        .size   Banner_Wait26_07AA78, .-Banner_Wait26_07AA78

        .globl  Banner_Close_07AA94
        .type   Banner_Close_07AA94, @function
        .section .text.Banner_Close_07AA94, "ax", @progbits
Banner_Close_07AA94:
        move.b  #0xff, 0x20(a6)                        | +000
        cmpi.b  #0x0, 0x7b(a6)                         | +006
        bne.w   .L7aab4                                | +00c
        movea.l 0xc(a6), a1                            | +010
        clr.b   0x21(a1)                               | +014
        jmp     0x518.l                                | +018
        rts                                            | +01e
.L7aab4:
        move.w  #0x2, d0                               | +020
        jsr     0x5239e.l                              | +024
        move.w  #0x28, 0x74(a6)                        | +02a
        lea     .L7aaca(pc), a1                        | +030
        move.l  a1, (a6)                               | +034
.L7aaca:
        subq.w  #0x1, 0x74(a6)                         | +036
        bpl.w   .L7aade                                | +03a
        clr.b   0x106ed2.l                             | +03e
        jmp     0x518.l                                | +044
.L7aade:
        rts                                            | +04a
        .size   Banner_Close_07AA94, .-Banner_Close_07AA94

        .globl  BannerLetter_Drop_07AAE0
        .type   BannerLetter_Drop_07AAE0, @function
        .section .text.BannerLetter_Drop_07AAE0, "ax", @progbits
BannerLetter_Drop_07AAE0:
        move.w  0x74(a6), 0x76(a6)                     | +000
        lea     .L7aaec(pc), a1                        | +006
        move.l  a1, (a6)                               | +00a
.L7aaec:
        subq.w  #0x1, 0x74(a6)                         | +00c
        bmi.w   .L7aaf6                                | +010
        rts                                            | +014
.L7aaf6:
        move.w  #0x1be, d1                             | +016
        jsr     0x236e.l                               | +01a
        ori.b   #0x1, 0x3a(a6)                         | +020
        move.w  #0x1ff, 0x78(a6)                       | +026
        move.b  #0xff, 0x33(a6)                        | +02c
        move.b  #0x0, 0x7a(a6)                         | +032
        move.w  #0xffff, 0x38(a6)                      | +038
        jsr     0x267e2.l                              | +03e
        clr.l   d0                                     | +044
        move.b  0x20(a6), d0                           | +046
        cmpi.b  #0xd, d0                               | +04a
        bls.w   .L7ab36                                | +04e
        move.b  #0x0, d0                               | +052
.L7ab36:
        movea.l #0x2df684, a0                          | +056
        lsl.w   #0x2, d0                               | +05c
        movea.l (a0, d0.w), a0                         | +05e
        cmpa.l  #0xffffffff, a0                        | +062
        beq.w   .L7ab52                                | +068
        jsr     0x28cd4.l                              | +06c
.L7ab52:
        lea     BannerLetter_Tick_07ABD0(pc), a1       | +072
        move.l  a1, (a6)                               | +076
        bra.w   BannerLetter_Tick_07ABD0               | +078
        .size   BannerLetter_Drop_07AAE0, .-BannerLetter_Drop_07AAE0

        .globl  BannerDigit_Drop_07AB5C
        .type   BannerDigit_Drop_07AB5C, @function
        .section .text.BannerDigit_Drop_07AB5C, "ax", @progbits
BannerDigit_Drop_07AB5C:
        move.w  0x74(a6), 0x76(a6)                     | +000
        lea     .L7ab68(pc), a1                        | +006
        move.l  a1, (a6)                               | +00a
.L7ab68:
        subq.w  #0x1, 0x74(a6)                         | +00c
        bmi.w   .L7ab72                                | +010
        rts                                            | +014
.L7ab72:
        move.w  #0x1be, d1                             | +016
        jsr     0x236e.l                               | +01a
        ori.b   #0x1, 0x3a(a6)                         | +020
        move.w  #0x1ff, 0x78(a6)                       | +026
        move.b  #0xff, 0x33(a6)                        | +02c
        move.b  #0x0, 0x7a(a6)                         | +032
        jsr     0x267e2.l                              | +038
        clr.l   d0                                     | +03e
        move.b  0x106ece.l, d0                         | +040
        cmpi.b  #0x5, d0                               | +046
        bls.w   .L7abae                                | +04a
        move.b  #0x0, d0                               | +04e
.L7abae:
        movea.l #0x2df71c, a0                          | +052
        lsl.w   #0x2, d0                               | +058
        movea.l (a0, d0.w), a0                         | +05a
        cmpa.l  #0xffffffff, a0                        | +05e
        beq.w   .L7abca                                | +064
        jsr     0x28cd4.l                              | +068
.L7abca:
        lea     BannerLetter_Tick_07ABD0(pc), a1       | +06e
        move.l  a1, (a6)                               | +072
        .size   BannerDigit_Drop_07AB5C, .-BannerDigit_Drop_07AB5C

        .globl  BannerLetter_Tick_07ABD0
        .type   BannerLetter_Tick_07ABD0, @function
        .section .text.BannerLetter_Tick_07ABD0, "ax", @progbits
BannerLetter_Tick_07ABD0:
        jsr     BannerLetter_Seek_07B960(pc)           | +000
        jsr     0x28d70.l                              | +004
        jsr     BannerLetter_CheckSettle_07B9A4(pc)    | +00a
        bcc.w   SetHandlerRts_07abf2                   | +00e
        move.w  #0x0, d1                               | +012
        jsr     0x236e.l                               | +016
        .size   BannerLetter_Tick_07ABD0, .-BannerLetter_Tick_07ABD0

        .globl  Banner_Attach_07ABF4
        .type   Banner_Attach_07ABF4, @function
        .section .text.Banner_Attach_07ABF4, "ax", @progbits
Banner_Attach_07ABF4:
        move.w  0x18(a6), 0x14(a6)                     | +000
        movea.l 0xc(a6), a0                            | +006
        addq.b  #0x1, 0x21(a0)                         | +00a
        move.b  #0xff, 0x33(a6)                        | +00e
        move.b  #0xff, 0x32(a6)                        | +014
        lea     .L7ac14(pc), a1                        | +01a
        move.l  a1, (a6)                               | +01e
.L7ac14:
        cmpi.b  #0x0, 0x21(a6)                         | +020
        beq.w   .L7ac2e                                | +026
        movea.l 0xc(a6), a0                            | +02a
        move.w  0x74(a0), d0                           | +02e
        btst    #0x3, d0                               | +032
        bne.w   .L7ac34                                | +036
.L7ac2e:
        jsr     0x28d70.l                              | +03a
.L7ac34:
        movea.l 0xc(a6), a0                            | +040
        cmpi.b  #0xff, 0x20(a0)                        | +044
        bne.w   SetHandlerRts_07ac48                   | +04a
        .size   Banner_Attach_07ABF4, .-Banner_Attach_07ABF4

        .globl  BannerLetter_FlyOff_07AC4A
        .type   BannerLetter_FlyOff_07AC4A, @function
        .section .text.BannerLetter_FlyOff_07AC4A, "ax", @progbits
BannerLetter_FlyOff_07AC4A:
        move.w  #0x0, 0x36(a6)                         | +000
        lea     .L7ac56(pc), a1                        | +006
        move.l  a1, (a6)                               | +00a
.L7ac56:
        addi.w  #0x1ff, 0x36(a6)                       | +00c
        clr.l   d0                                     | +012
        move.w  0x36(a6), d0                           | +014
        btst    #0xf, d0                               | +018
        beq.w   .L7ac70                                | +01c
        move.w  #0x7fff, 0x36(a6)                      | +020
.L7ac70:
        move.w  #0xa0, d0                              | +026
        move.w  #0x190, d1                             | +02a
        sub.w   0x22(a6), d0                           | +02e
        sub.w   0x24(a6), d1                           | +032
        jsr     0x5e018.l                              | +036
        addi.b  #0x7f, d0                              | +03c
        andi.w  #0xff, d0                              | +040
        move.w  0x36(a6), d1                           | +044
        jsr     0x13c0e.l                              | +048
        move.w  d1, 0x28(a6)                           | +04e
        move.w  d2, 0x2a(a6)                           | +052
        bset    #0x6, 0x13(a6)                         | +056
        jsr     0x27cee.l                              | +05c
        jsr     0x28d70.l                              | +062
        movea.l #0xffffffff, a0                        | +068
        lea     0x2df734.l, a0                         | +06e
        jsr     0x5dd5c.l                              | +074
        bcc.w   .L7acce                                | +07a
        jmp     0x518.l                                | +07e
.L7acce:
        rts                                            | +084
        .size   BannerLetter_FlyOff_07AC4A, .-BannerLetter_FlyOff_07AC4A

        .globl  BannerLayout_Start_07ACD0
        .type   BannerLayout_Start_07ACD0, @function
        .section .text.BannerLayout_Start_07ACD0, "ax", @progbits
BannerLayout_Start_07ACD0:
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +000
        jsr     0x4ae.l                                | +004
        jsr     0x5dd02.l                              | +00a
        move.b  #0xc, 0x20(a0)                         | +010
        move.b  #0x1, 0x21(a0)                         | +016
        move.w  #0x18, 0x74(a0)                        | +01c
        move.w  #0xda, 0x70(a0)                        | +022
        move.w  #0x178, 0x72(a0)                       | +028
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +02e
        jsr     0x4ae.l                                | +032
        jsr     0x5dd02.l                              | +038
        move.b  #0xb, 0x20(a0)                         | +03e
        move.b  #0x1, 0x21(a0)                         | +044
        move.w  #0x16, 0x74(a0)                        | +04a
        move.w  #0xc4, 0x70(a0)                        | +050
        move.w  #0x178, 0x72(a0)                       | +056
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +05c
        jsr     0x4ae.l                                | +060
        jsr     0x5dd02.l                              | +066
        move.b  #0x6, 0x20(a0)                         | +06c
        move.b  #0x1, 0x21(a0)                         | +072
        move.w  #0x14, 0x74(a0)                        | +078
        move.w  #0xad, 0x70(a0)                        | +07e
        move.w  #0x178, 0x72(a0)                       | +084
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +08a
        jsr     0x4ae.l                                | +08e
        jsr     0x5dd02.l                              | +094
        move.b  #0x5, 0x20(a0)                         | +09a
        move.b  #0x1, 0x21(a0)                         | +0a0
        move.w  #0x12, 0x74(a0)                        | +0a6
        move.w  #0x94, 0x70(a0)                        | +0ac
        move.w  #0x178, 0x72(a0)                       | +0b2
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +0b8
        jsr     0x4ae.l                                | +0bc
        jsr     0x5dd02.l                              | +0c2
        move.b  #0xb, 0x20(a0)                         | +0c8
        move.b  #0x1, 0x21(a0)                         | +0ce
        move.w  #0x10, 0x74(a0)                        | +0d4
        move.w  #0x7e, 0x70(a0)                        | +0da
        move.w  #0x178, 0x72(a0)                       | +0e0
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +0e6
        jsr     0x4ae.l                                | +0ea
        jsr     0x5dd02.l                              | +0f0
        move.b  #0x2, 0x20(a0)                         | +0f6
        move.b  #0x1, 0x21(a0)                         | +0fc
        move.w  #0xe, 0x74(a0)                         | +102
        move.w  #0x64, 0x70(a0)                        | +108
        move.w  #0x178, 0x72(a0)                       | +10e
        lea     BannerDigit_Drop_07AB5C(pc), a1        | +114
        jsr     0x4ae.l                                | +118
        jsr     0x5dd02.l                              | +11e
        move.b  #0x0, 0x21(a0)                         | +124
        move.w  #0xe, 0x74(a0)                         | +12a
        move.w  #0xf0, 0x70(a0)                        | +130
        move.w  #0x1a8, 0x72(a0)                       | +136
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +13c
        jsr     0x4ae.l                                | +140
        jsr     0x5dd02.l                              | +146
        move.b  #0x4, 0x20(a0)                         | +14c
        move.b  #0x0, 0x21(a0)                         | +152
        move.w  #0xc, 0x74(a0)                         | +158
        move.w  #0xd0, 0x70(a0)                        | +15e
        move.w  #0x1a8, 0x72(a0)                       | +164
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +16a
        jsr     0x4ae.l                                | +16e
        jsr     0x5dd02.l                              | +174
        move.b  #0x3, 0x20(a0)                         | +17a
        move.b  #0x0, 0x21(a0)                         | +180
        move.w  #0xa, 0x74(a0)                         | +186
        move.w  #0xb7, 0x70(a0)                        | +18c
        move.w  #0x1a8, 0x72(a0)                       | +192
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +198
        jsr     0x4ae.l                                | +19c
        jsr     0x5dd02.l                              | +1a2
        move.b  #0x1, 0x20(a0)                         | +1a8
        move.b  #0x0, 0x21(a0)                         | +1ae
        move.w  #0x8, 0x74(a0)                         | +1b4
        move.w  #0xa4, 0x70(a0)                        | +1ba
        move.w  #0x1a8, 0x72(a0)                       | +1c0
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +1c6
        jsr     0x4ae.l                                | +1ca
        jsr     0x5dd02.l                              | +1d0
        move.b  #0x2, 0x20(a0)                         | +1d6
        move.b  #0x0, 0x21(a0)                         | +1dc
        move.w  #0x6, 0x74(a0)                         | +1e2
        move.w  #0x90, 0x70(a0)                        | +1e8
        move.w  #0x1a8, 0x72(a0)                       | +1ee
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +1f4
        jsr     0x4ae.l                                | +1f8
        jsr     0x5dd02.l                              | +1fe
        move.b  #0x2, 0x20(a0)                         | +204
        move.b  #0x0, 0x21(a0)                         | +20a
        move.w  #0x4, 0x74(a0)                         | +210
        move.w  #0x77, 0x70(a0)                        | +216
        move.w  #0x1a8, 0x72(a0)                       | +21c
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +222
        jsr     0x4ae.l                                | +226
        jsr     0x5dd02.l                              | +22c
        move.b  #0x1, 0x20(a0)                         | +232
        move.b  #0x0, 0x21(a0)                         | +238
        move.w  #0x2, 0x74(a0)                         | +23e
        move.w  #0x64, 0x70(a0)                        | +244
        move.w  #0x1a8, 0x72(a0)                       | +24a
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +250
        jsr     0x4ae.l                                | +254
        jsr     0x5dd02.l                              | +25a
        move.b  #0x0, 0x20(a0)                         | +260
        move.b  #0x0, 0x21(a0)                         | +266
        move.w  #0x0, 0x74(a0)                         | +26c
        move.w  #0x50, 0x70(a0)                        | +272
        move.w  #0x1a8, 0x72(a0)                       | +278
        rts                                            | +27e
        .size   BannerLayout_Start_07ACD0, .-BannerLayout_Start_07ACD0

        .globl  BannerLayout_StartFinal_07AF50
        .type   BannerLayout_StartFinal_07AF50, @function
        .section .text.BannerLayout_StartFinal_07AF50, "ax", @progbits
BannerLayout_StartFinal_07AF50:
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +000
        jsr     0x4ae.l                                | +004
        jsr     0x5dd02.l                              | +00a
        move.b  #0xc, 0x20(a0)                         | +010
        move.b  #0x1, 0x21(a0)                         | +016
        move.w  #0x22, 0x74(a0)                        | +01c
        move.w  #0xda, 0x70(a0)                        | +022
        move.w  #0x168, 0x72(a0)                       | +028
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +02e
        jsr     0x4ae.l                                | +032
        jsr     0x5dd02.l                              | +038
        move.b  #0xb, 0x20(a0)                         | +03e
        move.b  #0x1, 0x21(a0)                         | +044
        move.w  #0x20, 0x74(a0)                        | +04a
        move.w  #0xc6, 0x70(a0)                        | +050
        move.w  #0x168, 0x72(a0)                       | +056
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +05c
        jsr     0x4ae.l                                | +060
        jsr     0x5dd02.l                              | +066
        move.b  #0x6, 0x20(a0)                         | +06c
        move.b  #0x1, 0x21(a0)                         | +072
        move.w  #0x1e, 0x74(a0)                        | +078
        move.w  #0xaf, 0x70(a0)                        | +07e
        move.w  #0x168, 0x72(a0)                       | +084
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +08a
        jsr     0x4ae.l                                | +08e
        jsr     0x5dd02.l                              | +094
        move.b  #0x5, 0x20(a0)                         | +09a
        move.b  #0x1, 0x21(a0)                         | +0a0
        move.w  #0x1c, 0x74(a0)                        | +0a6
        move.w  #0x96, 0x70(a0)                        | +0ac
        move.w  #0x168, 0x72(a0)                       | +0b2
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +0b8
        jsr     0x4ae.l                                | +0bc
        jsr     0x5dd02.l                              | +0c2
        move.b  #0xb, 0x20(a0)                         | +0c8
        move.b  #0x1, 0x21(a0)                         | +0ce
        move.w  #0x1a, 0x74(a0)                        | +0d4
        move.w  #0x80, 0x70(a0)                        | +0da
        move.w  #0x168, 0x72(a0)                       | +0e0
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +0e6
        jsr     0x4ae.l                                | +0ea
        jsr     0x5dd02.l                              | +0f0
        move.b  #0x2, 0x20(a0)                         | +0f6
        move.b  #0x1, 0x21(a0)                         | +0fc
        move.w  #0x18, 0x74(a0)                        | +102
        move.w  #0x66, 0x70(a0)                        | +108
        move.w  #0x168, 0x72(a0)                       | +10e
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +114
        jsr     0x4ae.l                                | +118
        jsr     0x5dd02.l                              | +11e
        move.b  #0x4, 0x20(a0)                         | +124
        move.b  #0x0, 0x21(a0)                         | +12a
        move.w  #0x16, 0x74(a0)                        | +130
        move.w  #0xe0, 0x70(a0)                        | +136
        move.w  #0x198, 0x72(a0)                       | +13c
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +142
        jsr     0x4ae.l                                | +146
        jsr     0x5dd02.l                              | +14c
        move.b  #0x3, 0x20(a0)                         | +152
        move.b  #0x0, 0x21(a0)                         | +158
        move.w  #0x14, 0x74(a0)                        | +15e
        move.w  #0xc7, 0x70(a0)                        | +164
        move.w  #0x198, 0x72(a0)                       | +16a
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +170
        jsr     0x4ae.l                                | +174
        jsr     0x5dd02.l                              | +17a
        move.b  #0x1, 0x20(a0)                         | +180
        move.b  #0x0, 0x21(a0)                         | +186
        move.w  #0x12, 0x74(a0)                        | +18c
        move.w  #0xb4, 0x70(a0)                        | +192
        move.w  #0x198, 0x72(a0)                       | +198
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +19e
        jsr     0x4ae.l                                | +1a2
        jsr     0x5dd02.l                              | +1a8
        move.b  #0x2, 0x20(a0)                         | +1ae
        move.b  #0x0, 0x21(a0)                         | +1b4
        move.w  #0x10, 0x74(a0)                        | +1ba
        move.w  #0xa0, 0x70(a0)                        | +1c0
        move.w  #0x198, 0x72(a0)                       | +1c6
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +1cc
        jsr     0x4ae.l                                | +1d0
        jsr     0x5dd02.l                              | +1d6
        move.b  #0x2, 0x20(a0)                         | +1dc
        move.b  #0x0, 0x21(a0)                         | +1e2
        move.w  #0xe, 0x74(a0)                         | +1e8
        move.w  #0x87, 0x70(a0)                        | +1ee
        move.w  #0x198, 0x72(a0)                       | +1f4
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +1fa
        jsr     0x4ae.l                                | +1fe
        jsr     0x5dd02.l                              | +204
        move.b  #0x1, 0x20(a0)                         | +20a
        move.b  #0x0, 0x21(a0)                         | +210
        move.w  #0xc, 0x74(a0)                         | +216
        move.w  #0x74, 0x70(a0)                        | +21c
        move.w  #0x198, 0x72(a0)                       | +222
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +228
        jsr     0x4ae.l                                | +22c
        jsr     0x5dd02.l                              | +232
        move.b  #0x0, 0x20(a0)                         | +238
        move.b  #0x0, 0x21(a0)                         | +23e
        move.w  #0xa, 0x74(a0)                         | +244
        move.w  #0x60, 0x70(a0)                        | +24a
        move.w  #0x198, 0x72(a0)                       | +250
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +256
        jsr     0x4ae.l                                | +25a
        jsr     0x5dd02.l                              | +260
        move.b  #0x9, 0x20(a0)                         | +266
        move.b  #0x0, 0x21(a0)                         | +26c
        move.w  #0x8, 0x74(a0)                         | +272
        move.w  #0xcd, 0x70(a0)                        | +278
        move.w  #0x1b8, 0x72(a0)                       | +27e
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +284
        jsr     0x4ae.l                                | +288
        jsr     0x5dd02.l                              | +28e
        move.b  #0x5, 0x20(a0)                         | +294
        move.b  #0x0, 0x21(a0)                         | +29a
        move.w  #0x6, 0x74(a0)                         | +2a0
        move.w  #0xb4, 0x70(a0)                        | +2a6
        move.w  #0x1b8, 0x72(a0)                       | +2ac
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +2b2
        jsr     0x4ae.l                                | +2b6
        jsr     0x5dd02.l                              | +2bc
        move.b  #0x4, 0x20(a0)                         | +2c2
        move.b  #0x0, 0x21(a0)                         | +2c8
        move.w  #0x4, 0x74(a0)                         | +2ce
        move.w  #0x9c, 0x70(a0)                        | +2d4
        move.w  #0x1b8, 0x72(a0)                       | +2da
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +2e0
        jsr     0x4ae.l                                | +2e4
        jsr     0x5dd02.l                              | +2ea
        move.b  #0x1, 0x20(a0)                         | +2f0
        move.b  #0x0, 0x21(a0)                         | +2f6
        move.w  #0x2, 0x74(a0)                         | +2fc
        move.w  #0x89, 0x70(a0)                        | +302
        move.w  #0x1b8, 0x72(a0)                       | +308
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +30e
        jsr     0x4ae.l                                | +312
        jsr     0x5dd02.l                              | +318
        move.b  #0xd, 0x20(a0)                         | +31e
        move.b  #0x0, 0x21(a0)                         | +324
        move.w  #0x0, 0x74(a0)                         | +32a
        move.w  #0x76, 0x70(a0)                        | +330
        move.w  #0x1b8, 0x72(a0)                       | +336
        rts                                            | +33c
        .size   BannerLayout_StartFinal_07AF50, .-BannerLayout_StartFinal_07AF50

        .globl  BannerLayout_Complete_07B28E
        .type   BannerLayout_Complete_07B28E, @function
        .section .text.BannerLayout_Complete_07B28E, "ax", @progbits
BannerLayout_Complete_07B28E:
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +000
        jsr     0x4ae.l                                | +004
        jsr     0x5dd02.l                              | +00a
        move.b  #0xc, 0x20(a0)                         | +010
        move.b  #0x1, 0x21(a0)                         | +016
        move.w  #0x20, 0x74(a0)                        | +01c
        move.w  #0x102, 0x70(a0)                       | +022
        move.w  #0x178, 0x72(a0)                       | +028
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +02e
        jsr     0x4ae.l                                | +032
        jsr     0x5dd02.l                              | +038
        move.b  #0xa, 0x20(a0)                         | +03e
        move.b  #0x1, 0x21(a0)                         | +044
        move.w  #0x1e, 0x74(a0)                        | +04a
        move.w  #0xec, 0x70(a0)                        | +050
        move.w  #0x178, 0x72(a0)                       | +056
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +05c
        jsr     0x4ae.l                                | +060
        jsr     0x5dd02.l                              | +066
        move.b  #0xb, 0x20(a0)                         | +06c
        move.b  #0x1, 0x21(a0)                         | +072
        move.w  #0x1c, 0x74(a0)                        | +078
        move.w  #0xd4, 0x70(a0)                        | +07e
        move.w  #0x178, 0x72(a0)                       | +084
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +08a
        jsr     0x4ae.l                                | +08e
        jsr     0x5dd02.l                              | +094
        move.b  #0xa, 0x20(a0)                         | +09a
        move.b  #0x1, 0x21(a0)                         | +0a0
        move.w  #0x1a, 0x74(a0)                        | +0a6
        move.w  #0xbc, 0x70(a0)                        | +0ac
        move.w  #0x178, 0x72(a0)                       | +0b2
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +0b8
        jsr     0x4ae.l                                | +0bc
        jsr     0x5dd02.l                              | +0c2
        move.b  #0x9, 0x20(a0)                         | +0c8
        move.b  #0x1, 0x21(a0)                         | +0ce
        move.w  #0x18, 0x74(a0)                        | +0d4
        move.w  #0xa5, 0x70(a0)                        | +0da
        move.w  #0x178, 0x72(a0)                       | +0e0
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +0e6
        jsr     0x4ae.l                                | +0ea
        jsr     0x5dd02.l                              | +0f0
        move.b  #0x8, 0x20(a0)                         | +0f6
        move.b  #0x1, 0x21(a0)                         | +0fc
        move.w  #0x16, 0x74(a0)                        | +102
        move.w  #0x8d, 0x70(a0)                        | +108
        move.w  #0x178, 0x72(a0)                       | +10e
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +114
        jsr     0x4ae.l                                | +118
        jsr     0x5dd02.l                              | +11e
        move.b  #0x0, 0x20(a0)                         | +124
        move.b  #0x1, 0x21(a0)                         | +12a
        move.w  #0x14, 0x74(a0)                        | +130
        move.w  #0x74, 0x70(a0)                        | +136
        move.w  #0x178, 0x72(a0)                       | +13c
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +142
        jsr     0x4ae.l                                | +146
        jsr     0x5dd02.l                              | +14c
        move.b  #0x3, 0x20(a0)                         | +152
        move.b  #0x1, 0x21(a0)                         | +158
        move.w  #0x12, 0x74(a0)                        | +15e
        move.w  #0x5b, 0x70(a0)                        | +164
        move.w  #0x178, 0x72(a0)                       | +16a
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +170
        jsr     0x4ae.l                                | +174
        jsr     0x5dd02.l                              | +17a
        move.b  #0x7, 0x20(a0)                         | +180
        move.b  #0x1, 0x21(a0)                         | +186
        move.w  #0x10, 0x74(a0)                        | +18c
        move.w  #0x42, 0x70(a0)                        | +192
        move.w  #0x178, 0x72(a0)                       | +198
        lea     BannerDigit_Drop_07AB5C(pc), a1        | +19e
        jsr     0x4ae.l                                | +1a2
        jsr     0x5dd02.l                              | +1a8
        move.b  #0x0, 0x21(a0)                         | +1ae
        move.w  #0xe, 0x74(a0)                         | +1b4
        move.w  #0xf0, 0x70(a0)                        | +1ba
        move.w  #0x1a8, 0x72(a0)                       | +1c0
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +1c6
        jsr     0x4ae.l                                | +1ca
        jsr     0x5dd02.l                              | +1d0
        move.b  #0x4, 0x20(a0)                         | +1d6
        move.b  #0x0, 0x21(a0)                         | +1dc
        move.w  #0xc, 0x74(a0)                         | +1e2
        move.w  #0xd0, 0x70(a0)                        | +1e8
        move.w  #0x1a8, 0x72(a0)                       | +1ee
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +1f4
        jsr     0x4ae.l                                | +1f8
        jsr     0x5dd02.l                              | +1fe
        move.b  #0x3, 0x20(a0)                         | +204
        move.b  #0x0, 0x21(a0)                         | +20a
        move.w  #0xa, 0x74(a0)                         | +210
        move.w  #0xb7, 0x70(a0)                        | +216
        move.w  #0x1a8, 0x72(a0)                       | +21c
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +222
        jsr     0x4ae.l                                | +226
        jsr     0x5dd02.l                              | +22c
        move.b  #0x1, 0x20(a0)                         | +232
        move.b  #0x0, 0x21(a0)                         | +238
        move.w  #0x8, 0x74(a0)                         | +23e
        move.w  #0xa4, 0x70(a0)                        | +244
        move.w  #0x1a8, 0x72(a0)                       | +24a
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +250
        jsr     0x4ae.l                                | +254
        jsr     0x5dd02.l                              | +25a
        move.b  #0x2, 0x20(a0)                         | +260
        move.b  #0x0, 0x21(a0)                         | +266
        move.w  #0x6, 0x74(a0)                         | +26c
        move.w  #0x90, 0x70(a0)                        | +272
        move.w  #0x1a8, 0x72(a0)                       | +278
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +27e
        jsr     0x4ae.l                                | +282
        jsr     0x5dd02.l                              | +288
        move.b  #0x2, 0x20(a0)                         | +28e
        move.b  #0x0, 0x21(a0)                         | +294
        move.w  #0x4, 0x74(a0)                         | +29a
        move.w  #0x77, 0x70(a0)                        | +2a0
        move.w  #0x1a8, 0x72(a0)                       | +2a6
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +2ac
        jsr     0x4ae.l                                | +2b0
        jsr     0x5dd02.l                              | +2b6
        move.b  #0x1, 0x20(a0)                         | +2bc
        move.b  #0x0, 0x21(a0)                         | +2c2
        move.w  #0x2, 0x74(a0)                         | +2c8
        move.w  #0x64, 0x70(a0)                        | +2ce
        move.w  #0x1a8, 0x72(a0)                       | +2d4
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +2da
        jsr     0x4ae.l                                | +2de
        jsr     0x5dd02.l                              | +2e4
        move.b  #0x0, 0x20(a0)                         | +2ea
        move.b  #0x0, 0x21(a0)                         | +2f0
        move.w  #0x0, 0x74(a0)                         | +2f6
        move.w  #0x50, 0x70(a0)                        | +2fc
        move.w  #0x1a8, 0x72(a0)                       | +302
        rts                                            | +308
        .size   BannerLayout_Complete_07B28E, .-BannerLayout_Complete_07B28E

        .globl  BannerLayout_CompleteFinal_07B598
        .type   BannerLayout_CompleteFinal_07B598, @function
        .section .text.BannerLayout_CompleteFinal_07B598, "ax", @progbits
BannerLayout_CompleteFinal_07B598:
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +000
        jsr     0x4ae.l                                | +004
        jsr     0x5dd02.l                              | +00a
        move.b  #0xc, 0x20(a0)                         | +010
        move.b  #0x1, 0x21(a0)                         | +016
        move.w  #0x28, 0x74(a0)                        | +01c
        move.w  #0x104, 0x70(a0)                       | +022
        move.w  #0x168, 0x72(a0)                       | +028
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +02e
        jsr     0x4ae.l                                | +032
        jsr     0x5dd02.l                              | +038
        move.b  #0xa, 0x20(a0)                         | +03e
        move.b  #0x1, 0x21(a0)                         | +044
        move.w  #0x26, 0x74(a0)                        | +04a
        move.w  #0xee, 0x70(a0)                        | +050
        move.w  #0x168, 0x72(a0)                       | +056
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +05c
        jsr     0x4ae.l                                | +060
        jsr     0x5dd02.l                              | +066
        move.b  #0xb, 0x20(a0)                         | +06c
        move.b  #0x1, 0x21(a0)                         | +072
        move.w  #0x24, 0x74(a0)                        | +078
        move.w  #0xd6, 0x70(a0)                        | +07e
        move.w  #0x168, 0x72(a0)                       | +084
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +08a
        jsr     0x4ae.l                                | +08e
        jsr     0x5dd02.l                              | +094
        move.b  #0xa, 0x20(a0)                         | +09a
        move.b  #0x1, 0x21(a0)                         | +0a0
        move.w  #0x22, 0x74(a0)                        | +0a6
        move.w  #0xbe, 0x70(a0)                        | +0ac
        move.w  #0x168, 0x72(a0)                       | +0b2
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +0b8
        jsr     0x4ae.l                                | +0bc
        jsr     0x5dd02.l                              | +0c2
        move.b  #0x9, 0x20(a0)                         | +0c8
        move.b  #0x1, 0x21(a0)                         | +0ce
        move.w  #0x20, 0x74(a0)                        | +0d4
        move.w  #0xa7, 0x70(a0)                        | +0da
        move.w  #0x168, 0x72(a0)                       | +0e0
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +0e6
        jsr     0x4ae.l                                | +0ea
        jsr     0x5dd02.l                              | +0f0
        move.b  #0x8, 0x20(a0)                         | +0f6
        move.b  #0x1, 0x21(a0)                         | +0fc
        move.w  #0x1e, 0x74(a0)                        | +102
        move.w  #0x8f, 0x70(a0)                        | +108
        move.w  #0x168, 0x72(a0)                       | +10e
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +114
        jsr     0x4ae.l                                | +118
        jsr     0x5dd02.l                              | +11e
        move.b  #0x0, 0x20(a0)                         | +124
        move.b  #0x1, 0x21(a0)                         | +12a
        move.w  #0x1c, 0x74(a0)                        | +130
        move.w  #0x76, 0x70(a0)                        | +136
        move.w  #0x168, 0x72(a0)                       | +13c
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +142
        jsr     0x4ae.l                                | +146
        jsr     0x5dd02.l                              | +14c
        move.b  #0x3, 0x20(a0)                         | +152
        move.b  #0x1, 0x21(a0)                         | +158
        move.w  #0x1a, 0x74(a0)                        | +15e
        move.w  #0x5d, 0x70(a0)                        | +164
        move.w  #0x168, 0x72(a0)                       | +16a
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +170
        jsr     0x4ae.l                                | +174
        jsr     0x5dd02.l                              | +17a
        move.b  #0x7, 0x20(a0)                         | +180
        move.b  #0x1, 0x21(a0)                         | +186
        move.w  #0x18, 0x74(a0)                        | +18c
        move.w  #0x44, 0x70(a0)                        | +192
        move.w  #0x168, 0x72(a0)                       | +198
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +19e
        jsr     0x4ae.l                                | +1a2
        jsr     0x5dd02.l                              | +1a8
        move.b  #0x4, 0x20(a0)                         | +1ae
        move.b  #0x0, 0x21(a0)                         | +1b4
        move.w  #0x16, 0x74(a0)                        | +1ba
        move.w  #0xe0, 0x70(a0)                        | +1c0
        move.w  #0x198, 0x72(a0)                       | +1c6
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +1cc
        jsr     0x4ae.l                                | +1d0
        jsr     0x5dd02.l                              | +1d6
        move.b  #0x3, 0x20(a0)                         | +1dc
        move.b  #0x0, 0x21(a0)                         | +1e2
        move.w  #0x14, 0x74(a0)                        | +1e8
        move.w  #0xc7, 0x70(a0)                        | +1ee
        move.w  #0x198, 0x72(a0)                       | +1f4
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +1fa
        jsr     0x4ae.l                                | +1fe
        jsr     0x5dd02.l                              | +204
        move.b  #0x1, 0x20(a0)                         | +20a
        move.b  #0x0, 0x21(a0)                         | +210
        move.w  #0x12, 0x74(a0)                        | +216
        move.w  #0xb4, 0x70(a0)                        | +21c
        move.w  #0x198, 0x72(a0)                       | +222
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +228
        jsr     0x4ae.l                                | +22c
        jsr     0x5dd02.l                              | +232
        move.b  #0x2, 0x20(a0)                         | +238
        move.b  #0x0, 0x21(a0)                         | +23e
        move.w  #0x10, 0x74(a0)                        | +244
        move.w  #0xa0, 0x70(a0)                        | +24a
        move.w  #0x198, 0x72(a0)                       | +250
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +256
        jsr     0x4ae.l                                | +25a
        jsr     0x5dd02.l                              | +260
        move.b  #0x2, 0x20(a0)                         | +266
        move.b  #0x0, 0x21(a0)                         | +26c
        move.w  #0xe, 0x74(a0)                         | +272
        move.w  #0x87, 0x70(a0)                        | +278
        move.w  #0x198, 0x72(a0)                       | +27e
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +284
        jsr     0x4ae.l                                | +288
        jsr     0x5dd02.l                              | +28e
        move.b  #0x1, 0x20(a0)                         | +294
        move.b  #0x0, 0x21(a0)                         | +29a
        move.w  #0xc, 0x74(a0)                         | +2a0
        move.w  #0x74, 0x70(a0)                        | +2a6
        move.w  #0x198, 0x72(a0)                       | +2ac
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +2b2
        jsr     0x4ae.l                                | +2b6
        jsr     0x5dd02.l                              | +2bc
        move.b  #0x0, 0x20(a0)                         | +2c2
        move.b  #0x0, 0x21(a0)                         | +2c8
        move.w  #0xa, 0x74(a0)                         | +2ce
        move.w  #0x60, 0x70(a0)                        | +2d4
        move.w  #0x198, 0x72(a0)                       | +2da
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +2e0
        jsr     0x4ae.l                                | +2e4
        jsr     0x5dd02.l                              | +2ea
        move.b  #0x9, 0x20(a0)                         | +2f0
        move.b  #0x0, 0x21(a0)                         | +2f6
        move.w  #0x8, 0x74(a0)                         | +2fc
        move.w  #0xcd, 0x70(a0)                        | +302
        move.w  #0x1b8, 0x72(a0)                       | +308
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +30e
        jsr     0x4ae.l                                | +312
        jsr     0x5dd02.l                              | +318
        move.b  #0x5, 0x20(a0)                         | +31e
        move.b  #0x0, 0x21(a0)                         | +324
        move.w  #0x6, 0x74(a0)                         | +32a
        move.w  #0xb4, 0x70(a0)                        | +330
        move.w  #0x1b8, 0x72(a0)                       | +336
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +33c
        jsr     0x4ae.l                                | +340
        jsr     0x5dd02.l                              | +346
        move.b  #0x4, 0x20(a0)                         | +34c
        move.b  #0x0, 0x21(a0)                         | +352
        move.w  #0x4, 0x74(a0)                         | +358
        move.w  #0x9c, 0x70(a0)                        | +35e
        move.w  #0x1b8, 0x72(a0)                       | +364
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +36a
        jsr     0x4ae.l                                | +36e
        jsr     0x5dd02.l                              | +374
        move.b  #0x1, 0x20(a0)                         | +37a
        move.b  #0x0, 0x21(a0)                         | +380
        move.w  #0x2, 0x74(a0)                         | +386
        move.w  #0x89, 0x70(a0)                        | +38c
        move.w  #0x1b8, 0x72(a0)                       | +392
        lea     BannerLetter_Drop_07AAE0(pc), a1       | +398
        jsr     0x4ae.l                                | +39c
        jsr     0x5dd02.l                              | +3a2
        move.b  #0xd, 0x20(a0)                         | +3a8
        move.b  #0x0, 0x21(a0)                         | +3ae
        move.w  #0x0, 0x74(a0)                         | +3b4
        move.w  #0x76, 0x70(a0)                        | +3ba
        move.w  #0x1b8, 0x72(a0)                       | +3c0
        rts                                            | +3c6
        .size   BannerLayout_CompleteFinal_07B598, .-BannerLayout_CompleteFinal_07B598

        .globl  BannerLetter_Seek_07B960
        .type   BannerLetter_Seek_07B960, @function
        .section .text.BannerLetter_Seek_07B960, "ax", @progbits
BannerLetter_Seek_07B960:
        move.w  0x70(a6), d0                           | +000
        move.w  0x72(a6), d1                           | +004
        sub.w   0x22(a6), d0                           | +008
        sub.w   0x24(a6), d1                           | +00c
        cmpi.w  #0x0, d0                               | +010
        bne.w   .L7b980                                | +014
        cmpi.w  #0x0, d1                               | +018
        beq.w   SetXN_07b99e                           | +01c
.L7b980:
        asl.w   #0x6, d0                               | +020
        asl.w   #0x6, d1                               | +022
        move.w  d0, 0x28(a6)                           | +024
        move.w  d1, 0x2a(a6)                           | +028
        bset    #0x6, 0x13(a6)                         | +02c
        jsr     0x27cee.l                              | +032
        .size   BannerLetter_Seek_07B960, .-BannerLetter_Seek_07B960

        .globl  BannerLetter_CheckSettle_07B9A4
        .type   BannerLetter_CheckSettle_07B9A4, @function
        .section .text.BannerLetter_CheckSettle_07B9A4, "ax", @progbits
BannerLetter_CheckSettle_07B9A4:
        cmpi.w  #0x0, 0x78(a6)                         | +000
        bne.w   BannerLetter_Approach_07B9B4           | +006
        .size   BannerLetter_CheckSettle_07B9A4, .-BannerLetter_CheckSettle_07B9A4

        .globl  BannerLetter_Approach_07B9B4
        .type   BannerLetter_Approach_07B9B4, @function
        .section .text.BannerLetter_Approach_07B9B4, "ax", @progbits
BannerLetter_Approach_07B9B4:
        move.w  0x70(a6), d2                           | +000
        move.w  0x72(a6), d3                           | +004
        move.w  0x22(a6), d0                           | +008
        move.w  0x24(a6), d1                           | +00c
        jsr     0x5e23a.l                              | +010
        cmpi.w  #0x30, d0                              | +016
        bgt.w   BannerLetter_Blink_07B9EA              | +01a
        move.w  #0x18, d0                              | +01e
        sub.w   d0, 0x78(a6)                           | +022
        bpl.w   BannerLetter_Blink_07B9EA              | +026
        move.w  #0x0, 0x78(a6)                         | +02a
        .size   BannerLetter_Approach_07B9B4, .-BannerLetter_Approach_07B9B4

        .globl  BannerLetter_Blink_07B9EA
        .type   BannerLetter_Blink_07B9EA, @function
        .section .text.BannerLetter_Blink_07B9EA, "ax", @progbits
BannerLetter_Blink_07B9EA:
        move.w  0x78(a6), d0                           | +000
        btst    #0x8, d0                               | +004
        bne.w   .L7b9f8                                | +008
        neg.b   d0                                     | +00c
.L7b9f8:
        move.b  d0, 0x32(a6)                           | +00e
        move.w  0x78(a6), d0                           | +012
        btst    #0x8, d0                               | +016
        bne.w   ClearXN_07ba28                         | +01a
        cmpi.b  #0x0, 0x7a(a6)                         | +01e
        bne.w   ClearXN_07ba28                         | +024
        move.w  #0x1aa, d1                             | +028
        jsr     0x236e.l                               | +02c
        move.b  #0xff, 0x7a(a6)                        | +032
        move.b  #0x0, 0x3a(a6)                         | +038
        .size   BannerLetter_Blink_07B9EA, .-BannerLetter_Blink_07B9EA

