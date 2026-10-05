# Metal Slug 1 — Cobertura real de la ROM

**Ultimo update:** 2026-10-05  (Wave VVV cerrada)

Este documento complementa `docs/PROGRESO.md` con el analisis **real** de
cobertura de codigo, no la metrica bruta del matcher que compara contra los
2 MiB de la P-ROM (que incluye ~1.5 MiB de datos que **no** son "codigo a
decompilar").

Se regenera con `python3 tools/measure_coverage.py --zones` (la tabla por
zonas sale del mapa curado `ZONES` del propio script; actualizar ese mapa
cada vez que una wave descubra una frontera codigo/datos nueva).

---

## Los tres porcentajes que hay que distinguir

| Metrica | Cifra (post-VVV) | Que mide realmente |
|---|---:|---|
| **`ROM total`** (`match_batch.py`) | **11.57 %**  (242,582 / 2,097,152 B) | Bytes registrados vs P-ROM completa. Es la metrica del matcher pero es enganosa: incluye 1.5 MiB de datos/graficos/padding. |
| **`Codigo real`** (mapa curado) | **39.3 %**  (198,774 / 505,608 B) | Bytes registrados dentro de las zonas CODE del mapa curado vs total de esas zonas. **Es la metrica util de progreso.** |
| **`Datos registrados`** | 43,750 B | Tablas transcritas byte a byte porque el codigo las referencia (`--data`, streams de mision, indice `$E8000`...). No cuentan como "codigo". |

> La heuristica antigua por bloques de 4 KiB (entropia + densidad de
> opcodes) clasifica como `CODE?` el 83 % de la ROM porque los datos densos
> (pares `{id,tile}`, punteros, animaciones) decodifican como instrucciones
> validas. Se mantiene en el script solo como orientacion; la referencia es
> el mapa curado de abajo.

---

## Mapa curado de la P-ROM (archivo `201-p1` byte-swapped, 2 MiB)

Fronteras obtenidas con sondeos de densidad `rts`/`jsr.l`/`lea (pc)` por
bloques de 256..4096 B, hexdumps manuales y las tablas identificadas en las
waves. La segunda mitad del archivo (`$100000..$1FFFFF`) se mapea en CPU en
`$200000..$2FFFFF` (los punteros `$28Dxxx`/`$29Cxxx` del codigo apuntan ahi).

| Zona | Rango | Tipo | Total | Cubierto | % zona |
|---|---|---|---:|---:|---:|
| Vectores 68000 + cabecera Neo-Geo | `$000000..$000400` | SYSTEM | 1,024 B | 58 B | 5.7 % |
| BIOS entries, IRQ, scheduler, bootstrap, task runtime | `$000400..$002F30` | CODE | 11,056 B | 7,106 B | 64.3 % |
| Tablas de sprites/slots (pares {id,tile}, LUTs 16x16) | `$002F30..$0133B0` | DATA | 66,688 B | 0 B | 0.0 % |
| Runtime: entidades, spawn, scratch, texto PAUSE | `$0133B0..$013D6A` | CODE | 2,490 B | 408 B | 16.4 % |
| Relleno $00 + tablas escasas | `$013D6A..$024E10` | DATA | 69,798 B | 0 B | 0.0 % |
| Core: player, armas, fisica, probes, camara, scene VM | `$024E10..$05E000` | CODE | 233,968 B | 83,144 B | 35.5 % |
| Runtime tardio: input, debug, blits fix, VRAM, RNG | `$05E000..$083000` | CODE | 151,552 B | 25,222 B | 16.6 % |
| Enemigos, jefes, escenas, items, hiscore, mobs | `$083000..$09C608` | CODE | 103,944 B | 80,296 B | 77.2 % |
| Datos: animaciones, paletas, listas de spawn | `$09C608..$0E8000` | DATA | 309,752 B | 0 B | 0.0 % |
| Indice de templates $E8000 + streams de mision | `$0E8000..$0F2FFC` | DATA-REG | 45,052 B | 43,736 B | 97.1 % |
| Datos graficos / mapas / scripts de nivel | `$0F2FFC..$18D152` | DATA | 631,126 B | 0 B | 0.0 % |
| Granadas del jugador (banco alto, CPU $28Dxxx) | `$18D152..$18DB78` | CODE | 2,598 B | 2,598 B | 100.0 % |
| Datos de animacion + 2 islas C ($19C95A/$19CB64) | `$18DB78..$1F8000` | DATA | 435,336 B | 14 B | 0.0 % |
| Relleno $00 final | `$1F8000..$200000` | ZERO | 32,768 B | 0 B | 0.0 % |

| Tipo | Total | Cubierto | % |
|---|---:|---:|---:|
| CODE | 505,608 B | 178,066 B | 35.2 % |
| DATA-REG | 45,052 B | 43,736 B | 97.1 % |
| DATA | 1,512,700 B | 14 B | 0.0 % |
| SYSTEM | 1,024 B | 58 B | 5.7 % |
| ZERO | 32,768 B | 0 B | 0.0 % |

Huecos pendientes en zonas CODE: 1381 huecos, 306,834 B

### Notas por zona

- **`$000400..$002F30` (CODE)**: vectores de BIOS, IRQ/VBlank, scheduler
  threaded (`$0518`/`$0FC6`/`$0FE0`), super-tabla de arranque `$000B92`
  (datos-en-.text, 766 B), instaladores de slots `$000A7C`, runtime de tareas
  (`$4AE` alloc, `$518`, `$5B6`, `$6FE`), input `$2000..$2C00` y los selectores
  de banco de sprites `Sprite_SetupSlotFromTableA/B` (`$2C26/$2C30`). Los
  ~4 KB pendientes son el tramo `$001354..$001744` (attract), `$1EF6..$20DA`,
  `$29F2..$2C26` y `$2C66..$2F30`.
- **`$002F30..$0133B0` (DATA)**: 66 KB de tablas de sprites: pares word
  `{id, tile}` secuenciales (`00 00 10 00 | 00 01 10 01 ...`) seguidos de LUTs
  triangulares 16x16 (`$0130xx..$0133AA`, interpolacion `i*j/15`). Sin un solo
  `rts`. No se decompila.
- **`$0133B0..$013D6A` (CODE)**: `Entity_FlushSlotHistory_013600`,
  `Scratch_Alloc_01390E`, `Spawn_TypeA/B`, `SpriteTrapGuard`, los helpers CCR,
  el seno `$13C0E` y el texto **PAUSE** (`Fix_DrawPause_013d46`, cadena ASCII
  en `$13D32`). Hueco principal `$0133AA..$013600` (~600 B: iterador de
  tablas con `movem`).
- **`$013D6A..$024E10` (DATA)**: relleno `$00` hasta `$01A000` y tablas
  escasas (`$024D00`: triples `{a,b,c,0}` crecientes). Sin codigo.
- **`$024E10..$05E000` (CODE, 234 KB)**: el nucleo del juego. Ya cubiertos:
  dispatcher Start `$24E38`, cluster probes `$27A92/$27C8C/$27CEE/$27D50`,
  fisica `$2783A`, sprite map `$28CD4`, dano `$2870A`, prioridades `$28134`,
  camara `$06896A`.., `PlayerRoute_PublishState_033522`, Wave TTT (`player_core_032axx.s`,
  `$032A02..$0342C4`: nucleo del jugador, tabla de 68 punteros `$3338A` ->
  `$376xx..$37B00`), Wave UUU (`player_states_0342xx.s`, `$0342C4..$036632`:
  Stand/Walk/Turn/Melee/Grenade/RideSlug), Wave VVV
  (`player_air_death_crouch_0366xx.s`, `$036632..$0388F0`: salto, knockback,
  7 handlers de muerte de la tabla de 68, agachado/gateo), Wave WWW
  (`player_arm_weapon_fx_0388xx.s`, `$0388F0..$03A60A`: acciones agachado,
  arma soltada, paracaidas, sensores de agachado, fx de muerte y el overlay
  de brazo/arma `PlayerArm_*` con 44 tablas de sprites), Wave XXX
  (`player_arm_air_death_crouch_03a6xx.s`, `$03A60A..$03C62A`: los 47
  handlers de brazo restantes + 105 tablas), Wave YYY
  (`player_fire_shells_03c6xx.s`, `$03C62A..$03DA98`: spawners de proyectil
  por arma `PlayerFire_*`, casquillos, brazo sobre el Slug), squads/charger `$040EF2..$0434C2`,
  `SceneLoader_Main $43568`, `SceneScriptVM $437DA`, `MissionDriver $4422A`,
  jefes `$044AFE`.., dispatcher multi-slot `$051914`. Pendientes grandes:
  `$02E000..$032A00` (player core), `$03C8D8..$03DA98`, `$0478FC..$048A3C`, `$04AC3A..$04BB8E`,
  `$0527BA..$0539E2`, `$053F96..$0550BE`, `$057D04..$059342`,
  `$05AA96..$05CA2A` (8 KB).
- **`$05E000..$083000` (CODE, 152 KB)**: input mask dispatchers
  `$5CDFC..$5D1D9`, debug hex `$5D6A0`, blits fix `$5DA56..$5DB1A`, copias
  `$5DD02`, suelo `$5DD56/$5DD5C`, RNG `$5E9B6`, `$5DCA4`, VRAM autoclear
  `$5A824`, proyectiles/efectos `$06xxxx..$07xxxx` (muchas islas pequenas ya
  en C). Pendiente la mayoria de `$060000..$083000`.
- **`$083000..$09C608` (CODE, 104 KB, 77 %)**: Waves GG..RRR: miniboss
  finale, rescates, carrier M4, fuerte escena 4, dirigible escena 5,
  cutscenes, grunts, bichos, Game Over/Continue, hiscore/memcard/mobs,
  items/score/cajas. Pendiente: `$0916C8..$0967B4` (SceneDescriptor[256] +
  scripts, DATOS) y `$096BBC..$097730` (listas de spawn, DATOS) — ambos
  dentro de esta zona pero clasificables como datos (~24 KB), por lo que el
  codigo real pendiente aqui es < 1 KB.
- **`$09C608..$0E8000` (DATA)**: animaciones (registros de 10 B
  `{dx,dy,flags,ptr}`), paletas, listas. Un unico falso `rts` en 310 KB.
- **`$0E8000..$0F2FFC` (DATA-REG)**: indice de templates `$E8000[idx]`
  (u32 x 329) + los 13 streams de bytecode de la Mission VM
  (`mission_streams_0e8524.s`, Wave AAA, 43,736 B transcritos).
- **`$0F2FFC..$18D152` (DATA)**: mapas de tiles, scripts de nivel, listas de
  sprites por escena (`$1880xx`: registros `{flags, ptr $2436xx, $FFFF}`).
  2,162 punteros distintos desde la ROM apuntan aqui; cero `rts` reales.
- **`$18D152..$18DB78` (CODE, 100 %)**: Wave SSS — granadas del jugador
  (`Grenade_*`, `player_grenade_18d1xx.s`), unico codigo del banco alto.
  Despachado desde `$033346..$033358` via `jmp $28Dxxx.l`.
- **`$18DB78..$1F8000` (DATA)**: mas animaciones (`$19C8xx`: registros con
  punteros `$2330xx/$23DBxx`, terminador `$1600`) y 2 islas C registradas
  (`JsrAbsThunk_19c95a`, `SetTaskW_19cb64` — colas `jsr X.l; rts` dentro de
  tablas; revisar si son falsos positivos como los de `$18D56C..`).
- **`$1F8000..$200000` (ZERO)**: relleno `$00`.

---

## Que "partes del juego" estan decompiladas

### Sistemas completos o casi

- **Arranque BIOS + scheduler** (`SchedulerBootstrap_Boot_000E8E`, super-tabla
  `$000B92`, bucle `$0FC6/$0FE0`, slots `$1008A0`).
- **Camara** (Waves HH/JJ/KK), **colision/probes** (`$27A92..$27D50`),
  **VRAM/fix layer** (`$5A824`, blits `$5DA56..`, texto PAUSE).
- **Scene VM + Mission VM** (`SceneLoader_Main`, `SceneScriptVM`,
  `MissionDriver`, streams `$E8524..`).
- **Enemigos/jefes de las 6 misiones** (`$083000..$09C608` al 77 %).
- **Items, score popups, cajas, paracaidas** (Wave RRR), **hiscore, memcard,
  name entry, options, mobs** (Wave QQQ), **Game Over/Continue** (PPP).
- **Granadas del jugador** (Wave SSS); **estados en suelo del jugador**
  (Wave UUU: stand/walk/turn/melee/grenade/ride); **aire / muerte /
  agachado** (Wave VVV); **brazo/arma, paracaidas, arma soltada, fx de
  muerte** (Wave WWW); **handlers de brazo aire/muerte/agachado** (Wave XXX);
  **disparo por arma, casquillos** (Wave YYY).

### Cosas que faltan (por tamano de codigo pendiente)

1. **Nucleo del jugador y armas de fuego** — `$02E000..$032A00` (~15 KB):
   callbacks del player core, pistola/HMG/shotgun/bazooka, SV-001. La tabla de 68 estados `$3338A` (Wave TTT) y
   los targets `$376xx..$37B00` son la puerta de entrada.
2. **Proyectiles y efectos** —
   `$060000..$083000` (~100 KB, muchas islas C ya cerradas).
3. **Dispatchers grandes** — `$0478FC`, `$04AC3A`, `$0527BA`, `$053F96`,
   `$057D04`, `$05AA96` (4..8 KB cada uno).
4. **Attract/title residual** — `$001354..$001744`.
5. **Bridge de sonido M68K<->Z80** — `$236E`/`$2352` ya nombrados
   (`Entity_AllocSpriteSlot_00236E` es en realidad el emisor de snd id;
   `InputGuardCall219c` el de musica); falta el driver de `$300000`/`$320000`.

---

## Ranking de proxima prioridad

| # | Rango | Pendiente | Contexto |
|---:|---|---:|---|
| 1 | `$02E000..$032A00` | ~15 KB | Player core (publicadores `$2575C/$25766`, slots `$100440/$1004E0`, callbacks `Sub_000324BC..Sub_0003292C` usados por `Player_*`) |
| 2 | `$05AA96..$05CA2A` | 8 KB | Bloque contiguo mas grande sin tocar del runtime tardio |
| 3 | `$057D04..$059342` | 5.6 KB | Dispatcher grande del nucleo |
| 4 | `$0527BA..$0539E2` | 4.6 KB | Dispatcher del nucleo |

---

## Como regenerar este documento

```bash
pip install capstone          # solo para la heuristica por bloques
python3 tools/measure_coverage.py --zones   # tabla por zonas (mapa curado)
python3 tools/measure_coverage.py --blocks  # mapa heuristico 4 KiB (orientativo)
```

Al cerrar una wave: (1) si descubre una frontera codigo/datos nueva, editar
`ZONES` en `tools/measure_coverage.py`; (2) pegar la salida de `--zones` en
la seccion "Mapa curado"; (3) actualizar la fecha y las tres cifras de arriba.
