# M-ROM — formatos de datos de sonido

Formato de los datos que el driver Z80 de Metal Slug 1 lee de la mitad en
bancos de `201-m1.bin` (fichero `0x08000-0x1FFFF`) y de las tablas de la región
fija relacionadas. Todo lo descrito aquí está expresado en las fuentes de
`mrom/sound/` mediante las macros de `mrom/sound/macros.inc`; las fuentes se
regeneran con `python3 mrom/tools/gen_sound_data.py` y `build_mrom.py`
comprueba que reensamblan byte a byte.

Marcas: **(✔)** comprobado en el código del driver; **(?)** hipótesis.

## 1. Punteros de 24 bits

Todas las tablas usan **direcciones lineales de fichero** de 3 bytes (`lo, hi,
banco`) seguidas de un byte de relleno `00` (4 bytes en total).
`Bank_ResolvePtr24_1FA6` obtiene banco = `addr >> 14` y desplazamiento en la
ventana = `(addr & 0x3FFF) | 0x8000`, y cambia de banco leyendo los puertos
`0B..08` (✔). Macro: `PTR24 etiqueta` (`PTR24 0` = hueco vacío).

`BankPtrTable_0080` (región fija, 18 entradas):

| Índice | Destino | Uso |
|---|---|---|
| 0 | `InstrPtrTable_57A1` | instrumentos (§5) |
| 1 | `MusicTable` `0x0815F` | 13 músicas (✔ `Cmd_PlayMusic_20D4`) |
| 2 | `SfxTableA` `0x08472` | SFX tipo `10h` (✔ `Cmd_PlaySfx_2115`) |
| 3 | `SfxTableB` `0x087F2` | SFX tipo `11h` |
| 4..17 | `SfxTableA` | tipos `12h..1Fh` (alias de la tabla A) |

## 2. Comandos del 68000

El 68k escribe en `REG_SOUND`; el Z80 lo lee por el puerto `0` en la NMI. Ver
`mrom/README.md`. Un byte `≥ 20h` reproduce el sonido `cmd − 20h` de la tabla
seleccionada por el último byte de tipo enviado (`< 10h`: música si es `00`;
`10h..1Fh`: SFX). El id de 16 bits de un sonido es `cmd | tipo << 8`
(`wSfxId_F81B` / `wCmdType_F81C`), y es el valor que usan las listas de
«detener» de las cabeceras de SFX.

## 3. Cabeceras (`mrom/sound/headers.s`)

Tablas y cabeceras están contiguas en `0x0815F-0x0953C`, en este orden:
`MusicTable`, cabeceras de música, `SfxTableA`, `SfxTableB`, cabeceras de SFX.

### Música — `Seq_StartMusic_4FA3` (✔)

```
MUSIC_HDR tempo          ; 1 byte: índice en NoteMapTable_4B84 → tempo inicial (FDE7/FDE8/FDEF)
TRACK seq, canal         ; hasta 14 (wMusicTracks_FBFC, 14 × 0x23 B)
...
TRACKS_END               ; puntero nulo (4 B)
```

### SFX — `Seq_StartSfx_507B` (✔)

```
SFX_HDR id0, id1, id2, id3, prioridad   ; 4 ids (u16) que se detienen antes (Seq_StopTrackById_5210)
TRACK seq, canal                        ; hasta 8 (wSfxTracks_FDF2)
...
TRACKS_END
```

`TRACK` = `PTR24` + 1 byte que se guarda en `+02` de la pista. En SFX es el
valor que `Seq_FindFreeTrack_50DB` compara para elegir / expulsar una pista
(prioridad de canal); en música vale `0` salvo pistas marcadas `255` (?
pistas opcionales / de efecto).

## 4. Secuencias (`mrom/sound/music_NN.s`, `sfx.s`)

Byte-code interpretado por `Seq_TrackFetch_5362` sobre la estructura de pista
(0x23 B). Un único flujo contiguo `0x0953D-0x1D286`: cada punto de entrada y
cada destino de salto cae en un límite de instrucción (comprobado por
`gen_sound_data.py`). 31 650 instrucciones, 328 entradas, 1 123 destinos.

### Eventos de nota (`00..BF`) — `Seq_NoteEvent_56EA` (✔)

```
op = (delta + 11) << 3 | máscara      ; 00..B7, delta = −11..+11 respecto a la nota actual (+17)
op = B8 | máscara, nota               ; B8..BF, nota absoluta
máscara bit0/1/2 = 1 → se omite len / vol / gate (se mantiene el valor de +18 / +19 / +1A)
```

