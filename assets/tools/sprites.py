#!/usr/bin/env python3
"""
sprites.py - sprites de Metal Slug montados como los dibuja el juego.

Lee el P-ROM (build/mslug_prom.bin, orden CPU) y los C-ROM (rom/201-c1..c4)
y exporta, con las paletas reales:

  palettes/   las 1536 paletas del P-ROM ($14E00: 1024, $1CE00: 512) en PNG+JSON
  frames/     los frames multi-pieza (formato de SpriteDispatchJT_05AA96)
  anims/      las animaciones (byte-code de Script_OpcodeTable $28CF0) en GIF
              con los tiempos reales (frames de 59,185 Hz) + tira PNG,
              nombradas por la rutina que las usa; anims.json e index.html

Formatos (ver assets/README.md):
  paleta  : 16 x {r, g, b, 0}, canales de 5 bits; el color 0 es transparente.
  frame   : u8 npiezas (1..40), u8 attr (banco<<4); piezas
            {s16 dx, s16 dy (hacia ARRIBA), u8 w, u8 h, u16 tiles[w*h]} con
            los tiles por columnas (h por columna); nº tile = banco<<16 | tile.
  script  : 00 n espera n frames · 01 .l salto · 02 fl .l prio.w [dx.w dy.w]
            (fl bit0 flip H, bit1 flip V, bit3 = sin dx/dy -> 8 B en vez de 12;
            varios 02 seguidos forman una sola imagen) · 16 repite la imagen ·
            03 n .l bucle · 1D/1B/1C/1E terminan el recorrido estático.

    python3 assets/tools/sprites.py extract                 # -> build/assets/sprites/
    python3 assets/tools/sprites.py render SCRIPT_HEX [PAL] [--scale N]
"""
import bisect, glob, html, json, os, re, struct, sys
from collections import Counter, defaultdict
import numpy as np
from PIL import Image

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, HERE)
import gfx  # noqa: E402

PROM = os.path.join(ROOT, "build", "mslug_prom.bin")
OUT = os.path.join(ROOT, "build", "assets", "sprites")
MANUAL = os.path.join(ROOT, "assets", "map", "anim_palettes.txt")
FPS = 59.185
PAL_LO, PAL_HI = 0x14E00, 0x1CE00
N_TILES = 0x20000
OP_SIZE = {0x03: 6, 0x04: 4, 0x05: 4, 0x06: 6, 0x07: 8, 0x08: 6, 0x09: 6,
           0x0A: 6, 0x0B: 6, 0x0C: 2, 0x0D: 4, 0x0E: 6, 0x0F: 8, 0x10: 4,
           0x11: 6, 0x12: 8, 0x13: 4, 0x14: 6, 0x15: 8, 0x17: 2, 0x18: 2,
           0x19: 10, 0x1A: 2, 0x1F: 4}
STOP_OPS = {0x1B, 0x1C, 0x1D}
MAX_W, MAX_H = 1024, 768


# ---- P-ROM -------------------------------------------------------------------
class PRom:
    def __init__(self, path=PROM):
        self.d = open(path, "rb").read()

    def off(self, a):
        if 0 <= a < 0x100000:
            return a
        if 0x200000 <= a < 0x300000:
            return a - 0x100000
        return None

    def valid(self, a, n=1):
        o = self.off(a)
        return o is not None and self.off(a + n - 1) is not None

    def B(self, a):
        return self.d[self.off(a)]

    def W(self, a):
        o = self.off(a)
        return (self.d[o] << 8) | self.d[o + 1]

    def SW(self, a):
        v = self.W(a)
        return v - 0x10000 if v & 0x8000 else v

    def L(self, a):
        return (self.W(a) << 16) | self.W(a + 2)

    def cpu(self, o):
        return o if o < 0x100000 else o + 0x100000


