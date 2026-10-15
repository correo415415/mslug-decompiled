# HANDOFF — estado completo del proyecto y cómo continuarlo

> **Para quien llegue nuevo (persona o IA):** lee este fichero de arriba abajo.
> Contiene todo lo necesario para entender el repositorio, reproducir el
> estado actual, verificar que nada se ha roto y continuar el trabajo sin
> más contexto. Los demás documentos (`README.md`, `docs/`, `mrom/README.md`,
> `TODO.md`) amplían detalles pero no son imprescindibles para empezar.

Última actualización: ver `git log -1 -- HANDOFF.md`. Rama de trabajo:
`genspark_ai_developer`; PR abierto hacia `main`:
<https://github.com/correo415415/mslug-decompiled/pull/21>.

---

## 1. Qué es este proyecto

Descompilación **matching** (byte-idéntica) de **Metal Slug 1 (Neo Geo, SNK 1996)**,
romset MAME `mslug.zip`.

| ROM | Fichero | CPU | Estado |
|---|---|---|---|
| P-ROM (programa principal) | `201-p1.bin`, 2 MiB | Motorola 68000 | **100 % matched**: 17 608 secciones, ROM reconstruida byte-idéntica |
| M-ROM (driver de sonido) | `201-m1.bin`, 128 KiB | Zilog Z80 | **reensamblado byte-idéntico 100 %**; región fija con 157 rutinas/tablas con nombre y 0 etiquetas anónimas; todos los datos de sonido (instrumentos, tablas, cabeceras, 31 650 ops de secuencia) como fuentes simbólicas en `mrom/sound/`; músicas/voces con nombre real; export MIDI, WAV idéntico al juego (driver real + ymfm; músicas en bucle+fundido, 150 SFX, 27 voces con nombre) y VGM |
| S-ROM / C-ROM / V-ROM (gráficos / samples) | `201-s1`, `201-c1..c4`, `201-v1..v2` | — | **extracción + reconstrucción byte-idéntica** (`assets/tools/gfx.py`, `audio.py`; ver `assets/README.md`): tiles PNG indexados y 375 muestras con nombre |

Objetivo a largo plazo: fuentes legibles (C + asm con nombres semánticos) que
compilen a los ROMs originales, más herramientas para parchear y para extraer
assets.

**Las ROMs nunca entran en git** (`.gitignore`). Se aportan como
`internal/mslug.zip` (directorio `internal/` sí está en git pero el zip está
ignorado) o se copian a mano.

---

## 2. Reproducir el entorno desde cero (sandbox Linux limpio)

```bash
cd /home/user && git clone https://github.com/correo415415/mslug-decompiled.git webapp
cd webapp && git checkout genspark_ai_developer
# coloca el romset en internal/mslug.zip (o pásalo como argumento) y:
./scripts/bootstrap_sandbox.sh internal/mslug.zip
```

`bootstrap_sandbox.sh` instala `gcc-m68k-linux-gnu`, `binutils-m68k-linux-gnu`,
`z80asm`, `z80dasm`, dependencias Python (`requirements.txt`), extrae
`rom/201-p1.bin` → `build/mslug_prom.bin` (formato CPU, MD5
`816b3f74c76b3373993407615f1850fe`) y `rom/201-m1.bin` (MD5
`6449db0459bcf7584348792009e8f12c`), y ejecuta `registry_lint` y
`check_section_sizes`. Si hay AI Drive montado guarda una copia del zip en
`/mnt/aidrive/mslug_rom/mslug.zip` para futuros resets.

### Verificación completa (debe dar todo OK antes y después de cualquier cambio)

```bash
python3 tools/registry_lint.py            # registry_lint: OK (0 errores)
python3 tools/check_section_sizes.py      # 0 discrepancias
python3 tools/build_rom.py                # ✔ IDÉNTICA (P-ROM 2 MiB, MD5 816b3f74…)
python3 mrom/tools/build_mrom.py          # ✔ IDÉNTICA (M-ROM, MD5 6449db04…)
# matcher completo (≈15-20 min; lánzalo en segundo plano, NUNCA lo mates con pkill -f):
(nohup bash -c 'timeout 1500 python3 tools/match_batch.py > /tmp/match_full.txt 2>&1' >/dev/null 2>&1 &)
tail -3 /tmp/match_full.txt               # MATCHED : 17608/17608
```

