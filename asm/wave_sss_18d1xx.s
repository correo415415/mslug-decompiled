| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave ??? — (borrador)
|  Región: $18D152..$18DB78  (2,498 B, 17 entradas, 8 huecos)
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
|  Data_18d152  @ $18D152  (164 B)
| ----------------------------------------------------------------------------
        .section .text.Data_18d152, "ax", @progbits
        .global Data_18d152
Data_18d152:
        .dc.w   0x0317                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0204                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfffa                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xfffa                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +010  (dato / opcode no decodificado)
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
        .dc.w   0xfffa                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xfffa                        | +032  (dato / opcode no decodificado)
        .dc.w   0xfffa                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +04a  (dato / opcode no decodificado)
        .dc.w   0xfffa                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0417                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0204                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +058  (dato / opcode no decodificado)
        .dc.w   0xfffa                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +05c  (dato / opcode no decodificado)
        .dc.w   0xfffa                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +060  (dato / opcode no decodificado)
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
        .dc.w   0xfffa                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +07e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +080  (dato / opcode no decodificado)
        .dc.w   0xfffa                        | +082  (dato / opcode no decodificado)
        .dc.w   0xfffa                        | +084  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +088  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +08a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +090  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +094  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +096  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +098  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +09a  (dato / opcode no decodificado)
        .dc.w   0xfffa                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +09e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a2  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Data_18d1f6  @ $18D1F6  (164 B)
| ----------------------------------------------------------------------------
        .section .text.Data_18d1f6, "ax", @progbits
        .global Data_18d1f6
Data_18d1f6:
        .dc.w   0x0306                        | +000  (dato / opcode no decodificado)
        .dc.w   0x00c8                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0204                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfffa                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xfffa                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +010  (dato / opcode no decodificado)
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
        .dc.w   0xfffa                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xfffa                        | +032  (dato / opcode no decodificado)
        .dc.w   0xfffa                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +04a  (dato / opcode no decodificado)
        .dc.w   0xfffa                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0406                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0190                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0204                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +058  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +05c  (dato / opcode no decodificado)
        .dc.w   0xfffa                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +060  (dato / opcode no decodificado)
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
        .dc.w   0xfff8                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +07e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +080  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +082  (dato / opcode no decodificado)
        .dc.w   0xfffa                        | +084  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +088  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +08a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +090  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +094  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +096  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +098  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +09a  (dato / opcode no decodificado)
        .dc.w   0xfffa                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +09e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a2  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Data_18d29a  @ $18D29A  (328 B)
| ----------------------------------------------------------------------------
        .section .text.Data_18d29a, "ax", @progbits
        .global Data_18d29a
Data_18d29a:
        .dc.w   0x0318                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0064                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0404                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xffe8                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0080                        | +010  (dato / opcode no decodificado)
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
        .dc.w   0xffe8                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0080                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xffe8                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0080                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0318                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0064                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0404                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +058  (dato / opcode no decodificado)
        .dc.w   0xffd8                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +05c  (dato / opcode no decodificado)
        .dc.w   0xffe0                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +060  (dato / opcode no decodificado)
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
        .dc.w   0xffd8                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +07e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +080  (dato / opcode no decodificado)
        .dc.w   0xffd8                        | +082  (dato / opcode no decodificado)
        .dc.w   0xffe0                        | +084  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +088  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +08a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +090  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +094  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +096  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +098  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +09a  (dato / opcode no decodificado)
        .dc.w   0xffe0                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +09e  (dato / opcode no decodificado)
        .dc.w   0x0418                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0x0064                        | +0a2  (dato / opcode no decodificado)
        .dc.w   0x0404                        | +0a4  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0a6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0a8  (dato / opcode no decodificado)
        .dc.w   0xffe8                        | +0aa  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +0ac  (dato / opcode no decodificado)
        .dc.w   0xffe8                        | +0ae  (dato / opcode no decodificado)
        .dc.w   0x0080                        | +0b0  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0b2  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0b4  (dato / opcode no decodificado)
        .dc.w   0x363a                        | +0b6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0b8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0ba  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0bc  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0be  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0c0  (dato / opcode no decodificado)
        .dc.w   0x3644                        | +0c2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0c4  (dato / opcode no decodificado)
        .dc.w   0xffe8                        | +0c6  (dato / opcode no decodificado)
        .dc.w   0x0080                        | +0c8  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0ca  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0cc  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +0ce  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0d0  (dato / opcode no decodificado)
        .dc.w   0xffe8                        | +0d2  (dato / opcode no decodificado)
        .dc.w   0xffe8                        | +0d4  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0d6  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0d8  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +0da  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0dc  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +0de  (dato / opcode no decodificado)
        .dc.w   0x0080                        | +0e0  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0e2  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0e4  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +0e6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0e8  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +0ea  (dato / opcode no decodificado)
        .dc.w   0xffe8                        | +0ec  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +0ee  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0f0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0f2  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +0f4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0f6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0f8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0fa  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0fc  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +0fe  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +100  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +102  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +104  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +106  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +108  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +10a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +10c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +10e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +110  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +112  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +114  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +116  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +118  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +11a  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +11c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +11e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +120  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +122  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +124  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +126  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +128  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +12a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +12c  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +12e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +130  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +132  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +134  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +136  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +138  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +13a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +13c  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +13e  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +140  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +142  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +144  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +146  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Data_18d3e2  @ $18D3E2  (332 B)
