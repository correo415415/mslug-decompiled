#!/usr/bin/env bash
# =============================================================================
#  Metal Slug 1 - bootstrap de un sandbox limpio
# -----------------------------------------------------------------------------
#  Instala el toolchain m68k + dependencias Python y procesa la P ROM desde
#  un zip de MAME (que el usuario aporta) en build/mslug_prom.bin.
#
#  Uso:  ./scripts/bootstrap_sandbox.sh [ruta/al/mslug.zip]
#        (si no se da zip, se espera rom/201-p1.bin ya presente)
# =============================================================================
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.."; pwd)"
ZIP="${1:-}"

if ! command -v m68k-linux-gnu-gcc >/dev/null 2>&1; then
    echo "[bootstrap] instalando gcc-m68k-linux-gnu / binutils..."
    (sudo apt-get install -y -qq gcc-m68k-linux-gnu binutils-m68k-linux-gnu >/dev/null 2>&1) \
      || (sudo apt-get update -qq >/dev/null 2>&1 && \
          sudo apt-get install -y -qq gcc-m68k-linux-gnu binutils-m68k-linux-gnu >/dev/null 2>&1)
fi
python3 -c "import capstone" 2>/dev/null || python3 -m pip install -q -r "$ROOT/requirements.txt"

mkdir -p "$ROOT/rom"
if [[ -n "$ZIP" && -f "$ZIP" ]]; then
    echo "[bootstrap] extrayendo 201-p1.bin de $ZIP"
    unzip -o -q "$ZIP" 201-p1.bin -d "$ROOT/rom"
fi
"$ROOT/scripts/setup.sh"
python3 "$ROOT/tools/registry_lint.py" | tail -1
echo "[bootstrap] listo: m68k-linux-gnu-gcc $(m68k-linux-gnu-gcc -dumpversion), build/mslug_prom.bin OK"