### Emulador (opcional, para verificar parches jugando)

`internal/pydmg/` contiene el emulador Neo Geo en Rust con parches de
desarrollo (watch/poke de RAM, script de mandos). Compilar:

```bash
(command -v cargo >/dev/null || curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal)
export PATH=$HOME/.cargo/bin:$PATH; sudo apt-get install -y -qq libsdl2-dev
cd internal/pydmg && cargo build --release        # ~45 s → target/release/pydmg-neogeo
```

Flags útiles: `--headless --cart ZIP --max-frames N --auto-coin-frame 400
--auto-press-start-frame 500 --input-script "F1-F2:MASK,..." --watch-ram
"ADDR:LEN,..." --poke-ram "FRAME@ADDR=HEX" --dump-frames-dir DIR
--dump-every-frames K`. Máscara de botones: 1 Up, 2 Down, 4 Left, 8 Right,
16 A = disparo, 32 B = salto, 64 C = granada, 128 D. El juego empieza
~frame 4400. Wrapper: `scripts/dev/run_emu.sh`.

---

## 3. Mapa del repositorio

```
asm/            fuentes 68k matcheadas (GAS), por tema: boot/ kernel/ input/ video/
                entity/ player/ slug/ enemies/{soldiers,squad,vehicles,bosses}/ pow/
                props/ mission/ frontend/ data/   — nombre: <qué>_<dir_cpu_más_baja>.s
src/            fuentes C 68k matcheadas (bios/ kernel/ entity/ video/ thunks/)
include/        cabeceras C compartidas
linker/         scripts de enlazado
tools/          pipeline P-ROM: registry.py (lista maestra), symbols.py, match_batch.py,
                build_rom.py, registry_lint.py, check_section_sizes.py, gen_asm_region.py,
                gen_data_region.py, measure_coverage.py, reorg/ (reorganización aplicada)
scripts/        setup.sh, setup_mrom.sh, bootstrap_sandbox.sh, rebuild_rom.sh, pack_prom.sh,
                apply_patch.sh (recetas de parche), dev/ (emulador), legacy/
mrom/           M-ROM Z80 (ver §5)
docs/           COVERAGE.md (métricas), PROGRESO.md (bitácora por "waves"), CONVENTIONS.md
                (nombres, registry, layout), waves/ (nombres por wave), images/
internal/       solo desarrollo: mslug.zip (ignorado), pydmg/ (emulador), experiments/
                (recetas 01-05 de modificación con resultados y capturas)
TODO.md         lista de tareas pendientes (incluida la extracción de assets)
CHANGELOG.md    cambios por versión; README.md  visión general para usuarios
.github/workflows/matcher.yml   CI self-hosted: lint, matcher, build_rom, build_mrom
```

Convenciones clave (`docs/CONVENTIONS.md`):
* Cada función/dato matcheado tiene una entrada en `tools/registry.py`:
  `(nombre, dir_cpu, tamaño, fichero)`. `registry_lint.py` tras cada edición.
* Nombres: `Subsistema_QuéHace_DIRHEX` (p. ej. `Player_ThrowGrenade_0332BC`).
* `symbols.py`: placeholders externos resueltos con `--defsym`; cuidado con
  claves duplicadas (ver "duplicate-key gotcha").
* La ruta de registro usa `path.replace("/","__")` para los ficheros en
  subcarpetas.

---

## 4. P-ROM (68000): hecho

* **100 % matched.** `match_batch.py` → `MATCHED 17608/17608`, ROM
  `2,097,152/2,097,152`. Bitácora completa de las ~55 "waves" en
  `docs/PROGRESO.md`; cobertura real código/datos en `docs/COVERAGE.md`.
