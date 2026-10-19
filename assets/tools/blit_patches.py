"""blit_patches.py - parches de tilemap de los fondos (puertas, restos...).

Extraído de `assets/tools/data_census.py` de la rama de la decompilación
(sólo las dos funciones que usa `levels.py`; el censo completo depende del
registro de la decompilación, que no está en el port).
"""


def _cells_ok(p, a, h):
    """Una columna de h celdas {tile.w, attr.w} plausible: byte bajo del attr
    sólo con bits de tile_msb (b4) / flip (b0..b1) en >= 95 % de las celdas."""
    return sum(1 for r in range(h) if p.B(a + 4 * r + 3) & 0xEC == 0) >= 0.95 * h


def blit_patches(p, code_end=0x9C608):
    """-> [(rec, desc, w, h, tiles, ncol)] de los registros de 20 B que
    Sprite_InvokeBlit8Params pasa a CellCommit_MMIO_051DE2 (a0 = ent, a1 =
    desc, d0..d5 = col_dest, fila_dest, col_origen, fila_origen, ancho, alto).
    ncol = máxima columna de origen copiada (col_origen + ancho), ampliada
    mientras las columnas siguientes del descriptor sigan siendo celdas
    válidas (el w de algunos descriptores excede los datos reales)."""
    recs = set()
    for o in range(0, code_end, 2):
        if p.W(o) != 0x45F9:
            continue
        x = p.W(o + 6)
        if (x == 0x4EB9 and p.L(o + 8) == 0x5022A or
                x in (0x4EBA, 0x6100) and o + 8 + p.SW(o + 8) == 0x5022A):
            recs.add(p.L(o + 2))
        # Prop_BlitSequence_04f1f4: `lea X,a2; move.l a2,$70/$74(a0)` (registros
        # de parche que la tarea hija aplica en secuencia)
        elif x == 0x214A and p.W(o + 8) in (0x70, 0x74):
            recs.add(p.L(o + 2))
    recs = {r for r in recs if r & 1 == 0 and p.valid(r, 20) and p.valid(p.L(r + 4), 8)}
    by_desc = {}
    for r in recs:
        by_desc.setdefault(p.L(r + 4), []).append(r)
    out = []
    for desc, rs in sorted(by_desc.items()):
        w, h, tiles = p.W(desc), p.W(desc + 2), p.L(desc + 4)
        ncol = min(w, max(p.W(r + 12) + p.W(r + 16) for r in rs))
        while ncol < w and _cells_ok(p, tiles + 4 * h * ncol, h):
            ncol += 1                       # columnas no copiadas pero válidas
        for r in rs:
            out.append((r, desc, w, h, tiles, ncol))
    return out
