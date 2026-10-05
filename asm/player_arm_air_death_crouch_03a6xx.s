| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave ??? — (borrador)
|  Región: $03A60A..$03C62A  (8,224 B, 152 entradas, 1 huecos)
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
|  PlayerArm_Jump_03a60a  @ $03A60A  (34 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_Jump_03a60a, "ax", @progbits
        .global PlayerArm_Jump_03a60a
PlayerArm_Jump_03a60a:
        bclr    #0x2,0x8c(a6)                   | +000
        bclr    #0x2,0x8c(a6)                   | +006
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +00c
        lea     PlayerArm_SpriteTbl_03a62c(pc),a0 | +010
        movea.l (a0,d0.w),a0                    | +014
        jsr     0x28cd4.l                       | +018
        bra.w   PlayerArm_SpriteTbl_03a62c__L03a654 | +01e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03a62c  @ $03A62C  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03a62c, "ax", @progbits
        .global PlayerArm_SpriteTbl_03a62c
PlayerArm_SpriteTbl_03a62c:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xb966                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xef8e                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xef8e                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xef8e                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xef8e                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xb966                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xef8e                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xef8e                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xef8e                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xef8e                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03a62c__L03a654
PlayerArm_SpriteTbl_03a62c__L03a654:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_Fall_03a656  @ $03A656  (34 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_Fall_03a656, "ax", @progbits
        .global PlayerArm_Fall_03a656
PlayerArm_Fall_03a656:
        bclr    #0x2,0x8c(a6)                   | +000
        bclr    #0x2,0x8c(a6)                   | +006
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +00c
        lea     PlayerArm_SpriteTbl_03a678(pc),a0 | +010
        movea.l (a0,d0.w),a0                    | +014
        jsr     0x28cd4.l                       | +018
        bra.w   PlayerArm_SpriteTbl_03a678__L03a6a0 | +01e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03a678  @ $03A678  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03a678, "ax", @progbits
        .global PlayerArm_SpriteTbl_03a678
PlayerArm_SpriteTbl_03a678:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xb872                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xee9a                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xee9a                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xee9a                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xee9a                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xb872                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xee9a                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xee9a                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xee9a                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xee9a                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03a678__L03a6a0
PlayerArm_SpriteTbl_03a678__L03a6a0:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_AirUnref_03a6a2  @ $03A6A2  (34 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_AirUnref_03a6a2, "ax", @progbits
        .global PlayerArm_AirUnref_03a6a2
PlayerArm_AirUnref_03a6a2:
        bclr    #0x2,0x8c(a6)                   | +000
        bclr    #0x2,0x8c(a6)                   | +006
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +00c
        lea     PlayerArm_SpriteTbl_03a6c4(pc),a0 | +010
        movea.l (a0,d0.w),a0                    | +014
        jsr     0x28cd4.l                       | +018
        bra.w   PlayerArm_SpriteTbl_03a6c4__L03a6ec | +01e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03a6c4  @ $03A6C4  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03a6c4, "ax", @progbits
        .global PlayerArm_SpriteTbl_03a6c4
PlayerArm_SpriteTbl_03a6c4:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xb966                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xef8e                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xef8e                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xef8e                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xef8e                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xb966                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xef8e                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xef8e                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xef8e                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xef8e                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03a6c4__L03a6ec
PlayerArm_SpriteTbl_03a6c4__L03a6ec:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_AirB_03a6ee  @ $03A6EE  (34 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_AirB_03a6ee, "ax", @progbits
        .global PlayerArm_AirB_03a6ee
PlayerArm_AirB_03a6ee:
        bclr    #0x2,0x8c(a6)                   | +000
        bclr    #0x2,0x8c(a6)                   | +006
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +00c
        lea     PlayerArm_SpriteTbl_03a710(pc),a0 | +010
        movea.l (a0,d0.w),a0                    | +014
        jsr     0x28cd4.l                       | +018
        bra.w   PlayerArm_SpriteTbl_03a710__L03a738 | +01e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03a710  @ $03A710  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03a710, "ax", @progbits
        .global PlayerArm_SpriteTbl_03a710
PlayerArm_SpriteTbl_03a710:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xb872                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xee9a                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xee9a                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xee9a                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xee9a                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xb872                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xee9a                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xee9a                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xee9a                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xee9a                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03a710__L03a738
PlayerArm_SpriteTbl_03a710__L03a738:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_AirShootA_03a73a  @ $03A73A  (38 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_AirShootA_03a73a, "ax", @progbits
        .global PlayerArm_AirShootA_03a73a
PlayerArm_AirShootA_03a73a:
        cmpi.w  #0x1,0x72(a6)                   | +000
        bne.w   PlayerArm_SpriteTbl_03a760__L03a78c | +006
        bclr    #0x2,0x8c(a6)                   | +00a
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +010
        lea     PlayerArm_SpriteTbl_03a760(pc),a0 | +014
        movea.l (a0,d0.w),a0                    | +018
        jsr     0x28cd4.l                       | +01c
        bra.w   PlayerArm_SpriteTbl_03a760__L03a788 | +022

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03a760  @ $03A760  (82 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03a760, "ax", @progbits
        .global PlayerArm_SpriteTbl_03a760
PlayerArm_SpriteTbl_03a760:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc8a0                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x1000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x195c                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x23f2                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xfee2                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc8a0                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x1000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x195c                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x23f2                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xfee2                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03a760__L03a788
PlayerArm_SpriteTbl_03a760__L03a788:
        bra.w   PlayerArm_SpriteTbl_03a7fa__L03a822 | +028
        .global PlayerArm_SpriteTbl_03a760__L03a78c
PlayerArm_SpriteTbl_03a760__L03a78c:
        cmpi.w  #0x3,0x72(a6)                   | +02c
        bne.w   PlayerArm_SpriteTbl_03a7b2__L03a7de | +032
        bclr    #0x2,0x8c(a6)                   | +036
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +03c
        lea     PlayerArm_SpriteTbl_03a7b2(pc),a0 | +040
        movea.l (a0,d0.w),a0                    | +044
        jsr     0x28cd4.l                       | +048
        bra.w   PlayerArm_SpriteTbl_03a7b2__L03a7da | +04e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03a7b2  @ $03A7B2  (72 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03a7b2, "ax", @progbits
        .global PlayerArm_SpriteTbl_03a7b2
PlayerArm_SpriteTbl_03a7b2:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc86e                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0fce                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x192a                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x23c0                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xfeb0                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc86e                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0fce                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x192a                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x23c0                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xfeb0                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03a7b2__L03a7da
PlayerArm_SpriteTbl_03a7b2__L03a7da:
        bra.w   PlayerArm_SpriteTbl_03a7fa__L03a822 | +028
        .global PlayerArm_SpriteTbl_03a7b2__L03a7de
PlayerArm_SpriteTbl_03a7b2__L03a7de:
        bclr    #0x2,0x8c(a6)                   | +02c
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +032
        lea     PlayerArm_SpriteTbl_03a7fa(pc),a0 | +036
        movea.l (a0,d0.w),a0                    | +03a
        jsr     0x28cd4.l                       | +03e
        bra.w   PlayerArm_SpriteTbl_03a7fa__L03a822 | +044

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03a7fa  @ $03A7FA  (80 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03a7fa, "ax", @progbits
        .global PlayerArm_SpriteTbl_03a7fa
PlayerArm_SpriteTbl_03a7fa:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xba04                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xf02c                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xf02c                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xf02c                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xf02c                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xba04                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xf02c                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xf02c                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xf02c                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xf02c                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03a7fa__L03a822
PlayerArm_SpriteTbl_03a7fa__L03a822:
        rts                                     | +028
        cmpi.w  #0x1,0x72(a6)                   | +02a
        bne.w   PlayerArm_SpriteTbl_03a84a__L03a874 | +030
        bclr    #0x2,0x8c(a6)                   | +034
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +03a
        lea     PlayerArm_SpriteTbl_03a84a(pc),a0 | +03e
        movea.l (a0,d0.w),a0                    | +042
        jsr     0x28cd4.l                       | +046
        bra.w   PlayerArm_SpriteTbl_03a84a__L03a872 | +04c

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03a84a  @ $03A84A  (80 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03a84a, "ax", @progbits
        .global PlayerArm_SpriteTbl_03a84a
PlayerArm_SpriteTbl_03a84a:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc8a0                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x1000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x195c                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x23f2                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xfee2                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc8a0                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x1000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x195c                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x23f2                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xfee2                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03a84a__L03a872
PlayerArm_SpriteTbl_03a84a__L03a872:
        bra.b   PlayerArm_SpriteTbl_03a7fa__L03a822 | +028
        .global PlayerArm_SpriteTbl_03a84a__L03a874
PlayerArm_SpriteTbl_03a84a__L03a874:
        cmpi.w  #0x3,0x72(a6)                   | +02a
        bne.w   PlayerArm_SpriteTbl_03a89a__L03a8c6 | +030
        bclr    #0x2,0x8c(a6)                   | +034
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +03a
        lea     PlayerArm_SpriteTbl_03a89a(pc),a0 | +03e
        movea.l (a0,d0.w),a0                    | +042
        jsr     0x28cd4.l                       | +046
        bra.w   PlayerArm_SpriteTbl_03a89a__L03a8c2 | +04c

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03a89a  @ $03A89A  (72 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03a89a, "ax", @progbits
        .global PlayerArm_SpriteTbl_03a89a
PlayerArm_SpriteTbl_03a89a:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc86e                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0fce                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x192a                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x23c0                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xfeb0                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc86e                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0fce                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x192a                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x23c0                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xfeb0                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03a89a__L03a8c2
PlayerArm_SpriteTbl_03a89a__L03a8c2:
        bra.w   PlayerArm_SpriteTbl_03a8e2__L03a90a | +028
        .global PlayerArm_SpriteTbl_03a89a__L03a8c6
PlayerArm_SpriteTbl_03a89a__L03a8c6:
        bclr    #0x2,0x8c(a6)                   | +02c
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +032
        lea     PlayerArm_SpriteTbl_03a8e2(pc),a0 | +036
        movea.l (a0,d0.w),a0                    | +03a
        jsr     0x28cd4.l                       | +03e
        bra.w   PlayerArm_SpriteTbl_03a8e2__L03a90a | +044

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03a8e2  @ $03A8E2  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03a8e2, "ax", @progbits
        .global PlayerArm_SpriteTbl_03a8e2
PlayerArm_SpriteTbl_03a8e2:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xba04                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xf02c                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xf02c                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xf02c                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xf02c                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xba04                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xf02c                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xf02c                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xf02c                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xf02c                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03a8e2__L03a90a
PlayerArm_SpriteTbl_03a8e2__L03a90a:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_AirShootB_03a90c  @ $03A90C  (38 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_AirShootB_03a90c, "ax", @progbits
        .global PlayerArm_AirShootB_03a90c
PlayerArm_AirShootB_03a90c:
        cmpi.w  #0x0,0x72(a6)                   | +000
        bne.w   PlayerArm_SpriteTbl_03a932__L03a95e | +006
        bclr    #0x2,0x8c(a6)                   | +00a
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +010
        lea     PlayerArm_SpriteTbl_03a932(pc),a0 | +014
        movea.l (a0,d0.w),a0                    | +018
        jsr     0x28cd4.l                       | +01c
        bra.w   PlayerArm_SpriteTbl_03a932__L03a95a | +022

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03a932  @ $03A932  (82 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03a932, "ax", @progbits
        .global PlayerArm_SpriteTbl_03a932
PlayerArm_SpriteTbl_03a932:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc83c                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0f9c                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x18f8                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x238e                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xfe7e                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc83c                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0f9c                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x18f8                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x238e                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xfe7e                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03a932__L03a95a
PlayerArm_SpriteTbl_03a932__L03a95a:
        bra.w   PlayerArm_SpriteTbl_03a9cc__L03a9f4 | +028
        .global PlayerArm_SpriteTbl_03a932__L03a95e
PlayerArm_SpriteTbl_03a932__L03a95e:
        cmpi.w  #0x1,0x72(a6)                   | +02c
        bne.w   PlayerArm_SpriteTbl_03a984__L03a9b0 | +032
        bclr    #0x2,0x8c(a6)                   | +036
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +03c
        lea     PlayerArm_SpriteTbl_03a984(pc),a0 | +040
        movea.l (a0,d0.w),a0                    | +044
        jsr     0x28cd4.l                       | +048
        bra.w   PlayerArm_SpriteTbl_03a984__L03a9ac | +04e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03a984  @ $03A984  (72 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03a984, "ax", @progbits
        .global PlayerArm_SpriteTbl_03a984
PlayerArm_SpriteTbl_03a984:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc83c                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0f9c                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x18f8                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x238e                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xfe7e                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc83c                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0f9c                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x18f8                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x238e                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xfe7e                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03a984__L03a9ac
PlayerArm_SpriteTbl_03a984__L03a9ac:
        bra.w   PlayerArm_SpriteTbl_03a9cc__L03a9f4 | +028
        .global PlayerArm_SpriteTbl_03a984__L03a9b0
PlayerArm_SpriteTbl_03a984__L03a9b0:
        bclr    #0x2,0x8c(a6)                   | +02c
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +032
        lea     PlayerArm_SpriteTbl_03a9cc(pc),a0 | +036
        movea.l (a0,d0.w),a0                    | +03a
        jsr     0x28cd4.l                       | +03e
        bra.w   PlayerArm_SpriteTbl_03a9cc__L03a9f4 | +044

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03a9cc  @ $03A9CC  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03a9cc, "ax", @progbits
        .global PlayerArm_SpriteTbl_03a9cc
PlayerArm_SpriteTbl_03a9cc:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xb9ec                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xf014                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xf014                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xf014                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xf014                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xb9ec                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xf014                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xf014                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xf014                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xf014                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03a9cc__L03a9f4
PlayerArm_SpriteTbl_03a9cc__L03a9f4:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_AirShootC_03a9f6  @ $03A9F6  (38 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_AirShootC_03a9f6, "ax", @progbits
        .global PlayerArm_AirShootC_03a9f6
PlayerArm_AirShootC_03a9f6:
        cmpi.w  #0x0,0x72(a6)                   | +000
        bne.w   PlayerArm_SpriteTbl_03aa1c__L03aa48 | +006
        bclr    #0x2,0x8c(a6)                   | +00a
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +010
        lea     PlayerArm_SpriteTbl_03aa1c(pc),a0 | +014
        movea.l (a0,d0.w),a0                    | +018
        jsr     0x28cd4.l                       | +01c
        bra.w   PlayerArm_SpriteTbl_03aa1c__L03aa44 | +022

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03aa1c  @ $03AA1C  (82 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03aa1c, "ax", @progbits
        .global PlayerArm_SpriteTbl_03aa1c
PlayerArm_SpriteTbl_03aa1c:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc83c                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0f9c                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x18f8                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x238e                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xfe7e                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc83c                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0f9c                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x18f8                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x238e                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xfe7e                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03aa1c__L03aa44
PlayerArm_SpriteTbl_03aa1c__L03aa44:
        bra.w   PlayerArm_SpriteTbl_03aab6__L03aade | +028
        .global PlayerArm_SpriteTbl_03aa1c__L03aa48
PlayerArm_SpriteTbl_03aa1c__L03aa48:
        cmpi.w  #0x1,0x72(a6)                   | +02c
        bne.w   PlayerArm_SpriteTbl_03aa6e__L03aa9a | +032
        bclr    #0x2,0x8c(a6)                   | +036
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +03c
        lea     PlayerArm_SpriteTbl_03aa6e(pc),a0 | +040
        movea.l (a0,d0.w),a0                    | +044
        jsr     0x28cd4.l                       | +048
        bra.w   PlayerArm_SpriteTbl_03aa6e__L03aa96 | +04e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03aa6e  @ $03AA6E  (72 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03aa6e, "ax", @progbits
        .global PlayerArm_SpriteTbl_03aa6e
PlayerArm_SpriteTbl_03aa6e:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc83c                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0f9c                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x18f8                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x238e                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xfe7e                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc83c                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0f9c                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x18f8                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x238e                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xfe7e                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03aa6e__L03aa96
PlayerArm_SpriteTbl_03aa6e__L03aa96:
        bra.w   PlayerArm_SpriteTbl_03aab6__L03aade | +028
        .global PlayerArm_SpriteTbl_03aa6e__L03aa9a
PlayerArm_SpriteTbl_03aa6e__L03aa9a:
        bclr    #0x2,0x8c(a6)                   | +02c
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +032
        lea     PlayerArm_SpriteTbl_03aab6(pc),a0 | +036
        movea.l (a0,d0.w),a0                    | +03a
        jsr     0x28cd4.l                       | +03e
        bra.w   PlayerArm_SpriteTbl_03aab6__L03aade | +044

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03aab6  @ $03AAB6  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03aab6, "ax", @progbits
        .global PlayerArm_SpriteTbl_03aab6
PlayerArm_SpriteTbl_03aab6:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xb9ec                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xf014                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xf014                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xf014                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xf014                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xb9ec                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xf014                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xf014                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xf014                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xf014                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03aab6__L03aade
PlayerArm_SpriteTbl_03aab6__L03aade:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_JumpShootA_03aae0  @ $03AAE0  (38 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_JumpShootA_03aae0, "ax", @progbits
        .global PlayerArm_JumpShootA_03aae0
PlayerArm_JumpShootA_03aae0:
        cmpi.w  #0x1,0x72(a6)                   | +000
        bne.w   PlayerArm_SpriteTbl_03ab06__L03ab32 | +006
        bclr    #0x2,0x8c(a6)                   | +00a
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +010
        lea     PlayerArm_SpriteTbl_03ab06(pc),a0 | +014
        movea.l (a0,d0.w),a0                    | +018
        jsr     0x28cd4.l                       | +01c
        bra.w   PlayerArm_SpriteTbl_03ab06__L03ab2e | +022

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03ab06  @ $03AB06  (82 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03ab06, "ax", @progbits
        .global PlayerArm_SpriteTbl_03ab06
PlayerArm_SpriteTbl_03ab06:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xcbe8                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x1338                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1c84                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x271a                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x02b6                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xcbe8                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x1338                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1c84                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x271a                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x02b6                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03ab06__L03ab2e
PlayerArm_SpriteTbl_03ab06__L03ab2e:
        bra.w   PlayerArm_SpriteTbl_03abf2__L03ac1a | +028
        .global PlayerArm_SpriteTbl_03ab06__L03ab32
PlayerArm_SpriteTbl_03ab06__L03ab32:
        cmpi.w  #0x2,0x72(a6)                   | +02c
        bne.w   PlayerArm_SpriteTbl_03ab58__L03ab84 | +032
        bclr    #0x2,0x8c(a6)                   | +036
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +03c
        lea     PlayerArm_SpriteTbl_03ab58(pc),a0 | +040
        movea.l (a0,d0.w),a0                    | +044
        jsr     0x28cd4.l                       | +048
        bra.w   PlayerArm_SpriteTbl_03ab58__L03ab80 | +04e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03ab58  @ $03AB58  (82 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03ab58, "ax", @progbits
        .global PlayerArm_SpriteTbl_03ab58
PlayerArm_SpriteTbl_03ab58:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xcc64                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x13ac                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1cf8                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x278e                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x034a                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xcc64                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x13ac                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1cf8                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x278e                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x034a                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03ab58__L03ab80
PlayerArm_SpriteTbl_03ab58__L03ab80:
        bra.w   PlayerArm_SpriteTbl_03abf2__L03ac1a | +028
        .global PlayerArm_SpriteTbl_03ab58__L03ab84
PlayerArm_SpriteTbl_03ab58__L03ab84:
        cmpi.w  #0x3,0x72(a6)                   | +02c
        bne.w   PlayerArm_SpriteTbl_03abaa__L03abd6 | +032
        bclr    #0x2,0x8c(a6)                   | +036
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +03c
        lea     PlayerArm_SpriteTbl_03abaa(pc),a0 | +040
        movea.l (a0,d0.w),a0                    | +044
        jsr     0x28cd4.l                       | +048
        bra.w   PlayerArm_SpriteTbl_03abaa__L03abd2 | +04e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03abaa  @ $03ABAA  (72 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03abaa, "ax", @progbits
        .global PlayerArm_SpriteTbl_03abaa
PlayerArm_SpriteTbl_03abaa:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc918                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x1068                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x19c4                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x245a                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xff56                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc918                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x1068                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x19c4                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x245a                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xff56                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03abaa__L03abd2
PlayerArm_SpriteTbl_03abaa__L03abd2:
        bra.w   PlayerArm_SpriteTbl_03abf2__L03ac1a | +028
        .global PlayerArm_SpriteTbl_03abaa__L03abd6
PlayerArm_SpriteTbl_03abaa__L03abd6:
        bclr    #0x2,0x8c(a6)                   | +02c
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +032
        lea     PlayerArm_SpriteTbl_03abf2(pc),a0 | +036
        movea.l (a0,d0.w),a0                    | +03a
        jsr     0x28cd4.l                       | +03e
        bra.w   PlayerArm_SpriteTbl_03abf2__L03ac1a | +044

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03abf2  @ $03ABF2  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03abf2, "ax", @progbits
        .global PlayerArm_SpriteTbl_03abf2
PlayerArm_SpriteTbl_03abf2:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xbfc8                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0c72                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1858                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x22c2                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xf636                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xbfc8                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0c72                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1858                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x22c2                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xf636                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03abf2__L03ac1a
PlayerArm_SpriteTbl_03abf2__L03ac1a:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_FallShootA_03ac1c  @ $03AC1C  (38 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_FallShootA_03ac1c, "ax", @progbits
        .global PlayerArm_FallShootA_03ac1c
PlayerArm_FallShootA_03ac1c:
        cmpi.w  #0x1,0x72(a6)                   | +000
        bne.w   PlayerArm_SpriteTbl_03ac42__L03ac6e | +006
        bclr    #0x2,0x8c(a6)                   | +00a
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +010
        lea     PlayerArm_SpriteTbl_03ac42(pc),a0 | +014
        movea.l (a0,d0.w),a0                    | +018
        jsr     0x28cd4.l                       | +01c
        bra.w   PlayerArm_SpriteTbl_03ac42__L03ac6a | +022

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03ac42  @ $03AC42  (82 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03ac42, "ax", @progbits
        .global PlayerArm_SpriteTbl_03ac42
PlayerArm_SpriteTbl_03ac42:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xca7c                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x11cc                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1b20                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x25b6                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x010a                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xca7c                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x11cc                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1b20                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x25b6                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x010a                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03ac42__L03ac6a
PlayerArm_SpriteTbl_03ac42__L03ac6a:
        bra.w   PlayerArm_SpriteTbl_03ad2e__L03ad56 | +028
        .global PlayerArm_SpriteTbl_03ac42__L03ac6e
PlayerArm_SpriteTbl_03ac42__L03ac6e:
        cmpi.w  #0x2,0x72(a6)                   | +02c
        bne.w   PlayerArm_SpriteTbl_03ac94__L03acc0 | +032
        bclr    #0x2,0x8c(a6)                   | +036
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +03c
        lea     PlayerArm_SpriteTbl_03ac94(pc),a0 | +040
        movea.l (a0,d0.w),a0                    | +044
        jsr     0x28cd4.l                       | +048
        bra.w   PlayerArm_SpriteTbl_03ac94__L03acbc | +04e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03ac94  @ $03AC94  (82 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03ac94, "ax", @progbits
        .global PlayerArm_SpriteTbl_03ac94
PlayerArm_SpriteTbl_03ac94:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xcaf8                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x1240                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1b94                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x262a                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0192                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xcaf8                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x1240                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1b94                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x262a                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0192                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03ac94__L03acbc
PlayerArm_SpriteTbl_03ac94__L03acbc:
        bra.w   PlayerArm_SpriteTbl_03ad2e__L03ad56 | +028
        .global PlayerArm_SpriteTbl_03ac94__L03acc0
PlayerArm_SpriteTbl_03ac94__L03acc0:
        cmpi.w  #0x3,0x72(a6)                   | +02c
        bne.w   PlayerArm_SpriteTbl_03ace6__L03ad12 | +032
        bclr    #0x2,0x8c(a6)                   | +036
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +03c
        lea     PlayerArm_SpriteTbl_03ace6(pc),a0 | +040
        movea.l (a0,d0.w),a0                    | +044
        jsr     0x28cd4.l                       | +048
        bra.w   PlayerArm_SpriteTbl_03ace6__L03ad0e | +04e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03ace6  @ $03ACE6  (72 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03ace6, "ax", @progbits
        .global PlayerArm_SpriteTbl_03ace6
PlayerArm_SpriteTbl_03ace6:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc918                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x1068                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x19c4                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x245a                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xff56                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc918                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x1068                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x19c4                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x245a                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xff56                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03ace6__L03ad0e
PlayerArm_SpriteTbl_03ace6__L03ad0e:
        bra.w   PlayerArm_SpriteTbl_03ad2e__L03ad56 | +028
        .global PlayerArm_SpriteTbl_03ace6__L03ad12
PlayerArm_SpriteTbl_03ace6__L03ad12:
        bclr    #0x2,0x8c(a6)                   | +02c
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +032
        lea     PlayerArm_SpriteTbl_03ad2e(pc),a0 | +036
        movea.l (a0,d0.w),a0                    | +03a
        jsr     0x28cd4.l                       | +03e
        bra.w   PlayerArm_SpriteTbl_03ad2e__L03ad56 | +044

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03ad2e  @ $03AD2E  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03ad2e, "ax", @progbits
        .global PlayerArm_SpriteTbl_03ad2e
PlayerArm_SpriteTbl_03ad2e:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xbd30                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0c72                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1858                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x22c2                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xf4bc                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xbd30                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0c72                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1858                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x22c2                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xf4bc                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03ad2e__L03ad56
PlayerArm_SpriteTbl_03ad2e__L03ad56:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_JumpShootB_03ad58  @ $03AD58  (38 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_JumpShootB_03ad58, "ax", @progbits
        .global PlayerArm_JumpShootB_03ad58
PlayerArm_JumpShootB_03ad58:
        cmpi.w  #0x1,0x72(a6)                   | +000
        bne.w   PlayerArm_SpriteTbl_03ad7e__L03adaa | +006
        bclr    #0x2,0x8c(a6)                   | +00a
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +010
        lea     PlayerArm_SpriteTbl_03ad7e(pc),a0 | +014
        movea.l (a0,d0.w),a0                    | +018
        jsr     0x28cd4.l                       | +01c
        bra.w   PlayerArm_SpriteTbl_03ad7e__L03ada6 | +022

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03ad7e  @ $03AD7E  (82 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03ad7e, "ax", @progbits
        .global PlayerArm_SpriteTbl_03ad7e
PlayerArm_SpriteTbl_03ad7e:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xcbe8                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x1338                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1c84                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x271a                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0716                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xcbe8                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x1338                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1c84                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x271a                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0716                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03ad7e__L03ada6
PlayerArm_SpriteTbl_03ad7e__L03ada6:
        bra.w   PlayerArm_SpriteTbl_03ae6a__L03ae92 | +028
        .global PlayerArm_SpriteTbl_03ad7e__L03adaa
PlayerArm_SpriteTbl_03ad7e__L03adaa:
        cmpi.w  #0x2,0x72(a6)                   | +02c
        bne.w   PlayerArm_SpriteTbl_03add0__L03adfc | +032
        bclr    #0x2,0x8c(a6)                   | +036
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +03c
        lea     PlayerArm_SpriteTbl_03add0(pc),a0 | +040
        movea.l (a0,d0.w),a0                    | +044
        jsr     0x28cd4.l                       | +048
        bra.w   PlayerArm_SpriteTbl_03add0__L03adf8 | +04e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03add0  @ $03ADD0  (82 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03add0, "ax", @progbits
        .global PlayerArm_SpriteTbl_03add0
PlayerArm_SpriteTbl_03add0:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xcc64                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x13ac                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1cf8                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x278e                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x07aa                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xcc64                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x13ac                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1cf8                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x278e                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x07aa                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03add0__L03adf8
PlayerArm_SpriteTbl_03add0__L03adf8:
        bra.w   PlayerArm_SpriteTbl_03ae6a__L03ae92 | +028
        .global PlayerArm_SpriteTbl_03add0__L03adfc
PlayerArm_SpriteTbl_03add0__L03adfc:
        cmpi.w  #0x3,0x72(a6)                   | +02c
        bne.w   PlayerArm_SpriteTbl_03ae22__L03ae4e | +032
        bclr    #0x2,0x8c(a6)                   | +036
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +03c
        lea     PlayerArm_SpriteTbl_03ae22(pc),a0 | +040
        movea.l (a0,d0.w),a0                    | +044
        jsr     0x28cd4.l                       | +048
        bra.w   PlayerArm_SpriteTbl_03ae22__L03ae4a | +04e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03ae22  @ $03AE22  (72 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03ae22, "ax", @progbits
        .global PlayerArm_SpriteTbl_03ae22
PlayerArm_SpriteTbl_03ae22:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc918                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x1068                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x19c4                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x245a                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x03d6                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc918                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x1068                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x19c4                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x245a                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x03d6                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03ae22__L03ae4a
PlayerArm_SpriteTbl_03ae22__L03ae4a:
        bra.w   PlayerArm_SpriteTbl_03ae6a__L03ae92 | +028
        .global PlayerArm_SpriteTbl_03ae22__L03ae4e
PlayerArm_SpriteTbl_03ae22__L03ae4e:
        bclr    #0x2,0x8c(a6)                   | +02c
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +032
        lea     PlayerArm_SpriteTbl_03ae6a(pc),a0 | +036
        movea.l (a0,d0.w),a0                    | +03a
        jsr     0x28cd4.l                       | +03e
        bra.w   PlayerArm_SpriteTbl_03ae6a__L03ae92 | +044

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03ae6a  @ $03AE6A  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03ae6a, "ax", @progbits
        .global PlayerArm_SpriteTbl_03ae6a
PlayerArm_SpriteTbl_03ae6a:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xbfc8                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0c72                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1858                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x22c2                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xf63c                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xbfc8                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0c72                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1858                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x22c2                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xf63c                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03ae6a__L03ae92
PlayerArm_SpriteTbl_03ae6a__L03ae92:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_FallShootB_03ae94  @ $03AE94  (38 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_FallShootB_03ae94, "ax", @progbits
        .global PlayerArm_FallShootB_03ae94
PlayerArm_FallShootB_03ae94:
        cmpi.w  #0x1,0x72(a6)                   | +000
        bne.w   PlayerArm_SpriteTbl_03aeba__L03aee6 | +006
        bclr    #0x2,0x8c(a6)                   | +00a
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +010
        lea     PlayerArm_SpriteTbl_03aeba(pc),a0 | +014
        movea.l (a0,d0.w),a0                    | +018
        jsr     0x28cd4.l                       | +01c
        bra.w   PlayerArm_SpriteTbl_03aeba__L03aee2 | +022

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03aeba  @ $03AEBA  (82 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03aeba, "ax", @progbits
        .global PlayerArm_SpriteTbl_03aeba
PlayerArm_SpriteTbl_03aeba:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xca7c                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x11cc                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1b20                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x25b6                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x056a                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xca7c                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x11cc                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1b20                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x25b6                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x056a                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03aeba__L03aee2
PlayerArm_SpriteTbl_03aeba__L03aee2:
        bra.w   PlayerArm_SpriteTbl_03afa6__L03afce | +028
        .global PlayerArm_SpriteTbl_03aeba__L03aee6
PlayerArm_SpriteTbl_03aeba__L03aee6:
        cmpi.w  #0x2,0x72(a6)                   | +02c
        bne.w   PlayerArm_SpriteTbl_03af0c__L03af38 | +032
        bclr    #0x2,0x8c(a6)                   | +036
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +03c
        lea     PlayerArm_SpriteTbl_03af0c(pc),a0 | +040
        movea.l (a0,d0.w),a0                    | +044
        jsr     0x28cd4.l                       | +048
        bra.w   PlayerArm_SpriteTbl_03af0c__L03af34 | +04e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03af0c  @ $03AF0C  (82 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03af0c, "ax", @progbits
        .global PlayerArm_SpriteTbl_03af0c
PlayerArm_SpriteTbl_03af0c:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xcaf8                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x1240                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1b94                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x262a                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x05fe                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xcaf8                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x1240                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1b94                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x262a                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x05fe                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03af0c__L03af34
PlayerArm_SpriteTbl_03af0c__L03af34:
        bra.w   PlayerArm_SpriteTbl_03afa6__L03afce | +028
        .global PlayerArm_SpriteTbl_03af0c__L03af38
PlayerArm_SpriteTbl_03af0c__L03af38:
        cmpi.w  #0x3,0x72(a6)                   | +02c
        bne.w   PlayerArm_SpriteTbl_03af5e__L03af8a | +032
        bclr    #0x2,0x8c(a6)                   | +036
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +03c
        lea     PlayerArm_SpriteTbl_03af5e(pc),a0 | +040
        movea.l (a0,d0.w),a0                    | +044
        jsr     0x28cd4.l                       | +048
        bra.w   PlayerArm_SpriteTbl_03af5e__L03af86 | +04e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03af5e  @ $03AF5E  (72 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03af5e, "ax", @progbits
        .global PlayerArm_SpriteTbl_03af5e
PlayerArm_SpriteTbl_03af5e:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc918                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x1068                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x19c4                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x245a                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x03d6                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc918                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x1068                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x19c4                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x245a                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x03d6                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03af5e__L03af86
PlayerArm_SpriteTbl_03af5e__L03af86:
        bra.w   PlayerArm_SpriteTbl_03afa6__L03afce | +028
        .global PlayerArm_SpriteTbl_03af5e__L03af8a
PlayerArm_SpriteTbl_03af5e__L03af8a:
        bclr    #0x2,0x8c(a6)                   | +02c
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +032
        lea     PlayerArm_SpriteTbl_03afa6(pc),a0 | +036
        movea.l (a0,d0.w),a0                    | +03a
        jsr     0x28cd4.l                       | +03e
        bra.w   PlayerArm_SpriteTbl_03afa6__L03afce | +044

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03afa6  @ $03AFA6  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03afa6, "ax", @progbits
        .global PlayerArm_SpriteTbl_03afa6
PlayerArm_SpriteTbl_03afa6:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xbd30                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0c72                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1858                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x22c2                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xf4c2                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xbd30                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0c72                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1858                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x22c2                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xf4c2                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03afa6__L03afce
PlayerArm_SpriteTbl_03afa6__L03afce:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_JumpShootC_03afd0  @ $03AFD0  (38 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_JumpShootC_03afd0, "ax", @progbits
        .global PlayerArm_JumpShootC_03afd0
PlayerArm_JumpShootC_03afd0:
        cmpi.w  #0x1,0x72(a6)                   | +000
        bne.w   PlayerArm_SpriteTbl_03aff6__L03b022 | +006
        bclr    #0x2,0x8c(a6)                   | +00a
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +010
        lea     PlayerArm_SpriteTbl_03aff6(pc),a0 | +014
        movea.l (a0,d0.w),a0                    | +018
        jsr     0x28cd4.l                       | +01c
        bra.w   PlayerArm_SpriteTbl_03aff6__L03b01e | +022

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03aff6  @ $03AFF6  (82 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03aff6, "ax", @progbits
        .global PlayerArm_SpriteTbl_03aff6
PlayerArm_SpriteTbl_03aff6:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xcb74                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x12cc                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1c18                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x26ae                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x022a                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xcb74                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x12cc                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1c18                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x26ae                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x022a                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03aff6__L03b01e
PlayerArm_SpriteTbl_03aff6__L03b01e:
        bra.w   PlayerArm_SpriteTbl_03b0e2__L03b10a | +028
        .global PlayerArm_SpriteTbl_03aff6__L03b022
PlayerArm_SpriteTbl_03aff6__L03b022:
        cmpi.w  #0x0,0x72(a6)                   | +02c
        bne.w   PlayerArm_SpriteTbl_03b048__L03b074 | +032
        bclr    #0x2,0x8c(a6)                   | +036
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +03c
        lea     PlayerArm_SpriteTbl_03b048(pc),a0 | +040
        movea.l (a0,d0.w),a0                    | +044
        jsr     0x28cd4.l                       | +048
        bra.w   PlayerArm_SpriteTbl_03b048__L03b070 | +04e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03b048  @ $03B048  (82 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03b048, "ax", @progbits
        .global PlayerArm_SpriteTbl_03b048
PlayerArm_SpriteTbl_03b048:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xcc1e                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x136e                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1cba                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x2750                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0300                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xcc1e                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x136e                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1cba                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x2750                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0300                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03b048__L03b070
PlayerArm_SpriteTbl_03b048__L03b070:
        bra.w   PlayerArm_SpriteTbl_03b0e2__L03b10a | +028
        .global PlayerArm_SpriteTbl_03b048__L03b074
PlayerArm_SpriteTbl_03b048__L03b074:
        cmpi.w  #0x3,0x72(a6)                   | +02c
        bne.w   PlayerArm_SpriteTbl_03b09a__L03b0c6 | +032
        bclr    #0x2,0x8c(a6)                   | +036
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +03c
        lea     PlayerArm_SpriteTbl_03b09a(pc),a0 | +040
        movea.l (a0,d0.w),a0                    | +044
        jsr     0x28cd4.l                       | +048
        bra.w   PlayerArm_SpriteTbl_03b09a__L03b0c2 | +04e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03b09a  @ $03B09A  (72 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03b09a, "ax", @progbits
        .global PlayerArm_SpriteTbl_03b09a
PlayerArm_SpriteTbl_03b09a:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc98c                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x10dc                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1a38                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x24ce                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xffe2                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc98c                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x10dc                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1a38                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x24ce                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xffe2                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03b09a__L03b0c2
PlayerArm_SpriteTbl_03b09a__L03b0c2:
        bra.w   PlayerArm_SpriteTbl_03b0e2__L03b10a | +028
        .global PlayerArm_SpriteTbl_03b09a__L03b0c6
PlayerArm_SpriteTbl_03b09a__L03b0c6:
        bclr    #0x2,0x8c(a6)                   | +02c
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +032
        lea     PlayerArm_SpriteTbl_03b0e2(pc),a0 | +036
        movea.l (a0,d0.w),a0                    | +03a
        jsr     0x28cd4.l                       | +03e
        bra.w   PlayerArm_SpriteTbl_03b0e2__L03b10a | +044

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03b0e2  @ $03B0E2  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03b0e2, "ax", @progbits
        .global PlayerArm_SpriteTbl_03b0e2
PlayerArm_SpriteTbl_03b0e2:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc0d0                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0d12                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1864                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x22ce                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xf64e                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc0d0                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0d12                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1864                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x22ce                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xf64e                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03b0e2__L03b10a
PlayerArm_SpriteTbl_03b0e2__L03b10a:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_FallShootC_03b10c  @ $03B10C  (38 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_FallShootC_03b10c, "ax", @progbits
        .global PlayerArm_FallShootC_03b10c
PlayerArm_FallShootC_03b10c:
        cmpi.w  #0x1,0x72(a6)                   | +000
        bne.w   PlayerArm_SpriteTbl_03b132__L03b15e | +006
        bclr    #0x2,0x8c(a6)                   | +00a
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +010
        lea     PlayerArm_SpriteTbl_03b132(pc),a0 | +014
        movea.l (a0,d0.w),a0                    | +018
        jsr     0x28cd4.l                       | +01c
        bra.w   PlayerArm_SpriteTbl_03b132__L03b15a | +022

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03b132  @ $03B132  (82 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03b132, "ax", @progbits
        .global PlayerArm_SpriteTbl_03b132
PlayerArm_SpriteTbl_03b132:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xca10                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x1160                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1ab4                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x254a                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x007e                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xca10                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x1160                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1ab4                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x254a                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x007e                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03b132__L03b15a
PlayerArm_SpriteTbl_03b132__L03b15a:
        bra.w   PlayerArm_SpriteTbl_03b21e__L03b246 | +028
        .global PlayerArm_SpriteTbl_03b132__L03b15e
PlayerArm_SpriteTbl_03b132__L03b15e:
        cmpi.w  #0x0,0x72(a6)                   | +02c
        bne.w   PlayerArm_SpriteTbl_03b184__L03b1b0 | +032
        bclr    #0x2,0x8c(a6)                   | +036
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +03c
        lea     PlayerArm_SpriteTbl_03b184(pc),a0 | +040
        movea.l (a0,d0.w),a0                    | +044
        jsr     0x28cd4.l                       | +048
        bra.w   PlayerArm_SpriteTbl_03b184__L03b1ac | +04e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03b184  @ $03B184  (82 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03b184, "ax", @progbits
        .global PlayerArm_SpriteTbl_03b184
PlayerArm_SpriteTbl_03b184:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xcac2                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x1202                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1b56                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x25ec                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0154                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xcac2                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x1202                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1b56                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x25ec                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0154                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03b184__L03b1ac
PlayerArm_SpriteTbl_03b184__L03b1ac:
        bra.w   PlayerArm_SpriteTbl_03b21e__L03b246 | +028
        .global PlayerArm_SpriteTbl_03b184__L03b1b0
PlayerArm_SpriteTbl_03b184__L03b1b0:
        cmpi.w  #0x3,0x72(a6)                   | +02c
        bne.w   PlayerArm_SpriteTbl_03b1d6__L03b202 | +032
        bclr    #0x2,0x8c(a6)                   | +036
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +03c
        lea     PlayerArm_SpriteTbl_03b1d6(pc),a0 | +040
        movea.l (a0,d0.w),a0                    | +044
        jsr     0x28cd4.l                       | +048
        bra.w   PlayerArm_SpriteTbl_03b1d6__L03b1fe | +04e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03b1d6  @ $03B1D6  (72 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03b1d6, "ax", @progbits
        .global PlayerArm_SpriteTbl_03b1d6
PlayerArm_SpriteTbl_03b1d6:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc98c                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x10dc                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1a38                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x24ce                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xffe2                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc98c                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x10dc                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1a38                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x24ce                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xffe2                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03b1d6__L03b1fe
PlayerArm_SpriteTbl_03b1d6__L03b1fe:
        bra.w   PlayerArm_SpriteTbl_03b21e__L03b246 | +028
        .global PlayerArm_SpriteTbl_03b1d6__L03b202
PlayerArm_SpriteTbl_03b1d6__L03b202:
        bclr    #0x2,0x8c(a6)                   | +02c
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +032
        lea     PlayerArm_SpriteTbl_03b21e(pc),a0 | +036
        movea.l (a0,d0.w),a0                    | +03a
        jsr     0x28cd4.l                       | +03e
        bra.w   PlayerArm_SpriteTbl_03b21e__L03b246 | +044

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03b21e  @ $03B21E  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03b21e, "ax", @progbits
        .global PlayerArm_SpriteTbl_03b21e
PlayerArm_SpriteTbl_03b21e:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xbf60                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0d12                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1864                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x22ce                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xf4d4                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xbf60                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0d12                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1864                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x22ce                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xf4d4                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03b21e__L03b246
PlayerArm_SpriteTbl_03b21e__L03b246:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_JumpShootD_03b248  @ $03B248  (38 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_JumpShootD_03b248, "ax", @progbits
        .global PlayerArm_JumpShootD_03b248
PlayerArm_JumpShootD_03b248:
        cmpi.w  #0x1,0x72(a6)                   | +000
        bne.w   PlayerArm_SpriteTbl_03b26e__L03b29a | +006
        bclr    #0x2,0x8c(a6)                   | +00a
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +010
        lea     PlayerArm_SpriteTbl_03b26e(pc),a0 | +014
        movea.l (a0,d0.w),a0                    | +018
        jsr     0x28cd4.l                       | +01c
        bra.w   PlayerArm_SpriteTbl_03b26e__L03b296 | +022

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03b26e  @ $03B26E  (82 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03b26e, "ax", @progbits
        .global PlayerArm_SpriteTbl_03b26e
PlayerArm_SpriteTbl_03b26e:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xcb74                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x12cc                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1c18                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x26ae                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x068a                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xcb74                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x12cc                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1c18                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x26ae                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x068a                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03b26e__L03b296
PlayerArm_SpriteTbl_03b26e__L03b296:
        bra.w   PlayerArm_SpriteTbl_03b35a__L03b382 | +028
        .global PlayerArm_SpriteTbl_03b26e__L03b29a
PlayerArm_SpriteTbl_03b26e__L03b29a:
        cmpi.w  #0x0,0x72(a6)                   | +02c
        bne.w   PlayerArm_SpriteTbl_03b2c0__L03b2ec | +032
        bclr    #0x2,0x8c(a6)                   | +036
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +03c
        lea     PlayerArm_SpriteTbl_03b2c0(pc),a0 | +040
        movea.l (a0,d0.w),a0                    | +044
        jsr     0x28cd4.l                       | +048
        bra.w   PlayerArm_SpriteTbl_03b2c0__L03b2e8 | +04e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03b2c0  @ $03B2C0  (82 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03b2c0, "ax", @progbits
        .global PlayerArm_SpriteTbl_03b2c0
PlayerArm_SpriteTbl_03b2c0:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xcc1e                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x136e                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1cba                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x2750                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0760                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xcc1e                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x136e                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1cba                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x2750                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0760                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03b2c0__L03b2e8
PlayerArm_SpriteTbl_03b2c0__L03b2e8:
        bra.w   PlayerArm_SpriteTbl_03b35a__L03b382 | +028
        .global PlayerArm_SpriteTbl_03b2c0__L03b2ec
PlayerArm_SpriteTbl_03b2c0__L03b2ec:
        cmpi.w  #0x3,0x72(a6)                   | +02c
        bne.w   PlayerArm_SpriteTbl_03b312__L03b33e | +032
        bclr    #0x2,0x8c(a6)                   | +036
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +03c
        lea     PlayerArm_SpriteTbl_03b312(pc),a0 | +040
        movea.l (a0,d0.w),a0                    | +044
        jsr     0x28cd4.l                       | +048
        bra.w   PlayerArm_SpriteTbl_03b312__L03b33a | +04e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03b312  @ $03B312  (72 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03b312, "ax", @progbits
        .global PlayerArm_SpriteTbl_03b312
PlayerArm_SpriteTbl_03b312:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc98c                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x10dc                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1a38                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x24ce                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x045a                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc98c                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x10dc                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1a38                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x24ce                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x045a                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03b312__L03b33a
PlayerArm_SpriteTbl_03b312__L03b33a:
        bra.w   PlayerArm_SpriteTbl_03b35a__L03b382 | +028
        .global PlayerArm_SpriteTbl_03b312__L03b33e
PlayerArm_SpriteTbl_03b312__L03b33e:
        bclr    #0x2,0x8c(a6)                   | +02c
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +032
        lea     PlayerArm_SpriteTbl_03b35a(pc),a0 | +036
        movea.l (a0,d0.w),a0                    | +03a
        jsr     0x28cd4.l                       | +03e
        bra.w   PlayerArm_SpriteTbl_03b35a__L03b382 | +044

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03b35a  @ $03B35A  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03b35a, "ax", @progbits
        .global PlayerArm_SpriteTbl_03b35a
PlayerArm_SpriteTbl_03b35a:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc0d0                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0d12                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1864                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x22ce                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xf654                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc0d0                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0d12                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1864                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x22ce                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xf654                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03b35a__L03b382
PlayerArm_SpriteTbl_03b35a__L03b382:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_FallShootD_03b384  @ $03B384  (38 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_FallShootD_03b384, "ax", @progbits
        .global PlayerArm_FallShootD_03b384
PlayerArm_FallShootD_03b384:
        cmpi.w  #0x1,0x72(a6)                   | +000
        bne.w   PlayerArm_SpriteTbl_03b3aa__L03b3d6 | +006
        bclr    #0x2,0x8c(a6)                   | +00a
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +010
        lea     PlayerArm_SpriteTbl_03b3aa(pc),a0 | +014
        movea.l (a0,d0.w),a0                    | +018
        jsr     0x28cd4.l                       | +01c
        bra.w   PlayerArm_SpriteTbl_03b3aa__L03b3d2 | +022

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03b3aa  @ $03B3AA  (82 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03b3aa, "ax", @progbits
        .global PlayerArm_SpriteTbl_03b3aa
PlayerArm_SpriteTbl_03b3aa:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xca10                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x1160                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1ab4                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x254a                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x04de                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xca10                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x1160                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1ab4                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x254a                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x04de                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03b3aa__L03b3d2
PlayerArm_SpriteTbl_03b3aa__L03b3d2:
        bra.w   PlayerArm_SpriteTbl_03b496__L03b4be | +028
        .global PlayerArm_SpriteTbl_03b3aa__L03b3d6
PlayerArm_SpriteTbl_03b3aa__L03b3d6:
        cmpi.w  #0x0,0x72(a6)                   | +02c
        bne.w   PlayerArm_SpriteTbl_03b3fc__L03b428 | +032
        bclr    #0x2,0x8c(a6)                   | +036
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +03c
        lea     PlayerArm_SpriteTbl_03b3fc(pc),a0 | +040
        movea.l (a0,d0.w),a0                    | +044
        jsr     0x28cd4.l                       | +048
        bra.w   PlayerArm_SpriteTbl_03b3fc__L03b424 | +04e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03b3fc  @ $03B3FC  (82 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03b3fc, "ax", @progbits
        .global PlayerArm_SpriteTbl_03b3fc
PlayerArm_SpriteTbl_03b3fc:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xcac2                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x1202                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1b56                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x25ec                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x05b4                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xcac2                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x1202                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1b56                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x25ec                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x05b4                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03b3fc__L03b424
PlayerArm_SpriteTbl_03b3fc__L03b424:
        bra.w   PlayerArm_SpriteTbl_03b496__L03b4be | +028
        .global PlayerArm_SpriteTbl_03b3fc__L03b428
PlayerArm_SpriteTbl_03b3fc__L03b428:
        cmpi.w  #0x3,0x72(a6)                   | +02c
        bne.w   PlayerArm_SpriteTbl_03b44e__L03b47a | +032
        bclr    #0x2,0x8c(a6)                   | +036
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +03c
        lea     PlayerArm_SpriteTbl_03b44e(pc),a0 | +040
        movea.l (a0,d0.w),a0                    | +044
        jsr     0x28cd4.l                       | +048
        bra.w   PlayerArm_SpriteTbl_03b44e__L03b476 | +04e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03b44e  @ $03B44E  (72 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03b44e, "ax", @progbits
        .global PlayerArm_SpriteTbl_03b44e
PlayerArm_SpriteTbl_03b44e:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc98c                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x10dc                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1a38                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x24ce                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x045a                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc98c                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x10dc                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1a38                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x24ce                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x045a                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03b44e__L03b476
PlayerArm_SpriteTbl_03b44e__L03b476:
        bra.w   PlayerArm_SpriteTbl_03b496__L03b4be | +028
        .global PlayerArm_SpriteTbl_03b44e__L03b47a
PlayerArm_SpriteTbl_03b44e__L03b47a:
        bclr    #0x2,0x8c(a6)                   | +02c
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +032
        lea     PlayerArm_SpriteTbl_03b496(pc),a0 | +036
        movea.l (a0,d0.w),a0                    | +03a
        jsr     0x28cd4.l                       | +03e
        bra.w   PlayerArm_SpriteTbl_03b496__L03b4be | +044

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03b496  @ $03B496  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03b496, "ax", @progbits
        .global PlayerArm_SpriteTbl_03b496
PlayerArm_SpriteTbl_03b496:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xbf60                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0d12                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1864                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x22ce                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xf558                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xbf60                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0d12                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1864                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x22ce                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xf558                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03b496__L03b4be
PlayerArm_SpriteTbl_03b496__L03b4be:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_JumpShootDownA_03b4c0  @ $03B4C0  (38 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_JumpShootDownA_03b4c0, "ax", @progbits
        .global PlayerArm_JumpShootDownA_03b4c0
PlayerArm_JumpShootDownA_03b4c0:
        cmpi.w  #0x0,0x72(a6)                   | +000
        bne.w   PlayerArm_SpriteTbl_03b4e6__L03b512 | +006
        bclr    #0x2,0x8c(a6)                   | +00a
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +010
        lea     PlayerArm_SpriteTbl_03b4e6(pc),a0 | +014
        movea.l (a0,d0.w),a0                    | +018
        jsr     0x28cd4.l                       | +01c
        bra.w   PlayerArm_SpriteTbl_03b4e6__L03b50e | +022

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03b4e6  @ $03B4E6  (82 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03b4e6, "ax", @progbits
        .global PlayerArm_SpriteTbl_03b4e6
PlayerArm_SpriteTbl_03b4e6:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xcbb2                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x1302                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1c4e                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x26e4                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x026c                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xcbb2                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x1302                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1c4e                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x26e4                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x026c                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03b4e6__L03b50e
PlayerArm_SpriteTbl_03b4e6__L03b50e:
        bra.w   PlayerArm_SpriteTbl_03b580__L03b5a8 | +028
        .global PlayerArm_SpriteTbl_03b4e6__L03b512
PlayerArm_SpriteTbl_03b4e6__L03b512:
        cmpi.w  #0x2,0x72(a6)                   | +02c
        bne.w   PlayerArm_SpriteTbl_03b538__L03b564 | +032
        bclr    #0x2,0x8c(a6)                   | +036
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +03c
        lea     PlayerArm_SpriteTbl_03b538(pc),a0 | +040
        movea.l (a0,d0.w),a0                    | +044
        jsr     0x28cd4.l                       | +048
        bra.w   PlayerArm_SpriteTbl_03b538__L03b560 | +04e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03b538  @ $03B538  (72 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03b538, "ax", @progbits
        .global PlayerArm_SpriteTbl_03b538
PlayerArm_SpriteTbl_03b538:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xcb3e                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x128e                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1be2                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x2678                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x01e0                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xcb3e                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x128e                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1be2                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x2678                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x01e0                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03b538__L03b560
PlayerArm_SpriteTbl_03b538__L03b560:
        bra.w   PlayerArm_SpriteTbl_03b580__L03b5a8 | +028
        .global PlayerArm_SpriteTbl_03b538__L03b564
PlayerArm_SpriteTbl_03b538__L03b564:
        bclr    #0x2,0x8c(a6)                   | +02c
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +032
        lea     PlayerArm_SpriteTbl_03b580(pc),a0 | +036
        movea.l (a0,d0.w),a0                    | +03a
        jsr     0x28cd4.l                       | +03e
        bra.w   PlayerArm_SpriteTbl_03b580__L03b5a8 | +044

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03b580  @ $03B580  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03b580, "ax", @progbits
        .global PlayerArm_SpriteTbl_03b580
PlayerArm_SpriteTbl_03b580:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc0ca                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0c78                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x185e                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x22c8                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xf642                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc0ca                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0c78                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x185e                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x22c8                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xf642                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03b580__L03b5a8
PlayerArm_SpriteTbl_03b580__L03b5a8:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_FallShootDownA_03b5aa  @ $03B5AA  (38 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_FallShootDownA_03b5aa, "ax", @progbits
        .global PlayerArm_FallShootDownA_03b5aa
PlayerArm_FallShootDownA_03b5aa:
        cmpi.w  #0x0,0x72(a6)                   | +000
        bne.w   PlayerArm_SpriteTbl_03b5d0__L03b5fc | +006
        bclr    #0x2,0x8c(a6)                   | +00a
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +010
        lea     PlayerArm_SpriteTbl_03b5d0(pc),a0 | +014
        movea.l (a0,d0.w),a0                    | +018
        jsr     0x28cd4.l                       | +01c
        bra.w   PlayerArm_SpriteTbl_03b5d0__L03b5f8 | +022

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03b5d0  @ $03B5D0  (82 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03b5d0, "ax", @progbits
        .global PlayerArm_SpriteTbl_03b5d0
PlayerArm_SpriteTbl_03b5d0:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xca46                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x1196                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1aea                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x2580                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x00c0                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xca46                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x1196                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1aea                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x2580                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x00c0                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03b5d0__L03b5f8
PlayerArm_SpriteTbl_03b5d0__L03b5f8:
        bra.w   PlayerArm_SpriteTbl_03b66a__L03b692 | +028
        .global PlayerArm_SpriteTbl_03b5d0__L03b5fc
PlayerArm_SpriteTbl_03b5d0__L03b5fc:
        cmpi.w  #0x2,0x72(a6)                   | +02c
        bne.w   PlayerArm_SpriteTbl_03b622__L03b64e | +032
        bclr    #0x2,0x8c(a6)                   | +036
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +03c
        lea     PlayerArm_SpriteTbl_03b622(pc),a0 | +040
        movea.l (a0,d0.w),a0                    | +044
        jsr     0x28cd4.l                       | +048
        bra.w   PlayerArm_SpriteTbl_03b622__L03b64a | +04e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03b622  @ $03B622  (72 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03b622, "ax", @progbits
        .global PlayerArm_SpriteTbl_03b622
PlayerArm_SpriteTbl_03b622:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc9d2                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x1122                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1a7e                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x2514                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0034                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc9d2                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x1122                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1a7e                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x2514                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0034                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03b622__L03b64a
PlayerArm_SpriteTbl_03b622__L03b64a:
        bra.w   PlayerArm_SpriteTbl_03b66a__L03b692 | +028
        .global PlayerArm_SpriteTbl_03b622__L03b64e
PlayerArm_SpriteTbl_03b622__L03b64e:
        bclr    #0x2,0x8c(a6)                   | +02c
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +032
        lea     PlayerArm_SpriteTbl_03b66a(pc),a0 | +036
        movea.l (a0,d0.w),a0                    | +03a
        jsr     0x28cd4.l                       | +03e
        bra.w   PlayerArm_SpriteTbl_03b66a__L03b692 | +044

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03b66a  @ $03B66A  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03b66a, "ax", @progbits
        .global PlayerArm_SpriteTbl_03b66a
PlayerArm_SpriteTbl_03b66a:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xbe48                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0c78                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x185e                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x22c8                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xf4c8                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xbe48                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0c78                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x185e                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x22c8                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xf4c8                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03b66a__L03b692
PlayerArm_SpriteTbl_03b66a__L03b692:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_JumpShootDownB_03b694  @ $03B694  (38 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_JumpShootDownB_03b694, "ax", @progbits
        .global PlayerArm_JumpShootDownB_03b694
PlayerArm_JumpShootDownB_03b694:
        cmpi.w  #0x0,0x72(a6)                   | +000
        bne.w   PlayerArm_SpriteTbl_03b6ba__L03b6e6 | +006
        bclr    #0x2,0x8c(a6)                   | +00a
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +010
        lea     PlayerArm_SpriteTbl_03b6ba(pc),a0 | +014
        movea.l (a0,d0.w),a0                    | +018
        jsr     0x28cd4.l                       | +01c
        bra.w   PlayerArm_SpriteTbl_03b6ba__L03b6e2 | +022

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03b6ba  @ $03B6BA  (82 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03b6ba, "ax", @progbits
        .global PlayerArm_SpriteTbl_03b6ba
PlayerArm_SpriteTbl_03b6ba:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xcbb2                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x1302                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1c4e                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x26e4                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x06cc                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xcbb2                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x1302                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1c4e                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x26e4                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x06cc                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03b6ba__L03b6e2
PlayerArm_SpriteTbl_03b6ba__L03b6e2:
        bra.w   PlayerArm_SpriteTbl_03b754__L03b77c | +028
        .global PlayerArm_SpriteTbl_03b6ba__L03b6e6
PlayerArm_SpriteTbl_03b6ba__L03b6e6:
        cmpi.w  #0x2,0x72(a6)                   | +02c
        bne.w   PlayerArm_SpriteTbl_03b70c__L03b738 | +032
        bclr    #0x2,0x8c(a6)                   | +036
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +03c
        lea     PlayerArm_SpriteTbl_03b70c(pc),a0 | +040
        movea.l (a0,d0.w),a0                    | +044
        jsr     0x28cd4.l                       | +048
        bra.w   PlayerArm_SpriteTbl_03b70c__L03b734 | +04e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03b70c  @ $03B70C  (72 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03b70c, "ax", @progbits
        .global PlayerArm_SpriteTbl_03b70c
PlayerArm_SpriteTbl_03b70c:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xcb3e                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x128e                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1be2                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x2678                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0648                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xcb3e                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x128e                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1be2                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x2678                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0648                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03b70c__L03b734
PlayerArm_SpriteTbl_03b70c__L03b734:
        bra.w   PlayerArm_SpriteTbl_03b754__L03b77c | +028
        .global PlayerArm_SpriteTbl_03b70c__L03b738
PlayerArm_SpriteTbl_03b70c__L03b738:
        bclr    #0x2,0x8c(a6)                   | +02c
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +032
        lea     PlayerArm_SpriteTbl_03b754(pc),a0 | +036
        movea.l (a0,d0.w),a0                    | +03a
        jsr     0x28cd4.l                       | +03e
        bra.w   PlayerArm_SpriteTbl_03b754__L03b77c | +044

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03b754  @ $03B754  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03b754, "ax", @progbits
        .global PlayerArm_SpriteTbl_03b754
PlayerArm_SpriteTbl_03b754:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc0ca                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0c78                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x185e                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x22c8                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xf648                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc0ca                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0c78                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x185e                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x22c8                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xf648                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03b754__L03b77c
PlayerArm_SpriteTbl_03b754__L03b77c:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_FallShootDownB_03b77e  @ $03B77E  (38 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_FallShootDownB_03b77e, "ax", @progbits
        .global PlayerArm_FallShootDownB_03b77e
PlayerArm_FallShootDownB_03b77e:
        cmpi.w  #0x0,0x72(a6)                   | +000
        bne.w   PlayerArm_SpriteTbl_03b7a4__L03b7d0 | +006
        bclr    #0x2,0x8c(a6)                   | +00a
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +010
        lea     PlayerArm_SpriteTbl_03b7a4(pc),a0 | +014
        movea.l (a0,d0.w),a0                    | +018
        jsr     0x28cd4.l                       | +01c
        bra.w   PlayerArm_SpriteTbl_03b7a4__L03b7cc | +022

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03b7a4  @ $03B7A4  (82 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03b7a4, "ax", @progbits
        .global PlayerArm_SpriteTbl_03b7a4
PlayerArm_SpriteTbl_03b7a4:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xca46                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x1196                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1aea                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x2580                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0520                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xca46                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x1196                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1aea                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x2580                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0520                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03b7a4__L03b7cc
PlayerArm_SpriteTbl_03b7a4__L03b7cc:
        bra.w   PlayerArm_SpriteTbl_03b83e__L03b866 | +028
        .global PlayerArm_SpriteTbl_03b7a4__L03b7d0
PlayerArm_SpriteTbl_03b7a4__L03b7d0:
        cmpi.w  #0x2,0x72(a6)                   | +02c
        bne.w   PlayerArm_SpriteTbl_03b7f6__L03b822 | +032
        bclr    #0x2,0x8c(a6)                   | +036
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +03c
        lea     PlayerArm_SpriteTbl_03b7f6(pc),a0 | +040
        movea.l (a0,d0.w),a0                    | +044
        jsr     0x28cd4.l                       | +048
        bra.w   PlayerArm_SpriteTbl_03b7f6__L03b81e | +04e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03b7f6  @ $03B7F6  (72 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03b7f6, "ax", @progbits
        .global PlayerArm_SpriteTbl_03b7f6
PlayerArm_SpriteTbl_03b7f6:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc9d2                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x1122                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1a7e                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x2514                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x049c                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc9d2                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x1122                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1a7e                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x2514                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x049c                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03b7f6__L03b81e
PlayerArm_SpriteTbl_03b7f6__L03b81e:
        bra.w   PlayerArm_SpriteTbl_03b83e__L03b866 | +028
        .global PlayerArm_SpriteTbl_03b7f6__L03b822
PlayerArm_SpriteTbl_03b7f6__L03b822:
        bclr    #0x2,0x8c(a6)                   | +02c
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +032
        lea     PlayerArm_SpriteTbl_03b83e(pc),a0 | +036
        movea.l (a0,d0.w),a0                    | +03a
        jsr     0x28cd4.l                       | +03e
        bra.w   PlayerArm_SpriteTbl_03b83e__L03b866 | +044

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03b83e  @ $03B83E  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03b83e, "ax", @progbits
        .global PlayerArm_SpriteTbl_03b83e
PlayerArm_SpriteTbl_03b83e:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xbe48                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0c78                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x185e                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x22c8                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xf4ce                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xbe48                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0c78                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x185e                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x22c8                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xf4ce                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03b83e__L03b866
PlayerArm_SpriteTbl_03b83e__L03b866:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_JumpShootDownC_03b868  @ $03B868  (38 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_JumpShootDownC_03b868, "ax", @progbits
        .global PlayerArm_JumpShootDownC_03b868
PlayerArm_JumpShootDownC_03b868:
        cmpi.w  #0x0,0x72(a6)                   | +000
        bne.w   PlayerArm_SpriteTbl_03b88e__L03b8ba | +006
        bclr    #0x2,0x8c(a6)                   | +00a
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +010
        lea     PlayerArm_SpriteTbl_03b88e(pc),a0 | +014
        movea.l (a0,d0.w),a0                    | +018
        jsr     0x28cd4.l                       | +01c
        bra.w   PlayerArm_SpriteTbl_03b88e__L03b8b6 | +022

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03b88e  @ $03B88E  (82 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03b88e, "ax", @progbits
        .global PlayerArm_SpriteTbl_03b88e
PlayerArm_SpriteTbl_03b88e:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc8e2                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x1032                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x198e                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x2424                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xff14                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc8e2                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x1032                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x198e                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x2424                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xff14                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03b88e__L03b8b6
PlayerArm_SpriteTbl_03b88e__L03b8b6:
        bra.w   PlayerArm_SpriteTbl_03b928__L03b950 | +028
        .global PlayerArm_SpriteTbl_03b88e__L03b8ba
PlayerArm_SpriteTbl_03b88e__L03b8ba:
        cmpi.w  #0x2,0x72(a6)                   | +02c
        bne.w   PlayerArm_SpriteTbl_03b8e0__L03b90c | +032
        bclr    #0x2,0x8c(a6)                   | +036
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +03c
        lea     PlayerArm_SpriteTbl_03b8e0(pc),a0 | +040
        movea.l (a0,d0.w),a0                    | +044
        jsr     0x28cd4.l                       | +048
        bra.w   PlayerArm_SpriteTbl_03b8e0__L03b908 | +04e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03b8e0  @ $03B8E0  (72 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03b8e0, "ax", @progbits
        .global PlayerArm_SpriteTbl_03b8e0
PlayerArm_SpriteTbl_03b8e0:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc94e                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x109e                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x19fa                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x2490                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xff98                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc94e                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x109e                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x19fa                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x2490                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xff98                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03b8e0__L03b908
PlayerArm_SpriteTbl_03b8e0__L03b908:
        bra.w   PlayerArm_SpriteTbl_03b928__L03b950 | +028
        .global PlayerArm_SpriteTbl_03b8e0__L03b90c
PlayerArm_SpriteTbl_03b8e0__L03b90c:
        bclr    #0x2,0x8c(a6)                   | +02c
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +032
        lea     PlayerArm_SpriteTbl_03b928(pc),a0 | +036
        movea.l (a0,d0.w),a0                    | +03a
        jsr     0x28cd4.l                       | +03e
        bra.w   PlayerArm_SpriteTbl_03b928__L03b950 | +044

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03b928  @ $03B928  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03b928, "ax", @progbits
        .global PlayerArm_SpriteTbl_03b928
PlayerArm_SpriteTbl_03b928:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc0f2                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x099c                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1582                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x1f26                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xf676                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc0f2                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x099c                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1582                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x1f26                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xf676                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03b928__L03b950
PlayerArm_SpriteTbl_03b928__L03b950:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_FallShootDownC_03b952  @ $03B952  (38 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_FallShootDownC_03b952, "ax", @progbits
        .global PlayerArm_FallShootDownC_03b952
PlayerArm_FallShootDownC_03b952:
        cmpi.w  #0x0,0x72(a6)                   | +000
        bne.w   PlayerArm_SpriteTbl_03b978__L03b9a4 | +006
        bclr    #0x2,0x8c(a6)                   | +00a
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +010
        lea     PlayerArm_SpriteTbl_03b978(pc),a0 | +014
        movea.l (a0,d0.w),a0                    | +018
        jsr     0x28cd4.l                       | +01c
        bra.w   PlayerArm_SpriteTbl_03b978__L03b9a0 | +022

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03b978  @ $03B978  (82 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03b978, "ax", @progbits
        .global PlayerArm_SpriteTbl_03b978
PlayerArm_SpriteTbl_03b978:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc8e2                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x1032                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x198e                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x2424                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xff14                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc8e2                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x1032                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x198e                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x2424                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xff14                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03b978__L03b9a0
PlayerArm_SpriteTbl_03b978__L03b9a0:
        bra.w   PlayerArm_SpriteTbl_03ba12__L03ba3a | +028
        .global PlayerArm_SpriteTbl_03b978__L03b9a4
PlayerArm_SpriteTbl_03b978__L03b9a4:
        cmpi.w  #0x2,0x72(a6)                   | +02c
        bne.w   PlayerArm_SpriteTbl_03b9ca__L03b9f6 | +032
        bclr    #0x2,0x8c(a6)                   | +036
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +03c
        lea     PlayerArm_SpriteTbl_03b9ca(pc),a0 | +040
        movea.l (a0,d0.w),a0                    | +044
        jsr     0x28cd4.l                       | +048
        bra.w   PlayerArm_SpriteTbl_03b9ca__L03b9f2 | +04e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03b9ca  @ $03B9CA  (72 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03b9ca, "ax", @progbits
        .global PlayerArm_SpriteTbl_03b9ca
PlayerArm_SpriteTbl_03b9ca:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc94e                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x109e                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x19fa                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x2490                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xff98                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc94e                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x109e                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x19fa                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x2490                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xff98                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03b9ca__L03b9f2
PlayerArm_SpriteTbl_03b9ca__L03b9f2:
        bra.w   PlayerArm_SpriteTbl_03ba12__L03ba3a | +028
        .global PlayerArm_SpriteTbl_03b9ca__L03b9f6
PlayerArm_SpriteTbl_03b9ca__L03b9f6:
        bclr    #0x2,0x8c(a6)                   | +02c
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +032
        lea     PlayerArm_SpriteTbl_03ba12(pc),a0 | +036
        movea.l (a0,d0.w),a0                    | +03a
        jsr     0x28cd4.l                       | +03e
        bra.w   PlayerArm_SpriteTbl_03ba12__L03ba3a | +044

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03ba12  @ $03BA12  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03ba12, "ax", @progbits
        .global PlayerArm_SpriteTbl_03ba12
PlayerArm_SpriteTbl_03ba12:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc0f2                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x099c                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1582                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x1f26                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xf676                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc0f2                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x099c                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1582                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x1f26                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xf676                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03ba12__L03ba3a
PlayerArm_SpriteTbl_03ba12__L03ba3a:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_ShootDownUnrefA_03ba3c  @ $03BA3C  (38 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_ShootDownUnrefA_03ba3c, "ax", @progbits
        .global PlayerArm_ShootDownUnrefA_03ba3c
PlayerArm_ShootDownUnrefA_03ba3c:
        cmpi.w  #0x0,0x72(a6)                   | +000
        bne.w   PlayerArm_SpriteTbl_03ba62__L03ba8e | +006
        bclr    #0x2,0x8c(a6)                   | +00a
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +010
        lea     PlayerArm_SpriteTbl_03ba62(pc),a0 | +014
        movea.l (a0,d0.w),a0                    | +018
        jsr     0x28cd4.l                       | +01c
        bra.w   PlayerArm_SpriteTbl_03ba62__L03ba8a | +022

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03ba62  @ $03BA62  (82 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03ba62, "ax", @progbits
        .global PlayerArm_SpriteTbl_03ba62
PlayerArm_SpriteTbl_03ba62:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc8e2                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x1032                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x198e                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x2424                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0394                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc8e2                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x1032                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x198e                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x2424                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0394                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03ba62__L03ba8a
PlayerArm_SpriteTbl_03ba62__L03ba8a:
        bra.w   PlayerArm_SpriteTbl_03bafc__L03bb24 | +028
        .global PlayerArm_SpriteTbl_03ba62__L03ba8e
PlayerArm_SpriteTbl_03ba62__L03ba8e:
        cmpi.w  #0x2,0x72(a6)                   | +02c
        bne.w   PlayerArm_SpriteTbl_03bab4__L03bae0 | +032
        bclr    #0x2,0x8c(a6)                   | +036
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +03c
        lea     PlayerArm_SpriteTbl_03bab4(pc),a0 | +040
        movea.l (a0,d0.w),a0                    | +044
        jsr     0x28cd4.l                       | +048
        bra.w   PlayerArm_SpriteTbl_03bab4__L03badc | +04e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03bab4  @ $03BAB4  (72 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03bab4, "ax", @progbits
        .global PlayerArm_SpriteTbl_03bab4
PlayerArm_SpriteTbl_03bab4:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc94e                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x109e                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x19fa                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x2490                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0418                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc94e                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x109e                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x19fa                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x2490                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0418                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03bab4__L03badc
PlayerArm_SpriteTbl_03bab4__L03badc:
        bra.w   PlayerArm_SpriteTbl_03bafc__L03bb24 | +028
        .global PlayerArm_SpriteTbl_03bab4__L03bae0
PlayerArm_SpriteTbl_03bab4__L03bae0:
        bclr    #0x2,0x8c(a6)                   | +02c
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +032
        lea     PlayerArm_SpriteTbl_03bafc(pc),a0 | +036
        movea.l (a0,d0.w),a0                    | +03a
        jsr     0x28cd4.l                       | +03e
        bra.w   PlayerArm_SpriteTbl_03bafc__L03bb24 | +044

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03bafc  @ $03BAFC  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03bafc, "ax", @progbits
        .global PlayerArm_SpriteTbl_03bafc
PlayerArm_SpriteTbl_03bafc:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc0f2                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x099c                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1582                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x1f26                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xf676                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc0f2                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x099c                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1582                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x1f26                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xf676                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03bafc__L03bb24
PlayerArm_SpriteTbl_03bafc__L03bb24:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_ShootDownUnrefB_03bb26  @ $03BB26  (38 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_ShootDownUnrefB_03bb26, "ax", @progbits
        .global PlayerArm_ShootDownUnrefB_03bb26
PlayerArm_ShootDownUnrefB_03bb26:
        cmpi.w  #0x0,0x72(a6)                   | +000
        bne.w   PlayerArm_SpriteTbl_03bb4c__L03bb78 | +006
        bclr    #0x2,0x8c(a6)                   | +00a
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +010
        lea     PlayerArm_SpriteTbl_03bb4c(pc),a0 | +014
        movea.l (a0,d0.w),a0                    | +018
        jsr     0x28cd4.l                       | +01c
        bra.w   PlayerArm_SpriteTbl_03bb4c__L03bb74 | +022

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03bb4c  @ $03BB4C  (82 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03bb4c, "ax", @progbits
        .global PlayerArm_SpriteTbl_03bb4c
PlayerArm_SpriteTbl_03bb4c:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc8e2                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x1032                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x198e                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x2424                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0394                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc8e2                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x1032                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x198e                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x2424                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0394                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03bb4c__L03bb74
PlayerArm_SpriteTbl_03bb4c__L03bb74:
        bra.w   PlayerArm_SpriteTbl_03bbe6__L03bc0e | +028
        .global PlayerArm_SpriteTbl_03bb4c__L03bb78
PlayerArm_SpriteTbl_03bb4c__L03bb78:
        cmpi.w  #0x2,0x72(a6)                   | +02c
        bne.w   PlayerArm_SpriteTbl_03bb9e__L03bbca | +032
        bclr    #0x2,0x8c(a6)                   | +036
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +03c
        lea     PlayerArm_SpriteTbl_03bb9e(pc),a0 | +040
        movea.l (a0,d0.w),a0                    | +044
        jsr     0x28cd4.l                       | +048
        bra.w   PlayerArm_SpriteTbl_03bb9e__L03bbc6 | +04e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03bb9e  @ $03BB9E  (72 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03bb9e, "ax", @progbits
        .global PlayerArm_SpriteTbl_03bb9e
PlayerArm_SpriteTbl_03bb9e:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc94e                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x109e                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x19fa                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x2490                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0418                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc94e                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x109e                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x19fa                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x2490                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0418                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03bb9e__L03bbc6
PlayerArm_SpriteTbl_03bb9e__L03bbc6:
        bra.w   PlayerArm_SpriteTbl_03bbe6__L03bc0e | +028
        .global PlayerArm_SpriteTbl_03bb9e__L03bbca
PlayerArm_SpriteTbl_03bb9e__L03bbca:
        bclr    #0x2,0x8c(a6)                   | +02c
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +032
        lea     PlayerArm_SpriteTbl_03bbe6(pc),a0 | +036
        movea.l (a0,d0.w),a0                    | +03a
        jsr     0x28cd4.l                       | +03e
        bra.w   PlayerArm_SpriteTbl_03bbe6__L03bc0e | +044

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03bbe6  @ $03BBE6  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03bbe6, "ax", @progbits
        .global PlayerArm_SpriteTbl_03bbe6
PlayerArm_SpriteTbl_03bbe6:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc0f2                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x099c                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1582                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x1f26                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xf700                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc0f2                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x099c                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1582                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x1f26                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xf700                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03bbe6__L03bc0e
PlayerArm_SpriteTbl_03bbe6__L03bc0e:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_DeathA_03bc10  @ $03BC10  (40 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_DeathA_03bc10, "ax", @progbits
        .global PlayerArm_DeathA_03bc10
PlayerArm_DeathA_03bc10:
        bclr    #0x2,0x8c(a6)                   | +000
        bset    #0x0,0x13(a6)                   | +006
        bclr    #0x2,0x8c(a6)                   | +00c
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +012
        lea     PlayerArm_SpriteTbl_03bc38(pc),a0 | +016
        movea.l (a0,d0.w),a0                    | +01a
        jsr     0x28cd4.l                       | +01e
        bra.w   PlayerArm_SpriteTbl_03bc38__L03bc60 | +024

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03bc38  @ $03BC38  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03bc38, "ax", @progbits
        .global PlayerArm_SpriteTbl_03bc38
PlayerArm_SpriteTbl_03bc38:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xd3d8                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xd3d8                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xd3d8                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd3d8                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xd3d8                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xd3d8                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xd3d8                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xd3d8                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xd3d8                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xd3d8                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03bc38__L03bc60
PlayerArm_SpriteTbl_03bc38__L03bc60:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_DeathB_03bc62  @ $03BC62  (40 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_DeathB_03bc62, "ax", @progbits
        .global PlayerArm_DeathB_03bc62
PlayerArm_DeathB_03bc62:
        bclr    #0x2,0x8c(a6)                   | +000
        bset    #0x0,0x13(a6)                   | +006
        bclr    #0x2,0x8c(a6)                   | +00c
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +012
        lea     PlayerArm_SpriteTbl_03bc8a(pc),a0 | +016
        movea.l (a0,d0.w),a0                    | +01a
        jsr     0x28cd4.l                       | +01e
        bra.w   PlayerArm_SpriteTbl_03bc8a__L03bcb2 | +024

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03bc8a  @ $03BC8A  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03bc8a, "ax", @progbits
        .global PlayerArm_SpriteTbl_03bc8a
PlayerArm_SpriteTbl_03bc8a:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xd3e4                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xd3e4                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xd3e4                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd3e4                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xd3e4                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xd3e4                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xd3e4                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xd3e4                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xd3e4                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xd3e4                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03bc8a__L03bcb2
PlayerArm_SpriteTbl_03bc8a__L03bcb2:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_DeathC_03bcb4  @ $03BCB4  (40 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_DeathC_03bcb4, "ax", @progbits
        .global PlayerArm_DeathC_03bcb4
PlayerArm_DeathC_03bcb4:
        bclr    #0x2,0x8c(a6)                   | +000
        bset    #0x0,0x13(a6)                   | +006
        bclr    #0x2,0x8c(a6)                   | +00c
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +012
        lea     PlayerArm_SpriteTbl_03bcdc(pc),a0 | +016
        movea.l (a0,d0.w),a0                    | +01a
        jsr     0x28cd4.l                       | +01e
        bra.w   PlayerArm_SpriteTbl_03bcdc__L03bd04 | +024

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03bcdc  @ $03BCDC  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03bcdc, "ax", @progbits
        .global PlayerArm_SpriteTbl_03bcdc
PlayerArm_SpriteTbl_03bcdc:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xd33c                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xd33c                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xd33c                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd33c                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xd33c                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xd33c                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xd33c                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xd33c                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xd33c                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xd33c                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03bcdc__L03bd04
PlayerArm_SpriteTbl_03bcdc__L03bd04:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_DeathD_03bd06  @ $03BD06  (40 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_DeathD_03bd06, "ax", @progbits
        .global PlayerArm_DeathD_03bd06
PlayerArm_DeathD_03bd06:
        bclr    #0x2,0x8c(a6)                   | +000
        bset    #0x0,0x13(a6)                   | +006
        bclr    #0x2,0x8c(a6)                   | +00c
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +012
        lea     PlayerArm_SpriteTbl_03bd2e(pc),a0 | +016
        movea.l (a0,d0.w),a0                    | +01a
        jsr     0x28cd4.l                       | +01e
        bra.w   PlayerArm_SpriteTbl_03bd2e__L03bd56 | +024

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03bd2e  @ $03BD2E  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03bd2e, "ax", @progbits
        .global PlayerArm_SpriteTbl_03bd2e
PlayerArm_SpriteTbl_03bd2e:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xd788                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xd788                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xd788                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd788                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xd788                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xd788                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xd788                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xd788                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xd788                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xd788                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03bd2e__L03bd56
PlayerArm_SpriteTbl_03bd2e__L03bd56:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_DeathE_03bd58  @ $03BD58  (40 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_DeathE_03bd58, "ax", @progbits
        .global PlayerArm_DeathE_03bd58
PlayerArm_DeathE_03bd58:
        bclr    #0x2,0x8c(a6)                   | +000
        bset    #0x0,0x13(a6)                   | +006
        bclr    #0x2,0x8c(a6)                   | +00c
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +012
        lea     PlayerArm_SpriteTbl_03bd80(pc),a0 | +016
        movea.l (a0,d0.w),a0                    | +01a
        jsr     0x28cd4.l                       | +01e
        bra.w   PlayerArm_SpriteTbl_03bd80__L03bda8 | +024

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03bd80  @ $03BD80  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03bd80, "ax", @progbits
        .global PlayerArm_SpriteTbl_03bd80
PlayerArm_SpriteTbl_03bd80:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xd14c                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xd14c                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xd14c                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd14c                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xd14c                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xd14c                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xd14c                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xd14c                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xd14c                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xd14c                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03bd80__L03bda8
PlayerArm_SpriteTbl_03bd80__L03bda8:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_DeathF_03bdaa  @ $03BDAA  (40 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_DeathF_03bdaa, "ax", @progbits
        .global PlayerArm_DeathF_03bdaa
PlayerArm_DeathF_03bdaa:
        bclr    #0x2,0x8c(a6)                   | +000
        bset    #0x0,0x13(a6)                   | +006
        bclr    #0x2,0x8c(a6)                   | +00c
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +012
        lea     PlayerArm_SpriteTbl_03bdd2(pc),a0 | +016
        movea.l (a0,d0.w),a0                    | +01a
        jsr     0x28cd4.l                       | +01e
        bra.w   PlayerArm_SpriteTbl_03bdd2__L03bdfa | +024

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03bdd2  @ $03BDD2  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03bdd2, "ax", @progbits
        .global PlayerArm_SpriteTbl_03bdd2
PlayerArm_SpriteTbl_03bdd2:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xd2b2                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xd2b2                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xd2b2                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd2b2                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xd2b2                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xd2b2                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xd2b2                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xd2b2                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xd2b2                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xd2b2                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03bdd2__L03bdfa
PlayerArm_SpriteTbl_03bdd2__L03bdfa:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_DeathG_03bdfc  @ $03BDFC  (40 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_DeathG_03bdfc, "ax", @progbits
        .global PlayerArm_DeathG_03bdfc
PlayerArm_DeathG_03bdfc:
        bclr    #0x2,0x8c(a6)                   | +000
        bset    #0x0,0x13(a6)                   | +006
        bclr    #0x2,0x8c(a6)                   | +00c
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +012
        lea     PlayerArm_SpriteTbl_03be24(pc),a0 | +016
        movea.l (a0,d0.w),a0                    | +01a
        jsr     0x28cd4.l                       | +01e
        bra.w   PlayerArm_SpriteTbl_03be24__L03be4c | +024

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03be24  @ $03BE24  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03be24, "ax", @progbits
        .global PlayerArm_SpriteTbl_03be24
PlayerArm_SpriteTbl_03be24:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xd812                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xd812                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xd812                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd812                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xd812                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xd812                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xd812                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xd812                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xd812                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xd812                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03be24__L03be4c
PlayerArm_SpriteTbl_03be24__L03be4c:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_DeathH_03be4e  @ $03BE4E  (40 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_DeathH_03be4e, "ax", @progbits
        .global PlayerArm_DeathH_03be4e
PlayerArm_DeathH_03be4e:
        bclr    #0x2,0x8c(a6)                   | +000
        bset    #0x0,0x13(a6)                   | +006
        bclr    #0x2,0x8c(a6)                   | +00c
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +012
        lea     PlayerArm_SpriteTbl_03be76(pc),a0 | +016
        movea.l (a0,d0.w),a0                    | +01a
        jsr     0x28cd4.l                       | +01e
        bra.w   PlayerArm_SpriteTbl_03be76__L03be9e | +024

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03be76  @ $03BE76  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03be76, "ax", @progbits
        .global PlayerArm_SpriteTbl_03be76
PlayerArm_SpriteTbl_03be76:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xcf78                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xcf78                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xcf78                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xcf78                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xcf78                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xcf78                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xcf78                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xcf78                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xcf78                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xcf78                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03be76__L03be9e
PlayerArm_SpriteTbl_03be76__L03be9e:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_SpawnFall_03bea0  @ $03BEA0  (34 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpawnFall_03bea0, "ax", @progbits
        .global PlayerArm_SpawnFall_03bea0
PlayerArm_SpawnFall_03bea0:
        bclr    #0x2,0x8c(a6)                   | +000
        bclr    #0x2,0x8c(a6)                   | +006
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +00c
        lea     PlayerArm_SpriteTbl_03bec2(pc),a0 | +010
        movea.l (a0,d0.w),a0                    | +014
        jsr     0x28cd4.l                       | +018
        bra.w   PlayerArm_SpriteTbl_03bec2__L03beea | +01e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03bec2  @ $03BEC2  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03bec2, "ax", @progbits
        .global PlayerArm_SpriteTbl_03bec2
PlayerArm_SpriteTbl_03bec2:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xce9a                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xce9a                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xce9a                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xce9a                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xce9a                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xce9a                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xce9a                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xce9a                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xce9a                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xce9a                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03bec2__L03beea
PlayerArm_SpriteTbl_03bec2__L03beea:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_CrouchEnterA_03beec  @ $03BEEC  (34 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_CrouchEnterA_03beec, "ax", @progbits
        .global PlayerArm_CrouchEnterA_03beec
PlayerArm_CrouchEnterA_03beec:
        bclr    #0x2,0x8c(a6)                   | +000
        bclr    #0x2,0x8c(a6)                   | +006
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +00c
        lea     PlayerArm_SpriteTbl_03bf0e(pc),a0 | +010
        movea.l (a0,d0.w),a0                    | +014
        jsr     0x28cd4.l                       | +018
        bra.w   PlayerArm_SpriteTbl_03bf0e__L03bf36 | +01e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03bf0e  @ $03BF0E  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03bf0e, "ax", @progbits
        .global PlayerArm_SpriteTbl_03bf0e
PlayerArm_SpriteTbl_03bf0e:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xb2f6                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xe91e                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xe91e                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xe91e                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xe91e                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xb2f6                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xe91e                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xe91e                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xe91e                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xe91e                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03bf0e__L03bf36
PlayerArm_SpriteTbl_03bf0e__L03bf36:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_CrouchUnref_03bf38  @ $03BF38  (34 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_CrouchUnref_03bf38, "ax", @progbits
        .global PlayerArm_CrouchUnref_03bf38
PlayerArm_CrouchUnref_03bf38:
        bclr    #0x2,0x8c(a6)                   | +000
        bclr    #0x2,0x8c(a6)                   | +006
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +00c
        lea     PlayerArm_SpriteTbl_03bf5a(pc),a0 | +010
        movea.l (a0,d0.w),a0                    | +014
        jsr     0x28cd4.l                       | +018
        bra.w   PlayerArm_SpriteTbl_03bf5a__L03bf82 | +01e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03bf5a  @ $03BF5A  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03bf5a, "ax", @progbits
        .global PlayerArm_SpriteTbl_03bf5a
PlayerArm_SpriteTbl_03bf5a:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xb2f6                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xe91e                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xe91e                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xe91e                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xe91e                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xb2f6                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xe91e                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xe91e                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xe91e                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xe91e                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03bf5a__L03bf82
PlayerArm_SpriteTbl_03bf5a__L03bf82:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_CrouchEnterB_03bf84  @ $03BF84  (34 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_CrouchEnterB_03bf84, "ax", @progbits
        .global PlayerArm_CrouchEnterB_03bf84
PlayerArm_CrouchEnterB_03bf84:
        bclr    #0x2,0x8c(a6)                   | +000
        bclr    #0x2,0x8c(a6)                   | +006
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +00c
        lea     PlayerArm_SpriteTbl_03bfa6(pc),a0 | +010
        movea.l (a0,d0.w),a0                    | +014
        jsr     0x28cd4.l                       | +018
        bra.w   PlayerArm_SpriteTbl_03bfa6__L03bfce | +01e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03bfa6  @ $03BFA6  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03bfa6, "ax", @progbits
        .global PlayerArm_SpriteTbl_03bfa6
PlayerArm_SpriteTbl_03bfa6:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xb33a                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xe962                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xe962                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xe962                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xe962                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xb33a                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xe962                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xe962                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xe962                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xe962                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03bfa6__L03bfce
PlayerArm_SpriteTbl_03bfa6__L03bfce:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_CrouchIdleA_03bfd0  @ $03BFD0  (44 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_CrouchIdleA_03bfd0, "ax", @progbits
        .global PlayerArm_CrouchIdleA_03bfd0
PlayerArm_CrouchIdleA_03bfd0:
        bclr    #0x2,0x8c(a6)                   | +000
        jsr     0x2abcc.l                       | +006
        bcs.w   PlayerArm_SpriteTbl_03bffc__L03c028 | +00c
        bclr    #0x2,0x8c(a6)                   | +010
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +016
        lea     PlayerArm_SpriteTbl_03bffc(pc),a0 | +01a
        movea.l (a0,d0.w),a0                    | +01e
        jsr     0x28cd4.l                       | +022
        bra.w   PlayerArm_SpriteTbl_03bffc__L03c024 | +028

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03bffc  @ $03BFFC  (72 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03bffc, "ax", @progbits
        .global PlayerArm_SpriteTbl_03bffc
PlayerArm_SpriteTbl_03bffc:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xb1d6                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xe7fe                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xe7fe                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xe7fe                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xe7fe                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xb1d6                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xe7fe                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xe7fe                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xe7fe                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xe7fe                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03bffc__L03c024
PlayerArm_SpriteTbl_03bffc__L03c024:
        bra.w   PlayerArm_SpriteTbl_03c044__L03c06c | +028
        .global PlayerArm_SpriteTbl_03bffc__L03c028
PlayerArm_SpriteTbl_03bffc__L03c028:
        bclr    #0x2,0x8c(a6)                   | +02c
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +032
        lea     PlayerArm_SpriteTbl_03c044(pc),a0 | +036
        movea.l (a0,d0.w),a0                    | +03a
        jsr     0x28cd4.l                       | +03e
        bra.w   PlayerArm_SpriteTbl_03c044__L03c06c | +044

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03c044  @ $03C044  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03c044, "ax", @progbits
        .global PlayerArm_SpriteTbl_03c044
PlayerArm_SpriteTbl_03c044:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xb736                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xb736                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03c044__L03c06c
PlayerArm_SpriteTbl_03c044__L03c06c:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_CrouchIdleB_03c06e  @ $03C06E  (44 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_CrouchIdleB_03c06e, "ax", @progbits
        .global PlayerArm_CrouchIdleB_03c06e
PlayerArm_CrouchIdleB_03c06e:
        bclr    #0x2,0x8c(a6)                   | +000
        jsr     0x2abcc.l                       | +006
        bcs.w   PlayerArm_SpriteTbl_03c09a__L03c0c6 | +00c
        bclr    #0x2,0x8c(a6)                   | +010
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +016
        lea     PlayerArm_SpriteTbl_03c09a(pc),a0 | +01a
        movea.l (a0,d0.w),a0                    | +01e
        jsr     0x28cd4.l                       | +022
        bra.w   PlayerArm_SpriteTbl_03c09a__L03c0c2 | +028

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03c09a  @ $03C09A  (72 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03c09a, "ax", @progbits
        .global PlayerArm_SpriteTbl_03c09a
PlayerArm_SpriteTbl_03c09a:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xb266                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xe88e                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xe88e                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xe88e                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xe88e                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xb266                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xe88e                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xe88e                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xe88e                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xe88e                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03c09a__L03c0c2
PlayerArm_SpriteTbl_03c09a__L03c0c2:
        bra.w   PlayerArm_SpriteTbl_03c0e2__L03c10a | +028
        .global PlayerArm_SpriteTbl_03c09a__L03c0c6
PlayerArm_SpriteTbl_03c09a__L03c0c6:
        bclr    #0x2,0x8c(a6)                   | +02c
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +032
        lea     PlayerArm_SpriteTbl_03c0e2(pc),a0 | +036
        movea.l (a0,d0.w),a0                    | +03a
        jsr     0x28cd4.l                       | +03e
        bra.w   PlayerArm_SpriteTbl_03c0e2__L03c10a | +044

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03c0e2  @ $03C0E2  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03c0e2, "ax", @progbits
        .global PlayerArm_SpriteTbl_03c0e2
PlayerArm_SpriteTbl_03c0e2:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xb736                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xb736                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03c0e2__L03c10a
PlayerArm_SpriteTbl_03c0e2__L03c10a:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_CrouchShoot_03c10c  @ $03C10C  (44 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_CrouchShoot_03c10c, "ax", @progbits
        .global PlayerArm_CrouchShoot_03c10c
PlayerArm_CrouchShoot_03c10c:
        bclr    #0x2,0x8c(a6)                   | +000
        jsr     0x2abcc.l                       | +006
        bcs.w   PlayerArm_SpriteTbl_03c138__L03c164 | +00c
        bclr    #0x2,0x8c(a6)                   | +010
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +016
        lea     PlayerArm_SpriteTbl_03c138(pc),a0 | +01a
        movea.l (a0,d0.w),a0                    | +01e
        jsr     0x28cd4.l                       | +022
        bra.w   PlayerArm_SpriteTbl_03c138__L03c160 | +028

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03c138  @ $03C138  (72 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03c138, "ax", @progbits
        .global PlayerArm_SpriteTbl_03c138
PlayerArm_SpriteTbl_03c138:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xb146                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xe76e                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xe76e                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xe76e                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xe76e                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xb146                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xe76e                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xe76e                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xe76e                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xe76e                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03c138__L03c160
PlayerArm_SpriteTbl_03c138__L03c160:
        bra.w   PlayerArm_SpriteTbl_03c180__L03c1a8 | +028
        .global PlayerArm_SpriteTbl_03c138__L03c164
PlayerArm_SpriteTbl_03c138__L03c164:
        bclr    #0x2,0x8c(a6)                   | +02c
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +032
        lea     PlayerArm_SpriteTbl_03c180(pc),a0 | +036
        movea.l (a0,d0.w),a0                    | +03a
        jsr     0x28cd4.l                       | +03e
        bra.w   PlayerArm_SpriteTbl_03c180__L03c1a8 | +044

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03c180  @ $03C180  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03c180, "ax", @progbits
        .global PlayerArm_SpriteTbl_03c180
PlayerArm_SpriteTbl_03c180:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xb736                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xb736                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03c180__L03c1a8
PlayerArm_SpriteTbl_03c180__L03c1a8:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_CrawlA_03c1aa  @ $03C1AA  (34 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_CrawlA_03c1aa, "ax", @progbits
        .global PlayerArm_CrawlA_03c1aa
PlayerArm_CrawlA_03c1aa:
        bclr    #0x2,0x8c(a6)                   | +000
        bclr    #0x2,0x8c(a6)                   | +006
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +00c
        lea     PlayerArm_SpriteTbl_03c1cc(pc),a0 | +010
        movea.l (a0,d0.w),a0                    | +014
        jsr     0x28cd4.l                       | +018
        bra.w   PlayerArm_SpriteTbl_03c1cc__L03c1f4 | +01e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03c1cc  @ $03C1CC  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03c1cc, "ax", @progbits
        .global PlayerArm_SpriteTbl_03c1cc
PlayerArm_SpriteTbl_03c1cc:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xb736                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xb736                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03c1cc__L03c1f4
PlayerArm_SpriteTbl_03c1cc__L03c1f4:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_CrawlUnref_03c1f6  @ $03C1F6  (34 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_CrawlUnref_03c1f6, "ax", @progbits
        .global PlayerArm_CrawlUnref_03c1f6
PlayerArm_CrawlUnref_03c1f6:
        bclr    #0x2,0x8c(a6)                   | +000
        bclr    #0x2,0x8c(a6)                   | +006
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +00c
        lea     PlayerArm_SpriteTbl_03c218(pc),a0 | +010
        movea.l (a0,d0.w),a0                    | +014
        jsr     0x28cd4.l                       | +018
        bra.w   PlayerArm_SpriteTbl_03c218__L03c240 | +01e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03c218  @ $03C218  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03c218, "ax", @progbits
        .global PlayerArm_SpriteTbl_03c218
PlayerArm_SpriteTbl_03c218:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xb736                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xb736                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xed5e                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03c218__L03c240
PlayerArm_SpriteTbl_03c218__L03c240:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_CrawlB_03c242  @ $03C242  (34 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_CrawlB_03c242, "ax", @progbits
        .global PlayerArm_CrawlB_03c242
PlayerArm_CrawlB_03c242:
        bclr    #0x2,0x8c(a6)                   | +000
        bclr    #0x2,0x8c(a6)                   | +006
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +00c
        lea     PlayerArm_SpriteTbl_03c264(pc),a0 | +010
        movea.l (a0,d0.w),a0                    | +014
        jsr     0x28cd4.l                       | +018
        bra.w   PlayerArm_SpriteTbl_03c264__L03c28c | +01e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03c264  @ $03C264  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03c264, "ax", @progbits
        .global PlayerArm_SpriteTbl_03c264
PlayerArm_SpriteTbl_03c264:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xb704                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xed2c                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xed2c                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xed2c                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xed2c                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xb704                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xed2c                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xed2c                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xed2c                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xed2c                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03c264__L03c28c
PlayerArm_SpriteTbl_03c264__L03c28c:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_CrouchGrenade_03c28e  @ $03C28E  (28 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_CrouchGrenade_03c28e, "ax", @progbits
        .global PlayerArm_CrouchGrenade_03c28e
PlayerArm_CrouchGrenade_03c28e:
        bclr    #0x2,0x8c(a6)                   | +000
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +006
        lea     PlayerArm_SpriteTbl_03c2aa(pc),a0 | +00a
        movea.l (a0,d0.w),a0                    | +00e
        jsr     0x28cd4.l                       | +012
        bra.w   PlayerArm_SpriteTbl_03c2aa__L03c2d2 | +018

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03c2aa  @ $03C2AA  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03c2aa, "ax", @progbits
        .global PlayerArm_SpriteTbl_03c2aa
PlayerArm_SpriteTbl_03c2aa:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xba34                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x08ec                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x14d2                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x1e1e                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xf05c                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xba34                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x08ec                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x14d2                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x1e1e                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xf05c                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03c2aa__L03c2d2
PlayerArm_SpriteTbl_03c2aa__L03c2d2:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_MeleeC_03c2d4  @ $03C2D4  (40 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_MeleeC_03c2d4, "ax", @progbits
        .global PlayerArm_MeleeC_03c2d4
PlayerArm_MeleeC_03c2d4:
        movea.l 0xc(a6),a0                      | +000
        move.b  0x80(a0),d0                     | +004
        bne.w   PlayerArm_SpriteTbl_03c2fc__L03c328 | +008
        bclr    #0x2,0x8c(a6)                   | +00c
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +012
        lea     PlayerArm_SpriteTbl_03c2fc(pc),a0 | +016
        movea.l (a0,d0.w),a0                    | +01a
        jsr     0x28cd4.l                       | +01e
        bra.w   PlayerArm_SpriteTbl_03c2fc__L03c324 | +024

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03c2fc  @ $03C2FC  (72 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03c2fc, "ax", @progbits
        .global PlayerArm_SpriteTbl_03c2fc
PlayerArm_SpriteTbl_03c2fc:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc230                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xf872                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xf872                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xf872                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xf872                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc230                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xf872                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xf872                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xf872                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xf872                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03c2fc__L03c324
PlayerArm_SpriteTbl_03c2fc__L03c324:
        bra.w   PlayerArm_SpriteTbl_03c344__L03c36c | +028
        .global PlayerArm_SpriteTbl_03c2fc__L03c328
PlayerArm_SpriteTbl_03c2fc__L03c328:
        bclr    #0x2,0x8c(a6)                   | +02c
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +032
        lea     PlayerArm_SpriteTbl_03c344(pc),a0 | +036
        movea.l (a0,d0.w),a0                    | +03a
        jsr     0x28cd4.l                       | +03e
        bra.w   PlayerArm_SpriteTbl_03c344__L03c36c | +044

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03c344  @ $03C344  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03c344, "ax", @progbits
        .global PlayerArm_SpriteTbl_03c344
PlayerArm_SpriteTbl_03c344:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc19c                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xf7de                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xf7de                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xf7de                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xf7de                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc19c                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xf7de                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xf7de                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xf7de                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xf7de                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03c344__L03c36c
PlayerArm_SpriteTbl_03c344__L03c36c:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_AirMeleeA_03c36e  @ $03C36E  (38 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_AirMeleeA_03c36e, "ax", @progbits
        .global PlayerArm_AirMeleeA_03c36e
PlayerArm_AirMeleeA_03c36e:
        cmpi.w  #0x1,0x72(a6)                   | +000
        bne.w   PlayerArm_SpriteTbl_03c394__L03c3c0 | +006
        bclr    #0x2,0x8c(a6)                   | +00a
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +010
        lea     PlayerArm_SpriteTbl_03c394(pc),a0 | +014
        movea.l (a0,d0.w),a0                    | +018
        jsr     0x28cd4.l                       | +01c
        bra.w   PlayerArm_SpriteTbl_03c394__L03c3bc | +022

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03c394  @ $03C394  (82 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03c394, "ax", @progbits
        .global PlayerArm_SpriteTbl_03c394
PlayerArm_SpriteTbl_03c394:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xcce0                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x1420                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1d6c                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x2802                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x082a                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xcce0                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x1420                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1d6c                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x2802                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x082a                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03c394__L03c3bc
PlayerArm_SpriteTbl_03c394__L03c3bc:
        bra.w   PlayerArm_SpriteTbl_03c42e__L03c456 | +028
        .global PlayerArm_SpriteTbl_03c394__L03c3c0
PlayerArm_SpriteTbl_03c394__L03c3c0:
        cmpi.w  #0x2,0x72(a6)                   | +02c
        bne.w   PlayerArm_SpriteTbl_03c3e6__L03c412 | +032
        bclr    #0x2,0x8c(a6)                   | +036
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +03c
        lea     PlayerArm_SpriteTbl_03c3e6(pc),a0 | +040
        movea.l (a0,d0.w),a0                    | +044
        jsr     0x28cd4.l                       | +048
        bra.w   PlayerArm_SpriteTbl_03c3e6__L03c40e | +04e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03c3e6  @ $03C3E6  (72 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03c3e6, "ax", @progbits
        .global PlayerArm_SpriteTbl_03c3e6
PlayerArm_SpriteTbl_03c3e6:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xcd5c                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x149c                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1de8                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x287e                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x08ae                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xcd5c                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x149c                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1de8                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x287e                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x08ae                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03c3e6__L03c40e
PlayerArm_SpriteTbl_03c3e6__L03c40e:
        bra.w   PlayerArm_SpriteTbl_03c42e__L03c456 | +028
        .global PlayerArm_SpriteTbl_03c3e6__L03c412
PlayerArm_SpriteTbl_03c3e6__L03c412:
        bclr    #0x2,0x8c(a6)                   | +02c
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +032
        lea     PlayerArm_SpriteTbl_03c42e(pc),a0 | +036
        movea.l (a0,d0.w),a0                    | +03a
        jsr     0x28cd4.l                       | +03e
        bra.w   PlayerArm_SpriteTbl_03c42e__L03c456 | +044

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03c42e  @ $03C42E  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03c42e, "ax", @progbits
        .global PlayerArm_SpriteTbl_03c42e
PlayerArm_SpriteTbl_03c42e:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc358                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xf99a                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xf99a                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xf99a                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xf99a                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc358                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xf99a                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xf99a                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xf99a                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xf99a                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03c42e__L03c456
PlayerArm_SpriteTbl_03c42e__L03c456:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_AirMeleeB_03c458  @ $03C458  (38 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_AirMeleeB_03c458, "ax", @progbits
        .global PlayerArm_AirMeleeB_03c458
PlayerArm_AirMeleeB_03c458:
        cmpi.w  #0x1,0x72(a6)                   | +000
        bne.w   PlayerArm_SpriteTbl_03c47e__L03c4aa | +006
        bclr    #0x2,0x8c(a6)                   | +00a
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +010
        lea     PlayerArm_SpriteTbl_03c47e(pc),a0 | +014
        movea.l (a0,d0.w),a0                    | +018
        jsr     0x28cd4.l                       | +01c
        bra.w   PlayerArm_SpriteTbl_03c47e__L03c4a6 | +022

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03c47e  @ $03C47E  (82 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03c47e, "ax", @progbits
        .global PlayerArm_SpriteTbl_03c47e
PlayerArm_SpriteTbl_03c47e:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xccaa                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x13ea                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1d36                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x27cc                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x07f4                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xccaa                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x13ea                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1d36                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x27cc                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x07f4                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03c47e__L03c4a6
PlayerArm_SpriteTbl_03c47e__L03c4a6:
        bra.w   PlayerArm_SpriteTbl_03c518__L03c540 | +028
        .global PlayerArm_SpriteTbl_03c47e__L03c4aa
PlayerArm_SpriteTbl_03c47e__L03c4aa:
        cmpi.w  #0x0,0x72(a6)                   | +02c
        bne.w   PlayerArm_SpriteTbl_03c4d0__L03c4fc | +032
        bclr    #0x2,0x8c(a6)                   | +036
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +03c
        lea     PlayerArm_SpriteTbl_03c4d0(pc),a0 | +040
        movea.l (a0,d0.w),a0                    | +044
        jsr     0x28cd4.l                       | +048
        bra.w   PlayerArm_SpriteTbl_03c4d0__L03c4f8 | +04e

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03c4d0  @ $03C4D0  (72 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03c4d0, "ax", @progbits
        .global PlayerArm_SpriteTbl_03c4d0
PlayerArm_SpriteTbl_03c4d0:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xcd16                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +004  (dato / opcode no decodificado)
        .dc.w   0x1456                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1da2                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x2838                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0860                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xcd16                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +018  (dato / opcode no decodificado)
        .dc.w   0x1456                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1da2                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +020  (dato / opcode no decodificado)
        .dc.w   0x2838                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0860                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03c4d0__L03c4f8
PlayerArm_SpriteTbl_03c4d0__L03c4f8:
        bra.w   PlayerArm_SpriteTbl_03c518__L03c540 | +028
        .global PlayerArm_SpriteTbl_03c4d0__L03c4fc
PlayerArm_SpriteTbl_03c4d0__L03c4fc:
        bclr    #0x2,0x8c(a6)                   | +02c
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +032
        lea     PlayerArm_SpriteTbl_03c518(pc),a0 | +036
        movea.l (a0,d0.w),a0                    | +03a
        jsr     0x28cd4.l                       | +03e
        bra.w   PlayerArm_SpriteTbl_03c518__L03c540 | +044

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03c518  @ $03C518  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03c518, "ax", @progbits
        .global PlayerArm_SpriteTbl_03c518
PlayerArm_SpriteTbl_03c518:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc35e                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xf9a0                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xf9a0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xf9a0                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xf9a0                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc35e                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xf9a0                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xf9a0                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xf9a0                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xf9a0                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03c518__L03c540
PlayerArm_SpriteTbl_03c518__L03c540:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_CrouchMelee_03c542  @ $03C542  (28 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_CrouchMelee_03c542, "ax", @progbits
        .global PlayerArm_CrouchMelee_03c542
PlayerArm_CrouchMelee_03c542:
        bclr    #0x2,0x8c(a6)                   | +000
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +006
        lea     PlayerArm_SpriteTbl_03c55e(pc),a0 | +00a
        movea.l (a0,d0.w),a0                    | +00e
        jsr     0x28cd4.l                       | +012
        bra.w   PlayerArm_SpriteTbl_03c55e__L03c586 | +018

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03c55e  @ $03C55E  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03c55e, "ax", @progbits
        .global PlayerArm_SpriteTbl_03c55e
PlayerArm_SpriteTbl_03c55e:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc2c4                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xf906                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xf906                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xf906                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xf906                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc2c4                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xf906                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xf906                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xf906                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xf906                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03c55e__L03c586
PlayerArm_SpriteTbl_03c55e__L03c586:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  PlayerArm_CrouchReload_03c588  @ $03C588  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_CrouchReload_03c588, "ax", @progbits
        .global PlayerArm_CrouchReload_03c588
PlayerArm_CrouchReload_03c588:
        addq.b  #0x1,0x30(a6)                   | +000
        btst    #0x0,0x30(a6)                   | +004
        bne.w   PlayerArm_SpriteTbl_03c5b2__L03c5de | +00a
        bclr    #0x2,0x8c(a6)                   | +00e
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +014
        lea     PlayerArm_SpriteTbl_03c5b2(pc),a0 | +018
        movea.l (a0,d0.w),a0                    | +01c
        jsr     0x28cd4.l                       | +020
        bra.w   PlayerArm_SpriteTbl_03c5b2__L03c5da | +026

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03c5b2  @ $03C5B2  (72 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03c5b2, "ax", @progbits
        .global PlayerArm_SpriteTbl_03c5b2
PlayerArm_SpriteTbl_03c5b2:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc5b4                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xfbf6                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfbf6                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xfbf6                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xfbf6                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc5b4                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xfbf6                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xfbf6                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xfbf6                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xfbf6                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03c5b2__L03c5da
PlayerArm_SpriteTbl_03c5b2__L03c5da:
        bra.w   PlayerArm_SpriteTbl_03c5fa__L03c622 | +028
        .global PlayerArm_SpriteTbl_03c5b2__L03c5de
PlayerArm_SpriteTbl_03c5b2__L03c5de:
        bclr    #0x2,0x8c(a6)                   | +02c
        jsr     PlayerArm_WeaponTableIndex_03933a(pc) | +032
        lea     PlayerArm_SpriteTbl_03c5fa(pc),a0 | +036
        movea.l (a0,d0.w),a0                    | +03a
        jsr     0x28cd4.l                       | +03e
        bra.w   PlayerArm_SpriteTbl_03c5fa__L03c622 | +044

| ----------------------------------------------------------------------------
|  PlayerArm_SpriteTbl_03c5fa  @ $03C5FA  (48 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerArm_SpriteTbl_03c5fa, "ax", @progbits
        .global PlayerArm_SpriteTbl_03c5fa
PlayerArm_SpriteTbl_03c5fa:
        .dc.w   0x0027                        | +000  (dato / opcode no decodificado)
        .dc.w   0xc6cc                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +004  (dato / opcode no decodificado)
        .dc.w   0xfd0e                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfd0e                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xfd0e                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +010  (dato / opcode no decodificado)
        .dc.w   0xfd0e                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +014  (dato / opcode no decodificado)
        .dc.w   0xc6cc                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +018  (dato / opcode no decodificado)
        .dc.w   0xfd0e                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xfd0e                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +020  (dato / opcode no decodificado)
        .dc.w   0xfd0e                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0027                        | +024  (dato / opcode no decodificado)
        .dc.w   0xfd0e                        | +026  (dato / opcode no decodificado)
        .global PlayerArm_SpriteTbl_03c5fa__L03c622
PlayerArm_SpriteTbl_03c5fa__L03c622:
        lea     PlayerArm_MeleeAttackTbl_0392a4(pc),a0 | +028
        move.l  a0,0x4c(a6)                     | +02c
