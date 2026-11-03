#!/bin/bash
# Verificación larga (16000 frames); resultados en /tmp/v3.
cd /home/user/webapp/.port
B=${B:-./target/release/mslug}
mkdir -p /tmp/v3
timeout 6000 $B verify-system --frames 16000 > /tmp/v3/c16 2>&1
timeout 6000 $B verify-system --coin 200 --frames 16000 > /tmp/v3/c200 2>&1
timeout 6000 $B verify-system --fuzz 21 --frames 16000 > /tmp/v3/f21 2>&1
