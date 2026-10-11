#!/usr/bin/env python3
"""Genera un módulo .s de DATOS byte-exacto para una región de la P-ROM.

Uso:
  python3 tools/gen_data_region.py START END -o asm/FILE.s --src FILE.s \
        [--name ADDR=Nombre]* [--entry ADDR]* [--registry] [--no-verify] \
        [--min-zero N] [--wave "Wave X"] [--no-ptr]

Estrategia (datos, no código):
  * Fronteras de entrada = direcciones de la región referenciadas como
    inmediato/abs.l de 32 bits desde el código ya matcheado (REGISTRY .s/.c),
    + --entry explícitos, + inicio/fin de rachas de $00 >= --min-zero.
  * Cada entrada es una sección `.text.<Nombre>` (misma convención que el
    matcher, que coloca cada sección en su dirección CPU).
  * Cuerpo:  rachas de ceros (>= 8 B)  -> `.fill N,1,0x00`
             long alineado que coincide con una entrada del REGISTRY / un
             símbolo / una entrada de esta región -> `.dc.l Nombre`
             resto -> `.dc.w` (8 por línea), `.dc.b` en extremos impares.
  * Nombres provisionales: Zero_/PtrTab_/Str_/Data_ + dirección. Se renombran
    con --name igual que en gen_asm_region.py.
  * Verificación: ensambla + enlaza con --defsym de SYMBOLS/REGISTRY y compara
    byte a byte con build/mslug_prom.bin.
"""
import argparse, os, subprocess, sys, tempfile

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(ROOT, "tools"))
from registry import REGISTRY
from symbols import SYMBOLS
PROM = os.path.join(ROOT, "build", "mslug_prom.bin")
ROM_END = 0x200000


def collect_code_refs(rom, lo, hi):
    refs = set()
    for n, a, s, f in REGISTRY:
        if not f.endswith((".s", ".c")):
            continue
        for i in range(a, a + s - 3, 2):
            v = int.from_bytes(rom[i:i + 4], "big")
            if lo <= v < hi:
                refs.add(v)
    return refs


def zero_runs(rom, lo, hi, minlen):
    runs = []
    i = lo
    while i < hi:
        if rom[i] == 0:
            j = i
            while j < hi and rom[j] == 0:
                j += 1
            if j - i >= minlen:
                runs.append((i, j))
            i = j
        else:
            i += 1
    return runs


