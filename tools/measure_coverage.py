#!/usr/bin/env python3
"""
measure_coverage.py
-------------------
Metrica de cobertura REAL del proyecto (la que alimenta docs/COVERAGE.md).

`match_batch.py` reporta bytes matcheados frente a los 2 MiB completos de
la P-ROM, pero la mayor parte de la ROM son datos (paletas, tilemaps,
scripts de nivel, padding) que nunca seran "codigo a decompilar". Este
script separa las tres metricas que hay que distinguir:

  1. ROM total          bytes registrados / 2 MiB           (la del matcher)
  2. Codigo real        bytes de CODIGO registrados / total de codigo
                        ejecutable estimado en la ROM  <- la metrica util
  3. Composicion        clasificacion heuristica de la ROM por bloques

Heuristica de clasificacion (bloques de 4 KiB):
  - H         entropia de Shannon del bloque (bits/byte)
  - opdens    fraccion de muestras (cada 16 B) cuya primera instruccion
              decodificada por capstone es un mnemonic 68000 COMUN
              (whitelist), no solo "decodificable" (los datos densos
              tambien decodifican linealmente, pero rara vez a mnemonics
              frecuentes como move/bsr/rts)
  - zfrac     fraccion de bytes 0x00 / 0xFF
  - afrac     fraccion de bytes ASCII imprimibles

  ZERO      zfrac > 0.90
  ASCII     afrac > 0.60
  CODE?     H <= 7.6  y  opdens > 0.45
  DATA-LO   H <= 5.0  (y no CODE?)
  DATA-MID  resto (datos densos: paletas/tilemaps/scripts)

Uso:
    python3 tools/measure_coverage.py            # resumen
    python3 tools/measure_coverage.py --blocks   # ademas, mapa por bloque
"""
from __future__ import annotations

import argparse
import math
import os
import sys
from collections import Counter

from capstone import Cs, CS_ARCH_M68K, CS_MODE_M68K_000

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from registry import REGISTRY  # noqa: E402

ROM_PATH = os.path.join(HERE, "..", "build", "mslug_prom.bin")
BLOCK = 0x1000  # 4 KiB

COMMON_MNEM = {
    "move", "movea", "moveq", "movem", "lea", "pea", "jsr", "jmp", "rts",
    "rte", "rtr", "bra", "bsr", "beq", "bne", "bge", "bgt", "ble", "blt",
    "bcc", "bcs", "bpl", "bmi", "bhi", "bls", "add", "adda", "addi", "addq",
    "sub", "suba", "subi", "subq", "cmp", "cmpa", "cmpi", "tst", "clr",
    "and", "andi", "or", "ori", "eor", "eori", "not", "neg", "ext", "swap",
    "btst", "bset", "bclr", "bchg", "lsl", "lsr", "asl", "asr", "rol",
    "ror", "roxl", "roxr", "mulu", "muls", "divu", "divs", "dbra", "dbf",
    "link", "unlk", "exg", "nop",
}


def entropy(chunk: bytes) -> float:
    if not chunk:
        return 0.0
    counts = Counter(chunk)
    n = len(chunk)
    return -sum((c / n) * math.log2(c / n) for c in counts.values())


def opcode_density(md, chunk: bytes, base: int) -> float:
    """
    Fraccion de MUESTRAS (cada 16 B) que decodifican a un opcode 68000
    COMUN. El muestreo con whitelist discrimina mucho mejor que el
    desensamblado lineal puro: los datos densos (tilemaps, paletas)
    tambien "decodifican" linealmente sin error, pero rara vez a
    mnemonics frecuentes.
    """
    samples = plaus = 0
    for off in range(0, len(chunk) - 10, 16):
        samples += 1
        for ins in md.disasm(bytes(chunk[off:off + 10]), base + off):
            mn = ins.mnemonic.split(".")[0]
            if mn in COMMON_MNEM:
                plaus += 1
            break
    return plaus / samples if samples else 0.0


def classify(md, chunk: bytes, base: int) -> str:
    n = len(chunk)
    zfrac = (chunk.count(0x00) + chunk.count(0xFF)) / n
    if zfrac > 0.90:
        return "ZERO"
    afrac = sum(1 for b in chunk if 0x20 <= b < 0x7F) / n
    if afrac > 0.60:
        return "ASCII"
    h = entropy(chunk)
    if h < 7.6 and opcode_density(md, chunk, base) > 0.45:
        return "CODE?"
    if h <= 5.0:
        return "DATA-LO"
    return "DATA-MID"


def main():
    ap = argparse.ArgumentParser(description=__doc__.strip().splitlines()[0])
    ap.add_argument("--blocks", action="store_true",
                    help="imprimir el mapa bloque a bloque")
    args = ap.parse_args()

    if not os.path.exists(ROM_PATH):
        sys.exit(f"[!] Falta {ROM_PATH} - ejecuta scripts/setup.sh primero.")
    rom = open(ROM_PATH, "rb").read()
    rom_size = len(rom)

    md = Cs(CS_ARCH_M68K, CS_MODE_M68K_000)

    # --- bitmap de bytes registrados -----------------------------------------
    covered = bytearray(rom_size)
    total_reg = 0
    for _name, off, size, _src in REGISTRY:
        end = min(off + size, rom_size)
        for i in range(off, end):
            covered[i] = 1
        total_reg += end - off

    # --- clasificacion por bloques ------------------------------------------
    cat_bytes = Counter()
    cat_cov = Counter()
    block_map = []
    for base in range(0, rom_size, BLOCK):
        chunk = rom[base:base + BLOCK]
        cat = classify(md, chunk, base)
        cov = sum(covered[base:base + BLOCK])
        cat_bytes[cat] += len(chunk)
        cat_cov[cat] += cov
        block_map.append((base, cat, cov))

    code_total = cat_bytes["CODE?"]
    code_cov = cat_cov["CODE?"]

    print("=" * 68)
    print("  COBERTURA REAL - Metal Slug 1 matching decomp")
    print("=" * 68)
    print(f"  Registrado (matcher)  : {total_reg:>9,} B / {rom_size:,} B "
          f"({100.0*total_reg/rom_size:.4f} % ROM total)")
    print(f"  Codigo estimado (ROM) : {code_total:>9,} B "
          f"({100.0*code_total/rom_size:.1f} % de la ROM)")
    print(f"  Codigo ya cubierto    : {code_cov:>9,} B / {code_total:,} B "
          f"(** {100.0*code_cov/max(code_total,1):.2f} % del codigo real **)")
    print("-" * 68)
    print("  Composicion de la P-ROM (bloques de 4 KiB):")
    for cat in ("CODE?", "DATA-MID", "DATA-LO", "ZERO", "ASCII"):
        b = cat_bytes[cat]
        print(f"    {cat:<9} {b:>9,} B  {100.0*b/rom_size:5.1f} %   "
              f"cubierto: {cat_cov[cat]:,} B")
    print("=" * 68)

    if args.blocks:
        print("\n  Mapa por bloque ($addr  categoria  cubierto):")
        for base, cat, cov in block_map:
            if cat != "ZERO" or cov:
                mark = f"  <-- {cov} B" if cov else ""
                print(f"    ${base:06x}  {cat:<9}{mark}")


if __name__ == "__main__":
    main()
