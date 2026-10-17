#!/usr/bin/env python3
# Herramienta de sesión: lista las rachas de bytes "otro" del censo más grandes
# (cortando en rachas de ceros >= 16, como el censo). Cachea las marcas en
# /tmp/census_mark.pkl; -f recalcula.   python3 tools/dev/other_runs.py [-f] [N]
import os, pickle, sys
ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path[:0] = [os.path.join(ROOT, "assets", "tools"), os.path.join(ROOT, "tools")]
import sprites as S, data_census as D   # noqa: E402
CACHE = "/tmp/census_mark.pkl"
p = S.PRom()
if os.path.exists(CACHE) and "-f" not in sys.argv:
    mark = pickle.load(open(CACHE, "rb"))
else:
    mark = D.classify(p)
    pickle.dump(mark, open(CACHE, "wb"))
runs = []
for f, (lo, hi) in D.modules():
    o = lo
    while o < hi:
        if mark.get(p.cpu(o)) is None and p.d[o] != 0:
            s = e = o
            z = 0
            while o < hi and mark.get(p.cpu(o)) is None:
                if p.d[o] == 0:
                    z += 1
                    if z >= 16:
                        break
                else:
                    z, e = 0, o + 1
                o += 1
            runs.append((e - s, s, e, f))
        else:
            o += 1
runs.sort(reverse=True)
print("total", sum(r[0] for r in runs), "rachas", len(runs))
n = next((int(a) for a in sys.argv[1:] if a.isdigit()), 40)
for r in runs[:n]:
    print("%6d $%06X-$%06X %s" % r)
