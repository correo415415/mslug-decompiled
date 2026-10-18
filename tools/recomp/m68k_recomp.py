#!/usr/bin/env python3
"""
m68k_recomp.py — recompilador estático 68000 -> Rust para el port de Metal Slug.

Cada instrucción 68000 alcanzable del programa (P-ROM del juego + BIOS) se
traduce a una función Rust especializada: operandos, modos de
direccionamiento, desplazamientos y destinos de salto se resuelven aquí, en
tiempo de generación. El resultado no decodifica nada en tiempo de
ejecución: es el programa del juego escrito en Rust, instrucción a
instrucción, con la misma semántica (registros, flags X N Z V C, accesos al
bus en el mismo orden) y el mismo número de ciclos que el núcleo 68000 de
referencia (crates/neogeo-hw/src/cpu/m68k/exec.rs), para que el resto del
sistema (vídeo, sonido, interrupciones) no note la diferencia.

Entrada:
    build/recomp/prom.bin   P-ROM tal como la ve la CPU (mslug dump-roms)
    build/recomp/bios.bin   BIOS cargada
    build/recomp/trace.bin  bitset de instrucciones ejecutadas (mslug trace)
    tools/recomp/seeds.txt  entradas extra (opcional, una dirección hex por línea)
    tools/recomp/symbols.tsv nombres de la decompilación (comentarios)

Salida: crates/mslug-code/src/gen/*.rs

Descubrimiento de código: descenso recursivo desde las semillas (traza +
vectores + semillas extra), siguiendo saltos, llamadas y ramas estáticas.
Los saltos indirectos se resuelven en ejecución por la tabla de despacho; si
el PC cae en una dirección no traducida el runtime lo registra (`misses`) y
esa dirección se añade a seeds.txt en la siguiente pasada.
"""
import os
import sys
import struct
import argparse

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", ".."))

M = {"b": 0xFF, "w": 0xFFFF, "l": 0xFFFFFFFF}
SB = {"b": 0x80, "w": 0x8000, "l": 0x80000000}
NB = {"b": 1, "w": 2, "l": 4}
RS = {"b": "Size::Byte", "w": "Size::Word", "l": "Size::Long"}
RT = {"b": "u8", "w": "u16", "l": "u32"}


class Unsupported(Exception):
    pass


# ---------------------------------------------------------------------------
# Memoria de programa (vista CPU)
# ---------------------------------------------------------------------------
class Mem:
    def __init__(self, prom, bios):
        self.prom = prom
        self.bios = bios

    def valid(self, a):
        a &= 0xFFFFFF
        return (0x80 <= a < 0x100000) or (0x200000 <= a < 0x300000) or (0xC00000 <= a < 0xD00000)

    def r16(self, a):
        a &= 0xFFFFFF
        if 0x80 <= a < 0x100000:
            return struct.unpack_from(">H", self.prom, a)[0]
        if 0x200000 <= a < 0x300000:
            o = 0x100000 + (a - 0x200000)
            return struct.unpack_from(">H", self.prom, o)[0]
        if 0xC00000 <= a < 0xD00000:
            return struct.unpack_from(">H", self.bios, a & 0x1FFFF)[0]
        raise Unsupported("fetch fuera de ROM %06X" % a)


def sx8(v):
    v &= 0xFF
    return v - 0x100 if v & 0x80 else v


def sx16(v):
    v &= 0xFFFF
    return v - 0x10000 if v & 0x8000 else v


def u32(v):
    return v & 0xFFFFFFFF


def hx(v):
    return "0x%X" % (v & 0xFFFFFFFF)


# ---------------------------------------------------------------------------
# Emisor: acumula sentencias Rust con nombres temporales únicos
# ---------------------------------------------------------------------------
class Emit:
    def __init__(self):
        self.lines = []
        self.n = 0

    def tmp(self, p="t"):
        self.n += 1
        return "%s%d" % (p, self.n)

    def __call__(self, s):
        self.lines.append(s)


