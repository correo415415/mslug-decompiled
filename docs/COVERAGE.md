# Metal Slug 1 — Cobertura real de la ROM

**Ultimo update:** 2026-10-15  (P-ROM 100 % + módulos de datos con cabecera A–F y tipos; M-ROM 100 %; extracción de assets)

Este documento complementa `docs/PROGRESO.md` con el analisis **real** de
cobertura de codigo, no la metrica bruta del matcher que compara contra los
2 MiB de la P-ROM (que incluye ~1.5 MiB de datos que **no** son "codigo a
decompilar").

Se regenera con `python3 tools/measure_coverage.py --zones` (la tabla por
zonas sale del mapa curado `ZONES` del propio script; actualizar ese mapa
cada vez que una wave descubra una frontera codigo/datos nueva).

---

## Los tres porcentajes que hay que distinguir

| Metrica | Cifra (post-VVVVV) | Que mide realmente |
|---|---:|---|
| **`ROM total`** (`match_batch.py`) | **100.0 %**  (2,097,152 / 2,097,152 B) | Bytes registrados vs P-ROM completa. Desde las waves de datos (KKKKK..VVVVV) la ROM entera se reconstruye byte a byte: `python3 tools/build_rom.py` → MD5 `816b3f74…`; `scripts/pack_prom.sh` → `201-p1.bin` MD5 `b6804bc6…`. |
| **`Codigo real`** (mapa curado) | **100.0 %**  (505,608 / 505,608 B) | Bytes registrados dentro de las zonas CODE del mapa curado vs total de esas zonas. **Es la metrica util de progreso.** |
| **`Datos registrados`** | 1,591,544 B (DATA-REG 45,052 + DATA 1,512,700 + SYSTEM 1,024 + ZERO 32,768) | Tablas transcritas byte a byte porque el codigo las referencia (`--data`, streams de mision, indice `$E8000`...). No cuentan como "codigo". |

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
| BIOS entries, IRQ, scheduler, bootstrap, task runtime | `$000400..$002F30` | CODE | 11,056 B | 11,056 B | 100.0 % |
| Tablas de sprites/slots (pares {id,tile}, LUTs 16x16) | `$002F30..$0133B0` | DATA | 66,688 B | 0 B | 0.0 % |
| Runtime: entidades, spawn, scratch, texto PAUSE | `$0133B0..$013D6A` | CODE | 2,490 B | 2,490 B | 100.0 % |
| Relleno $00 + tablas escasas | `$013D6A..$024E10` | DATA | 69,798 B | 0 B | 0.0 % |
| Core: player, armas, fisica, probes, camara, scene VM | `$024E10..$05E000` | CODE | 233,968 B | 233,968 B | 100.0 % |
| Runtime tardio: input, debug, blits fix, VRAM, RNG | `$05E000..$083000` | CODE | 151,552 B | 151,552 B | 100.0 % |
| Enemigos, jefes, escenas, items, hiscore, mobs | `$083000..$09C608` | CODE | 103,944 B | 103,944 B | 100.0 % |
| Datos: animaciones, paletas, listas de spawn | `$09C608..$0E8000` | DATA | 309,752 B | 0 B | 0.0 % |
| Indice de templates $E8000 + streams de mision | `$0E8000..$0F2FFC` | DATA-REG | 45,052 B | 43,736 B | 97.1 % |
| Datos graficos / mapas / scripts de nivel | `$0F2FFC..$18D152` | DATA | 631,126 B | 0 B | 0.0 % |
| Granadas del jugador (banco alto, CPU $28Dxxx) | `$18D152..$18DB78` | CODE | 2,598 B | 2,598 B | 100.0 % |
| Datos de animacion + 2 islas C ($19C95A/$19CB64) | `$18DB78..$1F8000` | DATA | 435,336 B | 14 B | 0.0 % |
| Relleno $00 final | `$1F8000..$200000` | ZERO | 32,768 B | 0 B | 0.0 % |

| Tipo | Total | Cubierto | % |
|---|---:|---:|---:|
| CODE | 505,608 B | 505,608 B | 100.0 % |
| DATA-REG | 45,052 B | 45,052 B | 100.0 % |
| DATA | 1,512,700 B | 1,512,700 B | 100.0 % |
| SYSTEM | 1,024 B | 1,024 B | 100.0 % |
| ZERO | 32,768 B | 32,768 B | 100.0 % |

Huecos pendientes en zonas CODE: 0 huecos, 0 B — **todas las zonas CODE cubiertas.**
Zonas DATA/SYSTEM/ZERO: **100 %** registradas como volcados estructurados
(`tools/gen_data_region.py` + `scripts/data_wave.sh`), waves KKKKK..VVVVV:

| Wave | Rango | Modulo | Entradas |
|---|---|---|---:|
| KKKKK | `$002F30..$0133B0` | `data/sprite_tables_002f30.s` | 428 |
| LLLLL | `$013D6A..` (huecos dispersos) | `data/sparse_tables_013d6a.s` | 322 |
| MMMMM | `$09C608..$0E8000` | `data/anim_pal_spawn_09c608.s` | 959 |
| NNNNN | `$0F2FFC..$118028` | `data/level_a_0f2ffc.s` | 4,499 |
| OOOOO | `$118028..$140000` | `data/level_b_118028.s` | 1,190 |
| PPPPP | `$140000..$16B035` | `data/level_c_140000.s` | 490 |
| QQQQQ | `$16B035..$18D152` | `data/level_d_16b035.s` | 327 |
| RRRRR | `$18DB78..$1B0000` | `data/anim_a_18db78.s` | 344 |
| SSSSS | `$1B0000..$1D0000` | `data/anim_b_1b0000.s` | 328 |
| TTTTT | `$1D0000..$1F8000` | `data/anim_c_1d0000.s` | 513 |
| UUUUU | `$000000..$000400` + `$0001CA` | `boot/vectors_and_neogeo_header_000000.s`, `entity/find_by_key4_0001ca.s` | 9 |
| VVVVV | `$0E8000` (1,316 B) + `$1F8000..$200000` | `data/template_index_0e8000.s`, `data/rom_tail_fill_1f8000.s` | 5 |

Convenciones de los volcados: `Zero_` (runs de `$00` ≥ 32 B, `.fill`),
`PtrTab_`/`.dc.l Simbolo` (longs que apuntan a entradas/simbolos conocidos),
`Str_` (ASCII), `Data_` (resto, `.dc.w`), `RomData_` cuando la direccion ROM
`$10xxxx` colisiona numericamente con un simbolo de RAM. Pendiente: cabeceras
A–F semanticas y renombres (`RgbLut_002f30`, `PalRamps_012f30`, bancos de
paleta `$14E00`/`$1CE00`, soft-DIP `$1781F4`...).

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
  camara `$06896A`.., `PlayerRoute_PublishState_033522`, Wave TTT (`player/core_032aa8.s`,
  `$032A02..$0342C4`: nucleo del jugador, tabla de 68 punteros `$3338A` ->
  `$376xx..$37B00`), Wave UUU (`player/states_0342c4.s`, `$0342C4..$036632`:
  Stand/Walk/Turn/Melee/Grenade/RideSlug), Wave VVV
  (`player/air_death_crouch_036632.s`, `$036632..$0388F0`: salto, knockback,
  7 handlers de muerte de la tabla de 68, agachado/gateo), Wave WWW
  (`player/arm_weapon_fx_0388f0.s`, `$0388F0..$03A60A`: acciones agachado,
  arma soltada, paracaidas, sensores de agachado, fx de muerte y el overlay
  de brazo/arma `PlayerArm_*` con 44 tablas de sprites), Wave XXX
  (`player/arm_air_death_crouch_03a60a.s`, `$03A60A..$03C62A`: los 47
  handlers de brazo restantes + 105 tablas), Wave YYY
  (`player/fire_shells_03c62a.s`, `$03C62A..$03DA98`: spawners de proyectil
  por arma `PlayerFire_*`, casquillos, brazo sobre el Slug), Wave ZZZ
  (`slug/vehicle_02dd20.s`, `$02DD20..$030602`: maquina de estados del
  SV-001 `Slug_*`, tabla de 80 punteros `$2E582`, dano/destruccion,
  `Chain3_*`), Wave AAAA (`player/tables_fx_030696.s`, `$030602..$032A02`:
  granada del player, `EnemyShot_*`, `VehicleLaunch_*`, `Fx_*`,
  `PlayerIcon_*`, tablas estaticas `Player_Hitbox*`/`Player_*Tbl`), Wave BBBB
  (`slug/helpers_0295a6.s`, `$0295A6..$02AE3E`: `Slug_Init/InitBoss`,
  hitboxes, 17 tablas de ataque, `Slug_StateByAnglePtrTbl`, sondas de
  terreno, fisica, input por layout, HP/gauge), Wave CCCC
  (`slug/states_02ae3e.s`, `$02AE3E..$02DD20`: spawn en paracaidas
  `Slug_SpawnDrop/DropVariant0..4`, `Slug_IdleFlat/IdleSlope`, `Slug_Fire*`,
  `Slug_Jump*`, `Slug_Fall*`, `Slug_Hit*`, `Slug_Death*`), squads/charger `$040EF2..$0434C2`,
  `SceneLoader_Main $43568`, `SceneScriptVM $437DA`, `MissionDriver $4422A`,
  jefes `$044AFE`.., dispatcher multi-slot `$051914`. Pendientes grandes:
  `$024E10..$05E000` (**100 %**; Wave HHHHH cerró `$051AA4..$0527AE` en
  `video/cellmap_fix_overlay_palfade_051aa4.s`: blits de mapa de celdas, overlay
  CREDITS/PAUSE, fundidos de paleta; Wave GGGGG cerró `$05CADE..$05E000` en
  `input/thunks_debug_atan_luts_05cae0.s`: thunks de input, HUD hex de
  debug, LUTs de atan, checks de pantalla; Wave FFFFF cerró `$055B96..$056ACC` en
  `enemies/projectiles_grenade_mortar_055b96.s`: granada enemiga, bola
  rebotante, obús de mortero y rodillo; Wave EEEEE cerró `$05934E..$05A9D6` en
  `mission/result_ending_gunner_walker_05934e.s`: pantalla de resultados, secuencia
  final, Gunner/Gunner2, Walker, LUT de fade; Wave DDDDD cerró `$046000..$048000` en
  `enemies/vehicles/enemy46_drops_banners_046260.s`: Enemy46, drops, banners CONTINUE/MISSION/
  TIME UP, typewriter y fuentes; Wave CCCCC cerró `$027400..$02A000` en
  `entity/collmap_hitbox_script_ops_027400.s`: colisión con mapa, hitboxes, 32 opcodes
  de script), `$000400..$002F30` (64 %), `$0133B0..$013D6A` (**100 %**, Wave IIIII: `video/palette_engine_sprite_grid_0133b0.s`). Wave ZZZZ
  (`enemies/bosses/boss2_crab_carrier_07a00a.s`, `$07A002..$083000`): rehén/captor de
  la tripulación, menú SOUND TEST (strings ASCII + driver $2152/$219C), jefe de
  misión 2 (tmpl 132: cuerpo, brazo pendular, torreta, proyectil, humo, 9+9
  patrones de piezas/hitbox), cangrejo mecánico (tmpl 138..140: pinzas, 3 patas,
  esquirlas, patrulla) y transporte blindado (tmpl 129..131: 3 cascos, cabina,
  trampilla, cañón, spawner de 3 ranuras, tropas, artillero, jinete). La zona
  `$05E000..$083000` queda al 100 %. Wave YYYY (`props/fx_explosions_platforms_crew_076000.s`,
  `$076000..$07A000`): hijos del prop guionizado, fragmentos, listas de
  plataformas, tablones de puente, FX de cañón, spawner genérico, 49 variantes
  de explosión + humo + debris, destructibles tmpl 23..26, PathScript VM (11 ops),
  autodemo del attract y tripulación de vehículos (tmpl 125..128).
  Wave XXXX (`enemies/bosses/m5tank_finalboss_scriptedprop_071ffc.s`,
  `$071FFC..$076000`): helpers del jefe final (chatarra, chispas, humo, partes,
  patrones de ataque), tanque pesado de la misión 5 con torreta/cohetes/casquillos,
  misil M5, intérprete MiniScript de 8 ops y prop destructible guionizado
  (3 plantillas `$E8000`). Wave WWWW (`enemies/bosses/gunship_m5boss_finalboss_06dfe8.s`,
  `$06DFE8..$071FFC`): chorros de fuego y helpers del Walker, aeronave con
  artillero (Gunship), jefe de la misión 5 (M5Boss: rotor, partes, bombas,
  misiles, soldados/granaderos hijos) y jefe final en dos mitades con cabeza y
  cañón (FinalBoss) (7 plantillas `$E8000`). Wave VVVV (`enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s`,
  `$06A000..$06DFE8`): soldado bazooka (3 variantes + aliado del escuadrón de
  rescate, arma hija con cohete), vehículo lanzacohetes sobre eslabones Chain3
  (29 tablas de poses por fase de rueda, jinete), enemigo Walker (9 plantillas),
  fragmentos y chorro de fuego (19 plantillas `$E8000`). Wave UUUU (`enemies/soldiers/barrel_paratrooper_shield_tank_066000.s`,
  `$066000..$06A000`): barril flotante / mina acuática, spawner de
  paracaidistas, soldado con escudo (2 variantes) + escudo, puerta y
  dirigible de escena 5, tanque enemigo (conductor, torreta, misil guiado)
  (7 plantillas `$E8000`). Wave TTTT (`enemies/soldiers/sniper_camper_mortar_062008.s`,
  `$062000..$066000`): cola de LateProp, artillero del coche-torreta,
  francotirador, soldado atrincherado (Camper), escombros de tienda, mortero,
  cañón, rehén/POW (contador `$10E276`), patrulla, prop de escena 3 y barril
  (16 plantillas `$E8000`). Wave SSSS (`props/late_props_turrets_05e000.s`,
  `$05E000..$062000`): helpers de runtime tardío (Atan2 `$5E018`, target de
  jugadores `$5E086`, RNG `$5E998/$5EA1C`, clasificación de golpes), debug de
  colisión, soldado de torre / ocupante de choza, rompibles, cartel,
  obstáculos, cajas, torreta apuntadora y props tardíos (42 plantillas `$E8000`).
  Wave RRRR (`pow/results_squadleader_03daa8.s`,
  `$03DA98..$040EF2`): pantalla de resultados de misión (columnas por
  jugador, roster de POWs, premio, banners), POW (atado/liberado/rescatado,
  crédito de rescate) y líder del escuadrón volador. Wave QQQQ (`enemies/bosses/allen_oneil_04fa58.s`,
  `$04FA50..$051914`): jefe Allen O'Neil (IA, saltos, cuchillo, MG, granada,
  bala), 19 spawners de piezas de barrera/caseta/fortaleza, blits de
  torre/búnker/nido, pegamento de memory card, nibbles. Wave PPPP (`props/fortress_04e580.s`, `$04E580..$04FA50`):
  props de la misión del fuerte (barrera, caseta/portón, fortaleza con cadena
  de spawns por scroll), tejado, sensor, caja, escombros rebotantes, carteles,
  barca, parpadeo del fix y FixTile. Wave OOOO (`enemies/vehicles/turret_car_props_04cbd4.s`,
  `$04CBD4..$04E580`): vehículo-torreta (base, cañón con histórico de
  ángulos, conductor, proyectil) + 3ª tanda de props de misión (farola,
  choza, torre, búnker, puente, nido, cobertizo, barrera). Wave NNNN (`enemies/vehicles/gun_platform_04bb9a.s`,
  `$04BB9A..$04CBD4`): emplazamiento de cañón enemigo (base con 4 variantes,
  escotilla, escudo, tiradores, cañón hijo, destrucción) + lector del stream
  de spawn. Wave MMMM (`props/mission_053f96.s`,
  `$053F96..$055258`): props destructibles de misión (2ª tanda: edificio,
  columna, muro del recinto, letrero de neón con fix-layer, puesto, frágiles,
  multietapa, bloqueador). Wave LLLL (`pow/hang_049430.s`, `$049430..$049FC4`):
  POW colgado de la cuerda (balanceo, forcejeo, cuerda hija, liberación),
  POW en caída con sombra, tablas de estados de muerte. Wave KKKK
  (`pow/human_death_049ff2.s`, `$049FF2..$04BB8E`): muerte de humanos
  (despacho por tipo de daño, caídas, lanzamiento, quemado, sangre, humo,
  3.8 KB de tablas de sprites). Wave JJJJ
  (`pow/helpers_048a44.s`, `$048A44..$049430`): helpers del POW (colas de
  estado, ítem lanzado, cuerda del atado, decisión por distancia). Wave IIII (`pow/prisoner_0478fc.s`,
  `$0478FC..$048A3C`): prisionero POW (atado, rescate, saludo, regalo de
  item, huida; variantes libre/atado). Wave HHHH (`props/helpers_0539f0.s`,
  `$0539F0..$053F96`): trampa de fuego, quemado, música/escombros por fase.
  Wave GGGG
  (`props/destructible_0527ba.s`, `$0527BA..$0539E2`): props destructibles
  del escenario (`Prop_Sign/Wall/Large/Explosive/HouseVariants/HutVariants/
  Tower/Gate` + `*Stage2/*Wreck`, `PropDebris_*`, `PropDrop_Item`,
  `FixBlink_Phase*`, `Prop_Indestructible`). Wave FFFF
  (`enemies/soldiers/helpers_056acc.s`, `$056ACC..$057D04`): helpers del soldado
  (`Soldier_Think`, `Soldier_PhysicsStep`, `Soldier_FindNearestPlayer`,
  `Soldier_PickGrabAnchor`, `Soldier_SpawnVariants`, `Soldier_Walk_Loop`,
  `Soldier_Leap`, `Soldier_Grab*`, tablas `Soldier_AttackTblMelee*`). Wave EEEE
  (`enemies/soldiers/states_057d04.s`, `$057D04..$059342`): estados del soldado
  rebelde (`Soldier_Grab*`, `Soldier_Run*`, `Soldier_Idle`, `Soldier_Flee*`,
  `Soldier_Surrender*`, `Soldier_ThrowGrenade*`, `Soldier_Taunt*`,
  `Soldier_Spawn*`).
