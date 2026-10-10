#!/usr/bin/env python3
"""Comprueba que el tamaño registrado de cada entrada (.s y .c) coincide con el tamaño
real de su sección .text.<Sym> tras ensamblar/compilar. Detecta las discrepancias que
producen solapes LMA en el enlace del matcher (p.ej. una función cuyo rts
final quedó fuera del registry y luego se re-registró como entrada aparte)."""
import os, subprocess, sys, tempfile, collections
sys.path.insert(0, os.path.dirname(__file__))
from registry import REGISTRY
import importlib.util
_mb = importlib.util.spec_from_file_location("mb", os.path.join(os.path.dirname(__file__), "match_batch.py"))
mb = importlib.util.module_from_spec(_mb); _mb.loader.exec_module(mb)   # mismos CFLAGS que el matcher
AS = ["m68k-linux-gnu-as", "-m68000", "--register-prefix-optional"]
by_file = collections.defaultdict(dict)
for n, a, s, f in REGISTRY:
    if f.endswith(".s") or f.endswith(".c"): by_file[f][n] = (a, s)
bad = 0
with tempfile.TemporaryDirectory() as td:
    for f, ents in sorted(by_file.items()):
        is_c = f.endswith(".c")
        src = os.path.join("src" if is_c else "asm", f)
        if not os.path.exists(src): continue
        o = os.path.join(td, f + ".o")
        cmd = (["m68k-linux-gnu-gcc", *mb.CFLAGS, *mb.PER_FILE_CFLAGS.get(f, []), "-c"] if is_c else AS) + [src, "-o", o]
        if subprocess.run(cmd, capture_output=True).returncode: continue
        out = subprocess.run(["m68k-linux-gnu-objdump", "-h", o], capture_output=True, text=True).stdout
        for line in out.splitlines():
            p = line.split()
            if len(p) > 2 and p[1].startswith(".text."):
                name = p[1][6:]; size = int(p[2], 16)
                if name in ents and ents[name][1] != size:
                    a0, rs = ents[name]
                    if is_c and size > rs:
                        # islas C: el rts/cola final queda fuera del registry por
                        # diseño (se expone como defsym *Rts_xxx). Solo es error si
                        # otra entrada del registry empieza dentro de esa cola.
                        clash = [n2 for n2, a2, s2, f2 in REGISTRY if a0 + rs <= a2 < a0 + size]
                        if not clash: continue
                        print(f"[!] {f}: {name} registry={rs} real={size} (@ ${a0:06X}) solapa con {', '.join(clash)}"); bad += 1
                        continue
                    print(f"[!] {f}: {name} registry={rs} real={size} (@ ${a0:06X})"); bad += 1
print(f"check_section_sizes: {bad} discrepancias")
sys.exit(1 if bad else 0)