# ---------------------------------------------------------------------------
# Modos de direccionamiento (equivalente a ea.rs, resuelto en generación)
# ---------------------------------------------------------------------------
class EA:
    """mode/reg del 68000 + extensiones ya leídas."""

    def __init__(self, kind, reg=0, val=0, xreg=0, xaddr=False, xlong=False):
        self.kind = kind      # dn an ind post pre d16 idx absw absl pcd pcidx imm
        self.reg = reg
        self.val = val        # disp / dirección / inmediato
        self.xreg = xreg
        self.xaddr = xaddr
        self.xlong = xlong

    @staticmethod
    def decode(cur, bits, size):
        mode = (bits >> 3) & 7
        reg = bits & 7
        if mode == 0:
            return EA("dn", reg)
        if mode == 1:
            return EA("an", reg)
        if mode == 2:
            return EA("ind", reg)
        if mode == 3:
            return EA("post", reg)
        if mode == 4:
            return EA("pre", reg)
        if mode == 5:
            return EA("d16", reg, sx16(cur.fetch16()))
        if mode == 6:
            ext = cur.fetch16()
            return EA("idx", reg, sx8(ext), (ext >> 12) & 7, bool(ext & 0x8000), bool(ext & 0x800))
        if reg == 0:
            return EA("absw", 0, u32(sx16(cur.fetch16())))
        if reg == 1:
            return EA("absl", 0, cur.fetch32())
        if reg == 2:
            pc = cur.pc
            return EA("pcd", 0, u32(pc + sx16(cur.fetch16())))
        if reg == 3:
            pc = cur.pc
            ext = cur.fetch16()
            return EA("pcidx", 0, u32(pc + sx8(ext)), (ext >> 12) & 7, bool(ext & 0x8000), bool(ext & 0x800))
        if reg == 4:
            if size == "b":
                return EA("imm", 0, cur.fetch16() & 0xFF)
            if size == "w":
                return EA("imm", 0, cur.fetch16())
            return EA("imm", 0, cur.fetch32())
        return EA("imm", 0, 0)

    def is_reg(self):
        return self.kind in ("dn", "an")

    def is_mem(self):
        return self.kind not in ("dn", "an", "imm")

    def cycles(self, size):
        lg = size == "l"
        k = self.kind
        if k in ("dn", "an", "imm"):
            return 0
        if k in ("ind", "post"):
            return 8 if lg else 4
        if k == "pre":
            return 10 if lg else 6
        if k in ("d16", "absw", "pcd"):
            return 12 if lg else 8
        if k in ("idx", "pcidx"):
            return 14 if lg else 10
        if k == "absl":
            return 16 if lg else 12
        raise AssertionError(k)

    def _idx(self):
        src = "cpu.a[%d]" % self.xreg if self.xaddr else "cpu.d[%d]" % self.xreg
        if self.xlong:
            return src
        return "(%s as i16 as i32 as u32)" % src

    def addr(self, e, size):
        """Calcula la dirección (aplica post/pre-incremento UNA vez)."""
        k, r = self.kind, self.reg
        if k == "ind":
            return "cpu.a[%d]" % r
        if k == "post":
            inc = 2 if (r == 7 and size == "b") else NB[size]
            t = e.tmp("ad")
            e("let %s = cpu.a[%d]; cpu.a[%d] = %s.wrapping_add(%d);" % (t, r, r, t, inc))
            return t
        if k == "pre":
            dec = 2 if (r == 7 and size == "b") else NB[size]
            t = e.tmp("ad")
            e("let %s = cpu.a[%d].wrapping_sub(%d); cpu.a[%d] = %s;" % (t, r, dec, r, t))
            return t
        if k == "d16":
            t = e.tmp("ad")
            e("let %s = cpu.a[%d].wrapping_add(%s);" % (t, r, hx(u32(self.val))))
            return t
        if k == "idx":
            t = e.tmp("ad")
            e("let %s = cpu.a[%d].wrapping_add(%s).wrapping_add(%s);" % (t, r, hx(u32(self.val)), self._idx()))
            return t
        if k in ("absw", "absl", "pcd"):
            return hx(self.val)
        if k == "pcidx":
            t = e.tmp("ad")
            e("let %s = %s_u32.wrapping_add(%s);" % (t, hx(self.val), self._idx()))
            return t
        raise Unsupported("addr de %s" % k)

    def static_addr(self):
        if self.kind in ("absw", "absl", "pcd"):
            return self.val & 0xFFFFFF
        return None


