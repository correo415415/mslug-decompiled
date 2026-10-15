#!/usr/bin/env python3
"""resplit_data.py - reparte un tramo de un módulo de datos en entradas con nombre.

Sustituye las entradas consecutivas de asm/data/<módulo>.s que cubren
[START, END) por las entradas que se indican (dirección=Nombre[:comentario]).
Cada entrada nueva va desde su dirección hasta la siguiente (o END).

Cuerpo de cada entrada:
  * rachas de ceros (>= 8 B) -> `.fill N,1,0x00`
  * `.dc.l Símbolo` donde el módulo original ya tenía un `.dc.l` simbólico en
    esa misma dirección (se conservan las referencias del enlazador)
  * resto -> `.dc.w` (8 por línea), `.dc.b` si queda un byte impar
  * --text ADDR: la entrada se emite como `.ascii` en trozos de 32 B (sólo
    bytes imprimibles; el resto en `.dc.b`)

Actualiza tools/registry.py (mismas posiciones) y comprueba que las
etiquetas eliminadas no se usan fuera del módulo. Comprobar después con
check_section_sizes.py y build_rom.py.

  python3 tools/resplit_data.py data/level_d_16b035.s 0x182C3C 0x185CF4 \\
      0x182C3C=RosterLenTabB_182c3c:"longitudes (modo B)" ...
"""
import argparse, glob, os, re, sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(ROOT, "tools"))
from registry import REGISTRY  # noqa: E402

PROM = os.path.join(ROOT, "build", "mslug_prom.bin")
HDR_RE = re.compile(r"^\| -{20,}\n\|  (\w+)  @ \$([0-9A-F]{6})", re.M)
DCL_RE = re.compile(r"^\s+\.dc\.l\s+([A-Za-z_]\w*)\s*\|\s*\+([0-9a-f]+)", re.M)


def entry_block(src, name):
    """(inicio, fin) del bloque de texto de una entrada (cabecera incluida)."""
    m = re.search(r"^\| -{20,}\n\|  %s  @" % re.escape(name), src, re.M)
    if not m:
        raise SystemExit("no encuentro la cabecera de " + name)
    nxt = HDR_RE.search(src, m.end())
    return m.start(), (nxt.start() if nxt else len(src))


def printable(b):
    return 0x20 <= b < 0x7F and b not in (0x22, 0x5C)


