#!/usr/bin/env python3
"""
scan_unmatched_callees.py
-------------------------
Diagnostico que apoya la siguiente ola de decompilacion.

1. Carga REGISTRY  (lista de (nombre, offset, size, file)) y construye un
   bitmap de los bytes ya reclamados.
2. Desensambla la P ROM procesada (decomp/build/mslug_prom.bin) con
   capstone linealmente por cada funcion ya conocida (para no perseguir
   datos), y extrae los destinos absolutos de:
     * `jsr abs.l` / `jmp abs.l`           (opcodes 4EB9 / 4EF9, 6 B)
     * `jsr d16(pc)` / `jmp d16(pc)`       (opcodes 4EBA / 4EFA, 4 B)
     * `bsr.w` / `bsr.s` / `bra.w` / `bra.s`  (opcodes 6100/60xx/61xx, 2/4 B)
3. Discrimina rigurosamente los modos de direccionamiento capstone:
     * M68K_AM_ABSOLUTE_DATA_LONG  (17)  -> target = op.mem.disp
     * M68K_AM_PCI_DISP            (11)  -> target = PC + 2 + op.mem.disp
     * M68K_AM_BRANCH_DISPLACEMENT (19)  -> target = op.br_disp.disp (ya
                                            resuelto por capstone)
4. Aplica filtros de plausibilidad para descartar falsos positivos:
     * --min-addr (default $400)  -> descarta zona vectores 68000
                                     ($000..$0FF vector table +
                                      $100..$3FF Neo Geo cartridge header)
     * --min-size (default 20)    -> descarta targets con desensamblado
                                     "vacio" (probable datos o vector)
5. Cuenta llamadas entrantes por destino no matcheado y muestra los Top-N
   ordenados por popularidad, con el desensamblado de cada uno (hasta el
   primer rts/rte/jmp/bra incondicional, max --max-size bytes).

Cambios respecto a la version original (bug fix):

  * ANTES: el path que leia `op.type == M68K_OP_MEM` y `op.mem.disp` NO
    distinguia entre absolute long y PC-relative. Ambos aparecen como
    M68K_OP_MEM con base_reg=0 e index_reg=0. Un thunk `4EBA 0004`
    (`jsr $XXX(pc)` con d=4) generaba `dst = 4`, contabilizando 6
    falsos "callers" hacia el vector Reset del 68000. Mismo bug con
    d=$3C (target $000), d=$4E, d=$C6, etc.
  * ANTES: habia un "Path 2" (parseo naive de op_str) que duplicaba las
    aristas de `jsr abs.l` legitimos, inflando popularidad 2x.
  * AHORA: un unico path por modo de direccionamiento, sin duplicacion,
    y con calculo correcto del target en cada modo.

Uso:
    python3 tools/scan_unmatched_callees.py [--top 40] [--min-addr 0x400]
                                            [--min-size 20]
"""
from __future__ import annotations

import argparse
import importlib.util
import os
import sys
from collections import Counter

from capstone import Cs, CS_ARCH_M68K, CS_MODE_M68K_000
from capstone.m68k import (
    M68K_OP_IMM, M68K_OP_MEM, M68K_OP_BR_DISP,
    M68K_AM_ABSOLUTE_DATA_LONG,   # =17
    M68K_AM_ABSOLUTE_DATA_SHORT,  # =16
    M68K_AM_PCI_DISP,             # =11
    M68K_AM_BRANCH_DISPLACEMENT,  # =19
)

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, ".."))
PROM = os.path.join(ROOT, "build", "mslug_prom.bin")


def load_registry():
    spec = importlib.util.spec_from_file_location(
        "registry", os.path.join(HERE, "registry.py")
    )
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m.REGISTRY


def build_covered(reg):
    """Bitmap byte a byte de la P ROM: 1 = reclamado, 0 = libre."""
    covered = bytearray(0x200000)
    ranges = []
    for name, off, size, _f in reg:
        if 0 <= off < 0x200000 and size > 0:
            end = min(off + size, 0x200000)
            for i in range(off, end):
                covered[i] = 1
            ranges.append((off, end, name))
    return covered, ranges


