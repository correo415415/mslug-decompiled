#!/usr/bin/env bash
# =============================================================================
#  Metal Slug 1 - baserom setup
# -----------------------------------------------------------------------------
#  Reads rom/201-p1.bin (which the user must supply themselves) and produces
#  build/mslug_prom.bin, the byte-swapped and bank-swapped P ROM against which
#  the matcher compares every built function.
#
#  MD5 hashes (both son P-ROM validos, ninguno del hex de rom/201-p1.bin de
#  MAME que trae cada usuario, que puede variar segun set/version):
#    Input   201-p1.bin (formato MAME, cartucho):  b6804bc6be580c80d43d187f6f9d2e7c
#    Output  build/mslug_prom.bin (formato CPU):   816b3f74c76b3373993407615f1850fe
#
#  El matcher (tools/match_batch.py) compara byte-a-byte contra la salida.
#  Si tu 201-p1.bin ya tiene MD5 = 816b3f74... (formato CPU ya procesado),
#  simplemente copialo a build/mslug_prom.bin sin ejecutar este script.
# =============================================================================
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.."; pwd)"
IN="$ROOT/rom/201-p1.bin"
OUT="$ROOT/build/mslug_prom.bin"
EXPECT_IN="b6804bc6be580c80d43d187f6f9d2e7c"    # MAME 201-p1.bin (formato cartucho)
EXPECT_OUT="816b3f74c76b3373993407615f1850fe"   # build/mslug_prom.bin (formato CPU)

if [[ ! -f "$IN" ]]; then
    echo "error: $IN not found." >&2
    echo "       Place your original Metal Slug 1 program ROM at that path." >&2
    exit 1
fi

mkdir -p "$ROOT/build"

python3 - "$IN" "$OUT" "$EXPECT_IN" "$EXPECT_OUT" <<'PY'
import hashlib, sys
inp, out, expect_in, expect_out = sys.argv[1:5]
raw = open(inp, "rb").read()
if len(raw) != 0x200000:
    sys.exit(f"error: expected 2 MiB ROM, got {len(raw)} bytes")

md5_in = hashlib.md5(raw).hexdigest()

# Ruta rapida: si el usuario ya tiene una P-ROM ya procesada (MD5 esperado
# de salida), no la volvemos a transformar - saldria distinta.
if md5_in == expect_out:
    print(f"[NOTE] input ya es formato CPU (MD5={md5_in})")
    print(f"       copiando directamente sin re-swap")
    open(out, "wb").write(raw)
    sys.exit(0)

# Byte-swap every 16-bit word (ROM_LOAD16_WORD_SWAP in MAME)
sw = bytearray(len(raw))
for i in range(0, len(raw), 2):
    sw[i]     = raw[i+1]
    sw[i+1]   = raw[i]
# Swap the two 1 MiB banks: bank 1 lives at file offset 0x100000
p = bytearray(0x200000)
p[:0x100000]         = sw[0x100000:]
p[0x100000:0x200000] = sw[:0x100000]
open(out, "wb").write(p)
md5_out = hashlib.md5(p).hexdigest()

status = "OK" if md5_out == expect_out else "MISMATCH"
print(f"[{status}] input  MD5 = {md5_in}")
if md5_in != expect_in:
    print(f"          (esperaba {expect_in} para el formato MAME)")
print(f"       output MD5 = {md5_out}")
print(f"          (esperaba {expect_out} para el formato CPU)")
if md5_out != expect_out:
    sys.exit(1)
PY