- **`$05E000..$083000` (CODE, 152 KB)**: cola de sprites y render SCB1..4
  `$05AA96..$05CA2A` (Wave DDDD: `SpriteQueue_*`, `SCB1_WriteTileColumn*`,
  `Vblank_FlushSpriteQueue_05c9d6`), input mask dispatchers
  `$5CDFC..$5D1D9`, debug hex `$5D6A0`, blits fix `$5DA56..$5DB1A`, copias
  `$5DD02`, suelo `$5DD56/$5DD5C`, RNG `$5E9B6`, `$5DCA4`, VRAM autoclear
  `$5A824`, proyectiles/efectos `$06xxxx..$07xxxx` (muchas islas pequenas ya
  en C). Pendiente la mayoria de `$060000..$083000`.
- **`$083000..$09C608` (CODE, 104 KB, 100 %)**: Waves GG..RRR: miniboss
  finale, rescates, carrier M4, fuerte escena 4, dirigible escena 5,
  cutscenes, grunts, bichos, Game Over/Continue, hiscore/memcard/mobs,
  items/score/cajas. Wave AAAAA cerro la zona: `$0916C8..$0967B4`
  (`data/scene_scripts_0916c8.s`: SceneDescTable[16], 14 scripts de la VM de
  escena con callbacks 68000 embebidos, entidades-camara, tablas de trigger)
  y `$096BBC..$097730` (`frontend/attract_sprite_lists_096bbc.s`: 8 listas de sprites
  del attract, registros de 20 B) transcritos byte-exacto como datos (~24 KB).
