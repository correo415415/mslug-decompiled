#!/usr/bin/env python3
"""apply_reorg.py — Ejecuta la reorganización descrita en reorg_map.txt.

1. git mv asm/<old> asm/<new>  (y src/)
2. Reescribe toda mención textual del nombre antiguo ('asm/old.s' -> 'asm/new.s',
   'old.s' -> 'new.s' como palabra completa) en asm/**, src/**, include/**, tools/*.py,
   scripts/**, docs/**, README/CHANGELOG/CONTRIBUTING, Makefile, .github/**, internal/experiments.
   Esto incluye el campo fichero del REGISTRY (tools/registry.py) y las claves de
   PER_FILE_CFLAGS (tools/match_batch.py).
Idempotente: si asm/<old> ya no existe, se salta el mv.
"""
import os, re, subprocess
ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
os.chdir(ROOT)

MAP = []
for l in open("tools/reorg/reorg_map.txt"):
    l = l.split("#")[0].strip()
    if l:
        a, b = l.split(); MAP.append((a, b))

def top(name): return "src" if name.endswith(".c") else "asm"

moved = 0
for old, new in MAP:
    d = top(old); src = f"{d}/{old}"; dst = f"{d}/{new}"
    if os.path.exists(src):
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        subprocess.run(["git", "mv", src, dst], check=True); moved += 1
print(f"[reorg] movidos {moved}/{len(MAP)} ficheros")

files = []
for base in ("asm", "src", "include", "tools", "scripts", "docs", ".github", "internal/experiments"):
    for dp, dn, fn in os.walk(base):
        if "asm-differ" in dp or "__pycache__" in dp: continue
        for f in fn:
            if f.endswith((".s", ".c", ".h", ".py", ".sh", ".md", ".txt", ".yml", ".ld")):
                files.append(os.path.join(dp, f))
files += ["README.md", "CHANGELOG.md", "CONTRIBUTING.md", "Makefile", "linker/neogeo.ld", "linker/neogeo_matched.ld"]
files = [f for f in files if os.path.exists(f) and f != "tools/reorg/reorg_map.txt"]

pairs = sorted(MAP, key=lambda p: -len(p[0]))
rx_full = {o: re.compile(r"(?<![\w/])" + re.escape(top(o) + "/" + o) + r"(?![\w])") for o, _ in pairs}
rx_bare = {o: re.compile(r"(?<![\w/.-])" + re.escape(o) + r"(?![\w])") for o, _ in pairs}
changed = 0
for f in files:
    try: s = open(f, encoding="utf-8").read()
    except UnicodeDecodeError: continue
    o = s
    for old, new in pairs:
        if old not in s: continue
        s = rx_full[old].sub(top(old) + "/" + new, s)
        s = rx_bare[old].sub(new, s)
    if s != o:
        open(f, "w", encoding="utf-8").write(s); changed += 1
print(f"[reorg] {changed} ficheros con referencias reescritas")
