#!/usr/bin/env python3
"""
dump_sound_tables.py - list the music / SFX / instrument tables of the M-ROM.

Reads rom/201-m1.bin and walks the driver's data structures:

    BankPtrTable_0080[1] -> music table   (13 x 24-bit ptr @ 0x815F) -> music headers
    BankPtrTable_0080[2] -> SFX table A   (259 x 24-bit ptr @ 0x8472) -> sfx headers
    BankPtrTable_0080[3] -> SFX table B   (35 x 24-bit ptr @ 0x87F2)  -> sfx headers
    InstrPtrTable_57A1   -> 1024 x 24-bit ptr to instrument patches

Music header : volume(1) ; N x track{ seq_ptr24, flags, channel } ; 000000
SFX header   : stop_ids(4 x u16) ; priority(1) ; N x track{...} ; 000000

    python3 mrom/tools/dump_sound_tables.py [--json out.json]

Addresses are 24-bit linear file offsets; the Z80 sees bank N at
0x8000 + (addr & 0x3FFF) with bank = addr >> 14.
"""
import json, os, sys

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
ROM = os.path.join(ROOT, "rom", "201-m1.bin")

MUSIC_TABLE, MUSIC_N = 0x815f, 13
SFXA_TABLE, SFXA_N = 0x8472, 259
SFXB_TABLE, SFXB_N = 0x87f2, 35
INSTR_TABLE, INSTR_N = 0x57a1, 1024


def p24(rom, a):
    return rom[a] | rom[a + 1] << 8 | rom[a + 2] << 16


def w16(rom, a):
    return rom[a] | rom[a + 1] << 8


def read_tracks(rom, a, limit=16):
    """5-byte entries: ptr24, flags, channel; terminated by ptr == 0."""
    out = []
    while len(out) < limit:
        ptr = p24(rom, a)
        if ptr == 0:
            return out, a + 3
        out.append({"seq": ptr, "flags": rom[a + 3], "channel": rom[a + 4]})
        a += 5
    return out, a


def music_header(rom, h):
    tracks, end = read_tracks(rom, h + 1)
    return {"addr": h, "end": end, "volume": rom[h], "tracks": tracks}


def sfx_header(rom, h):
    stop = [w16(rom, h + k * 2) for k in range(4)]
    tracks, end = read_tracks(rom, h + 9)
    return {"addr": h, "end": end, "stop_ids": stop, "priority": rom[h + 8], "tracks": tracks}


def load(rom):
    out = {"music": [], "sfx_a": [], "sfx_b": [], "instruments": []}
    for i in range(MUSIC_N):
        h = p24(rom, MUSIC_TABLE + i * 4)
        out["music"].append({"id": i, **music_header(rom, h)} if h else {"id": i, "addr": 0})
    for key, base, n in (("sfx_a", SFXA_TABLE, SFXA_N), ("sfx_b", SFXB_TABLE, SFXB_N)):
        for i in range(n):
            h = p24(rom, base + i * 4)
            out[key].append({"id": i, **sfx_header(rom, h)} if h else {"id": i, "addr": 0})
    for i in range(INSTR_N):
        out["instruments"].append({"id": i, "group": i >> 7, "num": i & 0x7f,
                                   "addr": p24(rom, INSTR_TABLE + i * 4)})
    return out


def main():
    rom = open(ROM, "rb").read()
    out = load(rom)
    if "--json" in sys.argv:
        path = sys.argv[sys.argv.index("--json") + 1]
        json.dump(out, open(path, "w"), indent=1)
        print(f"written {path}")
        return

    print("MUSIC (68k command 0x20+id with type 0)")
    for m in out["music"]:
        if not m["addr"]:
            continue
        seqs = " ".join(f"{t['seq']:05x}" + ("*" if t["channel"] else "") for t in m["tracks"])
        print(f"  {m['id']:2}  hdr {m['addr']:05x}  vol {m['volume']:02x}  {len(m['tracks']):2} tracks: {seqs}")
    for key, label in (("sfx_a", "SFX A (type 1)"), ("sfx_b", "SFX B (type 2)")):
        used = [s for s in out[key] if s["addr"]]
        print(f"\n{label}: {len(used)} defined of {len(out[key])}")
        for s in used:
            seqs = " ".join(f"{t['seq']:05x}/ch{t['channel']}" for t in s["tracks"])
            stop = ",".join(f"{v:04x}" for v in s["stop_ids"] if v)
            print(f"  {s['id']:3}  hdr {s['addr']:05x}  prio {s['priority']:02x}  {seqs}"
                  + (f"  stops [{stop}]" if stop else ""))
    inst = {}
    for e in out["instruments"]:
        inst.setdefault(e["addr"], []).append(e["id"])
    print(f"\nINSTRUMENTS: {len(inst)} distinct patches referenced by 1024 slots")
    for a in sorted(inst):
        ids = inst[a]
        print(f"  {a:05x}  x{len(ids):3}  first id {ids[0]:4} (group {ids[0] >> 7}, num {ids[0] & 0x7f})")


if __name__ == "__main__":
    main()
