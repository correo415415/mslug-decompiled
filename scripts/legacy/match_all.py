#!/usr/bin/env python3
"""
Metal Slug 1 - Universal Batch Matcher
========================================
Compila TODOS los archivos .c de decomp/src, y verifica cada funcion
declarada contra los bytes originales de la ROM.

Cada funcion debe estar declarada con:
    __attribute__((naked, section(".text.FUN_XXXXXX"))) ...

y registrada en el diccionario FUNCTIONS con (nombre, direccion, tamano).
"""

import os
import sys
import subprocess
import json

PROJECT_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
DECOMP_ROOT = os.path.join(PROJECT_ROOT, "decomp")
SRC_DIR = os.path.join(DECOMP_ROOT, "src")
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
    "-ffunction-sections",
    f"-I{INCLUDE_DIR}",
]

# Registro maestro: (nombre, direccion CPU, tamano en bytes, archivo fuente)
FUNCTIONS = [
    # === Grandes ya matched (compilacion individual, referencias externas) ===
    ("GameFrame",         0x00097A, 46, "game_frame.c"),
    ("ResetIrqCallback",  0x0009A8, 12, "reset_irq_callback.c"),
    ("FUN_000267e2",      0x0267E2, 18, "entity_reset_state.c"),

    # === 40 funciones stub (solo rts, 2 bytes) ===
    ("FUN_00024fb6", 0x024FB6, 2, "trivial_rts.c"),
    ("FUN_00025880", 0x025880, 2, "trivial_rts.c"),
    ("FUN_00030702", 0x030702, 2, "trivial_rts.c"),
    ("FUN_00032e8e", 0x032E8E, 2, "trivial_rts.c"),
    ("FUN_00032f1c", 0x032F1C, 2, "trivial_rts.c"),
    ("FUN_00032f3a", 0x032F3A, 2, "trivial_rts.c"),
    ("FUN_00032ff0", 0x032FF0, 2, "trivial_rts.c"),
    ("FUN_000332ba", 0x0332BA, 2, "trivial_rts.c"),
    ("FUN_0003df54", 0x03DF54, 2, "trivial_rts.c"),
    ("FUN_0003ee1c", 0x03EE1C, 2, "trivial_rts.c"),
    ("FUN_000423ea", 0x0423EA, 2, "trivial_rts.c"),
    ("FUN_000434ce", 0x0434CE, 2, "trivial_rts.c"),
    ("FUN_000434dc", 0x0434DC, 2, "trivial_rts.c"),
    ("FUN_00044558", 0x044558, 2, "trivial_rts.c"),
    ("FUN_000448a4", 0x0448A4, 2, "trivial_rts.c"),
    ("FUN_0004698a", 0x04698A, 2, "trivial_rts.c"),
    ("FUN_000469cc", 0x0469CC, 2, "trivial_rts.c"),
    ("FUN_0004fb3a", 0x04FB3A, 2, "trivial_rts.c"),
    ("FUN_00051c80", 0x051C80, 2, "trivial_rts.c"),
    ("FUN_0005204e", 0x05204E, 2, "trivial_rts.c"),
    ("FUN_0005e8b8", 0x05E8B8, 2, "trivial_rts.c"),
    ("FUN_0005ea94", 0x05EA94, 2, "trivial_rts.c"),
    ("FUN_00060e44", 0x060E44, 2, "trivial_rts.c"),
    ("FUN_00068b46", 0x068B46, 2, "trivial_rts.c"),
    ("FUN_0006ef0e", 0x06EF0E, 2, "trivial_rts.c"),
    ("FUN_00072de8", 0x072DE8, 2, "trivial_rts.c"),
    ("FUN_00077c26", 0x077C26, 2, "trivial_rts.c"),
    ("FUN_00077d86", 0x077D86, 2, "trivial_rts.c"),
    ("FUN_0007962a", 0x07962A, 2, "trivial_rts.c"),
    ("FUN_00079950", 0x079950, 2, "trivial_rts.c"),
    ("FUN_00088b3c", 0x088B3C, 2, "trivial_rts.c"),
    ("FUN_0008cc64", 0x08CC64, 2, "trivial_rts.c"),
    ("FUN_0008d182", 0x08D182, 2, "trivial_rts.c"),
    ("FUN_0008e4e4", 0x08E4E4, 2, "trivial_rts.c"),
    ("FUN_0008ea4e", 0x08EA4E, 2, "trivial_rts.c"),
    ("FUN_0008eaec", 0x08EAEC, 2, "trivial_rts.c"),
    ("FUN_0008eb54", 0x08EB54, 2, "trivial_rts.c"),
    ("FUN_0008f06e", 0x08F06E, 2, "trivial_rts.c"),
    ("FUN_00099ba4", 0x099BA4, 2, "trivial_rts.c"),
    ("FUN_00099de2", 0x099DE2, 2, "trivial_rts.c"),

    # === 4 funciones "return constant" (moveq/clr + rts, 4 bytes) ===
    ("FUN_000437d6", 0x0437D6, 4, "trivial_returns.c"),
    ("FUN_00077144", 0x077144, 4, "trivial_returns.c"),
    ("FUN_0008f826", 0x08F826, 4, "trivial_returns.c"),
    ("FUN_00043e88", 0x043E88, 4, "trivial_returns.c"),

    # === Sistema de tareas (task_system.c) — analizado semánticamente ===
    ("TaskInsertAfter",        0x0004AE, 88,  "task_system.c"),
    ("EntityClearSpriteSlots", 0x05DC1C, 24,  "task_system.c"),
    ("EntityClearMiscState",   0x05DC34, 112, "task_system.c"),

    # === Entity/Task field setters (entity_setters.c) ===
    ("EntitySetSpriteMap",        0x028CD4, 28, "entity_setters.c"),
    ("EntitySetField38AndUpdate", 0x028134, 8,  "entity_setters.c"),
    ("ClampD0ToRange",            0x00219C, 10, "entity_setters.c"),
    ("InputGuardCall219c",        0x002352, 28, "entity_setters.c"),
]

