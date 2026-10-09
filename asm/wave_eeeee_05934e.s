| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave ??? — (borrador)
|  Región: $05934E..$05A9D6  (5,290 B, 71 entradas, 38 huecos)
| ============================================================================
|
|  BORRADOR generado por tools/gen_asm_region.py — pendiente de análisis
|  semántico (nombres, comentarios de campo, evidencias).
|
|  Verificación: cada sección .text.<Sym> se coloca en su dirección CPU
|  absoluta y reensambla byte-exacta contra build/mslug_prom.bin
|  (MD5 816b3f74c76b3373993407615f1850fe).
| ============================================================================

        .text

| ----------------------------------------------------------------------------
|  ResultText_Strings_05934e  @ $05934E  (172 B)
| ----------------------------------------------------------------------------
        .section .text.ResultText_Strings_05934e, "ax", @progbits
        .global ResultText_Strings_05934e
ResultText_Strings_05934e:
        .dc.b   0x4d                          | +000  'M'  (dato, rango --data)
        .dc.b   0x45                          | +001  'E'  (dato, rango --data)
        .dc.b   0x54                          | +002  'T'  (dato, rango --data)
        .dc.b   0x41                          | +003  'A'  (dato, rango --data)
        .dc.b   0x4c                          | +004  'L'  (dato, rango --data)
        .dc.b   0x20                          | +005  ' '  (dato, rango --data)
        .dc.b   0x53                          | +006  'S'  (dato, rango --data)
        .dc.b   0x4c                          | +007  'L'  (dato, rango --data)
        .dc.b   0x55                          | +008  'U'  (dato, rango --data)
        .dc.b   0x47                          | +009  'G'  (dato, rango --data)
        .dc.b   0x20                          | +00a  ' '  (dato, rango --data)
        .dc.b   0x20                          | +00b  ' '  (dato, rango --data)
        .dc.b   0x20                          | +00c  ' '  (dato, rango --data)
        .dc.b   0x20                          | +00d  ' '  (dato, rango --data)
        .dc.b   0x20                          | +00e  ' '  (dato, rango --data)
        .dc.b   0x20                          | +00f  ' '  (dato, rango --data)
        .dc.b   0x20                          | +010  ' '  (dato, rango --data)
        .dc.b   0x20                          | +011  ' '  (dato, rango --data)
        .dc.b   0x20                          | +012  ' '  (dato, rango --data)
        .dc.b   0x20                          | +013  ' '  (dato, rango --data)
        .dc.b   0x41                          | +014  'A'  (dato, rango --data)
        .dc.b   0x4c                          | +015  'L'  (dato, rango --data)
        .dc.b   0x4c                          | +016  'L'  (dato, rango --data)
        .dc.b   0xff                          | +017  '.'  (dato, rango --data)
        .dc.b   0x20                          | +018  ' '  (dato, rango --data)
        .dc.b   0x20                          | +019  ' '  (dato, rango --data)
        .dc.b   0x20                          | +01a  ' '  (dato, rango --data)
        .dc.b   0xff                          | +01b  '.'  (dato, rango --data)
        .dc.b   0x4d                          | +01c  'M'  (dato, rango --data)
        .dc.b   0x49                          | +01d  'I'  (dato, rango --data)
        .dc.b   0x53                          | +01e  'S'  (dato, rango --data)
        .dc.b   0x53                          | +01f  'S'  (dato, rango --data)
        .dc.b   0x49                          | +020  'I'  (dato, rango --data)
        .dc.b   0x4f                          | +021  'O'  (dato, rango --data)
        .dc.b   0x4e                          | +022  'N'  (dato, rango --data)
        .dc.b   0xff                          | +023  '.'  (dato, rango --data)
        .dc.b   0x20                          | +024  ' '  (dato, rango --data)
        .dc.b   0x20                          | +025  ' '  (dato, rango --data)
        .dc.b   0x20                          | +026  ' '  (dato, rango --data)
        .dc.b   0x20                          | +027  ' '  (dato, rango --data)
        .dc.b   0x20                          | +028  ' '  (dato, rango --data)
        .dc.b   0x20                          | +029  ' '  (dato, rango --data)
        .dc.b   0x20                          | +02a  ' '  (dato, rango --data)
        .dc.b   0xff                          | +02b  '.'  (dato, rango --data)
        .dc.b   0x4f                          | +02c  'O'  (dato, rango --data)
        .dc.b   0x56                          | +02d  'V'  (dato, rango --data)
        .dc.b   0x45                          | +02e  'E'  (dato, rango --data)
        .dc.b   0x52                          | +02f  'R'  (dato, rango --data)
        .dc.b   0x21                          | +030  '!'  (dato, rango --data)
        .dc.b   0xff                          | +031  '.'  (dato, rango --data)
        .dc.b   0x20                          | +032  ' '  (dato, rango --data)
        .dc.b   0x20                          | +033  ' '  (dato, rango --data)
        .dc.b   0x20                          | +034  ' '  (dato, rango --data)
        .dc.b   0x20                          | +035  ' '  (dato, rango --data)
        .dc.b   0x20                          | +036  ' '  (dato, rango --data)
        .dc.b   0xff                          | +037  '.'  (dato, rango --data)
        .global ResultText_Strings_05934e__L059386
ResultText_Strings_05934e__L059386:
.L059386:
        .dc.b   0x20                          | +038  ' '  (dato, rango --data)
        .dc.b   0x20                          | +039  ' '  (dato, rango --data)
        .dc.b   0x20                          | +03a  ' '  (dato, rango --data)
        .dc.b   0x31                          | +03b  '1'  (dato, rango --data)
        .dc.b   0x50                          | +03c  'P'  (dato, rango --data)
        .dc.b   0x20                          | +03d  ' '  (dato, rango --data)
        .dc.b   0x52                          | +03e  'R'  (dato, rango --data)
        .dc.b   0x45                          | +03f  'E'  (dato, rango --data)
        .dc.b   0x53                          | +040  'S'  (dato, rango --data)
        .dc.b   0x55                          | +041  'U'  (dato, rango --data)
        .dc.b   0x4c                          | +042  'L'  (dato, rango --data)
        .dc.b   0x54                          | +043  'T'  (dato, rango --data)
        .dc.b   0xff                          | +044  '.'  (dato, rango --data)
        .global ResultText_Strings_05934e__L059393
ResultText_Strings_05934e__L059393:
.L059393:
        .dc.b   0x20                          | +045  ' '  (dato, rango --data)
        .dc.b   0x20                          | +046  ' '  (dato, rango --data)
        .dc.b   0x20                          | +047  ' '  (dato, rango --data)
        .dc.b   0x32                          | +048  '2'  (dato, rango --data)
        .dc.b   0x50                          | +049  'P'  (dato, rango --data)
        .dc.b   0x20                          | +04a  ' '  (dato, rango --data)
        .dc.b   0x52                          | +04b  'R'  (dato, rango --data)
        .dc.b   0x45                          | +04c  'E'  (dato, rango --data)
        .dc.b   0x53                          | +04d  'S'  (dato, rango --data)
        .dc.b   0x55                          | +04e  'U'  (dato, rango --data)
        .dc.b   0x4c                          | +04f  'L'  (dato, rango --data)
        .dc.b   0x54                          | +050  'T'  (dato, rango --data)
        .dc.b   0xff                          | +051  '.'  (dato, rango --data)
        .global ResultText_Strings_05934e__L0593a0
ResultText_Strings_05934e__L0593a0:
.L0593a0:
        .dc.b   0x53                          | +052  'S'  (dato, rango --data)
        .dc.b   0x43                          | +053  'C'  (dato, rango --data)
        .dc.b   0x4f                          | +054  'O'  (dato, rango --data)
        .dc.b   0x52                          | +055  'R'  (dato, rango --data)
        .dc.b   0x45                          | +056  'E'  (dato, rango --data)
        .dc.b   0xff                          | +057  '.'  (dato, rango --data)
        .global ResultText_Strings_05934e__L0593a6
ResultText_Strings_05934e__L0593a6:
.L0593a6:
        .dc.b   0x43                          | +058  'C'  (dato, rango --data)
        .dc.b   0x4f                          | +059  'O'  (dato, rango --data)
        .dc.b   0x4e                          | +05a  'N'  (dato, rango --data)
        .dc.b   0x54                          | +05b  'T'  (dato, rango --data)
        .dc.b   0x49                          | +05c  'I'  (dato, rango --data)
        .dc.b   0x4e                          | +05d  'N'  (dato, rango --data)
        .dc.b   0x55                          | +05e  'U'  (dato, rango --data)
        .dc.b   0x45                          | +05f  'E'  (dato, rango --data)
        .dc.b   0xff                          | +060  '.'  (dato, rango --data)
        .dc.b   0x54                          | +061  'T'  (dato, rango --data)
        .dc.b   0x4f                          | +062  'O'  (dato, rango --data)
        .dc.b   0x54                          | +063  'T'  (dato, rango --data)
        .dc.b   0x41                          | +064  'A'  (dato, rango --data)
        .dc.b   0x4c                          | +065  'L'  (dato, rango --data)
        .dc.b   0xff                          | +066  '.'  (dato, rango --data)
        .global ResultText_Strings_05934e__L0593b5
ResultText_Strings_05934e__L0593b5:
.L0593b5:
        .dc.b   0x52                          | +067  'R'  (dato, rango --data)
        .dc.b   0x45                          | +068  'E'  (dato, rango --data)
        .dc.b   0x43                          | +069  'C'  (dato, rango --data)
        .dc.b   0x41                          | +06a  'A'  (dato, rango --data)
        .dc.b   0x50                          | +06b  'P'  (dato, rango --data)
        .dc.b   0x54                          | +06c  'T'  (dato, rango --data)
        .dc.b   0x55                          | +06d  'U'  (dato, rango --data)
        .dc.b   0x52                          | +06e  'R'  (dato, rango --data)
        .dc.b   0x45                          | +06f  'E'  (dato, rango --data)
        .dc.b   0x44                          | +070  'D'  (dato, rango --data)
        .dc.b   0xfd                          | +071  '.'  (dato, rango --data)
        .dc.b   0x50                          | +072  'P'  (dato, rango --data)
        .dc.b   0x52                          | +073  'R'  (dato, rango --data)
        .dc.b   0x49                          | +074  'I'  (dato, rango --data)
        .dc.b   0x53                          | +075  'S'  (dato, rango --data)
        .dc.b   0x4f                          | +076  'O'  (dato, rango --data)
        .dc.b   0x4e                          | +077  'N'  (dato, rango --data)
        .dc.b   0x45                          | +078  'E'  (dato, rango --data)
        .dc.b   0x52                          | +079  'R'  (dato, rango --data)
        .dc.b   0x20                          | +07a  ' '  (dato, rango --data)
        .dc.b   0x20                          | +07b  ' '  (dato, rango --data)
        .dc.b   0xfe                          | +07c  '.'  (dato, rango --data)
        .dc.b   0x00                          | +07d  '.'  (dato, rango --data)
        .global ResultText_Strings_05934e__L0593cc
ResultText_Strings_05934e__L0593cc:
.L0593cc:
        .dc.b   0x0d                          | +07e  '.'  (dato, rango --data)
        .dc.b   0x22                          | +07f  '"'  (dato, rango --data)
        .dc.b   0x0d                          | +080  '.'  (dato, rango --data)
        .dc.b   0x24                          | +081  '$'  (dato, rango --data)
        .dc.b   0x0d                          | +082  '.'  (dato, rango --data)
        .dc.b   0xe6                          | +083  '.'  (dato, rango --data)
        .dc.b   0x0d                          | +084  '.'  (dato, rango --data)
        .dc.b   0xe8                          | +085  '.'  (dato, rango --data)
        .dc.b   0x0d                          | +086  '.'  (dato, rango --data)
        .dc.b   0xea                          | +087  '.'  (dato, rango --data)
        .dc.b   0x00                          | +088  '.'  (dato, rango --data)
        .dc.b   0xff                          | +089  '.'  (dato, rango --data)
        .global ResultText_Strings_05934e__L0593d8
ResultText_Strings_05934e__L0593d8:
.L0593d8:
        .dc.b   0x08                          | +08a  '.'  (dato, rango --data)
        .dc.b   0x00                          | +08b  '.'  (dato, rango --data)
        .dc.b   0x01                          | +08c  '.'  (dato, rango --data)
        .dc.b   0x93                          | +08d  '.'  (dato, rango --data)
        .dc.b   0x08                          | +08e  '.'  (dato, rango --data)
        .dc.b   0x01                          | +08f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +090  '.'  (dato, rango --data)
        .dc.b   0x00                          | +091  '.'  (dato, rango --data)
        .dc.b   0x00                          | +092  '.'  (dato, rango --data)
        .dc.b   0x02                          | +093  '.'  (dato, rango --data)
        .dc.b   0x01                          | +094  '.'  (dato, rango --data)
        .dc.b   0x90                          | +095  '.'  (dato, rango --data)
        .dc.b   0x00                          | +096  '.'  (dato, rango --data)
        .dc.b   0x03                          | +097  '.'  (dato, rango --data)
        .dc.b   0x01                          | +098  '.'  (dato, rango --data)
        .dc.b   0x91                          | +099  '.'  (dato, rango --data)
        .dc.b   0x00                          | +09a  '.'  (dato, rango --data)
        .dc.b   0x04                          | +09b  '.'  (dato, rango --data)
        .dc.b   0x01                          | +09c  '.'  (dato, rango --data)
        .dc.b   0x92                          | +09d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +09e  '.'  (dato, rango --data)
        .dc.b   0x05                          | +09f  '.'  (dato, rango --data)
        .dc.b   0x01                          | +0a0  '.'  (dato, rango --data)
        .dc.b   0x94                          | +0a1  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0a2  '.'  (dato, rango --data)
        .dc.b   0x09                          | +0a3  '.'  (dato, rango --data)
        .dc.b   0x01                          | +0a4  '.'  (dato, rango --data)
        .dc.b   0x90                          | +0a5  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0a6  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0a7  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0a8  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0a9  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0aa  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0ab  '.'  (dato, rango --data)

| ----------------------------------------------------------------------------
|  Ending_ShowMission_0593fa  @ $0593FA  (56 B)
| ----------------------------------------------------------------------------
        .section .text.Ending_ShowMission_0593fa, "ax", @progbits
        .global Ending_ShowMission_0593fa
Ending_ShowMission_0593fa:
        lea     0x4737e.l,a1                    | +000
        jsr     0x4ae.l                         | +006
        move.w  #0x708f,d0                      | +00c
        move.w  d0,0x22(a0)                     | +010
        move.l  #0x5936a,0x3c(a0)               | +014
        move.w  #0x1,0x30(a0)                   | +01c
        move.b  #0x4,0x16(a0)                   | +022
        lea     .L059428(pc),a1                 | +028
        move.l  a1,(a6)                         | +02c
.L059428:
        jsr     0x6f0.l                         | +02e
        bcs.w   SetHandlerRts_059438            | +034

| ----------------------------------------------------------------------------
|  Ending_ShowAll_05943a  @ $05943A  (56 B)
| ----------------------------------------------------------------------------
        .section .text.Ending_ShowAll_05943a, "ax", @progbits
        .global Ending_ShowAll_05943a
Ending_ShowAll_05943a:
        lea     0x4737e.l,a1                    | +000
        jsr     0x4ae.l                         | +006
        move.w  #0x726f,d0                      | +00c
        move.w  d0,0x22(a0)                     | +010
        move.l  #0x59362,0x3c(a0)               | +014
        move.w  #0x1,0x30(a0)                   | +01c
        move.b  #0x4,0x16(a0)                   | +022
        lea     .L059468(pc),a1                 | +028
        move.l  a1,(a6)                         | +02c
.L059468:
        jsr     0x6f0.l                         | +02e
        bcs.w   SetHandlerRts_059478            | +034

| ----------------------------------------------------------------------------
|  Ending_ShowOver_05947a  @ $05947A  (56 B)
| ----------------------------------------------------------------------------
        .section .text.Ending_ShowOver_05947a, "ax", @progbits
        .global Ending_ShowOver_05947a
Ending_ShowOver_05947a:
        lea     0x4737e.l,a1                    | +000
        jsr     0x4ae.l                         | +006
        move.w  #0x734f,d0                      | +00c
        move.w  d0,0x22(a0)                     | +010
        move.l  #0x5937a,0x3c(a0)               | +014
        move.w  #0x1,0x30(a0)                   | +01c
        move.b  #0x4,0x16(a0)                   | +022
        lea     .L0594a8(pc),a1                 | +028
        move.l  a1,(a6)                         | +02c
.L0594a8:
        jsr     0x6f0.l                         | +02e
        bcs.w   SetHandlerRts_0594b8            | +034

| ----------------------------------------------------------------------------
|  Ending_Exit_0594ba  @ $0594BA  (12 B)
| ----------------------------------------------------------------------------
        .section .text.Ending_Exit_0594ba, "ax", @progbits
        .global Ending_Exit_0594ba
Ending_Exit_0594ba:
        jsr     0x5b6.l                         | +000
        jmp     0x518.l                         | +006

| ----------------------------------------------------------------------------
|  Ending_WipeAllOver_0594c6  @ $0594C6  (218 B)
| ----------------------------------------------------------------------------
        .section .text.Ending_WipeAllOver_0594c6, "ax", @progbits
        .global Ending_WipeAllOver_0594c6
