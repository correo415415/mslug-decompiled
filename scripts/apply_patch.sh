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
#   boot=soundtest arrancar en el SOUND TEST interno (T6[13] BootScr_SoundTest_001566).
#   boot=ending    arrancar en el final / créditos (T4: escena 12, BootScr_SceneBCD_Init).
#   debug-hud      HUD de variables de depuración (= debug=0001).
#   debugcoll      HUD + inspector de colisión DebugColl (= debug=0003).
#   altjump        arco de salto alternativo de depuración (= debug=0020).
#   slughud        HUD de cadena del Slug (= debug=0010).
#   blood          sangre siempre activa (ignora el soft-DIP BLOOD $10FD8F, 9 lectores).
#   no-blood       sangre siempre desactivada (versión "sin gore").
#   ending=jp|intl fuerza el stream del final japonés ($0F1CD4) o internacional ($0F260E).
#   bossshadow=a|b lanza Boss_Shadow_Init variante A ($86586) / B ($86582) al empezar
#                  cada misión, sin flags ni mando de P2 (DebugTriggers_TwoBits_096B24).
#   warpkeys       conecta AutoDemo_DebugWarpKeys_0797ce (huérfana) al demo automático:
#                  en el attract, P1 D+C teletransporta, D+B borra $106E92.
#   fix-input-bug  corrige el bug de SNK de InputLayout_ReadField0_05D616 (orden lea/movea).
#   fix-deadstore  quita el store muerto `movea.l #-1,a0` de $041A96 (nop x3).
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
F_SCHED=asm/kernel/scheduler_bootstrap_000e8e.s
F_PARA=asm/enemies/soldiers/barrel_paratrooper_shield_tank_066000.s
F_HDEATH=asm/pow/human_death_049ff2.s
F_CREW=asm/props/fx_explosions_platforms_crew_076000.s
F_EVM=asm/mission/event_vm_04422a.s
F_DTRIG=asm/frontend/attract_sub_helpers_0967c0.s
F_AIM=asm/input/aim_tables_05d316.s
F_SQUAD=asm/enemies/squad/children_handlers_041408.s
PATCHABLE=("$F_MODE" "$F_SCHED" "$F_PARA" "$F_HDEATH" "$F_CREW" "$F_EVM" "$F_DTRIG" "$F_AIM" "$F_SQUAD" "$F_HUD" "$F_FIRE" "$F_CORE" "$F_INIT" asm/player/air_death_crouch_036632.s "$F_ROUTE" "$F_SLUG" "$F_BIOS")
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
# alias (funciones ocultas = combinaciones de flags de depuración)
for i in "${!PATCHES[@]}"; do
  case "${PATCHES[$i]}" in
    debug-hud) PATCHES[$i]="debug=0001" ;;
    debugcoll) PATCHES[$i]="debug=0003" ;;
    altjump)   PATCHES[$i]="debug=0020" ;;
    slughud)   PATCHES[$i]="debug=0010" ;;
  esac
done
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
    blood|no-blood)
      # Cada lector hace 'tst.b $10FD8F' + 'bne/beq'. blood: rama de "BLOOD ON"
      # siempre; no-blood: nunca. bne.w <-> bra.w / nop;nop (4 B), beq.w igual.
      python3 - "$p" "$F_PARA" "$F_HDEATH" "$F_CREW" "$F_CORE" <<'PY'
import re,sys
mode=sys.argv[1]; n=0
for p in sys.argv[2:]:
    s=open(p).read()
    def rep(m):
        global n
        br,tgt=m.group(2),m.group(3)
        on = (br=="bne")              # bne = salta si BLOOD ON
        if mode=="blood":  new = "bra.w   " if on else "nop; nop"
        else:              new = "nop; nop" if on else "bra.w   "
        n+=1
        if new.startswith("nop"):
            return m.group(1)+"nop\n        nop                                     | (parche %s)\n"%mode
        return m.group(1)+"bra.w   %s | (parche %s)\n"%(tgt,mode)
    s=re.sub(r"(tst\.b\s+0x10fd8f\.l[^\n]*\n\s+)(bne|beq)\.w\s+(\S+)[^\n]*\n",rep,s)
    open(p,"w").write(s)