| ----------------------------------------------------------------------------
        .section .text.Data_18d3e2, "ax", @progbits
        .global Data_18d3e2
Data_18d3e2:
        .dc.w   0x0800                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +002  (dato / opcode no decodificado)
        .dc.w   0x83ca                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0d54                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0d64                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0d74                        | +020  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0d84                        | +02a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0d94                        | +034  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +038  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0da4                        | +03e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +044  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0db4                        | +048  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0dc4                        | +052  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +058  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0dd8                        | +05c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +060  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +062  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +064  (dato / opcode no decodificado)
        .dc.w   0x0de8                        | +066  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +068  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x0dfc                        | +070  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +072  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +074  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0e10                        | +07a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +07e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +080  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +082  (dato / opcode no decodificado)
        .dc.w   0x0e1c                        | +084  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +088  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +08a  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x0e30                        | +08e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +090  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +094  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +096  (dato / opcode no decodificado)
        .dc.w   0x0e44                        | +098  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +09a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +09e  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0x0e58                        | +0a2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a4  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +0a6  (dato / opcode no decodificado)
        .dc.w   0x020b                        | +0a8  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +0aa  (dato / opcode no decodificado)
        .dc.w   0x0d54                        | +0ac  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0ae  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +0b0  (dato / opcode no decodificado)
        .dc.w   0x020b                        | +0b2  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +0b4  (dato / opcode no decodificado)
        .dc.w   0x0d64                        | +0b6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0b8  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +0ba  (dato / opcode no decodificado)
        .dc.w   0x020b                        | +0bc  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +0be  (dato / opcode no decodificado)
        .dc.w   0x0d74                        | +0c0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0c2  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +0c4  (dato / opcode no decodificado)
        .dc.w   0x020b                        | +0c6  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +0c8  (dato / opcode no decodificado)
        .dc.w   0x0d84                        | +0ca  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0cc  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +0ce  (dato / opcode no decodificado)
        .dc.w   0x020b                        | +0d0  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +0d2  (dato / opcode no decodificado)
        .dc.w   0x0d94                        | +0d4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0d6  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +0d8  (dato / opcode no decodificado)
        .dc.w   0x020b                        | +0da  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +0dc  (dato / opcode no decodificado)
        .dc.w   0x0da4                        | +0de  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0e0  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +0e2  (dato / opcode no decodificado)
        .dc.w   0x020b                        | +0e4  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +0e6  (dato / opcode no decodificado)
        .dc.w   0x0db4                        | +0e8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0ea  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +0ec  (dato / opcode no decodificado)
        .dc.w   0x020b                        | +0ee  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +0f0  (dato / opcode no decodificado)
        .dc.w   0x0dc4                        | +0f2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0f4  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +0f6  (dato / opcode no decodificado)
        .dc.w   0x020b                        | +0f8  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +0fa  (dato / opcode no decodificado)
        .dc.w   0x0dd8                        | +0fc  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0fe  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +100  (dato / opcode no decodificado)
        .dc.w   0x020b                        | +102  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +104  (dato / opcode no decodificado)
        .dc.w   0x0de8                        | +106  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +108  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +10a  (dato / opcode no decodificado)
        .dc.w   0x020b                        | +10c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +10e  (dato / opcode no decodificado)
        .dc.w   0x0dfc                        | +110  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +112  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +114  (dato / opcode no decodificado)
        .dc.w   0x020b                        | +116  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +118  (dato / opcode no decodificado)
        .dc.w   0x0e10                        | +11a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +11c  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +11e  (dato / opcode no decodificado)
        .dc.w   0x020b                        | +120  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +122  (dato / opcode no decodificado)
        .dc.w   0x0e1c                        | +124  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +126  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +128  (dato / opcode no decodificado)
        .dc.w   0x020b                        | +12a  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +12c  (dato / opcode no decodificado)
        .dc.w   0x0e30                        | +12e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +130  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +132  (dato / opcode no decodificado)
        .dc.w   0x020b                        | +134  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +136  (dato / opcode no decodificado)
        .dc.w   0x0e44                        | +138  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +13a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +13c  (dato / opcode no decodificado)
        .dc.w   0x020b                        | +13e  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +140  (dato / opcode no decodificado)
        .dc.w   0x0e58                        | +142  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +144  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +146  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +148  (dato / opcode no decodificado)
        .dc.w   0xd3e8                        | +14a  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Data_18d52e  @ $18D52E  (62 B)
