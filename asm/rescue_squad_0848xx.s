| =============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave III — Escuadrón de rescate y ciclo de vuelo
|  Región: $084836..$08512C  (2,192 B, 22 entradas, 12 huecos cerrados)
| =============================================================================
|
|  Continuación del bloque del miniboss (Waves GGG/HHH): rutinas del
|  escuadrón de rescate (tabla de listas $2E77CA indexada por +$21) y el
|  ciclo de vuelo del transporte con sus fases de aleteo. Estructura:
|
|   * $84836..$8492A — pasos del escuadrón: snd $A9, sprites de la tabla
|     $2E77CA[(+$21<<1)<<2] y [(+$21<<1)+1<<2] (con centinela $FFFFFFFF),
|     espera de sincronía con el padre (+$21) y avance con retroceso
|     -$10 y bset bit6/$12 del par $2E993C (cae en el thunk
|     JsrAbsThunk_08492a que reinstala $84932).
|   * $84932..$8495E — guardián de salida: helper Sub_0008601C (hueco
|     futuro), luego Sub_000863A0 + probe $27CEE hasta que y<=$40.
|   * $8495E..$84AAA — handler del hijo spawneado desde el Wave HHH
|     ($843B8 via $4AE): snd $85, timer aleatorio de $2C0218 ($799DE),
|     avance +$98 hasta x>$150, luego montaje de la pareja $84AAA/$6AA14
|     (offset +$65, +$3A=1, +$98=$12) y fase con hit-test ($2870A,
|     flash $5E766/$5E770), armadura x2 a partir de scroll $4F0
|     ($106F50 -> lista $2E915A) y muerte con snd $1028, pares
|     $2E98A2/$2E98B4/$2E9AEE y blitter $2EACC4.
|   * $84AAA..$84B1C — pareja del transporte: snd $E, offsets +$17/+$28,
|     sprite $2C10D8, espera a que el padre muera (+$20==$FF) y spawnea
|     el paracaidista $84C5E via $4AE + $5DD22.
|   * $84B24..$84C50 — secuencia de caída en 4 etapas (sprites $2B5B92/
|     $2B5C22/$2B5CFA/$2B604A): retrocesos -$20/-$180, vel +$27, snd
|     $1040 al tocar suelo (y<=$84) y liberación del abuelo
|     (+$21=$FF via a0=$C(a0)) cuando y<=$64.
|   * $84C5E..$84C9C — paracaidista: snd $4, sprite $2E75C0, sigue
|     x/y del padre con offset +$1E (cae en el thunk JsrAbsThunk_084c9c).
|   * $84CA4..$84DB0 — rescatado con premio: +$21=+$38, lista $2E9202,
|     hit-test y muerte con score $2000 (snd $1027, helper Sub_00086504,
|     par $2E9AB8); variante con snd $A8, sprite $2E77BA y giro $8000.
|   * $84DB0..$84F4E — selector por tipo +$98 (0/1/2): listas
|     $2E92AA/$2E92FE/$2E9352, pares $2E9C12/$2E9C24/$2E9C36 y colas
|     $2E9838/$2E9842/$2E984C; bucle de spawn multiple segun +$99 y
|     salida via $5DD5C; entrada con sprite $2E7812 (+$66=$7FFF, limpia
|     bit0/bit3 de +$13) y arranque del vuelo: snd $1B1, +$72=0,
|     +$38=$2000, helpers Sub_00085F44/Sub_00085F60 (huecos futuros).
|   * $84F5E..$8512C — ciclo de vuelo del transporte: snd $1074, sprites
|     $2E7828/$2E7866 (subida/bajada con vel +-(+$9A<<3)), helpers
|     Sub_00085EE8/Sub_00085F08 (huecos futuros), aterrizaje cuando
|     scroll $106F5C<=$108 con probe $6F0 spawneando $8512C via $4AE +
|     $5DD02; fase de planeo (snd $1075, sprite $2E78A4, timer +$72=
|     $A/$14) que alterna de vuelta a $84FCA o $84F5E segun Sub_00085F60.
|
|  Los bcc/bne/bgt.w colgantes apuntan a los RTS internos de las islas C
|  contiguas (SetHandlerRts_084898/_0848dc/_084b22/_084b98/_084bd0/
|  _084c24 en +6, Jsr5B6Rts_084c5c en +12); los saltos a $8492A/$84B5A/
|  $84F56/$8512C usan los simbolos reales de las islas.
|
|  Verificación: cada sección .text.<Sym> se coloca en su dirección CPU
|  absoluta y reensambla byte-exacta contra build/mslug_prom.bin
|  (MD5 816b3f74c76b3373993407615f1850fe).
| =============================================================================

        .text

