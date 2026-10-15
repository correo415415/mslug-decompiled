# TODO

Lista viva de tareas pendientes del proyecto. Las tareas se mueven a
`CHANGELOG.md` / `docs/PROGRESO.md` cuando se completan.

## P-ROM (68000) — 100 % matching

- [ ] Cabeceras A–F / nombres semánticos para los 13 módulos de datos
      (`asm/data/*`), igual que el resto de módulos.
- [ ] Revisar que el matcher completo (`tools/match_batch.py`) siga en
      17608/17608 tras cada reorganización.

## M-ROM (Z80, `201-m1.bin`) — en curso

- [x] Separar código y datos en la región fija (`mrom/map/fixed_0000_blocks.txt`).
- [x] Símbolos semánticos para el núcleo del driver y mapa de RAM
      (`mrom/map/fixed_0000_syms.txt`, 126 rutinas + 61 variables).
- [x] Equates de hardware en `mrom/include/neogeo_z80.inc`.
- [x] Región fija troceada en `mrom/asm/{core,channel,seq,data}/` (18 ficheros).
- [x] Etiquetas de la región fija: 0 anónimas. 157 rutinas/tablas con nombre
      (`fixed_0000_syms.txt`), 503 destinos locales `<Rutina>_L<dir>`, 137
      inmediatos falsos (pares YM de RST 8, offsets) como constantes.
- [x] Documentar la estructura de canal (offsets `+00..+4C` SSG, `+67` FM,
      `+29` ADPCM-A, `+1F` ADPCM-B) en `mrom/docs/FORMATS.md` §10.
- [x] Tablas de página `0100-1DFF` con nombre (FORMATS.md §9).
- [x] Documentar el formato de secuencia (opcodes `C0..D9`, bytes de nota) y
      el de las cabeceras de música/SFX → `mrom/docs/FORMATS.md`.
- [x] Bancos 2..7 (solo datos): tablas, cabeceras y secuencias decodificadas a
      fuentes simbólicas `mrom/sound/` (`gen_sound_data.py`), ✔ IDÉNTICA.
- [x] Formato de instrumentos (SSG 24 B / FM 50 B / ADPCM-A / ADPCM-B + tablas
      de samples) → `mrom/sound/instruments.s` con macros `INSTR_*` (incluida
      la parte de la región fija `0x57A1-0x7FFF`, vía `incbin`).
- [x] Nombres reales de músicas/SFX (ids enviados desde el P-ROM):
      `mrom/tools/find_sound_ids.py` → `mrom/map/sound_ids.txt` (149 ids, 13
      músicas con nombre, 19 voces con nombre).
- [ ] Nombrar los SFX restantes de la tabla A (hoy `SfxA_NNN` + usuarios) y
      recorrer los streams de script para los ids que no son constantes.
- [ ] Actualizar `mrom/README.md`, `docs/COVERAGE.md` (sección M-ROM),
      `docs/PROGRESO.md` y `CHANGELOG.md` a medida que avance.

## Al tener el M-ROM al 100 %

- [x] **Extracción de assets S/C/V-ROM con herramienta inversa** (`assets/`):
      `gfx.py` (fix 8×8 y sprites 16×16 ↔ PNG indexados) y `audio.py` (V-ROM →
      375 muestras con nombre e índice CSV, ADPCM crudo + WAV); los 7 ROMs se
      reconstruyen byte-idénticos.
- [x] Sprites con paletas del P-ROM, frames multi-pieza y animaciones con sus
      tiempos (`assets/tools/sprites.py`), nombradas por la rutina que las usa.
- [ ] Sprites: paleta para las ~130 animaciones que salen en grises (revisar
      a mano → `assets/map/anim_palettes.txt`); nombres por personaje/arma.
- [x] Fondos de las misiones → PNG (`assets/tools/levels.py`, 52 capas de 13
      escenas desde los `bind_path` de los scripts de escena).
- [x] Fondos: slot de paleta 0 (ScriptSlotPairTable_0009B4) — 52/52 capas con color.
- [ ] Fondos: composición con parallax/anclas.
- [x] Codificador ADPCM-A/B para sustituir muestras desde WAV (`audio.py encode`).
- [x] Exportar las pistas musicales a MIDI con sus nombres
      (`mrom/tools/export_midi.py`). La curva de `S_TEMPO` ya está modelada (set-tempo exactos).
- [x] Exportar todo el audio a WAV idéntico al juego (`mrom/tools/export_wav.py`:
      driver real en Z80 + YM2610 ymfm + V-ROM; 13 músicas + 177 SFX).
- [x] VGM 1.51 (`export_wav.py --vgm`): escrituras YM2610 + bloques ADPCM-A/B.
- [x] Voces (tabla B) exportadas por separado y con nombre por su frase;
      las canciones terminan en su punto de bucle + fundido (no cortadas).
- [ ] Nombrar los 150 SFX de la tabla A (por escucha + usuarios del 68k).

## Herramientas / experimentos

- [ ] Más recetas en `internal/experiments/` (selección de misión, debug
      DIP, invulnerabilidad) siguiendo la regla de "mismo tamaño".
- [x] CI: comprobar también `build_mrom.py` ✔ IDÉNTICA en GitHub Actions.
