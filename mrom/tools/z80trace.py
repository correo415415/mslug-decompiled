#!/usr/bin/env python3
"""
z80trace.py - byte-level recursive-descent code tracer for Z80 ROM images.

Unlike trace_code.py (which walks z80dasm's text output and therefore inherits
its misalignments when data precedes code), this decodes instruction lengths
and control flow directly from the ROM bytes, so it can be used to build the
code/data block map that is then fed back into z80dasm with `-b`.

    from z80trace import trace, ranges_of
    seen = trace(rom_bytes, roots, limit=0x8000)   # {addr: insn_len}
    for start, end in ranges_of(seen): ...

Run as a script it prints the code map of the fixed region:

    python3 mrom/tools/z80trace.py [EXTRA_ROOT_HEX ...]

Only what is needed for tracing is decoded: instruction length, static branch
targets, and whether control falls through.
"""
import os, sys

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
ROM = os.path.join(ROOT, "rom", "201-m1.bin")

# Known 16-bit pointer tables in the fixed region: (address, entries, stride, offset)
TABLES = [
    (0x2076, 16, 2, 0),   # command dispatcher (jp (hl) at 2075), cmds 00..0F
    (0x20b3, 16, 2, 0),   # sub-dispatcher for cmds 0x20.. (jp (hl) at 20b2)
    (0x4aa0, 4, 2, 0),    # channel-type handlers (jp (hl) at 4a9f)
    (0x4abc, 4, 2, 0),    # (4abb)
    (0x4ad8, 4, 2, 0),    # (4ad7)
    (0x53a3, 26, 3, 1),   # sequence opcodes C0..D9: 'jp nnnn' entries
]
VECTORS = [0x0000, 0x0008, 0x0030, 0x0038, 0x0066]


# ---- instruction length tables -------------------------------------------

def _len_main(op):
    """Length of an un-prefixed opcode (CB/DD/ED/FD handled separately)."""
    x, y, z = op >> 6, (op >> 3) & 7, op & 7
    if x == 0:
        if z == 0:
            return 1 if y < 2 else 2            # NOP, EX AF / DJNZ, JR, JR cc
        if z == 1:
            return 3 if y % 2 == 0 else 1       # LD rr,nn / ADD HL,rr
        if z == 2:
            return 3 if y >= 4 else 1           # LD (nn),HL/A ... / LD (BC),A ...
        if z == 6:
            return 2                            # LD r,n
        return 1
    if x == 1 or x == 2:
        return 1
    # x == 3
    if z == 2 or z == 4:                        # JP cc,nn / CALL cc,nn
        return 3
    if z == 3:
        return {0: 3, 1: 1, 2: 2, 3: 2, 4: 1, 5: 1, 6: 1, 7: 1}[y]  # JP nn, CB, OUT, IN, EX, EX, DI, EI
    if z == 5:
        return 3 if y == 1 else 1               # CALL nn (y==1), else PUSH/RET/EXX/...
    if z == 6:
        return 2                                # ALU A,n
    return 1


def insn_len(rom, pc):
    op = rom[pc]
    if op == 0xCB:
        return 2
    if op == 0xED:
        op2 = rom[pc + 1]
        if (op2 & 0xC7) == 0x43:                # LD (nn),rr / LD rr,(nn)
            return 4
        return 2
    if op in (0xDD, 0xFD):
        op2 = rom[pc + 1]
        if op2 == 0xCB:
            return 4
        if op2 in (0xDD, 0xFD, 0xED):
            return 1                            # prefix overridden
        base = _len_main(op2)
        # instructions with (IX+d) get an extra displacement byte
        x, y, z = op2 >> 6, (op2 >> 3) & 7, op2 & 7
        has_mem = ((x == 1 and (y == 6 or z == 6) and op2 != 0x76) or
                   (x == 2 and z == 6) or
                   (x == 0 and z == 6 and y == 6) or   # LD (IX+d),n
                   (x == 0 and z in (4, 5) and y == 6))  # INC/DEC (IX+d)
        return 1 + base + (1 if has_mem else 0)
    return _len_main(op)


def flow(rom, pc):
    """Return (targets, falls_through) for the instruction at pc."""
    op = rom[pc]
    n = insn_len(rom, pc)
    nn = (rom[pc + 1] | rom[pc + 2] << 8) if n >= 3 else None
    rel = pc + 2 + (rom[pc + 1] - 256 if rom[pc + 1] >= 128 else rom[pc + 1]) if n == 2 else None
    if op == 0xC3:                                   # JP nn
        return [nn], False
    if op == 0xE9:                                   # JP (HL)
        return [], False
    if op in (0xDD, 0xFD) and rom[pc + 1] == 0xE9:   # JP (IX)/(IY)
        return [], False
    if op == 0xC9:                                   # RET
        return [], False
    if op == 0xED and rom[pc + 1] in (0x45, 0x4D):   # RETN / RETI
        return [], False
    if op == 0x18:                                   # JR e
        return [rel], False
    if op in (0x20, 0x28, 0x30, 0x38, 0x10):         # JR cc / DJNZ
        return [rel], True
    if (op & 0xC7) == 0xC2 or (op & 0xC7) == 0xC4:   # JP cc / CALL cc
        return [nn], True
    if op == 0xCD:                                   # CALL nn
        return [nn], True
    if (op & 0xC7) == 0xC7:                          # RST p
        return [op & 0x38], True
    return [], True


def trace(rom, roots, limit=None):
    """Recursive descent; returns dict addr -> length for every instruction reached."""
    limit = len(rom) if limit is None else limit
    seen = {}
    work = list(roots)
    while work:
        pc = work.pop()
        while 0 <= pc < limit and pc not in seen:
            n = insn_len(rom, pc)
            if pc + n > limit:
                break
            seen[pc] = n
            tg, ft = flow(rom, pc)
            for t in tg:
                if t is not None and t < limit:
                    work.append(t)
            if not ft:
                break
            pc += n
    return seen


def ranges_of(seen):
    """Merge instruction addresses into [start, end) ranges."""
    out = []
    for a in sorted(seen):
        e = a + seen[a]
        if out and out[-1][1] == a:
            out[-1][1] = e
        else:
            out.append([a, e])
    return out


def fixed_roots(rom):
    roots = list(VECTORS)
    for base, n, stride, off in TABLES:
        for i in range(n):
            p = base + i * stride + off
            roots.append(rom[p] | rom[p + 1] << 8)
    return roots


def main():
    rom = open(ROM, "rb").read()
    roots = fixed_roots(rom) + [int(a, 16) for a in sys.argv[1:]]
    seen = trace(rom, roots, limit=0x8000)
    total = sum(seen.values())
    print(f"code bytes reached: {total:#x} ({total}) in {len(ranges_of(seen))} ranges")
    for s, e in ranges_of(seen):
        if e - s >= 4:
            print(f"  {s:04x}-{e:04x}  ({e-s:#x})")
    print("indirect jumps:")
    for a in sorted(seen):
        if rom[a] == 0xE9 or (rom[a] in (0xDD, 0xFD) and rom[a + 1] == 0xE9):
            print(f"  {a:04x}")


if __name__ == "__main__":
    main()
