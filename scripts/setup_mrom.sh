#!/usr/bin/env bash
# =============================================================================
#  Metal Slug 1 - M-ROM (Z80 sound program) setup
# -----------------------------------------------------------------------------
#  Extracts 201-m1.bin from a MAME romset zip (or copies a loose file) into
#  rom/201-m1.bin and verifies its MD5.  The M-ROM is 128 KiB:
#     0x00000-0x07FFF  fixed region   (Z80 0x0000-0x7FFF)
#     0x08000-0x1FFFF  7 x 16 KiB banks, switched via ports 0x08-0x0B
#  Expected MD5: 6449db0459bcf7584348792009e8f12c
#  Usage: scripts/setup_mrom.sh [mslug.zip | 201-m1.bin]
# =============================================================================
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.."; pwd)"
SRC="${1:-$ROOT/internal/mslug.zip}"
OUT="$ROOT/rom/201-m1.bin"
EXPECT="6449db0459bcf7584348792009e8f12c"
mkdir -p "$ROOT/rom"
if [[ "$SRC" == *.zip ]]; then
    unzip -o -q "$SRC" 201-m1.bin -d "$ROOT/rom"
elif [[ "$SRC" != "$OUT" ]]; then
    cp "$SRC" "$OUT"
fi
got=$(md5sum "$OUT" | cut -d' ' -f1)
echo "[setup_mrom] rom/201-m1.bin MD5 = $got"
[[ "$got" == "$EXPECT" ]] || { echo "[setup_mrom] WARNING: expected $EXPECT" >&2; exit 1; }
