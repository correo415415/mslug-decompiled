#!/usr/bin/env python3
"""
trace_code.py - recursive-descent code/data map for the M-ROM fixed region.

Starts from the known entry points (reset, RST 8, IRQ 0x38, NMI 0x66) plus
the targets of the known jump tables, follows every static
jp/jr/call/djnz/rst and stops at ret/jp/retn/reti.  Indirect jumps
(jp (hl)) are listed so new pointer tables can be added as extra roots:

    python3 mrom/tools/trace_code.py [EXTRA_ROOT_HEX ...]
"""
import os, re, sys

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
RAW = os.path.join(ROOT, "mrom", "asm", "raw", "fixed_0000.s")
ROM = os.path.join(ROOT, "rom", "201-m1.bin")

# Known 16-bit pointer tables in the fixed region: (address, entries, stride, offset)
TABLES = [
    (0x2076, 16, 2, 0),   # command dispatcher (jp (hl) at 2075)
    (0x4aa0, 4, 2, 0),    # channel-type handlers (jp (hl) at 4a9f)
    (0x4abc, 4, 2, 0),    # (4abb)
    (0x4ad8, 4, 2, 0),    # (4ad7)
    (0x53a3, 26, 3, 1),   # sequence opcodes C0..D9: 'jp nnnn' entries
]


def main():
    insn, addrs = {}, []
    for ln in open(RAW):
        m = re.search(r"^\t(.*?)\s*;([0-9a-f]{4})$", ln)
        if m:
            a = int(m.group(2), 16)
            insn[a] = m.group(1).strip()
            addrs.append(a)
    addrs.sort()
    nxt = {a: b for a, b in zip(addrs, addrs[1:] + [0x8000])}

    def target(t):
        m = re.fullmatch(r"(?:l|sub_)([0-9a-f]{4})h", t) or re.fullmatch(r"0?([0-9a-f]{1,4})h", t)
        return int(m.group(1), 16) if m else None

    roots = [0x0000, 0x0008, 0x0038, 0x0066] + [int(a, 16) for a in sys.argv[1:]]
    if os.path.exists(ROM):
        rom = open(ROM, "rb").read()
        for base, n, stride, off in TABLES:
            for i in range(n):
                p = base + i * stride + off
                roots.append(rom[p] | rom[p + 1] << 8)

    seen, indirect, work = set(), [], list(roots)
    while work:
        a = work.pop()
        while a not in seen and a in insn:
            seen.add(a)
            t = insn[a]
            op = t.split()[0]
            args = t[len(op):].strip()
            if op in ("jp", "jr", "call", "djnz"):
                parts = [p.strip() for p in args.split(",")]
                if parts[-1].startswith("("):
                    indirect.append((a, t)); break
                d = target(parts[-1])
                if d is not None and d < 0x8000: work.append(d)
                if op in ("jp", "jr") and len(parts) == 1: break
            elif op == "rst":
                d = target(args)
                if d is not None: work.append(d)
            elif op in ("ret", "retn", "reti") and args == "":
                break
            a = nxt[a]

    ranges = []
    for a in sorted(seen):
        if ranges and ranges[-1][1] == a: ranges[-1][1] = nxt[a]
        else: ranges.append([a, nxt[a]])
    total = sum(e - s for s, e in ranges)
    print(f"code bytes reached: {total:#x} ({total}) in {len(ranges)} ranges")
    for s, e in ranges:
        if e - s >= 8: print(f"  {s:04x}-{e:04x}  ({e-s:#x})")
    print("indirect jumps:")
    for a, t in indirect: print(f"  {a:04x}: {t}")


if __name__ == "__main__":
    main()
