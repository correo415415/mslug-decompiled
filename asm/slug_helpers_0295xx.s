| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave ??? — (borrador)
|  Región: $0295A6..$02AE3E  (5,846 B, 113 entradas, 51 huecos)
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
|  Slug_AngleToSpriteIdx_0295a6  @ $0295A6  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_AngleToSpriteIdx_0295a6, "ax", @progbits
        .global Slug_AngleToSpriteIdx_0295a6
Slug_AngleToSpriteIdx_0295a6:
        lea     0x2b0bee.l,a0                   | +000
        move.b  (a0,d0.w),d0                    | +006
        ext.w   d0                              | +00a
        rts                                     | +00c

| ----------------------------------------------------------------------------
|  Slug_HitboxIdle_0295b4  @ $0295B4  (76 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_HitboxIdle_0295b4, "ax", @progbits
        .global Slug_HitboxIdle_0295b4
Slug_HitboxIdle_0295b4:
        .dc.w   0x8001                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +004  (dato / opcode no decodificado)
        .dc.w   0xffec                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +010  (dato / opcode no decodificado)
        .dc.w   0x3608                        | +012  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x3612                        | +01e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +020  (dato / opcode no decodificado)
        .dc.w   0xffec                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +028  (dato / opcode no decodificado)
        .dc.w   0x361c                        | +02a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +02c  (dato / opcode no decodificado)
        .dc.w   0xffec                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +034  (dato / opcode no decodificado)
        .dc.w   0x3626                        | +036  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +038  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +040  (dato / opcode no decodificado)
        .dc.w   0x3630                        | +042  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +044  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +048  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +04a  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_HitboxIdleB_029600  @ $029600  (76 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_HitboxIdleB_029600, "ax", @progbits
        .global Slug_HitboxIdleB_029600
Slug_HitboxIdleB_029600:
        .dc.w   0x8001                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +004  (dato / opcode no decodificado)
        .dc.w   0xffec                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +010  (dato / opcode no decodificado)
        .dc.w   0x3608                        | +012  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x3612                        | +01e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +020  (dato / opcode no decodificado)
        .dc.w   0xffec                        | +022  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +028  (dato / opcode no decodificado)
        .dc.w   0x361c                        | +02a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +02c  (dato / opcode no decodificado)
        .dc.w   0xffec                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +034  (dato / opcode no decodificado)
        .dc.w   0x3626                        | +036  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +038  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +040  (dato / opcode no decodificado)
        .dc.w   0x3630                        | +042  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +044  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +048  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +04a  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_HitboxDestroyed_02964c  @ $02964C  (324 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_HitboxDestroyed_02964c, "ax", @progbits
        .global Slug_HitboxDestroyed_02964c
Slug_HitboxDestroyed_02964c:
        .dc.w   0x8001                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +004  (dato / opcode no decodificado)
        .dc.w   0xfff6                        | +006  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +010  (dato / opcode no decodificado)
        .dc.w   0x3608                        | +012  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x3612                        | +01e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +020  (dato / opcode no decodificado)
        .dc.w   0xfff6                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +028  (dato / opcode no decodificado)
        .dc.w   0x361c                        | +02a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +02c  (dato / opcode no decodificado)
        .dc.w   0xfff6                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +034  (dato / opcode no decodificado)
        .dc.w   0x3626                        | +036  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +038  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +040  (dato / opcode no decodificado)
        .dc.w   0x3630                        | +042  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +044  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +048  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0304                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0204                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +054  (dato / opcode no decodificado)
        .dc.w   0xffec                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +058  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +060  (dato / opcode no decodificado)
        .dc.w   0x363a                        | +062  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +064  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +066  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +068  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x3644                        | +06e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +070  (dato / opcode no decodificado)
        .dc.w   0xffec                        | +072  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +074  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +078  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +07a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +07c  (dato / opcode no decodificado)
        .dc.w   0xffec                        | +07e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +080  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +082  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +084  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +086  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +088  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +08a  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +090  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +092  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +094  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +096  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +098  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +09a  (dato / opcode no decodificado)
        .dc.w   0x0405                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +09e  (dato / opcode no decodificado)
        .dc.w   0x0204                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0a2  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0a4  (dato / opcode no decodificado)
        .dc.w   0xffec                        | +0a6  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +0a8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0aa  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +0ac  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0ae  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0b0  (dato / opcode no decodificado)
        .dc.w   0x363a                        | +0b2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0b4  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0b6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0b8  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0ba  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0bc  (dato / opcode no decodificado)
        .dc.w   0x3644                        | +0be  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0c0  (dato / opcode no decodificado)
        .dc.w   0xffec                        | +0c2  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +0c4  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0c6  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0c8  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +0ca  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0cc  (dato / opcode no decodificado)
        .dc.w   0xffec                        | +0ce  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0d0  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0d2  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0d4  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +0d6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0d8  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +0da  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +0dc  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0de  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0e0  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +0e2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0e4  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +0e6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0e8  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +0ea  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0ec  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0ee  (dato / opcode no decodificado)
        .dc.w   0x031c                        | +0f0  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +0f2  (dato / opcode no decodificado)
        .dc.w   0x0404                        | +0f4  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0f6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0f8  (dato / opcode no decodificado)
        .dc.w   0xffc0                        | +0fa  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +0fc  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +0fe  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +100  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +102  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +104  (dato / opcode no decodificado)
        .dc.w   0x363a                        | +106  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +108  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +10a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +10c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +10e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +110  (dato / opcode no decodificado)
        .dc.w   0x3644                        | +112  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +114  (dato / opcode no decodificado)
        .dc.w   0xffc0                        | +116  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +118  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +11a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +11c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +11e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +120  (dato / opcode no decodificado)
        .dc.w   0xffc0                        | +122  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +124  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +126  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +128  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +12a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +12c  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +12e  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +130  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +132  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +134  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +136  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +138  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +13a  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +13c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +13e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +140  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +142  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_HitboxA_029790  @ $029790  (164 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_HitboxA_029790, "ax", @progbits
        .global Slug_HitboxA_029790
Slug_HitboxA_029790:
        .dc.w   0x031c                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0258                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0404                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xffc0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +014  (dato / opcode no decodificado)
        .dc.w   0x363a                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +020  (dato / opcode no decodificado)
        .dc.w   0x3644                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +024  (dato / opcode no decodificado)
        .dc.w   0xffc0                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xffc0                        | +032  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +04a  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x041c                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0258                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0404                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +058  (dato / opcode no decodificado)
        .dc.w   0xffc0                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +05c  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +060  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +062  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +064  (dato / opcode no decodificado)
        .dc.w   0x363a                        | +066  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +068  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +070  (dato / opcode no decodificado)
        .dc.w   0x3644                        | +072  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +074  (dato / opcode no decodificado)
        .dc.w   0xffc0                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +07e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +080  (dato / opcode no decodificado)
        .dc.w   0xffc0                        | +082  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +084  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +088  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +08a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +090  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +094  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +096  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +098  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +09a  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +09e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a2  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_HitboxB_029834  @ $029834  (164 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_HitboxB_029834, "ax", @progbits
        .global Slug_HitboxB_029834
Slug_HitboxB_029834:
        .dc.w   0x031e                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0404                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xffd0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +014  (dato / opcode no decodificado)
        .dc.w   0x363a                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +020  (dato / opcode no decodificado)
        .dc.w   0x3644                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +024  (dato / opcode no decodificado)
        .dc.w   0xffd0                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xffd0                        | +032  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +04a  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x041e                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0404                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +058  (dato / opcode no decodificado)
        .dc.w   0xffc0                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +05c  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +060  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +062  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +064  (dato / opcode no decodificado)
        .dc.w   0x363a                        | +066  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +068  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +070  (dato / opcode no decodificado)
        .dc.w   0x3644                        | +072  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +074  (dato / opcode no decodificado)
        .dc.w   0xffc0                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +07e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +080  (dato / opcode no decodificado)
        .dc.w   0xffc0                        | +082  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +084  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +088  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +08a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +090  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +094  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +096  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +098  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +09a  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +09e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a2  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_HitboxC_0298d8  @ $0298D8  (164 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_HitboxC_0298d8, "ax", @progbits
        .global Slug_HitboxC_0298d8
Slug_HitboxC_0298d8:
        .dc.w   0x031c                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0404                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xff50                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x00b0                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xff70                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0090                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +014  (dato / opcode no decodificado)
        .dc.w   0x363a                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +020  (dato / opcode no decodificado)
        .dc.w   0x3644                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +024  (dato / opcode no decodificado)
        .dc.w   0xff50                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0090                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xff50                        | +032  (dato / opcode no decodificado)
        .dc.w   0xff70                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x00b0                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0090                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x00b0                        | +04a  (dato / opcode no decodificado)
        .dc.w   0xff70                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x041c                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0404                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +058  (dato / opcode no decodificado)
        .dc.w   0xff50                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x00b0                        | +05c  (dato / opcode no decodificado)
        .dc.w   0xff70                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0090                        | +060  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +062  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +064  (dato / opcode no decodificado)
        .dc.w   0x363a                        | +066  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +068  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +070  (dato / opcode no decodificado)
        .dc.w   0x3644                        | +072  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +074  (dato / opcode no decodificado)
        .dc.w   0xff50                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0090                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +07e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +080  (dato / opcode no decodificado)
        .dc.w   0xff50                        | +082  (dato / opcode no decodificado)
        .dc.w   0xff70                        | +084  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +088  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +08a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x00b0                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x0090                        | +090  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +094  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +096  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +098  (dato / opcode no decodificado)
        .dc.w   0x00b0                        | +09a  (dato / opcode no decodificado)
        .dc.w   0xff70                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +09e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a2  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_HitboxD_02997c  @ $02997C  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_HitboxD_02997c, "ax", @progbits
        .global Slug_HitboxD_02997c
Slug_HitboxD_02997c:
        .dc.w   0x0101                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +020  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_HitboxCb_02999e  @ $02999E  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_HitboxCb_02999e, "ax", @progbits
        .global Slug_HitboxCb_02999e
Slug_HitboxCb_02999e:
        .dc.w   0x0004                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +002  (dato / opcode no decodificado)
        .dc.w   0x997c                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfff4                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +014  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +020  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +024  (dato / opcode no decodificado)
        .dc.w   0xfff4                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xfff4                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_HitboxCbB_0299f2  @ $0299F2  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_HitboxCbB_0299f2, "ax", @progbits
        .global Slug_HitboxCbB_0299f2
Slug_HitboxCbB_0299f2:
        .dc.w   0x0000                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +020  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_HitboxCbC_029a14  @ $029A14  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_HitboxCbC_029a14, "ax", @progbits
        .global Slug_HitboxCbC_029a14
Slug_HitboxCbC_029a14:
        .dc.w   0x0004                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +002  (dato / opcode no decodificado)
        .dc.w   0x99f2                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfff4                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +014  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +020  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +024  (dato / opcode no decodificado)
        .dc.w   0xfff4                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xfff4                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_AttackTbl00_029a68  @ $029A68  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_AttackTbl00_029a68, "ax", @progbits
        .global Slug_AttackTbl00_029a68
Slug_AttackTbl00_029a68:
        .dc.w   0x0004                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +002  (dato / opcode no decodificado)
        .dc.w   0x99f2                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfff4                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +014  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +020  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +024  (dato / opcode no decodificado)
        .dc.w   0xfff4                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xfff4                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_AttackTbl01_029abc  @ $029ABC  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_AttackTbl01_029abc, "ax", @progbits
        .global Slug_AttackTbl01_029abc
Slug_AttackTbl01_029abc:
        .dc.w   0x0004                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +002  (dato / opcode no decodificado)
        .dc.w   0x99f2                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfff4                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +014  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +020  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +024  (dato / opcode no decodificado)
        .dc.w   0xfff4                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xfff4                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_AttackTbl02_029b10  @ $029B10  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_AttackTbl02_029b10, "ax", @progbits
        .global Slug_AttackTbl02_029b10
Slug_AttackTbl02_029b10:
        .dc.w   0x0004                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +002  (dato / opcode no decodificado)
        .dc.w   0x99f2                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfff4                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +014  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +020  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +024  (dato / opcode no decodificado)
        .dc.w   0xfff4                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xfff4                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_AttackTbl03_029b64  @ $029B64  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_AttackTbl03_029b64, "ax", @progbits
        .global Slug_AttackTbl03_029b64
Slug_AttackTbl03_029b64:
        .dc.w   0x0004                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +002  (dato / opcode no decodificado)
        .dc.w   0x99f2                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xffe8                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +014  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +020  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +024  (dato / opcode no decodificado)
        .dc.w   0xffe8                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xffe8                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_AttackTbl04_029bb8  @ $029BB8  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_AttackTbl04_029bb8, "ax", @progbits
        .global Slug_AttackTbl04_029bb8
Slug_AttackTbl04_029bb8:
        .dc.w   0x0004                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +002  (dato / opcode no decodificado)
        .dc.w   0x99f2                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xffec                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +014  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +020  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +024  (dato / opcode no decodificado)
        .dc.w   0xffec                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xffec                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_AttackTbl05_029c0c  @ $029C0C  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_AttackTbl05_029c0c, "ax", @progbits
        .global Slug_AttackTbl05_029c0c
Slug_AttackTbl05_029c0c:
        .dc.w   0x0004                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +002  (dato / opcode no decodificado)
        .dc.w   0x99f2                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xffe8                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +014  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +020  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +024  (dato / opcode no decodificado)
        .dc.w   0xffe8                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xffe8                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_AttackTbl06_029c60  @ $029C60  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_AttackTbl06_029c60, "ax", @progbits
        .global Slug_AttackTbl06_029c60
Slug_AttackTbl06_029c60:
        .dc.w   0x0004                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +002  (dato / opcode no decodificado)
        .dc.w   0x99f2                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xffec                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0038                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +014  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +020  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +024  (dato / opcode no decodificado)
        .dc.w   0xffec                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0038                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xffec                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0038                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_AttackTbl07_029cb4  @ $029CB4  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_AttackTbl07_029cb4, "ax", @progbits
        .global Slug_AttackTbl07_029cb4
Slug_AttackTbl07_029cb4:
        .dc.w   0x0004                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +002  (dato / opcode no decodificado)
        .dc.w   0x99f2                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfff2                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +014  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +020  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +024  (dato / opcode no decodificado)
        .dc.w   0xfff2                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xfff2                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_AttackTbl08_029d08  @ $029D08  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_AttackTbl08_029d08, "ax", @progbits
        .global Slug_AttackTbl08_029d08
Slug_AttackTbl08_029d08:
        .dc.w   0x0004                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +002  (dato / opcode no decodificado)
        .dc.w   0x99f2                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xffec                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0038                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +014  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +020  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +024  (dato / opcode no decodificado)
        .dc.w   0xffec                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0038                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xffec                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0038                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_AttackTbl09_029d5c  @ $029D5C  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_AttackTbl09_029d5c, "ax", @progbits
        .global Slug_AttackTbl09_029d5c
Slug_AttackTbl09_029d5c:
        .dc.w   0x0004                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +002  (dato / opcode no decodificado)
        .dc.w   0x99f2                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xffdd                        | +00a  (dato / opcode no decodificado)
        .dc.w   0xfffd                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xfffd                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +014  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +020  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +024  (dato / opcode no decodificado)
        .dc.w   0xffdd                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xffdd                        | +032  (dato / opcode no decodificado)
        .dc.w   0xfffd                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0xfffd                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0xfffd                        | +04a  (dato / opcode no decodificado)
        .dc.w   0xfffd                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_AttackTbl0A_029db0  @ $029DB0  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_AttackTbl0A_029db0, "ax", @progbits
        .global Slug_AttackTbl0A_029db0
Slug_AttackTbl0A_029db0:
        .dc.w   0x0004                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +002  (dato / opcode no decodificado)
        .dc.w   0x99f2                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xffe8                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +014  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +020  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +024  (dato / opcode no decodificado)
        .dc.w   0xffe8                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xffe8                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_AttackTbl0B_029e04  @ $029E04  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_AttackTbl0B_029e04, "ax", @progbits
        .global Slug_AttackTbl0B_029e04
Slug_AttackTbl0B_029e04:
        .dc.w   0x0004                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +002  (dato / opcode no decodificado)
        .dc.w   0x99f2                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xffe1                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xfffd                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x001d                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +014  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +020  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +024  (dato / opcode no decodificado)
        .dc.w   0xffe1                        | +026  (dato / opcode no decodificado)
        .dc.w   0x001d                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xffe1                        | +032  (dato / opcode no decodificado)
        .dc.w   0xfffd                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x001d                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +04a  (dato / opcode no decodificado)
        .dc.w   0xfffd                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_AttackTbl0C_029e58  @ $029E58  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_AttackTbl0C_029e58, "ax", @progbits
        .global Slug_AttackTbl0C_029e58
Slug_AttackTbl0C_029e58:
        .dc.w   0x0004                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +002  (dato / opcode no decodificado)
        .dc.w   0x99f2                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xffdd                        | +00a  (dato / opcode no decodificado)
        .dc.w   0xfffb                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +014  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +020  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +024  (dato / opcode no decodificado)
        .dc.w   0xffdd                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xffdd                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0xfffb                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0xfffb                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_AttackTbl0D_029eac  @ $029EAC  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_AttackTbl0D_029eac, "ax", @progbits
        .global Slug_AttackTbl0D_029eac
Slug_AttackTbl0D_029eac:
        .dc.w   0x0004                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +002  (dato / opcode no decodificado)
        .dc.w   0x99f2                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfffd                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x001d                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +014  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +020  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +024  (dato / opcode no decodificado)
        .dc.w   0xfffd                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xfffd                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x001d                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x001d                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_AttackTbl0E_029f00  @ $029F00  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_AttackTbl0E_029f00, "ax", @progbits
        .global Slug_AttackTbl0E_029f00
Slug_AttackTbl0E_029f00:
        .dc.w   0x0004                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +002  (dato / opcode no decodificado)
        .dc.w   0x99f2                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +014  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +020  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_AttackTbl0F_029f54  @ $029F54  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_AttackTbl0F_029f54, "ax", @progbits
        .global Slug_AttackTbl0F_029f54
Slug_AttackTbl0F_029f54:
        .dc.w   0x0004                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +002  (dato / opcode no decodificado)
        .dc.w   0x99f2                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfff9                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0019                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +014  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +020  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +024  (dato / opcode no decodificado)
        .dc.w   0xfff9                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xfff9                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0019                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0019                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_AttackTbl10_029fa8  @ $029FA8  (124 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_AttackTbl10_029fa8, "ax", @progbits
        .global Slug_AttackTbl10_029fa8
Slug_AttackTbl10_029fa8:
        .dc.w   0x0004                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +002  (dato / opcode no decodificado)
        .dc.w   0x99f2                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfffd                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0019                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +014  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +020  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +024  (dato / opcode no decodificado)
        .dc.w   0xfffd                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xfffd                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0019                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0019                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +054  (dato / opcode no decodificado)
        .dc.w   0x9a14                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +058  (dato / opcode no decodificado)
        .dc.w   0x9b64                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x9cb4                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +060  (dato / opcode no decodificado)
        .dc.w   0x9d5c                        | +062  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +064  (dato / opcode no decodificado)
        .dc.w   0x9eac                        | +066  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +068  (dato / opcode no decodificado)
        .dc.w   0x9a68                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x9bb8                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +070  (dato / opcode no decodificado)
        .dc.w   0x9a68                        | +072  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +074  (dato / opcode no decodificado)
        .dc.w   0x9db0                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +078  (dato / opcode no decodificado)
        .dc.w   0x9f00                        | +07a  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_AttackPtrTbl_02a024  @ $02A024  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_AttackPtrTbl_02a024, "ax", @progbits
        .global Slug_AttackPtrTbl_02a024
Slug_AttackPtrTbl_02a024:
        .dc.w   0x0002                        | +000  (dato / opcode no decodificado)
        .dc.w   0x9a68                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +004  (dato / opcode no decodificado)
        .dc.w   0x9bb8                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +008  (dato / opcode no decodificado)
        .dc.w   0x9a68                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x9db0                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +010  (dato / opcode no decodificado)
        .dc.w   0x9f00                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +014  (dato / opcode no decodificado)
        .dc.w   0x9abc                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +018  (dato / opcode no decodificado)
        .dc.w   0x9c0c                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x9abc                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +020  (dato / opcode no decodificado)
        .dc.w   0x9e04                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +024  (dato / opcode no decodificado)
        .dc.w   0x9f54                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +028  (dato / opcode no decodificado)
        .dc.w   0x9b10                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x9c60                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +030  (dato / opcode no decodificado)
        .dc.w   0x9d08                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +034  (dato / opcode no decodificado)
        .dc.w   0x9e58                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +038  (dato / opcode no decodificado)
        .dc.w   0x9fa8                        | +03a  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_StateByAnglePtrTbl_02a060  @ $02A060  (152 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_StateByAnglePtrTbl_02a060, "ax", @progbits
        .global Slug_StateByAnglePtrTbl_02a060
Slug_StateByAnglePtrTbl_02a060:
        .dc.w   0xffff                        | +000  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +004  (dato / opcode no decodificado)
        .dc.w   0xd67c                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +008  (dato / opcode no decodificado)
        .dc.w   0xd63e                        | +00a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +010  (dato / opcode no decodificado)
        .dc.w   0xd802                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +014  (dato / opcode no decodificado)
        .dc.w   0xd736                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +018  (dato / opcode no decodificado)
        .dc.w   0xe6c2                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xe6c2                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +020  (dato / opcode no decodificado)
        .dc.w   0xe6c2                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +024  (dato / opcode no decodificado)
        .dc.w   0xe6c2                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +028  (dato / opcode no decodificado)
        .dc.w   0xe6c2                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +02c  (dato / opcode no decodificado)
        .dc.w   0xe7fa                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +030  (dato / opcode no decodificado)
        .dc.w   0xe7fa                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +034  (dato / opcode no decodificado)
        .dc.w   0xe7fa                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +038  (dato / opcode no decodificado)
        .dc.w   0xe7fa                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +03c  (dato / opcode no decodificado)
        .dc.w   0xe7fa                        | +03e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +040  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +044  (dato / opcode no decodificado)
        .dc.w   0xb38c                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0012                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +04a  (dato / opcode no decodificado)
        .dc.w   0xffee                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +052  (dato / opcode no decodificado)
        .dc.w   0xfff4                        | +054  (dato / opcode no decodificado)
        .dc.w   0xfffd                        | +056  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +058  (dato / opcode no decodificado)
        .dc.w   0xfffd                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +060  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +062  (dato / opcode no decodificado)
        .dc.w   0xfff7                        | +064  (dato / opcode no decodificado)
        .dc.w   0xfff7                        | +066  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +068  (dato / opcode no decodificado)
        .dc.w   0xfff7                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +070  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +072  (dato / opcode no decodificado)
        .dc.w   0xfff4                        | +074  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +076  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +07a  (dato / opcode no decodificado)
        .dc.w   0xfff4                        | +07c  (dato / opcode no decodificado)
        .dc.w   0xfffd                        | +07e  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +080  (dato / opcode no decodificado)
        .dc.w   0xfffd                        | +082  (dato / opcode no decodificado)
        .dc.w   0xfff4                        | +084  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +086  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +088  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +08a  (dato / opcode no decodificado)
        .dc.w   0xfff4                        | +08c  (dato / opcode no decodificado)
        .dc.w   0xfff4                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +090  (dato / opcode no decodificado)
        .dc.w   0xfff4                        | +092  (dato / opcode no decodificado)
        .dc.w   0xfff4                        | +094  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +096  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_Init_02a0f8  @ $02A0F8  (178 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_Init_02a0f8, "ax", @progbits
        .global Slug_Init_02a0f8
Slug_Init_02a0f8:
        jsr     0x236e.l                        | +000
        move.w  #0x1,d1                         | +006
        jsr     0x236e.l                        | +00a
        move.w  #0x1e,0x1c(a6)                  | +010
        jsr     0x138fe.l                       | +016
        move.w  0x16(a6),0x14(a6)               | +01c
        bset    #0x4,0x12(a6)                   | +022
        bset    #0x2,0x5b(a6)                   | +028
        bset    #0x0,0x6b(a6)                   | +02e
        move.b  #0xff,0x32(a6)                  | +034
        move.b  #0xff,0x33(a6)                  | +03a
        move.b  #0x0,0x3a(a6)                   | +040
        move.w  #0x8000,0x38(a6)                | +046
        ori.w   #0xc,0x38(a6)                   | +04c
        jsr     0x5e98a.l                       | +052
        clr.b   0x8c(a6)                        | +058
        move.l  #0x295b4,0x60(a6)               | +05c
        jsr     0x8f6d2.l                       | +064
        move.w  #0x30,0x66(a6)                  | +06a
        jsr     Slug_ResetDamageIdx_02fada(pc)  | +070
        clr.w   0x94(a6)                        | +074
        clr.w   0x36(a6)                        | +078
        clr.b   0x92(a6)                        | +07c
        clr.b   0x8f(a6)                        | +080
        move.b  #0xa,0x90(a6)                   | +084
        bclr    #0x3,0x13(a6)                   | +08a
        lea     0x7773e.l,a1                    | +090
        jsr     0x4ae.l                         | +096
        jsr     0x5dd02.l                       | +09c
        jsr     0x3060a.l                       | +0a2
        move.l  #0x10000,0x98(a6)               | +0a8
        rts                                     | +0b0

| ----------------------------------------------------------------------------
|  Slug_InitBoss_02a1aa  @ $02A1AA  (152 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_InitBoss_02a1aa, "ax", @progbits
        .global Slug_InitBoss_02a1aa
Slug_InitBoss_02a1aa:
        jsr     0x236e.l                        | +000
        move.w  #0x1,d1                         | +006
        jsr     0x236e.l                        | +00a
        move.w  #0x3,0x1c(a6)                   | +010
        jsr     0x138fe.l                       | +016
        move.w  0x16(a6),0x14(a6)               | +01c
        bset    #0x4,0x12(a6)                   | +022
        bset    #0x2,0x5b(a6)                   | +028
        bset    #0x0,0x6b(a6)                   | +02e
        move.b  #0xff,0x32(a6)                  | +034
        move.b  #0xff,0x33(a6)                  | +03a
        move.b  #0x0,0x3a(a6)                   | +040
        move.w  #0x8000,0x38(a6)                | +046
        ori.w   #0xc,0x38(a6)                   | +04c
        jsr     0x5e98a.l                       | +052
        move.w  #0x30,0x66(a6)                  | +058
        clr.b   0x8c(a6)                        | +05e
        move.l  #0x295b4,0x60(a6)               | +062
        jsr     0x8f6d2.l                       | +06a
        clr.w   0x94(a6)                        | +070
        clr.w   0x36(a6)                        | +074
        clr.b   0x92(a6)                        | +078
        clr.b   0x8f(a6)                        | +07c
        bclr    #0x3,0x13(a6)                   | +080
        lea     0x7773e.l,a1                    | +086
        jsr     0x4ae.l                         | +08c
        jsr     0x5dd02.l                       | +092

| ----------------------------------------------------------------------------
|  Slug_MarkRidden_02a252  @ $02A252  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_MarkRidden_02a252, "ax", @progbits
        .global Slug_MarkRidden_02a252
Slug_MarkRidden_02a252:
        move.b  #0xff,0x106f4b.l                | +000
        rts                                     | +008

| ----------------------------------------------------------------------------
|  Slug_IsIdleFlagClear_02a25c  @ $02A25C  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_IsIdleFlagClear_02a25c, "ax", @progbits
        .global Slug_IsIdleFlagClear_02a25c
Slug_IsIdleFlagClear_02a25c:
        cmpi.b  #0x0,0x106ed3.l                 | +000
        bne.w   ClearXN_02a270                  | +008
        ori.b   #0x11,ccr                       | +00c
        bra.w   ClearXNMid_02a274               | +010

| ----------------------------------------------------------------------------
|  Slug_TestBit5Field8D_02a276  @ $02A276  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_TestBit5Field8D_02a276, "ax", @progbits
        .global Slug_TestBit5Field8D_02a276
Slug_TestBit5Field8D_02a276:
        btst    #0x5,0x8d(a6)                   | +000
        beq.w   SetXN_02a288                    | +006
        andi.b  #0xee,ccr                       | +00a
        bra.w   SetXNMid_02a28c                 | +00e

| ----------------------------------------------------------------------------
|  Slug_AddGaugeFromField98_02a28e  @ $02A28E  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_AddGaugeFromField98_02a28e, "ax", @progbits
        .global Slug_AddGaugeFromField98_02a28e
Slug_AddGaugeFromField98_02a28e:
        lea     0x100580.l,a1                   | +000
        cmpi.b  #0xff,0x98(a6)                  | +006
        beq.w   .L02a2aa                        | +00c
        move.b  0x90(a1),d0                     | +010
        add.b   0x98(a6),d0                     | +014
        move.b  d0,0x90(a1)                     | +018
.L02a2aa:
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  Slug_GetGauge_02a2ac  @ $02A2AC  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_GetGauge_02a2ac, "ax", @progbits
        .global Slug_GetGauge_02a2ac
Slug_GetGauge_02a2ac:
        lea     0x100580.l,a0                   | +000
        moveq   #0,d0                           | +006
        move.b  0x90(a0),d0                     | +008
        rts                                     | +00c

| ----------------------------------------------------------------------------
|  Slug_AddHP_02a2ba  @ $02A2BA  (56 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_AddHP_02a2ba, "ax", @progbits
        .global Slug_AddHP_02a2ba
Slug_AddHP_02a2ba:
        lea     0x100580.l,a0                   | +000
        cmpi.w  #0x30,0x66(a0)                  | +006
        bge.w   SetXN_02a2f2                    | +00c
        move.w  0x66(a0),d1                     | +010
        add.w   d0,d1                           | +014
        cmpi.w  #0x30,d1                        | +016
        ble.w   .L02a2dc                        | +01a
        move.w  #0x30,d1                        | +01e
.L02a2dc:
        move.w  d1,0x66(a0)                     | +022
        move.w  #0x1083,d0                      | +026
        jsr     0x2352.l                        | +02a
        andi.b  #0xee,ccr                       | +030
        bra.w   SetXNMid_02a2f6                 | +034

| ----------------------------------------------------------------------------
|  Slug_StopMusic108F_02a2f8  @ $02A2F8  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_StopMusic108F_02a2f8, "ax", @progbits
        .global Slug_StopMusic108F_02a2f8
Slug_StopMusic108F_02a2f8:
        tst.b   0x8b(a6)                        | +000
        beq.w   .L02a30e                        | +004
        move.w  #0x108f,d0                      | +008
        jsr     0x2352.l                        | +00c
        clr.b   0x8b(a6)                        | +012
.L02a30e:
        rts                                     | +016

| ----------------------------------------------------------------------------
|  Slug_StopMusic1092_02a310  @ $02A310  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_StopMusic1092_02a310, "ax", @progbits
        .global Slug_StopMusic1092_02a310
Slug_StopMusic1092_02a310:
        tst.b   0x8b(a6)                        | +000
        beq.w   .L02a326                        | +004
        move.w  #0x1092,d0                      | +008
        jsr     0x2352.l                        | +00c
        clr.b   0x8b(a6)                        | +012
.L02a326:
        rts                                     | +016

| ----------------------------------------------------------------------------
|  Slug_CallGroundProbeA_02a328  @ $02A328  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_CallGroundProbeA_02a328, "ax", @progbits
        .global Slug_CallGroundProbeA_02a328
Slug_CallGroundProbeA_02a328:
        move.w  0x94(a6),d0                     | +000
        asl.w   #0x2,d0                         | +004
        lea     Slug_GroundProbeTblA_02a33a(pc),a0 | +006
        movea.l (a0,d0.w),a0                    | +00a
        jsr     (a0)                            | +00e
        rts                                     | +010

| ----------------------------------------------------------------------------
|  Slug_GroundProbeTblA_02a33a  @ $02A33A  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_GroundProbeTblA_02a33a, "ax", @progbits
        .global Slug_GroundProbeTblA_02a33a
Slug_GroundProbeTblA_02a33a:
        .dc.w   0x0005                        | +000  (dato / opcode no decodificado)
        .dc.w   0xcf04                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +004  (dato / opcode no decodificado)
        .dc.w   0xcf04                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +008  (dato / opcode no decodificado)
        .dc.w   0xcf04                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xcf04                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +010  (dato / opcode no decodificado)
        .dc.w   0xcf04                        | +012  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_CallGroundProbeB_02a34e  @ $02A34E  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_CallGroundProbeB_02a34e, "ax", @progbits
        .global Slug_CallGroundProbeB_02a34e
Slug_CallGroundProbeB_02a34e:
        move.w  0x94(a6),d0                     | +000
        asl.w   #0x2,d0                         | +004
        lea     Slug_GroundProbeTblB_02a360(pc),a0 | +006
        movea.l (a0,d0.w),a0                    | +00a
        jsr     (a0)                            | +00e
        rts                                     | +010

| ----------------------------------------------------------------------------
|  Slug_GroundProbeTblB_02a360  @ $02A360  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_GroundProbeTblB_02a360, "ax", @progbits
        .global Slug_GroundProbeTblB_02a360
Slug_GroundProbeTblB_02a360:
        .dc.w   0x0005                        | +000  (dato / opcode no decodificado)
        .dc.w   0xcf10                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +004  (dato / opcode no decodificado)
        .dc.w   0xcf10                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +008  (dato / opcode no decodificado)
        .dc.w   0xcf10                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xcf10                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +010  (dato / opcode no decodificado)
        .dc.w   0xcf10                        | +012  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_CallGroundProbeC_02a374  @ $02A374  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_CallGroundProbeC_02a374, "ax", @progbits
        .global Slug_CallGroundProbeC_02a374
Slug_CallGroundProbeC_02a374:
        move.w  0x94(a6),d0                     | +000
        asl.w   #0x2,d0                         | +004
        lea     Slug_GroundProbeTblC_02a386(pc),a0 | +006
        movea.l (a0,d0.w),a0                    | +00a
        jsr     (a0)                            | +00e
        rts                                     | +010

| ----------------------------------------------------------------------------
|  Slug_GroundProbeTblC_02a386  @ $02A386  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_GroundProbeTblC_02a386, "ax", @progbits
        .global Slug_GroundProbeTblC_02a386
Slug_GroundProbeTblC_02a386:
        .dc.w   0x0002                        | +000  (dato / opcode no decodificado)
        .dc.w   0xa39a                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +004  (dato / opcode no decodificado)
        .dc.w   0xa39a                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +008  (dato / opcode no decodificado)
        .dc.w   0xa39a                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xa39a                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +010  (dato / opcode no decodificado)
        .dc.w   0xa39a                        | +012  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_ProbeFrontThenGround_02a39a  @ $02A39A  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_ProbeFrontThenGround_02a39a, "ax", @progbits
        .global Slug_ProbeFrontThenGround_02a39a
Slug_ProbeFrontThenGround_02a39a:
        jsr     0x5cf84.l                       | +000
        bcc.w   .L02a3a6                        | +006
        rts                                     | +00a
.L02a3a6:
        jsr     0x5ceec.l                       | +00c
        bcc.w   .L02a3b6                        | +012
        jmp     0x5cf9c.l                       | +016
.L02a3b6:
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  Slug_ProbeFrontThenA_02a3b8  @ $02A3B8  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_ProbeFrontThenA_02a3b8, "ax", @progbits
        .global Slug_ProbeFrontThenA_02a3b8
Slug_ProbeFrontThenA_02a3b8:
        jsr     0x5cf84.l                       | +000
        bcc.w   ClearXN_02a3d0                  | +006
        jsr     Slug_CallGroundProbeA_02a328(pc) | +00a
        bcs.w   ClearXN_02a3d0                  | +00e

| ----------------------------------------------------------------------------
|  Slug_ProbeFrontThenB_02a3d6  @ $02A3D6  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_ProbeFrontThenB_02a3d6, "ax", @progbits
        .global Slug_ProbeFrontThenB_02a3d6
Slug_ProbeFrontThenB_02a3d6:
        jsr     0x5cf84.l                       | +000
        bcc.w   ClearXN_02a3ee                  | +006
        jsr     Slug_CallGroundProbeB_02a34e(pc) | +00a
        bcs.w   ClearXN_02a3ee                  | +00e

| ----------------------------------------------------------------------------
|  Slug_ProbeLeftWall_02a3f4  @ $02A3F4  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_ProbeLeftWall_02a3f4, "ax", @progbits
        .global Slug_ProbeLeftWall_02a3f4
Slug_ProbeLeftWall_02a3f4:
        jsr     0x5cf2c.l                       | +000
        bcc.w   ClearXN_02a418                  | +006
        jsr     0x5cf5c.l                       | +00a
        bcs.w   ClearXN_02a418                  | +010
        jsr     0x5d140.l                       | +014
        bcs.w   ClearXN_02a418                  | +01a

| ----------------------------------------------------------------------------
|  Slug_ProbeRightWall_02a41e  @ $02A41E  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_ProbeRightWall_02a41e, "ax", @progbits
        .global Slug_ProbeRightWall_02a41e
Slug_ProbeRightWall_02a41e:
        jsr     0x5cf3c.l                       | +000
        bcc.b   ClearXN_02a418                  | +006
        jsr     0x5cf4c.l                       | +008
        bcs.b   ClearXN_02a418                  | +00e
        jsr     0x5d14c.l                       | +010
        bcs.b   ClearXN_02a418                  | +016
        bra.b   SetXN_02a412                    | +018

| ----------------------------------------------------------------------------
|  Slug_ProbeRightWallB_02a438  @ $02A438  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_ProbeRightWallB_02a438, "ax", @progbits
        .global Slug_ProbeRightWallB_02a438
Slug_ProbeRightWallB_02a438:
        jsr     0x5cf4c.l                       | +000
        bcc.b   ClearXN_02a418                  | +006
        jsr     0x5cf3c.l                       | +008
        bcs.b   ClearXN_02a418                  | +00e
        jsr     0x5d140.l                       | +010
        bcs.b   ClearXN_02a418                  | +016
        bra.b   SetXN_02a412                    | +018

| ----------------------------------------------------------------------------
|  Slug_ProbeLeftWallB_02a452  @ $02A452  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_ProbeLeftWallB_02a452, "ax", @progbits
        .global Slug_ProbeLeftWallB_02a452
Slug_ProbeLeftWallB_02a452:
        jsr     0x5cf5c.l                       | +000
        bcc.b   ClearXN_02a418                  | +006
        jsr     0x5cf2c.l                       | +008
        bcs.b   ClearXN_02a418                  | +00e
        jsr     0x5d14c.l                       | +010
        bcs.b   ClearXN_02a418                  | +016
        bra.b   SetXN_02a412                    | +018

| ----------------------------------------------------------------------------
|  Slug_ClearFlags8D_02a46c  @ $02A46C  (12 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_ClearFlags8D_02a46c, "ax", @progbits
        .global Slug_ClearFlags8D_02a46c
Slug_ClearFlags8D_02a46c:
        lea     0x100580.l,a0                   | +000
        clr.b   0x8d(a0)                        | +006
        rts                                     | +00a

| ----------------------------------------------------------------------------
|  Slug_UpdateAirFlag_02a478  @ $02A478  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_UpdateAirFlag_02a478, "ax", @progbits
        .global Slug_UpdateAirFlag_02a478
Slug_UpdateAirFlag_02a478:
        jsr     0x5cf1c.l                       | +000
        bcc.w   .L02a48c                        | +006
        bset    #0x4,0x12(a6)                   | +00a
        bra.w   .L02a4a0                        | +010
.L02a48c:
        bclr    #0x4,0x12(a6)                   | +014
        jsr     ClearXN_02abcc(pc)              | +01a
        bcc.w   .L02a4a0                        | +01e
        bset    #0x4,0x12(a6)                   | +022
.L02a4a0:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  Slug_ResetTurnTimer_02a4a2  @ $02A4A2  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_ResetTurnTimer_02a4a2, "ax", @progbits
        .global Slug_ResetTurnTimer_02a4a2
Slug_ResetTurnTimer_02a4a2:
        clr.b   0x84(a6)                        | +000
        rts                                     | +004

| ----------------------------------------------------------------------------
|  Slug_TurnTimerTick_02a4a8  @ $02A4A8  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_TurnTimerTick_02a4a8, "ax", @progbits
        .global Slug_TurnTimerTick_02a4a8
Slug_TurnTimerTick_02a4a8:
        jsr     0x5cd60.l                       | +000
        bcc.w   Slug_TurnTimerTickB_02a4ce      | +006
        cmpi.b  #0x14,0x84(a6)                  | +00a
        bne.w   Slug_TurnTimerDone_02a4c2__L02a4c6 | +010

| ----------------------------------------------------------------------------
|  Slug_TurnTimerDone_02a4c2  @ $02A4C2  (12 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_TurnTimerDone_02a4c2, "ax", @progbits
        .global Slug_TurnTimerDone_02a4c2
Slug_TurnTimerDone_02a4c2:
        bra.w   .L02a4ca                        | +000
        .global Slug_TurnTimerDone_02a4c2__L02a4c6
Slug_TurnTimerDone_02a4c2__L02a4c6:
        addq.b  #0x1,0x84(a6)                   | +004
.L02a4ca:
        bra.w   ClearXN_02a4e6                  | +008

| ----------------------------------------------------------------------------
|  Slug_TurnTimerTickB_02a4ce  @ $02A4CE  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_TurnTimerTickB_02a4ce, "ax", @progbits
        .global Slug_TurnTimerTickB_02a4ce
Slug_TurnTimerTickB_02a4ce:
        cmpi.b  #0x14,0x84(a6)                  | +000
        bne.w   .L02a4e2                        | +006
        move.b  #0xff,0x84(a6)                  | +00a
        bra.w   ClearXN_02a4e6                  | +010
.L02a4e2:
        clr.b   0x84(a6)                        | +014

| ----------------------------------------------------------------------------
|  Slug_TerrainIsSlope_02a4ec  @ $02A4EC  (4 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_TerrainIsSlope_02a4ec, "ax", @progbits
        .global Slug_TerrainIsSlope_02a4ec
Slug_TerrainIsSlope_02a4ec:
        bra.w   Slug_SetAngleIsSlope_02a502__L02a506 | +000

| ----------------------------------------------------------------------------
|  Slug_UpdateAngleIsSlope_02a4f0  @ $02A4F0  (12 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_UpdateAngleIsSlope_02a4f0, "ax", @progbits
        .global Slug_UpdateAngleIsSlope_02a4f0
Slug_UpdateAngleIsSlope_02a4f0:
        jsr     Slug_TerrainSlope_02a958(pc)    | +000
        cmp.w   0x80(a6),d2                     | +004
        bne.w   Slug_SetAngleIsSlope_02a502     | +008

| ----------------------------------------------------------------------------
|  Slug_SetAngleIsSlope_02a502  @ $02A502  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_SetAngleIsSlope_02a502, "ax", @progbits
        .global Slug_SetAngleIsSlope_02a502
Slug_SetAngleIsSlope_02a502:
        move.w  d2,0x80(a6)                     | +000
        .global Slug_SetAngleIsSlope_02a502__L02a506
Slug_SetAngleIsSlope_02a502__L02a506:
        move.w  0x80(a6),d7                     | +004
        lea     Slug_SlopeByAngleTblB_02a558(pc),a0 | +008
        move.b  (a0,d7.w),d0                    | +00c
        cmpi.b  #0x1,d0                         | +010
        rts                                     | +014

| ----------------------------------------------------------------------------
|  Slug_SlopeByAngleTblA_02a518  @ $02A518  (64 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_SlopeByAngleTblA_02a518, "ax", @progbits
        .global Slug_SlopeByAngleTblA_02a518
Slug_SlopeByAngleTblA_02a518:
        .dc.w   0x0000                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +038  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +03e  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_SlopeByAngleTblB_02a558  @ $02A558  (66 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_SlopeByAngleTblB_02a558, "ax", @progbits
        .global Slug_SlopeByAngleTblB_02a558
Slug_SlopeByAngleTblB_02a558:
        .dc.w   0x0100                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +038  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +040  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_CheckFreeThenC_02a59a  @ $02A59A  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_CheckFreeThenC_02a59a, "ax", @progbits
        .global Slug_CheckFreeThenC_02a59a
Slug_CheckFreeThenC_02a59a:
        jsr     JmpAbsThunk_02abc6(pc)          | +000
        bcc.w   ClearXN_02a5a8                  | +004

| ----------------------------------------------------------------------------
|  Slug_CheckFreeTbl_02a5ae  @ $02A5AE  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_CheckFreeTbl_02a5ae, "ax", @progbits
        .global Slug_CheckFreeTbl_02a5ae
Slug_CheckFreeTbl_02a5ae:
        .dc.w   0x0000                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0801                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0800                        | +004  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0f0c                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +00a  (dato / opcode no decodificado)
        .dc.w   0xd5a6                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +00e  (dato / opcode no decodificado)
        .dc.w   0xd5a6                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +012  (dato / opcode no decodificado)
        .dc.w   0xd5a6                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +016  (dato / opcode no decodificado)
        .dc.w   0xd586                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +01a  (dato / opcode no decodificado)
        .dc.w   0xd596                        | +01c  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_MusicByTerrainA_02a5d4  @ $02A5D4  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_MusicByTerrainA_02a5d4, "ax", @progbits
        .global Slug_MusicByTerrainA_02a5d4
Slug_MusicByTerrainA_02a5d4:
        jsr     0x27eba.l                       | +000
        andi.b  #0xc0,d0                        | +006
        cmpi.b  #0x40,d0                        | +00a
        beq.w   Slug_Music1052_02a5fa           | +00e
        cmpi.b  #0x80,d0                        | +012
        beq.w   Slug_Music1052_02a5fa           | +016
        move.w  #0x1090,d0                      | +01a

| ----------------------------------------------------------------------------
|  Slug_Music1052_02a5fa  @ $02A5FA  (4 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_Music1052_02a5fa, "ax", @progbits
        .global Slug_Music1052_02a5fa
Slug_Music1052_02a5fa:
        move.w  #0x1052,d0                      | +000

| ----------------------------------------------------------------------------
|  Slug_MusicByTerrainB_02a606  @ $02A606  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_MusicByTerrainB_02a606, "ax", @progbits
        .global Slug_MusicByTerrainB_02a606
Slug_MusicByTerrainB_02a606:
        jsr     0x27eba.l                       | +000
        andi.b  #0xc0,d0                        | +006
        cmpi.b  #0x40,d0                        | +00a
        beq.w   Slug_Music1051_02a62c           | +00e
        cmpi.b  #0x80,d0                        | +012
        beq.w   Slug_Music1051_02a62c           | +016
        move.w  #0x1091,d0                      | +01a

| ----------------------------------------------------------------------------
|  Slug_Music1051_02a62c  @ $02A62C  (4 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_Music1051_02a62c, "ax", @progbits
        .global Slug_Music1051_02a62c
Slug_Music1051_02a62c:
        move.w  #0x1051,d0                      | +000

| ----------------------------------------------------------------------------
|  Slug_DestroyedMusicAndBubble_02a638  @ $02A638  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_DestroyedMusicAndBubble_02a638, "ax", @progbits
        .global Slug_DestroyedMusicAndBubble_02a638
Slug_DestroyedMusicAndBubble_02a638:
        btst    #0x0,0x13(a6)                   | +000
        beq.w   .L02a64c                        | +006
        move.w  #0x10e9,d0                      | +00a
        jsr     0x2352.l                        | +00e
.L02a64c:
        btst    #0x0,0x13(a6)                   | +014
        beq.w   JsrAbsRts_02a662                | +01a
        lea     0x3207c.l,a1                    | +01e

| ----------------------------------------------------------------------------
|  Slug_TryStartDestroyed_02a664  @ $02A664  (32 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_TryStartDestroyed_02a664, "ax", @progbits
        .global Slug_TryStartDestroyed_02a664
Slug_TryStartDestroyed_02a664:
        btst    #0x5,0x8d(a6)                   | +000
        bne.w   JmpAbsThunk_02a68a              | +006
        tst.w   0x106e92.l                      | +00a
        bne.w   JmpAbsThunk_02a68a              | +010
        bset    #0x0,0x13(a6)                   | +014
        lea     Sub_0002DCC0(pc),a1             | +01a
        move.l  a1,(a6)                         | +01e

| ----------------------------------------------------------------------------
|  Slug_TryStartDestroyedB_02a690  @ $02A690  (40 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_TryStartDestroyedB_02a690, "ax", @progbits
        .global Slug_TryStartDestroyedB_02a690
Slug_TryStartDestroyedB_02a690:
        btst    #0x5,0x8d(a6)                   | +000
        bne.w   Slug_DamageTick_02a6be          | +006
        tst.w   0x106e92.l                      | +00a
        bne.w   Slug_DamageTick_02a6be          | +010
        bset    #0x3,0x13(a6)                   | +014
        bset    #0x0,0x13(a6)                   | +01a
        lea     Sub_0002DCC0(pc),a1             | +020
        move.l  a1,(a6)                         | +024
        clr.w   d0                              | +026

| ----------------------------------------------------------------------------
|  Slug_DamageTick_02a6be  @ $02A6BE  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_DamageTick_02a6be, "ax", @progbits
        .global Slug_DamageTick_02a6be
Slug_DamageTick_02a6be:
        tst.b   0x45(a6)                        | +000
        beq.w   .L02a6cc                        | +004
        jsr     0x8f6da.l                       | +008
.L02a6cc:
        btst    #0x3,0x100001.l                 | +00e
        bne.w   Slug_ResetHP_02a712             | +016
        btst    #0x5,0x100001.l                 | +01a
        beq.w   .L02a6ea                        | +022
        move.w  #0x300,0x66(a6)                 | +026
.L02a6ea:
        jsr     0x2870a.l                       | +02c
        bcc.w   ClearXN_02a70c                  | +032
        movem.w d0,-(a7)                        | +036
        move.w  #0x1089,d0                      | +03a
        jsr     0x2352.l                        | +03e
        movem.w (a7)+,d0                        | +044

| ----------------------------------------------------------------------------
|  Slug_ResetHP_02a712  @ $02A712  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_ResetHP_02a712, "ax", @progbits
        .global Slug_ResetHP_02a712
Slug_ResetHP_02a712:
        move.w  #0x30,0x66(a6)                  | +000
        clr.w   d0                              | +006

| ----------------------------------------------------------------------------
|  Slug_UpdateInputFlags_02a720  @ $02A720  (50 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_UpdateInputFlags_02a720, "ax", @progbits
        .global Slug_UpdateInputFlags_02a720
Slug_UpdateInputFlags_02a720:
        bclr    #0x4,0x13(a6)                   | +000
        bclr    #0x5,0x13(a6)                   | +006
        jsr     0x5ceec.l                       | +00c
        bcc.w   .L02a740                        | +012
        bset    #0x4,0x13(a6)                   | +016
        bra.w   .L02a740                        | +01c
.L02a740:
        jsr     0x5cef8.l                       | +020
        bcc.w   .L02a750                        | +026
        bset    #0x5,0x13(a6)                   | +02a
.L02a750:
        rts                                     | +030

| ----------------------------------------------------------------------------
|  Slug_PhysicsA_02a752  @ $02A752  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_PhysicsA_02a752, "ax", @progbits
        .global Slug_PhysicsA_02a752
Slug_PhysicsA_02a752:
        jsr     0x28992.l                       | +000

| ----------------------------------------------------------------------------
|  Slug_PhysicsB_02a760  @ $02A760  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_PhysicsB_02a760, "ax", @progbits
        .global Slug_PhysicsB_02a760
Slug_PhysicsB_02a760:
        jsr     0x28992.l                       | +000

| ----------------------------------------------------------------------------
|  Slug_PhysicsC_02a766  @ $02A766  (58 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_PhysicsC_02a766, "ax", @progbits
        .global Slug_PhysicsC_02a766
Slug_PhysicsC_02a766:
        jsr     0x304c4.l                       | +000
        jsr     0x30704.l                       | +006
        beq.w   .L02a77c                        | +00c
        jsr     0x281b0.l                       | +010
.L02a77c:
        jsr     0x2788c.l                       | +016
        jsr     Slug_UpdateInputFlags_02a720(pc) | +01c
        btst    #0x5,0x5a(a6)                   | +020
        beq.w   .L02a796                        | +026
        addi.w  #0x10,0x82(a6)                  | +02a
.L02a796:
        jsr     0x3076a.l                       | +030
        beq.w   ClearXN_02a7a6                  | +036

| ----------------------------------------------------------------------------
|  Slug_PhysicsD_02a7ac  @ $02A7AC  (32 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_PhysicsD_02a7ac, "ax", @progbits
        .global Slug_PhysicsD_02a7ac
Slug_PhysicsD_02a7ac:
        jsr     0x28992.l                       | +000
        jsr     0x304c4.l                       | +006
        jsr     0x27a18.l                       | +00c
        jsr     Slug_UpdateInputFlags_02a720(pc) | +012
        jsr     0x3076a.l                       | +016
        beq.w   ClearXN_02a7d2                  | +01c

| ----------------------------------------------------------------------------
|  Slug_PhysicsE_02a7d8  @ $02A7D8  (64 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_PhysicsE_02a7d8, "ax", @progbits
        .global Slug_PhysicsE_02a7d8
Slug_PhysicsE_02a7d8:
        jsr     0x28992.l                       | +000
        jsr     0x30704.l                       | +006
        beq.w   .L02a7ee                        | +00c
        jsr     0x281b0.l                       | +010
.L02a7ee:
        jsr     0x304c4.l                       | +016
        jsr     0x2788c.l                       | +01c
        jsr     Slug_UpdateInputFlags_02a720(pc) | +022
        btst    #0x5,0x5a(a6)                   | +026
        beq.w   .L02a80e                        | +02c
        addi.w  #0x10,0x82(a6)                  | +030
.L02a80e:
        jsr     0x3076a.l                       | +036
        beq.w   ClearXN_02a81e                  | +03c

| ----------------------------------------------------------------------------
|  Slug_PhysicsF_02a824  @ $02A824  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_PhysicsF_02a824, "ax", @progbits
        .global Slug_PhysicsF_02a824
Slug_PhysicsF_02a824:
        jsr     0x304c4.l                       | +000
        jsr     0x30704.l                       | +006
        beq.w   .L02a83a                        | +00c
        jsr     0x281b0.l                       | +010
.L02a83a:
        jsr     0x2788c.l                       | +016
        jsr     Slug_UpdateInputFlags_02a720(pc) | +01c
        jsr     0x3076a.l                       | +020
        beq.w   ClearXN_02a854                  | +026

| ----------------------------------------------------------------------------
|  Slug_PhysicsG_02a85a  @ $02A85A  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_PhysicsG_02a85a, "ax", @progbits
        .global Slug_PhysicsG_02a85a
Slug_PhysicsG_02a85a:
        jsr     0x28992.l                       | +000
        jsr     0x30704.l                       | +006
        beq.w   JsrAbsThunk_02a870              | +00c
        jsr     0x281b0.l                       | +010

| ----------------------------------------------------------------------------
|  Slug_PhysicsAir_02a878  @ $02A878  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_PhysicsAir_02a878, "ax", @progbits
        .global Slug_PhysicsAir_02a878
Slug_PhysicsAir_02a878:
        jsr     0x28992.l                       | +000
        jsr     0x30704.l                       | +006
        beq.w   .L02a88e                        | +00c
        jsr     0x281b0.l                       | +010
.L02a88e:
        jsr     0x27a18.l                       | +016
        bcs.w   Slug_ClearBit1Field8D_02a8b4    | +01c
        tst.w   0x2a(a6)                        | +020
        bge.w   .L02a8a8                        | +024
        jsr     Slug_GroundContact_02a8c0(pc)   | +028
        bcc.w   Slug_ClearBit1Field8D_02a8b4    | +02c
.L02a8a8:
        bset    #0x1,0x8d(a6)                   | +030

| ----------------------------------------------------------------------------
|  Slug_ClearBit1Field8D_02a8b4  @ $02A8B4  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_ClearBit1Field8D_02a8b4, "ax", @progbits
        .global Slug_ClearBit1Field8D_02a8b4
Slug_ClearBit1Field8D_02a8b4:
        bclr    #0x1,0x8d(a6)                   | +000

| ----------------------------------------------------------------------------
|  Slug_GroundContact_02a8c0  @ $02A8C0  (110 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_GroundContact_02a8c0, "ax", @progbits
        .global Slug_GroundContact_02a8c0
Slug_GroundContact_02a8c0:
        btst    #0x6,0x6b(a6)                   | +000
        beq.w   .L02a8d0                        | +006
        jmp     0x27eba.l                       | +00a
.L02a8d0:
        bclr    #0x3,0x5b(a6)                   | +010
        move.w  0x22(a6),d1                     | +016
        move.w  0x82(a6),d2                     | +01a
        jsr     0x27ec2.l                       | +01e
        bcs.w   .L02a8fa                        | +024
        movea.l 0x74(a6),a2                     | +028
        movea.l 0x78(a6),a3                     | +02c
        movea.l 0x7c(a6),a4                     | +030
        moveq   #3,d1                           | +034
        bra.w   Slug_GroundContactWrapY_02a934__L02a940 | +036
.L02a8fa:
        movea.l 0x74(a6),a2                     | +03a
        movea.l 0x78(a6),a3                     | +03e
        movea.l 0x7c(a6),a4                     | +042
        moveq   #0,d1                           | +046
        add.b   0x84(a4),d1                     | +048
        add.b   0x84(a2),d1                     | +04c
        add.b   0x84(a3),d1                     | +050
        cmpi.b  #0x2,d1                         | +054
        bge.w   Slug_GroundContactWrapY_02a934  | +058
        andi.w  #0x1ff,0x24(a2)                 | +05c
        andi.w  #0x1ff,0x24(a3)                 | +062
        andi.w  #0x1ff,0x24(a4)                 | +068

| ----------------------------------------------------------------------------
|  Slug_GroundContactWrapY_02a934  @ $02A934  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_GroundContactWrapY_02a934, "ax", @progbits
        .global Slug_GroundContactWrapY_02a934
Slug_GroundContactWrapY_02a934:
        bset    #0x3,0x5b(a6)                   | +000
        move.w  0x24(a6),0x82(a6)               | +006
        .global Slug_GroundContactWrapY_02a934__L02a940
Slug_GroundContactWrapY_02a934__L02a940:
        andi.w  #0x1ff,0x24(a2)                 | +00c
        andi.w  #0x1ff,0x24(a3)                 | +012
        andi.w  #0x1ff,0x24(a4)                 | +018

| ----------------------------------------------------------------------------
|  Slug_TerrainSlope_02a958  @ $02A958  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_TerrainSlope_02a958, "ax", @progbits
        .global Slug_TerrainSlope_02a958
Slug_TerrainSlope_02a958:
        movea.l 0x74(a6),a1                     | +000
        move.w  0x22(a1),d1                     | +004
        move.w  0x9a(a1),d2                     | +008
        movea.l 0x78(a6),a2                     | +00c
        move.w  0x22(a2),d3                     | +010
        move.w  0x9a(a2),d4                     | +014
        sub.w   d3,d1                           | +018
        move.w  d1,d0                           | +01a
        sub.w   d4,d2                           | +01c
        bne.w   .L02a980                        | +01e
        clr.w   d1                              | +022
        bra.w   .L02a982                        | +024
.L02a980:
        muls.w  d2,d1                           | +028
.L02a982:
        cmpi.w  #0xffc0,d2                      | +02a
        bge.w   .L02a992                        | +02e
        move.w  #0xffc0,d2                      | +032
        bra.w   .L02a99e                        | +036
.L02a992:
        cmpi.w  #0x40,d2                        | +03a
        ble.w   .L02a99e                        | +03e
        move.w  #0x40,d2                        | +042
.L02a99e:
        rts                                     | +046

| ----------------------------------------------------------------------------
|  Slug_SlopeToAnimIdx_02a9a0  @ $02A9A0  (110 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_SlopeToAnimIdx_02a9a0, "ax", @progbits
        .global Slug_SlopeToAnimIdx_02a9a0
Slug_SlopeToAnimIdx_02a9a0:
        bra.w   .L02a9ce                        | +000
        clr.w   d0                              | +004
        move.w  0x64(a6),d7                     | +006
        cmpi.w  #0x400,d7                       | +00a
        beq.w   .L02a9f6                        | +00e
        cmpi.w  #0xfc00,d7                      | +012
        beq.w   .L02a9fc                        | +016
        cmpi.w  #0x1000,d7                      | +01a
        beq.w   .L02aa02                        | +01e
        cmpi.w  #0xf000,d7                      | +022
        beq.w   .L02aa08                        | +026
        bra.w   .L02a9f4                        | +02a
.L02a9ce:
        jsr     Slug_TerrainSlope_02a958(pc)    | +02e
        clr.w   d0                              | +032
        cmpi.w  #0x384,d1                       | +034
        bge.w   .L02a9f6                        | +038
        cmpi.w  #0xfc7c,d1                      | +03c
        ble.w   .L02a9fc                        | +040
        cmpi.w  #0x5a,d1                        | +044
        bge.w   .L02aa02                        | +048
        cmpi.w  #0xffa6,d1                      | +04c
        ble.w   .L02aa08                        | +050
.L02a9f4:
        rts                                     | +054
.L02a9f6:
        move.w  #0x3,d0                         | +056
        bra.b   .L02a9f4                        | +05a
.L02a9fc:
        move.w  #0x4,d0                         | +05c
        bra.b   .L02a9f4                        | +060
.L02aa02:
        move.w  #0x1,d0                         | +062
        bra.b   .L02a9f4                        | +066
.L02aa08:
        move.w  #0x2,d0                         | +068
        bra.b   .L02a9f4                        | +06c

| ----------------------------------------------------------------------------
|  Slug_UpdateAnimKeepIdx_02aa0e  @ $02AA0E  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_UpdateAnimKeepIdx_02aa0e, "ax", @progbits
        .global Slug_UpdateAnimKeepIdx_02aa0e
Slug_UpdateAnimKeepIdx_02aa0e:
        move.w  0x94(a6),d0                     | +000
        movem.w d0,-(a7)                        | +004
        jsr     Slug_UpdateAnimAndChassis_02aa24(pc) | +008
        movem.w (a7)+,d0                        | +00c

| ----------------------------------------------------------------------------
|  Slug_UpdateAnimAndChassis_02aa24  @ $02AA24  (140 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_UpdateAnimAndChassis_02aa24, "ax", @progbits
        .global Slug_UpdateAnimAndChassis_02aa24
Slug_UpdateAnimAndChassis_02aa24:
        jsr     0x283ca.l                       | +000
        cmpi.b  #0x0,0x68(a6)                   | +006
        beq.b   .L02aa3c                        | +00c
        cmpi.b  #0x1,0x68(a6)                   | +00e
        bne.w   .L02aa42                        | +014
.L02aa3c:
        move.b  0x68(a6),0x85(a6)               | +018
.L02aa42:
        jsr     Slug_ResetTurnTimer_02a4a2(pc)  | +01e
        btst    #0x0,0x13(a6)                   | +022
        bne.w   .L02aa54                        | +028
        jsr     Slug_UpdateDamageSprite_02fae4(pc) | +02c
.L02aa54:
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +030
        move.w  d0,0x94(a6)                     | +034
        move.b  0x8e(a6),d0                     | +038
        clr.b   d1                              | +03c
        subi.b  #0x1,d0                         | +03e
        addx.b  d1,d0                           | +042
        move.b  d0,0x8e(a6)                     | +044
        movea.l 0x74(a6),a2                     | +048
        movea.l 0x78(a6),a3                     | +04c
        move.w  0x9a(a2),d1                     | +050
        move.w  0x9a(a3),d0                     | +054
        clr.w   d3                              | +058
        sub.w   d0,d1                           | +05a
        bge.w   .L02aa8a                        | +05c
        move.w  0x9a(a2),d0                     | +060
        moveq   #-1,d3                          | +064
.L02aa8a:
        asl.w   #0x5,d1                         | +066
        move.w  0x22(a2),d2                     | +068
        sub.w   0x22(a3),d2                     | +06c
        beq.w   .L02aaa0                        | +070
        ext.l   d1                              | +074
        divs.w  d2,d1                           | +076
        bra.w   .L02aaa2                        | +078
.L02aaa0:
        clr.w   d1                              | +07c
.L02aaa2:
        asr.w   #0x1,d1                         | +07e
        eor.w   d3,d1                           | +080
        sub.w   d3,d1                           | +082
        add.w   d0,d1                           | +084
        move.w  d1,0x24(a6)                     | +086
        rts                                     | +08a

| ----------------------------------------------------------------------------
|  Slug_GaugeTick_02aab0  @ $02AAB0  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_GaugeTick_02aab0, "ax", @progbits
        .global Slug_GaugeTick_02aab0
Slug_GaugeTick_02aab0:
        move.b  0x90(a6),d0                     | +000
        moveq   #0,d1                           | +004
        subq.b  #0x1,d0                         | +006
        addx.b  d1,d0                           | +008

| ----------------------------------------------------------------------------
|  Slug_CanFire_02aac0  @ $02AAC0  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_CanFire_02aac0, "ax", @progbits
        .global Slug_CanFire_02aac0
Slug_CanFire_02aac0:
        cmpi.b  #0x0,0x90(a6)                   | +000
        beq.w   ClearXN_02aade                  | +006
        move.b  #0x2,d0                         | +00a
        cmp.b   0x8f(a6),d0                     | +00e
        bls.w   ClearXN_02aade                  | +012
        tst.b   0x8e(a6)                        | +016
        beq.w   JsrPcThunk_02aae4               | +01a

| ----------------------------------------------------------------------------
|  Slug_JmpInput5CDA8_02aaea  @ $02AAEA  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_JmpInput5CDA8_02aaea, "ax", @progbits
        .global Slug_JmpInput5CDA8_02aaea
Slug_JmpInput5CDA8_02aaea:
        jmp     0x5cda8.l                       | +000

| ----------------------------------------------------------------------------
|  Slug_InputDirByLayoutA_02aaf0  @ $02AAF0  (56 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_InputDirByLayoutA_02aaf0, "ax", @progbits
        .global Slug_InputDirByLayoutA_02aaf0
Slug_InputDirByLayoutA_02aaf0:
        cmpi.b  #0x0,0x106f2a.l                 | +000
        bne.w   .L02ab06                        | +008
        jsr     0x5cdb4.l                       | +00c
        bra.w   .L02ab22                        | +012
.L02ab06:
        cmpi.b  #0x4,0x106f2a.l                 | +016
        bne.w   .L02ab1c                        | +01e
        jsr     0x5cdb4.l                       | +022
        bra.w   .L02ab22                        | +028
.L02ab1c:
        jsr     0x5ceec.l                       | +02c
.L02ab22:
        bcs.w   Slug_InputDirA_02ab2e           | +032
        moveq   #0,d0                           | +036

| ----------------------------------------------------------------------------
|  Slug_InputDirA_02ab2e  @ $02AB2E  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_InputDirA_02ab2e, "ax", @progbits
        .global Slug_InputDirA_02ab2e
Slug_InputDirA_02ab2e:
        jsr     0x5d5b6.l                       | +000
        addq.w  #0x1,d0                         | +006

| ----------------------------------------------------------------------------
|  Slug_InputDirByLayoutB_02ab3c  @ $02AB3C  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_InputDirByLayoutB_02ab3c, "ax", @progbits
        .global Slug_InputDirByLayoutB_02ab3c
Slug_InputDirByLayoutB_02ab3c:
        cmpi.b  #0x0,0x106f2a.l                 | +000
        bne.w   .L02ab52                        | +008
        jsr     0x5cdb4.l                       | +00c
        bra.w   .L02ab6c                        | +012
.L02ab52:
        cmpi.b  #0x4,0x106f2a.l                 | +016
        bne.w   .L02ab68                        | +01e
        jsr     0x5cdb4.l                       | +022
        bra.w   .L02ab6c                        | +028
.L02ab68:
        jsr     Slug_CallGroundProbeC_02a374(pc) | +02c
.L02ab6c:
        bcs.w   Slug_InputDirB_02ab78           | +030
        moveq   #0,d0                           | +034

| ----------------------------------------------------------------------------
|  Slug_InputDirB_02ab78  @ $02AB78  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_InputDirB_02ab78, "ax", @progbits
        .global Slug_InputDirB_02ab78
Slug_InputDirB_02ab78:
        jsr     0x5d5b6.l                       | +000
        addq.w  #0x1,d0                         | +006

| ----------------------------------------------------------------------------
|  Slug_InputFireByLayout_02ab86  @ $02AB86  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_InputFireByLayout_02ab86, "ax", @progbits
        .global Slug_InputFireByLayout_02ab86
Slug_InputFireByLayout_02ab86:
        cmpi.b  #0x4,0x106f2a.l                 | +000
        bne.w   .L02ab9c                        | +008
        jsr     0x5cda8.l                       | +00c
        bra.w   JsrAbsRts_02abb8                | +012
.L02ab9c:
        cmpi.b  #0x0,0x106f2a.l                 | +016
        bne.w   JsrAbsThunk_02abb2              | +01e
        jsr     0x5cdc0.l                       | +022
        bra.w   JsrAbsRts_02abb8                | +028

| ----------------------------------------------------------------------------
|  Slug_TestBit4Field8D_02abd2  @ $02ABD2  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_TestBit4Field8D_02abd2, "ax", @progbits
        .global Slug_TestBit4Field8D_02abd2
Slug_TestBit4Field8D_02abd2:
        lea     0x100580.l,a0                   | +000
        btst    #0x4,0x8d(a0)                   | +006
        bne.w   SetXN_02abea                    | +00c
        andi.b  #0xee,ccr                       | +010
        bra.w   SetXNMid_02abee                 | +014

| ----------------------------------------------------------------------------
|  Slug_TestBit1Field8D_02abf0  @ $02ABF0  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_TestBit1Field8D_02abf0, "ax", @progbits
        .global Slug_TestBit1Field8D_02abf0
Slug_TestBit1Field8D_02abf0:
        lea     0x100580.l,a0                   | +000
        btst    #0x1,0x8d(a0)                   | +006
        bne.w   SetXN_02ac08                    | +00c
        andi.b  #0xee,ccr                       | +010
        bra.w   SetXNMid_02ac0c                 | +014

| ----------------------------------------------------------------------------
|  Slug_IsRiddenByPlayer_02ac0e  @ $02AC0E  (50 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_IsRiddenByPlayer_02ac0e, "ax", @progbits
        .global Slug_IsRiddenByPlayer_02ac0e
Slug_IsRiddenByPlayer_02ac0e:
        cmpa.l  #0x100440,a0                    | +000
        bne.w   .L02ac1e                        | +006
        moveq   #1,d0                           | +00a
        bra.w   .L02ac32                        | +00c
.L02ac1e:
        cmpa.l  #0x1004e0,a0                    | +010
        bne.w   .L02ac2e                        | +016
        moveq   #2,d0                           | +01a
        bra.w   .L02ac32                        | +01c
.L02ac2e:
        bra.w   ClearXN_02ac46                  | +020
.L02ac32:
        lea     0x100580.l,a1                   | +024
        cmp.b   0x6d(a1),d0                     | +02a
        bne.w   ClearXN_02ac46                  | +02e

| ----------------------------------------------------------------------------
|  Slug_TestField100609_02ac4c  @ $02AC4C  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_TestField100609_02ac4c, "ax", @progbits
        .global Slug_TestField100609_02ac4c
Slug_TestField100609_02ac4c:
        move.b  0x100609.l,d0                   | +000
        bne.w   Slug_CopyField68_02ac5c         | +006

| ----------------------------------------------------------------------------
|  Slug_CopyField68_02ac5c  @ $02AC5C  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_CopyField68_02ac5c, "ax", @progbits
        .global Slug_CopyField68_02ac5c
Slug_CopyField68_02ac5c:
        move.b  0x68(a0),d2                     | +000
        move.b  d2,0x68(a6)                     | +004

| ----------------------------------------------------------------------------
|  Slug_TestField100609B_02ac6a  @ $02AC6A  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_TestField100609B_02ac6a, "ax", @progbits
        .global Slug_TestField100609B_02ac6a
Slug_TestField100609B_02ac6a:
        move.b  0x100609.l,d0                   | +000
        bne.w   SetXN_02ac7a                    | +006

| ----------------------------------------------------------------------------
|  Slug_ConsumeField89_02ac80  @ $02AC80  (28 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_ConsumeField89_02ac80, "ax", @progbits
        .global Slug_ConsumeField89_02ac80
Slug_ConsumeField89_02ac80:
        cmpi.b  #0x0,0x89(a6)                   | +000
        bne.w   .L02ac98                        | +006
        move.b  #0xff,0x6d(a6)                  | +00a
        ori.b   #0x11,ccr                       | +010
        bra.w   ClearXNMid_02aca0               | +014
.L02ac98:
        clr.b   0x89(a6)                        | +018

| ----------------------------------------------------------------------------
|  Slug_SetField89_02aca2  @ $02ACA2  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_SetField89_02aca2, "ax", @progbits
        .global Slug_SetField89_02aca2
Slug_SetField89_02aca2:
        movem.l a0,-(a7)                        | +000
        lea     0x100580.l,a0                   | +004
        move.b  #0xff,0x89(a0)                  | +00a
        movem.l (a7)+,a0                        | +010
        rts                                     | +014

| ----------------------------------------------------------------------------
|  Slug_CheckPlayersNear_02acb8  @ $02ACB8  (56 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_CheckPlayersNear_02acb8, "ax", @progbits
        .global Slug_CheckPlayersNear_02acb8
Slug_CheckPlayersNear_02acb8:
        cmpi.b  #0x0,0x106ed3.l                 | +000
        bne.w   .L02acc8                        | +008
        bra.w   ClearXN_02acf0                  | +00c
.L02acc8:
        jsr     0x3eecc.l                       | +010
        bcc.w   .L02acdc                        | +016
        jsr     0x32d6c.l                       | +01a
        bcs.w   SetXN_02acf6                    | +020
.L02acdc:
        jsr     0x3ef14.l                       | +024
        bcc.w   ClearXN_02acf0                  | +02a
        jsr     0x32d6c.l                       | +02e
        bcs.w   SetXN_02acf6                    | +034

| ----------------------------------------------------------------------------
|  Slug_IsAlive_02acfc  @ $02ACFC  (46 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_IsAlive_02acfc, "ax", @progbits
        .global Slug_IsAlive_02acfc
Slug_IsAlive_02acfc:
        lea     0x100580.l,a0                   | +000
        cmpi.l  #0xffffffff,(a0)                | +006
        beq.w   ClearXN_02ad30                  | +00c
        cmpi.l  #0x52a,(a0)                     | +010
        beq.w   ClearXN_02ad30                  | +016
        cmpi.l  #0x400,(a0)                     | +01a
        beq.w   ClearXN_02ad30                  | +020
        cmpi.l  #0x2ae3e,(a0)                   | +024
        beq.w   ClearXN_02ad30                  | +02a

| ----------------------------------------------------------------------------
|  Slug_GetRiderAndField98_02ad36  @ $02AD36  (40 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_GetRiderAndField98_02ad36, "ax", @progbits
        .global Slug_GetRiderAndField98_02ad36
Slug_GetRiderAndField98_02ad36:
        jsr     Slug_IsAlive_02acfc(pc)         | +000
        bcc.w   Slug_NoRider_02ad64              | +004
        move.b  0x85(a0),d0                     | +008
        cmp.b   0x6e(a6),d0                     | +00c
        bne.w   Slug_NoRider_02ad64              | +010
        move.l  0x98(a0),d2                     | +014
        cmpi.l  #0xffffffff,d2                  | +018
        bne.w   SetXN_02ad5e                    | +01e
        move.l  #0x10000,d2                     | +022

| ----------------------------------------------------------------------------
|  Slug_NoRider_02ad64  @ $02AD64  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_NoRider_02ad64, "ax", @progbits
        .global Slug_NoRider_02ad64
Slug_NoRider_02ad64:
        move.b  #0xff,d0                        | +000
        clr.l   d2                              | +004

| ----------------------------------------------------------------------------
|  Slug_SpawnRiderMarker_02ad70  @ $02AD70  (102 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_SpawnRiderMarker_02ad70, "ax", @progbits
        .global Slug_SpawnRiderMarker_02ad70
Slug_SpawnRiderMarker_02ad70:
        jsr     Slug_IsAlive_02acfc(pc)         | +000
        bcc.w   .L02add4                        | +004
        move.b  0x85(a0),d0                     | +008
        cmpi.b  #0x0,d0                         | +00c
        bne.w   .L02ada6                        | +010
        lea     0x28dd3c.l,a1                   | +014
        jsr     0x4ae.l                         | +01a
        jsr     0x5dd02.l                       | +020
        move.w  #0x30,0x22(a0)                  | +026
        move.w  #0x174,0x24(a0)                 | +02c
        bra.w   .L02add4                        | +032
.L02ada6:
        move.b  0x85(a0),d0                     | +036
        cmpi.b  #0x1,d0                         | +03a
        bne.w   .L02add4                        | +03e
        lea     0x28dd3c.l,a1                   | +042
        jsr     0x4ae.l                         | +048
        jsr     0x5dd02.l                       | +04e
        move.w  #0xb8,0x22(a0)                  | +054
        move.w  #0x174,0x24(a0)                 | +05a
        bra.w   .L02add4                        | +060
.L02add4:
        rts                                     | +064

| ----------------------------------------------------------------------------
|  Slug_SpawnAtBossArena_02add6  @ $02ADD6  (96 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_SpawnAtBossArena_02add6, "ax", @progbits
        .global Slug_SpawnAtBossArena_02add6
Slug_SpawnAtBossArena_02add6:
        move.w  #0x40,0x22(a6)                  | +000
        move.w  #0x171,0x24(a6)                 | +006
        move.w  #0x171,0x82(a6)                 | +00c
        move.w  #0x3,d1                         | +012
        jsr     Slug_InitBoss_02a1aa(pc)        | +016
        ori.w   #0x2,0x38(a6)                   | +01a
        lea     0x2792b0.l,a0                   | +020
        jsr     0x28cd4.l                       | +026
        clr.w   0x28(a6)                        | +02c
        clr.w   0x2a(a6)                        | +030
        clr.w   0x2c(a6)                        | +034
        clr.w   0x2e(a6)                        | +038
        clr.b   0x26(a6)                        | +03c
        clr.b   0x27(a6)                        | +040
        clr.b   0x8e(a6)                        | +044
        bset    #0x6,0x13(a6)                   | +048
        jsr     Slug_UpdateAnimKeepIdx_02aa0e(pc) | +04e
        jsr     Slug_PhysicsAir_02a878(pc)      | +052
        jsr     Slug_UpdateAirFlag_02a478(pc)   | +056
        jsr     0x28d70.l                       | +05a
