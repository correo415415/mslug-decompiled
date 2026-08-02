# Metal Slug 1 — Cobertura real de la ROM

**Ultimo update:** 2026-07-26  (Wave MM batch 3 cerrada + escaner corregido)

Este documento complementa `docs/PROGRESO.md` con el analisis **real** de cobertura
de codigo, no la metrica bruta del matcher que compara contra los 2 MiB de la
P-ROM (que incluye ~1.6 MiB de datos que **no** son "codigo a decompilar").

Se regenera con `tools/measure_coverage.py` (script anadido en la misma
tanda que este documento).

---

## Los tres porcentajes que hay que distinguir

| Metrica | Cifra (MM#3) | Que mide realmente |
|---|---:|---|
| **`ROM total`** (`match_batch.py`) | **1.94 %**  (40 716 / 2 097 152 B) | Bytes registrados vs P-ROM completa. Es la metrica del matcher pero es enganosa: incluye 1.6 MiB de datos/graficos/padding que no son "codigo a decompilar". |
| **`Codigo real estimado`** (heuristica) | **7.32 %**  (27 886 / 380 928 B) | Bytes de **codigo ejecutable ya decompilado** vs total de codigo ejecutable estimado en la ROM (entropia media + densidad de opcodes 68000 validos). Es la metrica util de progreso. |
| **`Nucleo del juego`** (categorias clave) | **CORE-DATA 100 %, CORE 63 %, ATTRACT 18 %** | Cobertura ponderada por zonas semanticas del juego. |

---

## Composicion de la P-ROM (heuristica por bloques de 4 KiB)

Clasificacion basada en entropia + densidad de opcodes 68000 validos +
fraccion ASCII + fraccion de bytes cero:

| Categoria | Bytes | % ROM | Descripcion |
|---|---:|---:|---|
| **CODE?**    |   380 928 B | **18.2 %** | Codigo ejecutable estimado (5.0 < H < 7.6, opcode density > 15 %) |
| DATA-MID     | 1 081 344 B | 51.6 % | Paletas, mapas de tiles, scripts de nivel, secuencias de sprites |
| DATA-LO      |   352 256 B | 16.8 % | Tablas escasas (LUTs de fases, coords, timing) |
| ZERO         |   245 760 B | 11.7 % | Padding entre bancos (`$0A0000..$0BFFFF` casi todo vacio) |
| ASCII        |    36 864 B |  1.8 % | Strings de menu/config, dip switches, texto de misiones |
| **TOTAL**    | 2 097 152 B | 100 % | |

**Conclusion:** el trabajo real de decompilacion se centra en los **~372 KiB
de codigo ejecutable estimado**. El resto (1.6 MiB) son datos que se
registraran como `.long`/`.byte` arrays cuando sea necesario para el matching
byte-a-byte, pero no son "logica a decompilar".

---

## Mapa semantico por zonas

### Zonas categorizadas del ROM

| Zona | Rango | Total | Cubierto | % zona | Contenido |
|---|---|---:|---:|---:|---|
| Vectors 68000 + Neo-Geo header | `$000000..$000400` | 1024 B | 0 B | 0 % | Vector table 68000 + cabecera cartucho Neo-Geo (`"NEO-GEO"` en `$100`). No se decompila. |
| BIOS entry + IRQ/VBlank tick | `$000400..$000C00` | 2048 B | 1472 B | **72 %** | Waves P (BIOS entries), Q (IRQ handlers), R (Scheduler central) |
| **Bootstrap dispatch table** | `$000B92..$000E90` | 766 B | 766 B | **100 %** | La super-tabla de arranque (Wave MM#2) |
| **Scheduler bootstrap + handlers** | `$000E8E..$001300` | 1138 B | 1130 B | **99 %** | Waves MM#1 y MM#3 |
| Attract mode + title screen | `$001300..$002000` | 3328 B | 1290 B | **39 %** | Waves EE, FF, GG |
| Input + range guards + helpers | `$002000..$003000` | 4096 B | 1968 B | 48 % | Waves A, D, E, F, H, I |
| Task/entity/collision runtime | `$003000..$005000` | 8192 B | 0 B | 0 % | Motor de tareas + actualizacion entidades + colisiones |
| Player/enemy state machines | `$005000..$006000` | 4096 B | 264 B | 6 % | State machines de player y enemigos |
| Runtime helpers + probes | `$006000..$009000` | 12288 B | 1234 B | 10 % | Helpers de runtime, fisica, probes |
| Attract handlers (waves FF/GG) | `$009000..$00A000` | 4096 B | 720 B | 18 % | Handlers restantes del attract mode |
| (vacio / padding) | `$00A000..$0C0000` | 1.4 MiB | 0 B | 0 % | Padding entre bancos + gaps |
| Cluster runtime avanzado | `$0C0000..$0D0000` | 64 KiB | 58 B | 0.1 % | Sub-rutinas de gameplay (llamadas BIOS `$C004C2`) |
| (vacio / padding) | `$0D0000..$130000` | 384 KiB | 0 B | 0 % | |
| Level-specific handlers | `$130000..$140000` | 64 KiB | 14 B | 0 % | Logica especifica por Mission (5 misiones) |
| (datos + tablas + fin) | `$140000..$200000` | 768 KiB | 0 B | 0 % | Datos gaming: paletas, mapas, scripts, headers |

### Cobertura por categoria

| CATEGORIA | Total | Cubierto | % categoria |
|---|---:|---:|---:|
| **CORE-DATA** | 766 B | 766 B | **100 %** |
| **CORE** | 8 192 B | 5 145 B | **62.8 %** |
| **ATTRACT** | 7 424 B | 2 010 B | **27.1 %** |
| **RUNTIME** | 20 480 B | 1 234 B | 6.0 % |
| **GAMEPLAY** | 69 632 B | 322 B | 0.5 % |
| **LEVEL** | 65 536 B | 14 B | 0.0 % |
| SYSTEM | 1 024 B | 0 B | 0 % |
| PAD | 1 835 008 B | 0 B | (no aplica) |
| DATA | 786 432 B | 0 B | (no aplica) |

---

## Que "partes del juego" estan decompiladas

### Vistas en pantalla que ya podemos reconstruir

- **Arranque BIOS completo** (5 modos: reset, mode-2, title, demo, hardstart)
  - `SchedulerBootstrap_Boot_000E8E` con selector por `$10FDAE`/`$10FDAF`
  - Interpreta la super-tabla `$000B92` (186 handlers) como bytecode virtual
- **Bucle principal (main loop)**
  - `SchedulerLoopA_000FC6` + `SchedulerDispatch_LoopB_000FE0`
  - Threaded continuation-passing con auto-avance sobre centinelas
- **Sistema de tareas** (parcial)
  - `Task_FreeListInit`, `Task_Alloc`, `SetTaskW/B/Handler` (Waves D, E, H)
- **Sistema de sprites** (parcial)
  - Asignador de 381 sprites hardware, blit primitives, Fix Layer backends
    (Waves W, II)
- **Sistema de camara** completo
  - Aplicacion por handler + smoothing + hooks Probe08/82/F6
    (Waves HH, JJ, KK)
- **Sistema de colision** (parcial)
  - `Collision_ProbeRange/X/Y` + `CellApply_BidirScan` + `CellCommit_MMIO`
    (Waves KK, LL)
- **Attract mode** (parcial)
  - Titulo "METAL SLUG", handlers de escenas attract, `Attract_InitBIOS`
    (Waves EE, FF, GG, MM)
- **Sistema de estados** (parcial)
  - Dispatcher grande de `$051914`, state publishers per-entity,
    maquina F1->F6 (Waves AA, BB, GG)
- **Input mask event dispatchers**
  - Los 53 handlers de `$5CDFC..$5D1D9` (Wave U)
- **Pubcleaner + `Pubcleaner_10A2Cx`** (Wave LL)
- **Super-tabla dispatch del BIOS** (Wave MM#2, la 1a entrada datos-en-.text)

### Cosas que faltan (por prioridad y tamano)

1. **Logica de gameplay por Mission** (Missions 1-5)
   - Todo el bloque `$130000..$140000` (64 KiB) sin tocar
   - Aqui vive el scripting especifico de cada nivel
2. **State machines de player y enemigos**
   - `$005000..$006000` (4 KiB) + partes de `$083000..$092000` (60 KiB)
   - Comportamiento de Marco/Tarma vs enemigos
3. **Motor de fisica/colision completo**
   - Hay probes matcheados pero falta el runtime que los ata
   - `$003000..$005000` + `$006000..$009000` (~20 KiB)
4. **Sistema de armas**
   - Pistola, heavy machine gun, granadas, bazooka, prisioneros
   - Sin identificar aun
5. **Sonido/musica bridge**
   - El Z80 vive en `201-m1.bin` (aparte)
   - El bridge M68K<->Z80 (comunicacion por `$300000`) sigue sin decompilarse
6. **Sistema de particulas y explosiones**
   - Probablemente en `$083000..$092000`
7. **Menu de configuracion (soft-dip)**
   - Strings en `$1781D0`: HERO/CONTINUE/DIFFICULTY/PLAY TIME/DEMO SOUND/
     PLAY MANUAL/BLOOD/LANGUAGE ENGLISH/PORTUGUESE/SPANISH/1UP=...

---

## Ranking de proxima prioridad (escaner corregido)

Top 5 zonas pendientes por cantidad de referencias entrantes desde codigo ya
matcheado (`tools/scan_unmatched_callees.py` con filtros aplicados,
`--min-addr 0x400 --min-size 20`):

| # | Zona 8 KiB | Aristas entrantes | Contexto probable |
|---:|---|---:|---|
| 1 | **`$08E000..$08FFFF`** | 26 | Sub-rutinas de gameplay (post-Wave GG#2 anim state machine `$08Cxxx`) |
| 2 | **`$032000..$033FFF`** | 24 | Contiene el **Top-1 del scan** (`$033522`, 18 callers reales) |
| 3 | **`$028000..$029FFF`** | 14 | Cluster entity setters (post-Waves A, S) |
| 4 | `$000000..$001FFF` | 14 | Falsos positivos por bytes que coinciden con opcodes -- IGNORAR |
| 5 | **`$02A000..$02BFFF`** | 11 | Probe/collision cluster (post-Wave T#7-T#15) |

### Top-1 individual del scan

```
[  1] $033522  callers=18  ~28B
  033522: jsr     $334a2(pc)
  033526: bcc.w   $33570
  03352a: jsr     $27eba.l
  033530: bcs.w   $3353e
  033534: lea.l   $33572(pc), a1
  033538: move.l  a1, (a6)
  03353a: bra.w   $33544
```

Es un **helper micro** que enlaza probablemente el sistema de entidades con el
de scripts (patron `jsr; bcc.w; jsr; bcs.w; lea.l XXX(pc), a1; move.l a1, (a6);
bra.w` = "publica siguiente handler y salta"). Con 18 callers es el candidato
natural para Wave NN.

---

## Como regenerar este documento

```bash
# Requisitos: capstone + python3
python3 tools/measure_coverage.py > docs/COVERAGE.md
```

El script mide:

1. **Composicion del ROM** (heuristica CODE?/DATA-*/ZERO/ASCII por bloques 4 KiB)
2. **Cobertura del registry** (cuanto de cada categoria semantica esta ya
   registrado)
3. **Ranking de zonas pendientes** (usando el escaner corregido)

Se ejecuta cada vez que se cierra una oleada para tener un pulso continuo del
proyecto mas alla de la cifra del matcher.
