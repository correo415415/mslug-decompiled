#!/usr/bin/env python3
"""
gen_asm_region.py — genera un borrador .s (sintaxis GAS m68k del proyecto)
para una región de la P ROM aún no matcheada.
=============================================================================

Automatiza la parte mecánica del flujo de decompilación de las waves
ASM: desensambla el hueco con capstone, parte la región en entradas
(funciones) usando como fronteras

  * las islas C ya matcheadas (REGISTRY),
  * los símbolos conocidos de tools/symbols.py (TaskHandler_*, Sub_*),
  * los targets de `lea X(pc),a1 ; move.l a1,(a6)` (instalación de
    handler de tarea -> el target es el inicio de una función),
  * los targets de `jsr X(pc)` / `bsr`,
  * los puntos muertos tras rts/jmp/bra.w cuando el siguiente código
    es alcanzado desde fuera (heurística),

y emite por cada entrada una sección `.text.<Sym>` con el bloque de
comentarios `| +off` que usan todos los `.s` del proyecto. Los saltos
internos se traducen a labels locales `.Lxxxxxx`; los saltos a islas C
contiguas usan el símbolo global del REGISTRY (o un `*Rts_*` si caen en
medio de una isla); las llamadas absolutas se dejan en hex (`0x236e.l`)
como en el resto de los archivos, salvo que `--name-abs` esté activo.

El borrador se verifica inmediatamente reensamblando cada sección en su
LMA y comparando contra build/mslug_prom.bin (mismo método que
match_batch.py). Las entradas que no reensamblan byte-exactas se marcan
con `| !! MISMATCH` para revisión manual.

Lo que NO hace: poner nombres semánticos ni escribir el análisis. Eso
sigue siendo trabajo de ingeniería inversa manual sobre el borrador.

Uso:
    python3 tools/gen_asm_region.py 0x084836 0x0865BE -o asm/draft.s
    python3 tools/gen_asm_region.py 0x084836 0x0865BE --print-entries
    python3 tools/gen_asm_region.py 0x084836 0x0865BE --registry   # líneas para registry.py
"""
import argparse
import importlib.util
import os
import re
import subprocess
import sys
import tempfile

from capstone import Cs, CS_ARCH_M68K, CS_MODE_M68K_000

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, ".."))
PROM = os.path.join(ROOT, "build", "mslug_prom.bin")


def load(name):
    spec = importlib.util.spec_from_file_location(name, os.path.join(HERE, name + ".py"))
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m


REGISTRY = load("registry").REGISTRY
SYMBOLS = load("symbols").SYMBOLS

REG_BY_ADDR = {a: (n, s, f) for n, a, s, f in REGISTRY}
REG_SORTED = sorted(REG_BY_ADDR)

BRANCHES = {"bra", "bsr", "bhi", "bls", "bcc", "bcs", "bne", "beq", "bvc", "bvs",
            "bpl", "bmi", "bge", "blt", "bgt", "ble"}
DBCC = {"dbra", "dbf", "dbt", "dbhi", "dbls", "dbcc", "dbcs", "dbne", "dbeq", "dbvc",
        "dbvs", "dbpl", "dbmi", "dbge", "dblt", "dbgt", "dble"}


# ---------------------------------------------------------------------------
#  Desensamblado
# ---------------------------------------------------------------------------
class Insn:
    __slots__ = ("addr", "size", "bytes", "mn", "ops", "cs")

    def __init__(self, cs):
        self.cs = cs
        self.addr = cs.address
        self.size = cs.size
        self.bytes = bytes(cs.bytes)
        self.mn = cs.mnemonic
        # capstone emite "exg.l dX,dY"; GAS no admite sufijo de tamaño en EXG.
        if self.mn.startswith("exg"):
            self.mn = "exg"
        self.ops = cs.op_str
        # capstone calcula mal el target de `movem.l $d(pc),regs` (forma
        # memoria->registros): usa PC+2 como base cuando la palabra de
        # mascara va ANTES del desplazamiento (base real PC+4). Recalculamos
        # el target absoluto desde los bytes crudos.
        if self.mn.startswith("movem") and "(pc)" in self.ops \
                and self.size == 6 and (self.bytes[1] & 0x38) == 0x38 \
                and (self.bytes[1] & 0x07) == 0x02 and (self.bytes[0] & 0x04) == 0x04:
            disp = int.from_bytes(self.bytes[4:6], "big", signed=True)
            t = self.addr + 4 + disp
            self.ops = re.sub(r"\$[0-9a-f]+\(pc\)", f"${t:x}(pc)", self.ops, count=1)


