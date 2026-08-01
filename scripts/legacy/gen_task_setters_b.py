#!/usr/bin/env python3
"""
Metal Slug 1 — Generador de la familia SetTaskB_<addr>.
=========================================================
Escanea el P ROM procesado buscando el patrón exacto:

    1D40  00XX  4E75      ; move.b d0, XX(fp) ; rts    (6 bytes)

y produce decomp/src/task_setters_b.c con una función C por sitio,
además de un fragmento imprimible para registry.py.

GCC 13 -Os emite estos 6 bytes desde:
    void f(void) { TASK_B(0xXX) = (u8)_d0_w; }
"""
import os, importlib.util

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
PROM = os.path.join(ROOT, "decomp", "build", "mslug_prom.bin")
OUT  = os.path.join(ROOT, "decomp", "src", "task_setters_b.c")
REG  = os.path.join(ROOT, "decomp", "tools", "registry.py")


def scan(prom):
    hits = []
    for i in range(0, len(prom) - 6, 2):
        if prom[i:i+2] == b'\x1D\x40' and prom[i+4:i+6] == b'\x4E\x75':
            off = (prom[i+2] << 8) | prom[i+3]
            hits.append((i, off))
    return hits


def load_covered():
    spec = importlib.util.spec_from_file_location("reg", REG)
    m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
    covered = set()
    for name, addr, size, src in m.REGISTRY:
        covered.update(range(addr, addr + size))
    return covered


HEADER = '''/*
 * Metal Slug 1 — Familia SetTaskB: setters `move.b d0, off(fp) ; rts`
 * =====================================================================
 * Funciones de 6 bytes que guardan el byte bajo de d0 en un offset
 * concreto de la entidad apuntada por fp (A6) y retornan. Son inlines
 * del compilador original del tipo:
 *
 *     void SetField_XX(u8 v) { fp->field_XX = v; }
 *
 * GCC 13 -Os reproduce el patrón exacto (1D40 00XX 4E75) desde el C
 * canónico `TASK_B(0x??) = (u8)_d0_w;`.
 *
 * ARCHIVO AUTO-GENERADO por decomp/tools/gen_task_setters_b.py.
 */

#include "mslug.h"

'''

def main():
    with open(PROM, 'rb') as f: prom = f.read()
    hits = scan(prom)
    covered = load_covered()
    safe = [(a, o) for a, o in hits
            if not any(k in covered for k in range(a, a + 6))]
    print(f"Encontrados {len(hits)} setters SetTaskB; sin solapes: {len(safe)}")

    with open(OUT, 'w') as f:
        f.write(HEADER)
        for addr, off in safe:
            f.write(f'__attribute__((section(".text.SetTaskB_{addr:06x}")))\n')
            f.write(f'void SetTaskB_{addr:06x}(void) '
                    f'{{ TASK_B(0x{off:02X}) = (u8)_d0_w; }}\n\n')
    print(f"Escrito {OUT}")

    return safe


if __name__ == "__main__":
    main()
