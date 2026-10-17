# 09 — Auditoría de funciones ocultas (verificación empírica, 2026-10-17)

Comprobación una a una de una lista de "hallazgos" sobre restos de desarrollo.
Herramientas: `tools/dev/emu_run.sh` (pydmg headless), `emu_pokes.py` (pokes
repetidos: el juego reescribe `$100000` al pulsar START), `emu_input.py`
(avanzar disparando), `bytepatch_zip.sh` y objdump sobre `build/mslug_prom.bin`.
Leyenda: ✅ confirmado en emulador · 🔎 confirmado en el binario (estático) ·
⚠️ parcialmente cierto · ❌ falso.

## A. Flags de depuración `$100000/$100001`

| Bit | Afirmación | Veredicto | Evidencia |
|---|---|---|---|
| `$100001` b0 | HUD de variables | ✅ con matiz | `debug=0001`: aparecen columnas hex (results/debug_b0_b1.png). `Debug_DrawHUDVars_096A80` imprime `$106E88`, `$106F5C`, `$106F50`, `$106F54`, dos valores derivados ($5D8F2) y `$10E39C`. **`$106F5C` no es un RNG**: en emulador sigue a la parte alta de `$106F50` (`$01AE` = `$01AE0800`), es la X de scroll de cámara. b0 además habilita el atajo al sound test en el attract (C) y DebugColl (b1). |
| b1 | Inspector de colisión DebugColl | ✅ | `DebugColl_Init_05efca` sólo se instala desde `Task_InstallMissionSlots_001dcc` si b0=1; con `debug=0003` aparece una columna extra de valores hex (results/debug_b0_b1.png) que con `0001` no está. Con `debug=0002` sola no cambia ningún píxel en 52 capturas (b1 en `ScriptOp02_Sprite` sólo pone el bit 7 de `$5A`). |
| b2 / b7 | Boss_Shadow_Init A/B con P2 A+Derecha | 🔎 | `DebugTriggers_TwoBits_096B24` (llamada por los 7 handlers de inicio de misión `$96832..$969B4`) comprueba `InputEvtThunk_05cd18` (A de P2, `$10E206`) y `_05d11c` (Derecha) y lanza `$86586`/`$86582`: dos entradas de `Boss_Shadow_Init_08656e` (hitbox `$2ECD80` vs la normal `$2ECEC4`). No reproducido en emulador (pydmg sólo inyecta mando P1). b2 también se lee 5 veces en `Player_AirActionSelect_037168`. |
| b3 | Invulnerabilidad | ✅ | receta 06 (ya verificado). |
| b4 | HUD de cadena del Slug + flag +$124 | 🔎 | lectores `Chain3_TplC/Follow/DebugHud` y `ScriptOp02_Sprite` (+$124: `jsr $2F84A` si b0 de `$6B`). Sin Slug en pantalla, `debug=0010` no cambia ningún píxel (0/52 capturas); efecto visible no comprobado. |
| b5 | Arco de salto alternativo | ✅ | `debug=0020`: la partida diverge desde el frame 3650 (47/52 capturas distintas). El jugador salta distinto (`Player_JumpAir_036a70`) y no avanza. |
| `$100000` b0/b1/b7 | grabación / replay | 🔎 parcial | b7 en `Ent_InputSample_04546E`, `InputMask_*`, `InputLayout_ReadField*`; b0 en `Entity_MoveAndCollide_C`; b1 en `PlayerGrenade_SpawnB`. |
| — | lo escribe `UserMode0_080C`, 0 al arrancar | 🔎 | `$81C clr.w $100000` / `$826 move.w #0` (ambas ramas según `$10FE80`). |

## B. Asserts `trap #15`

* ⚠️ **No son 15 en 6 módulos.** En la zona de código (< `$9C608`) hay **120**
  opcodes `4E4F`; **107** con el esqueleto `nop` delante, repartidos en
  **17 ficheros** (los más: barrel_paratrooper 32, collmap_hitbox 26,
  bazooka_walker 8, event_vm 6, vehicle 5, gunship 5, fx_explosions 5,
  hitbox_collide 4, sniper 4, hud_inputrec 3, wave_motion 2, late_props 2,
  table_lookup 1, script_dispatch 1, arm_weapon 1, spawn_boss 1, allen 1).
  Los conteos citados para los 6 módulos sí son correctos.
* ✅🔎 `Bios_TrapF_CalendarSeq_0008a4`: 5 × `move.w #$1234,d0; move.w #$0A..$0E,d1; trap #15`.
  **Corrección**: está apuntado por el vector `$14` (división por cero), no
  por el de trap #15. Los vectores `$18..$24` (CHK/TRAPV/privilegio/traza)
  apuntan a las entradas `$8AE/$8B8/$8C2/$8CC` dentro de la misma secuencia,
  y el vector de trap #15 (`$BC`) vale `$C0` (bucle `move.l #1234,$0; bra`).
  Es un manejador de excepciones de CPU que pasa un código por excepción, no
  una secuencia de calendario.

## C. Sound Test interno — ✅

* Strings en ROM: `SOUND TEST` `$7A7E8`, `1P JOYSTICK` `$7A7FA`, `PUSH A-CSL CHANGE` `$7A84B`.
* **Cómo se llega** (antes sin documentar): en el attract,
  `Attract_PostStart_Cleanup_001AB6` con `$100001` b0 y `InputEvt_ToggleChain_05d288`
  (P1 START / D en `$10E200`) salta a `$F76`, que carga la sub-tabla T6[12] →
  T6[13] `BootScr_SoundTest_001566` → `SoundTest_Init_07a456`.
