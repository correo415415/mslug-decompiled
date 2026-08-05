| =============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave HHH — Fases finales del miniboss y transiciones de oleada
|  Región: $083BE2..$084828  (3,110 B, 28 entradas, 5 huecos cerrados)
| =============================================================================
|
|  Continuación directa del módulo de miniboss del Wave GGG: fases de
|  ataque/muerte del jefe, contador de fase global $10E39A/$10E39C y
|  spawner de refuerzos. Estructura:
|
|   * $83BE2..$83D6E — entradas dobles con jingle: snd $166/$AA convergen
|     en $83C0C (sprites $2E6EFC/$2E6F12, gate x>$140, muerte con snd
|     $10A6 y par $2E9A0C); snd $A6 + sprite $2E6F28 con snd $102E que
|     transiciona a TaskHandler_083d2a (par $2E9A94, sprite $2E6F3E).
|   * $83D6E..$83F26 — variantes según +$21 (sprites $2E6F54/$2E6F6A)
|     que convergen en la fase de disparo (snd $1027, helpers
|     Sub_000863E4/Sub_000863F2, par $2E9AA6); entrada con snd $89
|     (sprite $2E6F80, probe $27AFC) y bucle de premio: score $2000
|     ($51A28), pares $2E9890/$2E9A82, sprite $2E6FBA, velocidad
|     $280/-$40 (TaskHandler_083e8c, global interno, se reinstala
|     cruzando entradas desde $83F0E).
|   * $83F26..$8414C — fase temporizada $F0 con helper Sub_00086328 y
|     salida via scheduler; gate x<=$140 con sprite $2E6FCA, blitter
|     $2EACB8 + snd $1030, score $500, bset del índice en el padre y
|     máquina bit-scan sobre la tabla $2EAA60[+$21<<4] (2x StateMachineRun,
|     helpers Sub_000864B6/Sub_000864D0) que termina cuando +$21==$11.
|   * $8414C..$842D0 — cadena de sprites del desenlace ($2E6FE0/$2E6FF6/
|     $2E7006/$2E701C) con helpers Sub_00086400/Sub_00086364/Sub_000864EA.
|   * $842D0..$84408 — explosión final: sprite $2E7032, score $1000, snd
|     $1030, helpers Sub_0008640E/Sub_0008641C, pares $2E9B00/$2E9ACA,
|     blitters $2EACB8/$2EAE76; epílogo con snd $A9, helpers
|     Sub_000860E4/Sub_00086300, spawn del hijo $8495E (hueco futuro,
|     via $4AE + $5DD22), sprite $2E7048 y limpieza de $10E39A.
|   * $84410..$84504 — transición de oleada: $10E39C=3, instala
|     TaskHandler_0845b8 (referencia cruzada entre huecos), snd $1033,
|     $2C26 (d1=$27,d2=$129), spawn absoluto $86586 (hueco futuro),
|     helpers Sub_00086196/Sub_0008610C, pares $2E98D8/$2E98EA y
|     contador +$21>=9 -> $10E39A=2; variante con blitter $2EACD4 y
|     sprite $2E7486 (probe $6F0).
|   * $8450C..$845E4 — cierre de fase: clr $10E39C + $320D4 + scheduler;
|     guardián con +$70=$100, +$48=$2E9256, +$66=$7FFF, probe de
|     $106F28 bit0 y +$58==1/2, lista $53886 via $4AE; parpadeo de
|     $10A2D1 con contador +$72 (cae en la isla JmpToScheduler_0845e4).
|   * $845EC..$84828 — sincronización padre/hijo: tres etapas que esperan
|     bit3 de +$13 del padre y comparan +$20/+$21 (helpers Sub_0008651E/
|     Sub_00086538, score $1000, snd $108D); relanzamiento aleatorio con
|     snd $A9, offsets +$80/+$29 (+$40 por índice), tabla $2E7556[+$21<<2]
|     y selector aleatorio $2EB02C[rnd&$F<<2] via $4AE + $5DD22; fase
|     final con gate x>$150, snd $102E, despawn ($FFFF + par $2E9ADC)
|     y bcc.w colgantes hacia Jsr5B6Rts_084834 (+12 de la isla
|     Jsr5B6ThenJmpScheduler_084828).
|
|  Los bcc/blt/bcs.w colgantes apuntan a los RTS internos de las islas C
|  contiguas (SetHandlerRts_08440e/_0844be/_08450a en +6, Jsr5B6Rts_084834
|  en +12); los saltos a $845E4 usan el símbolo real JmpToScheduler_0845e4.
|
|  Verificación: cada sección .text.<Sym> se coloca en su dirección CPU
|  absoluta y reensambla byte-exacta contra build/mslug_prom.bin
|  (MD5 816b3f74c76b3373993407615f1850fe).
| =============================================================================

        .text

