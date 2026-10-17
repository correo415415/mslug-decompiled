#!/usr/bin/env python3
"""
data_census.py - censo de los módulos de datos del P-ROM (asm/data/*.s).

Clasifica cada byte de las 14 regiones de datos según el tipo de asset que
el código del juego le da, usando los mismos decodificadores que los
extractores (sprites.py, levels.py, scene_script_dump.py):

  palette     paletas de 64 B ($14E00 x1024, $1CE00 x512)
  frame       frames de sprite {npiezas, attr, piezas{dx,dy,w,h,tiles}}
              (+ rachas sin puntero que se recorren enteras como frames)
  anim        bytecode de animación (op 02 y demás ops recorridos; más los
              scripts que el código instala con EntitySetSpriteMap, recorridos
              con walk_struct: sprite oculto $FFFFFFFF y tablas de 1B/1C)
  tilemap     mapas de celdas de fondo {tile.w, attr.w} de los bind_path
  collision   mapas de colisión de los bind_path (4 B por celda, w*h celdas)
              (+ parches por Sprite_InvokeBlit8Params: `lea rec,a2; jsr $5022A`,
              rec = {ent.l, desc.l, 6 w}, desc = {w, h, tiles.l}; sólo las
              columnas que los registros llegan a copiar)
  struct      estructuras decodificadas con formato documentado: bytecode de
              misión (MissionStream_/MissionAux_), plantillas (TemplateIndex_),
              tabla/scripts/entidades/triggers de escena (Scene*_), registros
              y descriptores de parche de Sprite_InvokeBlit8Params, cajas de
              Entity_CheckOnScreenBox, rejillas de $43FAC y registros de 12 B
              del op 19 de animación, diccionarios
              de nombres de Results_RosterPick (ROSTER_NAMES)
  lut         tablas generadas por fórmula (ColorRGB555ToNeoGeo, ColorRamp,
              SinTab256)
  zero        rachas de $00 >= 16 B
  other       resto (tablas de código, parámetros, strings, etc.)

    python3 assets/tools/data_census.py            # tabla por módulo
    python3 assets/tools/data_census.py --json F   # además vuelca a JSON
"""
import json, os, sys
from collections import Counter

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, HERE)
sys.path.insert(0, os.path.join(ROOT, "tools"))
import sprites as S          # noqa: E402
import levels as LV          # noqa: E402
import registry              # noqa: E402

KINDS = ("palette", "frame", "anim", "tilemap", "collision", "struct", "lut", "zero", "other")

# prefijos de registry cuyo contenido entero ya está decodificado por su parser
STRUCT_PREFIXES = ("MissionStream_", "MissionAux_", "TemplateIndex_", "SceneDescTable_",
                   "SceneScript_", "SceneEntities_", "SceneTrig_", "TaskBootTab_", "SoftDip_", "PropItemPtrs_", "HudText_",
                   "CollOffsets_", "CollSlopes_", "CollTileTypes_", "CollMasks_", "HitShapeTab_",
                   "InputDemo")

# tablas de color del motor de paletas (asm/data/sprite_tables_002f30.s)
LUTS = ((0x002F30, 0x012F30), (0x012F30, 0x012FB0), (0x012FB0, 0x0133B0),
        (0x2C072C, 0x2C09AC),     # SinTab256_1c072c (CPU $2C072C, 320 palabras)
        # puntería por dx (Bazooka_AimStep_06b5ea / RocketVehicle_AimStep_06d340):
        # dirección 0..15 = T[(clamp(dx,-256,256)+256)>>4] (33 B por tabla, stride
        # $22) y la tabla de espejo dir -> (16-dir)&15 (16 B)
        (0x2CA5F0, 0x2CA644),     # Bazooka: arriba, abajo, espejo
        (0x2CD4EA, 0x2CD606))     # RocketVehicle: 2 x 8 punteros, 6 tablas, espejo

# tablas de registros con usuario conocido (CPU, [lo, hi)) -> "struct"
STRUCTS = (
    # RocketVehicle_Fire_06d484: 4 tablas de 4 punteros (+$48/+$60/+$4C)
    (0x2CCFEA, 0x2CD02A),
    # RocketVehicle_FireRocket_06d4c2: $2CD03A = 4 punteros a 16 x {dx, dy} de
    # boca de cañón; RocketVehicle_Rocket_06cf06: $2CD10A/$2CD11A = [4][8]
    # punteros a 16 x {vx, vy, ay} (6 B) por dirección
    (0x2CD03A, 0x2CD4EA),
    # AimTurret_AnimScript_0614e6: 7 listas de 16 x {.l frame, .b dur, pad}
    # (+ -1) por nivel de daño y la tabla de 7 punteros
    (0x2C2E76, 0x2C322E),
)


def modules():
    m = {}
    for name, a, n, f in registry.REGISTRY:
        if f.startswith("data/"):
            lo, hi = m.get(f, (1 << 30, 0))
            m[f] = (min(lo, a), max(hi, a + n))
    return sorted(m.items(), key=lambda t: t[1][0])


