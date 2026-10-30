# Estado del port

> **Dirección actual:** port escrito a mano en Rust legible (`crates/game`, sin
> direcciones ni registros del 68000), sistema a sistema, verificado contra la
> ROM con `mslug verify-manual`. Ver [docs/PORT_MANUAL.md](docs/PORT_MANUAL.md).
> El recompilador descrito abajo queda como red de seguridad.
>
> | Port manual | |
> |---|---|
> | **Funciones del juego** (rutinas de código de la decompilación, `tools/recomp/decomp_funcs.tsv`) | **7 887** (474 564 bytes) |
> | **Portadas a Rust legible** | **1384 = 17,55 %** (por bytes: 131 176 = 27,64 %); 1914 entradas sustituidas contando vectores y entradas internas |
> | **Hasta dónde llega** (`--only-manual`) | arranque → logo → título → demo → START → inicio de la misión 1 → manual de juego (con sus iconos) y scroll (capas, colisión, listas por frame); el Slug entra en escena; el Slug cae y baja, con su torreta, el motor de pendientes y las ruedas; aparece el jugador con su brazo, el texto del manual y el jugador en el aire y en el suelo (golpe, caída, aterrizaje, espera); el Slug aparca y espera a que se monte un jugador; el jugador de pie y su disparo de pistola, los escombros de la escena 3; la bala de pistola y sus impactos, el giro; el Slug se agacha al montarse, el brazo y el jugador montado; el Slug conducido (`system::slug_drive`: conducir, parado en llano, cañón y sonidos de motor) y su bala; el salto del Slug (`system::slug_jump`), la escena de mensaje y el diálogo de la tarjeta de memoria (`system::memcard`), el selector de guion (`system::msg_select`) y la partida del modo de mensaje (`script::scene_play`); el HUD completo del jugador (mensajes, rótulos, vidas, barra de vida del Slug y marcador, en `system::player_hud`), la reaparición del soldado en la marca de la misión y la bajada en paracaídas (`system::player`, `system::player_air`, `system::parachute`); el rótulo MISSION START con sus letras que caen y salen volando (`system::banner`), la caída libre al aparecer con el indicador 1UP (`system::player_air`, `system::icons`) y el andar disparando (`system::player_ground`); los soldados rebeldes aparecen con sus rasgos, la agresividad según la dificultad y la primera percepción de los jugadores, y andan, corren, frenan, huyen, saltan, se rinden y atacan cuerpo a cuerpo (`system::soldier`); mueren (`system::human_death`: caída, voltereta, salir despedido, trozos, quemarse y sangre) y se burlan del jugador (`system::soldier`); el prisionero atado (`system::items`) y el contacto entre cajas con los jugadores (`system::contact`); los carteles del fuerte que estampan su silueta en el mapa de colisión (`system::props`, `system::scroll`) y el soldado de guardia que embiste y el escaramuzador (`system::charger`); la torreta que apunta de la misión 1 con su cañón, proyectiles y casquillos (`system::aim_turret`) y la granada que lanzan los soldados (`system::enemy_grenade`) y los objetos arrojados con su sombra (`system::thrown`); la farola con su chispa, la choza y los tejados que se rompen (`system::scenery`) el ocupante de la choza con su puerta (`system::hut`), el barril flotante del río con su mina acuática (`system::barrel`) los humos y salpicaduras (`system::smoke`) y la torre de vigía, el búnker y el puente que se rompen por piezas (`system::tower`) y el soldado de la torre y de los pilares con su mortero (`system::tower_soldier`, con el movimiento «C» `motion::move_c`) y la muerte del jugador (`system::player_death`: los estados por tipo de muerte, la caída al vacío, la desaparición, la salpicadura y la sangre) y su caída al entrar en escena (`system::player_air::spawn_fall`); se para en el frame 4147 en `ContinueBig_Init_0467f6`; con `--hands-off` recorre el tutorial del Slug entero (destrucción, ataque suicida A + B, polvaredas) y llega a la misión 1, donde aparecen los soldados (`system::soldier`) y sueltan objetos (`system::items`) sin pararse en 9000 frames. Sin moneda (`--coin 0`), el attract hace la escena de Game Over/Continue entera (`system::game_over`), el arranque de los jugadores (`$001846`) y la demo jugable: tabla de modos, arranque de misión (`script::start_mission`), tiempo de la misión (`system::rank`), tarea de la misión con cámara y apariciones (`system::mission`) las rutinas embebidas de los guiones de escena (en `system::scroll`), el HUD de cada jugador (`system::player_hud`), los eventos de la misión (`system::events`) la comida (`system::food`), las plataformas móviles (`system::platforms`), la colisión con el mapa y la caída de los objetos (`system::collision`) el movimiento con colisión (`system::motion`), la recogida de objetos (`system::pickup`), la cola de los objetos del suelo (`system::items`) y los soldados que saltan (`system::grunt`); se para en el frame 1636 en `Pillbox_Init_Right_0868a6` (casamatas del fuerte). |
> | Cómo medirlo | `mslug progress` (también en el resumen de la CI) |
> | Rutinas originales con función Rust legible | 60 (input, player::controls/voice/weapons/invuln) |
> | `verify-manual` (auto 8000 + fuzz 1–3 × 15 000 frames) | ✅ 0 fallos |
> | Jugable: `mslug-sdl --manual` | 57 sustituidas (3 aún llaman a rutinas sin portar) |
> | `lockstep-manual --atomic-ref` (auto 8000 + fuzz 1–3) | ✅ mismo estado que el recompilado en cada frame |
> | Rutinas de sistema portadas (`crates/game/src/system`) | 203 entradas: arranque/USER, vblank, temporizador, planificador, bucle de frame, render, mandos, pausa, sonido, paletas y fundidos, lista de dibujo y SCB1–4, título, capa fija, VRAM, carga de escena, scroll y cámara, manual de juego |
> | `verify-system` (estado completo; auto 1500 + fuzz 1) | ✅ 0 fallos (12 000 frames con fuzz 1: 127 324 llamadas) |
> | **Sólo port manual** (`--only-manual`, sin recompilado ni intérprete) | arranca, inicializa, ejecuta la tarea de arranque y el guion de arranque; ejecuta la primera escena (logo) y el HUD de créditos; ejecuta la pantalla de título (logo animado, copyright, mensaje, cuenta atrás) y las tareas fijas; hace el render del frame (paletas y fundidos, orden de la lista de dibujo, SCB1–SCB4); da de alta los sprites en la lista de dibujo (`system::sprite`), acepta el inicio de partida (`PLAYER_START`) y entra en la misión, carga la escena, corre el manual de juego (`PLAY MANUAL`) y el guion de scroll; el Slug conducido (`system::slug_drive`: conducir, parado en llano, cañón y sonidos de motor) y su bala; el salto del Slug (`system::slug_jump`), la escena de mensaje y el diálogo de la tarjeta de memoria (`system::memcard`), el selector de guion (`system::msg_select`) y la partida del modo de mensaje (`script::scene_play`); el HUD completo del jugador (mensajes, rótulos, vidas, barra de vida del Slug y marcador, en `system::player_hud`), la reaparición del soldado en la marca de la misión y la bajada en paracaídas (`system::player`, `system::player_air`, `system::parachute`); el rótulo MISSION START con sus letras que caen y salen volando (`system::banner`), la caída libre al aparecer con el indicador 1UP (`system::player_air`, `system::icons`) y el andar disparando (`system::player_ground`); los soldados rebeldes aparecen con sus rasgos, la agresividad según la dificultad y la primera percepción de los jugadores, y andan, corren, frenan, huyen, saltan, se rinden y atacan cuerpo a cuerpo (`system::soldier`); mueren (`system::human_death`: caída, voltereta, salir despedido, trozos, quemarse y sangre) y se burlan del jugador (`system::soldier`); el prisionero atado (`system::items`) y el contacto entre cajas con los jugadores (`system::contact`); los carteles del fuerte que estampan su silueta en el mapa de colisión (`system::props`, `system::scroll`) y el soldado de guardia que embiste y el escaramuzador (`system::charger`); la torreta que apunta de la misión 1 con su cañón, proyectiles y casquillos (`system::aim_turret`) y la granada que lanzan los soldados (`system::enemy_grenade`) y los objetos arrojados con su sombra (`system::thrown`); la farola con su chispa, la choza y los tejados que se rompen (`system::scenery`) el ocupante de la choza con su puerta (`system::hut`), el barril flotante del río con su mina acuática (`system::barrel`) los humos y salpicaduras (`system::smoke`) y la torre de vigía, el búnker y el puente que se rompen por piezas (`system::tower`) y el soldado de la torre y de los pilares con su mortero (`system::tower_soldier`, con el movimiento «C» `motion::move_c`) y la muerte del jugador (`system::player_death`: los estados por tipo de muerte, la caída al vacío, la desaparición, la salpicadura y la sangre) y su caída al entrar en escena (`system::player_air::spawn_fall`); se para en el frame 4147 en `ContinueBig_Init_0467f6`; con `--hands-off` recorre el tutorial del Slug entero (destrucción, ataque suicida A + B, polvaredas) y llega a la misión 1, donde aparecen los soldados (`system::soldier`) y sueltan objetos (`system::items`) sin pararse en 9000 frames (antes: `Soldier_WalkStart_057558`, `Soldier_SpawnVariants_057226+0x17C`, `$02E9C8`, `$02E3B0`, `$035BF8`, `$077A96`, `$03093A`, `$03C776`, `$0342C4`, `$03D616`, `$034704`, `$02DD20`, `$033EDE`, `$036D64`, `$08C678`, `$0394A8`, `$0300BA`, `$02B004`, `Slug_DropVariant3_02af64`, `Slug_SpawnDrop_02ae50`, `Platform_ClearListByScroll_077092`, `Icon_Anchor_Init_08c5b2`, `Anim_State_F1_08C008`, `Sprite_Dispatch_05A9E2`, `VBlank_RenderFrame_002098`, `$091330`, `TaskSlots_BootInstall`, `SchedulerBootstrap_Boot`, `AttractHandler_00109C`) |
> | Hoja de ruta (`mslug roadmap`) | rutinas en orden de primera ejecución, portadas o no |
> | Assets (`scripts/extract_assets.sh`) | ✅ idénticos a la rama de la decompilación |

