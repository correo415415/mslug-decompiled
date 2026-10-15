#!/usr/bin/env python3
"""
export_midi.py - export the M-ROM music (and optionally SFX) sequences to MIDI.

Interprets the sequence byte-code of rom/201-m1.bin like Seq_TrackFetch_5362
(mrom/docs/FORMATS.md §4) and writes one Standard MIDI File (type 1) per song
to build/mrom/midi/ (derived from the ROM, never committed).

Timing model (driver code):
  * tick = YM2610 Timer A, value (ACh << 2) | 3 = 691
    -> Timer A counts once per FM sample (144 YM clocks):
       144 * (1024 - 691) / 8 MHz = 5.994 ms (TIMER_HZ ~ 166.8 Hz), measured by
       running the driver on ymfm (mrom/tools/sndrender);
  * music tracks step when the accumulator FDF0 += (tempo + 129) carries,
    i.e. (tempo + 129) / 256 steps per tick, tempo = signed
    NoteMapTable_4B84[header byte] (FDE7) + offset (FDEB, signed saturation);
  * S_TEMPO m, s (CD): Seq_SetTempo_4F39 -> FDEE = m, FDED = s; the offset
    target is Seq_ScaleByCurve_4F69(FDE7, m) - FDE7, i.e. a multiply in the
    log domain with the tables at 0x1400 (log) / 0x1500 (exp): 0x40 = x1.
    s = ramp speed (0 = immediate; every S_TEMPO in the game uses 0), so each
    one becomes an exact MIDI set-tempo event;
  * SFX tracks step once per tick.
One sequence step = one MIDI tick; the set-tempo meta event gives the real
duration.  `wait` = steps to the next event, `dur` = key-on steps (0 = rest),
`vel` -> velocity.  Semitone number -> MIDI note (octave not verified).
Tracks whose first instrument is ADPCM-A (type 2) go to MIDI channel 10.
Jumps / calls / loops are followed; a track ends at its first backward jump
or infinite loop (one pass).

    python3 mrom/tools/export_midi.py          # music -> build/mrom/midi/
    python3 mrom/tools/export_midi.py --sfx    # + every SFX
"""
import os, struct, sys

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
ROM = os.path.join(ROOT, "rom", "201-m1.bin")
OUT = os.path.join(ROOT, "build", "mrom", "midi")
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from gen_sound_data import (OPS, p24, s16, MUSIC_TABLE, MUSIC_N, SFXA_TABLE, SFXA_N,  # noqa: E402
                            SFXB_TABLE, SFXB_N, INSTR_TABLE, load_sound_ids)

TIMER_HZ = 8_000_000 / (144 * (1024 - 691))
MAX_STEPS = 50000
PPQ = 48
RESET = (0x40, 0xC0, 0xC0, 0x7F)          # note, wait, dur, vel after C2/C4/C5/CF/D0


def s8(v):
    return v - 256 if v & 0x80 else v


def scale_by_curve(rom, a, l):
    """Seq_ScaleByCurve_4F69: a (signed tempo) scaled by curve index l."""
    a = (a + 0x80) & 0xFF
    e = rom[0x1400 + l]
    a = rom[0x1400 + a] + e
    if a > 0xFF:                       # carry
        a = (a & 0xFF) - 0xC0
        a = 0xFF if a >= 0 else a & 0xFF
    else:
        a -= 0xC0
        a = 0 if a < 0 else a
    return s8((rom[0x1500 + a] - 0x80) & 0xFF)


def tempo_with(rom, base, m):
    """tempo efectivo (FDE7 + FDEB) tras S_TEMPO m,0."""
    if m == 0x40:
        return base
    off = s8((scale_by_curve(rom, base, m) - base) & 0xFF)
    return max(-128, min(127, base + off))


