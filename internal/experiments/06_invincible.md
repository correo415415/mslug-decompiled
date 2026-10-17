# 06 — Invulnerabilidad (jugador y Slug)

**Origen**: el propio juego trae un modo invulnerable de depuración: el bit 3 del
byte de flags de debug `$100001` (word `$100000`, que `UserMode0_080C` pone a 0
al arrancar). Dos rutinas lo consultan:

| Rutina | Qué hace con el bit 3 |
|---|---|
| `Probe_Bit3At100001_0334A2` (`asm/player/route_publish_027eba.s`) | llamada por `PlayerRoute_PublishState` y otros 2 sitios cuando el jugador recibe un golpe: con el bit a 1 borra los bits 3/0 de `$13(a6)` (marca de muerte), pone `$66(a6)=1` y vuelve sin matar; con el bit a 0 salta a `Player_DeathGate_0334c6` |
| `Slug_DamageTick_02a6be` (`asm/slug/helpers_0295a6.s`) | con el bit a 1 salta a `Slug_ResetHP_02a712` en vez de restar vida al Slug |

**Cambio** (mismo tamaño):
* `beq.w Player_DeathGate_0334c6` (`67 00 00 1A`, 4 B) → `nop ; nop` (`4E 71 4E 71`).
* `bne.w Slug_ResetHP_02a712` (`66 00 ..`, 4 B) → `bra.w` (`60 00 ..`).

```bash
bash scripts/apply_patch.sh invincible --zip internal/mslug.zip
```
Bytes: `$02A6D4: 66→60`, `$0334AA..AD: 67 00 00 1A → 4E 71 4E 71`.

**Verificación** (emulador, `--input-script "3400-12000:8"`: el jugador avanza
a la derecha sin disparar en la misión 1): con la ROM original el HUD pasa de
`1UP=2` a `1UP=1` (le matan); con el parche sigue `1UP=2` en el frame 9000 y
los soldados le atacan sin efecto. Captura (izq. original, dcha. parche):
`results/invincible_demo.png`.

Alternativa sin tocar código: `debug=0008` (receta 07) activa el mismo bit.
