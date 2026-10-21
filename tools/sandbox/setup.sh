#!/bin/bash
# Recuperación del entorno tras un reset del sandbox (sin secretos: el
# token va en ~/.git-credentials, que se escribe a mano).
D=$(cd "$(dirname "$0")" && pwd)
mkdir -p /tmp/ghm; cp "$D/gh.py" /tmp/ghm/; cp "$D/m68dis.py" /tmp/
if ! grep -q swapfile2 /proc/swaps 2>/dev/null; then
  sudo fallocate -l 3G /swapfile2 2>/dev/null; sudo chmod 600 /swapfile2; sudo mkswap /swapfile2 >/dev/null; sudo /sbin/swapon /swapfile2 2>/dev/null || sudo swapon /swapfile2
fi
pip install -q capstone >/dev/null 2>&1 &
[ -x $HOME/.cargo/bin/cargo ] || (curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal >/dev/null 2>&1)
export PATH=$HOME/.cargo/bin:$PATH
cd "$D/../.." && cargo build -j2 --release -p mslug > /tmp/b.log 2>&1; echo "RC=$?" >> /tmp/b.log
./target/release/mslug dump-roms >/dev/null 2>&1 && cp build/recomp/prom.bin /tmp/prom.bin
wait
echo READY >> /tmp/b.log
