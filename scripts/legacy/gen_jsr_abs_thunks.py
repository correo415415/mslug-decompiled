#!/usr/bin/env python3
"""
Metal Slug 1 — Generador de la familia JsrAbsThunk_<addr>.
=============================================================
Escanea el P ROM procesado buscando el patrón exacto de 8 bytes:

    4EB9  <target:4>  4E75

que corresponde a:
    jsr    target.L        ; llamada absoluta long a otra rutina
    rts

Semánticamente son "trampolines" o "adapters": funciones cuyo cuerpo
consiste en llamar a otra función más y retornar. En C original:

    void Thunk_XXXXXX(void) {
        RealFunction_YYYYYY();
    }

GCC 13 -Os emitiría normalmente `jmp abs.l` (tail-call de 6 B), pero
añadiendo una barrera `__asm__ volatile("" ::: "memory")` tras la
llamada, fuerza el `rts` explícito y los 8 bytes exactos del ROM.

Genera:
  1) decomp/src/jsr_abs_thunks.c
  2) Fragmento imprimible para registry.py.
  3) Auto-añade a symbols.py el defsym de cada ThunkTarget_XXXXXX.
"""
import os, importlib.util

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
PROM = os.path.join(ROOT, "decomp", "build", "mslug_prom.bin")
OUT  = os.path.join(ROOT, "decomp", "src", "jsr_abs_thunks.c")
REG  = os.path.join(ROOT, "decomp", "tools", "registry.py")


def scan(prom):
    hits = []
    for i in range(0, len(prom) - 8, 2):
        if prom[i:i+2] != b'\x4E\xB9': continue
        if prom[i+6:i+8] != b'\x4E\x75': continue
        target = int.from_bytes(prom[i+2:i+6], 'big')
        hits.append((i, target))
    return hits


def load_covered_and_matched():
    """Devuelve (covered, name_at) consultando registry.py Y symbols.py."""
    spec = importlib.util.spec_from_file_location("reg", REG)
    m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
    cov = set()
    name_at = {}
    for name, addr, size, src in m.REGISTRY:
        cov.update(range(addr, addr + size))
        name_at[addr] = name
    # Consultar también symbols.py para los defsym semánticos
    sym_path = os.path.join(ROOT, "decomp", "tools", "symbols.py")
    spec2 = importlib.util.spec_from_file_location("sym", sym_path)
    ms = importlib.util.module_from_spec(spec2); spec2.loader.exec_module(ms)
    for addr, nm in ms.SYMBOLS.items():
        if addr not in name_at:
            name_at[addr] = nm
    return cov, name_at


HEADER = '''/*
 * Metal Slug 1 — Familia JsrAbsThunk (trampolines jsr abs.l ; rts)
 * ====================================================================
 * Mini-funciones de 8 bytes que llaman a otra rutina y retornan:
 *
 *     jsr    RealFunction.L        ; 4EB9 XXXXXXXX  (6 B)
 *     rts                          ; 4E75           (2 B)
 *
 * En C original:
 *     void Thunk_XXXXXX(void) { RealFunction_YYYYYY(); }
 *
 * GCC prefiere convertir en tail-call (`jmp abs.l`, 6 B), pero con una
 * barrera `__asm__ volatile("" ::: "memory")` tras la llamada mantiene el
 * `rts` explícito y emite los 8 bytes exactos del ROM.
 *
 * ARCHIVO AUTO-GENERADO por decomp/tools/gen_jsr_abs_thunks.py.
 */

#include "mslug.h"

'''

def main():
    with open(PROM, 'rb') as f: prom = f.read()
    hits = scan(prom)
    covered, name_at = load_covered_and_matched()
    safe = [(a, t) for a, t in hits
            if not any(k in covered for k in range(a, a + 8))]
    print(f"Encontrados {len(hits)} JsrAbsThunk; sin solapes: {len(safe)}")

    # Targets únicos
    targets = sorted(set(t for _, t in safe))
    # Para cada target, usar el nombre ya registrado si existe;
    # si no, un genérico ThunkTarget_XXXXXX
    def target_name(t):
        return name_at.get(t, f"ThunkTarget_{t:06x}")

    with open(OUT, 'w') as f:
        f.write(HEADER)
        # externs — sólo declarar los que NO están ya registrados
        # (los ya registrados se declaran naturalmente al aparecer en registry)
        externs_needed = [t for t in targets if t not in name_at]
        for t in externs_needed:
            f.write(f'extern void ThunkTarget_{t:06x}(void);\n')
        f.write('\n')
        for addr, tgt in safe:
            sym = f"JsrAbsThunk_{addr:06x}"
            name = target_name(tgt)
            f.write(f'__attribute__((section(".text.{sym}")))\n')
            f.write(f'void {sym}(void) {{\n')
            f.write(f'    extern void {name}(void);\n')
            f.write(f'    {name}();\n')
            f.write(f'    __asm__ volatile("" ::: "memory");\n')
            f.write(f'}}\n\n')
    print(f"Escrito {OUT}  ({len(safe)} funciones, {len(externs_needed)} externs nuevos)")

    return safe, externs_needed


if __name__ == "__main__":
    main()
