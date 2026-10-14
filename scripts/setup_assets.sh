#!/usr/bin/env bash
# Extract the graphics / sample ROMs (201-s1, 201-c1..c4, 201-v1..v2) from the
# MAME romset into rom/ for assets/tools/{gfx,audio}.py.  Never committed.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.."; pwd)"
ZIP="${1:-$ROOT/internal/mslug.zip}"
mkdir -p "$ROOT/rom"
unzip -o -q "$ZIP" 201-s1.bin 201-c1.bin 201-c2.bin 201-c3.bin 201-c4.bin 201-v1.bin 201-v2.bin -d "$ROOT/rom"
python3 -c "import numpy, PIL" 2>/dev/null || python3 -m pip install -q --user numpy Pillow
echo "[setup_assets] rom/201-{s1,c1..c4,v1,v2}.bin listos"
