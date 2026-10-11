#!/usr/bin/env bash
# Uso: scripts/data_wave.sh START END FILE.s "Wave X" "descripcion" [extra gen args...]
# Genera, verifica byte a byte y registra una ola de DATOS.
set -e
S=$1; E=$2; F=$3; W=$4; D=$5; shift 5
cd "$(dirname "$0")/.."
python3 tools/gen_data_region.py "$S" "$E" -o "asm/$F" --src "$F" --registry --wave "$W" "$@" > "/tmp/${F%.s}.txt" 2>&1 || { tail -5 "/tmp/${F%.s}.txt"; exit 1; }
grep -E "^\[verify\]" "/tmp/${F%.s}.txt"
python3 tools/wave_apply.py "/tmp/${F%.s}.txt" "$F" "$W" "$D"
python3 tools/registry_lint.py | tail -1
