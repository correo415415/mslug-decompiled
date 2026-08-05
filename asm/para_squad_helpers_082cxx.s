| =============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave FFF — Helpers y handlers de escape del escuadrón paracaidista
|  Región: $08283C..$08325A  (2,456 B, 36 entradas, 17 huecos cerrados)
| =============================================================================
|
|  Este módulo cierra la región de helpers pc-relativos y handlers de escape
|  referenciados por el módulo del escuadrón (asm/para_squad_module_0818xx.s,
|  Wave EEE). Estructura:
|
|   * $8283C..$82924 — continuación del escape: TaskHandler_08283c (post-
|     escape, retorna a la cola de frame ParaSquad_FrameTail_0827fe del módulo
|     EEE via bra.w), TaskHandler_082884 (rama alternativa, spawnea pieza
|     $77E10), transición $828D2 (marca +$48=-1 y jmp TaskHandler_056204),
|     TaskHandler_0828e0 (snd $109F + bra.w TaskHandler_08246c) y
|     TaskHandler_0828ee (hijo: snd $193, sprite $2E6728, hereda +$5E).
|   * $8292C..$82C74 — handlers de los hijos: disparo de venganza con jitter
|     aleatorio ($5E9B6), respawn desde tabla $2E588E con clamp de suelo
|     ($440D0), salto balístico con seno/coseno ($2C07AC/$2C072C), caída en
|     paracaídas (sprite $2E6AEA), variante $2E6992, y fila aleatoria
|     ($2E596A/$2E594A, HP $800, jmp $6DCE0); TaskHandler_082c54 recorta
|     +$45 del padre según x>$120 y encola via jmp FUN_00000518.
|   * $82C7C..$82F8A — helpers jsr-pc del módulo EEE: ring de historia de
|     anclaje +$7E (mod 16), copias de anclaje del padre, relay de flags
|     bit6/7<->bit0(+$5A), dificultad -> +$73=$FF (tablas $2BE098/$2BE11A),
|     probes de bounds (caen en islas SetXN_*/ClearXN_* de ccr_helpers.c),
|     máquina de flags +$72, y puntería vertical con clamp 0..6.
|   * $82F90..$8325A — spawners: reacquire de target ($5E338/$5E0D4), spawn
|     de pieza $77E10 + escolta TaskHandler_082720 (offsets $2E5852/$2E586A),
|     spawn de hijos $8292C+$82A66 (snd $10FF, tablas $2E58AE/$2E58C6/$2E589A),
|     bucle de 10 tropas TaskHandler_082480, 3 oleadas TaskHandler_082562,
|     flag global $10A2D1, montaje de pares $2E5A22/$2E597A ($77C7E) y
|     escolta $77FD6 con jitter; LeaList_083254 es el prefijo (lea $2E5AB8,a1)
|     del thunk JsrAbsThunk_08325a.
|
|  Los saltos bcc.w colgantes al final de cada handler apuntan al RTS interno
|  (+6) de la isla C SetTaskHandler_* contigua (idiom "SetHandlerRts").
|
|  Verificación: cada sección .text.<Sym> se coloca en su dirección CPU
|  absoluta y reensambla byte-exacta contra build/mslug_prom.bin
|  (MD5 816b3f74c76b3373993407615f1850fe).
| =============================================================================

        .text

