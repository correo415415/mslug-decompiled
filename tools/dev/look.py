#!/usr/bin/env python3
# Herramienta de sesión: volcado + referencias long a un tramo de la P-ROM.
#   python3 tools/dev/look.py 1F4712-1F4A12 [...]   (offsets de fichero)
import os, struct, sys
ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
d = open(os.path.join(ROOT, "build", "mslug_prom.bin"), "rb").read()
cpu = lambda o: o + 0x100000 if o >= 0x100000 else o
for arg in sys.argv[1:]:
    lo, hi = [int(x, 16) for x in arg.split("-")]
    r = [(o, struct.unpack(">I", d[o:o + 4])[0]) for o in range(0, len(d) - 4, 2)
         if cpu(lo) - 0x40 <= struct.unpack(">I", d[o:o + 4])[0] < cpu(hi)]
    print("=== $%06X-$%06X  refs:" % (lo, hi), " ".join("%06X->%06X" % x for x in r[:24]), len(r))
    a = lo & ~0xF
    for x in range(a - 0x20, min(hi, a + 0x100), 16):
        print("%06X " % x + " ".join("%04X" % struct.unpack(">H", d[x + i:x + i + 2])[0] for i in range(0, 16, 2)))