def is_ascii(b):
    return len(b) >= 6 and sum(1 for c in b if 0x20 <= c < 0x7F or c in (0, 0x0A, 0x0D)) >= 0.9 * len(b)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("start", type=lambda x: int(x, 16))
    ap.add_argument("end", type=lambda x: int(x, 16))
    ap.add_argument("-o", "--output", required=True)
    ap.add_argument("--src", default=None)
    ap.add_argument("--wave", default="Wave ??? (datos)")
    ap.add_argument("--name", action="append", default=[])
    ap.add_argument("--entry", action="append", default=[])
    ap.add_argument("--registry", action="store_true")
    ap.add_argument("--no-verify", action="store_true")
    ap.add_argument("--min-zero", type=int, default=32)
    ap.add_argument("--no-ptr", action="store_true", help="no emitir .dc.l Simbolo")
    ap.add_argument("--no-code-refs", action="store_true",
                    help="no usar refs desde código como fronteras (zonas de dirección baja)")
    args = ap.parse_args()
    src = args.src or os.path.basename(args.output)
    rom = open(PROM, "rb").read()
    lo, hi = args.start, args.end

    covered = sorted((a, a + s) for n, a, s, f in REGISTRY if a < hi and a + s > lo)
    gaps = []
    cur = lo
    for a, b in covered:
        if a > cur:
            gaps.append((cur, a))
        cur = max(cur, b)
    if cur < hi:
        gaps.append((cur, hi))
    if not gaps:
        print("sin huecos en el rango"); sys.exit(0)

    names = {}
    for nv in args.name:
        a, n = nv.split("=")
        names[int(a, 16)] = n

    reg_by_addr = {a: n for n, a, s, f in REGISTRY}
    sym_by_addr = dict(SYMBOLS)

    bounds = set()
    code_refs = set() if args.no_code_refs else collect_code_refs(rom, lo, hi)
    bounds |= code_refs
    for e in args.entry:
        bounds.add(int(e, 16))
    bounds |= set(names)
    zruns = []
    for g0, g1 in gaps:
        for z0, z1 in zero_runs(rom, g0, g1, args.min_zero):
            zruns.append((z0, z1))
            bounds.add(z0); bounds.add(z1)
    inside = set()
    for b in bounds:
        for g0, g1 in gaps:
            if g0 <= b < g1:
                inside.add(b); break
    for g0, g1 in gaps:
        inside.add(g0)
    entries = sorted(inside)
    zset = {z0: z1 for z0, z1 in zruns}

    ent_list = []
    for i, a in enumerate(entries):
        g1 = next(g1 for g0, g1 in gaps if g0 <= a < g1)
        b = entries[i + 1] if i + 1 < len(entries) and entries[i + 1] < g1 else g1
        ent_list.append((a, b))

    def kind(a, b):
        if a in zset and zset[a] >= b:
            return "Zero"
        seg = rom[a:b]
        n_ptr = 0
        for i in range(0, min(len(seg), 64) - 3, 4):
            v = int.from_bytes(seg[i:i + 4], "big")
            if 0x400 <= v < ROM_END and (v in reg_by_addr or v in inside or v in sym_by_addr):
                n_ptr += 1
            else:
                break
        if n_ptr >= 3:
            return "PtrTab"
        if is_ascii(seg[:min(len(seg), 32)]):
            return "Str"
        return "Data"

    ent_names = {}
    for a, b in ent_list:
        # ROM >= $100000 comparte valor numérico con RAM de trabajo ($100000..
        # $10FFFF): prefijo Rom para no confundir con los símbolos RAM.
        pre = kind(a, b)
        if 0x100000 <= a < 0x110000 and pre == "Data":
            pre = "RomData"
        ent_names[a] = names.get(a, f"{pre}_{a:06x}")

    def sym_for(v):
        if v in ent_names:
            return ent_names[v]
        if v in reg_by_addr:
            return reg_by_addr[v]
        if v in sym_by_addr:
            return sym_by_addr[v]
        return None

    L = []
    L.append("| " + "=" * 76)
    L.append("|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching — DATOS")
    L.append(f"|  {args.wave}")
    L.append(f"|  Región: ${lo:06X}..${hi:06X}  ({sum(b - a for a, b in gaps):,} B en {len(gaps)} hueco(s), {len(ent_list)} entradas)")
    L.append("| " + "=" * 76)
    L.append("|")
    L.append("|  BORRADOR generado por tools/gen_data_region.py — volcado estructurado:")
    L.append("|  .fill para rachas de $00, .dc.l Simbolo para punteros a entradas")
    L.append("|  conocidas, .dc.w para el resto. Cada entrada empieza en una dirección")
    L.append("|  referenciada desde el código (o frontera de racha de ceros).")
    L.append("|")
    L.append("        .text")
    sizes = {}
    for a, b in ent_list:
        n = ent_names[a]
        sizes[n] = (a, b - a)
        L.append("")
        L.append("| " + "-" * 76)
        L.append(f"|  {n}  @ ${a:06X}  ({b - a:,} B)" + ("  [ref. desde código]" if a in code_refs else ""))
        L.append("| " + "-" * 76)
        L.append(f'        .section .text.{n}, "ax", @progbits')
        L.append(f"        .global {n}")
        L.append(f"{n}:")
        i = a
        while i < b:
            if rom[i] == 0:
                j = i
                while j < b and rom[j] == 0:
                    j += 1
                if j - i >= 8:
                    L.append(f"        .fill   {j - i},1,0x00{'':<22}| +{i - a:05x}")
                    i = j
                    continue
            if i & 1:
                L.append(f"        .dc.b   0x{rom[i]:02x}{'':<28}| +{i - a:05x}")
                i += 1
                continue
            if not args.no_ptr and i + 4 <= b:
                v = int.from_bytes(rom[i:i + 4], "big")
                s = sym_for(v) if 0x400 <= v < ROM_END else None
                if s:
                    L.append(f"        .dc.l   {s:<30}| +{i - a:05x}  -> ${v:06X}")
                    i += 4
                    continue
            ws = []
            j = i
            while j + 1 < b and len(ws) < 8:
                if j + 4 <= b and not args.no_ptr:
                    v = int.from_bytes(rom[j:j + 4], "big")
                    if 0x400 <= v < ROM_END and sym_for(v) and ws:
                        break
                if rom[j] == 0 and j + 8 <= b and rom[j:j + 8] == b"\0" * 8 and ws:
                    break
                ws.append(int.from_bytes(rom[j:j + 2], "big"))
                j += 2
            if not ws:
                L.append(f"        .dc.b   0x{rom[i]:02x}{'':<28}| +{i - a:05x}")
                i += 1
                continue
            txt = ",".join(f"0x{w:04x}" for w in ws)
            L.append(f"        .dc.w   {txt:<60}| +{i - a:05x}")
            i = j
            if i == b - 1:
                L.append(f"        .dc.b   0x{rom[i]:02x}{'':<28}| +{i - a:05x}")
                i += 1
    open(args.output, "w").write("\n".join(L) + "\n")

    if args.registry:
        print("# --- registry")
        for a, b in ent_list:
            n = ent_names[a]
            print(f'    ("{n}",{"":<{max(1, 45 - len(n) - 3)}} 0x{a:06X}, {b - a:3d}, "{src}"),')

    if not args.no_verify:
        with tempfile.TemporaryDirectory() as td:
            o = os.path.join(td, "d.o")
            r = subprocess.run(["m68k-linux-gnu-as", "-m68000", "--register-prefix-optional",
                                args.output, "-o", o], capture_output=True, text=True)
            if r.returncode:
                print("[ASM FAIL]\n" + r.stderr); sys.exit(1)
            ld = os.path.join(td, "d.ld")
            Ls = ["OUTPUT_FORMAT(elf32-m68k)", "OUTPUT_ARCH(m68k)", "SECTIONS {"]
            for n, (a, s) in sorted(sizes.items(), key=lambda kv: kv[1][0]):
                Ls.append(f"  . = 0x{a:X};")
                Ls.append(f"  .text.{n} : {{ KEEP(*(.text.{n})) }}")
            Ls.append("  /DISCARD/ : { *(.text) *(.data*) *(.bss*) *(.comment) *(.note.*) }")
            Ls.append("}")
            open(ld, "w").write("\n".join(Ls))
            defs = [f"-Wl,--defsym={n}=0x{a:X}" for a, n in SYMBOLS.items() if n not in sizes]
            defs += [f"-Wl,--defsym={n}=0x{a:X}" for n, a, _, _ in REGISTRY if n not in sizes]
            elf = os.path.join(td, "d.elf")
            r = subprocess.run(["m68k-linux-gnu-gcc", "-m68000", "-nostdlib", "-nostartfiles",
                                "-Wl,-T", ld, *defs, o, "-o", elf], capture_output=True, text=True)
            if r.returncode:
                print("[LINK FAIL]\n" + r.stderr); sys.exit(1)
            bad = 0
            for n, (a, s) in sizes.items():
                b = os.path.join(td, n + ".bin")
                subprocess.run(["m68k-linux-gnu-objcopy", "-O", "binary",
                                f"--only-section=.text.{n}", elf, b], capture_output=True)
                got = open(b, "rb").read()[:s] if os.path.exists(b) else b""
                if got != rom[a:a + s]:
                    bad += 1
                    d = next((i for i in range(min(len(got), s)) if got[i] != rom[a + i]), min(len(got), s))
                    print(f"   [!] {n} @ ${a:06X} ({s} B) primer diff en +{d:05x} (got {len(got)} B)")
            print(f"[verify] {len(sizes) - bad}/{len(sizes)} entradas byte-exactas")
            if bad:
                sys.exit(1)
    print(f"[ok] escrito {args.output}")


if __name__ == "__main__":
    main()