def target_of(ins):
    """
    Devuelve el destino absoluto de una instruccion de salto/llamada, o
    None si no lo tiene claro. Cubre los tres modos que jsr/jmp/bsr/bra
    pueden usar en 68000:

      * ABSOLUTE_LONG   (opcode 4EB9/4EF9, 6 B)  -> op.mem.disp
      * ABSOLUTE_SHORT  (opcode 4EB8/4EF8, 4 B)  -> op.mem.disp (sign-ext)
      * PC-RELATIVE     (opcode 4EBA/4EFA, 4 B)  -> ins.address + 2 + disp
      * BRANCH_DISP     (opcode 6xxx,      2/4B) -> op.br_disp.disp (ya
                                                     absoluto en capstone)
    """
    for op in ins.operands:
        am = getattr(op, "address_mode", None)
        # BRANCH_DISPLACEMENT: bsr.w/bsr.s/bra.w/bra.s -- capstone ya
        # devuelve el target absoluto en op.br_disp.disp.
        if op.type == M68K_OP_BR_DISP and am == M68K_AM_BRANCH_DISPLACEMENT:
            return op.br_disp.disp & 0xFFFFFF
        # MEM absoluto long: 6-byte instr, target explicito en el opcode.
        if op.type == M68K_OP_MEM and am == M68K_AM_ABSOLUTE_DATA_LONG:
            return op.mem.disp & 0xFFFFFF
        # MEM absoluto short: 4-byte instr, target signo-extendido a 32b.
        if op.type == M68K_OP_MEM and am == M68K_AM_ABSOLUTE_DATA_SHORT:
            d = op.mem.disp & 0xFFFF
            if d & 0x8000:
                d |= 0xFFFF0000
            return d & 0xFFFFFF
        # MEM PC-relative: 4-byte instr, target = ins.address + 2 + disp.
        # Capstone da disp signed; sumamos manualmente.
        if op.type == M68K_OP_MEM and am == M68K_AM_PCI_DISP:
            disp = op.mem.disp
            # disp es signed 16-bit; capstone lo devuelve extendido pero
            # con signo correcto ya aplicado.
            return (ins.address + 2 + disp) & 0xFFFFFF
    return None


def extract_calls(md, rom, ranges):
    """
    Recorre cada funcion matcheada linealmente y devuelve:
      calls_abs : Counter{target: veces}  aristas por jsr/jmp/bsr/bra a
                                          destinos absolutos correctamente
                                          resueltos.
      by_mode   : Counter{modo_str: veces} desglose para diagnostico
                                          del tipo de arista.
    """
    calls_abs = Counter()
    by_mode = Counter()
    for (start, end, _name) in ranges:
        chunk = bytes(rom[start:end])
        for ins in md.disasm(chunk, start):
            mn = ins.mnemonic
            # jsr, jmp, bsr, bra (con sufijos .w/.s tambien)
            if not (mn.startswith("jsr") or mn.startswith("jmp") or
                    mn.startswith("bsr") or mn == "bra" or
                    mn.startswith("bra.")):
                continue
            tgt = target_of(ins)
            if tgt is None:
                continue
            if not (0 <= tgt < 0x200000):
                continue
            calls_abs[tgt] += 1
            # Anota el modo para reporte de diagnostico
            for op in ins.operands:
                am = getattr(op, "address_mode", None)
                if am == M68K_AM_ABSOLUTE_DATA_LONG:
                    by_mode["abs.l"] += 1
                elif am == M68K_AM_ABSOLUTE_DATA_SHORT:
                    by_mode["abs.w"] += 1
                elif am == M68K_AM_PCI_DISP:
                    by_mode["pc-rel"] += 1
                elif am == M68K_AM_BRANCH_DISPLACEMENT:
                    by_mode["bsr/bra"] += 1
    return calls_abs, by_mode


def disasm_function(md, rom, start, hard_end=0x200000):
    """
    Desensambla desde `start` hasta el primer rts/rte/jmp/bra
    incondicional o hasta 128 instrucciones. Devuelve lista de
    instrucciones capstone.
    """
    out = []
    off = start
    for _ in range(128):
        if off >= hard_end:
            break
        chunk = bytes(rom[off:off + 16])
        gen = md.disasm(chunk, off)
        try:
            ins = next(gen)
        except StopIteration:
            break
        out.append(ins)
        mn = ins.mnemonic
        if mn in ("rts", "rte", "rtr"):
            break
        if mn.startswith("jmp") or mn == "bra" or mn.startswith("bra."):
            break
        off += ins.size
        if ins.size == 0:
            break
    return out


def fmt_dis(insts, limit_bytes=64):
    lines = []
    total = 0
    for ins in insts:
        lines.append(f"  {ins.address:06x}: {ins.mnemonic:<7s} {ins.op_str}")
        total += ins.size
        if total >= limit_bytes:
            break
    return "\n".join(lines)


