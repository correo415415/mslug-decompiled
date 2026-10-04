#!/usr/bin/env bash
# =============================================================================
#  Metal Slug 1 - bootstrap de un sandbox limpio
# -----------------------------------------------------------------------------
#  Instala el toolchain m68k + dependencias Python y procesa la P ROM desde
#  un zip de MAME (que el usuario aporta) en build/mslug_prom.bin.
#
#  Uso:  ./scripts/bootstrap_sandbox.sh [ruta/al/mslug.zip]
#
#  Si no se pasa zip se busca, en este orden:
#     1. rom/201-p1.bin ya presente
#     2. /home/user/uploaded_files/mslug.zip   (subida del usuario)
#     3. /mnt/aidrive/mslug_rom/mslug.zip      (copia persistente en AI Drive)
#  Cuando el zip procede de una subida y AI Drive esta montado, se guarda una
#  copia en /mnt/aidrive/mslug_rom/ para que sobreviva a futuros resets del
#  sandbox (el ROM nunca entra en git: ver .gitignore).
# =============================================================================
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.."; pwd)"
ZIP="${1:-}"
AIDRIVE_DIR="/mnt/aidrive/mslug_rom"

if ! command -v m68k-linux-gnu-gcc >/dev/null 2>&1; then
    echo "[bootstrap] instalando gcc-m68k-linux-gnu / binutils..."
    (sudo apt-get install -y -qq gcc-m68k-linux-gnu binutils-m68k-linux-gnu >/dev/null 2>&1) \
      || (sudo apt-get update -qq >/dev/null 2>&1 && \
          sudo apt-get install -y -qq gcc-m68k-linux-gnu binutils-m68k-linux-gnu >/dev/null 2>&1)
fi
python3 -c "import capstone" 2>/dev/null || python3 -m pip install -q -r "$ROOT/requirements.txt"

mkdir -p "$ROOT/rom"
if [[ -z "$ZIP" && ! -f "$ROOT/rom/201-p1.bin" ]]; then
    for cand in /home/user/uploaded_files/mslug.zip "$AIDRIVE_DIR/mslug.zip"; do
        if [[ -f "$cand" ]]; then ZIP="$cand"; break; fi
    done
fi
if [[ -n "$ZIP" ]]; then
    [[ -f "$ZIP" ]] || { echo "[bootstrap] ERROR: no existe $ZIP" >&2; exit 1; }
    echo "[bootstrap] extrayendo 201-p1.bin de $ZIP"
    unzip -o -q "$ZIP" 201-p1.bin -d "$ROOT/rom"
    # persistir en AI Drive si procede de otra ruta y el drive esta montado
    if [[ -d /mnt/aidrive && "$ZIP" != "$AIDRIVE_DIR/"* ]]; then
        mkdir -p "$AIDRIVE_DIR" 2>/dev/null && cp -f "$ZIP" "$AIDRIVE_DIR/mslug.zip" 2>/dev/null \
          && echo "[bootstrap] copia persistente guardada en $AIDRIVE_DIR/mslug.zip" || true
    fi
elif [[ ! -f "$ROOT/rom/201-p1.bin" ]]; then
    echo "[bootstrap] ERROR: no se encontro mslug.zip ni rom/201-p1.bin." >&2
    echo "            Sube mslug.zip (o pasa su ruta como argumento)." >&2
    exit 1
fi
"$ROOT/scripts/setup.sh"
python3 "$ROOT/tools/registry_lint.py" | tail -1
echo "[bootstrap] listo: m68k-linux-gnu-gcc $(m68k-linux-gnu-gcc -dumpversion), build/mslug_prom.bin OK"
