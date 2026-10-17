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
import bisect, importlib.util, json, os, sys
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
    slots, out, scale = {}, [], {}
    for a, kind, n, op in ops:
        if kind != "op":
            continue
        if op == 0x01 and m.L(a + 2) in CAMERAS:     # set_fields: +$74/+$76 = escala 8.8
            scale[CAMERAS[m.L(a + 2)]] = (m.W(a + 8), m.W(a + 10))
        elif op == 0x0A:
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
                            anchor=(m.SW(a + 18), m.SW(a + 20)), slots=dict(slots),
                            scale=scale.get(CAMERAS[ent], (256, 256))))
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


def compose(scene_entries, images):
    """Una imagen por cámara: cada capa pegada en su ancla (coordenadas de mundo).

    Las capas de una misma cámara se enlazan una tras otra a lo largo de la
    misión (la cámara las recicla), así que juntas forman la tira completa del
    plano. Cada cámara tiene su propio scroll (parallax), por eso no se mezclan.
    """
    out = {}
    for cam in sorted({e["camera"] for e in scene_entries}):
        es = [e for e in scene_entries if e["camera"] == cam]
        x0 = min(e["anchor"][0] for e in es)
        y0 = min(e["anchor"][1] for e in es)
        x1 = max(e["anchor"][0] + 16 * e["width"] for e in es)
        y1 = max(e["anchor"][1] + 16 * e["height"] for e in es)
        canvas = Image.new("RGBA", (x1 - x0, y1 - y0), (0, 0, 0, 0))
        for e in es:                       # orden del script: lo posterior encima
            im = images[e["file"]]
            canvas.alpha_composite(im, (e["anchor"][0] - x0, e["anchor"][1] - y0))
        out[cam] = (canvas, (x0, y0))
    return out


SCREEN_W = 320