| ----------------------------------------------------------------------------
        .section .text.Data_18d52e, "ax", @progbits
        .global Data_18d52e
Data_18d52e:
        .dc.w   0x0001                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +004  (dato / opcode no decodificado)
        .dc.w   0x06e0                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x06f0                        | +010  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0700                        | +01a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0722                        | +024  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x074a                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +032  (dato / opcode no decodificado)
        .dc.w   0x43fa                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +036  (dato / opcode no decodificado)
        .dc.w   0x4eb9                        | +038  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x04ae                        | +03c  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  TaskHandler_18d574  @ $18D574  (10 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_18d574, "ax", @progbits
        .global TaskHandler_18d574
TaskHandler_18d574:
        lea     TaskHandler_18d586__L18d5aa(pc),a1 | +000
        jsr     0x4ae.l                         | +004

| ----------------------------------------------------------------------------
|  TaskHandler_18d586  @ $18D586  (310 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_18d586, "ax", @progbits
        .global TaskHandler_18d586
TaskHandler_18d586:
        move.w  #0xf64e,d0                      | +000
        jsr     0x5dca4.l                       | +004
        move.w  d0,0x28(a6)                     | +00a
        move.w  #0xb9a,0x2a(a6)                 | +00e
        move.w  #0xff4c,0x2e(a6)                | +014
        move.w  #0x0,0x2c(a6)                   | +01a
        bra.w   .L18d5ca                        | +020
        .global TaskHandler_18d586__L18d5aa
TaskHandler_18d586__L18d5aa:
        move.w  #0xf000,d0                      | +024
        jsr     0x5dca4.l                       | +028
        move.w  d0,0x28(a6)                     | +02e
        move.w  #0x663,0x2a(a6)                 | +032
        move.w  #0xfeb9,0x2e(a6)                | +038
        move.w  #0x0,0x2c(a6)                   | +03e
.L18d5ca:
        bset    #0x4,0x6b(a6)                   | +044
        move.w  #0xd000,0x38(a6)                | +04a
        lea     Data_18d3e2(pc),a0              | +050
        jsr     0x28cd4.l                       | +054
        lea     0xffff.w,a0                     | +05a
        move.l  a0,0x48(a6)                     | +05e
        lea     Data_18d152(pc),a0              | +062
        move.l  a0,0x4c(a6)                     | +066
        jsr     0x283ca.l                       | +06a
        jsr     0x283ca.l                       | +070
        movea.l 0xc(a6),a0                      | +076
        cmpa.l  #0x100440,a0                    | +07a
        beq.b   .L18d612                        | +080
        cmpa.l  #0x1004e0,a0                    | +082
        bne.w   .L18d616                        | +088
.L18d612:
        bra.w   .L18d61a                        | +08c
.L18d616:
        movea.l 0xc(a0),a0                      | +090
.L18d61a:
        cmpa.l  #0x100440,a0                    | +094
        bne.w   .L18d62c                        | +09a
        move.w  #0x7c,d1                        | +09e
        bra.w   .L18d630                        | +0a2
.L18d62c:
        move.w  #0x14c,d1                       | +0a6
.L18d630:
        jsr     0x236e.l                        | +0aa
        movea.l 0xc(a6),a0                      | +0b0
        move.b  0x3a(a0),0x3a(a6)               | +0b4
        move.w  0x22(a0),0x22(a6)               | +0ba
        move.w  0x24(a0),d0                     | +0c0
        addi.w  #0x20,d0                        | +0c4
        move.w  d0,0x82(a6)                     | +0c8
        move.w  d0,0x24(a6)                     | +0cc
        btst    #0x0,0x3a(a6)                   | +0d0
        beq.w   .L18d664                        | +0d6
        neg.w   0x28(a6)                        | +0da
.L18d664:
        lea     .L18d66a(pc),a1                 | +0de
        move.l  a1,(a6)                         | +0e2
.L18d66a:
        move.w  0x2a(a6),d0                     | +0e4
        bpl.w   .L18d67c                        | +0e8
        move.w  0x28(a6),d0                     | +0ec
        asr.w   #0x4,d0                         | +0f0
        sub.w   d0,0x28(a6)                     | +0f2
.L18d67c:
        jsr     0x27d50.l                       | +0f6
        bcs.w   TaskHandler_18d6f8              | +0fc
        jsr     0x28d70.l                       | +100
        jsr     0x283d8.l                       | +106
        btst    #0x1,0x13(a6)                   | +10c
        bne.w   TaskHandler_18d6f8__L18d714     | +112
        btst    #0x3,0x13(a6)                   | +116
        bne.w   TaskHandler_18d6f8__L18d706     | +11c
        cmpi.w  #0x100,0x24(a6)                 | +120
        bmi.w   JmpToScheduler_18d794           | +126
        cmpi.w  #0x150,0x22(a6)                 | +12a
        bcc.w   JmpToScheduler_18d794           | +130
        rts                                     | +134

| ----------------------------------------------------------------------------
|  TaskHandler_18d6bc  @ $18D6BC  (52 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_18d6bc, "ax", @progbits
        .global TaskHandler_18d6bc
TaskHandler_18d6bc:
        move.w  #0x1027,d0                      | +000
        jsr     0x2352.l                        | +004
        jsr     0x13600.l                       | +00a
        move.w  #0x178,d1                       | +010
        jsr     0x236e.l                        | +014
        lea     0x29e76c.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        lea     Data_18d29a(pc),a0              | +026
        move.l  a0,0x4c(a6)                     | +02a
        jsr     0x283ca.l                       | +02e

| ----------------------------------------------------------------------------
|  TaskHandler_18d6f8  @ $18D6F8  (78 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_18d6f8, "ax", @progbits
        .global TaskHandler_18d6f8
TaskHandler_18d6f8:
        move.w  #0x1027,d0                      | +000
        jsr     0x2352.l                        | +004
        bra.w   .L18d71e                        | +00a
        .global TaskHandler_18d6f8__L18d706
TaskHandler_18d6f8__L18d706:
        move.w  #0x1027,d0                      | +00e
        jsr     0x2352.l                        | +012
        bra.w   .L18d71e                        | +018
        .global TaskHandler_18d6f8__L18d714
TaskHandler_18d6f8__L18d714:
        move.w  #0x1027,d0                      | +01c
        jsr     0x2352.l                        | +020
.L18d71e:
        jsr     0x13600.l                       | +026
        move.w  #0xd,d1                         | +02c
        jsr     0x236e.l                        | +030
        lea     Data_18d52e(pc),a0              | +036
        jsr     0x28cd4.l                       | +03a
        lea     TaskHandler_18d74e(pc),a1       | +040
        move.l  a1,(a6)                         | +044
        lea     Data_18d1f6(pc),a0              | +046
        move.l  a0,0x4c(a6)                     | +04a

| ----------------------------------------------------------------------------
|  TaskHandler_18d74e  @ $18D74E  (24 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_18d74e, "ax", @progbits
        .global TaskHandler_18d74e
TaskHandler_18d74e:
        jsr     0x283ca.l                       | +000
        jsr     0x283d8.l                       | +006
        lea     TaskHandler_18d76e(pc),a1       | +00c
        move.l  a1,(a6)                         | +010
        jsr     0x2783a.l                       | +012

| ----------------------------------------------------------------------------
|  TaskHandler_18d76e  @ $18D76E  (38 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_18d76e, "ax", @progbits
        .global TaskHandler_18d76e
TaskHandler_18d76e:
        lea     0xffff.w,a0                     | +000
        move.l  a0,0x4c(a6)                     | +004
        jsr     0x283ca.l                       | +008
        jsr     0x2783a.l                       | +00e
        jsr     0x28d70.l                       | +014
        bcc.w   .L18d792                        | +01a
        jmp     0x518.l                         | +01e
.L18d792:
        rts                                     | +024

| ----------------------------------------------------------------------------
|  TaskHandler_18d7aa  @ $18D7AA  (204 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_18d7aa, "ax", @progbits
        .global TaskHandler_18d7aa
TaskHandler_18d7aa:
        bset    #0x4,0x6b(a6)                   | +000
        move.w  #0xd000,0x38(a6)                | +006
        lea     Data_18d3e2(pc),a0              | +00c
        jsr     0x28cd4.l                       | +010
        lea     0xffff.w,a0                     | +016
        move.l  a0,0x48(a6)                     | +01a
        lea     Data_18d152(pc),a0              | +01e
        move.l  a0,0x4c(a6)                     | +022
        jsr     0x283ca.l                       | +026
        jsr     0x283ca.l                       | +02c
        move.w  #0x7c,d1                        | +032
        jsr     0x236e.l                        | +036
        movea.l 0xc(a6),a0                      | +03c
        cmpa.l  #0x1008a0,a0                    | +040
        bne.w   .L18d7fe                        | +046
        addi.w  #0x20,0x24(a6)                  | +04a
        bra.w   .L18d814                        | +050
.L18d7fe:
        move.w  0x22(a0),0x22(a6)               | +054
        move.w  0x24(a0),d0                     | +05a
        addi.w  #0x20,d0                        | +05e
        move.w  d0,0x82(a6)                     | +062
        move.w  d0,0x24(a6)                     | +066
.L18d814:
        move.w  #0xfc00,d0                      | +06a
        jsr     0x5dca4.l                       | +06e
        move.w  d0,0x28(a6)                     | +074
        move.w  #0xf800,0x2a(a6)                | +078
        clr.w   0x2c(a6)                        | +07e
        clr.w   0x2e(a6)                        | +082
        lea     .L18d836(pc),a1                 | +086
        move.l  a1,(a6)                         | +08a
.L18d836:
        jsr     0x27d50.l                       | +08c
        bcs.w   TaskHandler_18d6f8              | +092
        jsr     0x28d70.l                       | +096
        jsr     0x283d8.l                       | +09c
        btst    #0x1,0x13(a6)                   | +0a2
        bne.w   TaskHandler_18d6f8__L18d714     | +0a8
        btst    #0x3,0x13(a6)                   | +0ac
        bne.w   TaskHandler_18d6f8__L18d706     | +0b2
        cmpi.w  #0x100,0x24(a6)                 | +0b6
        bmi.w   JmpToScheduler_18d794           | +0bc
        cmpi.w  #0x150,0x22(a6)                 | +0c0
        bcc.w   JmpToScheduler_18d794           | +0c6
        rts                                     | +0ca

| ----------------------------------------------------------------------------
|  TaskHandler_18d876  @ $18D876  (168 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_18d876, "ax", @progbits
        .global TaskHandler_18d876
TaskHandler_18d876:
        move.w  #0xfae2,d0                      | +000
        jsr     0x5dca4.l                       | +004
        move.w  d0,0x28(a6)                     | +00a
        move.w  #0xccb,0x2a(a6)                 | +00e
        move.w  #0xfefa,0x2e(a6)                | +014
        move.w  #0x0,0x2c(a6)                   | +01a
        bset    #0x4,0x6b(a6)                   | +020
        move.w  #0xd000,0x38(a6)                | +026
        lea     Data_18d3e2(pc),a0              | +02c
        jsr     0x28cd4.l                       | +030
        lea     0xffff.w,a0                     | +036
        move.l  a0,0x48(a6)                     | +03a
        lea     Data_18d152(pc),a0              | +03e
        move.l  a0,0x4c(a6)                     | +042
        jsr     0x283ca.l                       | +046
        jsr     0x283ca.l                       | +04c
        move.w  #0x14c,d1                       | +052
        jsr     0x236e.l                        | +056
        move.w  #0x1,d1                         | +05c
        jsr     0x236e.l                        | +060
        move.w  #0x14c,d1                       | +066
        jsr     0x236e.l                        | +06a
        movea.l 0xc(a6),a0                      | +070
        cmpa.l  #0x1008a0,a0                    | +074
        bne.w   .L18d8fe                        | +07a
        addi.w  #0x10,0x24(a6)                  | +07e
        bra.w   .L18d914                        | +084
.L18d8fe:
        move.w  0x22(a0),0x22(a6)               | +088
        move.w  0x24(a0),d0                     | +08e
        addi.w  #0x10,d0                        | +092
        move.w  d0,0x82(a6)                     | +096
        move.w  d0,0x24(a6)                     | +09a
.L18d914:
        clr.w   0x5c(a6)                        | +09e
        lea     TaskHandler_18d91e(pc),a1       | +0a2
        move.l  a1,(a6)                         | +0a6

| ----------------------------------------------------------------------------
|  TaskHandler_18d91e  @ $18D91E  (182 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_18d91e, "ax", @progbits
        .global TaskHandler_18d91e
TaskHandler_18d91e:
        move.w  0x2a(a6),d0                     | +000
        cmpi.w  #0xff00,d0                      | +004
        bpl.w   .L18d934                        | +008
        move.w  0x28(a6),d0                     | +00c
        asr.w   #0x4,d0                         | +010
        sub.w   d0,0x28(a6)                     | +012
.L18d934:
        cmpi.b  #0x0,0x1081ae.l                 | +016
        bne.w   .L18d94a                        | +01e
        jsr     0x27d50.l                       | +022
        bra.w   .L18d950                        | +028
.L18d94a:
        jsr     0x27bc8.l                       | +02c
.L18d950:
        bcc.w   .L18d996                        | +032
        cmpi.w  #0x1,0x5c(a6)                   | +036
        bcs.w   .L18d966                        | +03c
        bra.w   TaskHandler_18d6bc              | +040
        bra.w   .L18d996                        | +044
.L18d966:
        move.w  #0xfce0,d0                      | +048
        jsr     0x5dca4.l                       | +04c
        move.w  d0,0x28(a6)                     | +052
        move.w  #0x3c0,0x2a(a6)                 | +056
        move.w  #0xff88,0x2e(a6)                | +05c
        move.w  #0x0,0x2c(a6)                   | +062
        addq.w  #0x8,0x24(a6)                   | +068
        addi.w  #0x1,0x5c(a6)                   | +06c
        lea     TaskHandler_18d91e(pc),a1       | +072
        move.l  a1,(a6)                         | +076
.L18d996:
        jsr     0x28d70.l                       | +078
        cmpi.b  #0x1,0x3b(a6)                   | +07e
        bcs.w   .L18d9b6                        | +084
        jsr     0x283d8.l                       | +088
        btst    #0x1,0x13(a6)                   | +08e
        bne.w   TaskHandler_18d6bc              | +094
.L18d9b6:
        btst    #0x3,0x13(a6)                   | +098
        bne.w   TaskHandler_18d6f8__L18d706     | +09e
        cmpi.w  #0x100,0x24(a6)                 | +0a2
        bmi.w   JmpToScheduler_18d794           | +0a8
        cmpi.w  #0x150,0x22(a6)                 | +0ac
        bcc.w   JmpToScheduler_18d794           | +0b2

| ----------------------------------------------------------------------------
|  TaskHandler_18d9dc  @ $18D9DC  (154 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_18d9dc, "ax", @progbits
        .global TaskHandler_18d9dc
TaskHandler_18d9dc:
        move.w  #0xfeab,d0                      | +000
        jsr     0x5dca4.l                       | +004
        move.w  d0,0x28(a6)                     | +00a
        move.w  #0x2a6,0x2a(a6)                 | +00e
        move.w  #0xff8f,0x2e(a6)                | +014
        move.w  #0x0,0x2c(a6)                   | +01a
        bset    #0x4,0x6b(a6)                   | +020
        move.w  #0xd000,0x38(a6)                | +026
        lea     Data_18d3e2(pc),a0              | +02c
        jsr     0x28cd4.l                       | +030
        lea     0xffff.w,a0                     | +036
        move.l  a0,0x48(a6)                     | +03a
        lea     Data_18d152(pc),a0              | +03e
        move.l  a0,0x4c(a6)                     | +042
        jsr     0x283ca.l                       | +046
        jsr     0x283ca.l                       | +04c
        move.w  #0x7c,d1                        | +052
        jsr     0x236e.l                        | +056
        movea.l 0xc(a6),a0                      | +05c
        cmpa.l  #0x1008a0,a0                    | +060
        bne.w   .L18da50                        | +066
        addi.w  #0x10,0x24(a6)                  | +06a
        bra.w   .L18da66                        | +070
.L18da50:
        move.w  0x22(a0),0x22(a6)               | +074
        move.w  0x24(a0),d0                     | +07a
        addi.w  #0x10,d0                        | +07e
        move.w  d0,0x82(a6)                     | +082
        move.w  d0,0x24(a6)                     | +086
.L18da66:
        move.w  #0x12,0x5c(a6)                  | +08a
        lea     TaskHandler_18daf4(pc),a1       | +090
        move.l  a1,(a6)                         | +094
        bra.w   TaskHandler_18daf4              | +096

| ----------------------------------------------------------------------------
|  TaskHandler_18da76  @ $18DA76  (126 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_18da76, "ax", @progbits
        .global TaskHandler_18da76
TaskHandler_18da76:
        move.w  0x2a(a6),d0                     | +000
        bpl.w   .L18da88                        | +004
        move.w  0x28(a6),d0                     | +008
        asr.w   #0x4,d0                         | +00c
        sub.w   d0,0x28(a6)                     | +00e
.L18da88:
        jsr     0x27bc8.l                       | +012
        bcc.w   .L18dabe                        | +018
        move.w  #0xff2b,d0                      | +01c
        jsr     0x5dca4.l                       | +020
        move.w  d0,0x28(a6)                     | +026
        move.w  #0x4fe,0x2a(a6)                 | +02a
        move.w  #0xff2b,0x2e(a6)                | +030
        move.w  #0x0,0x2c(a6)                   | +036
        move.w  #0x12,0x5c(a6)                  | +03c
        lea     TaskHandler_18daf4(pc),a1       | +042
        move.l  a1,(a6)                         | +046
.L18dabe:
        jsr     0x28d70.l                       | +048
        jsr     0x283d8.l                       | +04e
        btst    #0x1,0x13(a6)                   | +054
        bne.w   TaskHandler_18d6f8__L18d714     | +05a
        btst    #0x3,0x13(a6)                   | +05e
        bne.w   TaskHandler_18d6f8__L18d706     | +064
        cmpi.w  #0x100,0x24(a6)                 | +068
        bmi.w   JmpToScheduler_18d794           | +06e
        cmpi.w  #0x150,0x22(a6)                 | +072
        bcc.w   JmpToScheduler_18d794           | +078
        rts                                     | +07c

| ----------------------------------------------------------------------------
|  TaskHandler_18daf4  @ $18DAF4  (102 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_18daf4, "ax", @progbits
        .global TaskHandler_18daf4
TaskHandler_18daf4:
        move.w  0x2a(a6),d0                     | +000
        bpl.w   .L18db06                        | +004
        move.w  0x28(a6),d0                     | +008
        asr.w   #0x4,d0                         | +00c
        sub.w   d0,0x28(a6)                     | +00e
.L18db06:
        jsr     0x27d50.l                       | +012
        bcc.w   .L18db14                        | +018
        bra.w   TaskHandler_18d6f8__L18d714     | +01c
.L18db14:
        subi.w  #0x1,0x5c(a6)                   | +020
        bne.w   .L18db24                        | +026
        lea     TaskHandler_18da76(pc),a1       | +02a
        move.l  a1,(a6)                         | +02e
.L18db24:
        jsr     0x28d70.l                       | +030
        jsr     0x283d8.l                       | +036
        btst    #0x1,0x13(a6)                   | +03c
        bne.w   TaskHandler_18d6f8__L18d714     | +042
        btst    #0x3,0x13(a6)                   | +046
        bne.w   TaskHandler_18d6f8__L18d706     | +04c
        cmpi.w  #0x100,0x24(a6)                 | +050
        bmi.w   JmpToScheduler_18d794           | +056
        cmpi.w  #0x150,0x22(a6)                 | +05a
        bcc.w   JmpToScheduler_18d794           | +060
        rts                                     | +064