* **Reconstrucción**: `tools/build_rom.py` enlaza todo → `build/mslug_prom_rebuilt.bin`
  idéntico; `scripts/rebuild_rom.sh [--mod] [zip]` produce `build/repack/201-p1.bin`
  (formato MAME, MD5 `b6804bc6…`) y `build/repack/mslug.zip` listo para jugar.
* **Reorganización** (267 ficheros movidos a subcarpetas temáticas con nombres
  semánticos) hecha con `tools/reorg/apply_reorg.py` + `reorg_map.txt`.
* **Parches de ejemplo** (`scripts/apply_patch.sh <recetas> --zip internal/mslug.zip`;
  `--undo` restaura; backups en `build/patch_backup/`). Regla: **sustituir
  instrucciones por otras del mismo tamaño** para que las secciones no cambien
  (`check_section_sizes` = 0):

  | Receta | Qué cambia | Verificado |
  |---|---|---|
  | `lives=N` | `HUD_Task_Init_025706`: `move.b 0x4(a3),d1` → `move.b #N,d1` | emulador |
  | `hmg-infinite` | `PlayerFire_HMG_*Spawn`: `subi.w #1,0x82(a2)` → `ori.w #0,0x82(a2)` ($03C898/$03C89B) | emulador, `$1004C2` fijo en 00C8 |
  | `all-infinite` | ídem para shotgun/rocket/flame (`$03C9EA,$03CB3C,$03CCF8`) | emulador |
  | `bombs=N` | `PlayerEntity_InitAuxState_032A02`: `move.b #0xa,0x80(a6)` → `#N` ($032A3D) | emulador, `$1004C0` |
  | `bombs-infinite` | `Player_ThrowGrenade{,Back}_0332BC/033256`: `subi.b #1,0x80(a0)` → `ori.b #0,…` | emulador, captura en `docs/images/bombs_infinite_demo.png` |
  | `pistol-ammo=N` | `move.w #0xa,0x82(a6)` en 3 sitios (munición al perder arma) | rebuild |

* **RAM del juego** (para experimentos): entidad P1 `$100440` (P2 `$1004E0`);
  `+$71` arma (0 pistola, 1 HMG, 2 shotgun, 3 rocket, 4 flame) → `$1004B1`;
  `+$80` bombas → `$1004C0`; `+$82` munición del arma → `$1004C2`; `+$85`
  latch de disparo; vidas soft-DIP `$10FD88`. Más en
  `internal/experiments/05_ram_map_and_ideas.md`.

### P-ROM: pendiente (baja prioridad)
* Cabeceras A–F y nombres semánticos para los 13 módulos de datos
  (`asm/data/*`), como en el resto.
* Más recetas (selección de misión, invulnerabilidad, debug DIP).

---

## 5. M-ROM (Z80, `201-m1.bin`): hecho

Toda la documentación está en `mrom/README.md` (mapa de memoria, layout,
workflow, mapa de RAM, estado) y en la sección M-ROM de `docs/COVERAGE.md`.
Resumen:

**Hardware/mapa**: Z80 `0000-7FFF` = fichero `0x00000-0x07FFF` (región fija);
`8000-BFFF` = ventana de 16 KiB sobre los **bancos de fichero 2..7**
(banco N = fichero `N*0x4000`) seleccionada leyendo el puerto `0B` con el
banco en B (`Bank_ResolvePtr24_1FA6`). Los punteros de las tablas son
**direcciones lineales de 24 bits** (banco = addr>>14, offset = addr&0x3FFF|0x8000).
RAM `F800-FFFF`. Puertos: 0 = latch de comando del 68k, `0C` = respuesta,
`08`/`18` = NMI on/off, `04-07` = YM2610. Vectores: `0000` reset→`Main_Init_1E00`,
`0008` RST8 = `YM_Write_RST8`, `0038` IRQ→`IRQ_Timer_1E7A`, `0066` NMI→`NMI_Command_1ED6`.

**Pipeline** (todo regenerable, el `.bin` reconstruido debe ser siempre idéntico):