def compose_parallax(scene_entries, images):
    """Panorama de la escena tal como la ve el jugador, con todas las cámaras.

    El juego coloca cada cámara en (scroll principal * escala / 256)
    (CameraApplyOne_043DAA: $108160 * +$74/+$76 << 8; la escala 8.8 la fija el
    op $01 del script). Una capa lejana no tiene una única posición sobre el
    plano principal, así que se renderiza pantalla a pantalla: para cada
    scroll X (paso SCREEN_W) cada capa se pinta en ancla_x - X*escala/256, se
    recorta la ventana de SCREEN_W px y las ventanas se concatenan. El scroll
    vertical se ignora (y = ancla). Orden: escala menor primero (lo más
    lejano), luego orden del script.
    """
    if not scene_entries:
        return None, None
    main_ = [e for e in scene_entries if tuple(e["scale"]) == (256, 256)] or scene_entries
    xs0 = min(e["anchor"][0] for e in main_)
    xs1 = max(e["anchor"][0] + 16 * e["width"] for e in main_)
    y0 = min(e["anchor"][1] for e in scene_entries)
    y1 = max(e["anchor"][1] + 16 * e["height"] for e in scene_entries)
    order = sorted(scene_entries, key=lambda e: (e["scale"][0], e["script_op"]))
    nwin = max(1, -(-(xs1 - xs0) // SCREEN_W))
    canvas = Image.new("RGBA", (nwin * SCREEN_W, y1 - y0), (0, 0, 0, 0))
    for k in range(nwin):
        X = xs0 + k * SCREEN_W
        win = Image.new("RGBA", (SCREEN_W, y1 - y0), (0, 0, 0, 0))
        for e in order:
            lx = e["anchor"][0] - X * e["scale"][0] // 256
            w = 16 * e["width"]
            if lx >= SCREEN_W or lx + w <= 0:
                continue
            im = images[e["file"]]
            if lx < 0:
                im, lx = im.crop((-lx, 0, min(w, SCREEN_W - lx), 16 * e["height"])), 0
            win.alpha_composite(im, (lx, e["anchor"][1] - y0))
        canvas.paste(win, (k * SCREEN_W, 0))
    return canvas, (xs0, y0)


SCREEN_H = 224


def camera_track(entries, step=SCREEN_W // 4):
    """Trayectoria (X, Y) de la cámara principal deducida de las capas del
    plano principal (escala 256): para cada X la Y válida es la que deja la
    pantalla dentro de alguna capa de cam0 que cubre ese X ([ancla_y,
    ancla_y + alto - SCREEN_H]); de las válidas se toma la más cercana a la
    anterior (la cámara se mueve lo mínimo, como el seguimiento suavizado de
    Camera_SmoothingIntegrate). Los límites del op $0F no se usan
    directamente: el orden de los ops del script no es el de ejecución
    (esperas por progreso), y las capas ya codifican por dónde pasa la
    cámara. -> [(x, y)] cada `step` px."""
    main_ = [e for e in entries if tuple(e["scale"]) == (256, 256)] or entries
    cam0 = [e for e in main_ if e["camera"] == "cam0"] or main_
    x0 = min(e["anchor"][0] for e in main_)
    x1 = max(e["anchor"][0] + 16 * e["width"] for e in main_)
    pts, y = [], None
    for x in range(x0, x1 + 1, step):
        ivs = []
        for e in cam0:
            ax, ay = e["anchor"]
            if ax <= x < ax + 16 * e["width"]:
                lo, hi = ay, ay + 16 * e["height"] - SCREEN_H
                ivs.append((min(lo, hi), max(lo, hi)))
        if not ivs:
            pts.append((x, y if y is not None else 0))
            continue
        if y is None:
            y = max(hi for lo, hi in ivs)          # empieza abajo (suelo)
        y = min((min(max(y, lo), hi) for lo, hi in ivs), key=lambda v: abs(v - y))
        pts.append((x, y))
    return pts


def compose_scroll(scene_entries, images, track):
    """Como compose_parallax pero con scroll vertical: cada ventana de
    SCREEN_W x SCREEN_H se recorta a la Y de la cámara en ese X (track de
    camera_track) y cada capa se desplaza Y * escala_y / 256."""
    if not scene_entries:
        return None
    main_ = [e for e in scene_entries if tuple(e["scale"]) == (256, 256)] or scene_entries
    xs0 = min(e["anchor"][0] for e in main_)
    xs1 = max(e["anchor"][0] + 16 * e["width"] for e in main_)
    order = sorted(scene_entries, key=lambda e: (e["scale"][0], e["script_op"]))
    nwin = max(1, -(-(xs1 - xs0) // SCREEN_W))
    tx = [t[0] for t in track]

    def cam_y(X):                       # interpolación lineal entre puntos
        i = bisect.bisect_right(tx, X) - 1
        if i < 0:
            return track[0][1]
        if i + 1 >= len(track) or track[i + 1][0] == track[i][0]:
            return track[i][1]
        (x0, y0), (x1, y1) = track[i], track[i + 1]
        return y0 + (y1 - y0) * (X - x0) // (x1 - x0)
    canvas = Image.new("RGBA", (nwin * SCREEN_W, SCREEN_H), (0, 0, 0, 255))
    for k in range(nwin):
        X = xs0 + k * SCREEN_W
        Y = cam_y(X + SCREEN_W // 2)
        win = Image.new("RGBA", (SCREEN_W, SCREEN_H), (0, 0, 0, 0))
        for e in order:
            lx = e["anchor"][0] - X * e["scale"][0] // 256
            ly = e["anchor"][1] - Y * e["scale"][1] // 256
            w, h = 16 * e["width"], 16 * e["height"]
            if lx >= SCREEN_W or lx + w <= 0 or ly >= SCREEN_H or ly + h <= 0:
                continue
            im = images[e["file"]]
            cx, cy = max(0, -lx), max(0, -ly)
            im = im.crop((cx, cy, min(w, SCREEN_W - lx), min(h, SCREEN_H - ly)))
            win.alpha_composite(im, (max(lx, 0), max(ly, 0)))
        canvas.alpha_composite(win, (k * SCREEN_W, 0))
    return canvas


def render_patches(p, tiles, pals, all_slots):
    """Mapas fuente de los parches de tilemap (registros de Sprite_InvokeBlit8Params,
    ver data_census.blit_patches): cada registro copia un rectángulo de un
    descriptor {w, h, tiles} a la capa de la cámara `ent` (puertas, restos de
    edificios destruidos, etc.). Se renderizan las columnas que algún registro
    copia; las paletas salen de los slots conocidos de todas las escenas."""
    import data_census as D
    os.makedirs(os.path.join(OUT, "patches"), exist_ok=True)
    by = {}
    for rec, desc, w, h, t, ncol in D.blit_patches(p):
        by.setdefault((desc, w, h, t, ncol), []).append(rec)
    out = []
    for (desc, w, h, t, ncol), recs in sorted(by.items()):
        L = dict(w=ncol, h=h, tiles=t, slots={})
        img, missing = render(p, tiles, pals, L, all_slots)
        fname = "patches/patch_%06X.png" % desc
        Image.fromarray(img, "RGBA").save(os.path.join(OUT, fname))
        recs = sorted(recs)
        out.append(dict(file=fname, desc="%06X" % desc, width=w, height=h, columns=ncol,
                        tiles="%06X" % t,
                        camera=CAMERAS.get(p.L(recs[0]), "%06X" % p.L(recs[0])),
                        records=[dict(rec="%06X" % r, dst=(p.W(r + 8), p.W(r + 10)),
                                      src=(p.W(r + 12), p.W(r + 14)),
                                      size=(p.W(r + 16), p.W(r + 18))) for r in recs],
                        missing_palette_slots=missing))
    return out


def main():
    m = load_ssd()
    p = S.PRom()
    tiles = S.Tiles()
    pals = S.palettes(p)
    os.makedirs(os.path.join(OUT, "composed"), exist_ok=True)
    composed = {}
    scenes = {i: scene_layers(m, i) for i in range(m.N_DESC)}
    all_slots = {}
    for ls in scenes.values():
        for L in ls:
            for k, v in L["slots"].items():
                all_slots.setdefault(k, v)
    for k, v in boot_slots(m).items():
        all_slots.setdefault(k, v)
    done, index, n, images = {}, {}, 0, {}
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
                images[fname] = Image.fromarray(img, "RGBA")
                images[fname].save(os.path.join(OUT, fname))
                done[key] = fname
                n += 1
            entries.append(dict(file=fname, camera=L["camera"], width=L["w"], height=L["h"],
                                tiles="%06X" % L["tiles"],
                                collision="%06X" % L["collision"] if L["collision"] else None,
                                anchor=L["anchor"], script_op="%06X" % L["op_addr"],
                                scale=L["scale"],
                                missing_palette_slots=missing))
        index[sname] = entries
        if entries:
            pim, porg = compose_parallax(entries, images)
            if pim is not None:
                pname = "%s_parallax.png" % sname
                pim.save(os.path.join(OUT, "composed", pname))
                composed.setdefault(sname, []).append(dict(file="composed/" + pname, camera="all",
                                                           origin=porg, size=pim.size))
                track = camera_track(entries)
                sim = compose_scroll(entries, images, track)
                sname2 = "%s_scroll.png" % sname
                sim.save(os.path.join(OUT, "composed", sname2))
                composed.setdefault(sname, []).append(dict(
                    file="composed/" + sname2, camera="all+scrollY", origin=porg, size=sim.size,
                    track=[list(t) for t in track]))
            for cam, (im, org) in compose(entries, images).items():
                cname = "%s_%s_full.png" % (sname, cam)
                im.save(os.path.join(OUT, "composed", cname))
                composed.setdefault(sname, []).append(dict(file="composed/" + cname, camera=cam,
                                                           origin=org, size=im.size))
    patches = render_patches(p, tiles, pals, all_slots)
    json.dump(dict(layers=index, composed=composed, patches=patches),
              open(os.path.join(OUT, "levels.json"), "w"), indent=1)
    with open(os.path.join(OUT, "index.html"), "w") as f:
        f.write("<!doctype html><meta charset=utf-8><title>Metal Slug - fondos</title>"
                "<style>body{background:#334;color:#eee;font:12px sans-serif}"
                "img{image-rendering:pixelated;max-width:100%;background:#000;display:block}"
                "h2{margin-top:2em}</style>\n")
        for s, es in index.items():
            if es:
                f.write("<h2>%s</h2>\n" % s)
                for c in composed.get(s, []):
                    f.write("<p><b>%s compuesto</b> · origen %s · %dx%d px</p><img src='%s' loading=lazy>\n"
                            % (c["camera"], tuple(c["origin"]), c["size"][0], c["size"][1], c["file"]))
                for e in es:
                    f.write("<p>%s · %s · %dx%d celdas · ancla %s</p><img src='%s' loading=lazy>\n"
                            % (e["file"], e["camera"], e["width"], e["height"], tuple(e["anchor"]), e["file"]))
        if patches:
            f.write("<h2>Parches (Sprite_InvokeBlit8Params)</h2>\n")
            for e in patches:
                f.write("<p>desc $%s · %dx%d celdas (%d copiadas) · %d registros</p><img src='%s' loading=lazy>\n"
                        % (e["desc"], e["width"], e["height"], e["columns"], len(e["records"]), e["file"]))
    print("[levels] %d escenas, %d capas únicas, %d planos compuestos, %d parches -> %s"
          % (sum(1 for v in index.values() if v), n, sum(len(v) for v in composed.values()),
             len(patches), OUT))


if __name__ == "__main__":
    main()