def body(rom, a, b, syms, text):
    out, o = [], a
    while o < b:
        if o in syms and o + 4 <= b:
            out.append("        .dc.l   %-30s | +%05x" % (syms[o], o - a))
            o += 4
            continue
        z = o
        while z < b and rom[z] == 0 and z not in syms:
            z += 1
        if z - o >= 8:
            out.append("        .fill   %d,1,0x00%s | +%05x" % (z - o, " " * 17, o - a))
            o = z
            continue
        if text:
            e = o
            while e < b and e - o < 32 and printable(rom[e]) and e not in syms:
                e += 1
            if e > o:
                out.append('        .ascii  "%s"%s | +%05x' % (rom[o:e].decode("latin1"),
                                                          " " * max(1, 34 - (e - o)), o - a))
                o = e
                continue
            out.append("        .dc.b   0x%02x%s | +%05x" % (rom[o], " " * 26, o - a))
            o += 1
            continue
        stop = o + 1
        while (stop < b and stop - o < 16 and stop not in syms
               and not (stop + 8 <= b and not any(rom[stop:stop + 8]))):
            stop += 1
        n = stop - o
        if o & 1 or n == 1:
            out.append("        .dc.b   0x%02x%s | +%05x" % (rom[o], " " * 26, o - a))
            o += 1
            continue
        n &= ~1
        ws = ",".join("0x%02x%02x" % (rom[o + i], rom[o + i + 1]) for i in range(0, n, 2))
        out.append("        .dc.w   %-55s | +%05x" % (ws, o - a))
        o += n
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("module")
    ap.add_argument("start", type=lambda x: int(x, 0))
    ap.add_argument("end", type=lambda x: int(x, 0))
    ap.add_argument("entries", nargs="+")
    ap.add_argument("--text", action="append", default=[], type=lambda x: int(x, 0))
    a = ap.parse_args()
    rom = open(PROM, "rb").read()
    path = os.path.join(ROOT, "asm", a.module)
    src = open(path).read()

    old = sorted((ad, s, n) for n, ad, s, f in REGISTRY if f == a.module and a.start <= ad < a.end)
    if not old or old[0][0] != a.start or old[-1][0] + old[-1][1] != a.end:
        raise SystemExit("el tramo no coincide con fronteras de entradas existentes")
    new = []
    for spec in a.entries:
        ad, rest = spec.split("=", 1)
        name, _, cm = rest.partition(":")
        new.append((int(ad, 0), name, cm))
    new.sort()
    if new[0][0] != a.start:
        raise SystemExit("la primera entrada nueva debe empezar en START")

    # .dc.l simbólicos del tramo original (dirección absoluta -> símbolo)
    syms = {}
    lo_txt = entry_block(src, old[0][2])[0]
    hi_txt = entry_block(src, old[-1][2])[1]
    for ad, s, n in old:
        b0, b1 = entry_block(src, n)
        for m in DCL_RE.finditer(src, b0, b1):
            syms[ad + int(m.group(2), 16)] = m.group(1)

    # las etiquetas viejas que desaparecen no pueden usarse fuera del módulo
    gone = {n for ad, s, n in old} - {n for ad, n, c in new}
    if gone:
        pat = re.compile(r"\b(" + "|".join(map(re.escape, gone)) + r")\b")
        for f in glob.glob(os.path.join(ROOT, "asm", "**", "*.s"), recursive=True) + \
                glob.glob(os.path.join(ROOT, "src", "**", "*.[ch]"), recursive=True):
            if os.path.abspath(f) == os.path.abspath(path):
                continue
            if pat.search(open(f).read()):
                raise SystemExit("etiqueta eliminada usada en " + f)
        rest = src[:lo_txt] + src[hi_txt:]
        if pat.search(rest):
            raise SystemExit("etiqueta eliminada usada en otra parte del módulo")

    txt = []
    for i, (ad, name, cm) in enumerate(new):
        end = new[i + 1][0] if i + 1 < len(new) else a.end
        txt.append("| " + "-" * 76)
        txt.append("|  %s  @ $%06X  (%s B)" % (name, ad, format(end - ad, ",")))
        for line in cm.split("\\n"):
            if line:
                txt.append("|  " + line)
        txt.append("| " + "-" * 76)
        txt.append('        .section .text.%s, "ax", @progbits' % name)
        txt.append("        .global %s" % name)
        txt.append("%s:" % name)
        txt += body(rom, ad, end, syms, ad in a.text)
        txt.append("")
    src = src[:lo_txt] + "\n".join(txt) + "\n" + src[hi_txt:]
    open(path, "w").write(src)

    # registry.py: sustituye las líneas del tramo, en el mismo sitio
    rp = os.path.join(ROOT, "tools", "registry.py")
    lines = open(rp).read().split("\n")
    names = {n for ad, s, n in old}
    idx = [i for i, l in enumerate(lines)
           if re.match(r'\s*\("(\w+)"', l) and re.match(r'\s*\("(\w+)"', l).group(1) in names
           and '"%s"' % a.module in l]
    if len(idx) != len(old) or idx != list(range(idx[0], idx[0] + len(idx))):
        raise SystemExit("las líneas del registry no son contiguas")
    reg = []
    for i, (ad, name, cm) in enumerate(new):
        end = new[i + 1][0] if i + 1 < len(new) else a.end
        reg.append('    (%-46s 0x%06X, %4d, "%s"),' % ('"%s",' % name, ad, end - ad, a.module))
    lines[idx[0]:idx[-1] + 1] = reg
    open(rp, "w").write("\n".join(lines))
    print("[resplit] %s: %d entradas -> %d (%d .dc.l simbólicos conservados)"
          % (a.module, len(old), len(new), len([s for s in syms if a.start <= s < a.end])))


if __name__ == "__main__":
    main()
