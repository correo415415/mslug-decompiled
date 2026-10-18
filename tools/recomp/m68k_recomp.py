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
        return "6 + 2 * cnt"
    cnt = cf or 8
    e("let r = do_shift_rotate(cpu, cpu.d[%d] & %s, %d, %s, %d, %s);" % (reg, hx(M[size]), cnt, RS[size], kind, lr))
    ea_write(e, EA("dn", reg), size, "r")
    return 6 + 2 * cnt


def line0(cur, e, op):
    ccr_ops = {0x003C: "|", 0x023C: "&", 0x0A3C: "^"}
    sr_ops = {0x007C: "|", 0x027C: "&", 0x0A7C: "^"}
    if op in ccr_ops:
        v = cur.fetch16() & 0x1F
        o = ccr_ops[op]
        e("cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) %s %s);" % (o, hx(v)))
        return 20
    if op in sr_ops:
        v = cur.fetch16()
        priv_check(e)
        e("write_sr(cpu, cpu.sr.0 %s %s);" % (sr_ops[op], hx(v)))
        return 20
    if (op & 0xF138) == 0x0108:  # MOVEP
        dn, an = (op >> 9) & 7, op & 7
        opm = (op >> 6) & 7
        lg, to_mem = bool(opm & 1), bool(opm & 2)
        disp = u32(sx16(cur.fetch16()))
        e("let a = cpu.a[%d].wrapping_add(%s);" % (an, hx(disp)))
        if to_mem:
            if lg:
                e("let v = cpu.d[%d]; bus.write8(a, (v >> 24) as u8); bus.write8(a.wrapping_add(2), (v >> 16) as u8); bus.write8(a.wrapping_add(4), (v >> 8) as u8); bus.write8(a.wrapping_add(6), v as u8);" % dn)
                return 24
            e("let v = cpu.d[%d]; bus.write8(a, (v >> 8) as u8); bus.write8(a.wrapping_add(2), v as u8);" % dn)
            return 16
        if lg:
            e("let b3 = bus.read8(a) as u32; let b2 = bus.read8(a.wrapping_add(2)) as u32; let b1 = bus.read8(a.wrapping_add(4)) as u32; let b0 = bus.read8(a.wrapping_add(6)) as u32;")
            e("cpu.d[%d] = (b3 << 24) | (b2 << 16) | (b1 << 8) | b0;" % dn)
            return 24
        e("let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;")
        e("cpu.d[%d] = (cpu.d[%d] & 0xFFFF_0000) | (b1 << 8) | b0;" % (dn, dn))
        return 16
    if (op & 0xFF00) == 0x0800 or (op & 0xF100) == 0x0100:
        if (op & 0xFF00) == 0x0800:
            bit = str(cur.fetch16() & 0xFF)
        else:
            bit = "cpu.d[%d]" % ((op >> 9) & 7)
        ea = EA.decode(cur, op & 0x3F, "b")
        kind = (op >> 6) & 3
        size = "l" if ea.is_reg() else "b"
        mod = 32 if size == "l" else 8
        e("let mk: u32 = 1u32 << ((%s) %% %d);" % (bit, mod))
        if kind == 0:
            v = ea_read(e, ea, size)
            e("cpu.sr.set(StatusRegister::Z, %s & mk == 0);" % v)
        else:
            o = {1: "%s ^ mk", 2: "%s & !mk", 3: "%s | mk"}[kind]

            def m(e, v):
                t = e.tmp("r")
                e("cpu.sr.set(StatusRegister::Z, %s & mk == 0);" % v)
                e("let %s = %s;" % (t, o % v))
                return t
            ea_modify(e, ea, size, m)
        return 8 + ea.cycles(size)
    size = size2((op >> 6) & 3)
    if size is None:
        exc(e, "IllegalInstruction", 34)
        return "dead"
    if size == "l":
        imm = cur.fetch32()
    elif size == "w":
        imm = cur.fetch16()
    else:
        imm = cur.fetch16() & 0xFF
    ea = EA.decode(cur, op & 0x3F, size)
    up = (op >> 8) & 0xF
    I = hx(imm)
    if up == 0xC:
        v = ea_read(e, ea, size)
        e("sub_with_flags(cpu, %s, %s, %s, false);" % (v, I, RS[size]))
        return 8 + ea.cycles(size)
    if up not in (0, 2, 4, 6, 0xA):
        exc(e, "IllegalInstruction", 34)
        return "dead"

    def m(e, v):
        t = e.tmp("r")
        if up in (0, 2, 0xA):
            o = {0: "|", 2: "&", 0xA: "^"}[up]
            e("let %s = (%s %s %s) & %s;" % (t, v, o, I, hx(M[size])))
            logic(e, t, size)
        else:
            fn = "sub_with_flags" if up == 4 else "add_with_flags"
            e("let %s = %s(cpu, %s, %s, %s, true);" % (t, fn, v, I, RS[size]))
        return t
    # rmw_with_au: en registro = read/modify/write
    ea_modify(e, ea, size, m)
    return 8 + ea.cycles(size)