def main():
    ap = argparse.ArgumentParser(description=__doc__.strip().splitlines()[0])
    ap.add_argument("--top", type=int, default=40,
                    help="Cuantos candidatos mostrar (default 40)")
    ap.add_argument("--max-size", type=int, default=64,
                    help="Bytes maximos de disassembly por candidato (default 64)")
    ap.add_argument("--min-addr", type=lambda x: int(x, 0), default=0x400,
                    help="Descartar targets bajo esta direccion (default 0x400: "
                         "vectores 68000 en $0..$FF + Neo Geo cartridge header "
                         "en $100..$3FF)")
    ap.add_argument("--min-size", type=int, default=20,
                    help="Descartar candidatos con desensamblado corto (default 20)")
    ap.add_argument("--show-vectors", action="store_true",
                    help="No filtrar zona baja (util para debugging)")
    ap.add_argument("--json", metavar="FILE",
                    help="Volcar el reporte completo en JSON")
    args = ap.parse_args()

    if not os.path.exists(PROM):
        sys.exit(f"Falta {PROM}. Reconstruye la P ROM procesada primero.")

    rom = open(PROM, "rb").read()
    reg = load_registry()
    covered, ranges = build_covered(reg)

    md = Cs(CS_ARCH_M68K, CS_MODE_M68K_000)
    md.detail = True

    calls, by_mode = extract_calls(md, rom, ranges)

    # Filtrar destinos ya matcheados
    unmatched_all = Counter({d: n for d, n in calls.items() if not covered[d]})

    # Aplicar filtros de plausibilidad
    min_addr = 0 if args.show_vectors else args.min_addr

    # Pre-calcular sizes para filtro --min-size
    def approx_size(dst):
        insts = disasm_function(md, rom, dst)
        return sum(x.size for x in insts)

    filtered = Counter()
    dropped_low = 0
    dropped_small = 0
    for dst, n in unmatched_all.items():
        if dst < min_addr:
            dropped_low += n
            continue
        sz = approx_size(dst)
        if sz < args.min_size:
            dropped_small += n
            continue
        filtered[dst] = n

    total_targets = len(calls)
    total_edges = sum(calls.values())
    unmatched_targets_all = len(unmatched_all)
    unmatched_edges_all = sum(unmatched_all.values())
    unmatched_targets_filt = len(filtered)
    unmatched_edges_filt = sum(filtered.values())

    print("=" * 72)
    print("Aristas de llamada extraidas de codigo matcheado:")
    print(f"  Total aristas          : {total_edges}")
    print(f"  Desglose por modo      : "
          + ", ".join(f"{k}={v}" for k, v in by_mode.most_common()))
    print(f"  Targets unicos totales : {total_targets}")
    print(f"  Ya matcheados          : {total_targets - unmatched_targets_all}")
    print(f"  NO matcheados          : {unmatched_targets_all}  "
          f"({unmatched_edges_all} aristas)")
    if not args.show_vectors:
        print(f"  ---- Tras filtros de plausibilidad ----")
        print(f"  Descartados <${min_addr:04X} : {dropped_low} aristas "
              f"(vector table + cabecera Neo Geo)")
        print(f"  Descartados <{args.min_size}B    : {dropped_small} aristas "
              f"(disasm corto)")
        print(f"  Candidatos plausibles  : {unmatched_targets_filt}  "
              f"({unmatched_edges_filt} aristas)")
    print("=" * 72)
    print(f"\nTOP {args.top} destinos NO matcheados "
          f"(por # llamadas entrantes):\n")

    top_list = []
    for i, (dst, n) in enumerate(filtered.most_common(args.top), 1):
        insts = disasm_function(md, rom, dst)
        size = sum(x.size for x in insts)
        print(f"[{i:3d}] ${dst:06x}  callers={n}  ~{size}B")
        print(fmt_dis(insts, args.max_size))
        print()
        top_list.append({"rank": i, "addr": dst, "callers": n, "size": size})

    if args.json:
        import json
        with open(args.json, "w") as f:
            json.dump({
                "matched_bytes": sum(1 for b in covered if b),
                "total_edges": total_edges,
                "by_mode": dict(by_mode),
                "unmatched_targets_all": unmatched_targets_all,
                "candidates": top_list,
            }, f, indent=2)


if __name__ == "__main__":
    main()
