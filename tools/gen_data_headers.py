#!/usr/bin/env python3
"""
gen_data_headers.py - cabeceras A–F de los módulos de datos asm/data/*.s.

Sustituye el párrafo "BORRADOR generado por gen_data_region.py" de cada
módulo por una cabecera A–F. El texto de A..E es manual (TEXT, o GEN para los
bancos de frames/animación). F se calcula con el censo de
assets/tools/data_census.py y con los usuarios del código (lea / move.l #abs).
Sólo toca comentarios, así que la ROM sigue idéntica.

    python3 tools/gen_data_headers.py            # (re)genera las cabeceras
"""
import os
ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
os.chdir(ROOT)
import sys, pickle, collections, re
sys.path.insert(0, 'tools')
import registry as R

sys.path.insert(0, 'assets/tools')
import bisect                    # noqa: E402
import sprites as S              # noqa: E402
import data_census as DC         # noqa: E402


def code_users(p):
    """módulo de datos -> Counter(rutina) de lea abs/pc y move.l #abs (>= $20000)."""
    d = p.d
    code = sorted((a, a + n, nm) for nm, a, n, f in R.REGISTRY if not f.startswith('data/'))
    data = sorted((a, a + n, f) for nm, a, n, f in R.REGISTRY if f.startswith('data/'))
    ds = [x[0] for x in data]
    u = collections.defaultdict(collections.Counter)
    for s, e, nm in code:
        for o in range(s, e - 4, 2):
            op = int.from_bytes(d[o:o + 2], 'big')
            if (op & 0xF1FF) == 0x41F9:
                v = int.from_bytes(d[o + 2:o + 6], 'big')
            elif (op & 0xF1FF) == 0x41FA:
                v = o + 2 + int.from_bytes(d[o + 2:o + 4], 'big', signed=True)
            elif op in (0x2D7C, 0x217C, 0x2B7C, 0x23FC) or (op & 0xF1FF) == 0x203C:
                v = int.from_bytes(d[o + 2:o + 6], 'big')
                if v < 0x20000:          # VRAM fix / constantes, no punteros
                    continue
            else:
                continue
            if not p.valid(v):
                continue
            vo = p.off(v)
            i = bisect.bisect_right(ds, vo) - 1
            if i >= 0 and vo < data[i][1]:
                u[data[i][2]][nm] += 1
    return u


_p = S.PRom()
cen = {f: (lo, hi, dict(c)) for f, lo, hi, c in DC.census(_p, DC.classify(_p))}
uses = code_users(_p)
KINDS = ("palette", "frame", "anim", "tilemap", "collision", "zero", "other")
KN = dict(palette="paletas", frame="frames", anim="animación", tilemap="mapas de fondo",
          collision="colisión", zero="ceros", other="otro")

