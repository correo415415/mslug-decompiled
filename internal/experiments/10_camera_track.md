# 10 — Trayectoria real de la cámara (pydmg, 2026-10-18)

Receta: `mission=N invincible bombs-infinite` + `tools/dev/emu_input.py` (derecha + disparo alterno + salto) + granada cada 90 frames, `--watch-ram "106F50:4,106F54:4,106ECE:1"`, volcado cada 30–60 frames; `python3 tools/dev/camera_log.py build/camN/watch.txt ESCENA 128`.

* `$106F50` = X de cámara 16.16, `$106F54` = Y 16.16, `$106ECE` = escena (M1→0, M2→1, M3→3, M4→2, M5→4, M6→5).
* **Corrección para `assets/tools/levels.py`**: al renderizar una ventana con las capas en (X, Y) de la RAM, el mejor ajuste píxel a píxel contra la captura es **(X−4…−8, Y+16)** en 5 frames de M1 (Y = $90, $55, $20): la pantalla visible empieza 16 líneas por debajo de la Y de cámara. `camera_track` estimaba Y = 192 → 80 en M1; la real es 144 → 32 (= estimación − 48) y el ascenso empieza en X≈3200, no ≈3500.
* M2, M3 y M6 (segunda pasada, guion con izquierda/derecha alternas, granadas cada 4 pulsos y salto):
  * **M3** (escena 03, scroll vertical: X=0, Y $430 → $3B0, sube) y luego pasa a la **escena 04** (tramo horizontal Y=0 hasta X≥1309).
  * **M6** (escena 05): Y=0, X 0 → 994; el avance se bloquea en X≈$102 hasta matar la oleada (frame 8700).
  * **M2** (escena 01): bloqueada en X=$32, Y=$140 por el soldado del escudo (no se le vence con este guion).
* Las trayectorias se guardan en `assets/tools/camera_tracks.json`; `levels.py` las usa (con Y + 16) para `*_scroll.png` de las escenas 00, 02, 04 y 05 (`track_source` en levels.json) y recurre a `camera_track` (estimada) en el resto.