```
rom/201-m1.bin
  └─ mrom/tools/gen_mrom_asm.py  (z80dasm -b mrom/map/<region>_blocks.txt -S <region>_syms.txt)
       └─ mrom/asm/raw/<region>.s          (fixed_0000, bank2_8000 … bank7_8000)
            └─ mrom/tools/split_region.py fixed_0000  (según mrom/map/fixed_0000_split.txt)
                 └─ mrom/asm/{core,channel,seq,data}/*.s  +  mrom/asm/fixed_0000.s (org + includes)
                      └─ mrom/tools/build_mrom.py  (z80asm -I mrom/asm -I mrom/include) → ✔ IDÉNTICA
```

* `mrom/map/fixed_0000_blocks.txt` — 74 bloques código/datos (14 288 B código,
  18 480 B datos), construido con `mrom/tools/z80trace.py` (trazador recursivo
  a nivel de bytes desde vectores + tablas de salto `2076`, `20B3`, `4AA0/4ABC/4AD8`, `53A3`).
* `mrom/map/fixed_0000_syms.txt` — 128 nombres de rutinas/tablas + 61 variables
  de RAM (`wTick_F800`, `wCurBank_F811`, `wSSG_ChA_F81F`, `wFM_Ch1_F907`,
  `wPCMA_Ch1_FAA5`, `wPCMB_Ch_FB9B`, `wMusicTracks_FBFC`, `wSfxTracks_FDF2`,
  `wSeqPtr_FF0D`, …). **El trabajo de nombrar se hace aquí**, no en los `.s`.
* `mrom/map/fixed_0000_split.txt` — 18 ficheros: `core/` (vectores, main/IRQ/NMI/bancos,
  dispatcher de comandos, tablas, handlers), `channel/` (envolventes, SSG, FM,
  ADPCM-A, ADPCM-B, tablas por tipo de canal), `seq/` (secuenciador, opcodes),
  `data/` (tabla de punteros `0080`, páginas `0100-1DFF`, `NoteMapTable_4B84`,
  `InstrPtrTable_57A1`, cola `67A1`).
* `mrom/include/neogeo_z80.inc` — puertos, ventanas, registros YM2610.
* `mrom/tools/dump_sound_tables.py` — lista música / SFX / instrumentos.

**Formatos descubiertos**:
* Comandos del 68k: `01` reset suave, `03` reinicio, `<10` → `CmdJumpTable_2076`
  (stop música `02`, stop all `04/05`, stop seq `06`, flag `0C`, stop sfx `0F`),
  `≥20` → id de sonido = cmd−0x20, tipo según `wCmdType_F81C` (0 música, 1 SFX A,
  2 SFX B) → `CmdHiJumpTable_20B3`.
* `BankPtrTable_0080` (18 × 24 bits): `[0]`=`InstrPtrTable_57A1`, `[1]`=tabla de
  música `0x815F` (13 entradas), `[2]`=tabla SFX A `0x8472` (259 entradas, 177 usadas),
  `[3]`=tabla SFX B `0x87F2` (35, 27 usadas); el resto repiten `0x8472`.
* **Cabecera de música**: `volumen(1)`, N × `{ptr24 secuencia, flags, canal}` (5 B),
  terminador `00 00 00`. 13 músicas, 5–14 pistas cada una; secuencias en bancos 2..6.
* **Cabecera de SFX**: `4 × u16 ids que detiene`, `prioridad(1)`, N × pista (5 B),
  terminador. 1–3 pistas; secuencias en bancos 6..7 (`0x1BFCB-0x1D196`).
* **Secuencias** (`Seq_TrackFetch_5362`): byte `≥DA` fin de pista, `C0..D9` →
  `SeqOpJumpTable_53A3` (C0/C1 instrumento, C2 salto, C4 llamada a subsecuencia
  —pila de 3 niveles en `+1C`—, C5 retorno, C6 volumen, C7 pan, C8 parámetro,
  CA detune, CB espera, CC gate, CD tempo, CE transposición, CF/D0 bucle,
  D1 portamento, D2 LFO, D3–D6 nota directa por tipo, D7/D8 pitch bend,
  D9 responde al 68k por el puerto `0C`), `<C0` → `Seq_NoteEvent_56EA`
  (nota + bytes opcionales len/vol/gate según bits).
