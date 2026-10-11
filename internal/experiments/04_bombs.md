# 04 — Bombas (granadas): cantidad inicial e infinitas

**Modelo**: la entidad del jugador (P1 `$100440`, P2 `$1004E0`) guarda las
bombas en el byte `+$80` (`$1004C0` para P1). El HUD lo lee con
`Player_GetBombs_032fce` (`move.b 0x80(a0),d0`). **No confundir con `+$82`**
(word), que es la munición del arma actual: los tres `move.w #0xa,0x82(a6)`
de `air_death_crouch_036632.s` / `core_032aa8.s` recargan la *pistola* con 10
al perder el arma (ahora receta `pistol-ammo=N`). Esa era la causa de que el
antiguo `bombs=N` "no hiciera nada" en el contador BOMB.

| Qué | Dónde | Instrucción |
|---|---|---|
| Valor inicial (10) | `PlayerEntity_InitAuxState_032A02` (`asm/player/entity_init_aux_state_032a02.s`) | `move.b #0xa,0x80(a6)` → `$032A3D` |
| Gasto al lanzar hacia delante | `Player_ThrowGrenade_0332bc` (`asm/player/core_032aa8.s`) | `subi.b #1,0x80(a0)` → `$0332E6` |
| Gasto al lanzar hacia atrás | `Player_ThrowGrenadeBack_033256` | `subi.b #1,0x80(a0)` → `$033280` |
| Recoger item bombas (+N, tope 99) | `Item_GiveBombs_ToPlayer_032c7e` | `add.b 0x98(a6),d0 ; cmpi.w #0x63,d0` |

## Recetas

```bash
bash scripts/apply_patch.sh bombs=99 --zip internal/mslug.zip         # empezar con 99 bombas
bash scripts/apply_patch.sh bombs-infinite --zip internal/mslug.zip   # nunca se gastan
bash scripts/apply_patch.sh bombs=99 bombs-infinite --zip internal/mslug.zip
```

Bytes (mismo tamaño, resto de la ROM idéntico):
* `bombs=99`: `$032A3D: 0A → 63`
* `bombs-infinite`: `$033280: 04→00`, `$033283: 01→00`, `$0332E6: 04→00`, `$0332E9: 01→00`
  (`subi.b #1,0x80(a0)` = `04 28 00 01 00 80` → `ori.b #0,0x80(a0)` = `00 28 00 00 00 80`).

## Verificación en emulador

La granada se lanza con **C** (máscara 64; B=32 es salto). Pulsaciones cortas
(3 frames) en reposo:

```bash
internal/pydmg/target/release/pydmg-neogeo --headless --cart build/repack/mslug.zip \
  --max-frames 4800 --auto-coin-frame 400 --auto-press-start-frame 500 --press-period-frames 300 \
  --input-script "4600-4603:64,4650-4653:64,4700-4703:64" \
  --watch-ram "1004C0:1" --dump-every-frames 25 2>/dev/null | grep WATCH | tail -8
```
Original: `$1004C0` 0A → 09 → 08 → 07 (y el antiguo `bombs=99` solo cambiaba
`$1004C2`=0063, la munición de la pistola). Con `bombs-infinite` se queda en 0A;
con `bombs=99` arranca en 63 (HUD BOMB 99).

Captura (`bombs=99 bombs-infinite`, granada en el aire y BOMB sigue en 99):
`results/bombs_99_infinite_demo.png`.
