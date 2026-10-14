#!/usr/bin/env python3
"""
export_wav.py - render every music track and SFX of Metal Slug to WAV exactly
as the board plays them.

No re-synthesis: mrom/tools/sndrender (C++) runs the original sound driver
(rom/201-m1.bin) on a Z80 emulator connected to ymfm's YM2610 (the same core
MAME uses) with the V-ROM (201-v1/v2) as ADPCM memory, and sends the command
bytes the 68000 would send (type byte, then the sound byte, one per frame).
What comes out is the chip's own output at its native rate (8 MHz / 144 =
55,556 Hz, 16-bit stereo), mixed like the Neo Geo board.

    python3 mrom/tools/export_wav.py                 # everything: 13 songs, 150 SFX, 27 voices
    python3 mrom/tools/export_wav.py --music         # songs only
    python3 mrom/tools/export_wav.py --loops 1       # songs: one pass (default 2) + fade
    python3 mrom/tools/export_wav.py --only 21 2C    # selected music ids
    python3 mrom/tools/export_wav.py --vgm           # also VGM 1.51 logs (build/mrom/vgm/)
    python3 mrom/tools/export_wav.py --flac          # also .flac (ffmpeg)

Output: build/mrom/wav/{music,sfx,voices}/ (derived from the ROMs, never
committed).  Looping songs: the renderer hooks the driver's infinite-loop
jumps and ends after --loops full passes (the pass ends when the last track
to loop wraps), then fades out over --fade seconds (default 8) as game rips
do; songs that do not loop (jingles) and SFX stop by themselves after 1 s of
silence.  Table B of the SFX are the voice samples ("Mission 1 start!",
"Thank you!", Allen's lines...).
"""
import os, sys, subprocess, shutil

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, HERE)
from gen_sound_data import p24, MUSIC_TABLE, MUSIC_N, SFXA_TABLE, SFXA_N, SFXB_TABLE, SFXB_N, load_sound_ids  # noqa: E402
from export_midi import interpret, header_tracks, s8, TIMER_HZ  # noqa: E402

ROM = os.path.join(ROOT, "rom")
SRC = os.path.join(HERE, "sndrender")
BIN = os.path.join(ROOT, "build", "mrom", "sndrender")
OUT = os.path.join(ROOT, "build", "mrom", "wav")
VGM_OUT = os.path.join(ROOT, "build", "mrom", "vgm")


def build_renderer():
    srcs = [os.path.join(SRC, "sndrender.cpp")] + [
        os.path.join(SRC, "third_party", "ymfm", f) for f in ("ymfm_opn.cpp", "ymfm_adpcm.cpp", "ymfm_ssg.cpp")]
    z80c = os.path.join(SRC, "third_party", "z80", "z80.c")
    deps = srcs + [z80c, __file__]
    if os.path.exists(BIN) and os.path.getmtime(BIN) >= max(os.path.getmtime(d) for d in deps):
        return
    os.makedirs(os.path.dirname(BIN), exist_ok=True)
    obj = BIN + "_z80.o"
    print("[export_wav] compilando sndrender (ymfm + z80)…")
    subprocess.check_call(["gcc", "-O2", "-c", z80c, "-o", obj])
    subprocess.check_call(["g++", "-O2", "-std=c++14",
                           "-I" + os.path.join(SRC, "third_party", "ymfm"),
                           "-I" + os.path.join(SRC, "third_party", "z80"),
                           *srcs, obj, "-o", BIN])


def render(path, cmd_bytes, opts):
    args = [BIN, os.path.join(ROM, "201-m1.bin"), os.path.join(ROM, "201-v1.bin"),
            os.path.join(ROM, "201-v2.bin"), path, "0", *opts]
    args += [f"{b:02x}" for b in cmd_bytes]
    r = subprocess.run(args, capture_output=True, text=True)
    if r.returncode:
        sys.exit(r.stderr)
    return r.stderr.strip()


def opt(argv, name, default, conv=float):
    return conv(argv[argv.index(name) + 1]) if name in argv else default


def main():
    for f in ("201-m1.bin", "201-v1.bin", "201-v2.bin"):
        if not os.path.exists(os.path.join(ROM, f)):
            sys.exit(f"falta rom/{f}: scripts/setup_mrom.sh y scripts/setup_assets.sh")
    argv = sys.argv[1:]
    loops = opt(argv, "--loops", 2, int)
    fade = opt(argv, "--fade", 8.0)
    only = None
    if "--only" in argv:
        only = {int(x, 16) for x in argv[argv.index("--only") + 1:] if not x.startswith("--")}
    music_only = "--music" in argv or only is not None
    vgm = "--vgm" in argv
    flac = "--flac" in argv and shutil.which("ffmpeg")
    build_renderer()
    rom = open(os.path.join(ROM, "201-m1.bin"), "rb").read()
    ids = load_sound_ids()
    jobs = []
    for i in range(MUSIC_N):
        sid = 0x20 + i
        if only is not None and sid not in only:
            continue
        h = p24(rom, MUSIC_TABLE + 4 * i)
        sps = TIMER_HZ * (s8(rom[0x4B84 + rom[h]]) + 129) / 256
        res = [interpret(rom, s) for s, _c in header_tracks(rom, h + 1)]
        k = sum(1 for ev, _L in res if any(e[1] == "loop" for e in ev))
        one = max(L for _ev, L in res) / sps
        name = ids.get(sid, (f"Music_{i:02d}",))[0]
        if k:
            o = ["--loop-tracks", str(k), "--loops", str(loops), "--fade-len", str(fade),
                 "--max", f"{one * (loops + 1) + 60:.0f}"]
        else:
            o = ["--auto-stop", "--max", f"{one + 30:.0f}"]
        jobs.append(("music", f"music_{i:02d}_{name}", [0x00, sid], o, name))
    if not music_only:
        for tag, base, cnt, typ, sub in (("A", SFXA_TABLE, SFXA_N, 0x10, "sfx"),
                                         ("B", SFXB_TABLE, SFXB_N, 0x11, "voices")):
            for i in range(cnt):
                if not p24(rom, base + 4 * i):
                    continue
                sid = typ << 8 | (0x20 + i)
                name = ids.get(sid, (f"Sfx{tag}_{i:03d}",))[0]
                jobs.append((sub, f"{sid:04x}_{name}", [typ, 0x20 + i],
                             ["--auto-stop", "--max", "30", "--fade-len", "3"], name))
    for sub, base, cmd, o, title in jobs:
        d = os.path.join(OUT, sub)
        os.makedirs(d, exist_ok=True)
        path = os.path.join(d, base + ".wav")
        o = list(o) + ["--title", title]
        if vgm:
            vd = os.path.join(VGM_OUT, sub)
            os.makedirs(vd, exist_ok=True)
            o += ["--vgm", os.path.join(vd, base + ".vgm")]
        msg = render(path, cmd, o)
        if flac:
            subprocess.run(["ffmpeg", "-loglevel", "error", "-y", "-i", path, path[:-4] + ".flac"], check=True)
        last = msg.splitlines()[-1]
        print(f"[export_wav] {os.path.relpath(path, ROOT)}  {last.split('  ', 1)[1] if '  ' in last else last}")
    print(f"[export_wav] {len(jobs)} ficheros en {os.path.relpath(OUT, ROOT)}/"
          + (f" (+ VGM en {os.path.relpath(VGM_OUT, ROOT)}/)" if vgm else ""))


if __name__ == "__main__":
    main()
