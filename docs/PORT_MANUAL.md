# Port escrito a mano (sin recompilación) — investigación y plan

Objetivo: un Metal Slug en **Rust legible**, escrito leyendo y entendiendo la
decompilación, con funciones con nombre organizadas por sistema
(`player/weapons.rs`, `enemies/soldier.rs`, ...), **sin direcciones del juego
original** en la lógica y sin recompilación instrucción a instrucción.

## 1. Qué tiene la decompilación que lo hace posible (medido, oct. 2026)

| Medida | Valor |
|---|---|
| Rutinas de código en el registro | 7 887 |
| ... con nombre semántico (no `Fun_xxxx`/thunk) | 7 171 (**90 %**, 92 % de los bytes) |
| Subsistemas al 100 % nombrados | player, props, pow, mission, slug, entity, video, kernel, frontend, input, boot |
| Subsistema menos nombrado | `enemies` 94 %, `thunks` 63 % (trampolines `jsr/jmp abs`, desaparecen en Rust) |
| Instrucciones de la P-ROM | 100 772 (≈ 440 KB de código) |
| Datos (tablas, sprites, scripts, niveles) | ≈ 1,39 MB |
| Accesos a campos de entidad `d(a6)` | 19 600 usos de **110** offsets distintos |
| Direcciones absolutas de RAM | 2 388 usos de **277** variables globales |
| Accesos directos a hardware | 303 (casi todo VRAM del LSPC `$3C0000`) |
| Llamadas a la BIOS | 36 (10 rutinas) |
| Saltos indirectos `jmp/jsr (An)` | 68 (máquinas de estado por puntero en `(a6)`) |
| Llamadas `jsr/bsr` | 17 821; sólo ≈ 14 % usan registros al volver (d0/a0 sobre todo) |

Consecuencias:

1. **Las 277 globales y los 110 campos de entidad son finitos.** Se pueden
   convertir en `struct GameState { ... }` y `struct Entity { ... }` con
   nombres: es lo que elimina las "direcciones raras". Los nombres ya están
   en los comentarios de la decompilación (p. ej. `+$71` arma, `+$80` bombas,
   `+$82` munición, `+$87` invulnerabilidad; `$106ECE` escena, `$1081BF`
   créditos...).
2. **El 68000 se usa con una "ABI" casi convencional**: el 86 % de las
   llamadas no reutiliza registros al volver, el resto devuelve en d0/a0 o en
   flags (C = éxito). Eso permite firmas Rust normales (`fn f(..) -> T`,
   `-> bool`, `-> Option<..>`) en vez de "registros globales".
3. **Las máquinas de estado** (`move.l #handler,(a6)` + `jmp (a0)`) son el
   patrón principal (tareas, enemigos, jugador con 68 estados). En Rust:
   `enum State` + `match`, o `fn` pointers con nombre.
4. **Los datos se quedan como datos**: tablas de sprites, scripts de misión,
   niveles... se exportan desde la decompilación (`.dc.w` ya etiquetados) a
   ficheros/`const` con nombre. No hace falta "entenderlos" para portar el
   código que los lee, sólo su formato (que la decompilación documenta en la
   sección D) de cada fichero).

## 2. Qué lo hace difícil (y cómo se resuelve)

| Problema | Solución |
|---|---|
| Comportamiento idéntico sin ejecutar instrucciones | **Verificación diferencial**: cada función Rust se ejecuta en el sitio de su rutina original durante una partida y se compara la RAM resultante (ya existe `mslug verify`). Las funciones legibles NO simulan flags ni ciclos; sólo efectos visibles (RAM/VRAM, valor devuelto). |
| Ciclos (el 68000 tarda lo que tarda) | En un port legible no importan dentro del frame: el juego hace una pasada de lógica por VBlank. Sólo hay que conservar el orden de los efectos y el "un frame = una pasada". |
| Hardware Neo Geo (sprites, fix, paleta, sonido) | Capa `platform/` con API de alto nivel (`draw_sprite`, `fix_print`, `play_sound(id)`), implementada primero sobre el hardware emulado existente (para verificar) y después sobre SDL2 directamente (port final sin emulación). |
| BIOS | 10 rutinas usadas (input, créditos, memory card, FIX_CLEAR...). Se reescriben en `platform/bios.rs`. |
| Sonido (Z80 + YM2610) | El juego sólo envía **códigos de sonido** (`$2352` / `$10FD` ...). El driver Z80 está decompilado aparte (`mrom/`); a medio plazo se sustituye por un reproductor propio o se mantiene como único componente emulado hasta reescribirlo. |
| Código "raro" (opcodes literales, saltos al medio de otra función, colas compartidas) | La decompilación ya lo señala (`.short` + comentario). En Rust se escribe como funciones normales con el flujo real. |
| Volumen: ≈ 7 900 rutinas | Por sistema, de abajo arriba. Muchas son tiny (getters, thunks) y se funden. El objetivo es un nº de funciones Rust bastante menor. |

## 3. Arquitectura propuesta

```
crates/game/                 (crate `mslug-game`: el juego, Rust legible)
  src/lib.rs
  src/state.rs               GameState: globales con nombre (escena, créditos, ...)
  src/entity.rs              Entity: campos con nombre (pos, vel, estado, ...)
  src/mem.rs                 trait Memory (lectura/escritura del estado) — puente
  src/player/
      weapons.rs             armas, munición, bombas
      invuln.rs              invulnerabilidad y parpadeo
      ...                    estados, input, spawn, muerte
  src/enemies/ soldier.rs, tank.rs, bosses/...
  src/items/ pow/ slug/ mission/ hud/ frontend/ kernel/ ...
  src/data/                  tablas exportadas de la decompilación (con nombre)
  src/platform/              vídeo, sonido, input, BIOS (API de alto nivel)
crates/mslug/src/manual.rs   adaptador de verificación: llama a la función de
                             `mslug-game` en el sitio de la rutina original y
                             compara con la ROM (único sitio con direcciones)
```

