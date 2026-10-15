# assets/ — extracción y reconstrucción de S-ROM, C-ROM y V-ROM

Herramientas (sin datos: los assets derivan de las ROMs y se escriben en
`build/assets/`, ignorado por git). Requieren `numpy` y `Pillow`.

```bash
./scripts/setup_assets.sh            # extrae 201-s1/c1..c4/v1/v2 del zip a rom/
python3 assets/tools/gfx.py extract  # fix + sprites → PNG indexados
python3 assets/tools/gfx.py build    # PNG → 201-s1/c1..c4.bin  ✔ MD5 idéntico
python3 assets/tools/audio.py extract  # V-ROM → muestras con nombre (ADPCM + WAV)
python3 assets/tools/audio.py build    # ADPCM → 201-v1/v2.bin   ✔ MD5 idéntico
```

## Gráficos (`gfx.py`)

| ROM | Formato | Salida |
|---|---|---|
| `201-s1.bin` (128 KiB) | capa fija, tiles 8×8, 4 bpp, 32 B/tile (columnas en orden `10,18,00,08`, nibble bajo = píxel izquierdo) | `build/assets/fix/fix_000000.png` (4 096 tiles, 32×32 por hoja) |
| `201-c1..c4.bin` (4×4 MiB) | sprites 16×16, 4 bpp; pares c1/c2 y c3/c4 (impar = planos 0-1, par = planos 2-3); 4 bloques 8×8 en orden derecha-arriba, derecha-abajo, izquierda-arriba, izquierda-abajo | `build/assets/sprites/bank{0,1}/spr*_XXXXXX.png` (2 × 65 536 tiles, 64×64 por hoja; el nombre es el nº del primer tile) |

Los PNG son de paleta con los **índices de color crudos** (rampa de grises);
editarlos sin cambiar de modo de paleta y reconstruir da la ROM exacta.
`sheets.json` guarda el orden y el nº de tiles por hoja.

## Sprites tal como los monta el juego (`sprites.py`)

`python3 assets/tools/sprites.py extract` (≈ 45 s) lee el P-ROM
(`build/mslug_prom.bin`) y los C-ROM y escribe en `build/assets/sprites/`:

| Carpeta | Contenido |
|---|---|
| `palettes/` | las 1 536 paletas (`$14E00`: 1 024, `$1CE00`: 512) en PNG + JSON |
| `frames/` | 8 364 frames multi-pieza `frame_XXXXXX.png` (RGBA) + `frames.json` (piezas, tiles, origen) |
| `anims/` | ~980 animaciones `<Rutina>_<script>.gif` (tiempos reales a 59,185 Hz) + tira `.png`, `anims.json` e `index.html` (galería) |

Formatos (verificados contra el código):

* **Paleta**: 16 × `{r, g, b, 0}` de 5 bits; color 0 transparente. Las cargan
  `Entity_AllocSpriteSlot_00236E` (`move.w #pal,d1; jsr $236E`, 750 sitios),
  `PalSlot_LoadList_002a7c` y `PalSlot_LoadListHi_002b58` (banco `$1CE00`).
* **Frame** (lo recorre `SpriteDispatchJT_05AA96`): `u8 npiezas, u8 attr`
  (banco de tiles << 4); cada pieza `s16 dx, s16 dy` (dy hacia **arriba**),
  `u8 w, u8 h` y `w·h` tiles de 16 bits **por columnas**; nº de tile =
  banco << 16 | tile.
* **Script de animación** (`Script_OpcodeTable_028cf0`, puntero en `$3C(a6)`):
  `00 n` espera, `01 .l` salto, `02 fl .l prio.w [dx.w dy.w]` imagen (fl bit0
  flip H, bit1 flip V, bit3 = sin dx/dy → 8 B en vez de 12; varios `02`
  seguidos forman una imagen), `16` repite, `03 n .l` bucle, `1B/1C` índice
  en tabla, `1D` retrocede, `1E` hijo; el resto de opcodes tienen tamaño fijo.

Cómo se encuentran y nombran: se validan todos los `02` del P-ROM; los
scripts son destinos de punteros o cabezas de secuencias de `02`; los
alcanzados desde otro se omiten y las poses sueltas contiguas (tablas `1B/1C`
de ángulos) se agrupan en un `..._Poses_XXXXXX`. El nombre es la rutina que
referencia el script (directamente con `lea`/`move.l #`, o la que referencia
la tabla de punteros que lo contiene); la paleta, la del `jsr $236E` más
cercano en esa rutina o su Init vecina, o heredada de otra animación que
comparta frames (`pal_how` en `anims.json`). Las que siguen sin paleta salen
en grises; se pueden forzar en `assets/map/anim_palettes.txt`.

