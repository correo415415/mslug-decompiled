#!/usr/bin/env bash
# =============================================================================
#  Extrae todos los assets del romset a build/assets/ (nunca entran en git).
#
#  Uso:  scripts/extract_assets.sh [ruta/mslug.zip] [gfx|sprites|levels|audio ...]
#        (por defecto internal/mslug.zip y todo)
#
#  Salida:
#    build/assets/fix/                tiles 8x8 de la capa fija (PNG indexado)
#    build/assets/sprites/bank{0,1}/  tiles 16x16 de los C-ROM (PNG indexado)
#    build/assets/sprites/palettes/   1 536 paletas
#    build/assets/sprites/frames/     frames montados como los dibuja el juego
#    build/assets/sprites/anims/      ~980 animaciones GIF + index.html
#    build/assets/levels/             fondos de las misiones + index.html
#    build/assets/audio/              muestras ADPCM-A/B con nombre (WAV) + index.csv
#
#  Las herramientas (assets/tools/) vienen de la rama de la decompilación
#  (assets/tools, tools/scene_script_dump.py) y leen el zip directamente.
# =============================================================================
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.."; pwd)"
cd "$ROOT"
if [[ $# -gt 0 && "$1" == *.zip ]]; then export MSLUG_ROM="$(realpath "$1")"; shift; fi
python3 -c "import numpy, PIL, capstone" 2>/dev/null || python3 -m pip install -q numpy Pillow capstone
STEPS=("$@"); [[ ${#STEPS[@]} -eq 0 ]] && STEPS=(gfx sprites levels audio)
for s in "${STEPS[@]}"; do
    t0=$(date +%s)
    case "$s" in
        gfx)     python3 assets/tools/gfx.py extract ;;
        sprites) python3 assets/tools/sprites.py extract ;;
        levels)  python3 assets/tools/levels.py ;;
        audio)   python3 assets/tools/audio.py extract ;;
        *) echo "paso desconocido: $s" >&2; exit 2 ;;
    esac
    echo "[extract_assets] $s: $(( $(date +%s) - t0 )) s"
done
echo "[extract_assets] listo -> build/assets/ (abre build/assets/sprites/anims/index.html y build/assets/levels/index.html)"