def jump_ea_addr(e, ea):
    """Dirección de control (jmp/jsr/lea/pea). None si el modo no vale."""
    if ea.kind in ("dn", "an", "post", "pre", "imm"):
        return None
    return ea.addr(e, "l")


def line4(cur, I, op, e):
    if op == 0x4E71:
        return 4
    if op == 0x4E75:  # RTS
        e("let r = cpu.pop32(bus); cpu.jump_to(r);")
        I.falls = False
        I.dynamic = True
        return 16
    if op == 0x4E73:  # RTE
        priv_check(e)
        e("let s = cpu.pop16(bus); let r = cpu.pop32(bus); write_sr(cpu, s); cpu.jump_to(r);")
        I.falls = False
        I.dynamic = True
        return 20
    if op == 0x4E77:  # RTR
        e("let s = cpu.pop16(bus); let r = cpu.pop32(bus); cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | (s & 0x1F); cpu.jump_to(r);")
        I.falls = False
        I.dynamic = True
        return 20
    if op == 0x4E70:
        priv_check(e)
        return 132
    if op == 0x4E72:  # STOP
        v = cur.fetch16()
        e("if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }")
        e("write_sr(cpu, %s); cpu.pc = cpu.instr_pc; cpu.stopped = true;" % hx(v))
        I.falls = False
        I.dynamic = True
        return 4
    if op == 0x4E76:
        e("if cpu.sr.get(StatusRegister::V) { cpu.enter_exception(bus, Exception::TrapV); return 34; }")
        return 4
    if (op & 0xFFF0) == 0x4E40:  # TRAP
        n = op & 0xF
        e("let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }")
        e("let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);")
        e("cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(%s);" % hx(0x80 + 4 * n))
        I.falls = False
        I.dynamic = True
        return 34
    if (op & 0xFFF8) == 0x4E50:  # LINK
        n = op & 7
        d = u32(sx16(cur.fetch16()))
        e("cpu.push32(bus, cpu.a[%d]); cpu.a[%d] = cpu.a[7]; cpu.a[7] = cpu.a[7].wrapping_add(%s);" % (n, n, hx(d)))
        return 16
    if (op & 0xFFF8) == 0x4E58:  # UNLK
        n = op & 7
        e("cpu.a[7] = cpu.a[%d]; let v = cpu.pop32(bus); cpu.a[%d] = v;" % (n, n))
        return 12
    if (op & 0xFFF0) == 0x4E60:  # MOVE USP
        priv_check(e)
        n = op & 7
        if op & 8:
            e("cpu.a[%d] = cpu.usp;" % n)
        else:
            e("cpu.usp = cpu.a[%d];" % n)
        return 4
    if (op & 0xFFF8) == 0x4840:  # SWAP
        n = op & 7
        e("cpu.d[%d] = cpu.d[%d].rotate_left(16);" % (n, n))
        logic(e, "cpu.d[%d]" % n, "l")
        return 4
    if (op & 0xFFB8) == 0x4880:  # EXT
        n = op & 7
        if op & 0x40:
            e("cpu.d[%d] = cpu.d[%d] as i16 as i32 as u32;" % (n, n))
            logic(e, "cpu.d[%d]" % n, "l")
        else:
            e("let v = cpu.d[%d] as i8 as i16 as u16 as u32; cpu.d[%d] = (cpu.d[%d] & 0xFFFF_0000) | v;" % (n, n, n))
            logic(e, "v", "w")
        return 4
    if (op & 0xFFC0) == 0x4EC0 or (op & 0xFFC0) == 0x4E80:  # JMP / JSR
        is_jsr = (op & 0xFFC0) == 0x4E80
        ea = EA.decode(cur, op & 0x3F, "l")
        st = ea.static_addr()
        a = jump_ea_addr(e, ea)
        if a is None:
            exc(e, "IllegalInstruction", 34)
            I.falls = False
            return "dead"
        if is_jsr:
            e("cpu.push32(bus, %s);" % hx(cur.pc))
        if st is not None and not (st & 1):
            I.targets.append(st)
            e("cpu.pc = %s;" % hx(st))
        else:
            e("cpu.jump_to(%s);" % a)
            I.dynamic = True
        if not is_jsr:
            I.falls = False
        return 16 if is_jsr else 8
    if (op & 0xF1C0) == 0x41C0:  # LEA
        n = (op >> 9) & 7
        ea = EA.decode(cur, op & 0x3F, "l")
        a = jump_ea_addr(e, ea)
        if a is None:
            raise Unsupported("lea ea")
        e("cpu.a[%d] = %s;" % (n, a))
        return 4 + ea.cycles("l")
    if (op & 0xFFC0) == 0x4840:  # PEA
        ea = EA.decode(cur, op & 0x3F, "l")
        a = jump_ea_addr(e, ea)
        if a is None:
            raise Unsupported("pea ea")
        e("cpu.push32(bus, %s);" % a)
        return 12 + ea.cycles("l")
    if (op & 0xFF00) == 0x4A00:  # TST / TAS
        size = size2((op >> 6) & 3)
        if size:
            ea = EA.decode(cur, op & 0x3F, size)
            v = ea_read(e, ea, size)
            logic(e, v, size)
            return 4 + ea.cycles(size)
        if op == 0x4AFC:
            exc(e, "IllegalInstruction", 34)
            I.falls = False
            return "dead"
        ea = EA.decode(cur, op & 0x3F, "b")
        if ea.kind in ("an", "pcd", "pcidx", "imm"):
            exc(e, "IllegalInstruction", 34)
            I.falls = False
            return "dead"

        def m(e, v):
            logic(e, v, "b")
            t = e.tmp("r")
            e("let %s = %s | 0x80;" % (t, v))
            return t
        ea_modify(e, ea, "b", m)
        return 4 + ea.cycles("b")
    unary = {0x4200: "clr", 0x4600: "not", 0x4400: "neg", 0x4000: "negx"}
    if (op & 0xFF00) in unary and size2((op >> 6) & 3):
        size = size2((op >> 6) & 3)
        k = unary[op & 0xFF00]
        ea = EA.decode(cur, op & 0x3F, size)
        mk, sb = hx(M[size]), hx(SB[size])

        def m(e, v):
            t = e.tmp("r")
            if k == "clr":
                e("cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let %s: u32 = 0;" % t)
            elif k == "not":
                e("let %s = !%s & %s;" % (t, v, mk))
                logic(e, t, size)
            elif k == "neg":
                e("let %s = 0u32.wrapping_sub(%s) & %s;" % (t, v, mk))
                e("cpu.sr.set_nz(%s, %s); cpu.sr.set(StatusRegister::V, %s == %s); cpu.sr.set(StatusRegister::C, %s != 0); cpu.sr.set(StatusRegister::X, %s != 0);" % (t, RS[size], v, sb, v, v))
            else:
                e("let x = cpu.sr.get(StatusRegister::X) as u32; let %s = 0u32.wrapping_sub(%s).wrapping_sub(x) & %s;" % (t, v, mk))
                e("let bw = %s != 0 || x != 0; let vf = (%s & %s) != 0 && (%s & %s) != 0;" % (v, v, sb, t, sb))
                e("cpu.sr.set(StatusRegister::N, %s & %s != 0); if %s != 0 { cpu.sr.set(StatusRegister::Z, false); }" % (t, sb, t))
                e("cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, bw); cpu.sr.set(StatusRegister::X, bw);")
            return t
        ea_modify(e, ea, size, m)
        return 4 + ea.cycles(size)
    if (op & 0xFFC0) == 0x4800:  # NBCD
        ea = EA.decode(cur, op & 0x3F, "b")

        def m(e, v):
            t = e.tmp("r")
            e("let x = cpu.sr.get(StatusRegister::X) as u8; let (r, carry, vf) = sbcd_calc(0, %s as u8, x);" % v)
            e("if r != 0 { cpu.sr.set(StatusRegister::Z, false); } cpu.sr.set(StatusRegister::N, r & 0x80 != 0); cpu.sr.set(StatusRegister::V, vf); cpu.sr.set(StatusRegister::C, carry); cpu.sr.set(StatusRegister::X, carry);")
            e("let %s = r as u32;" % t)
            return t
        ea_modify(e, ea, "b", m)
        return 6 + ea.cycles("b")
    if (op & 0xFFC0) == 0x46C0:  # MOVE to SR
        ea = EA.decode(cur, op & 0x3F, "w")
        priv_check(e)
        v = ea_read(e, ea, "w")
        e("write_sr(cpu, %s as u16);" % v)
        return 12 + ea.cycles("w")
    if (op & 0xFFC0) == 0x40C0:  # MOVE from SR
        ea = EA.decode(cur, op & 0x3F, "w")
        ea_modify(e, ea, "w", lambda e, v: "(cpu.sr.0 as u32)")
        return 6 + ea.cycles("w")
    if (op & 0xFFC0) == 0x44C0:  # MOVE to CCR
        ea = EA.decode(cur, op & 0x3F, "w")
        v = ea_read(e, ea, "w")
        e("cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | (%s as u16 & 0x1F);" % v)
        return 12 + ea.cycles("w")
    if (op & 0xF1C0) == 0x4180:  # CHK
        n = (op >> 9) & 7
        ea = EA.decode(cur, op & 0x3F, "w")
        v = ea_read(e, ea, "w")
        e("let bd = %s as u16 as i16 as i32; let vv = cpu.d[%d] as u16 as i16 as i32;" % (v, n))
        e("cpu.sr.set(StatusRegister::Z, false); cpu.sr.set(StatusRegister::V, false); cpu.sr.set(StatusRegister::C, false);")
        e("if vv < 0 { cpu.sr.set(StatusRegister::N, true); cpu.enter_exception(bus, Exception::Chk); return 40; }")
        e("if vv > bd { cpu.sr.set(StatusRegister::N, false); cpu.enter_exception(bus, Exception::Chk); return 40; }")
        e("cpu.sr.set(StatusRegister::N, false);")
        return 10 + ea.cycles("w")
    if (op & 0xFB80) == 0x4880:  # MOVEM
        return movem(cur, e, op)
    exc(e, "IllegalInstruction", 34)
    I.falls = False
    return "dead"