## Resumen

| | |
|---|---|
| Fuente del código | **la decompilación** (`tools/recomp/decomp_code.bin`, exportado con `tools/export_code_map.py` del repo de la decompilación): 101 142 instrucciones, 0 discrepancias de longitud con la ROM |
| Programa 68000 traducido a Rust (`crates/gen/NN`, generado) | **115 189 instrucciones**: P-ROM 101 168 + BIOS 14 021 |
| Trazas / ejecuciones usadas para generar | **ninguna** (`m68k_recomp.py --static`) |
| Instrucciones no traducibles | 0 en la P-ROM (1 en la BIOS, nunca alcanzada) |
| `mslug lockstep --frames 6000` | ✅ 99 245 067 instrucciones idénticas al intérprete (registros, SR, PC, ciclos tras **cada** instrucción); 100 % código Rust |
| `mslug play --fuzz 1..3` (25 000 frames, entradas aleatorias, M1–M3) | ✅ 0 instrucciones interpretadas, 0 direcciones sin traducir |
| Frontend SDL2 (`crates/mslug-sdl`) | ✅ ventana, sonido, P1/P2, capturas |
| CI (`.github/workflows/port.yml`, runner local) | build + lockstep + play + SDL headless; binarios, logs y capturas en Releases `port-*` |

