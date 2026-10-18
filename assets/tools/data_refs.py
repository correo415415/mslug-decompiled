#!/usr/bin/env python3
"""
data_refs.py - qué código usa los bytes de datos que el censo aún no clasifica.

Lista los bloques "otro" de asm/data/* (bytes que data_census.classify no
marca), ordenados por tamaño, con las rutinas que los referencian:
  * `lea abs.l / d16(pc),aN`
  * `move.l #abs,…` (>= $20000, para no confundir constantes de VRAM)
y la siguiente llamada (jsr/bsr <= 12 B después) que recibe el registro, que
suele indicar el formato de la tabla.

    python3 assets/tools/data_refs.py            # 30 bloques mayores
    python3 assets/tools/data_refs.py -n 100
    python3 assets/tools/data_refs.py --callee   # bytes por rutina receptora
"""
import argparse, collections, os, pickle, sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, HERE)
sys.path.insert(0, os.path.join(ROOT, "tools"))
import sprites as S          # noqa: E402
import data_census as D      # noqa: E402
import registry as R         # noqa: E402
import symbols as SY         # noqa: E402

CACHE = os.path.join(ROOT, "build", "assets", "census_mark.pkl")


def load_mark(p, fresh=False):
    """Marcas del censo, cacheadas en build/assets (se invalidan si cambia el
    código del censo o de sprites.py)."""
    src = max(os.path.getmtime(os.path.join(HERE, f)) for f in ("data_census.py", "sprites.py"))
    if not fresh and os.path.exists(CACHE) and os.path.getmtime(CACHE) > src:
        return pickle.load(open(CACHE, "rb"))
    mark = D.classify(p)
    os.makedirs(os.path.dirname(CACHE), exist_ok=True)
    pickle.dump(mark, open(CACHE, "wb"))
    return mark


def other_blocks(p, mark, min_nz=8):
    """-> [(lo, hi, bytes_no_cero)] (offsets) de rachas de bytes sin clasificar."""
    out, cur = [], None
    for f, (lo, hi) in D.modules():
        for o in range(lo, hi):
            if mark.get(p.cpu(o)) is None:
                if cur and cur[1] == o:
                    cur[1] = o + 1
                else:
                    cur = [o, o + 1]
                    out.append(cur)
    res = [(a, b, sum(1 for x in range(a, b) if p.d[x])) for a, b in out]
    return sorted((r for r in res if r[2] >= min_nz), key=lambda r: -r[2])


def code_refs(p, mark):
    """-> {cpu_addr: [(pc, rutina, reg, callee)]} para destinos sin clasificar."""
    mods = [(lo, hi) for f, (lo, hi) in D.modules()]
    code = sorted((a, a + n, nm) for nm, a, n, f in R.REGISTRY
                  if not f.startswith("data/") and a + n <= 0x100000)   # código: < $100000
    refs = collections.defaultdict(list)
    for s, e, nm in code:
        for o in range(s, e - 4, 2):
            w = p.W(o)
            if (w & 0xF1FF) == 0x41F9:
                v, ln, reg = p.L(o + 2), 6, "a%d" % ((w >> 9) & 7)
            elif (w & 0xF1FF) == 0x41FA:
                v, ln, reg = o + 2 + p.SW(o + 2), 4, "a%d" % ((w >> 9) & 7)
            elif w in (0x2D7C, 0x217C, 0x2B7C):
                v, ln, reg = p.L(o + 2), 8, "$%X(a%d)" % (p.W(o + 6), w >> 9 & 7)
                if v < 0x20000:
                    continue
            elif w == 0x23FC:
                v, ln, reg = p.L(o + 2), 10, "$%06X" % p.L(o + 6)
                if v < 0x20000:
                    continue
            else:
                continue
            if not p.valid(v):
                continue
            vo = p.off(v)
            if not any(lo <= vo < hi for lo, hi in mods) or mark.get(v) is not None or not p.d[vo]:
                continue
            callee = None
            for q in range(o + ln, o + ln + 12, 2):
                x = p.W(q)
                if x in (0x4EB9, 0x4EF9):
                    callee = p.L(q + 2)
                    break
                if x in (0x4EBA, 0x4EFA, 0x6100):
                    callee = q + 2 + p.SW(q + 2)
                    break
            refs[v].append((o, nm, reg, callee))
    return refs


def names():
    n = dict(SY.SYMBOLS)
    n.update({a: s for s, a, sz, f in R.REGISTRY})
    return n


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("-n", type=int, default=30)
    ap.add_argument("--callee", action="store_true")
    ap.add_argument("--fresh", action="store_true")
    a = ap.parse_args()
    p = S.PRom()
    mark = load_mark(p, a.fresh)
    blocks = other_blocks(p, mark)
    refs = code_refs(p, mark)
    nm = names()
    print("bloques 'otro': %d, %d B no cero" % (len(blocks), sum(b[2] for b in blocks)))
    if a.callee:
        by, cnt = collections.Counter(), collections.Counter()
        starts = sorted((lo, hi, n) for lo, hi, n in blocks)
        los = [s[0] for s in starts]
        import bisect
        for v, rs in refs.items():
            i = bisect.bisect_right(los, p.off(v)) - 1
            n = starts[i][2] if i >= 0 and p.off(v) < starts[i][1] else 0
            for reg, cal in {(r[2], r[3]) for r in rs}:
                by[(reg, cal)] += n
                cnt[(reg, cal)] += 1
        for (reg, cal), b in by.most_common(a.n):
            print("%7d B %4d refs  %-10s -> %s" % (b, cnt[(reg, cal)], reg,
                                                 nm.get(cal, "$%06X" % cal if cal else "—")))
        return
    for lo, hi, n in blocks[:a.n]:
        us = sorted({(r[1], r[2], nm.get(r[3], "$%06X" % r[3] if r[3] else "—"))
                     for v, rs in refs.items() if lo <= p.off(v) < hi for r in rs})
        print("$%06X-$%06X %6d B (%d no cero)" % (lo, hi, hi - lo, n))
        for u in us[:6]:
            print("      %s  %s -> %s" % u)
        if len(us) > 6:
            print("      … %d más" % (len(us) - 6))


if __name__ == "__main__":
    main()
