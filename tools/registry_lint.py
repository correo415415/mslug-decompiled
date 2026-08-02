#!/usr/bin/env python3
"""
registry_lint.py
----------------
Auditoria estatica del registro de matching. Es el "CI check" del proyecto:
si esto no pasa, `match_batch.py` puede fallar con errores crIpticos del
linker (overlaps de LMA, relocations truncadas) o, peor, dar verde con
metadatos incoherentes.

Checks (cada uno con codigo de fallo propio):

  E1  OVERLAP     dos entradas del REGISTRY se solapan en direcciones.
  E2  DUP-ADDR    misma direccion registrada dos veces.
  E3  DUP-NAME    mismo nombre registrado dos veces (el linker colapsaria
                  ambas secciones .text.<Sym> en una).
  E4  ODD-ADDR    direccion impar (el 68000 no puede ejecutar ahi).
  E5  BAD-SIZE    tamano <= 0 o impar (todo opcode 68k son multiplos de 2).
  E6  SYM-CLASH   un nombre de SYMBOLS apunta a direccion distinta de la
                  que ese mismo nombre tiene en REGISTRY (el --defsym
                  ganaria y el byte-compare fallaria en silencio en otro
                  sitio). NOTA: dict de Python -> una addr duplicada como
                  clave en symbols.py NO es detectable aqui; por eso W1.
  W1  SYM-DUP-SRC (warning) direcciones que aparecen >1 vez como clave en
                  el TEXTO de symbols.py: la ultima gana en silencio y ya
                  causo un fallo real (TaskHandler_064d8a, Wave OO#2).
  W2  FILE-MISS   (warning) source_file referenciado que no existe en
                  src/ ni asm/.

Uso:
    python3 tools/registry_lint.py            # exit 0 = limpio
    python3 tools/registry_lint.py --quiet    # solo resumen
"""
from __future__ import annotations

import argparse
import os
import re
import sys
from collections import Counter

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)
sys.path.insert(0, HERE)

from registry import REGISTRY  # noqa: E402
from symbols import SYMBOLS    # noqa: E402


def main():
    ap = argparse.ArgumentParser(description=__doc__.strip().splitlines()[0])
    ap.add_argument("--quiet", action="store_true", help="solo resumen")
    args = ap.parse_args()

    errors, warnings = [], []
    say = (lambda *_a: None) if args.quiet else print

    # --- E4/E5: sanidad por entrada ---------------------------------------
    for name, off, size, src in REGISTRY:
        if off % 2:
            errors.append(f"E4 ODD-ADDR  {name} @ ${off:06x}")
        if size <= 0 or size % 2:
            errors.append(f"E5 BAD-SIZE  {name} @ ${off:06x} size={size}")

    # --- E1/E2: overlaps y duplicados de direccion -------------------------
    by_addr = sorted(REGISTRY, key=lambda e: e[1])
    for a, b in zip(by_addr, by_addr[1:]):
        if a[1] == b[1]:
            errors.append(f"E2 DUP-ADDR  ${a[1]:06x}: {a[0]} y {b[0]}")
        elif a[1] + a[2] > b[1]:
            errors.append(
                f"E1 OVERLAP   {a[0]} [${a[1]:06x}..${a[1]+a[2]:06x}) pisa "
                f"{b[0]} @ ${b[1]:06x} (+{a[1]+a[2]-b[1]} B)")

    # --- E3: nombres duplicados --------------------------------------------
    names = Counter(e[0] for e in REGISTRY)
    for n, c in names.items():
        if c > 1:
            errors.append(f"E3 DUP-NAME  {n} x{c}")

    # --- E6: clash SYMBOLS vs REGISTRY -------------------------------------
    reg_by_name = {e[0]: e[1] for e in REGISTRY}
    n_shared = 0
    for addr, sym in SYMBOLS.items():
        if sym in reg_by_name:
            n_shared += 1
            if reg_by_name[sym] != addr:
                errors.append(
                    f"E6 SYM-CLASH {sym}: SYMBOLS=${addr:06x} "
                    f"REGISTRY=${reg_by_name[sym]:06x}")

    # --- W1: claves duplicadas en el TEXTO de symbols.py --------------------
    sym_src = open(os.path.join(HERE, "symbols.py")).read()
    keys = re.findall(r"^\s*(0x[0-9A-Fa-f]{6,8})\s*:", sym_src, re.M)
    kcount = Counter(int(k, 16) for k in keys)
    for k, c in sorted(kcount.items()):
        if c > 1:
            warnings.append(
                f"W1 SYM-DUP-SRC ${k:06x} aparece {c} veces como clave en "
                f"symbols.py (la ultima gana en silencio)")

    # --- W2: source files inexistentes --------------------------------------
    for src in sorted({e[3] for e in REGISTRY}):
        if not (os.path.exists(os.path.join(ROOT, "src", src)) or
                os.path.exists(os.path.join(ROOT, "asm", src))):
            warnings.append(f"W2 FILE-MISS {src}")

    # --- Informe -------------------------------------------------------------
    for e in errors:
        print("  [E]", e)
    for w in warnings:
        print("  [W]", w)

    total_b = sum(e[2] for e in REGISTRY)
    say(f"\nregistry_lint: {len(REGISTRY)} entradas, {total_b:,} B, "
        f"{len(SYMBOLS)} simbolos ({n_shared} compartidos con REGISTRY)")
    if errors:
        print(f"registry_lint: FAIL  ({len(errors)} errores, "
              f"{len(warnings)} warnings)")
        sys.exit(1)
    print(f"registry_lint: OK    (0 errores, {len(warnings)} warnings)")
    sys.exit(0)


if __name__ == "__main__":
    main()