def movem(cur, e, op):
    dr = bool(op & 0x400)
    size = "l" if op & 0x40 else "w"
    mask = cur.fetch16()
    mode, reg = (op >> 3) & 7, op & 7
    regs = [i for i in range(16) if (mask >> i) & 1]

    def R(i):
        return "cpu.d[%d]" % i if i < 8 else "cpu.a[%d]" % (i - 8)
    if not dr:
        if mode == 4:  # -(An): máscara invertida, An se confirma al final
            e("let mut a = cpu.a[%d];" % reg)
            for i in regs:
                r = 15 - i
                if size == "l":
                    e("let v = %s; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);" % R(r))
                else:
                    e("a = a.wrapping_sub(2); bus.write16(a, %s as u16);" % R(r))
            e("cpu.a[%d] = a;" % reg)
        else:
            ea = EA.decode(cur, op & 0x3F, size)
            if ea.kind in ("dn", "an", "post", "imm", "pcd", "pcidx"):
                raise Unsupported("movem r->m ea")
            a0 = ea.addr(e, size)
            e("let mut a = %s;" % a0)
            for i in regs:
                if size == "l":
                    e("bus.write32(a, %s); a = a.wrapping_add(4);" % R(i))
                else:
                    e("bus.write16(a, %s as u16); a = a.wrapping_add(2);" % R(i))
            e("let _ = a;")
    else:
        if mode == 3:
            e("let mut a = cpu.a[%d];" % reg)
        else:
            ea = EA.decode(cur, op & 0x3F, size)
            if ea.kind in ("dn", "an", "pre", "imm"):
                raise Unsupported("movem m->r ea")
            e("let mut a = %s;" % ea.addr(e, size))
        for i in regs:
            if size == "l":
                e("%s = bus.read32(a); a = a.wrapping_add(4);" % R(i))
            else:
                e("%s = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);" % R(i))
        if mode == 3:
            e("cpu.a[%d] = a;" % reg)
        else:
            e("let _ = a;")
    return 20


