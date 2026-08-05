| ============================================================================
|  Metal Slug 1 - asm/para_squad_module_0818xx.s
|  ----------------------------------------------------------------------------
|  Wave EEE - modulo de escuadron con lider movil (transporte + tropas).
|  Region $081816..$082834, 23 entradas, 3 862 B (cierra los 23 huecos entre
|  las islas JsrAbsThunk_*/SetTaskW_*/SetTaskHandler_*/ClrRamWord_*/
|  ClearXN_*/SetXN_* ya matcheadas en C).
|
|  ARQUITECTURA DEL MODULO
|  -----------------------
|  1. Lider (TaskHandler_081908, 924 B - la entrada mas grande): suena
|     $28/$267E2, spawnea el escolta $82C54 y dos piezas ($81DB0 via pc,
|     $82052/$82062 - variante con flip +0x3a y offset x -$40), luego crea
|     6 hijos en cadena ($81CD6) heredando +0x7C como indice de slot.
|     Maquina de estados con self-patch `lea .Lxxx(pc),a1 ; move.l a1,(a6)`:
|     avance por tabla de sprites $2E541E (indexada por +0x5C, sentinel
|     $FFFFFFFF), fisica $2783A + probes pc-relativos ($82D12/$82CB2/
|     $82DAE), giro segun flag +0x74 (vel +-$20/$60 con curva $2E5B30/
|     $2E5B7A), y muerte TaskHandler_081bee: jingle $1071, congela input
|     ($106ED3), timer $B4 con hija $82BB8, pieza $2E5AEE a mitad de
|     cuenta ($3C) y sonido $1032 en $6E.
|  2. Pieza con humo (TaskHandler_081db0): HP por tabla 2D $2BE322/$2BE3A4
|     segun +0x73 (dificultad), contadores +0x80/+0x82 (x/y de anclaje),
|     dispara de a 3 ($5C) con cadencia por bit-gate de frame ($106F28&7);
|     alterna estados montando pares $2E5AA6/$2E59FE ($77C7E) y variantes
|     de sprite por tabla $2E5832; al agotarse notifica al padre poniendo
|     +0x7B=1 y bset #5 en +0x72 del padre (TaskHandler_081fac: espera
|     a que el padre llegue a estado $77).
|  3. Pieza movil (TaskHandler_082052/_082062): entra por la derecha
|     ($4D/$2E) o izquierda (flip, -$40/$2A); tabla de anim $2E5846 +
|     probe $772 con ENTITY_NIL; ciclo de "oleadas" por +0x5C con
|     cadencias 2D $2BE426..$2BE9BC segun dificultad: camina/dispara
|     ($2E5D2C), rafagas de a 5 ($5E) spawneando hijos $828EE con bset #1
|     en +0x72 del padre, pausa $2E5E94 y despiece final: suelta el
|     escolta $77FD6 (+8 en y) y sprite de retirada $2E6142.
|  4. Tropas de a pie (TaskHandler_08246c): al morir el padre saltan
|     ($13600) y escapan via $77F6A; TaskHandler_082480 mueve por pares
|     de tablas 2D anidadas $2E54A2[+0x5C][+0x5E] (dx,dy) + sprite
|     $2E547A[+0x5E]; $82514 cae desde el techo (-$38 en y, hereda +0x5C
|     del lider via $5E506); $82562 dispersa con offsets $2E55D6[fila]
|     y venga con angulo aleatorio ($5E9B6 & $F -> tripletas vel/angulo
|     en $2E560E[fila]); $82626 rebota (-1 en +0x38) con snd $75.
|  5. Escape final (TaskHandler_08267c): recorre lista de deltas +0x5C
|     (tripletas de 3 words) moviendo DOS escoltas $77F22 por tick
|     (bit-gate +0x70), viraje $4000/$28134, 6 saltos ($7A) y sale;
|     TaskHandler_082720: retirada con viraje $D000, HP $2BE2A0, sprite
|     $2E6908, fisica $27BC8 y rebote contra el jugador con knockback
|     (+0x28/+0x2A desde el atacante +0x50, y extra $F800 si tipo $20).
|
|  Notas de matching:
|  - Los bcc.w "colgantes" saltan al RTS interno (+6) de la isla C
|    siguiente -> 11 defsyms nuevos (ClrRamWordRts_081caa incluido).
|  - $82456/$82464 se resuelven con los defsyms TaskHandler_* existentes
|    (alias de Jsr5B6ThenJmpScheduler_082456 / JmpToScheduler_082464).
|  - 23 refs forward a los helpers pc-relativos del bloque $82C7C..$831DA
|    (proxima wave) via defsyms.
|  - Verificado byte a byte contra build/mslug_prom.bin (pre-link:
|    4 exactas + 19 solo-reloc de 23).
| ============================================================================

        .text

| ----------------------------------------------------------------------------
|  LeaSprite_081816  @ $081816  (6 B)
| ----------------------------------------------------------------------------
        .section .text.LeaSprite_081816, "ax", @progbits
        .global LeaSprite_081816
LeaSprite_081816:
        lea     0x2e308c.l,a0                   | +000

| ----------------------------------------------------------------------------
|  LeaSpriteAlt_081824  @ $081824  (14 B)
| ----------------------------------------------------------------------------
        .section .text.LeaSpriteAlt_081824, "ax", @progbits
        .global LeaSpriteAlt_081824
LeaSpriteAlt_081824:
        tst.b   0x7f(a6)                        | +000
        bne.w   LeaSprite_08183a                | +004
        lea     0x2e30ae.l,a0                   | +008

| ----------------------------------------------------------------------------
|  LeaSprite_08183a  @ $08183A  (6 B)
| ----------------------------------------------------------------------------
        .section .text.LeaSprite_08183a, "ax", @progbits
        .global LeaSprite_08183a
LeaSprite_08183a:
        lea     0x2e31dc.l,a0                   | +000

| ----------------------------------------------------------------------------
|  LeaSpriteAlt_081848  @ $081848  (14 B)
| ----------------------------------------------------------------------------
        .section .text.LeaSpriteAlt_081848, "ax", @progbits
        .global LeaSpriteAlt_081848
LeaSpriteAlt_081848:
        tst.b   0x7f(a6)                        | +000
        bne.w   LeaSprite_08185e                | +004
        lea     0x2e330a.l,a0                   | +008

| ----------------------------------------------------------------------------
|  LeaSprite_08185e  @ $08185E  (6 B)
| ----------------------------------------------------------------------------
        .section .text.LeaSprite_08185e, "ax", @progbits
        .global LeaSprite_08185e
LeaSprite_08185e:
        lea     0x2e3438.l,a0                   | +000

| ----------------------------------------------------------------------------
|  Sub_0008186C  @ $08186C  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0008186C, "ax", @progbits
        .global Sub_0008186C
Sub_0008186C:
        move.w  0x36(a6),d0                     | +000
        asr.w   #0x7,d0                         | +004
        btst    #0x0,0x3a(a6)                   | +006
        bne.w   SetTaskW_08187e                 | +00c
        neg.w   d0                              | +010