DATA_RANGES = []      # [(a, b)] rangos marcados como datos con --data (tablas en .text)


def in_data(off):
    return any(a <= off < b for a, b in DATA_RANGES)


def disasm_region(rom, start, end):
    md = Cs(CS_ARCH_M68K, CS_MODE_M68K_000)
    md.detail = False
    out = []
    off = start
    while off < end:
        if in_data(off):
            out.append(("data", off, rom[off:off + 2]))
            off += 2
            continue
        ins = next(md.disasm(rom[off:off + 10], off), None)
        if ins is None or ins.size == 0 or off + ins.size > end:
            # capstone no decodifica movem.w/.l regs,abs.l (48B9/48F9) ni la
            # inversa abs.l,regs (4CB9/4CF9): se emiten los 8 bytes crudos.
            w = int.from_bytes(rom[off:off + 2], "big")
            if w in (0x48B9, 0x48F9, 0x4CB9, 0x4CF9) and off + 8 <= end:
                out.append(("rawinsn", off, rom[off:off + 8]))
                off += 8
                continue
            # palabra de datos
            out.append(("data", off, rom[off:off + 2]))
            off += 2
            continue
        out.append(("insn", off, Insn(ins)))
        off += ins.size
    return out


def branch_target(ins):
    """Devuelve el target absoluto de un branch/dbcc/bsr, o None."""
    base = ins.mn.split(".")[0]
    if base in BRANCHES:
        m = re.match(r"\$([0-9a-f]+)$", ins.ops)
        return int(m.group(1), 16) if m else None
    if base in DBCC:
        m = re.match(r"d\d, \$([0-9a-f]+)$", ins.ops)
        return int(m.group(1), 16) if m else None
    return None


def pcrel_target(ins):
    """Target de `X(pc)` en jsr/jmp/lea/pea/move, y la base de
    `X(pc, dN.w)` (jump tables indexadas: capstone da X absoluto)."""
    m = re.search(r"\$([0-9a-f]+)\(pc\)", ins.ops)
    if m:
        return int(m.group(1), 16)
    m = re.search(r"(-?\$[0-9a-f]+)?\(pc, [ad][0-7]\.[wl]\)", ins.ops)
    if m:
        return int(m.group(1).lstrip("-$"), 16) if m.group(1) else ins.addr + 2
    return None


# ---------------------------------------------------------------------------
#  Partición en entradas
# ---------------------------------------------------------------------------
def find_entries(items, start, end):
    """Devuelve el conjunto ordenado de direcciones de inicio de función."""
    entries = {start}
    # símbolos conocidos dentro del rango
    for a in SYMBOLS:
        if start < a < end:
            entries.add(a)
    insns = {off: it for kind, off, it in items if kind == "insn"}
    # fin de flujo -> la siguiente instrucción empieza una entrada si
    # nadie salta a ella desde ANTES dentro de la misma entrada (heurística
    # simple: la marcamos y luego la fusionamos si hay un branch hacia atrás
    # que la cruce).
    flow_ends = set()
    jump_targets = set()
    install_targets = set()
    prev = None
    for kind, off, it in items:
        if kind != "insn":
            prev = None
            continue
        base = it.mn.split(".")[0]
        if base in ("rts", "rte", "rtr", "jmp") or base == "bra":
            flow_ends.add(off + it.size)
        t = branch_target(it)
        if t is not None and start <= t < end:
            jump_targets.add((off, t))
        # lea X(pc),a1 ; move.l a1,(a6)   -> X es handler
        if base == "lea" and prev is not None:
            pass
        if base == "jsr" or base == "bsr":
            t2 = pcrel_target(it) if base == "jsr" else branch_target(it)
            if t2 is not None and start <= t2 < end:
                install_targets.add(t2)
        if base == "lea":
            t2 = pcrel_target(it)
            if t2 is not None and start <= t2 < end and not (off < t2 <= off + 8):
                # un lea a una dirección que no es la siguiente instrucción:
                # handler diferido (lea X(pc),a1 ; move.l a1,(a6) con X lejos)
                install_targets.add(t2)
        if base == "pea":
            t2 = pcrel_target(it)
            if t2 is not None and start <= t2 < end:
                install_targets.add(t2)
        prev = it
    entries |= install_targets
    # Puntos tras fin de flujo: son entrada salvo que un branch los cruce
    # (es decir, exista un salto desde antes del punto a después o igual).
    for fe in flow_ends:
        if fe >= end or fe not in insns:
            continue
        crossed = any(src < fe <= t for src, t in jump_targets) or \
                  any(t < fe <= src for src, t in jump_targets)
        if not crossed:
            entries.add(fe)
    # Los jump_targets que estén a mitad de nada pero sean alcanzados sólo
    # desde una entrada anterior ya cerrada (branch hacia delante cruzando
    # un fin de flujo) — se mantienen como labels locales cross-entry: GAS
    # los resuelve si están en el mismo archivo porque usamos símbolos
    # globales para cualquier label de otra sección (ver emit()).
    return sorted(a for a in entries if start <= a < end)


