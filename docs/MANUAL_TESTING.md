# Probar el port a mano (SDL2) y extraer los assets

## 1. Assets del juego

```sh
scripts/extract_assets.sh                     # internal/mslug.zip -> build/assets/ (~1,5 min)
scripts/extract_assets.sh otro/mslug.zip sprites levels   # otro romset / sólo algunos pasos
```

Las herramientas de `assets/tools/` son las de la rama de la decompilación
(`assets/tools/{gfx,sprites,levels,audio}.py`, `tools/scene_script_dump.py`),
adaptadas para leer el romset directamente (`assets/tools/romset.py`, sin
extraer ROMs a disco) y con los nombres de rutina de la decompilación en
`assets/map/labels.tsv`. La salida es **idéntica** a la de la otra rama
(comparada fichero a fichero), y `gfx.py build` / `audio.py build`
reconstruyen las ROMs con el MD5 original.

| Carpeta | Contenido |
|---|---|
| `build/assets/sprites/anims/` | ~950 animaciones (GIF a 59,185 Hz + tira PNG) con el nombre de la rutina que las usa; galería en `index.html` |
| `build/assets/sprites/frames/` | ~9 800 frames montados como los dibuja el juego |
| `build/assets/sprites/palettes/` | las 1 536 paletas |
| `build/assets/levels/` | fondos de las 13 escenas (6 misiones, attract, game over...) + `index.html` |
| `build/assets/audio/` | 375 muestras ADPCM-A/B con nombre (WAV) + `index.csv` |
| `build/assets/fix/`, `sprites/bank{0,1}/` | tiles crudos (PNG indexado, reversibles a ROM) |

Sirven de referencia visual al portar (qué animación usa cada rutina,
cómo debe verse un fondo, qué muestra suena) y, más adelante, como datos del
port sin ROM.

## 2. Jugar con SDL2

```sh
cargo build --release -p mslug-sdl
./target/release/mslug-sdl                    # ventana x3 con sonido
./target/release/mslug-sdl --scale 4 --mute
```

Teclas: P1 cursores, Z/X/C/V = A/B/C/D (disparo, salto, granada), Enter/1
START, 5 moneda, F2 servicio · P2 I/K/J/L, Q/W/E/R, 2 START, 6 moneda ·
F11 pantalla completa · F12 captura (`build/shots/`) · P pausa · Tab turbo ·
Esc salir.

Sin pantalla (como la CI): capturas automáticas.

```sh
SDL_VIDEODRIVER=dummy SDL_AUDIODRIVER=dummy ./target/release/mslug-sdl \
    --frames 4000 --auto-coin 400 --mute --shot-at 1000,2000,3000,4000
```

## 3. El port manual en la partida

`mslug-sdl` ejecuta el juego entero (programa recompilado). Lo que ya está
reescrito a mano en `crates/game` se comprueba contra la ROM con

```sh
./target/release/mslug verify-manual --fuzz 1 --frames 15000
```

(ver `docs/PORT_MANUAL.md`). Cuando un sistema entero esté portado se
enganchará en `mslug-sdl` en lugar de la versión recompilada, y se podrá
probar jugando.
