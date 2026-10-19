#!/usr/bin/env python3
"""
audio.py - V-ROM (201-v1.bin + 201-v2.bin, YM2610 ADPCM) <-> named samples.

The V-ROM is one 8 MiB ADPCM space (v1 = 0x000000, v2 = 0x400000) shared by
ADPCM-A and ADPCM-B.  The sample boundaries come from the M-ROM instrument
records (mrom/sound/instruments.s, mrom/docs/FORMATS.md §5):

  PCMA_SAMPLE vol, start, end        start/end in 256-byte units (inclusive)
  PCMB_SAMPLE dN, pan, loops, start, len1, len2, len3   (256-byte units;
             segments start..+len1 (attack), +len2 (loop), +len3 (tail))

extract writes to build/assets/audio/:
  pcma/<name>.adpcma + .wav   raw ADPCM-A bytes + 16-bit PCM preview (18 500 Hz)
  pcmb/<name>.adpcmb + .wav   raw ADPCM-B bytes + preview at the delta-N rate
  gaps/gap_<off>.bin          bytes not covered by any sample (kept verbatim)
  index.csv                   offset, size, kind, name, used by
build concatenates the raw pieces back into 201-v1.bin / 201-v2.bin and checks
the MD5.  The raw ADPCM files are the editable source for a byte-identical
rebuild; the WAVs are previews (ADPCM re-encoding is lossy, so a modified WAV
is re-encoded with `encode`, which picks per sample the nibble whose decoded
value is closest, using the exact decoder arithmetic).

    python3 assets/tools/audio.py extract
    python3 assets/tools/audio.py build
    python3 assets/tools/audio.py encode a|b IN.wav OUT.adpcma|.adpcmb [--rate HZ]
"""
import csv, hashlib, os, re, struct, sys, wave

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from romset import rom_file  # noqa: E402
OUT = os.path.join(ROOT, "build", "assets", "audio")
INSTR = os.path.join(ROOT, "assets", "data", "instruments.s")  # copia de mrom/sound/ de la decompilación
MD5 = {"201-v1.bin": "0e3bc8071f2bef5a117c59583517652a",
       "201-v2.bin": "d667bbbaa1665e0cdd8dbb04b1a891d9"}
VSIZE = 0x400000
YM_CLOCK = 8_000_000

# ---- ADPCM decoders (YM2610) ---------------------------------------------------
A_STEP = [16, 17, 19, 21, 23, 25, 28, 31, 34, 37, 41, 45, 50, 55, 60, 66, 73, 80, 88, 97,
          107, 118, 130, 143, 157, 173, 190, 209, 230, 253, 279, 307, 337, 371, 408, 449,
          494, 544, 598, 658, 724, 796, 876, 963, 1060, 1166, 1282, 1411, 1552]
A_INC = [-1, -1, -1, -1, 2, 5, 7, 9]
B_TAB = [57, 57, 57, 57, 77, 102, 128, 153]


def decode_a(data):
    acc, step, out = 0, 0, bytearray()
    for byte in data:
        for nib in (byte >> 4, byte & 0xF):
            d = ((2 * (nib & 7) + 1) * A_STEP[step]) >> 3
            acc += -d if nib & 8 else d
            acc = ((acc + 0x800) & 0xFFF) - 0x800        # 12-bit wrap
            step = min(48, max(0, step + A_INC[nib & 7]))
            out += struct.pack("<h", acc << 4)
    return bytes(out)


def decode_b(data):
    acc, delta, out = 0, 127, bytearray()
    for byte in data:
        for nib in (byte >> 4, byte & 0xF):
            d = ((2 * (nib & 7) + 1) * delta) >> 3
            acc = max(-32768, min(32767, acc - d if nib & 8 else acc + d))
            delta = min(24576, max(127, (delta * B_TAB[nib & 7]) >> 6))
            out += struct.pack("<h", acc)
    return bytes(out)


# ---- ADPCM encoders (inversos exactos de los decodificadores) --------------------
def encode_a(pcm16):
    """PCM 16 bits con signo (lista/array) -> bytes ADPCM-A. Por cada muestra
    elige el nibble cuyo resultado DECODIFICADO (misma aritmética que
    decode_a, 12 bits) queda más cerca; así el error no se acumula."""
    acc, step, nibs = 0, 0, []
    for x in pcm16:
        target = max(-2048, min(2047, int(x) >> 4))
        best = None
        for nib in range(16):
            d = ((2 * (nib & 7) + 1) * A_STEP[step]) >> 3
            a = acc - d if nib & 8 else acc + d
            a = ((a + 0x800) & 0xFFF) - 0x800
            e = abs(a - target)
            if best is None or e < best[0]:
                best = (e, nib, a)
        _, nib, acc = best
        step = min(48, max(0, step + A_INC[nib & 7]))
        nibs.append(nib)
    return _pack(nibs)


