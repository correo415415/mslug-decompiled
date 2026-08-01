#!/usr/bin/env python3
"""
Metal Slug 1 — Matcher por lotes (C + ASM).
============================================
Compila todas las fuentes bajo decomp/src/ (.c) y decomp/asm/ (.s), enlaza
cada símbolo del registro maestro en su dirección CPU absoluta y compara
byte-a-byte con la porción correspondiente del P ROM procesado.

TOOLCHAIN OFICIAL DEL PROYECTO
------------------------------
GCC bare-metal 68000: `m68k-linux-gnu-gcc` (Debian/Ubuntu 13.3.0),
usado en modo freestanding:  `-mcpu=68000 -nostdlib -nostartfiles
-ffreestanding -fno-builtin -fomit-frame-pointer -Os`.
Es el sustituto local del `m68k-neogeo-elf-gcc` de ngdevkit: mismo
codegen 68000 (mismos opcodes emitidos), solo cambia el runtime — que
no usamos porque ligamos con nuestro propio linker script.

Doble frente:
  - decomp/src/*.c   : funciones cuya salida GCC coincide byte-a-byte
                       con el ROM (thunks, stubs, dispatchers, …).
  - decomp/asm/*.s   : funciones semánticas del juego cuya secuencia
                       exacta de instrucciones NO es rederivable por
                       GCC 1:1 y se escribe como asm 68000 puro
                       (sintaxis GAS m68k, --register-prefix-optional).

Registro maestro: `decomp/tools/registry.py` — lista de tuplas
    (nombre_símbolo, direccion_cpu, tamano_bytes, archivo_fuente)

Uso:
    python3 decomp/tools/match_batch.py
"""
import os, sys, subprocess, tempfile, importlib.util, json

ROOT   = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
INC    = os.path.join(ROOT, "include")
SRC    = os.path.join(ROOT, "src")
ASM    = os.path.join(ROOT, "asm")
BUILD  = os.path.join(ROOT, "build")
PROM   = os.path.join(BUILD, "mslug_prom.bin")

def load_module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m); return m

SYMBOLS  = load_module("symbols",  os.path.join(ROOT, "tools", "symbols.py")).SYMBOLS
REGISTRY = load_module("registry", os.path.join(ROOT, "tools", "registry.py")).REGISTRY

CFLAGS = [
    "-m68000", "-nostdlib", "-nostartfiles",
    "-ffreestanding", "-fno-builtin", "-fno-exceptions",
    "-fomit-frame-pointer", "-fno-strict-aliasing",
    "-fno-toplevel-reorder", "-fno-common",
    "-Os", "-Wall", "-Wno-unused",
    "-ffunction-sections", "-fdata-sections",
    # NOTA: -mpcrel NO va aquí — forzaría PC-rel 16-bit incluso para
    # llamadas fuera de rango (p. ej. jsr al BIOS en $C004C2 desde $000097A
    # está fuera del rango 16-bit con signo). Se aplica por archivo vía
    # PER_FILE_CFLAGS.
    f"-I{INC}",
]

# Flags adicionales por unidad de compilación. Clave = basename del .c.
PER_FILE_CFLAGS = {
    "reset_irq_callback.c": ["-mpcrel"],   # lea pc+d,a0 corto (4 B)
    "task_handlers.c":      ["-mpcrel"],   # lea pc+d,a1 corto (4 B) x 731
    "jsr_pc_thunks.c":      ["-mpcrel"],   # jsr pc+d,pc corto (4 B) x 164
}

def sh(cmd, **kw):
    r = subprocess.run(cmd, capture_output=True, text=True, **kw)
    return r

def build_linker_script(td):
    """Coloca .text.<Sym> en cpu_addr para cada entrada del registro."""
    ld = os.path.join(td, "batch.ld")
    lines = ["OUTPUT_FORMAT(elf32-m68k)", "OUTPUT_ARCH(m68k)", "SECTIONS {"]
    # Ordenar por dirección CPU (necesario para el linker)
    for name, cpu, size, src in sorted(REGISTRY, key=lambda e: e[1]):
        lines.append(f"  . = 0x{cpu:X};")
        lines.append(f"  .text.{name} : {{ KEEP(*(.text.{name})) }}")
    lines.append("  /DISCARD/ : { *(.text) *(.text.*) *(.data*) *(.bss*) *(.rodata*)")
    lines.append("                *(.comment) *(.eh_frame) *(.note.*) }")
    lines.append("}")
    with open(ld, "w") as f: f.write("\n".join(lines))
    return ld

