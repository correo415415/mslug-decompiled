| =============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave GGG — Módulo de miniboss con secuencia de estados
|  Región: $083262..$083BDA  (2,374 B, 24 entradas, 6 huecos cerrados)
| =============================================================================
|
|  Máquina de estados de un miniboss (jingles $A4/$A5/$A7/$AA/$AB, sprites
|  en $2E6Cxx/$2E6Dxx/$2E6Exx). Estructura:
|
|   * $83262..$832E0 — prefijos/helpers cortos: LeaList_083262 (prefijo del
|     thunk JsrAbsThunk_083268), Sub_00083270 (setea $10A2D0=4/$10A2D1=0 y
|     +$20=$77 en el padre), Sub_0008328C (monta par $2E5ADC/$2E5A10 via
|     $77C7E + snd $102F, cae en el thunk $832AE), Sub_000832B6 (carga
|     paleta/lista $2E545E[+$5C] en +$4C, cae en el thunk $832C8) y
|     Sub_000832D0 (compara prioridad +$10 con el sibling +$8; cae en las
|     islas ClearXN_0832e0/SetXN_0832e6).
|   * $832EC..$834A4 — tres variantes de entrada (snd $A4, sprite $2E6CDC,
|     drift +-$80 según x>=$A0): con probe $27CEE, con probe $2783A y
|     empuje aleatorio cada 4 frames ($5E9B6/$5E9E4), y con helper
|     Sub_00086050 (hueco futuro).
|   * $834A4..$83612 — fase de combate: snd $A7, sprite $2E6C00, limpia
|     $10E39A, dispara blitter de fila ($43FAC con lista $2EAC8C, snd
|     $1037), muere hacia TaskHandler_08354e (score $100 via $51A28,
|     sprite $2E6C16) o TaskHandler_083596 (snd $AB, sprite $2E6D28).
|   * $835F2..$83926 — secuencia de huida en 3 variantes (+$21=0/1/2,
|     sprites $2E6D38/$2E6DA4) que convergen en TaskCont_08364e (global
|     interno): snd $AA, timers +$66/+$70, giro $8000, spawn de pareja
|     ($2E9A1E/$2E9A30 via $77C7E), blitter $2EACA0/$2EACAC, retroceso
|     -$3 con vel +-$40, flash $F0 ($5E722), snd $1054 en t=$50 y
|     transformación final ($2E987E, 30 frames).
|   * $8396A..$83B84 — variantes con protecciones: snd $A5, sprite $2E6E10
|     con parpadeo +$44 y timer aleatorio ($5E9E4+$23) reinstalando
|     TaskHandler_0839a2 (global interno); versiones acorazadas $8000
|     (sprites $2E6EBC/$2E6ECC) y spawner de hijo $83B92 con snd $A7 y
|     helper Sub_00086076 (hueco futuro).
|   * $83B92..$83BDA — handler del hijo: copia x/y del padre cada frame y
|     se autodestruye si x<-$80 (cae en el thunk JsrAbsThunk_083bda).
|
|  Los bcc.w colgantes apuntan a los RTS internos de las islas C contiguas
|  (Jsr5B6Rts_083b90 en +12, JsrAbsRts_083be0 en +6).
|
|  Verificación: cada sección .text.<Sym> se coloca en su dirección CPU
|  absoluta y reensambla byte-exacta contra build/mslug_prom.bin
|  (MD5 816b3f74c76b3373993407615f1850fe).
| =============================================================================

        .text

| ----------------------------------------------------------------------------
|  LeaList_083262  @ $083262  (6 B)
| ----------------------------------------------------------------------------
        .section .text.LeaList_083262, "ax", @progbits
        .global LeaList_083262
LeaList_083262:
        lea     0x2e5aca.l,a1                   | +000

