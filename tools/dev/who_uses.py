#!/usr/bin/env python3
# Herramienta de sesión: para cada racha "otro" grande (de other_runs.py),
# lista las instrucciones asm que cargan una dirección dentro de ella
# (literal hex o Sym+0x100000).   python3 tools/dev/who_uses.py [N]
import os, re, subprocess, sys
ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
n = next((a for a in sys.argv[1:] if a.isdigit()), "20")
out = subprocess.run([sys.executable, os.path.join(ROOT, "tools/dev/other_runs.py"), n],
                     capture_output=True, text=True).stdout.splitlines()
runs = []
for l in out[1:]:
    m = re.search(r"\$([0-9A-F]{6})-\$([0-9A-F]{6})", l)
    if m:
        runs.append((int(m.group(1), 16), int(m.group(2), 16), l))
cpu = lambda o: o + 0x100000 if o >= 0x100000 else o
lines = []
for dp, _, fs in os.walk(os.path.join(ROOT, "asm")):
    for f in fs:
        if f.endswith(".s"):
            path = os.path.join(dp, f)
            for i, l in enumerate(open(path, errors="replace"), 1):
                lines.append((os.path.relpath(path, ROOT), i, l.rstrip()))
HEX = re.compile(r"0x([0-9a-fA-F]{5,6})\b")
for lo, hi, desc in runs:
    a, b = cpu(lo) - 0x40, cpu(hi) + 0x10
    print(desc)
    k = 0
    for path, i, l in lines:
        if l.lstrip().startswith((".dc", "|", "#", ";")):
            continue
        for m in HEX.finditer(l):
            v = int(m.group(1), 16)
            if a <= v < b:
                print("   %s:%d  %s" % (path, i, l.strip()[:90]))
                k += 1
                break
        if k >= 12:
            break
