| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave ??? — (borrador)
|  Región: $055B96..$056ACC  (3,738 B, 33 entradas, 15 huecos)
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
|  Entity_CmpDepthToParent_055b96  @ $055B96  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpDepthToParent_055b96, "ax", @progbits
        .global Entity_CmpDepthToParent_055b96
Entity_CmpDepthToParent_055b96:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_055bac                    | +00c

| ----------------------------------------------------------------------------
|  Entity_CmpDepthToParent_055bb2  @ $055BB2  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpDepthToParent_055bb2, "ax", @progbits
        .global Entity_CmpDepthToParent_055bb2
Entity_CmpDepthToParent_055bb2:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_055bc8                    | +00c

| ----------------------------------------------------------------------------
|  Grenade_SpawnFromThrower_055bce  @ $055BCE  (58 B)
| ----------------------------------------------------------------------------
        .section .text.Grenade_SpawnFromThrower_055bce, "ax", @progbits
        .global Grenade_SpawnFromThrower_055bce
Grenade_SpawnFromThrower_055bce:
        movem.l a6,-(a7)                        | +000
        lea     0x100800.l,a6                   | +004
        lea     Grenade_Task_055cd8(pc),a1      | +00a
        jsr     0x4ae.l                         | +00e
        movem.l (a7)+,a6                        | +014
        move.w  0x22(a6),d0                     | +018
        move.w  0x24(a6),d1                     | +01c
        addi.w  #0x20,d1                        | +020
        move.w  d0,0x22(a0)                     | +024
        move.w  d1,0x24(a0)                     | +028
        move.b  0x3a(a6),0x3a(a0)               | +02c
        move.b  0x5c(a6),0x5c(a0)               | +032
        rts                                     | +038

| ----------------------------------------------------------------------------
|  Grenade_TrajTable_055c08  @ $055C08  (192 B)
| ----------------------------------------------------------------------------
        .section .text.Grenade_TrajTable_055c08, "ax", @progbits
        .global Grenade_TrajTable_055c08
Grenade_TrajTable_055c08:
        .dc.b   0xfe                          | +000  '.'  (dato, rango --data)
        .dc.b   0xf7                          | +001  '.'  (dato, rango --data)
        .dc.b   0xff                          | +002  '.'  (dato, rango --data)
        .dc.b   0xbd                          | +003  '.'  (dato, rango --data)
        .dc.b   0x07                          | +004  '.'  (dato, rango --data)
        .dc.b   0x11                          | +005  '.'  (dato, rango --data)
        .dc.b   0x00                          | +006  '.'  (dato, rango --data)
        .dc.b   0x00                          | +007  '.'  (dato, rango --data)
        .dc.b   0xfe                          | +008  '.'  (dato, rango --data)
        .dc.b   0x13                          | +009  '.'  (dato, rango --data)
        .dc.b   0xff                          | +00a  '.'  (dato, rango --data)
        .dc.b   0xc8                          | +00b  '.'  (dato, rango --data)
        .dc.b   0x05                          | +00c  '.'  (dato, rango --data)
        .dc.b   0xe8                          | +00d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +00e  '.'  (dato, rango --data)
        .dc.b   0x00                          | +00f  '.'  (dato, rango --data)
        .dc.b   0xfd                          | +010  '.'  (dato, rango --data)
        .dc.b   0xc8                          | +011  '.'  (dato, rango --data)
        .dc.b   0xff                          | +012  '.'  (dato, rango --data)
        .dc.b   0xdc                          | +013  '.'  (dato, rango --data)
        .dc.b   0x05                          | +014  '.'  (dato, rango --data)
        .dc.b   0x10                          | +015  '.'  (dato, rango --data)
        .dc.b   0x00                          | +016  '.'  (dato, rango --data)
        .dc.b   0x00                          | +017  '.'  (dato, rango --data)
        .dc.b   0xfd                          | +018  '.'  (dato, rango --data)
        .dc.b   0x6f                          | +019  'o'  (dato, rango --data)
        .dc.b   0xff                          | +01a  '.'  (dato, rango --data)
        .dc.b   0xe6                          | +01b  '.'  (dato, rango --data)
        .dc.b   0x04                          | +01c  '.'  (dato, rango --data)
        .dc.b   0x1d                          | +01d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +01e  '.'  (dato, rango --data)
        .dc.b   0x00                          | +01f  '.'  (dato, rango --data)
        .dc.b   0xfd                          | +020  '.'  (dato, rango --data)
        .dc.b   0xee                          | +021  '.'  (dato, rango --data)
        .dc.b   0xff                          | +022  '.'  (dato, rango --data)
        .dc.b   0xd4                          | +023  '.'  (dato, rango --data)
        .dc.b   0x04                          | +024  '.'  (dato, rango --data)
        .dc.b   0xa4                          | +025  '.'  (dato, rango --data)
        .dc.b   0xff                          | +026  '.'  (dato, rango --data)
        .dc.b   0xff                          | +027  '.'  (dato, rango --data)
        .dc.b   0xfe                          | +028  '.'  (dato, rango --data)
        .dc.b   0x85                          | +029  '.'  (dato, rango --data)
        .dc.b   0xff                          | +02a  '.'  (dato, rango --data)
        .dc.b   0xb2                          | +02b  '.'  (dato, rango --data)
        .dc.b   0x08                          | +02c  '.'  (dato, rango --data)
        .dc.b   0x3a                          | +02d  ':'  (dato, rango --data)
        .dc.b   0xff                          | +02e  '.'  (dato, rango --data)
        .dc.b   0xff                          | +02f  '.'  (dato, rango --data)
        .dc.b   0xfd                          | +030  '.'  (dato, rango --data)
        .dc.b   0x56                          | +031  'V'  (dato, rango --data)
        .dc.b   0xff                          | +032  '.'  (dato, rango --data)
        .dc.b   0x9b                          | +033  '.'  (dato, rango --data)
        .dc.b   0x07                          | +034  '.'  (dato, rango --data)
        .dc.b   0x1a                          | +035  '.'  (dato, rango --data)
        .dc.b   0xff                          | +036  '.'  (dato, rango --data)
        .dc.b   0xff                          | +037  '.'  (dato, rango --data)
        .dc.b   0xfe                          | +038  '.'  (dato, rango --data)
        .dc.b   0x39                          | +039  '9'  (dato, rango --data)
        .dc.b   0xff                          | +03a  '.'  (dato, rango --data)
        .dc.b   0xce                          | +03b  '.'  (dato, rango --data)
        .dc.b   0x03                          | +03c  '.'  (dato, rango --data)
        .dc.b   0x84                          | +03d  '.'  (dato, rango --data)
        .dc.b   0xff                          | +03e  '.'  (dato, rango --data)
        .dc.b   0xff                          | +03f  '.'  (dato, rango --data)
        .dc.b   0x01                          | +040  '.'  (dato, rango --data)
        .dc.b   0xc7                          | +041  '.'  (dato, rango --data)
        .dc.b   0xff                          | +042  '.'  (dato, rango --data)
        .dc.b   0xce                          | +043  '.'  (dato, rango --data)
        .dc.b   0x03                          | +044  '.'  (dato, rango --data)
        .dc.b   0x84                          | +045  '.'  (dato, rango --data)
        .dc.b   0xff                          | +046  '.'  (dato, rango --data)
        .dc.b   0xff                          | +047  '.'  (dato, rango --data)
        .dc.b   0xfc                          | +048  '.'  (dato, rango --data)
        .dc.b   0xe4                          | +049  '.'  (dato, rango --data)
        .dc.b   0xff                          | +04a  '.'  (dato, rango --data)
        .dc.b   0xce                          | +04b  '.'  (dato, rango --data)
        .dc.b   0x01                          | +04c  '.'  (dato, rango --data)
        .dc.b   0xc2                          | +04d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +04e  '.'  (dato, rango --data)
        .dc.b   0x00                          | +04f  '.'  (dato, rango --data)
        .dc.b   0xfc                          | +050  '.'  (dato, rango --data)
        .dc.b   0xe4                          | +051  '.'  (dato, rango --data)
        .dc.b   0xff                          | +052  '.'  (dato, rango --data)
        .dc.b   0xce                          | +053  '.'  (dato, rango --data)
        .dc.b   0x01                          | +054  '.'  (dato, rango --data)
        .dc.b   0xc2                          | +055  '.'  (dato, rango --data)
        .dc.b   0x00                          | +056  '.'  (dato, rango --data)
        .dc.b   0x00                          | +057  '.'  (dato, rango --data)
        .dc.b   0xfc                          | +058  '.'  (dato, rango --data)
        .dc.b   0xe4                          | +059  '.'  (dato, rango --data)
        .dc.b   0xff                          | +05a  '.'  (dato, rango --data)
        .dc.b   0xce                          | +05b  '.'  (dato, rango --data)
        .dc.b   0x01                          | +05c  '.'  (dato, rango --data)
        .dc.b   0xc2                          | +05d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +05e  '.'  (dato, rango --data)
        .dc.b   0x00                          | +05f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +060  '.'  (dato, rango --data)
        .dc.b   0x00                          | +061  '.'  (dato, rango --data)
        .dc.b   0xff                          | +062  '.'  (dato, rango --data)
        .dc.b   0xce                          | +063  '.'  (dato, rango --data)
        .dc.b   0x01                          | +064  '.'  (dato, rango --data)
        .dc.b   0xc2                          | +065  '.'  (dato, rango --data)
        .dc.b   0xff                          | +066  '.'  (dato, rango --data)
        .dc.b   0xfe                          | +067  '.'  (dato, rango --data)
        .dc.b   0xfe                          | +068  '.'  (dato, rango --data)
        .dc.b   0xab                          | +069  '.'  (dato, rango --data)
        .dc.b   0xff                          | +06a  '.'  (dato, rango --data)
        .dc.b   0xce                          | +06b  '.'  (dato, rango --data)
        .dc.b   0x01                          | +06c  '.'  (dato, rango --data)
        .dc.b   0xc2                          | +06d  '.'  (dato, rango --data)
        .dc.b   0xff                          | +06e  '.'  (dato, rango --data)
        .dc.b   0xfe                          | +06f  '.'  (dato, rango --data)
        .dc.b   0xfd                          | +070  '.'  (dato, rango --data)
        .dc.b   0x56                          | +071  'V'  (dato, rango --data)
        .dc.b   0xff                          | +072  '.'  (dato, rango --data)
        .dc.b   0xce                          | +073  '.'  (dato, rango --data)
        .dc.b   0x01                          | +074  '.'  (dato, rango --data)
        .dc.b   0xc2                          | +075  '.'  (dato, rango --data)
        .dc.b   0xff                          | +076  '.'  (dato, rango --data)
        .dc.b   0xfe                          | +077  '.'  (dato, rango --data)
        .dc.b   0xfc                          | +078  '.'  (dato, rango --data)
        .dc.b   0x00                          | +079  '.'  (dato, rango --data)
        .dc.b   0xff                          | +07a  '.'  (dato, rango --data)
        .dc.b   0xce                          | +07b  '.'  (dato, rango --data)
        .dc.b   0x01                          | +07c  '.'  (dato, rango --data)
        .dc.b   0xc2                          | +07d  '.'  (dato, rango --data)
        .dc.b   0xff                          | +07e  '.'  (dato, rango --data)
        .dc.b   0xfe                          | +07f  '.'  (dato, rango --data)
        .dc.b   0xfe                          | +080  '.'  (dato, rango --data)
        .dc.b   0x23                          | +081  '#'  (dato, rango --data)
        .dc.b   0xff                          | +082  '.'  (dato, rango --data)
        .dc.b   0x93                          | +083  '.'  (dato, rango --data)
        .dc.b   0x06                          | +084  '.'  (dato, rango --data)
        .dc.b   0x63                          | +085  'c'  (dato, rango --data)
        .dc.b   0xff                          | +086  '.'  (dato, rango --data)
        .dc.b   0xff                          | +087  '.'  (dato, rango --data)
        .dc.b   0xfd                          | +088  '.'  (dato, rango --data)
        .dc.b   0x56                          | +089  'V'  (dato, rango --data)
        .dc.b   0xff                          | +08a  '.'  (dato, rango --data)
        .dc.b   0xd4                          | +08b  '.'  (dato, rango --data)
        .dc.b   0x04                          | +08c  '.'  (dato, rango --data)
        .dc.b   0xa4                          | +08d  '.'  (dato, rango --data)
        .dc.b   0xff                          | +08e  '.'  (dato, rango --data)
        .dc.b   0xff                          | +08f  '.'  (dato, rango --data)
        .dc.b   0xfd                          | +090  '.'  (dato, rango --data)
        .dc.b   0xee                          | +091  '.'  (dato, rango --data)
        .dc.b   0xff                          | +092  '.'  (dato, rango --data)
        .dc.b   0xb2                          | +093  '.'  (dato, rango --data)
        .dc.b   0x08                          | +094  '.'  (dato, rango --data)
        .dc.b   0x3a                          | +095  ':'  (dato, rango --data)
        .dc.b   0xff                          | +096  '.'  (dato, rango --data)
        .dc.b   0xff                          | +097  '.'  (dato, rango --data)
        .dc.b   0xfd                          | +098  '.'  (dato, rango --data)
        .dc.b   0x56                          | +099  'V'  (dato, rango --data)
        .dc.b   0xff                          | +09a  '.'  (dato, rango --data)
        .dc.b   0x93                          | +09b  '.'  (dato, rango --data)
        .dc.b   0x06                          | +09c  '.'  (dato, rango --data)
        .dc.b   0x63                          | +09d  'c'  (dato, rango --data)
        .dc.b   0xff                          | +09e  '.'  (dato, rango --data)
        .dc.b   0xff                          | +09f  '.'  (dato, rango --data)
        .dc.b   0xfe                          | +0a0  '.'  (dato, rango --data)
        .dc.b   0xf7                          | +0a1  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0a2  '.'  (dato, rango --data)
        .dc.b   0xbd                          | +0a3  '.'  (dato, rango --data)
        .dc.b   0x07                          | +0a4  '.'  (dato, rango --data)
        .dc.b   0x11                          | +0a5  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0a6  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0a7  '.'  (dato, rango --data)
        .dc.b   0xfe                          | +0a8  '.'  (dato, rango --data)
        .dc.b   0x13                          | +0a9  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0aa  '.'  (dato, rango --data)
        .dc.b   0xc8                          | +0ab  '.'  (dato, rango --data)
        .dc.b   0x05                          | +0ac  '.'  (dato, rango --data)
        .dc.b   0xe8                          | +0ad  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0ae  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0af  '.'  (dato, rango --data)
        .dc.b   0xfd                          | +0b0  '.'  (dato, rango --data)
        .dc.b   0x0a                          | +0b1  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0b2  '.'  (dato, rango --data)
        .dc.b   0xc0                          | +0b3  '.'  (dato, rango --data)
        .dc.b   0x06                          | +0b4  '.'  (dato, rango --data)
        .dc.b   0xc0                          | +0b5  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0b6  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0b7  '.'  (dato, rango --data)
        .dc.b   0xfc                          | +0b8  '.'  (dato, rango --data)
        .dc.b   0xa6                          | +0b9  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0ba  '.'  (dato, rango --data)
        .dc.b   0xd4                          | +0bb  '.'  (dato, rango --data)
        .dc.b   0x05                          | +0bc  '.'  (dato, rango --data)
        .dc.b   0x54                          | +0bd  'T'  (dato, rango --data)
        .dc.b   0xff                          | +0be  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0bf  '.'  (dato, rango --data)

