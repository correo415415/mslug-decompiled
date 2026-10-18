# Estado del port

## Resumen

| | |
|---|---|
| Programa 68000 traducido a Rust (`crates/mslug-code`, generado) | **96 937 instrucciones** (P-ROM del juego + BIOS) |
| Semillas | traza de 3 partidas (moneda en 0/400/2000, 8000 frames) + vectores + 8 482 entradas del registro de la decompilación |
| Instrucciones no traducibles | 0 |
| `mslug lockstep --frames 6000` (programa de 28 593 instr.) | ✅ 99 245 067 instrucciones idénticas al intérprete (registros, SR, PC, ciclos tras **cada** instrucción; RAM, paleta y ciclos totales tras cada frame); **100 % ejecutado por código Rust**, 0 instrucciones interpretadas |
| Frontend SDL2 (`crates/mslug-sdl`) | ✅ ventana, sonido, P1/P2, capturas; partida jugada por el programa recompilado |
| Rutinas reescritas a mano en Rust idiomático (`crates/mslug/src/game`) | 15 (verificadas con `mslug verify`) |

## Cómo está hecho

1. `mslug dump-roms` vuelca la P-ROM tal como la ve la CPU y la BIOS.
2. `mslug trace` ejecuta el intérprete y anota cada dirección ejecutada.
3. `tools/recomp/m68k_recomp.py` recorre el código desde esas semillas
   (descenso recursivo por saltos/llamadas/ramas) y traduce **cada
   instrucción** a una función Rust especializada: modos de
   direccionamiento, inmediatos, destinos de salto y ciclos se resuelven en
   la generación. La semántica es la del núcleo 68000 de referencia
   (`neogeo-hw/src/cpu/m68k/exec.rs`): flags X N Z V C, orden de accesos al
   bus, ciclos.
4. `mslug_code::Program` despacha por PC a esas funciones. El hardware
   (vídeo, Z80/YM2610, IRQ) avanza con `System::post_cpu`, igual que con el
   intérprete.
5. `mslug lockstep` ejecuta intérprete y programa traducido a la vez y para
   en la primera instrucción que difiera.

## Camino a "0 emulación"

El objetivo final es que no quede ningún intérprete de CPU:

- [x] Programa del juego y BIOS traducidos a Rust (sin decodificar opcodes
      en tiempo de ejecución).
- [x] Verificación instrucción a instrucción (lockstep).
- [ ] 0 direcciones sin traducir en todas las partidas de prueba (`misses`):
      ampliar trazas (todas las misiones, final, sound test, 2 jugadores) y
      semillas; después **quitar el intérprete** de `Program::step`.
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
