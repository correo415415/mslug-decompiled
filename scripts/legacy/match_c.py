#!/usr/bin/env python3
"""
Metal Slug 1 — Matcher unitario para el pipeline "C puro".
============================================================
Compila una unidad (.c), enlaza el objeto en su dirección de ROM real
resolviendo todos los símbolos externos con direcciones absolutas
(desde `decomp/tools/symbols.py`), extrae los bytes de la función y
los compara byte-a-byte con la porción exacta del P ROM procesado.

Uso:
    python3 decomp/tools/match_c.py <src.c> <FuncName> <cpu_addr_hex> <size>
"""
import os, sys, subprocess, tempfile, importlib.util

ROOT   = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
INC    = os.path.join(ROOT, "decomp", "include")
BUILD  = os.path.join(ROOT, "decomp", "build")
PROM   = os.path.join(BUILD, "mslug_prom.bin")

# Cargar la tabla de símbolos (dirección CPU absoluta -> nombre)
sym_path = os.path.join(ROOT, "decomp", "tools", "symbols.py")
spec = importlib.util.spec_from_file_location("symbols", sym_path)
_symbols = importlib.util.module_from_spec(spec); spec.loader.exec_module(_symbols)
SYMBOLS = _symbols.SYMBOLS

CFLAGS = [
    "-m68000", "-nostdlib", "-nostartfiles",
    "-ffreestanding", "-fno-builtin", "-fno-exceptions",
    "-fomit-frame-pointer", "-fno-strict-aliasing",
    "-fno-toplevel-reorder", "-fno-common",
    "-Os", "-Wall", "-Wno-unused", "-Wno-register",
    "-ffunction-sections", "-fdata-sections",
    # -mpcrel se aplica por archivo via PER_FILE_CFLAGS.
    f"-I{INC}",
]

# Flags extra por unidad de compilación (basename del .c)
PER_FILE_CFLAGS = {
    "reset_irq_callback.c": ["-mpcrel"],
}

def sh(cmd, **kw):
    r = subprocess.run(cmd, capture_output=True, text=True, **kw)
    if r.returncode:
        sys.stderr.write("[cmd] " + " ".join(cmd) + "\n")
        sys.stderr.write(r.stdout + r.stderr)
    return r

def build_linker_script(td, func, cpu):
    """Linker script mínimo: coloca .text.<func> exactamente en su dirección CPU."""
    ld = os.path.join(td, "one.ld")
    with open(ld, "w") as f:
        f.write(f"""
OUTPUT_FORMAT(elf32-m68k)
OUTPUT_ARCH(m68k)
SECTIONS {{
  . = 0x{cpu:X};
  .text.{func} : {{ KEEP(*(.text.{func})) }}
  /DISCARD/ : {{ *(.text) *(.text.*) *(.data*) *(.bss*) *(.rodata*)
                *(.comment) *(.eh_frame) *(.note.*) }}
}}
""")
    return ld

def build_symbol_defs(td):
    """Fichero de defsym: -Wl,--defsym=Name=0xADDR para cada símbolo conocido."""
    args = []
    for addr, name in SYMBOLS.items():
        args.append(f"-Wl,--defsym={name}=0x{addr:X}")
    return args

def main():
    if len(sys.argv) != 5:
        print(__doc__); sys.exit(2)
    src, func, cpu_hex, size = sys.argv[1], sys.argv[2], sys.argv[3], int(sys.argv[4])
    cpu = int(cpu_hex, 16)

    with tempfile.TemporaryDirectory() as td:
        obj = os.path.join(td, "a.o")
        elf = os.path.join(td, "a.elf")
        bin_ = os.path.join(td, "a.bin")

        # 1) Compilar (con flags extra si el archivo los declara)
        extra = PER_FILE_CFLAGS.get(os.path.basename(src), [])
        r = sh(["m68k-linux-gnu-gcc", *CFLAGS, *extra, "-c", src, "-o", obj])
        if r.returncode:
            print("[FAIL] compilación falló"); sys.exit(1)

        # 2) Linkear en la dirección CPU real, resolviendo externos con defsym
        ld = build_linker_script(td, func, cpu)
        defsyms = build_symbol_defs(td)
        r = sh(["m68k-linux-gnu-gcc", "-m68000", "-nostdlib", "-nostartfiles",
                "-Wl,-T", ld, *defsyms, obj, "-o", elf])
        if r.returncode:
            print("[FAIL] link falló"); sys.exit(1)

        # 3) Extraer solo la sección .text.<func>
        r = sh(["m68k-linux-gnu-objcopy", "-O", "binary",
                f"--only-section=.text.{func}", elf, bin_])
        if r.returncode:
            print("[FAIL] objcopy falló"); sys.exit(1)

        with open(bin_, "rb") as f:
            got = f.read()

        with open(PROM, "rb") as f:
            f.seek(cpu); want = f.read(size)

        # Recortar / diagnosticar
        if len(got) < size:
            print(f"[FAIL] función compilada mide {len(got)} B, esperados {size} B")
            print("  got:", got.hex())
            sys.exit(1)
        got_trim = got[:size]

        print(f"  want ${cpu:06X}: {want.hex()}")
        print(f"  got  {func:>18s}: {got_trim.hex()}")

        if got_trim == want:
            extra = len(got) - size
            tail = "" if extra == 0 else f"  (+{extra} B extra tras la función, ignorados)"
            print(f"[MATCHED]  {func} @ ${cpu:06X}  ({size} B){tail}")
            sys.exit(0)

        # Diff visual
        marks = "".join("  " if a == b else "^^" for a, b in zip(got_trim, want))
        print(f"  diff:                {marks}")
        print(f"[NO MATCH] {func} @ ${cpu:06X}")
        sys.exit(1)

if __name__ == "__main__":
    main()
