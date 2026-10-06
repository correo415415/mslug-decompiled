| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave ??? — (borrador)
|  Región: $053F96..$055258  (4,746 B, 41 entradas, 8 huecos)
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
|  TaskHandler_053f96  @ $053F96  (198 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_053f96, "ax", @progbits
        .global TaskHandler_053f96
TaskHandler_053f96:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x8b,d1                        | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0x90,0x70(a6)                  | +014
        move.b  #0xff,0x32(a6)                  | +01a
        move.b  #0xff,0x33(a6)                  | +020
        move.b  #0x0,0x3a(a6)                   | +026
        move.w  #0x78,0x66(a6)                  | +02c
        lea     0x298936.l,a0                   | +032
        jsr     0x28cd4.l                       | +038
        lea     .L053fda(pc),a1                 | +03e
        move.l  a1,(a6)                         | +042
.L053fda:
        clr.b   0x10e39a.l                      | +044
        jsr     0x2783a.l                       | +04a
        jsr     0x28d70.l                       | +050
        cmpi.b  #0x5,0x58(a6)                   | +056
        beq.w   .L05401c                        | +05c
        jsr     0x2870a.l                       | +060
        bcc.w   .L054012                        | +066
        lea     0x5e766.l,a0                    | +06a
        jsr     0x5e770.l                       | +070
        bclr    #0x3,0x13(a6)                   | +076
.L054012:
        cmpi.w  #0x3c,0x66(a6)                  | +07c
        bgt.w   .L05404a                        | +082
.L05401c:
        move.w  #0x1023,d0                      | +086
        jsr     0x2352.l                        | +08a
        bclr    #0x3,0x13(a6)                   | +090
        lea     0x29a278.l,a1                   | +096
        jsr     0x77c7e.l                       | +09c
        lea     0x29a28a.l,a1                   | +0a2
        jsr     0x77c7e.l                       | +0a8
        lea     TaskHandler_05405c(pc),a1       | +0ae
        move.l  a1,(a6)                         | +0b2
.L05404a:
        jsr     0x4fa70.l                       | +0b4
        bcc.w   .L05405a                        | +0ba
        jmp     0x518.l                         | +0be
.L05405a:
        rts                                     | +0c4

| ----------------------------------------------------------------------------
|  TaskHandler_05405c  @ $05405C  (174 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_05405c, "ax", @progbits
        .global TaskHandler_05405c
TaskHandler_05405c:
        jsr     0x2783a.l                       | +000
        lea     0x29b29a.l,a1                   | +006
        jsr     0x43fac.l                       | +00c
        lea     0x298958.l,a0                   | +012
        jsr     0x28cd4.l                       | +018
        lea     .L054080(pc),a1                 | +01e
        move.l  a1,(a6)                         | +022
.L054080:
        clr.b   0x10e39a.l                      | +024
        jsr     0x2783a.l                       | +02a
        jsr     0x28d70.l                       | +030
        jsr     0x2870a.l                       | +036
        bcc.w   .L0540ae                        | +03c
        lea     0x5e766.l,a0                    | +040
        jsr     0x5e770.l                       | +046
        bclr    #0x3,0x13(a6)                   | +04c
.L0540ae:
        jsr     0x28758.l                       | +052
        bcc.w   .L0540f8                        | +058
        bclr    #0x0,0x13(a6)                   | +05c
        move.w  #0x1030,d0                      | +062
        jsr     0x2352.l                        | +066
        lea     0x29a278.l,a1                   | +06c
        jsr     0x77c7e.l                       | +072
        lea     0x29a28a.l,a1                   | +078
        jsr     0x77c7e.l                       | +07e
        lea     0x29a3ce.l,a1                   | +084
        jsr     0x77c7e.l                       | +08a
        move.w  #0xf000,0x38(a0)                | +090
        lea     TaskHandler_05410a(pc),a1       | +096
        move.l  a1,(a6)                         | +09a
.L0540f8:
        jsr     0x4fa70.l                       | +09c
        bcc.w   .L054108                        | +0a2
        jmp     0x518.l                         | +0a6
.L054108:
        rts                                     | +0ac

| ----------------------------------------------------------------------------
|  TaskHandler_05410a  @ $05410A  (86 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_05410a, "ax", @progbits
        .global TaskHandler_05410a
TaskHandler_05410a:
        move.l  #0x2000,d0                      | +000
        jsr     0x51a28.l                       | +006
        jsr     0x2783a.l                       | +00c
        lea     0x29b2b2.l,a1                   | +012
        jsr     0x43fac.l                       | +018
        lea     0xffff.w,a0                     | +01e
        move.l  a0,0x48(a6)                     | +022
        lea     0x29897a.l,a0                   | +026
        jsr     0x28cd4.l                       | +02c
        lea     .L054142(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L054142:
        jsr     0x2783a.l                       | +038
        jsr     0x28d70.l                       | +03e
        jsr     0x4fa70.l                       | +044
        bcc.w   .L05415e                        | +04a
        jmp     0x518.l                         | +04e
.L05415e:
        rts                                     | +054

| ----------------------------------------------------------------------------
|  TaskHandler_054160  @ $054160  (244 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_054160, "ax", @progbits
        .global TaskHandler_054160
TaskHandler_054160:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x8c,d1                        | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0x40,0x70(a6)                  | +014
        move.b  #0xff,0x32(a6)                  | +01a
        move.b  #0xff,0x33(a6)                  | +020
        move.b  #0x0,0x3a(a6)                   | +026
        move.w  #0x3c,0x66(a6)                  | +02c
        move.w  #0x8000,0x38(a6)                | +032
        jsr     0x267e2.l                       | +038
        jsr     0x27cee.l                       | +03e
        lea     0x29899e.l,a0                   | +044
        jsr     0x28cd4.l                       | +04a
        lea     .L0541b6(pc),a1                 | +050
        move.l  a1,(a6)                         | +054
.L0541b6:
        jsr     0x2783a.l                       | +056
        jsr     0x28108.l                       | +05c
        jsr     0x28d70.l                       | +062
        jsr     0x2870a.l                       | +068
        bcc.w   .L0541e4                        | +06e
        lea     0x5e766.l,a0                    | +072
        jsr     0x5e770.l                       | +078
        bclr    #0x3,0x13(a6)                   | +07e
.L0541e4:
        jsr     0x28758.l                       | +084
        bcc.w   .L054242                        | +08a
        bclr    #0x0,0x13(a6)                   | +08e
        move.l  #0x3000,d0                      | +094
        jsr     0x51a28.l                       | +09a
        move.w  #0x1023,d0                      | +0a0
        jsr     0x2352.l                        | +0a4
        lea     0x29b368.l,a1                   | +0aa
        jsr     0x43fac.l                       | +0b0
        lea     0x29a29c.l,a1                   | +0b6
        jsr     0x77c7e.l                       | +0bc
        lea     0x29a2ae.l,a1                   | +0c2
        jsr     0x77c7e.l                       | +0c8
        lea     0x29a3e0.l,a1                   | +0ce
        jsr     0x77c7e.l                       | +0d4
        jsr     SDS_055266(pc)                  | +0da
        bra.w   .L05424c                        | +0de
.L054242:
        jsr     0x4fa70.l                       | +0e2
        bcc.w   .L054252                        | +0e8
.L05424c:
        jmp     0x518.l                         | +0ec
.L054252:
        rts                                     | +0f2

| ----------------------------------------------------------------------------
|  TaskHandler_054254  @ $054254  (228 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_054254, "ax", @progbits
        .global TaskHandler_054254
TaskHandler_054254:
        move.b  #0x0,0x20(a6)                   | +000
        move.w  #0x118,0x66(a6)                 | +006
        lea     0x2989b4.l,a0                   | +00c
        jsr     0x28cd4.l                       | +012
        jsr     0x2783a.l                       | +018
        lea     0x29b2ca.l,a1                   | +01e
        jsr     0x43fac.l                       | +024
        bra.w   .L0542b6                        | +02a
        movea.l 0x3c(a6),a1                     | +02e
        jsr     0x2942a.l                       | +032
        move.b  #0x1,0x20(a6)                   | +038
        move.w  #0x280,0x66(a6)                 | +03e
        lea     0x298a10.l,a0                   | +044
        jsr     0x28cd4.l                       | +04a
        jsr     0x2783a.l                       | +050
        lea     0x29b2f2.l,a1                   | +056
        jsr     0x43fac.l                       | +05c
.L0542b6:
        move.w  #0x60,0x70(a6)                  | +062
        lea     .L0542c2(pc),a1                 | +068
        move.l  a1,(a6)                         | +06c
.L0542c2:
        clr.b   0x10e39a.l                      | +06e
        jsr     0x2783a.l                       | +074
        jsr     0x28d70.l                       | +07a
        jsr     0x2870a.l                       | +080
        bcc.w   .L0542f0                        | +086
        lea     0x5e766.l,a0                    | +08a
        jsr     0x5e770.l                       | +090
        bclr    #0x3,0x13(a6)                   | +096
.L0542f0:
        cmpi.w  #0xa0,0x66(a6)                  | +09c
        bgt.w   .L054326                        | +0a2
        move.w  #0x102a,d0                      | +0a6
        jsr     0x2352.l                        | +0aa
        lea     0x29a2d2.l,a1                   | +0b0
        jsr     0x77c7e.l                       | +0b6
        cmpi.b  #0x0,0x20(a6)                   | +0bc
        beq.w   .L054320                        | +0c2
        subi.w  #0x30,0x22(a0)                  | +0c6
.L054320:
        lea     TaskHandler_054338(pc),a1       | +0cc
        move.l  a1,(a6)                         | +0d0
.L054326:
        jsr     0x4fa70.l                       | +0d2
        bcc.w   .L054336                        | +0d8
        jmp     0x518.l                         | +0dc
.L054336:
        rts                                     | +0e2

| ----------------------------------------------------------------------------
|  TaskHandler_054338  @ $054338  (192 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_054338, "ax", @progbits
        .global TaskHandler_054338
TaskHandler_054338:
        cmpi.b  #0x0,0x20(a6)                   | +000
        bne.w   .L054352                        | +006
        lea     0x2989ca.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        bra.w   .L05435e                        | +016
.L054352:
        lea     0x298a26.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
.L05435e:
        lea     .L054364(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L054364:
        clr.b   0x10e39a.l                      | +02c
        jsr     0x2783a.l                       | +032
        jsr     0x28d70.l                       | +038
        jsr     0x2870a.l                       | +03e
        bcc.w   .L054392                        | +044
        lea     0x5e766.l,a0                    | +048
        jsr     0x5e770.l                       | +04e
        bclr    #0x3,0x13(a6)                   | +054
.L054392:
        jsr     0x28758.l                       | +05a
        bcc.w   .L0543e6                        | +060
        bclr    #0x0,0x13(a6)                   | +064
        move.w  #0x102b,d0                      | +06a
        jsr     0x2352.l                        | +06e
        cmpi.b  #0x0,0x20(a6)                   | +074
        beq.w   .L0543bc                        | +07a
        subi.w  #0x30,0x22(a6)                  | +07e
.L0543bc:
        lea     0x29a2c0.l,a1                   | +084
        jsr     0x77c7e.l                       | +08a
        move.w  #0xc000,0x38(a0)                | +090
        lea     0x29a3f2.l,a1                   | +096
        jsr     0x77c7e.l                       | +09c
        move.w  #0xc000,0x38(a0)                | +0a2
        lea     TaskHandler_0543f8(pc),a1       | +0a8
        move.l  a1,(a6)                         | +0ac
.L0543e6:
        jsr     0x4fa70.l                       | +0ae
        bcc.w   .L0543f6                        | +0b4
        jmp     0x518.l                         | +0b8
.L0543f6:
        rts                                     | +0be

| ----------------------------------------------------------------------------
|  TaskHandler_0543f8  @ $0543F8  (132 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0543f8, "ax", @progbits
        .global TaskHandler_0543f8
TaskHandler_0543f8:
        jsr     0x2783a.l                       | +000
        cmpi.b  #0x0,0x20(a6)                   | +006
        bne.w   .L05443a                        | +00c
        move.l  #0x3000,d0                      | +010
        jsr     0x51a28.l                       | +016
        movea.l 0xc(a6),a0                      | +01c
        move.b  #0xff,0x20(a0)                  | +020
        lea     0x2989f4.l,a0                   | +026
        jsr     0x28cd4.l                       | +02c
        lea     0x29b2de.l,a1                   | +032
        jsr     0x43fac.l                       | +038
        bra.w   .L054464                        | +03e
.L05443a:
        move.l  #0x1000,d0                      | +042
        jsr     0x51a28.l                       | +048
        addi.w  #0x30,0x22(a6)                  | +04e
        lea     0x298a3c.l,a0                   | +054
        jsr     0x28cd4.l                       | +05a
        lea     0x29b306.l,a1                   | +060
        jsr     0x43fac.l                       | +066
.L054464:
        lea     .L05446a(pc),a1                 | +06c
        move.l  a1,(a6)                         | +070
.L05446a:
        jsr     0x2783a.l                       | +072
        jsr     0x28d70.l                       | +078
        jmp     0x518.l                         | +07e

| ----------------------------------------------------------------------------
|  TaskHandler_05447c  @ $05447C  (2 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_05447c, "ax", @progbits
        .global TaskHandler_05447c
TaskHandler_05447c:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  TaskHandler_05447e  @ $05447E  (76 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_05447e, "ax", @progbits
        .global TaskHandler_05447e
TaskHandler_05447e:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x60,0x70(a6)                  | +00a
        lea     TaskHandler_054254(pc),a1       | +010
        jsr     0x4ae.l                         | +014
        jsr     0x5dd22.l                       | +01a
        lea     TaskHandler_0545c0(pc),a1       | +020
        jsr     0x4ae.l                         | +024
        jsr     0x5dd22.l                       | +02a
        lea     TaskHandler_054584(pc),a1       | +030
        jsr     0x4ae.l                         | +034
        jsr     0x5dd22.l                       | +03a
        move.b  #0x0,0x20(a6)                   | +040
        move.b  #0x0,0x80(a6)                   | +046

| ----------------------------------------------------------------------------
|  TaskHandler_0544ca  @ $0544CA  (78 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0544ca, "ax", @progbits
        .global TaskHandler_0544ca
TaskHandler_0544ca:
        move.b  #0x0,0x21(a6)                   | +000
        lea     .L0544d6(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L0544d6:
        jsr     0x2783a.l                       | +00c
        cmpi.b  #0x1,0x21(a6)                   | +012
        bne.w   .L0544ec                        | +018
        lea     TaskHandler_054518(pc),a1       | +01c
        move.l  a1,(a6)                         | +020
.L0544ec:
        cmpi.b  #0xff,0x80(a6)                  | +022
        bne.w   .L0544fc                        | +028
        lea     TaskHandler_054566(pc),a1       | +02c
        move.l  a1,(a6)                         | +030
.L0544fc:
        jsr     0x6f0.l                         | +032
        bcs.w   .L054516                        | +038
        jsr     0x4fa70.l                       | +03c
        bcc.w   .L054516                        | +042
        jmp     0x518.l                         | +046
.L054516:
        rts                                     | +04c

| ----------------------------------------------------------------------------
|  TaskHandler_054518  @ $054518  (78 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_054518, "ax", @progbits
        .global TaskHandler_054518
TaskHandler_054518:
        move.b  #0xff,0x21(a6)                  | +000
        lea     .L054524(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L054524:
        jsr     0x2783a.l                       | +00c
        cmpi.b  #0x2,0x21(a6)                   | +012
        bne.w   .L05453a                        | +018
        lea     TaskHandler_0544ca(pc),a1       | +01c
        move.l  a1,(a6)                         | +020
.L05453a:
        cmpi.b  #0xff,0x80(a6)                  | +022
        bne.w   .L05454a                        | +028
        lea     TaskHandler_054566(pc),a1       | +02c
        move.l  a1,(a6)                         | +030
.L05454a:
        jsr     0x6f0.l                         | +032
        bcs.w   .L054564                        | +038
        jsr     0x4fa70.l                       | +03c
        bcc.w   .L054564                        | +042
        jmp     0x518.l                         | +046
.L054564:
        rts                                     | +04c

| ----------------------------------------------------------------------------
|  TaskHandler_054566  @ $054566  (30 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_054566, "ax", @progbits
        .global TaskHandler_054566
TaskHandler_054566:
        move.b  #0xff,0x21(a6)                  | +000
        lea     .L054572(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L054572:
        jsr     0x6f0.l                         | +00c
        bcs.w   .L054582                        | +012
        jmp     0x518.l                         | +016
.L054582:
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  TaskHandler_054584  @ $054584  (60 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_054584, "ax", @progbits
        .global TaskHandler_054584
TaskHandler_054584:
        lea     .L05458a(pc),a1                 | +000
        move.l  a1,(a6)                         | +004
.L05458a:
        jsr     0x2783a.l                       | +006
        cmpi.w  #0xe0,0x22(a6)                  | +00c
        bgt.w   .L0545be                        | +012
        lea     0x79eb8.l,a1                    | +016
        jsr     0x4ae.l                         | +01c
        jsr     0x5dd22.l                       | +022
        addi.w  #0xb0,0x22(a0)                  | +028
        addi.w  #0x40,0x24(a0)                  | +02e
        jmp     0x518.l                         | +034
.L0545be:
        rts                                     | +03a

| ----------------------------------------------------------------------------
|  TaskHandler_0545c0  @ $0545C0  (250 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0545c0, "ax", @progbits
        .global TaskHandler_0545c0
TaskHandler_0545c0:
        jsr     0x2783a.l                       | +000
        cmpi.w  #0x120,0x22(a6)                 | +006
        ble.w   .L0545d2                        | +00c
        rts                                     | +010
.L0545d2:
        move.w  #0x8d,d1                        | +012
        jsr     0x236e.l                        | +016
        move.w  #0x40,0x70(a6)                  | +01c
        addi.w  #0xa0,0x22(a6)                  | +022
        addi.w  #0x60,0x24(a6)                  | +028
        move.b  #0xff,0x32(a6)                  | +02e
        move.b  #0xff,0x33(a6)                  | +034
        move.w  #0x8000,d0                      | +03a
        move.w  #0xb0,d1                        | +03e
        jsr     0x2813c.l                       | +042
        move.b  #0x0,0x3a(a6)                   | +048
        move.w  #0x3c,0x66(a6)                  | +04e
        lea     0x29908c.l,a0                   | +054
        jsr     0x28cd4.l                       | +05a
        move.w  #0x0,0x72(a6)                   | +060
        move.b  #0x0,0x74(a6)                   | +066
        lea     .L054632(pc),a1                 | +06c
        move.l  a1,(a6)                         | +070
.L054632:
        jsr     0x2783a.l                       | +072
        jsr     0x28108.l                       | +078
        jsr     Sub_0005509C(pc)                | +07e
        jsr     0x28d70.l                       | +082
        jsr     0x2870a.l                       | +088
        bcc.w   .L054664                        | +08e
        lea     0x5e766.l,a0                    | +092
        jsr     0x5e770.l                       | +098
        bclr    #0x3,0x13(a6)                   | +09e
.L054664:
        jsr     0x28758.l                       | +0a4
        bcc.w   .L0546a8                        | +0aa
        bclr    #0x0,0x13(a6)                   | +0ae
        move.w  #0x102a,d0                      | +0b4
        jsr     0x2352.l                        | +0b8
        lea     0x29a2e4.l,a1                   | +0be
        jsr     0x77c7e.l                       | +0c4
        lea     0x29a2f6.l,a1                   | +0ca
        jsr     0x77c7e.l                       | +0d0
        lea     0x29a404.l,a1                   | +0d6
        jsr     0x77c7e.l                       | +0dc
        lea     TaskHandler_0546ba(pc),a1       | +0e2
        move.l  a1,(a6)                         | +0e6
.L0546a8:
        jsr     0x4fa70.l                       | +0e8
        bcc.w   .L0546b8                        | +0ee
        jmp     0x518.l                         | +0f2
.L0546b8:
        rts                                     | +0f8

| ----------------------------------------------------------------------------
|  TaskHandler_0546ba  @ $0546BA  (58 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0546ba, "ax", @progbits
        .global TaskHandler_0546ba
TaskHandler_0546ba:
        lea     0x2990a2.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L0546cc(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L0546cc:
        jsr     0x2783a.l                       | +012
        jsr     0x28108.l                       | +018
        jsr     Sub_0005509C(pc)                | +01e
        jsr     0x28d70.l                       | +022
        jsr     0x4fa70.l                       | +028
        bcc.w   .L0546f2                        | +02e
        jmp     0x518.l                         | +032
.L0546f2:
        rts                                     | +038

| ----------------------------------------------------------------------------
|  TaskHandler_0546f4  @ $0546F4  (214 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0546f4, "ax", @progbits
        .global TaskHandler_0546f4
TaskHandler_0546f4:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x91,d1                        | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0xa0,0x70(a6)                  | +014
        move.b  #0xff,0x32(a6)                  | +01a
        move.b  #0xff,0x33(a6)                  | +020
        move.b  #0x0,0x3a(a6)                   | +026
        move.w  #0x3c,0x66(a6)                  | +02c
        move.w  #0x8000,d0                      | +032
        move.w  #0xa0,d1                        | +036
        jsr     0x2813c.l                       | +03a
        lea     0x298aa8.l,a0                   | +040
        jsr     0x28cd4.l                       | +046
        lea     .L054746(pc),a1                 | +04c
        move.l  a1,(a6)                         | +050
.L054746:
        jsr     0x2783a.l                       | +052
        jsr     0x28108.l                       | +058
        jsr     0x28d70.l                       | +05e
        jsr     0x2870a.l                       | +064
        bcc.w   .L054774                        | +06a
        lea     0x5e766.l,a0                    | +06e
        jsr     0x5e770.l                       | +074
        bclr    #0x3,0x13(a6)                   | +07a
.L054774:
        jsr     0x28758.l                       | +080
        bcc.w   .L0547b8                        | +086
        bclr    #0x0,0x13(a6)                   | +08a
        move.w  #0x102c,d0                      | +090
        jsr     0x2352.l                        | +094
        lea     0x29a32c.l,a1                   | +09a
        jsr     0x77c7e.l                       | +0a0
        lea     0x29a33e.l,a1                   | +0a6
        jsr     0x77c7e.l                       | +0ac
        lea     0x29a428.l,a1                   | +0b2
        jsr     0x77c7e.l                       | +0b8
        lea     TaskHandler_0547ca(pc),a1       | +0be
        move.l  a1,(a6)                         | +0c2
.L0547b8:
        jsr     0x4fa70.l                       | +0c4
        bcc.w   .L0547c8                        | +0ca
        jmp     0x518.l                         | +0ce
.L0547c8:
        rts                                     | +0d4

| ----------------------------------------------------------------------------
|  TaskHandler_0547ca  @ $0547CA  (80 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0547ca, "ax", @progbits
        .global TaskHandler_0547ca
TaskHandler_0547ca:
        jsr     0x2783a.l                       | +000
        lea     0x298b24.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        move.l  0x106f50.l,d0                   | +012
        swap    d0                              | +018
        cmpi.w  #0x390,d0                       | +01a
        bge.w   .L0547fc                        | +01e
        jsr     SDS_0552c8(pc)                  | +022
        lea     0x29b3b2.l,a1                   | +026
        jsr     0x43fac.l                       | +02c
.L0547fc:
        lea     .L054802(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L054802:
        jsr     0x2783a.l                       | +038
        jsr     0x28108.l                       | +03e
        jsr     0x28d70.l                       | +044
        jmp     0x518.l                         | +04a

| ----------------------------------------------------------------------------
|  TaskHandler_05481a  @ $05481A  (2 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_05481a, "ax", @progbits
        .global TaskHandler_05481a
TaskHandler_05481a:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  TaskHandler_05481c  @ $05481C  (172 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_05481c, "ax", @progbits
        .global TaskHandler_05481c
TaskHandler_05481c:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x92,d1                        | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0x80,0x70(a6)                  | +014
        move.b  #0xff,0x32(a6)                  | +01a
        move.b  #0xff,0x33(a6)                  | +020
        move.b  #0x0,0x3a(a6)                   | +026
        move.w  #0x1,0x66(a6)                   | +02c
        lea     0x298b34.l,a0                   | +032
        jsr     0x28cd4.l                       | +038
        lea     .L054860(pc),a1                 | +03e
        move.l  a1,(a6)                         | +042
.L054860:
        jsr     0x2783a.l                       | +044
        jsr     0x28d70.l                       | +04a
        jsr     0x2870a.l                       | +050
        bcc.w   .L054888                        | +056
        lea     0x5e766.l,a0                    | +05a
        jsr     0x5e770.l                       | +060
        bclr    #0x3,0x13(a6)                   | +066
.L054888:
        jsr     0x28758.l                       | +06c
        bcc.w   .L0548b6                        | +072
        bclr    #0x0,0x13(a6)                   | +076
        lea     0x29a350.l,a1                   | +07c
        jsr     0x77c7e.l                       | +082
        lea     0x29a43a.l,a1                   | +088
        jsr     0x77c7e.l                       | +08e
        lea     TaskHandler_0548c8(pc),a1       | +094
        move.l  a1,(a6)                         | +098
.L0548b6:
        jsr     0x4fa70.l                       | +09a
        bcc.w   .L0548c6                        | +0a0
        jmp     0x518.l                         | +0a4
.L0548c6:
        rts                                     | +0aa

| ----------------------------------------------------------------------------
|  TaskHandler_0548c8  @ $0548C8  (68 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0548c8, "ax", @progbits
        .global TaskHandler_0548c8
TaskHandler_0548c8:
        move.w  #0x1054,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x298b4a.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L0548e4(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L0548e4:
        jsr     0x2783a.l                       | +01c
        jsr     0x28d70.l                       | +022
        bcc.w   .L0548fa                        | +028
        lea     TaskHandler_05490c(pc),a1       | +02c
        move.l  a1,(a6)                         | +030
.L0548fa:
        jsr     0x4fa70.l                       | +032
        bcc.w   .L05490a                        | +038
        jmp     0x518.l                         | +03c
.L05490a:
        rts                                     | +042

| ----------------------------------------------------------------------------
|  TaskHandler_05490c  @ $05490C  (78 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_05490c, "ax", @progbits
        .global TaskHandler_05490c
TaskHandler_05490c:
        jsr     0x2783a.l                       | +000
        move.l  #0x1000,d0                      | +006
        jsr     0x51a28.l                       | +00c
        lea     0x29b380.l,a1                   | +012
        jsr     0x43fac.l                       | +018
        lea     0x298b78.l,a0                   | +01e
        jsr     0x28cd4.l                       | +024
        lea     .L05493c(pc),a1                 | +02a
        move.l  a1,(a6)                         | +02e
.L05493c:
        jsr     0x2783a.l                       | +030
        jsr     0x28d70.l                       | +036
        jsr     0x4fa70.l                       | +03c
        bcc.w   .L054958                        | +042
        jmp     0x518.l                         | +046
.L054958:
        rts                                     | +04c

| ----------------------------------------------------------------------------
|  TaskHandler_05495a  @ $05495A  (192 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_05495a, "ax", @progbits
        .global TaskHandler_05495a
TaskHandler_05495a:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x93,d1                        | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0x50,0x70(a6)                  | +014
        move.b  #0xff,0x32(a6)                  | +01a
        move.b  #0xff,0x33(a6)                  | +020
        move.b  #0x0,0x3a(a6)                   | +026
        move.w  #0x1,0x66(a6)                   | +02c
        lea     0x298b88.l,a0                   | +032
        jsr     0x28cd4.l                       | +038
        lea     .L05499e(pc),a1                 | +03e
        move.l  a1,(a6)                         | +042
.L05499e:
        jsr     0x2783a.l                       | +044
        jsr     0x28d70.l                       | +04a
        jsr     0x2870a.l                       | +050
        bcc.w   .L0549ba                        | +056
        bclr    #0x3,0x13(a6)                   | +05a
.L0549ba:
        jsr     0x28758.l                       | +060
        bcc.w   .L054a08                        | +066
        bclr    #0x0,0x13(a6)                   | +06a
        move.w  #0x102e,d0                      | +070
        jsr     0x2352.l                        | +074
        lea     0x29b358.l,a1                   | +07a
        jsr     0x43fac.l                       | +080
        lea     0x29a362.l,a1                   | +086
        jsr     0x77c7e.l                       | +08c
        lea     0x29a374.l,a1                   | +092
        jsr     0x77c7e.l                       | +098
        lea     0x29a44c.l,a1                   | +09e
        jsr     0x77c7e.l                       | +0a4
        bra.w   .L054a12                        | +0aa
.L054a08:
        jsr     0x4fa70.l                       | +0ae
        bcc.w   .L054a18                        | +0b4
.L054a12:
        jmp     0x518.l                         | +0b8
.L054a18:
        rts                                     | +0be

| ----------------------------------------------------------------------------
|  TaskHandler_054a1a  @ $054A1A  (258 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_054a1a, "ax", @progbits
        .global TaskHandler_054a1a
TaskHandler_054a1a:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x7a,d1                        | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0x40,0x70(a6)                  | +014
        move.b  #0xff,0x32(a6)                  | +01a
        move.b  #0xff,0x33(a6)                  | +020
        move.b  #0x0,0x3a(a6)                   | +026
        move.w  #0x46,0x66(a6)                  | +02c
        move.b  #0x6,0x44(a6)                   | +032
        lea     0x298b9e.l,a0                   | +038
        jsr     0x28cd4.l                       | +03e
        lea     .L054a64(pc),a1                 | +044
        move.l  a1,(a6)                         | +048
.L054a64:
        jsr     0x2783a.l                       | +04a
        jsr     0x28d70.l                       | +050
        bcc.w   .L054a80                        | +056
        lea     0x298b9e.l,a0                   | +05a
        jsr     0x28cd4.l                       | +060
.L054a80:
        jsr     0x2870a.l                       | +066
        bcc.w   .L054abc                        | +06c
        lea     0x5e766.l,a0                    | +070
        jsr     0x5e770.l                       | +076
        bclr    #0x3,0x13(a6)                   | +07c
        move.w  0x72(a6),d0                     | +082
        movea.l #0x298c46,a0                    | +086
        lsl.w   #0x2,d0                         | +08c
        movea.l (a0,d0.w),a0                    | +08e
        cmpa.l  #0xffffffff,a0                  | +092
        beq.w   .L054abc                        | +098
        jsr     0x28cd4.l                       | +09c
.L054abc:
        jsr     0x28758.l                       | +0a2
        bcc.w   .L054b0a                        | +0a8
        move.l  #0x300,d0                       | +0ac
        jsr     0x51a28.l                       | +0b2
        bclr    #0x0,0x13(a6)                   | +0b8
        move.w  #0x1023,d0                      | +0be
        jsr     0x2352.l                        | +0c2
        lea     0x29a386.l,a1                   | +0c8
        jsr     0x77c7e.l                       | +0ce
        lea     0x29a398.l,a1                   | +0d4
        jsr     0x77c7e.l                       | +0da
        lea     0x29a45e.l,a1                   | +0e0
        jsr     0x77c7e.l                       | +0e6
        bra.w   .L054b14                        | +0ec
.L054b0a:
        jsr     0x4fa70.l                       | +0f0
        bcc.w   .L054b1a                        | +0f6
.L054b14:
        jmp     0x518.l                         | +0fa
.L054b1a:
        rts                                     | +100

| ----------------------------------------------------------------------------
|  TaskHandler_054b1c  @ $054B1C  (194 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_054b1c, "ax", @progbits
        .global TaskHandler_054b1c
TaskHandler_054b1c:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x94,d1                        | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0x50,0x70(a6)                  | +014
        move.b  #0xff,0x32(a6)                  | +01a
        move.b  #0xff,0x33(a6)                  | +020
        move.b  #0x0,0x3a(a6)                   | +026
        move.w  #0x28,0x66(a6)                  | +02c
        lea     0x299050.l,a0                   | +032
        jsr     0x28cd4.l                       | +038
        lea     .L054b60(pc),a1                 | +03e
        move.l  a1,(a6)                         | +042
.L054b60:
        jsr     0x2783a.l                       | +044
        jsr     0x28d70.l                       | +04a
        jsr     0x2870a.l                       | +050
        bcc.w   .L054b88                        | +056
        lea     0x5e766.l,a0                    | +05a
        jsr     0x5e770.l                       | +060
        bclr    #0x3,0x13(a6)                   | +066
.L054b88:
        jsr     0x28758.l                       | +06c
        bcc.w   .L054bcc                        | +072
        bclr    #0x0,0x13(a6)                   | +076
        move.w  #0x102e,d0                      | +07c
        jsr     0x2352.l                        | +080
        lea     0x29a3aa.l,a1                   | +086
        jsr     0x77c7e.l                       | +08c
        lea     0x29a3bc.l,a1                   | +092
        jsr     0x77c7e.l                       | +098
        lea     0x29a470.l,a1                   | +09e
        jsr     0x77c7e.l                       | +0a4
        lea     TaskHandler_054bde(pc),a1       | +0aa
        move.l  a1,(a6)                         | +0ae
.L054bcc:
        jsr     0x4fa70.l                       | +0b0
        bcc.w   .L054bdc                        | +0b6
        jmp     0x518.l                         | +0ba
.L054bdc:
        rts                                     | +0c0

| ----------------------------------------------------------------------------
|  TaskHandler_054bde  @ $054BDE  (48 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_054bde, "ax", @progbits
        .global TaskHandler_054bde
TaskHandler_054bde:
        lea     0x299066.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L054bf0(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L054bf0:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        jsr     0x4fa70.l                       | +01e
        bcc.w   .L054c0c                        | +024
        jmp     0x518.l                         | +028
.L054c0c:
        rts                                     | +02e

| ----------------------------------------------------------------------------
|  TaskHandler_054c0e  @ $054C0E  (228 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_054c0e, "ax", @progbits
        .global TaskHandler_054c0e
TaskHandler_054c0e:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        jsr     0x2783a.l                       | +00a
        lea     0x29b31a.l,a1                   | +010
        jsr     0x43fac.l                       | +016
        move.w  #0x60,0x70(a6)                  | +01c
        lea     0x2c0196.l,a0                   | +022
        jsr     0x799de.l                       | +028
        move.w  d0,0x66(a6)                     | +02e
        lea     0x298a52.l,a0                   | +032
        jsr     0x28cd4.l                       | +038
        lea     .L054c52(pc),a1                 | +03e
        move.l  a1,(a6)                         | +042
.L054c52:
        clr.b   0x10e39a.l                      | +044
        jsr     0x2783a.l                       | +04a
        jsr     0x28d70.l                       | +050
        jsr     0x2870a.l                       | +056
        bcc.w   .L054c80                        | +05c
        lea     0x5e766.l,a0                    | +060
        jsr     0x5e770.l                       | +066
        bclr    #0x3,0x13(a6)                   | +06c
.L054c80:
        jsr     0x28758.l                       | +072
        bcc.w   .L054ce0                        | +078
        move.l  #0x1000,d0                      | +07c
        jsr     0x51a28.l                       | +082
        move.w  #0x102b,d0                      | +088
        jsr     0x2352.l                        | +08c
        lea     0x29b32e.l,a1                   | +092
        jsr     0x43fac.l                       | +098
        lea     0x29a606.l,a2                   | +09e
        jsr     0x5022a.l                       | +0a4
        lea     0x29a308.l,a1                   | +0aa
        jsr     0x77c7e.l                       | +0b0
        lea     0x29a308.l,a1                   | +0b6
        jsr     0x77c7e.l                       | +0bc
        lea     0x29a416.l,a1                   | +0c2
        jsr     0x77c7e.l                       | +0c8
        bra.w   .L054cea                        | +0ce
.L054ce0:
        jsr     0x4fa70.l                       | +0d2
        bcc.w   .L054cf0                        | +0d8
.L054cea:
        jmp     0x518.l                         | +0dc
.L054cf0:
        rts                                     | +0e2

| ----------------------------------------------------------------------------
|  TaskHandler_054cf2  @ $054CF2  (382 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_054cf2, "ax", @progbits
        .global TaskHandler_054cf2
TaskHandler_054cf2:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x0,0x72(a6)                   | +00a
        lea     0x2990b8.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        bra.w   .L054d8e                        | +01c
        movea.l 0x3c(a6),a1                     | +020
        jsr     0x2942a.l                       | +024
        move.w  #0x1,0x72(a6)                   | +02a
        lea     0x2990b8.l,a0                   | +030
        jsr     0x28cd4.l                       | +036
        bra.w   .L054d8e                        | +03c
        movea.l 0x3c(a6),a1                     | +040
        jsr     0x2942a.l                       | +044
        move.w  #0x2,0x72(a6)                   | +04a
        lea     0x2990b8.l,a0                   | +050
        jsr     0x28cd4.l                       | +056
        bra.w   .L054d8e                        | +05c
        movea.l 0x3c(a6),a1                     | +060
        jsr     0x2942a.l                       | +064
        move.w  #0x3,0x72(a6)                   | +06a
        lea     0x2990b8.l,a0                   | +070
        jsr     0x28cd4.l                       | +076
        bra.w   .L054d8e                        | +07c
        movea.l 0x3c(a6),a1                     | +080
        jsr     0x2942a.l                       | +084
        move.w  #0x4,0x72(a6)                   | +08a
        lea     0x299d6e.l,a0                   | +090
        jsr     0x28cd4.l                       | +096
.L054d8e:
        move.w  #0x95,d1                        | +09c
        jsr     0x236e.l                        | +0a0
        cmpi.w  #0x4,0x72(a6)                   | +0a6
        beq.w   .L054dc2                        | +0ac
        lea     0x6293e.l,a1                    | +0b0
        jsr     0x4ae.l                         | +0b6
        jsr     0x5dd22.l                       | +0bc
        move.w  0x72(a6),d0                     | +0c2
        move.b  d0,0x62(a0)                     | +0c6
        addi.w  #0x40,0x22(a0)                  | +0ca
.L054dc2:
        move.w  #0x80,0x70(a6)                  | +0d0
        move.b  #0xff,0x32(a6)                  | +0d6
        move.b  #0xff,0x33(a6)                  | +0dc
        move.b  #0x0,0x3a(a6)                   | +0e2
        lea     0x2c0114.l,a0                   | +0e8
        jsr     0x799de.l                       | +0ee
        move.w  d0,0x66(a6)                     | +0f4
        addi.w  #0x140,0x66(a6)                 | +0f8
        move.b  #0x0,0x20(a6)                   | +0fe
        move.w  0x72(a6),d0                     | +104
        cmpi.w  #0x4,0x72(a6)                   | +108
        bne.w   .L054e0c                        | +10e
        lea     0xffff.w,a0                     | +112
        move.l  a0,0x48(a6)                     | +116
.L054e0c:
        lea     .L054e12(pc),a1                 | +11a
        move.l  a1,(a6)                         | +11e
.L054e12:
        jsr     0x2783a.l                       | +120
        jsr     0x28d70.l                       | +126
        cmpi.w  #0x4,0x72(a6)                   | +12c
        beq.w   .L054e5e                        | +132
        jsr     0x2870a.l                       | +136
        bcc.w   .L054e44                        | +13c
        lea     0x5e766.l,a0                    | +140
        jsr     0x5e770.l                       | +146
        bclr    #0x3,0x13(a6)                   | +14c
.L054e44:
        cmpi.w  #0xd4,0x66(a6)                  | +152
        bgt.w   .L054e5e                        | +158
        move.w  #0x1027,d0                      | +15c
        jsr     0x2352.l                        | +160
        lea     TaskHandler_054e70(pc),a1       | +166
        move.l  a1,(a6)                         | +16a
.L054e5e:
        jsr     0x4fa70.l                       | +16c
        bcc.w   .L054e6e                        | +172
        jmp     0x518.l                         | +176
.L054e6e:
        rts                                     | +17c

| ----------------------------------------------------------------------------
|  TaskHandler_054e70  @ $054E70  (122 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_054e70, "ax", @progbits
        .global TaskHandler_054e70
TaskHandler_054e70:
        move.w  0x72(a6),d0                     | +000
        movea.l #0x2990ce,a0                    | +004
        lsl.w   #0x2,d0                         | +00a
        movea.l (a0,d0.w),a0                    | +00c
        cmpa.l  #0xffffffff,a0                  | +010
        beq.w   .L054e90                        | +016
        jsr     0x28cd4.l                       | +01a
.L054e90:
        lea     .L054e96(pc),a1                 | +020
        move.l  a1,(a6)                         | +024
.L054e96:
        jsr     0x2783a.l                       | +026
        jsr     0x28d70.l                       | +02c
        jsr     0x2870a.l                       | +032
        bcc.w   .L054ebe                        | +038
        lea     0x5e766.l,a0                    | +03c
        jsr     0x5e770.l                       | +042
        bclr    #0x3,0x13(a6)                   | +048
.L054ebe:
        cmpi.w  #0x6a,0x66(a6)                  | +04e
        bgt.w   .L054ed8                        | +054
        move.w  #0x1027,d0                      | +058
        jsr     0x2352.l                        | +05c
        lea     TaskHandler_054eea(pc),a1       | +062
        move.l  a1,(a6)                         | +066
.L054ed8:
        jsr     0x4fa70.l                       | +068
        bcc.w   .L054ee8                        | +06e
        jmp     0x518.l                         | +072
.L054ee8:
        rts                                     | +078

| ----------------------------------------------------------------------------
|  TaskHandler_054eea  @ $054EEA  (128 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_054eea, "ax", @progbits
        .global TaskHandler_054eea
TaskHandler_054eea:
        move.w  0x72(a6),d0                     | +000
        movea.l #0x2990de,a0                    | +004
        lsl.w   #0x2,d0                         | +00a
        movea.l (a0,d0.w),a0                    | +00c
        cmpa.l  #0xffffffff,a0                  | +010
        beq.w   .L054f0a                        | +016
        jsr     0x28cd4.l                       | +01a
.L054f0a:
        lea     .L054f10(pc),a1                 | +020
        move.l  a1,(a6)                         | +024
.L054f10:
        jsr     0x2783a.l                       | +026
        jsr     0x28d70.l                       | +02c
        jsr     0x2870a.l                       | +032
        bcc.w   .L054f38                        | +038
        lea     0x5e766.l,a0                    | +03c
        jsr     0x5e770.l                       | +042
        bclr    #0x3,0x13(a6)                   | +048
.L054f38:
        jsr     0x28758.l                       | +04e
        bcc.w   .L054f58                        | +054
        bclr    #0x0,0x13(a6)                   | +058
        move.w  #0x1035,d0                      | +05e
        jsr     0x2352.l                        | +062
        lea     TaskHandler_054f6a(pc),a1       | +068
        move.l  a1,(a6)                         | +06c
.L054f58:
        jsr     0x4fa70.l                       | +06e
        bcc.w   .L054f68                        | +074
        jmp     0x518.l                         | +078
.L054f68:
        rts                                     | +07e

| ----------------------------------------------------------------------------
|  TaskHandler_054f6a  @ $054F6A  (80 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_054f6a, "ax", @progbits
        .global TaskHandler_054f6a
TaskHandler_054f6a:
        move.l  #0x4000,d0                      | +000
        jsr     0x51a28.l                       | +006
        move.w  0x72(a6),d0                     | +00c
        movea.l #0x2990ee,a0                    | +010
        lsl.w   #0x2,d0                         | +016
        movea.l (a0,d0.w),a0                    | +018
        cmpa.l  #0xffffffff,a0                  | +01c
        beq.w   .L054f96                        | +022
        jsr     0x28cd4.l                       | +026
.L054f96:
        lea     .L054f9c(pc),a1                 | +02c
        move.l  a1,(a6)                         | +030
.L054f9c:
        jsr     0x2783a.l                       | +032
        jsr     0x28d70.l                       | +038
        jsr     0x4fa70.l                       | +03e
        bcc.w   .L054fb8                        | +044
        jmp     0x518.l                         | +048
.L054fb8:
        rts                                     | +04e

| ----------------------------------------------------------------------------
|  TaskHandler_054fba  @ $054FBA  (72 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_054fba, "ax", @progbits
        .global TaskHandler_054fba
TaskHandler_054fba:
        move.w  #0x48,d1                        | +000
        move.w  #0x79,d2                        | +004
        move.w  #0x2,d3                         | +008
        move.w  #0x1,d4                         | +00c
        jsr     0x2c26.l                        | +010
        move.w  #0x40,0x72(a6)                  | +016
        lea     .L054fdc(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L054fdc:
        subq.w  #0x1,0x72(a6)                   | +022
        bne.w   .L054fea                        | +026
        lea     TaskHandler_055002(pc),a1       | +02a
        move.l  a1,(a6)                         | +02e
.L054fea:
        move.l  0x106f5c.l,d0                   | +030
        swap    d0                              | +036
        cmpi.w  #0x300,d0                       | +038
        blt.w   .L055000                        | +03c
        jmp     0x518.l                         | +040
.L055000:
        rts                                     | +046

| ----------------------------------------------------------------------------
|  TaskHandler_055002  @ $055002  (72 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_055002, "ax", @progbits
        .global TaskHandler_055002
TaskHandler_055002:
        move.w  #0x48,d1                        | +000
        move.w  #0x78,d2                        | +004
        move.w  #0x1,d3                         | +008
        move.w  #0x1,d4                         | +00c
        jsr     0x2c26.l                        | +010
        move.w  #0x20,0x72(a6)                  | +016
        lea     .L055024(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L055024:
        subq.w  #0x1,0x72(a6)                   | +022
        bne.w   .L055032                        | +026
        lea     TaskHandler_054fba(pc),a1       | +02a
        move.l  a1,(a6)                         | +02e
.L055032:
        move.l  0x106f5c.l,d0                   | +030
        swap    d0                              | +036
        cmpi.w  #0x300,d0                       | +038
        blt.w   .L055048                        | +03c
        jmp     0x518.l                         | +040
.L055048:
        rts                                     | +046

| ----------------------------------------------------------------------------
|  TaskHandler_05504a  @ $05504A  (82 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_05504a, "ax", @progbits
        .global TaskHandler_05504a
TaskHandler_05504a:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0xe,d1                         | +00a
        jsr     0x236e.l                        | +00e
        bset    #0x6,0x12(a6)                   | +014
        move.w  #0xf000,0x38(a6)                | +01a
        move.w  #0x60,0x70(a6)                  | +020
        move.l  #0x29b3c0,0x60(a6)              | +026
        lea     .L05507e(pc),a1                 | +02e
        move.l  a1,(a6)                         | +032
.L05507e:
        jsr     0x28998.l                       | +034
        jsr     0x2783a.l                       | +03a
        jsr     0x4fa70.l                       | +040
        bcc.w   .L05509a                        | +046
        jmp     0x518.l                         | +04a
.L05509a:
        rts                                     | +050

| ----------------------------------------------------------------------------
|  Sub_0005509C  @ $05509C  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0005509C, "ax", @progbits
        .global Sub_0005509C
Sub_0005509C:
        cmpi.b  #0x0,0x74(a6)                   | +000
        bne.w   JsrPcRts_055146                 | +006
        move.b  0x1081b1.l,d0                   | +00a
        cmpi.b  #0x0,d0                         | +010
        beq.w   TaskHandler_0550c4              | +014
        jsr     TaskHandler_0551d0(pc)          | +018
        move.b  0x1081b1.l,d0                   | +01c

| ----------------------------------------------------------------------------
|  TaskHandler_0550c4  @ $0550C4  (68 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0550c4, "ax", @progbits
        .global TaskHandler_0550c4
TaskHandler_0550c4:
        movea.l 0xc(a6),a0                      | +000
        cmpi.b  #0x7f,0x80(a0)                  | +004
        beq.w   .L0550e6                        | +00a
        cmpi.b  #0xff,0x80(a0)                  | +00e
        bne.w   .L0550e8                        | +014
        jsr     TaskHandler_0551d0(pc)          | +018
        move.b  #0x7f,0x80(a0)                  | +01c
.L0550e6:
        rts                                     | +022
.L0550e8:
        movea.l 0xc(a6),a0                      | +024
        cmpi.b  #0xff,0x21(a0)                  | +028
        beq.w   JsrPcRts_055146                 | +02e
        addq.w  #0x1,0x72(a6)                   | +032
        movea.l 0xc(a6),a0                      | +036
        cmpi.b  #0x1,0x21(a0)                   | +03a
        bne.w   TaskHandler_05510e              | +040

| ----------------------------------------------------------------------------
|  TaskHandler_05510e  @ $05510E  (14 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_05510e, "ax", @progbits
        .global TaskHandler_05510e
TaskHandler_05510e:
        movea.l 0xc(a6),a0                      | +000
        cmpi.b  #0x2,0x21(a0)                   | +004
        bne.w   TaskHandler_055122              | +00a

| ----------------------------------------------------------------------------
|  TaskHandler_055122  @ $055122  (32 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_055122, "ax", @progbits
        .global TaskHandler_055122
TaskHandler_055122:
        move.w  0x72(a6),d0                     | +000
        andi.w  #0x3,d0                         | +004
        bne.w   JsrPcRts_055146                 | +008
        move.w  0x72(a6),d0                     | +00c
        btst    #0x2,d0                         | +010
        bne.w   JsrPcThunk_055142               | +014
        jsr     TaskHandler_05518c(pc)          | +018
        bra.w   JsrPcRts_055146                 | +01c

| ----------------------------------------------------------------------------
|  PcThunkTarget_055148  @ $055148  (60 B)
| ----------------------------------------------------------------------------
        .section .text.PcThunkTarget_055148, "ax", @progbits
        .global PcThunkTarget_055148
PcThunkTarget_055148:
        move.w  #0x1a,d1                        | +000
        move.w  #0x4b,d2                        | +004
        move.w  #0xffff,d3                      | +008
        move.w  #0x1,d4                         | +00c
        jsr     0x2c26.l                        | +010
        move.w  #0x1d,d1                        | +016
        move.w  #0x4e,d2                        | +01a
        move.w  #0xffff,d3                      | +01e
        move.w  #0x1,d4                         | +022
        jsr     0x2c26.l                        | +026
        move.w  0x14(a6),d1                     | +02c
        move.w  #0x8e,d2                        | +030
        move.w  #0xffff,d3                      | +034
        move.w  #0x1,d4                         | +038

| ----------------------------------------------------------------------------
|  TaskHandler_05518c  @ $05518C  (60 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_05518c, "ax", @progbits
        .global TaskHandler_05518c
TaskHandler_05518c:
        move.w  #0x1a,d1                        | +000
        move.w  #0x4a,d2                        | +004
        move.w  #0xffff,d3                      | +008
        move.w  #0x1,d4                         | +00c
        jsr     0x2c26.l                        | +010
        move.w  #0x1d,d1                        | +016
        move.w  #0x4d,d2                        | +01a
        move.w  #0xffff,d3                      | +01e
        move.w  #0x1,d4                         | +022
        jsr     0x2c26.l                        | +026
        move.w  0x14(a6),d1                     | +02c
        move.w  #0x8d,d2                        | +030
        move.w  #0xffff,d3                      | +034
        move.w  #0x1,d4                         | +038

| ----------------------------------------------------------------------------
|  TaskHandler_0551d0  @ $0551D0  (60 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0551d0, "ax", @progbits
        .global TaskHandler_0551d0
TaskHandler_0551d0:
        move.w  #0x1a,d1                        | +000
        move.w  #0xbd,d2                        | +004
        move.w  #0x1,d3                         | +008
        move.w  #0x2,d4                         | +00c
        jsr     0x2c26.l                        | +010
        move.w  #0x1d,d1                        | +016
        move.w  #0xbe,d2                        | +01a
        move.w  #0x1,d3                         | +01e
        move.w  #0x2,d4                         | +022
        jsr     0x2c26.l                        | +026
        move.w  0x14(a6),d1                     | +02c
        move.w  #0x123,d2                       | +030
        move.w  #0x1,d3                         | +034
        move.w  #0x2,d4                         | +038

| ----------------------------------------------------------------------------
|  PcThunkTarget_055214  @ $055214  (60 B)
| ----------------------------------------------------------------------------
        .section .text.PcThunkTarget_055214, "ax", @progbits
        .global PcThunkTarget_055214
PcThunkTarget_055214:
        move.w  #0x1a,d1                        | +000
        move.w  #0x4c,d2                        | +004
        move.w  #0x1,d3                         | +008
        move.w  #0x1,d4                         | +00c
        jsr     0x2c26.l                        | +010
        move.w  #0x1d,d1                        | +016
        move.w  #0x4f,d2                        | +01a
        move.w  #0x1,d3                         | +01e
        move.w  #0x1,d4                         | +022
        jsr     0x2c26.l                        | +026
        move.w  0x14(a6),d1                     | +02c
        move.w  #0x8f,d2                        | +030
        move.w  #0x1,d3                         | +034
        move.w  #0x1,d4                         | +038