# Cargar registry auto-generado de trampolines (64 funciones jmp abs.l, 6B)
try:
    from _trampolines_registry import TRAMPOLINES
    FUNCTIONS.extend(TRAMPOLINES)
except ImportError:
    print("WARN: _trampolines_registry.py no encontrado, ejecuta generate_trampolines.py")

# Cargar registry de bra.w y jsr+rts thunks
try:
    from _thunks_registry import BRA_THUNKS, JSR_THUNKS
    FUNCTIONS.extend(BRA_THUNKS)
    FUNCTIONS.extend(JSR_THUNKS)
except ImportError:
    print("WARN: _thunks_registry.py no encontrado, ejecuta generate_more_thunks.py")

# Cargar registry de CCR helpers (342 mini-funciones de 6B)
try:
    from _ccr_registry import CCR_HELPERS
    FUNCTIONS.extend(CCR_HELPERS)
except ImportError:
    print("WARN: _ccr_registry.py no encontrado, ejecuta generate_ccr_helpers.py")

# Cargar registry de Wave 3 (EntityCompareOwnerDepth, StateDispatchStub, ClearNZ)
try:
    from _wave3_registry import WAVE3
    FUNCTIONS.extend(WAVE3)
except ImportError:
    print("WARN: _wave3_registry.py no encontrado, ejecuta generate_wave3.py")

# Cargar registry de Wave 4 (JsrJmp518, EntityCallSetSpriteMap, LeaPcRelStore, CallCheckAndSetC)
try:
    from _wave4_registry import WAVE4
    FUNCTIONS.extend(WAVE4)
except ImportError:
    print("WARN: _wave4_registry.py no encontrado, ejecuta generate_wave4.py")

# Cargar registry de Wave 5 (GetStateData: data-in-code pattern)
try:
    from _wave5_registry import WAVE5
    FUNCTIONS.extend(WAVE5)
