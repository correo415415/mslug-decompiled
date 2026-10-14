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
`sheets.json` guarda el orden y el nº de tiles por hoja. Pendiente: aplicar
las paletas del P-ROM y reconstruir sprites completos desde sus tablas.

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
el MD5. Los WAV son vista previa (re-codificar ADPCM es con pérdida).

### Músicas y SFX completos tal como suenan en el juego

`audio.py` da las muestras sueltas. Para las canciones y efectos **mezclados
exactamente como en la placa** (FM + SSG + ADPCM, con los parches,
envolventes, tempo y bucles del driver) usa
`python3 mrom/tools/export_wav.py [--sfx]`, que ejecuta el driver original
sobre un YM2610 emulado (ver `mrom/docs/FORMATS.md` §8).
