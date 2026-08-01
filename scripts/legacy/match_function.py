#!/usr/bin/env python3
"""
Metal Slug 1 - Function Matching Verifier
==========================================
Compila una funcion C individual, la linkea con las direcciones correctas
de sus dependencias, y compara sus bytes con los del ROM original.

Reporta MATCHED (100% identica) o NON-MATCHED con diff byte a byte.
"""

import os
import sys
import subprocess
import argparse

PROJECT_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
DECOMP_ROOT = os.path.join(PROJECT_ROOT, "decomp")
INCLUDE_DIR = os.path.join(DECOMP_ROOT, "include")
BUILD_DIR = os.path.join(DECOMP_ROOT, "build")

sys.path.insert(0, os.path.join(PROJECT_ROOT, "tools"))
from rom_loader import RomLoader

# Direcciones de simbolos conocidos (para el linker)
# El linker debe resolver los JSR a estas direcciones exactas
SYMBOL_ADDRESSES = {
    # Funciones del BIOS
    "FIX_CLEAR":         0xC004C2,
    "SYSTEM_INT1":       0xC00438,
    "SYSTEM_INT2":       0xC0043E,
    "SYSTEM_RETURN":     0xC00444,
    "SYSTEM_IO":         0xC0044A,
    "MESS_OUT":          0xC004CE,
    "CREDIT_CHECK":      0xC00450,
    "CREDIT_DOWN":       0xC00456,
    # Funciones del juego (entry points conocidos)
    "GameInit":          0x0007CC,
    "GameStart":         0x00084A,
    "GameReturn":        0x000852,
    "ClearUserRam":      0x000868,
    "DivZeroHandler":    0x0008A4,
    "ChkHandler":        0x0008AE,
    "TrapvHandler":      0x0008B8,
    "Irq2Handler":       0x0008D6,
    "VBlankHandler":     0x0008F6,
    "CheckTimer":        0x000960,
    "GameFrame":         0x00097A,
    "ResetIrqCallback":  0x0009A8,
    "VBlankUpdate":      0x001E5E,
    "GameLogicUpdate":   0x001EFE,
    "UpdateFrameCounter":0x00226A,
    # Funciones auxiliares
    "FUN_000020e2":      0x0020E2,
    "FUN_0000212e":      0x00212E,
    "FUN_000137c6":      0x0137C6,
    "FUN_0005c9d6":      0x05C9D6,
    "FUN_00099afc":      0x099AFC,
}

CFLAGS = [
    "-m68000",
    "-nostdlib", "-nostartfiles",
    "-ffreestanding", "-fno-builtin",
    "-fomit-frame-pointer",
    "-fno-exceptions",
    "-fno-strict-aliasing",
    "-Os",
    "-Wall", "-Wno-unused",
    f"-I{INCLUDE_DIR}",
]


def compile_c_file(c_path, obj_path):
    """Compila un archivo C a object file."""
    cmd = ["m68k-linux-gnu-gcc"] + CFLAGS + ["-c", c_path, "-o", obj_path]
    r = subprocess.run(cmd, capture_output=True, text=True)
    return r.returncode == 0, r.stderr


def build_linker_script(func_addr, func_size, symbols_used):
    """Genera un linker script que coloca la funcion en su direccion original
    y define todos los simbolos externos en sus direcciones correctas."""
    ld = f"""
OUTPUT_FORMAT(elf32-m68k)
OUTPUT_ARCH(m68k)

MEMORY
{{
    ROM (rx) : ORIGIN = 0x000000, LENGTH = 0x1000000
}}

SECTIONS
{{
    . = 0x{func_addr:06X};
    .text : {{
        KEEP(*(.text.func_start))
        *(.text)
        *(.text.*)
    }} > ROM

    /DISCARD/ : {{
        *(.data)
        *(.data.*)
        *(.bss)
        *(.bss.*)
        *(.rodata)
        *(.rodata.*)
        *(.comment)
        *(.note.*)
        *(.eh_frame)
        *(COMMON)
    }}
}}
"""
    # Definir simbolos externos en sus direcciones
    for sym in symbols_used:
        if sym in SYMBOL_ADDRESSES:
            ld = ld.rstrip() + f"\nPROVIDE({sym} = 0x{SYMBOL_ADDRESSES[sym]:06X});"
    return ld + "\n"


def get_undefined_symbols(obj_path):
    """Devuelve los simbolos externos referenciados en el .o."""
    r = subprocess.run(["m68k-linux-gnu-nm", obj_path], capture_output=True, text=True)
    undefined = []
    for line in r.stdout.split('\n'):
        parts = line.strip().split()
        if len(parts) == 2 and parts[0] == "U":
            undefined.append(parts[1])
    return undefined


def link_function(obj_path, elf_path, func_addr, symbols_used):
    """Linkea el .o en la direccion correcta."""
    ld_path = obj_path.replace('.o', '.ld')
    with open(ld_path, 'w') as f:
        f.write(build_linker_script(func_addr, 0, symbols_used))

    cmd = ["m68k-linux-gnu-ld", "-T", ld_path, "-o", elf_path, obj_path]
    r = subprocess.run(cmd, capture_output=True, text=True)
    return r.returncode == 0, r.stderr


