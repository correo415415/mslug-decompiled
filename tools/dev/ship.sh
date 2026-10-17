#!/bin/bash
# uso: ship.sh "mensaje"
cd /home/user/webapp
python3 tools/gen_data_headers.py >/dev/null || exit 1
L=$(python3 tools/registry_lint.py 2>&1 | tail -1); echo "$L"; [[ "$L" == *"OK"* ]] || exit 1
C=$(python3 tools/check_section_sizes.py 2>&1 | tail -1); echo "$C"; [[ "$C" == *" 0 discrepancias"* ]] || exit 1
B=$(python3 tools/build_rom.py 2>&1 | tail -1); echo "$B"; [[ "$B" == *"IDÉNTICA"* ]] || exit 1
M=$(python3 mrom/tools/build_mrom.py 2>&1 | tail -1); echo "$M"; [[ "$M" == *"IDÉNTICA"* ]] || exit 1
git add -A && git commit -qm "$1" && git push -q origin genspark_ai_developer && git log --oneline -1
