# Metal Slug — port 1:1 a Rust

Proyecto independiente (rama `port` del repositorio de la decompilación).
Objetivo: reescribir en Rust **todas** las rutinas del programa 68000 de
Metal Slug (Neo Geo MVS, 1996, P-ROM `201-p1.bin`) de forma que el juego se
comporte **exactamente** igual que el original: mismos frames, mismo audio,
mismas partidas con las mismas entradas.

> El romset MAME original (`mslug.zip`, con la BIOS `neogeo`) está en
> `internal/mslug.zip` mientras el repositorio sea privado. **Borrar
> `internal/` antes de cualquier publicación.** `--rom` permite usar otro.

## Cómo funciona

```
internal/      mslug.zip (romset MAME original, sólo uso privado)
crates/
  neogeo-hw/   hardware Neo Geo: 68000, Z80, YM2610, LSPC, bus, cargador de ROM
               (vendorizado de pydmg-neogeo, MIT/Apache, sin frontend SDL)
               + `native.rs`: tabla de funciones nativas enganchada a la CPU
  mslug/       el juego
    src/game/  rutinas portadas, una función Rust por rutina 68000
    src/verify.rs  verificador diferencial ROM ↔ Rust
```

1. **Hardware emulado, juego nativo.** El Neo Geo (vídeo, sonido Z80/YM2610,
   BIOS) sigue emulado. El programa del juego se sustituye **función a
   función**: cada rutina portada se registra por su dirección de entrada y,
   cuando el PC del 68000 llega ahí, se ejecuta la versión Rust en lugar de
   interpretar la ROM.
2. **Contrato 1:1.** Una función portada debe dejar D0–D7, A0–A7, SR
   (X N Z V C), PC, la RAM y **los ciclos consumidos** idénticos a los de la
   rutina original. Así el resto del programa (y el temporizado de vídeo y
   sonido) no nota la diferencia.
3. **Verificación diferencial.** `mslug verify` juega con la ROM y, en cada
   llamada a una función portada, ejecuta la versión 68000 y la Rust desde
   el mismo estado y compara todo. Cualquier diferencia se informa con el
   primer registro/byte/ciclo distinto.
4. **Nombres y semántica** vienen de la decompilación byte-idéntica (rama
   principal: `asm/`, `tools/registry.py`, `docs/`). Cada función Rust
   lleva el nombre y la dirección de la rutina que sustituye.

A medida que se portan funciones, los llamantes también pasan a Rust; cuando
todo el programa esté portado, la tabla de funciones nativas cubrirá desde el
punto de entrada del juego y la P-ROM sólo se usará como fuente de datos
(gráficos, tablas, guiones), igual que los datos de un port tradicional.

## Uso

```bash
cargo build --release
./target/release/mslug list                         # funciones portadas
./target/release/mslug verify --frames 6000          # ROM vs Rust en cada llamada
./target/release/mslug run --frames 4000 --dump build/port --every 500
./target/release/mslug run --frames 4000 --dump build/rom  --every 500 --rom-only
```

`verify` y `run` insertan moneda y START automáticamente (frame 400) y desde
el frame 1700 juegan solos (derecha, disparo, salto y granadas), igual que
las recetas de la decompilación.

## Estado

Ver [PORT_STATUS.md](PORT_STATUS.md).