TEXT = {
"sprite_tables_002f30.s": dict(
 A="""Tablas de conversión de color del motor de paletas (video/palette_engine_
   sprite_grid_0133b0.s) más los registros de interfaz que las siguen.
     $002F30..$012F30  ColorRGB555ToNeoGeo[32768]: palabra de color Neo Geo
                       (bit de brillo bajo de cada canal en b14..b12, 4 bits
                       altos en b11..b0) para cada RGB555 de 15 bits.
                       Verificado: las 32768 entradas == fórmula.
     $012F30..$012FB0  ColorRamp_RowPtr[32]: punteros a las filas de la rampa.
     $012FB0..$0133B0  ColorRamp[32][32]: fila k, columna v = v*k/31
                       (división entera; verificado 1024/1024). La usa el
                       fundido de paletas para escalar cada canal 0..31.
   Las etiquetas Data_/Str_ del interior sólo marcan direcciones a las que
   apuntan bytes que parecen punteros; la mayoría son falsos positivos.""",
 B="""`lea $2F30.l` en video/palette_engine_sprite_grid_0133b0.s ($133C8,
   $13404) y `lea Data_002f30(pc)` en BootScr ($002E10), siempre tras
   Pal_ApplyFadeDarken_013624: d4 = r<<10|g<<5|b, se lee la palabra y se
   escribe en la paleta en RAM. La fórmula se comprobó sobre las 32768
   entradas.""",
 C="""Las etiquetas que hay dentro de la tabla de color (Str_00316e, Data_00a000,
   ...) salen de valores de otras tablas que coinciden con una dirección. No
   son estructuras propias.""",
 D="""ColorRGB555ToNeoGeo: .dc.w por índice r<<10|g<<5|b.
   ColorRamp: 32 filas de 32 B (1 B por valor 0..31).""",
 E="""Pal_LoadRaw16_0133b0, PalAnim_Blend16_013480, PalAnim_StepRGB_013504,
   Pal_ApplyFadeDarken/Lighten_013624/013694 (video/palette_engine_sprite_
   grid_0133b0.s) y PalSlot_UpdateAll_002c86 (boot)."""),
"sparse_tables_013d6a.s": dict(
 A="""Banco de paletas principal.
     $013D6A..$014E00  ceros (4 246 B, relleno de alineado).
     $014E00..$024E00  PaletteBankLo[1024] × 64 B (16 colores × 4 B: R,G,B,0
                       en 5 bits). Los índices 0..1023 de Entity_AllocSprite-
                       Slot_00236E (`move.w #pal,d1; jsr $236E`).
     $024E00..$024E10  16 B de código/datos residual antes de la siguiente
                       región de código.
   El "banco alto" ($1CE00, 512 paletas) está dentro de este mismo módulo
   (índices 1024.. de assets/tools/sprites.py) — en realidad es una ventana
   sobre esta misma tabla: $1CE00 = PaletteBankLo[512].""",
 B="""PalSlot_LoadList_002a7c (slots $10..$74 desde $14E00), PalSlot_LoadList-
   Hi_002b58 (desde $1CE00), Entity_AllocSpriteSlot_00236E; extraídas a PNG
   por assets/tools/sprites.py (palettes.png, 1536 paletas).""",
 C="""El formato de 4 B por color (en lugar de la palabra Neo Geo) se convierte
   en tiempo de ejecución con ColorRGB555ToNeoGeo ($2F30).""",
 D="""Pal_XXXXXX: 64 B = 16 × {R, G, B, 0}, 0..31 por canal.""",
 E="""boot/boot_script_sound_palslots_000400.s (PalSlot_*), video/palette_engine_*."""),
"anim_pal_spawn_09c608.s": dict(
 A="""Mapas de fondo de las misiones (tilemaps de los bind_path de los scripts de
   escena) y tablas asociadas.
     BgMap_XXXXXX  celdas {tile.w, attr.w} por columnas (formato SCB1:
                   attr = pal<<8 | tile_msb<<4 | flipV<<1 | flipH). Las 52
                   capas de las 13 escenas; ver assets/tools/levels.py.
     Data_XXXXXX   resto: más mapas no enlazados desde scripts (mismo formato,
                   p. ej. $0B9F68 18 KiB) y tablas de spawn/paletas.""",
 B="""Op $00 bind_path de SceneScriptVM_Frame_0437DA (data/scene_scripts_
   0916c8.s) -> CellCommit_MMIO_051DE2. levels.py los renderiza a PNG
   (52/52 con color).""",
 C="""Muchos Data_ grandes con patrón `xxxx 1x10` son tilemaps del mismo
   formato que no se enlazan con bind_path (probablemente se escriben con
   SCB1_WriteTileColumn_05b52e desde código).""",
 D="""Celda = 4 B; mapa = w columnas × h celdas.""",
 E="""SCB1_WriteTileColumn_05b52e, SCB1_WriteTileColumnTerm_05bf76,
   SceneScriptVM_Frame_0437DA."""),
"template_index_0e8000.s": dict(
 A="""TemplateIndex_0e8000: tabla de 329 punteros .l a las plantillas de
   tarea/entidad que instancia el bytecode de misión (campo +4 de los
   registros spawn de data/mission_streams_0e8524.s).""",
 B="""Spawn_FromStream_0448A6 (mission/spawn_boss_0448a6.s +$42): d0 =
   +4(registro)*4; `lea $E8000.l,a0; movea.l (a0,d0.w),a1` -> Task_Alloc.""",
 C="""—""",
 D=""".dc.l Plantilla × N.""",
 E="""mission/event_vm_04422a.s (MissionVM_*)."""),
"level_a_0f2ffc.s": dict(
 A="""Mapas de colisión de los fondos ($200000+ en el espacio de CPU, banco
   alto del P-ROM) más relleno de ceros.
     ColMap_XXXXXX  4 B por celda, mismo recorrido (w × h por columnas) que el
                    tilemap de su capa; los referencia el campo `cont` del
                    bind_path. 18 mapas distintos.""",
 B="""Campo +$0E del op $00 bind_path; Camera0_RelinkAndWrapScroll_06896A y
   collision_cell_apply_051ba8 los leen.""",
 C="""El significado de los 4 bytes (tipo de suelo, pendiente, flags) está por
   documentar celda a celda.""",
 D="""ColMap: w*h*4 B.""",
 E="""entity/collision_cell_apply_051ba8.s, video/cellmap_fix_overlay_palfade_
   051aa4.s."""),
}
GEN = dict(
 A="""Banco de datos gráficos referenciado por el código: frames de sprite
   (SprFrame_), bytecode de animación (AnimScr_), mapas de colisión
   (ColMap_) y tablas sin decodificar (Data_), clasificados con
   assets/tools/data_census.py.""",
 B="""Punteros desde el código (lea/move.l #abs) y desde otros scripts de
   animación (op 01/03 y op 02 `02 fl .l prio.w`); los frames se validan con
   sprites.parse_frame y los scripts con sprites.walk.""",
 C="""Las entradas Data_ que quedan son en su mayoría tablas indexadas
   (offsets, velocidades, listas de piezas) cuyo formato depende de la
   rutina que las usa; ver la lista de usuarios abajo.""",
 D="""SprFrame_: {npiezas.b, attr.b(banco<<4), piezas{dx.w, dy.w, w.b, h.b,
   tiles[w*h].w por columnas}}. AnimScr_: ops de 1..12 B (02 = dibuja,
   00 n = espera, 01 = salto, 03 = bucle, 1B..1E = fin); ver assets/README.md.""",
 E="""ScriptOp02_Sprite_028dfe y el intérprete de animación de entity/*.""")
