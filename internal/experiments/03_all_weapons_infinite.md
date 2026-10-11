# 03 — Todas las armas especiales infinitas

Misma técnica que 02 aplicada a las cuatro rutinas de disparo:

| Función | `subi.w #1,$82(a2)` en |
|---|---|
| `PlayerFire_HMG_Spawn_03c872`     | `$03C898` |
| `PlayerFire_Shotgun_Spawn_03c9c4` | `$03C9EA` |
| `PlayerFire_Rocket_Spawn_03cb16`  | `$03CB3C` |
| `PlayerFire_Flame_Spawn_03ccd2`   | `$03CCF8` |

```bash
bash scripts/apply_patch.sh all-infinite --zip internal/mslug.zip
```
(La pistola, `PlayerFire_Pistol_Spawn_03c710`, también decrementa `$82` pero
el HUD y la lógica la tratan como infinita porque `$71`=0.)
