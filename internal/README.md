# internal/ — material de desarrollo (NO para distribución)

> Esta carpeta existe solo mientras el proyecto es privado. Contiene el romset
> original y un emulador para probar las ROMs reconstruidas. **Borrar antes de
> cualquier versión pública.**

| Ruta | Qué es |
|---|---|
| `mslug.zip` | romset MAME original (`201-p1.bin` MD5 `b6804bc6…`) |
| `pydmg/` | emulador Neo Geo en Rust (`pydmg-neogeo`) + parche de dev-tools (`scripts/dev/pydmg_devtools.patch`): `--input-script`, `--watch-ram`, `--poke-ram` |
| `experiments/` | recetario de modificaciones: comandos listos para copiar, qué bytes tocan, cómo verificarlas |
| `experiments/results/` | capturas / logs de verificación |

## Puesta en marcha
```bash
bash scripts/dev/setup_internal.sh internal/mslug.zip /ruta/pydmg.zip   # compila el emulador (cargo + SDL2)
bash scripts/rebuild_rom.sh internal/mslug.zip                          # ROM desde fuentes → build/repack/mslug.zip
bash scripts/dev/run_emu.sh build/repack/mslug.zip 4800 /tmp/frames      # headless, PNG cada 200 frames
```
Con ventana (en un PC con SDL2): `internal/pydmg/target/release/pydmg-neogeo --cart build/repack/mslug.zip --windowed --ui-scale 3`
