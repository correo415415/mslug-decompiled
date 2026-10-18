#!/usr/bin/env python3
"""
build_rom.py — Reconstruye la P-ROM completa (2 MiB, formato CPU) a partir
de las fuentes del repositorio (asm/*.s + src/*.c) usando el registro
(tools/registry.py) como mapa de colocación.

Uso:
    python3 tools/build_rom.py                 # construye y compara con build/mslug_prom.bin
    python3 tools/build_rom.py -o out.bin      # salida alternativa
    python3 tools/build_rom.py --no-compare    # no comparar (ROM modificada a propósito)
    python3 tools/build_rom.py -j 8            # paralelismo de compilación

Salida por defecto: build/mslug_prom_rebuilt.bin  (formato CPU).
Para obtener el fichero que carga MAME (201-p1.bin):
    bash scripts/pack_prom.sh build/mslug_prom_rebuilt.bin
"""
import os, sys, argparse, hashlib, subprocess, tempfile, importlib.util
from concurrent.futures import ThreadPoolExecutor

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

def load_module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m

mb = load_module("match_batch", os.path.join(ROOT, "tools", "match_batch.py"))
REGISTRY, SYMBOLS = mb.REGISTRY, mb.SYMBOLS
ROM_SIZE = 0x200000

def compile_one(td, src):
    ext = os.path.splitext(src)[1].lower()
    obj = os.path.join(td, src.replace("/", "__") + ".o")
    if ext == ".s":
        cmd = ["m68k-linux-gnu-gcc", "-m68000", "-nostdlib", "-nostartfiles",
               "-ffreestanding", "-Wa,--register-prefix-optional",
               f"-I{mb.INC}", "-c", os.path.join(mb.ASM, src), "-o", obj]
    else:
        cmd = ["m68k-linux-gnu-gcc", *mb.CFLAGS, *mb.PER_FILE_CFLAGS.get(src, []),
               "-c", os.path.join(mb.SRC, src), "-o", obj]
    r = subprocess.run(cmd, capture_output=True, text=True)
    return src, obj, r

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("-o", "--output",
                    default=os.path.join(mb.BUILD, "mslug_prom_rebuilt.bin"))
    ap.add_argument("--no-compare", action="store_true")
    ap.add_argument("--reference", default=mb.PROM)
    ap.add_argument("-j", "--jobs", type=int, default=os.cpu_count() or 2)
    a = ap.parse_args()

    sources = sorted(set(e[3] for e in REGISTRY))
    print(f"[build_rom] {len(REGISTRY)} entradas, {len(sources)} fuentes, -j{a.jobs}")

    with tempfile.TemporaryDirectory() as td:
        objs = []
        with ThreadPoolExecutor(max_workers=a.jobs) as ex:
            for src, obj, r in ex.map(lambda s: compile_one(td, s), sources):
                if r.returncode:
                    print(f"[COMPILE FAIL] {src}\n{r.stdout}{r.stderr}")
                    sys.exit(1)
                objs.append(obj)
        print(f"[build_rom] compiladas {len(objs)} unidades")

        # Linker script: una sección por entrada, en su dirección CPU.
        ld = os.path.join(td, "rom.ld")
        lines = ["OUTPUT_FORMAT(elf32-m68k)", "OUTPUT_ARCH(m68k)", "SECTIONS {"]
        for name, cpu, size, src in sorted(REGISTRY, key=lambda e: e[1]):
            lines.append(f"  . = 0x{cpu:X};")
            lines.append(f"  .text.{name} : {{ KEEP(*(.text.{name})) }}")
        lines.append(f"  . = 0x{ROM_SIZE:X};")
        lines.append("  .rom_end : { BYTE(0) }")
        lines.append("  /DISCARD/ : { *(.text) *(.text.*) *(.data*) *(.bss*) *(.rodata*)")
        lines.append("                *(.comment) *(.eh_frame) *(.note.*) }")
        lines.append("}")
        with open(ld, "w") as f:
            f.write("\n".join(lines))

        elf = os.path.join(td, "rom.elf")
        r = subprocess.run(["m68k-linux-gnu-gcc", "-m68000", "-nostdlib",
                            "-nostartfiles", "-Wl,-T", ld, *mb.build_defsyms(),
                            *objs, "-o", elf], capture_output=True, text=True)
        if r.returncode:
            print("[LINK FAIL]\n" + r.stdout + r.stderr)
            sys.exit(1)

        # Toda entrada del registro debe existir en el ELF con su tamaño exacto:
        # el linker script sólo conserva (KEEP) las secciones del registro, así
        # que una fuente con un nombre viejo/distinto se descartaría en silencio
        # y su hueco quedaría a 0x00 (también con --no-compare).
        r = subprocess.run(["m68k-linux-gnu-objdump", "-h", elf],
                           capture_output=True, text=True)
        got = {}
        for ln in r.stdout.splitlines():
            f = ln.split()
            if len(f) >= 4 and f[1].startswith(".text."):
                got[f[1][6:]] = int(f[2], 16)
        bad = [(n, sz, got.get(n)) for n, cpu, sz, src in REGISTRY if got.get(n) != sz]
        if bad:
            print(f"[SECTION FAIL] {len(bad)} entradas del registro ausentes o con tamaño distinto:")
            for n, sz, g in bad[:20]:
                print(f"    {n}: registro {sz} B, ELF {'AUSENTE' if g is None else str(g) + ' B'}")
            sys.exit(1)

        raw = os.path.join(td, "rom.raw")
        r = subprocess.run(["m68k-linux-gnu-objcopy", "-O", "binary",
                            "--gap-fill", "0x00", "-R", ".rom_end", elf, raw],
                           capture_output=True, text=True)
        if r.returncode:
            print("[OBJCOPY FAIL]\n" + r.stdout + r.stderr)
            sys.exit(1)
        with open(raw, "rb") as f:
            data = f.read()

    # objcopy empieza en la primera sección (debe ser $000000) — normalizamos.
    data = (data + b"\x00" * ROM_SIZE)[:ROM_SIZE]
    os.makedirs(os.path.dirname(os.path.abspath(a.output)), exist_ok=True)
    with open(a.output, "wb") as f:
        f.write(data)
    md5 = hashlib.md5(data).hexdigest()
    print(f"[build_rom] escrito {a.output}  ({len(data):,} B)  MD5 = {md5}")

    if a.no_compare:
        return 0
    with open(a.reference, "rb") as f:
        ref = f.read()
    if ref == data:
        print("[build_rom] ✔ IDÉNTICA a", a.reference)
        return 0
    diffs = [i for i in range(min(len(ref), len(data))) if ref[i] != data[i]]
    print(f"[build_rom] ✘ DIFIERE: {len(diffs):,} bytes distintos; primer diff en "
          f"${diffs[0]:06X} (ref={ref[diffs[0]]:02X} got={data[diffs[0]]:02X})"
          if diffs else f"[build_rom] ✘ tamaño distinto: {len(ref)} vs {len(data)}")
    return 1

if __name__ == "__main__":
    sys.exit(main())
