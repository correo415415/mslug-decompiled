# Recetario de modificaciones

Cada receta = qué cambia en las fuentes, bytes que toca en la ROM (direcciones
CPU del P-ROM en formato `build/mslug_prom.bin`), y cómo comprobarla.
Regla de oro: **conservar el tamaño de cada función** (sustituir instrucciones
por otras del mismo número de bytes) para que el resto de la ROM siga
byte-idéntica y `check_section_sizes.py` dé 0 discrepancias.

Flujo general:
```bash
bash scripts/apply_patch.sh <receta...> --zip internal/mslug.zip   # edita .s → rebuild → build/repack/mslug.zip
bash scripts/dev/run_emu.sh build/repack/mslug.zip 4800 /tmp/f     # jugar/verificar en headless
bash scripts/apply_patch.sh --undo                                 # restaurar fuentes
```

| Receta | Fichero | Verificada |
|---|---|---|
| [01 vidas iniciales](01_lives.md) | `scripts/apply_patch.sh lives=N` | ✅ emulador (1UP=8) |
| [02 HMG infinita](02_hmg_infinite_ammo.md) | `scripts/apply_patch.sh hmg-infinite` | ✅ emulador (ARMS fijo 200, RAM `$1004C2`) |
| [03 todas las armas infinitas](03_all_weapons_infinite.md) | `scripts/apply_patch.sh all-infinite` | rebuild OK |
| [04 bombas iniciales](04_bombs.md) | `scripts/apply_patch.sh bombs=N` | rebuild OK |
| [05 ideas / mapa de RAM](05_ram_map_and_ideas.md) | — | notas |

Herramientas del emulador parcheado (solo `internal/pydmg`):
* `--input-script "F1-F2:MASK,..."` mantiene bits del mando P1 entre frames (1=Up 2=Down 4=Left 8=Right 16=A 32=B 64=C 128=D).
* `--watch-ram "ADDR:LEN,..."` imprime bytes de RAM cada `--dump-every-frames`.
* `--poke-ram "FRAME@ADDR=HEX,..."` escribe RAM justo antes de un frame (p. ej. dar el HMG: `4400@1004B1=01,4400@1004C2=00C8`).