def rd(e, addr, size):
    t = e.tmp("v")
    if size == "b":
        e("let %s = bus.read8(%s) as u32;" % (t, addr))
    elif size == "w":
        e("let %s = bus.read16(%s) as u32;" % (t, addr))
    else:
        e("let %s = bus.read32(%s);" % (t, addr))
    return t


def wr(e, addr, size, v):
    if size == "b":
        e("bus.write8(%s, %s as u8);" % (addr, v))
    elif size == "w":
        e("bus.write16(%s, %s as u16);" % (addr, v))
    else:
        e("bus.write32(%s, %s);" % (addr, v))


def ea_read(e, ea, size):
    """Ea::read: devuelve el nombre de una variable u32."""
    t = e.tmp("v")
    if ea.kind == "dn":
        e("let %s = cpu.d[%d] & %s;" % (t, ea.reg, hx(M[size])))
        return t
    if ea.kind == "an":
        if size == "w":
            e("let %s = cpu.a[%d] as i16 as i32 as u32;" % (t, ea.reg))
        else:
            e("let %s = cpu.a[%d] & %s;" % (t, ea.reg, hx(M[size])))
        return t
    if ea.kind == "imm":
        e("let %s: u32 = %s;" % (t, hx(ea.val & M[size])))
        return t
    a = ea.addr(e, size)
    return rd(e, a, size)


def ea_write(e, ea, size, v):
    """Ea::write."""
    k = ea.kind
    if k == "dn":
        m = M[size]
        if size == "l":
            e("cpu.d[%d] = %s;" % (ea.reg, v))
        else:
            e("cpu.d[%d] = (cpu.d[%d] & !%s) | (%s & %s);" % (ea.reg, ea.reg, hx(m), v, hx(m)))
        return
    if k == "an":
        if size == "w":
            e("cpu.a[%d] = %s as i16 as i32 as u32;" % (ea.reg, v))
        else:
            e("cpu.a[%d] = %s;" % (ea.reg, v))
        return
    if k in ("imm", "pcd", "pcidx"):
        return
    if k == "pre" and size == "l":
        # MAME: palabra baja en An-2, luego alta en An-4; An se confirma al final
        t = e.tmp("o")
        e("let %s = cpu.a[%d];" % (t, ea.reg))
        e("bus.write16(%s.wrapping_sub(2), %s as u16);" % (t, v))
        e("cpu.a[%d] = %s.wrapping_sub(4);" % (ea.reg, t))
        e("bus.write16(%s.wrapping_sub(4), (%s >> 16) as u16);" % (t, v))
        return
    a = ea.addr(e, size)
    wr(e, a, size, v)


def ea_modify(e, ea, size, fn):
    """Ea::modify: fn(e, v) -> nombre del nuevo valor (u32)."""
    k = ea.kind
    m = M[size]
    if k == "dn":
        v = e.tmp("v")
        e("let %s = cpu.d[%d] & %s;" % (v, ea.reg, hx(m)))
        n = fn(e, v)
        if size == "l":
            e("cpu.d[%d] = %s;" % (ea.reg, n))
        else:
            e("cpu.d[%d] = (cpu.d[%d] & !%s) | (%s & %s);" % (ea.reg, ea.reg, hx(m), n, hx(m)))
        return
    if k == "an":
        v = e.tmp("v")
        e("let %s = cpu.a[%d] & %s;" % (v, ea.reg, hx(m)))
        n = fn(e, v)
        if size == "w":
            e("cpu.a[%d] = %s as i16 as i32 as u32;" % (ea.reg, n))
        else:
            e("cpu.a[%d] = %s;" % (ea.reg, n))
        return
    if k in ("imm", "pcd", "pcidx"):
        v = ea_read(e, ea, size)
        fn(e, v)
        return
    a = ea.addr(e, size)
    if not a.startswith("ad") and not a.startswith("cpu"):
        pass
    if a.startswith("cpu."):
        t = e.tmp("ad")
        e("let %s = %s;" % (t, a))
        a = t
    v = rd(e, a, size)
    n = fn(e, v)
    wr(e, a, size, n)


