| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave ??? — (borrador)
|  Región: $0001CA..$000200  (54 B, 1 entradas, 1 huecos)
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
|  Entity_FindByKey4_0001ca  @ $0001CA  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_FindByKey4_0001ca, "ax", @progbits
        .global Entity_FindByKey4_0001ca
Entity_FindByKey4_0001ca:
        movea.l 0xa04(a5),a0                    | +000
        move.w  0xa08(a5),d7                    | +004
.L0001d2:
        move.w  (a0),d1                         | +008
        lsr.w   #0x8,d1                         | +00a
        cmpi.b  #0xff,d1                        | +00c
        beq.b   .L0001f6                        | +010
        move.w  (a0),d0                         | +012
        cmp.b   0xace(a5),d0                    | +014
        bne.b   .L0001f6                        | +018
        move.w  0x2(a0),d0                      | +01a
        lsr.w   #0x8,d0                         | +01e
        cmp.b   0xacf(a5),d0                    | +020
        bne.b   .L0001f6                        | +024
        cmp.b   0xad0(a5),d1                    | +026
        beq.b   .L0001fe                        | +02a
.L0001f6:
        addq.l  #0x4,a0                         | +02c
        dbra    d7,.L0001d2                     | +02e
        move.w  d7,d3                           | +032
.L0001fe:
        rts                                     | +034
