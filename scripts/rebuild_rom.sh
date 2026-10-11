#!/usr/bin/env bash
# rebuild_rom.sh — Reconstruye la P-ROM completa desde las fuentes y genera
# el fichero cargable por MAME (201-p1.bin) + un mslug.zip listo para usar.
#
# Uso:
#   bash scripts/rebuild_rom.sh                 # verifica identidad con el original
#   bash scripts/rebuild_rom.sh --mod           # ROM modificada (no comparar)
#   bash scripts/rebuild_rom.sh --mod /ruta/mslug.zip   # también genera build/repack/mslug.zip
#
# Salidas:
#   build/mslug_prom_rebuilt.bin   formato CPU (big-endian, bancos en orden)
#   build/repack/201-p1.bin        formato MAME (word-swap + bank-swap)
#   build/repack/mslug.zip         (si se pasa un zip base) con 201-p1.bin sustituido
set -euo pipefail
cd "$(dirname "$0")/.."
MOD=0; ZIP=""
for a in "$@"; do
  case "$a" in
    --mod) MOD=1 ;;
    *.zip) ZIP="$a" ;;
  esac
done
if [ $MOD -eq 1 ]; then
  python3 tools/build_rom.py --no-compare
else
  python3 tools/build_rom.py
fi
bash scripts/pack_prom.sh build/mslug_prom_rebuilt.bin >/dev/null
echo "[rebuild_rom] build/repack/201-p1.bin  MD5 = $(md5sum build/repack/201-p1.bin | cut -d' ' -f1)"
if [ -n "$ZIP" ]; then
  cp "$ZIP" build/repack/mslug.zip
  (cd build/repack && zip -q -u mslug.zip 201-p1.bin)
  echo "[rebuild_rom] build/repack/mslug.zip listo (201-p1.bin sustituido)"
fi
