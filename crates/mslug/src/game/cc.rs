//! Helpers con la semántica exacta del 68000 (flags y ciclos), para que las
//! funciones portadas se escriban en Rust idiomático sin perder la
//! equivalencia 1:1 con la ROM.
//!
//! Ciclos: mismo modelo que el núcleo 68000 de `neogeo-hw` (base de la
//! instrucción + coste del modo de direccionamiento):
//!   Dn/An/#imm 0 · (An) (An)+ 4/8 · -(An) 6/10 · d16(An) abs.w d16(PC) 8/12
//!   · d8(An,Xn) 10/14 · abs.l 12/16  (byte·word / long)

use neogeo_hw::cpu::m68k::{Bus, Cpu, Size, StatusRegister as SR};

pub type B<'a, 'b> = &'a mut &'b mut dyn Bus;

// ---- costes de direccionamiento (byte/word, long) ------------------------
pub const EA_DN: (u32, u32) = (0, 0);
pub const EA_IND: (u32, u32) = (4, 8);
pub const EA_PREDEC: (u32, u32) = (6, 10);
pub const EA_D16: (u32, u32) = (8, 12);
pub const EA_IDX: (u32, u32) = (10, 14);
pub const EA_ABSL: (u32, u32) = (12, 16);

#[inline]
pub fn ea(c: (u32, u32), s: Size) -> u32 {
    if s == Size::Long { c.1 } else { c.0 }
}

/// Ciclos de `rts` (pop de 4 B + salto).
pub const RTS: u32 = 16;

/// Simula `rts`: saca la dirección de retorno de la pila y salta.
#[inline]
pub fn rts(cpu: &mut Cpu, bus: B) -> u32 {
    let ret = cpu.pop32(bus);
    cpu.jump_to(ret);
    RTS
}

// ---- flags ---------------------------------------------------------------
#[inline]
pub fn mask(s: Size) -> u32 { s.mask() }

/// N,Z según el resultado; V=C=0; X intacto (move, and, or, eor, tst, clr...).
#[inline]
pub fn logic(cpu: &mut Cpu, v: u32, s: Size) {
    cpu.sr.set_nz(v, s);
    cpu.sr.set(SR::V, false);
    cpu.sr.set(SR::C, false);
}

/// `add` con flags (X incluido).
pub fn add(cpu: &mut Cpu, a: u32, b: u32, s: Size) -> u32 {
    let (am, bm) = (a & s.mask(), b & s.mask());
    let full = am as u64 + bm as u64;
    let r = (full as u32) & s.mask();
    let carry = full > s.mask() as u64;
    let ov = ((!(am ^ bm)) & (am ^ r) & s.sign_bit()) != 0;
    cpu.sr.set_nz(r, s);
    cpu.sr.set(SR::V, ov);
    cpu.sr.set(SR::C, carry);
    cpu.sr.set(SR::X, carry);
    r
}

/// `sub` (a - b) con flags; `x` = actualizar X (sub sí, cmp no).
pub fn sub(cpu: &mut Cpu, a: u32, b: u32, s: Size, x: bool) -> u32 {
    let (am, bm) = (a & s.mask(), b & s.mask());
    let r = am.wrapping_sub(bm) & s.mask();
    let borrow = bm > am;
    let ov = ((am ^ bm) & (am ^ r) & s.sign_bit()) != 0;
    cpu.sr.set_nz(r, s);
    cpu.sr.set(SR::V, ov);
    cpu.sr.set(SR::C, borrow);
    if x {
        cpu.sr.set(SR::X, borrow);
    }
    r
}

/// `neg` con flags.
pub fn neg(cpu: &mut Cpu, v: u32, s: Size) -> u32 {
    let vm = v & s.mask();
    let r = 0u32.wrapping_sub(vm) & s.mask();
    cpu.sr.set_nz(r, s);
    cpu.sr.set(SR::V, vm == s.sign_bit());
    cpu.sr.set(SR::C, vm != 0);
    cpu.sr.set(SR::X, vm != 0);
    r
}