assert n==9, "BLOOD: %d/9 lectores"%n
print("[patch] BLOOD forzado a %s en %d lectores de $10FD8F"%("ON" if mode=="blood" else "OFF",n))
PY
      ;;
    ending=*)
      e="${p#ending=}"
      case "$e" in
        intl) in_fn "$F_EVM" MissionDriver_Init_0442E6 "beq.w   .L44328                                | +038" "nop; nop                                       | +038 (parche ending=intl)" "final internacional siempre" ;;
        jp)   in_fn "$F_EVM" MissionDriver_Init_0442E6 "move.l  #0xf260e, d0                           | +03c" "move.l  #0xf1cd4, d0                           | +03c (parche ending=jp)" "final japonés siempre" ;;
        *) echo "[patch] ending=jp|intl"; exit 1 ;;
      esac ;;
    bossshadow=*)
      v="${p#bossshadow=}"
      case "$v" in a) L=".Lskip_bit2"; B=2 ;; b) L=".Lskip_bit7"; B=7 ;; *) echo "[patch] bossshadow=a|b"; exit 1 ;; esac
      python3 - "$F_DTRIG" "$L" "$B" <<'PY'
import sys,re
p,L,B=sys.argv[1:]; s=open(p).read()
i=s.find("\nDebugTriggers_TwoBits_096B24:\n"); j=s.find("rts",i)
blk=s[i:j]
k=blk.find("btst.b  #0x%s, 0x100001.l"%B); e=blk.find(L+":",k)
sub=blk[k:e]
sub2=re.sub(r"(beq|bcc)\.w   "+re.escape(L)+r"\s*",lambda m:"nop; nop        ",sub)
assert sub2.count("nop; nop")==3, sub2
s=s[:i]+blk[:k]+sub2+blk[e:]+s[j:]
open(p,"w").write(s); print("[patch] DebugTriggers: Boss_Shadow variante bit%s siempre al empezar misión"%B)
PY
      ;;
    warpkeys)
      # AutoDemo_JumpIfBlocked_0796f6 (llamada cada frame por el demo
      # automático de la misión 2) termina en 'rts' justo antes de
      # AutoDemo_DebugWarpKeys_0797ce: con 'nop' cae en ella (2 B).
      in_fn "$F_CREW" AutoDemo_JumpIfBlocked_0796f6 "        rts                                     | +0d6" "        nop                                     | +0d6 (parche warpkeys)" "cae en AutoDemo_DebugWarpKeys_0797ce" ;;
    fix-input-bug)
      in_fn "$F_AIM" InputLayout_ReadField0_05D616 "        movea.l 0x72(a1), a2                           | +012  BUG SNK: lee \$72 con el
        lea     0x100300.l, a1                         | +016  a1 VIEJO (orden invertido)" "        lea     0x100300.l, a1                         | +012  (parche fix-input-bug)
        movea.l 0x72(a1), a2                           | +018" "orden lea/movea corregido" ;;
    fix-deadstore)
      python3 - "$F_SQUAD" <<'PY'
import sys
p=sys.argv[1]; s=open(p).read()
a="        movea.l #-1, a0                         | +09a  DEAD STORE (pisado)\n"
assert s.count(a)==1
s=s.replace(a,"        nop                                     | +09a (parche fix-deadstore)\n        nop\n        nop\n")
open(p,"w").write(s); print("[patch] store muerto de \$041A96 eliminado")
PY
      ;;
    boot=*)
      b="${p#boot=}"
      case "$b" in
        soundtest) tgt="BootTblEntry_E6E" ;;
        ending)    tgt="BootDispatchTable_000B92+0x1f8" ;;
        *) echo "[patch] boot=soundtest|ending"; exit 1 ;;
      esac
      python3 - "$F_SCHED" "$tgt" <<'PY'
import sys
p,t=sys.argv[1:]; s=open(p).read()
a="        lea.l   BootTblEntry_B92(pc), a0        | +0b8 a0 = &tbl_$B92 (T1)\n"
assert s.count(a)==1, "patrón de SchedulerBootstrap no encontrado"
s=s.replace(a,"        lea.l   %s(pc), a0        | +0b8 (parche boot)\n"%t)
open(p,"w").write(s); print("[patch] arranque redirigido a",t)
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