- **`$09C608..$0E8000` (DATA)**: animaciones (registros de 10 B
  `{dx,dy,flags,ptr}`), paletas, listas. Un unico falso `rts` en 310 KB.
- **`$0E8000..$0F2FFC` (DATA-REG)**: indice de templates `$E8000[idx]`
  (u32 x 329) + los 13 streams de bytecode de la Mission VM
  (`data/mission_streams_0e8524.s`, Wave AAA, 43,736 B transcritos).
- **`$0F2FFC..$18D152` (DATA)**: mapas de tiles, scripts de nivel, listas de
  sprites por escena (`$1880xx`: registros `{flags, ptr $2436xx, $FFFF}`).
  2,162 punteros distintos desde la ROM apuntan aqui; cero `rts` reales.
- **`$18D152..$18DB78` (CODE, 100 %)**: Wave SSS — granadas del jugador
  (`Grenade_*`, `player/grenade_18d152.s`), unico codigo del banco alto.
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
- **Enemigos/jefes de las 6 misiones** (`$083000..$09C608` al 100 %).
- **Items, score popups, cajas, paracaidas** (Wave RRR), **hiscore, memcard,
  name entry, options, mobs** (Wave QQQ), **Game Over/Continue** (PPP).
- **Granadas del jugador** (Wave SSS); **estados en suelo del jugador**
  (Wave UUU: stand/walk/turn/melee/grenade/ride); **aire / muerte /
  agachado** (Wave VVV); **brazo/arma, paracaidas, arma soltada, fx de
  muerte** (Wave WWW); **handlers de brazo aire/muerte/agachado** (Wave XXX);
  **disparo por arma, casquillos** (Wave YYY); **vehiculo SV-001: estados,
  dano, destruccion, Chain3** (Wave ZZZ); **granada, proyectiles, caida del
  vehiculo, fx, iconos y tablas del player** (Wave AAAA); **helpers del
  SV-001: init, hitboxes, tablas de ataque, sondas, HP** (Wave BBBB);
  **estados del SV-001 1a mitad: paracaidas, pendiente, disparo, salto,
  caida, impacto, muerte** (Wave CCCC). El modulo del Slug
  `$0295A6..$030602` esta completo. **Cola de sprites y volcado a VRAM
  (heapsort, SCB1..SCB4, reflejo en agua)** (Wave DDDD). **Soldado rebelde:
  agarre, carrera, huida, rendicion, granadas, burla, spawn** (Wave EEEE) y **sus helpers: IA de
  decisión, física, agarre por anclas, spawn por variante** (Wave FFFF).
  El soldado rebelde `$056ACC..$059342` esta completo. **Props
  destructibles del escenario (carteles, muros, casas, chozas, torres,
  portones, escombros)** (Wave GGGG) **y sus helpers (trampa de fuego,
  quemado del jugador)** (Wave HHHH). Props `$0527BA..$053F96` completos. **Prisionero POW: atado, liberación, saludo,
  entrega de item, agradecimiento y huida** (Wave IIII). **Helpers del POW: cuerda, ítem,
  efectos y decisión por distancia** (Wave JJJJ). POW `$0478FC..$049430`
  completo. **Muerte de humanos: despacho por tipo de daño, caídas,
  desplome, lanzamiento, quemado, sangre y humo** (Wave KKKK). **POW colgado
  (balanceo/cuerda/liberación) y POW en caída** (Wave LLLL). Todo el
  bloque `$0478FC..$04BB8E` completo. **Props destructibles de misión, 2ª
  tanda (edificio, muro, neón con fix-layer, puesto, multietapa)** (Wave
  MMMM). Props `$0527BA..$055258` completos. **Emplazamiento de cañón
  enemigo (base, escotilla, escudo, tiradores, cañón, destrucción) y lector
  del stream de spawn** (Wave NNNN). **Vehículo-torreta con cañón guiado y
  conductor, y props de misión 3ª tanda** (Wave OOOO). **Props de la misión
  del fuerte (barrera/caseta/fortaleza encadenadas por scroll), tejado,
  sensor, caja, escombros rebotantes, carteles, barca** (Wave PPPP). **Jefe
  Allen O'Neil completo, spawners de piezas de props, memcard glue** (Wave QQQQ).
  **Pantalla de resultados, POW completo y líder de escuadrón** (Wave RRRR).
  **Helpers de runtime tardío (Atan2, target, RNG, golpes), debug de colisión,
  soldado de torre, rompibles, obstáculos, cajas, torreta apuntadora** (Wave SSSS).
  **Francotirador, soldado atrincherado, mortero, cañón, rehén, patrulla,
  barril y escombros de tienda** (Wave TTTT). **Barril flotante, paracaidistas,
  soldado con escudo, puerta/dirigible S5, tanque enemigo** (Wave UUUU).
  **Soldado bazooka (+aliado), vehículo lanzacohetes, Walker, fragmentos y
  chorro de fuego** (Wave VVVV). **Aeronave con artillero, jefe de misión 5 y
  jefe final** (Wave WWWW). **Helpers del jefe final, tanque pesado M5, misil M5,
  MiniScript y prop guionizado** (Wave XXXX). **Hijos del prop, fragmentos,
  plataformas, FX de cañón, explosiones/humo, destructibles, PathScript VM,
  autodemo y tripulación** (Wave YYYY). **Rehén/captor, SOUND TEST, jefe de
  misión 2, cangrejo mecánico y transporte blindado** (Wave ZZZZ).

