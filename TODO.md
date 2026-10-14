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
- [ ] Documentar la estructura de canal (offsets `+00..+4C` SSG, `+67` FM,
      `+29` ADPCM-A, `+1F` ADPCM-B) en `mrom/docs/FORMATS.md`.
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
- [ ] Assets, siguiente nivel: aplicar paletas del P-ROM a los sprites,
      reconstruir sprites completos (animaciones) desde las tablas de sprites
      del P-ROM, nombres por personaje/arma/enemigo; codificador ADPCM para
      sustituir muestras desde WAV.
- [x] Exportar las pistas musicales a MIDI con sus nombres
      (`mrom/tools/export_midi.py`). Pendiente: modelar la curva de `S_TEMPO`
      y exportar VGM (requiere emular el YM2610 con los parches).

## Herramientas / experimentos

- [ ] Más recetas en `internal/experiments/` (selección de misión, debug
      DIP, invulnerabilidad) siguiendo la regla de "mismo tamaño".
- [x] CI: comprobar también `build_mrom.py` ✔ IDÉNTICA en GitHub Actions.
