#!/usr/bin/env bash
# Construye un zip parcheado con nombre propio (para lanzar varios en paralelo):
#   bash tools/dev/patch_zip.sh NOMBRE parche1 [parche2 ...]  -> build/zips/NOMBRE.zip
set -euo pipefail
cd "$(dirname "$0")/../.."
n=$1; shift; mkdir -p build/zips
bash scripts/apply_patch.sh "$@" --zip internal/mslug.zip >/dev/null 2>&1 || { bash scripts/apply_patch.sh --undo >/dev/null; echo "[patch_zip] $n: FALLO"; exit 1; }
cp build/repack/mslug.zip build/zips/$n.zip
bash scripts/apply_patch.sh --undo >/dev/null
echo "[patch_zip] build/zips/$n.zip"