## Recompilación estática "pura" desde la decompilación

Pregunta: ¿se puede generar el port leyendo sólo la decompilación, sin
ejecutar el juego para descubrir qué bytes son código?

Investigación (oct. 2026):

1. **El mapa código/datos ya está en la decompilación.** Cada fuente del
   registro se ensambla con listado (`-Wa,-aln`); una línea que emite bytes
   es instrucción si su fuente es un mnemónico y dato si es `.dc/.byte/.fill/
   .rept/...`. Con la dirección de cada sección (`tools/registry.py`) sale la
   dirección exacta de las 101 142 instrucciones del juego. `--check`
   decodifica cada una con capstone en la ROM: 0 diferencias.
2. **Saltos indirectos.** El 68000 del juego usa tablas de punteros
   (`jmp (a0)`, máquinas de estado `(a6)`, tablas `$B92`...). La
   decompilación ya decodificó como código los destinos de esas tablas, así
   que no hace falta resolverlos: se traducen **todas** las instrucciones de
   la decompilación, alcanzables o no.
3. **Comparación con el método por trazas.** Las 1 670 direcciones que las
   partidas habían encontrado "a mano" (`seeds.txt`) están todas en el mapa
   de la decompilación. La única diferencia real fue **un error de la
   decompilación**: 38 bytes en `$4B052` (`HumanDeath`, estado alcanzado sólo
   por un puntero de tabla) estaban como `.dc.w`. Se corrigió **en la
   decompilación** (sigue IDÉNTICA) y el port lo hereda. Una búsqueda de
   punteros a zonas de datos que decodifiquen como rutina no encontró más
   casos.