Le siguen, en orden, los bytes no omitidos: duración (`+18`, también espera
`+16`), volumen (`+19`; `0` = silencio sin key-on) y gate (`+1A`). La nota
actual vale `0x40` (len `C0`, vol `C0`, gate `7F`) al iniciar la pista y tras
`C2`, `C4`, `C5`, `CF` y un `D0` que salta. Las macros `N nota[, len[, vol[,
gate]]]` y `NX …` escriben la nota **absoluta** y calculan el delta (un
argumento vacío = byte omitido).

### Opcodes de control (`C0..D9`) — `SeqOpJumpTable_53A3`

| Op | Macro | Args | Efecto | |
|---|---|---|---|---|
| C0 | `S_INSTR n` | 1 | instrumento `+0A` | ✔ |
| C1 | `S_INSTRG n, g` | 2 | instrumento `+0A` y grupo `+0B` | ✔ |
| C2 | `S_JUMP L` | rel16 | salto (desplazamiento desde el final del campo) | ✔ |
| C3 | `S_KEYMODE v` | 1 | `+09` bit 0 = `v≠0` (no re-key si la nota sigue sonando) | ? |
| C4 | `S_CALL L` | rel16 | llamada (pila de 3 niveles en `+1C..+21`, puntero `+22`) | ✔ |
| C5 | `S_RET` | — | retorno | ✔ |
| C6 | `S_PMOD v` | 1 | `+0C` (profundidad envolvente de tono, `Env_Update_231E`) | ✔/? |
| C7 | `S_VMOD v` | 1 | `+0D` (profundidad envolvente de volumen) | ✔/? |
| C8 | `S_PARAM v` | 1 | `+0E` (parámetro de `Pitch_Update_2461`) | ? |
| C9 | `S_SUSTAIN v` | 1 | `+09` bit 4 | ✔ |
| CA | `S_DETUNE w, t` | 3 | `+10/+11` = `w·8` (signo extendido), espera `t` | ✔ |
| CB | `S_WAIT n` | 1 | silencio de `n` ticks (`+16`) | ✔ |
| CC | `S_PAN v` | 1 | `+0F` = 3 (`40h` centro), 2 (`<40h`), 1 (`>40h`): bits L/R del YM | ✔ |
| CD | `S_TEMPO a, b` | 2 | `Seq_SetTempo_4F39` | ✔ |
| CE | `S_TRKVOL v` | 1 | atenuación de pista `+12` | ✔ |
| CF | `S_LOOPBEGIN` | — | abre un nivel de bucle (contador a 0) | ✔ |
| D0 | `S_LOOPEND n, L` | 1+rel16 | repite desde `L` `n` veces | ✔ |
| D1 | `S_PORTA v` | 1 | portamento on (`≥40h`, `+09` bit 6) / off | ✔ |
| D2 | `S_LFO v` | 1 | `+13` (índice de vibrato en la página `1Axx`) | ✔ |
| D3..D6 | `S_D3..S_D6 a, t` | 2 | `Ch_SetNote{,2,3,4}`: acción directa sobre el canal según tipo | ? |
| D7 | `S_PITCHDN a, t` | 2 | `+10/+11` −= a (saturado) | ✔ |
| D8 | `S_PITCHUP a, t` | 2 | `+10/+11` += a (saturado) | ✔ |
| D9 | `S_REPLY c` | 1 | `out (0Ch)` = `(c&3F)·2+80h` \| bit de alternancia: aviso al 68k | ✔ |
| DA | — | — | (tabla) `Seq_SetVolumeLevel_4F88`; **no** se alcanza: `≥DA` termina la pista | ✔ |

Los bytes `≥ DA` paran la pista; este ROM no los usa (las pistas acaban en
`S_JUMP` en bucle o en `S_RET` de la llamada inicial).

### Secuencias sin referencia

`seq_unused.s`: tres secuencias tras el último SFX que ninguna cabecera
apunta (restos de autoría). `leftover.s`: `0x1D287-0x1FFFF`, bytes que el
driver no lee (parece basura gráfica / datos de otra herramienta); se
conservan tal cual.

## 5. Instrumentos (`mrom/sound/instruments.s`)

`InstrPtrTable_57A1`: 1 024 `PTR24` (8 grupos × 128); índice =
`grupo·128 + nº` desde `+0B/+0A` de la pista (`C0 nº` / `C1 nº,grupo`;
`Instr_ResolvePtr_4B5D`; grupo `FF` → `InstrDefault_2509`, 3 punteros a
parches de la página `19xx`). Apuntan a **105 parches distintos** en
`0x067A1-0x0815E`; junto con las tablas de samples que estos referencian son
**174 registros contiguos sin huecos** (verificado por `gen_sound_data.py`).
La parte `< 0x08000` vive en la región fija: `mrom/asm/data/instruments_57a1.s`
la incluye con `incbin` del binario que `build_mrom.py` ensambla antes desde
`mrom/sound/`.