### Cosas que faltan (por tamano de codigo pendiente)

1. **Runtime tardio `$05E000..$083000`** (input, debug, blits, VRAM).
2. **Proyectiles y efectos** —
   `$060000..$083000` (~100 KB, muchas islas C ya cerradas).
3. **Dispatchers grandes** — `$055BCE..$055FC2`, `$05627A..$056A06`,
   (4..8 KB cada uno).
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

## M-ROM (`201-m1.bin`, programa de sonido Z80, 128 KiB)

Segundo frente, abierto tras cerrar la P-ROM al 100 %. Toolchain y build
propios en `mrom/` (`z80dasm -b/-S` con `mrom/map/` → `mrom/asm/raw/*.s` →
`split_region.py` → `mrom/asm/{core,channel,seq,data}/` → `z80asm` →
`build/mrom/201-m1_rebuilt.bin`), ver `mrom/README.md`.

| Métrica | Valor |
|---|---|
| Tamaño | 131,072 B (MD5 `6449db0459bcf7584348792009e8f12c`) |
| Regiones | fija `0000-7FFF` (32 KiB) + bancos 2..7 de 16 KiB en la ventana `8000-BFFF` |
| **Reensamblado byte-idéntico** | **100 %** (7/7 regiones, `mrom/tools/build_mrom.py`) |
| Código alcanzable (`z80trace.py`, nivel de bytes) | **14,288 B (`0x37D0`)** en la región fija, 16 rangos; 0 B de código detectado en los bancos |
| Código/datos separados (región fija) | **sí**: 14,288 B código / 18,480 B datos (`mrom/map/fixed_0000_blocks.txt`, 74 bloques) |
| Rutinas con nombre semántico | 157 etiquetas de código/datos + 61 símbolos de RAM (`mrom/map/fixed_0000_syms.txt`); 503 saltos locales `<Rutina>_L<dir>`; 0 etiquetas anónimas |
| Ficheros por subsistema | 18 (`core/` 4, `channel/` 6, `seq/` 2, `data/` 6) vía `mrom/map/fixed_0000_split.txt` |
| Equates de hardware | `mrom/include/neogeo_z80.inc` (puertos, ventanas de banco, registros YM2610) |
| Bancos 2..7 | **decodificados** a fuentes simbólicas `mrom/sound/` (instrumentos, tablas, cabeceras, 31 650 ops de secuencia) |
| Nombres de sonido | 13 músicas y 27 voces con nombre real; 68 de los 117 SFX de la tabla A con usuario constante nombrados (`mrom/map/sound_ids.txt`) |
| Documentación | `mrom/docs/FORMATS.md` §1–§10: secuencias, cabeceras, instrumentos, S_TEMPO, tablas de página, estructuras de canal |