def _cells_ok(p, a, h):
    """Una columna de h celdas {tile.w, attr.w} plausible: byte bajo del attr
    sólo con bits de tile_msb (b4) / flip (b0..b1) en >= 95 % de las celdas."""
    return sum(1 for r in range(h) if p.B(a + 4 * r + 3) & 0xEC == 0) >= 0.95 * h


def blit_patches(p, code_end=0x90000):
    """-> [(rec, desc, w, h, tiles, ncol)] de los registros de 20 B que
    Sprite_InvokeBlit8Params pasa a CellCommit_MMIO_051DE2 (a0 = ent, a1 =
    desc, d0..d5 = col_dest, fila_dest, col_origen, fila_origen, ancho, alto).
    ncol = máxima columna de origen copiada (col_origen + ancho), ampliada
    mientras las columnas siguientes del descriptor sigan siendo celdas
    válidas (el w de algunos descriptores excede los datos reales)."""
    recs = set()
    for o in range(0, code_end, 2):
        if p.W(o) == 0x45F9 and p.W(o + 6) == 0x4EB9 and p.L(o + 8) == 0x5022A:
            recs.add(p.L(o + 2))
    by_desc = {}
    for r in recs:
        by_desc.setdefault(p.L(r + 4), []).append(r)
    out = []
    for desc, rs in sorted(by_desc.items()):
        w, h, tiles = p.W(desc), p.W(desc + 2), p.L(desc + 4)
        ncol = min(w, max(p.W(r + 12) + p.W(r + 16) for r in rs))
        while ncol < w and _cells_ok(p, tiles + 4 * h * ncol, h):
            ncol += 1                       # columnas no copiadas pero válidas
        for r in rs:
            out.append((r, desc, w, h, tiles, ncol))
    return out


def blit_records(p, descs):
    """Registros de 20 B de Sprite_InvokeBlit8Params sin `lea ...,a2 ; jsr`
    directo (llamadas pc-rel, tablas): {l entidad ($10xxxx), l descriptor
    conocido, w col_dest, w fila_dest, w col_origen, w fila_origen, w ancho,
    w alto} con el rectángulo de origen dentro del descriptor -> [(addr, 20)]."""
    descs = set(descs)
    out = []
    for x in range(0x200000, 0x300000, 2):
        if not p.valid(x, 20) or p.L(x + 4) not in descs or not 0x100000 <= p.L(x) < 0x110000:
            continue
        ds = p.L(x + 4)
        w, h = p.W(ds), p.W(ds + 2)
        c1, r1, ww, hh = p.W(x + 12), p.W(x + 14), p.W(x + 16), p.W(x + 18)
        if 1 <= ww and 1 <= hh and c1 + ww <= w + 1 and r1 + hh <= h + 1:
            out.append((x, 20))
    return out



# ---- PathScript (VM de movimiento, PathScript_Step_078f90, tabla $078FC0) ----
# opcode = 1 byte, tamaños fijos; 04 salta (.l), 0A fin del tramo (espera), 08
# llama a código (.l); 05/06/07 escriben un campo de la tarea (< $A0).
PATH_OPSZ = {0: 2, 1: 4, 2: 6, 3: 4, 5: 8, 6: 6, 7: 4, 8: 6, 9: 4}


def path_walk(p, c, code_end=0xF0000):
    """Recorre un PathScript (+saltos 04) -> [(addr, n)] o None si no encaja."""
    spans, seen, todo = [], set(), [c]
    while todo:
        a = todo.pop()
        while a not in seen:
            if a & 1 or not p.valid(a, 2) or a < code_end:
                return None
            op = p.B(a)
            if op > 10:
                return None
            seen.add(a)
            if op == 4:
                spans.append((a, 6))
                todo.append(p.L(a + 2))
                break
            if op == 10:
                spans.append((a, 2))
                break
            n = PATH_OPSZ[op]
            if not p.valid(a, n):
                return None
            if op == 8 and (not 0x400 <= p.L(a + 2) < code_end or p.L(a + 2) & 1):
                return None
            if op in (5, 7) and p.W(a + 2) >= 0xA0 or op == 6 and p.W(a + 4) >= 0xA0:
                return None
            spans.append((a, n))
            a += n
            if len(spans) > 600:
                return None
    return spans


def path_scripts(p, code_end=0xF0000):
    """PathScripts instalados por el código: `move.l #X,$90(a6)` y tablas
    `lea T,a0 ... move.l (a0,dN.w),$90(a6)` (se leen mientras cada entrada
    sea un script válido) -> [(addr, n)] de scripts + [(tabla, n)]."""
    ent, tabs, out = set(), [], []
    for o in range(0, code_end - 8, 2):
        w = p.W(o)
        if w == 0x2D7C and p.W(o + 6) == 0x0090:
            ent.add(p.L(o + 2))
        elif w == 0x2D70 and p.W(o + 4) == 0x0090:
            for b in range(o - 2, o - 40, -2):
                if p.W(b) == 0x41F9:
                    tabs.append(p.L(b + 2))
                    break
    for t in sorted(set(tabs)):
        k = 0
        while p.valid(t + 4 * k, 4) and path_walk(p, p.L(t + 4 * k)):
            ent.add(p.L(t + 4 * k))
            k += 1
        if k:
            out.append((t, 4 * k, "struct"))
    for e in sorted(ent):
        r = path_walk(p, e)
        if r:
            out += [(a, n, "struct") for a, n in r]
    return out