El **byte 0 es el tipo de canal**; al dar nota con un instrumento nuevo
(`sub_4a03h` → `l4a8c`) se llama al cargador `ChTypeJumpTable0_4AA0[tipo]`
con `hl` = parche (en el banco ya mapeado):

| Tipo | Macro | Tamaño | Cargador | Contenido |
|---|---|---|---|---|
| 0 | `INSTR_SSG` | 24 B | `l2aa8h` | ruido, mezclador, `I_ENV`, `I_LFO`×2, `I_PITCH` |
| 1 | `INSTR_FM` | 50 B | `l3568h` | 5 B de canal, 4 × `FM_OP` (7 B), `I_LFO`×2, `I_PITCH` |
| 2 | `INSTR_PCMA` | 17 B + mapa | `l3f41h` / nota `l3f60h` | `I_ENV`, `PCMA_MAP` (→ `PcmaSampleTab`, nota base), 1 B por nota |
| 3 | `INSTR_PCMB` | 22 B + 6·n + 1 | `l427fh` / nota `l42efh` | `I_ENV`, `I_LFO`×2, `I_PITCH`, rangos `PCMB_KEY`, `PCMB_END` (`FF`) |

Sub-bloques comunes:

* `I_ENV a,d,s,r,x` (5 B, `Seq_LoadOpPtrs_219B`): tasas de la envolvente de
  volumen; cada byte ≠0 indexa las tablas de tasas `1Bxx/1Cxx` (0 = instantáneo);
  el 3.º (sustain) se guarda complementado. (Orden A/D/S/R: hipótesis por el uso
  en `Env_Step_222D`.)
* `I_LFO` (6 B, `Lfo_Load_22A8`): `+0` bit 7 activo, bits 0-6 retardo; `+1`
  velocidad; `+2` profundidad; `+3`/`+5` forma (con signo); `+4` índice en las
  tablas `14xx/15xx/16xx`. El primero modula volumen, el segundo tono.
* `I_PITCH` (4 B, `Pitch_Load_242E`): `+0` bit 0 activo, bits 4-7 modo; `+1`
  velocidad; `+2` ajuste fino; `+3` desplazamiento de nota.

FM (`INSTR_FM slots,fbalg,lfo,amspms,transp`): `slots` bits 4-7 = máscara de
operadores del key-on (reg `28h`); `fbalg` → reg `B0h` (bit 7 forzado);
`lfo` → reg `22h` si bit 3 (LFO global, `wFM_Shared_FAA3`); `amspms` → reg
`B4h` (bits 6-7 si no hay LFO); `transp` → canal `+11`. `FM_OP` sigue el orden
de registros del YM2610: DT/MUL `30h`, TL `40h`, KS/AR `50h`, AM/DR `60h`, SR
`70h`, SL/RR `80h`, SSG-EG `90h` (el TL se reescribe luego con el volumen).

ADPCM-A: la nota menos `base` indexa el mapa; ese byte ·5 indexa la
`PcmaSampleTab` (`PCMA_SAMPLE vol, inicio, fin`, en unidades de 256 B de la
V-ROM; se copian 5 B al canal `+15`). Los 6 bytes iniciales de `PCMA_MAP` no se
leen (son 0 salvo en `InstrPCMA_06FCF`).

ADPCM-B: `PCMB_KEY techo, raíz, sample` — se elige el primer rango cuyo
`techo ≥ nota`; `PcmbSample` (12 B, `l432bh`): `delta-N` base, pan/modo,
nº de bucles (`FF` = sin bucle), inicio y tres longitudes (ataque, bucle,
cola) en unidades de 256 B.

## 6. Estructura de pista (0x23 B; `wMusicTracks_FBFC` ×14, `wSfxTracks_FDF2` ×8)

| Off | Campo | Off | Campo |
|---|---|---|---|
| `00/01` | puntero al canal asignado | `12` | atenuación (CE) |
| `02` | canal / prioridad (TRACK) | `13` | LFO (D2) |
| `03` | prioridad de cabecera SFX | `14/15` | puntero de secuencia |
| `04` | banco de la secuencia | `16` | espera |
| `05/06` | id de sonido | `17` | nota actual |
| `07/08` | contador de comandos al iniciar | `18/19/1A` | len / vol / gate actuales |
| `09` | flags (b0 C3, b1 música, b2 activa, b3 libre, b4 sustain, b5 sonando, b6 portamento) | `1B` | última nota (portamento) |
| `0A/0B` | instrumento / grupo | `1C..21` | pila de llamadas / bucles |
| `0C/0D/0E` | C6 / C7 / C8 | `22` | puntero de pila (6 = vacía) |
| `0F` | pan (CC) | `10/11` | desafinación / pitch bend |