## M1 (escena 00)
```
escena 00  x=    0 ($0000)  y=   0 ($0000)  frame 0
escena 00  x=    0 ($0000)  y= 128 ($0080)  frame 1740
escena 00  x=  227 ($00E3)  y= 144 ($0090)  frame 2100
escena 00  x=  434 ($01B2)  y= 144 ($0090)  frame 2220
escena 00  x=  564 ($0234)  y= 144 ($0090)  frame 2280
escena 00  x=  784 ($0310)  y= 144 ($0090)  frame 2400
escena 00  x= 1003 ($03EB)  y= 144 ($0090)  frame 2520
escena 00  x= 1225 ($04C9)  y= 144 ($0090)  frame 2640
escena 00  x= 1427 ($0593)  y= 144 ($0090)  frame 2760
escena 00  x= 1648 ($0670)  y= 144 ($0090)  frame 5520
escena 00  x= 1808 ($0710)  y= 144 ($0090)  frame 5580
escena 00  x= 1965 ($07AD)  y= 144 ($0090)  frame 5700
escena 00  x= 2194 ($0892)  y= 144 ($0090)  frame 5820
escena 00  x= 2420 ($0974)  y= 144 ($0090)  frame 5940
escena 00  x= 2608 ($0A30)  y= 144 ($0090)  frame 6060
escena 00  x= 2828 ($0B0C)  y= 144 ($0090)  frame 6480
escena 00  x= 3084 ($0C0C)  y= 144 ($0090)  frame 6660
escena 00  x= 3318 ($0CF6)  y= 114 ($0072)  frame 6780
escena 00  x= 3557 ($0DE5)  y=  54 ($0036)  frame 6900
escena 00  x= 3815 ($0EE7)  y=  32 ($0020)  frame 7020
```
## M4 (escena 02)
```
escena 02  x=    0 ($0000)  y=   0 ($0000)  frame 1710
escena 02  x=  168 ($00A8)  y=  10 ($000A)  frame 2010
escena 02  x=  320 ($0140)  y=  22 ($0016)  frame 2280
escena 02  x=  465 ($01D1)  y=  10 ($000A)  frame 2400
escena 02  x=  630 ($0276)  y=   8 ($0008)  frame 2490
escena 02  x=  790 ($0316)  y=   8 ($0008)  frame 2580
escena 02  x=  960 ($03C0)  y=   8 ($0008)  frame 2670
escena 02  x= 1144 ($0478)  y=   8 ($0008)  frame 3150
escena 02  x= 1284 ($0504)  y=  18 ($0012)  frame 3210
escena 02  x= 1463 ($05B7)  y=   8 ($0008)  frame 3300
escena 02  x= 1613 ($064D)  y=  19 ($0013)  frame 3390
escena 02  x= 1743 ($06CF)  y=  10 ($000A)  frame 3480
escena 02  x= 1932 ($078C)  y=   8 ($0008)  frame 3600
escena 02  x= 2101 ($0835)  y=  25 ($0019)  frame 3690
escena 02  x= 2249 ($08C9)  y=   8 ($0008)  frame 3780
escena 02  x= 2411 ($096B)  y=   8 ($0008)  frame 3870
escena 02  x= 2549 ($09F5)  y=   8 ($0008)  frame 3990
escena 02  x= 2725 ($0AA5)  y=  14 ($000E)  frame 4230
escena 02  x= 2902 ($0B56)  y=   8 ($0008)  frame 4380
escena 02  x= 3046 ($0BE6)  y=  11 ($000B)  frame 4530
escena 02  x= 3200 ($0C80)  y=   8 ($0008)  frame 4680
escena 02  x= 3341 ($0D0D)  y=   8 ($0008)  frame 4800
escena 02  x= 3476 ($0D94)  y=   8 ($0008)  frame 4920
escena 02  x= 3605 ($0E15)  y=   8 ($0008)  frame 5010
escena 02  x= 3744 ($0EA0)  y=   8 ($0008)  frame 5130
```
## M5 (escena 04)
```
escena 04  x=    0 ($0000)  y=   0 ($0000)  frame 1710
escena 04  x=  173 ($00AD)  y=   0 ($0000)  frame 1980
escena 04  x=  335 ($014F)  y=   0 ($0000)  frame 2070
escena 04  x=  501 ($01F5)  y=   0 ($0000)  frame 2160
escena 04  x=  675 ($02A3)  y=   0 ($0000)  frame 2250
escena 04  x=  844 ($034C)  y=   0 ($0000)  frame 2370
escena 04  x= 1021 ($03FD)  y=   0 ($0000)  frame 2460
escena 04  x= 1189 ($04A5)  y=   0 ($0000)  frame 2550
escena 04  x= 1367 ($0557)  y=   0 ($0000)  frame 2640
escena 04  x= 1541 ($0605)  y=   0 ($0000)  frame 2730
escena 04  x= 1686 ($0696)  y=   0 ($0000)  frame 2820
escena 04  x= 1829 ($0725)  y=   0 ($0000)  frame 2910
escena 04  x= 1983 ($07BF)  y=   0 ($0000)  frame 3000
escena 04  x= 2153 ($0869)  y=   0 ($0000)  frame 3090
escena 04  x= 2319 ($090F)  y=   0 ($0000)  frame 3180
escena 04  x= 2484 ($09B4)  y=   0 ($0000)  frame 3270
escena 04  x= 2643 ($0A53)  y=   0 ($0000)  frame 3360
escena 04  x= 2833 ($0B11)  y=   0 ($0000)  frame 3450
escena 04  x= 3017 ($0BC9)  y=   0 ($0000)  frame 4140
escena 04  x= 3192 ($0C78)  y=   0 ($0000)  frame 4230
escena 04  x= 3320 ($0CF8)  y=   0 ($0000)  frame 5070
escena 04  x= 3485 ($0D9D)  y=   0 ($0000)  frame 5160
escena 04  x= 3645 ($0E3D)  y=   0 ($0000)  frame 5250
escena 04  x= 3807 ($0EDF)  y=   0 ($0000)  frame 5340
escena 04  x= 3967 ($0F7F)  y=   0 ($0000)  frame 5430
```