Ending_WipeAllOver_0594c6:
        move.w  #0xf,0x30(a6)                   | +000
        lea     .L0594d2(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L0594d2:
        move.w  #0x4,d0                         | +00c
        lsl.w   #0x5,d0                         | +010
        add.w   0x30(a6),d0                     | +012
        addi.w  #0x7000,d0                      | +016
        movea.w d0,a1                           | +01a
        movea.l #0x59372,a2                     | +01c
        move.w  #0x4,d1                         | +022
        jsr     0x4784c.l                       | +026
        move.w  #0x13,d0                        | +02c
        lsl.w   #0x5,d0                         | +030
        add.w   0x30(a6),d0                     | +032
        addi.w  #0x7000,d0                      | +036
        movea.w d0,a1                           | +03a
        movea.l #0x59366,a2                     | +03c
        move.w  #0x4,d1                         | +042
        jsr     0x4784c.l                       | +046
        move.w  #0x1a,d0                        | +04c
        lsl.w   #0x5,d0                         | +050
        add.w   0x30(a6),d0                     | +052
        addi.w  #0x7000,d0                      | +056
        movea.w d0,a1                           | +05a
        movea.l #0x59380,a2                     | +05c
        move.w  #0x4,d1                         | +062
        jsr     0x4784c.l                       | +066
        subq.w  #0x1,0x30(a6)                   | +06c
        move.w  #0x4,d0                         | +070
        lsl.w   #0x5,d0                         | +074
        add.w   0x30(a6),d0                     | +076
        addi.w  #0x7000,d0                      | +07a
        movea.w d0,a1                           | +07e
        movea.l #0x5936a,a2                     | +080
        move.w  #0x4,d1                         | +086
        jsr     0x4784c.l                       | +08a
        move.w  #0x13,d0                        | +090
        lsl.w   #0x5,d0                         | +094
        add.w   0x30(a6),d0                     | +096
        addi.w  #0x7000,d0                      | +09a
        movea.w d0,a1                           | +09e
        movea.l #0x59362,a2                     | +0a0
        move.w  #0x4,d1                         | +0a6
        jsr     0x4784c.l                       | +0aa
        move.w  #0x1a,d0                        | +0b0
        lsl.w   #0x5,d0                         | +0b4
        add.w   0x30(a6),d0                     | +0b6
        addi.w  #0x7000,d0                      | +0ba
        movea.w d0,a1                           | +0be
        movea.l #0x5937a,a2                     | +0c0
        move.w  #0x4,d1                         | +0c6
        jsr     0x4784c.l                       | +0ca
        cmpi.w  #0x5,0x30(a6)                   | +0d0
        bne.w   Jsr5B6Rts_0595ac                | +0d6

| ----------------------------------------------------------------------------
|  Result_DrawDigits_0595ae  @ $0595AE  (160 B)
| ----------------------------------------------------------------------------
        .section .text.Result_DrawDigits_0595ae, "ax", @progbits
        .global Result_DrawDigits_0595ae
Result_DrawDigits_0595ae:
        clr.b   d6                              | +000
        bra.w   .L05960c                        | +002
.L0595b4:
        clr.w   d1                              | +006
        move.b  (a0)+,d1                        | +008
        tst.l   d7                              | +00a
        bne.w   .L0595c2                        | +00c
        move.b  #0xff,d6                        | +010
.L0595c2:
        or.b    d1,d6                           | +014
        movem.l d0/d6-d7/a0,-(a7)               | +016
        beq.w   .L0595ea                        | +01a
        addi.w  #0x3460,d1                      | +01e
        movem.w d0-d1,0x3c0000.l                | +022
        addq.w  #0x1,d0                         | +02a
        addi.w  #0x10,d1                        | +02c
        movem.w d0-d1,0x3c0000.l                | +030
        bra.w   .L059604                        | +038
.L0595ea:
        move.w  #0xb40,d1                       | +03c
        movem.w d0-d1,0x3c0000.l                | +040
        addq.w  #0x1,d0                         | +048
        addi.w  #0x10,d1                        | +04a
        movem.w d0-d1,0x3c0000.l                | +04e
.L059604:
        movem.l (a7)+,d0/d6-d7/a0               | +056
        addi.w  #0x20,d0                        | +05a
.L05960c:
        dbra    d7,.L0595b4                     | +05e
        rts                                     | +062
        .global Result_DrawDigits_0595ae__L059612
Result_DrawDigits_0595ae__L059612:
.L059612:
        andi.l  #0xff,d1                        | +064
        divu.w  #0xa,d1                         | +06a
        swap    d1                              | +06e
        move.w  d1,d2                           | +070
        clr.w   d1                              | +072
        swap    d1                              | +074
        divu.w  #0xa,d1                         | +076
        swap    d1                              | +07a
        lsl.w   #0x8,d1                         | +07c
        or.w    d1,d2                           | +07e
        clr.w   d1                              | +080
        swap    d1                              | +082
        swap    d2                              | +084
        divu.w  #0xa,d1                         | +086
        swap    d1                              | +08a
        move.w  d1,d2                           | +08c
        swap    d2                              | +08e
        move.l  d2,-(a7)                        | +090
        lea     0x1(a7),a0                      | +092
        moveq   #3,d7                           | +096
        bsr.w   Result_DrawDigits_0595ae        | +098
        move.l  (a7)+,d2                        | +09c
        rts                                     | +09e

| ----------------------------------------------------------------------------
|  Result_TickSound_05964e  @ $05964E  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Result_TickSound_05964e, "ax", @progbits
        .global Result_TickSound_05964e
Result_TickSound_05964e:
        move.b  0x30(a6),d0                     | +000
        andi.b  #0x3,d0                         | +004
        bne.w   JsrAbsRts_059664                | +008
        move.w  #0x10d8,d0                      | +00c

| ----------------------------------------------------------------------------
|  Result_PlayerPanel_Init_059666  @ $059666  (180 B)
| ----------------------------------------------------------------------------
        .section .text.Result_PlayerPanel_Init_059666, "ax", @progbits
        .global Result_PlayerPanel_Init_059666
Result_PlayerPanel_Init_059666:
        tst.b   0x98(a6)                        | +000
        bne.w   .L0596a6                        | +004
        cmpi.b  #0x1,0x10e3b8.l                 | +008
        beq.w   .L059686                        | +010
        jsr     0x5b6.l                         | +014
        jmp     0x518.l                         | +01a
.L059686:
        move.w  #0x7040,d0                      | +020
        move.l  #0x106e94,0x72(a6)              | +024
        move.b  0x10e3ba.l,0x77(a6)             | +02c
        move.b  0x10e3bc.l,0x76(a6)             | +034
        bra.w   .L0596da                        | +03c
.L0596a6:
        cmpi.b  #0x1,0x10e3b9.l                 | +040
        beq.w   .L0596be                        | +048
        jsr     0x5b6.l                         | +04c
        jmp     0x518.l                         | +052
.L0596be:
        move.w  #0x72a0,d0                      | +058
        move.l  #0x106e9c,0x72(a6)              | +05c
        move.b  0x10e3bb.l,0x77(a6)             | +064
        move.b  0x10e3bd.l,0x76(a6)             | +06c
.L0596da:
        move.w  d0,0x70(a6)                     | +074
        tst.b   0x98(a6)                        | +078
        bne.w   .L0596ee                        | +07c
        lea     ResultText_Strings_05934e__L059386(pc),a2 | +080
        bra.w   .L0596f2                        | +084
.L0596ee:
        lea     ResultText_Strings_05934e__L059393(pc),a2 | +088
.L0596f2:
        movea.l #0xa,a1                         | +08c
        adda.w  0x70(a6),a1                     | +092
        move.b  #0x3,d1                         | +096
        jsr     0x477fc.l                       | +09a
        move.w  #0xf,0x30(a6)                   | +0a0
        lea     .L059712(pc),a1                 | +0a6
        move.l  a1,(a6)                         | +0aa
.L059712:
        subq.w  #0x1,0x30(a6)                   | +0ac
        bne.w   SetHandlerRts_059720            | +0b0

| ----------------------------------------------------------------------------
|  Result_PrintContinueLabel_059722  @ $059722  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Result_PrintContinueLabel_059722, "ax", @progbits
        .global Result_PrintContinueLabel_059722
Result_PrintContinueLabel_059722:
        movea.l #0xd,a1                         | +000
        adda.w  0x70(a6),a1                     | +006
        lea     ResultText_Strings_05934e__L0593a6(pc),a2 | +00a
        move.b  #0x3,d1                         | +00e
        jsr     0x477fc.l                       | +012
        move.w  #0x8,0x30(a6)                   | +018
        lea     .L059746(pc),a1                 | +01e
        move.l  a1,(a6)                         | +022
.L059746:
        subq.w  #0x1,0x30(a6)                   | +024
        bne.w   SetHandlerRts_059754            | +028

| ----------------------------------------------------------------------------
|  Result_RollContinues_059756  @ $059756  (62 B)
| ----------------------------------------------------------------------------
        .section .text.Result_RollContinues_059756, "ax", @progbits
        .global Result_RollContinues_059756
Result_RollContinues_059756:
        move.b  #0x1e,0x30(a6)                  | +000
        lea     .L059762(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L059762:
        bsr.w   Result_TickSound_05964e         | +00c
        subq.b  #0x1,0x30(a6)                   | +010
        beq.w   .L05977a                        | +014
        jsr     0x5e9b6.l                       | +018
        move.b  d0,d1                           | +01e
        bra.w   .L059784                        | +020
.L05977a:
        move.b  0x77(a6),d1                     | +024
        lea     Result_Wait15_059794(pc),a1     | +028
        move.l  a1,(a6)                         | +02c
.L059784:
        movea.l #0x1ad,a1                       | +02e
        move.l  a1,d0                           | +034
        add.w   0x70(a6),d0                     | +036
        bra.w   Result_DrawDigits_0595ae__L059612 | +03a

| ----------------------------------------------------------------------------
|  Result_Wait15_059794  @ $059794  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Result_Wait15_059794, "ax", @progbits
        .global Result_Wait15_059794
Result_Wait15_059794:
        move.w  #0xf,0x30(a6)                   | +000
        lea     .L0597a0(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L0597a0:
        subq.w  #0x1,0x30(a6)                   | +00c
        bne.w   SetHandlerRts_0597ae            | +010

| ----------------------------------------------------------------------------
|  Result_PrintPrisonerLabel_0597b0  @ $0597B0  (82 B)
| ----------------------------------------------------------------------------
        .section .text.Result_PrintPrisonerLabel_0597b0, "ax", @progbits
        .global Result_PrintPrisonerLabel_0597b0
Result_PrintPrisonerLabel_0597b0:
        tst.b   0x10fd83.l                      | +000
        bne.w   .L0597d6                        | +006
        movea.l #0x10,a1                        | +00a
        adda.w  0x70(a6),a1                     | +010
        lea     ResultText_Strings_05934e__L0593cc(pc),a2 | +014
        move.b  #0x9,d1                         | +018
        jsr     0x47888.l                       | +01c
        bra.w   .L0597ee                        | +022
.L0597d6:
        movea.l #0x10,a1                        | +026
        adda.w  0x70(a6),a1                     | +02c
        lea     ResultText_Strings_05934e__L0593b5(pc),a2 | +030
        move.w  #0x2300,d0                      | +034
        jsr     0x5dad8.l                       | +038
.L0597ee:
        move.w  #0x8,0x30(a6)                   | +03e
        lea     .L0597fa(pc),a1                 | +044
        move.l  a1,(a6)                         | +048
.L0597fa:
        subq.w  #0x1,0x30(a6)                   | +04a
        bne.w   SetHandlerRts_059808            | +04e

| ----------------------------------------------------------------------------
|  Result_RollPrisoners_05980a  @ $05980A  (62 B)
| ----------------------------------------------------------------------------
        .section .text.Result_RollPrisoners_05980a, "ax", @progbits
        .global Result_RollPrisoners_05980a
Result_RollPrisoners_05980a:
        move.b  #0x1e,0x30(a6)                  | +000
        lea     .L059816(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L059816:
        bsr.w   Result_TickSound_05964e         | +00c
        subq.b  #0x1,0x30(a6)                   | +010
        beq.w   .L05982e                        | +014
        jsr     0x5e9b6.l                       | +018
        move.b  d0,d1                           | +01e
        bra.w   .L059838                        | +020
.L05982e:
        lea     Result_Wait15_B_059848(pc),a1   | +024
        move.l  a1,(a6)                         | +028
        move.b  0x76(a6),d1                     | +02a
.L059838:
        movea.l #0x1b0,a1                       | +02e
        move.l  a1,d0                           | +034
        add.w   0x70(a6),d0                     | +036
        bra.w   Result_DrawDigits_0595ae__L059612 | +03a

| ----------------------------------------------------------------------------
|  Result_Wait15_B_059848  @ $059848  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Result_Wait15_B_059848, "ax", @progbits
        .global Result_Wait15_B_059848
Result_Wait15_B_059848:
        move.w  #0xf,0x30(a6)                   | +000
        lea     .L059854(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L059854:
        subq.w  #0x1,0x30(a6)                   | +00c
        bne.w   SetHandlerRts_059862            | +010

| ----------------------------------------------------------------------------
|  Result_PrintScore_059864  @ $059864  (66 B)
| ----------------------------------------------------------------------------
        .section .text.Result_PrintScore_059864, "ax", @progbits
        .global Result_PrintScore_059864
Result_PrintScore_059864:
        movea.l #0x14,a1                        | +000
        adda.w  0x70(a6),a1                     | +006
        lea     ResultText_Strings_05934e__L0593a0(pc),a2 | +00a
        move.b  #0x3,d1                         | +00e
        jsr     0x477fc.l                       | +012
        movea.l #0x114,a1                       | +018
        move.l  a1,d0                           | +01e
        add.w   0x70(a6),d0                     | +020
        movea.l 0x72(a6),a0                     | +024
        moveq   #8,d7                           | +028
        bsr.w   Result_DrawDigits_0595ae        | +02a
        move.w  #0x1e,0x30(a6)                  | +02e
        lea     .L05989e(pc),a1                 | +034
        move.l  a1,(a6)                         | +038
.L05989e:
        subq.w  #0x1,0x30(a6)                   | +03a
        bne.w   SetHandlerRts_0598ac            | +03e

| ----------------------------------------------------------------------------
|  Result_HiScoreEntry_0598ae  @ $0598AE  (64 B)
| ----------------------------------------------------------------------------
        .section .text.Result_HiScoreEntry_0598ae, "ax", @progbits
        .global Result_HiScoreEntry_0598ae
Result_HiScoreEntry_0598ae:
        move.b  #0xff,0x21(a6)                  | +000
        movea.l 0x72(a6),a0                     | +006
        jsr     0x51aa4.l                       | +00a
        movea.l #0x78,a1                        | +010
        adda.w  0x70(a6),a1                     | +016
        tst.b   0x98(a6)                        | +01a
        bne.w   .L0598da                        | +01e
        jsr     0x97a60.l                       | +022
        bra.w   .L0598e0                        | +028
.L0598da:
        jsr     0x97a72.l                       | +02c
.L0598e0:
        lea     .L0598e6(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L0598e6:
        tst.b   0x21(a6)                        | +038
        bne.w   Jsr5B6Rts_0598fa                | +03c

| ----------------------------------------------------------------------------
|  Ending_SpawnOrbitFx_0598fc  @ $0598FC  (70 B)
| ----------------------------------------------------------------------------
        .section .text.Ending_SpawnOrbitFx_0598fc, "ax", @progbits
        .global Ending_SpawnOrbitFx_0598fc
Ending_SpawnOrbitFx_0598fc:
        move.l  a6,-(a7)                        | +000
        lea     0x100800.l,a6                   | +002
        lea     EndingOrbit_Parent_059a86(pc),a1 | +008
        jsr     0x4ae.l                         | +00c
        movea.l (a7)+,a6                        | +012
        move.w  #0x1,d0                         | +014
        lea     ResultText_Strings_05934e__L0593d8(pc),a0 | +018
        jsr     0x2b58.l                        | +01c
        jsr     0x523b2.l                       | +022
        move.w  #0x2b,d0                        | +028
        jsr     0x2352.l                        | +02c
        move.w  #0x1e,0x30(a6)                  | +032
        lea     .L05993a(pc),a1                 | +038
        move.l  a1,(a6)                         | +03c
.L05993a:
        subq.w  #0x1,0x30(a6)                   | +03e
        bne.w   SetHandlerRts_059948            | +042

| ----------------------------------------------------------------------------
|  Ending_Seq_ShowMission_05994a  @ $05994A  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Ending_Seq_ShowMission_05994a, "ax", @progbits
        .global Ending_Seq_ShowMission_05994a
Ending_Seq_ShowMission_05994a:
        lea     Ending_ShowMission_0593fa(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        lea     .L05995a(pc),a1                 | +00a
        move.l  a1,(a6)                         | +00e
.L05995a:
        jsr     0x6f0.l                         | +010
        bcs.w   SetHandlerRts_05996a            | +016

| ----------------------------------------------------------------------------
|  Ending_Seq_Wait30_05996c  @ $05996C  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Ending_Seq_Wait30_05996c, "ax", @progbits
        .global Ending_Seq_Wait30_05996c
Ending_Seq_Wait30_05996c:
        move.w  #0x1e,0x30(a6)                  | +000
        lea     .L059978(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L059978:
        subq.w  #0x1,0x30(a6)                   | +00c
        bne.w   SetHandlerRts_059986            | +010

| ----------------------------------------------------------------------------
|  Ending_Seq_Wipe_059988  @ $059988  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Ending_Seq_Wipe_059988, "ax", @progbits
        .global Ending_Seq_Wipe_059988
Ending_Seq_Wipe_059988:
        lea     Ending_WipeAllOver_0594c6(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        lea     .L059998(pc),a1                 | +00a
        move.l  a1,(a6)                         | +00e
.L059998:
        jsr     0x6f0.l                         | +010
        bcs.w   SetHandlerRts_0599a8            | +016

| ----------------------------------------------------------------------------
|  Ending_Seq_Wait15_0599aa  @ $0599AA  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Ending_Seq_Wait15_0599aa, "ax", @progbits
        .global Ending_Seq_Wait15_0599aa
Ending_Seq_Wait15_0599aa:
        move.w  #0xf,0x30(a6)                   | +000
        lea     .L0599b6(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L0599b6:
        subq.w  #0x1,0x30(a6)                   | +00c
        bne.w   SetHandlerRts_0599c4            | +010

| ----------------------------------------------------------------------------
|  Ending_Seq_PanelP1_0599c6  @ $0599C6  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Ending_Seq_PanelP1_0599c6, "ax", @progbits
        .global Ending_Seq_PanelP1_0599c6
Ending_Seq_PanelP1_0599c6:
        lea     Result_PlayerPanel_Init_059666(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        move.b  #0x0,0x98(a0)                   | +00a
        move.w  #0x4,0x30(a6)                   | +010
        lea     .L0599e2(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L0599e2:
        subq.w  #0x1,0x30(a6)                   | +01c
        bne.w   SetHandlerRts_0599f0            | +020

| ----------------------------------------------------------------------------
|  Ending_Seq_PanelP2_0599f2  @ $0599F2  (32 B)
| ----------------------------------------------------------------------------
        .section .text.Ending_Seq_PanelP2_0599f2, "ax", @progbits
        .global Ending_Seq_PanelP2_0599f2
Ending_Seq_PanelP2_0599f2:
        lea     Result_PlayerPanel_Init_059666(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        move.b  #0x1,0x98(a0)                   | +00a
        lea     .L059a08(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L059a08:
        jsr     0x6f0.l                         | +016
        bcs.w   SetHandlerRts_059a18            | +01c

| ----------------------------------------------------------------------------
|  Ending_Seq_FadeA0_059a1a  @ $059A1A  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Ending_Seq_FadeA0_059a1a, "ax", @progbits
        .global Ending_Seq_FadeA0_059a1a
Ending_Seq_FadeA0_059a1a:
        move.b  #0xa0,d1                        | +000
        jsr     0x2308.l                        | +004
        move.w  #0xc8,0x30(a6)                  | +00a
        lea     .L059a30(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L059a30:
        subq.w  #0x1,0x30(a6)                   | +016
        bne.w   SetHandlerRts_059a3e            | +01a

| ----------------------------------------------------------------------------
|  Ending_Seq_Fade40_059a40  @ $059A40  (40 B)
| ----------------------------------------------------------------------------
        .section .text.Ending_Seq_Fade40_059a40, "ax", @progbits
        .global Ending_Seq_Fade40_059a40
Ending_Seq_Fade40_059a40:
        move.w  #0x1,d0                         | +000
        jsr     0x5239e.l                       | +004
        move.b  #0x40,d0                        | +00a
        jsr     0x2308.l                        | +00e
        move.w  #0x3c,0x30(a6)                  | +014
        lea     .L059a60(pc),a1                 | +01a
        move.l  a1,(a6)                         | +01e
.L059a60:
        subq.w  #0x1,0x30(a6)                   | +020
        bne.w   SetHandlerRts_059a6e            | +024

| ----------------------------------------------------------------------------
|  Ending_Seq_Done_059a70  @ $059A70  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Ending_Seq_Done_059a70, "ax", @progbits
        .global Ending_Seq_Done_059a70
Ending_Seq_Done_059a70:
        movea.l 0xc(a6),a0                      | +000
        move.b  #0xff,0x20(a0)                  | +004
        jsr     0x5b6.l                         | +00a
        jmp     0x518.l                         | +010

| ----------------------------------------------------------------------------
|  EndingOrbit_Parent_059a86  @ $059A86  (128 B)
| ----------------------------------------------------------------------------
        .section .text.EndingOrbit_Parent_059a86, "ax", @progbits
        .global EndingOrbit_Parent_059a86
EndingOrbit_Parent_059a86:
        lea     EndingOrbit_Child_059b06(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        move.l  #0x24e208,0x3c(a0)              | +00a
        move.w  #0x0,0x34(a0)                   | +012
        lea     EndingOrbit_Child_059b06(pc),a1 | +018
        jsr     0x4ae.l                         | +01c
        bset    #0x0,0x3a(a0)                   | +022
        move.l  #0x24e030,0x3c(a0)              | +028
        move.w  #0x4000,0x34(a0)                | +030
        lea     EndingOrbit_Child_059b06(pc),a1 | +036
        jsr     0x4ae.l                         | +03a
        bset    #0x0,0x3a(a0)                   | +040
        move.l  #0x24e208,0x3c(a0)              | +046
        move.w  #0x8000,0x34(a0)                | +04e
        lea     EndingOrbit_Child_059b06(pc),a1 | +054
        jsr     0x4ae.l                         | +058
        move.l  #0x24e030,0x3c(a0)              | +05e
        move.w  #0xc000,0x34(a0)                | +066
        move.w  #0x4000,0x34(a6)                | +06c
        lea     .L059afe(pc),a1                 | +072
        move.l  a1,(a6)                         | +076
.L059afe:
        subi.w  #0x100,0x34(a6)                 | +078
        rts                                     | +07e

| ----------------------------------------------------------------------------
|  EndingOrbit_Child_059b06  @ $059B06  (76 B)
| ----------------------------------------------------------------------------
        .section .text.EndingOrbit_Child_059b06, "ax", @progbits
        .global EndingOrbit_Child_059b06
EndingOrbit_Child_059b06:
        move.w  #0x15f,d1                       | +000
        jsr     0x236e.l                        | +004
        move.w  #0x180,0x24(a6)                 | +00a
        lea     .L059b1c(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L059b1c:
        movea.l 0xc(a6),a0                      | +016
        move.w  0x34(a0),d0                     | +01a
        add.w   0x34(a6),d0                     | +01e
        bmi.w   JsrAbsRts_059b58                | +022
        lsr.w   #0x8,d0                         | +026
        move.w  d0,-(a7)                        | +028
        move.w  #0xff,d1                        | +02a
        jsr     0x13c0e.l                       | +02e
        move.b  d2,0x32(a6)                     | +034
        move.w  (a7)+,d0                        | +038
        move.w  #0x88,d1                        | +03a
        jsr     0x13c0e.l                       | +03e
        addi.w  #0xa0,d1                        | +044
        move.w  d1,0x22(a6)                     | +048

| ----------------------------------------------------------------------------
|  Str_PeaceForever_059b5a  @ $059B5A  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Str_PeaceForever_059b5a, "ax", @progbits
        .global Str_PeaceForever_059b5a
Str_PeaceForever_059b5a:
        .dc.b   0x50                          | +000  'P'  (dato, rango --data)
        .dc.b   0x45                          | +001  'E'  (dato, rango --data)
        .dc.b   0x41                          | +002  'A'  (dato, rango --data)
        .dc.b   0x43                          | +003  'C'  (dato, rango --data)
        .dc.b   0x45                          | +004  'E'  (dato, rango --data)
        .dc.b   0x20                          | +005  ' '  (dato, rango --data)
        .dc.b   0x46                          | +006  'F'  (dato, rango --data)
        .dc.b   0x4f                          | +007  'O'  (dato, rango --data)
        .dc.b   0x52                          | +008  'R'  (dato, rango --data)
        .dc.b   0x45                          | +009  'E'  (dato, rango --data)
        .dc.b   0x56                          | +00a  'V'  (dato, rango --data)
        .dc.b   0x45                          | +00b  'E'  (dato, rango --data)
        .dc.b   0x52                          | +00c  'R'  (dato, rango --data)
        .dc.b   0x21                          | +00d  '!'  (dato, rango --data)
        .dc.b   0xff                          | +00e  '.'  (dato, rango --data)
        .dc.b   0x00                          | +00f  '.'  (dato, rango --data)

| ----------------------------------------------------------------------------
|  Ending_PeaceWait_059b6a  @ $059B6A  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Ending_PeaceWait_059b6a, "ax", @progbits
        .global Ending_PeaceWait_059b6a
Ending_PeaceWait_059b6a:
        move.w  #0x2d,0x30(a6)                  | +000
        lea     .L059b76(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L059b76:
        subq.w  #0x1,0x30(a6)                   | +00c
        bne.w   SetHandlerRts_059b84            | +010

| ----------------------------------------------------------------------------
|  Ending_ShowPeaceForever_059b86  @ $059B86  (56 B)
| ----------------------------------------------------------------------------
        .section .text.Ending_ShowPeaceForever_059b86, "ax", @progbits
        .global Ending_ShowPeaceForever_059b86
Ending_ShowPeaceForever_059b86:
        lea     0x4737e.l,a1                    | +000
        jsr     0x4ae.l                         | +006
        move.w  #0x711a,d0                      | +00c
        move.w  d0,0x22(a0)                     | +010
        move.l  #0x59b5a,0x3c(a0)               | +014
        move.w  #0x1,0x30(a0)                   | +01c
        move.b  #0x4,0x16(a0)                   | +022
        lea     .L059bb4(pc),a1                 | +028
        move.l  a1,(a6)                         | +02c
.L059bb4:
        jsr     0x6f0.l                         | +02e
        bcs.w   SetHandlerRts_059bc4            | +034

| ----------------------------------------------------------------------------
|  Ending_PeaceExit_059bc6  @ $059BC6  (12 B)
| ----------------------------------------------------------------------------
        .section .text.Ending_PeaceExit_059bc6, "ax", @progbits
        .global Ending_PeaceExit_059bc6
Ending_PeaceExit_059bc6:
        jsr     0x5b6.l                         | +000
        jmp     0x518.l                         | +006

| ----------------------------------------------------------------------------
|  Entity_CmpDepthToParent_059bd2  @ $059BD2  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpDepthToParent_059bd2, "ax", @progbits
        .global Entity_CmpDepthToParent_059bd2
Entity_CmpDepthToParent_059bd2:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_059be8                    | +00c

| ----------------------------------------------------------------------------
|  Gunner_InitShadeRingId_059bee  @ $059BEE  (28 B)
| ----------------------------------------------------------------------------
        .section .text.Gunner_InitShadeRingId_059bee, "ax", @progbits
        .global Gunner_InitShadeRingId_059bee
Gunner_InitShadeRingId_059bee:
        move.w  #0x8000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x4,0x38(a6)                   | +010
        jsr     0x8f3a6.l                       | +016

| ----------------------------------------------------------------------------
|  Gunner_ProbeOrDie_059c10  @ $059C10  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Gunner_ProbeOrDie_059c10, "ax", @progbits
        .global Gunner_ProbeOrDie_059c10
Gunner_ProbeOrDie_059c10:
        lea     0x2b73b4.l,a0                   | +000
        jsr     0x5dd5c.l                       | +006
        bcc.w   Jsr5B6Rts_059c2c                | +00c

| ----------------------------------------------------------------------------
|  Gunner_Boot_059c2e  @ $059C2E  (12 B)
| ----------------------------------------------------------------------------
        .section .text.Gunner_Boot_059c2e, "ax", @progbits
        .global Gunner_Boot_059c2e
Gunner_Boot_059c2e:
        bsr.b   Gunner_InitShadeRingId_059bee   | +000
        lea     Gunner_Child_Init_059d50(pc),a1 | +002
        jsr     0x4ae.l                         | +006

| ----------------------------------------------------------------------------
|  Gunner_Search_059c42  @ $059C42  (94 B)
| ----------------------------------------------------------------------------
        .section .text.Gunner_Search_059c42, "ax", @progbits
        .global Gunner_Search_059c42
Gunner_Search_059c42:
        lea     0x2b732c.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L059c54(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L059c54:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        move.w  0x22(a6),d0                     | +01e
        cmpi.w  #0x20,d0                        | +022
        ble.w   .L059c9c                        | +026
        move.w  0x24(a6),d1                     | +02a
        move.b  0x70(a6),d2                     | +02e
        jsr     0x8f3be.l                       | +032
        tst.b   d4                              | +038
        bmi.w   .L059c9c                        | +03a
        move.w  d4,-(a7)                        | +03e
        jsr     0x13600.l                       | +040
        move.w  (a7)+,d4                        | +046
        jsr     0x8f69c.l                       | +048
        jsr     0x236e.l                        | +04e
        lea     Gunner_Fire_059ca0(pc),a1       | +054
        move.l  a1,(a6)                         | +058
.L059c9c:
        bra.w   Gunner_ProbeOrDie_059c10        | +05a

| ----------------------------------------------------------------------------
|  Gunner_Fire_059ca0  @ $059CA0  (74 B)
| ----------------------------------------------------------------------------
        .section .text.Gunner_Fire_059ca0, "ax", @progbits
        .global Gunner_Fire_059ca0
Gunner_Fire_059ca0:
        lea     0x2b7340.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        move.b  #0x3,0x76(a6)                   | +00c
        lea     .L059cb8(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L059cb8:
        jsr     0x2783a.l                       | +018
        jsr     0x28d70.l                       | +01e
        bcc.w   .L059ce6                        | +024
        subq.b  #0x1,0x76(a6)                   | +028
        beq.w   .L059ce0                        | +02c
        lea     0x2b7340.l,a0                   | +030
        jsr     0x28cd4.l                       | +036
        bra.w   .L059ce6                        | +03c
.L059ce0:
        lea     Gunner_Reload_059cea(pc),a1     | +040
        move.l  a1,(a6)                         | +044
.L059ce6:
        bra.w   Gunner_Reload_059cea__L059d1c   | +046

| ----------------------------------------------------------------------------
|  Gunner_Reload_059cea  @ $059CEA  (102 B)
| ----------------------------------------------------------------------------
        .section .text.Gunner_Reload_059cea, "ax", @progbits
        .global Gunner_Reload_059cea
Gunner_Reload_059cea:
        move.b  #0x78,0x76(a6)                  | +000
        lea     0x2b737c.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     .L059d02(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L059d02:
        jsr     0x2783a.l                       | +018
        jsr     0x28d70.l                       | +01e
        subq.b  #0x1,0x76(a6)                   | +024
        bne.w   .L059d1c                        | +028
        lea     Gunner_Fire_059ca0(pc),a1       | +02c
        move.l  a1,(a6)                         | +030
        .global Gunner_Reload_059cea__L059d1c
Gunner_Reload_059cea__L059d1c:
.L059d1c:
        move.w  0x22(a6),d0                     | +032
        cmpi.w  #0x20,d0                        | +036
        ble.w   .L059d46                        | +03a
        move.w  0x24(a6),d1                     | +03e
        move.b  0x70(a6),d2                     | +042
        jsr     0x8f3be.l                       | +046
        tst.b   d4                              | +04c
        bpl.w   .L059d42                        | +04e
        lea     Gunner_Search_059c42(pc),a1     | +052
        move.l  a1,(a6)                         | +056
.L059d42:
        bra.w   .L059d4c                        | +058
.L059d46:
        lea     Gunner_Search_059c42(pc),a1     | +05c
        move.l  a1,(a6)                         | +060
.L059d4c:
        bra.w   Gunner_ProbeOrDie_059c10        | +062

| ----------------------------------------------------------------------------
|  Gunner_Child_Init_059d50  @ $059D50  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Gunner_Child_Init_059d50, "ax", @progbits
        .global Gunner_Child_Init_059d50
Gunner_Child_Init_059d50:
        move.w  #0x56,d1                        | +000
        jsr     0x236e.l                        | +004

| ----------------------------------------------------------------------------
|  Gunner_Child_Sync_059d62  @ $059D62  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Gunner_Child_Sync_059d62, "ax", @progbits
        .global Gunner_Child_Sync_059d62
Gunner_Child_Sync_059d62:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x22(a0),0x22(a6)               | +004
        move.w  0x24(a0),0x24(a6)               | +00a
        move.w  0x38(a0),0x38(a6)               | +010
        move.l  0x72(a0),0x3c(a6)               | +016
        jmp     0x5ca2a.l                       | +01c

| ----------------------------------------------------------------------------
|  Gunner_SpawnShell_059d84  @ $059D84  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Gunner_SpawnShell_059d84, "ax", @progbits
        .global Gunner_SpawnShell_059d84
Gunner_SpawnShell_059d84:
        move.l  a6,-(a7)                        | +000
        lea     0x100800.l,a6                   | +002
        lea     Gunner_Shell_059da6(pc),a1      | +008
        jsr     0x4ae.l                         | +00c
        movea.l (a7)+,a6                        | +012
        move.w  0x22(a6),0x22(a0)               | +014
        move.w  0x24(a6),0x24(a0)               | +01a
        rts                                     | +020

| ----------------------------------------------------------------------------
|  Gunner_Shell_059da6  @ $059DA6  (142 B)
| ----------------------------------------------------------------------------
        .section .text.Gunner_Shell_059da6, "ax", @progbits
        .global Gunner_Shell_059da6
Gunner_Shell_059da6:
        move.w  #0x108c,d0                      | +000
        jsr     0x2352.l                        | +004
        move.w  #0x57,d1                        | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0xd000,0x38(a6)                | +014
        addi.w  #0x20,0x22(a6)                  | +01a
        addi.w  #0x14,0x24(a6)                  | +020
        move.w  #0x400,0x28(a6)                 | +026
        lea     0x2b7390.l,a0                   | +02c
        jsr     0x28cd4.l                       | +032
        lea     0x2b73bc.l,a0                   | +038
        move.l  a0,0x4c(a6)                     | +03e
        jsr     0x283ca.l                       | +042
        jsr     0x283ca.l                       | +048
        lea     .L059dfa(pc),a1                 | +04e
        move.l  a1,(a6)                         | +052
.L059dfa:
        jsr     0x27cee.l                       | +054
        move.w  0x22(a6),d0                     | +05a
        addi.w  #0x10,d0                        | +05e
        subi.w  #0x160,d0                       | +062
        bcs.w   .L059e16                        | +066
        jmp     0x518.l                         | +06a
.L059e16:
        jsr     0x28d70.l                       | +070
        jsr     0x283d8.l                       | +076
        btst    #0x1,0x13(a6)                   | +07c
        beq.w   .L059e32                        | +082
        jmp     0x518.l                         | +086
.L059e32:
        rts                                     | +08c

| ----------------------------------------------------------------------------
|  Entity_CmpDepthToParent_059e34  @ $059E34  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpDepthToParent_059e34, "ax", @progbits
        .global Entity_CmpDepthToParent_059e34
Entity_CmpDepthToParent_059e34:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_059e4a                    | +00c

| ----------------------------------------------------------------------------
|  GunnerAnim_Table_059e50  @ $059E50  (830 B)
| ----------------------------------------------------------------------------
        .section .text.GunnerAnim_Table_059e50, "ax", @progbits
        .global GunnerAnim_Table_059e50
GunnerAnim_Table_059e50:
        .dc.b   0x07                          | +000  '.'  (dato, rango --data)
        .dc.b   0x00                          | +001  '.'  (dato, rango --data)
        .dc.b   0x00                          | +002  '.'  (dato, rango --data)
        .dc.b   0x24                          | +003  '$'  (dato, rango --data)
        .dc.b   0x29                          | +004  ')'  (dato, rango --data)
        .dc.b   0x86                          | +005  '.'  (dato, rango --data)
        .dc.b   0x00                          | +006  '.'  (dato, rango --data)
        .dc.b   0x5c                          | +007  '.'  (dato, rango --data)
        .dc.b   0x00                          | +008  '.'  (dato, rango --data)
        .dc.b   0x01                          | +009  '.'  (dato, rango --data)
        .dc.b   0x02                          | +00a  '.'  (dato, rango --data)
        .dc.b   0x08                          | +00b  '.'  (dato, rango --data)
        .dc.b   0xff                          | +00c  '.'  (dato, rango --data)
        .dc.b   0xff                          | +00d  '.'  (dato, rango --data)
        .dc.b   0xff                          | +00e  '.'  (dato, rango --data)
        .dc.b   0xff                          | +00f  '.'  (dato, rango --data)
        .dc.b   0xff                          | +010  '.'  (dato, rango --data)
        .dc.b   0xff                          | +011  '.'  (dato, rango --data)
        .dc.b   0x16                          | +012  '.'  (dato, rango --data)
        .dc.b   0x00                          | +013  '.'  (dato, rango --data)
        .dc.b   0x07                          | +014  '.'  (dato, rango --data)
        .dc.b   0x00                          | +015  '.'  (dato, rango --data)
        .dc.b   0x00                          | +016  '.'  (dato, rango --data)
        .dc.b   0x24                          | +017  '$'  (dato, rango --data)
        .dc.b   0x29                          | +018  ')'  (dato, rango --data)
        .dc.b   0xb0                          | +019  '.'  (dato, rango --data)
        .dc.b   0x00                          | +01a  '.'  (dato, rango --data)
        .dc.b   0x5c                          | +01b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +01c  '.'  (dato, rango --data)
        .dc.b   0x01                          | +01d  '.'  (dato, rango --data)
        .dc.b   0x02                          | +01e  '.'  (dato, rango --data)
        .dc.b   0x08                          | +01f  '.'  (dato, rango --data)
        .dc.b   0xff                          | +020  '.'  (dato, rango --data)
        .dc.b   0xff                          | +021  '.'  (dato, rango --data)
        .dc.b   0xff                          | +022  '.'  (dato, rango --data)
        .dc.b   0xff                          | +023  '.'  (dato, rango --data)
        .dc.b   0xff                          | +024  '.'  (dato, rango --data)
        .dc.b   0xff                          | +025  '.'  (dato, rango --data)
        .dc.b   0x16                          | +026  '.'  (dato, rango --data)
        .dc.b   0x00                          | +027  '.'  (dato, rango --data)
        .dc.b   0x07                          | +028  '.'  (dato, rango --data)
        .dc.b   0x00                          | +029  '.'  (dato, rango --data)
        .dc.b   0x00                          | +02a  '.'  (dato, rango --data)
        .dc.b   0x24                          | +02b  '$'  (dato, rango --data)
        .dc.b   0x29                          | +02c  ')'  (dato, rango --data)
        .dc.b   0xda                          | +02d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +02e  '.'  (dato, rango --data)
        .dc.b   0x5c                          | +02f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +030  '.'  (dato, rango --data)
        .dc.b   0x01                          | +031  '.'  (dato, rango --data)
        .dc.b   0x02                          | +032  '.'  (dato, rango --data)
        .dc.b   0x08                          | +033  '.'  (dato, rango --data)
        .dc.b   0xff                          | +034  '.'  (dato, rango --data)
        .dc.b   0xff                          | +035  '.'  (dato, rango --data)
        .dc.b   0xff                          | +036  '.'  (dato, rango --data)
        .dc.b   0xff                          | +037  '.'  (dato, rango --data)
        .dc.b   0xff                          | +038  '.'  (dato, rango --data)
        .dc.b   0xff                          | +039  '.'  (dato, rango --data)
        .dc.b   0x16                          | +03a  '.'  (dato, rango --data)
        .dc.b   0x00                          | +03b  '.'  (dato, rango --data)
        .dc.b   0x07                          | +03c  '.'  (dato, rango --data)
        .dc.b   0x00                          | +03d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +03e  '.'  (dato, rango --data)
        .dc.b   0x24                          | +03f  '$'  (dato, rango --data)
        .dc.b   0x29                          | +040  ')'  (dato, rango --data)
        .dc.b   0xf8                          | +041  '.'  (dato, rango --data)
        .dc.b   0x00                          | +042  '.'  (dato, rango --data)
        .dc.b   0x5c                          | +043  '.'  (dato, rango --data)
        .dc.b   0x00                          | +044  '.'  (dato, rango --data)
        .dc.b   0x01                          | +045  '.'  (dato, rango --data)
        .dc.b   0x02                          | +046  '.'  (dato, rango --data)
        .dc.b   0x08                          | +047  '.'  (dato, rango --data)
        .dc.b   0xff                          | +048  '.'  (dato, rango --data)
        .dc.b   0xff                          | +049  '.'  (dato, rango --data)
        .dc.b   0xff                          | +04a  '.'  (dato, rango --data)
        .dc.b   0xff                          | +04b  '.'  (dato, rango --data)
        .dc.b   0xff                          | +04c  '.'  (dato, rango --data)
        .dc.b   0xff                          | +04d  '.'  (dato, rango --data)
        .dc.b   0x16                          | +04e  '.'  (dato, rango --data)
        .dc.b   0x00                          | +04f  '.'  (dato, rango --data)
        .dc.b   0x07                          | +050  '.'  (dato, rango --data)
        .dc.b   0x00                          | +051  '.'  (dato, rango --data)
        .dc.b   0x00                          | +052  '.'  (dato, rango --data)
        .dc.b   0x24                          | +053  '$'  (dato, rango --data)
        .dc.b   0x2a                          | +054  '*'  (dato, rango --data)
        .dc.b   0x16                          | +055  '.'  (dato, rango --data)
        .dc.b   0x00                          | +056  '.'  (dato, rango --data)
        .dc.b   0x5c                          | +057  '.'  (dato, rango --data)
        .dc.b   0x00                          | +058  '.'  (dato, rango --data)
        .dc.b   0x01                          | +059  '.'  (dato, rango --data)
        .dc.b   0x02                          | +05a  '.'  (dato, rango --data)
        .dc.b   0x08                          | +05b  '.'  (dato, rango --data)
        .dc.b   0xff                          | +05c  '.'  (dato, rango --data)
        .dc.b   0xff                          | +05d  '.'  (dato, rango --data)
        .dc.b   0xff                          | +05e  '.'  (dato, rango --data)
        .dc.b   0xff                          | +05f  '.'  (dato, rango --data)
        .dc.b   0xff                          | +060  '.'  (dato, rango --data)
        .dc.b   0xff                          | +061  '.'  (dato, rango --data)
        .dc.b   0x16                          | +062  '.'  (dato, rango --data)
        .dc.b   0x00                          | +063  '.'  (dato, rango --data)
        .dc.b   0x07                          | +064  '.'  (dato, rango --data)
        .dc.b   0x00                          | +065  '.'  (dato, rango --data)
        .dc.b   0x00                          | +066  '.'  (dato, rango --data)
        .dc.b   0x24                          | +067  '$'  (dato, rango --data)
        .dc.b   0x28                          | +068  '('  (dato, rango --data)
        .dc.b   0xdc                          | +069  '.'  (dato, rango --data)
        .dc.b   0x00                          | +06a  '.'  (dato, rango --data)
        .dc.b   0x5c                          | +06b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +06c  '.'  (dato, rango --data)
        .dc.b   0x01                          | +06d  '.'  (dato, rango --data)
        .dc.b   0x02                          | +06e  '.'  (dato, rango --data)
        .dc.b   0x08                          | +06f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +070  '.'  (dato, rango --data)
        .dc.b   0x24                          | +071  '$'  (dato, rango --data)
        .dc.b   0x2d                          | +072  '-'  (dato, rango --data)
        .dc.b   0x84                          | +073  '.'  (dato, rango --data)
        .dc.b   0xff                          | +074  '.'  (dato, rango --data)
        .dc.b   0xff                          | +075  '.'  (dato, rango --data)
        .dc.b   0x16                          | +076  '.'  (dato, rango --data)
        .dc.b   0x00                          | +077  '.'  (dato, rango --data)
        .dc.b   0x07                          | +078  '.'  (dato, rango --data)
        .dc.b   0x00                          | +079  '.'  (dato, rango --data)
        .dc.b   0x00                          | +07a  '.'  (dato, rango --data)
        .dc.b   0x24                          | +07b  '$'  (dato, rango --data)
        .dc.b   0x29                          | +07c  ')'  (dato, rango --data)
        .dc.b   0x04                          | +07d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +07e  '.'  (dato, rango --data)
        .dc.b   0x5c                          | +07f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +080  '.'  (dato, rango --data)
        .dc.b   0x01                          | +081  '.'  (dato, rango --data)
        .dc.b   0x02                          | +082  '.'  (dato, rango --data)
        .dc.b   0x08                          | +083  '.'  (dato, rango --data)
        .dc.b   0x00                          | +084  '.'  (dato, rango --data)
        .dc.b   0x24                          | +085  '$'  (dato, rango --data)
        .dc.b   0x2d                          | +086  '-'  (dato, rango --data)
        .dc.b   0xd4                          | +087  '.'  (dato, rango --data)
        .dc.b   0xff                          | +088  '.'  (dato, rango --data)
        .dc.b   0xff                          | +089  '.'  (dato, rango --data)
        .dc.b   0x16                          | +08a  '.'  (dato, rango --data)
        .dc.b   0x00                          | +08b  '.'  (dato, rango --data)
        .dc.b   0x07                          | +08c  '.'  (dato, rango --data)
        .dc.b   0x00                          | +08d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +08e  '.'  (dato, rango --data)
        .dc.b   0x24                          | +08f  '$'  (dato, rango --data)
        .dc.b   0x29                          | +090  ')'  (dato, rango --data)
        .dc.b   0x2c                          | +091  ','  (dato, rango --data)
        .dc.b   0x00                          | +092  '.'  (dato, rango --data)
        .dc.b   0x5c                          | +093  '.'  (dato, rango --data)
        .dc.b   0x00                          | +094  '.'  (dato, rango --data)
        .dc.b   0x01                          | +095  '.'  (dato, rango --data)
        .dc.b   0x02                          | +096  '.'  (dato, rango --data)
        .dc.b   0x08                          | +097  '.'  (dato, rango --data)
        .dc.b   0x00                          | +098  '.'  (dato, rango --data)
        .dc.b   0x24                          | +099  '$'  (dato, rango --data)
        .dc.b   0x2e                          | +09a  '.'  (dato, rango --data)
        .dc.b   0x24                          | +09b  '$'  (dato, rango --data)
        .dc.b   0xff                          | +09c  '.'  (dato, rango --data)
        .dc.b   0xff                          | +09d  '.'  (dato, rango --data)
        .dc.b   0x16                          | +09e  '.'  (dato, rango --data)
        .dc.b   0x00                          | +09f  '.'  (dato, rango --data)
        .dc.b   0x07                          | +0a0  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0a1  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0a2  '.'  (dato, rango --data)
        .dc.b   0x24                          | +0a3  '$'  (dato, rango --data)
        .dc.b   0x29                          | +0a4  ')'  (dato, rango --data)
        .dc.b   0x4a                          | +0a5  'J'  (dato, rango --data)
        .dc.b   0x00                          | +0a6  '.'  (dato, rango --data)
        .dc.b   0x5c                          | +0a7  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0a8  '.'  (dato, rango --data)
        .dc.b   0x01                          | +0a9  '.'  (dato, rango --data)
        .dc.b   0x02                          | +0aa  '.'  (dato, rango --data)
        .dc.b   0x08                          | +0ab  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0ac  '.'  (dato, rango --data)
        .dc.b   0x24                          | +0ad  '$'  (dato, rango --data)
        .dc.b   0x2e                          | +0ae  '.'  (dato, rango --data)
        .dc.b   0x74                          | +0af  't'  (dato, rango --data)
        .dc.b   0xff                          | +0b0  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0b1  '.'  (dato, rango --data)
        .dc.b   0x16                          | +0b2  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0b3  '.'  (dato, rango --data)
        .dc.b   0x07                          | +0b4  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0b5  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0b6  '.'  (dato, rango --data)
        .dc.b   0x24                          | +0b7  '$'  (dato, rango --data)
        .dc.b   0x29                          | +0b8  ')'  (dato, rango --data)
        .dc.b   0x68                          | +0b9  'h'  (dato, rango --data)
        .dc.b   0x00                          | +0ba  '.'  (dato, rango --data)
        .dc.b   0x5c                          | +0bb  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0bc  '.'  (dato, rango --data)
        .dc.b   0x01                          | +0bd  '.'  (dato, rango --data)
        .dc.b   0x02                          | +0be  '.'  (dato, rango --data)
        .dc.b   0x08                          | +0bf  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0c0  '.'  (dato, rango --data)
        .dc.b   0x24                          | +0c1  '$'  (dato, rango --data)
        .dc.b   0x2e                          | +0c2  '.'  (dato, rango --data)
        .dc.b   0xc4                          | +0c3  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0c4  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0c5  '.'  (dato, rango --data)
        .dc.b   0x16                          | +0c6  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0c7  '.'  (dato, rango --data)
        .dc.b   0x07                          | +0c8  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0c9  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0ca  '.'  (dato, rango --data)
        .dc.b   0x24                          | +0cb  '$'  (dato, rango --data)
        .dc.b   0x2a                          | +0cc  '*'  (dato, rango --data)
        .dc.b   0x34                          | +0cd  '4'  (dato, rango --data)
        .dc.b   0x00                          | +0ce  '.'  (dato, rango --data)
        .dc.b   0x5c                          | +0cf  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0d0  '.'  (dato, rango --data)
        .dc.b   0x01                          | +0d1  '.'  (dato, rango --data)
        .dc.b   0x02                          | +0d2  '.'  (dato, rango --data)
        .dc.b   0x08                          | +0d3  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0d4  '.'  (dato, rango --data)
        .dc.b   0x24                          | +0d5  '$'  (dato, rango --data)
        .dc.b   0x2d                          | +0d6  '-'  (dato, rango --data)
        .dc.b   0x84                          | +0d7  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0d8  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0d9  '.'  (dato, rango --data)
        .dc.b   0x07                          | +0da  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0db  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0dc  '.'  (dato, rango --data)
        .dc.b   0x24                          | +0dd  '$'  (dato, rango --data)
        .dc.b   0x2a                          | +0de  '*'  (dato, rango --data)
        .dc.b   0x72                          | +0df  'r'  (dato, rango --data)
        .dc.b   0x00                          | +0e0  '.'  (dato, rango --data)
        .dc.b   0x5c                          | +0e1  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0e2  '.'  (dato, rango --data)
        .dc.b   0x01                          | +0e3  '.'  (dato, rango --data)
        .dc.b   0x02                          | +0e4  '.'  (dato, rango --data)
        .dc.b   0x08                          | +0e5  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0e6  '.'  (dato, rango --data)
        .dc.b   0x24                          | +0e7  '$'  (dato, rango --data)
        .dc.b   0x2d                          | +0e8  '-'  (dato, rango --data)
        .dc.b   0x98                          | +0e9  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0ea  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0eb  '.'  (dato, rango --data)
        .dc.b   0x07                          | +0ec  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0ed  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0ee  '.'  (dato, rango --data)
        .dc.b   0x24                          | +0ef  '$'  (dato, rango --data)
        .dc.b   0x2a                          | +0f0  '*'  (dato, rango --data)
        .dc.b   0xa6                          | +0f1  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0f2  '.'  (dato, rango --data)
        .dc.b   0x5c                          | +0f3  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0f4  '.'  (dato, rango --data)
        .dc.b   0x01                          | +0f5  '.'  (dato, rango --data)
        .dc.b   0x02                          | +0f6  '.'  (dato, rango --data)
        .dc.b   0x08                          | +0f7  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0f8  '.'  (dato, rango --data)
        .dc.b   0x24                          | +0f9  '$'  (dato, rango --data)
        .dc.b   0x2d                          | +0fa  '-'  (dato, rango --data)
        .dc.b   0xac                          | +0fb  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0fc  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0fd  '.'  (dato, rango --data)
        .dc.b   0x07                          | +0fe  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0ff  '.'  (dato, rango --data)
        .dc.b   0x00                          | +100  '.'  (dato, rango --data)
        .dc.b   0x24                          | +101  '$'  (dato, rango --data)
        .dc.b   0x2a                          | +102  '*'  (dato, rango --data)
        .dc.b   0xda                          | +103  '.'  (dato, rango --data)
        .dc.b   0x00                          | +104  '.'  (dato, rango --data)
        .dc.b   0x5c                          | +105  '.'  (dato, rango --data)
        .dc.b   0x00                          | +106  '.'  (dato, rango --data)
        .dc.b   0x01                          | +107  '.'  (dato, rango --data)
        .dc.b   0x02                          | +108  '.'  (dato, rango --data)
        .dc.b   0x08                          | +109  '.'  (dato, rango --data)
        .dc.b   0x00                          | +10a  '.'  (dato, rango --data)
        .dc.b   0x24                          | +10b  '$'  (dato, rango --data)
        .dc.b   0x2d                          | +10c  '-'  (dato, rango --data)
        .dc.b   0xc0                          | +10d  '.'  (dato, rango --data)
        .dc.b   0xff                          | +10e  '.'  (dato, rango --data)
        .dc.b   0xff                          | +10f  '.'  (dato, rango --data)
        .dc.b   0x16                          | +110  '.'  (dato, rango --data)
        .dc.b   0x00                          | +111  '.'  (dato, rango --data)
        .dc.b   0x07                          | +112  '.'  (dato, rango --data)
        .dc.b   0x00                          | +113  '.'  (dato, rango --data)
        .dc.b   0x00                          | +114  '.'  (dato, rango --data)
        .dc.b   0x24                          | +115  '$'  (dato, rango --data)
        .dc.b   0x2b                          | +116  '+'  (dato, rango --data)
        .dc.b   0x04                          | +117  '.'  (dato, rango --data)
        .dc.b   0x00                          | +118  '.'  (dato, rango --data)
        .dc.b   0x5c                          | +119  '.'  (dato, rango --data)
        .dc.b   0x00                          | +11a  '.'  (dato, rango --data)
        .dc.b   0x01                          | +11b  '.'  (dato, rango --data)
        .dc.b   0x02                          | +11c  '.'  (dato, rango --data)
        .dc.b   0x08                          | +11d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +11e  '.'  (dato, rango --data)
        .dc.b   0x24                          | +11f  '$'  (dato, rango --data)
        .dc.b   0x2d                          | +120  '-'  (dato, rango --data)
        .dc.b   0xd4                          | +121  '.'  (dato, rango --data)
        .dc.b   0xff                          | +122  '.'  (dato, rango --data)
        .dc.b   0xff                          | +123  '.'  (dato, rango --data)
        .dc.b   0x07                          | +124  '.'  (dato, rango --data)
        .dc.b   0x00                          | +125  '.'  (dato, rango --data)
        .dc.b   0x00                          | +126  '.'  (dato, rango --data)
        .dc.b   0x24                          | +127  '$'  (dato, rango --data)
        .dc.b   0x2b                          | +128  '+'  (dato, rango --data)
        .dc.b   0x3c                          | +129  '<'  (dato, rango --data)
        .dc.b   0x00                          | +12a  '.'  (dato, rango --data)
        .dc.b   0x5c                          | +12b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +12c  '.'  (dato, rango --data)
        .dc.b   0x01                          | +12d  '.'  (dato, rango --data)
        .dc.b   0x02                          | +12e  '.'  (dato, rango --data)
        .dc.b   0x08                          | +12f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +130  '.'  (dato, rango --data)
        .dc.b   0x24                          | +131  '$'  (dato, rango --data)
        .dc.b   0x2d                          | +132  '-'  (dato, rango --data)
        .dc.b   0xe8                          | +133  '.'  (dato, rango --data)
        .dc.b   0xff                          | +134  '.'  (dato, rango --data)
        .dc.b   0xff                          | +135  '.'  (dato, rango --data)
        .dc.b   0x07                          | +136  '.'  (dato, rango --data)
        .dc.b   0x00                          | +137  '.'  (dato, rango --data)
        .dc.b   0x00                          | +138  '.'  (dato, rango --data)
        .dc.b   0x24                          | +139  '$'  (dato, rango --data)
        .dc.b   0x2b                          | +13a  '+'  (dato, rango --data)
        .dc.b   0x6a                          | +13b  'j'  (dato, rango --data)
        .dc.b   0x00                          | +13c  '.'  (dato, rango --data)
        .dc.b   0x5c                          | +13d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +13e  '.'  (dato, rango --data)
        .dc.b   0x01                          | +13f  '.'  (dato, rango --data)
        .dc.b   0x02                          | +140  '.'  (dato, rango --data)
        .dc.b   0x08                          | +141  '.'  (dato, rango --data)
        .dc.b   0x00                          | +142  '.'  (dato, rango --data)
        .dc.b   0x24                          | +143  '$'  (dato, rango --data)
        .dc.b   0x2d                          | +144  '-'  (dato, rango --data)
        .dc.b   0xfc                          | +145  '.'  (dato, rango --data)
        .dc.b   0xff                          | +146  '.'  (dato, rango --data)
        .dc.b   0xff                          | +147  '.'  (dato, rango --data)
        .dc.b   0x07                          | +148  '.'  (dato, rango --data)
        .dc.b   0x00                          | +149  '.'  (dato, rango --data)
        .dc.b   0x00                          | +14a  '.'  (dato, rango --data)
        .dc.b   0x24                          | +14b  '$'  (dato, rango --data)
        .dc.b   0x2b                          | +14c  '+'  (dato, rango --data)
        .dc.b   0x98                          | +14d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +14e  '.'  (dato, rango --data)
        .dc.b   0x5c                          | +14f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +150  '.'  (dato, rango --data)
        .dc.b   0x01                          | +151  '.'  (dato, rango --data)
        .dc.b   0x02                          | +152  '.'  (dato, rango --data)
        .dc.b   0x08                          | +153  '.'  (dato, rango --data)
        .dc.b   0x00                          | +154  '.'  (dato, rango --data)
        .dc.b   0x24                          | +155  '$'  (dato, rango --data)
        .dc.b   0x2e                          | +156  '.'  (dato, rango --data)
        .dc.b   0x10                          | +157  '.'  (dato, rango --data)
        .dc.b   0xff                          | +158  '.'  (dato, rango --data)
        .dc.b   0xff                          | +159  '.'  (dato, rango --data)
        .dc.b   0x16                          | +15a  '.'  (dato, rango --data)
        .dc.b   0x00                          | +15b  '.'  (dato, rango --data)
        .dc.b   0x07                          | +15c  '.'  (dato, rango --data)
        .dc.b   0x00                          | +15d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +15e  '.'  (dato, rango --data)
        .dc.b   0x24                          | +15f  '$'  (dato, rango --data)
        .dc.b   0x2b                          | +160  '+'  (dato, rango --data)
        .dc.b   0xbe                          | +161  '.'  (dato, rango --data)
        .dc.b   0x00                          | +162  '.'  (dato, rango --data)
        .dc.b   0x5c                          | +163  '.'  (dato, rango --data)
        .dc.b   0x00                          | +164  '.'  (dato, rango --data)
        .dc.b   0x01                          | +165  '.'  (dato, rango --data)
        .dc.b   0x02                          | +166  '.'  (dato, rango --data)
        .dc.b   0x08                          | +167  '.'  (dato, rango --data)
        .dc.b   0x00                          | +168  '.'  (dato, rango --data)
        .dc.b   0x24                          | +169  '$'  (dato, rango --data)
        .dc.b   0x2e                          | +16a  '.'  (dato, rango --data)
        .dc.b   0x24                          | +16b  '$'  (dato, rango --data)
        .dc.b   0xff                          | +16c  '.'  (dato, rango --data)
        .dc.b   0xff                          | +16d  '.'  (dato, rango --data)
        .dc.b   0x07                          | +16e  '.'  (dato, rango --data)
        .dc.b   0x00                          | +16f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +170  '.'  (dato, rango --data)
        .dc.b   0x24                          | +171  '$'  (dato, rango --data)
        .dc.b   0x2b                          | +172  '+'  (dato, rango --data)
        .dc.b   0xec                          | +173  '.'  (dato, rango --data)
        .dc.b   0x00                          | +174  '.'  (dato, rango --data)
        .dc.b   0x5c                          | +175  '.'  (dato, rango --data)
        .dc.b   0x00                          | +176  '.'  (dato, rango --data)
        .dc.b   0x01                          | +177  '.'  (dato, rango --data)
        .dc.b   0x02                          | +178  '.'  (dato, rango --data)
        .dc.b   0x08                          | +179  '.'  (dato, rango --data)
        .dc.b   0x00                          | +17a  '.'  (dato, rango --data)
        .dc.b   0x24                          | +17b  '$'  (dato, rango --data)
        .dc.b   0x2e                          | +17c  '.'  (dato, rango --data)
        .dc.b   0x38                          | +17d  '8'  (dato, rango --data)
        .dc.b   0xff                          | +17e  '.'  (dato, rango --data)
        .dc.b   0xff                          | +17f  '.'  (dato, rango --data)
        .dc.b   0x07                          | +180  '.'  (dato, rango --data)
        .dc.b   0x00                          | +181  '.'  (dato, rango --data)
        .dc.b   0x00                          | +182  '.'  (dato, rango --data)
        .dc.b   0x24                          | +183  '$'  (dato, rango --data)
        .dc.b   0x2c                          | +184  ','  (dato, rango --data)
        .dc.b   0x0e                          | +185  '.'  (dato, rango --data)
        .dc.b   0x00                          | +186  '.'  (dato, rango --data)
        .dc.b   0x5c                          | +187  '.'  (dato, rango --data)
        .dc.b   0x00                          | +188  '.'  (dato, rango --data)
        .dc.b   0x01                          | +189  '.'  (dato, rango --data)
        .dc.b   0x02                          | +18a  '.'  (dato, rango --data)
        .dc.b   0x08                          | +18b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +18c  '.'  (dato, rango --data)
        .dc.b   0x24                          | +18d  '$'  (dato, rango --data)
        .dc.b   0x2e                          | +18e  '.'  (dato, rango --data)
        .dc.b   0x4c                          | +18f  'L'  (dato, rango --data)
        .dc.b   0xff                          | +190  '.'  (dato, rango --data)
        .dc.b   0xff                          | +191  '.'  (dato, rango --data)
        .dc.b   0x07                          | +192  '.'  (dato, rango --data)
        .dc.b   0x00                          | +193  '.'  (dato, rango --data)
        .dc.b   0x00                          | +194  '.'  (dato, rango --data)
        .dc.b   0x24                          | +195  '$'  (dato, rango --data)
        .dc.b   0x2c                          | +196  ','  (dato, rango --data)
        .dc.b   0x36                          | +197  '6'  (dato, rango --data)
        .dc.b   0x00                          | +198  '.'  (dato, rango --data)
        .dc.b   0x5c                          | +199  '.'  (dato, rango --data)
        .dc.b   0x00                          | +19a  '.'  (dato, rango --data)
        .dc.b   0x01                          | +19b  '.'  (dato, rango --data)
        .dc.b   0x02                          | +19c  '.'  (dato, rango --data)
        .dc.b   0x08                          | +19d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +19e  '.'  (dato, rango --data)
        .dc.b   0x24                          | +19f  '$'  (dato, rango --data)
        .dc.b   0x2e                          | +1a0  '.'  (dato, rango --data)
        .dc.b   0x60                          | +1a1  '`'  (dato, rango --data)
        .dc.b   0xff                          | +1a2  '.'  (dato, rango --data)
        .dc.b   0xff                          | +1a3  '.'  (dato, rango --data)
        .dc.b   0x16                          | +1a4  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1a5  '.'  (dato, rango --data)
        .dc.b   0x07                          | +1a6  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1a7  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1a8  '.'  (dato, rango --data)
        .dc.b   0x24                          | +1a9  '$'  (dato, rango --data)
        .dc.b   0x2c                          | +1aa  ','  (dato, rango --data)
        .dc.b   0x5c                          | +1ab  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1ac  '.'  (dato, rango --data)
        .dc.b   0x5c                          | +1ad  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1ae  '.'  (dato, rango --data)
        .dc.b   0x01                          | +1af  '.'  (dato, rango --data)
        .dc.b   0x02                          | +1b0  '.'  (dato, rango --data)
        .dc.b   0x08                          | +1b1  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1b2  '.'  (dato, rango --data)
        .dc.b   0x24                          | +1b3  '$'  (dato, rango --data)
        .dc.b   0x2e                          | +1b4  '.'  (dato, rango --data)
        .dc.b   0x74                          | +1b5  't'  (dato, rango --data)
        .dc.b   0xff                          | +1b6  '.'  (dato, rango --data)
        .dc.b   0xff                          | +1b7  '.'  (dato, rango --data)
        .dc.b   0x07                          | +1b8  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1b9  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1ba  '.'  (dato, rango --data)
        .dc.b   0x24                          | +1bb  '$'  (dato, rango --data)
        .dc.b   0x2c                          | +1bc  ','  (dato, rango --data)
        .dc.b   0x8a                          | +1bd  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1be  '.'  (dato, rango --data)
        .dc.b   0x5c                          | +1bf  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1c0  '.'  (dato, rango --data)
        .dc.b   0x01                          | +1c1  '.'  (dato, rango --data)
        .dc.b   0x02                          | +1c2  '.'  (dato, rango --data)
        .dc.b   0x08                          | +1c3  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1c4  '.'  (dato, rango --data)
        .dc.b   0x24                          | +1c5  '$'  (dato, rango --data)
        .dc.b   0x2e                          | +1c6  '.'  (dato, rango --data)
        .dc.b   0x88                          | +1c7  '.'  (dato, rango --data)
        .dc.b   0xff                          | +1c8  '.'  (dato, rango --data)
        .dc.b   0xff                          | +1c9  '.'  (dato, rango --data)
        .dc.b   0x07                          | +1ca  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1cb  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1cc  '.'  (dato, rango --data)
        .dc.b   0x24                          | +1cd  '$'  (dato, rango --data)
        .dc.b   0x2c                          | +1ce  ','  (dato, rango --data)
        .dc.b   0xaa                          | +1cf  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1d0  '.'  (dato, rango --data)
        .dc.b   0x5c                          | +1d1  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1d2  '.'  (dato, rango --data)
        .dc.b   0x01                          | +1d3  '.'  (dato, rango --data)
        .dc.b   0x02                          | +1d4  '.'  (dato, rango --data)
        .dc.b   0x08                          | +1d5  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1d6  '.'  (dato, rango --data)
        .dc.b   0x24                          | +1d7  '$'  (dato, rango --data)
        .dc.b   0x2e                          | +1d8  '.'  (dato, rango --data)
        .dc.b   0x9c                          | +1d9  '.'  (dato, rango --data)
        .dc.b   0xff                          | +1da  '.'  (dato, rango --data)
        .dc.b   0xff                          | +1db  '.'  (dato, rango --data)
        .dc.b   0x07                          | +1dc  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1dd  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1de  '.'  (dato, rango --data)
        .dc.b   0x24                          | +1df  '$'  (dato, rango --data)
        .dc.b   0x2c                          | +1e0  ','  (dato, rango --data)
        .dc.b   0xca                          | +1e1  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1e2  '.'  (dato, rango --data)
        .dc.b   0x5c                          | +1e3  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1e4  '.'  (dato, rango --data)
        .dc.b   0x01                          | +1e5  '.'  (dato, rango --data)
        .dc.b   0x02                          | +1e6  '.'  (dato, rango --data)
        .dc.b   0x08                          | +1e7  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1e8  '.'  (dato, rango --data)
        .dc.b   0x24                          | +1e9  '$'  (dato, rango --data)
        .dc.b   0x2e                          | +1ea  '.'  (dato, rango --data)
        .dc.b   0xb0                          | +1eb  '.'  (dato, rango --data)
        .dc.b   0xff                          | +1ec  '.'  (dato, rango --data)
        .dc.b   0xff                          | +1ed  '.'  (dato, rango --data)
        .dc.b   0x16                          | +1ee  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1ef  '.'  (dato, rango --data)
        .dc.b   0x07                          | +1f0  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1f1  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1f2  '.'  (dato, rango --data)
        .dc.b   0x24                          | +1f3  '$'  (dato, rango --data)
        .dc.b   0x2c                          | +1f4  ','  (dato, rango --data)
        .dc.b   0xec                          | +1f5  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1f6  '.'  (dato, rango --data)
        .dc.b   0x5c                          | +1f7  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1f8  '.'  (dato, rango --data)
        .dc.b   0x01                          | +1f9  '.'  (dato, rango --data)
        .dc.b   0x02                          | +1fa  '.'  (dato, rango --data)
        .dc.b   0x08                          | +1fb  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1fc  '.'  (dato, rango --data)
        .dc.b   0x24                          | +1fd  '$'  (dato, rango --data)
        .dc.b   0x2e                          | +1fe  '.'  (dato, rango --data)
        .dc.b   0xc4                          | +1ff  '.'  (dato, rango --data)
        .dc.b   0xff                          | +200  '.'  (dato, rango --data)
        .dc.b   0xff                          | +201  '.'  (dato, rango --data)
        .dc.b   0x07                          | +202  '.'  (dato, rango --data)
        .dc.b   0x00                          | +203  '.'  (dato, rango --data)
        .dc.b   0x00                          | +204  '.'  (dato, rango --data)
        .dc.b   0x24                          | +205  '$'  (dato, rango --data)
        .dc.b   0x2d                          | +206  '-'  (dato, rango --data)
        .dc.b   0x18                          | +207  '.'  (dato, rango --data)
        .dc.b   0x00                          | +208  '.'  (dato, rango --data)
        .dc.b   0x5c                          | +209  '.'  (dato, rango --data)
        .dc.b   0x00                          | +20a  '.'  (dato, rango --data)
        .dc.b   0x01                          | +20b  '.'  (dato, rango --data)
        .dc.b   0x02                          | +20c  '.'  (dato, rango --data)
        .dc.b   0x08                          | +20d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +20e  '.'  (dato, rango --data)
        .dc.b   0x24                          | +20f  '$'  (dato, rango --data)
        .dc.b   0x2e                          | +210  '.'  (dato, rango --data)
        .dc.b   0xd8                          | +211  '.'  (dato, rango --data)
        .dc.b   0xff                          | +212  '.'  (dato, rango --data)
        .dc.b   0xff                          | +213  '.'  (dato, rango --data)
        .dc.b   0x07                          | +214  '.'  (dato, rango --data)
        .dc.b   0x00                          | +215  '.'  (dato, rango --data)
        .dc.b   0x00                          | +216  '.'  (dato, rango --data)
        .dc.b   0x24                          | +217  '$'  (dato, rango --data)
        .dc.b   0x2d                          | +218  '-'  (dato, rango --data)
        .dc.b   0x3e                          | +219  '>'  (dato, rango --data)
        .dc.b   0x00                          | +21a  '.'  (dato, rango --data)
        .dc.b   0x5c                          | +21b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +21c  '.'  (dato, rango --data)
        .dc.b   0x01                          | +21d  '.'  (dato, rango --data)
        .dc.b   0x02                          | +21e  '.'  (dato, rango --data)
        .dc.b   0x08                          | +21f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +220  '.'  (dato, rango --data)
        .dc.b   0x24                          | +221  '$'  (dato, rango --data)
        .dc.b   0x2e                          | +222  '.'  (dato, rango --data)
        .dc.b   0xec                          | +223  '.'  (dato, rango --data)
        .dc.b   0xff                          | +224  '.'  (dato, rango --data)
        .dc.b   0xff                          | +225  '.'  (dato, rango --data)
        .dc.b   0x07                          | +226  '.'  (dato, rango --data)
        .dc.b   0x00                          | +227  '.'  (dato, rango --data)
        .dc.b   0x00                          | +228  '.'  (dato, rango --data)
        .dc.b   0x24                          | +229  '$'  (dato, rango --data)
        .dc.b   0x2d                          | +22a  '-'  (dato, rango --data)
        .dc.b   0x62                          | +22b  'b'  (dato, rango --data)
        .dc.b   0x00                          | +22c  '.'  (dato, rango --data)
        .dc.b   0x5c                          | +22d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +22e  '.'  (dato, rango --data)
        .dc.b   0x01                          | +22f  '.'  (dato, rango --data)
        .dc.b   0x02                          | +230  '.'  (dato, rango --data)
        .dc.b   0x08                          | +231  '.'  (dato, rango --data)
        .dc.b   0x00                          | +232  '.'  (dato, rango --data)
        .dc.b   0x24                          | +233  '$'  (dato, rango --data)
        .dc.b   0x2f                          | +234  '/'  (dato, rango --data)
        .dc.b   0x00                          | +235  '.'  (dato, rango --data)
        .dc.b   0xff                          | +236  '.'  (dato, rango --data)
        .dc.b   0xff                          | +237  '.'  (dato, rango --data)
        .dc.b   0x16                          | +238  '.'  (dato, rango --data)
        .dc.b   0x00                          | +239  '.'  (dato, rango --data)
        .dc.b   0x00                          | +23a  '.'  (dato, rango --data)
        .dc.b   0x01                          | +23b  '.'  (dato, rango --data)
        .dc.b   0x02                          | +23c  '.'  (dato, rango --data)
        .dc.b   0x08                          | +23d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +23e  '.'  (dato, rango --data)
        .dc.b   0x24                          | +23f  '$'  (dato, rango --data)
        .dc.b   0x2f                          | +240  '/'  (dato, rango --data)
        .dc.b   0xd2                          | +241  '.'  (dato, rango --data)
        .dc.b   0xff                          | +242  '.'  (dato, rango --data)
        .dc.b   0xff                          | +243  '.'  (dato, rango --data)
        .dc.b   0x00                          | +244  '.'  (dato, rango --data)
        .dc.b   0x01                          | +245  '.'  (dato, rango --data)
        .dc.b   0x02                          | +246  '.'  (dato, rango --data)
        .dc.b   0x08                          | +247  '.'  (dato, rango --data)
        .dc.b   0x00                          | +248  '.'  (dato, rango --data)
        .dc.b   0x24                          | +249  '$'  (dato, rango --data)
        .dc.b   0x2f                          | +24a  '/'  (dato, rango --data)
        .dc.b   0xde                          | +24b  '.'  (dato, rango --data)
        .dc.b   0xff                          | +24c  '.'  (dato, rango --data)
        .dc.b   0xff                          | +24d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +24e  '.'  (dato, rango --data)
        .dc.b   0x01                          | +24f  '.'  (dato, rango --data)
        .dc.b   0x02                          | +250  '.'  (dato, rango --data)
        .dc.b   0x08                          | +251  '.'  (dato, rango --data)
        .dc.b   0x00                          | +252  '.'  (dato, rango --data)
        .dc.b   0x24                          | +253  '$'  (dato, rango --data)
        .dc.b   0x2f                          | +254  '/'  (dato, rango --data)
        .dc.b   0xea                          | +255  '.'  (dato, rango --data)
        .dc.b   0xff                          | +256  '.'  (dato, rango --data)
        .dc.b   0xff                          | +257  '.'  (dato, rango --data)
        .dc.b   0x01                          | +258  '.'  (dato, rango --data)
        .dc.b   0x00                          | +259  '.'  (dato, rango --data)
        .dc.b   0x00                          | +25a  '.'  (dato, rango --data)
        .dc.b   0x05                          | +25b  '.'  (dato, rango --data)
        .dc.b   0xa0                          | +25c  '.'  (dato, rango --data)
        .dc.b   0x8a                          | +25d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +25e  '.'  (dato, rango --data)
        .dc.b   0x01                          | +25f  '.'  (dato, rango --data)
        .dc.b   0x02                          | +260  '.'  (dato, rango --data)
        .dc.b   0x08                          | +261  '.'  (dato, rango --data)
        .dc.b   0x00                          | +262  '.'  (dato, rango --data)
        .dc.b   0x24                          | +263  '$'  (dato, rango --data)
        .dc.b   0x2f                          | +264  '/'  (dato, rango --data)
        .dc.b   0x9c                          | +265  '.'  (dato, rango --data)
        .dc.b   0xff                          | +266  '.'  (dato, rango --data)
        .dc.b   0xff                          | +267  '.'  (dato, rango --data)
        .dc.b   0x00                          | +268  '.'  (dato, rango --data)
        .dc.b   0x01                          | +269  '.'  (dato, rango --data)
        .dc.b   0x02                          | +26a  '.'  (dato, rango --data)
        .dc.b   0x08                          | +26b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +26c  '.'  (dato, rango --data)
        .dc.b   0x24                          | +26d  '$'  (dato, rango --data)
        .dc.b   0x2f                          | +26e  '/'  (dato, rango --data)
        .dc.b   0xae                          | +26f  '.'  (dato, rango --data)
        .dc.b   0xff                          | +270  '.'  (dato, rango --data)
        .dc.b   0xff                          | +271  '.'  (dato, rango --data)
        .dc.b   0x00                          | +272  '.'  (dato, rango --data)
        .dc.b   0x01                          | +273  '.'  (dato, rango --data)
        .dc.b   0x02                          | +274  '.'  (dato, rango --data)
        .dc.b   0x08                          | +275  '.'  (dato, rango --data)
        .dc.b   0x00                          | +276  '.'  (dato, rango --data)
        .dc.b   0x24                          | +277  '$'  (dato, rango --data)
        .dc.b   0x2f                          | +278  '/'  (dato, rango --data)
        .dc.b   0xc0                          | +279  '.'  (dato, rango --data)
        .dc.b   0xff                          | +27a  '.'  (dato, rango --data)
        .dc.b   0xff                          | +27b  '.'  (dato, rango --data)
        .dc.b   0x01                          | +27c  '.'  (dato, rango --data)
        .dc.b   0x00                          | +27d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +27e  '.'  (dato, rango --data)
        .dc.b   0x05                          | +27f  '.'  (dato, rango --data)
        .dc.b   0xa0                          | +280  '.'  (dato, rango --data)
        .dc.b   0xae                          | +281  '.'  (dato, rango --data)
        .dc.b   0x00                          | +282  '.'  (dato, rango --data)
        .dc.b   0x01                          | +283  '.'  (dato, rango --data)
        .dc.b   0x02                          | +284  '.'  (dato, rango --data)
        .dc.b   0x08                          | +285  '.'  (dato, rango --data)
        .dc.b   0x00                          | +286  '.'  (dato, rango --data)
        .dc.b   0x24                          | +287  '$'  (dato, rango --data)
        .dc.b   0x2f                          | +288  '/'  (dato, rango --data)
        .dc.b   0x66                          | +289  'f'  (dato, rango --data)
        .dc.b   0xff                          | +28a  '.'  (dato, rango --data)
        .dc.b   0xff                          | +28b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +28c  '.'  (dato, rango --data)
        .dc.b   0x01                          | +28d  '.'  (dato, rango --data)
        .dc.b   0x02                          | +28e  '.'  (dato, rango --data)
        .dc.b   0x08                          | +28f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +290  '.'  (dato, rango --data)
        .dc.b   0x24                          | +291  '$'  (dato, rango --data)
        .dc.b   0x2f                          | +292  '/'  (dato, rango --data)
        .dc.b   0x78                          | +293  'x'  (dato, rango --data)
        .dc.b   0xff                          | +294  '.'  (dato, rango --data)
        .dc.b   0xff                          | +295  '.'  (dato, rango --data)
        .dc.b   0x00                          | +296  '.'  (dato, rango --data)
        .dc.b   0x01                          | +297  '.'  (dato, rango --data)
        .dc.b   0x02                          | +298  '.'  (dato, rango --data)
        .dc.b   0x08                          | +299  '.'  (dato, rango --data)
        .dc.b   0x00                          | +29a  '.'  (dato, rango --data)
        .dc.b   0x24                          | +29b  '$'  (dato, rango --data)
        .dc.b   0x2f                          | +29c  '/'  (dato, rango --data)
        .dc.b   0x88                          | +29d  '.'  (dato, rango --data)
        .dc.b   0xff                          | +29e  '.'  (dato, rango --data)
        .dc.b   0xff                          | +29f  '.'  (dato, rango --data)
        .dc.b   0x01                          | +2a0  '.'  (dato, rango --data)
        .dc.b   0x00                          | +2a1  '.'  (dato, rango --data)
        .dc.b   0x00                          | +2a2  '.'  (dato, rango --data)
        .dc.b   0x05                          | +2a3  '.'  (dato, rango --data)
        .dc.b   0xa0                          | +2a4  '.'  (dato, rango --data)
        .dc.b   0xd2                          | +2a5  '.'  (dato, rango --data)
        .dc.b   0x00                          | +2a6  '.'  (dato, rango --data)
        .dc.b   0x01                          | +2a7  '.'  (dato, rango --data)
        .dc.b   0x02                          | +2a8  '.'  (dato, rango --data)
        .dc.b   0x08                          | +2a9  '.'  (dato, rango --data)
        .dc.b   0x00                          | +2aa  '.'  (dato, rango --data)
        .dc.b   0x24                          | +2ab  '$'  (dato, rango --data)
        .dc.b   0x2f                          | +2ac  '/'  (dato, rango --data)
        .dc.b   0x38                          | +2ad  '8'  (dato, rango --data)
        .dc.b   0xff                          | +2ae  '.'  (dato, rango --data)
        .dc.b   0xff                          | +2af  '.'  (dato, rango --data)
        .dc.b   0x00                          | +2b0  '.'  (dato, rango --data)
        .dc.b   0x01                          | +2b1  '.'  (dato, rango --data)
        .dc.b   0x02                          | +2b2  '.'  (dato, rango --data)
        .dc.b   0x08                          | +2b3  '.'  (dato, rango --data)
        .dc.b   0x00                          | +2b4  '.'  (dato, rango --data)
        .dc.b   0x24                          | +2b5  '$'  (dato, rango --data)
        .dc.b   0x2f                          | +2b6  '/'  (dato, rango --data)
        .dc.b   0x44                          | +2b7  'D'  (dato, rango --data)
        .dc.b   0xff                          | +2b8  '.'  (dato, rango --data)
        .dc.b   0xff                          | +2b9  '.'  (dato, rango --data)
        .dc.b   0x00                          | +2ba  '.'  (dato, rango --data)
        .dc.b   0x01                          | +2bb  '.'  (dato, rango --data)
        .dc.b   0x02                          | +2bc  '.'  (dato, rango --data)
        .dc.b   0x08                          | +2bd  '.'  (dato, rango --data)
        .dc.b   0x00                          | +2be  '.'  (dato, rango --data)
        .dc.b   0x24                          | +2bf  '$'  (dato, rango --data)
        .dc.b   0x2f                          | +2c0  '/'  (dato, rango --data)
        .dc.b   0x5a                          | +2c1  'Z'  (dato, rango --data)
        .dc.b   0xff                          | +2c2  '.'  (dato, rango --data)
        .dc.b   0xff                          | +2c3  '.'  (dato, rango --data)
        .dc.b   0x01                          | +2c4  '.'  (dato, rango --data)
        .dc.b   0x00                          | +2c5  '.'  (dato, rango --data)
        .dc.b   0x00                          | +2c6  '.'  (dato, rango --data)
        .dc.b   0x05                          | +2c7  '.'  (dato, rango --data)
        .dc.b   0xa0                          | +2c8  '.'  (dato, rango --data)
        .dc.b   0xf6                          | +2c9  '.'  (dato, rango --data)
        .dc.b   0x00                          | +2ca  '.'  (dato, rango --data)
        .dc.b   0x01                          | +2cb  '.'  (dato, rango --data)
        .dc.b   0x02                          | +2cc  '.'  (dato, rango --data)
        .dc.b   0x08                          | +2cd  '.'  (dato, rango --data)
        .dc.b   0x00                          | +2ce  '.'  (dato, rango --data)
        .dc.b   0x24                          | +2cf  '$'  (dato, rango --data)
        .dc.b   0x2f                          | +2d0  '/'  (dato, rango --data)
        .dc.b   0x14                          | +2d1  '.'  (dato, rango --data)
        .dc.b   0xff                          | +2d2  '.'  (dato, rango --data)
        .dc.b   0xff                          | +2d3  '.'  (dato, rango --data)
        .dc.b   0x00                          | +2d4  '.'  (dato, rango --data)
        .dc.b   0x01                          | +2d5  '.'  (dato, rango --data)
        .dc.b   0x02                          | +2d6  '.'  (dato, rango --data)
        .dc.b   0x08                          | +2d7  '.'  (dato, rango --data)
        .dc.b   0x00                          | +2d8  '.'  (dato, rango --data)
        .dc.b   0x24                          | +2d9  '$'  (dato, rango --data)
        .dc.b   0x2f                          | +2da  '/'  (dato, rango --data)
        .dc.b   0x20                          | +2db  ' '  (dato, rango --data)
        .dc.b   0xff                          | +2dc  '.'  (dato, rango --data)
        .dc.b   0xff                          | +2dd  '.'  (dato, rango --data)
        .dc.b   0x00                          | +2de  '.'  (dato, rango --data)
        .dc.b   0x01                          | +2df  '.'  (dato, rango --data)
        .dc.b   0x02                          | +2e0  '.'  (dato, rango --data)
        .dc.b   0x08                          | +2e1  '.'  (dato, rango --data)
        .dc.b   0x00                          | +2e2  '.'  (dato, rango --data)
        .dc.b   0x24                          | +2e3  '$'  (dato, rango --data)
        .dc.b   0x2f                          | +2e4  '/'  (dato, rango --data)
        .dc.b   0x2c                          | +2e5  ','  (dato, rango --data)
        .dc.b   0xff                          | +2e6  '.'  (dato, rango --data)
        .dc.b   0xff                          | +2e7  '.'  (dato, rango --data)
        .dc.b   0x01                          | +2e8  '.'  (dato, rango --data)
        .dc.b   0x00                          | +2e9  '.'  (dato, rango --data)
        .dc.b   0x00                          | +2ea  '.'  (dato, rango --data)
        .dc.b   0x05                          | +2eb  '.'  (dato, rango --data)
        .dc.b   0xa1                          | +2ec  '.'  (dato, rango --data)
        .dc.b   0x1a                          | +2ed  '.'  (dato, rango --data)
        .global GunnerAnim_Table_059e50__L05a13e
GunnerAnim_Table_059e50__L05a13e:
.L05a13e:
        .dc.b   0x00                          | +2ee  '.'  (dato, rango --data)
        .dc.b   0x05                          | +2ef  '.'  (dato, rango --data)
        .dc.b   0x9e                          | +2f0  '.'  (dato, rango --data)
        .dc.b   0x50                          | +2f1  'P'  (dato, rango --data)
        .dc.b   0x00                          | +2f2  '.'  (dato, rango --data)
        .dc.b   0x05                          | +2f3  '.'  (dato, rango --data)
        .dc.b   0x9e                          | +2f4  '.'  (dato, rango --data)
        .dc.b   0x64                          | +2f5  'd'  (dato, rango --data)
        .dc.b   0x00                          | +2f6  '.'  (dato, rango --data)
        .dc.b   0x05                          | +2f7  '.'  (dato, rango --data)
        .dc.b   0x9e                          | +2f8  '.'  (dato, rango --data)
        .dc.b   0x78                          | +2f9  'x'  (dato, rango --data)
        .dc.b   0x00                          | +2fa  '.'  (dato, rango --data)
        .dc.b   0x05                          | +2fb  '.'  (dato, rango --data)
        .dc.b   0x9e                          | +2fc  '.'  (dato, rango --data)
        .dc.b   0x8c                          | +2fd  '.'  (dato, rango --data)
        .dc.b   0x00                          | +2fe  '.'  (dato, rango --data)
        .dc.b   0x05                          | +2ff  '.'  (dato, rango --data)
        .dc.b   0x9e                          | +300  '.'  (dato, rango --data)
        .dc.b   0xa0                          | +301  '.'  (dato, rango --data)
        .global GunnerAnim_Table_059e50__L05a152
GunnerAnim_Table_059e50__L05a152:
.L05a152:
        .dc.b   0x00                          | +302  '.'  (dato, rango --data)
        .dc.b   0x05                          | +303  '.'  (dato, rango --data)
        .dc.b   0x9e                          | +304  '.'  (dato, rango --data)
        .dc.b   0xb4                          | +305  '.'  (dato, rango --data)
        .dc.b   0x00                          | +306  '.'  (dato, rango --data)
        .dc.b   0x05                          | +307  '.'  (dato, rango --data)
        .dc.b   0x9e                          | +308  '.'  (dato, rango --data)
        .dc.b   0xc8                          | +309  '.'  (dato, rango --data)
        .dc.b   0x00                          | +30a  '.'  (dato, rango --data)
        .dc.b   0x05                          | +30b  '.'  (dato, rango --data)
        .dc.b   0x9e                          | +30c  '.'  (dato, rango --data)
        .dc.b   0xdc                          | +30d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +30e  '.'  (dato, rango --data)
        .dc.b   0x05                          | +30f  '.'  (dato, rango --data)
        .dc.b   0x9e                          | +310  '.'  (dato, rango --data)
        .dc.b   0xf0                          | +311  '.'  (dato, rango --data)
        .dc.b   0x00                          | +312  '.'  (dato, rango --data)
        .dc.b   0x05                          | +313  '.'  (dato, rango --data)
        .dc.b   0x9f                          | +314  '.'  (dato, rango --data)
        .dc.b   0x04                          | +315  '.'  (dato, rango --data)
        .global GunnerAnim_Table_059e50__L05a166
GunnerAnim_Table_059e50__L05a166:
.L05a166:
        .dc.b   0x00                          | +316  '.'  (dato, rango --data)
        .dc.b   0x05                          | +317  '.'  (dato, rango --data)
        .dc.b   0x9f                          | +318  '.'  (dato, rango --data)
        .dc.b   0x18                          | +319  '.'  (dato, rango --data)
        .dc.b   0x00                          | +31a  '.'  (dato, rango --data)
        .dc.b   0x05                          | +31b  '.'  (dato, rango --data)
        .dc.b   0x9f                          | +31c  '.'  (dato, rango --data)
        .dc.b   0x62                          | +31d  'b'  (dato, rango --data)
        .dc.b   0x00                          | +31e  '.'  (dato, rango --data)
        .dc.b   0x05                          | +31f  '.'  (dato, rango --data)
        .dc.b   0x9f                          | +320  '.'  (dato, rango --data)
        .dc.b   0xac                          | +321  '.'  (dato, rango --data)
        .dc.b   0x00                          | +322  '.'  (dato, rango --data)
        .dc.b   0x05                          | +323  '.'  (dato, rango --data)
        .dc.b   0x9f                          | +324  '.'  (dato, rango --data)
        .dc.b   0xf6                          | +325  '.'  (dato, rango --data)
        .dc.b   0x00                          | +326  '.'  (dato, rango --data)
        .dc.b   0x05                          | +327  '.'  (dato, rango --data)
        .dc.b   0xa0                          | +328  '.'  (dato, rango --data)
        .dc.b   0x40                          | +329  '@'  (dato, rango --data)
        .global GunnerAnim_Table_059e50__L05a17a
GunnerAnim_Table_059e50__L05a17a:
.L05a17a:
        .dc.b   0x00                          | +32a  '.'  (dato, rango --data)
        .dc.b   0x05                          | +32b  '.'  (dato, rango --data)
        .dc.b   0xa0                          | +32c  '.'  (dato, rango --data)
        .dc.b   0x8a                          | +32d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +32e  '.'  (dato, rango --data)
        .dc.b   0x05                          | +32f  '.'  (dato, rango --data)
        .dc.b   0xa0                          | +330  '.'  (dato, rango --data)
        .dc.b   0xae                          | +331  '.'  (dato, rango --data)
        .dc.b   0x00                          | +332  '.'  (dato, rango --data)
        .dc.b   0x05                          | +333  '.'  (dato, rango --data)
        .dc.b   0xa0                          | +334  '.'  (dato, rango --data)
        .dc.b   0xd2                          | +335  '.'  (dato, rango --data)
        .dc.b   0x00                          | +336  '.'  (dato, rango --data)
        .dc.b   0x05                          | +337  '.'  (dato, rango --data)
        .dc.b   0xa0                          | +338  '.'  (dato, rango --data)
        .dc.b   0xf6                          | +339  '.'  (dato, rango --data)
        .dc.b   0x00                          | +33a  '.'  (dato, rango --data)
        .dc.b   0x05                          | +33b  '.'  (dato, rango --data)
        .dc.b   0xa1                          | +33c  '.'  (dato, rango --data)
        .dc.b   0x1a                          | +33d  '.'  (dato, rango --data)

| ----------------------------------------------------------------------------
|  GunnerAim_DxTable_05a18e  @ $05A18E  (20 B)
| ----------------------------------------------------------------------------
        .section .text.GunnerAim_DxTable_05a18e, "ax", @progbits
        .global GunnerAim_DxTable_05a18e
GunnerAim_DxTable_05a18e:
        .dc.b   0x00                          | +000  '.'  (dato, rango --data)
        .dc.b   0x00                          | +001  '.'  (dato, rango --data)
        .dc.b   0x00                          | +002  '.'  (dato, rango --data)
        .dc.b   0x18                          | +003  '.'  (dato, rango --data)
        .dc.b   0x00                          | +004  '.'  (dato, rango --data)
        .dc.b   0x00                          | +005  '.'  (dato, rango --data)
        .dc.b   0x00                          | +006  '.'  (dato, rango --data)
        .dc.b   0x10                          | +007  '.'  (dato, rango --data)
        .dc.b   0x00                          | +008  '.'  (dato, rango --data)
        .dc.b   0x00                          | +009  '.'  (dato, rango --data)
        .dc.b   0x00                          | +00a  '.'  (dato, rango --data)
        .dc.b   0x08                          | +00b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +00c  '.'  (dato, rango --data)
        .dc.b   0x00                          | +00d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +00e  '.'  (dato, rango --data)
        .dc.b   0x04                          | +00f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +010  '.'  (dato, rango --data)
        .dc.b   0x00                          | +011  '.'  (dato, rango --data)
        .dc.b   0x00                          | +012  '.'  (dato, rango --data)
        .dc.b   0x00                          | +013  '.'  (dato, rango --data)

| ----------------------------------------------------------------------------
|  GunnerAim_DyTable_05a1a2  @ $05A1A2  (20 B)
| ----------------------------------------------------------------------------
        .section .text.GunnerAim_DyTable_05a1a2, "ax", @progbits
        .global GunnerAim_DyTable_05a1a2
GunnerAim_DyTable_05a1a2:
        .dc.b   0x00                          | +000  '.'  (dato, rango --data)
        .dc.b   0x18                          | +001  '.'  (dato, rango --data)
        .dc.b   0xff                          | +002  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +003  '.'  (dato, rango --data)
        .dc.b   0x00                          | +004  '.'  (dato, rango --data)
        .dc.b   0x0c                          | +005  '.'  (dato, rango --data)
        .dc.b   0xff                          | +006  '.'  (dato, rango --data)
        .dc.b   0xe4                          | +007  '.'  (dato, rango --data)
        .dc.b   0x00                          | +008  '.'  (dato, rango --data)
        .dc.b   0x08                          | +009  '.'  (dato, rango --data)
        .dc.b   0xff                          | +00a  '.'  (dato, rango --data)
        .dc.b   0xec                          | +00b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +00c  '.'  (dato, rango --data)
        .dc.b   0x04                          | +00d  '.'  (dato, rango --data)
        .dc.b   0xff                          | +00e  '.'  (dato, rango --data)
        .dc.b   0xf4                          | +00f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +010  '.'  (dato, rango --data)
        .dc.b   0x00                          | +011  '.'  (dato, rango --data)
        .dc.b   0x00                          | +012  '.'  (dato, rango --data)
        .dc.b   0x00                          | +013  '.'  (dato, rango --data)

| ----------------------------------------------------------------------------
|  Gunner2_AimFromAngle_05a1b6  @ $05A1B6  (128 B)
| ----------------------------------------------------------------------------
        .section .text.Gunner2_AimFromAngle_05a1b6, "ax", @progbits
        .global Gunner2_AimFromAngle_05a1b6
Gunner2_AimFromAngle_05a1b6:
        moveq   #0,d0                           | +000
        move.b  0x70(a6),d0                     | +002
        lsr.b   #0x4,d0                         | +006
        move.w  d0,d1                           | +008
        andi.b  #0x7,d0                         | +00a
        cmpi.b  #0x5,d0                         | +00e
        bcs.w   .L05a1d0                        | +012
        neg.b   d0                              | +016
        addq.b  #0x8,d0                         | +018
.L05a1d0:
        addq.w  #0x3,d1                         | +01a
        andi.b  #0xf,d1                         | +01c
        cmpi.w  #0x7,d1                         | +020
        bcc.w   .L05a1e8                        | +024
        bset    #0x0,0x3a(a6)                   | +028
        bra.w   .L05a1ee                        | +02e
.L05a1e8:
        bclr    #0x0,0x3a(a6)                   | +032
.L05a1ee:
        add.w   d0,d0                           | +038
        add.w   d0,d0                           | +03a
        move.w  d0,0x7c(a6)                     | +03c
        movea.l (a0,d0.w),a0                    | +040
        jsr     0x28cd4.l                       | +044
        lea     GunnerAim_DxTable_05a18e(pc),a1 | +04a
        lea     GunnerAim_DyTable_05a1a2(pc),a2 | +04e
        move.l  (a1,d0.w),d1                    | +052
        move.l  (a2,d0.w),d2                    | +056
        btst    #0x0,0x3a(a6)                   | +05a
        beq.w   .L05a21e                        | +060
        neg.w   d1                              | +064
        neg.w   d2                              | +066
.L05a21e:
        add.w   0x22(a6),d1                     | +068
        move.w  d1,0x76(a6)                     | +06c
        add.w   0x22(a6),d2                     | +070
        swap    d2                              | +074
        add.w   0x24(a6),d2                     | +076
        move.l  d2,0x78(a6)                     | +07a
        rts                                     | +07e

| ----------------------------------------------------------------------------
|  Gunner2_ProbeBox_05a236  @ $05A236  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Gunner2_ProbeBox_05a236, "ax", @progbits
        .global Gunner2_ProbeBox_05a236
Gunner2_ProbeBox_05a236:
        .dc.b   0xff                          | +000  '.'  (dato, rango --data)
        .dc.b   0xd0                          | +001  '.'  (dato, rango --data)
        .dc.b   0x00                          | +002  '.'  (dato, rango --data)
        .dc.b   0x30                          | +003  '0'  (dato, rango --data)
        .dc.b   0xff                          | +004  '.'  (dato, rango --data)
        .dc.b   0xd0                          | +005  '.'  (dato, rango --data)
        .dc.b   0x00                          | +006  '.'  (dato, rango --data)
        .dc.b   0x30                          | +007  '0'  (dato, rango --data)

| ----------------------------------------------------------------------------
|  Gunner2_ProbeOrDie_05a23e  @ $05A23E  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Gunner2_ProbeOrDie_05a23e, "ax", @progbits
        .global Gunner2_ProbeOrDie_05a23e
Gunner2_ProbeOrDie_05a23e:
        lea     Gunner2_ProbeBox_05a236(pc),a0  | +000
        jsr     0x5dd5c.l                       | +004
        bcc.w   Jsr5B6Rts_05a258                | +00a

| ----------------------------------------------------------------------------
|  Gunner2_Init_05a25a  @ $05A25A  (40 B)
| ----------------------------------------------------------------------------
        .section .text.Gunner2_Init_05a25a, "ax", @progbits
        .global Gunner2_Init_05a25a
Gunner2_Init_05a25a:
        move.w  0x4000.w,d0                     | +000
        ori.w   #0x4,d0                         | +004
        move.w  d0,0x38(a6)                     | +008
        jsr     0x8f3a6.l                       | +00c
        not.b   d0                              | +012
        move.b  d0,0x72(a6)                     | +014
        lea     Gunner2_Child_Init_05a65c(pc),a1 | +018
        jsr     0x4ae.l                         | +01c
        move.w  #0x8000,0x70(a6)                | +022

| ----------------------------------------------------------------------------
|  Gunner2_Search_05a28a  @ $05A28A  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Gunner2_Search_05a28a, "ax", @progbits
        .global Gunner2_Search_05a28a
Gunner2_Search_05a28a:
        lea     GunnerAnim_Table_059e50__L05a13e(pc),a0 | +000
        bsr.w   Gunner2_AimFromAngle_05a1b6     | +004
        jsr     0x2783a.l                       | +008
        jsr     0x28d70.l                       | +00e
        move.w  0x76(a6),d0                     | +014
        move.w  0x24(a6),d1                     | +018
        move.b  0x72(a6),d2                     | +01c
        jsr     0x8f3be.l                       | +020
        tst.b   d4                              | +026
        bmi.w   .L05a2c2                        | +028
        move.b  #0x0,0x73(a6)                   | +02c
        lea     Gunner2_Acquire_05a2c6(pc),a1   | +032
        move.l  a1,(a6)                         | +036
.L05a2c2:
        bra.w   Gunner2_ProbeOrDie_05a23e       | +038

| ----------------------------------------------------------------------------
|  Gunner2_Acquire_05a2c6  @ $05A2C6  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Gunner2_Acquire_05a2c6, "ax", @progbits
        .global Gunner2_Acquire_05a2c6
Gunner2_Acquire_05a2c6:
        jsr     0x13600.l                       | +000
        move.b  0x73(a6),d4                     | +006
        jsr     0x8f69c.l                       | +00a
        jsr     0x236e.l                        | +010

| ----------------------------------------------------------------------------
|  Gunner2_Track_05a2dc  @ $05A2DC  (172 B)
| ----------------------------------------------------------------------------
        .section .text.Gunner2_Track_05a2dc, "ax", @progbits
        .global Gunner2_Track_05a2dc
Gunner2_Track_05a2dc:
        move.w  #0x78,0x74(a6)                  | +000
        lea     .L05a2e8(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L05a2e8:
        cmpi.b  #0x2,0x73(a6)                   | +00c
        bne.w   .L05a320                        | +012
        jsr     0x5e136.l                       | +016
        lsl.w   #0x8,d0                         | +01c
        sub.w   0x70(a6),d0                     | +01e
        asr.w   #0x4,d0                         | +022
        add.w   d0,0x70(a6)                     | +024
        jsr     0x5e1aa.l                       | +028
        bcs.w   .L05a31c                        | +02e
        subq.w  #0x1,0x74(a6)                   | +032
        bne.w   .L05a31c                        | +036
        lea     Gunner2_Fire_05a388(pc),a1      | +03a
        move.l  a1,(a6)                         | +03e
.L05a31c:
        bra.w   .L05a370                        | +040
.L05a320:
        cmpi.b  #0x0,0x73(a6)                   | +044
        bne.w   .L05a334                        | +04a
        lea     0x10e200.l,a2                   | +04e
        bra.w   .L05a33a                        | +054
.L05a334:
        lea     0x10e206.l,a2                   | +058
.L05a33a:
        move.b  0x2(a2),d0                      | +05e
        andi.w  #0xf,d0                         | +062
        add.w   d0,d0                           | +066
        lea     0x5d326.l,a0                    | +068
        move.w  (a0,d0.w),d0                    | +06e
        cmpi.w  #0xffff,d0                      | +072
        beq.w   .L05a360                        | +076
        sub.w   0x70(a6),d0                     | +07a
        asr.w   #0x4,d0                         | +07e
        add.w   d0,0x70(a6)                     | +080
.L05a360:
        btst    #0x4,0x3(a2)                    | +084
        beq.w   .L05a370                        | +08a
        lea     Gunner2_Fire_05a388(pc),a1      | +08e
        move.l  a1,(a6)                         | +092
.L05a370:
        jsr     0x2783a.l                       | +094
        lea     GunnerAnim_Table_059e50__L05a152(pc),a0 | +09a
        bsr.w   Gunner2_AimFromAngle_05a1b6     | +09e
        jsr     0x28d70.l                       | +0a2
        bra.w   Gunner2_Fire_05a388__L05a3ec    | +0a8

| ----------------------------------------------------------------------------
|  Gunner2_Fire_05a388  @ $05A388  (134 B)
| ----------------------------------------------------------------------------
        .section .text.Gunner2_Fire_05a388, "ax", @progbits
        .global Gunner2_Fire_05a388
Gunner2_Fire_05a388:
        lea     GunnerAnim_Table_059e50__L05a166(pc),a0 | +000
        bsr.w   Gunner2_AimFromAngle_05a1b6     | +004
        lea     .L05a396(pc),a1                 | +008
        move.l  a1,(a6)                         | +00c
.L05a396:
        jsr     0x2783a.l                       | +00e
        jsr     0x28d70.l                       | +014
        bcc.w   .L05a3ec                        | +01a
        move.l  a6,-(a7)                        | +01e
        lea     0x100800.l,a6                   | +020
        lea     Gunner2_Shell_05a55a(pc),a1     | +026
        jsr     0x4ae.l                         | +02a
        movea.l (a7)+,a6                        | +030
        move.b  0x73(a6),0x98(a0)               | +032
        move.w  0x78(a6),0x22(a0)               | +038
        move.w  0x7a(a6),0x24(a0)               | +03e
        move.b  0x3a(a6),0x3a(a0)               | +044
        move.w  0x7c(a6),0x5c(a0)               | +04a
        move.w  0x70(a6),d0                     | +050
        ble.w   .L05a3e2                        | +054
        neg.w   d0                              | +058
.L05a3e2:
        move.w  d0,0x34(a0)                     | +05a
        lea     Gunner2_Track_05a2dc(pc),a1     | +05e
        move.l  a1,(a6)                         | +062
        .global Gunner2_Fire_05a388__L05a3ec
Gunner2_Fire_05a388__L05a3ec:
.L05a3ec:
        move.w  0x76(a6),d0                     | +064
        move.w  0x24(a6),d1                     | +068
        move.b  0x72(a6),d2                     | +06c
        jsr     0x8f3be.l                       | +070
        tst.b   d4                              | +076
        bpl.w   .L05a40a                        | +078
        lea     Gunner2_Search_05a28a(pc),a1    | +07c
        move.l  a1,(a6)                         | +080
.L05a40a:
        bra.w   Gunner2_ProbeOrDie_05a23e       | +082

| ----------------------------------------------------------------------------
|  Shell_HitboxList_A_05a40e  @ $05A40E  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Shell_HitboxList_A_05a40e, "ax", @progbits
        .global Shell_HitboxList_A_05a40e
Shell_HitboxList_A_05a40e:
        .dc.b   0x00                          | +000  '.'  (dato, rango --data)
        .dc.b   0x0e                          | +001  '.'  (dato, rango --data)
        .dc.b   0x00                          | +002  '.'  (dato, rango --data)
        .dc.b   0x10                          | +003  '.'  (dato, rango --data)
        .dc.b   0x02                          | +004  '.'  (dato, rango --data)
        .dc.b   0x04                          | +005  '.'  (dato, rango --data)
        .dc.b   0xff                          | +006  '.'  (dato, rango --data)
        .dc.b   0x00                          | +007  '.'  (dato, rango --data)
        .dc.b   0x00                          | +008  '.'  (dato, rango --data)
        .dc.b   0x00                          | +009  '.'  (dato, rango --data)
        .dc.b   0xff                          | +00a  '.'  (dato, rango --data)
        .dc.b   0xf8                          | +00b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +00c  '.'  (dato, rango --data)
        .dc.b   0x08                          | +00d  '.'  (dato, rango --data)
        .dc.b   0xff                          | +00e  '.'  (dato, rango --data)
        .dc.b   0xf8                          | +00f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +010  '.'  (dato, rango --data)
        .dc.b   0x08                          | +011  '.'  (dato, rango --data)
        .dc.b   0x02                          | +012  '.'  (dato, rango --data)
        .dc.b   0x00                          | +013  '.'  (dato, rango --data)
        .dc.b   0x00                          | +014  '.'  (dato, rango --data)
        .dc.b   0x24                          | +015  '$'  (dato, rango --data)
        .dc.b   0x36                          | +016  '6'  (dato, rango --data)
        .dc.b   0x3a                          | +017  ':'  (dato, rango --data)
        .dc.b   0xff                          | +018  '.'  (dato, rango --data)
        .dc.b   0xff                          | +019  '.'  (dato, rango --data)
        .dc.b   0x00                          | +01a  '.'  (dato, rango --data)
        .dc.b   0x00                          | +01b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +01c  '.'  (dato, rango --data)
        .dc.b   0x00                          | +01d  '.'  (dato, rango --data)
        .dc.b   0x02                          | +01e  '.'  (dato, rango --data)
        .dc.b   0x00                          | +01f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +020  '.'  (dato, rango --data)
        .dc.b   0x24                          | +021  '$'  (dato, rango --data)
        .dc.b   0x36                          | +022  '6'  (dato, rango --data)
        .dc.b   0x44                          | +023  'D'  (dato, rango --data)
        .dc.b   0xff                          | +024  '.'  (dato, rango --data)
        .dc.b   0xff                          | +025  '.'  (dato, rango --data)
        .dc.b   0xff                          | +026  '.'  (dato, rango --data)
        .dc.b   0xf8                          | +027  '.'  (dato, rango --data)
        .dc.b   0x00                          | +028  '.'  (dato, rango --data)
        .dc.b   0x08                          | +029  '.'  (dato, rango --data)
        .dc.b   0x02                          | +02a  '.'  (dato, rango --data)
        .dc.b   0x00                          | +02b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +02c  '.'  (dato, rango --data)
        .dc.b   0x24                          | +02d  '$'  (dato, rango --data)
        .dc.b   0x36                          | +02e  '6'  (dato, rango --data)
        .dc.b   0x4e                          | +02f  'N'  (dato, rango --data)
        .dc.b   0xff                          | +030  '.'  (dato, rango --data)
        .dc.b   0xff                          | +031  '.'  (dato, rango --data)
        .dc.b   0xff                          | +032  '.'  (dato, rango --data)
        .dc.b   0xf8                          | +033  '.'  (dato, rango --data)
        .dc.b   0xff                          | +034  '.'  (dato, rango --data)
        .dc.b   0xf8                          | +035  '.'  (dato, rango --data)
        .dc.b   0x02                          | +036  '.'  (dato, rango --data)
        .dc.b   0x00                          | +037  '.'  (dato, rango --data)
        .dc.b   0x00                          | +038  '.'  (dato, rango --data)
        .dc.b   0x24                          | +039  '$'  (dato, rango --data)
        .dc.b   0x36                          | +03a  '6'  (dato, rango --data)
        .dc.b   0x58                          | +03b  'X'  (dato, rango --data)
        .dc.b   0xff                          | +03c  '.'  (dato, rango --data)
        .dc.b   0xff                          | +03d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +03e  '.'  (dato, rango --data)
        .dc.b   0x08                          | +03f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +040  '.'  (dato, rango --data)
        .dc.b   0x08                          | +041  '.'  (dato, rango --data)
        .dc.b   0x02                          | +042  '.'  (dato, rango --data)
        .dc.b   0x00                          | +043  '.'  (dato, rango --data)
        .dc.b   0x00                          | +044  '.'  (dato, rango --data)
        .dc.b   0x24                          | +045  '$'  (dato, rango --data)
        .dc.b   0x36                          | +046  '6'  (dato, rango --data)
        .dc.b   0x62                          | +047  'b'  (dato, rango --data)
        .dc.b   0xff                          | +048  '.'  (dato, rango --data)
        .dc.b   0xff                          | +049  '.'  (dato, rango --data)
        .dc.b   0x00                          | +04a  '.'  (dato, rango --data)
        .dc.b   0x08                          | +04b  '.'  (dato, rango --data)
        .dc.b   0xff                          | +04c  '.'  (dato, rango --data)
        .dc.b   0xf8                          | +04d  '.'  (dato, rango --data)
        .dc.b   0x1d                          | +04e  '.'  (dato, rango --data)
        .dc.b   0x00                          | +04f  '.'  (dato, rango --data)
        .dc.b   0xff                          | +050  '.'  (dato, rango --data)
        .dc.b   0xff                          | +051  '.'  (dato, rango --data)
        .dc.b   0xff                          | +052  '.'  (dato, rango --data)
        .dc.b   0xff                          | +053  '.'  (dato, rango --data)

| ----------------------------------------------------------------------------
|  Shell_HitboxList_B_05a462  @ $05A462  (164 B)
| ----------------------------------------------------------------------------
        .section .text.Shell_HitboxList_B_05a462, "ax", @progbits
        .global Shell_HitboxList_B_05a462
Shell_HitboxList_B_05a462:
        .dc.b   0x03                          | +000  '.'  (dato, rango --data)
        .dc.b   0x06                          | +001  '.'  (dato, rango --data)
        .dc.b   0x00                          | +002  '.'  (dato, rango --data)
        .dc.b   0x32                          | +003  '2'  (dato, rango --data)
        .dc.b   0x02                          | +004  '.'  (dato, rango --data)
        .dc.b   0x04                          | +005  '.'  (dato, rango --data)
        .dc.b   0xff                          | +006  '.'  (dato, rango --data)
        .dc.b   0x00                          | +007  '.'  (dato, rango --data)
        .dc.b   0x00                          | +008  '.'  (dato, rango --data)
        .dc.b   0x00                          | +009  '.'  (dato, rango --data)
        .dc.b   0xff                          | +00a  '.'  (dato, rango --data)
        .dc.b   0xf0                          | +00b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +00c  '.'  (dato, rango --data)
        .dc.b   0x10                          | +00d  '.'  (dato, rango --data)
        .dc.b   0xff                          | +00e  '.'  (dato, rango --data)
        .dc.b   0xf0                          | +00f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +010  '.'  (dato, rango --data)
        .dc.b   0x10                          | +011  '.'  (dato, rango --data)
        .dc.b   0x02                          | +012  '.'  (dato, rango --data)
        .dc.b   0x00                          | +013  '.'  (dato, rango --data)
        .dc.b   0x00                          | +014  '.'  (dato, rango --data)
        .dc.b   0x24                          | +015  '$'  (dato, rango --data)
        .dc.b   0x36                          | +016  '6'  (dato, rango --data)
        .dc.b   0x3a                          | +017  ':'  (dato, rango --data)
        .dc.b   0xff                          | +018  '.'  (dato, rango --data)
        .dc.b   0xff                          | +019  '.'  (dato, rango --data)
        .dc.b   0x00                          | +01a  '.'  (dato, rango --data)
        .dc.b   0x00                          | +01b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +01c  '.'  (dato, rango --data)
        .dc.b   0x00                          | +01d  '.'  (dato, rango --data)
        .dc.b   0x02                          | +01e  '.'  (dato, rango --data)
        .dc.b   0x00                          | +01f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +020  '.'  (dato, rango --data)
        .dc.b   0x24                          | +021  '$'  (dato, rango --data)
        .dc.b   0x36                          | +022  '6'  (dato, rango --data)
        .dc.b   0x44                          | +023  'D'  (dato, rango --data)
        .dc.b   0xff                          | +024  '.'  (dato, rango --data)
        .dc.b   0xff                          | +025  '.'  (dato, rango --data)
        .dc.b   0xff                          | +026  '.'  (dato, rango --data)
        .dc.b   0xf0                          | +027  '.'  (dato, rango --data)
        .dc.b   0x00                          | +028  '.'  (dato, rango --data)
        .dc.b   0x10                          | +029  '.'  (dato, rango --data)
        .dc.b   0x02                          | +02a  '.'  (dato, rango --data)
        .dc.b   0x00                          | +02b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +02c  '.'  (dato, rango --data)
        .dc.b   0x24                          | +02d  '$'  (dato, rango --data)
        .dc.b   0x36                          | +02e  '6'  (dato, rango --data)
        .dc.b   0x4e                          | +02f  'N'  (dato, rango --data)
        .dc.b   0xff                          | +030  '.'  (dato, rango --data)
        .dc.b   0xff                          | +031  '.'  (dato, rango --data)
        .dc.b   0xff                          | +032  '.'  (dato, rango --data)
        .dc.b   0xf0                          | +033  '.'  (dato, rango --data)
        .dc.b   0xff                          | +034  '.'  (dato, rango --data)
        .dc.b   0xf0                          | +035  '.'  (dato, rango --data)
        .dc.b   0x02                          | +036  '.'  (dato, rango --data)
        .dc.b   0x00                          | +037  '.'  (dato, rango --data)
        .dc.b   0x00                          | +038  '.'  (dato, rango --data)
        .dc.b   0x24                          | +039  '$'  (dato, rango --data)
        .dc.b   0x36                          | +03a  '6'  (dato, rango --data)
        .dc.b   0x58                          | +03b  'X'  (dato, rango --data)
        .dc.b   0xff                          | +03c  '.'  (dato, rango --data)
        .dc.b   0xff                          | +03d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +03e  '.'  (dato, rango --data)
        .dc.b   0x10                          | +03f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +040  '.'  (dato, rango --data)
        .dc.b   0x10                          | +041  '.'  (dato, rango --data)
        .dc.b   0x02                          | +042  '.'  (dato, rango --data)
        .dc.b   0x00                          | +043  '.'  (dato, rango --data)
        .dc.b   0x00                          | +044  '.'  (dato, rango --data)
        .dc.b   0x24                          | +045  '$'  (dato, rango --data)
        .dc.b   0x36                          | +046  '6'  (dato, rango --data)
        .dc.b   0x62                          | +047  'b'  (dato, rango --data)
        .dc.b   0xff                          | +048  '.'  (dato, rango --data)
        .dc.b   0xff                          | +049  '.'  (dato, rango --data)
        .dc.b   0x00                          | +04a  '.'  (dato, rango --data)
        .dc.b   0x10                          | +04b  '.'  (dato, rango --data)
        .dc.b   0xff                          | +04c  '.'  (dato, rango --data)
        .dc.b   0xf0                          | +04d  '.'  (dato, rango --data)
        .dc.b   0x1d                          | +04e  '.'  (dato, rango --data)
        .dc.b   0x00                          | +04f  '.'  (dato, rango --data)
        .dc.b   0x04                          | +050  '.'  (dato, rango --data)
        .dc.b   0x06                          | +051  '.'  (dato, rango --data)
        .dc.b   0x00                          | +052  '.'  (dato, rango --data)
        .dc.b   0x32                          | +053  '2'  (dato, rango --data)
        .dc.b   0x02                          | +054  '.'  (dato, rango --data)
        .dc.b   0x04                          | +055  '.'  (dato, rango --data)
        .dc.b   0xff                          | +056  '.'  (dato, rango --data)
        .dc.b   0x00                          | +057  '.'  (dato, rango --data)
        .dc.b   0x00                          | +058  '.'  (dato, rango --data)
        .dc.b   0x00                          | +059  '.'  (dato, rango --data)
        .dc.b   0xff                          | +05a  '.'  (dato, rango --data)
        .dc.b   0xf0                          | +05b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +05c  '.'  (dato, rango --data)
        .dc.b   0x10                          | +05d  '.'  (dato, rango --data)
        .dc.b   0xff                          | +05e  '.'  (dato, rango --data)
        .dc.b   0xf0                          | +05f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +060  '.'  (dato, rango --data)
        .dc.b   0x10                          | +061  '.'  (dato, rango --data)
        .dc.b   0x02                          | +062  '.'  (dato, rango --data)
        .dc.b   0x00                          | +063  '.'  (dato, rango --data)
        .dc.b   0x00                          | +064  '.'  (dato, rango --data)
        .dc.b   0x24                          | +065  '$'  (dato, rango --data)
        .dc.b   0x36                          | +066  '6'  (dato, rango --data)
        .dc.b   0x3a                          | +067  ':'  (dato, rango --data)
        .dc.b   0xff                          | +068  '.'  (dato, rango --data)
        .dc.b   0xff                          | +069  '.'  (dato, rango --data)
        .dc.b   0x00                          | +06a  '.'  (dato, rango --data)
        .dc.b   0x00                          | +06b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +06c  '.'  (dato, rango --data)
        .dc.b   0x00                          | +06d  '.'  (dato, rango --data)
        .dc.b   0x02                          | +06e  '.'  (dato, rango --data)
        .dc.b   0x00                          | +06f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +070  '.'  (dato, rango --data)
        .dc.b   0x24                          | +071  '$'  (dato, rango --data)
        .dc.b   0x36                          | +072  '6'  (dato, rango --data)
        .dc.b   0x44                          | +073  'D'  (dato, rango --data)
        .dc.b   0xff                          | +074  '.'  (dato, rango --data)
        .dc.b   0xff                          | +075  '.'  (dato, rango --data)
        .dc.b   0xff                          | +076  '.'  (dato, rango --data)
        .dc.b   0xf0                          | +077  '.'  (dato, rango --data)
        .dc.b   0x00                          | +078  '.'  (dato, rango --data)
        .dc.b   0x10                          | +079  '.'  (dato, rango --data)
        .dc.b   0x02                          | +07a  '.'  (dato, rango --data)
        .dc.b   0x00                          | +07b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +07c  '.'  (dato, rango --data)
        .dc.b   0x24                          | +07d  '$'  (dato, rango --data)
        .dc.b   0x36                          | +07e  '6'  (dato, rango --data)
        .dc.b   0x4e                          | +07f  'N'  (dato, rango --data)
        .dc.b   0xff                          | +080  '.'  (dato, rango --data)
        .dc.b   0xff                          | +081  '.'  (dato, rango --data)
        .dc.b   0xff                          | +082  '.'  (dato, rango --data)
        .dc.b   0xf0                          | +083  '.'  (dato, rango --data)
        .dc.b   0xff                          | +084  '.'  (dato, rango --data)
        .dc.b   0xf0                          | +085  '.'  (dato, rango --data)
        .dc.b   0x02                          | +086  '.'  (dato, rango --data)
        .dc.b   0x00                          | +087  '.'  (dato, rango --data)
        .dc.b   0x00                          | +088  '.'  (dato, rango --data)
        .dc.b   0x24                          | +089  '$'  (dato, rango --data)
        .dc.b   0x36                          | +08a  '6'  (dato, rango --data)
        .dc.b   0x58                          | +08b  'X'  (dato, rango --data)
        .dc.b   0xff                          | +08c  '.'  (dato, rango --data)
        .dc.b   0xff                          | +08d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +08e  '.'  (dato, rango --data)
        .dc.b   0x10                          | +08f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +090  '.'  (dato, rango --data)
        .dc.b   0x10                          | +091  '.'  (dato, rango --data)
        .dc.b   0x02                          | +092  '.'  (dato, rango --data)
        .dc.b   0x00                          | +093  '.'  (dato, rango --data)
        .dc.b   0x00                          | +094  '.'  (dato, rango --data)
        .dc.b   0x24                          | +095  '$'  (dato, rango --data)
        .dc.b   0x36                          | +096  '6'  (dato, rango --data)
        .dc.b   0x62                          | +097  'b'  (dato, rango --data)
        .dc.b   0xff                          | +098  '.'  (dato, rango --data)
        .dc.b   0xff                          | +099  '.'  (dato, rango --data)
        .dc.b   0x00                          | +09a  '.'  (dato, rango --data)
        .dc.b   0x10                          | +09b  '.'  (dato, rango --data)
        .dc.b   0xff                          | +09c  '.'  (dato, rango --data)
        .dc.b   0xf0                          | +09d  '.'  (dato, rango --data)
        .dc.b   0x1d                          | +09e  '.'  (dato, rango --data)
        .dc.b   0x00                          | +09f  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0a0  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0a1  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0a2  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0a3  '.'  (dato, rango --data)

| ----------------------------------------------------------------------------
|  Shell_HitboxList_C_05a506  @ $05A506  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Shell_HitboxList_C_05a506, "ax", @progbits
        .global Shell_HitboxList_C_05a506
Shell_HitboxList_C_05a506:
        .dc.b   0x00                          | +000  '.'  (dato, rango --data)
        .dc.b   0x01                          | +001  '.'  (dato, rango --data)
        .dc.b   0xff                          | +002  '.'  (dato, rango --data)
        .dc.b   0xff                          | +003  '.'  (dato, rango --data)
        .dc.b   0xff                          | +004  '.'  (dato, rango --data)
        .dc.b   0xff                          | +005  '.'  (dato, rango --data)
        .dc.b   0x00                          | +006  '.'  (dato, rango --data)
        .dc.b   0x00                          | +007  '.'  (dato, rango --data)
        .dc.b   0x00                          | +008  '.'  (dato, rango --data)
        .dc.b   0x00                          | +009  '.'  (dato, rango --data)
        .dc.b   0xff                          | +00a  '.'  (dato, rango --data)
        .dc.b   0xf8                          | +00b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +00c  '.'  (dato, rango --data)
        .dc.b   0x08                          | +00d  '.'  (dato, rango --data)
        .dc.b   0xff                          | +00e  '.'  (dato, rango --data)
        .dc.b   0xf8                          | +00f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +010  '.'  (dato, rango --data)
        .dc.b   0x08                          | +011  '.'  (dato, rango --data)
        .dc.b   0x02                          | +012  '.'  (dato, rango --data)
        .dc.b   0x00                          | +013  '.'  (dato, rango --data)
        .dc.b   0x00                          | +014  '.'  (dato, rango --data)
        .dc.b   0x24                          | +015  '$'  (dato, rango --data)
        .dc.b   0x36                          | +016  '6'  (dato, rango --data)
        .dc.b   0x6c                          | +017  'l'  (dato, rango --data)
        .dc.b   0xff                          | +018  '.'  (dato, rango --data)
        .dc.b   0xff                          | +019  '.'  (dato, rango --data)
        .dc.b   0x00                          | +01a  '.'  (dato, rango --data)
        .dc.b   0x00                          | +01b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +01c  '.'  (dato, rango --data)
        .dc.b   0x00                          | +01d  '.'  (dato, rango --data)
        .dc.b   0x02                          | +01e  '.'  (dato, rango --data)
        .dc.b   0x00                          | +01f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +020  '.'  (dato, rango --data)
        .dc.b   0x24                          | +021  '$'  (dato, rango --data)
        .dc.b   0x36                          | +022  '6'  (dato, rango --data)
        .dc.b   0x76                          | +023  'v'  (dato, rango --data)
        .dc.b   0xff                          | +024  '.'  (dato, rango --data)
        .dc.b   0xff                          | +025  '.'  (dato, rango --data)
        .dc.b   0xff                          | +026  '.'  (dato, rango --data)
        .dc.b   0xf8                          | +027  '.'  (dato, rango --data)
        .dc.b   0x00                          | +028  '.'  (dato, rango --data)
        .dc.b   0x08                          | +029  '.'  (dato, rango --data)
        .dc.b   0x02                          | +02a  '.'  (dato, rango --data)
        .dc.b   0x00                          | +02b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +02c  '.'  (dato, rango --data)
        .dc.b   0x24                          | +02d  '$'  (dato, rango --data)
        .dc.b   0x36                          | +02e  '6'  (dato, rango --data)
        .dc.b   0x80                          | +02f  '.'  (dato, rango --data)
        .dc.b   0xff                          | +030  '.'  (dato, rango --data)
        .dc.b   0xff                          | +031  '.'  (dato, rango --data)
        .dc.b   0xff                          | +032  '.'  (dato, rango --data)
        .dc.b   0xf8                          | +033  '.'  (dato, rango --data)
        .dc.b   0xff                          | +034  '.'  (dato, rango --data)
        .dc.b   0xf8                          | +035  '.'  (dato, rango --data)
        .dc.b   0x02                          | +036  '.'  (dato, rango --data)
        .dc.b   0x00                          | +037  '.'  (dato, rango --data)
        .dc.b   0x00                          | +038  '.'  (dato, rango --data)
        .dc.b   0x24                          | +039  '$'  (dato, rango --data)
        .dc.b   0x36                          | +03a  '6'  (dato, rango --data)
        .dc.b   0x8a                          | +03b  '.'  (dato, rango --data)
        .dc.b   0xff                          | +03c  '.'  (dato, rango --data)
        .dc.b   0xff                          | +03d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +03e  '.'  (dato, rango --data)
        .dc.b   0x08                          | +03f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +040  '.'  (dato, rango --data)
        .dc.b   0x08                          | +041  '.'  (dato, rango --data)
        .dc.b   0x02                          | +042  '.'  (dato, rango --data)
        .dc.b   0x00                          | +043  '.'  (dato, rango --data)
        .dc.b   0x00                          | +044  '.'  (dato, rango --data)
        .dc.b   0x24                          | +045  '$'  (dato, rango --data)
        .dc.b   0x36                          | +046  '6'  (dato, rango --data)
        .dc.b   0x94                          | +047  '.'  (dato, rango --data)
        .dc.b   0xff                          | +048  '.'  (dato, rango --data)
        .dc.b   0xff                          | +049  '.'  (dato, rango --data)
        .dc.b   0x00                          | +04a  '.'  (dato, rango --data)
        .dc.b   0x08                          | +04b  '.'  (dato, rango --data)
        .dc.b   0xff                          | +04c  '.'  (dato, rango --data)
        .dc.b   0xf8                          | +04d  '.'  (dato, rango --data)
        .dc.b   0x1d                          | +04e  '.'  (dato, rango --data)
        .dc.b   0x01                          | +04f  '.'  (dato, rango --data)
        .dc.b   0xff                          | +050  '.'  (dato, rango --data)
        .dc.b   0xff                          | +051  '.'  (dato, rango --data)
        .dc.b   0xff                          | +052  '.'  (dato, rango --data)
        .dc.b   0xff                          | +053  '.'  (dato, rango --data)

| ----------------------------------------------------------------------------
|  Gunner2_Shell_05a55a  @ $05A55A  (226 B)
| ----------------------------------------------------------------------------
        .section .text.Gunner2_Shell_05a55a, "ax", @progbits
        .global Gunner2_Shell_05a55a
Gunner2_Shell_05a55a:
        cmpi.b  #0x2,0x98(a6)                   | +000
        bne.w   .L05a584                        | +006
        lea     Shell_HitboxList_C_05a506(pc),a0 | +00a
        move.l  a0,0x48(a6)                     | +00e
        lea     Shell_HitboxList_A_05a40e(pc),a0 | +012
        move.l  a0,0x4c(a6)                     | +016
        jsr     0x283ca.l                       | +01a
        move.w  #0x200,0x36(a6)                 | +020
        bra.w   .L05a5a0                        | +026
.L05a584:
        lea     0xffff.w,a0                     | +02a
        move.l  a0,0x48(a6)                     | +02e
        lea     Shell_HitboxList_B_05a462(pc),a0 | +032
        move.l  a0,0x4c(a6)                     | +036
        jsr     0x283ca.l                       | +03a
        move.w  #0x400,0x36(a6)                 | +040
.L05a5a0:
        move.w  #0x1064,d0                      | +046
        jsr     0x2352.l                        | +04a
        move.w  #0x57,d1                        | +050
        jsr     0x236e.l                        | +054
        bset    #0x4,0x6b(a6)                   | +05a
        move.w  #0xd000,0x38(a6)                | +060
        move.w  0x36(a6),d1                     | +066
        moveq   #0,d0                           | +06a
        move.b  0x34(a6),d0                     | +06c
        andi.b  #0xf0,d0                        | +070
        jsr     0x13c0e.l                       | +074
        move.w  d1,0x28(a6)                     | +07a
        move.w  d2,0x2a(a6)                     | +07e
        lea     GunnerAnim_Table_059e50__L05a17a(pc),a0 | +082
        move.w  0x5c(a6),d0                     | +086
        movea.l (a0,d0.w),a0                    | +08a
        jsr     0x28cd4.l                       | +08e
        lea     .L05a5f4(pc),a1                 | +094
        move.l  a1,(a6)                         | +098
.L05a5f4:
        jsr     0x27cee.l                       | +09a
        jsr     0x28d70.l                       | +0a0
        jsr     0x283d8.l                       | +0a6
        btst    #0x1,0x13(a6)                   | +0ac
        bne.w   Gunner2_Shell_Explode_05a642    | +0b2
        jsr     0x2870a.l                       | +0b6
        bcs.w   Gunner2_Shell_Explode_05a642    | +0bc
        move.w  0x22(a6),d0                     | +0c0
        addi.w  #0x10,d0                        | +0c4
        cmpi.w  #0x160,d0                       | +0c8
        bcc.w   JmpAbsThunk_05a63c              | +0cc
        move.w  0x24(a6),d0                     | +0d0
        subi.w  #0xf0,d0                        | +0d4
        cmpi.w  #0x120,d0                       | +0d8
        bcc.w   JmpAbsThunk_05a63c              | +0dc
        rts                                     | +0e0

| ----------------------------------------------------------------------------
|  Gunner2_Shell_Explode_05a642  @ $05A642  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Gunner2_Shell_Explode_05a642, "ax", @progbits
        .global Gunner2_Shell_Explode_05a642
Gunner2_Shell_Explode_05a642:
        move.w  #0x1027,d0                      | +000
        jsr     0x2352.l                        | +004
        jsr     0x13600.l                       | +00a
        lea     0x77f6a.l,a1                    | +010
        move.l  a1,(a6)                         | +016
        rts                                     | +018

| ----------------------------------------------------------------------------
|  Gunner2_Child_Init_05a65c  @ $05A65C  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Gunner2_Child_Init_05a65c, "ax", @progbits
        .global Gunner2_Child_Init_05a65c
Gunner2_Child_Init_05a65c:
        move.w  #0x56,d1                        | +000
        jsr     0x236e.l                        | +004

| ----------------------------------------------------------------------------
|  Gunner2_Child_Sync_05a66e  @ $05A66E  (40 B)
| ----------------------------------------------------------------------------
        .section .text.Gunner2_Child_Sync_05a66e, "ax", @progbits
        .global Gunner2_Child_Sync_05a66e
Gunner2_Child_Sync_05a66e:
        movea.l 0xc(a6),a0                      | +000
        move.b  0x3a(a0),0x3a(a6)               | +004
        move.w  0x22(a0),0x22(a6)               | +00a
        move.w  0x24(a0),0x24(a6)               | +010
        move.w  0x38(a0),0x38(a6)               | +016
        move.l  0x5c(a0),0x3c(a6)               | +01c
        jmp     0x5ca2a.l                       | +022

| ----------------------------------------------------------------------------
|  SpriteMap_Walker_05a696  @ $05A696  (152 B)
| ----------------------------------------------------------------------------
        .section .text.SpriteMap_Walker_05a696, "ax", @progbits
        .global SpriteMap_Walker_05a696
SpriteMap_Walker_05a696:
        .dc.b   0x00                          | +000  '.'  (dato, rango --data)
        .dc.b   0x02                          | +001  '.'  (dato, rango --data)
        .dc.b   0x1e                          | +002  '.'  (dato, rango --data)
        .dc.b   0x08                          | +003  '.'  (dato, rango --data)
        .dc.b   0x00                          | +004  '.'  (dato, rango --data)
        .dc.b   0x24                          | +005  '$'  (dato, rango --data)
        .dc.b   0x98                          | +006  '.'  (dato, rango --data)
        .dc.b   0x7a                          | +007  'z'  (dato, rango --data)
        .dc.b   0xff                          | +008  '.'  (dato, rango --data)
        .dc.b   0xff                          | +009  '.'  (dato, rango --data)
        .dc.b   0x00                          | +00a  '.'  (dato, rango --data)
        .dc.b   0x00                          | +00b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +00c  '.'  (dato, rango --data)
        .dc.b   0x00                          | +00d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +00e  '.'  (dato, rango --data)
        .dc.b   0x02                          | +00f  '.'  (dato, rango --data)
        .dc.b   0x1e                          | +010  '.'  (dato, rango --data)
        .dc.b   0x08                          | +011  '.'  (dato, rango --data)
        .dc.b   0x00                          | +012  '.'  (dato, rango --data)
        .dc.b   0x24                          | +013  '$'  (dato, rango --data)
        .dc.b   0x98                          | +014  '.'  (dato, rango --data)
        .dc.b   0x84                          | +015  '.'  (dato, rango --data)
        .dc.b   0xff                          | +016  '.'  (dato, rango --data)
        .dc.b   0xff                          | +017  '.'  (dato, rango --data)
        .dc.b   0x00                          | +018  '.'  (dato, rango --data)
        .dc.b   0x00                          | +019  '.'  (dato, rango --data)
        .dc.b   0x00                          | +01a  '.'  (dato, rango --data)
        .dc.b   0x00                          | +01b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +01c  '.'  (dato, rango --data)
        .dc.b   0x02                          | +01d  '.'  (dato, rango --data)
        .dc.b   0x1e                          | +01e  '.'  (dato, rango --data)
        .dc.b   0x08                          | +01f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +020  '.'  (dato, rango --data)
        .dc.b   0x24                          | +021  '$'  (dato, rango --data)
        .dc.b   0x98                          | +022  '.'  (dato, rango --data)
        .dc.b   0x90                          | +023  '.'  (dato, rango --data)
        .dc.b   0xff                          | +024  '.'  (dato, rango --data)
        .dc.b   0xff                          | +025  '.'  (dato, rango --data)
        .dc.b   0x00                          | +026  '.'  (dato, rango --data)
        .dc.b   0x00                          | +027  '.'  (dato, rango --data)
        .dc.b   0x00                          | +028  '.'  (dato, rango --data)
        .dc.b   0x00                          | +029  '.'  (dato, rango --data)
        .dc.b   0x00                          | +02a  '.'  (dato, rango --data)
        .dc.b   0x02                          | +02b  '.'  (dato, rango --data)
        .dc.b   0x1e                          | +02c  '.'  (dato, rango --data)
        .dc.b   0x08                          | +02d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +02e  '.'  (dato, rango --data)
        .dc.b   0x24                          | +02f  '$'  (dato, rango --data)
        .dc.b   0x98                          | +030  '.'  (dato, rango --data)
        .dc.b   0x9e                          | +031  '.'  (dato, rango --data)
        .dc.b   0xff                          | +032  '.'  (dato, rango --data)
        .dc.b   0xff                          | +033  '.'  (dato, rango --data)
        .dc.b   0x00                          | +034  '.'  (dato, rango --data)
        .dc.b   0x00                          | +035  '.'  (dato, rango --data)
        .dc.b   0x00                          | +036  '.'  (dato, rango --data)
        .dc.b   0x00                          | +037  '.'  (dato, rango --data)
        .dc.b   0x00                          | +038  '.'  (dato, rango --data)
        .dc.b   0x02                          | +039  '.'  (dato, rango --data)
        .dc.b   0x1e                          | +03a  '.'  (dato, rango --data)
        .dc.b   0x08                          | +03b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +03c  '.'  (dato, rango --data)
        .dc.b   0x24                          | +03d  '$'  (dato, rango --data)
        .dc.b   0x98                          | +03e  '.'  (dato, rango --data)
        .dc.b   0xac                          | +03f  '.'  (dato, rango --data)
        .dc.b   0xff                          | +040  '.'  (dato, rango --data)
        .dc.b   0xff                          | +041  '.'  (dato, rango --data)
        .dc.b   0x00                          | +042  '.'  (dato, rango --data)
        .dc.b   0x00                          | +043  '.'  (dato, rango --data)
        .dc.b   0x00                          | +044  '.'  (dato, rango --data)
        .dc.b   0x00                          | +045  '.'  (dato, rango --data)
        .dc.b   0x00                          | +046  '.'  (dato, rango --data)
        .dc.b   0x02                          | +047  '.'  (dato, rango --data)
        .dc.b   0x1e                          | +048  '.'  (dato, rango --data)
        .dc.b   0x08                          | +049  '.'  (dato, rango --data)
        .dc.b   0x00                          | +04a  '.'  (dato, rango --data)
        .dc.b   0x24                          | +04b  '$'  (dato, rango --data)
        .dc.b   0x98                          | +04c  '.'  (dato, rango --data)
        .dc.b   0xba                          | +04d  '.'  (dato, rango --data)
        .dc.b   0xff                          | +04e  '.'  (dato, rango --data)
        .dc.b   0xff                          | +04f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +050  '.'  (dato, rango --data)
        .dc.b   0x00                          | +051  '.'  (dato, rango --data)
        .dc.b   0x00                          | +052  '.'  (dato, rango --data)
        .dc.b   0x00                          | +053  '.'  (dato, rango --data)
        .dc.b   0x00                          | +054  '.'  (dato, rango --data)
        .dc.b   0x02                          | +055  '.'  (dato, rango --data)
        .dc.b   0x1e                          | +056  '.'  (dato, rango --data)
        .dc.b   0x08                          | +057  '.'  (dato, rango --data)
        .dc.b   0x00                          | +058  '.'  (dato, rango --data)
        .dc.b   0x24                          | +059  '$'  (dato, rango --data)
        .dc.b   0x98                          | +05a  '.'  (dato, rango --data)
        .dc.b   0xc8                          | +05b  '.'  (dato, rango --data)
        .dc.b   0xff                          | +05c  '.'  (dato, rango --data)
        .dc.b   0xff                          | +05d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +05e  '.'  (dato, rango --data)
        .dc.b   0x00                          | +05f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +060  '.'  (dato, rango --data)
        .dc.b   0x00                          | +061  '.'  (dato, rango --data)
        .dc.b   0x00                          | +062  '.'  (dato, rango --data)
        .dc.b   0x02                          | +063  '.'  (dato, rango --data)
        .dc.b   0x1e                          | +064  '.'  (dato, rango --data)
        .dc.b   0x08                          | +065  '.'  (dato, rango --data)
        .dc.b   0x00                          | +066  '.'  (dato, rango --data)
        .dc.b   0x24                          | +067  '$'  (dato, rango --data)
        .dc.b   0x98                          | +068  '.'  (dato, rango --data)
        .dc.b   0xd6                          | +069  '.'  (dato, rango --data)
        .dc.b   0xff                          | +06a  '.'  (dato, rango --data)
        .dc.b   0xff                          | +06b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +06c  '.'  (dato, rango --data)
        .dc.b   0x00                          | +06d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +06e  '.'  (dato, rango --data)
        .dc.b   0x00                          | +06f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +070  '.'  (dato, rango --data)
        .dc.b   0x02                          | +071  '.'  (dato, rango --data)
        .dc.b   0x1e                          | +072  '.'  (dato, rango --data)
        .dc.b   0x08                          | +073  '.'  (dato, rango --data)
        .dc.b   0x00                          | +074  '.'  (dato, rango --data)
        .dc.b   0x24                          | +075  '$'  (dato, rango --data)
        .dc.b   0x98                          | +076  '.'  (dato, rango --data)
        .dc.b   0xec                          | +077  '.'  (dato, rango --data)
        .dc.b   0xff                          | +078  '.'  (dato, rango --data)
        .dc.b   0xff                          | +079  '.'  (dato, rango --data)
        .dc.b   0x00                          | +07a  '.'  (dato, rango --data)
        .dc.b   0x00                          | +07b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +07c  '.'  (dato, rango --data)
        .dc.b   0x00                          | +07d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +07e  '.'  (dato, rango --data)
        .dc.b   0x02                          | +07f  '.'  (dato, rango --data)
        .dc.b   0x1e                          | +080  '.'  (dato, rango --data)
        .dc.b   0x08                          | +081  '.'  (dato, rango --data)
        .dc.b   0x00                          | +082  '.'  (dato, rango --data)
        .dc.b   0x24                          | +083  '$'  (dato, rango --data)
        .dc.b   0x99                          | +084  '.'  (dato, rango --data)
        .dc.b   0x02                          | +085  '.'  (dato, rango --data)
        .dc.b   0xff                          | +086  '.'  (dato, rango --data)
        .dc.b   0xff                          | +087  '.'  (dato, rango --data)
        .dc.b   0x00                          | +088  '.'  (dato, rango --data)
        .dc.b   0x00                          | +089  '.'  (dato, rango --data)
        .dc.b   0x00                          | +08a  '.'  (dato, rango --data)
        .dc.b   0x00                          | +08b  '.'  (dato, rango --data)
        .dc.b   0x06                          | +08c  '.'  (dato, rango --data)
        .dc.b   0x00                          | +08d  '.'  (dato, rango --data)
        .dc.b   0xfc                          | +08e  '.'  (dato, rango --data)
        .dc.b   0x00                          | +08f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +090  '.'  (dato, rango --data)
        .dc.b   0x28                          | +091  '('  (dato, rango --data)
        .dc.b   0x01                          | +092  '.'  (dato, rango --data)
        .dc.b   0x00                          | +093  '.'  (dato, rango --data)
        .dc.b   0x00                          | +094  '.'  (dato, rango --data)
        .dc.b   0x29                          | +095  ')'  (dato, rango --data)
        .dc.b   0xb7                          | +096  '.'  (dato, rango --data)
        .dc.b   0x44                          | +097  'D'  (dato, rango --data)

| ----------------------------------------------------------------------------
|  Walker_Init_05a72e  @ $05A72E  (46 B)
| ----------------------------------------------------------------------------
        .section .text.Walker_Init_05a72e, "ax", @progbits
        .global Walker_Init_05a72e
Walker_Init_05a72e:
        move.w  #0xe,d1                         | +000
        jsr     0x236e.l                        | +004
        move.w  #0x8000,d0                      | +00a
        jsr     0x28134.l                       | +00e
        andi.w  #0xffe3,0x38(a6)                | +014
        ori.w   #0x14,0x38(a6)                  | +01a
        lea     SpriteMap_Walker_05a696(pc),a0  | +020
        jsr     0x28cd4.l                       | +024
        clr.w   0x28(a6)                        | +02a

| ----------------------------------------------------------------------------
|  Walker_Patrol_05a764  @ $05A764  (102 B)
| ----------------------------------------------------------------------------
        .section .text.Walker_Patrol_05a764, "ax", @progbits
        .global Walker_Patrol_05a764
Walker_Patrol_05a764:
        jsr     0x2783a.l                       | +000
        jsr     0x27eba.l                       | +006
        bcc.w   .L05a77e                        | +00c
        jsr     0x27c8c.l                       | +010
        bra.w   .L05a784                        | +016
.L05a77e:
        jsr     0x27a92.l                       | +01a
.L05a784:
        jsr     0x8f470.l                       | +020
        bcc.w   .L05a7a4                        | +026
        tst.w   d2                              | +02a
        bmi.w   .L05a7a4                        | +02c
        move.b  d2,0x72(a6)                     | +030
        lea     Walker_Claimed_05a7ca(pc),a1    | +034
        move.l  a1,(a6)                         | +038
        jsr     0x8f520.l                       | +03a
.L05a7a4:
        btst    #0x5,0x5a(a6)                   | +040
        beq.w   .L05a7b8                        | +046
        bchg    #0x0,0x3a(a6)                   | +04a
        neg.w   0x28(a6)                        | +050
.L05a7b8:
        jsr     0x28d70.l                       | +054
        jsr     0x49fd0.l                       | +05a
        jmp     0x56e1e.l                       | +060

| ----------------------------------------------------------------------------
|  Walker_Claimed_05a7ca  @ $05A7CA  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Walker_Claimed_05a7ca, "ax", @progbits
        .global Walker_Claimed_05a7ca
Walker_Claimed_05a7ca:
        lea     0x29b4a4.l,a0                   | +000
        move.l  a0,0x48(a6)                     | +006
        lea     .L05a7da(pc),a1                 | +00a
        move.l  a1,(a6)                         | +00e
.L05a7da:
        move.b  0x72(a6),d0                     | +010
        jsr     0x8f5dc.l                       | +014
        bcs.w   Walker_Tail_05a7f0__L05a7f4     | +01a

| ----------------------------------------------------------------------------
|  Walker_Tail_05a7f0  @ $05A7F0  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Walker_Tail_05a7f0, "ax", @progbits
        .global Walker_Tail_05a7f0
Walker_Tail_05a7f0:
        bra.w   .L05a7fc                        | +000
        .global Walker_Tail_05a7f0__L05a7f4
Walker_Tail_05a7f0__L05a7f4:
.L05a7f4:
        move.w  d0,0x22(a6)                     | +004
        move.w  d1,0x24(a6)                     | +008
.L05a7fc:
        jsr     0x49fd0.l                       | +00c
        jmp     0x56e1e.l                       | +012

| ----------------------------------------------------------------------------
|  Entity_CmpDepthToParent_05a808  @ $05A808  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpDepthToParent_05a808, "ax", @progbits
        .global Entity_CmpDepthToParent_05a808
Entity_CmpDepthToParent_05a808:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_05a81e                    | +00c

| ----------------------------------------------------------------------------
|  FadeLut_16x16_05a8ba  @ $05A8BA  (256 B)
| ----------------------------------------------------------------------------
        .section .text.FadeLut_16x16_05a8ba, "ax", @progbits
        .global FadeLut_16x16_05a8ba
FadeLut_16x16_05a8ba:
        .dc.b   0x0f                          | +000  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +001  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +002  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +003  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +004  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +005  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +006  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +007  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +008  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +009  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +00a  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +00b  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +00c  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +00d  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +00e  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +00f  '.'  (dato, rango --data)
        .dc.b   0x08                          | +010  '.'  (dato, rango --data)
        .dc.b   0x08                          | +011  '.'  (dato, rango --data)
        .dc.b   0x08                          | +012  '.'  (dato, rango --data)
        .dc.b   0x08                          | +013  '.'  (dato, rango --data)
        .dc.b   0x08                          | +014  '.'  (dato, rango --data)
        .dc.b   0x08                          | +015  '.'  (dato, rango --data)
        .dc.b   0x08                          | +016  '.'  (dato, rango --data)
        .dc.b   0x08                          | +017  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +018  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +019  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +01a  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +01b  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +01c  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +01d  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +01e  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +01f  '.'  (dato, rango --data)
        .dc.b   0x04                          | +020  '.'  (dato, rango --data)
        .dc.b   0x04                          | +021  '.'  (dato, rango --data)
        .dc.b   0x04                          | +022  '.'  (dato, rango --data)
        .dc.b   0x04                          | +023  '.'  (dato, rango --data)
        .dc.b   0x04                          | +024  '.'  (dato, rango --data)
        .dc.b   0x08                          | +025  '.'  (dato, rango --data)
        .dc.b   0x08                          | +026  '.'  (dato, rango --data)
        .dc.b   0x08                          | +027  '.'  (dato, rango --data)
        .dc.b   0x08                          | +028  '.'  (dato, rango --data)
        .dc.b   0x08                          | +029  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +02a  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +02b  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +02c  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +02d  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +02e  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +02f  '.'  (dato, rango --data)
        .dc.b   0x04                          | +030  '.'  (dato, rango --data)
        .dc.b   0x04                          | +031  '.'  (dato, rango --data)
        .dc.b   0x04                          | +032  '.'  (dato, rango --data)
        .dc.b   0x04                          | +033  '.'  (dato, rango --data)
        .dc.b   0x08                          | +034  '.'  (dato, rango --data)
        .dc.b   0x08                          | +035  '.'  (dato, rango --data)
        .dc.b   0x08                          | +036  '.'  (dato, rango --data)
        .dc.b   0x08                          | +037  '.'  (dato, rango --data)
        .dc.b   0x0c                          | +038  '.'  (dato, rango --data)
        .dc.b   0x0c                          | +039  '.'  (dato, rango --data)
        .dc.b   0x0c                          | +03a  '.'  (dato, rango --data)
        .dc.b   0x0c                          | +03b  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +03c  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +03d  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +03e  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +03f  '.'  (dato, rango --data)
        .dc.b   0x02                          | +040  '.'  (dato, rango --data)
        .dc.b   0x02                          | +041  '.'  (dato, rango --data)
        .dc.b   0x02                          | +042  '.'  (dato, rango --data)
        .dc.b   0x04                          | +043  '.'  (dato, rango --data)
        .dc.b   0x04                          | +044  '.'  (dato, rango --data)
        .dc.b   0x04                          | +045  '.'  (dato, rango --data)
        .dc.b   0x08                          | +046  '.'  (dato, rango --data)
        .dc.b   0x08                          | +047  '.'  (dato, rango --data)
        .dc.b   0x08                          | +048  '.'  (dato, rango --data)
        .dc.b   0x0c                          | +049  '.'  (dato, rango --data)
        .dc.b   0x0c                          | +04a  '.'  (dato, rango --data)
        .dc.b   0x0c                          | +04b  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +04c  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +04d  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +04e  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +04f  '.'  (dato, rango --data)
        .dc.b   0x02                          | +050  '.'  (dato, rango --data)
        .dc.b   0x02                          | +051  '.'  (dato, rango --data)
        .dc.b   0x04                          | +052  '.'  (dato, rango --data)
        .dc.b   0x04                          | +053  '.'  (dato, rango --data)
        .dc.b   0x04                          | +054  '.'  (dato, rango --data)
        .dc.b   0x08                          | +055  '.'  (dato, rango --data)
        .dc.b   0x08                          | +056  '.'  (dato, rango --data)
        .dc.b   0x08                          | +057  '.'  (dato, rango --data)
        .dc.b   0x0a                          | +058  '.'  (dato, rango --data)
        .dc.b   0x0a                          | +059  '.'  (dato, rango --data)
        .dc.b   0x0c                          | +05a  '.'  (dato, rango --data)
        .dc.b   0x0c                          | +05b  '.'  (dato, rango --data)
        .dc.b   0x0c                          | +05c  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +05d  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +05e  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +05f  '.'  (dato, rango --data)
        .dc.b   0x02                          | +060  '.'  (dato, rango --data)
        .dc.b   0x02                          | +061  '.'  (dato, rango --data)
        .dc.b   0x04                          | +062  '.'  (dato, rango --data)
        .dc.b   0x04                          | +063  '.'  (dato, rango --data)
        .dc.b   0x06                          | +064  '.'  (dato, rango --data)
        .dc.b   0x06                          | +065  '.'  (dato, rango --data)
        .dc.b   0x08                          | +066  '.'  (dato, rango --data)
        .dc.b   0x08                          | +067  '.'  (dato, rango --data)
        .dc.b   0x08                          | +068  '.'  (dato, rango --data)
        .dc.b   0x0a                          | +069  '.'  (dato, rango --data)
        .dc.b   0x0a                          | +06a  '.'  (dato, rango --data)
        .dc.b   0x0c                          | +06b  '.'  (dato, rango --data)
        .dc.b   0x0c                          | +06c  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +06d  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +06e  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +06f  '.'  (dato, rango --data)
        .dc.b   0x02                          | +070  '.'  (dato, rango --data)
        .dc.b   0x02                          | +071  '.'  (dato, rango --data)
        .dc.b   0x04                          | +072  '.'  (dato, rango --data)
        .dc.b   0x04                          | +073  '.'  (dato, rango --data)
        .dc.b   0x06                          | +074  '.'  (dato, rango --data)
        .dc.b   0x06                          | +075  '.'  (dato, rango --data)
        .dc.b   0x08                          | +076  '.'  (dato, rango --data)
        .dc.b   0x08                          | +077  '.'  (dato, rango --data)
        .dc.b   0x0a                          | +078  '.'  (dato, rango --data)
        .dc.b   0x0a                          | +079  '.'  (dato, rango --data)
        .dc.b   0x0c                          | +07a  '.'  (dato, rango --data)
        .dc.b   0x0c                          | +07b  '.'  (dato, rango --data)
        .dc.b   0x0e                          | +07c  '.'  (dato, rango --data)
        .dc.b   0x0e                          | +07d  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +07e  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +07f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +080  '.'  (dato, rango --data)
        .dc.b   0x02                          | +081  '.'  (dato, rango --data)
        .dc.b   0x02                          | +082  '.'  (dato, rango --data)
        .dc.b   0x04                          | +083  '.'  (dato, rango --data)
        .dc.b   0x04                          | +084  '.'  (dato, rango --data)
        .dc.b   0x06                          | +085  '.'  (dato, rango --data)
        .dc.b   0x06                          | +086  '.'  (dato, rango --data)
        .dc.b   0x08                          | +087  '.'  (dato, rango --data)
        .dc.b   0x0a                          | +088  '.'  (dato, rango --data)
        .dc.b   0x0a                          | +089  '.'  (dato, rango --data)
        .dc.b   0x0c                          | +08a  '.'  (dato, rango --data)
        .dc.b   0x0c                          | +08b  '.'  (dato, rango --data)
        .dc.b   0x0e                          | +08c  '.'  (dato, rango --data)
        .dc.b   0x0e                          | +08d  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +08e  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +08f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +090  '.'  (dato, rango --data)
        .dc.b   0x02                          | +091  '.'  (dato, rango --data)
        .dc.b   0x02                          | +092  '.'  (dato, rango --data)
        .dc.b   0x04                          | +093  '.'  (dato, rango --data)
        .dc.b   0x06                          | +094  '.'  (dato, rango --data)
        .dc.b   0x06                          | +095  '.'  (dato, rango --data)
        .dc.b   0x07                          | +096  '.'  (dato, rango --data)
        .dc.b   0x07                          | +097  '.'  (dato, rango --data)
        .dc.b   0x08                          | +098  '.'  (dato, rango --data)
        .dc.b   0x0a                          | +099  '.'  (dato, rango --data)
        .dc.b   0x0a                          | +09a  '.'  (dato, rango --data)
        .dc.b   0x0c                          | +09b  '.'  (dato, rango --data)
        .dc.b   0x0e                          | +09c  '.'  (dato, rango --data)
        .dc.b   0x0e                          | +09d  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +09e  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +09f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0a0  '.'  (dato, rango --data)
        .dc.b   0x02                          | +0a1  '.'  (dato, rango --data)
        .dc.b   0x03                          | +0a2  '.'  (dato, rango --data)
        .dc.b   0x03                          | +0a3  '.'  (dato, rango --data)
        .dc.b   0x04                          | +0a4  '.'  (dato, rango --data)
        .dc.b   0x06                          | +0a5  '.'  (dato, rango --data)
        .dc.b   0x06                          | +0a6  '.'  (dato, rango --data)
        .dc.b   0x07                          | +0a7  '.'  (dato, rango --data)
        .dc.b   0x08                          | +0a8  '.'  (dato, rango --data)
        .dc.b   0x08                          | +0a9  '.'  (dato, rango --data)
        .dc.b   0x0a                          | +0aa  '.'  (dato, rango --data)
        .dc.b   0x0c                          | +0ab  '.'  (dato, rango --data)
        .dc.b   0x0c                          | +0ac  '.'  (dato, rango --data)
        .dc.b   0x0e                          | +0ad  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +0ae  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +0af  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0b0  '.'  (dato, rango --data)
        .dc.b   0x02                          | +0b1  '.'  (dato, rango --data)
        .dc.b   0x03                          | +0b2  '.'  (dato, rango --data)
        .dc.b   0x03                          | +0b3  '.'  (dato, rango --data)
        .dc.b   0x04                          | +0b4  '.'  (dato, rango --data)
        .dc.b   0x06                          | +0b5  '.'  (dato, rango --data)
        .dc.b   0x07                          | +0b6  '.'  (dato, rango --data)
        .dc.b   0x07                          | +0b7  '.'  (dato, rango --data)
        .dc.b   0x08                          | +0b8  '.'  (dato, rango --data)
        .dc.b   0x0a                          | +0b9  '.'  (dato, rango --data)
        .dc.b   0x0b                          | +0ba  '.'  (dato, rango --data)
        .dc.b   0x0b                          | +0bb  '.'  (dato, rango --data)
        .dc.b   0x0c                          | +0bc  '.'  (dato, rango --data)
        .dc.b   0x0e                          | +0bd  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +0be  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +0bf  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0c0  '.'  (dato, rango --data)
        .dc.b   0x01                          | +0c1  '.'  (dato, rango --data)
        .dc.b   0x02                          | +0c2  '.'  (dato, rango --data)
        .dc.b   0x03                          | +0c3  '.'  (dato, rango --data)
        .dc.b   0x04                          | +0c4  '.'  (dato, rango --data)
        .dc.b   0x04                          | +0c5  '.'  (dato, rango --data)
        .dc.b   0x06                          | +0c6  '.'  (dato, rango --data)
        .dc.b   0x07                          | +0c7  '.'  (dato, rango --data)
        .dc.b   0x08                          | +0c8  '.'  (dato, rango --data)
        .dc.b   0x0a                          | +0c9  '.'  (dato, rango --data)
        .dc.b   0x0a                          | +0ca  '.'  (dato, rango --data)
        .dc.b   0x0b                          | +0cb  '.'  (dato, rango --data)
        .dc.b   0x0c                          | +0cc  '.'  (dato, rango --data)
        .dc.b   0x0e                          | +0cd  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +0ce  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +0cf  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0d0  '.'  (dato, rango --data)
        .dc.b   0x01                          | +0d1  '.'  (dato, rango --data)
        .dc.b   0x02                          | +0d2  '.'  (dato, rango --data)
        .dc.b   0x03                          | +0d3  '.'  (dato, rango --data)
        .dc.b   0x04                          | +0d4  '.'  (dato, rango --data)
        .dc.b   0x06                          | +0d5  '.'  (dato, rango --data)
        .dc.b   0x07                          | +0d6  '.'  (dato, rango --data)
        .dc.b   0x07                          | +0d7  '.'  (dato, rango --data)
        .dc.b   0x08                          | +0d8  '.'  (dato, rango --data)
        .dc.b   0x09                          | +0d9  '.'  (dato, rango --data)
        .dc.b   0x0a                          | +0da  '.'  (dato, rango --data)
        .dc.b   0x0b                          | +0db  '.'  (dato, rango --data)
        .dc.b   0x0c                          | +0dc  '.'  (dato, rango --data)
        .dc.b   0x0e                          | +0dd  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +0de  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +0df  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0e0  '.'  (dato, rango --data)
        .dc.b   0x01                          | +0e1  '.'  (dato, rango --data)
        .dc.b   0x02                          | +0e2  '.'  (dato, rango --data)
        .dc.b   0x03                          | +0e3  '.'  (dato, rango --data)
        .dc.b   0x04                          | +0e4  '.'  (dato, rango --data)
        .dc.b   0x05                          | +0e5  '.'  (dato, rango --data)
        .dc.b   0x06                          | +0e6  '.'  (dato, rango --data)
        .dc.b   0x07                          | +0e7  '.'  (dato, rango --data)
        .dc.b   0x08                          | +0e8  '.'  (dato, rango --data)
        .dc.b   0x09                          | +0e9  '.'  (dato, rango --data)
        .dc.b   0x0a                          | +0ea  '.'  (dato, rango --data)
        .dc.b   0x0b                          | +0eb  '.'  (dato, rango --data)
        .dc.b   0x0c                          | +0ec  '.'  (dato, rango --data)
        .dc.b   0x0e                          | +0ed  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +0ee  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +0ef  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0f0  '.'  (dato, rango --data)
        .dc.b   0x01                          | +0f1  '.'  (dato, rango --data)
        .dc.b   0x02                          | +0f2  '.'  (dato, rango --data)
        .dc.b   0x03                          | +0f3  '.'  (dato, rango --data)
        .dc.b   0x04                          | +0f4  '.'  (dato, rango --data)
        .dc.b   0x05                          | +0f5  '.'  (dato, rango --data)
        .dc.b   0x06                          | +0f6  '.'  (dato, rango --data)
        .dc.b   0x07                          | +0f7  '.'  (dato, rango --data)
        .dc.b   0x08                          | +0f8  '.'  (dato, rango --data)
        .dc.b   0x09                          | +0f9  '.'  (dato, rango --data)
        .dc.b   0x0a                          | +0fa  '.'  (dato, rango --data)
        .dc.b   0x0b                          | +0fb  '.'  (dato, rango --data)
        .dc.b   0x0c                          | +0fc  '.'  (dato, rango --data)
        .dc.b   0x0d                          | +0fd  '.'  (dato, rango --data)
        .dc.b   0x0e                          | +0fe  '.'  (dato, rango --data)
        .dc.b   0x0f                          | +0ff  '.'  (dato, rango --data)

| ----------------------------------------------------------------------------
|  VRAM_FixAutoclear_Reset_05a9ba  @ $05A9BA  (28 B)
| ----------------------------------------------------------------------------
        .section .text.VRAM_FixAutoclear_Reset_05a9ba, "ax", @progbits
        .global VRAM_FixAutoclear_Reset_05a9ba
VRAM_FixAutoclear_Reset_05a9ba:
        lea     0x108080.l,a5                   | +000
        clr.b   0x10e1ec.l                      | +006
        clr.w   0x4254(a5)                      | +00c
        clr.w   0x6148(a5)                      | +010
        move.w  #0x348,0x614a(a5)               | +014
        rts                                     | +01a
