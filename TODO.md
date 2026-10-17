# TODO

Lista viva de tareas pendientes del proyecto. Las tareas se mueven a
`CHANGELOG.md` / `docs/PROGRESO.md` cuando se completan.

## P-ROM (68000) — 100 % matching

- [x] Cabeceras A–F en los 14 módulos de datos (`tools/gen_data_headers.py`,
      12 generadas + scene_scripts/mission_streams ya manuales) y 7661
      entradas renombradas por tipo (`ColMap_`, `SprFrame_`, `BgMap_`,
      `AnimScr_`, `Pal_`, `Zero_`). Tabla de color `ColorRGB555ToNeoGeo_002f30`
      y rampa `ColorRamp_012fb0` identificadas y verificadas.
- [ ] Decodificar las ~1 400 `Data_*` restantes (tablas indexadas por rutina).
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
- [x] Nombres por usuarios para 68 de los 117 SFX de la tabla A con usuario
      constante (`SfxA_NNN_<Nombre>`, SFX_NAMES de find_sound_ids.py).
- [ ] Confirmar esos nombres escuchando, nombrar los 49 restantes y recorrer
      los streams de script para los ids que no son constantes.
- [x] `docs/COVERAGE.md` (M-ROM, módulos de datos, assets), `docs/PROGRESO.md`,
      `HANDOFF.md` y `README.md` actualizados (2026-10-15).

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
- [x] Fondos: composición por cámara en las anclas (`composed/`, 38 planos).
- [x] Fondos: parallax (escala 8.8 del op $01) → panorama único por escena (`*_parallax.png`, 13).
- [ ] Fondos: scroll vertical en el panorama.
- [x] Codificador ADPCM-A/B para sustituir muestras desde WAV (`audio.py encode`).
- [x] Exportar las pistas musicales a MIDI con sus nombres
      (`mrom/tools/export_midi.py`). La curva de `S_TEMPO` ya está modelada (set-tempo exactos).
- [x] Exportar todo el audio a WAV idéntico al juego (`mrom/tools/export_wav.py`:
      driver real en Z80 + YM2610 ymfm + V-ROM; 13 músicas + 177 SFX).
- [x] VGM 1.51 (`export_wav.py --vgm`): escrituras YM2610 + bloques ADPCM-A/B.
- [x] Voces (tabla B) exportadas por separado y con nombre por su frase;
      las canciones terminan en su punto de bucle + fundido (no cortadas).
- [ ] Nombrar los 150 SFX de la tabla A: 68 hechos por usuarios (ver M-ROM); faltan escucha y los ids que sólo salen en los scripts.

## Herramientas / experimentos

- [ ] Más recetas en `internal/experiments/` (selección de misión, debug
      DIP, invulnerabilidad) siguiendo la regla de "mismo tamaño".
- [x] CI: comprobar también `build_mrom.py` ✔ IDÉNTICA en GitHub Actions.

- [x] levels.py: renderizar también los 11 descriptores de parches de
  Sprite_InvokeBlit8Params (data_census.blit_patches) como capas.
- [x] Tablas de Tbl_Decode2D_0799DE (230, `decode2d_tables`) como struct.
- [x] Registros de Spawner_Handler_077c98 (18 B) como struct.
- [x] Soft-DIP NEO-GEO (JP/US/EU) y TaskBootTab_178000 con nombre y punteros simbólicos.
- [x] Textos a máquina de la presentación de misión (FixTypePages_JP/EN/PT/ES + 84 streams).
- [x] PatchDescChain_19810c + tilemaps sin descriptor ($0AAF78, $0B2FD0...).
- [x] SpawnStream_M1..M6 (streams de spawn por misión) + AnimScr_188414.
- [x] Tablas indexadas de scripts (code_script_tables) y jmp de cola en los escáneres.
- [x] Scripts apuntados por cualquier long (pointed_scripts).
- [x] LUTs de puntería Bazooka/RocketVehicle + 319 registros de blit (blit_records).
- [x] Tablas del motor de colisión (CollTileTypes/CollMasks/CollSlopes/CollOffsets, HitShapeTab, HudText) con nombre y 33 lea simbólicos.
- [x] Tablas de punteros a datos ya clasificados (pointer_tables, 2 pasadas).
- [x] Op 1E (SpawnChild_Sprite) recorrido como 02 en walk_struct.
- [x] sprites.py: op 1E no corta las animaciones exportadas
- [x] Pasada de rename_data_by_census tras el op 1E (81 AnimScr_)
- [x] Demos grabadas del attract (InputDemo_*) y PathScripts (VM de movimiento +$90) en el censo
- [x] Tablas de RocketVehicle/AimTurret, Decode2D indirectas y frames de op 1E en el censo
- [x] Texto de Cut_TextPanel, cadenas $FF, blits sobre PatchDescChain
- [x] Escáneres hasta $09C608, tablas del para-squad, scripts sin puntero en huecos
- [x] Decode2D del jefe M5, tablas de muerte/despliegue del escuadrón, listas de animación del fix
- [x] Decode2D vía aN y variantes contiguas, tablas de Bazooka, jefe M5 y escuadrón
- [x] Rangos de resultados, jefes 2/4/5, torreta, motion_ptr_tables
- [x] Mortero, popups de puntuación, pinza del cangrejo, coche torreta, scripts de 1 frame
- [x] Islas de código del banco alto, listas de hitbox, gameover/continue, hiscore, pasos del jefe final, Patrol
- [x] Decode2D normal/espejo y registros op 19 sin referencia
- [x] Listas de ataque +$4C, parches de Prop_BlitSequence, rejillas indirectas, Tank, proyectil del jefe final
- [x] Tablas indexadas, spawners sin referencia, pools de nombres completos, variantes Decode2D hacia atrás
- [x] Tablas indexadas por add.l/adda y tras bra.w
- [x] Descriptores de escena sueltos y huecos en punteros exactos del código
- [x] Listas de ataque del cangrejo, semillas de hitbox lea+move.l, tablas de formas contiguas
- [x] Rejillas contiguas, scripts que saltan a otros conocidos, tablas de punteros a +$48/+$4C
- [x] Spawners contiguos/sueltos y parámetros de scripts de Player
- [x] Censo: referencias de op 07 SetLong y tablas de saltos a código
- [x] Censo: huecos apuntados desde datos, colas de registro, mapa de colisión de cámara en $20A000, colas de rampa, continuación periódica; islas ChildRank_CmpByte10 fuera de los módulos de datos
- [x] Censo: terminadores $FF sueltos, cabecera de Prop_BoatDebris, $FFFF final y marca $12345678 en $2FF800 — 0 bytes otro
- [x] Datos "otro": 0 bytes no nulos sin clasificar (tools/dev/other_runs.py -> total 0).
- [ ] Revisar a mano Data_196000 / AnimScr_1d6700 y los MissionStream con bytes sin decodificar (ya clasificados, formato interno pendiente).
