#!/usr/bin/env python3
"""
gfx.py - Neo Geo graphics ROMs <-> indexed PNG tile sheets (lossless).

  S-ROM (201-s1.bin, fix layer): 8x8 tiles, 4 bpp, 32 bytes per tile.
      byte (col_group * 8 + row) holds 2 pixels, low nibble = left pixel;
      column groups in x order: bytes 0x10-0x17 (x 0,1), 0x18-0x1F (x 2,3),
      0x00-0x07 (x 4,5), 0x08-0x0F (x 6,7).
  C-ROM (201-c1..c4.bin, sprites): 16x16 tiles, 4 bpp, split over a ROM pair:
      odd ROM (c1/c3) = bitplanes 0,1, even ROM (c2/c4) = bitplanes 2,3;
      64 bytes per tile in each ROM = 4 blocks of 8x8 in the order
      right-top, right-bottom, left-top, left-bottom; each 8-pixel row is
      2 bytes (one per bitplane), bit 0 = leftmost pixel.

Sheets are paletted PNGs whose pixel values are the raw 4-bit colour
indices (a fixed grey ramp palette), so PNG -> ROM is exact.  Sheets are
TILES_X x TILES_Y tiles; the last sheet is padded with tiles of a sentinel
(index 0) that the packer ignores using the tile count in sheet metadata.

    python3 assets/tools/gfx.py extract   # rom/ -> build/assets/{fix,sprites}/
    python3 assets/tools/gfx.py build     # build/assets -> build/assets/rebuilt/*.bin + MD5 check
"""
import hashlib, json, os, sys
import numpy as np
from PIL import Image

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from romset import rom_file  # noqa: E402
OUT = os.path.join(ROOT, "build", "assets")
GREY = sum(([v, v, v] for v in range(0, 256, 17)), [])      # 16-entry grey ramp

MD5 = {
    "201-s1.bin": "59b0200b7ed4f9704e6f34fa6d05796e",
    "201-c1.bin": "db70f526ea5eef2f8d95687542c5216c",
    "201-c2.bin": "9c1cdd4ddef664cc00dd9bafe7fc7d48",
    "201-c3.bin": "96af10f773441ca60bacfd626d385caa",
    "201-c4.bin": "530c64ea8ac47bb8e4ec527b18fa6c5f",
}
FIX_ORDER = [0x10, 0x18, 0x00, 0x08]     # byte group for x = 0-1, 2-3, 4-5, 6-7


