#!/usr/bin/env python3
"""
Metal Slug 1 - Multi-Stub Matcher
==================================
Matchea en lote todas las funciones stub (solo RTS) de un archivo C
que contiene multiples funciones cada una en su direccion original.

Estrategia:
  1. Compila trivial_rts.c a un solo .o
  2. Genera un linker script con secciones absolutas por funcion
     (cada funcion en su propia seccion .text.FUN_XXXXXX en su direccion)
  3. Extrae los bytes por cada funcion y compara con la ROM
"""

import os
import sys
import subprocess
import re

PROJECT_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
DECOMP_ROOT = os.path.join(PROJECT_ROOT, "decomp")
INCLUDE_DIR = os.path.join(DECOMP_ROOT, "include")
BUILD_DIR = os.path.join(DECOMP_ROOT, "build")

sys.path.insert(0, os.path.join(PROJECT_ROOT, "tools"))
from rom_loader import RomLoader

CFLAGS = [
    "-m68000", "-nostdlib", "-nostartfiles",
    "-ffreestanding", "-fno-builtin",
    "-fomit-frame-pointer", "-fno-exceptions",
    "-fno-strict-aliasing",
    "-Os", "-Wall", "-Wno-unused",
    "-ffunction-sections",  # Cada funcion en su propia seccion
    f"-I{INCLUDE_DIR}",
]


def compile_c(c_path, obj_path):
    cmd = ["m68k-linux-gnu-gcc"] + CFLAGS + ["-c", c_path, "-o", obj_path]
    r = subprocess.run(cmd, capture_output=True, text=True)
    return r.returncode == 0, r.stderr


def get_function_symbols(obj_path):
    """Devuelve dict {func_name: section_name} para funciones definidas."""
    r = subprocess.run(["m68k-linux-gnu-objdump", "-t", obj_path],
                       capture_output=True, text=True)
    funcs = {}
    # Formato: address l/g/w/... F .text.NAME size NAME
    for line in r.stdout.split('\n'):
        m = re.match(r'([0-9a-f]+)\s+\S+\s+F\s+(\S+)\s+([0-9a-f]+)\s+(\S+)', line)
        if m:
            _, section, _, name = m.groups()
            funcs[name] = section
    return funcs


def build_linker_script(func_addrs, func_sections):
    """Genera linker script que coloca cada funcion en su direccion original.

    func_addrs: {func_name: cpu_address}
    func_sections: {func_name: section_name in obj}
    """
    # Ordenar por direccion
    items = sorted(func_addrs.items(), key=lambda x: x[1])
    ld = """OUTPUT_FORMAT(elf32-m68k)
OUTPUT_ARCH(m68k)

MEMORY
{
    ROM (rx) : ORIGIN = 0x000000, LENGTH = 0x1000000
}

SECTIONS
{
"""
    for name, addr in items:
        sec = func_sections.get(name)
        if not sec:
            continue
        ld += f"    . = 0x{addr:06X};\n"
        ld += f"    .{name}_out 0x{addr:06X} : {{ *({sec}) }} > ROM\n"

    ld += """
    /DISCARD/ : {
        *(.text) *(.text.*)
        *(.data) *(.data.*)
        *(.bss) *(.bss.*)
        *(.rodata) *(.rodata.*)
        *(.comment) *(.note.*) *(.eh_frame) *(COMMON)
    }
}
"""
    return ld


def extract_bytes(elf_path, addr, size):
    """Extrae 'size' bytes en 'addr' del ELF usando objdump -s."""
    r = subprocess.run(
        ["m68k-linux-gnu-objcopy", "-O", "binary",
         f"--only-section=.{addr_to_secname(addr)}_out",
         elf_path, elf_path + f".{addr:06x}.bin"],
        capture_output=True, text=True)
    # Mejor: leer directamente del ELF con objdump -s de la direccion
    # Usar objdump para dumpear todo el ELF y extraer por direccion
    return None


def get_original_bytes(rom_dir, addr, size):
    loader = RomLoader(rom_dir)
    loader.load_all()
    prom = loader.get_p_rom_data()
    if addr < 0x100000:
        return prom[addr:addr + size]
    else:
        return prom[(addr - 0x200000) + 0x100000:(addr - 0x200000) + 0x100000 + size]


def addr_to_secname(addr):
    return f"a{addr:06x}"


