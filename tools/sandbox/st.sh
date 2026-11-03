#!/bin/bash
# Resumen: commit, binario verificado y última línea de cada resultado.
cd /home/user/webapp/.port; git log --oneline -1; cat /tmp/v_bin.txt 2>/dev/null; tail -1 /tmp/b2.log 2>/dev/null
for f in /tmp/v/* /tmp/v3/*; do [ -f $f ] && echo "$f: $(tail -1 $f)"; done
