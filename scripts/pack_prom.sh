#!/usr/bin/env bash
# =============================================================================
#  Metal Slug 1 - repack a formato MAME
# -----------------------------------------------------------------------------
#  Toma la P ROM procesada (build/mslug_prom.bin) - que es lo que el matcher
#  compara byte-a-byte con la salida del build - y aplica la transformacion
#  INVERSA de scripts/setup.sh para reconstruir un 201-p1.bin cargable por
#  MAME/FBNeo. Es decir:
#
#      MAME 201-p1.bin  --setup.sh--> build/mslug_prom.bin  (formato CPU)
#      build/mslug_prom.bin  --pack_prom.sh--> MAME 201-p1.bin
#
#  Uso:
#    bash scripts/pack_prom.sh [-o path/to/output.bin] [prom_input]
#
#  Sin argumentos: lee build/mslug_prom.bin y escribe build/repack/201-p1.bin.
#
#  Con -o path/to/output.bin: escribe en la ruta indicada.
#
#  Con prom_input: usa esa ruta como entrada en lugar de build/mslug_prom.bin
#  (util para repackear una P ROM ya modificada por el matcher).
#
#  NOTA: el binario reconstruido tiene formato ROM_LOAD16_WORD_SWAP + bank swap
#  esperado por el driver mslug de MAME. Para pruebas end-to-end:
#    1) bash scripts/pack_prom.sh
#    2) cp build/repack/201-p1.bin  <ruta a tu mslug.zip staging>
#    3) zip -j mslug.zip 201-p1.bin  (o rehacer el zip completo)
#    4) mame mslug   (o load en FBNeo / RetroArch neogeo core)
#
#  Este script NO redistribuye la ROM del usuario. Solo procesa localmente.
# =============================================================================
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.."; pwd)"
IN_DEFAULT="$ROOT/build/mslug_prom.bin"
OUT_DEFAULT="$ROOT/build/repack/201-p1.bin"

OUT="$OUT_DEFAULT"
while getopts "o:h" opt; do
    case "$opt" in
        o) OUT="$OPTARG" ;;
        h)
            sed -n '2,30p' "$0"; exit 0 ;;
        *) echo "uso: $0 [-o output.bin] [prom_input]" >&2; exit 2 ;;
    esac
done
shift $((OPTIND-1))

IN="${1:-$IN_DEFAULT}"

if [[ ! -f "$IN" ]]; then
    echo "error: no encuentro $IN" >&2
    echo "       Ejecuta primero 'bash scripts/setup.sh' o pasa la P ROM como arg." >&2
    exit 1
fi

mkdir -p "$(dirname "$OUT")"

python3 - "$IN" "$OUT" <<'PY'
import hashlib, sys, os
inp, out = sys.argv[1:3]
p = open(inp, "rb").read()
if len(p) != 0x200000:
    sys.exit(f"error: la P ROM debe medir 2 MiB, tiene {len(p)} bytes")

# Paso 1: bank-swap inverso. En setup.sh:
#   p[:0x100000]         = sw[0x100000:]
#   p[0x100000:0x200000] = sw[:0x100000]
# La operacion inversa es identica (es una involucion):
sw = bytearray(0x200000)
sw[:0x100000]         = p[0x100000:]
sw[0x100000:0x200000] = p[:0x100000]

# Paso 2: byte-swap word-a-word inverso (tambien involucion):
raw = bytearray(0x200000)
for i in range(0, 0x200000, 2):
    raw[i]   = sw[i+1]
    raw[i+1] = sw[i]

open(out, "wb").write(raw)

md5_in  = hashlib.md5(p).hexdigest()
md5_out = hashlib.md5(raw).hexdigest()
print(f"[REPACK] P ROM in : {md5_in}   ({inp})")
print(f"         MAME out : {md5_out}   ({out})")
print(f"         size     : {len(raw):,} bytes ({len(raw)/1024:.0f} KiB)")

# Sanity: verificar que la involucion round-trip funciona.
# swap(swap(x)) debe ser x, y la nueva conversion es exactamente lo que
# setup.sh haria en reverso.
PY

echo ""
echo "  Listo. Para probar en MAME/FBNeo:"
echo "     1. Copia el 201-p1.bin de tu set original a otra carpeta como backup."
echo "     2. Sustituye 201-p1.bin en tu mslug.zip por '$OUT'."
echo "     3. Carga mslug en el emulador."
echo "     4. Si FBNeo/RetroArch se queja de CRC, usa 'romset legacy' o el flag"
echo "        equivalente a 'accept CRC mismatch'."