def main():
    c_file = os.path.join(DECOMP_ROOT, "src", "trivial_rts.c")

    # Todas las stubs con su direccion y tamano
    STUBS = [
        ("FUN_00024fb6", 0x024FB6, 2),
        ("FUN_00025880", 0x025880, 2),
        ("FUN_00030702", 0x030702, 2),
        ("FUN_00032e8e", 0x032E8E, 2),
        ("FUN_00032f1c", 0x032F1C, 2),
        ("FUN_00032f3a", 0x032F3A, 2),
        ("FUN_00032ff0", 0x032FF0, 2),
        ("FUN_000332ba", 0x0332BA, 2),
        ("FUN_0003df54", 0x03DF54, 2),
        ("FUN_0003ee1c", 0x03EE1C, 2),
        ("FUN_000423ea", 0x0423EA, 2),
        ("FUN_000434ce", 0x0434CE, 2),
        ("FUN_000434dc", 0x0434DC, 2),
        ("FUN_00044558", 0x044558, 2),
        ("FUN_000448a4", 0x0448A4, 2),
        ("FUN_0004698a", 0x04698A, 2),
        ("FUN_000469cc", 0x0469CC, 2),
        ("FUN_0004fb3a", 0x04FB3A, 2),
        ("FUN_00051c80", 0x051C80, 2),
        ("FUN_0005204e", 0x05204E, 2),
        ("FUN_0005e8b8", 0x05E8B8, 2),
        ("FUN_0005ea94", 0x05EA94, 2),
        ("FUN_00060e44", 0x060E44, 2),
        ("FUN_00068b46", 0x068B46, 2),
        ("FUN_0006ef0e", 0x06EF0E, 2),
        ("FUN_00072de8", 0x072DE8, 2),
        ("FUN_00077c26", 0x077C26, 2),
        ("FUN_00077d86", 0x077D86, 2),
        ("FUN_0007962a", 0x07962A, 2),
        ("FUN_00079950", 0x079950, 2),
        ("FUN_00088b3c", 0x088B3C, 2),
        ("FUN_0008cc64", 0x08CC64, 2),
        ("FUN_0008d182", 0x08D182, 2),
        ("FUN_0008e4e4", 0x08E4E4, 2),
        ("FUN_0008ea4e", 0x08EA4E, 2),
        ("FUN_0008eaec", 0x08EAEC, 2),
        ("FUN_0008eb54", 0x08EB54, 2),
        ("FUN_0008f06e", 0x08F06E, 2),
        ("FUN_00099ba4", 0x099BA4, 2),
        ("FUN_00099de2", 0x099DE2, 2),
    ]

    os.makedirs(BUILD_DIR, exist_ok=True)
    obj_path = os.path.join(BUILD_DIR, "trivial_rts.o")

    # 1. Compilar con -ffunction-sections
    print(f"[1] Compilando {c_file} con -ffunction-sections...")
    ok, err = compile_c(c_file, obj_path)
    if not ok:
        print(f"    ERROR: {err}")
        sys.exit(1)
    print(f"    OK: {obj_path}")

    # 2. Obtener secciones por funcion
    print(f"[2] Detectando secciones de funciones...")
    func_sections = get_function_symbols(obj_path)
    print(f"    {len(func_sections)} funciones con seccion propia")

    # 3. Verificar cada funcion linkeandola independientemente (mas simple)
    matched = 0
    total = len(STUBS)
    total_bytes_matched = 0

    for name, addr, size in STUBS:
        sec = func_sections.get(name)
        if not sec:
            print(f"  [SKIP] {name} @ ${addr:06X}: sin seccion en el .o")
            continue

        # Generar linker script individual
        ld_content = f"""OUTPUT_FORMAT(elf32-m68k)
OUTPUT_ARCH(m68k)
MEMORY {{ ROM (rx) : ORIGIN = 0x000000, LENGTH = 0x1000000 }}
SECTIONS {{
    . = 0x{addr:06X};
    .text_here 0x{addr:06X} : {{ *({sec}) }} > ROM
    /DISCARD/ : {{
        *(.text) *(.text.*)
        *(.data) *(.data.*)
        *(.bss) *(.bss.*)
        *(.rodata) *(.rodata.*)
        *(.comment) *(.note.*) *(.eh_frame) *(COMMON)
    }}
}}
"""
        ld_path = os.path.join(BUILD_DIR, f"{name}.ld")
        with open(ld_path, 'w') as f:
            f.write(ld_content)

        elf_path = os.path.join(BUILD_DIR, f"{name}.elf")
        r = subprocess.run(
            ["m68k-linux-gnu-ld", "-T", ld_path, "-o", elf_path, obj_path],
            capture_output=True, text=True)
        if r.returncode != 0:
            print(f"  [ERR-LINK] {name}: {r.stderr.strip()}")
            continue

        bin_path = elf_path + ".bin"
        r = subprocess.run(
            ["m68k-linux-gnu-objcopy", "-O", "binary",
             "--only-section=.text_here", elf_path, bin_path],
            capture_output=True, text=True)
        if r.returncode != 0:
            print(f"  [ERR-OBJCOPY] {name}: {r.stderr.strip()}")
            continue

        with open(bin_path, 'rb') as f:
            compiled = f.read()[:size]

        original = get_original_bytes(os.path.join(PROJECT_ROOT, "rom"), addr, size)

        if compiled == original:
            print(f"  [OK] {name} @ ${addr:06X} ({size}B) : {compiled.hex()}")
            matched += 1
            total_bytes_matched += size
        else:
            print(f"  [!!] {name} @ ${addr:06X} : orig={original.hex()} comp={compiled.hex()}")

    print()
    print(f"=== RESULTADO: {matched}/{total} funciones matched ({total_bytes_matched} bytes) ===")


if __name__ == "__main__":
    main()
