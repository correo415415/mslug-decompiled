#!/usr/bin/env python3
"""
Metal Slug 1 — Generador de la macro-familia CCR helpers.
============================================================
Escanea el P ROM procesado buscando el patrón exacto:

    <opcode2>  00XX  4E75      (6 bytes)   con opcode2 ∈ { 023C, 003C }

y produce decomp/src/ccr_helpers.c con una función C por sitio.

Semántica descubierta:
  ClearXN : andi.b #$EE, ccr ; rts  (limpia flags X y N)
  ClearC  : andi.b #$FE, ccr ; rts  (limpia flag C)
  SetXN   : ori.b  #$11, ccr ; rts  (activa flags X y N)
  SetC    : ori.b  #$01, ccr ; rts  (activa flag C)
  NopCCR  : ori.b  #$00, ccr ; rts  (nop lógico — no cambia CCR)

En el diseño del compilador Nazca/SN Systems, estas mini-funciones se
usaban como "canal de retorno booleano" — el llamador comprobaba el CCR
con bcc/bne justo tras el `jsr`. Es un patrón muy poco común hoy pero
frecuente en compiladores de los 90.
"""
import os, importlib.util
from collections import Counter

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
PROM = os.path.join(ROOT, "decomp", "build", "mslug_prom.bin")
OUT  = os.path.join(ROOT, "decomp", "src", "ccr_helpers.c")
REG  = os.path.join(ROOT, "decomp", "tools", "registry.py")

# Mapa: (op2_hex, imm) -> (nombre_semántico, asm_instr)
SEMANTICS = {
    ("023c", 0xEE): ("ClearXN", "andi.b #0xEE, %%ccr"),
    ("023c", 0xFE): ("ClearC",  "andi.b #0xFE, %%ccr"),
    ("023c", 0x0E): ("ClearXNV","andi.b #0x0E, %%ccr"),  # variante rara
    ("003c", 0x11): ("SetXN",   "ori.b  #0x11, %%ccr"),
    ("003c", 0x01): ("SetC",    "ori.b  #0x01, %%ccr"),
    ("003c", 0x00): ("NopCCR",  "ori.b  #0x00, %%ccr"),
    ("003c", 0x02): ("SetV",    "ori.b  #0x02, %%ccr"),  # variante rara
}


def scan(prom):
    hits = []  # list of (addr, op2_hex, imm, name, asm_instr)
    for i in range(0, len(prom) - 6, 2):
        if prom[i+4:i+6] != b'\x4E\x75': continue
        if prom[i+2] != 0x00: continue
        op2 = prom[i:i+2].hex()
        if op2 not in ("023c", "003c"): continue
        imm = prom[i+3]
        key = (op2, imm)
        if key not in SEMANTICS: continue
        name, asm = SEMANTICS[key]
        hits.append((i, op2, imm, name, asm))
    return hits


def load_covered():
    spec = importlib.util.spec_from_file_location("reg", REG)
    m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
    cov = set()
    for name, addr, size, src in m.REGISTRY:
        cov.update(range(addr, addr + size))
    return cov


HEADER = '''/*
 * Metal Slug 1 — Familia CCR helpers (macro-familia de 1029+ funciones)
 * ========================================================================
 * Mini-funciones de 6 bytes que manipulan el Condition Code Register (CCR)
 * y retornan. En el diseño del compilador original (Nazca / SN Systems)
 * eran el "canal de retorno booleano" — el llamador comprobaba el CCR
 * con bcc/bne inmediatamente tras el `jsr`:
 *
 *     jsr  ChequeoQueDevuelveXN
 *     bpl  todo_ok            ; bit N indica "no OK"
 *     ...  fallback
 *
 * Todas las mini-funciones caben en 6 bytes: una única instrucción CCR
 * (andi.b o ori.b sobre %ccr) + rts. Aparecen replicadas cientos de veces
 * en el ROM porque el compilador no las unificó — cada sitio tiene su
 * propia dirección para que las llamadas jsr abs.l apunten al lugar
 * exacto.
 *
 * En decompilación semántica final, todas se plegarán a una sola función
 * inline con nombre semántico; el compilador re-generaría las copias
 * conforme sea necesario.
 *
 * ARCHIVO AUTO-GENERADO por decomp/tools/gen_ccr_helpers.py — no editar
 * a mano. Regenerar tras cambios en el P ROM o en las heurísticas.
 */

#include "mslug.h"

'''

def main():
    with open(PROM, 'rb') as f: prom = f.read()
    hits = scan(prom)
    covered = load_covered()
    safe = [h for h in hits
            if not any(k in covered for k in range(h[0], h[0] + 6))]

    print(f"Encontrados {len(hits)} sitios CCR; sin solapes con registry: {len(safe)}")
    dist = Counter((h[3]) for h in safe)
    for name, n in dist.most_common():
        print(f"  {name:10s}  {n} sitios")

    with open(OUT, 'w') as f:
        f.write(HEADER)
        for addr, op2, imm, name, asm in safe:
            sym = f"{name}_{addr:06x}"
            f.write(f'__attribute__((section(".text.{sym}")))\n')
            f.write(f'void {sym}(void) '
                    f'{{ __asm__ volatile("{asm}" ::: "cc"); }}\n\n')
    print(f"Escrito {OUT}  ({len(safe)} funciones)")

    return safe


if __name__ == "__main__":
    main()
