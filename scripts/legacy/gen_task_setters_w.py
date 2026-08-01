#!/usr/bin/env python3
"""
Metal Slug 1 — Generador de la familia SetTaskW_<addr>.
=========================================================
Escanea el P ROM procesado buscando el patrón exacto de setter simple:

    3D40  00XX  4E75      ; move.w d0, XX(fp) ; rts    (6 bytes)

y produce:
  1) decomp/src/task_setters_w.c con una función C por sitio (todas del
     mismo cuerpo, cambiando solo el offset).
  2) Un fragmento imprimible listo para pegar en decomp/tools/registry.py.

Cada función C tiene la forma canónica:

    __attribute__((section(".text.SetTaskW_<addr>")))
    void SetTaskW_<addr>(void) { TASK_W(0xXX) = _d0_w; }

GCC 13 -Os la emite en 6 bytes exactos `3D40 00XX 4E75`.
"""
import os, sys

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
PROM = os.path.join(ROOT, "decomp", "build", "mslug_prom.bin")
OUT  = os.path.join(ROOT, "decomp", "src", "task_setters_w.c")


def scan(prom):
    hits = []
    for i in range(0, len(prom) - 6, 2):
        if prom[i:i+2] == b'\x3D\x40' and prom[i+4:i+6] == b'\x4E\x75':
            off = (prom[i+2] << 8) | prom[i+3]
            hits.append((i, off))
    return hits


HEADER = '''/*
 * Metal Slug 1 — Familia SetTaskW: setters `move.w d0, off(fp) ; rts`
 * =====================================================================
 * Funciones de 6 bytes que guardan el word en d0 en un offset concreto
 * de la entidad apuntada por fp (A6) y retornan. En el compilador original
 * son inlines de un solo statement:
 *
 *     void SetField_XX(u16 v) { fp->field_XX = v; }
 *
 * GCC 13 -Os reproduce este patrón exacto (3D40 00XX 4E75) desde el C
 * canónico `TASK_W(0x??) = _d0_w;`. Cada sitio se emite en su propia
 * sección .text.SetTaskW_<addr> para que el linker las coloque en su
 * dirección de ROM correspondiente.
 *
 * ARCHIVO AUTO-GENERADO por decomp/tools/gen_task_setters_w.py — no
 * editar a mano; regenerar tras cambios en el P ROM o el escáner.
 */

#include "mslug.h"

'''

def main():
    with open(PROM, 'rb') as f:
        prom = f.read()
    hits = scan(prom)
    print(f"Encontrados {len(hits)} setters SetTaskW en el P ROM.")

    with open(OUT, 'w') as f:
        f.write(HEADER)
        for addr, off in hits:
            f.write(f'__attribute__((section(".text.SetTaskW_{addr:06x}")))\n')
            f.write(f'void SetTaskW_{addr:06x}(void) '
                    f'{{ TASK_W(0x{off:02X}) = _d0_w; }}\n\n')
    print(f"Escrito {OUT}")

    # Fragmento de registry
    print("\n# Fragmento para registry.py:")
    for addr, off in hits:
        print(f'    ("SetTaskW_{addr:06x}", 0x{addr:06X},  6, "task_setters_w.c"),')

if __name__ == "__main__":
    main()