def encode_b(pcm16):
    """PCM 16 bits -> bytes ADPCM-B (DELTA-T). Voraz como encode_a; si varios
    nibbles dan el mismo valor (saturación) pero distinto delta, desempata
    mirando las 2 muestras siguientes."""
    def step(acc, delta, nib):
        d = ((2 * (nib & 7) + 1) * delta) >> 3
        a = max(-32768, min(32767, acc - d if nib & 8 else acc + d))
        return a, min(24576, max(127, (delta * B_TAB[nib & 7]) >> 6))

    def cost(acc, delta, xs):
        tot = 0
        for x in xs:
            acc, delta = min((step(acc, delta, n) for n in range(16)), key=lambda r: abs(r[0] - x))
            tot += abs(acc - x)
        return tot

    acc, delta, nibs = 0, 127, []
    pcm16 = [int(x) for x in pcm16]
    for i, x in enumerate(pcm16):
        cands = [(abs(a - x), n, a, dl) for n in range(16) for a, dl in [step(acc, delta, n)]]
        e0 = min(c[0] for c in cands)
        ties = [c for c in cands if c[0] == e0]
        if len(ties) > 1 and len({c[3] for c in ties}) > 1:
            ahead = pcm16[i + 1:i + 3]
            ties.sort(key=lambda c: (cost(c[2], c[3], ahead), c[1]))
        _, nib, acc, delta = ties[0]
        nibs.append(nib)
    return _pack(nibs)


def _pack(nibs, block=256):
    if len(nibs) & 1:
        nibs.append(0)
    out = bytearray((nibs[i] << 4) | nibs[i + 1] for i in range(0, len(nibs), 2))
    pad = (-len(out)) % block               # las muestras ocupan bloques de 256 B
    return bytes(out) + _silence(pad)


def _silence(n):
    # byte 0x80 = nibbles 8,0: -d,+d con paso decreciente -> silencio sin deriva
    return bytes([0x80]) * n


