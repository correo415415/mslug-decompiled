#!/usr/bin/env python3
"""
levels.py - fondos de Metal Slug (mapas de tiles de las escenas) a PNG.

Los fondos son capas de sprites de 16x16 que mueven los 4 contextos de cámara
($106F6C, $107FE8, $108064, $1080E0). Los scripts de escena
(SceneDescTable_0916C8 -> SceneScriptVM_Frame_0437DA) las enlazan con el
op $00 bind_path (22 B):

    00 00  ent.l  map.w w, map.w h, map.l tiles  cont.l  ancla_x.w ancla_y.w

  * {w, h, tiles} es el descriptor que lee CellCommit_MMIO_051DE2 ($4(a1) =
    puntero, $2(a1) = alto): w columnas de h celdas, POR COLUMNAS, cada celda
    {tile.w, attr.w} con attr = paleta<<8 | tile_msb<<4 | autoanim<<2 | flipV<<1 | flipH
    (formato SCB1 del Neo Geo, se copia tal cual a la VRAM).
  * La paleta es un slot de la tabla dinámica: el op $0A de la misma escena
    da pares {slot, banco} cargados con PalSlot_LoadListHi_002b58 desde el
    banco $1CE00 (64 B por paleta).
  * Los slots que la escena no define (p. ej. el 0) los fija el arranque/attract
    con ScriptSlotPairTable_0009B4 (también vía PalSlot_LoadListHi_002b58):
    se usan como último recurso (BOOT_SLOTS).
  * `cont` es el mapa de colisión (o 0); ancla = posición en el mundo.

    python3 assets/tools/levels.py          # -> build/assets/levels/
"""
import importlib.util, json, os, sys
import numpy as np
from PIL import Image

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, HERE)
import sprites as S  # noqa: E402

OUT = os.path.join(ROOT, "build", "assets", "levels")
CAMERAS = {0x106F6C: "cam0", 0x107FE8: "cam1", 0x108064: "cam2", 0x1080E0: "cam3"}
SCENE_NAMES = {0: "Mission1", 1: "Mission2", 2: "Mission3", 3: "Mission4", 4: "Mission5",
               5: "Mission6", 6: "Mission2_Alias", 7: "Mission3_Alias", 8: "Scene8",
               9: "Scene9_Cutscene", 10: "GameOver", 11: "HiScore", 12: "Scene12",
               13: "Scene13", 14: "Attract14", 15: "Attract15"}


def load_ssd():
    spec = importlib.util.spec_from_file_location(
        "ssd", os.path.join(ROOT, "tools", "scene_script_dump.py"))
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m


BOOT_SLOT_TABLE = 0x0009B4   # ScriptSlotPairTable_0009B4


def boot_slots(m):
    """Pares {id, banco} de ScriptSlotPairTable_0009B4 (hasta banco $FFFF / id $FFFF)."""
    out, q = {}, BOOT_SLOT_TABLE
    while m.W(q) != 0xFFFF and m.W(q + 2) != 0xFFFF:
        sid = m.W(q) & 0xFF
        if sid != 0xFF:
            out[sid] = m.W(q + 2)
        q += 4
    return out


def scene_layers(m, idx):
    """-> [layer dict] en el orden del script, con los slots de paleta vigentes."""
    start = m.L(m.TABLE + 8 * idx)
    ops, _ = m.decode_script(start, m.REGION_END)
    slots, out = {}, []
    for a, kind, n, op in ops:
        if kind != "op":
            continue
        if op == 0x0A:
            q = a + 2
            while m.W(q) != 0xFFFF:
                slots[m.W(q) & 0xFF] = m.W(q + 2)
                q += 4
        elif op == 0x00:
            ent = m.L(a + 2)
            if ent not in CAMERAS:
                continue
            out.append(dict(op_addr=a, camera=CAMERAS[ent], w=m.W(a + 6), h=m.W(a + 8),
                            tiles=m.L(a + 10), collision=m.L(a + 14),
                            anchor=(m.SW(a + 18), m.SW(a + 20)), slots=dict(slots)))
    return out


def render(p, tiles, pals, L, all_slots):
    w, h, base = L["w"], L["h"], L["tiles"]
    img = np.zeros((h * 16, w * 16, 4), np.uint8)
    missing = set()
    cache = {}
    for x in range(w):
        for y in range(h):
            o = base + 4 * (x * h + y)
            t, at = p.W(o), p.W(o + 2)
            tile = t | ((at >> 4) & 0xF) << 16
            slot = at >> 8
            bank = L["slots"].get(slot, all_slots.get(slot))
            tl = tiles.get(tile % S.N_TILES)
            if bank is None:
                if tl.any():                 # celdas vacías (tile transparente) no cuentan
                    missing.add(slot)
                pal = S.GREY
            else:
                pal = pals[1024 + bank] if bank < 512 else pals[bank - 512]
            if at & 1:
                tl = tl[:, ::-1]
            if at & 2:
                tl = tl[::-1, :]
            cell = img[y * 16:y * 16 + 16, x * 16:x * 16 + 16]
            cell[..., :3] = pal[tl]
            cell[..., 3] = np.where(tl == 0, 0, 255)
    return img, sorted(missing)


def main():
    m = load_ssd()
    p = S.PRom()
    tiles = S.Tiles()
    pals = S.palettes(p)
    os.makedirs(OUT, exist_ok=True)
    scenes = {i: scene_layers(m, i) for i in range(m.N_DESC)}
    all_slots = {}
    for ls in scenes.values():
        for L in ls:
            for k, v in L["slots"].items():
                all_slots.setdefault(k, v)
    for k, v in boot_slots(m).items():
        all_slots.setdefault(k, v)
    done, index, n = {}, {}, 0
    for i, layers in scenes.items():
        sname = "%02d_%s" % (i, SCENE_NAMES.get(i, "Scene%d" % i))
        entries = []
        for L in layers:
            key = (L["tiles"], L["w"], L["h"])
            fname = "%s_%s_%06X.png" % (sname, L["camera"], L["tiles"])
            if key in done:
                fname = done[key]
                missing = []
            else:
                img, missing = render(p, tiles, pals, L, all_slots)
                Image.fromarray(img, "RGBA").save(os.path.join(OUT, fname))
                done[key] = fname
                n += 1
            entries.append(dict(file=fname, camera=L["camera"], width=L["w"], height=L["h"],
                                tiles="%06X" % L["tiles"],
                                collision="%06X" % L["collision"] if L["collision"] else None,
                                anchor=L["anchor"], script_op="%06X" % L["op_addr"],
                                missing_palette_slots=missing))
        index[sname] = entries
    json.dump(index, open(os.path.join(OUT, "levels.json"), "w"), indent=1)
    with open(os.path.join(OUT, "index.html"), "w") as f:
        f.write("<!doctype html><meta charset=utf-8><title>Metal Slug - fondos</title>"
                "<style>body{background:#334;color:#eee;font:12px sans-serif}"
                "img{image-rendering:pixelated;max-width:100%;background:#000;display:block}"
                "h2{margin-top:2em}</style>\n")
        for s, es in index.items():
            if es:
                f.write("<h2>%s</h2>\n" % s)
                for e in es:
                    f.write("<p>%s · %s · %dx%d celdas · ancla %s</p><img src='%s' loading=lazy>\n"
                            % (e["file"], e["camera"], e["width"], e["height"], tuple(e["anchor"]), e["file"]))
    print("[levels] %d escenas, %d capas únicas -> %s" % (sum(1 for v in index.values() if v), n, OUT))


if __name__ == "__main__":
    main()
