# Metal Slug — port 1:1 a Rust

Proyecto independiente (rama `port` del repositorio de la decompilación).
Objetivo: reescribir en Rust **todas** las rutinas del programa 68000 de
Metal Slug (Neo Geo MVS, 1996, P-ROM `201-p1.bin`) de forma que el juego se
comporte **exactamente** igual que el original: mismos frames, mismo audio,
mismas partidas con las mismas entradas.

> El romset MAME original (`mslug.zip`, con la BIOS `neogeo`) está en
> `internal/mslug.zip` mientras el repositorio sea privado. **Borrar
> `internal/` antes de cualquier publicación.** `--rom` permite usar otro.

## Cómo funciona

```
internal/      mslug.zip (romset MAME original, sólo uso privado)
crates/
  game/        mslug-game: el juego escrito a mano en Rust legible (player/,
               enemies/, items/...), sin direcciones; ver docs/PORT_MANUAL.md
  gen/NN/      mslug-gen-NN: el programa 68000 (juego + BIOS) traducido a Rust,
               generado por tools/recomp/m68k_recomp.py y repartido en ~18
               crates de ~5 400 instrucciones (cada rustc cabe en ~300 MB)
  mslug-code/  despacho por PC a esas funciones (+ intérprete de reserva)
  mslug-sdl/   frontend SDL2 (ventana, sonido, mandos)
  neogeo-hw/   hardware Neo Geo: 68000, Z80, YM2610, LSPC, bus, cargador de ROM
               (vendorizado de pydmg-neogeo, MIT/Apache, sin frontend SDL)
               + `native.rs`: tabla de funciones nativas enganchada a la CPU
  mslug/       el juego
    src/game/  rutinas portadas, una función Rust por rutina 68000
    src/verify.rs  verificador diferencial ROM ↔ Rust
```

1. **Hardware emulado, juego nativo.** El Neo Geo (vídeo, sonido Z80/YM2610,
   BIOS) sigue emulado. El programa del juego se sustituye **función a
   función**: cada rutina portada se registra por su dirección de entrada y,
   cuando el PC del 68000 llega ahí, se ejecuta la versión Rust en lugar de
   interpretar la ROM.
2. **Contrato 1:1.** Una función portada debe dejar D0–D7, A0–A7, SR
   (X N Z V C), PC, la RAM y **los ciclos consumidos** idénticos a los de la
   rutina original. Así el resto del programa (y el temporizado de vídeo y
   sonido) no nota la diferencia.
3. **Verificación diferencial.** `mslug verify` juega con la ROM y, en cada
   llamada a una función portada, ejecuta la versión 68000 y la Rust desde
   el mismo estado y compara todo. Cualquier diferencia se informa con el
   primer registro/byte/ciclo distinto.
4. **Nombres y semántica** vienen de la decompilación byte-idéntica (rama
   principal: `asm/`, `tools/registry.py`, `docs/`). Cada función Rust
   lleva el nombre y la dirección de la rutina que sustituye.

A medida que se portan funciones, los llamantes también pasan a Rust; cuando
todo el programa esté portado, la tabla de funciones nativas cubrirá desde el
punto de entrada del juego y la P-ROM sólo se usará como fuente de datos
(gráficos, tablas, guiones), igual que los datos de un port tradicional.

## Uso

```bash
cargo build --release                                # mslug (CLI) + programa recompilado
cargo build --release -p mslug-sdl                   # frontend con ventana (necesita libsdl2-dev)

./target/release/mslug-sdl                           # jugar (programa recompilado)
./target/release/mslug-sdl --interp                  # misma ventana, intérprete 68000 (referencia)
./target/release/mslug lockstep --frames 6000        # intérprete vs recompilado, instrucción a instrucción
./target/release/mslug play --frames 3000 --dump build/play --every 500
./target/release/mslug verify --frames 6000          # rutinas reescritas a mano vs ROM
```

Teclas de `mslug-sdl`: cursores, Z/X/C/V = A/B/C/D (disparo, salto,
granada), Enter = START, 5 = moneda; P2: I/J/K/L, Q/W/E/R, 2, 6.
F11 pantalla completa, F12 captura, P pausa, Tab turbo, Esc salir.

### Regenerar el programa traducido

```bash
# en el repo de la decompilación (rama principal):
python3 tools/export_code_map.py --check     # -> build/code_map/code.bin
cp build/code_map/code.bin <port>/tools/recomp/decomp_code.bin
# en el port:
./target/release/mslug dump-roms                               # build/recomp/{prom,bios}.bin
python3 tools/recomp/m68k_recomp.py --static                   # crates/gen/NN + Cargo.toml (sin trazas)
cargo build -j1 --release                                      # -j1 si hay poca RAM
```