SET_SPRITE_MAP = 0x028CD4          # EntitySetSpriteMap: a0 -> +$3C/+$40 (script)
_OPSZ = {**S.OP_SIZE, 0x00: 2, 0x01: 6, 0x03: 6, 0x16: 2, 0x17: 2, 0x18: 2}


def code_script_seeds(p, code_end=0x90000):
    """Scripts de sprite que el código instala directamente:
    `lea X,a0` (abs.l o d16(pc)) seguido (<= 12 B) de jsr/bsr EntitySetSpriteMap,
    o `move.l #X,$3C(a6)` (puntero de script de la tarea)."""
    seeds = set()
    for o in range(0, code_end - 8, 2):
        w = p.W(o)
        if w == 0x41F9:
            v, n = p.L(o + 2), 6
        elif w == 0x41FA:
            v, n = o + 2 + p.SW(o + 2), 4
        elif w == 0x2D7C and p.W(o + 6) == 0x003C:
            seeds.add(p.L(o + 2))
            continue
        else:
            continue
        for q in range(o + n, o + n + 12, 2):
            x = p.W(q)
            t = (p.L(q + 2) if x in (0x4EB9, 0x4EF9) else
                 q + 2 + p.SW(q + 2) if x in (0x4EBA, 0x4EFA, 0x6100) else None)
            if t is not None:
                if t == SET_SPRITE_MAP:
                    seeds.add(v)
                break
    return {v for v in seeds if v & 1 == 0 and p.valid(v, 2)}


def walk_struct(p, a, seen, depth=0):
    """Recorrido estructural del bytecode de sprite (Script_OpcodeTable, entity/
    collmap_hitbox_script_ops_027400.s) -> [(addr, len)] o None si es inválido.
    A diferencia de sprites.walk: acepta `02` con frame $FFFFFFFF (sprite
    oculto: ScriptOp02 hace `movea.l #-1,a0`) y recorre las tablas de 1B/1C
    (entradas de $12/$1C B tras el opcode, indexadas por una palabra de la
    tarea) hasta la primera entrada inválida."""
    out = []
    for _ in range(4000):
        if a in seen or a & 1 or not p.valid(a, 2):
            break
        op = p.B(a)
        if op in (0x02, 0x1E):                # 1E = SpawnChild_Sprite: mismo formato que 02
            fl, fr = p.B(a + 1), p.L(a + 2)
            if fl & 0xE0 or not (fr == 0xFFFFFFFF or
                                 (fr & 1 == 0 and p.valid(fr, 2) and S.parse_frame(p, fr))):
                return None
            n = 8 if fl & 8 else 12
            seen.add(a)
            out.append((a, n))
            a += n
            continue
        if op in (0x1B, 0x1C):
            st = 0x12 if op == 0x1B else 0x1C
            seen.add(a)
            out.append((a, 2))
            for k in range(256):
                e = a + 2 + st * k
                if e in seen:
                    continue
                r = walk_struct(p, e, seen, depth + 1) if depth < 4 else None
                if not r:
                    break
                out += r
            break
        if op in (0x1D, 0x16):                # 16 = RepeatSprite (a1 = $40(a6))
            seen.add(a)
            out.append((a, 2))
            break
        if op not in _OPSZ or not p.valid(a, _OPSZ[op]):
            return out or None
        n = _OPSZ[op]
        seen.add(a)
        out.append((a, n))
        if op == 0x01:
            t = p.L(a + 2)
            if t & 1 or not p.valid(t, 2):
                return None
            a = t
            continue
        if op == 0x03:
            t = p.L(a + 2)
            if t & 1 == 0 and p.valid(t, 2) and t not in seen:
                out += walk_struct(p, t, seen, depth + 1) or []
        a += n
    return out or None


