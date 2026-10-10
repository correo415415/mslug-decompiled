#!/usr/bin/env bash
# setup_internal.sh — Recrea internal/ (SOLO desarrollo local; está en .gitignore):
#   internal/mslug.zip   romset original
#   internal/pydmg/      emulador Neo Geo (Rust) + parche de herramientas de dev
#                        (--input-script, --watch-ram, --poke-ram)
# Uso: bash scripts/dev/setup_internal.sh /ruta/mslug.zip /ruta/pydmg.zip
set -euo pipefail
cd "$(dirname "$0")/../.."
ROMZIP=${1:-/home/user/uploaded_files/mslug.zip}
EMUZIP=${2:-/home/user/uploaded_files/pydmg.zip}
mkdir -p internal && cp "$ROMZIP" internal/mslug.zip
(cd internal && rm -rf pydmg && unzip -q -o "$EMUZIP")
patch -p1 -d internal/pydmg < scripts/dev/pydmg_devtools.patch >/dev/null
command -v cargo >/dev/null || { curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal >/dev/null 2>&1; }
export PATH=$HOME/.cargo/bin:$PATH
dpkg -s libsdl2-dev >/dev/null 2>&1 || sudo apt-get install -y -qq libsdl2-dev >/dev/null 2>&1
(cd internal/pydmg && cargo build --release 2>&1 | grep -E "^error|Finished")
echo "[internal] listo: internal/pydmg/target/release/pydmg-neogeo"
