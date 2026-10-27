//! Contacto entre entidades por sus cajas: el de una entidad con los tres
//! huecos de jugador (los dos soldados y el Slug).
//!
//! Original: `Player_Dispatch3Slots_028998`, `Entity_HitboxCollide_028A96`
//! y `Hitbox_OverlapTestXY_028B14`.

use crate::layout::contact as L;
use crate::mem::Memory;

/// Extremos `(min, max)` de la caja que empieza en `b`, espejados si la
/// entidad `e` tiene puesto `mirror` en sus banderas.
fn span(m: &mut impl Memory, e: u32, b: u32, mirror: u8) -> (i16, i16) {
    let (lo, hi) = (m.read_u16(b) as i16, m.read_u16(b + 2) as i16);
    if m.read_u8(e + L::FLAGS_3A) & mirror != 0 {
        (hi.wrapping_neg(), lo.wrapping_neg())
    } else {
        (lo, hi)
    }
}

/// ¿Se solapan los intervalos (ya en coordenadas del mundo)?
fn overlaps(a: (i16, i16), b: (i16, i16)) -> bool {
    b.0 <= a.1 && a.0 <= b.1
}

/// ¿Se tocan las cajas `ab` (de `t`) y `vb` (de `v`)? Las cajas son
/// {X mín., X máx., Y mín., Y máx.} relativas a la entidad. Devuelve además
/// si `t` queda a la izquierda de `v` (comparando la X más el desplazamiento
/// de las cabeceras). Original: `Hitbox_OverlapTestXY_028B14`.
fn overlap(m: &mut impl Memory, t: u32, ab: u32, v: u32, vb: u32) -> (bool, bool) {
    let (tx, ty) = (m.read_u16(t + L::X) as i16, m.read_u16(t + L::Y) as i16);
    let (vx, vy) = (m.read_u16(v + L::X) as i16, m.read_u16(v + L::Y) as i16);
    let a = span(m, t, ab + L::BOX_X, L::MIRROR_X);
    let b = span(m, v, vb + L::BOX_X, L::MIRROR_X);
    let a = (a.0.wrapping_add(tx), a.1.wrapping_add(tx));
    let b = (b.0.wrapping_add(vx), b.1.wrapping_add(vx));
    if !overlaps(a, b) {
        return (false, false);
    }
    let vc = vx.wrapping_add(m.read_u16(vb - L::HEADER + L::HEADER_X) as i16);
    let tc = tx.wrapping_add(m.read_u16(ab - L::HEADER + L::HEADER_X) as i16);
    let left = tc < vc;
    let a = span(m, t, ab + L::BOX_Y, L::MIRROR_Y);
    let b = span(m, v, vb + L::BOX_Y, L::MIRROR_Y);
    let a = (a.0.wrapping_add(ty), a.1.wrapping_add(ty));
    let b = (b.0.wrapping_add(vy), b.1.wrapping_add(vy));
    (overlaps(a, b), left)
}

/// Prueba la caja `desc` de `t` contra la caja de `v` y, si se tocan,
/// marca en las banderas de contacto de cada uno por qué lado (y en `v`
/// si el contacto es del tipo «fuerte»). Original: `Entity_HitboxCollide_028A96`.
pub fn collide(m: &mut impl Memory, t: u32, desc: u32, v: u32) -> bool {
    if v == t {
        return false;
    }
    let vdesc = m.read_u32(v + L::HITBOX);
    if vdesc == L::NIL || m.read_u8(vdesc) != L::ACTIVE || m.read_u8(v + L::CONTACT) & L::IMMUNE != 0 {
        return false;
    }
    let (hit, left) = overlap(m, t, desc + L::HEADER, v, vdesc + L::HEADER);
    if !hit {
        return false;
    }
    // El original deriva los bits de `slt`/`sge` + 1: 0 o 1.
    let (tb, vb) = if left { (0, 1) } else { (1, 0) };
    if m.read_u8(v + L::FLAGS_6B) & L::NO_MARK_ATTACKER == 0 {
        let c = m.read_u8(t + L::CONTACT);
        m.write_u8(t + L::CONTACT, c | (1 << tb));
    }
    let mut c = m.read_u8(v + L::CONTACT) | (1 << vb);
    c &= !L::STRONG;
    if m.read_u8(desc + 1) == L::KIND_STRONG {
        c |= L::STRONG;
    }
    m.write_u8(v + L::CONTACT, c);
    true
}

/// Prueba la caja activa de `t` contra el Slug y los dos jugadores.
/// Devuelve si tocó a algún jugador. Original: `Player_Dispatch3Slots_028998`.
pub fn touch_players(m: &mut impl Memory, t: u32) -> bool {
    if m.read_u16(t + L::HITBOX) == 0xFFFF {
        return false;
    }
    let desc = m.read_u32(t + L::HITBOX);
    if m.read_u8(desc) != L::ACTIVE {
        return false;
    }
    collide(m, t, desc, L::SLOTS[2]);
    if collide(m, t, desc, L::SLOTS[0]) {
        collide(m, t, desc, L::SLOTS[1]);
        return true;
    }
    collide(m, t, desc, L::SLOTS[1])
}