# ---------------------------------------------------------------------------
#  Formateo de operandos capstone -> GAS
# ---------------------------------------------------------------------------
def fmt_hex(v):
    return f"0x{v:x}"


def conv_operand(op, ins, labelfn):
    """Convierte un operando en sintaxis capstone a GAS."""
    op = op.strip()
    if op == "":
        return op
    # inmediato
    m = re.match(r"^#\$([0-9a-f]+)$", op)
    if m:
        return "#" + fmt_hex(int(m.group(1), 16))
    m = re.match(r"^#(-?\d+)$", op)
    if m:
        return "#" + m.group(1)
    # registro
    if re.match(r"^([ad][0-7]|sp|pc|ccr|sr|usp)$", op):
        return op.replace("sp", "a7") if op == "sp" else op
    # (aN) / (aN)+ / -(aN)
    m = re.match(r"^(-?)\((a[0-7]|sp)\)(\+?)$", op)
    if m:
        r = "a7" if m.group(2) == "sp" else m.group(2)
        return f"{m.group(1)}({r}){m.group(3)}"
    # dN-dN/aN... (movem list)
    if re.match(r"^[ad][0-7](-[ad][0-7])?(/[ad][0-7](-[ad][0-7])?)*$", op):
        return op
    # $d(aN)
    m = re.match(r"^(-?)\$([0-9a-f]+)\((a[0-7]|sp)\)$", op)
    if m:
        r = "a7" if m.group(3) == "sp" else m.group(3)
        return f"{m.group(1)}{fmt_hex(int(m.group(2), 16))}({r})"
    # $d(pc)
    m = re.match(r"^\$([0-9a-f]+)\(pc\)$", op)
    if m:
        t = int(m.group(1), 16)
        return f"{labelfn(t, pcrel=True)}(pc)"
    # (aN, dN.w) / $d(aN, dN.w) / (pc, dN.w)
    m = re.match(r"^(-?\$[0-9a-f]+)?\((a[0-7]|sp|pc), ([ad][0-7])\.([wl])\)$", op)
    if m:
        disp = m.group(1)
        base = "a7" if m.group(2) == "sp" else m.group(2)
        if disp:
            neg = disp.startswith("-")
            v = int(disp.lstrip("-$"), 16)
            d = ("-" if neg else "") + fmt_hex(v)
        else:
            d = ""
        if base == "pc":
            # capstone ya da el target absoluto en el displacement para pc
            t = int(disp.lstrip("-$"), 16) if disp else ins.addr + 2
            return f"{labelfn(t, pcrel=True)}(pc,{m.group(3)}.{m.group(4)})"
        return f"{d}({base},{m.group(3)}.{m.group(4)})"
    # $addr.l / $addr.w / $addr
    m = re.match(r"^\$([0-9a-f]+)\.([lw])$", op)
    if m:
        return f"{fmt_hex(int(m.group(1), 16))}.{m.group(2)}"
    m = re.match(r"^\$([0-9a-f]+)$", op)
    if m:
        # branch target (bcc/bsr/dbcc): también necesita símbolo (no literal)
        return labelfn(int(m.group(1), 16), pcrel=True)
    raise ValueError(f"operando no soportado '{op}' en {ins.addr:06x} {ins.mn} {ins.ops}")


def split_ops(s):
    # capstone separa con ', ' pero movem usa '/' y '-' sin comas; las
    # listas con índice llevan ', ' dentro de paréntesis.
    out, depth, cur = [], 0, ""
    for ch in s:
        if ch == "(":
            depth += 1
        elif ch == ")":
            depth -= 1
        if ch == "," and depth == 0:
            out.append(cur)
            cur = ""
        else:
            cur += ch
    if cur.strip():
        out.append(cur)
    return [o.strip() for o in out]


