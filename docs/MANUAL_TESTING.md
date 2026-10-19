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

## 3. Jugar con el port escrito a mano (`--manual`)

```sh
./target/release/mslug-sdl --manual           # ventana: lo portado a mano sustituye al original
./target/release/mslug play --manual --fuzz 1 --frames 15000   # sin ventana
```

Con `--manual`, cada rutina original que ya tiene función Rust legible en
`crates/game` (57 de las 60 portadas; ver `mslug_bridge::NEEDS_ROM_CALLEES`
para las 3 que aún llaman a rutinas sin portar) se sustituye en el programa
que se ejecuta: el juego llama directamente a la función Rust
(`mslug_bridge::call_rust`). Lo que no está portado sigue saliendo del
programa recompilado.

Comprobaciones:

```sh
# 1. Cada función Rust hace lo mismo que la ROM (RAM, resultado, llamadas)
./target/release/mslug verify-manual --fuzz 1 --frames 15000
# 2. La partida con --manual llega al mismo estado que sin él (RAM y paleta
#    en cada frame). Con la referencia "atómica" y ciclos exactos, compara
#    además el flujo instrucción a instrucción y señala el llamador culpable.
MSLUG_MANUAL_EXACT_CYCLES=1 ./target/release/mslug lockstep-manual --atomic-ref --fuzz 1 --frames 15000
```

Depuración: `MSLUG_MANUAL_ONLY=Player_,InputEvt` / `MSLUG_MANUAL_SKIP=...`
limitan qué rutinas se sustituyen (por nombre original);
`MSLUG_WATCH=1004C7` para el lockstep en el primer paso en que ese byte de RAM
difiere.

Sobre el tiempo: cada llamada Rust se ejecuta de una vez y cuenta 60 ciclos
(`CYCLES_PER_CALL`), no los del original. El juego **es sensible al tiempo**
(en qué instrucción cae la interrupción de vblank, cuánto tiempo libre queda
en el frame), así que una partida con `--manual` y otra sin él se separan
al cabo de unos cientos de frames, como si se jugara en una máquina un poco
más rápida: es jugable y correcta, pero no idéntica. Por eso la comprobación
de equivalencia usa `--atomic-ref` (la referencia también ejecuta cada rutina
original de una vez) y `MSLUG_MANUAL_EXACT_CYCLES=1` (la función Rust cuenta
los ciclos del original): así cualquier diferencia es de comportamiento, y el
lockstep compara además el flujo instrucción a instrucción. El contador de
tiempo libre `$106EE0` no se compara.

Lo que encontró `lockstep-manual` (y no `verify-manual`, que sólo compara
RAM y resultado): los llamadores usan valores que la rutina original deja
"de paso" en registros. `InputLayout_ReadField2` e `Input_*ByMode` dejan el
resultado en **todo** `d0` (el llamador hace `asl.w #1,d0`), y
`Player_SetWeaponAndAmmo` deja en `a1` la entidad del jugador. Forman parte
del contrato de la rutina y están en el adaptador (`Ret::D0L`, `Ret::A1`).
