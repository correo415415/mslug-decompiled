#!/usr/bin/env python3
"""
gen_sound_data.py - decode the sound data of the M-ROM (file 0x08000-0x1FFFF)
into structured, symbolic GNU-as sources under mrom/sound/.

The banked half of 201-m1.bin holds no Z80 code; it is pure data read by the
driver in the fixed region (formats: mrom/docs/FORMATS.md):

    0x08000-0x0815E  last instrument patches (continuation of the fixed-region
                     patch area 0x067A1-0x07FFF, pointed to by InstrPtrTable_57A1)
    0x0815F          MusicTable  (13 x PTR24)   BankPtrTable_0080[1]
    0x08472          SfxTableA   (224 x PTR24)  BankPtrTable_0080[2]
    0x087F2          SfxTableB   (33 x PTR24)   BankPtrTable_0080[3]
    0x08876-0x0953C  music + SFX headers
    0x0953D-0x1D286  sequence byte-code (all tracks, in header order)
    0x1D287-0x1FFFF  leftover bytes not referenced by the driver

Every byte goes through a macro of mrom/sound/macros.inc; pointers and the
relative offsets of jumps / calls / loops are symbolic, notes are absolute.
mrom/tools/build_mrom.py assembles mrom/sound/sound.s with binutils-z80 and
checks that the result is byte-identical.

    python3 mrom/tools/gen_sound_data.py          # (re)write mrom/sound/*.s
    python3 mrom/tools/gen_sound_data.py --check  # decode only, print stats
"""
import os, sys

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
ROM = os.path.join(ROOT, "rom", "201-m1.bin")
OUT = os.path.join(ROOT, "mrom", "sound")

BASE, END = 0x08000, 0x20000
INSTR_TABLE, INSTR_N = 0x57A1, 1024
MUSIC_TABLE, MUSIC_N = 0x815F, 13
SFXA_TABLE, SFXA_N = 0x8472, 224
SFXB_TABLE, SFXB_N = 0x87F2, 33
SEQ_START = 0x953D

# opcode -> (macro, argument kinds)   b = byte, w = word, r = rel16 target
OPS = {
    0xC0: ("S_INSTR", "b"), 0xC1: ("S_INSTRG", "bb"), 0xC2: ("S_JUMP", "r"),
    0xC3: ("S_KEYMODE", "b"), 0xC4: ("S_CALL", "r"), 0xC5: ("S_RET", ""),
    0xC6: ("S_PMOD", "b"), 0xC7: ("S_VMOD", "b"), 0xC8: ("S_PARAM", "b"),
    0xC9: ("S_SUSTAIN", "b"), 0xCA: ("S_DETUNE", "wb"), 0xCB: ("S_WAIT", "b"),
    0xCC: ("S_PAN", "b"), 0xCD: ("S_TEMPO", "bb"), 0xCE: ("S_TRKVOL", "b"),
    0xCF: ("S_LOOPBEGIN", ""), 0xD0: ("S_LOOPEND", "br"), 0xD1: ("S_PORTA", "b"),
    0xD2: ("S_LFO", "b"), 0xD3: ("S_D3", "bb"), 0xD4: ("S_D4", "bb"),
    0xD5: ("S_D5", "bb"), 0xD6: ("S_D6", "bb"), 0xD7: ("S_PITCHDN", "bb"),
    0xD8: ("S_PITCHUP", "bb"), 0xD9: ("S_REPLY", "b"),
}
# ops after which the interpreter's current note is 0x40 on the fall-through
# path (D0 resets it only when the loop is taken, i.e. at the target)
RESETS_NOTE = {0xC2, 0xC4, 0xC5, 0xCF}


def p24(rom, a):
    return rom[a] | rom[a + 1] << 8 | rom[a + 2] << 16


def s16(v):
    return v - 0x10000 if v & 0x8000 else v


def h8(v):
    return f"0x{v:02x}"


def oplen(rom, a):
    op = rom[a]
    if op >= 0xDA:
        raise ValueError(f"end opcode {op:#x} at {a:#x} (not used by this ROM)")
    if op >= 0xC0:
        return 1 + sum(2 if k in "wr" else 1 for k in OPS[op][1])
    n = 1 if (op & 0xF8) == 0xB8 else 0
    return 1 + n + sum(1 for b in range(3) if not (op >> b) & 1)


def rel_target(rom, a):
    op = rom[a]
    if op in (0xC2, 0xC4):
        return a + 3 + s16(rom[a + 1] | rom[a + 2] << 8)
    if op == 0xD0:
        return a + 4 + s16(rom[a + 2] | rom[a + 3] << 8)
    return None