| ----------------------------------------------------------------------------
|  TaskHandler_08283c  @ $08283C  (72 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08283c, "ax", @progbits
        .global TaskHandler_08283c
TaskHandler_08283c:
        bset    #0x4,0x6b(a6)                   | +000
        move.w  #0xffc0,0x2e(a6)                | +006
        lea     0x2e6940.l,a0                   | +00c
        jsr     0x28cd4.l                       | +012
        lea     .L8285a(pc),a1                  | +018
        move.l  a1,(a6)                         | +01c
.L8285a:
        jsr     0x27d50.l                       | +01e
        bcc.w   .L8286a                         | +024
        lea     TaskHandler_082884(pc),a1       | +028
        move.l  a1,(a6)                         | +02c
.L8286a:
        jsr     0x28d70.l                       | +02e
        jsr     0x2870a.l                       | +034
        bcc.w   .L82880                         | +03a
        bclr    #0x3,0x13(a6)                   | +03e
.L82880:
        bra.w   ParaSquad_FrameTail_0827fe      | +044

| ----------------------------------------------------------------------------
|  TaskHandler_082884  @ $082884  (92 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_082884, "ax", @progbits
        .global TaskHandler_082884
TaskHandler_082884:
        lea     0x77e10.l,a1                    | +000
        jsr     0x4ae.l                         | +006
        jsr     0x5dd02.l                       | +00c
        lea     0x2e6956.l,a0                   | +012
        jsr     0x28cd4.l                       | +018
        lea     .L828a8(pc),a1                  | +01e
        move.l  a1,(a6)                         | +022
.L828a8:
        jsr     0x2783a.l                       | +024
        jsr     0x28d70.l                       | +02a
        bcc.w   .L828be                         | +030
        lea     .L828d2(pc),a1                  | +034
        move.l  a1,(a6)                         | +038
.L828be:
        jsr     0x2870a.l                       | +03a
        bcc.w   .L828ce                         | +040
        bclr    #0x3,0x13(a6)                   | +044
.L828ce:
        bra.w   ParaSquad_FrameTail_0827fe      | +04a
.L828d2:
        move.l  #0xffffffff,0x48(a6)            | +04e
        jmp     0x56204.l                       | +056

| ----------------------------------------------------------------------------
|  TaskHandler_0828e0  @ $0828E0  (14 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0828e0, "ax", @progbits
        .global TaskHandler_0828e0
TaskHandler_0828e0:
        move.w  #0x109f,d0                      | +000
        jsr     0x2352.l                        | +004
        bra.w   TaskHandler_08246c              | +00a

| ----------------------------------------------------------------------------
|  TaskHandler_0828ee  @ $0828EE  (54 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0828ee, "ax", @progbits
        .global TaskHandler_0828ee
TaskHandler_0828ee:
        move.w  #0x193,d1                       | +000
        jsr     0x236e.l                        | +004
        lea     0x2e6728.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L8290a(pc),a1                  | +016
        move.l  a1,(a6)                         | +01a
.L8290a:
        jsr     0x5e506.l                       | +01c
        move.w  0x5e(a0),0x5e(a6)               | +022
        jsr     0x28d70.l                       | +028
        jsr     Sub_00082EE0(pc)                | +02e
        bcc.w   SetHandlerRts_08292a            | +032

| ----------------------------------------------------------------------------
|  TaskHandler_08292c  @ $08292C  (120 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08292c, "ax", @progbits
        .global TaskHandler_08292c
TaskHandler_08292c:
        move.w  #0xd2,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0xc000,d0                      | +00a
        jsr     0x28134.l                       | +00e
        andi.w  #0xffe3,0x38(a6)                | +014
        ori.w   #0x14,0x38(a6)                  | +01a
        jsr     0x5e9b6.l                       | +020
        andi.w  #0x7f,d0                        | +026
        subi.w  #0x40,d0                        | +02a
        add.w   d0,0x28(a6)                     | +02e
        jsr     0x5e9b6.l                       | +032
        andi.w  #0x3f,d0                        | +038
        add.w   d0,0x2a(a6)                     | +03c
        lea     0x2ee4e8.l,a0                   | +040
        jsr     0x28cd4.l                       | +046
        lea     .L8297e(pc),a1                  | +04c
        move.l  a1,(a6)                         | +050
.L8297e:
        jsr     0x27bc8.l                       | +052
        bcc.w   .L8298e                         | +058
        lea     TaskHandler_082456(pc),a1       | +05c
        move.l  a1,(a6)                         | +060
.L8298e:
        jsr     0x28d70.l                       | +062
        movea.l #0xffffffff,a0                  | +068
        jsr     0x5dd56.l                       | +06e
        bcc.w   SetHandlerRts_0829aa            | +074

| ----------------------------------------------------------------------------
|  TaskHandler_0829ac  @ $0829AC  (178 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0829ac, "ax", @progbits
        .global TaskHandler_0829ac
TaskHandler_0829ac:
        move.w  #0x1b4,d1                       | +000
        jsr     0x236e.l                        | +004
        move.w  0x5c(a6),d0                     | +00a
        add.w   d0,d0                           | +00e
        lea     0x2e588e.l,a1                   | +010
        move.w  (a1,d0.w),d0                    | +016
        btst    #0x0,0x3a(a6)                   | +01a
        beq.w   .L829d2                         | +020
        neg.w   d0                              | +024
.L829d2:
        add.w   0x5e(a6),d0                     | +026
        move.w  d0,0x22(a6)                     | +02a
        move.w  #0xe4c,d0                       | +02e
        move.w  #0x68,d1                        | +032
        jsr     0x440d0.l                       | +036
        move.w  d1,0x24(a6)                     | +03c
        jsr     0x5e9b6.l                       | +040
        andi.w  #0x7,d0                         | +046
        subq.w  #0x4,d0                         | +04a
        add.w   d0,0x22(a6)                     | +04c
        jsr     0x5e9b6.l                       | +050
        andi.w  #0x3,d0                         | +056
        sub.w   d0,0x24(a6)                     | +05a
        move.w  #0x8000,d0                      | +05e
        jsr     0x28134.l                       | +062
        andi.w  #0xffe3,0x38(a6)                | +068
        ori.w   #0x14,0x38(a6)                  | +06e
        lea     0x2e687e.l,a0                   | +074
        jsr     0x28cd4.l                       | +07a
        lea     .L82a32(pc),a1                  | +080
        move.l  a1,(a6)                         | +084
.L82a32:
        jsr     0x2783a.l                       | +086
        jsr     0x28d70.l                       | +08c
        bcc.w   .L82a48                         | +092
        lea     TaskHandler_082456(pc),a1       | +096
        move.l  a1,(a6)                         | +09a
.L82a48:
        jsr     0x283d8.l                       | +09c
        movea.l #0xffffffff,a0                  | +0a2
        jsr     0x5dd56.l                       | +0a8
        bcc.w   SetHandlerRts_082a64            | +0ae

| ----------------------------------------------------------------------------
|  TaskHandler_082a66  @ $082A66  (216 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_082a66, "ax", @progbits
        .global TaskHandler_082a66
TaskHandler_082a66:
        move.w  0x34(a6),d3                     | +000
        add.w   d3,d3                           | +004
        move.w  #0x2000,d0                      | +006
        lea     0x2c07ac.l,a1                   | +00a
        lea     0x2c072c.l,a2                   | +010
        move.w  (a1,d3.w),d1                    | +016
        move.w  (a2,d3.w),d2                    | +01a
        muls.w  d0,d1                           | +01e
        muls.w  d0,d2                           | +020
        asr.l   #0x8,d1                         | +022
        asr.l   #0x8,d2                         | +024
        btst    #0x0,0x3a(a6)                   | +026
        beq.w   .L82a98                         | +02c
        neg.w   d1                              | +030
.L82a98:
        move.w  d1,0x28(a6)                     | +032
        move.w  d2,0x2a(a6)                     | +036
        clr.w   0x34(a6)                        | +03a
        move.w  0x22(a6),0x5e(a6)               | +03e
        move.w  #0x2,d1                         | +044
        jsr     0x236e.l                        | +048
        bset    #0x4,0x6b(a6)                   | +04e
        move.w  #0xd000,d0                      | +054
        jsr     0x28134.l                       | +058
        andi.w  #0xffe3,0x38(a6)                | +05e
        ori.w   #0x14,0x38(a6)                  | +064
        lea     0x2e68f0.l,a0                   | +06a
        jsr     0x28cd4.l                       | +070
        lea     .L82ae2(pc),a1                  | +076
        move.l  a1,(a6)                         | +07a
.L82ae2:
        jsr     0x27cee.l                       | +07c
        bcc.w   .L82af4                         | +082
        lea     0x31d26.l,a1                    | +086
        move.l  a1,(a6)                         | +08c
.L82af4:
        jsr     0x28d70.l                       | +08e
        move.w  #0xe4c,d0                       | +094
        move.w  #0x68,d1                        | +098
        jsr     0x440d0.l                       | +09c
        cmp.w   0x24(a6),d1                     | +0a2
        bgt.w   .L82b16                         | +0a6
        lea     TaskHandler_0829ac(pc),a1       | +0aa
        move.l  a1,(a6)                         | +0ae
.L82b16:
        jsr     0x283d8.l                       | +0b0
        btst    #0x1,0x13(a6)                   | +0b6
        beq.w   .L82b2e                         | +0bc
        lea     0x31d26.l,a1                    | +0c0
        move.l  a1,(a6)                         | +0c6
.L82b2e:
        movea.l #0xffffffff,a0                  | +0c8
        jsr     0x5dd56.l                       | +0ce
        bcc.w   SetHandlerRts_082b44            | +0d4

| ----------------------------------------------------------------------------
|  TaskHandler_082b46  @ $082B46  (106 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_082b46, "ax", @progbits
        .global TaskHandler_082b46
TaskHandler_082b46:
        move.w  #0x76,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0xd000,d0                      | +00a
        jsr     0x28134.l                       | +00e
        andi.w  #0xffe3,0x38(a6)                | +014
        ori.w   #0x1c,0x38(a6)                  | +01a
        move.w  #0x10,0x28(a6)                  | +020
        move.w  #0xc0,0x2a(a6)                  | +026
        lea     0x2e6aea.l,a0                   | +02c
        jsr     0x28cd4.l                       | +032
        lea     .L82b84(pc),a1                  | +038
        move.l  a1,(a6)                         | +03c
.L82b84:
        jsr     0x27cee.l                       | +03e
        jsr     0x28d70.l                       | +044
        bcc.w   .L82b9a                         | +04a
        lea     TaskHandler_082456(pc),a1       | +04e
        move.l  a1,(a6)                         | +052
.L82b9a:
        movea.l #0xffffffff,a0                  | +054
        lea     0x2e5416.l,a0                   | +05a
        jsr     0x5dd56.l                       | +060
        bcc.w   SetHandlerRts_082bb6            | +066

| ----------------------------------------------------------------------------
|  TaskHandler_082bb8  @ $082BB8  (72 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_082bb8, "ax", @progbits
        .global TaskHandler_082bb8
TaskHandler_082bb8:
        move.w  #0xc000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x1c,0x38(a6)                  | +010
        lea     0x2e6992.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
        lea     .L82be0(pc),a1                  | +022
        move.l  a1,(a6)                         | +026
.L82be0:
        jsr     0x2783a.l                       | +028
        jsr     0x28d70.l                       | +02e
        bcc.w   .L82bf6                         | +034
        lea     TaskHandler_082464(pc),a1       | +038
        move.l  a1,(a6)                         | +03c
.L82bf6:
        jsr     0x5e45a.l                       | +03e
        bcc.w   SetHandlerRts_082c06            | +044

| ----------------------------------------------------------------------------
|  TaskHandler_082c08  @ $082C08  (108 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_082c08, "ax", @progbits
        .global TaskHandler_082c08
TaskHandler_082c08:
        jsr     0x5e9b6.l                       | +000
        andi.w  #0x7,d0                         | +006
        move.w  d0,0x5c(a6)                     | +00a
        add.w   d0,d0                           | +00e
        lea     0x2e596a.l,a0                   | +010
        move.w  (a0,d0.w),d1                    | +016
        jsr     0x236e.l                        | +01a
        move.w  0x5c(a6),d0                     | +020
        movea.l #0x2e594a,a0                    | +024
        lsl.w   #0x2,d0                         | +02a
        movea.l (a0,d0.w),a0                    | +02c
        cmpa.l  #0xffffffff,a0                  | +030
        beq.w   .L82c48                         | +036
        jsr     0x28cd4.l                       | +03a
.L82c48:
        move.w  #0x800,0x36(a6)                 | +040
        jmp     0x6dce0.l                       | +046
        .global TaskHandler_082c54
TaskHandler_082c54:
        movea.l 0xc(a6),a0                      | +04c
        cmpi.w  #0x120,0x22(a0)                 | +050
        bgt.w   .L82c6e                         | +056
        clr.b   0x45(a0)                        | +05a
        jmp     0x518.l                         | +05e
        rts                                     | +064
.L82c6e:
        move.b  #0x2,0x45(a0)                   | +066

| ----------------------------------------------------------------------------
|  Sub_00082C7C  @ $082C7C  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00082C7C, "ax", @progbits
        .global Sub_00082C7C
Sub_00082C7C:
        move.b  0x7e(a6),d0                     | +000
        addq.b  #0x1,0x7e(a6)                   | +004
        andi.w  #0xf,d0                         | +008
        add.w   d0,d0                           | +00c
        move.w  0x22(a6),0x80(a6)               | +00e
        move.w  0x24(a6),d1                     | +014
        add.w   (a0,d0.w),d1                    | +018
        move.w  d1,0x82(a6)                     | +01c
        andi.b  #0xf,0x7e(a6)                   | +020
        beq.w   SetXN_082cac                    | +026

| ----------------------------------------------------------------------------
|  Sub_00082CB2  @ $082CB2  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00082CB2, "ax", @progbits
        .global Sub_00082CB2
Sub_00082CB2:
        clr.b   0x7e(a6)                        | +000
        move.w  0x22(a6),0x80(a6)               | +004
        move.w  0x24(a6),0x82(a6)               | +00a
        rts                                     | +010

| ----------------------------------------------------------------------------
|  Sub_00082CC4  @ $082CC4  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00082CC4, "ax", @progbits
        .global Sub_00082CC4
Sub_00082CC4:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x38(a0),0x38(a6)               | +004
        move.w  0x80(a0),0x22(a6)               | +00a
        move.w  0x82(a0),0x24(a6)               | +010
        move.b  0x73(a0),0x73(a6)               | +016
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  Sub_00082CE2  @ $082CE2  (48 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00082CE2, "ax", @progbits
        .global Sub_00082CE2
Sub_00082CE2:
        movea.l 0xc(a6),a0                      | +000
        move.l  0x94(a0),0x94(a6)               | +004
        move.w  0x38(a0),0x38(a6)               | +00a
        move.w  0x80(a0),d0                     | +010
        add.w   0x80(a6),d0                     | +014
        move.w  d0,0x22(a6)                     | +018
        move.w  0x82(a0),d0                     | +01c
        add.w   0x82(a6),d0                     | +020
        move.w  d0,0x24(a6)                     | +024
        move.b  0x73(a0),0x73(a6)               | +028
        rts                                     | +02e

| ----------------------------------------------------------------------------
|  Sub_00082D12  @ $082D12  (46 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00082D12, "ax", @progbits
        .global Sub_00082D12
Sub_00082D12:
        btst    #0x6,0x72(a6)                   | +000
        beq.w   .L82d22                         | +006
        bset    #0x0,0x5a(a6)                   | +00a
.L82d22:
        bclr    #0x6,0x72(a6)                   | +010
        bclr    #0x7,0x72(a6)                   | +016
        btst    #0x0,0x5a(a6)                   | +01c
        beq.w   .L82d3e                         | +022
        bset    #0x7,0x72(a6)                   | +026
.L82d3e:
        rts                                     | +02c

| ----------------------------------------------------------------------------
|  Sub_00082D40  @ $082D40  (48 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00082D40, "ax", @progbits
        .global Sub_00082D40
Sub_00082D40:
        btst    #0x0,0x5a(a6)                   | +000
        beq.w   .L82d5a                         | +006
        bclr    #0x0,0x5a(a6)                   | +00a
        movea.l 0xc(a6),a0                      | +010
        bset    #0x6,0x72(a0)                   | +014
.L82d5a:
        movea.l 0xc(a6),a0                      | +01a
        btst    #0x7,0x72(a0)                   | +01e
        beq.w   .L82d6e                         | +024
        bset    #0x0,0x5a(a6)                   | +028
.L82d6e:
        rts                                     | +02e

| ----------------------------------------------------------------------------
|  Sub_00082D70  @ $082D70  (62 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00082D70, "ax", @progbits
        .global Sub_00082D70
Sub_00082D70:
        tst.b   0x73(a6)                        | +000
        bne.w   .L82dac                         | +004
        move.b  0x7d(a6),d0                     | +008
        andi.b  #0x3,d0                         | +00c
        beq.w   .L82da6                         | +010
        lea     0x2be098.l,a0                   | +014
        cmpi.b  #0x3,d0                         | +01a
        beq.w   .L82d98                         | +01e
        lea     0x2be11a.l,a0                   | +022
.L82d98:
        jsr     0x799de.l                       | +028
        cmp.w   0x66(a6),d0                     | +02e
        blt.w   .L82dac                         | +032
.L82da6:
        move.b  #0xff,0x73(a6)                  | +036
.L82dac:
        rts                                     | +03c

| ----------------------------------------------------------------------------
|  Sub_00082DAE  @ $082DAE  (58 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00082DAE, "ax", @progbits
        .global Sub_00082DAE
Sub_00082DAE:
        btst    #0x5,0x72(a6)                   | +000
        bne.w   ClearXN_082de8                  | +006
        cmpi.w  #0x20,0x22(a6)                  | +00a
        blt.w   ClrFlag74_082dee                | +010
        cmpi.w  #0x120,0x22(a6)                 | +014
        bgt.w   SetFlag74_082df8                | +01a
        tst.b   0x7b(a6)                        | +01e
        bne.w   ClearXN_082de8                  | +022
        movea.l 0x94(a6),a0                     | +026
        move.w  0x22(a0),d0                     | +02a
        cmp.w   0x22(a6),d0                     | +02e
        bgt.w   ClrFlag74_082dee                | +032
        bra.w   SetFlag74_082df8                | +036

| ----------------------------------------------------------------------------
|  ClrFlag74_082dee  @ $082DEE  (4 B)
| ----------------------------------------------------------------------------
        .section .text.ClrFlag74_082dee, "ax", @progbits
        .global ClrFlag74_082dee
ClrFlag74_082dee:
        clr.b   0x74(a6)                        | +000

| ----------------------------------------------------------------------------
|  SetFlag74_082df8  @ $082DF8  (6 B)
| ----------------------------------------------------------------------------
        .section .text.SetFlag74_082df8, "ax", @progbits
        .global SetFlag74_082df8
SetFlag74_082df8:
        move.b  #0x1,0x74(a6)                   | +000

| ----------------------------------------------------------------------------
|  Sub_00082E04  @ $082E04  (106 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00082E04, "ax", @progbits
        .global Sub_00082E04
Sub_00082E04:
        btst    #0x5,0x72(a6)                   | +000
        bne.w   SetXN_082e6e                    | +006
        cmpi.w  #0x20,0x22(a6)                  | +00a
        blt.w   .L82e2a                         | +010
        cmpi.w  #0x120,0x22(a6)                 | +014
        bgt.w   .L82e2a                         | +01a
        tst.b   0x7b(a6)                        | +01e
        bne.w   SetXN_082e6e                    | +022
.L82e2a:
        tst.b   0x74(a6)                        | +026
        beq.w   .L82e50                         | +02a
        cmpi.w  #0x30,0x22(a6)                  | +02e
        blt.w   SetXN_082e6e                    | +034
        movea.l 0x94(a6),a0                     | +038
        move.w  0x22(a0),d0                     | +03c
        cmp.w   0x22(a6),d0                     | +040
        bgt.w   SetXN_082e6e                    | +044
        bra.w   ClearXN_082e74                  | +048
.L82e50:
        cmpi.w  #0x120,0x22(a6)                 | +04c
        bgt.w   SetXN_082e6e                    | +052
        movea.l 0x94(a6),a0                     | +056
        move.w  0x22(a0),d0                     | +05a
        cmp.w   0x22(a6),d0                     | +05e
        blt.w   SetXN_082e6e                    | +062
        bra.w   ClearXN_082e74                  | +066

| ----------------------------------------------------------------------------
|  Sub_00082E7A  @ $082E7A  (90 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00082E7A, "ax", @progbits
        .global Sub_00082E7A
Sub_00082E7A:
        move.b  0x72(a6),d0                     | +000
        andi.b  #0x7,d0                         | +004
        bne.w   ClearXN_082eda                  | +008
        btst    #0x4,0x72(a6)                   | +00c
        beq.w   .L82ea2                         | +012
        tst.b   0x7b(a6)                        | +016
        beq.w   .L82ea2                         | +01a
        bset    #0x2,0x72(a6)                   | +01e
        bra.w   SetXN_082ed4                    | +024
.L82ea2:
        btst    #0x3,0x72(a6)                   | +028
        beq.w   ClearXN_082eda                  | +02e
        jsr     0x5e3fc.l                       | +032
        bcs.w   .L82ece                         | +038
        jsr     0x5e9b6.l                       | +03c
        andi.w  #0x7,d0                         | +042
        bne.w   .L82ece                         | +046
        bset    #0x1,0x72(a6)                   | +04a
        bra.w   SetXN_082ed4                    | +050
.L82ece:
        bset    #0x0,0x72(a6)                   | +054

| ----------------------------------------------------------------------------
|  Sub_00082EE0  @ $082EE0  (12 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00082EE0, "ax", @progbits
        .global Sub_00082EE0
Sub_00082EE0:
        movea.l 0xc(a6),a0                      | +000
        tst.b   0x20(a0)                        | +004
        bne.w   SetXN_082ef2                    | +008

| ----------------------------------------------------------------------------
|  Sub_00082EF8  @ $082EF8  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00082EF8, "ax", @progbits
        .global Sub_00082EF8
Sub_00082EF8:
        movea.l 0xc(a6),a0                      | +000
        btst    #0x0,0x72(a0)                   | +004
        bne.w   .L82f18                         | +00a
        movea.l 0xc(a6),a0                      | +00e
        btst    #0x1,0x72(a0)                   | +012
        bne.w   .L82f1e                         | +018
        clr.w   d0                              | +01c
        rts                                     | +01e
.L82f18:
        move.w  #0x1,d0                         | +020
        rts                                     | +024
.L82f1e:
        move.w  #0x2,d0                         | +026
        rts                                     | +02a

| ----------------------------------------------------------------------------
|  Sub_00082F24  @ $082F24  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00082F24, "ax", @progbits
        .global Sub_00082F24
Sub_00082F24:
        movea.l 0x94(a6),a0                     | +000
        addi.w  #0x38,0x24(a6)                  | +004
        jsr     0x5e070.l                       | +00a
        subi.w  #0x38,0x24(a6)                  | +010
        subi.w  #0x24,d0                        | +016
        asr.w   #0x3,d0                         | +01a
        cmp.w   0x5c(a6),d0                     | +01c
        beq.w   SetFlag7b_082f7e                | +020
        move.w  #0x1,d1                         | +024
        cmp.w   0x5c(a6),d0                     | +028
        bgt.w   .L82f56                         | +02c
        neg.w   d1                              | +030
.L82f56:
        move.w  d1,d0                           | +032
        add.w   0x5c(a6),d1                     | +034
        cmpi.w  #0x0,d1                         | +038
        bge.w   .L82f66                         | +03c
        clr.w   d0                              | +040
.L82f66:
        cmpi.w  #0x6,d1                         | +042
        ble.w   .L82f70                         | +046
        clr.w   d0                              | +04a
.L82f70:
        movea.l 0xc(a6),a0                      | +04c
        clr.b   0x7b(a0)                        | +050

| ----------------------------------------------------------------------------
|  SetFlag7b_082f7e  @ $082F7E  (12 B)
| ----------------------------------------------------------------------------
        .section .text.SetFlag7b_082f7e, "ax", @progbits
        .global SetFlag7b_082f7e
SetFlag7b_082f7e:
        clr.w   d0                              | +000
        movea.l 0xc(a6),a0                      | +002
        move.b  #0x1,0x7b(a0)                   | +006

| ----------------------------------------------------------------------------
|  Sub_00082F90  @ $082F90  (58 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00082F90, "ax", @progbits
        .global Sub_00082F90
Sub_00082F90:
        movea.l a6,a1                           | +000
        cmpi.b  #0xff,0x7c(a6)                  | +002
        beq.w   .L82fa0                         | +008
        movea.l 0xc(a6),a1                      | +00c
.L82fa0:
        movea.l 0x94(a1),a0                     | +010
        jsr     0x5e338.l                       | +014
        bcc.w   .L82fc8                         | +01a
        jsr     0x5e0d4.l                       | +01e
        movea.l a6,a1                           | +024
        cmpi.b  #0xff,0x7c(a6)                  | +026
        beq.w   .L82fc4                         | +02c
        movea.l 0xc(a6),a1                      | +030
.L82fc4:
        move.l  a0,0x94(a1)                     | +034
.L82fc8:
        rts                                     | +038

| ----------------------------------------------------------------------------
|  Sub_00082FCA  @ $082FCA  (154 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00082FCA, "ax", @progbits
        .global Sub_00082FCA
Sub_00082FCA:
        lea     0x77e10.l,a1                    | +000
        jsr     0x4ae.l                         | +006
        jsr     0x5dd02.l                       | +00c
        move.w  0x5c(a6),d0                     | +012
        lsl.w   #0x2,d0                         | +016
        lea     0x2e5852.l,a1                   | +018
        move.w  (a1,d0.w),d1                    | +01e
        move.w  0x2(a1,d0.w),d2                 | +022
        add.w   d2,0x24(a0)                     | +026
        btst    #0x0,0x3a(a6)                   | +02a
        beq.w   .L83000                         | +030
        neg.w   d1                              | +034
.L83000:
        add.w   d1,0x22(a0)                     | +036
        lea     TaskHandler_082720(pc),a1       | +03a
        jsr     0x4ae.l                         | +03e
        jsr     0x5dd02.l                       | +044
        move.w  0x5c(a6),d0                     | +04a
        move.w  d0,d3                           | +04e
        lsl.w   #0x2,d0                         | +050
        lea     0x2e5852.l,a1                   | +052
        move.w  (a1,d0.w),d1                    | +058
        move.w  0x2(a1,d0.w),d2                 | +05c
        add.w   d2,0x24(a0)                     | +060
        lea     0x2e586a.l,a2                   | +064
        move.w  d3,d4                           | +06a
        add.w   d4,d3                           | +06c
        add.w   d4,d3                           | +06e
        add.w   d3,d3                           | +070
        move.w  0x2(a2,d3.w),0x2e(a0)           | +072
        move.w  0x4(a2,d3.w),0x2a(a0)           | +078
        move.w  (a2,d3.w),d4                    | +07e
        btst    #0x0,0x3a(a6)                   | +082
        beq.w   .L8305a                         | +088
        neg.w   d1                              | +08c
        neg.w   d4                              | +08e
.L8305a:
        add.w   d1,0x22(a0)                     | +090
        move.w  d4,0x28(a0)                     | +094
        rts                                     | +098

| ----------------------------------------------------------------------------
|  Sub_00083064  @ $083064  (190 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00083064, "ax", @progbits
        .global Sub_00083064
Sub_00083064:
        lea     TaskHandler_08292c(pc),a1       | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        move.w  0x5e(a6),d0                     | +010
        move.w  d0,d3                           | +014
        lea     0x2e58ae.l,a1                   | +016
        lsl.w   #0x2,d0                         | +01c
        move.w  (a1,d0.w),d1                    | +01e
        move.w  0x2(a1,d0.w),d2                 | +022
        add.w   d2,0x24(a0)                     | +026
        lea     0x2e58c6.l,a2                   | +02a
        move.w  d3,d4                           | +030
        add.w   d4,d3                           | +032
        add.w   d4,d3                           | +034
        add.w   d3,d3                           | +036
        move.w  0x2(a2,d3.w),0x2e(a0)           | +038
        move.w  0x4(a2,d3.w),0x2a(a0)           | +03e
        move.w  (a2,d3.w),d3                    | +044
        btst    #0x0,0x3a(a6)                   | +048
        beq.w   .L830ba                         | +04e
        neg.w   d1                              | +052
        neg.w   d3                              | +054
.L830ba:
        add.w   d1,0x22(a0)                     | +056
        add.w   d3,0x28(a0)                     | +05a
        move.w  #0x10ff,d0                      | +05e
        jsr     0x2352.l                        | +062
        lea     TaskHandler_082a66(pc),a1       | +068
        jsr     0x4ae.l                         | +06c
        jsr     0x5dd02.l                       | +072
        move.w  0x5e(a6),d0                     | +078
        move.w  d0,0x5c(a0)                     | +07c
        move.w  d0,d1                           | +080
        lsl.w   #0x3,d0                         | +082
        addi.w  #0x20,d0                        | +084
        cmpi.w  #0x80,d0                        | +088
        bcs.w   .L830f8                         | +08c
        move.w  #0x80,d0                        | +090
.L830f8:
        move.w  d0,0x34(a0)                     | +094
        lsl.w   #0x2,d1                         | +098
        lea     0x2e589a.l,a1                   | +09a
        move.w  0x2(a1,d1.w),d0                 | +0a0
        add.w   d0,0x24(a0)                     | +0a4
        move.w  (a1,d1.w),d0                    | +0a8
        btst    #0x0,0x3a(a6)                   | +0ac
        beq.w   .L8311c                         | +0b2
        neg.w   d0                              | +0b6
.L8311c:
        add.w   d0,0x22(a0)                     | +0b8
        rts                                     | +0bc

| ----------------------------------------------------------------------------
|  Sub_00083122  @ $083122  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00083122, "ax", @progbits
        .global Sub_00083122
Sub_00083122:
        move.b  #0xa,0x7a(a6)                   | +000
.L83128:
        subq.b  #0x1,0x7a(a6)                   | +006
        lea     TaskHandler_082480(pc),a1       | +00a
        jsr     0x4ae.l                         | +00e
        jsr     0x5dd02.l                       | +014
        move.b  0x7a(a6),d0                     | +01a
        andi.w  #0xf,d0                         | +01e
        move.w  d0,0x5e(a0)                     | +022
        move.w  0x5c(a6),0x5c(a0)               | +026
        cmpi.b  #0x0,0x7a(a6)                   | +02c
        bgt.b   .L83128                         | +032
        rts                                     | +034

| ----------------------------------------------------------------------------
|  Sub_00083158  @ $083158  (96 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00083158, "ax", @progbits
        .global Sub_00083158
Sub_00083158:
        cmpi.b  #0x3,0x7a(a6)                   | +000
        bne.w   .L83164                         | +006
        rts                                     | +00a
.L83164:
        lea     TaskHandler_082562(pc),a1       | +00c
        jsr     0x4ae.l                         | +010
        jsr     0x5dd02.l                       | +016
        move.w  0x5c(a6),0x5c(a0)               | +01c
        clr.w   0x5e(a6)                        | +022
        lea     TaskHandler_082562(pc),a1       | +026
        jsr     0x4ae.l                         | +02a
        jsr     0x5dd02.l                       | +030
        move.w  0x5c(a6),0x5c(a0)               | +036
        move.w  #0x1,0x5e(a6)                   | +03c
        lea     TaskHandler_082562(pc),a1       | +042
        jsr     0x4ae.l                         | +046
        jsr     0x5dd02.l                       | +04c
        move.w  0x5c(a6),0x5c(a0)               | +052
        move.w  #0x2,0x5e(a6)                   | +058
        rts                                     | +05e

| ----------------------------------------------------------------------------
|  Sub_000831B8  @ $0831B8  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_000831B8, "ax", @progbits
        .global Sub_000831B8
Sub_000831B8:
        move.b  #0x1,0x10a2d1.l                 | +000
        rts                                     | +008

| ----------------------------------------------------------------------------
|  Sub_000831C2  @ $0831C2  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_000831C2, "ax", @progbits
        .global Sub_000831C2
Sub_000831C2:
        lea     TaskHandler_08267c(pc),a1       | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        move.w  0x5c(a6),0x5c(a0)               | +010
        rts                                     | +016

| ----------------------------------------------------------------------------
|  Sub_000831DA  @ $0831DA  (66 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_000831DA, "ax", @progbits
        .global Sub_000831DA
Sub_000831DA:
        lea     0x2e5a22.l,a0                   | +000
        move.b  0x7c(a6),d0                     | +006
        andi.w  #0x7,d0                         | +00a
        lsl.w   #0x2,d0                         | +00e
        movea.l (a0,d0.w),a1                    | +010
        jsr     0x77c7e.l                       | +014
        move.w  #0xc000,0x38(a0)                | +01a
        lea     0x2e597a.l,a0                   | +020
        move.b  0x7c(a6),d0                     | +026
        andi.w  #0x7,d0                         | +02a
        lsl.w   #0x2,d0                         | +02e
        movea.l (a0,d0.w),a1                    | +030
        jsr     0x77c7e.l                       | +034
        move.w  #0xc000,0x38(a0)                | +03a
        rts                                     | +040

| ----------------------------------------------------------------------------
|  Sub_0008321C  @ $08321C  (56 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0008321C, "ax", @progbits
        .global Sub_0008321C
Sub_0008321C:
        lea     0x77fd6.l,a1                    | +000
        jsr     0x4ae.l                         | +006
        jsr     0x5dd02.l                       | +00c
        jsr     0x5e9b6.l                       | +012
        andi.w  #0x1f,d0                        | +018
        subi.w  #0x10,d0                        | +01c
        add.w   d0,0x22(a0)                     | +020
        jsr     0x5e9b6.l                       | +024
        andi.w  #0x3f,d0                        | +02a
        subi.w  #0x10,d0                        | +02e
        add.w   d0,0x24(a0)                     | +032
        rts                                     | +036

| ----------------------------------------------------------------------------
|  LeaList_083254  @ $083254  (6 B)
| ----------------------------------------------------------------------------
        .section .text.LeaList_083254, "ax", @progbits
        .global LeaList_083254
LeaList_083254:
        lea     0x2e5ab8.l,a1                   | +000
