#!/usr/bin/env python3
# --input-script de "avanzar jugando": derecha + A alterno cada 6 frames
# (mantener A no dispara) + salto (B) 6 de cada 48 frames.
#   python3 tools/dev/emu_input.py INICIO FIN [mascara_extra]
import sys
a, b = int(sys.argv[1]), int(sys.argv[2])
extra = int(sys.argv[3]) if len(sys.argv) > 3 else 0
out = []
for f in range(a, b, 6):
    m = 8 | extra
    if (f // 6) % 2:
        m |= 16
    if (f // 6) % 8 == 0:
        m |= 32
    out.append("%d-%d:%d" % (f, f + 6, m))
print(",".join(out))
