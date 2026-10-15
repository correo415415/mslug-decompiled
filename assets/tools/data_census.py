#!/usr/bin/env python3
"""
data_census.py - censo de los módulos de datos del P-ROM (asm/data/*.s).

Clasifica cada byte de las 14 regiones de datos según el tipo de asset que
el código del juego le da, usando los mismos decodificadores que los
extractores (sprites.py, levels.py, scene_script_dump.py):

  palette     paletas de 64 B ($14E00 x1024, $1CE00 x512)
  frame       frames de sprite {npiezas, attr, piezas{dx,dy,w,h,tiles}}
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
  lut         tablas generadas por fórmula (ColorRGB555ToNeoGeo, ColorRamp)
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
                   "SceneScript_", "SceneEntities_", "SceneTrig_")

# tablas de color del motor de paletas (asm/data/sprite_tables_002f30.s)
LUTS = ((0x002F30, 0x012F30), (0x012F30, 0x012FB0), (0x012FB0, 0x0133B0))


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
            t = (p.L(q + 2) if x == 0x4EB9 else
                 q + 2 + p.SW(q + 2) if x in (0x4EBA, 0x6100) else None)
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
        if op == 0x02:
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
        if op in (0x1D, 0x1E):
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


ONSCREEN_BOX = (0x05DD56, 0x05DD5C)  # a0 = caja {dx0, dx1, dy0, dy1} (8 B)
PROBE_GRID = 0x043FAC                # a1 = {dx, dy, w, h} + w*h bytes


def code_arg_tables(p, code_end=0x90000):
    """-> [(addr, len)] de tablas pasadas en a0/a1 a rutinas de formato fijo:
    `lea X,aN` seguido (<= 12 B) de jsr/bsr a ONSCREEN_BOX (caja de 8 B,
    a0) o a PROBE_GRID (rejilla de bytes, a1)."""
    out = []
    for o in range(0, code_end - 8, 2):
        w = p.W(o)
        if w in (0x41F9, 0x43F9):
            v, n, reg = p.L(o + 2), 6, (w >> 9) & 7
        elif w in (0x41FA, 0x43FA):
            v, n, reg = o + 2 + p.SW(o + 2), 4, (w >> 9) & 7
        else:
            continue
        for q in range(o + n, o + n + 12, 2):
            x = p.W(q)
            t = (p.L(q + 2) if x == 0x4EB9 else
                 q + 2 + p.SW(q + 2) if x in (0x4EBA, 0x6100) else None)
            if t is None:
                continue
            if v & 1 == 0 and p.valid(v, 8):
                if t in ONSCREEN_BOX and reg == 0:
                    out.append((v, 8))
                elif t == PROBE_GRID and reg == 1:
                    gw, gh = p.W(v + 4), p.W(v + 6)
                    if 0 < gw * gh <= 4096:
                        out.append((v, 8 + gw * gh))
            break
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


def classify(p):
    """-> dict cpu_addr -> kind (sólo bytes marcados)."""
    mark = {}

    def put(a, n, k):
        for x in range(a, a + n):
            mark.setdefault(x, k)

    # tablas por fórmula
    for lo, hi in LUTS:
        put(lo, hi - lo, "lut")
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
    # diccionarios de nombres de la pantalla de resultados
    for a, n in roster_names(p):
        put(a, n, "struct")
    # tablas de argumentos de formato fijo (cajas en pantalla, rejillas de sondeo)
    for a, n in code_arg_tables(p):
        put(a, n, "struct")
    # scripts instalados desde el código (EntitySetSpriteMap / move.l #,$3C(a6))
    seen = set()
    for v in sorted(code_script_seeds(p)):
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