class Sound:
    def __init__(self, rom):
        self.rom = rom
        self.labels, self.comments = {}, {}
        self.load()

    def name(self, a, n, comment=None):
        self.labels.setdefault(a, n)
        if comment and a not in self.comments:
            self.comments[a] = comment

    def tracks(self, a):
        out = []
        while p24(self.rom, a):
            out.append((p24(self.rom, a), self.rom[a + 4]))
            a += 5
        assert self.rom[a + 3] == 0
        return out, a + 4

    def load(self):
        rom = self.rom
        ptrs = sorted({p24(rom, INSTR_TABLE + 4 * i) for i in range(INSTR_N)})
        self.patches = [p for p in ptrs if p >= BASE]
        self.patch_before = max(p for p in ptrs if p < BASE)
        for p in self.patches:
            self.name(p, f"InstrPatch_{p:05X}")
        self.name(MUSIC_TABLE, "MusicTable", "BankPtrTable_0080[1]; 68k cmd 20h+id with type 00h")
        self.name(SFXA_TABLE, "SfxTableA", "BankPtrTable_0080[2]; type 10h (12h..1Fh alias it)")
        self.name(SFXB_TABLE, "SfxTableB", "BankPtrTable_0080[3]; type 11h")
        self.music, self.sfx = [], []
        for i in range(MUSIC_N):
            h = p24(rom, MUSIC_TABLE + 4 * i)
            trk, end = self.tracks(h + 1)
            self.music.append((i, h, trk, end))
            self.name(h, f"MusicHdr_{i:02d}")
            for k, (s, _) in enumerate(trk):
                self.name(s, f"Music{i:02d}_Trk{k}", f"music {i}, track {k}")
        for tag, base, n in (("A", SFXA_TABLE, SFXA_N), ("B", SFXB_TABLE, SFXB_N)):
            for i in range(n):
                h = p24(rom, base + 4 * i)
                if not h:
                    continue
                trk, end = self.tracks(h + 9)
                self.sfx.append((tag, i, h, trk, end))
                self.name(h, f"Sfx{tag}Hdr_{i:03d}")
                for k, (s, _) in enumerate(trk):
                    self.name(s, f"Sfx{tag}{i:03d}_Trk{k}", f"SFX {tag} {i}, track {k}")
        self.entries = sorted(a for a, n in self.labels.items() if "_Trk" in n)
        # linear decode of the byte-code area: it is one contiguous stream of
        # ops (verified: every entry point and every jump target is an op boundary)
        a, self.ops = SEQ_START, []
        while a < END and not (a > self.entries[-1] and rom[a:a + 4] == b"\0\0\0\0"):
            self.ops.append(a)
            a += oplen(rom, a)
        self.seq_end = a
        bounds = set(self.ops)
        for e in self.entries:
            assert e in bounds, f"entry {e:#x} not on an op boundary"
        for o in self.ops:
            t = rel_target(rom, o)
            if t is not None:
                assert t in bounds, f"jump at {o:#x} to non-boundary {t:#x}"
                self.name(t, f"L_{t:05X}")
        # code after the last referenced track: sequences no header points to
        last, prev = self.entries[-1], None
        for o in self.ops:
            if o > last and prev is not None and rom[prev] == 0xC5 and o not in self.labels:
                self.name(o, f"SeqUnused_{o:05X}", "not referenced by any header")
            prev = o

    # ---- emitters ---------------------------------------------------------
    def lbl(self, a):
        c = self.comments.get(a)
        return f"{self.labels[a]}:\t\t\t/* file 0x{a:05x}" + (f"; {c} */" if c else " */")

    def rows(self, a, b, per=16):
        return ["\t.byte " + ", ".join(h8(x) for x in self.rom[i:min(b, i + per)])
                for i in range(a, b, per)]

    def emit_patches(self):
        L = ["/* instr_patches.s - instrument patches stored in the banked area",
             " * (targets of InstrPtrTable_57A1; layout: mrom/docs/FORMATS.md) */", "",
             f"\t/* file 0x{BASE:05x}: tail of InstrPatch_{self.patch_before:05X} (starts in the fixed region) */"]
        L += self.rows(BASE, self.patches[0])
        for k, p in enumerate(self.patches):
            nxt = self.patches[k + 1] if k + 1 < len(self.patches) else MUSIC_TABLE
            L.append(self.lbl(p))
            L += self.rows(p, nxt, 25 if (nxt - p) % 25 == 0 else 24 if (nxt - p) % 24 == 0 else 16)
        return L

    def emit_tables_headers(self):
        """Pointer tables and headers, interleaved in address order:
        MusicTable, music headers, SfxTableA, SfxTableB, SFX headers."""
        L = ["/* headers.s - music / SFX pointer tables (indexed by 68k command - 20h) and the",
             " * headers they point to (Seq_StartMusic_4FA3 / Seq_StartSfx_507B) */", ""]
        items = [(MUSIC_TABLE, "t", MUSIC_N), (SFXA_TABLE, "t", SFXA_N), (SFXB_TABLE, "t", SFXB_N)]
        items += [(h, "m", (trk, end)) for i, h, trk, end in self.music]
        items += [(h, "s", (trk, end)) for tag, i, h, trk, end in self.sfx]
        items.sort()
        pos = MUSIC_TABLE
        for h, kind, d in items:
            assert h == pos, f"gap before {h:#x} (at {pos:#x})"
            L.append(self.lbl(h))
            if kind == "t":
                for i in range(d):
                    p = p24(self.rom, h + 4 * i)
                    L.append(f"\tPTR24 {self.labels[p] if p else 0}\t\t/* {i:3d} */")
                L.append("")
                pos = h + 4 * d
                continue
            trk, end = d
            if kind == "m":
                L.append(f"\tMUSIC_HDR {h8(self.rom[h])}")
            else:
                stops = [self.rom[h + 2 * k] | self.rom[h + 2 * k + 1] << 8 for k in range(4)]
                L.append("\tSFX_HDR " + ", ".join(f"0x{v:04x}" for v in stops) + f", {h8(self.rom[h + 8])}")
            for sq, ch in trk:
                L.append(f"\tTRACK {self.labels[sq]}, {ch}")
            L += ["\tTRACKS_END", ""]
            pos = end
        assert pos == SEQ_START, hex(pos)
        return L

    def emit_seq(self, a0, a1, title):
        rom = self.rom
        L = [f"/* {title} */"]
        note = 0x40
        for a in self.ops:
            if not a0 <= a < a1:
                continue
            if a in self.labels:
                n = self.labels[a]
                if "_Trk" in n or n.startswith("SeqUnused"):
                    L += ["", self.lbl(a), "\tS_TRACK"]
                    note = 0x40
                else:
                    L.append(self.lbl(a))
            op = rom[a]
            if op < 0xC0:
                p = a + 1
                explicit = (op & 0xF8) == 0xB8
                if explicit:
                    n = rom[p]; p += 1
                else:
                    n = (note + (op >> 3) - 11) & 0xFF
                args = [h8(n)]
                for bit in range(3):
                    if not (op >> bit) & 1:
                        args.append(h8(rom[p])); p += 1
                    else:
                        args.append("")
                while args[-1] == "":
                    args.pop()
                L.append(f"\t{'NX' if explicit else 'N'} " + ", ".join(args))
                note = n
                continue
            mac, kinds = OPS[op]
            args, p = [], a + 1
            for k in kinds:
                if k == "b":
                    args.append(h8(rom[p])); p += 1
                elif k == "w":
                    args.append(f"0x{rom[p] | rom[p + 1] << 8:04x}"); p += 2
                else:
                    args.append(self.labels[rel_target(rom, a)]); p += 2
            L.append(f"\t{mac}" + (" " + ", ".join(args) if args else ""))
            if op in RESETS_NOTE:
                note = 0x40
        return L

    def write(self):
        os.makedirs(OUT, exist_ok=True)
        files = []

        def put(name, lines):
            with open(os.path.join(OUT, name), "w") as f:
                f.write("\n".join(lines).rstrip() + "\n")
            files.append(name)

        put("instr_patches.s", self.emit_patches())
        put("headers.s", self.emit_tables_headers())
        parts = [(self.music[i][2][0][0], f"music_{i:02d}.s", f"music_{i:02d}.s - sequences of MusicHdr_{i:02d}")
                 for i in range(MUSIC_N)]
        parts.append((min(t[0] for *_, trk, _ in self.sfx for t in trk), "sfx.s",
                      "sfx.s - SFX sequences (tables A and B)"))
        unused = [a for a, n in self.labels.items() if n.startswith("SeqUnused")]
        if unused:
            parts.append((min(unused), "seq_unused.s", "seq_unused.s - sequences not referenced by any header"))
        parts.append((self.seq_end, None, None))
        for (a0, name, title), (a1, _, _) in zip(parts, parts[1:]):
            assert a0 < a1, (name, hex(a0), hex(a1))
            put(name, self.emit_seq(a0, a1, title))
        tail = ["/* leftover.s - bytes after the last sequence; not referenced by the driver",
                " * (authoring-tool leftovers, kept verbatim) */", "",
                f"Leftover_{self.seq_end:05X}:\t\t\t/* file 0x{self.seq_end:05x} */"]
        put("leftover.s", tail + self.rows(self.seq_end, END))
        top = ["/* sound.s - M-ROM sound data = file 0x08000-0x1FFFF of 201-m1.bin",
               " * Generated by mrom/tools/gen_sound_data.py, assembled by mrom/tools/build_mrom.py",
               " * (binutils-z80).  Labels are linear file addresses; the driver maps them",
               " * through Bank_ResolvePtr24_1FA6 (bank = addr >> 14). */", "",
               '\t.include "macros.inc"', "", "\t.text", "SD_Base:\t\t\t/* file 0x08000 */"]
        top += [f'\t.include "{n}"' for n in files]
        with open(os.path.join(OUT, "sound.s"), "w") as f:
            f.write("\n".join(top) + "\n")
        njt = sum(1 for n in self.labels.values() if n.startswith("L_"))
        print(f"[gen_sound_data] {len(files)} files in mrom/sound/: {len(self.ops)} ops, "
              f"{len(self.entries)} track entries, {njt} jump targets, {len(unused)} unused sequences")


def main():
    s = Sound(open(ROM, "rb").read())
    if "--check" in sys.argv:
        print(f"ops {len(s.ops)}  seq end {s.seq_end:#x}  labels {len(s.labels)}")
        return
    s.write()


if __name__ == "__main__":
    main()
