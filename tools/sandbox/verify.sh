#!/bin/bash
# Verificación corta (6000 frames) contra el original; resultados en /tmp/v.
cd /home/user/webapp/.port
B=${B:-./target/release/mslug}
mkdir -p /tmp/v
timeout 3000 $B verify-system --frames 6000 > /tmp/v/c.txt 2>&1
timeout 3000 $B verify-system --coin 0 --frames 6000 > /tmp/v/c0.txt 2>&1
timeout 3000 $B verify-system --hands-off --frames 4000 > /tmp/v/h.txt 2>&1
for n in 1 2 3 5; do timeout 3000 $B verify-system --fuzz $n --frames 6000 > /tmp/v/f$n.txt 2>&1; done
