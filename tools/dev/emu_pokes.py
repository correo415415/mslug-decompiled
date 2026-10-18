#!/usr/bin/env python3
# Pokes repetidos (el juego reescribe algunas variables): ADDR=HEX cada N frames.
#   python3 tools/dev/emu_pokes.py INICIO FIN PASO ADDR=HEX [ADDR=HEX ...]
import sys
a, b, s = map(int, sys.argv[1:4])
print(",".join("%d@%s" % (f, kv) for f in range(a, b, s) for kv in sys.argv[4:]))