/// Escribe la parte baja de un registro de datos (`.b` / `.w` / `.l`).
#[inline]
pub fn set_d(cpu: &mut Cpu, n: usize, v: u32, s: Size) {
    let m = s.mask();
    cpu.d[n] = (cpu.d[n] & !m) | (v & m);
}

// ---- memoria (el bus del 68000 es big-endian) -----------------------------
#[inline] pub fn r8(bus: B, a: u32) -> u32 { bus.read8(a & 0xFF_FFFF) as u32 }
#[inline] pub fn r16(bus: B, a: u32) -> u32 { bus.read16(a & 0xFF_FFFF) as u32 }
#[inline] pub fn r32(bus: B, a: u32) -> u32 { bus.read32(a & 0xFF_FFFF) }
#[inline] pub fn w8(bus: B, a: u32, v: u32) { bus.write8(a & 0xFF_FFFF, v as u8) }
#[inline] pub fn w16(bus: B, a: u32, v: u32) { bus.write16(a & 0xFF_FFFF, v as u16) }
#[inline] pub fn w32(bus: B, a: u32, v: u32) { bus.write32(a & 0xFF_FFFF, v) }

#[inline]
pub fn rd(bus: B, a: u32, s: Size) -> u32 {
    match s { Size::Byte => r8(bus, a), Size::Word => r16(bus, a), Size::Long => r32(bus, a) }
}
#[inline]
pub fn wr(bus: B, a: u32, v: u32, s: Size) {
    match s { Size::Byte => w8(bus, a, v), Size::Word => w16(bus, a, v), Size::Long => w32(bus, a, v) }
}

/// Desplazamiento con signo de 16 bits (`d16(An)`).
#[inline]
pub fn d16(base: u32, d: i16) -> u32 { base.wrapping_add(d as i32 as u32) }
/// Índice `.w` con signo (`(An,Dn.w)`).
#[inline]
pub fn idx_w(v: u32) -> u32 { (v as i16) as i32 as u32 }

// ---- desplazamientos / rotaciones (registro, cuenta inmediata 1..8) -------
/// Ciclos de shift/rotate de registro: 6 + 2·cuenta.
#[inline]
pub fn shift_cycles(count: u32) -> u32 { 6 + 2 * count }

/// `lsl` (count ≥ 1): C=X=último bit que sale, V=0.
pub fn lsl(cpu: &mut Cpu, v: u32, count: u32, s: Size) -> u32 {
    let mut v = v & s.mask();
    let mut c = false;
    for _ in 0..count {
        c = v & s.sign_bit() != 0;
        v = (v << 1) & s.mask();
    }
    cpu.sr.set_nz(v, s);
    cpu.sr.set(SR::V, false);
    cpu.sr.set(SR::C, c);
    cpu.sr.set(SR::X, c);
    v
}

/// `asr` (count ≥ 1): C=X=último bit que sale, V=0.
pub fn asr(cpu: &mut Cpu, v: u32, count: u32, s: Size) -> u32 {
    let mut v = v & s.mask();
    let mut c = false;
    for _ in 0..count {
        c = v & 1 != 0;
        v = (v >> 1) | (v & s.sign_bit());
    }
    cpu.sr.set_nz(v, s);
    cpu.sr.set(SR::V, false);
    cpu.sr.set(SR::C, c);
    cpu.sr.set(SR::X, c);
    v
}

/// `rol` (count ≥ 1): C=último bit que sale, V=0, X intacto.
pub fn rol(cpu: &mut Cpu, v: u32, count: u32, s: Size) -> u32 {
    let mut v = v & s.mask();
    let mut c = false;
    for _ in 0..count {
        c = v & s.sign_bit() != 0;
        v = ((v << 1) | c as u32) & s.mask();
    }
    cpu.sr.set_nz(v, s);
    cpu.sr.set(SR::V, false);
    cpu.sr.set(SR::C, c);
    v
}
