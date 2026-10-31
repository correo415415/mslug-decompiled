#!/usr/bin/env python3
"""
export_code_map.py — Mapa estático código/datos de la P-ROM sacado de la
decompilación (sin ejecutar el juego).

Ensambla/compila cada fuente del registro con listado (`-Wa,-aln`) y, para
cada línea del listado que emite bytes, decide por la línea fuente si es una
**instrucción** (mnemónico 68000) o un **dato** (.dc/.byte/.word/.long/
.fill/.rept/...). Con la dirección de cada sección (tools/registry.py) se
obtiene la dirección CPU exacta de cada instrucción.

Salidas (por defecto en build/code_map/):
    insns.txt     una línea por instrucción: "AAAAAA LEN FUNC"
    funcs.tsv     "AAAAAA SIZE NAME FILE KIND" por entrada del registro
                  (KIND = code | data | mixed)
    code.bin      bitset (1 bit por dirección par, 2 MiB/2/8 = 128 KiB):
                  inicio de instrucción según la decompilación

Uso:
    python3 tools/export_code_map.py [--out DIR] [--check]
    --check  verifica con capstone que cada instrucción del listado decodifica
             en la ROM (build/mslug_prom.bin) con la misma longitud (los bytes
             no se comparan: el .o sin enlazar lleva ceros en las relocaciones)
"""
import argparse, os, re, subprocess, sys, tempfile
from concurrent.futures import ThreadPoolExecutor

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(ROOT, "tools"))
import match_batch as mb  # noqa: E402

REG = {e[0]: e for e in mb.REGISTRY}

DATA_DIR = re.compile(
    r"^\s*(?:[\w.$]+:\s*)*\.(dc(\.[bwl])?|dcb(\.[bwl])?|byte|2byte|4byte|word|long|short|int|"
    r"fill|space|skip|zero|ds(\.[bwl])?|ascii|asciz|string|incbin|balign|p2align|align|even|"
    r"rept|endr|irp|irpc|endm|macro|quad|octa)\b", re.I)
# Llamadas a macros definidas en el propio fichero (todas son de datos).
LST_LINE = re.compile(r"^\s*(\d+)\s([0-9a-f]{4})\s((?:[0-9A-F]{2,8} ?)+)\s*(?:\t(.*))?$")
LST_CONT = re.compile(r"^\s*(\d+)\s{6}((?:[0-9A-F]{2,8} ?)+)\s*$")
SECTION = re.compile(r"^\s*\.section\s+\.text\.([\w$.]+)")
STRIP_LABEL = re.compile(r"^\s*(?:[\w.$]+:\s*)+")


def listing(src):
    """Devuelve el texto del listado de una fuente del registro."""
    ext = os.path.splitext(src)[1].lower()
    with tempfile.TemporaryDirectory() as td:
        lst = os.path.join(td, "a.lst")
        if ext == ".s":
            cmd = ["m68k-linux-gnu-gcc", "-m68000", "-nostdlib", "-nostartfiles",
                   "-ffreestanding", "-Wa,--register-prefix-optional", f"-Wa,-aln={lst}",
                   f"-I{mb.INC}", "-c", os.path.join(mb.ASM, src), "-o", os.path.join(td, "a.o")]
        else:
            cmd = ["m68k-linux-gnu-gcc", *mb.CFLAGS, *mb.PER_FILE_CFLAGS.get(src, []),
                   f"-Wa,-aln={lst}", "-c", os.path.join(mb.SRC, src), "-o", os.path.join(td, "a.o")]
        r = subprocess.run(cmd, capture_output=True, text=True)
        if r.returncode:
            raise SystemExit(f"[FAIL] {src}\n{r.stderr}")
        return open(lst, errors="replace").read()


