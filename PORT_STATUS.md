# Estado del port

| | |
|---|---|
| Rutinas de código en la P-ROM (registro de la decompilación, < `$9C608`) | 8 482 (640 520 B) |
| Portadas a Rust | 15 |
| Verificadas 1:1 (`mslug verify --frames 6000`) | 15 (849 llamadas, 0 fallos; 9 de ellas ejercitadas) |
| Frames idénticos `run` vs `run --rom-only` | 8/8 capturas en 4000 frames |

## Funciones portadas

| Dirección | Rutina | Módulo | Llamadas verificadas |
|---|---|---|---|
| $001E56 | Store106EDA | kernel | 0 |
| $05E9E4 | RNG_LFSRStep (generador aleatorio) | kernel | 8 |
| $043EDA | Buffer_ClearBlock1024L | kernel | 7 |
| $044048 | Coord_ApplyCameraDeltaToSelf | entity | 780 |
| $04405E | Coord_ApplyCameraDeltaToA0Mark | entity | 13 |
| $04407A | Coord_ApplyCameraTerciaryToSelf | entity | 0 |
| $044090 | Coord_ApplyCameraDeltaToGlobals | entity | 0 |
| $0440AA | Coord_ApplyCameraDeltaToD1D2 | entity | 0 |
| $0440BC | Coord_LocalToScreenSecondary | entity | 35 |
| $0440D0 | Coord_ScreenToLocalSecondary | entity | 3 |
| $065C64 | Hostage_SinCos | entity | 0 |
| $02A2AC | Slug_GetGauge | entity | 0 |
| $05180C | Clear8Bytes | entity | 2 |
| $0516BA | Players_SnapshotMods | entity | 0 |
| $09773C | HiScore_LoadDefaults | hiscore | 1 |

Las funciones con 0 llamadas compilan y siguen la ROM instrucción a
instrucción, pero el guion de `verify` aún no las ejercita: no cuentan como
verificadas.

## Plan

1. Hojas (rutinas sin llamadas): 323 en el registro. Lote actual.
2. Rutinas con `jsr/bsr` a funciones ya portadas: el llamante en Rust invoca
   directamente a la función Rust (sumando los ciclos del `jsr`/`rts`).
3. Despachadores de tareas / máquinas de estado (`Task_*`, `SetTaskHandler`):
   el planificador del juego pasa a Rust.
4. Programa principal desde el `USER` del BIOS: cuando el bucle de juego
   entero esté en Rust, la tabla nativa cubre todo el código y la P-ROM
   sólo aporta datos.
5. Más guiones para `verify` (todas las misiones, attract, final, sound test)
   para que cada función portada tenga llamadas verificadas.
