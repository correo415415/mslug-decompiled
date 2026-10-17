#!/usr/bin/env bash
# Parche binario para experimentos (no toca fuentes): copia la P-ROM CPU,
# aplica "ADDR=HEXBYTES" y empaqueta build/repack/mslug.zip.
#   bash tools/dev/bytepatch_zip.sh 000F46=41FAFF26 ...
set -euo pipefail
cd "$(dirname "$0")/../.."
cp build/mslug_prom.bin build/mslug_prom_rebuilt.bin
python3 - "$@" <<'PY'
import sys
p='build/mslug_prom_rebuilt.bin'; d=bytearray(open(p,'rb').read())
for a in sys.argv[1:]:
    ad,hx=a.split('='); ad=int(ad,16); b=bytes.fromhex(hx)
    print('[bytepatch] $%06X: %s -> %s'%(ad,d[ad:ad+len(b)].hex(),b.hex())); d[ad:ad+len(b)]=b
open(p,'wb').write(d)
PY
bash scripts/pack_prom.sh build/mslug_prom_rebuilt.bin >/dev/null
cp internal/mslug.zip build/repack/mslug.zip && (cd build/repack && zip -q -u mslug.zip 201-p1.bin)
echo "[bytepatch] build/repack/mslug.zip"