* `InstrPtrTable_57A1`: 1024 punteros (8 grupos × 128) a parches de instrumento
  (105 distintos; registros de 24 o 50 B en `67B9..` y banco 2) vía
  `Instr_ResolvePtr_4B5D` (índice = `(iy+0B)*128 + (iy+0A)`; `FF` = parche por
  defecto `InstrDefault_2509`).
* Pistas: 14 de música en `FBFC` + 8 de SFX en `FDF2`, 0x23 B cada una
  (`+00/01` contador, `+02` prioridad, `+04` banco, `+05/06` id, `+07/08` edad,
  `+09` flags, `+0A/0B` instrumento, `+0C-0E` vol/pan/param, `+10/11` pitch,
  `+12` gate, `+13` LFO, `+14/15` puntero de secuencia, `+16` espera, `+17-1A`
  nota/len/vol/gate actuales, `+1B` última nota, `+1C-21` pila de llamadas,
  `+22` puntero de pila).

### M-ROM: datos de sonido (hecho, sesión f_5.1 →)
Los bancos 2..7 (fichero `0x08000-0x1FFFF`) **no contienen código Z80**, solo
datos. `mrom/tools/gen_sound_data.py` los decodifica a `mrom/sound/*.s`
(GNU as de `binutils-z80`, por sus macros): `headers.s` (tablas + cabeceras),
`music_NN.s` (una por canción), `sfx.s`, `seq_unused.s`, `instruments.s`,
`leftover.s`. Las notas se escriben absolutas (`N 0x39, 0x0c, 0x0a, 0x42`); las
macros recalculan el delta relativo. `build_mrom.py` ensambla la región fija
con z80asm y `mrom/sound/sound.s` con `z80-unknown-coff-as` → ✔ IDÉNTICA.
`instruments.s` cubre además `0x057A1-0x07FFF` de la región fija
(`InstrPtrTable_57A1` + parches SSG/FM/ADPCM-A/B y tablas de samples, macros
`INSTR_*`): `build_mrom.py` ensambla primero `mrom/sound/` y escribe
`build/mrom/instruments_fixed.bin`, que `mrom/asm/data/instruments_57a1.s`
incluye con `incbin` (línea `incbin=` de `fixed_0000_split.txt`).
Los antiguos volcados z80dasm de los bancos se han eliminado. Formatos:
`mrom/docs/FORMATS.md`. Snapshot previo inmutable: rama/tag `f_5.1`.

### M-ROM: pendiente (orden sugerido)
1. ~~**Bancos 2..7**~~ (hecho, ver arriba). Texto original: crear `mrom/map/bank{2..7}_8000_blocks.txt` marcando como
   `bytedata`/`worddata` las tablas conocidas (tablas de música/SFX en banco 2,
   cabeceras `0x8193-0x953C`, secuencias, parches de instrumento) y nombrar
   (`_syms.txt`) cada música (`Music_00_Hdr_8193`…), SFX y secuencia. Regenerar
   con `gen_mrom_asm.py`, trocear con un `bankN_8000_split.txt` y mantener
   ✔ IDÉNTICA. (`gen_mrom_asm.py` ya aplica los mapas si existen.)
2. ~~Etiquetas internas~~ (hecho: `gen_mrom_asm.py` convierte inmediatos
   falsos en constantes y nombra los saltos locales `<Rutina>_L<dir>`).
   Pendiente: documentar la estructura de canal (`+00..+4C` SSG, `+67` FM,
   `+29` ADPCM-A).
3. Documentar formalmente el formato de secuencia e instrumentos en
   `mrom/docs/FORMATS.md` (los datos de arriba son el borrador) y escribir
   un desensamblador de secuencias (`mrom/tools/dump_sequence.py`) que
   convierta cada pista a texto legible (y, después, a MIDI/VGM).
4. ~~Identificar qué música/SFX es cada id~~ (hecho para músicas y voces:
   `find_sound_ids.py` → `mrom/map/sound_ids.txt`). Texto original: identificar qué música/SFX es cada id (jugar con el emulador y observar
   los comandos enviados al puerto de sonido desde el 68k; en el P-ROM buscar
   las escrituras a `$320000` / `REG_SOUND`), y poner nombres (`Music_Mission1`…).