def palettes(p):
    out = []
    for base, n in ((PAL_LO, 1024), (PAL_HI, 512)):
        raw = np.frombuffer(p.d[base:base + n * 64], np.uint8).reshape(n, 16, 4)
        out.append((raw[:, :, :3].astype(np.uint32) * 255 // 31).astype(np.uint8))
    return np.concatenate(out)          # (1536, 16, 3); 0..1023 = $14E00


# ---- frames ------------------------------------------------------------------
_frame_cache = {}


def parse_frame(p, a):
    """-> {'attr', 'pieces': [(dx, dy, w, h, [tiles])], 'size'} o None."""
    if a in _frame_cache:
        return _frame_cache[a]
    r = None
    if a & 1 == 0 and p.valid(a, 2):
        n, attr = p.B(a), p.B(a + 1)
        if 1 <= n <= 40 and attr & 0x0F == 0 and attr >> 4 <= 1:
            pieces, q, ok = [], a + 2, True
            for _ in range(n):
                if not p.valid(q, 6):
                    ok = False
                    break
                dx, dy, w, h = p.SW(q), p.SW(q + 2), p.B(q + 4), p.B(q + 5)
                if not (1 <= w <= 20 and 1 <= h <= 32 and -512 < dx < 512 and -512 < dy < 512):
                    ok = False
                    break
                q += 6
                if not p.valid(q, 2 * w * h):
                    ok = False
                    break
                tiles = [p.W(q + 2 * i) for i in range(w * h)]
                q += 2 * w * h
                pieces.append((dx, dy, w, h, tiles))
            if ok:
                r = {"attr": attr, "pieces": pieces, "size": q - a}
    _frame_cache[a] = r
    return r


def op02_at(p, a, op=0x02):
    """Valida un `02 fl .l prio.w [dx dy]` en a -> (frame, flags, dx, dy, len).
    op=0x1E valida un SpawnChild_Sprite (mismo formato)."""
    if not p.valid(a, 8) or p.B(a) != op:
        return None
    fl = p.B(a + 1)
    if fl & 0xE0:
        return None
    fr = p.L(a + 2)
    if fr & 1 or not p.valid(fr, 2) or parse_frame(p, fr) is None:
        return None
    if fl & 8:
        return fr, fl, 0, 0, 8
    if not p.valid(a, 12):
        return None
    return fr, fl, p.SW(a + 8), p.SW(a + 10), 12


def find_frames(p):
    """Todas las posiciones de op 02 válidas y los frames que referencian
    (también los de op 1E, que no entran en ops)."""
    ops, frames = {}, set()
    d = p.d
    for o in range(0, len(d) - 12, 2):
        if d[o] == 0x02:
            a = p.cpu(o)
            r = op02_at(p, a)
            if r:
                ops[a] = r
                frames.add(r[0])
        elif d[o] == 0x1E:
            r = op02_at(p, p.cpu(o), 0x1E)
            if r:
                frames.add(r[0])
    return ops, frames


# ---- intérprete de scripts ---------------------------------------------------
def walk(p, start, max_steps=600, max_pics=200):
    """Recorre el script desde start -> (pics, info). pics = [(parts, frames)]
    con parts = [(frame, flags, dx, dy)]; info: ops visitadas, fin, bucle."""
    pics, a, wait, visited, loops = [], start, 0, [], {}
    end, loop_to = "stop", None
    last = None
    for _ in range(max_steps):
        if not p.valid(a, 2) or a & 1:
            end = "invalid"
            break
        op = p.B(a)
        visited.append(a)
        if op == 0x00:
            wait = p.B(a + 1)
            a += 2
        elif op == 0x01:
            if not p.valid(a, 6):
                end = "invalid"
                break
            t = p.L(a + 2)
            if t in visited:
                end, loop_to = "loop", t
                break
            a = t
        elif op in (0x02, 0x17, 0x18, 0x16):
            if op in (0x17, 0x18):
                a += 2
                if p.valid(a, 1) and p.B(a) != 0x02:
                    end = "invalid"
                    break
            if op == 0x16:
                if last is None:
                    end = "invalid"
                    break
                parts = last
                a += 2
            else:
                parts = []
                while True:
                    r = op02_at(p, a)
                    if not r:
                        break
                    visited.append(a)
                    parts.append(r[:4])
                    a += r[4]
                if not parts:
                    end = "invalid"
                    break
            pics.append((parts, max(wait, 1)))
            last, wait = parts, 0
            if len(pics) >= max_pics:
                end = "long"
                break
        elif op == 0x03:
            if not p.valid(a, 6):
                end = "invalid"
                break
            cnt, t = p.B(a + 1), p.L(a + 2)
            k = loops.get(a, cnt)
            if k > 0 and t in visited and len(pics) < max_pics:
                loops[a] = k - 1
                a = t
            else:
                loops.pop(a, None)
                a += 6
        elif op == 0x1E:
            # SpawnChild_Sprite: mismo formato que 02 (8/12 B según fl&8); el
            # script del padre continúa.
            if not p.valid(a, 8):
                end = "invalid"
                break
            a += 8 if p.B(a + 1) & 8 else 12
        elif op in OP_SIZE:
            a += OP_SIZE[op]
        elif op in STOP_OPS:
            end = "op%02X" % op
            break
        else:
            end = "invalid"
            break
    return pics, {"visited": visited, "end": end, "loop_to": loop_to}


# ---- tiles -------------------------------------------------------------------
class Tiles:
    CH = 4096

    def __init__(self):
        rom = os.path.join(ROOT, "rom")
        self.c = [open(os.path.join(rom, "201-c%d.bin" % i), "rb").read() for i in range(1, 5)]
        self.cache = {}

    def get(self, t):
        ch = t // self.CH
        if ch not in self.cache:
            pair, i = divmod(ch * self.CH, 0x10000)
            odd, even = self.c[2 * pair], self.c[2 * pair + 1]
            self.cache[ch] = gfx.spr_decode(odd[i * 64:(i + self.CH) * 64],
                                            even[i * 64:(i + self.CH) * 64])
        return self.cache[ch][t % self.CH]


def frame_layers(fr, flags, dx, dy):
    """-> [(x, y, tile, fh, fv)] en coordenadas de pantalla (y hacia abajo)."""
    bank = fr["attr"] >> 4
    fh, fv = flags & 1, (flags >> 1) & 1
    out = []
    for px, py, w, h, tiles in fr["pieces"]:
        for c in range(w):
            for r in range(h):
                t = tiles[c * h + r]
                x, y = px + 16 * c, -py + 16 * r
                if fh:
                    x = -x - 16
                if fv:
                    y = -y - 16
                out.append((x + (-dx if fh else dx), y - (-dy if fv else dy),
                            (bank << 16) | t, fh, fv))
    return out


def compose(p, tiles, parts):
    """parts -> (layers, bbox)."""
    lay = []
    for fa, fl, dx, dy in parts:
        lay += frame_layers(parse_frame(p, fa), fl, dx, dy)
    if not lay:
        return lay, None
    xs = [l[0] for l in lay]
    ys = [l[1] for l in lay]
    return lay, (min(xs), min(ys), max(xs) + 16, max(ys) + 16)


def draw(tiles, layers, box, canvas_box):
    x0, y0, x1, y1 = canvas_box
    img = np.zeros((y1 - y0, x1 - x0), np.uint8)
    for x, y, t, fh, fv in layers:          # orden de la lista = orden de dibujo
        tl = tiles.get(t % N_TILES)
        if fh:
            tl = tl[:, ::-1]
        if fv:
            tl = tl[::-1, :]
        sub = img[y - y0:y - y0 + 16, x - x0:x - x0 + 16]
        m = tl != 0
        sub[m] = tl[m]
    return img


def to_image(idx, pal, scale=1):
    im = Image.fromarray(idx, "P")
    flat = [int(v) for v in pal.reshape(-1)] + [0] * (768 - 48)
    im.putpalette(flat)
    if scale > 1:
        im = im.resize((im.width * scale, im.height * scale), Image.NEAREST)
    return im


def rgba(idx, pal):
    out = np.zeros(idx.shape + (4,), np.uint8)
    out[..., :3] = pal[idx]
    out[..., 3] = np.where(idx == 0, 0, 255)
    return Image.fromarray(out, "RGBA")


GREY = np.array([[v, v, v] for v in range(0, 256, 17)], np.uint8)


# ---- nombres y paletas -------------------------------------------------------
LABEL_RE = re.compile(r"^([A-Za-z][A-Za-z0-9_]*?_([0-9a-fA-F]{5,6})):", re.M)


def load_labels():
    labs = {}
    for f in glob.glob(os.path.join(ROOT, "asm", "**", "*.s"), recursive=True):
        for m in LABEL_RE.finditer(open(f, encoding="utf-8", errors="replace").read()):
            name, a = m.group(1), int(m.group(2), 16)
            if "__L" in name:
                continue
            labs.setdefault(a, name)
    keys = sorted(labs)
    return keys, labs


def is_code_ref(p, loc):
    """¿el long en loc es el operando absoluto de lea/move.l #imm?"""
    if loc < 2:
        return False
    op = p.W(loc - 2)
    return (op & 0xF1FF) == 0x41F9 or (op & 0xF03F) == 0x203C


PALLOAD = bytes.fromhex("323C")      # move.w #pal,d1 ; jsr $236E.l
PALJSR = bytes.fromhex("4EB90000236E")


def pal_loads(p):
    """CPU addr -> paleta de cada `move.w #pal,d1; jsr $236E`."""
    out, d, i = {}, p.d, 0
    while True:
        i = d.find(PALJSR, i)
        if i < 0:
            break
        if i >= 4 and d[i - 4:i - 2] == PALLOAD:
            out[p.cpu(i - 4)] = (d[i - 2] << 8) | d[i - 1]
        i += 2
    return out


def owners(p, scripts, frames_of, labels):
    """script -> {'owner', 'pal', 'how'}."""
    keys, labs = labels
    pl = pal_loads(p)
    pl_keys = sorted(pl)
    sset = set(scripts)
    refs = defaultdict(list)          # script -> [loc]
    d = p.d
    for o in range(0, len(d) - 4, 2):
        v = (d[o] << 24) | (d[o + 1] << 16) | (d[o + 2] << 8) | d[o + 3]
        if v in sset:
            refs[v].append(p.cpu(o))
    code_refs = []                    # (target, loc) de todos los lea/move #abs
    for o in range(2, len(d) - 4, 2):
        op = (d[o - 2] << 8) | d[o - 1]
        if (op & 0xF1FF) == 0x41F9 or (op & 0xF03F) == 0x203C:
            v = (d[o] << 24) | (d[o + 1] << 16) | (d[o + 2] << 8) | d[o + 3]
            if p.valid(v):
                code_refs.append((v, p.cpu(o)))
    code_refs.sort()
    ct = [c[0] for c in code_refs]

    def routine(loc):
        i = bisect.bisect_right(keys, loc) - 1
        return (keys[i], labs[keys[i]]) if i >= 0 else (0, "Unknown")

    def pal_near(loc):
        start, _ = routine(loc)
        lo = max(start, loc - 0x400)
        i = bisect.bisect_right(pl_keys, loc) - 1
        if i >= 0 and pl_keys[i] >= lo:
            return pl[pl_keys[i]]
        j = bisect.bisect_right(pl_keys, loc)
        if j < len(pl_keys) and pl_keys[j] < loc + 0x400 and routine(pl_keys[j])[0] == start:
            return pl[pl_keys[j]]
        # la carga suele estar en la rutina Init vecina: la más cercana a <0x300
        best = None
        for k in (i, j):
            if 0 <= k < len(pl_keys) and abs(pl_keys[k] - loc) < 0x300:
                if best is None or abs(pl_keys[k] - loc) < abs(best - loc):
                    best = pl_keys[k]
        return pl[best] if best is not None else None

    def via_table(loc):
        i = bisect.bisect_right(ct, loc) - 1
        while i >= 0 and ct[i] >= loc - 0x800:
            t, cl = code_refs[i]
            if not (t <= cl <= t + 0x800):        # que no sea la propia tabla
                return cl
            i -= 1
        return None

    res = {}
    for s in scripts:
        best = None
        for loc in refs.get(s, []):
            if is_code_ref(p, loc):
                r = {"owner": routine(loc)[1], "pal": pal_near(loc), "how": "code", "ref": loc}
            else:
                cl = via_table(loc)
                if cl is None:
                    continue
                r = {"owner": routine(cl)[1], "pal": pal_near(cl), "how": "table", "ref": loc}
            if best is None or (best["pal"] is None and r["pal"] is not None):
                best = r
        res[s] = best or {"owner": None, "pal": None, "how": None, "ref": None}
    # herencia: frames compartidos
    for _ in range(3):
        fpal = {}
        for s, r in res.items():
            if r["pal"] is not None:
                for f in frames_of[s]:
                    fpal.setdefault(f, r["pal"])
        for s, r in res.items():
            if r["pal"] is None:
                for f in frames_of[s]:
                    if f in fpal:
                        r["pal"], r["how"] = fpal[f], (r["how"] or "") + "+inherited"
                        break
    # tiles compartidos: un script sin paleta toma la del script con paleta
    # cuyos tiles se solapan más (Jaccard >= 0.3; mismos gráficos = mismo
    # personaje/efecto)
    def tiles_of(s):
        t = set()
        for f in frames_of[s]:
            fr = parse_frame(p, f)
            if fr:
                for piece in fr["pieces"]:
                    t.update(x & 0xFFFF for x in piece[4] if x)
        return t
    known = [(s, tiles_of(s)) for s, r in res.items() if r["pal"] is not None]
    by_tile = defaultdict(list)
    for i, (s, t) in enumerate(known):
        for x in t:
            by_tile[x].append(i)
    for s, r in res.items():
        if r["pal"] is not None:
            continue
        t = tiles_of(s)
        if not t:
            continue
        cnt = Counter(i for x in t for i in by_tile.get(x, ()))
        best, bj = None, 0.0
        for i, n in cnt.items():
            j = n / len(t | known[i][1])
            if j > bj:
                best, bj = i, j
        if best is not None and bj >= 0.3:
            r["pal"], r["how"] = res[known[best][0]]["pal"], (r["how"] or "") + "+tiles"
    # vecino: el script con paleta más cercano en la ROM (≤ $800; los datos de
    # cada entidad van juntos)
    ks = sorted(s for s, r in res.items() if r["pal"] is not None)
    for s, r in res.items():
        if r["pal"] is not None or not ks:
            continue
        i = bisect.bisect_left(ks, s)
        cands = [ks[k] for k in (i - 1, i) if 0 <= k < len(ks)]
        n = min(cands, key=lambda x: abs(x - s))
        if abs(n - s) <= 0x800:
            r["pal"], r["how"] = res[n]["pal"], (r["how"] or "") + "+near"
    if os.path.exists(MANUAL):
        for line in open(MANUAL, encoding="utf-8"):
            line = line.split("#")[0].split()
            if len(line) >= 2:
                s = int(line[0], 16)
                if s in res:
                    res[s]["pal"], res[s]["how"] = int(line[1]), "manual"
    return res


# ---- búsqueda de scripts ------------------------------------------------------
def find_scripts(p, ops):
    """Scripts = destinos de punteros long (o de 01/03) cuyo recorrido es
    válido y muestra al menos una imagen; 'inner' = alcanzado desde otro."""
    d = p.d
    cand = set()
    opset = set(ops)
    for o in range(0, len(d) - 4, 2):
        v = (d[o] << 24) | (d[o + 1] << 16) | (d[o + 2] << 8) | d[o + 3]
        if v & 1 or not p.valid(v, 2):
            continue
        b = p.B(v)
        if v in opset or (b in (0x00, 0x03, 0x04, 0x05, 0x06, 0x07, 0x17, 0x18, 0x1F)
                          and p.valid(v, 2)):
            cand.add(v)
    by_ptr = set(cand)
    # cabezas de secuencias de op 02 (scripts alcanzados vía tablas indexadas
    # 1B/1C o punteros calculados): retrocede sobre un `00 n` / 17 / 18 previo
    for a in opset:
        if a - 12 in opset or a - 8 in opset:
            continue
        h = a
        if p.valid(a - 2, 2) and p.B(a - 2) in (0x00, 0x17, 0x18):
            h = a - 2
        cand.add(h)
    scripts = {}
    for s in sorted(cand):
        if s in _frame_cache and _frame_cache[s] is not None and s not in opset:
            continue
        pics, info = walk(p, s)
        if pics and info["end"] != "invalid":
            scripts[s] = (pics, info)
    vis = {s: set(info["visited"]) for s, (pics, info) in scripts.items()}
    inner = set()
    for s in scripts:
        for x in vis[s]:
            # x es interno si s llega a x y x no vuelve a s (en un ciclo
            # mutuo se conserva la entrada de dirección menor)
            if x != s and x in scripts and not (s in vis[x] and s > x):
                inner.add(x)
    # entradas sin puntero directo (tablas indexadas por 1B/1C: una pose por
    # ángulo/estado) -> se agrupan las contiguas en un único "set de poses"
    heads = sorted(x for x in scripts if x not in inner and x not in by_ptr
                   and len(scripts[x][0]) == 1)
    groups, cur = {}, None
    for x in heads:
        if cur is not None and x - last <= 0x40:
            groups[cur].append(x)
        else:
            cur = x
            groups[cur] = [x]
        last = max(vis[x])
    for g, members in groups.items():
        if len(members) > 1:
            pics = []
            for m in members:
                for parts, _ in scripts[m][0][:1]:
                    pics.append((parts, 8))
            for m in members[1:]:
                inner.add(m)
            scripts[g] = (pics, {"visited": sorted(set().union(*(vis[m] for m in members))),
                                 "end": "poses", "loop_to": None, "members": members})
    return scripts, inner


# ---- export --------------------------------------------------------------------
def save_anim(p, tiles, pics, pal, path_noext, scale=1):
    comp = [compose(p, tiles, parts) for parts, _ in pics]
    boxes = [b for _, b in comp if b]
    if not boxes:
        return None
    box = (min(b[0] for b in boxes), min(b[1] for b in boxes),
           max(b[2] for b in boxes), max(b[3] for b in boxes))
    w, h = box[2] - box[0], box[3] - box[1]
    if w > MAX_W or h > MAX_H:
        return None
    imgs = [draw(tiles, lay, b, box) for lay, b in comp]
    ims = [to_image(i, pal, scale) for i in imgs]
    durs = [max(20, round(n * 1000 / FPS / 10) * 10) for _, n in pics]
    ims[0].save(path_noext + ".gif", save_all=True, append_images=ims[1:],
                duration=durs, loop=0, transparency=0, disposal=2, optimize=False)
    strip = np.zeros((h, w * len(imgs)), np.uint8)
    for i, im in enumerate(imgs):
        strip[:, i * w:(i + 1) * w] = im
    rgba(strip, pal).save(path_noext + ".png")
    return {"box": box, "durations_frames": [n for _, n in pics]}


def extract():
    p = PRom()
    tiles = Tiles()
    pals = palettes(p)
    for sub in ("palettes", "frames", "anims"):
        os.makedirs(os.path.join(OUT, sub), exist_ok=True)
    # paletas
    img = np.zeros((1536, 16, 3), np.uint8)
    img[:] = pals
    Image.fromarray(np.repeat(np.repeat(img, 8, 0), 8, 1)).save(os.path.join(OUT, "palettes", "palettes.png"))
    json.dump({"note": "0..1023 = $14E00 + i*64 ; 1024..1535 = $1CE00 + (i-1024)*64",
               "palettes": pals.tolist()}, open(os.path.join(OUT, "palettes", "palettes.json"), "w"))
    print("[sprites] %d paletas" % len(pals))

    ops, frames = find_frames(p)
    scripts, inner = find_scripts(p, ops)
    frames_of = {s: {pt[0] for parts, _ in v[0] for pt in parts} for s, v in scripts.items()}
    labels = load_labels()
    own = owners(p, list(scripts), frames_of, labels)
    print("[sprites] %d frames, %d scripts (%d internos)" % (len(frames), len(scripts), len(inner)))

    # frames: paleta de la primera animación que lo usa
    fpal = {}
    for s in sorted(scripts):
        if own[s]["pal"] is not None:
            for f in frames_of[s]:
                fpal.setdefault(f, own[s]["pal"])
    fjson = {}
    for fa in sorted(frames):
        fr = parse_frame(p, fa)
        lay, box = compose(p, tiles, [(fa, 0, 0, 0)])
        pal = pals[fpal[fa]] if fa in fpal else GREY
        rgba(draw(tiles, lay, box, box), pal).save(os.path.join(OUT, "frames", "frame_%06X.png" % fa))
        fjson["%06X" % fa] = {"attr": fr["attr"], "size": fr["size"], "pal": fpal.get(fa),
                              "origin": [-box[0], -box[1]],
                              "pieces": [{"dx": a, "dy": b, "w": w, "h": h, "tiles": t}
                                         for a, b, w, h, t in fr["pieces"]]}
    json.dump(fjson, open(os.path.join(OUT, "frames", "frames.json"), "w"))

    ajson, rows, withpal = {}, [], 0
    for s in sorted(scripts):
        if s in inner:
            continue
        pics, info = scripts[s]
        o = own[s]
        name = "%s_%s%06X" % (o["owner"] or "Anim", "Poses_" if info["end"] == "poses" else "", s)
        pal = pals[o["pal"]] if o["pal"] is not None else GREY
        r = save_anim(p, tiles, pics, pal, os.path.join(OUT, "anims", name))
        if r is None:
            continue
        withpal += o["pal"] is not None
        ajson["%06X" % s] = dict(name=name, owner=o["owner"], pal=o["pal"], pal_how=o["how"],
                                 end=info["end"], loop_to=("%06X" % info["loop_to"]) if info["loop_to"] else None,
                                 pictures=[[{"frame": "%06X" % f, "flags": fl, "dx": dx, "dy": dy}
                                            for f, fl, dx, dy in parts] for parts, _ in pics], **r)
        rows.append(name)
    json.dump(ajson, open(os.path.join(OUT, "anims", "anims.json"), "w"), indent=1)
    with open(os.path.join(OUT, "anims", "index.html"), "w") as f:
        f.write("<!doctype html><meta charset=utf-8><title>Metal Slug - animaciones</title>"
                "<style>body{background:#446;color:#eee;font:12px sans-serif}"
                "div{display:inline-block;margin:6px;text-align:center;vertical-align:top}"
                "img{image-rendering:pixelated;background:#7a8;max-width:320px}</style>\n")
        for n in rows:
            f.write("<div><img src='%s.gif' loading=lazy><br>%s</div>\n" % (html.escape(n), html.escape(n)))
    print("[sprites] %d animaciones exportadas (%d con paleta) -> %s" % (len(rows), withpal, OUT))


def render_one(script, pal=None, scale=2):
    p = PRom()
    tiles = Tiles()
    pals = palettes(p)
    pics, info = walk(p, script)
    if not pics:
        sys.exit("script %06X no muestra imágenes (%s)" % (script, info["end"]))
    os.makedirs(os.path.join(ROOT, "build", "assets"), exist_ok=True)
    out = os.path.join(ROOT, "build", "assets", "render_%06X_p%s" % (script, pal))
    r = save_anim(p, tiles, pics, pals[pal] if pal is not None else GREY, out, scale)
    print("%s.gif  %d imágenes, fin=%s, %s" % (out, len(pics), info["end"], r and r["box"]))


if __name__ == "__main__":
    a = sys.argv[1:]
    if not a or a[0] not in ("extract", "render"):
        sys.exit(__doc__)
    if a[0] == "extract":
        extract()
    else:
        scale = 2
        if "--scale" in a:
            i = a.index("--scale")
            scale = int(a[i + 1])
            del a[i:i + 2]
        render_one(int(a[1], 16), int(a[2]) if len(a) > 2 else None, scale)