def interpret(rom, pc):
    """-> (events, length_in_steps); events = (step, kind, data)."""
    ev, t, stack, seen = [], 0, [], set()
    note, wait, dur, vel = RESET
    while t < MAX_STEPS:
        op = rom[pc]
        if op >= 0xDA:
            break
        if op < 0xC0:
            p = pc + 1
            if (op & 0xF8) == 0xB8:
                note = rom[p]; p += 1
            else:
                note = (note + (op >> 3) - 11) & 0xFF
            if not op & 1:
                wait = rom[p]; p += 1
            if not op & 2:
                dur = rom[p]; p += 1
            if not op & 4:
                vel = rom[p]; p += 1
            if dur and vel:
                ev.append((t, "note", (note, dur, vel)))
            t += wait
            pc = p
            continue
        kinds = OPS[op][1]
        args, p = [], pc + 1
        for k in kinds:
            if k == "b":
                args.append(rom[p]); p += 1
            elif k == "w":
                args.append(rom[p] | rom[p + 1] << 8); p += 2
            else:
                args.append(p + 2 + s16(rom[p] | rom[p + 1] << 8)); p += 2
        reset = op in (0xC2, 0xC4, 0xC5, 0xCF)
        if op == 0xC0:
            ev.append((t, "instr", (None, args[0])))
        elif op == 0xC1:
            ev.append((t, "instr", (args[1], args[0])))
        elif op == 0xC2:
            if args[0] <= pc or args[0] in seen:
                ev.append((t, "loop", args[0]))
                break
            seen.add(args[0]); p = args[0]
        elif op == 0xC4:
            if len(stack) >= 3:
                break
            stack.append(["ret", p]); p = args[0]
        elif op == 0xC5:
            if not stack or stack[-1][0] != "ret":
                break
            p = stack.pop()[1]
        elif op == 0xCF:
            stack.append(["loop", 0])
        elif op == 0xD0:
            if not stack or stack[-1][0] != "loop":
                break
            cnt, tgt = args
            if cnt == 0:
                ev.append((t, "loop", tgt))
                break
            fr = stack[-1]
            fr[1] = (cnt if fr[1] == 0 else fr[1]) - 1
            if fr[1] == 0:
                stack.pop()
            else:
                p, reset = tgt, True
        elif op == 0xCB:
            t += args[0]
        elif op in (0xCA, 0xD3, 0xD4, 0xD5, 0xD6, 0xD7, 0xD8):
            t += args[-1]
        elif op == 0xCD:
            ev.append((t, "tempo", tuple(args)))
        elif op == 0xCC:
            ev.append((t, "pan", args[0]))
        if reset:
            note, wait, dur, vel = RESET
        pc = p
    return ev, t


def vlq(n):
    b = [n & 0x7F]
    n >>= 7
    while n:
        b.insert(0, (n & 0x7F) | 0x80)
        n >>= 7
    return bytes(b)


def meta(kind, txt):
    raw = txt.encode()
    return bytes([0xFF, kind]) + vlq(len(raw)) + raw


def chunk(evs):
    evs.sort(key=lambda e: (e[0], e[1]))
    out, last = bytearray(), 0
    for t, _, data in evs:
        out += vlq(t - last) + data
        last = t
    out += b"\x00\xff\x2f\x00"
    return b"MTrk" + struct.pack(">I", len(out)) + bytes(out)