Reglas del crate `mslug-game`:

* **Sin direcciones de la ROM** en el código del juego: nada de `0x032B58`,
  ni nombres con sufijo de dirección. La correspondencia nombre ↔ rutina
  original vive sólo en el adaptador de verificación (`manual.rs`) y en un
  comentario `/// Original: Player_SetWeaponAndAmmo` para trazabilidad.
* **Sin registros del 68000**: argumentos y valores de retorno Rust.
* **Estado con nombre**: `player.ammo`, `player.bombs`, `state.credits`.
  Mientras convivan con la ROM, `GameState`/`Entity` son vistas sobre la RAM
  (trait `Memory`) cuyos offsets están **en un único fichero** (`layout.rs`),
  para poder verificar. Cuando todo esté portado, `layout.rs` se sustituye
  por structs nativos y desaparece el último número mágico.
* Docs en español, nombres en inglés.

## 4. Estrategia de migración

1. **Piloto** (hecho en este PR): `player/weapons.rs` + `player/invuln.rs`
   (munición, bombas, invulnerabilidad: 7 rutinas originales) verificados
   contra la ROM con `mslug verify-manual`.
2. Por sistema, de las hojas hacia arriba: utilidades (`math`, `rng`,
   `bcd`), entidad (física, colisión), jugador, items, enemigos, misión,
   HUD, frontend, kernel/tareas.
3. Cuando un sistema entero esté portado, su punto de entrada (p. ej. el
   update del jugador) se llama desde Rust y la ROM deja de ejecutarse ahí.
4. Al final: el bucle principal es Rust, `platform/` dibuja con SDL2, y la
   ROM sólo aporta **datos** (gráficos, sonido, niveles).

Estimación realista: es un proyecto largo (miles de rutinas), pero cada
paso es pequeño, verificable y deja el juego funcionando.

## 5. Estado del port manual

`mslug verify-manual --frames 15000 --fuzz 1..3` y la partida automática de
8000 frames: **0 fallos** (≈ 57 000–59 000 llamadas comprobadas por partida).
Las rutinas muy llamadas se muestrean: se comprueban todas las 1000
primeras llamadas y luego una de cada `n/1000`.

Sistemas portados (60 rutinas originales):

| Módulo | Contenido | Rutinas originales |
|---|---|---|
| `input` | lectura de mandos: botones mantenidos/recién pulsados del mando que controla una entidad, pausa de entrada al arrancar escena, mandos sintéticos del piloto automático, dirección horizontal, configuración de salto | 35 `InputEvtThunk_*`, `InputMask_CheckChannelAvail`, `InputMask_ReadCtxSwitchPlayer`, `InputMask_TestChannelBit`, `InputCtx_DemoOverride`, `InputLayout_ReadField2` |
| `player::controls` | delante/detrás según orientación, apuntado en el aire y en el suelo, acción de disparo según apuntado, inicio de salto, límites de disparos y granadas en pantalla | `Input_ForwardByFacing`, `Input_BackwardByFacing`, `Input_DownPressed`, `Input_JumpOrFire`, `Input_FireByMode`, `Input_JumpByMode`, `Player_ReadDirNibble`, `Player_ReadFireNibble`, `Player_DirNibbleToAction`, `Player_ConsumeFlag81`, `Player_ConsumeFireFlag84`, `Player_IncCounterAt81`, `Player_IncCounterAt84` |
| `player::voice` | voz del arma nueva, sonido de vida extra | `Player_PlayWeaponMusicIfFlag`, `Player_PlayLifeMusic` |
| `player::weapons` | munición, bombas, recogidas | `Player_SetWeaponAndAmmo`, `Player_RefillAmmoClamp999`, `Item_GiveAmmo_ToPlayer`, `Item_GiveBombs_ToPlayer`, `Player_GetAmmoOrFFFF`, `Player_GetBombs`, `Player_DecBombs` |
| `player::invuln` | parpadeo de invulnerabilidad | `Player_StartInvuln5`, `Player_InvulnBlinkStep` |

Sin cobertura en las partidas de prueba (sólo tests unitarios):
`Player_RefillAmmoClamp999`, `Item_GiveAmmo_ToPlayer`, `Player_DecBombs`,
`Player_PlayLifeMusic`, `Input_BackwardByFacing`, `Input_JumpOrFire` /
`Input_JumpByMode` (sólo con la configuración "saltar con arriba"), los
thunks de mando fijo y varias combinaciones de botones.

Lecciones:

* La verificación diferencial sólo prueba los caminos que la partida
  recorre. Una primera versión con la tabla de munición mal leída pasó
  porque ninguna llamada usaba la rama por defecto.
* El fuzz encontró un **comportamiento raro del original** que la primera
  lectura había "arreglado" sin querer: con la misma arma,
  `Player_SetAmmoForWeapon` suma la munición por defecto sólo si la cantidad
  pedida es **≠ 0** (y no suma nada si es 0): la condición está invertida
  respecto al caso de arma distinta. El port reproduce el original y lo
  documenta en el código.
* Las llamadas a rutinas aún no portadas se comparan también por argumento
  (p. ej. el código de sonido pedido).

Dudas abiertas: el orden de armas 2/3 (Shotgun / Rocket) del `enum Weapon`
viene de observación, no del código.

## 6. Relación con el recompilador

El recompilador (`crates/gen`, `mslug-code`) sigue siendo útil como **red de
seguridad y referencia** durante la migración (y para jugar ya), pero no es
el port final. El port final es `mslug-game`.