except ImportError:
    print("WARN: _wave5_registry.py no encontrado, ejecuta generate_wave5.py")

# Cargar registry de Wave 6 (cobertura total del ROM — chunks literales)
try:
    from _wave6_registry import WAVE6
    FUNCTIONS.extend(WAVE6)
except ImportError:
    print("WARN: _wave6_registry.py no encontrado, ejecuta generate_wave6.py")


def get_original_bytes(rom_dir, addr, size):
    loader = RomLoader(rom_dir)
    loader.load_all()
    prom = loader.get_p_rom_data()
    if addr < 0x100000:
        return prom[addr:addr + size]
    else:
        off = (addr - 0x200000) + 0x100000
        return prom[off:off + size]


def compile_c(c_path, obj_path):
    cmd = ["m68k-linux-gnu-gcc"] + CFLAGS + ["-c", c_path, "-o", obj_path]
    r = subprocess.run(cmd, capture_output=True, text=True)
    return r.returncode == 0, r.stderr


def match_one(name, addr, size, obj_path):
    """Linkea 'name' de obj_path en la direccion 'addr' y compara con la ROM."""
    section = f".text.{name}"
    ld = f"""OUTPUT_FORMAT(elf32-m68k)
OUTPUT_ARCH(m68k)
MEMORY {{ ROM (rx) : ORIGIN = 0x000000, LENGTH = 0x1000000 }}
SECTIONS {{
    . = 0x{addr:06X};
    .text_here 0x{addr:06X} : {{ *({section}) }} > ROM
    /DISCARD/ : {{
        *(.text) *(.text.*)
        *(.data) *(.data.*)
        *(.bss)  *(.bss.*)
        *(.rodata) *(.rodata.*)
        *(.comment) *(.note.*) *(.eh_frame) *(COMMON)
    }}
}}
"""
    ld_path = os.path.join(BUILD_DIR, f"{name}.ld")
    with open(ld_path, "w") as f:
        f.write(ld)

    elf_path = os.path.join(BUILD_DIR, f"{name}.elf")
    r = subprocess.run(
        ["m68k-linux-gnu-ld", "-T", ld_path, "-o", elf_path, obj_path],
        capture_output=True, text=True)
    if r.returncode != 0:
        return False, f"link error: {r.stderr.strip()}"

    bin_path = elf_path + ".bin"
    r = subprocess.run(
        ["m68k-linux-gnu-objcopy", "-O", "binary",
         "--only-section=.text_here", elf_path, bin_path],
        capture_output=True, text=True)
    if r.returncode != 0:
        return False, f"objcopy error: {r.stderr.strip()}"

    with open(bin_path, "rb") as f:
        compiled = f.read()[:size]

    original = get_original_bytes(os.path.join(PROJECT_ROOT, "rom"), addr, size)
    if compiled == original:
        return True, f"{compiled.hex()}"
    return False, f"orig={original.hex()} comp={compiled.hex()}"


def match_one_with_deps(c_file, name, addr, size):
    """Para funciones grandes que llaman a otras (usa match_function.py logica)."""
    # Reutilizamos el script existente
    script = os.path.join(DECOMP_ROOT, "tools", "match_function.py")
    r = subprocess.run(
        ["python3", script,
         os.path.join(SRC_DIR, c_file), name, f"0x{addr:X}", str(size)],
        capture_output=True, text=True)
    matched = "[MATCHED]" in r.stdout
    return matched, r.stdout.splitlines()[-1] if r.stdout else "no output"


def _match_worker(task):
    """Worker de matching (nivel modulo para poder picklearlo con multiprocessing)."""
    name, addr, size, src, obj_path = task
    if not obj_path:
        return (name, addr, size, src, False, "obj no disponible")
    ok, info = match_one(name, addr, size, obj_path)
    return (name, addr, size, src, ok, info)