TAIL = {
 "rom_tail_fill_1f8000.s": "Fin del P-ROM: 2 244 B de animación (objetos lanzados, bandera) y relleno $00; $1FF800 = firma 12345678.",
}


def stats(f):
    lo, hi, c = cen['data/' + f]
    n = hi - lo
    parts = ["%s %.1f%%" % (KN[k], 100 * c.get(k, 0) / n) for k in KINDS if c.get(k, 0)]
    pref = collections.Counter(nm.split('_')[0] for nm, a, s, ff in R.REGISTRY if ff == 'data/' + f)
    us = uses.get('data/' + f, collections.Counter())
    top = ", ".join("%s×%d" % kv for kv in us.most_common(8)) or "—"
    return n, parts, pref, top, len(us)


def wrap(s, ind="   "):
    out, line = [], ""
    for w in s.split():
        if len(line) + len(w) + 1 > 72:
            out.append(line); line = w
        else:
            line = (line + " " + w).strip()
    if line: out.append(line)
    return [ind + l for l in out]


def header(f):
    t = TEXT.get(f, GEN)
    n, parts, pref, top, nus = stats(f)
    L = ["|"]
    for key, title in (("A", "QUÉ HAY AQUÍ"), ("B", "EVIDENCIA"), ("C", "HIPÓTESIS / NOTAS"),
                       ("D", "FORMATOS"), ("E", "HELPERS / USUARIOS")):
        L.append("|  %s. %s" % (key, title))
        txt = t[key]
        if key == "A" and f in TAIL:
            txt = TAIL[f]
        for line in txt.splitlines():
            L.append("|  " + line.strip("\n") if line.startswith("   ") else "|   " + line)
        if key == "E":
            L.append("|   Rutinas con lea/move.l #abs a este módulo (%d):" % nus)
            L += ["|" + l for l in wrap(top, "     ")]
        L.append("|")
    L.append("|  F. ESTADO")
    L.append("|   Matching byte a byte (build_rom ✔). Censo (data_census.py, %d B):" % n)
    L += ["|" + l for l in wrap(", ".join(parts) + ".", "     ")]
    L.append("|   Entradas por prefijo: " + ", ".join("%s %d" % kv for kv in pref.most_common()) + ".")
    L.append("|   Pendiente: decodificar las Data_ restantes.")
    L.append("|")
    return L


HDR = re.compile(r"\|\n\|  A\. QUÉ HAY AQUÍ\n.*?\|  F\. ESTADO\n.*?\|   Pendiente: decodificar las Data_ restantes\.\n\|\n", re.S)
DRAFT = re.compile(r"\|\n\|  BORRADOR generado por tools/gen_data_region\.py.*?\n\|  referenciada desde el código \(o frontera de racha de ceros\)\.\n\|\n", re.S)
for f in sorted(cen):
    f = f[5:]
    path = 'asm/data/' + f
    s = open(path, encoding='utf-8').read()
    rx = DRAFT if DRAFT.search(s) else HDR if HDR.search(s) else None
    if rx is None:
        print("skip", f); continue
    s = rx.sub(lambda m: "\n".join(header(f)) + "\n", s, count=1)
    open(path, 'w', encoding='utf-8').write(s)
    print("ok", f)
