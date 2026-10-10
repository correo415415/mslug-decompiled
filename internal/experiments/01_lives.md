# 01 — Vidas iniciales fijas

**Dónde**: `asm/hud_inputrec_entity_move_024fb8.s`, funciones `HUD_Task_Init_025706`
(y la copia dentro de `PlayerSlotDesc_P2Rows_025588`, offset +$128). Leen el
soft-DIP de vidas `$10FD88` (`lea $10FD84,a3 ; move.b 4(a3),d1 ; move.b d1,$77(a6)`).

**Cambio** (4 B → 4 B): `move.b 0x4(a3),d1` (`1A 2B 00 04`) → `move.b #N,d1` (`1A 3C 00 N`).

```bash
bash scripts/apply_patch.sh lives=9 --zip internal/mslug.zip
```
Bytes: `$0256B7: 2B→3C`, `$0256B9: 04→09`, `$02570D: 2B→3C`, `$02570F: 04→09`.

**Verificación**: el HUD muestra reservas (vidas−1): original `1UP=2`, parche `1UP=8`.
Captura: `docs/images/lives_patch_demo.png`.

Variante manual con sed:
```bash
sed -i 's/move\.b  0x4(a3),d1 /move.b  #0x9,d1    /' asm/hud_inputrec_entity_move_024fb8.s
bash scripts/rebuild_rom.sh --mod internal/mslug.zip
```