| ----------------------------------------------------------------------------
|  Sub_00083270  @ $083270  (28 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00083270, "ax", @progbits
        .global Sub_00083270
Sub_00083270:
        move.b  #0x4,0x10a2d0.l                 | +000
        move.b  #0x0,0x10a2d1.l                 | +008
        movea.l 0xc(a6),a0                      | +010
        move.b  #0x77,0x20(a0)                  | +014
        rts                                     | +01a

| ----------------------------------------------------------------------------
|  Sub_0008328C  @ $08328C  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0008328C, "ax", @progbits
        .global Sub_0008328C
Sub_0008328C:
        lea     0x2e5adc.l,a1                   | +000
        jsr     0x77c7e.l                       | +006
        lea     0x2e5a10.l,a1                   | +00c
        jsr     0x77c7e.l                       | +012
        move.w  #0x102f,d0                      | +018
        jsr     0x2352.l                        | +01c

| ----------------------------------------------------------------------------
|  Sub_000832B6  @ $0832B6  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_000832B6, "ax", @progbits
        .global Sub_000832B6
Sub_000832B6:
        lea     0x2e545e.l,a0                   | +000
        move.w  0x5c(a6),d0                     | +006
        lsl.w   #0x2,d0                         | +00a
        move.l  (a0,d0.w),0x4c(a6)              | +00c

| ----------------------------------------------------------------------------
|  Sub_000832D0  @ $0832D0  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_000832D0, "ax", @progbits
        .global Sub_000832D0
Sub_000832D0:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_0832e6                    | +00c

| ----------------------------------------------------------------------------
|  TaskHandler_0832ec  @ $0832EC  (128 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0832ec, "ax", @progbits
        .global TaskHandler_0832ec
TaskHandler_0832ec:
        move.w  #0xa4,d1                        | +000
        jsr     0x236e.l                        | +004
        move.b  #0xff,0x32(a6)                  | +00a
        move.b  #0xff,0x33(a6)                  | +010
        move.b  #0x0,0x3a(a6)                   | +016
        move.w  #0x0,0x38(a6)                   | +01c
        jsr     0x267e2.l                       | +022
        move.w  #0x80,d0                        | +028
        cmpi.w  #0xa0,0x22(a6)                  | +02c
        blt.w   .L8332a                         | +032
        neg.w   d0                              | +036
        bset    #0x0,0x3a(a6)                   | +038
.L8332a:
        move.w  d0,0x28(a6)                     | +03e
        lea     0xffff.w,a0                     | +042
        move.l  a0,0x48(a6)                     | +046
        lea     0x2e6cdc.l,a0                   | +04a
        jsr     0x28cd4.l                       | +050
        lea     .L83348(pc),a1                  | +056
        move.l  a1,(a6)                         | +05a
.L83348:
        jsr     0x27cee.l                       | +05c
        jsr     0x28d70.l                       | +062
        movea.l #0xffffffff,a0                  | +068
        jsr     0x5dd5c.l                       | +06e
        bcc.w   .L8336a                         | +074
        jmp     0x518.l                         | +078
.L8336a:
        rts                                     | +07e

| ----------------------------------------------------------------------------
|  TaskHandler_08336c  @ $08336C  (186 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08336c, "ax", @progbits
        .global TaskHandler_08336c
TaskHandler_08336c:
        move.w  #0xa4,d1                        | +000
        jsr     0x236e.l                        | +004
        move.b  #0xff,0x32(a6)                  | +00a
        move.b  #0xff,0x33(a6)                  | +010
        move.b  #0x0,0x3a(a6)                   | +016
        move.w  #0x0,0x38(a6)                   | +01c
        jsr     0x267e2.l                       | +022
        move.w  #0x0,0x72(a6)                   | +028
        cmpi.w  #0xa0,0x22(a6)                  | +02e
        blt.w   .L833aa                         | +034
        bset    #0x0,0x3a(a6)                   | +038
.L833aa:
        lea     0x2e6cdc.l,a0                   | +03e
        jsr     0x28cd4.l                       | +044
        lea     0xffff.w,a0                     | +04a
        move.l  a0,0x48(a6)                     | +04e
        lea     .L833c4(pc),a1                  | +052
        move.l  a1,(a6)                         | +056
.L833c4:
        jsr     0x2783a.l                       | +058
        jsr     0x28d70.l                       | +05e
        addi.w  #0x1,0x72(a6)                   | +064
        move.w  0x72(a6),d0                     | +06a
        btst    #0x1,d0                         | +06e
        beq.w   .L8340e                         | +072
        jsr     0x5e9b6.l                       | +076
        andi.w  #0xf,d0                         | +07c
        cmpi.w  #0x0,d0                         | +080
        bne.w   .L8340e                         | +084
        move.w  #0x50,d0                        | +088
        jsr     0x5e9e4.l                       | +08c
        btst    #0x0,0x3a(a6)                   | +092
        beq.w   .L8340a                         | +098
        neg.w   d0                              | +09c
.L8340a:
        add.w   d0,0x22(a6)                     | +09e
.L8340e:
        movea.l #0xffffffff,a0                  | +0a2
        jsr     0x5dd5c.l                       | +0a8
        bcc.w   .L83424                         | +0ae
        jmp     0x518.l                         | +0b2
.L83424:
        rts                                     | +0b8

| ----------------------------------------------------------------------------
|  TaskHandler_083426  @ $083426  (126 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_083426, "ax", @progbits
        .global TaskHandler_083426
TaskHandler_083426:
        move.w  #0xa4,d1                        | +000
        jsr     0x236e.l                        | +004
        move.b  #0xff,0x32(a6)                  | +00a
        move.b  #0xff,0x33(a6)                  | +010
        move.b  #0x0,0x3a(a6)                   | +016
        move.w  #0x0,0x38(a6)                   | +01c
        jsr     0x267e2.l                       | +022
        move.w  #0x80,d0                        | +028
        cmpi.w  #0xa0,0x22(a6)                  | +02c
        blt.w   .L83464                         | +032
        neg.w   d0                              | +036
        bset    #0x0,0x3a(a6)                   | +038
.L83464:
        move.w  d0,0x28(a6)                     | +03e
        lea     0x2e6cdc.l,a0                   | +042
        jsr     0x28cd4.l                       | +048
        lea     0xffff.w,a0                     | +04e
        move.l  a0,0x48(a6)                     | +052
        lea     .L83482(pc),a1                  | +056
        move.l  a1,(a6)                         | +05a
.L83482:
        jsr     Sub_00086050(pc)                | +05c
        jsr     0x28d70.l                       | +060
        movea.l #0xffffffff,a0                  | +066
        jsr     0x5dd5c.l                       | +06c
        bcc.w   .L834a2                         | +072
        jmp     0x518.l                         | +076
.L834a2:
        rts                                     | +07c

| ----------------------------------------------------------------------------
|  TaskHandler_0834a4  @ $0834A4  (170 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0834a4, "ax", @progbits
        .global TaskHandler_0834a4
TaskHandler_0834a4:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0xa7,d1                        | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0x30,0x70(a6)                  | +014
        move.b  #0xff,0x32(a6)                  | +01a
        move.b  #0xff,0x33(a6)                  | +020
        move.b  #0x0,0x3a(a6)                   | +026
        move.w  #0x1e,0x66(a6)                  | +02c
        lea     0x2e6c00.l,a0                   | +032
        jsr     0x28cd4.l                       | +038
        lea     .L834e8(pc),a1                  | +03e
        move.l  a1,(a6)                         | +042
.L834e8:
        clr.b   0x10e39a.l                      | +044
        jsr     0x2783a.l                       | +04a
        jsr     0x28d70.l                       | +050
        jsr     0x2870a.l                       | +056
        bcc.w   .L83516                         | +05c
        lea     0x5e766.l,a0                    | +060
        jsr     0x5e770.l                       | +066
        bclr    #0x3,0x13(a6)                   | +06c
.L83516:
        jsr     0x28758.l                       | +072
        bcc.w   .L8353c                         | +078
        lea     0x2eac8c.l,a1                   | +07c
        jsr     0x43fac.l                       | +082
        move.w  #0x1037,d0                      | +088
        jsr     0x2352.l                        | +08c
        lea     TaskHandler_08354e(pc),a1       | +092
        move.l  a1,(a6)                         | +096
.L8353c:
        jsr     0x4fa70.l                       | +098
        bcc.w   .L8354c                         | +09e
        jmp     0x518.l                         | +0a2
.L8354c:
        rts                                     | +0a8

| ----------------------------------------------------------------------------
|  TaskHandler_08354e  @ $08354E  (72 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08354e, "ax", @progbits
        .global TaskHandler_08354e
TaskHandler_08354e:
        move.l  #0x100,d0                       | +000
        jsr     0x51a28.l                       | +006
        lea     0xffff.w,a0                     | +00c
        move.l  a0,0x48(a6)                     | +010
        lea     0x2e6c16.l,a0                   | +014
        jsr     0x28cd4.l                       | +01a
        lea     .L83574(pc),a1                  | +020
        move.l  a1,(a6)                         | +024
.L83574:
        jsr     0x2783a.l                       | +026
        jsr     0x28d70.l                       | +02c
        bcs.w   .L8358e                         | +032
        jsr     0x4fa70.l                       | +036
        bcc.w   .L83594                         | +03c
.L8358e:
        jmp     0x518.l                         | +040
.L83594:
        rts                                     | +046

| ----------------------------------------------------------------------------
|  TaskHandler_083596  @ $083596  (92 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_083596, "ax", @progbits
        .global TaskHandler_083596
TaskHandler_083596:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0xab,d1                        | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0x30,0x70(a6)                  | +014
        move.b  #0xff,0x32(a6)                  | +01a
        move.b  #0xff,0x33(a6)                  | +020
        move.b  #0x0,0x3a(a6)                   | +026
        lea     0x2e6d28.l,a0                   | +02c
        jsr     0x28cd4.l                       | +032
        lea     .L835d4(pc),a1                  | +038
        move.l  a1,(a6)                         | +03c
.L835d4:
        jsr     0x2783a.l                       | +03e
        jsr     0x28d70.l                       | +044
        jsr     0x4fa70.l                       | +04a
        bcc.w   .L835f0                         | +050
        jmp     0x518.l                         | +054
.L835f0:
        rts                                     | +05a

| ----------------------------------------------------------------------------
|  TaskHandler_0835f2  @ $0835F2  (32 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0835f2, "ax", @progbits
        .global TaskHandler_0835f2
TaskHandler_0835f2:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.b  #0x0,0x21(a6)                   | +00a
        lea     0x2e6d38.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        bra.w   TaskCont_08364e                 | +01c

| ----------------------------------------------------------------------------
|  TaskHandler_083612  @ $083612  (32 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_083612, "ax", @progbits
        .global TaskHandler_083612
TaskHandler_083612:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.b  #0x1,0x21(a6)                   | +00a
        lea     0x2e6da4.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        bra.w   TaskCont_08364e                 | +01c

| ----------------------------------------------------------------------------
|  TaskHandler_083632  @ $083632  (226 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_083632, "ax", @progbits
        .global TaskHandler_083632
TaskHandler_083632:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.b  #0x2,0x21(a6)                   | +00a
        lea     0x2e6da4.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        .global TaskCont_08364e
TaskCont_08364e:
        move.w  #0xaa,d1                        | +01c
        jsr     0x236e.l                        | +020
        move.b  #0x0,0x3a(a6)                   | +026
        move.w  0x70(a6),d0                     | +02c
        or.b    d0,0x3a(a6)                     | +030
        asr.w   #0x8,d0                         | +034
        move.b  d0,0x20(a6)                     | +036
        move.w  #0x30,0x70(a6)                  | +03a
        move.b  #0xff,0x32(a6)                  | +040
        move.b  #0xff,0x33(a6)                  | +046
        move.w  #0x3c,0x66(a6)                  | +04c
        move.w  #0x8000,0x38(a6)                | +052
        jsr     0x267e2.l                       | +058
        jsr     0x27cee.l                       | +05e
        btst    #0x0,0x3a(a6)                   | +064
        beq.w   .L836a6                         | +06a
        addi.w  #0x20,0x22(a6)                  | +06e
.L836a6:
        cmpi.b  #0x2,0x21(a6)                   | +074
        bne.w   .L836c0                         | +07a
        lea     0x2e9698.l,a0                   | +07e
        move.l  a0,0x4c(a6)                     | +084
        jsr     0x283ca.l                       | +088
.L836c0:
        lea     .L836c6(pc),a1                  | +08e
        move.l  a1,(a6)                         | +092
.L836c6:
        jsr     0x2783a.l                       | +094
        jsr     0x28d70.l                       | +09a
        jsr     Sub_000863BE(pc)                | +0a0
        jsr     0x2870a.l                       | +0a4
        bcc.w   .L836f2                         | +0aa
        lea     0x5e766.l,a0                    | +0ae
        jsr     0x5e770.l                       | +0b4
        bclr    #0x3,0x13(a6)                   | +0ba
.L836f2:
        cmpi.w  #0x28,0x66(a6)                  | +0c0
        bgt.w   .L83702                         | +0c6
        lea     TaskHandler_083714(pc),a1       | +0ca
        move.l  a1,(a6)                         | +0ce
.L83702:
        jsr     0x4fa70.l                       | +0d0
        bcc.w   .L83712                         | +0d6
        jmp     0x518.l                         | +0da
.L83712:
        rts                                     | +0e0

| ----------------------------------------------------------------------------
|  TaskHandler_083714  @ $083714  (118 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_083714, "ax", @progbits
        .global TaskHandler_083714
TaskHandler_083714:
        lea     0x2e6d4e.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        cmpi.b  #0x0,0x21(a6)                   | +00c
        beq.w   .L83736                         | +012
        lea     0x2e6dba.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
.L83736:
        lea     .L8373c(pc),a1                  | +022
        move.l  a1,(a6)                         | +026
.L8373c:
        jsr     0x2783a.l                       | +028
        jsr     0x28d70.l                       | +02e
        jsr     Sub_000863BE(pc)                | +034
        jsr     0x2870a.l                       | +038
        bcc.w   .L83768                         | +03e
        lea     0x5e766.l,a0                    | +042
        jsr     0x5e770.l                       | +048
        bclr    #0x3,0x13(a6)                   | +04e
.L83768:
        cmpi.w  #0x14,0x66(a6)                  | +054
        bgt.w   .L83778                         | +05a
        lea     TaskHandler_08378a(pc),a1       | +05e
        move.l  a1,(a6)                         | +062
.L83778:
        jsr     0x4fa70.l                       | +064
        bcc.w   .L83788                         | +06a
        jmp     0x518.l                         | +06e
.L83788:
        rts                                     | +074

| ----------------------------------------------------------------------------
|  TaskHandler_08378a  @ $08378A  (118 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08378a, "ax", @progbits
        .global TaskHandler_08378a
TaskHandler_08378a:
        lea     0x2e6d6a.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        cmpi.b  #0x0,0x21(a6)                   | +00c
        beq.w   .L837ac                         | +012
        lea     0x2e6dd6.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
.L837ac:
        lea     .L837b2(pc),a1                  | +022
        move.l  a1,(a6)                         | +026
.L837b2:
        jsr     0x2783a.l                       | +028
        jsr     0x28d70.l                       | +02e
        jsr     Sub_000863BE(pc)                | +034
        jsr     0x2870a.l                       | +038
        bcc.w   .L837de                         | +03e
        lea     0x5e766.l,a0                    | +042
        jsr     0x5e770.l                       | +048
        bclr    #0x3,0x13(a6)                   | +04e
.L837de:
        jsr     0x28758.l                       | +054
        bcc.w   .L837ee                         | +05a
        lea     TaskHandler_083800(pc),a1       | +05e
        move.l  a1,(a6)                         | +062
.L837ee:
        jsr     0x4fa70.l                       | +064
        bcc.w   .L837fe                         | +06a
        jmp     0x518.l                         | +06e
.L837fe:
        rts                                     | +074

| ----------------------------------------------------------------------------
|  TaskHandler_083800  @ $083800  (294 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_083800, "ax", @progbits
        .global TaskHandler_083800
TaskHandler_083800:
        jsr     0x2783a.l                       | +000
        move.l  #0x2000,d0                      | +006
        jsr     0x51a28.l                       | +00c
        jsr     Sub_00085FB0(pc)                | +012
        lea     0xffff.w,a0                     | +016
        move.l  a0,0x4c(a6)                     | +01a
        jsr     0x283ca.l                       | +01e
        lea     0xffff.w,a0                     | +024
        move.l  a0,0x48(a6)                     | +028
        cmpi.w  #0x170,0x24(a6)                 | +02c
        bgt.w   .L83852                         | +032
        lea     0x2e9a1e.l,a1                   | +036
        cmpi.b  #0x0,0x21(a6)                   | +03c
        beq.w   .L8384c                         | +042
        lea     0x2e9a30.l,a1                   | +046
.L8384c:
        jsr     0x77c7e.l                       | +04c
.L83852:
        lea     0x2e6d86.l,a0                   | +052
        jsr     0x28cd4.l                       | +058
        lea     0x2eaca0.l,a1                   | +05e
        cmpi.b  #0x0,0x21(a6)                   | +064
        beq.w   .L83880                         | +06a
        lea     0x2e6df2.l,a0                   | +06e
        jsr     0x28cd4.l                       | +074
        lea     0x2eacac.l,a1                   | +07a
.L83880:
        move.w  0x22(a6),d0                     | +080
        movem.w d0,-(a7)                        | +084
        btst    #0x0,0x3a(a6)                   | +088
        beq.w   .L83898                         | +08e
        subi.w  #0x18,0x22(a6)                  | +092
.L83898:
        cmpi.b  #0x1,0x20(a6)                   | +098
        beq.w   .L838a8                         | +09e
        jsr     0x43fac.l                       | +0a2
.L838a8:
        movem.w (a7)+,d0                        | +0a8
        move.w  d0,0x22(a6)                     | +0ac
        jsr     0x267e2.l                       | +0b0
        move.w  #0xfffd,0x2e(a6)                | +0b6
        move.w  #0x40,d0                        | +0bc
        btst    #0x0,0x3a(a6)                   | +0c0
        beq.w   .L838cc                         | +0c6
        neg.w   d0                              | +0ca
.L838cc:
        move.w  d0,0x28(a6)                     | +0cc
        move.w  #0x5a,0x72(a6)                  | +0d0
        move.b  #0xf0,d0                        | +0d6
        jsr     0x5e722.l                       | +0da
        lea     .L838e6(pc),a1                  | +0e0
        move.l  a1,(a6)                         | +0e4
.L838e6:
        cmpi.w  #0x50,0x72(a6)                  | +0e6
        bne.w   .L838fa                         | +0ec
        move.w  #0x1054,d0                      | +0f0
        jsr     0x2352.l                        | +0f4
.L838fa:
        jsr     0x27cee.l                       | +0fa
        jsr     0x28d70.l                       | +100
        subq.w  #0x1,0x72(a6)                   | +106
        bne.w   .L83914                         | +10a
        lea     TaskHandler_083926(pc),a1       | +10e
        move.l  a1,(a6)                         | +112
.L83914:
        jsr     0x4fa70.l                       | +114
        bcc.w   .L83924                         | +11a
        jmp     0x518.l                         | +11e
.L83924:
        rts                                     | +124

| ----------------------------------------------------------------------------
|  TaskHandler_083926  @ $083926  (68 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_083926, "ax", @progbits
        .global TaskHandler_083926
TaskHandler_083926:
        lea     0x2e987e.l,a1                   | +000
        jsr     0x77c7e.l                       | +006
        move.w  #0x1e,0x72(a6)                  | +00c
        jsr     0x267e2.l                       | +012
        lea     .L83944(pc),a1                  | +018
        move.l  a1,(a6)                         | +01c
.L83944:
        jsr     0x27cee.l                       | +01e
        jsr     0x28d70.l                       | +024
        subq.w  #0x1,0x72(a6)                   | +02a
        beq.w   .L83962                         | +02e
        jsr     0x4fa70.l                       | +032
        bcc.w   .L83968                         | +038
.L83962:
        jmp     0x518.l                         | +03c
.L83968:
        rts                                     | +042

| ----------------------------------------------------------------------------
|  TaskHandler_08396a  @ $08396A  (120 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08396a, "ax", @progbits
        .global TaskHandler_08396a
TaskHandler_08396a:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0xa5,d1                        | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0x10,0x70(a6)                  | +014
        move.b  #0xff,0x32(a6)                  | +01a
        move.b  #0xff,0x33(a6)                  | +020
        move.b  #0x0,0x3a(a6)                   | +026
        lea     0x2e6e10.l,a0                   | +02c
        jsr     0x28cd4.l                       | +032
        .global TaskHandler_0839a2
TaskHandler_0839a2:
        bclr    #0x3,0x13(a6)                   | +038
        lea     .L839ae(pc),a1                  | +03e
        move.l  a1,(a6)                         | +042
.L839ae:
        jsr     0x2783a.l                       | +044
        move.b  #0x1,0x44(a6)                   | +04a
        jsr     0x28d70.l                       | +050
        jsr     0x2870a.l                       | +056
        bcc.w   .L839d0                         | +05c
        lea     TaskHandler_0839e2(pc),a1       | +060
        move.l  a1,(a6)                         | +064
.L839d0:
        jsr     0x4fa70.l                       | +066
        bcc.w   .L839e0                         | +06c
        jmp     0x518.l                         | +070
.L839e0:
        rts                                     | +076

| ----------------------------------------------------------------------------
|  TaskHandler_0839e2  @ $0839E2  (84 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0839e2, "ax", @progbits
        .global TaskHandler_0839e2
TaskHandler_0839e2:
        move.w  #0x10a9,d0                      | +000
        jsr     0x2352.l                        | +004
        move.b  #0x0,0x44(a6)                   | +00a
        move.w  #0xa,d0                         | +010
        jsr     0x5e9e4.l                       | +014
        addi.w  #0x23,d0                        | +01a
        move.w  d0,0x72(a6)                     | +01e
        lea     .L83a0a(pc),a1                  | +022
        move.l  a1,(a6)                         | +026
.L83a0a:
        jsr     0x2783a.l                       | +028
        jsr     0x28d70.l                       | +02e
        subq.w  #0x1,0x72(a6)                   | +034
        bne.w   .L83a24                         | +038
        lea     TaskHandler_0839a2(pc),a1       | +03c
        move.l  a1,(a6)                         | +040
.L83a24:
        jsr     0x4fa70.l                       | +042
        bcc.w   .L83a34                         | +048
        jmp     0x518.l                         | +04c
.L83a34:
        rts                                     | +052

| ----------------------------------------------------------------------------
|  TaskHandler_083a36  @ $083A36  (110 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_083a36, "ax", @progbits
        .global TaskHandler_083a36
TaskHandler_083a36:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0xa5,d1                        | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0x0,0x70(a6)                   | +014
        move.b  #0xff,0x32(a6)                  | +01a
        move.b  #0xff,0x33(a6)                  | +020
        move.b  #0x0,0x3a(a6)                   | +026
        move.w  #0x8000,0x38(a6)                | +02c
        jsr     0x267e2.l                       | +032
        jsr     0x27cee.l                       | +038
        lea     0x2e6ebc.l,a0                   | +03e
        jsr     0x28cd4.l                       | +044
        lea     .L83a86(pc),a1                  | +04a
        move.l  a1,(a6)                         | +04e
.L83a86:
        jsr     0x2783a.l                       | +050
        jsr     0x28d70.l                       | +056
        jsr     0x4fa70.l                       | +05c
        bcc.w   .L83aa2                         | +062
        jmp     0x518.l                         | +066
.L83aa2:
        rts                                     | +06c

| ----------------------------------------------------------------------------
|  TaskHandler_083aa4  @ $083AA4  (110 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_083aa4, "ax", @progbits
        .global TaskHandler_083aa4
TaskHandler_083aa4:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0xa7,d1                        | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0x0,0x70(a6)                   | +014
        move.b  #0xff,0x32(a6)                  | +01a
        move.b  #0xff,0x33(a6)                  | +020
        move.b  #0x0,0x3a(a6)                   | +026
        move.w  #0x8000,0x38(a6)                | +02c
        jsr     0x267e2.l                       | +032
        jsr     0x27cee.l                       | +038
        lea     0x2e6ecc.l,a0                   | +03e
        jsr     0x28cd4.l                       | +044
        lea     .L83af4(pc),a1                  | +04a
        move.l  a1,(a6)                         | +04e
.L83af4:
        jsr     0x2783a.l                       | +050
        jsr     0x28d70.l                       | +056
        jsr     0x4fa70.l                       | +05c
        bcc.w   .L83b10                         | +062
        jmp     0x518.l                         | +066
.L83b10:
        rts                                     | +06c

| ----------------------------------------------------------------------------
|  TaskHandler_083b12  @ $083B12  (114 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_083b12, "ax", @progbits
        .global TaskHandler_083b12
TaskHandler_083b12:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0xa7,d1                        | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0x100,0x70(a6)                 | +014
        move.b  #0xff,0x32(a6)                  | +01a
        move.b  #0xff,0x33(a6)                  | +020
        move.b  #0x0,0x3a(a6)                   | +026
        lea     0x2e6edc.l,a0                   | +02c
        jsr     0x28cd4.l                       | +032
        lea     TaskHandler_083b92(pc),a1       | +038
        jsr     0x4ae.l                         | +03c
        move.w  0x38(a6),0x38(a0)               | +042
        jsr     Sub_00086076(pc)                | +048
        lea     .L83b64(pc),a1                  | +04c
        move.l  a1,(a6)                         | +050
.L83b64:
        jsr     0x2783a.l                       | +052
        cmpi.w  #0x120,0x22(a6)                 | +058
        bgt.w   .L83b7a                         | +05e
        jsr     0x28d70.l                       | +062
.L83b7a:
        jsr     0x4fa70.l                       | +068
        bcc.w   Jsr5B6Rts_083b90                | +06e

| ----------------------------------------------------------------------------
|  TaskHandler_083b92  @ $083B92  (72 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_083b92, "ax", @progbits
        .global TaskHandler_083b92
TaskHandler_083b92:
        move.w  #0xa7,d1                        | +000
        jsr     0x236e.l                        | +004
        move.b  #0xff,0x32(a6)                  | +00a
        move.b  #0xff,0x33(a6)                  | +010
        move.b  #0x0,0x3a(a6)                   | +016
        lea     0x2e6eec.l,a0                   | +01c
        jsr     0x28cd4.l                       | +022
        lea     .L83bc0(pc),a1                  | +028
        move.l  a1,(a6)                         | +02c
.L83bc0:
        movea.l 0xc(a6),a0                      | +02e
        move.w  0x22(a0),0x22(a6)               | +032
        move.w  0x24(a0),0x24(a6)               | +038
        cmpi.w  #0xff80,0x22(a6)                | +03e
        blt.w   JsrAbsRts_083be0                | +044