# ---------------------------------------------------------------------------
# Traducción de una instrucción (espejo de exec.rs)
# ---------------------------------------------------------------------------
COND = ["True", "False", "Hi", "Ls", "Cc", "Cs", "Ne", "Eq",
        "Vc", "Vs", "Pl", "Mi", "Ge", "Lt", "Gt", "Le"]


class Cur:
    def __init__(self, mem, pc):
        self.mem = mem
        self.start = pc
        self.pc = pc

    def fetch16(self):
        v = self.mem.r16(self.pc)
        self.pc = (self.pc + 2) & 0xFFFFFFFF
        return v

    def fetch32(self):
        hi = self.fetch16()
        lo = self.fetch16()
        return (hi << 16) | lo


class Ins:
    """Resultado: cuerpo Rust + flujo de control estático."""

    def __init__(self, addr):
        self.addr = addr
        self.e = Emit()
        self.next = None        # dirección de la siguiente instrucción (fallthrough)
        self.targets = []       # destinos estáticos (saltos / llamadas)
        self.falls = True       # ¿puede continuar en self.next?
        self.text = ""


def logic(e, v, size):
    e("cpu.sr.set_nz(%s, %s); cpu.sr.0 &= !0x3;" % (v, RS[size]))


def size2(bits):
    return {0: "b", 1: "w", 2: "l"}.get(bits)


def priv_check(e):
    e("if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }")


def exc(e, name, cyc, rewind=True):
    if rewind:
        e("cpu.pc = cpu.instr_pc;")
    e("cpu.enter_exception(bus, Exception::%s);" % name)
    e("return %d;" % cyc)