# ---------------------------------------------------------------------------
# Post-proceso: Rust no permite `f(cpu, cpu.d[0])` (préstamo mutable + lectura)
# -> se sacan a `let` los argumentos que leen `cpu.`
# ---------------------------------------------------------------------------
import re

HOIST_RE = re.compile(r"\b(add_with_flags|sub_with_flags|addx_with_flags|subx_with_flags|do_shift_rotate|write_sr)\(cpu, ")


def split_args(s, i):
    """s[i:] empieza tras '(' ; devuelve (lista_args, índice del ')')."""
    depth, cur, out = 0, "", []
    while i < len(s):
        c = s[i]
        if c in "([{":
            depth += 1
        elif c in ")]}":
            if depth == 0:
                out.append(cur)
                return out, i
            depth -= 1
        if c == "," and depth == 0:
            out.append(cur)
            cur = ""
        else:
            cur += c
        i += 1
    raise ValueError(s)


def hoist(lines):
    out = []
    n = 0
    for ln in lines:
        while True:
            m = None
            for mm in HOIST_RE.finditer(ln):
                args, end = split_args(ln, mm.end())
                if any("cpu." in a for a in args):
                    m = (mm, args, end)
                    break
            if not m:
                break
            mm, args, end = m
            new = []
            for a in args:
                if "cpu." in a:
                    n += 1
                    out.append("let h%d = %s;" % (n, a.strip()))
                    new.append("h%d" % n)
                else:
                    new.append(a.strip())
            ln = ln[:mm.end()] + ", ".join(new) + ln[end:]
        out.append(ln)
    return out


