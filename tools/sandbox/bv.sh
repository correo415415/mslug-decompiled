#!/bin/bash
# Compila, copia el binario y lanza las dos verificaciones.
export PATH=$HOME/.cargo/bin:$PATH
cd /home/user/webapp/.port
cargo build -j2 --release -p mslug > /tmp/b2.log 2>&1; echo "RC=$?" >> /tmp/b2.log
B=/tmp/mslug_v$(date +%s); cp target/release/mslug $B
echo "$B $(git rev-parse --short HEAD)" > /tmp/v_bin.txt
B=$B /tmp/verify.sh
B=$B /tmp/verify3.sh