def conv_insn(ins, labelfn):
    mn = ins.mn
    base, _, suf = mn.partition(".")
    ops = split_ops(ins.ops)
    # Normalizaciones de mnemónico
    if base in DBCC and suf:
        mn = base
    if base == "dbf":
        mn = "dbra"
    if base in ("bset", "bclr", "bchg", "btst"):
        mn = base  # GAS infiere .b/.l por el operando
    if base in ("lea", "pea") and suf:
        mn = base
    if base == "move" and ops and ops[-1] in ("ccr", "sr", "usp") or \
       (base == "move" and ops and ops[0] in ("sr", "usp", "ccr")):
        mn = "move" + ("." + suf if suf else "")
        if ops[-1] == "ccr":
            mn = "move.w"
    # Branches: respetar el tamaño .b/.w según la codificación
    if base in BRANCHES:
        if ins.size == 2:
            mn = base + ".b"
        else:
            mn = base + ".w"
    # moveq: capstone da "moveq #$1, d0"
    gops = [conv_operand(o, ins, labelfn) for o in ops]
    # moveq #$ff,dN: capstone muestra el byte sin signo; GAS exige -128..127
    if base == "moveq" and len(ops) == 2 and ops[0].startswith("#"):
        v = int.from_bytes(ins.bytes[1:2], "big", signed=True)
        gops[0] = f"#{v}"
    # move.l #imm,dN con imm pequeño: GAS lo convierte a moveq -> forzar
    if base == "move" and suf == "l" and len(ops) == 2 and ops[0].startswith("#") \
       and re.match(r"^d[0-7]$", ops[1]) and ins.bytes[1] == 0x3c:
        v = int.from_bytes(ins.bytes[2:6], "big", signed=True)
        if -128 <= v <= 127:
            # GAS optimiza siempre a moveq (incluso con :l) -> emitir crudo
            raise ValueError("move.l #imm8,dN no optimizado (GAS lo haría moveq)")
    # addi/subi/andi/ori/eori/cmpi: capstone ya los nombra así; add.w #q
    # con 1..8 => GAS emitiría addq: capstone emite addq cuando lo es.
    return mn, gops


# ---------------------------------------------------------------------------
#  Emisión
# ---------------------------------------------------------------------------
def sym_for_external(addr):
    """Nombre para una dirección fuera de la región actual."""
    if addr in REG_BY_ADDR:
        return REG_BY_ADDR[addr][0]
    if addr in SYMBOLS:
        return SYMBOLS[addr]
    # ¿cae dentro de una isla matcheada?
    import bisect
    i = bisect.bisect_right(REG_SORTED, addr) - 1
    if i >= 0:
        a = REG_SORTED[i]
        n, s, f = REG_BY_ADDR[a]
        if a < addr < a + s:
            if f.endswith(".s"):
                return f"{n}__L{addr:06x}"
            return midisland_name(n, addr, addr - a)
    return None


def midisland_name(island, addr, plus):
    """Nombre convencional (docs/CONVENTIONS.md) para un RTS interno de una
    isla C ya matcheada que es target de un bcc.w colgante."""
    fam = island.split("_")[0]
    pref = {"SetTaskHandler": "SetHandlerRts", "Jsr5B6ThenJmpScheduler": "Jsr5B6Rts",
            "JsrAbsThunk": "JsrAbsRts", "JsrPcThunk": "JsrPcRts", "SetTaskW": "SetTaskWRts",
            "SetTaskB": "SetTaskBRts", "SDS": "SdsRts", "JmpToScheduler": "JmpSchedRts",
            "ClrRamWord": "ClrRamWordRts", "ClrRamByte": "ClrRamByteRts"}.get(fam, fam + "Mid")
    return f"{pref}_{addr:06x}"


MIDISLAND_DEFS = {}   # addr -> (name, island, plus)
PROMOTE_LABELS = {}   # addr -> (name, func, file, plus): labels .L de otros .s a promover
GLOBAL_LABELS = {}    # addr -> nombre global para labels a mitad de entrada (cross-gap)
FORWARD_REFS = {}     # addr -> nombre provisional Sub_XXXXXXXX para targets pc-rel/branch
                      # fuera de la región y sin símbolo (huecos futuros): se emiten como
                      # defsym forward en symbols.py en vez de hex crudo (los pc-rel no
                      # admiten literal absoluto sin perder el matching del encoding).