# ---------------------------------------------------------------------------
# Descubrimiento + generación
# ---------------------------------------------------------------------------
def load_syms():
    syms = {}
    p = os.path.join(HERE, "symbols.tsv")
    if os.path.exists(p):
        for ln in open(p):
            if ln.startswith("#"):
                continue
            a, sz, name, f = ln.rstrip("\n").split("\t")
            syms[int(a, 16)] = (name, int(sz), f)
    return syms


def seeds_from(mem, trace, extra):
    s = set()
    for i, b in enumerate(trace):
        if b:
            for k in range(8):
                if (b >> k) & 1:
                    s.add((i * 8 + k) * 2)
    # vectores de la BIOS y del cartucho
    for base, blob in ((0xC00000, mem.bios), (0, mem.prom)):
        for v in range(1, 64):
            a = struct.unpack_from(">I", blob, v * 4)[0] & 0xFFFFFF
            if not (a & 1) and mem.valid(a):
                s.add(a)
    s |= extra
    # Entradas de función de la decompilación (tools/registry.py): código
    # conocido aunque la traza no lo haya ejecutado. Sólo regiones de código
    # (< $9C608) y no los ficheros de datos.
    # (Desactivado por defecto: el registro también contiene islas de datos
    # dentro de ficheros de código; ver --registry-seeds.)
    if REGISTRY_SEEDS:
        for a, (name, sz, f) in load_syms().items():
            if a < 0x9C608 and not f.startswith("data/") and not (a & 1) \
                    and not re.search(r"Str|Tbl|Table|Data|Glyph|Ptr|Pal|Sprite|Anim|Script|Tpl|Zero|JT", name):
                s.add(a)
    return s


REGISTRY_SEEDS = False


def discover(mem, seeds):
    done = {}
    bad = {}
    work = sorted(seeds, reverse=True)
    while work:
        a = work.pop()
        if a in done or a in bad or (a & 1) or not mem.valid(a):
            continue
        try:
            I = translate(mem, a)
        except Unsupported as ex:
            bad[a] = str(ex)
            continue
        except struct.error:
            bad[a] = "fuera de rango"
            continue
        done[a] = I
        for t in I.targets:
            work.append(t)
        if I.falls and I.next is not None:
            work.append(I.next)
    return done, bad


