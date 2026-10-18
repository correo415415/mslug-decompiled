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
