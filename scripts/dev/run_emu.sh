#!/usr/bin/env bash
# run_emu.sh — Lanza la ROM (original o reconstruida) en el emulador de internal/
# en modo headless, inserta moneda + START, llega a la partida y vuelca PNGs.
# Uso: bash scripts/dev/run_emu.sh [zip] [frames] [outdir] [extra args del emulador]
set -euo pipefail
cd "$(dirname "$0")/../.."
ZIP=${1:-build/repack/mslug.zip}; FR=${2:-4800}; OUT=${3:-/tmp/emu_frames}; shift $(( $# < 3 ? $# : 3 ))
rm -rf "$OUT"; mkdir -p "$OUT"
internal/pydmg/target/release/pydmg-neogeo --headless --cart "$ZIP" --max-frames "$FR" \
  --auto-coin-frame 400 --auto-press-start-frame 500 --press-period-frames 300 \
  --dump-frames-dir "$OUT" --dump-every-frames 200 "$@" 2>/dev/null | grep -E "WATCH|POKE" || true
echo "[emu] frames en $OUT"