def parse(src, text, macros):
    """-> lista de (addr, bytes, kind, func) ; kind 'i' instrucción / 'd' dato."""
    out = []
    sec = None
    base = None
    cur = None  # (addr, bytearray, kind, func) de la línea en curso
    in_macro_def = False
    for ln in text.splitlines():
        m = LST_LINE.match(ln)
        if m:
            off = int(m.group(2), 16)
            hx = m.group(3).replace(" ", "")
            body = m.group(4) or ""
            code = body.split("|")[0]
            sm = SECTION.match(code)
            if sm:
                sec = sm.group(1)
            if sec is None or sec not in REG:
                cur = None
                continue
            base = REG[sec][1]
            stripped = STRIP_LABEL.sub("", code).strip()
            word = stripped.split()[0].lower() if stripped else ""
            if body == "" or DATA_DIR.match(code) or word in macros:
                kind = "d"
            elif word.startswith("."):
                kind = "d"
            else:
                kind = "i"
            if body == "" and cur is not None:
                kind = cur[2]  # expansión de macro/.rept sin texto: hereda
            cur = [base + off, bytearray.fromhex(hx), kind, sec]
            out.append(cur)
            continue
        m = LST_CONT.match(ln)
        if m and cur is not None:
            cur[1] += bytearray.fromhex(m.group(2).replace(" ", ""))
            continue
        # línea fuente sin bytes: seguir la sección
        m = re.match(r"^\s*\d+\s+(?:\t)?(.*)$", ln)
        if m:
            sm = SECTION.match(m.group(1).split("|")[0])
            if sm:
                sec = sm.group(1)
                cur = None
    return out


def macro_names(src):
    if not src.endswith(".s"):
        return set()
    names = set()
    for ln in open(os.path.join(mb.ASM, src), errors="replace"):
        m = re.match(r"^\s*\.macro\s+(\w+)", ln)
        if m:
            names.add(m.group(1).lower())
    return names


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", default=os.path.join(mb.BUILD, "code_map"))
    ap.add_argument("--check", action="store_true")
    ap.add_argument("-j", "--jobs", type=int, default=os.cpu_count() or 2)
    a = ap.parse_args()
    sources = sorted(set(e[3] for e in mb.REGISTRY))
    with ThreadPoolExecutor(a.jobs) as ex:
        texts = dict(zip(sources, ex.map(listing, sources)))
    items = []
    for s in sources:
        items += parse(s, texts[s], macro_names(s))
    items.sort(key=lambda x: x[0])

    prom = md = None
    if a.check:
        import capstone
        prom = open(mb.PROM, "rb").read()
        md = capstone.Cs(capstone.CS_ARCH_M68K, capstone.CS_MODE_M68K_000)
    bad = 0
    insns = []
    kinds = {}
    for addr, blob, kind, func in items:
        if md is not None and kind == "i":
            d = list(md.disasm(prom[addr:addr + 10], addr, 1))
            if addr & 1 or not d or d[0].size != len(blob):
                bad += 1
                if bad <= 10:
                    print(f"  !! ${addr:06X} {func}: listado {len(blob)} B, capstone "
                          f"{d[0].size if d else '?'} B")
        k = kinds.setdefault(func, set())
        k.add(kind)
        if kind == "i":
            insns.append((addr, len(blob), func))

    os.makedirs(a.out, exist_ok=True)
    with open(os.path.join(a.out, "insns.txt"), "w") as f:
        f.write("# addr len func   (instrucciones según la decompilación; export_code_map.py)\n")
        for addr, n, func in insns:
            f.write(f"{addr:06X} {n} {func}\n")
    bits = bytearray(0x200000 // 16)
    for addr, n, func in insns:
        if addr & 1 == 0:
            i = addr >> 1
            bits[i >> 3] |= 1 << (i & 7)
    open(os.path.join(a.out, "code.bin"), "wb").write(bits)
    with open(os.path.join(a.out, "funcs.tsv"), "w") as f:
        f.write("# addr\tsize\tname\tfile\tkind\n")
        for name, cpu, size, src in sorted(mb.REGISTRY, key=lambda e: e[1]):
            k = kinds.get(name, set())
            kind = "code" if k == {"i"} else "data" if k <= {"d"} else "mixed"
            f.write(f"{cpu:06X}\t{size}\t{name}\t{src}\t{kind}\n")
    ib = sum(n for _, n, _ in insns)
    db = sum(len(b) for _, b, k, _ in items if k == "d")
    nk = {}
    for v in kinds.values():
        kk = "code" if v == {"i"} else "data" if v <= {"d"} else "mixed"
        nk[kk] = nk.get(kk, 0) + 1
    print(f"{len(insns)} instrucciones ({ib:,} B), datos {db:,} B; entradas: {nk}")
    if a.check:
        print(f"check: {bad} instrucciones con longitud distinta a la de la ROM")
    print(f"-> {a.out}")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
