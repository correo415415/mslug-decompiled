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

    python3 mrom/tools/export_wav.py                 # 13 songs
    python3 mrom/tools/export_wav.py --sfx           # + 177 SFX / voices
    python3 mrom/tools/export_wav.py --loops 2       # play songs' loop twice
    python3 mrom/tools/export_wav.py --only 21 2C    # selected music ids
    python3 mrom/tools/export_wav.py --flac          # also .flac (ffmpeg)

Output: build/mrom/wav/ (derived from the ROMs, never committed).  Song
length = one pass of the longest track (from export_midi.interpret) times
--loops, plus a 2 s tail; SFX stop by themselves after 1 s of silence.
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


def render(path, seconds, cmd_bytes, auto_stop=False):
    args = [BIN, os.path.join(ROM, "201-m1.bin"), os.path.join(ROM, "201-v1.bin"),
            os.path.join(ROM, "201-v2.bin"), path, f"{seconds:.3f}"]
    if auto_stop:
        args.append("--auto-stop")
    args += [f"{b:02x}" for b in cmd_bytes]
    r = subprocess.run(args, capture_output=True, text=True)
    if r.returncode:
        sys.exit(r.stderr)
    return r.stderr.strip()


def main():
    for f in ("201-m1.bin", "201-v1.bin", "201-v2.bin"):
        if not os.path.exists(os.path.join(ROM, f)):
            sys.exit(f"falta rom/{f}: scripts/setup_mrom.sh y scripts/setup_assets.sh")
    argv = sys.argv[1:]
    loops = int(argv[argv.index("--loops") + 1]) if "--loops" in argv else 1
    only = None
    if "--only" in argv:
        only = {int(x, 16) for x in argv[argv.index("--only") + 1:] if not x.startswith("--")}
    flac = "--flac" in argv and shutil.which("ffmpeg")
    build_renderer()
    rom = open(os.path.join(ROM, "201-m1.bin"), "rb").read()
    ids = load_sound_ids()
    os.makedirs(OUT, exist_ok=True)
    jobs = []
    for i in range(MUSIC_N):
        sid = 0x20 + i
        if only is not None and sid not in only:
            continue
        h = p24(rom, MUSIC_TABLE + 4 * i)
        sps = TIMER_HZ * (s8(rom[0x4B84 + rom[h]]) + 129) / 256
        trks = header_tracks(rom, h + 1)
        length = max(interpret(rom, s)[1] for s, _c in trks) / sps
        name = ids.get(sid, (f"Music_{i:02d}",))[0]
        jobs.append((f"music_{i:02d}_{name}.wav", length * loops + 2.0, [0x00, sid], False))
    if "--sfx" in argv:
        for tag, base, cnt, typ in (("A", SFXA_TABLE, SFXA_N, 0x10), ("B", SFXB_TABLE, SFXB_N, 0x11)):
            for i in range(cnt):
                if not p24(rom, base + 4 * i):
                    continue
                name = ids.get(typ << 8 | (0x20 + i), (f"Sfx{tag}_{i:03d}",))[0]
                jobs.append((f"sfx{tag}_{i:03d}_{name}.wav", 30.0, [typ, 0x20 + i], True))
    for fn, secs, cmd, auto in jobs:
        path = os.path.join(OUT, fn)
        msg = render(path, secs, cmd, auto)
        if flac:
            subprocess.run(["ffmpeg", "-loglevel", "error", "-y", "-i", path, path[:-4] + ".flac"], check=True)
        print(f"[export_wav] {os.path.relpath(path, ROOT)}  {msg.split('  ', 1)[1] if '  ' in msg else msg}")
    print(f"[export_wav] {len(jobs)} ficheros en {os.path.relpath(OUT, ROOT)}/")


if __name__ == "__main__":
    main()
