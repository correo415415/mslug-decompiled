#!/usr/bin/env python3
# Trayectoria real de la cámara a partir de un watch.txt de pydmg con
# --watch-ram "106F50:4,106F54:4,106ECE:1" ($106F50 = X 16.16, $106F54 = Y 16.16,
# $106ECE = escena). Muestra un punto cada vez que la cámara se ha movido
# `step` px (en X o en Y: hay misiones de scroll vertical) desde el anterior.
#   python3 tools/dev/camera_log.py build/cam1/watch.txt [escena_hex] [step]
import re, sys
path = sys.argv[1]
scene = sys.argv[2].upper() if len(sys.argv) > 2 else None
step = int(sys.argv[3]) if len(sys.argv) > 3 else 64
pat = re.compile(r"f=(\d+)\s+\$106F50=(\w{8})\s+\$106F54=(\w{8})\s+\$106ECE=(\w\w)")
last = {}
for line in open(path):
    m = pat.search(line)
    if not m:
        continue
    f, x, y, sc = int(m[1]), int(m[2][:4], 16), int(m[3][:4], 16), m[4].upper()
    if (scene and sc != scene) or (x, y) == (0xFFFF, 0xFFFF) or x == 0x6F50:
        continue
    p = last.get(sc)
    if p is None or max(abs(x - p[0]), abs(y - p[1])) >= step:
        last[sc] = (x, y)
        print(f"escena {sc}  x={x:5d} (${x:04X})  y={y:4d} (${y:04X})  frame {f}")
