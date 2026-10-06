| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave ??? — (borrador)
|  Región: $0478FC..$048A3C  (4,416 B, 42 entradas, 1 huecos)
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
|  Pow_SpawnVariantTbl_0478fc  @ $0478FC  (128 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_SpawnVariantTbl_0478fc, "ax", @progbits
        .global Pow_SpawnVariantTbl_0478fc
Pow_SpawnVariantTbl_0478fc:
        move.b  #0xc0,0x7d(a6)                  | +000
        move.b  #0x20,0x7e(a6)                  | +006
        bra.w   Pow_SpawnInit_04797c__L047992   | +00c
        move.b  #0xe0,0x7d(a6)                  | +010
        move.b  #0x10,0x7e(a6)                  | +016
        bra.w   Pow_SpawnInit_04797c__L047992   | +01c
        move.b  #0xf0,0x7d(a6)                  | +020
        move.b  #0x8,0x7e(a6)                   | +026
        bra.w   Pow_SpawnInit_04797c__L047992   | +02c
        move.b  #0xc0,0x7d(a6)                  | +030
        move.b  #0x20,0x7e(a6)                  | +036
        bra.w   Pow_SpawnInit_04797c__L0479e4   | +03c
        move.b  #0xe0,0x7d(a6)                  | +040
        move.b  #0x10,0x7e(a6)                  | +046
        bra.w   Pow_SpawnInit_04797c__L0479e4   | +04c
        move.b  #0xf0,0x7d(a6)                  | +050
        move.b  #0x8,0x7e(a6)                   | +056
        bra.w   Pow_SpawnInit_04797c__L0479e4   | +05c
        move.b  #0xf0,0x7d(a6)                  | +060
        move.b  #0x8,0x7e(a6)                   | +066
        bra.w   Pow_SpawnInit_04797c__L0479ba   | +06c
        move.b  #0xf0,0x7d(a6)                  | +070
        move.b  #0x8,0x7e(a6)                   | +076
        bra.w   Pow_SpawnInit_04797c__L0479ec   | +07c

| ----------------------------------------------------------------------------
|  Pow_SpawnInit_04797c  @ $04797C  (198 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_SpawnInit_04797c, "ax", @progbits
        .global Pow_SpawnInit_04797c
Pow_SpawnInit_04797c:
        jsr     0x13600.l                       | +000
        move.b  #0xf0,0x7d(a6)                  | +006
        move.b  #0x8,0x7e(a6)                   | +00c
        bra.w   .L047998                        | +012
        .global Pow_SpawnInit_04797c__L047992
Pow_SpawnInit_04797c__L047992:
        jsr     0x5e7c0.l                       | +016
.L047998:
        clr.b   0x86(a6)                        | +01c
        clr.b   0x85(a6)                        | +020
        jsr     Sub_00048EA6(pc)                | +024
        jsr     0x5e0d4.l                       | +028
        move.l  a0,0x94(a6)                     | +02e
        tst.b   0x98(a6)                        | +032
        beq.w   Pow_Idle_047a4c                 | +036
        bra.w   Pow_WalkToward_047b1e           | +03a
        .global Pow_SpawnInit_04797c__L0479ba
Pow_SpawnInit_04797c__L0479ba:
        clr.b   0x86(a6)                        | +03e
        move.b  #0x1,0x85(a6)                   | +042
        jsr     0x5e7c0.l                       | +048
        jsr     Sub_00048EA6(pc)                | +04e
        jsr     0x5e0d4.l                       | +052
        move.l  a0,0x94(a6)                     | +058
        tst.b   0x98(a6)                        | +05c
        beq.w   Pow_Idle_047a4c                 | +060
        bra.w   Pow_WalkToward_047b1e           | +064
        .global Pow_SpawnInit_04797c__L0479e4
Pow_SpawnInit_04797c__L0479e4:
        clr.b   0x85(a6)                        | +068
        bra.w   .L0479f2                        | +06c
        .global Pow_SpawnInit_04797c__L0479ec
Pow_SpawnInit_04797c__L0479ec:
        move.b  #0x1,0x85(a6)                   | +070
.L0479f2:
        clr.b   0x86(a6)                        | +076
        jsr     0x5e7c0.l                       | +07a
        jsr     Sub_00048EA6(pc)                | +080
        jsr     0x5e0d4.l                       | +084
        move.l  a0,0x94(a6)                     | +08a
        lea     0x28f4c0.l,a0                   | +08e
        jsr     0x28cd4.l                       | +094
        lea     .L047a1c(pc),a1                 | +09a
        move.l  a1,(a6)                         | +09e
.L047a1c:
        jsr     Sub_00048FB0(pc)                | +0a0
        jsr     0x28d70.l                       | +0a4
        bcc.w   .L047a30                        | +0aa
        lea     Pow_Idle_047a4c(pc),a1          | +0ae
        move.l  a1,(a6)                         | +0b2
.L047a30:
        jsr     Sub_0004932C(pc)                | +0b4
        bcc.w   .L047a3e                        | +0b8
        lea     Pow_Hurt_047f12(pc),a1          | +0bc
        move.l  a1,(a6)                         | +0c0
.L047a3e:
        bra.w   Pow_TiedFreed_0489c6__L0489f8   | +0c2

| ----------------------------------------------------------------------------
|  Pow_RetargetPlayer_047a42  @ $047A42  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_RetargetPlayer_047a42, "ax", @progbits
        .global Pow_RetargetPlayer_047a42
Pow_RetargetPlayer_047a42:
        jsr     0x5e1ea.l                       | +000
        move.l  a0,0x94(a6)                     | +006

| ----------------------------------------------------------------------------
|  Pow_Idle_047a4c  @ $047A4C  (210 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_Idle_047a4c, "ax", @progbits
        .global Pow_Idle_047a4c
Pow_Idle_047a4c:
        clr.w   0x28(a6)                        | +000
        move.b  0x3a(a6),d0                     | +004
        eori.b  #0x1,d0                         | +008
        move.b  d0,0x7c(a6)                     | +00c
        lea     0x28e312.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        lea     .L047a6e(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L047a6e:
        jsr     Sub_00048FB0(pc)                | +022
        jsr     0x28d70.l                       | +026
        tst.b   0x98(a6)                        | +02c
        beq.w   .L047aea                        | +030
        movea.l 0x94(a6),a0                     | +034
        jsr     0x5e338.l                       | +038
        bcs.w   .L047ae0                        | +03e
        jsr     Sub_00048FCC(pc)                | +042
        lea     0x28e1be.l,a0                   | +046
        movea.l #0xffffffff,a1                  | +04c
        jsr     0x772.l                         | +052
        jsr     Sub_00049172(pc)                | +058
        bcc.w   .L047ab2                        | +05c
        lea     Pow_WalkToward_047b1e(pc),a1    | +060
        move.l  a1,(a6)                         | +064
.L047ab2:
        jsr     Sub_00049256(pc)                | +066
        bcc.w   .L047ac0                        | +06a
        lea     Pow_Turn_047ed2(pc),a1          | +06e
        move.l  a1,(a6)                         | +072
.L047ac0:
        jsr     Sub_00049196(pc)                | +074
        bcc.w   .L047ace                        | +078
        lea     Pow_RunAway_047cfe(pc),a1       | +07c
        move.l  a1,(a6)                         | +080
.L047ace:
        jsr     Sub_000491DE(pc)                | +082
        bcc.w   .L047adc                        | +086
        lea     Pow_Wait_047dbc(pc),a1          | +08a
        move.l  a1,(a6)                         | +08e
.L047adc:
        bra.w   .L047aea                        | +090
.L047ae0:
        jsr     0x5e1ea.l                       | +094
        move.l  a0,0x94(a6)                     | +09a
.L047aea:
        cmpi.w  #0x18,0x22(a6)                  | +09e
        blt.w   .L047b0c                        | +0a4
        cmpi.w  #0x128,0x22(a6)                 | +0a8
        bgt.w   .L047b0c                        | +0ae
        jsr     Sub_00049010(pc)                | +0b2
        bcc.w   .L047b0c                        | +0b6
        lea     Pow_RescueStart_047f84(pc),a1   | +0ba
        move.l  a1,(a6)                         | +0be
.L047b0c:
        jsr     Sub_0004932C(pc)                | +0c0
        bcc.w   .L047b1a                        | +0c4
        lea     Pow_Hurt_047f12(pc),a1          | +0c8
        move.l  a1,(a6)                         | +0cc
.L047b1a:
        bra.w   Pow_TiedFreed_0489c6__L0489f8   | +0ce

| ----------------------------------------------------------------------------
|  Pow_WalkToward_047b1e  @ $047B1E  (142 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_WalkToward_047b1e, "ax", @progbits
        .global Pow_WalkToward_047b1e
Pow_WalkToward_047b1e:
        move.b  0x3a(a6),0x7c(a6)               | +000
        lea     0x2bfb2e.l,a0                   | +006
        jsr     0x799de.l                       | +00c
        move.w  d0,0x36(a6)                     | +012
        jsr     Sub_00048F04(pc)                | +016
        lea     0x28e394.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        lea     .L047b4a(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L047b4a:
        jsr     Sub_00048F2E(pc)                | +02c
        jsr     0x28d70.l                       | +030
        jsr     Sub_00049346(pc)                | +036
        bcc.w   .L047b62                        | +03a
        lea     Pow_Stop_047e88(pc),a1          | +03e
        move.l  a1,(a6)                         | +042
.L047b62:
        jsr     Sub_000490FA(pc)                | +044
        bcc.w   .L047b70                        | +048
        lea     Pow_Stop_047e88(pc),a1          | +04c
        move.l  a1,(a6)                         | +050
.L047b70:
        jsr     Sub_00049256(pc)                | +052
        bcc.w   .L047b7e                        | +056
        lea     Pow_Stop_047e88(pc),a1          | +05a
        move.l  a1,(a6)                         | +05e
.L047b7e:
        jsr     Sub_00049196(pc)                | +060
        bcc.w   .L047b8c                        | +064
        lea     Pow_Stop_047e88(pc),a1          | +068
        move.l  a1,(a6)                         | +06c
.L047b8c:
        jsr     Sub_000491DE(pc)                | +06e
        bcc.w   .L047b9a                        | +072
        lea     Pow_Stop_047e88(pc),a1          | +076
        move.l  a1,(a6)                         | +07a
.L047b9a:
        jsr     Sub_0004932C(pc)                | +07c
        bcc.w   .L047ba8                        | +080
        lea     Pow_Hurt_047f12(pc),a1          | +084
        move.l  a1,(a6)                         | +088
.L047ba8:
        bra.w   Pow_TiedFreed_0489c6__L0489f8   | +08a

| ----------------------------------------------------------------------------
|  Pow_RunRight_047bac  @ $047BAC  (164 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_RunRight_047bac, "ax", @progbits
        .global Pow_RunRight_047bac
Pow_RunRight_047bac:
        move.b  0x3a(a6),0x7c(a6)               | +000
        jsr     Sub_000492F8(pc)                | +006
        bcs.w   Pow_RunLeft_047c50              | +00a
        lea     0x2bfbb0.l,a0                   | +00e
        jsr     0x799de.l                       | +014
        move.w  d0,0x36(a6)                     | +01a
        jsr     Sub_00048F04(pc)                | +01e
        lea     0x28e44e.l,a0                   | +022
        jsr     0x28cd4.l                       | +028
        lea     .L047be0(pc),a1                 | +02e
        move.l  a1,(a6)                         | +032
.L047be0:
        jsr     Sub_00048F2E(pc)                | +034
        jsr     0x28d70.l                       | +038
        jsr     Sub_00049346(pc)                | +03e
        bcc.w   .L047bf8                        | +042
        lea     Pow_RunLeft_047c50(pc),a1       | +046
        move.l  a1,(a6)                         | +04a
.L047bf8:
        jsr     Sub_000490FA(pc)                | +04c
        bcc.w   .L047c06                        | +050
        lea     Pow_Idle_047a4c(pc),a1          | +054
        move.l  a1,(a6)                         | +058
.L047c06:
        jsr     Sub_00049256(pc)                | +05a
        bcc.w   .L047c14                        | +05e
        lea     Pow_Turn_047ed2(pc),a1          | +062
        move.l  a1,(a6)                         | +066
.L047c14:
        jsr     Sub_00049196(pc)                | +068
        bcc.w   .L047c22                        | +06c
        lea     Pow_RunAway_047cfe(pc),a1       | +070
        move.l  a1,(a6)                         | +074
.L047c22:
        jsr     Sub_000492F8(pc)                | +076
        bcc.w   .L047c30                        | +07a
        lea     Pow_RunLeft_047c50(pc),a1       | +07e
        move.l  a1,(a6)                         | +082
.L047c30:
        jsr     Sub_000491DE(pc)                | +084
        bcc.w   .L047c3e                        | +088
        lea     Pow_Wait_047dbc(pc),a1          | +08c
        move.l  a1,(a6)                         | +090
.L047c3e:
        jsr     Sub_0004932C(pc)                | +092
        bcc.w   .L047c4c                        | +096
        lea     Pow_Hurt_047f12(pc),a1          | +09a
        move.l  a1,(a6)                         | +09e
.L047c4c:
        bra.w   Pow_TiedFreed_0489c6__L0489f8   | +0a0

| ----------------------------------------------------------------------------
|  Pow_RunLeft_047c50  @ $047C50  (174 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_RunLeft_047c50, "ax", @progbits
        .global Pow_RunLeft_047c50
Pow_RunLeft_047c50:
        move.b  0x3a(a6),d0                     | +000
        eori.b  #0x1,d0                         | +004
        move.b  d0,0x7c(a6)                     | +008
        jsr     Sub_000492F8(pc)                | +00c
        bcs.w   Pow_RunRight_047bac             | +010
        lea     0x2bfbb0.l,a0                   | +014
        jsr     0x799de.l                       | +01a
        move.w  d0,0x36(a6)                     | +020
        jsr     Sub_00048F04(pc)                | +024
        neg.w   0x28(a6)                        | +028
        lea     0x28e49e.l,a0                   | +02c
        jsr     0x28cd4.l                       | +032
        lea     .L047c8e(pc),a1                 | +038
        move.l  a1,(a6)                         | +03c
.L047c8e:
        jsr     Sub_00048F2E(pc)                | +03e
        jsr     0x28d70.l                       | +042
        jsr     Sub_00049346(pc)                | +048
        bcc.w   .L047ca6                        | +04c
        lea     Pow_RunRight_047bac(pc),a1      | +050
        move.l  a1,(a6)                         | +054
.L047ca6:
        jsr     Sub_000490FA(pc)                | +056
        bcc.w   .L047cb4                        | +05a
        lea     Pow_Idle_047a4c(pc),a1          | +05e
        move.l  a1,(a6)                         | +062
.L047cb4:
        jsr     Sub_00049256(pc)                | +064
        bcc.w   .L047cc2                        | +068
        lea     Pow_Turn_047ed2(pc),a1          | +06c
        move.l  a1,(a6)                         | +070
.L047cc2:
        jsr     Sub_00049196(pc)                | +072
        bcc.w   .L047cd0                        | +076
        lea     Pow_RunAway_047cfe(pc),a1       | +07a
        move.l  a1,(a6)                         | +07e
.L047cd0:
        jsr     Sub_000492F8(pc)                | +080
        bcc.w   .L047cde                        | +084
        lea     Pow_RunRight_047bac(pc),a1      | +088
        move.l  a1,(a6)                         | +08c
.L047cde:
        jsr     Sub_000491DE(pc)                | +08e
        bcc.w   .L047cec                        | +092
        lea     Pow_Wait_047dbc(pc),a1          | +096
        move.l  a1,(a6)                         | +09a
.L047cec:
        jsr     Sub_0004932C(pc)                | +09c
        bcc.w   .L047cfa                        | +0a0
        lea     Pow_Hurt_047f12(pc),a1          | +0a4
        move.l  a1,(a6)                         | +0a8
.L047cfa:
        bra.w   Pow_TiedFreed_0489c6__L0489f8   | +0aa

| ----------------------------------------------------------------------------
|  Pow_RunAway_047cfe  @ $047CFE  (190 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_RunAway_047cfe, "ax", @progbits
        .global Pow_RunAway_047cfe
Pow_RunAway_047cfe:
        move.b  0x3a(a6),d0                     | +000
        eori.b  #0x1,d0                         | +004
        move.b  d0,0x7c(a6)                     | +008
        jsr     Sub_000492F8(pc)                | +00c
        bcs.w   Pow_Idle_047a4c                 | +010
        lea     0x28e1b6.l,a0                   | +014
        jsr     0x5e086.l                       | +01a
        move.l  a0,0x94(a6)                     | +020
        lea     0x2bfbb0.l,a0                   | +024
        jsr     0x799de.l                       | +02a
        move.w  d0,0x36(a6)                     | +030
        jsr     Sub_00048F04(pc)                | +034
        neg.w   0x28(a6)                        | +038
        lea     0x28e49e.l,a0                   | +03c
        jsr     0x28cd4.l                       | +042
        lea     .L047d4c(pc),a1                 | +048
        move.l  a1,(a6)                         | +04c
.L047d4c:
        jsr     Sub_00048F2E(pc)                | +04e
        jsr     0x28d70.l                       | +052
        jsr     Sub_00049346(pc)                | +058
        bcc.w   .L047d64                        | +05c
        lea     Pow_RescueStart_047f84(pc),a1   | +060
        move.l  a1,(a6)                         | +064
.L047d64:
        jsr     Sub_00049010(pc)                | +066
        bcc.w   .L047d72                        | +06a
        lea     Pow_Idle_047a4c(pc),a1          | +06e
        move.l  a1,(a6)                         | +072
.L047d72:
        jsr     Sub_00049256(pc)                | +074
        bcc.w   .L047d80                        | +078
        lea     Pow_Turn_047ed2(pc),a1          | +07c
        move.l  a1,(a6)                         | +080
.L047d80:
        jsr     Sub_00049196(pc)                | +082
        bcs.w   .L047d8e                        | +086
        lea     Pow_Idle_047a4c(pc),a1          | +08a
        move.l  a1,(a6)                         | +08e
.L047d8e:
        jsr     Sub_000492F8(pc)                | +090
        bcc.w   .L047d9c                        | +094
        lea     Pow_Idle_047a4c(pc),a1          | +098
        move.l  a1,(a6)                         | +09c
.L047d9c:
        jsr     Sub_000491DE(pc)                | +09e
        bcc.w   .L047daa                        | +0a2
        lea     Pow_Wait_047dbc(pc),a1          | +0a6
        move.l  a1,(a6)                         | +0aa
.L047daa:
        jsr     Sub_0004932C(pc)                | +0ac
        bcc.w   .L047db8                        | +0b0
        lea     Pow_Hurt_047f12(pc),a1          | +0b4
        move.l  a1,(a6)                         | +0b8
.L047db8:
        bra.w   Pow_TiedFreed_0489c6__L0489f8   | +0ba

| ----------------------------------------------------------------------------
|  Pow_Wait_047dbc  @ $047DBC  (104 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_Wait_047dbc, "ax", @progbits
        .global Pow_Wait_047dbc
Pow_Wait_047dbc:
        clr.w   0x28(a6)                        | +000
        move.b  0x3a(a6),d0                     | +004
        andi.b  #0x1,d0                         | +008
        move.b  d0,0x7c(a6)                     | +00c
        jsr     Sub_000492F8(pc)                | +010
        bcs.w   Pow_Turn_047ed2                 | +014
        jsr     Sub_0004926A(pc)                | +018
        bcs.w   Pow_WalkFree_047e24             | +01c
        lea     0x28e1b6.l,a0                   | +020
        jsr     0x5e086.l                       | +026
        move.l  a0,0x94(a6)                     | +02c
        lea     0x28e7bc.l,a0                   | +030
        jsr     0x28cd4.l                       | +036
        lea     .L047dfe(pc),a1                 | +03c
        move.l  a1,(a6)                         | +040
.L047dfe:
        jsr     Sub_00048FB0(pc)                | +042
        jsr     0x28d70.l                       | +046
        bcc.w   .L047e12                        | +04c
        lea     Pow_WalkFree_047e24(pc),a1      | +050
        move.l  a1,(a6)                         | +054
.L047e12:
        jsr     Sub_0004932C(pc)                | +056
        bcc.w   .L047e20                        | +05a
        lea     Pow_Hurt_047f12(pc),a1          | +05e
        move.l  a1,(a6)                         | +062
.L047e20:
        bra.w   Pow_TiedFreed_0489c6__L0489f8   | +064

| ----------------------------------------------------------------------------
|  Pow_WalkFree_047e24  @ $047E24  (100 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_WalkFree_047e24, "ax", @progbits
        .global Pow_WalkFree_047e24
Pow_WalkFree_047e24:
        move.b  0x3a(a6),0x7c(a6)               | +000
        lea     0x2bfb2e.l,a0                   | +006
        jsr     0x799de.l                       | +00c
        move.w  d0,0x36(a6)                     | +012
        jsr     Sub_00048F04(pc)                | +016
        lea     0x28e394.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        lea     .L047e50(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L047e50:
        jsr     Sub_00048F2E(pc)                | +02c
        jsr     0x28d70.l                       | +030
        jsr     Sub_0004921E(pc)                | +036
        bcs.w   .L047e68                        | +03a
        lea     Pow_Stop_047e88(pc),a1          | +03e
        move.l  a1,(a6)                         | +042
.L047e68:
        jsr     Sub_000492F8(pc)                | +044
        bcc.w   .L047e76                        | +048
        lea     Pow_Stop_047e88(pc),a1          | +04c
        move.l  a1,(a6)                         | +050
.L047e76:
        jsr     Sub_0004932C(pc)                | +052
        bcc.w   .L047e84                        | +056
        lea     Pow_Hurt_047f12(pc),a1          | +05a
        move.l  a1,(a6)                         | +05e
.L047e84:
        bra.w   Pow_TiedFreed_0489c6__L0489f8   | +060

| ----------------------------------------------------------------------------
|  Pow_Stop_047e88  @ $047E88  (74 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_Stop_047e88, "ax", @progbits
        .global Pow_Stop_047e88
Pow_Stop_047e88:
        movea.l 0x94(a6),a0                     | +000
        jsr     0x5e338.l                       | +004
        bcs.w   Pow_RetargetPlayer_047a42       | +00a
        clr.w   0x28(a6)                        | +00e
        lea     0x28e416.l,a0                   | +012
        jsr     0x28cd4.l                       | +018
        lea     .L047eac(pc),a1                 | +01e
        move.l  a1,(a6)                         | +022
.L047eac:
        jsr     Sub_00048FB0(pc)                | +024
        jsr     0x28d70.l                       | +028
        bcc.w   .L047ec0                        | +02e
        lea     Pow_Idle_047a4c(pc),a1          | +032
        move.l  a1,(a6)                         | +036
.L047ec0:
        jsr     Sub_0004932C(pc)                | +038
        bcc.w   .L047ece                        | +03c
        lea     Pow_Hurt_047f12(pc),a1          | +040
        move.l  a1,(a6)                         | +044
.L047ece:
        bra.w   Pow_TiedFreed_0489c6__L0489f8   | +046

| ----------------------------------------------------------------------------
|  Pow_Turn_047ed2  @ $047ED2  (64 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_Turn_047ed2, "ax", @progbits
        .global Pow_Turn_047ed2
Pow_Turn_047ed2:
        addq.b  #0x1,0x77(a6)                   | +000
        clr.w   0x28(a6)                        | +004
        lea     0x28e858.l,a0                   | +008
        jsr     0x28cd4.l                       | +00e
        lea     .L047eec(pc),a1                 | +014
        move.l  a1,(a6)                         | +018
.L047eec:
        jsr     Sub_00048FB0(pc)                | +01a
        jsr     0x28d70.l                       | +01e
        bcc.w   .L047f00                        | +024
        lea     Pow_Idle_047a4c(pc),a1          | +028
        move.l  a1,(a6)                         | +02c
.L047f00:
        jsr     Sub_0004932C(pc)                | +02e
        bcc.w   .L047f0e                        | +032
        lea     Pow_Hurt_047f12(pc),a1          | +036
        move.l  a1,(a6)                         | +03a
.L047f0e:
        bra.w   Pow_TiedFreed_0489c6__L0489f8   | +03c

| ----------------------------------------------------------------------------
|  Pow_Hurt_047f12  @ $047F12  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_Hurt_047f12, "ax", @progbits
        .global Pow_Hurt_047f12
Pow_Hurt_047f12:
        move.w  0x28(a6),d0                     | +000
        asr.w   #0x1,d0                         | +004
        move.w  d0,0x28(a6)                     | +006
        lea     0x28e75e.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L047f2e(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L047f2e:
        jsr     0x27c8c.l                       | +01c
        bcc.w   .L047f3e                        | +022
        lea     Pow_GetUp_047f48(pc),a1         | +026
        move.l  a1,(a6)                         | +02a
.L047f3e:
        jsr     0x28d70.l                       | +02c
        bra.w   Pow_TiedFreed_0489c6__L0489f8   | +032

| ----------------------------------------------------------------------------
|  Pow_GetUp_047f48  @ $047F48  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_GetUp_047f48, "ax", @progbits
        .global Pow_GetUp_047f48
Pow_GetUp_047f48:
        clr.w   0x28(a6)                        | +000
        lea     0x28e79c.l,a0                   | +004
        jsr     0x28cd4.l                       | +00a
        lea     .L047f5e(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L047f5e:
        jsr     Sub_00048FB0(pc)                | +016
        jsr     0x28d70.l                       | +01a
        bcc.w   .L047f72                        | +020
        lea     Pow_Idle_047a4c(pc),a1          | +024
        move.l  a1,(a6)                         | +028
.L047f72:
        jsr     Sub_0004932C(pc)                | +02a
        bcc.w   .L047f80                        | +02e
        lea     Pow_Hurt_047f12(pc),a1          | +032
        move.l  a1,(a6)                         | +036
.L047f80:
        bra.w   Pow_TiedFreed_0489c6__L0489f8   | +038

| ----------------------------------------------------------------------------
|  Pow_RescueStart_047f84  @ $047F84  (192 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_RescueStart_047f84, "ax", @progbits
        .global Pow_RescueStart_047f84
Pow_RescueStart_047f84:
        tst.b   0x85(a6)                        | +000
        beq.w   .L047faa                        | +004
        movea.l 0x94(a6),a0                     | +008
        jsr     0x32de0.l                       | +00c
        bcc.w   .L047faa                        | +012
        movea.l 0x94(a6),a0                     | +016
        move.w  0x24(a0),d0                     | +01a
        cmp.w   0x24(a6),d0                     | +01e
        beq.w   Pow_RescueGiveItem_048260__L0482c0 | +022
.L047faa:
        clr.b   0x77(a6)                        | +026
        clr.w   0x28(a6)                        | +02a
        movea.l 0x94(a6),a0                     | +02e
        jsr     0x5e070.l                       | +032
        add.b   0x7e(a6),d0                     | +038
        move.b  d0,0x7a(a6)                     | +03c
        move.b  0x7d(a6),d1                     | +040
        jsr     Sub_0004939C(pc)                | +044
        bcs.w   .L047fd4                        | +048
        move.b  #0x80,d1                        | +04c
.L047fd4:
        move.b  0x7a(a6),d0                     | +050
        and.b   d1,d0                           | +054
        lsr.b   #0x4,d0                         | +056
        move.b  d0,0x7a(a6)                     | +058
        clr.b   0x81(a6)                        | +05c
        lea     0x28e8f4.l,a0                   | +060
        jsr     0x28cd4.l                       | +066
        andi.b  #0xf,0x7a(a6)                   | +06c
        move.b  0x7a(a6),0x7b(a6)               | +072
        cmpi.b  #0x8,0x7a(a6)                   | +078
        ble.w   .L048018                        | +07e
        move.b  #0x1,0x81(a6)                   | +082
        lea     0x28e914.l,a0                   | +088
        jsr     0x28cd4.l                       | +08e
.L048018:
        lea     .L04801e(pc),a1                 | +094
        move.l  a1,(a6)                         | +098
.L04801e:
        jsr     Sub_00048FB0(pc)                | +09a
        jsr     0x28d70.l                       | +09e
        bcc.w   .L048032                        | +0a4
        lea     Pow_RescueFaceCount_048044(pc),a1 | +0a8
        move.l  a1,(a6)                         | +0ac
.L048032:
        jsr     Sub_0004932C(pc)                | +0ae
        bcc.w   .L048040                        | +0b2
        lea     Pow_Hurt_047f12(pc),a1          | +0b6
        move.l  a1,(a6)                         | +0ba
.L048040:
        bra.w   Pow_TiedFreed_0489c6__L0489f8   | +0bc

| ----------------------------------------------------------------------------
|  Pow_RescueFaceCount_048044  @ $048044  (172 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_RescueFaceCount_048044, "ax", @progbits
        .global Pow_RescueFaceCount_048044
Pow_RescueFaceCount_048044:
        clr.w   0x28(a6)                        | +000
        andi.b  #0xf,0x7a(a6)                   | +004
        tst.b   0x81(a6)                        | +00a
        bne.w   .L048080                        | +00e
        lea     0x28e974.l,a0                   | +012
        jsr     0x28cd4.l                       | +018
        clr.w   0x78(a6)                        | +01e
        btst    #0x0,0x3a(a6)                   | +022
        bne.w   .L0480ac                        | +028
        move.b  #0x8,d0                         | +02c
        sub.b   0x7a(a6),d0                     | +030
        move.b  d0,0x7a(a6)                     | +034
        bra.w   .L0480ac                        | +038
.L048080:
        lea     0x28eab0.l,a0                   | +03c
        jsr     0x28cd4.l                       | +042
        clr.w   0x78(a6)                        | +048
        subi.b  #0x9,0x7a(a6)                   | +04c
        btst    #0x0,0x3a(a6)                   | +052
        beq.w   .L0480ac                        | +058
        move.b  #0x6,d0                         | +05c
        sub.b   0x7a(a6),d0                     | +060
        move.b  d0,0x7a(a6)                     | +064
.L0480ac:
        move.b  0x7a(a6),d0                     | +068
        add.b   d0,0x7a(a6)                     | +06c
        lea     .L0480ba(pc),a1                 | +070
        move.l  a1,(a6)                         | +074
.L0480ba:
        jsr     Sub_00048FB0(pc)                | +076
        jsr     0x28d70.l                       | +07a
        move.b  0x7a(a6),d0                     | +080
        andi.w  #0x1f,d0                        | +084
        cmp.w   0x78(a6),d0                     | +088
        bne.w   .L0480da                        | +08c
        lea     Pow_RescueStartSalute_0480f0(pc),a1 | +090
        move.l  a1,(a6)                         | +094
.L0480da:
        addq.w  #0x1,0x78(a6)                   | +096
        jsr     Sub_0004932C(pc)                | +09a
        bcc.w   .L0480ec                        | +09e
        lea     Pow_Hurt_047f12(pc),a1          | +0a2
        move.l  a1,(a6)                         | +0a6
.L0480ec:
        bra.w   Pow_TiedFreed_0489c6__L0489f8   | +0a8

| ----------------------------------------------------------------------------
|  Pow_RescueStartSalute_0480f0  @ $0480F0  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_RescueStartSalute_0480f0, "ax", @progbits
        .global Pow_RescueStartSalute_0480f0
Pow_RescueStartSalute_0480f0:
        clr.w   0x28(a6)                        | +000
        subq.w  #0x1,0x78(a6)                   | +004
        lea     0x2bfd36.l,a0                   | +008
        jsr     0x799de.l                       | +00e
        move.b  d0,0x82(a6)                     | +014

| ----------------------------------------------------------------------------
|  Pow_RescueSalute_048108  @ $048108  (182 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_RescueSalute_048108, "ax", @progbits
        .global Pow_RescueSalute_048108
Pow_RescueSalute_048108:
        move.b  0x7b(a6),d0                     | +000
        andi.w  #0xf,d0                         | +004
        btst    #0x0,0x3a(a6)                   | +008
        bne.w   .L04813a                        | +00e
        movea.l #0x28e1ca,a0                    | +012
        lsl.w   #0x2,d0                         | +018
        movea.l (a0,d0.w),a0                    | +01a
        cmpa.l  #0xffffffff,a0                  | +01e
        beq.w   .L048136                        | +024
        jsr     0x28cd4.l                       | +028
.L048136:
        bra.w   .L048156                        | +02e
.L04813a:
        movea.l #0x28e20a,a0                    | +032
        lsl.w   #0x2,d0                         | +038
        movea.l (a0,d0.w),a0                    | +03a
        cmpa.l  #0xffffffff,a0                  | +03e
        beq.w   .L048156                        | +044
        jsr     0x28cd4.l                       | +048
.L048156:
        jsr     Sub_0004940E(pc)                | +04e
        lea     0x2bfcb4.l,a0                   | +052
        jsr     0x799de.l                       | +058
        move.w  d0,0x72(a6)                     | +05e
        lea     .L048170(pc),a1                 | +062
        move.l  a1,(a6)                         | +066
.L048170:
        jsr     Sub_00048FB0(pc)                | +068
        jsr     0x28d70.l                       | +06c
        bcs.w   .L048188                        | +072
        cmpi.b  #0x1,0x82(a6)                   | +076
        ble.w   .L0481ac                        | +07c
.L048188:
        cmpi.w  #0x0,0x72(a6)                   | +080
        bgt.w   .L0481ac                        | +086
        lea     Pow_RescueSalute_048108(pc),a1  | +08a
        move.l  a1,(a6)                         | +08e
        subq.b  #0x1,0x82(a6)                   | +090
        cmpi.b  #0x0,0x82(a6)                   | +094
        bgt.w   .L0481ac                        | +09a
        lea     Pow_RescueTurnBack_0481be(pc),a1 | +09e
        move.l  a1,(a6)                         | +0a2
.L0481ac:
        jsr     Sub_0004932C(pc)                | +0a4
        bcc.w   .L0481ba                        | +0a8
        lea     Pow_Hurt_047f12(pc),a1          | +0ac
        move.l  a1,(a6)                         | +0b0
.L0481ba:
        bra.w   Pow_TiedFreed_0489c6__L0489f8   | +0b2

| ----------------------------------------------------------------------------
|  Pow_RescueTurnBack_0481be  @ $0481BE  (162 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_RescueTurnBack_0481be, "ax", @progbits
        .global Pow_RescueTurnBack_0481be
Pow_RescueTurnBack_0481be:
        clr.w   0x28(a6)                        | +000
        tst.b   0x81(a6)                        | +004
        bne.w   .L0481f6                        | +008
        lea     0x28e974.l,a0                   | +00c
        jsr     0x28cd4.l                       | +012
        cmpi.w  #0x8,0x78(a6)                   | +018
        blt.w   .L04821e                        | +01e
        move.w  #0x10,d0                        | +022
        sub.w   0x78(a6),d0                     | +026
        move.w  d0,0x78(a6)                     | +02a
        bchg    #0x0,0x3a(a6)                   | +02e
        bra.w   .L04821e                        | +034
.L0481f6:
        lea     0x28eab0.l,a0                   | +038
        jsr     0x28cd4.l                       | +03e
        cmpi.w  #0x6,0x78(a6)                   | +044
        blt.w   .L04821e                        | +04a
        move.w  #0xc,d0                         | +04e
        sub.w   0x78(a6),d0                     | +052
        move.w  d0,0x78(a6)                     | +056
        bchg    #0x0,0x3a(a6)                   | +05a
.L04821e:
        lea     .L048224(pc),a1                 | +060
        move.l  a1,(a6)                         | +064
.L048224:
        jsr     Sub_00048FB0(pc)                | +066
        jsr     0x28d70.l                       | +06a
        tst.w   0x78(a6)                        | +070
        bne.w   .L04823c                        | +074
        lea     Pow_RescueGiveItem_048260(pc),a1 | +078
        move.l  a1,(a6)                         | +07c
.L04823c:
        move.b  0x106f28.l,d0                   | +07e
        andi.b  #0x1,d0                         | +084
        bne.w   .L04824e                        | +088
        subq.w  #0x1,0x78(a6)                   | +08c
.L04824e:
        jsr     Sub_0004932C(pc)                | +090
        bcc.w   .L04825c                        | +094
        lea     Pow_Hurt_047f12(pc),a1          | +098
        move.l  a1,(a6)                         | +09c
.L04825c:
        bra.w   Pow_TiedFreed_0489c6__L0489f8   | +09e

| ----------------------------------------------------------------------------
|  Pow_RescueGiveItem_048260  @ $048260  (160 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_RescueGiveItem_048260, "ax", @progbits
        .global Pow_RescueGiveItem_048260
Pow_RescueGiveItem_048260:
        clr.w   0x28(a6)                        | +000
        lea     0x2bfc32.l,a0                   | +004
        jsr     0x799de.l                       | +00a
        move.w  d0,0x72(a6)                     | +010
        lea     0x28e934.l,a0                   | +014
        jsr     0x28cd4.l                       | +01a
        tst.b   0x81(a6)                        | +020
        beq.w   .L048294                        | +024
        lea     0x28e954.l,a0                   | +028
        jsr     0x28cd4.l                       | +02e
.L048294:
        lea     .L04829a(pc),a1                 | +034
        move.l  a1,(a6)                         | +038
.L04829a:
        jsr     Sub_00048FB0(pc)                | +03a
        jsr     0x28d70.l                       | +03e
        bcc.w   .L0482ae                        | +044
        lea     Pow_RetargetPlayer_047a42(pc),a1 | +048
        move.l  a1,(a6)                         | +04c
.L0482ae:
        jsr     Sub_0004932C(pc)                | +04e
        bcc.w   .L0482bc                        | +052
        lea     Pow_Hurt_047f12(pc),a1          | +056
        move.l  a1,(a6)                         | +05a
.L0482bc:
        bra.w   Pow_TiedFreed_0489c6__L0489f8   | +05c
        .global Pow_RescueGiveItem_048260__L0482c0
Pow_RescueGiveItem_048260__L0482c0:
        move.l  #0x28e03e,0x48(a6)              | +060
        lea     0x28e656.l,a0                   | +068
        jsr     0x28cd4.l                       | +06e
        lea     .L0482da(pc),a1                 | +074
        move.l  a1,(a6)                         | +078
.L0482da:
        jsr     Sub_00048FB0(pc)                | +07a
        jsr     0x28d70.l                       | +07e
        bcc.w   .L0482ee                        | +084
        lea     Pow_RescueThanksInit_048300(pc),a1 | +088
        move.l  a1,(a6)                         | +08c
.L0482ee:
        jsr     Sub_0004932C(pc)                | +08e
        bcc.w   .L0482fc                        | +092
        lea     Pow_Hurt_047f12(pc),a1          | +096
        move.l  a1,(a6)                         | +09a
.L0482fc:
        bra.w   Pow_TiedFreed_0489c6__L0489f8   | +09c

| ----------------------------------------------------------------------------
|  Pow_RescueThanksInit_048300  @ $048300  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_RescueThanksInit_048300, "ax", @progbits
        .global Pow_RescueThanksInit_048300
Pow_RescueThanksInit_048300:
        clr.w   0x28(a6)                        | +000
        lea     0x2bfd36.l,a0                   | +004
        jsr     0x799de.l                       | +00a
        move.b  d0,0x82(a6)                     | +010

| ----------------------------------------------------------------------------
|  Pow_RescueThanks_048314  @ $048314  (126 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_RescueThanks_048314, "ax", @progbits
        .global Pow_RescueThanks_048314
Pow_RescueThanks_048314:
        lea     0x2bfcb4.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,0x72(a6)                     | +00c
        move.l  #0x28e092,0x48(a6)              | +010
        lea     0x28e6c8.l,a0                   | +018
        jsr     0x28cd4.l                       | +01e
        lea     .L04833e(pc),a1                 | +024
        move.l  a1,(a6)                         | +028
.L04833e:
        jsr     Sub_00048FB0(pc)                | +02a
        jsr     0x28d70.l                       | +02e
        bcc.w   .L048380                        | +034
        cmpi.w  #0x0,0x72(a6)                   | +038
        bgt.w   .L048380                        | +03e
        lea     Pow_RescueThanks_048314(pc),a1  | +042
        move.l  a1,(a6)                         | +046
        subq.b  #0x1,0x82(a6)                   | +048
        cmpi.b  #0x0,0x82(a6)                   | +04c
        bgt.w   .L048380                        | +052
        lea     Pow_RescueLeave_048392(pc),a1   | +056
        move.l  a1,(a6)                         | +05a
        lea     0x2bfc32.l,a0                   | +05c
        jsr     0x799de.l                       | +062
        move.w  d0,0x72(a6)                     | +068
.L048380:
        jsr     Sub_0004932C(pc)                | +06c
        bcc.w   .L04838e                        | +070
        lea     Pow_Hurt_047f12(pc),a1          | +074
        move.l  a1,(a6)                         | +078
.L04838e:
        bra.w   Sub_00048A44                    | +07a

| ----------------------------------------------------------------------------
|  Pow_RescueLeave_048392  @ $048392  (64 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_RescueLeave_048392, "ax", @progbits
        .global Pow_RescueLeave_048392
Pow_RescueLeave_048392:
        lea     0x28e6ee.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L0483a4(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L0483a4:
        jsr     Sub_00048FB0(pc)                | +012
        jsr     0x28d70.l                       | +016
        bcc.w   .L0483c0                        | +01c
        move.l  #0x28df42,0x48(a6)              | +020
        lea     Pow_RetargetPlayer_047a42(pc),a1 | +028
        move.l  a1,(a6)                         | +02c
.L0483c0:
        jsr     Sub_0004932C(pc)                | +02e
        bcc.w   .L0483ce                        | +032
        lea     Pow_Hurt_047f12(pc),a1          | +036
        move.l  a1,(a6)                         | +03a
.L0483ce:
        bra.w   Sub_00048A44                    | +03c

| ----------------------------------------------------------------------------
|  Pow_SpawnFreeVariantA_0483d2  @ $0483D2  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_SpawnFreeVariantA_0483d2, "ax", @progbits
        .global Pow_SpawnFreeVariantA_0483d2
Pow_SpawnFreeVariantA_0483d2:
        move.b  #0x1,0x9b(a6)                   | +000
        move.b  #0x1,0x86(a6)                   | +006
        bra.w   Pow_SpawnFreeVariantB_0483e2__L0483e6 | +00c

| ----------------------------------------------------------------------------
|  Pow_SpawnFreeVariantB_0483e2  @ $0483E2  (62 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_SpawnFreeVariantB_0483e2, "ax", @progbits
        .global Pow_SpawnFreeVariantB_0483e2
Pow_SpawnFreeVariantB_0483e2:
        clr.b   0x86(a6)                        | +000
        .global Pow_SpawnFreeVariantB_0483e2__L0483e6
Pow_SpawnFreeVariantB_0483e2__L0483e6:
        jsr     0x5e7c0.l                       | +004
        jsr     Sub_00048EA6(pc)                | +00a
        tst.b   0x98(a6)                        | +00e
        bne.w   .L048404                        | +012
        tst.b   0x9b(a6)                        | +016
        bne.w   Pow_FreeThanksLoop_04857c       | +01a
        bra.w   Pow_FreeStand_048710            | +01e
.L048404:
        tst.b   0x9c(a6)                        | +022
        beq.w   .L048418                        | +026
        move.b  0x9c(a6),d0                     | +02a
        andi.w  #0xff,d0                        | +02e
        move.w  d0,0x72(a6)                     | +032
.L048418:
        jsr     Sub_0004932C(pc)                | +036
        bcc.w   Pow_FreeWalkOut_048492          | +03a

| ----------------------------------------------------------------------------
|  Pow_FreeHurt_048420  @ $048420  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_FreeHurt_048420, "ax", @progbits
        .global Pow_FreeHurt_048420
Pow_FreeHurt_048420:
        move.w  0x28(a6),d0                     | +000
        asr.w   #0x1,d0                         | +004
        move.w  d0,0x28(a6)                     | +006
        lea     0x28e75e.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L04843c(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L04843c:
        jsr     0x27c8c.l                       | +01c
        bcc.w   .L04844c                        | +022
        lea     Pow_FreeGetUp_048456(pc),a1     | +026
        move.l  a1,(a6)                         | +02a
.L04844c:
        jsr     0x28d70.l                       | +02c
        bra.w   Pow_TiedFreed_0489c6__L0489f8   | +032

| ----------------------------------------------------------------------------
|  Pow_FreeGetUp_048456  @ $048456  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_FreeGetUp_048456, "ax", @progbits
        .global Pow_FreeGetUp_048456
Pow_FreeGetUp_048456:
        clr.w   0x28(a6)                        | +000
        lea     0x28e79c.l,a0                   | +004
        jsr     0x28cd4.l                       | +00a
        lea     .L04846c(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L04846c:
        jsr     Sub_00048FB0(pc)                | +016
        jsr     0x28d70.l                       | +01a
        bcc.w   .L048480                        | +020
        lea     Pow_FreeWalkOut_048492(pc),a1   | +024
        move.l  a1,(a6)                         | +028
.L048480:
        jsr     Sub_0004932C(pc)                | +02a
        bcc.w   .L04848e                        | +02e
        lea     Pow_FreeHurt_048420(pc),a1      | +032
        move.l  a1,(a6)                         | +036
.L04848e:
        bra.w   Pow_TiedFreed_0489c6__L0489f8   | +038

| ----------------------------------------------------------------------------
|  Pow_FreeWalkOut_048492  @ $048492  (112 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_FreeWalkOut_048492, "ax", @progbits
        .global Pow_FreeWalkOut_048492
Pow_FreeWalkOut_048492:
        lea     0x2bfb2e.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,0x36(a6)                     | +00c
        jsr     Sub_00048F04(pc)                | +010
        lea     0x28e394.l,a0                   | +014
        jsr     0x28cd4.l                       | +01a
        lea     .L0484b8(pc),a1                 | +020
        move.l  a1,(a6)                         | +024
.L0484b8:
        jsr     Sub_00048F2E(pc)                | +026
        jsr     0x28d70.l                       | +02a
        move.b  0x98(a6),d0                     | +030
        andi.w  #0xff,d0                        | +034
        lsl.w   #0x4,d0                         | +038
        btst    #0x0,0x3a(a6)                   | +03a
        bne.w   .L0484e2                        | +040
        cmp.w   0x22(a6),d0                     | +044
        bgt.w   .L0484ea                        | +048
        bra.w   .L0484f0                        | +04c
.L0484e2:
        cmp.w   0x22(a6),d0                     | +050
        bgt.w   .L0484f0                        | +054
.L0484ea:
        lea     Pow_FreeJumpOut_048502(pc),a1   | +058
        move.l  a1,(a6)                         | +05c
.L0484f0:
        jsr     Sub_0004932C(pc)                | +05e
        bcc.w   .L0484fe                        | +062
        lea     Pow_FreeHurt_048420(pc),a1      | +066
        move.l  a1,(a6)                         | +06a
.L0484fe:
        bra.w   Pow_TiedFreed_0489c6__L0489f8   | +06c

| ----------------------------------------------------------------------------
|  Pow_FreeJumpOut_048502  @ $048502  (122 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_FreeJumpOut_048502, "ax", @progbits
        .global Pow_FreeJumpOut_048502
Pow_FreeJumpOut_048502:
        tst.b   0x9b(a6)                        | +000
        beq.w   Pow_FreeLeave_04867a__L0486d4   | +004
        move.w  #0xfccd,d0                      | +008
        jsr     0x5dca4.l                       | +00c
        move.w  d0,0x28(a6)                     | +012
        move.w  #0xccb,0x2a(a6)                 | +016
        move.w  #0xfae2,0x2e(a6)                | +01c
        move.w  #0x0,0x2c(a6)                   | +022
        lea     0x28e656.l,a0                   | +028
        jsr     0x28cd4.l                       | +02e
        lea     .L04853c(pc),a1                 | +034
        move.l  a1,(a6)                         | +038
.L04853c:
        jsr     0x27bc8.l                       | +03a
        bcc.w   .L04854a                        | +040
        clr.w   0x28(a6)                        | +044
.L04854a:
        jsr     0x28d70.l                       | +048
        bcc.w   .L048562                        | +04e
        tst.w   0x28(a6)                        | +052
        bne.w   .L048562                        | +056
        lea     Pow_FreeThanksLoop_04857c(pc),a1 | +05a
        move.l  a1,(a6)                         | +05e
.L048562:
        tst.b   0x86(a6)                        | +060
        beq.w   .L048578                        | +064
        jsr     Sub_0004932C(pc)                | +068
        bcc.w   .L048578                        | +06c
        lea     Pow_FreeHurt_048420(pc),a1      | +070
        move.l  a1,(a6)                         | +074
.L048578:
        bra.w   Pow_TiedFreed_0489c6__L0489f8   | +076

| ----------------------------------------------------------------------------
|  Pow_FreeThanksLoop_04857c  @ $04857C  (108 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_FreeThanksLoop_04857c, "ax", @progbits
        .global Pow_FreeThanksLoop_04857c
Pow_FreeThanksLoop_04857c:
        clr.w   0x28(a6)                        | +000
        move.l  #0x28e03e,0x48(a6)              | +004
        lea     0x28e6b0.l,a0                   | +00c
        jsr     0x28cd4.l                       | +012
        lea     .L04859a(pc),a1                 | +018
        move.l  a1,(a6)                         | +01c
.L04859a:
        jsr     Sub_00048FB0(pc)                | +01e
        jsr     0x28d70.l                       | +022
        jsr     Sub_000492A4(pc)                | +028
        bcc.w   .L0485b2                        | +02c
        lea     Pow_FreeLeave_04867a(pc),a1     | +030
        move.l  a1,(a6)                         | +034
.L0485b2:
        cmpi.w  #0x18,0x22(a6)                  | +036
        blt.w   .L0485d6                        | +03c
        cmpi.w  #0x128,0x22(a6)                 | +040
        bgt.w   .L0485d6                        | +046
        cmpi.w  #0x0,0x72(a6)                   | +04a
        bgt.w   .L0485d6                        | +050
        lea     Pow_FreeThanksInit_0485e8(pc),a1 | +054
        move.l  a1,(a6)                         | +058
.L0485d6:
        jsr     Sub_0004932C(pc)                | +05a
        bcc.w   .L0485e4                        | +05e
        lea     Pow_FreeHurt_048420(pc),a1      | +062
        move.l  a1,(a6)                         | +066
.L0485e4:
        bra.w   Sub_00048A44                    | +068

| ----------------------------------------------------------------------------
|  Pow_FreeThanksInit_0485e8  @ $0485E8  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_FreeThanksInit_0485e8, "ax", @progbits
        .global Pow_FreeThanksInit_0485e8
Pow_FreeThanksInit_0485e8:
        clr.w   0x28(a6)                        | +000
        lea     0x2bfd36.l,a0                   | +004
        jsr     0x799de.l                       | +00a
        move.b  d0,0x82(a6)                     | +010

| ----------------------------------------------------------------------------
|  Pow_FreeThanks_0485fc  @ $0485FC  (126 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_FreeThanks_0485fc, "ax", @progbits
        .global Pow_FreeThanks_0485fc
Pow_FreeThanks_0485fc:
        lea     0x2bfcb4.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,0x72(a6)                     | +00c
        move.l  #0x28e092,0x48(a6)              | +010
        lea     0x28e6c8.l,a0                   | +018
        jsr     0x28cd4.l                       | +01e
        lea     .L048626(pc),a1                 | +024
        move.l  a1,(a6)                         | +028
.L048626:
        jsr     Sub_00048FB0(pc)                | +02a
        jsr     0x28d70.l                       | +02e
        bcc.w   .L048668                        | +034
        cmpi.w  #0x0,0x72(a6)                   | +038
        bgt.w   .L048668                        | +03e
        lea     Pow_FreeThanks_0485fc(pc),a1    | +042
        move.l  a1,(a6)                         | +046
        subq.b  #0x1,0x82(a6)                   | +048
        cmpi.b  #0x0,0x82(a6)                   | +04c
        bgt.w   .L048668                        | +052
        lea     Pow_FreeThanksLoop_04857c(pc),a1 | +056
        move.l  a1,(a6)                         | +05a
        lea     0x2bfc32.l,a0                   | +05c
        jsr     0x799de.l                       | +062
        move.w  d0,0x72(a6)                     | +068
.L048668:
        jsr     Sub_0004932C(pc)                | +06c
        bcc.w   .L048676                        | +070
        lea     Pow_FreeHurt_048420(pc),a1      | +074
        move.l  a1,(a6)                         | +078
.L048676:
        bra.w   Sub_00048A44                    | +07a

| ----------------------------------------------------------------------------
|  Pow_FreeLeave_04867a  @ $04867A  (150 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_FreeLeave_04867a, "ax", @progbits
        .global Pow_FreeLeave_04867a
Pow_FreeLeave_04867a:
        clr.w   0x28(a6)                        | +000
        move.l  #0x28df42,0x48(a6)              | +004
        lea     0x28e6ee.l,a0                   | +00c
        jsr     0x28cd4.l                       | +012
        lea     .L048698(pc),a1                 | +018
        move.l  a1,(a6)                         | +01c
.L048698:
        jsr     Sub_00048FB0(pc)                | +01e
        jsr     0x28d70.l                       | +022
        bcc.w   .L0486c2                        | +028
        jsr     0x58fe2.l                       | +02c
        lea     Sub_00048CA4(pc),a1             | +032
        jsr     0x4ae.l                         | +036
        jsr     0x5dd02.l                       | +03c
        addi.w  #0x18,0x24(a0)                  | +042
.L0486c2:
        jsr     Sub_0004932C(pc)                | +048
        bcc.w   .L0486d0                        | +04c
        lea     Pow_FreeHurt_048420(pc),a1      | +050
        move.l  a1,(a6)                         | +054
.L0486d0:
        bra.w   Sub_00048A44                    | +056
        .global Pow_FreeLeave_04867a__L0486d4
Pow_FreeLeave_04867a__L0486d4:
        clr.w   0x28(a6)                        | +05a
        lea     0x28e4ee.l,a0                   | +05e
        jsr     0x28cd4.l                       | +064
        lea     .L0486ea(pc),a1                 | +06a
        move.l  a1,(a6)                         | +06e
.L0486ea:
        jsr     Sub_00048FB0(pc)                | +070
        jsr     0x28d70.l                       | +074
        bcc.w   .L0486fe                        | +07a
        lea     Pow_FreeStand_048710(pc),a1     | +07e
        move.l  a1,(a6)                         | +082
.L0486fe:
        jsr     Sub_0004932C(pc)                | +084
        bcc.w   .L04870c                        | +088
        lea     Pow_FreeHurt_048420(pc),a1      | +08c
        move.l  a1,(a6)                         | +090
.L04870c:
        bra.w   Pow_TiedFreed_0489c6__L0489f8   | +092

| ----------------------------------------------------------------------------
|  Pow_FreeStand_048710  @ $048710  (88 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_FreeStand_048710, "ax", @progbits
        .global Pow_FreeStand_048710
Pow_FreeStand_048710:
        clr.w   0x28(a6)                        | +000
        move.l  #0x28dfea,0x48(a6)              | +004
        lea     0x28e536.l,a0                   | +00c
        jsr     0x28cd4.l                       | +012
        lea     .L04872e(pc),a1                 | +018
        move.l  a1,(a6)                         | +01c
.L04872e:
        jsr     Sub_00048FB0(pc)                | +01e
        jsr     0x28d70.l                       | +022
        jsr     Sub_000492A4(pc)                | +028
        bcc.w   .L048746                        | +02c
        lea     Pow_FreeExit_04883e(pc),a1      | +030
        move.l  a1,(a6)                         | +034
.L048746:
        cmpi.w  #0x0,0x72(a6)                   | +036
        bgt.w   .L048756                        | +03c
        lea     Pow_FreeSaluteInit_048768(pc),a1 | +040
        move.l  a1,(a6)                         | +044
.L048756:
        jsr     Sub_0004932C(pc)                | +046
        bcc.w   .L048764                        | +04a
        lea     Pow_FreeHurt_048420(pc),a1      | +04e
        move.l  a1,(a6)                         | +052
.L048764:
        bra.w   Pow_TiedFreed_0489c6__L0489f8   | +054

| ----------------------------------------------------------------------------
|  Pow_FreeSaluteInit_048768  @ $048768  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_FreeSaluteInit_048768, "ax", @progbits
        .global Pow_FreeSaluteInit_048768
Pow_FreeSaluteInit_048768:
        clr.w   0x28(a6)                        | +000
        move.l  #0x28df42,0x48(a6)              | +004
        lea     0x2bfd36.l,a0                   | +00c
        jsr     0x799de.l                       | +012
        move.b  d0,0x82(a6)                     | +018
        lea     0x28e5b6.l,a0                   | +01c
        jsr     0x28cd4.l                       | +022
        bra.w   Pow_FreeSalute_048794__L0487a0  | +028

| ----------------------------------------------------------------------------
|  Pow_FreeSalute_048794  @ $048794  (102 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_FreeSalute_048794, "ax", @progbits
        .global Pow_FreeSalute_048794
Pow_FreeSalute_048794:
        lea     0x28e610.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        .global Pow_FreeSalute_048794__L0487a0
Pow_FreeSalute_048794__L0487a0:
        lea     .L0487a6(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L0487a6:
        jsr     Sub_00048FB0(pc)                | +012
        jsr     0x28d70.l                       | +016
        bcc.w   .L0487e8                        | +01c
        cmpi.w  #0x0,0x72(a6)                   | +020
        bgt.w   .L0487e8                        | +026
        lea     Pow_FreeSalute_048794(pc),a1    | +02a
        move.l  a1,(a6)                         | +02e
        subq.b  #0x1,0x82(a6)                   | +030
        cmpi.b  #0x0,0x82(a6)                   | +034
        bgt.w   .L0487e8                        | +03a
        lea     Pow_FreeIdle_0487fa(pc),a1      | +03e
        move.l  a1,(a6)                         | +042
        lea     0x2bfc32.l,a0                   | +044
        jsr     0x799de.l                       | +04a
        move.w  d0,0x72(a6)                     | +050
.L0487e8:
        jsr     Sub_0004932C(pc)                | +054
        bcc.w   .L0487f6                        | +058
        lea     Pow_FreeHurt_048420(pc),a1      | +05c
        move.l  a1,(a6)                         | +060
.L0487f6:
        bra.w   Pow_TiedFreed_0489c6__L0489f8   | +062

| ----------------------------------------------------------------------------
|  Pow_FreeIdle_0487fa  @ $0487FA  (68 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_FreeIdle_0487fa, "ax", @progbits
        .global Pow_FreeIdle_0487fa
Pow_FreeIdle_0487fa:
        clr.w   0x28(a6)                        | +000
        move.l  #0x28dfea,0x48(a6)              | +004
        lea     0x28e632.l,a0                   | +00c
        jsr     0x28cd4.l                       | +012
        lea     .L048818(pc),a1                 | +018
        move.l  a1,(a6)                         | +01c
.L048818:
        jsr     Sub_00048FB0(pc)                | +01e
        jsr     0x28d70.l                       | +022
        bcc.w   .L04882c                        | +028
        lea     Pow_FreeStand_048710(pc),a1     | +02c
        move.l  a1,(a6)                         | +030
.L04882c:
        jsr     Sub_0004932C(pc)                | +032
        bcc.w   .L04883a                        | +036
        lea     Pow_FreeHurt_048420(pc),a1      | +03a
        move.l  a1,(a6)                         | +03e
.L04883a:
        bra.w   Pow_TiedFreed_0489c6__L0489f8   | +040

| ----------------------------------------------------------------------------
|  Pow_FreeExit_04883e  @ $04883E  (90 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_FreeExit_04883e, "ax", @progbits
        .global Pow_FreeExit_04883e
Pow_FreeExit_04883e:
        clr.w   0x28(a6)                        | +000
        move.l  #0x28df42,0x48(a6)              | +004
        lea     0x28e578.l,a0                   | +00c
        jsr     0x28cd4.l                       | +012
        lea     .L04885c(pc),a1                 | +018
        move.l  a1,(a6)                         | +01c
.L04885c:
        jsr     Sub_00048FB0(pc)                | +01e
        jsr     0x28d70.l                       | +022
        bcc.w   .L048886                        | +028
        jsr     0x58fe2.l                       | +02c
        lea     Sub_00048CA4(pc),a1             | +032
        jsr     0x4ae.l                         | +036
        jsr     0x5dd02.l                       | +03c
        addi.w  #0x18,0x24(a0)                  | +042
.L048886:
        jsr     Sub_0004932C(pc)                | +048
        bcc.w   .L048894                        | +04c
        lea     Pow_FreeHurt_048420(pc),a1      | +050
        move.l  a1,(a6)                         | +054
.L048894:
        bra.w   Pow_TiedFreed_0489c6__L0489f8   | +056

| ----------------------------------------------------------------------------
|  Pow_SpawnTiedVariant_048898  @ $048898  (130 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_SpawnTiedVariant_048898, "ax", @progbits
        .global Pow_SpawnTiedVariant_048898
Pow_SpawnTiedVariant_048898:
        clr.b   0x80(a6)                        | +000
        bra.w   .L0488a6                        | +004
        move.b  #0x1,0x80(a6)                   | +008
.L0488a6:
        move.w  #0x38,d1                        | +00e
        jsr     0x236e.l                        | +012
        move.w  #0x1,0x66(a6)                   | +018
        move.w  #0x8000,d0                      | +01e
        jsr     0x28134.l                       | +022
        andi.w  #0xffe3,0x38(a6)                | +028
        ori.w   #0x18,0x38(a6)                  | +02e
        move.l  #0x28df96,0x48(a6)              | +034
        move.b  0x9e(a6),d0                     | +03c
        ext.w   d0                              | +040
        lsl.w   #0x4,d0                         | +042
        move.w  d0,0x8c(a6)                     | +044
        lea     0x2bff8e.l,a0                   | +048
        jsr     0x799de.l                       | +04e
        move.w  d0,0x8e(a6)                     | +054
        jsr     0x5e0d4.l                       | +058
        move.l  a0,0x94(a6)                     | +05e
        clr.w   0x78(a6)                        | +062
        move.b  #0x8,0x7e(a6)                   | +066
        move.b  #0xf0,0x7d(a6)                  | +06c
        lea     Sub_00048D0E(pc),a1             | +072
        jsr     0x4ae.l                         | +076
        jsr     0x5dd02.l                       | +07c

| ----------------------------------------------------------------------------
|  Pow_TiedIdle_04891a  @ $04891A  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_TiedIdle_04891a, "ax", @progbits
        .global Pow_TiedIdle_04891a
Pow_TiedIdle_04891a:
        lea     0x2bff0c.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,0x72(a6)                     | +00c
        move.b  #0x0,0x83(a6)                   | +010
        lea     0x28ed0a.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
        lea     .L048942(pc),a1                 | +022
        move.l  a1,(a6)                         | +026
.L048942:
        jsr     Sub_00048F54(pc)                | +028
        bcc.w   .L048950                        | +02c
        lea     Pow_TiedFreed_0489c6(pc),a1     | +030
        move.l  a1,(a6)                         | +034
.L048950:
        jsr     0x28d70.l                       | +036
        jsr     Sub_00049054(pc)                | +03c
        bcc.w   .L04896a                        | +040
        move.b  #0x1,0x83(a6)                   | +044
        lea     Pow_TiedStruggle_04896e(pc),a1  | +04a
        move.l  a1,(a6)                         | +04e
.L04896a:
        bra.w   Sub_00048A90                    | +050

| ----------------------------------------------------------------------------
|  Pow_TiedStruggle_04896e  @ $04896E  (88 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_TiedStruggle_04896e, "ax", @progbits
        .global Pow_TiedStruggle_04896e
Pow_TiedStruggle_04896e:
        move.b  #0x1,0x83(a6)                   | +000
        jsr     Sub_000493E4(pc)                | +006
        move.w  0x78(a6),d0                     | +00a
        lsr.w   #0x1,d0                         | +00e
        andi.w  #0xf,d0                         | +010
        movea.l #0x28e24a,a0                    | +014
        lsl.w   #0x2,d0                         | +01a
        movea.l (a0,d0.w),a0                    | +01c
        cmpa.l  #0xffffffff,a0                  | +020
        beq.w   .L04899e                        | +026
        jsr     0x28cd4.l                       | +02a
.L04899e:
        lea     .L0489a4(pc),a1                 | +030
        move.l  a1,(a6)                         | +034
.L0489a4:
        jsr     Sub_00048F54(pc)                | +036
        bcc.w   .L0489b2                        | +03a
        lea     Pow_TiedFreed_0489c6(pc),a1     | +03e
        move.l  a1,(a6)                         | +042
.L0489b2:
        jsr     0x28d70.l                       | +044
        bcc.w   .L0489c2                        | +04a
        lea     Pow_TiedIdle_04891a(pc),a1      | +04e
        move.l  a1,(a6)                         | +052
.L0489c2:
        bra.w   Sub_00048A90                    | +054

| ----------------------------------------------------------------------------
|  Pow_TiedFreed_0489c6  @ $0489C6  (118 B)
| ----------------------------------------------------------------------------
        .section .text.Pow_TiedFreed_0489c6, "ax", @progbits
        .global Pow_TiedFreed_0489c6
Pow_TiedFreed_0489c6:
        move.b  #0x3,0x83(a6)                   | +000
        lea     0x28ef0c.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     .L0489de(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L0489de:
        jsr     0x2783a.l                       | +018
        jsr     0x28d70.l                       | +01e
        bcc.w   .L0489f4                        | +024
        lea     Pow_SpawnInit_04797c(pc),a1     | +028
        move.l  a1,(a6)                         | +02c
.L0489f4:
        bra.w   Sub_00048A90                    | +02e
        .global Pow_TiedFreed_0489c6__L0489f8
Pow_TiedFreed_0489c6__L0489f8:
        jsr     Sub_0004936E(pc)                | +032
        subq.w  #0x1,0x72(a6)                   | +036
        jsr     0x2870a.l                       | +03a
        bcc.w   .L048a26                        | +040
        lea     Sub_00048CA4(pc),a1             | +044
        jsr     0x4ae.l                         | +048
        jsr     0x5dd02.l                       | +04e
        addi.w  #0x18,0x24(a0)                  | +054
        jsr     0x49fd0.l                       | +05a
.L048a26:
        movea.l #0xffffffff,a0                  | +060
        lea     0x28e196.l,a0                   | +066
        jsr     0x5dd5c.l                       | +06c
        bcc.w   SetHandlerRts_048a42            | +072