## Port manual

```sh
cargo test -p mslug-game
cargo run --release -p mslug -- verify-manual --fuzz 1 --frames 15000   # Rust a mano vs ROM
MSLUG_MANUAL_EXACT_CYCLES=1 cargo run --release -p mslug -- lockstep-manual --atomic-ref --fuzz 1 --frames 15000
cargo run --release -p mslug -- verify-system --fuzz 1 --frames 6000    # rutinas de sistema vs ROM (estado completo)
cargo run --release -p mslug-sdl -- --manual                             # jugar con el Rust a mano
cargo run --release -p mslug -- play --only-manual --frames 6000         # SÓLO el Rust a mano: ¿dónde se para?
cargo run --release -p mslug -- roadmap --frames 6000                    # qué portar después
cargo run --release -p mslug -- progress                                 # funciones portadas, % y hasta dónde llega
scripts/extract_assets.sh                                                # assets -> build/assets/
```

Ver [docs/MANUAL_TESTING.md](docs/MANUAL_TESTING.md).

## Estado

Progreso del port escrito a mano (`mslug progress`):

| | |
|---|---|
| Funciones del juego (rutinas de código de la decompilación) | 7 887 (474 564 bytes) |
| Portadas a Rust legible | **910 (11,54 %)**; por bytes de código 86 366 (18,20 %) |
| Hasta dónde llega `--only-manual` | arranque, logo, título y demo hasta pulsar START; entra en la partida (inicio de misión) y corre el manual de juego (`system::manual`, sus iconos `system::manual_icons`) y el scroll de la escena (`system::scroll`, `system::layer`, `system::lists`); suelta el Slug (`system::slug`: alta, preparación y entrada en escena); el Slug cae, baja y estrena la torreta y el motor de pendientes (`system::turret`, `system::slope`), con sus ruedas (`system::wheels`) y el efecto de suelo (`system::ground_fx`); aparece el jugador (`system::player`) con su brazo y sus 79 poses (`system::player_arm`), el texto del manual (`system::manual_icons`) el jugador en el aire: golpe, caída, acciones y muerte (`system::player_air`) y en el suelo: aterrizaje, espera y cola de pie (`system::player_ground`); el Slug aparca y espera a que se monte un jugador (`system::slug_idle`); el jugador de pie y su disparo de pistola (`system::player_ground`, `system::player_fire`) y los escombros de la escena 3 (`system::debris`); la bala de pistola (`system::bullet`) y sus impactos (`system::ground_fx`), el giro y la cola de andar; el Slug se agacha al montarse, el brazo y el jugador montado; el Slug conducido (`system::slug_drive`: conducir, parado en llano, cañón y sonidos de motor) y su bala; el salto del Slug (`system::slug_jump`), la escena de mensaje y el diálogo de la tarjeta de memoria (`system::memcard`), el selector de guion (`system::msg_select`) y la partida del modo de mensaje (`script::scene_play`); el HUD completo del jugador (mensajes, rótulos, vidas, barra de vida del Slug y marcador, en `system::player_hud`), la reaparición del soldado en la marca de la misión y la bajada en paracaídas (`system::player`, `system::player_air`, `system::parachute`); el rótulo MISSION START con sus letras que caen y salen volando (`system::banner`), la caída libre al aparecer con el indicador 1UP (`system::player_air`, `system::icons`) y el andar disparando (`system::player_ground`); se para en el frame 1961 en `Soldier_SpawnVariants_057226+0x17C` (soldados enemigos). Sin moneda (`--coin 0`), el attract hace la escena de Game Over/Continue entera (`system::game_over`), el arranque de los jugadores (`$001846`) y la demo jugable: tabla de modos, arranque de misión (`script::start_mission`), tiempo de la misión (`system::rank`), tarea de la misión con cámara y apariciones (`system::mission`) las rutinas embebidas de los guiones de escena (en `system::scroll`), el HUD de cada jugador (`system::player_hud`), los eventos de la misión (`system::events`) la comida (`system::food`), las plataformas móviles (`system::platforms`), la colisión con el mapa y la caída de los objetos (`system::collision`) el movimiento con colisión (`system::motion`), la recogida de objetos (`system::pickup`), la cola de los objetos del suelo (`system::items`) y los soldados que saltan (`system::grunt`), la racha de puntos, el piloto automático de los tramos guiados (`system::autopilot`) y el indicador GO (`system::go_arrow`); se para en el frame 1636 en `Pillbox_Init_Right_0868a6` (casamatas del fuerte). |

Ver [PORT_STATUS.md](PORT_STATUS.md).
