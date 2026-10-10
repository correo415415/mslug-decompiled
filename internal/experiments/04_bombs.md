# 04 — Bombas (granadas) iniciales

`move.w #0xa,0x82(a6)` aparece 3 veces: `asm/player/air_death_crouch_036632.s`
(2 respawns) y `asm/player/core_032aa8.s` (init). Ojo: aquí `a6` es la entidad
*del jugador*, cuyo `+$82`… es el mismo campo que la munición del arma en `a2`
del punto 02: el juego usa `+$82` como "munición del arma actual" y `+$80`
(byte) como bombas (`Player_GetBombs_032fce`: `move.b 0x80(a0),d0`).
Estos 3 `move.w #10` recargan el arma al reaparecer (10 = valor de la pistola/HMG
inicial de respawn). Para bombas iniciales el valor está en `Player_WeaponAmmoTbl`
/ en `Player_DecBombs_032fd4` (`subq.b #1,0x80(a0)`): sustituir por `ori.b #0,0x80(a0)`
(6 B? no: `subq.b` son 4 B → usar `tst.b 0x80(a0)` 4 B, no-op) da bombas infinitas.

```bash
bash scripts/apply_patch.sh bombs=99 --zip internal/mslug.zip   # cambia los 3 move.w #10 → #99
```
Pendiente de verificar en emulador qué contador del HUD cambia (BOMB vs ARMS).
