# 07 — Flags de depuración (`$100000`/`$100001`)

`UserMode0_080C` (`src/bios/bios_user_dispatch.c`) borra el word `$100000` al
arrancar; el resto del programa consulta sus bits como interruptores de
depuración (restos del desarrollo, sin DIP físico que los active):

| Bit | Dónde se lee | Efecto |
|---|---|---|
| `$100001` b0 | `Debug_DrawHUDVars_096A80`, `Attract_PostStart_Cleanup_001AB6`, `Task_InstallMissionSlots_001dcc`, `SpawnStream_Dispatch_04cb88`, `M4_CamFloor_*` | dibuja variables en el HUD; atajos de escena/spawn |
| b1 | `ScriptOp02_Sprite_028dfe` (+$84), `DebugColl_Init_05efca` | fuerza el flag de sprite `$5A` b7 / cajas de colisión visibles |
| b2 | `DebugTriggers_TwoBits_096B24`, `Player_AirActionSelect_037168` | tarea de depuración A ($86586) con dos botones; acciones aéreas |
| b3 | `Probe_Bit3At100001_0334A2`, `Slug_DamageTick_02a6be` | **invulnerabilidad** (receta 06) |
| b4 | `ScriptOp02_Sprite_028dfe` (+$124), `Chain3_*` (vehicle_02dd20) | HUD de depuración del Slug / cadena 3 |
| b5 | `Player_JumpAir_036a70`, `Slug_DamageTick_02a6be` | salto aéreo alternativo; `$66(a6)=$300` en el Slug |
| b7 | `DebugTriggers_TwoBits_096B24` | tarea de depuración B ($86582) |
| `$100000` b0/b1/b7 | `Entity_MoveAndCollide_C_0272a8`, `PlayerGrenade_SpawnB_03093a`, `InputLayout_ReadField*` / `Ent_InputSample_04546E` | b7 = reproducción de entrada (replay) |

**Cambio** (mismo tamaño, 2 B tocados): en `UserMode0_080C` la rama
`bne.w 1f` pasa a `bra.w 1f` y el `move.w #0,$100000` de esa rama a
`move.w #N,$100000`.

```bash
bash scripts/apply_patch.sh debug=0008 --zip internal/mslug.zip   # b3: invulnerable
bash scripts/apply_patch.sh debug=0001 --zip internal/mslug.zip   # b0: HUD de variables
```
Bytes (`debug=0008`): `$000818: 66→60`, `$000829: 00→08`.

**Verificación**: `--watch-ram 100000:2` muestra `$100000=0008` durante la
partida (frames 4400+). Bits uno a uno: ver receta 09 (b0, b0+b1, b3 y b5
confirmados en emulador; b2/b7 necesitan el mando de P2; b4 sin efecto visible
sin Slug). Nota: el juego reescribe `$100000` al pulsar START, así que con
`--poke-ram` hay que repetir el poke (`tools/dev/emu_pokes.py`).
