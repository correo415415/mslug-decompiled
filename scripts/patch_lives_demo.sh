#!/usr/bin/env bash
# patch_lives_demo.sh — Demo de "editar un valor y reconstruir":
#   cambia las vidas iniciales (HUD_Task_Init_025706 / PlayerSlotDesc tail)
#   de "leer soft-DIP $10FD88" a un inmediato, reconstruye la P-ROM y deja
#   build/repack/mslug.zip listo para el emulador.
#
# Uso:  bash scripts/patch_lives_demo.sh <vidas 1..99> [/ruta/mslug.zip]
#       bash scripts/patch_lives_demo.sh --undo
#
# El HUD muestra vidas-1 (reservas): 9 vidas → "1UP=8".
set -euo pipefail
cd "$(dirname "$0")/.."
F=asm/entity/hud_inputrec_entity_move_024e1c.s
if [ "${1:-}" = "--undo" ]; then git checkout -- "$F"; echo "[demo] parche deshecho"; exit 0; fi
N=${1:?vidas}; ZIP=${2:-}
HEX=$(printf '0x%x' "$N")
git checkout -- "$F"
# Original: "move.b  0x4(a3),d1" (4 B) → "move.b  #imm,d1" (4 B): mismo tamaño.
sed -i "s/move\.b  0x4(a3),d1 /move.b  #${HEX},d1    /" "$F"
C=$(grep -c "move.b  #${HEX},d1" "$F" || true)
if [ "$C" != "2" ]; then echo "[demo] ERROR: el patrón no se aplicó ($C/2 sitios)"; exit 1; fi
echo "[demo] parche aplicado en 2 sitios (HUD_Task_Init_025706 y su variante P2)"
python3 tools/check_section_sizes.py | tail -1
bash scripts/rebuild_rom.sh --mod ${ZIP:+"$ZIP"}
cmp -l build/mslug_prom.bin build/mslug_prom_rebuilt.bin | while read off a b; do printf "  byte \$%06X (CPU): %02X -> %02X\n" $((off-1)) "0$a" "0$b"; done
echo "[demo] carga build/repack/mslug.zip (o 201-p1.bin) en el emulador: debe mostrar 1UP=$((N-1))"
echo "[demo] para deshacer: bash scripts/patch_lives_demo.sh --undo"