def translate(mem, addr):
    cur = Cur(mem, addr)
    I = Ins(addr)
    e = I.e
    op = cur.fetch16()
    e("cpu.ir = %s;" % hx(op))
    hi = op >> 12
    cyc = None

    def done(c):
        nonlocal cyc
        cyc = c

    def setpc_next():
        pass  # cpu.pc = siguiente instrucción se fija al principio del cuerpo

    # ---------------- MOVE / MOVEA ----------------
    if hi in (1, 2, 3):
        size = {1: "b", 2: "l", 3: "w"}[hi]
        src = EA.decode(cur, op & 0x3F, size)
        dbits = (op >> 6) & 0x3F
        dbits = ((dbits & 7) << 3) | (dbits >> 3)
        dst = EA.decode(cur, dbits, size)
        if dst.kind in ("imm", "pcd", "pcidx"):
            raise Unsupported("move a destino no alterable")
        if dst.kind == "an" and size == "b":
            raise Unsupported("movea.b")
        v = ea_read(e, src, size)
        if dst.kind != "an":
            if size != "l":
                logic(e, v, size)
        ea_write(e, dst, size, v)
        if dst.kind != "an" and size == "l":
            logic(e, v, size)
        setpc_next()
        done(4 + src.cycles(size) + dst.cycles(size))

    # ---------------- MOVEQ ----------------
    elif hi == 7:
        if op & 0x100:
            raise Unsupported("moveq ilegal")
        reg = (op >> 9) & 7
        val = u32(sx8(op & 0xFF))
        e("cpu.d[%d] = %s;" % (reg, hx(val)))
        logic(e, "cpu.d[%d]" % reg, "l")
        setpc_next()
        done(4)

    # ---------------- Bcc / BRA / BSR ----------------
    elif hi == 6:
        cond = (op >> 8) & 0xF
        d8 = sx8(op & 0xFF)
        base = cur.pc
        if d8 == 0:
            target = u32(base + sx16(cur.fetch16()))
        else:
            target = u32(base + d8)
        target &= 0xFFFFFF
        if target & 1:
            raise Unsupported("salto impar")
        I.targets.append(target)
        if cond == 0:
            e("cpu.pc = %s;" % hx(target))
            I.falls = False
            done(10)
        elif cond == 1:
            e("cpu.push32(bus, %s);" % hx(cur.pc))
            e("cpu.pc = %s;" % hx(target))
            done(18)
        else:
            e("if cpu.sr.evaluate(Condition::%s) { cpu.pc = %s; return 10; }" % (COND[cond], hx(target)))
            setpc_next()
            done(8)

    # ---------------- línea 4 ----------------
    elif hi == 4:
        done(line4(cur, I, op, e))

    # ---------------- ADDQ / SUBQ / Scc / DBcc ----------------
    elif hi == 5:
        sb = (op >> 6) & 3
        if sb == 3:
            mode = (op >> 3) & 7
            cond = (op >> 8) & 0xF
            if mode == 1:
                reg = op & 7
                dpc = cur.pc
                target = u32(dpc + sx16(cur.fetch16())) & 0xFFFFFF
                I.targets.append(target)
                e("if cpu.sr.evaluate(Condition::%s) { cpu.pc = %s; return 12; }" % (COND[cond], hx(cur.pc)))
                e("let n = (cpu.d[%d] as u16).wrapping_sub(1);" % reg)
                e("cpu.d[%d] = (cpu.d[%d] & 0xFFFF_0000) | n as u32;" % (reg, reg))
                e("if n == 0xFFFF { cpu.pc = %s; return 14; }" % hx(cur.pc))
                e("cpu.pc = %s;" % hx(target))
                done(10)
            else:
                ea = EA.decode(cur, op & 0x3F, "b")
                if ea.kind in ("an", "imm", "pcd", "pcidx"):
                    raise Unsupported("scc ea")
                t = e.tmp("c")
                e("let %s: u32 = if cpu.sr.evaluate(Condition::%s) { 0xFF } else { 0 };" % (t, COND[cond]))
                ea_modify(e, ea, "b", lambda e, v: t)
                setpc_next()
                done(4 + ea.cycles("b"))
        else:
            size = size2(sb)
            is_sub = bool(op & 0x100)
            imm = (op >> 9) & 7 or 8
            ea = EA.decode(cur, op & 0x3F, size)
            if ea.kind == "an":
                f = "wrapping_sub" if is_sub else "wrapping_add"
                e("cpu.a[%d] = cpu.a[%d].%s(%d);" % (ea.reg, ea.reg, f, imm))
                setpc_next()
                done(8)
            else:
                fn = "sub_with_flags" if is_sub else "add_with_flags"

                def m(e, v):
                    t = e.tmp("r")
                    e("let %s = %s(cpu, %s, %d, %s, true);" % (t, fn, v, imm, RS[size]))
                    return t
                ea_modify(e, ea, size, m)
                setpc_next()
                done(4 + ea.cycles(size))

    # ---------------- OR / DIV / SBCD ----------------
    elif hi == 8:
        opm = (op >> 6) & 7
        if opm == 3:
            done(div(cur, e, op, False))
        elif opm == 7:
            done(div(cur, e, op, True))
        elif (op & 0xF1F0) == 0x8100:
            done(bcd(cur, e, op, "sbcd"))
        else:
            done(or_and(cur, e, op, "|"))
        if cyc is not None:
            setpc_next()

    # ---------------- SUB ----------------
    elif hi == 9:
        done(addsub(cur, e, op, True))
        setpc_next()

    # ---------------- CMP / EOR ----------------
    elif hi == 0xB:
        done(cmp_eor(cur, e, op))
        setpc_next()

    # ---------------- AND / MUL / ABCD / EXG ----------------
    elif hi == 0xC:
        opm = (op >> 6) & 7
        if opm in (3, 7):
            ea = EA.decode(cur, op & 0x3F, "w")
            reg = (op >> 9) & 7
            v = ea_read(e, ea, "w")
            if opm == 3:
                e("let r = (%s & 0xFFFF).wrapping_mul(cpu.d[%d] & 0xFFFF);" % (v, reg))
            else:
                e("let r = ((%s as u16 as i16 as i32).wrapping_mul(cpu.d[%d] as u16 as i16 as i32)) as u32;" % (v, reg))
            e("cpu.d[%d] = r;" % reg)
            logic(e, "r", "l")
            done(70 + ea.cycles("w"))
        elif (op & 0xF1F0) == 0xC100:
            done(bcd(cur, e, op, "abcd"))
        elif (op & 0xF130) == 0xC100 and ((op >> 3) & 0x1F) in (0b01000, 0b01001, 0b10001):
            mode = (op >> 3) & 0x1F
            rx, ry = (op >> 9) & 7, op & 7
            if mode == 0b01000:
                e("cpu.d.swap(%d, %d);" % (rx, ry))
            elif mode == 0b01001:
                e("cpu.a.swap(%d, %d);" % (rx, ry))
            else:
                e("let t = cpu.d[%d]; cpu.d[%d] = cpu.a[%d]; cpu.a[%d] = t;" % (rx, rx, ry, ry))
            done(6)
        else:
            done(or_and(cur, e, op, "&"))
        setpc_next()

    # ---------------- ADD ----------------
    elif hi == 0xD:
        done(addsub(cur, e, op, False))
        setpc_next()

    # ---------------- shifts ----------------
    elif hi == 0xE:
        done(shift(cur, e, op))
        setpc_next()

    # ---------------- línea 0: inmediatos / bits / movep ----------------
    elif hi == 0:
        done(line0(cur, e, op))
        setpc_next()

    elif hi == 0xA:
        exc(e, "LineA", 34)
        I.falls = False
        cyc = "dead"
    elif hi == 0xF:
        exc(e, "LineF", 34)
        I.falls = False
        cyc = "dead"

    if cyc is None:
        raise Unsupported("opcode %04X" % op)
    I.next = cur.pc & 0xFFFFFF
    I.size = (cur.pc - addr) & 0xFFFFFF
    I.op = op
    if cyc != "dead":
        e("%d" % cyc if isinstance(cyc, int) else cyc)
    # Como en el intérprete: tras leer opcode + extensiones, PC apunta a la
    # siguiente instrucción (lo que apilan las excepciones / jsr / bsr).
    I.body = ["cpu.pc = %s;" % hx(I.next)] + e.lines
    return I