5. ~~Sustituir los `defb` en bruto~~ (hecho para sonido/instrumentos;
   quedan las páginas de tablas `0100-1DFF`). Texto original: sustituir los `defb` en bruto por datos estructurados cuando el formato
   esté claro, manteniendo la identidad byte a byte.

---

## 6. Después del M-ROM: extracción de assets (ver `TODO.md`)

Scripts para S-ROM (tiles 8×8 fix → PNG), C-ROM (desentrelazar c1/c2, c3/c4,
tiles 4 bpp 16×16, paletas del P-ROM → PNG y sprites reconstruidos desde las
tablas del P-ROM — **hecho**: `gfx.py` + `sprites.py`; fondos con `levels.py`, 52 capas a color; ver `assets/README.md`), V-ROM (cortar muestras ADPCM-A/B usando los parches del
M-ROM → WAV), con **nombres significativos** tomados de los símbolos ya
establecidos, índice CSV/Markdown y herramienta inversa (PNG/WAV → ROM
byte-idéntico). También exportar las 13 músicas a VGM/MIDI.

---

## 7. Flujo de trabajo y reglas

1. Trabajar en `genspark_ai_developer`. **Commit + push tras cada paso pequeño**
   (el entorno de desarrollo se reinicia a menudo y se pierde lo no subido).
   Mantener el PR abierto (hoy #22) actualizado (`gh pr edit 22 --body-file …`; hace falta un
   token de GitHub con permiso de workflows si se toca `.github/`).
2. Antes de cada commit que toque fuentes: `registry_lint`, `check_section_sizes`,
   `build_rom.py` y/o `build_mrom.py` deben dar OK/IDÉNTICA.
3. Nunca modificar `mrom/asm/raw/*.s` ni los ficheros troceados a mano mientras
   un rango siga en el split: editar `mrom/map/*` y regenerar. Si se reescribe
   un rango a mano, sacarlo de `fixed_0000_split.txt`.
4. Parches de juego: mismo tamaño de instrucción, documentar en
   `internal/experiments/NN_*.md` con bytes tocados y verificación.
5. Documentar avances en `CHANGELOG.md` (Unreleased), `docs/COVERAGE.md`
   (métricas) y `docs/PROGRESO.md` (bitácora), y marcar casillas en `TODO.md`
   y en este `HANDOFF.md`.
6. Idioma: documentación en español (algunos ficheros técnicos en inglés);
   nombres de símbolos en inglés.

---

## 8. Checklist rápido de estado

- [x] P-ROM 100 % matched, ROM reconstruida idéntica, CI verde (`matcher.yml`)
- [x] Repositorio reorganizado por temas con nombres semánticos
- [x] Parches lives / hmg-infinite / all-infinite / bombs / bombs-infinite / pistol-ammo verificados
- [x] M-ROM: reensamblado idéntico, región fija código/datos + nombres + troceo, include HW, CI `build_mrom`
- [x] M-ROM: tablas de música/SFX/instrumentos y formatos de cabecera identificados (`dump_sound_tables.py`)
- [x] M-ROM: bancos 2..7 decodificados a `mrom/sound/` (tablas, cabeceras, secuencias simbólicas) + `mrom/docs/FORMATS.md`
- [x] M-ROM: formato de instrumentos (`mrom/sound/instruments.s`); región fija sin etiquetas anónimas
- [x] Identificación de músicas y voces por id (`mrom/map/sound_ids.txt`); SFX de la tabla A con usuarios
- [x] Export MIDI (`export_midi.py`), WAV idéntico al juego y VGM (`export_wav.py` + `sndrender`)
- [x] M-ROM: tablas de páginas `0100-1DFF` con nombre (FORMATS.md §9)
- [x] M-ROM: estructura de canal documentada (FORMATS.md §10)
- [ ] Extracción de assets S/C/V-ROM (tras M-ROM al 100 %)
- [ ] P-ROM: cabeceras A–F en los 13 módulos de datos
