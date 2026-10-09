#!/usr/bin/env python3
"""Comprueba que el tamaño registrado de cada entrada .s coincide con el tamaño
real de su sección .text.<Sym> tras ensamblar. Detecta las discrepancias que
producen solapes LMA en el enlace del matcher (p.ej. una función cuyo rts
final quedó fuera del registry y luego se re-registró como entrada aparte)."""
import os, subprocess, sys, tempfile, collections
sys.path.insert(0, os.path.dirname(__file__))
from registry import REGISTRY
AS = ["m68k-linux-gnu-as", "-m68000", "--register-prefix-optional"]
by_file = collections.defaultdict(dict)
for n, a, s, f in REGISTRY:
    if f.endswith(".s"): by_file[f][n] = (a, s)
bad = 0
with tempfile.TemporaryDirectory() as td:
    for f, ents in sorted(by_file.items()):
        src = os.path.join("asm", f)
        if not os.path.exists(src): continue
        o = os.path.join(td, f + ".o")
        if subprocess.run(AS + [src, "-o", o], capture_output=True).returncode: continue
        out = subprocess.run(["m68k-linux-gnu-objdump", "-h", o], capture_output=True, text=True).stdout
        for line in out.splitlines():
            p = line.split()
            if len(p) > 2 and p[1].startswith(".text."):
                name = p[1][6:]; size = int(p[2], 16)
                if name in ents and ents[name][1] != size:
                    print(f"[!] {f}: {name} registry={ents[name][1]} real={size} (@ ${ents[name][0]:06X})"); bad += 1
print(f"check_section_sizes: {bad} discrepancias")
sys.exit(1 if bad else 0)