def to_midi(rom, tracks, steps_per_sec, title, base_tempo=None):
    def instr_type(hi, lo):
        if hi is None or hi == 0xFF:
            return None
        return rom[p24(rom, INSTR_TABLE + 4 * ((hi * 128 + lo) & 0x3FF))]

    usq = int(round(PPQ / steps_per_sec * 1e6))
    cond = [(0, 0, meta(3, title)), (0, 1, b"\xff\x51\x03" + struct.pack(">I", usq)[1:])]
    chunks, nchan = [], 0
    for k, (evs, chan_byte) in enumerate(tracks):
        hi, cur = None, None
        types = []
        for _, kind, d in evs:
            if kind == "instr":
                hi = d[0] if d[0] is not None else hi
                types.append(instr_type(hi, d[1]))
        drum = bool(types) and types[0] == 2
        ch = 9 if drum else [0, 1, 2, 3, 4, 5, 6, 7, 8, 10, 11, 12, 13, 14, 15][nchan % 15]
        nchan += 0 if drum else 1
        out = [(0, 0, meta(3, f"Trk{k} chan={chan_byte}" + (" ADPCM-A" if drum else "")))]
        hi = None
        for t, kind, d in evs:
            if kind == "note":
                n, dur, vel = d
                n = min(127, n)
                out.append((t, 2, bytes([0x90 | ch, n, max(1, min(127, vel))])))
                out.append((t + dur, 1, bytes([0x80 | ch, n, 0])))
            elif kind == "instr":
                hi = d[0] if d[0] is not None else hi
                out.append((t, 1, bytes([0xC0 | ch, d[1] & 0x7F])))
                out.append((t, 0, meta(1, f"instr {hi if hi is not None else 0xFF:02X}:{d[1]:02X} "
                                          f"type {instr_type(hi, d[1])}")))
            elif kind == "pan":
                out.append((t, 1, bytes([0xB0 | ch, 10, 64 if d == 0x40 else (0 if d < 0x40 else 127)])))
            elif kind == "tempo":
                cond.append((t, 0, meta(1, f"S_TEMPO {d[0]:02X},{d[1]:02X}")))
                if base_tempo is not None:
                    eff = tempo_with(rom, base_tempo, d[0])
                    sps = TIMER_HZ * (eff + 129) / 256
                    cond.append((t, 1, b"\xff\x51\x03" + struct.pack(">I", int(round(PPQ / sps * 1e6)))[1:]))
            elif kind == "loop":
                out.append((t, 0, meta(6, "loop")))
        chunks.append(chunk(out))
    return b"MThd" + struct.pack(">IHHH", 6, 1, len(chunks) + 1, PPQ) + chunk(cond) + b"".join(chunks)


def header_tracks(rom, a):
    out = []
    while p24(rom, a):
        out.append((p24(rom, a), rom[a + 4]))
        a += 5
    return out


def main():
    rom = open(ROM, "rb").read()
    ids = load_sound_ids()
    os.makedirs(OUT, exist_ok=True)
    n = 0
    for i in range(MUSIC_N):
        h = p24(rom, MUSIC_TABLE + 4 * i)
        name = ids.get(0x20 + i, (f"Music_{i:02d}",))[0]
        trks = [(interpret(rom, s), c) for s, c in header_tracks(rom, h + 1)]
        base = s8(rom[0x4B84 + rom[h]])
        sps = TIMER_HZ * (base + 129) / 256
        data = to_midi(rom, [(e, c) for (e, _), c in trks], sps, name, base)
        fn = os.path.join(OUT, f"music_{i:02d}_{name}.mid")
        open(fn, "wb").write(data)
        notes = sum(1 for (e, _), _c in trks for x in e if x[1] == "note")
        secs = max(L for (_, L), _c in trks) / sps
        print(f"[export_midi] {os.path.relpath(fn, ROOT)}: {len(trks)} tracks, {notes} notes, ~{secs:.0f} s")
        n += 1
    if "--sfx" in sys.argv:
        for tag, base, cnt, typ in (("A", SFXA_TABLE, SFXA_N, 0x10), ("B", SFXB_TABLE, SFXB_N, 0x11)):
            for i in range(cnt):
                h = p24(rom, base + 4 * i)
                if not h:
                    continue
                name = ids.get(typ << 8 | (0x20 + i), (f"Sfx{tag}_{i:03d}",))[0]
                trks = [(interpret(rom, s)[0], c) for s, c in header_tracks(rom, h + 9)]
                open(os.path.join(OUT, f"sfx{tag}_{i:03d}_{name}.mid"), "wb").write(
                    to_midi(rom, trks, TIMER_HZ, name))
                n += 1
    print(f"[export_midi] {n} files in {os.path.relpath(OUT, ROOT)}/")


if __name__ == "__main__":
    main()
