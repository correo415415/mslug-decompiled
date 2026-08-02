#!/usr/bin/env python3
"""
rank_candidates.py
------------------
Cola de prioridad ORDENADA POR TAMANO para la siguiente ola de decompilacion.

`scan_unmatched_callees.py` ordena por popularidad (numero de callers), lo que
optimiza "cuantas aristas cierro por funcion". Este script optimiza lo otro:
"cuantos BYTES avanzo por funcion", que es lo que pide una sesion orientada a
funciones grandes.

Para cada callee no matcheado:

  1. `size`   : bytes desensamblados linealmente hasta el primer terminador
                (rts / rte / jmp / bra incondicional), igual que el scanner.
  2. `gap`    : distancia hasta la SIGUIENTE entrada ya matcheada del REGISTRY.
                Si `size > gap`, el desensamblado lineal se ha metido dentro
                de una funcion ya matcheada -> la funcion real termina antes
                (tipicamente fall-through en el vecino, o el epilogo es un
                CCR-helper/thunk compartido). Se marca `TRUNCA` y se recorta
                el tamano efectivo al gap: ESA es la leccion de la Wave II.
  3. `score`  : eff_size * (1 + log2(callers)). Prima tamano, premia
                popularidad. Es el orden por defecto.

Salida por candidato: rango, addr, callers, size bruto, gap, size efectivo,
flag TRUNCA, y preview de desensamblado (primeras N instrucciones).

Uso:
    python3 tools/rank_candidates.py                 # top 25 por score
    python3 tools/rank_candidates.py --top 40 --sort size
    python3 tools/rank_candidates.py --min-eff 60    # solo >=60 B efectivos
    python3 tools/rank_candidates.py --json build/rank.json
"""
from __future__ import annotations

import argparse
import json
import math
import os
import sys

from capstone import Cs, CS_ARCH_M68K, CS_MODE_M68K_000

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)

# Reutilizamos el pipeline ya auditado del scanner (bitmap, extraccion de
# aristas por modo de direccionamiento correcto, desensamblado delimitado).
from scan_unmatched_callees import (  # noqa: E402
    load_registry, build_covered, extract_calls, disasm_function, fmt_dis,
)

ROM_PATH = os.path.join(HERE, "..", "build", "mslug_prom.bin")
ROM_SIZE = 0x200000


def next_matched_after(sorted_offsets, addr):
    """Primera direccion de inicio de una entrada matcheada > addr (o None)."""
    lo, hi = 0, len(sorted_offsets)
    while lo < hi:
        mid = (lo + hi) // 2
        if sorted_offsets[mid] <= addr:
            lo = mid + 1
        else:
            hi = mid
    return sorted_offsets[lo] if lo < len(sorted_offsets) else None


def main():
    ap = argparse.ArgumentParser(description=__doc__.strip().splitlines()[0])
    ap.add_argument("--top", type=int, default=25, help="candidatos a mostrar")
    ap.add_argument("--sort", choices=("score", "size", "callers"),
                    default="score", help="criterio de orden (default: score)")
    ap.add_argument("--min-addr", type=lambda x: int(x, 0), default=0x400,
                    help="descarta targets bajo esta direccion (default $400)")
    ap.add_argument("--min-eff", type=int, default=20,
                    help="tamano efectivo minimo en bytes (default 20)")
    ap.add_argument("--max-preview", type=int, default=48,
                    help="bytes de preview de desensamblado (default 48)")
    ap.add_argument("--json", metavar="FILE",
                    help="volcar la lista completa a JSON")
    args = ap.parse_args()

    if not os.path.exists(ROM_PATH):
        sys.exit(f"[!] Falta {ROM_PATH} - ejecuta scripts/setup.sh primero.")
    rom = open(ROM_PATH, "rb").read()

    reg = load_registry()
    covered, ranges = build_covered(reg)
    sorted_offsets = sorted(off for _n, off, _s, _f in reg)

    md = Cs(CS_ARCH_M68K, CS_MODE_M68K_000)
    md.detail = True

    calls, _by_mode = extract_calls(md, rom, ranges)

    rows = []
    for dst, n_callers in calls.items():
        if dst < args.min_addr or dst >= ROM_SIZE or dst % 2:
            continue
        if covered[dst]:
            continue
        insts = disasm_function(md, rom, dst)
        size = sum(x.size for x in insts)
        if size == 0:
            continue
        nxt = next_matched_after(sorted_offsets, dst)
        gap = (nxt - dst) if nxt is not None else size
        trunc = size > gap
        eff = min(size, gap)
        if eff < args.min_eff:
            continue
        score = eff * (1.0 + math.log2(max(n_callers, 1)))
        rows.append({
            "addr": dst, "callers": n_callers, "size": size,
            "gap": gap, "eff_size": eff, "truncated": trunc,
            "score": round(score, 1),
            "_insts": insts,
        })

    key = {"score": lambda r: r["score"],
           "size": lambda r: r["eff_size"],
           "callers": lambda r: r["callers"]}[args.sort]
    rows.sort(key=key, reverse=True)

    total_eff = sum(r["eff_size"] for r in rows)
    print(f"Candidatos no matcheados >= {args.min_eff} B efectivos : "
          f"{len(rows)}  (~{total_eff:,} B en cola)")
    print(f"Orden: {args.sort}\n" + "=" * 72)

    for i, r in enumerate(rows[:args.top], 1):
        flag = "  TRUNCA->vecino matcheado" if r["truncated"] else ""
        print(f"[{i:3d}] ${r['addr']:06x}  eff={r['eff_size']:4d}B "
              f"(lineal {r['size']}B / gap {r['gap']}B)  "
              f"callers={r['callers']:<3d} score={r['score']:8.1f}{flag}")
        print(fmt_dis(r["_insts"], args.max_preview))
        print()

    if args.json:
        out = [{k: v for k, v in r.items() if k != "_insts"} for r in rows]
        with open(args.json, "w") as f:
            json.dump({"sort": args.sort, "min_eff": args.min_eff,
                       "candidates": out}, f, indent=1)
        print(f"[json] {len(out)} candidatos -> {args.json}")


if __name__ == "__main__":
    main()
