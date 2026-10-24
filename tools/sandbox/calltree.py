#!/usr/bin/env python3
"""Árbol de llamadas de una rutina de la ROM marcando lo ya portado.

Uso: python3 calltree.py HEX [profundidad]
Necesita /tmp/m68dis.py y /tmp/prom.bin (los deja setup.sh) y /tmp/ported.txt
(lo regenera este script desde crates/bridge/src/system.rs).
"""
import os, re, subprocess, sys

ROOT = os.environ.get('MSLUG_ROOT', os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..'))
src = open(os.path.join(ROOT, 'crates/bridge/src/system.rs')).read()
ported = {re.sub('^0+', '', e.replace('_', '').upper()) for e in re.findall(r'entry: 0x([0-9A-Fa-f_]+)', src)}
open('/tmp/ported.txt', 'w').write('\n'.join(sorted(ported)) + '\n')

def callees(a):
    out = subprocess.run(['python3', '/tmp/m68dis.py', hex(a), '400'], capture_output=True, text=True).stdout
    res = []
    for line in out.splitlines():
        m = re.match(r'([0-9A-F]+): (\S+)\s*(.*)', line)
        if not m:
            continue
        op, arg = m.group(2), m.group(3)
        if op.startswith(('jsr', 'bsr', 'jmp')):
            t = re.match(r'\$([0-9a-f]+)', arg)
            if t:
                res.append((op, int(t.group(1), 16)))
        if op in ('rts', 'rte') or (op.startswith('jmp') and '(' not in arg):
            break
    return res

seen = set()
def walk(a, depth, ind=0):
    for op, c in callees(a):
        h = '%X' % c
        st = 'OK' if h in ported else 'NO'
        if (c, ind) in seen:
            continue
        seen.add((c, ind))
        print('  ' * ind + f'{op} ${h} {st}')
        if st == 'NO' and depth > 1:
            walk(c, depth - 1, ind + 1)

walk(int(sys.argv[1], 16), int(sys.argv[2]) if len(sys.argv) > 2 else 2)