### Mapa de la región fija

| Z80 | Fichero | Contenido |
|---|---|---|
| `0000-007F` | `core/vectors_0000.s` | reset (`di ; out (18h),a ; jp Main_Init_1E00`), RST 8 = `YM_Write_RST8` (c=puerto, d=reg, e=valor), RST 30 limpia canal, `0038` → `IRQ_Timer_1E7A`, `0066` → `NMI_Command_1ED6` |
| `0080-00C7` | `data/bank_ptr_table_0080.s` | `BankPtrTable_0080`: 18 punteros lineales de 24 bits (lo, hi, banco, 0) a tablas en los bancos |
| `0100-1DFF` | `data/lookup_pages_0100.s` | 29 tablas alineadas a página (`ld h,PAGE ; ld l,idx`): frecuencias FM/SSG (`11xx`), volumen (`14xx`/`15xx`), envolventes (`18xx`-`1Bxx`), deltas ADPCM-B (`1Axx`) |
| `1E00-1FE4` | `core/main_1e00.s` | `Main_Init_1E00` (inicializa SSG/FM/ADPCM-A/B/secuenciador, IM 1, NMI on), `Main_Loop_1E3F` (por tick: comandos → SSG → FM → ADPCM-A → ADPCM-B → pistas), `IRQ_Timer_1E7A`, `NMI_Command_1ED6` (cmd 1 = `NMI_SoftReset_1F0A`, 3 = `NMI_HardReset_1F4B`, resto → anillo `FFF0`), `Bank_SelectInit_1F7D`, `Bank_ResolvePtr24_1FA6` |
| `1FE5-2075` | `core/command_2050.s` | `Ch_InitStruct_1FE5`, `Ch_ResetStruct_201B`, `Cmd_Dispatch_2050` (saca del anillo; `<10` → `CmdJumpTable_2076`, `≥20` → `Cmd_DispatchHi_2096`) |
| `2076-20D2` | `core/command_tables_2076.s` | `CmdJumpTable_2076` (16), `Cmd_DispatchHi_2096`, `CmdHiJumpTable_20B3` (16 por tipo `wCmdType_F81C`) |
| `20D3-219A` | `core/command_handlers_20d3.s` | `Cmd_PlayMusic_20D4` (tabla `0084` + id×4 → `Seq_StartMusic_4FA3`), `Cmd_PlaySfx_2115` (tablas `0088`+tipo×8 → `Seq_StartSfx_507B`), stop/fade/tempo |
| `219B-2514` | `channel/envelope_219b.s` | envolventes (`Env_*`), LFO, `Rng_Next_2300` (registro R), `Pitch_*`, `Vib_Update_248E`, estructuras de comando `24DE/24F7` |
| `2515-2B42` | `channel/ssg_2515.s` | SSG A/B/C (`wSSG_ChA_F81F`…) |
| `2B43-373F` | `channel/fm_2b43.s` | FM 1-4 (`wFM_Ch1_F907`…) |
| `3740-3FC9` | `channel/adpcma_3740.s` | ADPCM-A 1-6 (`wPCMA_Ch1_FAA5`…) |
| `3FCA-4A9F` | `channel/adpcmb_3fca.s` | ADPCM-B (`wPCMB_Ch_FB9B`) + código común |
| `4AA0-4B83` | `channel/chtype_4aa0.s` | 3 tablas de 4 punteros por tipo de canal, `Ch_KeyOn_4AE2`, `Ch_SetNote*_4B0B..4B4F` |
| `4B84-4C83` | `data/note_map_4b84.s` | `NoteMapTable_4B84` (256 B) |
| `4C84-5361` | `seq/sequencer_4c84.s` | pistas: 14 de música en `FBFC` + 8 de SFX en `FDF2` (0x23 B cada una); `Seq_StartMusic_4FA3`, `Seq_StartSfx_507B`, `Seq_FindFreeTrack_50DB`, `Seq_UpdateTracks_514F`, `Seq_TrackInit_5251` |
| `5362-57A0` | `seq/opcodes_5362.s` | `Seq_TrackFetch_5362`: byte `≥DA` para, `C0..D9` → `SeqOpJumpTable_53A3` (26 `jp`), `<C0` → `Seq_NoteEvent_56EA` (nota + len/vol/gate opcionales según bits) |
| `57A1-7FFF` | `data/instruments_57a1.s` (`incbin`) → fuente `mrom/sound/instruments.s` | `InstrPtrTable_57A1` (1024 `PTR24`, 8 grupos × 128) + 174 registros tipados: parches SSG (24 B) / FM (50 B) / ADPCM-A / ADPCM-B y tablas de samples (`mrom/docs/FORMATS.md` §5) |
| `08000-1FFFF` | `mrom/sound/*.s` | últimos parches, tablas/cabeceras de música y SFX, 31 650 ops de secuencia, restos no usados |

