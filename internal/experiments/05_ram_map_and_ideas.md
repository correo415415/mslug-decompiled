# 05 — Mapa de RAM útil e ideas de experimentos

## RAM (direcciones CPU)
| Dirección | Significado |
|---|---|
| `$100440` / `$1004E0` | entidad jugador P1 / P2 (bloque de $A0 B) |
| `+$71` | arma (0 pistol, 1 HMG, 2 shotgun, 3 rocket, 4 flame) |
| `+$80` | bombas (byte) |
| `+$82` | munición del arma (word) |
| `+$7A` | contador de disparos (para patrón de balas) |
| `$10FD84` | bloque soft-DIP: `+4`=$10FD88 vidas, `+7`=$10FD8B dificultad, `+8` rank, `+A` continue |
| `$10FDAF` | modo (1 = ?, 2 = 2 jugadores activos) |
| `$1081BF/$1081C0` | créditos BCD |
| `$106ECE` | escena / modo actual (9 = misión; $FF = idle) |
| `$106ED2` | hud_dirty |
| `$10E486` | créditos/continues en opciones |
| `$100260..$1008A0` | slots de tarea fijos (HUD P1/P2, etc.) |

## Ideas (todas "mismo tamaño")
* **Invencibilidad**: en `HUD_State_PlayerDeath` / la rutina que resta `$77(a6)`
  (`subi.b #1,0x77(a6)` → `tst.b 0x77(a6)`; o mejor en la colisión del jugador).
* **Tiempo de misión**: `Hud_WriteTimerCounters_05e912` / `Rank_DelayTbl_079968`.
* **Dificultad fija**: `AttractInit_Single_099AE2` (`move.b #2,$10FD8B` → `#0`).
* **Empezar en otra misión**: `BootScr_Mission_Start_001368` escribe `$106ECE=9`; el
  índice de escena viene de `$10FD84+…`/`SceneDescriptor[256]` en `$0916C8`.
* **Textos**: soft-DIP strings en `$1781F4` (`Str_17873c` en `data/level_d_16b035.s`) —
  mismo largo (12 B por entrada, rellenar con espacios).
* **Velocidad del jugador**: tablas de deltas en `player/tables_fx_030696.s`.