| ----------------------------------------------------------------------------
|  TaskHandler_084836  @ $084836  (92 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_084836, "ax", @progbits
        .global TaskHandler_084836
TaskHandler_084836:
        move.w  #0xa9,d1                        | +000
        jsr     0x236e.l                        | +004
        move.b  #0xff,0x32(a6)                  | +00a
        move.b  #0xff,0x33(a6)                  | +010
        clr.l   d0                              | +016
        move.b  0x21(a6),d0                     | +018
        asl.l   #0x1,d0                         | +01c
        movea.l #0x2e77ca,a0                    | +01e
        lsl.w   #0x2,d0                         | +024
        movea.l (a0,d0.w),a0                    | +026
        cmpa.l  #0xffffffff,a0                  | +02a
        beq.w   .L84870                         | +030
        jsr     0x28cd4.l                       | +034
.L84870:
        lea     .L84876(pc),a1                  | +03a
        move.l  a1,(a6)                         | +03e
.L84876:
        jsr     0x2783a.l                       | +040
        jsr     0x28d70.l                       | +046
        movea.l 0xc(a6),a0                      | +04c
        move.b  0x21(a6),d0                     | +050
        cmp.b   0x21(a0),d0                     | +054
        bne.w   SetHandlerRts_084898            | +058

| ----------------------------------------------------------------------------
|  TaskHandler_08489a  @ $08489A  (60 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08489a, "ax", @progbits
        .global TaskHandler_08489a
TaskHandler_08489a:
        clr.l   d0                              | +000
        move.b  0x21(a6),d0                     | +002
        asl.l   #0x1,d0                         | +006
        addq.l  #0x1,d0                         | +008
        movea.l #0x2e77ca,a0                    | +00a
        lsl.w   #0x2,d0                         | +010
        movea.l (a0,d0.w),a0                    | +012
        cmpa.l  #0xffffffff,a0                  | +016
        beq.w   .L848c0                         | +01c
        jsr     0x28cd4.l                       | +020
.L848c0:
        lea     .L848c6(pc),a1                  | +026
        move.l  a1,(a6)                         | +02a
.L848c6:
        jsr     0x2783a.l                       | +02c
        jsr     0x28d70.l                       | +032
        bcc.w   SetHandlerRts_0848dc            | +038

| ----------------------------------------------------------------------------
|  TaskHandler_0848de  @ $0848DE  (76 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0848de, "ax", @progbits
        .global TaskHandler_0848de
TaskHandler_0848de:
        movea.l 0xc(a6),a0                      | +000
        addq.b  #0x1,0x21(a0)                   | +004
        jsr     0x267e2.l                       | +008
        move.w  #0xfff0,0x2e(a6)                | +00e
        lea     .L848f8(pc),a1                  | +014
        move.l  a1,(a6)                         | +018
.L848f8:
        jsr     Sub_000863A0(pc)                | +01a
        jsr     0x27cee.l                       | +01e
        bcc.w   JsrAbsThunk_08492a              | +024
        jsr     0x434ce.l                       | +028
        lea     0x2e993c.l,a1                   | +02e
        jsr     0x77c7e.l                       | +034
        bset    #0x6,0x12(a0)                   | +03a
        subi.w  #0x40,0x24(a0)                  | +040
        lea     TaskHandler_084932(pc),a1       | +046
        move.l  a1,(a6)                         | +04a

| ----------------------------------------------------------------------------
|  TaskHandler_084932  @ $084932  (44 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_084932, "ax", @progbits
        .global TaskHandler_084932
TaskHandler_084932:
        jsr     Sub_0008601C(pc)                | +000
        lea     .L8493c(pc),a1                  | +004
        move.l  a1,(a6)                         | +008
.L8493c:
        jsr     Sub_000863A0(pc)                | +00a
        jsr     0x27cee.l                       | +00e
        jsr     0x28d70.l                       | +014
        cmpi.w  #0x40,0x24(a6)                  | +01a
        bgt.w   .L8495c                         | +020
        jmp     0x518.l                         | +024
.L8495c:
        rts                                     | +02a

| ----------------------------------------------------------------------------
|  Sub_0008495E  @ $08495E  (92 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0008495E, "ax", @progbits
        .global Sub_0008495E
Sub_0008495E:
        move.w  #0x85,d1                        | +000
        jsr     0x236e.l                        | +004
        move.b  #0xff,0x32(a6)                  | +00a
        move.b  #0xff,0x33(a6)                  | +010
        move.b  #0x0,0x3a(a6)                   | +016
        lea     0x2c0218.l,a0                   | +01c
        jsr     0x799de.l                       | +022
        move.w  d0,0x66(a6)                     | +028
        move.w  #0x0,0x38(a6)                   | +02c
        addi.w  #0x98,0x22(a6)                  | +032
        subi.w  #0x8,0x24(a6)                   | +038
        move.b  #0x0,0x20(a6)                   | +03e
        lea     .L849a8(pc),a1                  | +044
        move.l  a1,(a6)                         | +048
.L849a8:
        jsr     0x2783a.l                       | +04a
        cmpi.w  #0x150,0x22(a6)                 | +050
        ble.w   TaskHandler_0849ba              | +056
        rts                                     | +05a

| ----------------------------------------------------------------------------
|  TaskHandler_0849ba  @ $0849BA  (240 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0849ba, "ax", @progbits
        .global TaskHandler_0849ba
TaskHandler_0849ba:
        lea     0x2e7566.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0xffff.w,a0                     | +00c
        move.l  a0,0x48(a6)                     | +010
        lea     TaskHandler_084aaa(pc),a1       | +014
        jsr     0x4ae.l                         | +018
        jsr     0x5dd22.l                       | +01e
        lea     0x6aa14.l,a1                    | +024
        jsr     0x4ae.l                         | +02a
        jsr     0x5dd22.l                       | +030
        addi.w  #0x65,0x22(a0)                  | +036
        move.b  #0x1,0x3a(a0)                   | +03c
        move.b  #0x12,0x98(a0)                  | +042
        move.l  #0x2eb06c,0x60(a6)              | +048
        lea     .L84a10(pc),a1                  | +050
        move.l  a1,(a6)                         | +054
.L84a10:
        jsr     0x28998.l                       | +056
        jsr     0x2783a.l                       | +05c
        jsr     0x28d70.l                       | +062
        jsr     0x2870a.l                       | +068
        bcc.w   .L84a3e                         | +06e
        lea     0x5e766.l,a0                    | +072
        jsr     0x5e770.l                       | +078
        bclr    #0x3,0x13(a6)                   | +07e
.L84a3e:
        move.l  0x106f50.l,d0                   | +084
        swap    d0                              | +08a
        cmpi.w  #0x4f0,d0                       | +08c
        blt.w   .L84a58                         | +090
        lea     0x2e915a.l,a0                   | +094
        move.l  a0,0x48(a6)                     | +09a
.L84a58:
        jsr     0x28758.l                       | +09e
        bcc.w   .L84aa8                         | +0a4
        move.b  #0xff,0x20(a6)                  | +0a8
        move.w  #0x1028,d0                      | +0ae
        jsr     0x2352.l                        | +0b2
        lea     0x2e98a2.l,a1                   | +0b8
        jsr     0x77c7e.l                       | +0be
        lea     0x2e98b4.l,a1                   | +0c4
        jsr     0x77c7e.l                       | +0ca
        lea     0x2e9aee.l,a1                   | +0d0
        jsr     0x77c7e.l                       | +0d6
        lea     0x2eacc4.l,a1                   | +0dc
        jsr     0x43fac.l                       | +0e2
        jmp     0x518.l                         | +0e8
.L84aa8:
        rts                                     | +0ee

| ----------------------------------------------------------------------------
|  TaskHandler_084aaa  @ $084AAA  (114 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_084aaa, "ax", @progbits
        .global TaskHandler_084aaa
TaskHandler_084aaa:
        move.w  #0xe,d1                         | +000
        jsr     0x236e.l                        | +004
        move.b  #0xff,0x32(a6)                  | +00a
        move.b  #0xff,0x33(a6)                  | +010
        move.b  #0x1,0x3a(a6)                   | +016
        addi.w  #0x17,0x22(a6)                  | +01c
        addi.w  #0x28,0x24(a6)                  | +022
        move.w  #0x0,0x38(a6)                   | +028
        lea     0x2c10d8.l,a0                   | +02e
        jsr     0x28cd4.l                       | +034
        lea     0xffff.w,a0                     | +03a
        move.l  a0,0x48(a6)                     | +03e
        lea     .L84af2(pc),a1                  | +042
        move.l  a1,(a6)                         | +046
.L84af2:
        jsr     0x2783a.l                       | +048
        jsr     0x28d70.l                       | +04e
        movea.l 0xc(a6),a0                      | +054
        cmpi.b  #0xff,0x20(a0)                  | +058
        bne.w   SetHandlerRts_084b22            | +05e
        lea     TaskHandler_084c5e(pc),a1       | +062
        jsr     0x4ae.l                         | +066
        jsr     0x5dd22.l                       | +06c

| ----------------------------------------------------------------------------
|  TaskHandler_084b24  @ $084B24  (54 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_084b24, "ax", @progbits
        .global TaskHandler_084b24
TaskHandler_084b24:
        jsr     0x267e2.l                       | +000
        move.w  #0xffe0,0x2e(a6)                | +006
        lea     0x2b5b92.l,a0                   | +00c
        jsr     0x28cd4.l                       | +012
        lea     0xffff.w,a0                     | +018
        move.l  a0,0x48(a6)                     | +01c
        lea     .L84b4a(pc),a1                  | +020
        move.l  a1,(a6)                         | +024
.L84b4a:
        jsr     0x27cee.l                       | +026
        bcc.w   JsrAbsThunk_084b5a              | +02c
        lea     TaskHandler_084b62(pc),a1       | +030
        move.l  a1,(a6)                         | +034

| ----------------------------------------------------------------------------
|  TaskHandler_084b62  @ $084B62  (48 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_084b62, "ax", @progbits
        .global TaskHandler_084b62
TaskHandler_084b62:
        jsr     0x267e2.l                       | +000
        lea     0x2b5c22.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     0xffff.w,a0                     | +012
        move.l  a0,0x48(a6)                     | +016
        lea     .L84b82(pc),a1                  | +01a
        move.l  a1,(a6)                         | +01e
.L84b82:
        jsr     0x2783a.l                       | +020
        jsr     0x28d70.l                       | +026
        bcc.w   SetHandlerRts_084b98            | +02c

| ----------------------------------------------------------------------------
|  TaskHandler_084b9a  @ $084B9A  (48 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_084b9a, "ax", @progbits
        .global TaskHandler_084b9a
TaskHandler_084b9a:
        jsr     0x267e2.l                       | +000
        lea     0x2b5cfa.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     0xffff.w,a0                     | +012
        move.l  a0,0x48(a6)                     | +016
        lea     .L84bba(pc),a1                  | +01a
        move.l  a1,(a6)                         | +01e
.L84bba:
        jsr     0x2783a.l                       | +020
        jsr     0x28d70.l                       | +026
        bcc.w   SetHandlerRts_084bd0            | +02c

| ----------------------------------------------------------------------------
|  TaskHandler_084bd2  @ $084BD2  (76 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_084bd2, "ax", @progbits
        .global TaskHandler_084bd2
TaskHandler_084bd2:
        jsr     0x267e2.l                       | +000
        move.w  #0xfe80,0x28(a6)                | +006
        move.w  #0x27,0x2a(a6)                  | +00c
        lea     0x2b604a.l,a0                   | +012
        jsr     0x28cd4.l                       | +018
        lea     0xffff.w,a0                     | +01e
        move.l  a0,0x48(a6)                     | +022
        lea     .L84bfe(pc),a1                  | +026
        move.l  a1,(a6)                         | +02a
.L84bfe:
        jsr     0x27cee.l                       | +02c
        jsr     0x28d70.l                       | +032
        cmpi.w  #0x84,0x22(a6)                  | +038
        bgt.w   SetHandlerRts_084c24            | +03e
        move.w  #0x1040,d0                      | +042
        jsr     0x2352.l                        | +046

| ----------------------------------------------------------------------------
|  TaskHandler_084c26  @ $084C26  (42 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_084c26, "ax", @progbits
        .global TaskHandler_084c26
TaskHandler_084c26:
        lea     .L84c2c(pc),a1                  | +000
        move.l  a1,(a6)                         | +004
.L84c2c:
        jsr     0x27cee.l                       | +006
        jsr     0x28d70.l                       | +00c
        cmpi.w  #0x64,0x22(a6)                  | +012
        bgt.w   Jsr5B6Rts_084c5c                | +018
        movea.l 0xc(a6),a0                      | +01c
        movea.l 0xc(a0),a0                      | +020
        move.b  #0xff,0x21(a0)                  | +024

| ----------------------------------------------------------------------------
|  TaskHandler_084c5e  @ $084C5E  (62 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_084c5e, "ax", @progbits
        .global TaskHandler_084c5e
TaskHandler_084c5e:
        move.w  #0x4,d1                         | +000
        jsr     0x236e.l                        | +004
        move.b  #0xff,0x32(a6)                  | +00a
        move.b  #0xff,0x33(a6)                  | +010
        lea     0x2e75c0.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
        lea     .L84c86(pc),a1                  | +022
        move.l  a1,(a6)                         | +026
.L84c86:
        movea.l 0xc(a6),a0                      | +028
        move.w  0x22(a0),0x22(a6)               | +02c
        move.w  0x24(a0),0x24(a6)               | +032
        addi.w  #0x1e,0x24(a6)                  | +038

| ----------------------------------------------------------------------------
|  TaskHandler_084ca4  @ $084CA4  (158 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_084ca4, "ax", @progbits
        .global TaskHandler_084ca4
TaskHandler_084ca4:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x20,0x70(a6)                  | +00a
        move.w  #0xa,0x66(a6)                   | +010
        move.w  0x38(a6),d0                     | +016
        move.b  d0,0x21(a6)                     | +01a
        lea     0x2e9202.l,a0                   | +01e
        move.l  a0,0x48(a6)                     | +024
        lea     .L84cd2(pc),a1                  | +028
        move.l  a1,(a6)                         | +02c
.L84cd2:
        jsr     0x2783a.l                       | +02e
        jsr     0x2870a.l                       | +034
        bcc.w   .L84cf4                         | +03a
        lea     0x5e766.l,a0                    | +03e
        jsr     0x5e770.l                       | +044
        bclr    #0x3,0x13(a6)                   | +04a
.L84cf4:
        jsr     0x28758.l                       | +050
        bcc.w   .L84d30                         | +056
        move.l  #0x2000,d0                      | +05a
        jsr     0x51a28.l                       | +060
        move.w  #0x1027,d0                      | +066
        jsr     0x2352.l                        | +06a
        lea     0xffff.w,a0                     | +070
        move.l  a0,0x48(a6)                     | +074
        jsr     Sub_00086504(pc)                | +078
        lea     0x2e9ab8.l,a1                   | +07c
        jsr     0x77c7e.l                       | +082
        bra.w   .L84d3a                         | +088
.L84d30:
        jsr     0x4fa70.l                       | +08c
        bcc.w   .L84d40                         | +092
.L84d3a:
        jmp     0x518.l                         | +096
.L84d40:
        rts                                     | +09c

| ----------------------------------------------------------------------------
|  TaskHandler_084d42  @ $084D42  (110 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_084d42, "ax", @progbits
        .global TaskHandler_084d42
TaskHandler_084d42:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0xa8,d1                        | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0x40,0x70(a6)                  | +014
        move.b  #0xff,0x32(a6)                  | +01a
        move.b  #0xff,0x33(a6)                  | +020
        move.b  #0x0,0x3a(a6)                   | +026
        lea     0x2e77ba.l,a0                   | +02c
        jsr     0x28cd4.l                       | +032
        move.w  #0x8000,0x38(a6)                | +038
        jsr     0x267e2.l                       | +03e
        jsr     0x27cee.l                       | +044
        lea     .L84d92(pc),a1                  | +04a
        move.l  a1,(a6)                         | +04e
.L84d92:
        jsr     0x2783a.l                       | +050
        jsr     0x28d70.l                       | +056
        jsr     0x4fa70.l                       | +05c
        bcc.w   .L84dae                         | +062
        jmp     0x518.l                         | +066
.L84dae:
        rts                                     | +06c

| ----------------------------------------------------------------------------
|  TaskHandler_084db0  @ $084DB0  (274 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_084db0, "ax", @progbits
        .global TaskHandler_084db0
TaskHandler_084db0:
        move.b  #0xff,0x32(a6)                  | +000
        move.b  #0xff,0x33(a6)                  | +006
        move.b  #0x0,0x3a(a6)                   | +00c
        cmpi.b  #0x0,0x98(a6)                   | +012
        bne.w   .L84dee                         | +018
        lea     0x2e92aa.l,a0                   | +01c
        move.l  a0,0x48(a6)                     | +022
        lea     0x2e9c12.l,a0                   | +026
        move.l  a0,0x70(a6)                     | +02c
        lea     0x2e9838.l,a0                   | +030
        move.l  a0,0x74(a6)                     | +036
        bra.w   .L84e38                         | +03a
.L84dee:
        cmpi.b  #0x1,0x98(a6)                   | +03e
        bne.w   .L84e1a                         | +044
        lea     0x2e92fe.l,a0                   | +048
        move.l  a0,0x48(a6)                     | +04e
        lea     0x2e9c24.l,a0                   | +052
        move.l  a0,0x70(a6)                     | +058
        lea     0x2e9842.l,a0                   | +05c
        move.l  a0,0x74(a6)                     | +062
        bra.w   .L84e38                         | +066
.L84e1a:
        lea     0x2e9352.l,a0                   | +06a
        move.l  a0,0x48(a6)                     | +070
        lea     0x2e9c36.l,a0                   | +074
        move.l  a0,0x70(a6)                     | +07a
        lea     0x2e984c.l,a0                   | +07e
        move.l  a0,0x74(a6)                     | +084
.L84e38:
        clr.w   d0                              | +088
        move.b  0x9a(a6),d0                     | +08a
        move.w  d0,0x66(a6)                     | +08e
        jsr     0x267e2.l                       | +092
        lea     0x2e6bf0.l,a0                   | +098
        jsr     0x28cd4.l                       | +09e
        lea     .L84e5a(pc),a1                  | +0a4
        move.l  a1,(a6)                         | +0a8
.L84e5a:
        jsr     0x2783a.l                       | +0aa
        jsr     0x28d70.l                       | +0b0
        jsr     0x2870a.l                       | +0b6
        bcc.w   .L84e82                         | +0bc
        lea     0x5e766.l,a0                    | +0c0
        jsr     0x5e770.l                       | +0c6
        bclr    #0x3,0x13(a6)                   | +0cc
.L84e82:
        jsr     0x28758.l                       | +0d2
        bcc.w   .L84eac                         | +0d8
        cmpi.b  #0x0,0x99(a6)                   | +0dc
        beq.w   .L84eba                         | +0e2
.L84e96:
        movea.l 0x70(a6),a1                     | +0e6
        jsr     0x77c7e.l                       | +0ea
        subi.b  #0x1,0x99(a6)                   | +0f0
        beq.w   .L84eba                         | +0f6
        bra.b   .L84e96                         | +0fa
.L84eac:
        movea.l 0x74(a6),a0                     | +0fc
        jsr     0x5dd5c.l                       | +100
        bcc.w   .L84ec0                         | +106
.L84eba:
        jmp     0x518.l                         | +10a
.L84ec0:
        rts                                     | +110

| ----------------------------------------------------------------------------
|  TaskHandler_084ec2  @ $084EC2  (100 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_084ec2, "ax", @progbits
        .global TaskHandler_084ec2
TaskHandler_084ec2:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        lea     0x2e7812.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        move.b  #0xff,0x32(a6)                  | +016
        move.b  #0xff,0x33(a6)                  | +01c
        move.b  #0x0,0x3a(a6)                   | +022
        lea     .L84ef0(pc),a1                  | +028
        move.l  a1,(a6)                         | +02c
.L84ef0:
        jsr     0x2783a.l                       | +02e
        jsr     0x28d70.l                       | +034
        bclr    #0x3,0x13(a6)                   | +03a
        bclr    #0x0,0x13(a6)                   | +040
        move.w  #0x7fff,0x66(a6)                | +046
        lea     0x2e9856.l,a0                   | +04c
        jsr     0x5dd5c.l                       | +052
        bcc.w   .L84f24                         | +058
        jmp     0x518.l                         | +05c
.L84f24:
        rts                                     | +062

| ----------------------------------------------------------------------------
|  TaskHandler_084f26  @ $084F26  (40 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_084f26, "ax", @progbits
        .global TaskHandler_084f26
TaskHandler_084f26:
        move.w  #0x1b1,d1                       | +000
        jsr     0x236e.l                        | +004
        move.w  #0x0,0x72(a6)                   | +00a
        move.w  #0x2000,0x38(a6)                | +010
        jsr     0x267e2.l                       | +016
        jsr     Sub_00085F44(pc)                | +01c
        jsr     Sub_00085F60(pc)                | +020
        bcc.w   SetTaskHandler_084f56           | +024

| ----------------------------------------------------------------------------
|  TaskHandler_084f5e  @ $084F5E  (108 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_084f5e, "ax", @progbits
        .global TaskHandler_084f5e
TaskHandler_084f5e:
        move.w  #0x1074,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x2e7828.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        clr.w   d0                              | +016
        move.b  0x9a(a6),d0                     | +018
        asl.w   #0x3,d0                         | +01c
        move.w  d0,0x2a(a6)                     | +01e
        lea     .L84f86(pc),a1                  | +022
        move.l  a1,(a6)                         | +026
.L84f86:
        jsr     0x27cee.l                       | +028
        jsr     Sub_00085EE8(pc)                | +02e
        jsr     Sub_00085F08(pc)                | +032
        jsr     0x28d70.l                       | +036
        jsr     Sub_00085F44(pc)                | +03c
        jsr     Sub_00085F60(pc)                | +040
        bcc.w   .L84fac                         | +044
        lea     TaskHandler_085062(pc),a1       | +048
        move.l  a1,(a6)                         | +04c
.L84fac:
        movea.l #0xffffffff,a0                  | +04e
        lea     0x2e9860.l,a0                   | +054
        jsr     0x5dd5c.l                       | +05a
        bcc.w   .L84fc8                         | +060
        jmp     0x518.l                         | +064
.L84fc8:
        rts                                     | +06a

| ----------------------------------------------------------------------------
|  TaskHandler_084fca  @ $084FCA  (152 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_084fca, "ax", @progbits
        .global TaskHandler_084fca
TaskHandler_084fca:
        move.w  #0x1074,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x2e7866.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        clr.w   d0                              | +016
        move.b  0x9a(a6),d0                     | +018
        asl.w   #0x3,d0                         | +01c
        neg.w   d0                              | +01e
        move.w  d0,0x2a(a6)                     | +020
        move.l  0x106f5c.l,d0                   | +024
        swap    d0                              | +02a
        cmpi.w  #0x108,d0                       | +02c
        bgt.w   .L85018                         | +030
        jsr     0x6f0.l                         | +034
        bcs.w   .L85018                         | +03a
        lea     SetTaskHandler_08512c(pc),a1    | +03e
        jsr     0x4ae.l                         | +042
        jsr     0x5dd02.l                       | +048
.L85018:
        lea     .L8501e(pc),a1                  | +04e
        move.l  a1,(a6)                         | +052
.L8501e:
        jsr     0x27cee.l                       | +054
        jsr     Sub_00085EE8(pc)                | +05a
        jsr     Sub_00085F08(pc)                | +05e
        jsr     0x28d70.l                       | +062
        jsr     Sub_00085F44(pc)                | +068
        jsr     Sub_00085F60(pc)                | +06c
        bcs.w   .L85044                         | +070
        lea     TaskHandler_085062(pc),a1       | +074
        move.l  a1,(a6)                         | +078
.L85044:
        movea.l #0xffffffff,a0                  | +07a
        lea     0x2e9860.l,a0                   | +080
        jsr     0x5dd5c.l                       | +086
        bcc.w   .L85060                         | +08c
        jmp     0x518.l                         | +090
.L85060:
        rts                                     | +096

| ----------------------------------------------------------------------------
|  TaskHandler_085062  @ $085062  (100 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_085062, "ax", @progbits
        .global TaskHandler_085062
TaskHandler_085062:
        move.w  #0x1075,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x2e78a4.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        move.w  #0xa,0x72(a6)                   | +016
        lea     .L85084(pc),a1                  | +01c
        move.l  a1,(a6)                         | +020
.L85084:
        jsr     0x2783a.l                       | +022
        jsr     Sub_00085F08(pc)                | +028
        jsr     0x28d70.l                       | +02c
        jsr     Sub_00085F44(pc)                | +032
        cmpi.w  #0x0,0x72(a6)                   | +036
        bgt.w   .L850a8                         | +03c
        lea     TaskHandler_0850c6(pc),a1       | +040
        move.l  a1,(a6)                         | +044
.L850a8:
        movea.l #0xffffffff,a0                  | +046
        lea     0x2e9860.l,a0                   | +04c
        jsr     0x5dd5c.l                       | +052
        bcc.w   .L850c4                         | +058
        jmp     0x518.l                         | +05c
.L850c4:
        rts                                     | +062

| ----------------------------------------------------------------------------
|  TaskHandler_0850c6  @ $0850C6  (102 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0850c6, "ax", @progbits
        .global TaskHandler_0850c6
TaskHandler_0850c6:
        lea     0x2e78a4.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        move.w  #0x14,0x72(a6)                  | +00c
        lea     .L850de(pc),a1                  | +012
        move.l  a1,(a6)                         | +016
.L850de:
        jsr     0x2783a.l                       | +018
        jsr     0x28d70.l                       | +01e
        jsr     Sub_00085F44(pc)                | +024
        subq.w  #0x1,0x72(a6)                   | +028
        bpl.w   .L8510e                         | +02c
        jsr     Sub_00085F60(pc)                | +030
        bcc.w   .L85108                         | +034
        lea     TaskHandler_084fca(pc),a1       | +038
        move.l  a1,(a6)                         | +03c
        bra.w   .L8510e                         | +03e
.L85108:
        lea     TaskHandler_084f5e(pc),a1       | +042
        move.l  a1,(a6)                         | +046
.L8510e:
        movea.l #0xffffffff,a0                  | +048
        lea     0x2e9860.l,a0                   | +04e
        jsr     0x5dd5c.l                       | +054
        bcc.w   .L8512a                         | +05a
        jmp     0x518.l                         | +05e
.L8512a:
        rts                                     | +064
