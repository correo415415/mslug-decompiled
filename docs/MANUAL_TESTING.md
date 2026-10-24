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

## 4. Sólo el port escrito a mano (`--only-manual`)

```sh
./target/release/mslug play --only-manual --frames 6000      # sin ventana: dice dónde se para
./target/release/mslug-sdl --only-manual                     # ventana; el título dice dónde se paró
./target/release/mslug-sdl --only-manual --no-bios-code      # tampoco ejecuta código recompilado de la BIOS
./target/release/mslug play --only-manual --halt-shot build/shots/parada.png  # y guarda captura de la parada
```

Al pararse, `mslug-sdl` guarda siempre una captura en `build/shots/halt-NNNNNN.png`, también con la ventana abierta. `mslug play` la guarda en el fichero que se le indique con `--halt-shot`.

Ejecuta **sólo** las funciones Rust escritas a mano: no hay programa
recompilado ni intérprete para el juego. La BIOS (que no es del juego) sigue
ejecutándose salvo con `--no-bios-code`. En cuanto el juego llega a una
rutina sin portar, la máquina se para y se informa de cuál es (con su nombre
de la decompilación) y, con `--frames`, se guarda una captura `halt-*.png`.
Es la medida de "hasta dónde llega el port a mano".

Además de las funciones de juego (`mslug_bridge::PORTS`), en este modo se
sustituyen las **rutinas de sistema** (`mslug_bridge::system::SYSTEM_PORTS`):
petición `USER` de la BIOS, inicialización, interrupciones de vblank y
temporizador, planificador de tareas, bucle de frame, mandos del frame,
pausa, cola de sonido, paleta, sprites y VRAM (`crates/game/src/system/`).
Estas rutinas pueden no "volver" (terminan una interrupción, saltan a la
BIOS o ceden el control a una tarea): devuelven un `Flow`. Cuando llaman a
una rutina original aún sin portar (`Machine::call_original`), en el modo
sólo manual la máquina se para ahí.

Estado (este PR): el juego arranca, inicializa todo, arranca el planificador
y la tarea de arranque, y entra en el guion de arranque (elige la lista de
escenas y la despacha); ejecuta la primera escena (logo: créditos y nivel en la capa fija),
la pantalla de título (logo animado por guion, copyright, mensaje parpadeante
según créditos, cuenta atrás) y las tareas fijas, hace el render del frame (`system::palette`,
`system::render`, `system::sprite`), acepta el inicio de partida
(`PLAYER_START`), entra en la misión, carga la escena y corre el manual
de juego (con sus iconos) y el guion de scroll (capas, colisión y listas
por frame) y suelta el Slug, que cae, baja y estrena la torreta
(`system::turret`), el motor de pendientes (`system::slope`) y las ruedas
(`system::wheels`); aparece el jugador (`system::player`) con su brazo
(`system::player_arm`), el texto del manual (`system::manual_icons`) y
el jugador en el aire (`system::player_air`: golpe, caída, acciones,
muerte) y en el suelo (`system::player_ground`: aterrizaje, espera, cola de
pie) y el Slug aparca y espera a que se monte un jugador
(`system::slug_idle`); se para en el frame 621 en `$034704`
(`Player_Stand_034704`, el jugador de pie, aún sin portar).

La captura de `--halt-shot` (y la de la ventana SDL) se toma tras avanzar
un frame de vídeo con la CPU congelada: la parada ocurre a mitad de frame y
la imagen presentada era la del VBLANK anterior (salía negra o atrasada).

### Progreso: `mslug progress`

```sh
./target/release/mslug progress
```

Cuenta las funciones del juego (las rutinas de código de la decompilación,
`tools/recomp/decomp_funcs.tsv`), cuántas tienen ya función Rust escrita a
mano (entradas de `PORTS` y `SYSTEM_PORTS`), el porcentaje por número y por
bytes, y ejecuta el modo sólo manual para decir en qué frame y rutina se
para y qué escena del guion estaba en curso. La CI lo añade a su resumen.

### Qué portar después: `mslug roadmap`

```sh
./target/release/mslug roadmap --frames 6000 --out build/roadmap.txt
```

Juega con el programa recompilado y lista las rutinas del juego en el orden
en que se ejecutan por primera vez, cuáles están portadas y las más
calientes. Las primeras sin portar son las que parará `--only-manual`.

### Comprobar las rutinas de sistema: `mslug verify-system`

```sh
./target/release/mslug verify-system --frames 6000 --fuzz 1
```

En cada entrada de una rutina de sistema portada, ejecuta la Rust y la ROM
desde el **mismo estado completo** de la máquina (CPU y bus clonados: RAM,
paleta, VRAM, registros de vídeo y sonido) hasta el mismo punto (PC y pila
que dice el `Flow` de la Rust) y compara RAM (salvo la zona de la pila),
paleta, VRAM, dirección/incremento de VRAM, máscara de interrupciones y la
última orden al Z80. Los registros distintos se cuentan aparte (`regs≠`):
son valores que la rutina original deja "de paso" y que los llamadores no
usan. La CI lo ejecuta con dos semillas de fuzz.

## Camino sin moneda (attract)

```
./target/release/mslug verify-system --frames 4000 --coin 0   # 0 fallos (149 043 llamadas)
./target/release/mslug play --only-manual --coin 0 --frames 20000 | grep parado
```

Sin moneda el attract carga la escena 10 (Game Over/Continue, `system::game_over`)
y después arranca a los jugadores (`script::step_start_players`, `$001846`).
Luego la demo jugable: tabla de modos (`script::mode`), arranque de misión
(`script::start_mission`), tiempo (`system::rank`), tarea de la misión con su
cámara y apariciones (`system::mission`) y las rutinas 68000 embebidas en los
guiones de escena (7 moldes, ejecutados en `scroll::embedded`).
Después el HUD de cada jugador (`system::player_hud`), los eventos de la
misión (`system::events`), la comida (`system::food`), las plataformas
móviles (`system::platforms`), la colisión y la caída de los objetos
(`system::collision`), el movimiento con colisión (`system::motion`), la
recogida (`system::pickup`), los objetos del suelo (`system::items`) y los
soldados que saltan (`system::grunt`), el piloto automático
(`system::autopilot`) y el indicador GO (`system::go_arrow`).
`--only-manual --coin 0` llega al frame 1636 y se para en
`Pillbox_Init_Right_0868a6` (casamatas del fuerte). `verify-system
--coin 0` (4000 frames): 0 fallos en 232 236 llamadas; con moneda (3000
frames): 0 fallos en 193 409.

Si el código original se para dentro de una llamada (`Machine::halted`), los
bucles que esperan algo de ella (guion de scroll, apariciones) salen en vez de
colgarse. Con moneda sigue
parándose en el frame 503 en `$02B004` (caída del Slug).
