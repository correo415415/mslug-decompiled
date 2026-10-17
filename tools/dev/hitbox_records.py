#!/usr/bin/env python3
# Censo/decodificador de registros de hitbox de $50 bytes (listas +$48/+$4C).
#   python3 tools/dev/hitbox_records.py            -> resumen por módulo de datos
#   python3 tools/dev/hitbox_records.py 29605E ... -> decodifica la lista (dir. CPU)
# Formato (ver asm/entity/collmap_hitbox_script_ops_027400.s, Hitbox_RunList /
# Hitbox_TestBoxes): +0 op/grupo (0..4; $FF = fin de lista +$4C), +1 tipo,
# +2 daño (w), +4 fuerza, +5 forma (0 rect / 4 punto, índice de
# Hitbox_ShapeTestTable), +6 long (−1 en listas de ataque; en listas +$48 el
# long +2 es la tabla de formas), +$A x0,x1,y0,y1 (w, relativos a $22/$24),
# +$12 guion de sprite de 62 B: 5 frames op 02 (tiles $36xx en las esquinas)
# + op 1D — dibujo de la caja. Familias por tile: $363A ataque (+$4C), $366C
# golpe (+$48); cajas de cuerpo +$60 = $4C B con cabecera $80 (tile $3608).
import os, struct, sys
ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
d = open(os.path.join(ROOT, "build", "mslug_prom.bin"), "rb").read()
off = lambda a: a - 0x100000 if a >= 0x200000 else a   # CPU -> fichero

def isrec(o):
    if o + 0x50 > len(d) or d[o] > 4:
        return False
    s = d[o + 0x12:o + 0x50]
    return all(s[i] == 2 for i in range(0, 60, 12)) and s[60] == 0x1D

def show(o):
    while isrec(o):
        op, ty, dmg, st, sh = d[o], d[o+1], struct.unpack(">H", d[o+2:o+4])[0], d[o+4], d[o+5]
        x0, x1, y0, y1 = struct.unpack(">hhhh", d[o+0xA:o+0x12])
        tiles = [struct.unpack(">H", d[o+0x12+i+4:o+0x12+i+6])[0] for i in range(0, 60, 12)]
        print("  $%06X op%d tipo $%02X daño $%04X fuerza %d forma %d  x[%d,%d] y[%d,%d]  tiles %s"
              % (o if o < 0x100000 else o + 0x100000, op, ty, dmg, st, sh, x0, x1, y0, y1,
                 "/".join("%04X" % t for t in tiles)))
        o += 0x50
    print("  fin: %s" % d[o:o+4].hex())

if len(sys.argv) > 1:
    for a in sys.argv[1:]:
        print("Lista $%s" % a.upper()); show(off(int(a, 16)))
    sys.exit()

sys.path.insert(0, os.path.join(ROOT, "tools"))
from registry import REGISTRY as R  # noqa
recs = [o for o in range(0, len(d) - 0x50, 2) if isrec(o)]
by = {}
for o in recs:
    for name, a, n, f in R:
        if a <= o < a + n:
            by.setdefault((name, f), []).append(o); break
    else:
        by.setdefault(("?", "?"), []).append(o)
print("registros $50: %d (%d B)" % (len(recs), len(recs) * 0x50))
for (name, f), v in sorted(by.items(), key=lambda kv: -len(kv[1])):
    print("  %4d  %-40s %s" % (len(v), name, f))