* Parche reproducible: `bash scripts/apply_patch.sh boot=soundtest --zip internal/mslug.zip`
  (cambia el `lea BootTblEntry_B92` del arranque por `BootTblEntry_E6E`, 2 bytes en `$F48`).
  Captura: results/sound_test.png (4 filas de código `0020`, cursor, leyenda
  completa de P1/P2: Arriba/Abajo ±1, Izq/Der ±$10, A-CSL change, B-bank change,
  C-BGM off, D-SE off; P2 A..D = canales 0..3).
* Las entradas de P1 se ven en el menú (código pasa a `0021`, el cursor baja).
  El envío al Z80 (`jsr $219C` con código + offset de página `$7A7F4`) se
  hace en la rama de **P2** (`$10E209` b4..b7), que pydmg no puede pulsar; el
  audio no se comprobó.

## D. Stream de misión de debug — ✅ con corrección

* 🔎 `MissionDriver_Init_0442E6`: `cmpi.l #$F1CD4,d0; bne; tst.b $10FD83; beq; move.l #$F260E,d0`. El programa nunca escribe `$10FD83` (0 escrituras).
* **Corrección**: `$10FD83` no es un "byte interno con 4 efectos": es el
  **código de región del BIOS** (0 = Japón, 1 = EE. UU., 2 = Europa; en
  pydmg vale `$02`, `$82` durante el arranque). Sus 16 lectores (créditos,
  resultados, textos, menú de opciones, `FixGlyphRun_DrawPad2P` "== 2") son
  ramas por región.
* **La escena 12 no es fantasma**: es el **final del juego**. La sub-tabla T4
  (`$D8A`: `BootScr_SceneBCD_Init` en `$165A` pone `$106ECE=12`) se llega por
  `BootScr_Ending_SelectBranch_001604` (y T3 `$D76` → `$1678`). Con
  `boot=ending` el emulador muestra el epílogo (Marco en el atardecer, el
  avión de papel, el Metal Slug con el humo, "SUPER VEHICLE 001 METAL SLUG",
  los créditos SOUND/PLANNING). El stream activo es `$F1CD4` con región 0
  (JP: `$100870` recorre `$F1D1C..$F1E82`) y `$F260E` con región ≠ 0
  (EU/US: `$F2656..$F2876`). Es decir, `MissionStream_Debug` es en realidad
  la **variante internacional del stream del final**, no un modo debug.

## E. `$10FD8F` — ✅ con corrección

* 9 lectores confirmados: `Player_SpawnDebugTask_033358`,
  `HumanDeath_PlayCryByKind` (#14/#47), `HumanDeath_Collapse` (#340/#341),
  `HumanDeath_BloodSplashA/B` (#378/#442), `HumanDeath_SpawnSmokePair`
  (#415/#419), `Paratrooper_Die` (#387/#471), `Crew_LandB`/`Crew_HatchB_Land` (#469).
  **Corrección**: los números no son sonidos; van a
  `Entity_AllocSpriteSlot_00236E` (`d1` = índice de descriptor de sprite).
* **Semántica resuelta**: es el soft-DIP **BLOOD** del menú de configuración
  (cabecera MVS en `$2781F4`: HERO, CONTINUE, DIFFICULTY 1–8, PLAY TIME 60–90,
  DEMO SOUND, PLAY MANUAL, **BLOOD**; layout `$10FD84` tiempo, `$10FD86`
  cuenta, `$10FD8A..` listas: CONTINUE `$8A`, DIFFICULTY `$8B`, PLAY TIME `$8C`,
  DEMO SOUND `$8D` (leído en `InputGuardCall219c`), PLAY MANUAL `$8E`, BLOOD `$8F`).
  Comprobado en emulador con `10FD8F=01` repetido: los soldados que mueren
  sueltan **sangre roja** en vez del humo/polvo gris por defecto
  (results/blood_on_off.png, frames 3700 y 4400).
* `Player_SpawnDebugTask` no es un marcador de debug: lo llama
  `Player_Death_Debug_037af2` (muerte del jugador) y con BLOOD activo crea
  `Player_DebugMarker_03d842`, que dibuja el descriptor #378 (el mismo de la
  salpicadura). Es la sangre de la muerte del jugador.

## F. Contenido muerto

* 🔎 Z80: `SeqUnused_1D1A8/1D238/1D262` ("not referenced by any header"),
  `Leftover_1D287` en `mrom/sound/` — existen así en el repo.
* 🔎 `$12345678` en P-ROM `$1FF800` (CPU `$2FF800`).
* 🔎 `AutoDemo_DebugWarpKeys_0797ce`: 0 referencias long, 0 pc-relativas, 0 `bsr.s`.
  Lee `$10E202/3` (P1 actual/flanco): D+C → posición `($A0,$180)` y `$4C=$2DF456`;
  D+B → `clr.w $106E92`. Huérfano confirmado.

## G. Bugs del original

* 🔎 `$05D628`: `movea.l $72(a1),a2` se ejecuta **antes** de `lea $100300,a1`
  (en la rama P2 el orden es el correcto: `lea` y luego `movea.l`). Confirmado.
* 🔎 `$041A96`: `movea.l #-1,a0` seguido de `lea $28610A,a0`: store muerto. Confirmado.