| ----------------------------------------------------------------------------
|  TaskHandler_083be2  @ $083BE2  (176 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_083be2, "ax", @progbits
        .global TaskHandler_083be2
TaskHandler_083be2:
        move.w  #0x166,d1                       | +000
        jsr     0x236e.l                        | +004
        move.w  #0xc000,0x38(a6)                | +00a
        move.b  0x98(a6),0x21(a6)               | +010
        move.b  0x99(a6),0x3a(a6)               | +016
        bra.w   .L83c0c                         | +01c
        move.w  #0xaa,d1                        | +020
        jsr     0x236e.l                        | +024
.L83c0c:
        move.b  #0xff,0x32(a6)                  | +02a
        move.b  #0xff,0x33(a6)                  | +030
        move.w  #0x20,0x70(a6)                  | +036
        lea     0x2e6efc.l,a0                   | +03c
        jsr     0x28cd4.l                       | +042
        btst    #0x0,0x21(a6)                   | +048
        beq.w   .L83c40                         | +04e
        lea     0x2e6f12.l,a0                   | +052
        jsr     0x28cd4.l                       | +058
.L83c40:
        lea     .L83c46(pc),a1                  | +05e
        move.l  a1,(a6)                         | +062
.L83c46:
        jsr     0x2783a.l                       | +064
        cmpi.w  #0x140,0x22(a6)                 | +06a
        bgt.w   .L83c5c                         | +070
        jsr     0x28d70.l                       | +074
.L83c5c:
        jsr     0x2870a.l                       | +07a
        bcc.w   .L83c80                         | +080
        move.w  #0x10a6,d0                      | +084
        jsr     0x2352.l                        | +088
        lea     0x2e9a0c.l,a1                   | +08e
        jsr     0x77c7e.l                       | +094
        bra.w   .L83c8a                         | +09a
.L83c80:
        jsr     0x4fa70.l                       | +09e
        bcc.w   .L83c90                         | +0a4
.L83c8a:
        jmp     0x518.l                         | +0a8
.L83c90:
        rts                                     | +0ae

| ----------------------------------------------------------------------------
|  TaskHandler_083c92  @ $083C92  (152 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_083c92, "ax", @progbits
        .global TaskHandler_083c92
TaskHandler_083c92:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0xa6,d1                        | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0x30,0x70(a6)                  | +014
        move.b  #0xff,0x32(a6)                  | +01a
        move.b  #0xff,0x33(a6)                  | +020
        move.b  #0x0,0x3a(a6)                   | +026
        move.w  #0x14,0x66(a6)                  | +02c
        lea     0x2e6f28.l,a0                   | +032
        jsr     0x28cd4.l                       | +038
        lea     .L83cd6(pc),a1                  | +03e
        move.l  a1,(a6)                         | +042
.L83cd6:
        jsr     0x2783a.l                       | +044
        jsr     0x28d70.l                       | +04a
        jsr     0x2870a.l                       | +050
        bcc.w   .L83cfe                         | +056
        lea     0x5e766.l,a0                    | +05a
        jsr     0x5e770.l                       | +060
        bclr    #0x3,0x13(a6)                   | +066
.L83cfe:
        jsr     0x28758.l                       | +06c
        bcc.w   .L83d18                         | +072
        move.w  #0x102e,d0                      | +076
        jsr     0x2352.l                        | +07a
        lea     TaskHandler_083d2a(pc),a1       | +080
        move.l  a1,(a6)                         | +084
.L83d18:
        jsr     0x4fa70.l                       | +086
        bcc.w   .L83d28                         | +08c
        jmp     0x518.l                         | +090
.L83d28:
        rts                                     | +096

| ----------------------------------------------------------------------------
|  TaskHandler_083d2a  @ $083D2A  (68 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_083d2a, "ax", @progbits
        .global TaskHandler_083d2a
TaskHandler_083d2a:
        lea     0xffff.w,a0                     | +000
        move.l  a0,0x48(a6)                     | +004
        lea     0x2e9a94.l,a1                   | +008
        jsr     0x77c7e.l                       | +00e
        lea     0x2e6f3e.l,a0                   | +014
        jsr     0x28cd4.l                       | +01a
        lea     .L83d50(pc),a1                  | +020
        move.l  a1,(a6)                         | +024
.L83d50:
        jsr     0x2783a.l                       | +026
        jsr     0x28d70.l                       | +02c
        jsr     0x4fa70.l                       | +032
        bcc.w   .L83d6c                         | +038
        jmp     0x518.l                         | +03c
.L83d6c:
        rts                                     | +042

| ----------------------------------------------------------------------------
|  TaskHandler_083d6e  @ $083D6E  (218 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_083d6e, "ax", @progbits
        .global TaskHandler_083d6e
TaskHandler_083d6e:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        lea     0x2e6f54.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        move.b  #0x0,0x21(a6)                   | +016
        bra.w   .L83daa                         | +01c
        movea.l 0x3c(a6),a1                     | +020
        jsr     0x2942a.l                       | +024
        lea     0x2e6f6a.l,a0                   | +02a
        jsr     0x28cd4.l                       | +030
        move.b  #0x1,0x21(a6)                   | +036
.L83daa:
        move.w  #0x30,0x70(a6)                  | +03c
        move.b  #0x0,0x3a(a6)                   | +042
        move.w  #0x14,0x66(a6)                  | +048
        lea     .L83dc2(pc),a1                  | +04e
        move.l  a1,(a6)                         | +052
.L83dc2:
        jsr     0x2783a.l                       | +054
        jsr     0x28d70.l                       | +05a
        jsr     0x2870a.l                       | +060
        bcc.w   .L83dea                         | +066
        lea     0x5e766.l,a0                    | +06a
        jsr     0x5e770.l                       | +070
        bclr    #0x3,0x13(a6)                   | +076
.L83dea:
        jsr     0x28758.l                       | +07c
        bcc.w   .L83e36                         | +082
        move.w  #0x1027,d0                      | +086
        jsr     0x2352.l                        | +08a
        cmpi.b  #0x0,0x21(a6)                   | +090
        bne.w   .L83e1c                         | +096
        jsr     Sub_000863E4(pc)                | +09a
        lea     0x2e9aa6.l,a1                   | +09e
        jsr     0x77c7e.l                       | +0a4
        bra.w   .L83e40                         | +0aa
.L83e1c:
        jsr     Sub_000863F2(pc)                | +0ae
        subi.w  #0x10,0x24(a6)                  | +0b2
        lea     0x2e9aa6.l,a1                   | +0b8
        jsr     0x77c7e.l                       | +0be
        bra.w   .L83e40                         | +0c4
.L83e36:
        jsr     0x4fa70.l                       | +0c8
        bcc.w   .L83e46                         | +0ce
.L83e40:
        jmp     0x518.l                         | +0d2
.L83e46:
        rts                                     | +0d8

| ----------------------------------------------------------------------------
|  TaskHandler_083e48  @ $083E48  (164 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_083e48, "ax", @progbits
        .global TaskHandler_083e48
TaskHandler_083e48:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x89,d1                        | +00a
        jsr     0x236e.l                        | +00e
        lea     0x2e6f80.l,a0                   | +014
        jsr     0x28cd4.l                       | +01a
        move.w  #0x50,0x70(a6)                  | +020
        move.b  #0x0,0x3a(a6)                   | +026
        move.w  #0x64,0x66(a6)                  | +02c
        move.w  #0x8000,0x38(a6)                | +032
        jsr     0x267e2.l                       | +038
        jsr     0x27afc.l                       | +03e
        .global TaskHandler_083e8c
TaskHandler_083e8c:
        lea     .L83e92(pc),a1                  | +044
        move.l  a1,(a6)                         | +048
.L83e92:
        jsr     0x2783a.l                       | +04a
        jsr     0x28d70.l                       | +050
        jsr     0x2870a.l                       | +056
        bcc.w   .L83ec0                         | +05c
        lea     0x5e766.l,a0                    | +060
        jsr     0x5e770.l                       | +066
        bclr    #0x3,0x13(a6)                   | +06c
        lea     TaskHandler_083eec(pc),a1       | +072
        move.l  a1,(a6)                         | +076
.L83ec0:
        jsr     0x28758.l                       | +078
        bcc.w   .L83eda                         | +07e
        move.w  #0x1023,d0                      | +082
        jsr     0x2352.l                        | +086
        lea     TaskHandler_083f26(pc),a1       | +08c
        move.l  a1,(a6)                         | +090
.L83eda:
        jsr     0x4fa70.l                       | +092
        bcc.w   .L83eea                         | +098
        jmp     0x518.l                         | +09c
.L83eea:
        rts                                     | +0a2

| ----------------------------------------------------------------------------
|  TaskHandler_083eec  @ $083EEC  (58 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_083eec, "ax", @progbits
        .global TaskHandler_083eec
TaskHandler_083eec:
        lea     0x2e6f96.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L83efe(pc),a1                  | +00c
        move.l  a1,(a6)                         | +010
.L83efe:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        bcc.w   .L83f14                         | +01e
        lea     TaskHandler_083e8c(pc),a1       | +022
        move.l  a1,(a6)                         | +026
.L83f14:
        jsr     0x4fa70.l                       | +028
        bcc.w   .L83f24                         | +02e
        jmp     0x518.l                         | +032
.L83f24:
        rts                                     | +038

| ----------------------------------------------------------------------------
|  TaskHandler_083f26  @ $083F26  (130 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_083f26, "ax", @progbits
        .global TaskHandler_083f26
TaskHandler_083f26:
        move.l  #0x2000,d0                      | +000
        jsr     0x51a28.l                       | +006
        lea     0xffff.w,a0                     | +00c
        move.l  a0,0x48(a6)                     | +010
        lea     0x2e9890.l,a1                   | +014
        jsr     0x77c7e.l                       | +01a
        lea     0x2e9a82.l,a1                   | +020
        jsr     0x77c7e.l                       | +026
        lea     0x2e6fba.l,a0                   | +02c
        jsr     0x28cd4.l                       | +032
        jsr     0x267e2.l                       | +038
        move.w  #0x280,0x2a(a6)                 | +03e
        move.w  #0xffc0,0x2e(a6)                | +044
        lea     .L83f76(pc),a1                  | +04a
        move.l  a1,(a6)                         | +04e
.L83f76:
        jsr     0x27cee.l                       | +050
        bcc.w   .L83f90                         | +056
        cmpi.w  #0x0,0x2a(a6)                   | +05a
        bgt.w   .L83f90                         | +060
        jsr     0x267e2.l                       | +064
.L83f90:
        jsr     0x28d70.l                       | +06a
        jsr     0x4fa70.l                       | +070
        bcc.w   .L83fa6                         | +076
        jmp     0x518.l                         | +07a
.L83fa6:
        rts                                     | +080

| ----------------------------------------------------------------------------
|  TaskHandler_083fa8  @ $083FA8  (40 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_083fa8, "ax", @progbits
        .global TaskHandler_083fa8
TaskHandler_083fa8:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0xf0,0x70(a6)                  | +00a
        move.b  #0x0,0x21(a6)                   | +010
        jsr     Sub_00086328(pc)                | +016
        lea     .L83fc8(pc),a1                  | +01a
        move.l  a1,(a6)                         | +01e
.L83fc8:
        jmp     0x518.l                         | +020
        rts                                     | +026

| ----------------------------------------------------------------------------
|  TaskHandler_083fd0  @ $083FD0  (18 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_083fd0, "ax", @progbits
        .global TaskHandler_083fd0
TaskHandler_083fd0:
        jsr     0x2783a.l                       | +000
        cmpi.w  #0x140,0x22(a6)                 | +006
        ble.w   TaskHandler_083fe2              | +00c
        rts                                     | +010

| ----------------------------------------------------------------------------
|  TaskHandler_083fe2  @ $083FE2  (126 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_083fe2, "ax", @progbits
        .global TaskHandler_083fe2
TaskHandler_083fe2:
        move.w  #0x20,0x70(a6)                  | +000
        move.w  #0xa,0x66(a6)                   | +006
        lea     0x2e6fca.l,a0                   | +00c
        jsr     0x28cd4.l                       | +012
        lea     .L84000(pc),a1                  | +018
        move.l  a1,(a6)                         | +01c
.L84000:
        jsr     0x2783a.l                       | +01e
        jsr     0x28d70.l                       | +024
        jsr     0x2870a.l                       | +02a
        bcc.w   .L84028                         | +030
        lea     0x5e766.l,a0                    | +034
        jsr     0x5e798.l                       | +03a
        bclr    #0x3,0x13(a6)                   | +040
.L84028:
        jsr     0x28758.l                       | +046
        bcc.w   .L8404e                         | +04c
        lea     0x2eacb8.l,a1                   | +050
        jsr     0x43fac.l                       | +056
        move.w  #0x1030,d0                      | +05c
        jsr     0x2352.l                        | +060
        lea     TaskHandler_084060(pc),a1       | +066
        move.l  a1,(a6)                         | +06a
.L8404e:
        jsr     0x4fa70.l                       | +06c
        bcc.w   .L8405e                         | +072
        jmp     0x518.l                         | +076
.L8405e:
        rts                                     | +07c

| ----------------------------------------------------------------------------
|  TaskHandler_084060  @ $084060  (236 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_084060, "ax", @progbits
        .global TaskHandler_084060
TaskHandler_084060:
        move.l  #0x500,d0                       | +000
        jsr     0x51a28.l                       | +006
        lea     0xffff.w,a0                     | +00c
        move.l  a0,0x48(a6)                     | +010
        move.b  0x21(a6),d0                     | +014
        movea.l 0xc(a6),a0                      | +018
        bset    d0,0x21(a0)                     | +01c
        lea     0x2e9aca.l,a1                   | +020
        jsr     0x77c7e.l                       | +026
        lea     0x2eaa60.l,a0                   | +02c
        clr.l   d0                              | +032
        move.b  0x21(a6),d0                     | +034
        asl.l   #0x4,d0                         | +038
        movea.l (a0,d0.w),a2                    | +03a
        movem.l a0,-(a7)                        | +03e
        movem.l d0,-(a7)                        | +042
        jsr     0x5022a.l                       | +046
        movem.l (a7)+,d0                        | +04c
        movem.l (a7)+,a0                        | +050
        movea.l 0x8(a0,d0.w),a2                 | +054
        jsr     0x5022a.l                       | +058
        move.b  #0x0,0x20(a6)                   | +05e
        lea     .L840ca(pc),a1                  | +064
        move.l  a1,(a6)                         | +068
.L840ca:
        btst    #0x4,0x20(a6)                   | +06a
        bne.w   .L84102                         | +070
        move.b  0x21(a6),d0                     | +074
        subq.b  #0x1,d0                         | +078
        cmpi.b  #0x0,d0                         | +07a
        bge.w   .L840ec                         | +07e
        bset    #0x4,0x20(a6)                   | +082
        bra.w   .L84102                         | +088
.L840ec:
        movea.l 0xc(a6),a0                      | +08c
        btst    d0,0x21(a0)                     | +090
        beq.w   .L84102                         | +094
        jsr     Sub_000864B6(pc)                | +098
        bset    #0x4,0x20(a6)                   | +09c
.L84102:
        btst    #0x0,0x20(a6)                   | +0a2
        bne.w   .L8413a                         | +0a8
        move.b  0x21(a6),d0                     | +0ac
        addq.b  #0x1,d0                         | +0b0
        cmpi.b  #0x7,d0                         | +0b2
        ble.w   .L84124                         | +0b6
        bset    #0x0,0x20(a6)                   | +0ba
        bra.w   .L8413a                         | +0c0
.L84124:
        movea.l 0xc(a6),a0                      | +0c4
        btst    d0,0x21(a0)                     | +0c8
        beq.w   .L8413a                         | +0cc
        jsr     Sub_000864D0(pc)                | +0d0
        bset    #0x0,0x20(a6)                   | +0d4
.L8413a:
        cmpi.b  #0x11,0x21(a6)                  | +0da
        bne.w   .L8414a                         | +0e0
        jmp     0x518.l                         | +0e4
.L8414a:
        rts                                     | +0ea

| ----------------------------------------------------------------------------
|  TaskHandler_08414c  @ $08414C  (114 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08414c, "ax", @progbits
        .global TaskHandler_08414c
TaskHandler_08414c:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0xa4,d1                        | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0x60,0x70(a6)                  | +014
        move.b  #0xff,0x32(a6)                  | +01a
        move.b  #0xff,0x33(a6)                  | +020
        move.b  #0x0,0x3a(a6)                   | +026
        move.w  #0x1,0x66(a6)                   | +02c
        lea     0x2e6fe0.l,a0                   | +032
        jsr     0x28cd4.l                       | +038
        lea     .L84190(pc),a1                  | +03e
        move.l  a1,(a6)                         | +042
.L84190:
        jsr     0x2783a.l                       | +044
        jsr     0x28d70.l                       | +04a
        jsr     0x2870a.l                       | +050
        bcc.w   .L841ac                         | +056
        lea     TaskHandler_0841be(pc),a1       | +05a
        move.l  a1,(a6)                         | +05e
.L841ac:
        jsr     0x4fa70.l                       | +060
        bcc.w   .L841bc                         | +066
        jmp     0x518.l                         | +06a
.L841bc:
        rts                                     | +070

| ----------------------------------------------------------------------------
|  TaskHandler_0841be  @ $0841BE  (56 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0841be, "ax", @progbits
        .global TaskHandler_0841be
TaskHandler_0841be:
        lea     0xffff.w,a0                     | +000
        move.l  a0,0x48(a6)                     | +004
        lea     0x2e6ff6.l,a0                   | +008
        jsr     0x28cd4.l                       | +00e
        lea     .L841d8(pc),a1                  | +014
        move.l  a1,(a6)                         | +018
.L841d8:
        jsr     0x2783a.l                       | +01a
        jsr     0x28d70.l                       | +020
        jsr     0x4fa70.l                       | +026
        bcc.w   .L841f4                         | +02c
        jmp     0x518.l                         | +030
.L841f4:
        rts                                     | +036

| ----------------------------------------------------------------------------
|  TaskHandler_0841f6  @ $0841F6  (86 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0841f6, "ax", @progbits
        .global TaskHandler_0841f6
TaskHandler_0841f6:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x40,0x70(a6)                  | +00a
        move.w  #0x1,0x66(a6)                   | +010
        lea     0x2e7006.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
        lea     .L8421e(pc),a1                  | +022
        move.l  a1,(a6)                         | +026
.L8421e:
        jsr     0x2783a.l                       | +028
        jsr     0x28d70.l                       | +02e
        jsr     0x2870a.l                       | +034
        bcc.w   .L8423a                         | +03a
        lea     TaskHandler_08424c(pc),a1       | +03e
        move.l  a1,(a6)                         | +042
.L8423a:
        jsr     0x4fa70.l                       | +044
        bcc.w   .L8424a                         | +04a
        jmp     0x518.l                         | +04e
.L8424a:
        rts                                     | +054

| ----------------------------------------------------------------------------
|  TaskHandler_08424c  @ $08424C  (26 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08424c, "ax", @progbits
        .global TaskHandler_08424c
TaskHandler_08424c:
        lea     0xffff.w,a0                     | +000
        move.l  a0,0x48(a6)                     | +004
        jsr     Sub_00086400(pc)                | +008
        lea     .L8425e(pc),a1                  | +00c
        move.l  a1,(a6)                         | +010
.L8425e:
        jmp     0x518.l                         | +012
        rts                                     | +018

| ----------------------------------------------------------------------------
|  TaskHandler_084266  @ $084266  (28 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_084266, "ax", @progbits
        .global TaskHandler_084266
TaskHandler_084266:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        jsr     Sub_00086364(pc)                | +00a
        lea     .L8427a(pc),a1                  | +00e
        move.l  a1,(a6)                         | +012
.L8427a:
        jmp     0x518.l                         | +014
        rts                                     | +01a

| ----------------------------------------------------------------------------
|  TaskHandler_084282  @ $084282  (78 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_084282, "ax", @progbits
        .global TaskHandler_084282
TaskHandler_084282:
        move.w  #0x0,0x70(a6)                   | +000
        move.w  #0x1,0x66(a6)                   | +006
        lea     0x2e701c.l,a0                   | +00c
        jsr     0x28cd4.l                       | +012
        lea     .L842a0(pc),a1                  | +018
        move.l  a1,(a6)                         | +01c
.L842a0:
        jsr     0x2783a.l                       | +01e
        jsr     0x28d70.l                       | +024
        jsr     0x2870a.l                       | +02a
        bcc.w   .L842be                         | +030
        jsr     Sub_000864EA(pc)                | +034
        bra.w   .L842c8                         | +038
.L842be:
        jsr     0x4fa70.l                       | +03c
        bcc.w   .L842ce                         | +042
.L842c8:
        jmp     0x518.l                         | +046
.L842ce:
        rts                                     | +04c

| ----------------------------------------------------------------------------
|  TaskHandler_0842d0  @ $0842D0  (198 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0842d0, "ax", @progbits
        .global TaskHandler_0842d0
TaskHandler_0842d0:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x30,0x70(a6)                  | +00a
        move.w  #0x1,0x66(a6)                   | +010
        lea     0x2e7032.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
        lea     .L842f8(pc),a1                  | +022
        move.l  a1,(a6)                         | +026
.L842f8:
        jsr     0x2783a.l                       | +028
        jsr     0x28d70.l                       | +02e
        jsr     0x2870a.l                       | +034
        bcc.w   .L84320                         | +03a
        lea     0x5e766.l,a0                    | +03e
        jsr     0x5e770.l                       | +044
        bclr    #0x3,0x13(a6)                   | +04a
.L84320:
        jsr     0x28758.l                       | +050
        bcc.w   .L84384                         | +056
        move.l  #0x1000,d0                      | +05a
        jsr     0x51a28.l                       | +060
        move.w  #0x1030,d0                      | +066
        jsr     0x2352.l                        | +06a
        lea     0xffff.w,a0                     | +070
        move.l  a0,0x48(a6)                     | +074
        jsr     Sub_0008640E(pc)                | +078
        jsr     Sub_0008641C(pc)                | +07c
        lea     0x2e9b00.l,a1                   | +080
        jsr     0x77c7e.l                       | +086
        lea     0x2e9aca.l,a1                   | +08c
        jsr     0x77c7e.l                       | +092
        lea     0x2eacb8.l,a1                   | +098
        jsr     0x43fac.l                       | +09e
        lea     0x2eae76.l,a1                   | +0a4
        jsr     0x43fac.l                       | +0aa
        bra.w   .L8438e                         | +0b0
.L84384:
        jsr     0x4fa70.l                       | +0b4
        bcc.w   .L84394                         | +0ba
.L8438e:
        jmp     0x518.l                         | +0be
.L84394:
        rts                                     | +0c4

| ----------------------------------------------------------------------------
|  TaskHandler_084396  @ $084396  (114 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_084396, "ax", @progbits
        .global TaskHandler_084396
TaskHandler_084396:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0xa9,d1                        | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0x1,0x38(a6)                   | +014
        jsr     Sub_000860E4(pc)                | +01a
        jsr     Sub_00086300(pc)                | +01e
        lea     Sub_0008495E(pc),a1             | +022
        jsr     0x4ae.l                         | +026
        jsr     0x5dd22.l                       | +02c
        move.b  #0x0,0x21(a6)                   | +032
        lea     0x2e7048.l,a0                   | +038
        jsr     0x28cd4.l                       | +03e
        lea     .L843e0(pc),a1                  | +044
        move.l  a1,(a6)                         | +048
.L843e0:
        clr.b   0x10e39a.l                      | +04a
        jsr     0x2783a.l                       | +050
        jsr     0x28d70.l                       | +056
        move.w  #0x7fff,0x66(a6)                | +05c
        cmpi.b  #0xff,0x21(a6)                  | +062
        bne.w   SetHandlerRts_08440e            | +068
        move.b  #0xff,0x74(a6)                  | +06c

| ----------------------------------------------------------------------------
|  TaskHandler_084410  @ $084410  (168 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_084410, "ax", @progbits
        .global TaskHandler_084410
TaskHandler_084410:
        move.b  #0x3,0x10e39c.l                 | +000
        lea     TaskHandler_0845b8(pc),a1       | +008
        jsr     0x4ae.l                         | +00c
        move.w  #0xa,0x72(a0)                   | +012
        lea     0xffff.w,a0                     | +018
        move.l  a0,0x48(a6)                     | +01c
        move.w  #0x1033,d0                      | +020
        jsr     0x2352.l                        | +024
        move.w  #0x27,d1                        | +02a
        move.w  #0x129,d2                       | +02e
        move.w  #0x2,d3                         | +032
        move.w  #0x1,d4                         | +036
        jsr     0x2c26.l                        | +03a
        lea     0x86586.l,a1                    | +040
        jsr     0x4ae.l                         | +046
        lea     0x2ea790.l,a2                   | +04c
        jsr     0x5022a.l                       | +052
        jsr     Sub_00086196(pc)                | +058
        lea     0x2e98d8.l,a1                   | +05c
        jsr     0x77c7e.l                       | +062
        lea     0x2e98ea.l,a1                   | +068
        jsr     0x77c7e.l                       | +06e
        move.w  #0xc000,0x38(a0)                | +074
        jsr     Sub_0008610C(pc)                | +07a
        move.b  #0x0,0x21(a6)                   | +07e
        lea     .L8449a(pc),a1                  | +084
        move.l  a1,(a6)                         | +088
.L8449a:
        clr.b   0x10e39a.l                      | +08a
        jsr     0x2783a.l                       | +090
        cmpi.b  #0x9,0x21(a6)                   | +096
        blt.w   SetHandlerRts_0844be            | +09c
        move.b  #0x2,0x10e39a.l                 | +0a0

| ----------------------------------------------------------------------------
|  TaskHandler_0844c0  @ $0844C0  (68 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0844c0, "ax", @progbits
        .global TaskHandler_0844c0
TaskHandler_0844c0:
        jsr     0x2783a.l                       | +000
        lea     0x2eacd4.l,a1                   | +006
        jsr     0x43fac.l                       | +00c
        lea     0x2e7486.l,a0                   | +012
        jsr     0x28cd4.l                       | +018
        lea     .L844e4(pc),a1                  | +01e
        move.l  a1,(a6)                         | +022
.L844e4:
        clr.b   0x10e39a.l                      | +024
        jsr     0x2783a.l                       | +02a
        jsr     0x28d70.l                       | +030
        bcc.w   SetHandlerRts_08450a            | +036
        jsr     0x6f0.l                         | +03a
        bcs.w   SetHandlerRts_08450a            | +040

| ----------------------------------------------------------------------------
|  TaskHandler_08450c  @ $08450C  (20 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08450c, "ax", @progbits
        .global TaskHandler_08450c
TaskHandler_08450c:
        clr.b   0x10e39c.l                      | +000
        jsr     0x320d4.l                       | +006
        jmp     0x518.l                         | +00c
        rts                                     | +012

| ----------------------------------------------------------------------------
|  TaskHandler_084520  @ $084520  (152 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_084520, "ax", @progbits
        .global TaskHandler_084520
TaskHandler_084520:
        move.w  #0x100,0x70(a6)                 | +000
        lea     0x2e9256.l,a0                   | +006
        move.l  a0,0x48(a6)                     | +00c
        lea     .L84536(pc),a1                  | +010
        move.l  a1,(a6)                         | +014
.L84536:
        jsr     0x2783a.l                       | +016
        move.w  #0x7fff,0x66(a6)                | +01c
        jsr     0x2870a.l                       | +022
        bcc.w   .L84596                         | +028
        lea     0x5e766.l,a0                    | +02c
        jsr     0x5e770.l                       | +032
        bclr    #0x3,0x13(a6)                   | +038
        move.b  0x106f28.l,d0                   | +03e
        cmpi.b  #0x1,0x58(a6)                   | +044
        beq.w   .L8457c                         | +04a
        cmpi.b  #0x2,0x58(a6)                   | +04e
        beq.w   .L8457c                         | +054
        bra.w   .L84596                         | +058
.L8457c:
        btst    #0x0,d0                         | +05c
        bne.w   .L84596                         | +060
        lea     0x53886.l,a1                    | +064
        jsr     0x4ae.l                         | +06a
        jsr     0x5dd22.l                       | +070
.L84596:
        jsr     0x28758.l                       | +076
        bcc.w   .L845a6                         | +07c
        bclr    #0x0,0x13(a6)                   | +080
.L845a6:
        jsr     0x4fa70.l                       | +086
        bcc.w   .L845b6                         | +08c
        jmp     0x518.l                         | +090
.L845b6:
        rts                                     | +096

| ----------------------------------------------------------------------------
|  TaskHandler_0845b8  @ $0845B8  (44 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0845b8, "ax", @progbits
        .global TaskHandler_0845b8
TaskHandler_0845b8:
        move.b  #0x1,0x10a2d1.l                 | +000
        lea     .L845c6(pc),a1                  | +008
        move.l  a1,(a6)                         | +00c
.L845c6:
        subq.w  #0x1,0x72(a6)                   | +00e
        beq.w   JmpToScheduler_0845e4           | +012
        move.w  0x72(a6),d0                     | +016
        andi.w  #0x1,d0                         | +01a
        beq.w   .L845e2                         | +01e
        move.b  #0x1,0x10a2d1.l                 | +022
.L845e2:
        rts                                     | +02a

| ----------------------------------------------------------------------------
|  TaskHandler_0845ec  @ $0845EC  (76 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0845ec, "ax", @progbits
        .global TaskHandler_0845ec
TaskHandler_0845ec:
        lea     .L845f2(pc),a1                  | +000
        move.l  a1,(a6)                         | +004
.L845f2:
        movea.l 0xc(a6),a0                      | +006
        btst    #0x3,0x13(a0)                   | +00a
        beq.w   .L84622                         | +010
        move.b  0x20(a0),d0                     | +014
        cmp.b   0x21(a6),d0                     | +018
        bne.w   .L84622                         | +01c
        bclr    #0x3,0x13(a0)                   | +020
        move.b  0x58(a0),0x58(a6)               | +026
        jsr     Sub_0008651E(pc)                | +02c
        lea     TaskHandler_084638(pc),a1       | +030
        move.l  a1,(a6)                         | +034
.L84622:
        movea.l 0xc(a6),a0                      | +036
        cmpi.b  #0xff,0x21(a0)                  | +03a
        bne.w   .L84636                         | +040
        jmp     0x518.l                         | +044
.L84636:
        rts                                     | +04a

| ----------------------------------------------------------------------------
|  TaskHandler_084638  @ $084638  (98 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_084638, "ax", @progbits
        .global TaskHandler_084638
TaskHandler_084638:
        move.l  #0x1000,d0                      | +000
        jsr     0x51a28.l                       | +006
        move.w  #0x108d,d0                      | +00c
        jsr     0x2352.l                        | +010
        lea     .L84654(pc),a1                  | +016
        move.l  a1,(a6)                         | +01a
.L84654:
        movea.l 0xc(a6),a0                      | +01c
        btst    #0x3,0x13(a0)                   | +020
        beq.w   .L84684                         | +026
        move.b  0x20(a0),d0                     | +02a
        cmp.b   0x21(a6),d0                     | +02e
        bne.w   .L84684                         | +032
        bclr    #0x3,0x13(a0)                   | +036
        move.b  0x58(a0),0x58(a6)               | +03c
        jsr     Sub_00086538(pc)                | +042
        lea     TaskHandler_08469a(pc),a1       | +046
        move.l  a1,(a6)                         | +04a
.L84684:
        movea.l 0xc(a6),a0                      | +04c
        cmpi.b  #0xff,0x21(a0)                  | +050
        bne.w   .L84698                         | +056
        jmp     0x518.l                         | +05a
.L84698:
        rts                                     | +060

| ----------------------------------------------------------------------------
|  TaskHandler_08469a  @ $08469A  (70 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08469a, "ax", @progbits
        .global TaskHandler_08469a
TaskHandler_08469a:
        move.w  #0x108d,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     .L846aa(pc),a1                  | +00a
        move.l  a1,(a6)                         | +00e
.L846aa:
        movea.l 0xc(a6),a0                      | +010
        btst    #0x3,0x13(a0)                   | +014
        beq.w   .L846ca                         | +01a
        move.b  0x20(a0),d0                     | +01e
        cmp.b   0x21(a6),d0                     | +022
        bne.w   .L846ca                         | +026
        bclr    #0x3,0x13(a0)                   | +02a
.L846ca:
        movea.l 0xc(a6),a0                      | +030
        cmpi.b  #0xff,0x21(a0)                  | +034
        bne.w   .L846de                         | +03a
        jmp     0x518.l                         | +03e
.L846de:
        rts                                     | +044

| ----------------------------------------------------------------------------
|  TaskHandler_0846e0  @ $0846E0  (328 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0846e0, "ax", @progbits
        .global TaskHandler_0846e0
TaskHandler_0846e0:
        move.w  #0xa9,d1                        | +000
        jsr     0x236e.l                        | +004
        move.b  #0xff,0x32(a6)                  | +00a
        move.b  #0xff,0x33(a6)                  | +010
        move.b  #0x0,0x3a(a6)                   | +016
        addi.w  #0x80,0x24(a6)                  | +01c
        addi.w  #0x29,0x22(a6)                  | +022
        cmpi.b  #0x0,0x21(a6)                   | +028
        beq.w   .L84720                         | +02e
        move.b  0x21(a6),d0                     | +032
.L84716:
        addi.w  #0x40,0x22(a6)                  | +036
        subq.b  #0x1,d0                         | +03c
        bne.b   .L84716                         | +03e
.L84720:
        move.w  #0x14,0x66(a6)                  | +040
        clr.l   d0                              | +046
        move.b  0x21(a6),d0                     | +048
        movea.l #0x2e7556,a0                    | +04c
        lsl.w   #0x2,d0                         | +052
        movea.l (a0,d0.w),a0                    | +054
        cmpa.l  #0xffffffff,a0                  | +058
        beq.w   .L84748                         | +05e
        jsr     0x28cd4.l                       | +062
.L84748:
        jsr     0x5e9b6.l                       | +068
        andi.l  #0xf,d0                         | +06e
        asl.l   #0x2,d0                         | +074
        lea     0x2eb02c.l,a0                   | +076
        movea.l (a0,d0.w),a1                    | +07c
        jsr     0x4ae.l                         | +080
        jsr     0x5dd22.l                       | +086
        addi.w  #0x4,0x22(a0)                   | +08c
        addi.w  #0x1,0x24(a0)                   | +092
        andi.b  #0x1,d0                         | +098
        add.b   d0,0x98(a0)                     | +09c
        addi.b  #0x7f,0x99(a0)                  | +0a0
        move.b  #0x1,0x11(a0)                   | +0a6
        move.w  #0x8000,0x38(a6)                | +0ac
        jsr     0x267e2.l                       | +0b2
        jsr     0x27cee.l                       | +0b8
        lea     .L847a4(pc),a1                  | +0be
        move.l  a1,(a6)                         | +0c2
.L847a4:
        jsr     0x2783a.l                       | +0c4
        cmpi.w  #0x150,0x22(a6)                 | +0ca
        bgt.w   Jsr5B6Rts_084834                | +0d0
        jsr     0x28d70.l                       | +0d4
        movea.l 0xc(a6),a0                      | +0da
        cmpi.b  #0xff,0x21(a0)                  | +0de
        beq.w   Jsr5B6ThenJmpScheduler_084828   | +0e4
        cmpi.l  #0xffffffff,0x3c(a6)            | +0e8
        beq.w   .L8481e                         | +0f0
        jsr     0x2870a.l                       | +0f4
        bcc.w   .L847f0                         | +0fa
        lea     0x5e766.l,a0                    | +0fe
        jsr     0x5e770.l                       | +104
        bclr    #0x3,0x13(a6)                   | +10a
.L847f0:
        jsr     0x28758.l                       | +110
        bcc.w   .L8481e                         | +116
        move.w  #0x102e,d0                      | +11a
        jsr     0x2352.l                        | +11e
        lea     0xffff.w,a0                     | +124
        jsr     0x28cd4.l                       | +128
        lea     0x2e9adc.l,a1                   | +12e
        jsr     0x77c7e.l                       | +134
        bra.w   Jsr5B6Rts_084834                | +13a
.L8481e:
        jsr     0x4fa70.l                       | +13e
        bcc.w   Jsr5B6Rts_084834                | +144