# ---- fix layer ---------------------------------------------------------------
def fix_decode(data):
    t = np.frombuffer(data, np.uint8).reshape(-1, 4, 8)     # tile, group, row
    t = t[:, [g // 8 for g in FIX_ORDER], :]                # reorder groups to x order
    lo, hi = t & 0x0F, t >> 4
    px = np.stack([lo, hi], axis=-1)                       # tile, group, row, 2
    return px.transpose(0, 2, 1, 3).reshape(-1, 8, 8)        # tile, row, x


def fix_encode(tiles):
    px = tiles.reshape(-1, 8, 4, 2).transpose(0, 2, 1, 3)   # tile, group, row, 2
    b = (px[..., 0] | (px[..., 1] << 4)).astype(np.uint8)
    out = np.empty_like(b)
    for x_group, g in enumerate(FIX_ORDER):
        out[:, g // 8, :] = b[:, x_group, :]
    return out.tobytes()


# ---- sprites -----------------------------------------------------------------
def spr_decode(odd, even):
    """odd = planes 0/1 ROM, even = planes 2/3 ROM -> (n, 16, 16) indices."""
    a = np.frombuffer(odd, np.uint8).reshape(-1, 4, 8, 2)   # tile, block, row, plane
    b = np.frombuffer(even, np.uint8).reshape(-1, 4, 8, 2)
    planes = np.concatenate([a, b], axis=-1)                 # tile, block, row, 4 planes
    bits = (planes[..., None] >> np.arange(8)) & 1           # ..., plane, x (bit0 = left)
    pix = (bits * (1 << np.arange(4))[:, None]).sum(axis=-2).astype(np.uint8)  # tile, block, row, x
    n = pix.shape[0]
    out = np.empty((n, 16, 16), np.uint8)
    out[:, 0:8, 8:16] = pix[:, 0]
    out[:, 8:16, 8:16] = pix[:, 1]
    out[:, 0:8, 0:8] = pix[:, 2]
    out[:, 8:16, 0:8] = pix[:, 3]
    return out


def spr_encode(tiles):
    blk = np.stack([tiles[:, 0:8, 8:16], tiles[:, 8:16, 8:16],
                    tiles[:, 0:8, 0:8], tiles[:, 8:16, 0:8]], axis=1)   # tile, block, row, x
    planes = ((blk[..., None] >> np.arange(4)) & 1)                     # ..., x, plane
    bytes_ = (planes * (1 << np.arange(8))[:, None]).sum(axis=-2).astype(np.uint8)  # ..., plane
    return bytes_[..., 0:2].tobytes(), bytes_[..., 2:4].tobytes()


# ---- sheets ------------------------------------------------------------------
def write_sheets(tiles, size, folder, prefix, per_x=64, per_y=64, base=0):
    os.makedirs(folder, exist_ok=True)
    per = per_x * per_y
    n = len(tiles)
    sheets = []
    for s in range(0, n, per):
        chunk = tiles[s:s + per]
        pad = per - len(chunk)
        if pad:
            chunk = np.concatenate([chunk, np.zeros((pad, size, size), np.uint8)])
        img = chunk.reshape(per_y, per_x, size, size).transpose(0, 2, 1, 3).reshape(per_y * size, per_x * size)
        im = Image.fromarray(img, "P")
        im.putpalette(GREY)
        name = f"{prefix}_{base + s:06x}.png"
        im.save(os.path.join(folder, name), optimize=True)
        sheets.append({"file": name, "first_tile": base + s, "tiles": per - pad})
    return sheets


def read_sheets(folder, meta, size):
    parts = []
    for sh in meta["sheets"]:
        img = np.array(Image.open(os.path.join(folder, sh["file"])))
        h, w = img.shape
        t = img.reshape(h // size, size, w // size, size).transpose(0, 2, 1, 3).reshape(-1, size, size)
        parts.append(t[:sh["tiles"]])
    return np.concatenate(parts)


def md5(b):
    return hashlib.md5(b).hexdigest()


def extract():
    meta = {}
    s1 = rom_file("201-s1.bin")
    fix = fix_decode(s1)
    d = os.path.join(OUT, "fix")
    meta["fix"] = {"rom": "201-s1.bin", "tile": 8, "sheets": write_sheets(fix, 8, d, "fix", 32, 32)}
    json.dump(meta["fix"], open(os.path.join(d, "sheets.json"), "w"), indent=1)
    print(f"[gfx] fix: {len(fix)} tiles -> {os.path.relpath(d, ROOT)}/ ({len(meta['fix']['sheets'])} sheets)")
    for pair, (o, e) in enumerate((("201-c1.bin", "201-c2.bin"), ("201-c3.bin", "201-c4.bin"))):
        odd = rom_file(o)
        even = rom_file(e)
        n = len(odd) // 64
        d = os.path.join(OUT, "sprites", f"bank{pair}")
        sheets, per = [], 64 * 64
        for s0 in range(0, n, per):                    # one sheet at a time (low RAM)
            tiles = spr_decode(odd[s0 * 64:(s0 + per) * 64], even[s0 * 64:(s0 + per) * 64])
            sheets += write_sheets(tiles, 16, d, f"spr{pair}", base=s0)
        m = {"roms": [o, e], "tile": 16, "first_tile_number": pair * n, "sheets": sheets}
        json.dump(m, open(os.path.join(d, "sheets.json"), "w"), indent=1)
        print(f"[gfx] sprites {o}/{e}: {n} tiles -> {os.path.relpath(d, ROOT)}/ ({len(m['sheets'])} sheets)")


def build():
    outd = os.path.join(OUT, "rebuilt")
    os.makedirs(outd, exist_ok=True)
    ok = True
    d = os.path.join(OUT, "fix")
    data = fix_encode(read_sheets(d, json.load(open(os.path.join(d, "sheets.json"))), 8))
    res = {"201-s1.bin": data}
    for pair in range(2):
        d = os.path.join(OUT, "sprites", f"bank{pair}")
        m = json.load(open(os.path.join(d, "sheets.json")))
        odd, even = bytearray(), bytearray()
        for sh in m["sheets"]:
            a, b = spr_encode(read_sheets(d, {"sheets": [sh]}, 16))
            odd += a; even += b
        res[m["roms"][0]], res[m["roms"][1]] = bytes(odd), bytes(even)
    for name, blob in res.items():
        open(os.path.join(outd, name), "wb").write(blob)
        same = md5(blob) == MD5[name]
        ok &= same
        print(f"[gfx] {'✔' if same else '✘'} {name}  {len(blob):,} B  MD5 {md5(blob)}")
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    {"extract": extract, "build": build}[sys.argv[1] if len(sys.argv) > 1 else "extract"]()
