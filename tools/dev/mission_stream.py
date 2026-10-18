#!/usr/bin/env python3
# Recorre streams de la Mission VM con los strides de MissionVM_SkipOp_0446B6.
#   python3 tools/dev/mission_stream.py 0E92B2 ...   (dir. CPU = fichero < $100000)
#   python3 tools/dev/mission_stream.py --aux        (comprueba los MissionAux_*)
# op 00 spawn (18 B), 01 periódico (10 B + 1), 02 bloque (4 B .. $0D 2 B),
# 03 aleatorio (4 B .. $0E 2 B), 04/0A espera (6 B + 2), 0B (6 B + 3),
# 05..09 (4 B), 0C fin (4 B: 0C 00 FF FF).
import os, sys
ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
d = open(os.path.join(ROOT, "build", "mslug_prom.bin"), "rb").read()

class Bad(Exception):
    pass

def skip(o, out, depth):
    op = d[o]
    if op > 0x0C:
        raise Bad("op $%02X en $%06X" % (op, o))
    out.append((o, depth, op))
    if op == 0x00:
        return o + 0x12
    if op == 0x01:
        return skip(o + 0xA, out, depth + 1)
    if op in (0x02, 0x03):
        end = 0x0D if op == 0x02 else 0x0E
        o += 4
        while d[o] != end:
            if d[o] == 0x0C:
                raise Bad("0C dentro de bloque en $%06X" % o)
            o = skip(o, out, depth + 1)
        out.append((o, depth, end))
        return o + 2
    if op in (0x04, 0x0A, 0x0B):
        o += 6
        for _ in range(3 if op == 0x0B else 2):
            o = skip(o, out, depth + 1)
        return o
    return o + 4

def walk(o, limit=0x4000):
    out, start = [], o
    while o - start < limit:
        if d[o] == 0x0C:
            out.append((o, 0, 0x0C))
            return o + 4, out
        o = skip(o, out, 0)
    raise Bad("sin 0C")

NAMES = {0: "spawn", 1: "periódico", 2: "bloque", 3: "aleatorio", 4: "espera",
         0xA: "esperaA", 0xB: "esperaB", 0xC: "fin", 0xD: "fin-bloque", 0xE: "fin-aleatorio"}

if __name__ == "__main__":
    if sys.argv[1:] == ["--aux"]:
        import re
        src = open(os.path.join(ROOT, "asm/data/mission_streams_0e8524.s")).read()
        heads = [int(x, 16) for x in re.findall(r"\| \$([0-9A-F]{6}): referenciado desde", src)]
        for h in heads:
            try:
                e, out = walk(h)
                print("$%06X ok  %4d B  %3d ops  tras fin: %s" % (h, e - h, len(out), d[e:e + 2].hex()))
            except Bad as ex:
                print("$%06X MAL %s" % (h, ex))
        sys.exit()
    for a in sys.argv[1:]:
        e, out = walk(int(a, 16))
        for o, dep, op in out:
            print("$%06X %s%-10s %s" % (o, "  " * dep, NAMES.get(op, "op%02X" % op),
                                         d[o:o + (0x12 if op == 0 else 4)].hex()))
        print("fin en $%06X" % e)
