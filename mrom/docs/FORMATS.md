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
máscara bit0/1/2 = 1 → se omite wait / dur / vel (se mantiene el valor de +18 / +19 / +1A)
```

Le siguen, en orden, los bytes no omitidos:

* `wait` (`+18`, copiado a `+16`): ticks hasta el siguiente evento de la pista;
* `dur` (`+19` → `wNoteVol_FBF7` → canal `+05`): ticks de key-on; el canal lo
  decrementa cada tick y hace release al llegar a 0 (`SSG_Update_L26F6`);
  `0` = silencio (no hay key-on);
* `vel` (`+1A` → `wNoteGate_FBF8` → canal `+0B`): nivel/velocidad usado en el
  cálculo de volumen del canal (`0` = mudo).

(Los nombres de RAM `wNoteVol`/`wNoteGate` son históricos; la semántica
correcta es la de arriba.) La nota actual vale `0x40` (wait `C0`, dur `C0`, vel
`7F`) al iniciar la pista y tras `C2`, `C4`, `C5`, `CF` y un `D0` que salta. Las
macros `N nota[, wait[, dur[, vel]]]` y `NX …` escriben la nota **absoluta** y
calculan el delta (un argumento vacío = byte omitido).

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

## 7. Ids de sonido usados por el 68000 (`mrom/map/sound_ids.txt`)

`mrom/tools/find_sound_ids.py` recorre el P-ROM (`build/mslug_prom.bin`)
buscando llamadas a `Sound_PushCommandFiltered_0021a6`, `ClampD0ToRange`
(`$219C`), `InputGuardCall219c` (`$2352`) y `Sound_Push0C_ThenClamp` (`$2222`)
precedidas de `moveq/move.w #id,d0`, más 9 tablas de ids indexadas en tiempo
de ejecución. Resultado: 419 llamadas con constante y **149 ids distintos**
(13 músicas, 117 SFX de la tabla A y 19 de la tabla B), cada uno con las
funciones que lo usan. `gen_sound_data.py` lee ese fichero, de modo que las
fuentes de sonido no dependen del P-ROM al construir.

Músicas (`MusicTable[id − 20h]`) — nombres por evidencia (hipótesis donde hay
más de un uso):

| id | Nombre | Evidencia |
|---|---|---|
| `20` | `Music_MissionComplete` | `MissionComplete_Task`, `BannerComplete_Boot` |
| `21..26` | `Music_Mission1..6` | `MissionMusicTable_28DF00[$106ECF]` (misiones 7/8 reutilizan la 1) |
| `27` | `Music_AttractDemo` | `Attract_InitShow27_TaskAdd` |
| `28` | `Music_Boss` | intro del jefe M2, `M5Tank`, `FinalBoss`, `ScriptedProp_Run` |
| `29` | `Music_SceneB` | `SceneB_Init` (secuencia de la misión `$0B`) |
| `2A` | `Music_SceneC` | `SceneC_Init` (misión `$0C`) |
| `2B` | `Music_Ending` | `Ending_SpawnOrbitFx`, `Anim_State_F1` |
| `2C` | `Music_Title` | `LogoScene_Tpl` (escena `$0D`) |

Tabla B = **voces digitalizadas** (27). Nombre por usuario del 68k y, donde no
hay usuario con constante, por escucha del render exacto (`export_wav.py`):

| id | Nombre | Frase / evidencia |
|---|---|---|
| `1121` | `Voice_FoodPickup` | `Item_Food_Pickup` (0,7 s; escucha no concluyente) |
| `1123` | `Voice_PlayerDeath` | grito; `Player_Death_PlayMusic`, `Player_DeathPit` |
| `1124` | `Voice_Okay` | «Okay!»; ítems de arma / bombas |
| `1126..112B` | `Voice_Bingo`, `Voice_LetsGo`, `Voice_HereWeGo`, `Voice_Yeah`, `Voice_YeeHaw`, `Voice_PowerUp` | por escucha |
| `112F/112C/112E/112D` | `Voice_Shotgun`, `Voice_FlameShot`, `Voice_RocketLauncher`, `Voice_HeavyMachineGun` | `Player_WeaponMusicTable_032d28[1..4]` = S, F, R, H |
| `1131..1136` | `Voice_Mission1Start..6Start` | `MissionStartVoiceTable_28DF10` |
| `1137` | `Voice_MissionComplete` | `BannerComplete_Boot` |
| `1138` | `Voice_GameOver` | «Game over» |
| `113A` | `Voice_PowThankYou` | «Thank you!»; `Pow_Rescued` |
| `113C/113D/113E` | `Voice_AllenComeOnBoy`, `Voice_AllenGoHomeToMommy`, `Voice_AllenMincemeat` | `Allen_TauntVoiceTbl_0509d6` |
| `113F` | `Voice_AllenLaugh` | «Ha ha ha» |
| `1140` | `Voice_AllenSeeYouInHell` | «See you in hell!»; `Allen_Death` |