def code_script_tables(p, code_end=0x90000):
    """Tablas de punteros a scripts que el código indexa antes de instalarlos:
    `lea T,a0` / `movea.l #T,a0`, luego `movea.l (a0,dN.w),a0` (<= 12 B) y un
    jsr/bsr EntitySetSpriteMap (<= 24 B). La tabla sigue mientras cada long sea
    $FFFFFFFF (sin sprite) o un script válido (walk_struct).
    -> ([(addr, len)] de tablas, [scripts])"""
    tabs, scripts = [], set()
    for o in range(0, code_end - 8, 2):
        w = p.W(o)
        if w in (0x41F9, 0x207C):
            v, n = p.L(o + 2), 6
        elif w == 0x41FA:
            v, n = o + 2 + p.SW(o + 2), 4
        else:
            continue
        if v & 1 or not p.valid(v, 4):
            continue
        idx = any(p.W(q) == 0x2070 for q in range(o + n, o + n + 12, 2))
        if not idx:
            continue
        hit = False
        for q in range(o + n, o + n + 24, 2):
            x = p.W(q)
            t = (p.L(q + 2) if x in (0x4EB9, 0x4EF9) else
                 q + 2 + p.SW(q + 2) if x in (0x4EBA, 0x4EFA, 0x6100) else None)
            if t == SET_SPRITE_MAP:
                hit = True
                break
        if not hit:
            continue
        k, ok = 0, []
        while k < 256 and p.valid(v + 4 * k, 4):
            e = p.L(v + 4 * k)
            if e == 0xFFFFFFFF:
                k += 1
                continue
            if e & 1 or not p.valid(e, 2) or not walk_struct(p, e, set()):
                break
            ok.append(e)
            k += 1
        if ok:
            tabs.append((v, 4 * k))
    # una tabla acaba donde empieza la siguiente conocida
    starts = sorted({v for v, n in tabs})
    out = []
    for i, v in enumerate(starts):
        n = max(n for t, n in tabs if t == v)
        if i + 1 < len(starts):
            n = min(n, starts[i + 1] - v)
        out.append((v, n))
        for k in range(n // 4):
            e = p.L(v + 4 * k)
            if e != 0xFFFFFFFF:
                scripts.add(e)
    return out, scripts


def pointed_scripts(p, mark):
    """Scripts sin clasificar a los que apunta algún long de la ROM y que
    walk_struct recorre con >= 2 frames (op 02) acabando en un op de salto o
    fin (01/16/1B/1C/1D/1E/00) -> [(addr, len)]."""
    d, cand = p.d, set()
    for o in range(0, len(d) - 3, 2):
        v = (d[o] << 24) | (d[o + 1] << 16) | (d[o + 2] << 8) | d[o + 3]
        if v & 1 == 0 and (0x90000 <= v < 0x100000 or 0x200000 <= v < 0x300000) \
                and v not in mark and p.valid(v, 2):
            cand.add(v)
    out = []
    for v in sorted(cand):
        r = walk_struct(p, v, set())
        if r and sum(1 for a, n in r if p.B(a) in (0x02, 0x1E)) >= 2 and \
                p.B(max(r)[0]) in (0x00, 0x01, 0x16, 0x1B, 0x1C, 0x1D):
            out += r
    return out


ONSCREEN_BOX = (0x05DD56, 0x05DD5C)  # a0 = caja {dx0, dx1, dy0, dy1} (8 B)
PROBE_GRID = 0x043FAC                # a1 = {dx, dy, w, h} + w*h bytes
FIX_STREAM = 0x05DAD8                # a2 = bytes de tile del fix ($FD = línea, $FE = fin)
LIST_FF = (0x0477FC, 0x04784C, 0x047888)   # a2 = lista de bytes hasta $FF (glifos)


def code_arg_tables(p, code_end=0x90000):
    """-> [(addr, len)] de tablas pasadas en a0/a1 a rutinas de formato fijo:
    `lea X,aN` seguido (<= 12 B) de jsr/bsr a ONSCREEN_BOX (caja de 8 B,
    a0) o a PROBE_GRID (rejilla de bytes, a1), y textos en a2 para
    Fix_BlitStream_05DAD8 (hasta $FE) o List_ApplyWithSentinelFF_* (hasta $FF).
    Los `lea` en a2 admiten también direcciones impares (texto)."""
    out = []
    for o in range(0, code_end - 8, 2):
        w = p.W(o)
        if w in (0x41F9, 0x43F9, 0x45F9):
            v, n, reg = p.L(o + 2), 6, (w >> 9) & 7
        elif w in (0x41FA, 0x43FA, 0x45FA):
            v, n, reg = o + 2 + p.SW(o + 2), 4, (w >> 9) & 7
        else:
            continue
        for q in range(o + n, o + n + 12, 2):
            x = p.W(q)
            t = (p.L(q + 2) if x in (0x4EB9, 0x4EF9) else
                 q + 2 + p.SW(q + 2) if x in (0x4EBA, 0x4EFA, 0x6100) else None)
            if t is None:
                continue
            if v & 1 == 0 and p.valid(v, 8):
                if t in ONSCREEN_BOX and reg == 0:
                    out.append((v, 8))
                elif t == PROBE_GRID and reg == 1:
                    gw, gh = p.W(v + 4), p.W(v + 6)
                    if 0 < gw * gh <= 4096:
                        out.append((v, 8 + gw * gh))
            if reg == 2 and (t == FIX_STREAM or t in LIST_FF) and p.valid(v, 1):
                end = 0xFE if t == FIX_STREAM else 0xFF
                k = 0
                while k < 4096 and p.valid(v + k, 1) and p.B(v + k) != end:
                    k += 1
                if k < 4096:
                    out.append((v, k + 1))
            break
    return out


TBL_DECODE2D = 0x0799DE   # Tbl_Decode2D_0799DE: a0 = tabla por dificultad/rango


def decode2d_tables(p, code_end=0xF0000):
    """-> [(addr, len)] de tablas pasadas en a0 a Tbl_Decode2D_0799DE
    (`lea X,a0` + jsr/bsr <= 12 B, sin otro `lea ..,a0` entre medias).
    Formato largo (palabra 0 != 2): 2 + 64 palabras indexadas por
    (flags11&3)*4 + ($106ED1&2) + (Rank_SubIndex&7)*16 -> 130 B.
    Formato corto (palabra 0 == 2): 2 + 64 bytes de índice + 16 B por índice
    (8 palabras elegidas con Random&$E) -> 2 + 64 + 16*(max+1)."""
    out = set()
    for o in range(0, code_end - 8, 2):
        w = p.W(o)
        if w == 0x41F9:
            v, n = p.L(o + 2), 6
        elif w == 0x41FA:
            v, n = o + 2 + p.SW(o + 2), 4
        else:
            continue
        ind = False
        for q in range(o + n, o + n + 12, 2):
            x = p.W(q)
            if x & 0xFFC0 == 0x41C0:          # otro lea ..,a0
                break
            if x == 0x2070 and p.B(q + 2) & 0x8F == 0:   # movea.l (a0,dN.w),a0
                ind = True
                continue
            t = (p.L(q + 2) if x in (0x4EB9, 0x4EF9) else
                 q + 2 + p.SW(q + 2) if x in (0x4EBA, 0x4EFA, 0x6100) else None)
            if t is None:
                continue
            if t == TBL_DECODE2D:
                if ind:
                    k = 0
                    while p.valid(v + 4 * k, 4) and _d2d_len(p, p.L(v + 4 * k)):
                        e = p.L(v + 4 * k)
                        out.add((e, _d2d_len(p, e)))
                        k += 1
                    if k:
                        out.add((v, 4 * k))
                elif _d2d_len(p, v):
                    out.add((v, _d2d_len(p, v)))
            break
    return sorted(out)


def _d2d_len(p, v):
    """Longitud de una tabla Tbl_Decode2D en v (0 si no encaja)."""
    if v & 1 or not 0x100000 <= v < 0x300000 or not p.valid(v, 130):
        return 0
    if p.W(v) == 2:
        mx = max(p.B(v + 2 + i) for i in range(64))
        return 66 + 16 * (mx + 1) if mx < 32 and p.valid(v, 66 + 16 * (mx + 1)) else 0
    return 130


INSTALL_SPAWNER = 0x077C7E   # Entity_InstallHandlerAndCopyXf: a1 -> $70(a0)


def spawner_records(p, code_end=0x90000):
    """-> [(addr, len)] de registros de Spawner_Handler_077c98 (18 B):
    {b offset_por_padre, b ritmo ($FF = todo de golpe), w cuenta, w dx, w rx,
    w dy, w ry, b ?, b aleatorio, l handler}; si +$D != 0, +$E apunta a una
    tabla de 16 handlers elegidos con Random(16)*4 (64 B). Se siguen los
    `lea X,a1` + jsr/bsr Entity_InstallHandlerAndCopyXf (<= 16 B)."""
    out = set()
    for o in range(0, code_end - 8, 2):
        w = p.W(o)
        if w == 0x43F9:
            v, n = p.L(o + 2), 6
        elif w == 0x43FA:
            v, n = o + 2 + p.SW(o + 2), 4
        else:
            continue
        for q in range(o + n, o + n + 16, 2):
            x = p.W(q)
            if x & 0xFFC0 == 0x43C0:          # otro lea ..,a1
                break
            t = (p.L(q + 2) if x in (0x4EB9, 0x4EF9) else
                 q + 2 + p.SW(q + 2) if x in (0x4EBA, 0x4EFA, 0x6100) else None)
            if t is None:
                continue
            if t == INSTALL_SPAWNER and v & 1 == 0 and p.valid(v, 18):
                out.add((v, 18))
                h = p.L(v + 14)
                if p.B(v + 13) and h & 1 == 0 and p.valid(h, 64) and \
                        all(p.L(h + 4 * i) < code_end for i in range(16)):
                    out.add((h, 64))
            break
    return sorted(out)


# Textos "a máquina" de la presentación de misión (mission/cutscene_anim_08ba0c):
# Icon_Anchor_Run_08c678 elige por idioma ($8A(a6) = 0..3) una de 4 tablas de
# 44 pares {stream.l, atributo_fix.l} indexadas por la página $76 del padre;
# cada stream (Anim_ScriptStepFix_A/B, Anim_ScriptStep_08bc74) son registros
# de 8 B {w espera ($FFFF = fin), w carácter/comando, l script de sprite o 0}.
FIX_TYPE_TABS = (0x2EFACC, 0x2EFC2C, 0x2EFD8C, 0x2EFEEC)
FIX_TYPE_PAGES = 44
FIX_TYPE_STREAMS = (0x2EF8AA, 0x2F0056)


def fix_type_streams(p):
    """-> ([(addr, len)] tablas + streams, [scripts de sprite referenciados])."""
    out, streams, scripts = [], set(FIX_TYPE_STREAMS), set()
    for t in FIX_TYPE_TABS:
        out.append((t, 8 * FIX_TYPE_PAGES))
        for k in range(FIX_TYPE_PAGES):
            streams.add(p.L(t + 8 * k))
    for s in sorted(streams):
        n = 0
        while p.valid(s + 8 * n, 2) and p.W(s + 8 * n) != 0xFFFF and n < 4096:
            f = p.L(s + 8 * n + 4)
            if f and f & 1 == 0 and p.valid(f, 2):
                scripts.add(f)
            n += 1
        out.append((s, 8 * n + 2))
    return out, sorted(scripts)


# SpawnStream_ReadNext_04cac4: por misión (SpriteSetPtrTbl6_04cb44[$106ECE])
# registros de 16 B {b ?, b ?, b rel_x, b rel_y, w x, w y, w d2, w d3, b d4,
# b d5, b d6, b ?} y luego w umbral de cámara del siguiente; fin con $FFFF.
SPAWN_STREAM_TAB, SPAWN_STREAM_N = 0x04CB44, 6


def spawn_streams(p):
    """-> [(addr, len)] de los 6 streams de spawn por misión."""
    ss = sorted({p.L(SPAWN_STREAM_TAB + 4 * i) for i in range(SPAWN_STREAM_N)})
    out = []
    for i, s in enumerate(ss):
        lim = ss[i + 1] if i + 1 < len(ss) else s + 16 * 512
        k = 0
        while s + 16 * k + 18 <= lim and p.W(s + 16 * k + 16) != 0xFFFF:
            k += 1
        out.append((s, 16 * (k + 1) + 2))
    return out


# Diccionarios de nombres de Results_RosterPickA/B_03e6da/03e740 (pantalla de
# resultados): d0 = Random(n) -> longitud = LenTab[d0]; {ptr.l, cnt.l} =
# PtrTab[longitud - base]; nombre = ptr + longitud * Random(cnt).
#   (LenTab, n, PtrTab, base): A = katakana (b>=$80 -> tile $2A00+b-$80),
#   B = ASCII (modo $10FD83).
ROSTER_NAMES = ((0x182DE0, 0x3B2, 0x1831B2, 1), (0x182C3C, 0x14C, 0x182D88, 2))


def roster_names(p):
    """-> [(addr, len)] de las tablas de longitudes, de punteros y los pools."""
    out = []
    for lt, n, pt, base in ROSTER_NAMES:
        lt, pt = p.cpu(lt), p.cpu(pt)
        out.append((lt, n))
        kmax = max(p.B(lt + i) for i in range(n))
        out.append((pt, 8 * (kmax - base + 1)))
        for k in range(base, kmax + 1):
            ptr, cnt = p.L(pt + 8 * (k - base)), p.L(pt + 8 * (k - base) + 4)
            if p.valid(ptr, k * cnt):
                out.append((ptr, k * cnt))
    return out


def _gaps(p, mark):
    """Rachas de offsets [lo, hi) de los módulos sin clasificar todavía."""
    out, cur = [], None
    for f, (lo, hi) in modules():
        for o in range(lo, hi):
            if p.cpu(o) not in mark:
                if cur and cur[1] == o:
                    cur[1] = o + 1
                else:
                    cur = [o, o + 1]
                    out.append(cur)
    return out


def _split_zeros(p, gaps, min_zero=4):
    """Corta cada racha [lo, hi) en los tramos de palabras a cero >= min_zero B
    (relleno entre bloques de frames) -> [(lo, hi)] sin esos ceros."""
    out = []
    for lo, hi in gaps:
        start, o = lo, lo + (lo & 1)
        while o < hi - 1:
            if p.W(p.cpu(o)) == 0:
                z = o
                while o < hi - 1 and p.W(p.cpu(o)) == 0:
                    o += 2
                if o - z >= min_zero:
                    if z > start:
                        out.append((start, z))
                    start = o
                continue
            o += 2
        if hi > start:
            out.append((start, hi))
    return out


def frame_runs(p, mark, min_cover=0.95):
    """Frames de sprite sin puntero directo: una racha sin clasificar que se
    recorre de punta a punta como frames consecutivos (parse_frame) en >= 95 %
    (si no, se prueba cortando los rellenos de ceros >= 16 B)
    (se tolera relleno final). Típico de animaciones cuyos frames se indexan
    por tabla calculada. -> [(cpu_addr, len)]"""
    def run(lo, hi):
        if lo & 1 or hi - lo < 16:
            return None
        o, fr = lo, []
        while o < hi - 2:
            r = S.parse_frame(p, p.cpu(o))
            if not r or o + r["size"] > hi:
                break
            fr.append((p.cpu(o), r["size"]))
            o += r["size"]
        return fr if fr and (o - lo) >= min_cover * (hi - lo) else None

    out = []
    for g in _gaps(p, mark):
        fr = run(*g)
        if fr is None:
            fr = []
            for sub in _split_zeros(p, [g], 16):
                fr += run(*sub) or []
        out += fr
    return out


def _cell_good(p, c):
    """Celda {tile.w, attr.w} típica: attr con byte bajo $10..$19 (tile_msb +
    flips) y paleta < $80, o celda vacía."""
    t, at = p.W(c), p.W(c + 2)
    return (t == 0 and at == 0) or (0x10 <= at & 0xFF <= 0x19 and at >> 8 < 0x80)


def desc_chains(p, min_n=8):
    """Tablas de descriptores de 8 B {w, h, tiles.l} (formato de los parches de
    Sprite_InvokeBlit8Params) en cadena, cuyos tilemaps son contiguos
    (tiles[i+1] == tiles[i] + 4*w*h) -> [(addr, len, kind)]."""
    out, a = [], 0
    while a < 0x200000 - 8:
        ds, q = [], a
        while True:
            c = p.cpu(q)
            if not p.valid(c, 8):
                break
            w, h, t = p.W(c), p.W(c + 2), p.L(c + 4)
            if not (1 <= w <= 128 and 1 <= h <= 32 and t & 3 == 0 and p.valid(t, 4 * w * h)):
                break
            if ds and ds[-1][2] + 4 * ds[-1][0] * ds[-1][1] != t:
                break
            ds.append((w, h, t))
            q += 8
        if len(ds) >= min_n and sum(_cell_good(p, t + 4 * i) for w, h, t in ds
                                    for i in range(w * h)) >= 0.95 * sum(w * h for w, h, t in ds):
            out.append((p.cpu(a), 8 * len(ds), "struct"))
            for w, h, t in ds:
                out.append((t, 4 * w * h, "tilemap"))
            a = q
        else:
            a += 2
    return out


def tilemap_gaps(p, mark, min_cells=64):
    """Rachas sin clasificar que son celdas de tilemap en >= 98 % (con >= 50 %
    no vacías): tilemaps de fondo sin descriptor conocido (extensiones de capas
    vecinas). -> [(addr, len)]"""
    out = []
    for lo, hi in _gaps(p, mark):
        o = (lo + 3) & ~3
        n = (hi - o) // 4
        if n < min_cells:
            continue
        cs = [p.cpu(o + 4 * i) for i in range(n)]
        good = sum(_cell_good(p, c) for c in cs)
        nz = sum(1 for c in cs if p.W(c + 2))
        if good >= 0.98 * n and nz >= 0.5 * n:
            out.append((p.cpu(o), 4 * n))
    return out


def pointer_tables(p, mark, min_n=3):
    """Rachas sin clasificar de longs que son todos punteros a datos ya
    clasificados ($09xxxx-$0Fxxxx / $2xxxxx) o $FFFFFFFF (entrada vacía), con
    >= 2 punteros reales: tablas de punteros por estado/dirección/fase.
    -> [(addr, len)]"""
    def ok(v):
        return v == 0xFFFFFFFF or ((0x90000 <= v < 0x100000 or 0x200000 <= v < 0x300000)
                                   and v in mark)
    out = []
    for lo, hi in _gaps(p, mark):
        o = lo + (lo & 1)
        while o + 4 <= hi:
            if not ok(p.L(p.cpu(o))):
                o += 2
                continue
            st, real = o, 0
            while o + 4 <= hi and ok(p.L(p.cpu(o))):
                real += p.L(p.cpu(o)) != 0xFFFFFFFF
                o += 4
            if (o - st) // 4 >= min_n and real >= 2:
                out.append((p.cpu(st), o - st))
    return out


def classify(p):
    """-> dict cpu_addr -> kind (sólo bytes marcados)."""
    mark = {}

    def put(a, n, k):
        for x in range(a, a + n):
            mark.setdefault(x, k)

    # tablas por fórmula
    for lo, hi in LUTS:
        put(lo, hi - lo, "lut")
    for lo, hi in STRUCTS:
        put(lo, hi - lo, "struct")
    # paletas
    put(S.PAL_LO, 1024 * 64, "palette")
    put(S.PAL_HI, 512 * 64, "palette")
    # tilemaps de fondo
    m = LV.load_ssd()
    for i in range(m.N_DESC):
        for L in LV.scene_layers(m, i):
            put(L["tiles"], 4 * L["w"] * L["h"], "tilemap")
            if L["collision"]:
                put(L["collision"], 4 * L["w"] * L["h"], "collision")
    # parches de tilemap: lea rec.l,a2 ; jsr Sprite_InvokeBlit8Params ($5022A)
    for rec, desc, w, h, tiles, ncol in blit_patches(p):
        put(tiles, 4 * h * ncol, "tilemap")
        put(rec, 20, "struct")
        put(desc, 8, "struct")
    for a, n in blit_records(p, {d for r, d, w, h, t, nc in blit_patches(p)}):
        put(a, n, "struct")
    # módulos con parser propio (mission_streams, scene_scripts, template_index)
    for name, a, n, f in registry.REGISTRY:
        if name.startswith(STRUCT_PREFIXES):
            put(p.cpu(a), n, "struct")
    # frames + scripts de animación
    ops, frames = S.find_frames(p)
    for fa in frames:
        fr = S.parse_frame(p, fa)
        if fr:
            put(fa, fr["size"], "frame")
    scripts, _ = S.find_scripts(p, ops)
    op_starts = set()
    for s, (pics, info) in scripts.items():
        for a in info["visited"]:
            op = p.B(a)
            n = ops[a][4] if a in ops else S.OP_SIZE.get(op, 2)
            if op == 0x01:
                n = 6
            put(a, n, "anim")
            op_starts.add(a)
    # streams de spawn por misión
    for a, n in spawn_streams(p):
        put(a, n, "struct")
    # diccionarios de nombres de la pantalla de resultados
    for a, n in roster_names(p):
        put(a, n, "struct")
    # tablas de argumentos de formato fijo (cajas en pantalla, rejillas de sondeo)
    for a, n in code_arg_tables(p):
        put(a, n, "struct")
    # tablas por dificultad/rango de Tbl_Decode2D_0799DE
    for a, n in decode2d_tables(p):
        put(a, n, "struct")
    # registros de Spawner_Handler_077c98 (+ tablas de 16 handlers)
    for a, n in spawner_records(p):
        put(a, n, "struct")
    # textos a máquina de la presentación de misión (+ sus scripts de sprite)
    fx_tabs, fx_scripts = fix_type_streams(p)
    for a, n in fx_tabs:
        put(a, n, "struct")
    # scripts instalados desde el código (EntitySetSpriteMap / move.l #,$3C(a6))
    seen = set()
    cs_tabs, cs_scripts = code_script_tables(p)
    for a, n in cs_tabs:
        put(a, n, "struct")
    for v in sorted(set(code_script_seeds(p)) | set(fx_scripts) | cs_scripts):
        for a, n in walk_struct(p, v, seen) or []:
            put(a, n, "anim")
            op_starts.add(a)
    # op 19 SetField70: $70(a6) = operando+2 -> registro de 12 B (6 palabras con
    # signo: caja de colisión / ataque por frame; destinos con stride 12)
    for a in op_starts:
        if p.B(a) == 0x19 and p.valid(a + 2, 4):
            t = p.L(a + 2)
            if t & 1 == 0 and p.valid(t, 12):
                put(t, 12, "struct")
    # PathScripts (VM de movimiento, +$90 de la tarea) y sus tablas
    for a, n, k in path_scripts(p):
        put(a, n, k)
    # scripts apuntados por algún long y recorribles de punta a punta
    for a, n in pointed_scripts(p, mark):
        put(a, n, "anim")
    # cadenas de descriptores {w, h, tiles} de parches (+ sus tilemaps)
    for a, n, k in desc_chains(p):
        put(a, n, k)
    # rachas completas de frames sin puntero directo
    for a, n in frame_runs(p, mark):
        put(a, n, "frame")
    # celdas de tilemap sin descriptor
    for a, n in tilemap_gaps(p, mark):
        put(a, n, "tilemap")
    # tablas de punteros a datos ya clasificados (dos pasadas: tablas de tablas)
    for _ in range(2):
        for a, n in pointer_tables(p, mark):
            put(a, n, "struct")
    return mark


def census(p, mark):
    rows = []
    for f, (lo, hi) in modules():
        c = Counter()
        zrun = 0
        for o in range(lo, hi):            # registry = offsets del fichero de 2 MiB
            k = mark.get(p.cpu(o))
            if k is None:
                if p.d[o] == 0:
                    zrun += 1
                    continue
                k = "other"
            if zrun:
                c["zero" if zrun >= 16 else "other"] += zrun
                zrun = 0
            c[k] += 1
        if zrun:
            c["zero" if zrun >= 16 else "other"] += zrun
        rows.append((f, lo, hi, c))
    return rows


def main():
    p = S.PRom()
    rows = census(p, classify(p))
    hdr = "%-34s %-17s %8s " % ("módulo", "región", "bytes") + " ".join("%9s" % k for k in KINDS)
    print(hdr)
    print("-" * len(hdr))
    tot = Counter()
    for f, lo, hi, c in rows:
        tot.update(c)
        print("%-34s $%06X-$%06X %8d " % (f, lo, hi, hi - lo)
              + " ".join("%8.1f%%" % (100 * c[k] / (hi - lo)) for k in KINDS))
    n = sum(tot.values())
    print("%-34s %-17s %8d " % ("TOTAL", "", n) + " ".join("%8.1f%%" % (100 * tot[k] / n) for k in KINDS))
    if "--json" in sys.argv:
        out = sys.argv[sys.argv.index("--json") + 1]
        json.dump([dict(module=f, start="%06X" % lo, end="%06X" % hi,
                        bytes={k: c[k] for k in KINDS}) for f, lo, hi, c in rows],
                  open(out, "w"), indent=1)


if __name__ == "__main__":
    main()