def disasm_all(mem, addrs):
    try:
        from capstone import Cs, CS_ARCH_M68K, CS_MODE_M68K_000
    except ImportError:
        return {}
    md = Cs(CS_ARCH_M68K, CS_MODE_M68K_000)
    out = {}
    for a in addrs:
        try:
            raw = b"".join(struct.pack(">H", mem.r16(a + 2 * k)) for k in range(5))
        except Exception:
            continue
        ins = next(md.disasm(raw, a), None)
        if ins:
            out[a] = ("%s %s" % (ins.mnemonic, ins.op_str)).strip()
    return out


HEADER = """// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región {region}) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{{Bus, Condition, Cpu, Exception, Size, StatusRegister}};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;

"""


CHUNK = 1500   # instrucciones por módulo (rustc necesita poca RAM por módulo)


def regions(addrs):
    """Parte las direcciones en módulos de <= CHUNK instrucciones."""
    out = {}
    cur, n, k = None, 0, 0
    for a in sorted(addrs):
        base = "bios" if a >= 0xC00000 else "p%02x" % (a >> 16)
        if base != cur or n >= CHUNK:
            if base != cur:
                k = 0
            name = "%s_%d" % (base, k)
            k += 1
            cur, n = base, 0
            out[name] = []
        out[name].append(a)
        n += 1
    return out


CRATE_INS = 6000   # instrucciones por crate generado (cada rustc cabe en ~1 GB)
GEN_MARK_A = "# >>> GENERADO por tools/recomp/m68k_recomp.py"
GEN_MARK_B = "# <<< GENERADO"


def _replace_marked(path, body):
    """Sustituye (o añade al final) el bloque entre GEN_MARK_A y GEN_MARK_B."""
    s = open(path).read()
    blk = GEN_MARK_A + "\n" + body.rstrip("\n") + "\n" + GEN_MARK_B + "\n"
    if GEN_MARK_A in s:
        a = s.index(GEN_MARK_A)
        b = s.index(GEN_MARK_B, a) + len(GEN_MARK_B) + 1
        s = s[:a] + blk + s[b:]
    else:
        s = s.rstrip("\n") + "\n\n" + blk
    open(path, "w").write(s)


def write_region(path, reg, addrs, done, sym_starts, dis):
    lines = [HEADER.format(region=reg)]
    for a in addrs:
        I = done[a]
        if a in sym_starts:
            n, sz, f = sym_starts[a]
            lines.append("// ===== %s  ($%06X, %d B, %s)" % (n, a, sz, f))
        lines.append("/// $%06X: %s" % (a, dis.get(a, "?")))
        lines.append("pub fn i_%06x(cpu: &mut Cpu, bus: &mut B) -> u32 {" % a)
        for ln in hoist(I.body):
            lines.append("    " + ln)
        lines.append("}")
    lines.append("")
    lines.append("pub const TABLE: &[(u32, crate::InsFn)] = &[")
    for a in addrs:
        lines.append("    (0x%06X, i_%06x)," % (a, a))
    lines.append("];")
    open(path, "w").write("\n".join(lines) + "\n")