# ---------------------------------------------------------------------------
# Familias
# ---------------------------------------------------------------------------
def addsub(cur, e, op, is_sub):
    reg = (op >> 9) & 7
    opm = (op >> 6) & 7
    fn = "sub_with_flags" if is_sub else "add_with_flags"
    if opm in (3, 7):  # ADDA / SUBA
        size = "w" if opm == 3 else "l"
        ea = EA.decode(cur, op & 0x3F, size)
        v = ea_read(e, ea, size)
        if size == "w":
            v2 = "(%s as u16 as i16 as i32 as u32)" % v
        else:
            v2 = v
        f = "wrapping_sub" if is_sub else "wrapping_add"
        e("cpu.a[%d] = cpu.a[%d].%s(%s);" % (reg, reg, f, v2))
        return 8 + ea.cycles(size)
    size = size2(opm & 3)
    dr_to_ea = bool(opm & 4)
    mb = (op >> 3) & 7
    if dr_to_ea and mb in (0, 1):  # ADDX / SUBX
        rm = bool(op & 8)
        sy, dx = op & 7, (op >> 9) & 7
        xf = "subx_with_flags" if is_sub else "addx_with_flags"
        e("let x = cpu.sr.get(StatusRegister::X) as u32;")
        if not rm:
            e("let r = %s(cpu, cpu.d[%d] & %s, cpu.d[%d] & %s, x, %s);" % (xf, dx, hx(M[size]), sy, hx(M[size]), RS[size]))
            ea_write(e, EA("dn", dx), size, "r")
        else:
            if size == "l":
                raise Unsupported("addx.l -(An)")
            dy = 2 if (sy == 7 and size == "b") else NB[size]
            dd = 2 if (dx == 7 and size == "b") else NB[size]
            e("cpu.a[%d] = cpu.a[%d].wrapping_sub(%d);" % (sy, sy, dy))
            s_ = rd(e, "cpu.a[%d]" % sy, size)
            e("cpu.a[%d] = cpu.a[%d].wrapping_sub(%d);" % (dx, dx, dd))
            d_ = rd(e, "cpu.a[%d]" % dx, size)
            e("let r = %s(cpu, %s, %s, x, %s);" % (xf, d_, s_, RS[size]))
            wr(e, "cpu.a[%d]" % dx, size, "r")
        return 8 if size == "l" else 4
    ea = EA.decode(cur, op & 0x3F, size)
    if dr_to_ea:
        def m(e, v):
            t = e.tmp("r")
            e("let %s = %s(cpu, %s, cpu.d[%d] & %s, %s, true);" % (t, fn, v, reg, hx(M[size]), RS[size]))
            return t
        ea_modify(e, ea, size, m)
    else:
        v = ea_read(e, ea, size)
        e("let r = %s(cpu, cpu.d[%d] & %s, %s, %s, true);" % (fn, reg, hx(M[size]), v, RS[size]))
        ea_write(e, EA("dn", reg), size, "r")
    return 4 + ea.cycles(size)