| ----------------------------------------------------------------------------
|  Grenade_ProbeBox_055cc8  @ $055CC8  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Grenade_ProbeBox_055cc8, "ax", @progbits
        .global Grenade_ProbeBox_055cc8
Grenade_ProbeBox_055cc8:
        .dc.b   0xff                          | +000  '.'  (dato, rango --data)
        .dc.b   0xf0                          | +001  '.'  (dato, rango --data)
        .dc.b   0x00                          | +002  '.'  (dato, rango --data)
        .dc.b   0x10                          | +003  '.'  (dato, rango --data)
        .dc.b   0xff                          | +004  '.'  (dato, rango --data)
        .dc.b   0xc0                          | +005  '.'  (dato, rango --data)
        .dc.b   0x00                          | +006  '.'  (dato, rango --data)
        .dc.b   0x10                          | +007  '.'  (dato, rango --data)

| ----------------------------------------------------------------------------
|  Grenade_PalFieldOffs_055cd0  @ $055CD0  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Grenade_PalFieldOffs_055cd0, "ax", @progbits
        .global Grenade_PalFieldOffs_055cd0
Grenade_PalFieldOffs_055cd0:
        .dc.b   0x00                          | +000  '.'  (dato, rango --data)
        .dc.b   0x16                          | +001  '.'  (dato, rango --data)
        .dc.b   0x00                          | +002  '.'  (dato, rango --data)
        .dc.b   0x18                          | +003  '.'  (dato, rango --data)
        .dc.b   0x00                          | +004  '.'  (dato, rango --data)
        .dc.b   0x16                          | +005  '.'  (dato, rango --data)
        .dc.b   0x00                          | +006  '.'  (dato, rango --data)
        .dc.b   0x1a                          | +007  '.'  (dato, rango --data)

| ----------------------------------------------------------------------------
|  Grenade_Task_055cd8  @ $055CD8  (746 B)
| ----------------------------------------------------------------------------
        .section .text.Grenade_Task_055cd8, "ax", @progbits
        .global Grenade_Task_055cd8
Grenade_Task_055cd8:
        bset    #0x4,0x6b(a6)                   | +000
        move.w  #0xd000,0x38(a6)                | +006
        lea     Grenade_TrajTable_055c08(pc),a0 | +00c
        moveq   #0,d0                           | +010
        move.b  0x5c(a6),d0                     | +012
        lsl.w   #0x3,d0                         | +016
        adda.l  d0,a0                           | +018
        move.w  (a0),0x28(a6)                   | +01a
        btst    #0x0,0x3a(a6)                   | +01e
        beq.w   .L055d04                        | +024
        neg.w   0x28(a6)                        | +028
.L055d04:
        move.w  #0x0,0x2c(a6)                   | +02c
        move.w  0x2(a0),0x2e(a6)                | +032
        move.w  0x4(a0),0x2a(a6)                | +038
        move.w  0x6(a0),0x70(a6)                | +03e
        cmpi.b  #0x2,0x106f2b.l                 | +044
        bne.w   .L055da8                        | +04c
        clr.b   0x3a(a6)                        | +050
        jsr     0x5e172.l                       | +054
        bcc.w   .L055d3a                        | +05a
        bra.w   Jsr5B6ThenJmpScheduler_056058   | +05e
.L055d3a:
        lea     Grenade_TrajTable_055c08(pc),a0 | +062
        moveq   #0,d1                           | +066
        move.b  0x5c(a6),d1                     | +068
        lsl.w   #0x3,d1                         | +06c
        adda.l  d1,a0                           | +06e
        move.w  (a0),d1                         | +070
        move.w  d1,d2                           | +072
        asr.w   #0x2,d2                         | +074
        add.w   d2,d1                           | +076
        bge.w   .L055d56                        | +078
        neg.w   d1                              | +07c
.L055d56:
        move.w  d0,d2                           | +07e
        andi.w  #0x10,d2                        | +080
        lsl.w   #0x1,d2                         | +084
        add.w   d2,d0                           | +086
        andi.w  #0xe0,d0                        | +088
        move.w  d0,d2                           | +08c
        andi.w  #0x60,d2                        | +08e
        cmpi.w  #0x20,d2                        | +092
        bne.w   .L055d7c                        | +096
        addq.w  #0x8,d0                         | +09a
        andi.w  #0xf8,d0                        | +09c
        bra.w   .L055d8e                        | +0a0
.L055d7c:
        cmpi.w  #0x60,d2                        | +0a4
        bne.w   .L055d8e                        | +0a8
        subq.w  #0x8,d0                         | +0ac
        andi.w  #0xf8,d0                        | +0ae
        bra.w   .L055d8e                        | +0b2
.L055d8e:
        move.w  d0,0x30(a6)                     | +0b6
        jsr     0x13c0e.l                       | +0ba
        move.w  d1,0x28(a6)                     | +0c0
        move.w  d2,0x2a(a6)                     | +0c4
        clr.w   0x2c(a6)                        | +0c8
        clr.w   0x2e(a6)                        | +0cc
.L055da8:
        move.w  #0x15d,d1                       | +0d0
        jsr     0x236e.l                        | +0d4
        move.w  #0x164,d1                       | +0da
        jsr     0x236e.l                        | +0de
        move.w  #0x165,d1                       | +0e4
        jsr     0x236e.l                        | +0e8
        move.w  0x16(a6),0x14(a6)               | +0ee
        cmpi.b  #0x3,0x106f2b.l                 | +0f4
        bne.w   .L055dfa                        | +0fc
        clr.b   0x3a(a6)                        | +100
        move.w  0x30(a6),d0                     | +104
        lsr.w   #0x3,d0                         | +108
        andi.w  #0x1f,d0                        | +10a
        move.w  d0,0x30(a6)                     | +10e
        lea     0x2c72c0.l,a0                   | +112
        jsr     0x28cd4.l                       | +118
        bra.w   .L055e5e                        | +11e
.L055dfa:
        cmpi.b  #0x1,0x106f2b.l                 | +122
        bne.w   .L055e28                        | +12a
        clr.b   0x3a(a6)                        | +12e
        move.w  0x30(a6),d0                     | +132
        lsr.w   #0x3,d0                         | +136
        andi.w  #0x1f,d0                        | +138
        move.w  d0,0x30(a6)                     | +13c
        lea     0x2c72c0.l,a0                   | +140
        jsr     0x28cd4.l                       | +146
        bra.w   .L055e5e                        | +14c
.L055e28:
        cmpi.b  #0x2,0x106f2b.l                 | +150
        bne.w   .L055e52                        | +158
        move.w  0x30(a6),d0                     | +15c
        lsr.w   #0x3,d0                         | +160
        andi.w  #0x1f,d0                        | +162
        move.w  d0,0x30(a6)                     | +166
        lea     0x2c72c0.l,a0                   | +16a
        jsr     0x28cd4.l                       | +170
        bra.w   .L055e5e                        | +176
.L055e52:
        lea     0x2c72ba.l,a0                   | +17a
        jsr     0x28cd4.l                       | +180
.L055e5e:
        lea     0x29cbfe.l,a0                   | +186
        move.l  a0,0x4c(a6)                     | +18c
        jsr     0x283ca.l                       | +190
        move.w  0x24(a6),0x72(a6)               | +196
        lea     .L055e7a(pc),a1                 | +19c
        move.l  a1,(a6)                         | +1a0
.L055e7a:
        cmpi.w  #0xfffe,0x70(a6)                | +1a2
        bne.w   .L055e9e                        | +1a8
        bset    #0x6,0x13(a6)                   | +1ac
        jsr     0x27bc8.l                       | +1b2
        bcc.w   .L055e9a                        | +1b8
        lea     Grenade_Explode_055fca(pc),a1   | +1bc
        move.l  a1,(a6)                         | +1c0
.L055e9a:
        bra.w   .L055eca                        | +1c2
.L055e9e:
        tst.w   0x70(a6)                        | +1c6
        bne.w   .L055eba                        | +1ca
        jsr     0x27bc8.l                       | +1ce
        bcc.w   .L055eb6                        | +1d4
        lea     Grenade_Explode_055fca(pc),a1   | +1d8
        move.l  a1,(a6)                         | +1dc
.L055eb6:
        bra.w   .L055eca                        | +1de
.L055eba:
        jsr     0x27d50.l                       | +1e2
        bcc.w   .L055eca                        | +1e8
        lea     Grenade_Explode_055fca(pc),a1   | +1ec
        move.l  a1,(a6)                         | +1f0
.L055eca:
        jsr     0x280c6.l                       | +1f2
        cmpi.b  #0x1,d0                         | +1f8
        beq.w   Grenade_Explode_055fca          | +1fc
        cmpi.b  #0x2,0x106f2b.l                 | +200
        beq.w   .L055f0e                        | +208
        move.w  0x24(a6),d0                     | +20c
        cmp.w   0x72(a6),d0                     | +210
        bcc.w   .L055f0e                        | +214
        move.w  0x2a(a6),d0                     | +218
        asr.w   #0x5,d0                         | +21c
        sub.w   d0,0x2a(a6)                     | +21e
        move.w  0x28(a6),d0                     | +222
        beq.w   .L055f0e                        | +226
        asr.w   #0x5,d0                         | +22a
        bne.w   .L055f0a                        | +22c
        moveq   #1,d0                           | +230