4. **Instrucciones solapadas** (22 destinos de salto que caen dentro de un
   `.short` de opcode literal, p. ej. `bne.w` codificado a mano en
   `$024E60`) se siguen igualmente por descenso recursivo desde el código
   decompilado.
5. **BIOS.** No está decompilada. Sus entradas se sacan también sin ejecutar:
   tabla de llamadas `$C00402` (SDK), vectores, todo puntero alineado a la
   BIOS en BIOS/P-ROM, código tras tablas de saltos en línea y barrido tras
   cada `rts/jmp/bra`. Cubre todo lo que las partidas ejecutaron.

Conclusión: **sí**. El port se genera de forma puramente estática a partir de
la decompilación (`m68k_recomp.py --static`); ejecutar el juego queda sólo
como **verificación** (lockstep, fuzz, CI), no como fuente. El método con
trazas (`--registry-seeds` + `seeds.txt`) se mantiene como alternativa.

## Cómo está hecho

1. Decompilación: `python3 tools/export_code_map.py --check` →
   `build/code_map/code.bin` (se copia a `tools/recomp/decomp_code.bin`).
2. `mslug dump-roms` vuelca la P-ROM tal como la ve la CPU y la BIOS (es la
   ROM; no se ejecuta nada).
3. `tools/recomp/m68k_recomp.py --static` traduce **cada instrucción** a
   una función Rust especializada: modos de direccionamiento, inmediatos,
   destinos de salto y ciclos se resuelven en la generación. La semántica es
   la del núcleo 68000 de referencia (`neogeo-hw/src/cpu/m68k/exec.rs`):
   flags X N Z V C, orden de accesos al bus, ciclos.
4. `mslug_code::Program` despacha por PC a esas funciones. El hardware
   (vídeo, Z80/YM2610, IRQ) avanza con `System::post_cpu`.
5. Verificación: `mslug lockstep` (intérprete y programa traducido a la vez,
   para en la primera instrucción que difiera) y `mslug play --fuzz N`.

## Camino a "0 emulación"

El objetivo final es que no quede ningún intérprete de CPU:

- [x] Programa del juego y BIOS traducidos a Rust (sin decodificar opcodes
      en tiempo de ejecución).
- [x] Verificación instrucción a instrucción (lockstep).
- [x] Código sacado de la decompilación, sin trazas (`--static`).
- [x] 0 direcciones sin traducir en lockstep y partidas fuzz (M1–M3).
- [ ] **Quitar el intérprete** de `Program::step` (fallo duro en un miss).
- [ ] Juntar instrucciones en bloques básicos / funciones (Rust por función
      de la decompilación, no por instrucción) y nombrarlas con los símbolos
      de `tools/recomp/symbols.tsv`.
- [ ] Sustituir funciones generadas por reescrituras idiomáticas
      (`crates/mslug/src/game`), verificadas con `mslug verify`.
- [ ] Hardware: el vídeo (LSPC → sprites/fix), el sonido (Z80 + YM2610) y la
      BIOS son ahora el "sistema" del port. El programa Z80 del driver de
      sonido se traducirá con el mismo método (recompilador Z80) para que
      tampoco quede intérprete del Z80.

## Funciones reescritas a mano

| Dirección | Rutina | Módulo |
|---|---|---|
| $001E56 | Store106EDA | kernel |
| $05E9E4 | RNG_LFSRStep (generador aleatorio) | kernel |
| $043EDA | Buffer_ClearBlock1024L | kernel |
| $044048 | Coord_ApplyCameraDeltaToSelf | entity |
| $04405E | Coord_ApplyCameraDeltaToA0Mark | entity |
| $04407A | Coord_ApplyCameraTerciaryToSelf | entity |
| $044090 | Coord_ApplyCameraDeltaToGlobals | entity |
| $0440AA | Coord_ApplyCameraDeltaToD1D2 | entity |
| $0440BC | Coord_LocalToScreenSecondary | entity |
| $0440D0 | Coord_ScreenToLocalSecondary | entity |
| $065C64 | Hostage_SinCos | entity |
| $02A2AC | Slug_GetGauge | entity |
| $05180C | Clear8Bytes | entity |
| $0516BA | Players_SnapshotMods | entity |
| $09773C | HiScore_LoadDefaults | hiscore |