Opcodes de secuencia identificados (provisional): `C0/C1` instrumento,
`C2` salto, `C4` llamada a subsecuencia (pila de 3 niveles en `+1C`), `C5`
retorno, `C6` volumen, `C7` pan, `C8` parámetro, `CA` detune, `CB` espera,
`CC` longitud de gate, `CD` tempo, `CE` transposición, `CF/D0` bucle, `D1`
portamento, `D2` LFO, `D3-D6` nota directa por tipo, `D7/D8` pitch bend,
`D9` responde al 68000 por el puerto `0C`, `DA..FF` fin de pista.

## Módulos de datos de la P-ROM (`asm/data/*.s`)

Los 1,6 MiB de datos se clasifican byte a byte con
`assets/tools/data_census.py` según el decodificador que los reconoce. Las
entradas se renombraron por tipo (`ColMap_` 4970, `SprFrame_` 663, `BgMap_`
614, `AnimScr_` 590, `Pal_` 306, `Zero_` 518) y los 14 módulos tienen
cabecera A–F (`tools/gen_data_headers.py`).

```
módulo                             región               bytes   palette     frame      anim   tilemap collision    struct       lut      zero     other
-------------------------------------------------------------------------------------------------------------------------------------------------------
data/sprite_tables_002f30.s        $002F30-$0133B0    66688      0.0%      0.0%      0.0%      0.0%      0.0%      0.0%    100.0%      0.0%      0.0%
data/sparse_tables_013d6a.s        $013D6A-$024E10    69798     93.9%      0.0%      0.0%      0.0%      0.0%      0.0%      0.0%      6.1%      0.0%
data/scene_scripts_0916c8.s        $0916C8-$0967B4    20716      0.0%      0.0%      0.0%      0.0%      0.0%     99.9%      0.0%      0.0%      0.1%
data/anim_pal_spawn_09c608.s       $09C608-$0E8000   309752      0.0%      0.0%      0.0%     89.4%      0.0%      0.3%      0.0%     10.2%      0.0%
data/template_index_0e8000.s       $0E8000-$0E8524     1316      0.0%      0.0%      0.0%      0.0%      0.0%    100.0%      0.0%      0.0%      0.0%
data/mission_streams_0e8524.s      $0E8524-$0F2FFC    43736      0.0%      0.0%      0.0%      0.0%      0.0%    100.0%      0.0%      0.0%      0.0%
data/level_a_0f2ffc.s              $0F2FFC-$118028   151596      0.0%      0.0%      0.0%      0.0%     61.5%      0.2%      0.0%     37.9%      0.5%
data/level_b_118028.s              $118028-$140000   163800      0.0%     78.7%      0.0%      0.0%     19.4%      0.0%      0.0%      1.8%      0.0%
data/level_c_140000.s              $140000-$16B035   176181      0.0%    100.0%      0.0%      0.0%      0.0%      0.0%      0.0%      0.0%      0.0%
data/level_d_16b035.s              $16B035-$18D152   139549      0.0%     30.8%     33.1%      0.0%      0.0%     28.2%      0.0%      7.3%      0.6%
data/anim_a_18db78.s               $18DB78-$1B0000   140266      0.0%      0.0%     88.7%      0.0%      0.0%     10.9%      0.0%      0.0%      0.4%
data/anim_b_1b0000.s               $1B0000-$1D0000   131056      0.0%      0.0%     62.6%      0.0%      0.0%     36.0%      0.8%      0.0%      0.6%
data/anim_c_1d0000.s               $1D0000-$1F8000   163840      0.0%      0.0%     70.3%      0.0%      0.0%     28.6%      0.0%      0.0%      1.1%
data/rom_tail_fill_1f8000.s        $1F8000-$200000    32768      0.0%      0.0%      5.9%      0.0%      0.0%      0.9%      0.0%     93.1%      0.1%
TOTAL                                                 1611062      4.1%     21.6%     22.9%     17.2%      7.8%     13.4%      4.2%      8.5%      0.3%
```