.L055f0a:
        sub.w   d0,0x28(a6)                     | +232
.L055f0e:
        moveq   #0,d0                           | +236
        move.b  0x106f28.l,d0                   | +238
        andi.w  #0x3,d0                         | +23e
        lsl.w   #0x1,d0                         | +242
        lea     Grenade_PalFieldOffs_055cd0(pc),a0 | +244
        move.w  (a0,d0.w),d1                    | +248
        move.w  (a6,d1.w),0x14(a6)              | +24c
        cmpi.b  #0x3,0x106f2b.l                 | +252
        bne.w   .L055f4e                        | +25a
        move.w  0x28(a6),d0                     | +25e
        move.w  0x2a(a6),d1                     | +262
        asr.w   #0x4,d0                         | +266
        asr.w   #0x4,d1                         | +268
        jsr     0x5e018.l                       | +26a
        lsr.w   #0x3,d0                         | +270
        move.w  d0,0x30(a6)                     | +272
.L055f4e:
        cmpi.b  #0x1,0x106f2b.l                 | +276
        bne.w   .L055f72                        | +27e
        move.w  0x28(a6),d0                     | +282
        move.w  0x2a(a6),d1                     | +286
        asr.w   #0x4,d0                         | +28a
        asr.w   #0x4,d1                         | +28c
        jsr     0x5e018.l                       | +28e
        lsr.w   #0x3,d0                         | +294
        move.w  d0,0x30(a6)                     | +296
.L055f72:
        jsr     0x28d70.l                       | +29a
        jsr     0x283d8.l                       | +2a0
        btst    #0x1,0x13(a6)                   | +2a6
        beq.w   .L055f8e                        | +2ac
        lea     Grenade_Explode_055fca__L056012(pc),a1 | +2b0
        move.l  a1,(a6)                         | +2b4
.L055f8e:
        jsr     0x2870a.l                       | +2b6
        bcc.w   .L055f9e                        | +2bc
        lea     Grenade_Explode_055fca__L055fee(pc),a1 | +2c0
        move.l  a1,(a6)                         | +2c4
.L055f9e:
        tst.b   0x10e39e.l                      | +2c6
        beq.w   .L055fae                        | +2cc
        lea     Grenade_Explode_055fca__L056012(pc),a1 | +2d0
        move.l  a1,(a6)                         | +2d4
.L055fae:
        movea.l #0xffffffff,a0                  | +2d6
        lea     Grenade_ProbeBox_055cc8(pc),a0  | +2dc
        jsr     0x5dd56.l                       | +2e0
        bcc.w   SetHandlerRts_055fc8            | +2e6

| ----------------------------------------------------------------------------
|  Grenade_Explode_055fca  @ $055FCA  (142 B)
| ----------------------------------------------------------------------------
        .section .text.Grenade_Explode_055fca, "ax", @progbits
        .global Grenade_Explode_055fca
Grenade_Explode_055fca:
        move.w  #0x2000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x10,0x38(a6)                  | +010
        move.w  #0x1027,d0                      | +016
        jsr     0x2352.l                        | +01a
        bra.w   .L056032                        | +020
        .global Grenade_Explode_055fca__L055fee
Grenade_Explode_055fca__L055fee:
.L055fee:
        move.w  #0x8000,d0                      | +024
        jsr     0x28134.l                       | +028
        andi.w  #0xffe3,0x38(a6)                | +02e
        ori.w   #0x4,0x38(a6)                   | +034
        move.w  #0x1027,d0                      | +03a
        jsr     0x2352.l                        | +03e
        bra.w   .L056032                        | +044
        .global Grenade_Explode_055fca__L056012
Grenade_Explode_055fca__L056012:
.L056012:
        move.w  #0x8000,d0                      | +048
        jsr     0x28134.l                       | +04c
        andi.w  #0xffe3,0x38(a6)                | +052
        ori.w   #0x10,0x38(a6)                  | +058
        move.w  #0xffff,d0                      | +05e
        jsr     0x2352.l                        | +062
.L056032:
        move.l  #0xffffffff,0x48(a6)            | +068
        bclr    #0x1,0x13(a6)                   | +070
        bclr    #0x3,0x13(a6)                   | +076
        bclr    #0x0,0x13(a6)                   | +07c
        jsr     0x13600.l                       | +082
        jmp     0x77eda.l                       | +088

| ----------------------------------------------------------------------------
|  Bounce_SpawnFromParent_056066  @ $056066  (40 B)
| ----------------------------------------------------------------------------
        .section .text.Bounce_SpawnFromParent_056066, "ax", @progbits
        .global Bounce_SpawnFromParent_056066
Bounce_SpawnFromParent_056066:
        move.l  a6,-(a7)                        | +000
        lea     0x100800.l,a6                   | +002
        lea     Bounce_Task_05608e(pc),a1       | +008
        jsr     0x4ae.l                         | +00c
        movea.l (a7)+,a6                        | +012
        move.w  0x22(a6),0x22(a0)               | +014
        move.w  0x24(a6),0x24(a0)               | +01a
        move.b  0x3a(a6),0x3a(a0)               | +020
        rts                                     | +026

| ----------------------------------------------------------------------------
|  Bounce_Task_05608e  @ $05608E  (228 B)
| ----------------------------------------------------------------------------
        .section .text.Bounce_Task_05608e, "ax", @progbits
        .global Bounce_Task_05608e
Bounce_Task_05608e:
        bset    #0x2,0x5b(a6)                   | +000
        bset    #0x4,0x6b(a6)                   | +006
        move.w  #0xd000,d0                      | +00c
        jsr     0x28134.l                       | +010
        andi.w  #0xffe3,0x38(a6)                | +016
        ori.w   #0x10,0x38(a6)                  | +01c
        lea     0x29cda4.l,a0                   | +022
        jsr     0x28cd4.l                       | +028
        jsr     0x13600.l                       | +02e
        move.w  #0x12,d1                        | +034
        jsr     0x236e.l                        | +038
        move.w  #0x13,d1                        | +03e
        jsr     0x236e.l                        | +042
        addi.w  #0x18,0x24(a6)                  | +048
        moveq   #24,d0                          | +04e
        move.w  #0x180,d1                       | +050
        btst    #0x0,0x3a(a6)                   | +054
        bne.w   .L0560f0                        | +05a
        neg.w   d0                              | +05e
        neg.w   d1                              | +060
.L0560f0:
        add.w   d0,0x22(a6)                     | +062
        move.w  d1,0x28(a6)                     | +066
        clr.w   0x2e(a6)                        | +06a
        clr.w   0x2c(a6)                        | +06e
        jsr     0x283ca.l                       | +072
        lea     .L05610c(pc),a1                 | +078
        move.l  a1,(a6)                         | +07c
.L05610c:
        jsr     0x27c8c.l                       | +07e
        bcc.w   .L05611c                        | +084
        lea     Bounce_Rest_05617a(pc),a1       | +088
        move.l  a1,(a6)                         | +08c
.L05611c:
        jsr     0x28d70.l                       | +08e
        .global Bounce_Task_05608e__L056122
Bounce_Task_05608e__L056122:
.L056122:
        jsr     0x2870a.l                       | +094
        bcc.w   .L056132                        | +09a
        lea     Bounce_Fizzle_0561da(pc),a1     | +09e
        move.l  a1,(a6)                         | +0a2
.L056132:
        subq.b  #0x1,0x5c(a6)                   | +0a4
        move.b  0x5c(a6),d0                     | +0a8
        andi.b  #0x3,d0                         | +0ac
        bne.w   .L056158                        | +0b0
        move.w  0x16(a6),d0                     | +0b4
        move.w  0x18(a6),d1                     | +0b8
        exg     d0,d1                           | +0bc
        move.w  d0,0x16(a6)                     | +0be
        move.w  d1,0x18(a6)                     | +0c2
        move.w  d0,0x14(a6)                     | +0c6
.L056158:
        movea.l #0xffffffff,a0                  | +0ca
        lea     Grenade_ProbeBox_055cc8(pc),a0  | +0d0
        jsr     0x5dd56.l                       | +0d4
        bcc.w   JsrAbsThunk_056172              | +0da
        jmp     0x518.l                         | +0de

| ----------------------------------------------------------------------------
|  Bounce_Rest_05617a  @ $05617A  (52 B)
| ----------------------------------------------------------------------------
        .section .text.Bounce_Rest_05617a, "ax", @progbits
        .global Bounce_Rest_05617a
Bounce_Rest_05617a:
        lea     0x29cdb6.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        clr.w   0x28(a6)                        | +00c
        clr.w   0x2a(a6)                        | +010
        lea     .L056194(pc),a1                 | +014
        move.l  a1,(a6)                         | +018
.L056194:
        jsr     0x27c8c.l                       | +01a
        jsr     0x28d70.l                       | +020
        bcc.w   .L0561aa                        | +026
        lea     Bounce_Rest2_0561ae(pc),a1      | +02a
        move.l  a1,(a6)                         | +02e
.L0561aa:
        bra.w   Bounce_Task_05608e__L056122     | +030

| ----------------------------------------------------------------------------
|  Bounce_Rest2_0561ae  @ $0561AE  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Bounce_Rest2_0561ae, "ax", @progbits
        .global Bounce_Rest2_0561ae