`python3 assets/tools/sprites.py render 2BCBC4 36 --scale 3` dibuja una sola
animación (`build/assets/render_<script>_p<pal>.gif`).

## Fondos de las misiones (`levels.py`)

`python3 assets/tools/levels.py` (≈ 5 s) → `build/assets/levels/`: 52 capas
de fondo únicas de 13 escenas (las 6 misiones, attract, game over,
cutscenes) en PNG RGBA con sus paletas, `levels.json` e `index.html`.

Los fondos son capas de sprites que mueven los 4 contextos de cámara
(`$106F6C`, `$107FE8`, `$108064`, `$1080E0` → `cam0..cam3`; cada uno con su
parallax). El script de cada escena (`SceneDescTable_0916C8`, VM
`SceneScriptVM_Frame_0437DA`) las enlaza con el op `$00 bind_path`:

    00 00  ent.l  w.w h.w tiles.l  colisión.l  ancla_x.w ancla_y.w

* `{w, h, tiles}` es el descriptor que lee `CellCommit_MMIO_051DE2`: `w`
  columnas de `h` celdas **por columnas**, cada celda `{tile.w, attr.w}` en
  formato SCB1 (attr = paleta << 8 | tile_msb << 4 | autoanim << 2 | flipV << 1 |
  flipH), copiada tal cual a la VRAM según avanza el scroll (ventana de
  32×32 celdas).
* La paleta es un slot dinámico: el op `$0A` de la escena da los pares
  `{slot, banco}` que `PalSlot_LoadListHi_002b58` carga del banco `$1CE00`.
* `colisión` apunta al mapa de colisión de la capa (o 0) y el ancla es su
  posición en el mundo.

Los slots que la escena no define se toman de otras escenas y, en último
término, de `ScriptSlotPairTable_0009B4` (slots globales $00–$0F que fija el
arranque/attract; p. ej. slot 0 → banco $193). Si aun así falta alguno, la
celda sale en grises y se lista en `missing_palette_slots` (hoy: ninguna capa).

## Sonido (`audio.py`)

`201-v1.bin + 201-v2.bin` forman un espacio ADPCM de 8 MiB. Los límites y
nombres de cada muestra salen de los instrumentos del M-ROM
(`mrom/sound/instruments.s`, `PCMA_SAMPLE` / `PCMB_SAMPLE`):

* 321 muestras ADPCM-A (`pcma/PcmA_<tabla>_<n>.adpcma` + `.wav` a 18 500 Hz);
* 54 muestras ADPCM-B (`pcmb/PcmB_<desc>.adpcmb` + `.wav` a la frecuencia de
  su delta-N);
* 8 huecos no referenciados (`gaps/`, 1 MiB; el último, `0x717B00-0x7FFFFF`, es
  relleno `FF`).

`index.csv` lista offset, tamaño, tipo, nombre, frecuencia, instrumentos que
la usan y fichero. `build` concatena los ficheros crudos en orden y comprueba
el MD5. Los WAV son vista previa; para sustituir una muestra:

    python3 assets/tools/audio.py encode a nuevo.wav build/assets/audio/pcma/PcmA_XXXXX_NN.adpcma
    python3 assets/tools/audio.py encode b nuevo.wav build/assets/audio/pcmb/PcmB_XXXXX.adpcmb --rate HZ
    python3 assets/tools/audio.py build

El codificador (mono, re-muestreo lineal; ADPCM-A a 18 500 Hz por defecto)
elige en cada muestra el nibble cuyo valor *decodificado* con la aritmética
exacta del YM2610 queda más cerca, así que no acumula error; en ADPCM-B
desempata los casos de saturación mirando 2 muestras por delante. Volver a
codificar un WAV extraído reproduce los 321 ADPCM-A byte a byte y 41/54
ADPCM-B (el resto a 33–49 dB SNR). Rellena con silencio hasta un múltiplo de
256 B; si la muestra cambia de tamaño hay que ajustar sus límites en
`mrom/sound/instruments.s` (`PCMA_SAMPLE` / `PCMB_SAMPLE`) y el índice.

### Músicas y SFX completos tal como suenan en el juego

`audio.py` da las muestras sueltas. Para las canciones y efectos **mezclados
exactamente como en la placa** (FM + SSG + ADPCM, con los parches,
envolventes, tempo y bucles del driver) usa
`python3 mrom/tools/export_wav.py [--sfx]`, que ejecuta el driver original
sobre un YM2610 emulado (ver `mrom/docs/FORMATS.md` §8).