def extract_function_bytes_from_elf(elf_path, func_addr, func_size):
    """Extrae los bytes de la funcion del ELF linkeado."""
    bin_path = elf_path + ".bin"
    r = subprocess.run(["m68k-linux-gnu-objcopy", "-O", "binary",
                        "--only-section=.text", elf_path, bin_path],
                       capture_output=True, text=True)
    if r.returncode != 0:
        return None, r.stderr

    with open(bin_path, "rb") as f:
        data = f.read()

    return data[:func_size], None


def get_original_bytes(rom_dir, addr, size):
    """Extrae los bytes originales de la ROM procesada (byte-swap + reordenamiento)."""
    loader = RomLoader(rom_dir)
    loader.load_all()
    prom = loader.get_p_rom_data()

    if addr < 0x100000:
        return prom[addr:addr + size]
    else:
        return prom[(addr - 0x200000) + 0x100000:(addr - 0x200000) + 0x100000 + size]


def format_diff(original, compiled):
    """Diff byte-a-byte formateado."""
    lines = []
    max_len = max(len(original), len(compiled))

    for i in range(0, max_len, 16):
        orig_chunk = original[i:i+16] if i < len(original) else b''
        comp_chunk = compiled[i:i+16] if i < len(compiled) else b''

        marker = ''
        for j in range(min(len(orig_chunk), len(comp_chunk))):
            marker += 'XX' if orig_chunk[j] != comp_chunk[j] else '  '

        match = orig_chunk == comp_chunk
        prefix = "OK" if match else "!!"
        lines.append(f"  {prefix} +{i:04X}: orig={orig_chunk.hex()}")
        lines.append(f"           comp={comp_chunk.hex()}")
        if not match:
            lines.append(f"           diff={marker}")

    return '\n'.join(lines)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("c_file")
    parser.add_argument("symbol")
    parser.add_argument("address")
    parser.add_argument("size", type=int)
    parser.add_argument("--rom", default=os.path.join(PROJECT_ROOT, "rom"))
    args = parser.parse_args()

    addr = int(args.address, 16)

    obj_path = os.path.join(BUILD_DIR, os.path.basename(args.c_file).replace('.c', '.o'))
    elf_path = obj_path.replace('.o', '.elf')
    os.makedirs(BUILD_DIR, exist_ok=True)

    # 1. Compilar
    print(f"[1] Compilando {args.c_file}...")
    ok, err = compile_c_file(args.c_file, obj_path)
    if not ok:
        print(f"    ERROR: {err}")
        sys.exit(1)
    print(f"    OK: {obj_path}")

    # 2. Detectar simbolos externos
    print(f"[2] Detectando simbolos externos...")
    undefined = get_undefined_symbols(obj_path)
    print(f"    {len(undefined)} simbolos referenciados: {undefined}")

    # Filtrar solo los que tenemos direcciones
    known = [s for s in undefined if s in SYMBOL_ADDRESSES]
    unknown = [s for s in undefined if s not in SYMBOL_ADDRESSES]
    if unknown:
        print(f"    Simbolos sin direccion (asumir 0): {unknown}")

    # 3. Linkear con direcciones correctas
    print(f"[3] Linkeando en ${addr:06X}...")
    ok, err = link_function(obj_path, elf_path, addr, undefined)
    if not ok:
        print(f"    ERROR: {err}")
        sys.exit(1)
    print(f"    OK: {elf_path}")

    # 4. Extraer bytes compilados
    print(f"[4] Extrayendo bytes compilados...")
    compiled_bytes, err = extract_function_bytes_from_elf(elf_path, addr, args.size)
    if compiled_bytes is None:
        print(f"    ERROR: {err}")
        sys.exit(1)
    print(f"    OK: {len(compiled_bytes)} bytes")

    # 5. Extraer bytes originales
    print(f"[5] Extrayendo bytes originales de la ROM...")
    original_bytes = get_original_bytes(args.rom, addr, args.size)
    print(f"    OK: {len(original_bytes)} bytes")

    # 6. Comparar
    print(f"\n=== Comparacion: {args.symbol} @ ${addr:06X} ===")

    if compiled_bytes == original_bytes:
        print(f"\n[MATCHED] {args.symbol} coincide 100% bit-a-bit ({args.size} bytes)!")
        sys.exit(0)

    cmp_len = min(len(compiled_bytes), len(original_bytes))
    matches = sum(1 for i in range(cmp_len) if compiled_bytes[i] == original_bytes[i])
    pct = matches / cmp_len * 100 if cmp_len > 0 else 0

    print(f"\n[NON-MATCHED] Coincidencia: {matches}/{cmp_len} bytes ({pct:.1f}%)")
    print(f"\nDiff detallado:")
    print(format_diff(original_bytes, compiled_bytes))
    sys.exit(1)


if __name__ == "__main__":
    main()
