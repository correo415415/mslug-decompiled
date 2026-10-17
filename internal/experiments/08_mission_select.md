# 08 — Empezar en otra misión

**Dónde**: `Dispatcher_ModeTable_001922` (`asm/boot/dispatcher_mode_table_001922.s`).
Al pulsar START la tabla `$000C8A` (fila del estado 0 de `BootDispatchTable`)
salta al bloque *state=0*, que publica `$106ECE = 0` y pone en el slot fijo
`$100260` el handler de escena `Attract_State0_Handler_0967FE`
(`move.b #0,d0; jsr SceneLoader_Main_043568` = escena 0, misión 1). Los demás
bloques del mismo macro son el resto de misiones: el par (estado, handler) va
siempre junto (el estado selecciona banner, música y datos de la misión).

| Misión | Estado `$106ECE` | Handler |
|---|---|---|
| 1 | 0 | `$0967FE` (escena 0) |
| 2 | 1 | `$096840` (escena 1) |
| 3 (nieve) | 3 | `$0968C4` (escena 3) |
| 4 (ciudad en ruinas) | 2 | `$096882` (escena 2) |
| 5 | 4 | `$096906` (escena 4) |
| 6 / final | 5 | `$096948` (escena 5) |

(El orden de las escenas 2 y 3 no es el de las misiones.)

**Cambio** (mismo tamaño): en el bloque *state=0* el inmediato de
`move.b #0,$106ECE` y el long de `move.l #$967FE,(a0)`.

```bash
bash scripts/apply_patch.sh mission=4 --zip internal/mslug.zip
```
Bytes (`mission=3`): `$001925: 00→03`, `$001934..35: 67 FE → 68 C4`.

**Verificación**: emulador, frame 3600 tras moneda+START: banner
"MISSION n START!" de la misión pedida para n = 2..6 (`results/mission_select_demo.png`).
Probado también que no basta con escribir `$106ED0` en RAM: el juego lo vuelve
a poner a 0 al empezar (`MemCard_CopySlotFlag_051696` / bloque state=0).