def collect_midisland(addr):
    import bisect
    i = bisect.bisect_right(REG_SORTED, addr) - 1
    if i >= 0:
        a = REG_SORTED[i]
        n, s, f = REG_BY_ADDR[a]
        if a < addr < a + s and addr not in SYMBOLS:
            if f.endswith(".s"):
                # label local de un .s ya matcheado: hay que promoverlo a global
                PROMOTE_LABELS[addr] = (f"{n}__L{addr:06x}", n, f, addr - a)
            else:
                MIDISLAND_DEFS[addr] = (midisland_name(n, addr, addr - a), n, addr - a)


def default_name(addr, kind):
    return {"handler": f"TaskHandler_{addr:06x}",
            "sub": f"Sub_{addr:08X}",
            "data": f"Data_{addr:06x}"}[kind] if kind in ("handler", "sub", "data") \
        else f"Code_{addr:06x}"


def classify_entry(addr, items_by_entry, install_targets, jsr_targets):
    if addr in SYMBOLS:
        return SYMBOLS[addr]
    if addr in jsr_targets:
        return default_name(addr, "sub")
    return default_name(addr, "handler")


def build(rom, start, end, wave_tag, names_override, known_names=None):
    known_names = known_names or {}
    items = disasm_region(rom, start, end)
    entries = find_entries(items, start, end)
    # clasificar targets
    install_targets, jsr_targets = set(), set()
    for kind, off, it in items:
        if kind != "insn":
            continue
        base = it.mn.split(".")[0]
        if base in ("jsr", "bsr"):
            t = pcrel_target(it) if base == "jsr" else branch_target(it)
            if t is not None and start <= t < end:
                jsr_targets.add(t)
        elif base in ("lea", "pea"):
            t = pcrel_target(it)
            if t is not None and start <= t < end:
                install_targets.add(t)
    entry_names = {}
    for a in entries:
        if a in names_override:
            entry_names[a] = names_override[a]
        else:
            entry_names[a] = classify_entry(a, None, install_targets, jsr_targets)
    # mapa dirección -> (entrada que la contiene)
    import bisect
    def entry_of(addr):
        i = bisect.bisect_right(entries, addr) - 1
        return entries[i] if i >= 0 else None

    # labels locales requeridas por entrada
    needed_labels = {}
    externals = set()
    for kind, off, it in items:
        if kind != "insn":
            continue
        tg = []
        t = branch_target(it)
        if t is not None:
            tg.append(t)
        t = pcrel_target(it)
        if t is not None:
            tg.append(t)
        for t in tg:
            if start <= t < end:
                if t not in entry_names:
                    needed_labels.setdefault(entry_of(t), set()).add(t)
            else:
                externals.add(t)

    # Labels que son referenciados desde OTRA entrada deben ser globales.
    cross_labels = {}
    for kind, off, it in items:
        if kind != "insn":
            continue
        for t in filter(None, [branch_target(it), pcrel_target(it)]):
            if start <= t < end and t not in entry_names:
                if entry_of(t) != entry_of(off):
                    cross_labels[t] = f"{entry_names[entry_of(t)]}__L{t:06x}"

    _cur_entry = [None]

    def labelfn(t, pcrel=False):
        if t in names_override:
            return names_override[t]
        if start <= t < end:
            if t in entry_names:
                return entry_names[t]
            if t in cross_labels or t in GLOBAL_LABELS:
                # label promovido a global (referenciado desde otra entrada u
                # otro hueco). Dentro de su PROPIA entrada usamos el alias
                # local .L (emitido junto al global): GAS no resuelve bien
                # bra.b hacia atras a un simbolo global (>128 B) en la misma
                # seccion ("value too large for field of 1 byte").
                if _cur_entry[0] is not None and entry_of(t) == _cur_entry[0]:
                    return f".L{t:06x}"
                return cross_labels.get(t) or GLOBAL_LABELS[t]
            return f".L{t:06x}"
        if t in known_names:
            return known_names[t]
        if t in GLOBAL_LABELS:
            return GLOBAL_LABELS[t]
        collect_midisland(t)
        s = sym_for_external(t)
        if s is None:
            if pcrel:
                # referencia a un hueco futuro: símbolo provisional (defsym forward)
                FORWARD_REFS.setdefault(t, f"Sub_{t:08X}")
                return FORWARD_REFS[t]
            return f"0x{t:x}"
        return s

    lines = []
    sizes = {}
    unresolved = set()
    for ei, ea in enumerate(entries):
        eend = entries[ei + 1] if ei + 1 < len(entries) else end
        name = entry_names[ea]
        _cur_entry[0] = ea
        sizes[name] = (ea, eend - ea)
        lines.append("")
        lines.append("| " + "-" * 76)
        lines.append(f"|  {name}  @ ${ea:06X}  ({eend - ea} B)")
        lines.append("| " + "-" * 76)
        lines.append(f'        .section .text.{name}, "ax", @progbits')
        lines.append(f"        .global {name}")
        lines.append(f"{name}:")
        for kind, off, it in items:
            if off < ea or off >= eend:
                continue
            if off in cross_labels or off in GLOBAL_LABELS:
                gl = cross_labels.get(off) or GLOBAL_LABELS[off]
                lines.append(f"        .global {gl}")
                lines.append(f"{gl}:")
                lines.append(f".L{off:06x}:")
            elif off in needed_labels.get(ea, ()):
                lines.append(f".L{off:06x}:")
            if kind == "data":
                lines.append(f"        .dc.w   0x{int.from_bytes(it, 'big'):04x}"
                             f"{'':<24}| +{off - ea:03x}  (dato / opcode no decodificado)")
                continue
            if kind == "rawinsn":
                ws = [int.from_bytes(it[i:i + 2], 'big') for i in range(0, 8, 2)]
                sz = "l" if ws[0] in (0x48F9, 0x4CF9) else "w"
                tgt = (ws[2] << 16) | ws[3]
                raw = ",".join(f"0x{w:04x}" for w in ws)
                if ws[0] in (0x48B9, 0x48F9):
                    desc = f"movem.{sz} regs(mask ${ws[1]:04X}), 0x{tgt:x}.l"
                else:
                    desc = f"movem.{sz} 0x{tgt:x}.l, regs(mask ${ws[1]:04X})"
                lines.append(f"        .dc.w   {raw:<28} | +{off - ea:03x}  {desc} (capstone no lo decodifica)")
                continue
            try:
                mn, gops = conv_insn(it, labelfn)
            except ValueError as e:
                raw = ",".join(f"0x{w:04x}" for w in
                               [int.from_bytes(it.bytes[i:i + 2], 'big') for i in range(0, it.size, 2)])
                if str(e).startswith("move.l #imm8"):
                    # bytes crudos byte-exactos; no es un hueco sin resolver
                    lines.append(f"        .dc.w   {raw:<28} | +{off - ea:03x}  {it.mn} {it.ops} (sin moveq)")
                    continue
                lines.append(f"        .dc.w   " + raw + f"   | +{off - ea:03x}  !! {it.mn} {it.ops}")
                unresolved.add(off)
                continue
            text = f"{mn:<7} {','.join(gops)}"
            comment = f"| +{off - ea:03x}"
            # anotar externos sin símbolo (hex crudo o forward Sub_XXXXXXXX)
            for t in filter(None, [branch_target(it), pcrel_target(it)]):
                if not (start <= t < end) and sym_for_external(t) is None \
                   and t not in known_names and t not in GLOBAL_LABELS \
                   and t not in names_override:
                    if t in FORWARD_REFS:
                        comment += f"  -> ${t:06X} (hueco futuro, defsym forward)"
                    else:
                        comment += f"  -> ${t:06X} (sin simbolo)"
                        unresolved.add(off)
            lines.append(f"        {text:<39} {comment}")
    header = [
        "| " + "=" * 76,
        "|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching",
        f"|  {wave_tag}",
        f"|  Región: ${start:06X}..${end:06X}  ({end - start:,} B, {len(entries)} entradas)",
        "| " + "=" * 76,
        "|",
        "|  BORRADOR generado por tools/gen_asm_region.py — pendiente de análisis",
        "|  semántico (nombres, comentarios de campo, evidencias).",
        "|",
        "|  Verificación: cada sección .text.<Sym> se coloca en su dirección CPU",
        "|  absoluta y reensambla byte-exacta contra build/mslug_prom.bin",
        "|  (MD5 816b3f74c76b3373993407615f1850fe).",
        "| " + "=" * 76,
        "",
        "        .text",
    ]
    return header + lines, sizes, externals, unresolved, entries, entry_names