def main():
    os.makedirs(BUILD_DIR, exist_ok=True)

    # Agrupar por archivo fuente
    by_file = {}
    for name, addr, size, src in FUNCTIONS:
        by_file.setdefault(src, []).append((name, addr, size))

    # Compilar cada archivo una vez
    obj_paths = {}
    print("=== Compilando fuentes ===")
    for src in by_file:
        c_path = os.path.join(SRC_DIR, src)
        obj_path = os.path.join(BUILD_DIR, src.replace(".c", ".o"))
        ok, err = compile_c(c_path, obj_path)
        if not ok:
            print(f"  [FAIL] {src}: {err}")
            continue
        obj_paths[src] = obj_path
        print(f"  [OK]   {src} -> {os.path.basename(obj_path)}")

    # Matchear cada funcion (paralelo con multiprocessing)
    print(f"\n=== Matching {len(FUNCTIONS)} funciones (paralelo) ===")
    matched_total = 0
    matched_bytes = 0
    total_bytes = 0
    non_matched = []
    matched_list = []

    # Separar funciones con deps externas de las normales
    deps_funcs = []
    normal_funcs = []
    for name, addr, size, src in FUNCTIONS:
        total_bytes += size
        if src in ("game_frame.c", "reset_irq_callback.c"):
            deps_funcs.append((name, addr, size, src))
        else:
            normal_funcs.append((name, addr, size, src))

    # Funciones con dependencias: secuencial
    for name, addr, size, src in deps_funcs:
        ok, info = match_one_with_deps(src, name, addr, size)
        if ok:
            matched_total += 1
            matched_bytes += size
            matched_list.append((name, addr, size, src))
            print(f"  [OK] {name} @ ${addr:06X} ({size}B, {src})")
        else:
            non_matched.append((name, addr, size, info))
            print(f"  [!!] {name} @ ${addr:06X}: {info}")

    # Funciones normales: paralelo con multiprocessing.Pool
    from multiprocessing import Pool

    # Preparar tareas
    tasks = [(name, addr, size, src, obj_paths.get(src))
             for name, addr, size, src in normal_funcs]

    # Pool de workers (8 procesos - el bottleneck es lanzar ld/objcopy)
    with Pool(processes=8) as pool:
        results = pool.map(_match_worker, tasks, chunksize=16)

    # Procesar resultados
    show_all = os.environ.get("MATCH_VERBOSE", "0") == "1"
    for name, addr, size, src, ok, info in results:
        if ok:
            matched_total += 1
            matched_bytes += size
            matched_list.append((name, addr, size, src))
            if show_all:
                print(f"  [OK] {name} @ ${addr:06X} ({size}B, {src})")
        else:
            non_matched.append((name, addr, size, info))
            print(f"  [!!] {name} @ ${addr:06X}: {info}")

    print(f"\n===============================================")
    print(f" RESUMEN: {matched_total}/{len(FUNCTIONS)} funciones matched")
    print(f"          {matched_bytes}/{total_bytes} bytes matched")
    print(f"          {matched_bytes/2097152*100:.4f}% del ROM total (2 MB)")
    print(f"===============================================")

    # Guardar reporte
    report = {
        "matched_count": matched_total,
        "total_count": len(FUNCTIONS),
        "matched_bytes": matched_bytes,
        "total_bytes": total_bytes,
        "rom_percentage": matched_bytes / 2097152 * 100,
        "matched": [{"name": n, "addr": f"0x{a:06X}", "size": s, "src": f}
                    for n, a, s, f in matched_list],
        "non_matched": [{"name": n, "addr": f"0x{a:06X}", "size": s, "info": i}
                        for n, a, s, i in non_matched],
    }
    with open(os.path.join(BUILD_DIR, "match_report.json"), "w") as f:
        json.dump(report, f, indent=2)
    print(f"\nReporte: {os.path.join(BUILD_DIR, 'match_report.json')}")

    sys.exit(0 if matched_total == len(FUNCTIONS) else 1)


if __name__ == "__main__":
    main()
