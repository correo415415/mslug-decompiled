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
- [ ] Nombrar las ~460 etiquetas internas restantes (cuerpos de actualización
      de cada canal SSG/FM/ADPCM) y documentar la estructura de canal
      (offsets `+00..+4C/+67/+29`) y de pista (`+00..+22`).
- [x] Documentar el formato de secuencia (opcodes `C0..D9`, bytes de nota) y
      el de las cabeceras de música/SFX → `mrom/docs/FORMATS.md`.
- [x] Bancos 2..7 (solo datos): tablas, cabeceras y secuencias decodificadas a
      fuentes simbólicas `mrom/sound/` (`gen_sound_data.py`), ✔ IDÉNTICA.
- [x] Formato de instrumentos (SSG 24 B / FM 50 B / ADPCM-A / ADPCM-B + tablas
      de samples) → `mrom/sound/instruments.s` con macros `INSTR_*` (incluida
      la parte de la región fija `0x57A1-0x7FFF`, vía `incbin`).
- [ ] Nombres reales de músicas/SFX (ids enviados desde el P-ROM).
- [ ] Actualizar `mrom/README.md`, `docs/COVERAGE.md` (sección M-ROM),
      `docs/PROGRESO.md` y `CHANGELOG.md` a medida que avance.

## Al tener el M-ROM al 100 %

- [ ] **Scripts de extracción de assets para S-ROM, C-ROM y V-ROM**:
      - `201-s1.bin` (S-ROM, fix layer 8x8): descodificar los tiles y
        exportarlos a PNG (hoja de tiles + tiles sueltos).
      - `201-c1..c4.bin` (C-ROM, sprites 16x16): desentrelazar los pares
        c1/c2 y c3/c4, descodificar los tiles de 4 bpp, aplicar las
        paletas del P-ROM y exportarlos a PNG (hojas de tiles y, cuando
        sea posible, sprites completos reconstruidos desde las tablas de
        sprites del P-ROM).
      - `201-v1.bin`, `201-v2.bin` (V-ROM, ADPCM-A/ADPCM-B): usar las
        tablas de muestras del M-ROM (inicio/fin/frecuencia) para cortar
        cada muestra, descodificar ADPCM → PCM y exportarlas a WAV.
      - Dar a cada asset el **nombre correspondiente** (personaje, arma,
        enemigo, efecto de sonido, pista musical…) usando los nombres
        semánticos ya establecidos en el P-ROM y el M-ROM; generar un
        índice (CSV/Markdown) con offset, tamaño, nombre y vista previa.
      - Herramienta inversa (PNG/WAV → ROM) para poder reconstruir los
        ROMs de gráficos y sonido byte-idénticos, igual que
        `tools/build_rom.py` y `mrom/tools/build_mrom.py`.
- [ ] Exportar las pistas musicales (secuencias del M-ROM) a un formato
      reproducible (VGM/MIDI) con sus nombres.

## Herramientas / experimentos

- [ ] Más recetas en `internal/experiments/` (selección de misión, debug
      DIP, invulnerabilidad) siguiendo la regla de "mismo tamaño".
- [x] CI: comprobar también `build_mrom.py` ✔ IDÉNTICA en GitHub Actions.
