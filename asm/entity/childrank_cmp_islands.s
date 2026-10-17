| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Comparadores ChildRank_CmpByte10 incrustados al final de módulos de datos
|  Regiones: $024E00..$024E10, $0967A4..$0967B4  (16 B cada una)
| ============================================================================
|
|  A) RESUMEN
|  ----------
|  Copias byte-idénticas del comparador CCR
|      movea.l $8(a6),a1 ; move.b $10(a6),d0 ; cmp.b $10(a1),d0 ; bcs SetXN
|  que caen en la pareja ClearXN/SetXN que les sigue (thunks/ccr_helpers.c).
|  C=1 si el rango (+$10) de la entidad es menor que el de su hija (+$08).
|  Sin callers: epílogo que el ensamblador original pegaba tras ciertos
|  bloques. Hay más copias en $05279E y $097720 (ya en ficheros de código).
|
|  B) EVIDENCIA
|  -----------
|  Los 16 bytes coinciden con las >100 copias del patrón en el banco de
|  código; antes estaban como Data_024e00 (sparse_tables_013d6a.s) y dentro
|  de scene_scripts_0916c8.s, y el censo los contaba como "otro".
| ============================================================================

        .text

| ----------------------------------------------------------------------------
|  ChildRank_CmpByte10_024e00  @ $024E00  (16 B)
| ----------------------------------------------------------------------------
        .section .text.ChildRank_CmpByte10_024e00, "ax", @progbits
        .global ChildRank_CmpByte10_024e00
ChildRank_CmpByte10_024e00:
        movea.l 0x8(a6), a1                    | +00  a1 = entidad hija (+$08)
        move.b  0x10(a6), d0                   | +04  d0 = rango propio
        cmp.b   0x10(a1), d0                   | +08  vs rango de la hija
        bcs.w   SetXN_024e16                   | +0c  menor -> C=1
                                               | +10  cae en ClearXN_024e10

| ----------------------------------------------------------------------------
|  ChildRank_CmpByte10_0967A4  @ $0967A4  (16 B)
| ----------------------------------------------------------------------------
        .section .text.ChildRank_CmpByte10_0967A4, "ax", @progbits
        .global ChildRank_CmpByte10_0967A4
ChildRank_CmpByte10_0967A4:
        movea.l 0x8(a6), a1                    | +00  a1 = entidad hija (+$08)
        move.b  0x10(a6), d0                   | +04  d0 = rango propio
        cmp.b   0x10(a1), d0                   | +08  vs rango de la hija
        bcs.w   SetXN_0967ba                   | +0c  menor -> C=1
                                               | +10  cae en ClearXN_0967b4
