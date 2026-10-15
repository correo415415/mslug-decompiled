#!/usr/bin/env python3
"""rename_data_by_census.py - renombra entradas Data_* según el censo de datos.

Una entrada `Data_XXXXXX` de asm/data/* cuyos bytes no cero son >= 90 % de un
solo tipo de assets/tools/data_census.py pasa a llevar el prefijo de ese tipo
(BgMap_, AnimScr_, SprFrame_, ColMap_, Pal_). Los tipos struct/lut no se
renombran (su nombre lo pone el módulo a mano). Sustituye el nombre con límites
de palabra en asm/, src/, include/, tools/*.py, docs y assets.

    python3 tools/rename_data_by_census.py [--dry-run]
"""
import argparse, collections, glob, os, re, sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(ROOT, "tools"))
sys.path.insert(0, os.path.join(ROOT, "assets", "tools"))
import registry as R          # noqa: E402
import sprites as S           # noqa: E402
import data_refs as DR        # noqa: E402

PFX = {"tilemap": "BgMap", "anim": "AnimScr", "frame": "SprFrame",
       "collision": "ColMap", "palette": "Pal"}


def plan(p, mark, thr=0.9):
    names = {n for n, a, s, f in R.REGISTRY}
    ren = {}
    for n, a, s, f in R.REGISTRY:
        if not n.startswith("Data_") or not f.startswith("data/"):
            continue
        k = collections.Counter(mark.get(p.cpu(o)) or ("zero" if p.d[o] == 0 else "other")
                                for o in range(a, a + s))
        nz = {x: v for x, v in k.items() if x != "zero"}
        if not nz:
            continue
        dom, v = max(nz.items(), key=lambda t: t[1])
        if v < thr * sum(nz.values()) or dom not in PFX:
            continue
        new = PFX[dom] + n[4:]
        if new in names:
            continue
        ren[n] = new
    return ren


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--dry-run", action="store_true")
    a = ap.parse_args()
    p = S.PRom()
    ren = plan(p, DR.load_mark(p))
    print("[rename] %d entradas: %s" % (len(ren), dict(collections.Counter(
        v.split("_")[0] for v in ren.values()))))
    if a.dry_run or not ren:
        return
    pat = re.compile(r"\b(" + "|".join(map(re.escape, ren)) + r")\b")
    files = set(glob.glob(os.path.join(ROOT, "asm", "**", "*.s"), recursive=True)
                + glob.glob(os.path.join(ROOT, "src", "**", "*.[ch]"), recursive=True)
                + glob.glob(os.path.join(ROOT, "include", "**", "*.h"), recursive=True)
                + glob.glob(os.path.join(ROOT, "tools", "*.py"))
                + glob.glob(os.path.join(ROOT, "docs", "*.md"))
                + glob.glob(os.path.join(ROOT, "*.md"))
                + glob.glob(os.path.join(ROOT, "assets", "**", "*.py"), recursive=True)
                + glob.glob(os.path.join(ROOT, "assets", "**", "*.md"), recursive=True)
                + glob.glob(os.path.join(ROOT, "assets", "map", "*.txt")))
    nf = 0
    for f in files:
        s = open(f).read()
        t = pat.sub(lambda m: ren[m.group(1)], s)
        if t != s:
            open(f, "w").write(t)
            nf += 1
    print("[rename] %d ficheros modificados" % nf)


if __name__ == "__main__":
    main()
