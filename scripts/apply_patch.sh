#!/usr/bin/env bash
# apply_patch.sh — Aplica parches de ejemplo sobre las fuentes, reconstruye la
# P-ROM y deja build/repack/201-p1.bin (+ mslug.zip si se indica un zip base).
# Demuestra que el árbol de fuentes ES la ROM: editar → rebuild → jugar.
# No usa git: guarda copias .orig de los ficheros tocados en build/patch_backup/.
#
# Uso:
#   bash scripts/apply_patch.sh <parche> [<parche>...] [--zip /ruta/mslug.zip]
#   bash scripts/apply_patch.sh --list
#   bash scripts/apply_patch.sh --undo
#
# Parches disponibles (todos conservan el tamaño de cada función, así el
# resto de la ROM queda byte-idéntico):
#   lives=N        vidas iniciales fijas (1..99). HUD muestra N-1 ("1UP=N-1").
#   hmg-infinite   Heavy Machine Gun con munición infinita (no decrementa $82(a2)).
#   all-infinite   todas las armas especiales (HMG, Shotgun, Rocket, Flame) infinitas.
#   bombs=N        bombas al reaparecer / al inicio (defecto del juego: 10).
set -euo pipefail
cd "$(dirname "$0")/.."

F_HUD=asm/entity/hud_inputrec_entity_move_024e1c.s
F_FIRE=asm/player/fire_shells_03c62a.s
F_BOMB_A=asm/player/air_death_crouch_036632.s
F_BOMB_B=asm/player/core_032aa8.s
PATCHABLE=("$F_HUD" "$F_FIRE" "$F_BOMB_A" "$F_BOMB_B")
BK=build/patch_backup

list() { sed -n '/^# Parches disponibles/,/^set -euo/p' "$0" | grep -E '^#   ' ; }

# Copia de seguridad (solo la primera vez: el .orig es siempre la fuente limpia)
backup() { mkdir -p "$BK"; for f in "${PATCHABLE[@]}"; do [ -f "$BK/$(basename "$f").orig" ] || cp "$f" "$BK/$(basename "$f").orig"; done; }
# Restaura las fuentes limpias desde los .orig
undo() {
  local n=0
  for f in "${PATCHABLE[@]}"; do
    if [ -f "$BK/$(basename "$f").orig" ]; then cp "$BK/$(basename "$f").orig" "$f"; n=$((n+1)); fi
  done
  echo "[patch] fuentes restauradas ($n ficheros desde $BK/)"
}

# Sustituye el 'subi.w #1,0x82(a2)' (6 B) de UNA función por 'ori.w #0,0x82(a2)' (6 B, no-op).
no_ammo_dec() {   # $1 = nombre de la función (label "Nombre:")
  python3 - "$F_FIRE" "$1" <<'PY'
import sys
p,fn=sys.argv[1:3]; s=open(p).read()
i=s.index(f"\n{fn}:\n")
j=s.find("\n| ------", i+10); j=len(s) if j<0 else j
blk=s[i:j]; old="subi.w  #0x1,0x82(a2)"; new="ori.w   #0x0,0x82(a2)"
assert blk.count(old)==1, (fn, blk.count(old))
s=s[:i]+blk.replace(old,new)+s[j:]; open(p,"w").write(s); print(f"[patch] {fn}: munición infinita")
PY
}

[ $# -eq 0 ] && { list; exit 0; }
ZIP=""; PATCHES=()
while [ $# -gt 0 ]; do
  case "$1" in
    --list) list; exit 0 ;;
    --undo) undo; exit 0 ;;
    --zip) ZIP=$2; shift ;;
    *) PATCHES+=("$1") ;;
  esac; shift
done

backup
undo >/dev/null
for p in "${PATCHES[@]}"; do
  case "$p" in
    lives=*)
      N=${p#lives=}; HEX=$(printf '0x%x' "$N")
      sed -i "s/move\.b  0x4(a3),d1 /move.b  #${HEX},d1    /" "$F_HUD"
      [ "$(grep -c "move.b  #${HEX},d1" "$F_HUD")" = 2 ] || { echo "[patch] lives: patrón no aplicado"; exit 1; }
      echo "[patch] vidas iniciales = $N (HUD 1UP=$((N-1)))" ;;
    hmg-infinite)
      no_ammo_dec PlayerFire_HMG_Spawn_03c872 ;;
    all-infinite)
      for fn in PlayerFire_HMG_Spawn_03c872 PlayerFire_Shotgun_Spawn_03c9c4 \
                PlayerFire_Rocket_Spawn_03cb16 PlayerFire_Flame_Spawn_03ccd2; do no_ammo_dec "$fn"; done ;;
    bombs=*)
      N=${p#bombs=}; HEX=$(printf '0x%x' "$N")
      sed -i "s/move\.w  #0xa,0x82(a6) /move.w  #${HEX},0x82(a6)/" "$F_BOMB_A" "$F_BOMB_B"
      C=$(cat "$F_BOMB_A" "$F_BOMB_B" | grep -c "move.w  #${HEX},0x82(a6)")
      [ "$C" = 3 ] || { echo "[patch] bombs: $C/3 sitios"; exit 1; }
      echo "[patch] bombas = $N" ;;
    *) echo "[patch] parche desconocido: $p"; list; exit 1 ;;
  esac
done

python3 tools/check_section_sizes.py | tail -1
bash scripts/rebuild_rom.sh --mod ${ZIP:+"$ZIP"} | grep -E "MD5|listo"

# Lista de bytes modificados: necesita la ROM original en formato CPU
# (build/mslug_prom.bin). Si no existe, se intenta generar desde el zip dado.
if [ ! -f build/mslug_prom.bin ] && [ -n "$ZIP" ] && [ -f "$ZIP" ]; then
  mkdir -p rom && unzip -o -q "$ZIP" 201-p1.bin -d rom && ./scripts/setup.sh >/dev/null 2>&1 || true
fi
if [ -f build/mslug_prom.bin ]; then
  echo "[patch] bytes modificados (direcciones CPU):"
  (cmp -l build/mslug_prom.bin build/mslug_prom_rebuilt.bin || true) | while read off a b; do printf "  \$%06X: %02X -> %02X\n" $((off-1)) "0$a" "0$b"; done
else
  echo "[patch] (sin build/mslug_prom.bin no se puede listar el diff; ejecuta ./scripts/setup.sh con rom/201-p1.bin)"
fi
echo "[patch] ROM parcheada: build/repack/201-p1.bin${ZIP:+  y build/repack/mslug.zip}"
echo "[patch] para deshacer: bash scripts/apply_patch.sh --undo"
