#!/usr/bin/env python3
"""
data_census.py - censo de los módulos de datos del P-ROM (asm/data/*.s).

Clasifica cada byte de las 14 regiones de datos según el tipo de asset que
el código del juego le da, usando los mismos decodificadores que los
extractores (sprites.py, levels.py, scene_script_dump.py):

  palette     paletas de 64 B ($14E00 x1024, $1CE00 x512)
  frame       frames de sprite {npiezas, attr, piezas{dx,dy,w,h,tiles}}
  anim        bytecode de animación (op 02 y demás ops recorridos)
  tilemap     mapas de celdas de fondo {tile.w, attr.w} de los bind_path
  collision   mapas de colisión de los bind_path (4 B por celda, w*h celdas)
  lut         tablas generadas por fórmula (ColorRGB555ToNeoGeo, ColorRamp)
  zero        rachas de $00 >= 16 B
  other       resto (tablas de código, parámetros, strings, etc.)

    python3 assets/tools/data_census.py            # tabla por módulo
    python3 assets/tools/data_census.py --json F   # además vuelca a JSON
"""
import json, os, sys
from collections import Counter

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, HERE)
sys.path.insert(0, os.path.join(ROOT, "tools"))
import sprites as S          # noqa: E402
import levels as LV          # noqa: E402
import registry              # noqa: E402

KINDS = ("palette", "frame", "anim", "tilemap", "collision", "lut", "zero", "other")

# tablas de color del motor de paletas (asm/data/sprite_tables_002f30.s)
LUTS = ((0x002F30, 0x012F30), (0x012F30, 0x012FB0), (0x012FB0, 0x0133B0))


def modules():
    m = {}
    for name, a, n, f in registry.REGISTRY:
        if f.startswith("data/"):
            lo, hi = m.get(f, (1 << 30, 0))
            m[f] = (min(lo, a), max(hi, a + n))
    return sorted(m.items(), key=lambda t: t[1][0])


def classify(p):
    """-> dict cpu_addr -> kind (sólo bytes marcados)."""
    mark = {}

    def put(a, n, k):
        for x in range(a, a + n):
            mark.setdefault(x, k)

    # tablas por fórmula
    for lo, hi in LUTS:
        put(lo, hi - lo, "lut")
    # paletas
    put(S.PAL_LO, 1024 * 64, "palette")
    put(S.PAL_HI, 512 * 64, "palette")
    # tilemaps de fondo
    m = LV.load_ssd()
    for i in range(m.N_DESC):
        for L in LV.scene_layers(m, i):
            put(L["tiles"], 4 * L["w"] * L["h"], "tilemap")
            if L["collision"]:
                put(L["collision"], 4 * L["w"] * L["h"], "collision")
    # frames + scripts de animación
    ops, frames = S.find_frames(p)
    for fa in frames:
        fr = S.parse_frame(p, fa)
        if fr:
            put(fa, fr["size"], "frame")
    scripts, _ = S.find_scripts(p, ops)
    for s, (pics, info) in scripts.items():
        for a in info["visited"]:
            op = p.B(a)
            n = ops[a][4] if a in ops else S.OP_SIZE.get(op, 2)
            if op == 0x01:
                n = 6
            put(a, n, "anim")
    return mark


def census(p, mark):
    rows = []
    for f, (lo, hi) in modules():
        c = Counter()
        zrun = 0
        for o in range(lo, hi):            # registry = offsets del fichero de 2 MiB
            k = mark.get(p.cpu(o))
            if k is None:
                if p.d[o] == 0:
                    zrun += 1
                    continue
                k = "other"
            if zrun:
                c["zero" if zrun >= 16 else "other"] += zrun
                zrun = 0
            c[k] += 1
        if zrun:
            c["zero" if zrun >= 16 else "other"] += zrun
        rows.append((f, lo, hi, c))
    return rows


def main():
    p = S.PRom()
    rows = census(p, classify(p))
    hdr = "%-34s %-17s %8s " % ("módulo", "región", "bytes") + " ".join("%9s" % k for k in KINDS)
    print(hdr)
    print("-" * len(hdr))
    tot = Counter()
    for f, lo, hi, c in rows:
        tot.update(c)
        print("%-34s $%06X-$%06X %8d " % (f, lo, hi, hi - lo)
              + " ".join("%8.1f%%" % (100 * c[k] / (hi - lo)) for k in KINDS))
    n = sum(tot.values())
    print("%-34s %-17s %8d " % ("TOTAL", "", n) + " ".join("%8.1f%%" % (100 * tot[k] / n) for k in KINDS))
    if "--json" in sys.argv:
        out = sys.argv[sys.argv.index("--json") + 1]
        json.dump([dict(module=f, start="%06X" % lo, end="%06X" % hi,
                        bytes={k: c[k] for k in KINDS}) for f, lo, hi, c in rows],
                  open(out, "w"), indent=1)


if __name__ == "__main__":
    main()