def cmp_eor(cur, e, op):
    reg = (op >> 9) & 7
    opm = (op >> 6) & 7
    if opm in (3, 7):  # CMPA
        size = "w" if opm == 3 else "l"
        ea = EA.decode(cur, op & 0x3F, size)
        v = ea_read(e, ea, size)
        v2 = "(%s as u16 as i16 as i32 as u32)" % v if size == "w" else v
        e("sub_with_flags(cpu, cpu.a[%d], %s, Size::Long, false);" % (reg, v2))
        return 6 + ea.cycles(size)
    size = size2(opm & 3)
    if opm & 4:
        mb = (op >> 3) & 7
        if mb == 1:  # CMPM
            if size == "l":
                raise Unsupported("cmpm.l")
            ay, ax = op & 7, (op >> 9) & 7
            iy = 2 if (ay == 7 and size == "b") else NB[size]
            ix = 2 if (ax == 7 and size == "b") else NB[size]
            e("let ay = cpu.a[%d]; cpu.a[%d] = ay.wrapping_add(%d);" % (ay, ay, iy))
            s_ = rd(e, "ay", size)
            e("let ax = cpu.a[%d];" % ax)
            d_ = rd(e, "ax", size)
            e("cpu.a[%d] = cpu.a[%d].wrapping_add(%d);" % (ax, ax, ix))
            e("sub_with_flags(cpu, %s, %s, %s, false);" % (d_, s_, RS[size]))
            return 12
        ea = EA.decode(cur, op & 0x3F, size)

        def m(e, v):
            t = e.tmp("r")
            e("let %s = (%s ^ cpu.d[%d]) & %s;" % (t, v, reg, hx(M[size])))
            logic(e, t, size)
            return t
        ea_modify(e, ea, size, m)
        return 4 + ea.cycles(size)
    ea = EA.decode(cur, op & 0x3F, size)
    v = ea_read(e, ea, size)
    e("sub_with_flags(cpu, cpu.d[%d] & %s, %s, %s, false);" % (reg, hx(M[size]), v, RS[size]))
    return 4 + ea.cycles(size)


def or_and(cur, e, op, o):
    reg = (op >> 9) & 7
    opm = (op >> 6) & 7
    size = size2(opm & 3)
    ea = EA.decode(cur, op & 0x3F, size)
    if opm & 4:
        def m(e, v):
            t = e.tmp("r")
            e("let %s = (%s %s cpu.d[%d]) & %s;" % (t, v, o, reg, hx(M[size])))
            logic(e, t, size)
            return t
        ea_modify(e, ea, size, m)
    else:
        v = ea_read(e, ea, size)
        e("let r = (%s %s cpu.d[%d]) & %s;" % (v, o, reg, hx(M[size])))
        ea_write(e, EA("dn", reg), size, "r")
        logic(e, "r", size)
    return 4 + ea.cycles(size)


