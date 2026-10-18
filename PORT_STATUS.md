# Estado del port

## Resumen

| | |
|---|---|
| Fuente del código | **la decompilación** (`tools/recomp/decomp_code.bin`, exportado con `tools/export_code_map.py` del repo de la decompilación): 101 142 instrucciones, 0 discrepancias de longitud con la ROM |
| Programa 68000 traducido a Rust (`crates/gen/NN`, generado) | **115 189 instrucciones**: P-ROM 101 168 + BIOS 14 021 |
| Trazas / ejecuciones usadas para generar | **ninguna** (`m68k_recomp.py --static`) |
| Instrucciones no traducibles | 0 en la P-ROM (1 en la BIOS, nunca alcanzada) |
| `mslug lockstep --frames 6000` | ✅ 99 245 067 instrucciones idénticas al intérprete (registros, SR, PC, ciclos tras **cada** instrucción); 100 % código Rust |
| `mslug play --fuzz 1..3` (25 000 frames, entradas aleatorias, M1–M3) | ✅ 0 instrucciones interpretadas, 0 direcciones sin traducir |
| Frontend SDL2 (`crates/mslug-sdl`) | ✅ ventana, sonido, P1/P2, capturas |
| CI (`.github/workflows/port.yml`, runner local) | build + lockstep + play + SDL headless; binarios, logs y capturas en Releases `port-*` |

## Recompilación estática "pura" desde la decompilación

Pregunta: ¿se puede generar el port leyendo sólo la decompilación, sin
ejecutar el juego para descubrir qué bytes son código?

Investigación (oct. 2026):

1. **El mapa código/datos ya está en la decompilación.** Cada fuente del
   registro se ensambla con listado (`-Wa,-aln`); una línea que emite bytes
   es instrucción si su fuente es un mnemónico y dato si es `.dc/.byte/.fill/
   .rept/...`. Con la dirección de cada sección (`tools/registry.py`) sale la
   dirección exacta de las 101 142 instrucciones del juego. `--check`
   decodifica cada una con capstone en la ROM: 0 diferencias.
2. **Saltos indirectos.** El 68000 del juego usa tablas de punteros
   (`jmp (a0)`, máquinas de estado `(a6)`, tablas `$B92`...). La
   decompilación ya decodificó como código los destinos de esas tablas, así
   que no hace falta resolverlos: se traducen **todas** las instrucciones de
   la decompilación, alcanzables o no.
3. **Comparación con el método por trazas.** Las 1 670 direcciones que las
   partidas habían encontrado "a mano" (`seeds.txt`) están todas en el mapa
   de la decompilación. La única diferencia real fue **un error de la
   decompilación**: 38 bytes en `$4B052` (`HumanDeath`, estado alcanzado sólo
   por un puntero de tabla) estaban como `.dc.w`. Se corrigió **en la
   decompilación** (sigue IDÉNTICA) y el port lo hereda. Una búsqueda de
   punteros a zonas de datos que decodifiquen como rutina no encontró más
   casos.
4. **Instrucciones solapadas** (22 destinos de salto que caen dentro de un
   `.short` de opcode literal, p. ej. `bne.w` codificado a mano en
   `$024E60`) se siguen igualmente por descenso recursivo desde el código
   decompilado.
5. **BIOS.** No está decompilada. Sus entradas se sacan también sin ejecutar:
   tabla de llamadas `$C00402` (SDK), vectores, todo puntero alineado a la
   BIOS en BIOS/P-ROM, código tras tablas de saltos en línea y barrido tras
   cada `rts/jmp/bra`. Cubre todo lo que las partidas ejecutaron.

Conclusión: **sí**. El port se genera de forma puramente estática a partir de
la decompilación (`m68k_recomp.py --static`); ejecutar el juego queda sólo
como **verificación** (lockstep, fuzz, CI), no como fuente. El método con
trazas (`--registry-seeds` + `seeds.txt`) se mantiene como alternativa.

## Cómo está hecho

1. Decompilación: `python3 tools/export_code_map.py --check` →
   `build/code_map/code.bin` (se copia a `tools/recomp/decomp_code.bin`).
2. `mslug dump-roms` vuelca la P-ROM tal como la ve la CPU y la BIOS (es la
   ROM; no se ejecuta nada).
3. `tools/recomp/m68k_recomp.py --static` traduce **cada instrucción** a
   una función Rust especializada: modos de direccionamiento, inmediatos,
   destinos de salto y ciclos se resuelven en la generación. La semántica es
   la del núcleo 68000 de referencia (`neogeo-hw/src/cpu/m68k/exec.rs`):
   flags X N Z V C, orden de accesos al bus, ciclos.
4. `mslug_code::Program` despacha por PC a esas funciones. El hardware
   (vídeo, Z80/YM2610, IRQ) avanza con `System::post_cpu`.
5. Verificación: `mslug lockstep` (intérprete y programa traducido a la vez,
   para en la primera instrucción que difiera) y `mslug play --fuzz N`.

## Camino a "0 emulación"

El objetivo final es que no quede ningún intérprete de CPU:

- [x] Programa del juego y BIOS traducidos a Rust (sin decodificar opcodes
      en tiempo de ejecución).
- [x] Verificación instrucción a instrucción (lockstep).
- [x] Código sacado de la decompilación, sin trazas (`--static`).
- [x] 0 direcciones sin traducir en lockstep y partidas fuzz (M1–M3).
- [ ] **Quitar el intérprete** de `Program::step` (fallo duro en un miss).
- [ ] Juntar instrucciones en bloques básicos / funciones (Rust por función
      de la decompilación, no por instrucción) y nombrarlas con los símbolos
      de `tools/recomp/symbols.tsv`.
- [ ] Sustituir funciones generadas por reescrituras idiomáticas
      (`crates/mslug/src/game`), verificadas con `mslug verify`.
- [ ] Hardware: el vídeo (LSPC → sprites/fix), el sonido (Z80 + YM2610) y la
      BIOS son ahora el "sistema" del port. El programa Z80 del driver de
      sonido se traducirá con el mismo método (recompilador Z80) para que
      tampoco quede intérprete del Z80.

## Funciones reescritas a mano

| Dirección | Rutina | Módulo |
|---|---|---|
| $001E56 | Store106EDA | kernel |
| $05E9E4 | RNG_LFSRStep (generador aleatorio) | kernel |
| $043EDA | Buffer_ClearBlock1024L | kernel |
| $044048 | Coord_ApplyCameraDeltaToSelf | entity |
| $04405E | Coord_ApplyCameraDeltaToA0Mark | entity |
| $04407A | Coord_ApplyCameraTerciaryToSelf | entity |
| $044090 | Coord_ApplyCameraDeltaToGlobals | entity |
| $0440AA | Coord_ApplyCameraDeltaToD1D2 | entity |
| $0440BC | Coord_LocalToScreenSecondary | entity |
| $0440D0 | Coord_ScreenToLocalSecondary | entity |
| $065C64 | Hostage_SinCos | entity |
| $02A2AC | Slug_GetGauge | entity |
| $05180C | Clear8Bytes | entity |
| $0516BA | Players_SnapshotMods | entity |
| $09773C | HiScore_LoadDefaults | hiscore |