def build_defsyms():
    return [f"-Wl,--defsym={name}=0x{addr:X}" for addr, name in SYMBOLS.items()]

def main():
    with open(PROM, "rb") as f: prom = f.read()

    with tempfile.TemporaryDirectory() as td:
        objs = []
        # Compilar cada fuente única del registro (C o ASM).
        # Convención: si termina en .s, vive en decomp/asm/; si en .c,
        # en decomp/src/. Se resuelve automáticamente.
        sources = sorted(set(e[3] for e in REGISTRY))
        for src in sources:
            ext = os.path.splitext(src)[1].lower()
            if ext == ".s":
                src_path = os.path.join(ASM, src)
                # Ensamblador puro: sin flags de optimización C, solo -m68000.
                # --register-prefix-optional permite usar 'a0' en lugar de '%a0'.
                obj = os.path.join(td, os.path.basename(src) + ".o")
                r = sh(["m68k-linux-gnu-gcc", "-m68000", "-nostdlib",
                        "-nostartfiles", "-ffreestanding",
                        "-Wa,--register-prefix-optional",
                        f"-I{INC}", "-c", src_path, "-o", obj])
            else:
                src_path = os.path.join(SRC, src)
                obj = os.path.join(td, os.path.basename(src) + ".o")
                extra = PER_FILE_CFLAGS.get(src, [])
                r = sh(["m68k-linux-gnu-gcc", *CFLAGS, *extra,
                        "-c", src_path, "-o", obj])
            if r.returncode:
                print(f"[COMPILE FAIL] {src}")
                print(r.stdout + r.stderr); sys.exit(1)
            objs.append(obj)

        # Linker script
        ld = build_linker_script(td)
        defsyms = build_defsyms()
        elf = os.path.join(td, "batch.elf")
        r = sh(["m68k-linux-gnu-gcc", "-m68000", "-nostdlib", "-nostartfiles",
                "-Wl,-T", ld, *defsyms, *objs, "-o", elf])
        if r.returncode:
            print("[LINK FAIL]"); print(r.stdout + r.stderr); sys.exit(1)

        # Extraer cada sección con objcopy y comparar
        matched, non_matched, total_bytes, matched_bytes = 0, [], 0, 0
        details = []
        for name, cpu, size, src in sorted(REGISTRY, key=lambda e: e[1]):
            total_bytes += size
            binf = os.path.join(td, name + ".bin")
            r = sh(["m68k-linux-gnu-objcopy", "-O", "binary",
                    f"--only-section=.text.{name}", elf, binf])
            if r.returncode or not os.path.exists(binf):
                non_matched.append((name, cpu, size, "objcopy failed"))
                continue
            with open(binf, "rb") as f: got = f.read()[:size]
            want = prom[cpu:cpu+size]
            if got == want and len(got) == size:
                matched += 1
                matched_bytes += size
                details.append({"name":name, "addr":f"0x{cpu:06X}", "size":size,
                                "src":src, "status":"MATCHED"})
            else:
                non_matched.append((name, cpu, size,
                    f"want={want.hex()} got={got.hex()}"))
                details.append({"name":name, "addr":f"0x{cpu:06X}", "size":size,
                                "src":src, "status":"NO_MATCH",
                                "want":want.hex(), "got":got.hex()})

        rom_pct = matched_bytes / 2097152 * 100
        print("="*68)
        print(f"  MATCHED : {matched}/{len(REGISTRY)} funciones")
        print(f"  BYTES   : {matched_bytes:,}/{total_bytes:,} (registrados)")
        print(f"  ROM     : {matched_bytes:,}/2,097,152  ({rom_pct:.4f}%)")
        print("="*68)
        for nm, cpu, sz, info in non_matched[:20]:
            print(f"  [!] {nm} @ ${cpu:06X}  ({sz} B)  {info}")
        if len(non_matched) > 20:
            print(f"  ... y {len(non_matched)-20} más")

        # Guardar reporte
        os.makedirs(BUILD, exist_ok=True)
        with open(os.path.join(BUILD, "match_report_c.json"), "w") as f:
            json.dump({
                "matched": matched, "total": len(REGISTRY),
                "matched_bytes": matched_bytes, "total_bytes": total_bytes,
                "rom_pct": rom_pct, "details": details,
            }, f, indent=2)
        sys.exit(0 if matched == len(REGISTRY) else 1)

if __name__ == "__main__":
    main()