| ----------------------------------------------------------------------------
|  Sub_00081884  @ $081884  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00081884, "ax", @progbits
        .global Sub_00081884
Sub_00081884:
        move.w  0x36(a6),d0                     | +000
        asr.w   #0x4,d0                         | +004
        btst    #0x0,0x3a(a6)                   | +006
        bne.w   SetTaskW_081896                 | +00c
        neg.w   d0                              | +010

| ----------------------------------------------------------------------------
|  LeaPieceList_08189c  @ $08189C  (6 B)
| ----------------------------------------------------------------------------
        .section .text.LeaPieceList_08189c, "ax", @progbits
        .global LeaPieceList_08189c
LeaPieceList_08189c:
        lea     0x2e3c54.l,a1                   | +000

| ----------------------------------------------------------------------------
|  Sub_000818EC  @ $0818EC  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_000818EC, "ax", @progbits
        .global Sub_000818EC
Sub_000818EC:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_081902                    | +00c

| ----------------------------------------------------------------------------
|  TaskHandler_081908  @ $081908  (924 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_081908, "ax", @progbits
        .global TaskHandler_081908
TaskHandler_081908:
        move.w  #0x28,d0                        | +000
        jsr     0x2352.l                        | +004
        jsr     0x267e2.l                       | +00a
        lea     TaskHandler_082c54(pc),a1       | +010
        jsr     0x4ae.l                         | +014
        lea     TaskHandler_081db0(pc),a1       | +01a
        jsr     0x4ae.l                         | +01e
        jsr     0x5dd02.l                       | +024
        lea     TaskHandler_082052(pc),a1       | +02a
        jsr     0x4ae.l                         | +02e
        jsr     0x5dd02.l                       | +034
        lea     TaskHandler_082062(pc),a1       | +03a
        jsr     0x4ae.l                         | +03e
        jsr     0x5dd02.l                       | +044
        move.b  #0x6,0x5c(a6)                   | +04a
.L81958:
        lea     TaskHandler_081cd6(pc),a1       | +050
        jsr     0x4ae.l                         | +054
        jsr     0x5dd02.l                       | +05a
        subq.b  #0x1,0x5c(a6)                   | +060
        move.b  0x5c(a6),0x7c(a0)               | +064
        tst.b   0x5c(a6)                        | +06a
        bne.b   .L81958                         | +06e
        clr.b   0x7b(a6)                        | +070
        clr.w   0x5c(a6)                        | +074
        clr.b   0x72(a6)                        | +078
        clr.b   0x73(a6)                        | +07c
        clr.b   0x20(a6)                        | +080
        move.b  #0x3,0x7d(a6)                   | +084
        move.l  #0x2e3f76,0x48(a6)              | +08a
        move.w  #0x8000,d0                      | +092
        jsr     0x28134.l                       | +096
        andi.w  #0xffe3,0x38(a6)                | +09c
        ori.w   #0x14,0x38(a6)                  | +0a2
        move.w  #0x72,d1                        | +0a8
        jsr     0x236e.l                        | +0ac
        move.w  #0xf,0x1c(a6)                   | +0b2
        jsr     0x138fe.l                       | +0b8
        lea     0x2be016.l,a0                   | +0be
        jsr     0x799de.l                       | +0c4
        move.w  d0,0x66(a6)                     | +0ca
        jsr     0x5e0d4.l                       | +0ce
        move.l  a0,0x94(a6)                     | +0d4
.L819e0:
        jsr     Sub_00082F90(pc)                | +0d8
        move.w  0x5c(a6),d0                     | +0dc
        movea.l #0x2e541e,a0                    | +0e0
        lsl.w   #0x2,d0                         | +0e6
        movea.l (a0,d0.w),a0                    | +0e8
        cmpa.l  #0xffffffff,a0                  | +0ec
        beq.w   .L81a04                         | +0f2
        jsr     0x28cd4.l                       | +0f6
.L81a04:
        lea     .L81a0a(pc),a1                  | +0fc
        move.l  a1,(a6)                         | +100
.L81a0a:
        jsr     0x2783a.l                       | +102
        jsr     Sub_00082D12(pc)                | +108
        jsr     Sub_00082CB2(pc)                | +10c
        jsr     0x28d70.l                       | +110
        jsr     Sub_00082DAE(pc)                | +116
        bcc.w   .L81a2c                         | +11a
        lea     .L81a30(pc),a1                  | +11e
        move.l  a1,(a6)                         | +122
.L81a2c:
        bra.w   Sub_00081CAC                    | +124
.L81a30:
        clr.b   0x7e(a6)                        | +128
        move.w  0x5c(a6),d0                     | +12c
        movea.l #0x2e541e,a0                    | +130
        lsl.w   #0x2,d0                         | +136
        movea.l (a0,d0.w),a0                    | +138
        cmpa.l  #0xffffffff,a0                  | +13c
        beq.w   .L81a54                         | +142
        jsr     0x28cd4.l                       | +146
.L81a54:
        lea     .L81a5a(pc),a1                  | +14c
        move.l  a1,(a6)                         | +150
.L81a5a:
        jsr     0x2783a.l                       | +152
        jsr     Sub_00082D12(pc)                | +158
        lea     0x2e590a.l,a0                   | +15c
        jsr     Sub_00082C7C(pc)                | +162
        bcc.w   .L81a78                         | +166
        lea     .L81a82(pc),a1                  | +16a
        move.l  a1,(a6)                         | +16e
.L81a78:
        jsr     0x28d70.l                       | +170
        bra.w   Sub_00081CAC                    | +176
.L81a82:
        jsr     Sub_00082F90(pc)                | +17a
        lea     0x2be732.l,a0                   | +17e
        tst.b   0x73(a6)                        | +184
        beq.w   .L81a9a                         | +188
        lea     0x2be7b4.l,a0                   | +18c
.L81a9a:
        jsr     0x799de.l                       | +192
        move.w  d0,0x36(a6)                     | +198
        clr.w   0x28(a6)                        | +19c
        btst    #0x0,0x74(a6)                   | +1a0
        beq.w   .L81ae4                         | +1a6
        neg.w   0x36(a6)                        | +1aa
        move.w  #0xffe0,0x2c(a6)                | +1ae
        lea     0x2e5b30.l,a0                   | +1b4
        jsr     0x28cd4.l                       | +1ba
        lea     .L81ace(pc),a1                  | +1c0
        move.l  a1,(a6)                         | +1c4
.L81ace:
        move.w  0x36(a6),d0                     | +1c6
        cmp.w   0x28(a6),d0                     | +1ca
        blt.w   .L81ae0                         | +1ce
        lea     .L81b12(pc),a1                  | +1d2
        move.l  a1,(a6)                         | +1d6
.L81ae0:
        bra.w   .L81b22                         | +1d8
.L81ae4:
        move.w  #0x20,0x2c(a6)                  | +1dc
        lea     0x2e5b7a.l,a0                   | +1e2
        jsr     0x28cd4.l                       | +1e8
        lea     .L81afc(pc),a1                  | +1ee
        move.l  a1,(a6)                         | +1f2
.L81afc:
        move.w  0x36(a6),d0                     | +1f4
        cmp.w   0x28(a6),d0                     | +1f8
        blt.w   .L81b0e                         | +1fc
        lea     .L81b12(pc),a1                  | +200
        move.l  a1,(a6)                         | +204
.L81b0e:
        bra.w   .L81b22                         | +206
.L81b12:
        clr.w   0x2c(a6)                        | +20a
        move.w  0x36(a6),0x28(a6)               | +20e
        lea     .L81b22(pc),a1                  | +214
        move.l  a1,(a6)                         | +218
.L81b22:
        jsr     Sub_00082E04(pc)                | +21a
        bcc.w   .L81b30                         | +21e
        lea     .L81b4e(pc),a1                  | +222
        move.l  a1,(a6)                         | +226
.L81b30:
        jsr     0x27cee.l                       | +228
        jsr     Sub_00082D12(pc)                | +22e
        lea     0x2e58ea.l,a0                   | +232
        jsr     Sub_00082C7C(pc)                | +238
        jsr     0x28d70.l                       | +23c
        bra.w   Sub_00081CAC                    | +242
.L81b4e:
        btst    #0x0,0x74(a6)                   | +246
        beq.w   .L81b76                         | +24c
        move.w  #0x60,0x2c(a6)                  | +250
        lea     .L81b64(pc),a1                  | +256
        move.l  a1,(a6)                         | +25a
.L81b64:
        cmpi.w  #0x0,0x28(a6)                   | +25c
        blt.w   .L81b74                         | +262
        lea     .L81b94(pc),a1                  | +266
        move.l  a1,(a6)                         | +26a
.L81b74:
        bra.b   .L81b30                         | +26c
.L81b76:
        move.w  #0xffa0,0x2c(a6)                | +26e
        lea     .L81b82(pc),a1                  | +274
        move.l  a1,(a6)                         | +278
.L81b82:
        cmpi.w  #0x0,0x28(a6)                   | +27a
        blt.w   .L81b92                         | +280
        lea     .L81b94(pc),a1                  | +284
        move.l  a1,(a6)                         | +288
.L81b92:
        bra.b   .L81b30                         | +28a
.L81b94:
        clr.b   0x7e(a6)                        | +28c
        clr.w   0x2c(a6)                        | +290
        clr.w   0x28(a6)                        | +294
        move.w  0x5c(a6),d0                     | +298
        movea.l #0x2e541e,a0                    | +29c
        lsl.w   #0x2,d0                         | +2a2
        movea.l (a0,d0.w),a0                    | +2a4
        cmpa.l  #0xffffffff,a0                  | +2a8
        beq.w   .L81bc0                         | +2ae
        jsr     0x28cd4.l                       | +2b2
.L81bc0:
        lea     .L81bc6(pc),a1                  | +2b8
        move.l  a1,(a6)                         | +2bc
.L81bc6:
        jsr     0x2783a.l                       | +2be
        jsr     Sub_00082D12(pc)                | +2c4
        lea     0x2e592a.l,a0                   | +2c8
        jsr     Sub_00082C7C(pc)                | +2ce
        bcc.w   .L81be4                         | +2d2
        lea     .L819e0(pc),a1                  | +2d6
        move.l  a1,(a6)                         | +2da
.L81be4:
        jsr     0x28d70.l                       | +2dc
        bra.w   Sub_00081CAC                    | +2e2
        .global TaskHandler_081bee
TaskHandler_081bee:
        move.w  #0x1071,d0                      | +2e6
        jsr     0x2222.l                        | +2ea
        clr.b   0x72(a6)                        | +2f0
        clr.b   0x106ed3.l                      | +2f4
        move.b  #0x1,0x20(a6)                   | +2fa
        move.w  #0xb4,0x70(a6)                  | +300
        move.l  #0xffffffff,0x48(a6)            | +306
        move.b  #0x1,0x10a2d1.l                 | +30e
        lea     TaskHandler_082bb8(pc),a1       | +316
        jsr     0x4ae.l                         | +31a
        jsr     0x5dd02.l                       | +320
        move.w  0x5c(a6),d0                     | +326
        movea.l #0x2e541e,a0                    | +32a
        lsl.w   #0x2,d0                         | +330
        movea.l (a0,d0.w),a0                    | +332
        cmpa.l  #0xffffffff,a0                  | +336
        beq.w   .L81c4e                         | +33c
        jsr     0x28cd4.l                       | +340
.L81c4e:
        move.w  #0x1033,d0                      | +346
        jsr     0x2352.l                        | +34a
        lea     .L81c5e(pc),a1                  | +350
        move.l  a1,(a6)                         | +354
.L81c5e:
        jsr     0x2783a.l                       | +356
        jsr     Sub_00082CB2(pc)                | +35c
        jsr     0x28d70.l                       | +360
        subq.w  #0x1,0x70(a6)                   | +366
        cmpi.w  #0x3c,0x70(a6)                  | +36a
        bne.w   .L81c88                         | +370
        lea     0x2e5aee.l,a1                   | +374
        jsr     0x77c7e.l                       | +37a
.L81c88:
        cmpi.w  #0x6e,0x70(a6)                  | +380
        bne.w   .L81c9c                         | +386
        move.w  #0x1032,d0                      | +38a
        jsr     0x2352.l                        | +38e
.L81c9c:
        tst.w   0x70(a6)                        | +394
        bne.w   ClrRamWordRts_081caa            | +398

| ----------------------------------------------------------------------------
|  Sub_00081CAC  @ $081CAC  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00081CAC, "ax", @progbits
        .global Sub_00081CAC
Sub_00081CAC:
        jsr     Sub_00082E7A(pc)                | +000
        jsr     Sub_00082D70(pc)                | +004
        jsr     0x2870a.l                       | +008
        bcc.w   .L81cc4                         | +00e
        bclr    #0x3,0x13(a6)                   | +012
.L81cc4:
        jsr     0x28758.l                       | +018
        bcc.w   SetHandlerRts_081cd4            | +01e

| ----------------------------------------------------------------------------
|  TaskHandler_081cd6  @ $081CD6  (134 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_081cd6, "ax", @progbits
        .global TaskHandler_081cd6
TaskHandler_081cd6:
        move.w  #0x71,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0xd,0x1c(a6)                   | +00a
        jsr     0x138fe.l                       | +010
        lea     0x2be21e.l,a0                   | +016
        jsr     0x799de.l                       | +01c
        move.w  d0,0x66(a6)                     | +022
        move.b  0x7c(a6),d0                     | +026
        andi.w  #0x7,d0                         | +02a
        movea.l #0x2e542e,a0                    | +02e
        lsl.w   #0x2,d0                         | +034
        movea.l (a0,d0.w),a0                    | +036
        cmpa.l  #0xffffffff,a0                  | +03a
        beq.w   .L81d20                         | +040
        jsr     0x28cd4.l                       | +044
.L81d20:
        lea     .L81d26(pc),a1                  | +04a
        move.l  a1,(a6)                         | +04e
.L81d26:
        jsr     Sub_00082D40(pc)                | +050
        jsr     Sub_00082CC4(pc)                | +054
        jsr     0x28d70.l                       | +058
        jsr     0x2870a.l                       | +05e
        bcc.w   .L81d44                         | +064
        bclr    #0x3,0x13(a6)                   | +068
.L81d44:
        jsr     0x28758.l                       | +06e
        bcc.w   .L81d54                         | +074
        lea     TaskHandler_081d64(pc),a1       | +078
        move.l  a1,(a6)                         | +07c
.L81d54:
        jsr     Sub_00082EE0(pc)                | +07e
        bcc.w   SetHandlerRts_081d62            | +082

| ----------------------------------------------------------------------------
|  TaskHandler_081d64  @ $081D64  (68 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_081d64, "ax", @progbits
        .global TaskHandler_081d64
TaskHandler_081d64:
        jsr     Sub_000831DA(pc)                | +000
        bclr    #0x0,0x13(a6)                   | +004
        move.l  #0xffffffff,0x48(a6)            | +00a
        move.b  0x7c(a6),d0                     | +012
        andi.w  #0x7,d0                         | +016
        movea.l #0x2e5446,a0                    | +01a
        lsl.w   #0x2,d0                         | +020
        movea.l (a0,d0.w),a0                    | +022
        cmpa.l  #0xffffffff,a0                  | +026
        beq.w   .L81d9a                         | +02c
        jsr     0x28cd4.l                       | +030
.L81d9a:
        lea     .L81da0(pc),a1                  | +036
        move.l  a1,(a6)                         | +03a
.L81da0:
        jsr     Sub_00082D40(pc)                | +03c
        jsr     Sub_00082CC4(pc)                | +040

| ----------------------------------------------------------------------------
|  TaskHandler_081db0  @ $081DB0  (568 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_081db0, "ax", @progbits
        .global TaskHandler_081db0
TaskHandler_081db0:
        move.w  #0x71,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0xd,0x1c(a6)                   | +00a
        jsr     0x138fe.l                       | +010
        clr.b   0x7c(a6)                        | +016
        clr.b   0x20(a6)                        | +01a
        clr.w   0x80(a6)                        | +01e
        move.w  #0x38,0x82(a6)                  | +022
        move.w  #0x3,0x5c(a6)                   | +028
        lea     0x2be322.l,a0                   | +02e
        tst.b   0x73(a6)                        | +034
        beq.w   .L81df2                         | +038
        lea     0x2be3a4.l,a0                   | +03c
.L81df2:
        jsr     0x799de.l                       | +042
        move.w  d0,0x70(a6)                     | +048
.L81dfc:
        lea     0x2e5c98.l,a0                   | +04c
        jsr     0x28cd4.l                       | +052
        lea     .L81e0e(pc),a1                  | +058
        move.l  a1,(a6)                         | +05c
.L81e0e:
        jsr     Sub_00082CE2(pc)                | +05e
        jsr     Sub_00082D40(pc)                | +062
        jsr     0x28d70.l                       | +066
        jsr     Sub_00082F24(pc)                | +06c
        bcc.w   .L81e36                         | +070
        move.b  0x106f28.l,d1                   | +074
        andi.b  #0x7,d1                         | +07a
        bne.w   .L81e36                         | +07e
        add.w   d0,0x5c(a6)                     | +082
.L81e36:
        movea.l 0xc(a6),a0                      | +086
        btst    #0x2,0x72(a0)                   | +08a
        beq.w   .L81e4a                         | +090
        lea     .L81e4e(pc),a1                  | +094
        move.l  a1,(a6)                         | +098
.L81e4a:
        bra.w   Sub_0008202A                    | +09a
.L81e4e:
        lea     0x2be62e.l,a0                   | +09e
        tst.b   0x73(a6)                        | +0a4
        beq.w   .L81e62                         | +0a8
        lea     0x2be6b0.l,a0                   | +0ac
.L81e62:
        jsr     0x799de.l                       | +0b2
        move.w  d0,0x70(a6)                     | +0b8
        jsr     Sub_00083122(pc)                | +0bc
        lea     TaskHandler_082514(pc),a1       | +0c0
        jsr     0x4ae.l                         | +0c4
        jsr     0x5dd02.l                       | +0ca
        move.w  #0x1099,d0                      | +0d0
        jsr     0x2352.l                        | +0d4
        lea     .L81e90(pc),a1                  | +0da
        move.l  a1,(a6)                         | +0de
.L81e90:
        jsr     Sub_00082F24(pc)                | +0e0
        bcc.w   .L81eca                         | +0e4
        tst.b   0x73(a6)                        | +0e8
        beq.w   .L81eca                         | +0ec
        move.b  0x106f28.l,d1                   | +0f0
        andi.b  #0x7,d1                         | +0f6
        bne.w   .L81eca                         | +0fa
        add.w   d0,0x5c(a6)                     | +0fe
        jsr     0x5e3fc.l                       | +102
        bcc.w   .L81eca                         | +108
        lea     .L81ec6(pc),a1                  | +10c
        move.l  a1,(a6)                         | +110
        bra.w   .L81eca                         | +112
.L81ec6:
        jsr     Sub_00082F24(pc)                | +116
.L81eca:
        jsr     Sub_00082CE2(pc)                | +11a
        jsr     Sub_00082D40(pc)                | +11e
        jsr     0x28d70.l                       | +122
        subq.w  #0x1,0x70(a6)                   | +128
        cmpi.w  #0x0,0x70(a6)                   | +12c
        bgt.w   .L81ef2                         | +132
        move.b  #0x1,0x20(a6)                   | +136
        lea     .L81f0a(pc),a1                  | +13c
        move.l  a1,(a6)                         | +140
.L81ef2:
        cmpi.w  #0xf,0x70(a6)                   | +142
        bgt.w   .L81f06                         | +148
        movea.l 0xc(a6),a0                      | +14c
        bset    #0x5,0x72(a0)                   | +150
.L81f06:
        bra.w   Sub_0008202A                    | +156
.L81f0a:
        clr.b   0x20(a6)                        | +15a
        movea.l 0xc(a6),a0                      | +15e
        move.b  #0x1,0x7b(a0)                   | +162
        movea.l 0xc(a6),a0                      | +168
        bset    #0x5,0x72(a0)                   | +16c
        clr.b   0x21(a6)                        | +172
        lea     TaskHandler_082626(pc),a1       | +176
        jsr     0x4ae.l                         | +17a
        jsr     0x5dd02.l                       | +180
        move.w  0x5c(a6),0x5c(a0)               | +186
        lea     .L81f42(pc),a1                  | +18c
        move.l  a1,(a6)                         | +190
.L81f42:
        move.w  #0x2,0x70(a6)                   | +192
        jsr     Sub_00082CE2(pc)                | +198
        jsr     Sub_00082D40(pc)                | +19c
        jsr     0x28d70.l                       | +1a0
        tst.b   0x21(a6)                        | +1a6
        beq.w   .L81f64                         | +1aa
        lea     .L81f68(pc),a1                  | +1ae
        move.l  a1,(a6)                         | +1b2
.L81f64:
        bra.w   Sub_0008202A                    | +1b4
.L81f68:
        movea.l 0xc(a6),a0                      | +1b8
        bclr    #0x2,0x72(a0)                   | +1bc
        movea.l 0xc(a6),a0                      | +1c2
        bclr    #0x4,0x72(a0)                   | +1c6
        movea.l 0xc(a6),a0                      | +1cc
        bclr    #0x5,0x72(a0)                   | +1d0
        lea     0x2be322.l,a0                   | +1d6
        tst.b   0x73(a6)                        | +1dc
        beq.w   .L81f9a                         | +1e0
        lea     0x2be3a4.l,a0                   | +1e4
.L81f9a:
        jsr     0x799de.l                       | +1ea
        move.w  d0,0x70(a6)                     | +1f0
        jsr     Sub_00082F90(pc)                | +1f4
        bra.w   .L81dfc                         | +1f8
        .global TaskHandler_081fac
TaskHandler_081fac:
        movea.l 0xc(a6),a0                      | +1fc
        bclr    #0x2,0x72(a0)                   | +200
        movea.l 0xc(a6),a0                      | +206
        bclr    #0x4,0x72(a0)                   | +20a
        move.b  #0x1,0x20(a6)                   | +210
        lea     .L81fcc(pc),a1                  | +216
        move.l  a1,(a6)                         | +21a
.L81fcc:
        jsr     Sub_00082CE2(pc)                | +21c
        jsr     Sub_00082D40(pc)                | +220
        jsr     0x28d70.l                       | +224
        movea.l 0xc(a6),a0                      | +22a
        cmpi.b  #0x77,0x20(a0)                  | +22e
        bne.w   SetHandlerRts_081fee            | +234

| ----------------------------------------------------------------------------
|  TaskHandler_081ff0  @ $081FF0  (50 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_081ff0, "ax", @progbits
        .global TaskHandler_081ff0
TaskHandler_081ff0:
        lea     0x2e5aa6.l,a1                   | +000
        jsr     0x77c7e.l                       | +006
        lea     0x2e59fe.l,a1                   | +00c
        jsr     0x77c7e.l                       | +012
        lea     0x2e5d20.l,a0                   | +018
        jsr     0x28cd4.l                       | +01e
        lea     .L8201a(pc),a1                  | +024
        move.l  a1,(a6)                         | +028
.L8201a:
        jsr     Sub_00082CE2(pc)                | +02a
        jsr     Sub_00082D40(pc)                | +02e

| ----------------------------------------------------------------------------
|  Sub_0008202A  @ $08202A  (32 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0008202A, "ax", @progbits
        .global Sub_0008202A
Sub_0008202A:
        subq.w  #0x1,0x70(a6)                   | +000
        cmpi.w  #0x0,0x70(a6)                   | +004
        bgt.w   .L82042                         | +00a
        movea.l 0xc(a6),a0                      | +00e
        bset    #0x4,0x72(a0)                   | +012
.L82042:
        jsr     Sub_00082EE0(pc)                | +018
        bcc.w   SetHandlerRts_082050            | +01c

| ----------------------------------------------------------------------------
|  TaskHandler_082052  @ $082052  (936 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_082052, "ax", @progbits
        .global TaskHandler_082052
TaskHandler_082052:
        move.w  #0x4d,0x80(a6)                  | +000
        move.w  #0x2e,0x82(a6)                  | +006
        bra.w   .L82074                         | +00c
        .global TaskHandler_082062
TaskHandler_082062:
        move.b  #0x1,0x3a(a6)                   | +010
        move.w  #0xffc0,0x80(a6)                | +016
        move.w  #0x2a,0x82(a6)                  | +01c
.L82074:
        move.w  #0x71,d1                        | +022
        jsr     0x236e.l                        | +026
        move.w  #0xe,0x1c(a6)                   | +02c
        jsr     0x138fe.l                       | +032
        lea     0x2be19c.l,a0                   | +038
        jsr     0x799de.l                       | +03e
        move.w  d0,0x66(a6)                     | +044
        clr.b   0x7c(a6)                        | +048
        move.l  #0x2e3fca,0x48(a6)              | +04c
        lea     0x2be426.l,a0                   | +054
        tst.b   0x73(a6)                        | +05a
        beq.w   .L820ba                         | +05e
        lea     0x2be4a8.l,a0                   | +062
.L820ba:
        jsr     0x799de.l                       | +068
        move.w  d0,0x70(a6)                     | +06e
.L820c4:
        clr.w   0x5c(a6)                        | +072
        lea     0x2e5d2c.l,a0                   | +076
        jsr     0x28cd4.l                       | +07c
        lea     .L820da(pc),a1                  | +082
        move.l  a1,(a6)                         | +086
.L820da:
        jsr     Sub_00082CE2(pc)                | +088
        jsr     0x28d70.l                       | +08c
        jsr     Sub_00082EF8(pc)                | +092
        lea     0x2e5846.l,a0                   | +096
        movea.l #0xffffffff,a1                  | +09c
        jsr     0x772.l                         | +0a2
        bra.w   Sub_00082402                    | +0a8
        tst.b   0x73(a6)                        | +0ac
        bne.w   .L82160                         | +0b0
        move.w  #0x1e,0x70(a6)                  | +0b4
.L8210c:
        lea     0x2e5d2c.l,a0                   | +0ba
        jsr     0x28cd4.l                       | +0c0
        lea     .L8211e(pc),a1                  | +0c6
        move.l  a1,(a6)                         | +0ca
.L8211e:
        movea.l 0xc(a6),a0                      | +0cc
        bset    #0x0,0x72(a0)                   | +0d0
        jsr     Sub_00082CE2(pc)                | +0d6
        jsr     0x28d70.l                       | +0da
        subq.w  #0x1,0x70(a6)                   | +0e0
        cmpi.w  #0x0,0x70(a6)                   | +0e4
        bgt.w   .L8215c                         | +0ea
        addq.w  #0x1,0x5c(a6)                   | +0ee
        move.w  #0x1e,0x70(a6)                  | +0f2
        move.w  0x5c(a6),d0                     | +0f8
        andi.w  #0x1,d0                         | +0fc
        bne.w   .L8215c                         | +100
        lea     .L82160(pc),a1                  | +104
        move.l  a1,(a6)                         | +108
.L8215c:
        bra.w   Sub_00082402                    | +10a
.L82160:
        jsr     Sub_00082FCA(pc)                | +10e
        move.w  0x5c(a6),d0                     | +112
        movea.l #0x2e5832,a0                    | +116
        lsl.w   #0x2,d0                         | +11c
        movea.l (a0,d0.w),a0                    | +11e
        cmpa.l  #0xffffffff,a0                  | +122
        beq.w   .L82184                         | +128
        jsr     0x28cd4.l                       | +12c
.L82184:
        lea     .L8218a(pc),a1                  | +132
        move.l  a1,(a6)                         | +136
.L8218a:
        movea.l 0xc(a6),a0                      | +138
        bset    #0x0,0x72(a0)                   | +13c
        jsr     Sub_00082CE2(pc)                | +142
        jsr     0x28d70.l                       | +146
        bcc.w   .L821b8                         | +14c
        lea     .L8210c(pc),a1                  | +150
        move.l  a1,(a6)                         | +154
        cmpi.w  #0x4,0x5c(a6)                   | +156
        blt.w   .L821b8                         | +15c
        lea     .L821bc(pc),a1                  | +160
        move.l  a1,(a6)                         | +164
.L821b8:
        bra.w   Sub_00082402                    | +166
.L821bc:
        lea     0x2e5d2c.l,a0                   | +16a
        jsr     0x28cd4.l                       | +170
        lea     .L821ce(pc),a1                  | +176
        move.l  a1,(a6)                         | +17a
.L821ce:
        jsr     Sub_00082CE2(pc)                | +17c
        jsr     0x28d70.l                       | +180
        move.b  0x106f28.l,d0                   | +186
        andi.b  #0xf,d0                         | +18c
        bne.w   .L821f8                         | +190
        subq.w  #0x1,0x5c(a6)                   | +194
        tst.w   0x5c(a6)                        | +198
        bne.w   .L821f8                         | +19c
        lea     .L82344(pc),a1                  | +1a0
        move.l  a1,(a6)                         | +1a4
.L821f8:
        bra.w   Sub_00082402                    | +1a6
        lea     0x2e5e94.l,a0                   | +1aa
        jsr     0x28cd4.l                       | +1b0
        lea     .L8220e(pc),a1                  | +1b6
        move.l  a1,(a6)                         | +1ba
.L8220e:
        jsr     Sub_00082CE2(pc)                | +1bc
        jsr     0x28d70.l                       | +1c0
        bcc.w   .L82222                         | +1c6
        lea     .L82226(pc),a1                  | +1ca
        move.l  a1,(a6)                         | +1ce
.L82222:
        bra.w   Sub_00082402                    | +1d0
.L82226:
        lea     0x2be836.l,a0                   | +1d4
        tst.b   0x73(a6)                        | +1da
        beq.w   .L8223a                         | +1de
        lea     0x2be8b8.l,a0                   | +1e2
.L8223a:
        jsr     0x799de.l                       | +1e8
        move.b  d0,0x7f(a6)                     | +1ee
        lea     0x2be93a.l,a0                   | +1f2
        tst.b   0x73(a6)                        | +1f8
        beq.w   .L82258                         | +1fc
        lea     0x2be9bc.l,a0                   | +200
.L82258:
        jsr     0x799de.l                       | +206
        move.w  d0,0x70(a6)                     | +20c
        clr.w   0x5e(a6)                        | +210
        clr.b   0x20(a6)                        | +214
        lea     TaskHandler_0828ee(pc),a1       | +218
        jsr     0x4ae.l                         | +21c
        jsr     0x5dd02.l                       | +222
        lea     0x2e5f4a.l,a0                   | +228
        jsr     0x28cd4.l                       | +22e
        lea     .L8228c(pc),a1                  | +234
        move.l  a1,(a6)                         | +238
.L8228c:
        movea.l 0xc(a6),a0                      | +23a
        bset    #0x1,0x72(a0)                   | +23e
        jsr     Sub_00082CE2(pc)                | +244
        jsr     0x28d70.l                       | +248
        subq.w  #0x1,0x70(a6)                   | +24e
        cmpi.w  #0x0,0x70(a6)                   | +252
        bgt.w   .L82310                         | +258
        jsr     Sub_00083064(pc)                | +25c
        lea     0x2be93a.l,a0                   | +260
        tst.b   0x73(a6)                        | +266
        beq.w   .L822c6                         | +26a
        lea     0x2be9bc.l,a0                   | +26e
.L822c6:
        jsr     0x799de.l                       | +274
        move.w  d0,0x70(a6)                     | +27a
        subq.b  #0x1,0x7f(a6)                   | +27e
        cmpi.b  #0x0,0x7f(a6)                   | +282
        bgt.w   .L82310                         | +288
        lea     0x2be836.l,a0                   | +28c
        tst.b   0x73(a6)                        | +292
        beq.w   .L822f2                         | +296
        lea     0x2be8b8.l,a0                   | +29a
.L822f2:
        jsr     0x799de.l                       | +2a0
        move.b  d0,0x7f(a6)                     | +2a6
        addq.w  #0x1,0x5e(a6)                   | +2aa
        cmpi.w  #0x5,0x5e(a6)                   | +2ae
        bne.w   .L82310                         | +2b4
        lea     .L82314(pc),a1                  | +2b8
        move.l  a1,(a6)                         | +2bc
.L82310:
        bra.w   Sub_00082402                    | +2be
.L82314:
        move.b  #0x1,0x20(a6)                   | +2c2
        lea     0x2e60a0.l,a0                   | +2c8
        jsr     0x28cd4.l                       | +2ce
        lea     .L8232c(pc),a1                  | +2d4
        move.l  a1,(a6)                         | +2d8
.L8232c:
        jsr     Sub_00082CE2(pc)                | +2da
        jsr     0x28d70.l                       | +2de
        bcc.w   .L82340                         | +2e4
        lea     .L82344(pc),a1                  | +2e8
        move.l  a1,(a6)                         | +2ec
.L82340:
        bra.w   Sub_00082402                    | +2ee
.L82344:
        movea.l 0xc(a6),a0                      | +2f2
        bclr    #0x0,0x72(a0)                   | +2f6
        movea.l 0xc(a6),a0                      | +2fc
        bclr    #0x1,0x72(a0)                   | +300
        movea.l 0xc(a6),a0                      | +306
        bclr    #0x3,0x72(a0)                   | +30a
        lea     0x2be426.l,a0                   | +310
        tst.b   0x73(a6)                        | +316
        beq.w   .L82376                         | +31a
        lea     0x2be4a8.l,a0                   | +31e
.L82376:
        jsr     0x799de.l                       | +324
        move.w  d0,0x70(a6)                     | +32a
        jsr     Sub_00082F90(pc)                | +32e
        bra.w   .L820c4                         | +332
        .global TaskHandler_082388
TaskHandler_082388:
        movea.l 0xc(a6),a0                      | +336
        bclr    #0x0,0x72(a0)                   | +33a
        movea.l 0xc(a6),a0                      | +340
        bclr    #0x1,0x72(a0)                   | +344
        movea.l 0xc(a6),a0                      | +34a
        bclr    #0x3,0x72(a0)                   | +34e
        clr.w   d0                              | +354
        btst    #0x0,0x3a(a6)                   | +356
        beq.w   .L823b6                         | +35c
        move.w  #0x1,d0                         | +360
.L823b6:
        movea.l 0xc(a6),a0                      | +364
        bclr    d0,0x7d(a0)                     | +368
        move.b  #0x1,0x20(a6)                   | +36c
        move.l  #0xffffffff,0x48(a6)            | +372
        lea     0x77fd6.l,a1                    | +37a
        jsr     0x4ae.l                         | +380
        jsr     0x5dd02.l                       | +386
        addi.w  #0x8,0x24(a0)                   | +38c
        lea     0x2e6142.l,a0                   | +392
        jsr     0x28cd4.l                       | +398
        lea     .L823f6(pc),a1                  | +39e
        move.l  a1,(a6)                         | +3a2
.L823f6:
        jsr     Sub_00082CE2(pc)                | +3a4

| ----------------------------------------------------------------------------
|  Sub_00082402  @ $082402  (76 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00082402, "ax", @progbits
        .global Sub_00082402
Sub_00082402:
        subq.w  #0x1,0x70(a6)                   | +000
        cmpi.w  #0x0,0x70(a6)                   | +004
        bgt.w   .L8241a                         | +00a
        movea.l 0xc(a6),a0                      | +00e
        bset    #0x3,0x72(a0)                   | +012
.L8241a:
        jsr     0x2870a.l                       | +018
        bcc.w   .L82436                         | +01e
        bclr    #0x3,0x13(a6)                   | +022
        lea     0x5e766.l,a0                    | +028
        jsr     0x5e770.l                       | +02e
.L82436:
        jsr     Sub_00082EE0(pc)                | +034
        bcc.w   .L82444                         | +038
        lea     TaskHandler_082388(pc),a1       | +03c
        move.l  a1,(a6)                         | +040
.L82444:
        jsr     0x28758.l                       | +042
        bcc.w   SetHandlerRts_082454            | +048

| ----------------------------------------------------------------------------
|  TaskHandler_08246c  @ $08246C  (160 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08246c, "ax", @progbits
        .global TaskHandler_08246c
TaskHandler_08246c:
        jsr     0x13600.l                       | +000
        move.l  #0xffffffff,0x48(a6)            | +006
        jmp     0x77f6a.l                       | +00e
        .global TaskHandler_082480
TaskHandler_082480:
        move.w  #0x74,d1                        | +014
        jsr     0x236e.l                        | +018
        lea     0x2e54a2.l,a0                   | +01e
        move.w  0x5c(a6),d0                     | +024
        lsl.w   #0x2,d0                         | +028
        movea.l (a0,d0.w),a0                    | +02a
        move.w  0x5e(a6),d0                     | +02e
        lsl.w   #0x2,d0                         | +032
        move.w  (a0,d0.w),d1                    | +034
        move.w  0x2(a0,d0.w),d2                 | +038
        move.w  d1,0x80(a6)                     | +03c
        move.w  d2,0x82(a6)                     | +040
        move.w  0x5e(a6),d0                     | +044
        movea.l #0x2e547a,a0                    | +048
        lsl.w   #0x2,d0                         | +04e
        movea.l (a0,d0.w),a0                    | +050
        cmpa.l  #0xffffffff,a0                  | +054
        beq.w   .L824d0                         | +05a
        jsr     0x28cd4.l                       | +05e
.L824d0:
        lea     .L824d6(pc),a1                  | +064
        move.l  a1,(a6)                         | +068
.L824d6:
        jsr     0x5e506.l                       | +06a
        move.w  0x80(a6),d1                     | +070
        move.w  0x82(a6),d2                     | +074
        add.w   d1,0x22(a6)                     | +078
        add.w   d2,0x24(a6)                     | +07c
        jsr     0x28d70.l                       | +080
        bcc.w   .L824fc                         | +086
        lea     TaskHandler_082456(pc),a1       | +08a
        move.l  a1,(a6)                         | +08e
.L824fc:
        movea.l #0xffffffff,a0                  | +090
        jsr     0x5dd56.l                       | +096
        bcc.w   SetHandlerRts_082512            | +09c

| ----------------------------------------------------------------------------
|  TaskHandler_082514  @ $082514  (70 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_082514, "ax", @progbits
        .global TaskHandler_082514
TaskHandler_082514:
        jsr     0x5e1aa.l                       | +000
        move.b  d0,0x7a(a6)                     | +006
        move.w  #0x74,d1                        | +00a
        jsr     0x236e.l                        | +00e
        lea     0x2e62fc.l,a0                   | +014
        jsr     0x28cd4.l                       | +01a
        lea     .L8253a(pc),a1                  | +020
        move.l  a1,(a6)                         | +024
.L8253a:
        jsr     0x5e506.l                       | +026
        move.w  0x5c(a0),0x5c(a6)               | +02c
        subi.w  #0x38,0x24(a6)                  | +032
        jsr     0x28d70.l                       | +038
        jsr     Sub_00082EE0(pc)                | +03e
        bcc.w   SetHandlerRts_082560            | +042

| ----------------------------------------------------------------------------
|  TaskHandler_082562  @ $082562  (188 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_082562, "ax", @progbits
        .global TaskHandler_082562
TaskHandler_082562:
        lea     0x2e55d6.l,a0                   | +000
        move.w  0x5c(a6),d0                     | +006
        lsl.w   #0x3,d0                         | +00a
        move.w  0x5e(a6),d1                     | +00c
        add.w   d1,d1                           | +010
        add.w   d1,d0                           | +012
        move.w  (a0,d0.w),d2                    | +014
        add.w   d2,0x22(a6)                     | +018
        subi.w  #0x18,0x24(a6)                  | +01c
        jsr     0x5e9b6.l                       | +022
        andi.w  #0xf,d0                         | +028
        move.w  d0,d1                           | +02c
        add.w   d0,d0                           | +02e
        add.w   d1,d0                           | +030
        add.w   d0,d0                           | +032
        lea     0x2e560e.l,a1                   | +034
        move.w  0x5c(a6),d1                     | +03a
        lsl.w   #0x2,d1                         | +03e
        movea.l (a1,d1.w),a0                    | +040
        move.w  (a0,d0.w),0x28(a6)              | +044
        move.w  0x2(a0,d0.w),0x2e(a6)           | +04a
        move.w  0x4(a0,d0.w),0x2a(a6)           | +050
        neg.w   0x2a(a6)                        | +056
        neg.w   0x2e(a6)                        | +05a
        move.w  #0x76,d1                        | +05e
        jsr     0x236e.l                        | +062
        move.w  #0xc000,d0                      | +068
        jsr     0x28134.l                       | +06c
        andi.w  #0xffe3,0x38(a6)                | +072
        ori.w   #0x14,0x38(a6)                  | +078
        lea     0x2e69e4.l,a0                   | +07e
        jsr     0x28cd4.l                       | +084
        lea     .L825f2(pc),a1                  | +08a
        move.l  a1,(a6)                         | +08e
.L825f2:
        jsr     0x27d50.l                       | +090
        jsr     0x28d70.l                       | +096
        bcc.w   .L82608                         | +09c
        lea     TaskHandler_082456(pc),a1       | +0a0
        move.l  a1,(a6)                         | +0a4
.L82608:
        movea.l #0xffffffff,a0                  | +0a6
        lea     0x2e5416.l,a0                   | +0ac
        jsr     0x5dd56.l                       | +0b2
        bcc.w   SetHandlerRts_082624            | +0b8

| ----------------------------------------------------------------------------
|  TaskHandler_082626  @ $082626  (78 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_082626, "ax", @progbits
        .global TaskHandler_082626
TaskHandler_082626:
        move.w  #0x75,d1                        | +000
        jsr     0x236e.l                        | +004
        lea     0x2e6152.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L82642(pc),a1                  | +016
        move.l  a1,(a6)                         | +01a
.L82642:
        jsr     0x5e506.l                       | +01c
        subq.w  #0x1,0x38(a6)                   | +022
        jsr     0x28d70.l                       | +026
        bcc.w   .L82666                         | +02c
        movea.l 0xc(a6),a0                      | +030
        move.b  #0x1,0x21(a0)                   | +034
        lea     TaskHandler_082464(pc),a1       | +03a
        move.l  a1,(a6)                         | +03e
.L82666:
        jsr     0x283d8.l                       | +040
        jsr     Sub_00082EE0(pc)                | +046
        bcc.w   SetHandlerRts_08267a            | +04a

| ----------------------------------------------------------------------------
|  TaskHandler_08267c  @ $08267C  (440 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08267c, "ax", @progbits
        .global TaskHandler_08267c
TaskHandler_08267c:
        clr.w   0x70(a6)                        | +000
        lea     0x2e571a.l,a0                   | +004
        move.w  0x5c(a6),d0                     | +00a
        lsl.w   #0x2,d0                         | +00e
        move.l  (a0,d0.w),0x5c(a6)              | +010
        move.b  #0x6,0x7a(a6)                   | +016
        lea     .L8269e(pc),a1                  | +01c
        move.l  a1,(a6)                         | +020
.L8269e:
        btst    #0x0,0x70(a6)                   | +022
        bne.w   .L8271a                         | +028
        move.w  #0x4000,d0                      | +02c
        jsr     0x28134.l                       | +030
        andi.w  #0xffe3,0x38(a6)                | +036
        ori.w   #0x14,0x38(a6)                  | +03c
        subq.b  #0x1,0x7a(a6)                   | +042
        bne.w   .L826cc                         | +046
        lea     TaskHandler_082464(pc),a1       | +04a
        move.l  a1,(a6)                         | +04e
.L826cc:
        lea     0x77f22.l,a1                    | +050
        jsr     0x4ae.l                         | +056
        jsr     0x5dd02.l                       | +05c
        movea.l 0x5c(a6),a1                     | +062
        move.w  (a1),d0                         | +066
        move.w  0x4(a1),d1                      | +068
        add.w   d0,0x22(a0)                     | +06c
        add.w   d1,0x24(a0)                     | +070
        lea     0x77f22.l,a1                    | +074
        jsr     0x4ae.l                         | +07a
        jsr     0x5dd02.l                       | +080
        movea.l 0x5c(a6),a1                     | +086
        move.w  0x2(a1),d0                      | +08a
        move.w  0x4(a1),d1                      | +08e
        add.w   d0,0x22(a0)                     | +092
        add.w   d1,0x24(a0)                     | +096
        addq.l  #0x6,0x5c(a6)                   | +09a
.L8271a:
        addq.b  #0x1,0x70(a6)                   | +09e
        rts                                     | +0a2
        .global TaskHandler_082720
TaskHandler_082720:
        move.w  #0x73,d1                        | +0a4
        jsr     0x236e.l                        | +0a8
        move.w  #0xd000,d0                      | +0ae
        jsr     0x28134.l                       | +0b2
        andi.w  #0xffe3,0x38(a6)                | +0b8
        ori.w   #0x14,0x38(a6)                  | +0be
        lea     0x2be2a0.l,a0                   | +0c4
        jsr     0x799de.l                       | +0ca
        move.w  d0,0x66(a6)                     | +0d0
        lea     0x2e6908.l,a0                   | +0d4
        jsr     0x28cd4.l                       | +0da
        lea     .L82762(pc),a1                  | +0e0
        move.l  a1,(a6)                         | +0e4
.L82762:
        jsr     0x27bc8.l                       | +0e6
        bcc.w   .L82772                         | +0ec
        lea     TaskHandler_082884(pc),a1       | +0f0
        move.l  a1,(a6)                         | +0f4
.L82772:
        jsr     0x28d70.l                       | +0f6
        jsr     0x2870a.l                       | +0fc
        bcc.w   ParaSquad_FrameTail_0827fe      | +102
        bclr    #0x3,0x13(a6)                   | +106
        cmpi.b  #0x20,0x58(a6)                  | +10c
        beq.w   .L8279c                         | +112
        cmpi.b  #0x1,0x58(a6)                   | +116
        bne.w   ParaSquad_FrameTail_0827fe      | +11c
.L8279c:
        cmpi.w  #0x20,0x58(a6)                  | +120
        bne.w   .L827c6                         | +126
        movea.l 0x50(a6),a0                     | +12a
        move.w  0x28(a0),d0                     | +12e
        add.w   0x28(a6),d0                     | +132
        move.w  d0,0x28(a6)                     | +136
        move.w  #0xf800,d0                      | +13a
        add.w   0x2a(a6),d0                     | +13e
        move.w  d0,0x2a(a6)                     | +142
        bra.w   .L827e2                         | +146
.L827c6:
        movea.l 0x50(a6),a0                     | +14a
        move.w  0x28(a0),d0                     | +14e
        add.w   0x28(a6),d0                     | +152
        move.w  d0,0x28(a6)                     | +156
        move.w  0x2a(a0),d0                     | +15a
        add.w   0x2a(a6),d0                     | +15e
        move.w  d0,0x2a(a6)                     | +162
.L827e2:
        bclr    #0x0,0x13(a6)                   | +166
        lea     0x2be2a0.l,a0                   | +16c
        jsr     0x799de.l                       | +172
        move.w  d0,0x66(a6)                     | +178
        lea     TaskHandler_08283c(pc),a1       | +17c
        move.l  a1,(a6)                         | +180
| Cola de frame comun del modulo: tambien la usan los handlers de Wave FFF
| ($82880/$828CE saltan aqui con bra.w). Promovido a global cross-file.
        .global ParaSquad_FrameTail_0827fe
ParaSquad_FrameTail_0827fe:
        jsr     0x283d8.l                       | +182
        btst    #0x1,0x13(a6)                   | +188
        beq.w   .L82814                         | +18e
        lea     TaskHandler_08246c(pc),a1       | +192
        move.l  a1,(a6)                         | +196
.L82814:
        jsr     0x28758.l                       | +198
        bcc.w   .L82824                         | +19e
        lea     TaskHandler_0828e0(pc),a1       | +1a2
        move.l  a1,(a6)                         | +1a6
.L82824:
        movea.l #0xffffffff,a0                  | +1a8
        jsr     0x5dd56.l                       | +1ae
        bcc.w   SetHandlerRts_08283a            | +1b4