def generate(gen_root, done, syms, dis):
    """Emite el programa traducido como varios crates `mslug-gen-NN`
    (crates/gen/NN) de <= CRATE_INS instrucciones, para que rustc compile
    cada uno por separado sin quedarse sin memoria, y el pegamento en
    mslug-code (src/gen.rs + dependencias) y en el Cargo.toml raíz
    (miembros y perfil)."""
    import shutil
    by_region = regions(done)
    gdir = os.path.join(gen_root, "gen")
    if os.path.isdir(gdir):
        shutil.rmtree(gdir)
    os.makedirs(gdir)
    # agrupar regiones (en orden) en crates
    crates, cur, n = [], [], 0
    for reg in sorted(by_region):
        k = len(by_region[reg])
        if cur and n + k > CRATE_INS:
            crates.append(cur)
            cur, n = [], 0
        cur.append(reg)
        n += k
    if cur:
        crates.append(cur)
    names = []
    for ci, regs in enumerate(crates):
        name = "mslug-gen-%02d" % ci
        names.append(name)
        cd = os.path.join(gdir, "%02d" % ci)
        os.makedirs(os.path.join(cd, "src"))
        open(os.path.join(cd, "Cargo.toml"), "w").write(
            "# GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.\n"
            "[package]\nname = \"%s\"\nversion = \"0.1.0\"\nedition = \"2021\"\n"
            "license = \"MIT OR Apache-2.0\"\npublish = false\n"
            "description = \"Metal Slug: trozo %d del programa 68000 traducido a Rust\"\n\n"
            "[dependencies]\nneogeo-hw = { path = \"../../neogeo-hw\" }\n" % (name, ci))
        cnt = 0
        for reg in regs:
            write_region(os.path.join(cd, "src", reg + ".rs"), reg, by_region[reg], done, syms, dis)
            cnt += len(by_region[reg])
        m = ["// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.",
             "#![allow(clippy::all)]",
             "pub type InsFn = fn(&mut neogeo_hw::cpu::m68k::Cpu, &mut neogeo_hw::NeoGeoBus) -> u32;",
             ""]
        for r in regs:
            m.append("pub mod %s;" % r)
        m.append("\npub const TABLES: &[&[(u32, InsFn)]] = &[%s];" % ", ".join("%s::TABLE" % r for r in regs))
        m.append("pub const COUNT: usize = %d;" % cnt)
        open(os.path.join(cd, "src", "lib.rs"), "w").write("\n".join(m) + "\n")
    # pegamento en mslug-code
    code = os.path.join(gen_root, "mslug-code")
    old = os.path.join(code, "src", "gen")
    if os.path.isdir(old):
        shutil.rmtree(old)
    g = ["// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.",
         "pub const TABLES: &[&[&[(u32, crate::InsFn)]]] = &[%s];"
         % ", ".join("%s::TABLES" % x.replace("-", "_") for x in names),
         "pub const COUNT: usize = %d;" % len(done)]
    open(os.path.join(code, "src", "gen.rs"), "w").write("\n".join(g) + "\n")
    _replace_marked(os.path.join(code, "Cargo.toml"),
                    "\n".join('%s = { path = "../gen/%02d" }' % (x, i) for i, x in enumerate(names)))
    prof = []
    for x in names:
        prof.append('[profile.release.package.%s]\nopt-level = 1\ncodegen-units = 16\ndebug = false\nincremental = false\n' % x)
        prof.append('[profile.dev.package.%s]\nopt-level = 0\n' % x)
    _replace_marked(os.path.join(ROOT, "Cargo.toml"), "\n".join(prof))
    print("%d crates generados en %s" % (len(names), gdir))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--build", default=os.path.join(ROOT, "build", "recomp"))
    ap.add_argument("--out", default=os.path.join(ROOT, "crates"),
                    help="directorio crates/ (se reescriben crates/gen/* y el pegamento de mslug-code)")
    ap.add_argument("--report", action="store_true")
    ap.add_argument("--registry-seeds", action="store_true",
                    help="usar también las entradas de función de la decompilación como semillas")
    a = ap.parse_args()
    global REGISTRY_SEEDS
    REGISTRY_SEEDS = a.registry_seeds
    prom = open(os.path.join(a.build, "prom.bin"), "rb").read()
    bios = open(os.path.join(a.build, "bios.bin"), "rb").read()
    trace = open(os.path.join(a.build, "trace.bin"), "rb").read()
    extra = set()
    sp = os.path.join(HERE, "seeds.txt")
    if os.path.exists(sp):
        for ln in open(sp):
            ln = ln.split("#")[0].strip()
            if ln:
                extra.add(int(ln, 16))
    mem = Mem(prom, bios)
    seeds = seeds_from(mem, trace, extra)
    done, bad = discover(mem, seeds)
    traced = {s for s in seeds if s in done}
    bad_traced = {s: bad[s] for s in seeds if s in bad and s not in extra}
    print("semillas %d, instrucciones traducidas %d, no traducibles %d (de la traza: %d)"
          % (len(seeds), len(done), len(bad), len(bad_traced)))
    for s, why in sorted(bad_traced.items())[:40]:
        print("  !! $%06X %s" % (s, why))
    if a.report:
        return
    dis = disasm_all(mem, done.keys())
    generate(a.out, done, load_syms(), dis)
    print("-> %s" % a.out)


if __name__ == "__main__":
    main()