| Tipo | Qué es | Herramienta / documento |
|---|---|---|
| paletas | `$14E00` × 1024 (y la ventana `$1CE00`), 16 colores × 4 B | `assets/tools/sprites.py` |
| frames + animación | frames multi-pieza y bytecode de animación (981 animaciones exportadas) | `assets/tools/sprites.py` |
| mapas de fondo | 52 capas de 13 escenas (`bind_path` de los scripts de escena) + panoramas con parallax | `assets/tools/levels.py` |
| colisión | 18 mapas, 4 B por celda (`$200000+`) | `data/level_a_0f2ffc.s` |
| tablas de color | `ColorRGB555ToNeoGeo_002f30` (32768 palabras), `ColorRamp_012fb0` (32×32) | `data/sprite_tables_002f30.s` |
| otro (33 %) | tablas indexadas por rutina, sin decodificar | pendiente |

## Assets (S/C/V-ROM)

`assets/` extrae y reconstruye los demás ROMs del juego, byte a byte:
`gfx.py` (fix 8×8 y sprites 16×16 ↔ PNG indexados), `audio.py` (375
muestras ADPCM-A/B ↔ WAV, con codificador), `sprites.py`, `levels.py` y
`data_census.py`. La música y los efectos se exportan a MIDI, WAV y VGM con
`mrom/tools/export_midi.py` y `export_wav.py`. Ver `assets/README.md`.

## Como regenerar este documento

```bash
pip install capstone          # solo para la heuristica por bloques
python3 tools/measure_coverage.py --zones   # tabla por zonas (mapa curado)
python3 tools/measure_coverage.py --blocks  # mapa heuristico 4 KiB (orientativo)
```

Al cerrar una wave: (1) si descubre una frontera codigo/datos nueva, editar
`ZONES` en `tools/measure_coverage.py`; (2) pegar la salida de `--zones` en
la seccion "Mapa curado"; (3) actualizar la fecha y las tres cifras de arriba.
