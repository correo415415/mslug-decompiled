#!/usr/bin/env python3
"""
Metal Slug 1 — Generador de la familia SetTaskHandler_<addr>.
================================================================
Escanea el P ROM procesado buscando el patrón exacto de 8 bytes:

    43FA  <disp16>  2C89  4E75

que corresponde a:
    lea    pc+disp, a1        ; a1 = &handler_cercano
    move.l a1, (fp)            ; fp->handler = a1     (offset 0)
    rts

Semánticamente: instala en el task actual (fp) un handler cercano (dentro
del rango PC-relativo 16-bit con signo) y retorna. En el código original C:

    void SetTaskHandler_N(void) {
        fp->handler = &TaskHandler_XXXXXX;
    }

donde GCC materializa la asignación con `lea + move.l a1,(a6) + rts`.

Genera:
  1) decomp/src/task_handlers.c con una función C por sitio.
  2) Auto-añade a symbols.py el defsym de cada TaskHandler_XXXXXX.
  3) Fragmento imprimible para registry.py (impreso por stdout).

REQUIERE compilar el .c con `-mpcrel` (via PER_FILE_CFLAGS en el matcher)
para que el `lea` salga en forma PC-relativa corta.
"""
import os, importlib.util

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
PROM = os.path.join(ROOT, "decomp", "build", "mslug_prom.bin")
OUT  = os.path.join(ROOT, "decomp", "src", "task_handlers.c")
REG  = os.path.join(ROOT, "decomp", "tools", "registry.py")


def scan(prom):
    """Devuelve [(addr, target_addr)] para el patrón 43FA disp 2C89 4E75."""
    hits = []
    for i in range(0, len(prom) - 8, 2):
        if prom[i:i+2] != b'\x43\xFA': continue
        if prom[i+4:i+8] != b'\x2C\x89\x4E\x75': continue
        disp = int.from_bytes(prom[i+2:i+4], 'big', signed=True)
        target = (i + 2 + disp) & 0xFFFFFFFF
        hits.append((i, target))
    return hits


def load_covered():
    spec = importlib.util.spec_from_file_location("reg", REG)
    m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
    cov = set()
    for name, addr, size, src in m.REGISTRY:
        cov.update(range(addr, addr + size))
    return cov


HEADER = '''/*
 * Metal Slug 1 — Familia SetTaskHandler_<addr>
 * ================================================
 * Mini-funciones de 8 bytes que instalan un handler cercano en el task
 * actual (fp->handler, offset 0) y retornan. Patrón exacto:
 *
 *     lea    pc+disp, a1        ; a1 = &handler
 *     move.l a1, (fp)            ; fp->handler = a1
 *     rts
 *
 * En el código original C:
 *
 *     void SetTaskHandler_N(void) {
 *         fp->handler = &TaskHandler_XXXXXX;
 *     }
 *
 * Cada handler destino es una rutina cercana (rango PC-rel 16-bit) que
 * se registra como símbolo extern TaskHandler_XXXXXX; el linker resuelve
 * su dirección exacta desde el defsym en symbols.py.
 *
 * COMPILAR CON `-mpcrel` (PER_FILE_CFLAGS) para que el `lea` salga corto
 * PC-relativo (4 B) en lugar de absolute long (6 B).
 *
 * ARCHIVO AUTO-GENERADO por decomp/tools/gen_task_handlers.py.
 */

#include "mslug.h"

/* Barrera de compilador que fuerza `move.l a1, (fp)` explícito
 * (sin optimizarlo como parte de otra expresión). */
#define STORE_A1_AT_FP()  __asm__ volatile("move.l %%a1, (%%fp)" ::: "memory")

'''


def main():
    with open(PROM, 'rb') as f: prom = f.read()
    hits = scan(prom)
    covered = load_covered()
    safe = [(a, t) for a, t in hits
            if not any(k in covered for k in range(a, a + 8))]
    print(f"Encontrados {len(hits)} SetTaskHandler; sin solapes: {len(safe)}")

    # Handlers únicos referenciados
    handlers = sorted(set(t for _, t in safe))
    print(f"Handlers únicos referenciados: {len(handlers)}")

    with open(OUT, 'w') as f:
        f.write(HEADER)
        # externs
        for h in handlers:
            f.write(f'extern void TaskHandler_{h:06x}(void);\n')
        f.write('\n')
        for addr, tgt in safe:
            sym = f"SetTaskHandler_{addr:06x}"
            f.write(f'__attribute__((section(".text.{sym}")))\n')
            f.write(f'void {sym}(void) {{\n')
            f.write(f'    _a1_ptr = &TaskHandler_{tgt:06x};\n')
            f.write(f'    STORE_A1_AT_FP();\n')
            f.write(f'}}\n\n')
    print(f"Escrito {OUT}  ({len(safe)} funciones)")

    return safe, handlers


if __name__ == "__main__":
    main()
