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
#   bombs=N        bombas iniciales (1..99; defecto del juego: 10).
#   bombs-infinite granadas infinitas (Player_ThrowGrenade* no decrementa $80).
#   pistol-ammo=N  munición que da la pistola al perder el arma (defecto 10; era el antiguo "bombs=").
#   invincible     invulnerabilidad del jugador y del Slug (el camino del bit 3 de debug $100001).
#   debug=HHHH     flags de depuración: escribe HHHH en $100000.w al arrancar (en vez de 0).
#   mission=N      empezar la partida en la misión N (1..6) en vez de la 1.
set -euo pipefail
cd "$(dirname "$0")/.."

F_HUD=asm/entity/hud_inputrec_entity_move_024e1c.s
F_FIRE=asm/player/fire_shells_03c62a.s
F_CORE=asm/player/core_032aa8.s
F_INIT=asm/player/entity_init_aux_state_032a02.s
F_ROUTE=asm/player/route_publish_027eba.s
F_SLUG=asm/slug/helpers_0295a6.s
F_BIOS=src/bios/bios_user_dispatch.c
F_MODE=asm/boot/dispatcher_mode_table_001922.s
PATCHABLE=("$F_MODE" "$F_HUD" "$F_FIRE" "$F_CORE" "$F_INIT" asm/player/air_death_crouch_036632.s "$F_ROUTE" "$F_SLUG" "$F_BIOS")
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

# Sustituye UNA instrucción dentro de UNA función (bloque "Nombre:" hasta el
# siguiente separador "| ------"). Falla si el patrón no aparece exactamente 1 vez.
#   in_fn FICHERO FUNCION 'texto viejo' 'texto nuevo' 'mensaje'
in_fn() {
  python3 - "$@" <<'PY'
import sys
p,fn,old,new,msg=sys.argv[1:6]; s=open(p).read()
i=s.find(f"\n{fn}:\n")
if i<0: sys.exit(f"[patch] ERROR: no encuentro la función {fn} en {p}")
j=s.find("\n| ------", i+10); j=len(s) if j<0 else j
blk=s[i:j]
if blk.count(old)!=1: sys.exit(f"[patch] ERROR: {fn}: patrón '{old}' aparece {blk.count(old)} veces (esperaba 1)")
s=s[:i]+blk.replace(old,new)+s[j:]; open(p,"w").write(s); print(f"[patch] {fn}: {msg}")
PY
}
# 'subi.w #1,0x82(a2)' (6 B) -> 'ori.w #0,0x82(a2)' (6 B, no-op)
no_ammo_dec() { in_fn "$F_FIRE" "$1" "subi.w  #0x1,0x82(a2)" "ori.w   #0x0,0x82(a2)" "munición infinita"; }

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
      in_fn "$F_INIT" PlayerEntity_InitAuxState_032A02 "move.b  #0xa, 0x80(a6)" "move.b  #${HEX}, 0x80(a6)" "bombas iniciales = $N" ;;
    bombs-infinite)
      # 'subi.b #1,0x80(a0)' (6 B) -> 'ori.b #0,0x80(a0)' (6 B, no-op) en las dos rutinas de lanzamiento
      for fn in Player_ThrowGrenadeBack_033256 Player_ThrowGrenade_0332bc; do
        in_fn "$F_CORE" "$fn" "subi.b  #0x1,0x80(a0)" "ori.b   #0x0,0x80(a0)" "granadas infinitas"
      done ;;
    pistol-ammo=*)
      N=${p#pistol-ammo=}; HEX=$(printf '0x%x' "$N")
      sed -i "s/move\.w  #0xa,0x82(a6) /move.w  #${HEX},0x82(a6)/" asm/player/air_death_crouch_036632.s "$F_CORE"
      C=$(cat asm/player/air_death_crouch_036632.s "$F_CORE" | grep -c "move.w  #${HEX},0x82(a6)")
      [ "$C" = 3 ] || { echo "[patch] pistol-ammo: $C/3 sitios"; exit 1; }
      echo "[patch] munición pistola al perder arma = $N" ;;
    invincible)
      # Probe_Bit3At100001_0334A2: 'beq.w Player_DeathGate' (4 B) -> 2 x nop (4 B):
      # siempre el camino de depuración (borra bits 3/0 de $13(a6), sin muerte)
      in_fn "$F_ROUTE" Probe_Bit3At100001_0334A2 "beq.w   Player_DeathGate_0334c6 " "nop
        nop                             " "jugador invulnerable"
      # Slug_DamageTick_02a6be: 'bne.w Slug_ResetHP' -> 'bra.w' (4 B): el Slug no pierde HP
      in_fn "$F_SLUG" Slug_DamageTick_02a6be "bne.w   Slug_ResetHP_02a712" "bra.w   Slug_ResetHP_02a712" "Slug invulnerable" ;;
    debug=*)
      N=${p#debug=}
      # UserMode0_080C: 'bne.w 1f' -> 'bra.w 1f' (4 B) y 'move.w #0' -> 'move.w #N' (8 B)
      python3 - "$F_BIOS" "$N" <<'PY'
import sys
p,n=sys.argv[1:]; s=open(p).read()
a='"bne.w   1f                     \\n"'; b='"1: move.w #0, 0x100000         \\n"'
assert s.count(a)==1 and s.count(b)==1, "patrón de UserMode0_080C no encontrado"
s=s.replace(a,'"bra.w   1f                     \\n"').replace(b,'"1: move.w #0x%s, 0x100000   \\n"'%n)
open(p,"w").write(s); print("[patch] UserMode0_080C: $100000.w = $%s al arrancar"%n)
PY
      ;;
    mission=*)
      N=${p#mission=}
      # Dispatcher_ModeTable_001922, bloque state=0 (lo que se ejecuta al pulsar
      # START): opcode $106ECE y handler de escena de la misión N (mismos bytes).
      python3 - "$F_MODE" "$N" <<'PY'
import sys
p,n=sys.argv[1],int(sys.argv[2])
# misión -> (estado de Dispatcher_ModeTable, handler): pares originales;
# el estado 2 es la misión 4 (ciudad en ruinas) y el 3 la misión 3 (nieve)
H={1:(0,0x967fe),2:(1,0x96840),3:(3,0x968c4),4:(2,0x96882),5:(4,0x96906),6:(5,0x96948)}
assert n in H, "mission=1..6"
s=open(p).read()
a="        move.b  #0x0, 0x106ece.l               | +00  opcode = 0\n"
b="        move.l  #0x967fe, (a0)                 | +0e  handler = $967FE\n"
assert s.count(a)==1 and s.count(b)==1, "patrón de Dispatcher_ModeTable no encontrado"
st,h=H[n]
s=s.replace(a,"        move.b  #0x%x, 0x106ece.l               | +00  opcode = %d (parche)\n"%(st,st))
s=s.replace(b,"        move.l  #0x%x, (a0)                 | +0e  handler misión %d (parche)\n"%(h,n))
open(p,"w").write(s); print("[patch] la partida empieza en la misión %d"%n)
PY
      ;;
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
