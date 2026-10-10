# TODO

Lista viva de tareas pendientes del proyecto. Las tareas se mueven a
`CHANGELOG.md` / `docs/PROGRESO.md` cuando se completan.

## P-ROM (68000) — 100 % matching

- [ ] Cabeceras A–F / nombres semánticos para los 13 módulos de datos
      (`asm/data/*`), igual que el resto de módulos.
- [ ] Revisar que el matcher completo (`tools/match_batch.py`) siga en
      17608/17608 tras cada reorganización.

## M-ROM (Z80, `201-m1.bin`) — en curso

- [ ] Separar código y datos en la región fija con bloques de `z80dasm -b`
      (tablas de punteros `0080-00C8`, `57A1-67A1`, tablas de salto
      `2076`, `4AA0/4ABC/4AD8`, `53A3`; datos `0100-1DFF`, `5800-7FFF`).
- [ ] Fichero de símbolos (`-S`) con nombres semánticos para el núcleo del
      driver: `Reset_0000`, `YM_Write_RST8`, `IRQ_Timer_1E7A`,
      `NMI_Command_1ED6`, `Main_1E00`, `BankSelect_1F7D`, `BankPtr_1FA6`,
      `CmdDispatch_2050`, motor de secuencias `2515-57A1`.
- [ ] Equates de hardware/RAM en `mrom/include/` (puertos YM2610, latch
      68k, ventanas de banco, mapa de RAM `F800-FFFF`).
- [ ] Trocear el núcleo en `mrom/asm/<subsistema>/` con `include`,
      manteniendo `build_mrom.py` ✔ IDÉNTICA en cada paso.
- [ ] Analizar los bancos 2..7 (secuencias musicales, instrumentos FM,
      tablas ADPCM-A/B) y documentar el formato de secuencia.
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
- [ ] CI: comprobar también `build_mrom.py` ✔ IDÉNTICA en GitHub Actions.