Los SFX de la tabla A conservan `SfxA_NNN` y en la etiqueta de su cabecera
figura la lista de usuarios. No se detectan los ids que solo llegan por los streams de
script (`ScriptOp04_Sound`, `PathScript_OpMusic`, `MiniScript_Step`,
`SceneScriptVM_Frame`) ni los calculados.

## 8. Temporización, exportación a MIDI y a WAV

* Tick del driver = Timer A del YM2610 con valor `(ACh << 2) | 3 = 691`
  (`YM_InitTimers_1E6B`). El contador del Timer A avanza una vez por muestra
  FM (144 ciclos de 8 MHz): `144 · (1024 − 691) / 8 MHz = 5,994 ms`
  (≈ **166,8 Hz**). Medido ejecutando el driver real (`sndrender`: 166,8 IRQ/s).
  *(La primera versión de este documento usaba 72 ciclos → 333,7 Hz: era el
  doble de rápido.)*
* Pistas de música: cada tick `FDF0 += tempo + 129`; avanzan un paso por cada
  acarreo al byte alto → `(tempo + 129) / 256` pasos por tick, con `tempo` =
  `NoteMapTable_4B84[byte de cabecera]` con signo (p. ej. Mission1: `1Ch` →
  ≈ 51 pasos/s). `CD` cambia el tempo de forma gradual mediante
  `Seq_ScaleByCurve_4F69` (no modelado en el MIDI; en el WAV sí, porque se
  ejecuta el driver).
* Pistas de SFX: un paso por tick.

### MIDI (`mrom/tools/export_midi.py`)

Interpreta las secuencias (saltos, llamadas, bucles; una pasada) y escribe un
MIDI tipo 1 por canción en `build/mrom/midi/` (no se versiona: deriva de la
ROM); `--sfx` añade los 177 SFX. Las pistas cuyo primer instrumento es ADPCM-A
van al canal 10. Es una partitura: los timbres reales (FM, muestras) no viajan
en el MIDI.

### WAV idéntico al juego (`mrom/tools/export_wav.py` + `mrom/tools/sndrender/`)

Para que suene **exactamente** como en la placa no se re-sintetiza nada:
`sndrender` (C++) ejecuta el propio `201-m1.bin` en un Z80 a 4 MHz conectado a
un YM2610 emulado con **ymfm** (el núcleo de MAME, BSD-3) a 8 MHz, con
`201-v1/v2` como memoria ADPCM. Del lado del 68000 solo se reproduce lo que hace
el hardware: escribir el byte de tipo y el de sonido en el latch (uno por
frame, 59,19 Hz), cada uno con su NMI. El Timer A genera la IRQ del driver,
los bancos se conmutan por los puertos `08..0B`, y la salida es la del chip a
su frecuencia nativa (8 MHz / 144 = **55 556 Hz**, estéreo 16 bit) mezclada
como en la placa (FM+ADPCM 0,98; SSG 0,28, igual que MAME).

```sh
python3 mrom/tools/export_wav.py             # todo: 13 músicas, 150 SFX, 27 voces
python3 mrom/tools/export_wav.py --music     # solo músicas
python3 mrom/tools/export_wav.py --loops 1   # una pasada del bucle (por defecto 2) + fundido
python3 mrom/tools/export_wav.py --vgm       # además VGM 1.51 → build/mrom/vgm/
python3 mrom/tools/export_wav.py --only 21 2C --flac
```

Salida en `build/mrom/wav/{music,sfx,voices}/` (SFX y voces con su id:
`1131_Voice_Mission1Start.wav`). **Fin de las canciones**: `sndrender`
intercepta los saltos de bucle infinito del propio driver (`C2` hacia atrás
en `56D3`, `D0` con contador 0 en `5626`) de cada pista de música; una pasada
termina cuando la última pista en hacer bucle vuelve a su punto de bucle, y
tras `--loops` pasadas se aplica un fundido de `--fade` s (8). Así cada
canción acaba en un punto musical, no a mitad de frase (la versión anterior
cortaba p. ej. Mission2 a los 140 s, en medio del bucle). Las canciones sin
bucle (jingles, `Music_Title`) y los SFX terminan solos tras 1 s de silencio.

**VGM**: con `--vgm` (o `sndrender --vgm f.vgm`) se registran todas las
escrituras del Z80 al YM2610 (comandos `58`/`59`, esperas a 44,1 kHz) con
bloques de datos `0x82` (ADPCM-A) y `0x83` (ADPCM-B) que contienen solo las
páginas de la V-ROM que el chip ha leído, y etiqueta GD3. Comprobado
reproduciendo el VGM con `vgmrender` de ymfm: envolvente de energía
correlación 0,91 con el WAV directo.

Cualquier comando del driver se puede renderizar directamente:
`build/mrom/sndrender rom/201-m1.bin rom/201-v1.bin rom/201-v2.bin out.wav 60 00 21`.
Comprobado de oído: Mission1 = «Assault Theme», `2C` = voz «Nasca» del logo, voces de la tabla B transcritas arriba (§7).
Dependencias vendorizadas en `mrom/tools/sndrender/third_party/` (ymfm BSD-3,
superzazu/z80 MIT; sin cambios).
