#!/usr/bin/env python3
"""
Metal Slug 1 — Generador de la familia StateDispatchStub.
============================================================
Escanea el P ROM procesado buscando el patrón de 14 bytes exactos:

    45F9  <table_addr:4>  4EB9  0005 022A  4E75

Es decir: `lea $tableAddr.L, a2 ; jsr $0005022A.L ; rts`.

Cada SDS es un "adapter" que instala su propia tabla de estados en A2 y
delega el trabajo al runtime común StateMachineRun($0005022A). En el
compilador original la definición C era una sola línea:

    void SDS_N(void) { RunStateMachine(&StateTable_N); }

donde el compilador materializaba la llamada con `lea + jsr abs.l + rts`.

Genera:
  1) decomp/src/state_dispatch_stubs.c con una función C por sitio.
  2) Fragmento imprimible para registry.py.
  3) Auto-añade a symbols.py el defsym de cada StateTable_XXXXXX.
"""
import os, importlib.util

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
PROM = os.path.join(ROOT, "decomp", "build", "mslug_prom.bin")
OUT  = os.path.join(ROOT, "decomp", "src", "state_dispatch_stubs.c")
REG  = os.path.join(ROOT, "decomp", "tools", "registry.py")


def scan(prom):
    """Devuelve [(addr, table_addr)] de todos los SDS."""
    pat_tail = b'\x4E\xB9\x00\x05\x02\x2A\x4E\x75'   # jsr $0005022A ; rts
    hits = []
    for i in range(0, len(prom) - 14, 2):
        if prom[i:i+2] != b'\x45\xF9': continue
        if prom[i+6:i+14] != pat_tail: continue
        table_addr = int.from_bytes(prom[i+2:i+6], 'big')
        hits.append((i, table_addr))
    return hits


def load_covered():
    spec = importlib.util.spec_from_file_location("reg", REG)
    m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
    cov = set()
    for name, addr, size, src in m.REGISTRY:
        cov.update(range(addr, addr + size))
    return cov


HEADER = '''/*
 * Metal Slug 1 — Familia StateDispatchStub (SDS)
 * =================================================
 * 269 mini-funciones de 14 bytes que ejecutan siempre el mismo patrón:
 *
 *     lea    StateTable_XXXXXX.L, a2      ; carga tabla propia en A2
 *     jsr    StateMachineRun.L            ; runtime común
 *     rts
 *
 * Semánticamente equivalente a: `void SDS_N(void) { _a2_tbl = &StateTable_N;
 * StateMachineRun(); }` — pero con el rts explícito (no tail-call) para
 * casar los 14 bytes originales `45F9 xxxxxxxx 4EB9 0005022A 4E75`.
 *
 * StateMachineRun ($0005022A) es un intérprete común que lee entradas
 * (opcode + payload) de la tabla en A2 y ejecuta cambios en la entidad
 * apuntada por fp. La estructura interna de las tablas se descubrirá al
 * decompilar $5022A.
 *
 * ARCHIVO AUTO-GENERADO por decomp/tools/gen_sds.py. Cada StateTable_*
 * queda como símbolo externo cuyo defsym se añade a symbols.py.
 */

#include "mslug.h"

'''


def main():
    with open(PROM, 'rb') as f: prom = f.read()
    hits = scan(prom)
    covered = load_covered()
    safe = [(a, t) for a, t in hits
            if not any(k in covered for k in range(a, a + 14))]
    print(f"Encontrados {len(hits)} SDS; sin solapes: {len(safe)}")

    # Tablas únicas
    tables = sorted(set(t for _, t in safe))
    print(f"Tablas únicas referenciadas: {len(tables)}")

    with open(OUT, 'w') as f:
        f.write(HEADER)
        # externs para cada tabla
        for t in tables:
            f.write(f'extern void StateTable_{t:06x}(void);\n')
        f.write('\n')
        for addr, tab in safe:
            sym = f"SDS_{addr:06x}"
            f.write(f'__attribute__((section(".text.{sym}")))\n')
            f.write(f'void {sym}(void) {{\n')
            f.write(f'    _a2_tbl = &StateTable_{tab:06x};\n')
            f.write(f'    StateMachineRun();\n')
            f.write(f'    __asm__ volatile("" ::: "memory");\n')
            f.write(f'}}\n\n')
    print(f"Escrito {OUT}  ({len(safe)} funciones)")

    return safe, tables


if __name__ == "__main__":
    main()