def div(cur, e, op, signed):
    reg = (op >> 9) & 7
    ea = EA.decode(cur, op & 0x3F, "w")
    v = ea_read(e, ea, "w")
    c = ea.cycles("w")
    e("cpu.sr.set(StatusRegister::C, false);")
    if not signed:
        e("let dv = %s & 0xFFFF;" % v)
        e("if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }")
        e("let q = cpu.d[%d] / dv; let rm = cpu.d[%d] %% dv;" % (reg, reg))
        e("if q > 0xFFFF { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return %d; }" % (76 + c))
        e("cpu.d[%d] = (rm << 16) | (q & 0xFFFF);" % reg)
        e("cpu.sr.set_nz(q, Size::Word); cpu.sr.set(StatusRegister::V, false);")
        return 140 + c
    e("let dv = %s as u16 as i16 as i32;" % v)
    e("if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }")
    e("let dd = cpu.d[%d] as i32; let q = dd.wrapping_div(dv); let rm = dd.wrapping_rem(dv);" % reg)
    e("if !(-32768..=32767).contains(&q) { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return %d; }" % (158 + c))
    e("cpu.d[%d] = ((rm as u32 & 0xFFFF) << 16) | (q as u32 & 0xFFFF);" % reg)
    e("cpu.sr.set_nz(q as u32 & 0xFFFF, Size::Word); cpu.sr.set(StatusRegister::V, false);")
    return 158 + c


def bcd(cur, e, op, kind):
    rm = bool(op & 8)
    dy, dx = op & 7, (op >> 9) & 7
    if rm:
        e("cpu.a[%d] = cpu.a[%d].wrapping_sub(%d);" % (dy, dy, 2 if dy == 7 else 1))
        e("let s = bus.read8(cpu.a[%d]);" % dy)
        e("cpu.a[%d] = cpu.a[%d].wrapping_sub(%d);" % (dx, dx, 2 if dx == 7 else 1))
        e("let d = bus.read8(cpu.a[%d]);" % dx)
    else:
        e("let s = cpu.d[%d] as u8; let d = cpu.d[%d] as u8;" % (dy, dx))
    e("let x = cpu.sr.get(StatusRegister::X) as u8;")
    e("let (r, carry, vf) = %s_calc(d, s, x);" % kind)
    e("if r != 0 { cpu.sr.set(StatusRegister::Z, false); }")
    e("cpu.sr.set(StatusRegister::N, r & 0x80 != 0); cpu.sr.set(StatusRegister::V, vf);")
    e("cpu.sr.set(StatusRegister::C, carry); cpu.sr.set(StatusRegister::X, carry);")
    if rm:
        e("bus.write8(cpu.a[%d], r);" % dx)
        return 18
    e("cpu.d[%d] = (cpu.d[%d] & !0xFF) | r as u32;" % (dx, dx))
    return 6


def shift(cur, e, op):
    sb = (op >> 6) & 3
    if sb == 3:  # memoria, 1 bit
        kind = (op >> 9) & 3
        left = bool(op & 0x100)
        ea = EA.decode(cur, op & 0x3F, "w")

        def m(e, v):
            t = e.tmp("r")
            e("let %s = do_shift_rotate(cpu, %s, 1, Size::Word, %d, %s);" % (t, v, kind, "true" if left else "false"))
            return t
        ea_modify(e, ea, "w", m)
        return 8 + ea.cycles("w")
    size = size2(sb)
    kind = (op >> 3) & 3
    left = bool(op & 0x100)
    reg = op & 7
    cf = (op >> 9) & 7
    lr = "true" if left else "false"
    if op & 0x20:
        e("let cnt = cpu.d[%d] & 0x3F;" % cf)
        e("let r = do_shift_rotate(cpu, cpu.d[%d] & %s, cnt, %s, %d, %s);" % (reg, hx(M[size]), RS[size], kind, lr))
        ea_write(e, EA("dn", reg), size, "r")
        e("cpu.pc = cpu.pc; ")
        return "6 + 2 * cnt"
    cnt = cf or 8
    e("let r = do_shift_rotate(cpu, cpu.d[%d] & %s, %d, %s, %d, %s);" % (reg, hx(M[size]), cnt, RS[size], kind, lr))
    ea_write(e, EA("dn", reg), size, "r")
    return 6 + 2 * cnt
