#!/usr/bin/env python3
"""Aplica la salida de gen_asm_region.py a registry.py / symbols.py.

Uso: python3 tools/wave_apply.py OUT.txt SRC.s "Wave X" "descripcion corta"

  * añade el bloque de entradas ("...") al final de REGISTRY;
  * sustituye en SYMBOLS los defsyms cuya direccion ahora esta en el registry
    por un comentario "promovido a <Name>";
  * añade los RTS mid-isla y las refs forward reportadas por el generador;
  * escribe en /tmp/<wave>_ren.txt los renombres (viejo nuevo) para
    aplicar a src/*.c y asm/*.s.
"""
import re, sys
out_path, src, wave, desc = sys.argv[1:5]
out = open(out_path).read()
new = [l for l in out.split('\n') if l.startswith('    ("')]
assert new, "sin entradas en la salida"
r = 'tools/registry.py'; t = open(r).read().rstrip('\n'); idx = t.rfind('\n]')
t = t[:idx] + f'\n    # --- {wave}: {desc} ({len(new)} entradas)\n' + '\n'.join(new) + t[idx:] + '\n'
open(r, 'w').write(t)
names_by_addr = {int(m.group(2), 16): m.group(1) for m in
                 re.finditer(r'\("(\w+)",\s*0x([0-9A-Fa-f]+),\s*\d+,\s*"' + re.escape(src) + r'"\)', t)}
s = open('tools/symbols.py').read(); outl = []; ren = {}; prom = 0
for l in s.split('\n'):
    m = re.match(r'\s*0x([0-9A-Fa-f]{8}):\s*"(\w+)"', l)
    if m and int(m.group(1), 16) in names_by_addr:
        a = int(m.group(1), 16); nw = names_by_addr[a]
        if m.group(2) != nw: ren[m.group(2)] = nw
        outl.append(f'    # 0x{a:08X} promovido a {nw} en registry ({wave}).'); prom += 1
    else:
        outl.append(l)
s = '\n'.join(outl)
extra = ''
m = re.search(r'\[i\] \d+ RTS internos.*?\n((?:    0x.*\n)+)', out)
if m: extra += f'\n    # --- {wave}: RTS internos de islas C\n' + m.group(1).rstrip('\n')
m = re.search(r'\[i\] \d+ refs forward.*?\n((?:    0x.*\n)+)', out)
if m: extra += f'\n    # --- {wave}: refs forward a huecos futuros\n' + m.group(1).rstrip('\n')
idx = s.rfind('\n}'); s = s[:idx] + extra + s[idx:]
open('tools/symbols.py', 'w').write(s)
tag = wave.split()[-1].lower()
open(f'/tmp/{tag}_ren.txt', 'w').write('\n'.join(f'{k} {v}' for k, v in ren.items()))
print(f'{len(new)} entradas añadidas, {prom} defsyms promovidos, {len(ren)} renombres -> /tmp/{tag}_ren.txt')