Bounce_Rest2_0561ae:
        lea     0x29cfa8.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L0561c0(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L0561c0:
        jsr     0x27c8c.l                       | +012
        jsr     0x28d70.l                       | +018
        bcc.w   .L0561d6                        | +01e
        lea     Bounce_Fizzle_0561da(pc),a1     | +022
        move.l  a1,(a6)                         | +026
.L0561d6:
        bra.w   Bounce_Task_05608e__L056122     | +028

| ----------------------------------------------------------------------------
|  Bounce_Fizzle_0561da  @ $0561DA  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Bounce_Fizzle_0561da, "ax", @progbits
        .global Bounce_Fizzle_0561da
Bounce_Fizzle_0561da:
        lea     0x29cfd2.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L0561ec(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L0561ec:
        jsr     0x27c8c.l                       | +012
        jsr     0x28d70.l                       | +018
        bcc.w   SetHandlerRts_056202            | +01e

| ----------------------------------------------------------------------------
|  Bounce_Explode_056204  @ $056204  (104 B)
| ----------------------------------------------------------------------------
        .section .text.Bounce_Explode_056204, "ax", @progbits
        .global Bounce_Explode_056204
Bounce_Explode_056204:
        move.w  #0x1022,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x29cca6.l,a0                   | +00a
        move.l  a0,0x4c(a6)                     | +010
        jsr     0x283ca.l                       | +014
        jsr     0x283d8.l                       | +01a
        jsr     0x13600.l                       | +020
        move.w  #0xc000,d0                      | +026
        jsr     0x28134.l                       | +02a
        andi.w  #0xffe3,0x38(a6)                | +030
        ori.w   #0x10,0x38(a6)                  | +036
        move.w  #0x16b,d1                       | +03c
        jsr     0x236e.l                        | +040
        lea     0x29d01a.l,a0                   | +046
        jsr     0x28cd4.l                       | +04c
        lea     .L05625c(pc),a1                 | +052
        move.l  a1,(a6)                         | +056
.L05625c:
        jsr     0x27c8c.l                       | +058
        jsr     0x28d70.l                       | +05e
        bcc.w   Jsr5B6Rts_056278                | +064

| ----------------------------------------------------------------------------
|  Mortar_HitboxList_05627a  @ $05627A  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Mortar_HitboxList_05627a, "ax", @progbits
        .global Mortar_HitboxList_05627a
Mortar_HitboxList_05627a:
        .dc.b   0x00                          | +000  '.'  (dato, rango --data)
        .dc.b   0x03                          | +001  '.'  (dato, rango --data)
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
        .dc.b   0x00                          | +00e  '.'  (dato, rango --data)
        .dc.b   0x00                          | +00f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +010  '.'  (dato, rango --data)
        .dc.b   0x10                          | +011  '.'  (dato, rango --data)
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
        .dc.b   0x10                          | +029  '.'  (dato, rango --data)
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
        .dc.b   0x00                          | +034  '.'  (dato, rango --data)
        .dc.b   0x00                          | +035  '.'  (dato, rango --data)
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
        .dc.b   0x10                          | +041  '.'  (dato, rango --data)
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
        .dc.b   0x00                          | +04c  '.'  (dato, rango --data)
        .dc.b   0x00                          | +04d  '.'  (dato, rango --data)
        .dc.b   0x1d                          | +04e  '.'  (dato, rango --data)
        .dc.b   0x01                          | +04f  '.'  (dato, rango --data)
        .dc.b   0xff                          | +050  '.'  (dato, rango --data)
        .dc.b   0xff                          | +051  '.'  (dato, rango --data)
        .dc.b   0xff                          | +052  '.'  (dato, rango --data)
        .dc.b   0xff                          | +053  '.'  (dato, rango --data)

| ----------------------------------------------------------------------------
|  Mortar_HitboxList_B_0562ce  @ $0562CE  (170 B)
| ----------------------------------------------------------------------------
        .section .text.Mortar_HitboxList_B_0562ce, "ax", @progbits
        .global Mortar_HitboxList_B_0562ce
Mortar_HitboxList_B_0562ce:
        .dc.b   0x00                          | +000  '.'  (dato, rango --data)
        .dc.b   0x06                          | +001  '.'  (dato, rango --data)
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
        .dc.b   0x00                          | +00e  '.'  (dato, rango --data)
        .dc.b   0x00                          | +00f  '.'  (dato, rango --data)
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
        .dc.b   0x00                          | +034  '.'  (dato, rango --data)
        .dc.b   0x00                          | +035  '.'  (dato, rango --data)
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
        .dc.b   0x00                          | +04c  '.'  (dato, rango --data)
        .dc.b   0x00                          | +04d  '.'  (dato, rango --data)
        .dc.b   0x1d                          | +04e  '.'  (dato, rango --data)
        .dc.b   0x00                          | +04f  '.'  (dato, rango --data)
        .dc.b   0x04                          | +050  '.'  (dato, rango --data)
        .dc.b   0x0b                          | +051  '.'  (dato, rango --data)
        .dc.b   0x00                          | +052  '.'  (dato, rango --data)
        .dc.b   0x01                          | +053  '.'  (dato, rango --data)
        .dc.b   0x02                          | +054  '.'  (dato, rango --data)
        .dc.b   0x04                          | +055  '.'  (dato, rango --data)
        .dc.b   0xff                          | +056  '.'  (dato, rango --data)
        .dc.b   0x00                          | +057  '.'  (dato, rango --data)
        .dc.b   0x00                          | +058  '.'  (dato, rango --data)
        .dc.b   0x00                          | +059  '.'  (dato, rango --data)
        .dc.b   0xff                          | +05a  '.'  (dato, rango --data)
        .dc.b   0xf8                          | +05b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +05c  '.'  (dato, rango --data)
        .dc.b   0x08                          | +05d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +05e  '.'  (dato, rango --data)
        .dc.b   0x00                          | +05f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +060  '.'  (dato, rango --data)
        .dc.b   0x08                          | +061  '.'  (dato, rango --data)
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
        .dc.b   0xf8                          | +077  '.'  (dato, rango --data)
        .dc.b   0x00                          | +078  '.'  (dato, rango --data)
        .dc.b   0x08                          | +079  '.'  (dato, rango --data)
        .dc.b   0x02                          | +07a  '.'  (dato, rango --data)
        .dc.b   0x00                          | +07b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +07c  '.'  (dato, rango --data)
        .dc.b   0x24                          | +07d  '$'  (dato, rango --data)
        .dc.b   0x36                          | +07e  '6'  (dato, rango --data)
        .dc.b   0x4e                          | +07f  'N'  (dato, rango --data)
        .dc.b   0xff                          | +080  '.'  (dato, rango --data)
        .dc.b   0xff                          | +081  '.'  (dato, rango --data)
        .dc.b   0xff                          | +082  '.'  (dato, rango --data)
        .dc.b   0xf8                          | +083  '.'  (dato, rango --data)
        .dc.b   0x00                          | +084  '.'  (dato, rango --data)
        .dc.b   0x00                          | +085  '.'  (dato, rango --data)
        .dc.b   0x02                          | +086  '.'  (dato, rango --data)
        .dc.b   0x00                          | +087  '.'  (dato, rango --data)
        .dc.b   0x00                          | +088  '.'  (dato, rango --data)
        .dc.b   0x24                          | +089  '$'  (dato, rango --data)
        .dc.b   0x36                          | +08a  '6'  (dato, rango --data)
        .dc.b   0x58                          | +08b  'X'  (dato, rango --data)
        .dc.b   0xff                          | +08c  '.'  (dato, rango --data)
        .dc.b   0xff                          | +08d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +08e  '.'  (dato, rango --data)
        .dc.b   0x08                          | +08f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +090  '.'  (dato, rango --data)
        .dc.b   0x08                          | +091  '.'  (dato, rango --data)
        .dc.b   0x02                          | +092  '.'  (dato, rango --data)
        .dc.b   0x00                          | +093  '.'  (dato, rango --data)
        .dc.b   0x00                          | +094  '.'  (dato, rango --data)
        .dc.b   0x24                          | +095  '$'  (dato, rango --data)
        .dc.b   0x36                          | +096  '6'  (dato, rango --data)
        .dc.b   0x62                          | +097  'b'  (dato, rango --data)
        .dc.b   0xff                          | +098  '.'  (dato, rango --data)
        .dc.b   0xff                          | +099  '.'  (dato, rango --data)
        .dc.b   0x00                          | +09a  '.'  (dato, rango --data)
        .dc.b   0x08                          | +09b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +09c  '.'  (dato, rango --data)
        .dc.b   0x00                          | +09d  '.'  (dato, rango --data)
        .dc.b   0x1d                          | +09e  '.'  (dato, rango --data)
        .dc.b   0x00                          | +09f  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0a0  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0a1  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0a2  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0a3  '.'  (dato, rango --data)
        .dc.b   0x0a                          | +0a4  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0a5  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0a6  '.'  (dato, rango --data)
        .dc.b   0x05                          | +0a7  '.'  (dato, rango --data)
        .dc.b   0x62                          | +0a8  'b'  (dato, rango --data)
        .dc.b   0xce                          | +0a9  '.'  (dato, rango --data)

| ----------------------------------------------------------------------------
|  SpriteMap_Mortar_056378  @ $056378  (124 B)
| ----------------------------------------------------------------------------
        .section .text.SpriteMap_Mortar_056378, "ax", @progbits
        .global SpriteMap_Mortar_056378
SpriteMap_Mortar_056378:
        .dc.b   0x09                          | +000  '.'  (dato, rango --data)
        .dc.b   0x00                          | +001  '.'  (dato, rango --data)
        .dc.b   0x00                          | +002  '.'  (dato, rango --data)
        .dc.b   0x05                          | +003  '.'  (dato, rango --data)
        .dc.b   0x62                          | +004  'b'  (dato, rango --data)
        .dc.b   0x7a                          | +005  'z'  (dato, rango --data)
        .dc.b   0x00                          | +006  '.'  (dato, rango --data)
        .dc.b   0x02                          | +007  '.'  (dato, rango --data)
        .dc.b   0x02                          | +008  '.'  (dato, rango --data)
        .dc.b   0x00                          | +009  '.'  (dato, rango --data)
        .dc.b   0x00                          | +00a  '.'  (dato, rango --data)
        .dc.b   0x23                          | +00b  '#'  (dato, rango --data)
        .dc.b   0x32                          | +00c  '2'  (dato, rango --data)
        .dc.b   0x08                          | +00d  '.'  (dato, rango --data)
        .dc.b   0xff                          | +00e  '.'  (dato, rango --data)
        .dc.b   0xff                          | +00f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +010  '.'  (dato, rango --data)
        .dc.b   0x00                          | +011  '.'  (dato, rango --data)
        .dc.b   0x00                          | +012  '.'  (dato, rango --data)
        .dc.b   0x07                          | +013  '.'  (dato, rango --data)
        .dc.b   0x00                          | +014  '.'  (dato, rango --data)
        .dc.b   0x02                          | +015  '.'  (dato, rango --data)
        .dc.b   0x02                          | +016  '.'  (dato, rango --data)
        .dc.b   0x00                          | +017  '.'  (dato, rango --data)
        .dc.b   0x00                          | +018  '.'  (dato, rango --data)
        .dc.b   0x23                          | +019  '#'  (dato, rango --data)
        .dc.b   0x32                          | +01a  '2'  (dato, rango --data)
        .dc.b   0x12                          | +01b  '.'  (dato, rango --data)
        .dc.b   0xff                          | +01c  '.'  (dato, rango --data)
        .dc.b   0xff                          | +01d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +01e  '.'  (dato, rango --data)
        .dc.b   0x00                          | +01f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +020  '.'  (dato, rango --data)
        .dc.b   0x07                          | +021  '.'  (dato, rango --data)
        .dc.b   0x00                          | +022  '.'  (dato, rango --data)
        .dc.b   0x02                          | +023  '.'  (dato, rango --data)
        .dc.b   0x02                          | +024  '.'  (dato, rango --data)
        .dc.b   0x00                          | +025  '.'  (dato, rango --data)
        .dc.b   0x00                          | +026  '.'  (dato, rango --data)
        .dc.b   0x23                          | +027  '#'  (dato, rango --data)
        .dc.b   0x32                          | +028  '2'  (dato, rango --data)
        .dc.b   0x1c                          | +029  '.'  (dato, rango --data)
        .dc.b   0xff                          | +02a  '.'  (dato, rango --data)
        .dc.b   0xff                          | +02b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +02c  '.'  (dato, rango --data)
        .dc.b   0x00                          | +02d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +02e  '.'  (dato, rango --data)
        .dc.b   0x07                          | +02f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +030  '.'  (dato, rango --data)
        .dc.b   0x02                          | +031  '.'  (dato, rango --data)
        .dc.b   0x02                          | +032  '.'  (dato, rango --data)
        .dc.b   0x00                          | +033  '.'  (dato, rango --data)
        .dc.b   0x00                          | +034  '.'  (dato, rango --data)
        .dc.b   0x23                          | +035  '#'  (dato, rango --data)
        .dc.b   0x32                          | +036  '2'  (dato, rango --data)
        .dc.b   0x26                          | +037  '&'  (dato, rango --data)
        .dc.b   0xff                          | +038  '.'  (dato, rango --data)
        .dc.b   0xff                          | +039  '.'  (dato, rango --data)
        .dc.b   0x00                          | +03a  '.'  (dato, rango --data)
        .dc.b   0x00                          | +03b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +03c  '.'  (dato, rango --data)
        .dc.b   0x07                          | +03d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +03e  '.'  (dato, rango --data)
        .dc.b   0x02                          | +03f  '.'  (dato, rango --data)
        .dc.b   0x02                          | +040  '.'  (dato, rango --data)
        .dc.b   0x00                          | +041  '.'  (dato, rango --data)
        .dc.b   0x00                          | +042  '.'  (dato, rango --data)
        .dc.b   0x23                          | +043  '#'  (dato, rango --data)
        .dc.b   0x32                          | +044  '2'  (dato, rango --data)
        .dc.b   0x30                          | +045  '0'  (dato, rango --data)
        .dc.b   0xff                          | +046  '.'  (dato, rango --data)
        .dc.b   0xff                          | +047  '.'  (dato, rango --data)
        .dc.b   0x00                          | +048  '.'  (dato, rango --data)
        .dc.b   0x00                          | +049  '.'  (dato, rango --data)
        .dc.b   0x00                          | +04a  '.'  (dato, rango --data)
        .dc.b   0x07                          | +04b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +04c  '.'  (dato, rango --data)
        .dc.b   0x02                          | +04d  '.'  (dato, rango --data)
        .dc.b   0x02                          | +04e  '.'  (dato, rango --data)
        .dc.b   0x00                          | +04f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +050  '.'  (dato, rango --data)
        .dc.b   0x23                          | +051  '#'  (dato, rango --data)
        .dc.b   0x32                          | +052  '2'  (dato, rango --data)
        .dc.b   0x3a                          | +053  ':'  (dato, rango --data)
        .dc.b   0xff                          | +054  '.'  (dato, rango --data)
        .dc.b   0xff                          | +055  '.'  (dato, rango --data)
        .dc.b   0x00                          | +056  '.'  (dato, rango --data)
        .dc.b   0x00                          | +057  '.'  (dato, rango --data)
        .dc.b   0x00                          | +058  '.'  (dato, rango --data)
        .dc.b   0x07                          | +059  '.'  (dato, rango --data)
        .dc.b   0x00                          | +05a  '.'  (dato, rango --data)
        .dc.b   0x02                          | +05b  '.'  (dato, rango --data)
        .dc.b   0x02                          | +05c  '.'  (dato, rango --data)
        .dc.b   0x00                          | +05d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +05e  '.'  (dato, rango --data)
        .dc.b   0x23                          | +05f  '#'  (dato, rango --data)
        .dc.b   0x32                          | +060  '2'  (dato, rango --data)
        .dc.b   0x44                          | +061  'D'  (dato, rango --data)
        .dc.b   0xff                          | +062  '.'  (dato, rango --data)
        .dc.b   0xff                          | +063  '.'  (dato, rango --data)
        .dc.b   0x00                          | +064  '.'  (dato, rango --data)
        .dc.b   0x00                          | +065  '.'  (dato, rango --data)
        .dc.b   0x00                          | +066  '.'  (dato, rango --data)
        .dc.b   0x07                          | +067  '.'  (dato, rango --data)
        .dc.b   0x00                          | +068  '.'  (dato, rango --data)
        .dc.b   0x02                          | +069  '.'  (dato, rango --data)
        .dc.b   0x02                          | +06a  '.'  (dato, rango --data)
        .dc.b   0x00                          | +06b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +06c  '.'  (dato, rango --data)
        .dc.b   0x23                          | +06d  '#'  (dato, rango --data)
        .dc.b   0x32                          | +06e  '2'  (dato, rango --data)
        .dc.b   0x4e                          | +06f  'N'  (dato, rango --data)
        .dc.b   0xff                          | +070  '.'  (dato, rango --data)
        .dc.b   0xff                          | +071  '.'  (dato, rango --data)
        .dc.b   0x00                          | +072  '.'  (dato, rango --data)
        .dc.b   0x00                          | +073  '.'  (dato, rango --data)
        .dc.b   0x00                          | +074  '.'  (dato, rango --data)
        .dc.b   0x07                          | +075  '.'  (dato, rango --data)
        .dc.b   0x01                          | +076  '.'  (dato, rango --data)
        .dc.b   0x00                          | +077  '.'  (dato, rango --data)
        .dc.b   0x00                          | +078  '.'  (dato, rango --data)
        .dc.b   0x05                          | +079  '.'  (dato, rango --data)
        .dc.b   0x63                          | +07a  'c'  (dato, rango --data)
        .dc.b   0x7e                          | +07b  '~'  (dato, rango --data)

| ----------------------------------------------------------------------------
|  Mortar_ApplyDrag_0563f4  @ $0563F4  (68 B)
| ----------------------------------------------------------------------------
        .section .text.Mortar_ApplyDrag_0563f4, "ax", @progbits
        .global Mortar_ApplyDrag_0563f4
Mortar_ApplyDrag_0563f4:
        move.w  0x28(a6),d0                     | +000
        beq.w   .L056408                        | +004
        asr.w   #0x8,d0                         | +008
        bne.w   .L056404                        | +00a
        moveq   #1,d0                           | +00e
.L056404:
        sub.w   d0,0x28(a6)                     | +010
.L056408:
        jsr     0x28158.l                       | +014
        tst.b   d0                              | +01a
        beq.w   .L056436                        | +01c
        cmpi.b  #0x3,d0                         | +020
        bcc.w   .L056424                        | +024
        move.w  #0x3,d1                         | +028
        bra.w   .L056428                        | +02c
.L056424:
        move.w  #0xb,d1                         | +030
.L056428:
        andi.b  #0x1,d0                         | +034
        bne.w   .L056432                        | +038
        neg.w   d1                              | +03c
.L056432:
        sub.w   d1,0x28(a6)                     | +03e
.L056436:
        rts                                     | +042

| ----------------------------------------------------------------------------
|  Mortar_SpawnFromParent_056438  @ $056438  (52 B)
| ----------------------------------------------------------------------------
        .section .text.Mortar_SpawnFromParent_056438, "ax", @progbits
        .global Mortar_SpawnFromParent_056438
Mortar_SpawnFromParent_056438:
        move.l  a6,-(a7)                        | +000
        lea     0x100800.l,a6                   | +002
        lea     Mortar_Shell_Task_05646c(pc),a1 | +008
        jsr     0x4ae.l                         | +00c
        movea.l (a7)+,a6                        | +012
        move.w  0x22(a6),0x22(a0)               | +014
        move.w  0x24(a6),0x24(a0)               | +01a
        move.b  0x3a(a6),0x3a(a0)               | +020
        move.b  0x11(a6),0x11(a0)               | +026
        addi.w  #0x14,0x24(a0)                  | +02c
        rts                                     | +032

| ----------------------------------------------------------------------------
|  Mortar_Shell_Task_05646c  @ $05646C  (290 B)
| ----------------------------------------------------------------------------
        .section .text.Mortar_Shell_Task_05646c, "ax", @progbits
        .global Mortar_Shell_Task_05646c
Mortar_Shell_Task_05646c:
        move.w  #0xe,d1                         | +000
        jsr     0x236e.l                        | +004
        lea     Mortar_HitboxList_B_0562ce(pc),a0 | +00a
        move.l  a0,0x4c(a6)                     | +00e
        jsr     0x283ca.l                       | +012
        jsr     0x283ca.l                       | +018
        bset    #0x5,0x13(a6)                   | +01e
        lea     0x2b7632.l,a0                   | +024
        jsr     0x799de.l                       | +02a
        move.w  d0,0x66(a6)                     | +030
        move.w  #0xd000,d0                      | +034
        jsr     0x28134.l                       | +038
        andi.w  #0xffe3,0x38(a6)                | +03e
        ori.w   #0x18,0x38(a6)                  | +044
        move.w  #0xfe00,d0                      | +04a
        jsr     0x5dca4.l                       | +04e
        move.w  d0,0x28(a6)                     | +054
        move.w  #0x195,0x2a(a6)                 | +058
        move.w  #0xffaf,0x2e(a6)                | +05e
        move.w  #0x0,0x2c(a6)                   | +064
        lea     SpriteMap_Mortar_056378(pc),a0  | +06a
        jsr     0x28cd4.l                       | +06e
        lea     .L0564e6(pc),a1                 | +074
        move.l  a1,(a6)                         | +078
.L0564e6:
        jsr     0x2783a.l                       | +07a
        jsr     0x27eba.l                       | +080
        bcc.w   .L056500                        | +086
        jsr     0x28364.l                       | +08a
        bra.w   .L05650a                        | +090
.L056500:
        jsr     0x28292.l                       | +094
        bsr.w   Mortar_ApplyDrag_0563f4         | +09a
.L05650a:
        move.w  0x28(a6),d0                     | +09e
        btst    #0x0,0x3a(a6)                   | +0a2
        bne.w   .L05652a                        | +0a8
        cmpi.w  #0x0,d0                         | +0ac
        blt.w   .L056526                        | +0b0
        lea     Mortar_Shell_Explode_05659c(pc),a1 | +0b4
        move.l  a1,(a6)                         | +0b8
.L056526:
        bra.w   .L056538                        | +0ba
.L05652a:
        cmpi.w  #0x0,d0                         | +0be
        bgt.w   .L056538                        | +0c2
        lea     Mortar_Shell_Explode_05659c(pc),a1 | +0c6
        move.l  a1,(a6)                         | +0ca
.L056538:
        btst    #0x5,0x5a(a6)                   | +0cc
        beq.w   .L056548                        | +0d2
        lea     Mortar_Shell_Explode_05659c(pc),a1 | +0d6
        move.l  a1,(a6)                         | +0da
.L056548:
        jsr     0x28d70.l                       | +0dc
        jsr     0x283d8.l                       | +0e2
        btst    #0x1,0x13(a6)                   | +0e8
        beq.w   .L056564                        | +0ee
        lea     Mortar_Shell_Explode_05659c(pc),a1 | +0f2
        move.l  a1,(a6)                         | +0f6
.L056564:
        bclr    #0x3,0x13(a6)                   | +0f8
        jsr     0x28758.l                       | +0fe
        bcc.w   .L05657a                        | +104
        lea     Mortar_Shell_Explode_05659c(pc),a1 | +108
        move.l  a1,(a6)                         | +10c
.L05657a:
        movea.l #0xffffffff,a0                  | +10e
        lea     Grenade_ProbeBox_055cc8(pc),a0  | +114
        jsr     0x5dd56.l                       | +118
        bcc.w   SetHandlerRts_056594            | +11e

| ----------------------------------------------------------------------------
|  Mortar_Shell_Explode_05659c  @ $05659C  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Mortar_Shell_Explode_05659c, "ax", @progbits
        .global Mortar_Shell_Explode_05659c
Mortar_Shell_Explode_05659c:
        move.w  #0x1022,d0                      | +000
        jsr     0x2352.l                        | +004
        jsr     0x13600.l                       | +00a
        move.w  #0x2000,d0                      | +010
        jsr     0x28134.l                       | +014
        andi.w  #0xffe3,0x38(a6)                | +01a
        ori.w   #0x10,0x38(a6)                  | +020
        jmp     0x77f6a.l                       | +026

| ----------------------------------------------------------------------------
|  Roller_HitboxList_0565c8  @ $0565C8  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Roller_HitboxList_0565c8, "ax", @progbits
        .global Roller_HitboxList_0565c8
Roller_HitboxList_0565c8:
        .dc.b   0x00                          | +000  '.'  (dato, rango --data)
        .dc.b   0x06                          | +001  '.'  (dato, rango --data)
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
        .dc.b   0xfc                          | +00f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +010  '.'  (dato, rango --data)
        .dc.b   0x04                          | +011  '.'  (dato, rango --data)
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
        .dc.b   0x04                          | +029  '.'  (dato, rango --data)
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
        .dc.b   0xfc                          | +035  '.'  (dato, rango --data)
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
        .dc.b   0x04                          | +041  '.'  (dato, rango --data)
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
        .dc.b   0xfc                          | +04d  '.'  (dato, rango --data)
        .dc.b   0x1d                          | +04e  '.'  (dato, rango --data)
        .dc.b   0x00                          | +04f  '.'  (dato, rango --data)
        .dc.b   0xff                          | +050  '.'  (dato, rango --data)
        .dc.b   0xff                          | +051  '.'  (dato, rango --data)
        .dc.b   0xff                          | +052  '.'  (dato, rango --data)
        .dc.b   0xff                          | +053  '.'  (dato, rango --data)

| ----------------------------------------------------------------------------
|  Roller_HitboxList_B_05661c  @ $05661C  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Roller_HitboxList_B_05661c, "ax", @progbits
        .global Roller_HitboxList_B_05661c
Roller_HitboxList_B_05661c:
        .dc.b   0x00                          | +000  '.'  (dato, rango --data)
        .dc.b   0x03                          | +001  '.'  (dato, rango --data)
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
|  SpriteMap_Roller_056670  @ $056670  (476 B)
| ----------------------------------------------------------------------------
        .section .text.SpriteMap_Roller_056670, "ax", @progbits
        .global SpriteMap_Roller_056670
SpriteMap_Roller_056670:
        .dc.b   0x04                          | +000  '.'  (dato, rango --data)
        .dc.b   0x00                          | +001  '.'  (dato, rango --data)
        .dc.b   0x10                          | +002  '.'  (dato, rango --data)
        .dc.b   0xaa                          | +003  '.'  (dato, rango --data)
        .dc.b   0x00                          | +004  '.'  (dato, rango --data)
        .dc.b   0x01                          | +005  '.'  (dato, rango --data)
        .dc.b   0x1b                          | +006  '.'  (dato, rango --data)
        .dc.b   0x70                          | +007  'p'  (dato, rango --data)
        .dc.b   0x02                          | +008  '.'  (dato, rango --data)
        .dc.b   0x00                          | +009  '.'  (dato, rango --data)
        .dc.b   0x00                          | +00a  '.'  (dato, rango --data)
        .dc.b   0x23                          | +00b  '#'  (dato, rango --data)
        .dc.b   0x33                          | +00c  '3'  (dato, rango --data)
        .dc.b   0x6a                          | +00d  'j'  (dato, rango --data)
        .dc.b   0xff                          | +00e  '.'  (dato, rango --data)
        .dc.b   0xff                          | +00f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +010  '.'  (dato, rango --data)
        .dc.b   0x00                          | +011  '.'  (dato, rango --data)
        .dc.b   0x00                          | +012  '.'  (dato, rango --data)
        .dc.b   0x00                          | +013  '.'  (dato, rango --data)
        .dc.b   0x01                          | +014  '.'  (dato, rango --data)
        .dc.b   0x00                          | +015  '.'  (dato, rango --data)
        .dc.b   0x00                          | +016  '.'  (dato, rango --data)
        .dc.b   0x05                          | +017  '.'  (dato, rango --data)
        .dc.b   0x66                          | +018  'f'  (dato, rango --data)
        .dc.b   0xae                          | +019  '.'  (dato, rango --data)
        .dc.b   0x02                          | +01a  '.'  (dato, rango --data)
        .dc.b   0x00                          | +01b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +01c  '.'  (dato, rango --data)
        .dc.b   0x23                          | +01d  '#'  (dato, rango --data)
        .dc.b   0x3c                          | +01e  '<'  (dato, rango --data)
        .dc.b   0x3e                          | +01f  '>'  (dato, rango --data)
        .dc.b   0xff                          | +020  '.'  (dato, rango --data)
        .dc.b   0xff                          | +021  '.'  (dato, rango --data)
        .dc.b   0x00                          | +022  '.'  (dato, rango --data)
        .dc.b   0x00                          | +023  '.'  (dato, rango --data)
        .dc.b   0x00                          | +024  '.'  (dato, rango --data)
        .dc.b   0x00                          | +025  '.'  (dato, rango --data)
        .dc.b   0x01                          | +026  '.'  (dato, rango --data)
        .dc.b   0x00                          | +027  '.'  (dato, rango --data)
        .dc.b   0x00                          | +028  '.'  (dato, rango --data)
        .dc.b   0x05                          | +029  '.'  (dato, rango --data)
        .dc.b   0x66                          | +02a  'f'  (dato, rango --data)
        .dc.b   0xae                          | +02b  '.'  (dato, rango --data)
        .dc.b   0x02                          | +02c  '.'  (dato, rango --data)
        .dc.b   0x00                          | +02d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +02e  '.'  (dato, rango --data)
        .dc.b   0x23                          | +02f  '#'  (dato, rango --data)
        .dc.b   0x33                          | +030  '3'  (dato, rango --data)
        .dc.b   0x1a                          | +031  '.'  (dato, rango --data)
        .dc.b   0xff                          | +032  '.'  (dato, rango --data)
        .dc.b   0xff                          | +033  '.'  (dato, rango --data)
        .dc.b   0x00                          | +034  '.'  (dato, rango --data)
        .dc.b   0x00                          | +035  '.'  (dato, rango --data)
        .dc.b   0x00                          | +036  '.'  (dato, rango --data)
        .dc.b   0x00                          | +037  '.'  (dato, rango --data)
        .dc.b   0x01                          | +038  '.'  (dato, rango --data)
        .dc.b   0x00                          | +039  '.'  (dato, rango --data)
        .dc.b   0x00                          | +03a  '.'  (dato, rango --data)
        .dc.b   0x05                          | +03b  '.'  (dato, rango --data)
        .dc.b   0x66                          | +03c  'f'  (dato, rango --data)
        .dc.b   0xae                          | +03d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +03e  '.'  (dato, rango --data)
        .dc.b   0x01                          | +03f  '.'  (dato, rango --data)
        .dc.b   0x1b                          | +040  '.'  (dato, rango --data)
        .dc.b   0x70                          | +041  'p'  (dato, rango --data)
        .dc.b   0x02                          | +042  '.'  (dato, rango --data)
        .dc.b   0x00                          | +043  '.'  (dato, rango --data)
        .dc.b   0x00                          | +044  '.'  (dato, rango --data)
        .dc.b   0x23                          | +045  '#'  (dato, rango --data)
        .dc.b   0x33                          | +046  '3'  (dato, rango --data)
        .dc.b   0x76                          | +047  'v'  (dato, rango --data)
        .dc.b   0xff                          | +048  '.'  (dato, rango --data)
        .dc.b   0xff                          | +049  '.'  (dato, rango --data)
        .dc.b   0x00                          | +04a  '.'  (dato, rango --data)
        .dc.b   0x00                          | +04b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +04c  '.'  (dato, rango --data)
        .dc.b   0x00                          | +04d  '.'  (dato, rango --data)
        .dc.b   0x01                          | +04e  '.'  (dato, rango --data)
        .dc.b   0x00                          | +04f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +050  '.'  (dato, rango --data)
        .dc.b   0x05                          | +051  '.'  (dato, rango --data)
        .dc.b   0x66                          | +052  'f'  (dato, rango --data)
        .dc.b   0xe8                          | +053  '.'  (dato, rango --data)
        .dc.b   0x02                          | +054  '.'  (dato, rango --data)
        .dc.b   0x00                          | +055  '.'  (dato, rango --data)
        .dc.b   0x00                          | +056  '.'  (dato, rango --data)
        .dc.b   0x23                          | +057  '#'  (dato, rango --data)
        .dc.b   0x3c                          | +058  '<'  (dato, rango --data)
        .dc.b   0x4a                          | +059  'J'  (dato, rango --data)
        .dc.b   0xff                          | +05a  '.'  (dato, rango --data)
        .dc.b   0xff                          | +05b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +05c  '.'  (dato, rango --data)
        .dc.b   0x00                          | +05d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +05e  '.'  (dato, rango --data)
        .dc.b   0x00                          | +05f  '.'  (dato, rango --data)
        .dc.b   0x01                          | +060  '.'  (dato, rango --data)
        .dc.b   0x00                          | +061  '.'  (dato, rango --data)
        .dc.b   0x00                          | +062  '.'  (dato, rango --data)
        .dc.b   0x05                          | +063  '.'  (dato, rango --data)
        .dc.b   0x66                          | +064  'f'  (dato, rango --data)
        .dc.b   0xe8                          | +065  '.'  (dato, rango --data)
        .dc.b   0x02                          | +066  '.'  (dato, rango --data)
        .dc.b   0x00                          | +067  '.'  (dato, rango --data)
        .dc.b   0x00                          | +068  '.'  (dato, rango --data)
        .dc.b   0x23                          | +069  '#'  (dato, rango --data)
        .dc.b   0x33                          | +06a  '3'  (dato, rango --data)
        .dc.b   0x26                          | +06b  '&'  (dato, rango --data)
        .dc.b   0xff                          | +06c  '.'  (dato, rango --data)
        .dc.b   0xff                          | +06d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +06e  '.'  (dato, rango --data)
        .dc.b   0x00                          | +06f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +070  '.'  (dato, rango --data)
        .dc.b   0x00                          | +071  '.'  (dato, rango --data)
        .dc.b   0x01                          | +072  '.'  (dato, rango --data)
        .dc.b   0x00                          | +073  '.'  (dato, rango --data)
        .dc.b   0x00                          | +074  '.'  (dato, rango --data)
        .dc.b   0x05                          | +075  '.'  (dato, rango --data)
        .dc.b   0x66                          | +076  'f'  (dato, rango --data)
        .dc.b   0xe8                          | +077  '.'  (dato, rango --data)
        .dc.b   0x00                          | +078  '.'  (dato, rango --data)
        .dc.b   0x01                          | +079  '.'  (dato, rango --data)
        .dc.b   0x1b                          | +07a  '.'  (dato, rango --data)
        .dc.b   0x70                          | +07b  'p'  (dato, rango --data)
        .dc.b   0x02                          | +07c  '.'  (dato, rango --data)
        .dc.b   0x00                          | +07d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +07e  '.'  (dato, rango --data)
        .dc.b   0x23                          | +07f  '#'  (dato, rango --data)
        .dc.b   0x33                          | +080  '3'  (dato, rango --data)
        .dc.b   0x82                          | +081  '.'  (dato, rango --data)
        .dc.b   0xff                          | +082  '.'  (dato, rango --data)
        .dc.b   0xff                          | +083  '.'  (dato, rango --data)
        .dc.b   0x00                          | +084  '.'  (dato, rango --data)
        .dc.b   0x00                          | +085  '.'  (dato, rango --data)
        .dc.b   0x00                          | +086  '.'  (dato, rango --data)
        .dc.b   0x00                          | +087  '.'  (dato, rango --data)
        .dc.b   0x01                          | +088  '.'  (dato, rango --data)
        .dc.b   0x00                          | +089  '.'  (dato, rango --data)
        .dc.b   0x00                          | +08a  '.'  (dato, rango --data)
        .dc.b   0x05                          | +08b  '.'  (dato, rango --data)
        .dc.b   0x67                          | +08c  'g'  (dato, rango --data)
        .dc.b   0x22                          | +08d  '"'  (dato, rango --data)
        .dc.b   0x02                          | +08e  '.'  (dato, rango --data)
        .dc.b   0x00                          | +08f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +090  '.'  (dato, rango --data)
        .dc.b   0x23                          | +091  '#'  (dato, rango --data)
        .dc.b   0x3c                          | +092  '<'  (dato, rango --data)
        .dc.b   0x56                          | +093  'V'  (dato, rango --data)
        .dc.b   0xff                          | +094  '.'  (dato, rango --data)
        .dc.b   0xff                          | +095  '.'  (dato, rango --data)
        .dc.b   0x00                          | +096  '.'  (dato, rango --data)
        .dc.b   0x00                          | +097  '.'  (dato, rango --data)
        .dc.b   0x00                          | +098  '.'  (dato, rango --data)
        .dc.b   0x00                          | +099  '.'  (dato, rango --data)
        .dc.b   0x01                          | +09a  '.'  (dato, rango --data)
        .dc.b   0x00                          | +09b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +09c  '.'  (dato, rango --data)
        .dc.b   0x05                          | +09d  '.'  (dato, rango --data)
        .dc.b   0x67                          | +09e  'g'  (dato, rango --data)
        .dc.b   0x22                          | +09f  '"'  (dato, rango --data)
        .dc.b   0x02                          | +0a0  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0a1  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0a2  '.'  (dato, rango --data)
        .dc.b   0x23                          | +0a3  '#'  (dato, rango --data)
        .dc.b   0x33                          | +0a4  '3'  (dato, rango --data)
        .dc.b   0x32                          | +0a5  '2'  (dato, rango --data)
        .dc.b   0xff                          | +0a6  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0a7  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0a8  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0a9  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0aa  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0ab  '.'  (dato, rango --data)
        .dc.b   0x01                          | +0ac  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0ad  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0ae  '.'  (dato, rango --data)
        .dc.b   0x05                          | +0af  '.'  (dato, rango --data)
        .dc.b   0x67                          | +0b0  'g'  (dato, rango --data)
        .dc.b   0x22                          | +0b1  '"'  (dato, rango --data)
        .dc.b   0x00                          | +0b2  '.'  (dato, rango --data)
        .dc.b   0x01                          | +0b3  '.'  (dato, rango --data)
        .dc.b   0x1b                          | +0b4  '.'  (dato, rango --data)
        .dc.b   0x70                          | +0b5  'p'  (dato, rango --data)
        .dc.b   0x02                          | +0b6  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0b7  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0b8  '.'  (dato, rango --data)
        .dc.b   0x23                          | +0b9  '#'  (dato, rango --data)
        .dc.b   0x33                          | +0ba  '3'  (dato, rango --data)
        .dc.b   0x8e                          | +0bb  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0bc  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0bd  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0be  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0bf  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0c0  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0c1  '.'  (dato, rango --data)
        .dc.b   0x01                          | +0c2  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0c3  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0c4  '.'  (dato, rango --data)
        .dc.b   0x05                          | +0c5  '.'  (dato, rango --data)
        .dc.b   0x67                          | +0c6  'g'  (dato, rango --data)
        .dc.b   0x5c                          | +0c7  '.'  (dato, rango --data)
        .dc.b   0x02                          | +0c8  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0c9  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0ca  '.'  (dato, rango --data)
        .dc.b   0x23                          | +0cb  '#'  (dato, rango --data)
        .dc.b   0x3c                          | +0cc  '<'  (dato, rango --data)
        .dc.b   0x62                          | +0cd  'b'  (dato, rango --data)
        .dc.b   0xff                          | +0ce  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0cf  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0d0  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0d1  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0d2  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0d3  '.'  (dato, rango --data)
        .dc.b   0x01                          | +0d4  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0d5  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0d6  '.'  (dato, rango --data)
        .dc.b   0x05                          | +0d7  '.'  (dato, rango --data)
        .dc.b   0x67                          | +0d8  'g'  (dato, rango --data)
        .dc.b   0x5c                          | +0d9  '.'  (dato, rango --data)
        .dc.b   0x02                          | +0da  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0db  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0dc  '.'  (dato, rango --data)
        .dc.b   0x23                          | +0dd  '#'  (dato, rango --data)
        .dc.b   0x33                          | +0de  '3'  (dato, rango --data)
        .dc.b   0x42                          | +0df  'B'  (dato, rango --data)
        .dc.b   0xff                          | +0e0  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0e1  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0e2  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0e3  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0e4  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0e5  '.'  (dato, rango --data)
        .dc.b   0x01                          | +0e6  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0e7  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0e8  '.'  (dato, rango --data)
        .dc.b   0x05                          | +0e9  '.'  (dato, rango --data)
        .dc.b   0x67                          | +0ea  'g'  (dato, rango --data)
        .dc.b   0x5c                          | +0eb  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0ec  '.'  (dato, rango --data)
        .dc.b   0x01                          | +0ed  '.'  (dato, rango --data)
        .dc.b   0x1b                          | +0ee  '.'  (dato, rango --data)
        .dc.b   0x70                          | +0ef  'p'  (dato, rango --data)
        .dc.b   0x02                          | +0f0  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0f1  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0f2  '.'  (dato, rango --data)
        .dc.b   0x23                          | +0f3  '#'  (dato, rango --data)
        .dc.b   0x33                          | +0f4  '3'  (dato, rango --data)
        .dc.b   0x6a                          | +0f5  'j'  (dato, rango --data)
        .dc.b   0xff                          | +0f6  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0f7  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0f8  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0f9  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0fa  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0fb  '.'  (dato, rango --data)
        .dc.b   0x01                          | +0fc  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0fd  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0fe  '.'  (dato, rango --data)
        .dc.b   0x05                          | +0ff  '.'  (dato, rango --data)
        .dc.b   0x67                          | +100  'g'  (dato, rango --data)
        .dc.b   0x96                          | +101  '.'  (dato, rango --data)
        .dc.b   0x02                          | +102  '.'  (dato, rango --data)
        .dc.b   0x00                          | +103  '.'  (dato, rango --data)
        .dc.b   0x00                          | +104  '.'  (dato, rango --data)
        .dc.b   0x23                          | +105  '#'  (dato, rango --data)
        .dc.b   0x3c                          | +106  '<'  (dato, rango --data)
        .dc.b   0x3e                          | +107  '>'  (dato, rango --data)
        .dc.b   0xff                          | +108  '.'  (dato, rango --data)
        .dc.b   0xff                          | +109  '.'  (dato, rango --data)
        .dc.b   0x00                          | +10a  '.'  (dato, rango --data)
        .dc.b   0x00                          | +10b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +10c  '.'  (dato, rango --data)
        .dc.b   0x00                          | +10d  '.'  (dato, rango --data)
        .dc.b   0x01                          | +10e  '.'  (dato, rango --data)
        .dc.b   0x00                          | +10f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +110  '.'  (dato, rango --data)
        .dc.b   0x05                          | +111  '.'  (dato, rango --data)
        .dc.b   0x67                          | +112  'g'  (dato, rango --data)
        .dc.b   0x96                          | +113  '.'  (dato, rango --data)
        .dc.b   0x02                          | +114  '.'  (dato, rango --data)
        .dc.b   0x00                          | +115  '.'  (dato, rango --data)
        .dc.b   0x00                          | +116  '.'  (dato, rango --data)
        .dc.b   0x23                          | +117  '#'  (dato, rango --data)
        .dc.b   0x33                          | +118  '3'  (dato, rango --data)
        .dc.b   0x1a                          | +119  '.'  (dato, rango --data)
        .dc.b   0xff                          | +11a  '.'  (dato, rango --data)
        .dc.b   0xff                          | +11b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +11c  '.'  (dato, rango --data)
        .dc.b   0x00                          | +11d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +11e  '.'  (dato, rango --data)
        .dc.b   0x00                          | +11f  '.'  (dato, rango --data)
        .dc.b   0x01                          | +120  '.'  (dato, rango --data)
        .dc.b   0x00                          | +121  '.'  (dato, rango --data)
        .dc.b   0x00                          | +122  '.'  (dato, rango --data)
        .dc.b   0x05                          | +123  '.'  (dato, rango --data)
        .dc.b   0x67                          | +124  'g'  (dato, rango --data)
        .dc.b   0x96                          | +125  '.'  (dato, rango --data)
        .dc.b   0x00                          | +126  '.'  (dato, rango --data)
        .dc.b   0x01                          | +127  '.'  (dato, rango --data)
        .dc.b   0x1b                          | +128  '.'  (dato, rango --data)
        .dc.b   0x70                          | +129  'p'  (dato, rango --data)
        .dc.b   0x02                          | +12a  '.'  (dato, rango --data)
        .dc.b   0x00                          | +12b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +12c  '.'  (dato, rango --data)
        .dc.b   0x23                          | +12d  '#'  (dato, rango --data)
        .dc.b   0x33                          | +12e  '3'  (dato, rango --data)
        .dc.b   0x76                          | +12f  'v'  (dato, rango --data)
        .dc.b   0xff                          | +130  '.'  (dato, rango --data)
        .dc.b   0xff                          | +131  '.'  (dato, rango --data)
        .dc.b   0x00                          | +132  '.'  (dato, rango --data)
        .dc.b   0x00                          | +133  '.'  (dato, rango --data)
        .dc.b   0x00                          | +134  '.'  (dato, rango --data)
        .dc.b   0x00                          | +135  '.'  (dato, rango --data)
        .dc.b   0x01                          | +136  '.'  (dato, rango --data)
        .dc.b   0x00                          | +137  '.'  (dato, rango --data)
        .dc.b   0x00                          | +138  '.'  (dato, rango --data)
        .dc.b   0x05                          | +139  '.'  (dato, rango --data)
        .dc.b   0x67                          | +13a  'g'  (dato, rango --data)
        .dc.b   0xd0                          | +13b  '.'  (dato, rango --data)
        .dc.b   0x02                          | +13c  '.'  (dato, rango --data)
        .dc.b   0x00                          | +13d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +13e  '.'  (dato, rango --data)
        .dc.b   0x23                          | +13f  '#'  (dato, rango --data)
        .dc.b   0x3c                          | +140  '<'  (dato, rango --data)
        .dc.b   0x4a                          | +141  'J'  (dato, rango --data)
        .dc.b   0xff                          | +142  '.'  (dato, rango --data)
        .dc.b   0xff                          | +143  '.'  (dato, rango --data)
        .dc.b   0x00                          | +144  '.'  (dato, rango --data)
        .dc.b   0x00                          | +145  '.'  (dato, rango --data)
        .dc.b   0x00                          | +146  '.'  (dato, rango --data)
        .dc.b   0x00                          | +147  '.'  (dato, rango --data)
        .dc.b   0x01                          | +148  '.'  (dato, rango --data)
        .dc.b   0x00                          | +149  '.'  (dato, rango --data)
        .dc.b   0x00                          | +14a  '.'  (dato, rango --data)
        .dc.b   0x05                          | +14b  '.'  (dato, rango --data)
        .dc.b   0x67                          | +14c  'g'  (dato, rango --data)
        .dc.b   0xd0                          | +14d  '.'  (dato, rango --data)
        .dc.b   0x02                          | +14e  '.'  (dato, rango --data)
        .dc.b   0x00                          | +14f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +150  '.'  (dato, rango --data)
        .dc.b   0x23                          | +151  '#'  (dato, rango --data)
        .dc.b   0x33                          | +152  '3'  (dato, rango --data)
        .dc.b   0x26                          | +153  '&'  (dato, rango --data)
        .dc.b   0xff                          | +154  '.'  (dato, rango --data)
        .dc.b   0xff                          | +155  '.'  (dato, rango --data)
        .dc.b   0x00                          | +156  '.'  (dato, rango --data)
        .dc.b   0x00                          | +157  '.'  (dato, rango --data)
        .dc.b   0x00                          | +158  '.'  (dato, rango --data)
        .dc.b   0x00                          | +159  '.'  (dato, rango --data)
        .dc.b   0x01                          | +15a  '.'  (dato, rango --data)
        .dc.b   0x00                          | +15b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +15c  '.'  (dato, rango --data)
        .dc.b   0x05                          | +15d  '.'  (dato, rango --data)
        .dc.b   0x67                          | +15e  'g'  (dato, rango --data)
        .dc.b   0xd0                          | +15f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +160  '.'  (dato, rango --data)
        .dc.b   0x01                          | +161  '.'  (dato, rango --data)
        .dc.b   0x1b                          | +162  '.'  (dato, rango --data)
        .dc.b   0x70                          | +163  'p'  (dato, rango --data)
        .dc.b   0x02                          | +164  '.'  (dato, rango --data)
        .dc.b   0x00                          | +165  '.'  (dato, rango --data)
        .dc.b   0x00                          | +166  '.'  (dato, rango --data)
        .dc.b   0x23                          | +167  '#'  (dato, rango --data)
        .dc.b   0x33                          | +168  '3'  (dato, rango --data)
        .dc.b   0x9a                          | +169  '.'  (dato, rango --data)
        .dc.b   0xff                          | +16a  '.'  (dato, rango --data)
        .dc.b   0xff                          | +16b  '.'  (dato, rango --data)
        .dc.b   0x00                          | +16c  '.'  (dato, rango --data)
        .dc.b   0x00                          | +16d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +16e  '.'  (dato, rango --data)
        .dc.b   0x00                          | +16f  '.'  (dato, rango --data)
        .dc.b   0x01                          | +170  '.'  (dato, rango --data)
        .dc.b   0x00                          | +171  '.'  (dato, rango --data)
        .dc.b   0x00                          | +172  '.'  (dato, rango --data)
        .dc.b   0x05                          | +173  '.'  (dato, rango --data)
        .dc.b   0x68                          | +174  'h'  (dato, rango --data)
        .dc.b   0x0a                          | +175  '.'  (dato, rango --data)
        .dc.b   0x02                          | +176  '.'  (dato, rango --data)
        .dc.b   0x00                          | +177  '.'  (dato, rango --data)
        .dc.b   0x00                          | +178  '.'  (dato, rango --data)
        .dc.b   0x23                          | +179  '#'  (dato, rango --data)
        .dc.b   0x3c                          | +17a  '<'  (dato, rango --data)
        .dc.b   0x6e                          | +17b  'n'  (dato, rango --data)
        .dc.b   0xff                          | +17c  '.'  (dato, rango --data)
        .dc.b   0xff                          | +17d  '.'  (dato, rango --data)
        .dc.b   0x00                          | +17e  '.'  (dato, rango --data)
        .dc.b   0x00                          | +17f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +180  '.'  (dato, rango --data)
        .dc.b   0x00                          | +181  '.'  (dato, rango --data)
        .dc.b   0x01                          | +182  '.'  (dato, rango --data)
        .dc.b   0x00                          | +183  '.'  (dato, rango --data)
        .dc.b   0x00                          | +184  '.'  (dato, rango --data)
        .dc.b   0x05                          | +185  '.'  (dato, rango --data)
        .dc.b   0x68                          | +186  'h'  (dato, rango --data)
        .dc.b   0x0a                          | +187  '.'  (dato, rango --data)
        .dc.b   0x02                          | +188  '.'  (dato, rango --data)
        .dc.b   0x00                          | +189  '.'  (dato, rango --data)
        .dc.b   0x00                          | +18a  '.'  (dato, rango --data)
        .dc.b   0x23                          | +18b  '#'  (dato, rango --data)
        .dc.b   0x33                          | +18c  '3'  (dato, rango --data)
        .dc.b   0x52                          | +18d  'R'  (dato, rango --data)
        .dc.b   0xff                          | +18e  '.'  (dato, rango --data)
        .dc.b   0xff                          | +18f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +190  '.'  (dato, rango --data)
        .dc.b   0x00                          | +191  '.'  (dato, rango --data)
        .dc.b   0x00                          | +192  '.'  (dato, rango --data)
        .dc.b   0x00                          | +193  '.'  (dato, rango --data)
        .dc.b   0x01                          | +194  '.'  (dato, rango --data)
        .dc.b   0x00                          | +195  '.'  (dato, rango --data)
        .dc.b   0x00                          | +196  '.'  (dato, rango --data)
        .dc.b   0x05                          | +197  '.'  (dato, rango --data)
        .dc.b   0x68                          | +198  'h'  (dato, rango --data)
        .dc.b   0x0a                          | +199  '.'  (dato, rango --data)
        .dc.b   0x00                          | +19a  '.'  (dato, rango --data)
        .dc.b   0x01                          | +19b  '.'  (dato, rango --data)
        .dc.b   0x1b                          | +19c  '.'  (dato, rango --data)
        .dc.b   0x70                          | +19d  'p'  (dato, rango --data)
        .dc.b   0x02                          | +19e  '.'  (dato, rango --data)
        .dc.b   0x00                          | +19f  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1a0  '.'  (dato, rango --data)
        .dc.b   0x23                          | +1a1  '#'  (dato, rango --data)
        .dc.b   0x33                          | +1a2  '3'  (dato, rango --data)
        .dc.b   0xa6                          | +1a3  '.'  (dato, rango --data)
        .dc.b   0xff                          | +1a4  '.'  (dato, rango --data)
        .dc.b   0xff                          | +1a5  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1a6  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1a7  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1a8  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1a9  '.'  (dato, rango --data)
        .dc.b   0x01                          | +1aa  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1ab  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1ac  '.'  (dato, rango --data)
        .dc.b   0x05                          | +1ad  '.'  (dato, rango --data)
        .dc.b   0x66                          | +1ae  'f'  (dato, rango --data)
        .dc.b   0x74                          | +1af  't'  (dato, rango --data)
        .dc.b   0x02                          | +1b0  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1b1  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1b2  '.'  (dato, rango --data)
        .dc.b   0x23                          | +1b3  '#'  (dato, rango --data)
        .dc.b   0x3c                          | +1b4  '<'  (dato, rango --data)
        .dc.b   0x7a                          | +1b5  'z'  (dato, rango --data)
        .dc.b   0xff                          | +1b6  '.'  (dato, rango --data)
        .dc.b   0xff                          | +1b7  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1b8  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1b9  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1ba  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1bb  '.'  (dato, rango --data)
        .dc.b   0x01                          | +1bc  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1bd  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1be  '.'  (dato, rango --data)
        .dc.b   0x05                          | +1bf  '.'  (dato, rango --data)
        .dc.b   0x66                          | +1c0  'f'  (dato, rango --data)
        .dc.b   0x74                          | +1c1  't'  (dato, rango --data)
        .dc.b   0x02                          | +1c2  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1c3  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1c4  '.'  (dato, rango --data)
        .dc.b   0x23                          | +1c5  '#'  (dato, rango --data)
        .dc.b   0x33                          | +1c6  '3'  (dato, rango --data)
        .dc.b   0x5e                          | +1c7  '^'  (dato, rango --data)
        .dc.b   0xff                          | +1c8  '.'  (dato, rango --data)
        .dc.b   0xff                          | +1c9  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1ca  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1cb  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1cc  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1cd  '.'  (dato, rango --data)
        .dc.b   0x01                          | +1ce  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1cf  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1d0  '.'  (dato, rango --data)
        .dc.b   0x05                          | +1d1  '.'  (dato, rango --data)
        .dc.b   0x66                          | +1d2  'f'  (dato, rango --data)
        .dc.b   0x74                          | +1d3  't'  (dato, rango --data)
        .dc.b   0x2f                          | +1d4  '/'  (dato, rango --data)
        .dc.b   0x0e                          | +1d5  '.'  (dato, rango --data)
        .dc.b   0x4d                          | +1d6  'M'  (dato, rango --data)
        .dc.b   0xf9                          | +1d7  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1d8  '.'  (dato, rango --data)
        .dc.b   0x10                          | +1d9  '.'  (dato, rango --data)
        .dc.b   0x08                          | +1da  '.'  (dato, rango --data)
        .dc.b   0x00                          | +1db  '.'  (dato, rango --data)

| ----------------------------------------------------------------------------
|  Roller_SpawnFromParent_05684c  @ $05684C  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Roller_SpawnFromParent_05684c, "ax", @progbits
        .global Roller_SpawnFromParent_05684c
Roller_SpawnFromParent_05684c:
        lea     Roller_Task_056870(pc),a1       | +000
        jsr     0x4ae.l                         | +004
        movea.l (a7)+,a6                        | +00a
        move.w  0x22(a6),0x22(a0)               | +00c
        move.w  0x24(a6),0x24(a0)               | +012
        move.b  0x3a(a6),0x3a(a0)               | +018
        clr.w   0x70(a6)                        | +01e
        rts                                     | +022

| ----------------------------------------------------------------------------
|  Roller_Task_056870  @ $056870  (406 B)
| ----------------------------------------------------------------------------
        .section .text.Roller_Task_056870, "ax", @progbits
        .global Roller_Task_056870
Roller_Task_056870:
        bset    #0x4,0x6b(a6)                   | +000
        move.w  #0xd000,d0                      | +006
        jsr     0x28134.l                       | +00a
        andi.w  #0xffe3,0x38(a6)                | +010
        ori.w   #0x10,0x38(a6)                  | +016
        move.w  #0xe,d1                         | +01c
        jsr     0x236e.l                        | +020
        lea     SpriteMap_Roller_056670(pc),a0  | +026
        jsr     0x28cd4.l                       | +02a
        move.w  #0x80,0x2a(a6)                  | +030
        move.w  #0xfffc,0x2e(a6)                | +036
        clr.w   0x28(a6)                        | +03c
        addi.w  #0x28,0x24(a6)                  | +040
        move.w  #0x64,0x66(a6)                  | +046
        lea     Roller_HitboxList_B_05661c(pc),a0 | +04c
        move.l  a0,0x48(a6)                     | +050
        lea     Roller_HitboxList_0565c8(pc),a0 | +054
        move.l  a0,0x4c(a6)                     | +058
        jsr     0x283ca.l                       | +05c
        jsr     0x283ca.l                       | +062
        clr.w   0x70(a6)                        | +068
        lea     .L0568e2(pc),a1                 | +06c
        move.l  a1,(a6)                         | +070
.L0568e2:
        move.w  #0x10,d0                        | +072
        btst    #0x0,0x3a(a6)                   | +076
        bne.w   .L0568f2                        | +07c
        neg.w   d0                              | +080
.L0568f2:
        add.w   d0,0x28(a6)                     | +082
        jsr     0x27bc8.l                       | +086
        bcc.w   .L056914                        | +08c
        addi.w  #0x1,0x24(a6)                   | +090
        clr.w   0x2a(a6)                        | +096
        move.w  #0x10,0x2e(a6)                  | +09a
        bra.w   .L056980                        | +0a0
.L056914:
        move.w  0x28(a6),d7                     | +0a4
        bmi.w   .L05691e                        | +0a8
        neg.w   d7                              | +0ac
.L05691e:
        move.w  d7,0x36(a6)                     | +0ae
        move.w  #0x7,d1                         | +0b2
        btst    #0x0,0x3a(a6)                   | +0b6
        bne.w   .L056932                        | +0bc
        neg.w   d1                              | +0c0
.L056932:
        add.w   0x22(a6),d1                     | +0c2
        move.w  0x24(a6),d2                     | +0c6
        subq.w  #0x8,d2                         | +0ca
        movem.w d1-d2,-(a7)                     | +0cc
        jsr     0x280c6.l                       | +0d0
        movem.w (a7)+,d1-d2                     | +0d6
        bcc.w   .L056962                        | +0da
        move.w  0x2a(a6),d0                     | +0de
        bpl.w   .L05695a                        | +0e2
        clr.w   0x2a(a6)                        | +0e6
.L05695a:
        neg.w   0x36(a6)                        | +0ea
        bra.w   .L056972                        | +0ee
.L056962:
        addq.w  #0x8,d2                         | +0f2
        jsr     0x280c6.l                       | +0f4
        bcc.w   .L056972                        | +0fa
        clr.w   0x36(a6)                        | +0fe
.L056972:
        move.w  0x36(a6),d0                     | +102
        sub.w   0x2a(a6),d0                     | +106
        asr.w   #0x5,d0                         | +10a
        move.w  d0,0x2e(a6)                     | +10c
.L056980:
        clr.w   0x70(a6)                        | +110
        move.b  0x2a(a6),d0                     | +114
        addq.b  #0x1,d0                         | +118
        andi.b  #0xfc,d0                        | +11a
        beq.w   .L0569a6                        | +11e
        bmi.w   .L0569a0                        | +122
        move.w  #0x1,0x70(a6)                   | +126
        bra.w   .L0569a6                        | +12c
.L0569a0:
        move.w  #0x2,0x70(a6)                   | +130
.L0569a6:
        jsr     0x28d70.l                       | +136
        movea.l #0xffffffff,a0                  | +13c
        lea     Grenade_ProbeBox_055cc8(pc),a0  | +142
        jsr     0x5dd56.l                       | +146
        bcs.w   JmpAbsThunk_056a06              | +14c
        jsr     0x283d8.l                       | +150
        btst    #0x1,0x13(a6)                   | +156
        bne.w   Roller_Explode_056a0c           | +15c
        jsr     0x28758.l                       | +160
        bcs.w   Roller_Explode_056a0c           | +166
        btst    #0x5,0x5a(a6)                   | +16a
        bne.w   Roller_Explode_056a0c           | +170
        jsr     0x2870a.l                       | +174
        bcc.w   .L056a04                        | +17a
        move.w  #0x108d,d0                      | +17e
        jsr     0x2352.l                        | +182
        bclr    #0x3,0x13(a6)                   | +188
        addi.w  #0x300,0x2a(a6)                 | +18e
.L056a04:
        rts                                     | +194

| ----------------------------------------------------------------------------
|  Roller_Explode_056a0c  @ $056A0C  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Roller_Explode_056a0c, "ax", @progbits
        .global Roller_Explode_056a0c
Roller_Explode_056a0c:
        move.w  #0x1022,d0                      | +000
        jsr     0x2352.l                        | +004
        jsr     0x13600.l                       | +00a
        move.w  #0x2000,d0                      | +010
        jsr     0x28134.l                       | +014
        andi.w  #0xffe3,0x38(a6)                | +01a
        ori.w   #0x10,0x38(a6)                  | +020
        jmp     0x77f6a.l                       | +026

| ----------------------------------------------------------------------------
|  Entity_CmpDepthToParent_056a38  @ $056A38  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpDepthToParent_056a38, "ax", @progbits
        .global Entity_CmpDepthToParent_056a38
Entity_CmpDepthToParent_056a38:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_056a4e                    | +00c

| ----------------------------------------------------------------------------
|  Entity_CmpDepthToParent_056a54  @ $056A54  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpDepthToParent_056a54, "ax", @progbits
        .global Entity_CmpDepthToParent_056a54
Entity_CmpDepthToParent_056a54:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_056a6a                    | +00c

| ----------------------------------------------------------------------------
|  Entity_CmpDepthToParent_056a70  @ $056A70  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpDepthToParent_056a70, "ax", @progbits
        .global Entity_CmpDepthToParent_056a70
Entity_CmpDepthToParent_056a70:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_056a86                    | +00c

| ----------------------------------------------------------------------------
|  Entity_CmpDepthToParent_056a8c  @ $056A8C  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpDepthToParent_056a8c, "ax", @progbits
        .global Entity_CmpDepthToParent_056a8c
Entity_CmpDepthToParent_056a8c:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_056aa2                    | +00c

| ----------------------------------------------------------------------------
|  Entity_CmpDepthToParent_056aa8  @ $056AA8  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpDepthToParent_056aa8, "ax", @progbits
        .global Entity_CmpDepthToParent_056aa8
Entity_CmpDepthToParent_056aa8:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_056abe                    | +00c

| ----------------------------------------------------------------------------
|  Soldier_PhysicsBox_056ac4  @ $056AC4  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Soldier_PhysicsBox_056ac4, "ax", @progbits
        .global Soldier_PhysicsBox_056ac4
Soldier_PhysicsBox_056ac4:
        .dc.b   0xff                          | +000  '.'  (dato, rango --data)
        .dc.b   0xc0                          | +001  '.'  (dato, rango --data)
        .dc.b   0x00                          | +002  '.'  (dato, rango --data)
        .dc.b   0x00                          | +003  '.'  (dato, rango --data)
        .dc.b   0x00                          | +004  '.'  (dato, rango --data)
        .dc.b   0x40                          | +005  '@'  (dato, rango --data)
        .dc.b   0x00                          | +006  '.'  (dato, rango --data)
        .dc.b   0x10                          | +007  '.'  (dato, rango --data)
