# 02 — Heavy Machine Gun con munición infinita

**Modelo**: la entidad del jugador (P1 en `$100440`, P2 en `$1004E0`) guarda
el tipo de arma en `+$71` (0 pistola, 1 HMG, 2 shotgun, 3 rocket, 4 flame) y
la munición en `+$82` (word). `Player_GetAmmoOrFFFF_032fba` devuelve `$FFFF`
(∞ en el HUD) si el arma es 0. Tabla de carga: `Player_WeaponAmmoTbl_0329d4`
(HMG 200 = `$00C8`, shotgun 10, rocket 15, flame 20 … pares `{max?, carga}`).

**Dónde se gasta**: cada `PlayerFire_*_Spawn_03cXXX` hace
`cmpi.w #0,$82(a2) ; beq ; subi.w #1,$82(a2)`.

**Cambio** (6 B → 6 B): en `PlayerFire_HMG_Spawn_03c872`,
`subi.w #1,0x82(a2)` (`04 6A 00 01 00 82`) → `ori.w #0,0x82(a2)` (`00 6A 00 00 00 82`), un no-op.

```bash
bash scripts/apply_patch.sh hmg-infinite --zip internal/mslug.zip
```
Bytes: `$03C898: 04→00`, `$03C89B: 01→00`.

**Verificación en emulador** (dar HMG por poke y disparar con A):
```bash
internal/pydmg/target/release/pydmg-neogeo --headless --cart build/repack/mslug.zip \
  --max-frames 5000 --auto-coin-frame 400 --auto-press-start-frame 500 --press-period-frames 300 \
  --poke-ram "4400@1004B1=01,4400@1004C2=00C8" \
  --input-script "4420-4428:16,4440-4448:16,4460-4468:16,4480-4488:16,4500-4700:16" \
  --watch-ram "1004B1:1,1004C2:2" --dump-frames-dir /tmp/f --dump-every-frames 100 2>/dev/null | grep WATCH
```
Original: `$1004C2` 00C8 → 00C6 → 00C5. Parche: se queda en 00C8. HUD: ARMS 197 vs 200.
Captura: `docs/images/hmg_infinite_demo.png`.