def read_wav(path, rate=None):
    """WAV mono/estéreo 8/16 bits -> lista de muestras 16 bits (mezcla a mono,
    re-muestreo lineal a `rate` si se indica)."""
    with wave.open(path, "rb") as w:
        ch, sw, fr, n = w.getnchannels(), w.getsampwidth(), w.getframerate(), w.getnframes()
        raw = w.readframes(n)
    if sw == 2:
        v = list(struct.unpack("<%dh" % (len(raw) // 2), raw))
    elif sw == 1:
        v = [(b - 128) << 8 for b in raw]
    else:
        raise SystemExit("WAV de %d bits no soportado (usa 8/16)" % (8 * sw))
    if ch > 1:
        v = [sum(v[i:i + ch]) // ch for i in range(0, len(v), ch)]
    if rate and rate != fr:
        k = fr / rate
        m = int(len(v) / k)
        out = []
        for i in range(m):
            t = i * k
            j = int(t)
            f = t - j
            b = v[j + 1] if j + 1 < len(v) else v[j]
            out.append(int(v[j] * (1 - f) + b * f))
        v = out
    return v


def encode(args):
    """encode <a|b> IN.wav OUT.adpcm[a|b] [--rate HZ]  (A: 18 500 Hz por defecto)."""
    if len(args) < 3 or args[0] not in ("a", "b"):
        raise SystemExit(encode.__doc__)
    kind, src, dst = args[:3]
    rate = int(args[args.index("--rate") + 1]) if "--rate" in args else (18500 if kind == "a" else None)
    pcm = read_wav(src, rate)
    data = encode_a(pcm) if kind == "a" else encode_b(pcm)
    open(dst, "wb").write(data)
    dec = decode_a(data) if kind == "a" else decode_b(data)
    ref = pcm + [0] * (len(dec) // 2 - len(pcm))
    got = struct.unpack("<%dh" % (len(dec) // 2), dec)
    err = sum((a - b) ** 2 for a, b in zip(ref, got)) / max(1, len(got))
    sig = sum(a * a for a in ref) / max(1, len(ref))
    snr = 10 * __import__("math").log10(sig / err) if err and sig else float("inf")
    print(f"[audio] {dst}: {len(pcm)} muestras -> {len(data):,} B ADPCM-{kind.upper()} "
          f"({len(data) // 256} bloques de 256 B), SNR {snr:.1f} dB")


def write_wav(path, pcm, rate):
    with wave.open(path, "wb") as w:
        w.setnchannels(1)
        w.setsampwidth(2)
        w.setframerate(int(rate))
        w.writeframes(pcm)


# ---- sample map from mrom/sound/instruments.s -------------------------------------
def sample_map():
    """-> list of (start_byte, end_byte_excl, kind, name, rate, users)."""
    src = open(INSTR).read()
    out, users = {}, {}
    tab = None
    instr_of = {}
    cur_instr = None
    for ln in src.splitlines():
        m = re.match(r"^(\w+):", ln)
        if m:
            lab = m.group(1)
            if lab.startswith("Instr"):
                cur_instr = lab
            if lab.startswith("PcmaSampleTab_"):
                tab, idx = lab, 0
            elif lab.startswith("PcmbSample_"):
                tab = lab
            else:
                tab = None if not lab.startswith("Pcm") else tab
            continue
        m = re.search(r"PCMA_MAP .*?, (PcmaSampleTab_\w+),", ln)
        if m:
            instr_of.setdefault(m.group(1), []).append(cur_instr)
        m = re.search(r"PCMB_KEY \w+, \w+, (PcmbSample_\w+)", ln)
        if m:
            instr_of.setdefault(m.group(1), []).append(cur_instr)
        m = re.search(r"PCMA_SAMPLE (\w+), (\w+), (\w+)", ln)
        if m and tab:
            s, e = int(m.group(2), 16), int(m.group(3), 16)
            key = (s * 256, (e + 1) * 256)
            out.setdefault(key, ("a", f"{tab.replace('PcmaSampleTab_', 'PcmA_')}_{idx:02d}", 18500))
            users.setdefault(key, set()).add(tab)
            idx += 1
        m = re.search(r"PCMB_SAMPLE (\w+), (\w+), (\w+), (\w+), (\w+), (\w+), (\w+)", ln)
        if m and tab:
            dn, st, l1, l2, l3 = (int(m.group(i), 16) for i in (1, 4, 5, 6, 7))
            key = (st * 256, (st + l1 + l2 + l3) * 256)
            rate = YM_CLOCK / 144 * dn / 65536
            out.setdefault(key, ("b", tab.replace("PcmbSample_", "PcmB_"), rate))
            users.setdefault(key, set()).add(tab)
    res = []
    for (s, e), (k, n, r) in sorted(out.items()):
        u = sorted({i for t in users[(s, e)] for i in instr_of.get(t, [])})
        res.append((s, e, k, n, r, u))
    return res


def md5(b):
    return hashlib.md5(b).hexdigest()


def extract():
    v = rom_file("201-v1.bin") + rom_file("201-v2.bin")
    smp = sample_map()
    for d in ("pcma", "pcmb", "gaps"):
        os.makedirs(os.path.join(OUT, d), exist_ok=True)
    rows, pos = [], 0
    pieces = []
    for s, e, k, n, r, u in smp:
        assert s >= pos, f"overlap at {s:#x} ({n})"
        if s > pos:
            pieces.append((pos, s, "gap", f"gap_{pos:06x}", 0, []))
        pieces.append((s, e, k, n, r, u))
        pos = e
    if pos < len(v):
        pieces.append((pos, len(v), "gap", f"gap_{pos:06x}", 0, []))
    for s, e, k, n, r, u in pieces:
        blob = v[s:e]
        if k == "gap":
            fn = os.path.join("gaps", n + ".bin")
            open(os.path.join(OUT, fn), "wb").write(blob)
        else:
            sub = "pcma" if k == "a" else "pcmb"
            fn = os.path.join(sub, f"{n}.adpcm{k}")
            open(os.path.join(OUT, fn), "wb").write(blob)
            write_wav(os.path.join(OUT, sub, n + ".wav"), (decode_a if k == "a" else decode_b)(blob), r)
        rows.append([f"0x{s:06x}", e - s, k, n, f"{r:.0f}" if r else "", " ".join(u), fn])
    with open(os.path.join(OUT, "index.csv"), "w", newline="") as f:
        w = csv.writer(f)
        w.writerow(["offset", "size", "kind", "name", "rate_hz", "used_by", "file"])
        w.writerows(rows)
    na = sum(1 for p in pieces if p[2] == "a")
    nb = sum(1 for p in pieces if p[2] == "b")
    gaps = sum(p[1] - p[0] for p in pieces if p[2] == "gap")
    print(f"[audio] {na} ADPCM-A + {nb} ADPCM-B samples, {gaps:,} B of gaps -> {os.path.relpath(OUT, ROOT)}/")


def build():
    rows = list(csv.DictReader(open(os.path.join(OUT, "index.csv"))))
    v = bytearray()
    for r in rows:
        assert int(r["offset"], 16) == len(v), r
        v += open(os.path.join(OUT, r["file"]), "rb").read()
    outd = os.path.join(ROOT, "build", "assets", "rebuilt")
    os.makedirs(outd, exist_ok=True)
    ok = True
    for i, name in enumerate(("201-v1.bin", "201-v2.bin")):
        blob = bytes(v[i * VSIZE:(i + 1) * VSIZE])
        open(os.path.join(outd, name), "wb").write(blob)
        same = md5(blob) == MD5[name]
        ok &= same
        print(f"[audio] {'✔' if same else '✘'} {name}  {len(blob):,} B  MD5 {md5(blob)}")
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    cmd = sys.argv[1] if len(sys.argv) > 1 else "extract"
    if cmd == "encode":
        encode(sys.argv[2:])
    else:
        {"extract": extract, "build": build}[cmd]()