# ---------------------------------------------------------------------------
#  Verificación
# ---------------------------------------------------------------------------
def verify(src_text, sizes, rom, extra_defsyms):
    with tempfile.TemporaryDirectory() as td:
        s = os.path.join(td, "draft.s")
        open(s, "w").write(src_text)
        o = os.path.join(td, "draft.o")
        r = subprocess.run(["m68k-linux-gnu-gcc", "-m68000", "-nostdlib", "-nostartfiles",
                            "-ffreestanding", "-Wa,--register-prefix-optional",
                            "-c", s, "-o", o], capture_output=True, text=True)
        if r.returncode:
            return None, r.stderr
        ld = os.path.join(td, "d.ld")
        L = ["OUTPUT_FORMAT(elf32-m68k)", "OUTPUT_ARCH(m68k)", "SECTIONS {"]
        for name, (addr, size) in sorted(sizes.items(), key=lambda kv: kv[1][0]):
            L.append(f"  . = 0x{addr:X};")
            L.append(f"  .text.{name} : {{ KEEP(*(.text.{name})) }}")
        L.append("  /DISCARD/ : { *(.text) *(.data*) *(.bss*) *(.comment) *(.note.*) }")
        L.append("}")
        open(ld, "w").write("\n".join(L))
        defs = [f"-Wl,--defsym={n}=0x{a:X}" for a, n in SYMBOLS.items() if n not in sizes]
        defs += [f"-Wl,--defsym={n}=0x{a:X}" for n, a, _, _ in REGISTRY if n not in sizes]
        defs += [f"-Wl,--defsym={n}=0x{a:X}" for n, a in extra_defsyms.items()]
        elf = os.path.join(td, "d.elf")
        r = subprocess.run(["m68k-linux-gnu-gcc", "-m68000", "-nostdlib", "-nostartfiles",
                            "-Wl,-T", ld, *defs, o, "-o", elf], capture_output=True, text=True)
        if r.returncode:
            return None, r.stderr
        results = {}
        for name, (addr, size) in sizes.items():
            b = os.path.join(td, name + ".bin")
            subprocess.run(["m68k-linux-gnu-objcopy", "-O", "binary",
                            f"--only-section=.text.{name}", elf, b], capture_output=True)
            got = open(b, "rb").read()[:size] if os.path.exists(b) else b""
            want = rom[addr:addr + size]
            results[name] = (got == want, got, want)
        return results, ""


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("start", type=lambda x: int(x, 16))
    ap.add_argument("end", type=lambda x: int(x, 16))
    ap.add_argument("-o", "--output")
    ap.add_argument("--wave", default="Wave ??? — (borrador)")
    ap.add_argument("--print-entries", action="store_true")
    ap.add_argument("--registry", action="store_true",
                    help="imprime las líneas para tools/registry.py")
    ap.add_argument("--src", default="draft.s", help="nombre de archivo para --registry")
    ap.add_argument("--name", action="append", default=[],
                    help="ADDR=Nombre para renombrar una entrada")
    ap.add_argument("--entry", action="append", default=[],
                    help="forzar una frontera de entrada adicional (hex)")
    ap.add_argument("--data", action="append", default=[],
                    help="START-END (hex) a emitir como .dc.w (tabla embebida en .text); "
                         "START pasa a ser una entrada Data_XXXXXX")
    ap.add_argument("--no-verify", action="store_true")
    args = ap.parse_args()
    for d in args.data:
        a, b = (int(x, 16) for x in d.split("-"))
        DATA_RANGES.append((a, b))
        SYMBOLS.setdefault(a, f"Data_{a:06x}")

    rom = open(PROM, "rb").read()
    names = {}
    for nv in args.name:
        a, n = nv.split("=")
        names[int(a, 16)] = n
    for e in args.entry:
        a = int(e, 16)
        SYMBOLS.setdefault(a, names.get(a, f"TaskHandler_{a:06x}"))

    # Recorre todos los huecos entre start y end saltando islas matcheadas.
    gaps = []
    cur = args.start
    for a in REG_SORTED:
        if a + REG_BY_ADDR[a][1] <= cur:
            continue
        if a >= args.end:
            break
        if a > cur:
            gaps.append((cur, a))
        cur = max(cur, a + REG_BY_ADDR[a][1])
    if cur < args.end:
        gaps.append((cur, args.end))

    # Pase 1: entradas de todos los huecos -> nombres globales conocidos
    known = {}
    all_entries = []
    for gs, ge in gaps:
        _, _, _, _, en, enm = build(rom, gs, ge, args.wave, names)
        known.update(enm)
        all_entries += en
    # targets de branch/pc-rel que caen a mitad de una entrada de OTRO hueco
    import bisect
    all_entries.sort()
    for gs, ge in gaps:
        for kind, off, it in disasm_region(rom, gs, ge):
            if kind != "insn":
                continue
            for t in filter(None, [branch_target(it), pcrel_target(it)]):
                if t in known or any(g0 <= t < g1 for g0, g1 in gaps if g0 <= off < g1):
                    continue
                if not any(g0 <= t < g1 for g0, g1 in gaps):
                    continue
                i = bisect.bisect_right(all_entries, t) - 1
                owner = known[all_entries[i]]
                GLOBAL_LABELS[t] = f"{owner}__L{t:06x}"
    MIDISLAND_DEFS.clear()
    FORWARD_REFS.clear()
    # Pase 2: emisión con referencias cruzadas resueltas por nombre
    lines, sizes, externals, unresolved, entries, entry_names = None, {}, set(), set(), [], {}
    for gi, (gs, ge) in enumerate(gaps):
        L, sz, ex, un, en, enm = build(rom, gs, ge, args.wave, names, known)
        if lines is None:
            lines = L
        else:
            lines += L[L.index("        .text") + 1:]
        sizes.update(sz); externals |= ex; unresolved |= un
        entries += en; entry_names.update(enm)
    if lines is None:
        print("sin huecos en el rango"); sys.exit(0)
    lines[3] = f"|  Región: ${args.start:06X}..${args.end:06X}  ({sum(s for _, s in sizes.values()):,} B, {len(entries)} entradas, {len(gaps)} huecos)"
    text = "\n".join(lines) + "\n"

    if args.print_entries:
        for a in entries:
            print(f"  ${a:06X}  {sizes[entry_names[a]][1]:5d} B  {entry_names[a]}")
    if args.registry:
        for a in entries:
            n = entry_names[a]
            print(f'    ("{n}",{"":<{max(1, 45 - len(n) - 3)}} 0x{a:06X}, {sizes[n][1]:3d}, "{args.src}"),')

    if PROMOTE_LABELS:
        print(f"[!] {len(PROMOTE_LABELS)} labels locales de otros .s deben promoverse a globales:")
        for a, (n, fn, f, plus) in sorted(PROMOTE_LABELS.items()):
            print(f"    asm/{f}: .L{a:x} (en {fn}+{plus}) -> .global {n}")
    if MIDISLAND_DEFS:
        print(f"[i] {len(MIDISLAND_DEFS)} RTS internos de islas C nuevos (añadir a symbols.py):")
        for a, (n, isl, plus) in sorted(MIDISLAND_DEFS.items()):
            print(f'    0x{a:08X}: "{n}",  # rts de {isl} (+{plus})')
    if FORWARD_REFS:
        print(f"[i] {len(FORWARD_REFS)} refs forward a huecos futuros (añadir a symbols.py):")
        for a, n in sorted(FORWARD_REFS.items()):
            print(f'    0x{a:08X}: "{n}",  # hueco futuro (ref pc-rel desde esta region)')
    ext_unknown = sorted(t for t in externals if sym_for_external(t) is None
                         and t not in known and t not in GLOBAL_LABELS and t not in names
                         and t not in FORWARD_REFS)
    if ext_unknown:
        print(f"[i] {len(ext_unknown)} referencias externas sin símbolo (se dejan en hex):")
        for t in ext_unknown:
            print(f"      ${t:06X}")

    if not args.no_verify:
        # defsyms para labels cross-entry y externos sin símbolo que NO son hex
        extra = {n: a for a, (n, _, _) in MIDISLAND_DEFS.items()}
        extra.update({n: a for a, (n, _, _, _) in PROMOTE_LABELS.items() if a not in names})
        extra.update({n: a for a, n in names.items() if n not in sizes})
        extra.update({n: a for a, n in FORWARD_REFS.items()})
        results, err = verify(text, sizes, rom, extra)
        if results is None:
            print("[ASM/LINK FAIL]\n" + err)
            sys.exit(1)
        bad = [n for n, (ok, g, w) in results.items() if not ok]
        print(f"[verify] {len(results) - len(bad)}/{len(results)} entradas byte-exactas")
        for n in bad:
            ok, g, w = results[n]
            addr, size = sizes[n]
            # primer byte distinto
            i = next((i for i in range(min(len(g), len(w))) if g[i] != w[i]), min(len(g), len(w)))
            print(f"   [!] {n} @ ${addr:06X} ({size} B) primer diff en +{i:03x}: "
                  f"got={g[i:i+8].hex()} want={w[i:i+8].hex()}")
    if unresolved:
        print(f"[i] {len(unresolved)} instrucciones marcadas !! para revisión")
    if args.output:
        open(args.output, "w").write(text)
        print(f"[ok] escrito {args.output}")


if __name__ == "__main__":
    main()
