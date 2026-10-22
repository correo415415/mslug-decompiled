//! Plataformas móviles: las entidades sobre las que se puede estar (cajas,
//! vagonetas...) apuntan cada frame su posición en un anillo de 16 muestras;
//! quien está encima la busca por su identificador para moverse con ella.
//!
//! El byte `ID` de cada entidad lleva dos identificadores: el nibble alto
//! es el suyo como plataforma y el bajo el de la plataforma sobre la que
//! está (`NONE` = ninguno).
//!
//! Original: `Entity_TrailRecord_099812`, `Trail_FindNearest_09993c`,
//! `Clipping_Test_0999DE`, `Handler_ApplyCameraSelf_0440E4`, `$02783A`.

use super::anim;
use crate::layout::{platforms as L, task as T};
use crate::mem::{Machine, Memory};

/// Muestras desde la más antigua a la más reciente.
fn samples(m: &mut impl Memory) -> impl Iterator<Item = u32> {
    let wrap = m.read_u16(L::WRAP);
    let mut i = m.read_u16(L::TAIL);
    std::iter::from_fn(move || {
        if i == wrap {
            return None;
        }
        let at = L::RING + u32::from(i);
        i += L::SAMPLE_LEN;
        if i >= L::RING_LEN {
            i = 0;
        }
        Some(at)
    })
}

fn next_head(m: &mut impl Memory) -> u32 {
    let h = m.read_u16(L::HEAD);
    let n = if h + L::SAMPLE_LEN >= L::RING_LEN { 0 } else { h + L::SAMPLE_LEN };
    m.write_u16(L::HEAD, n);
    L::RING + u32::from(h)
}

/// Apunta la posición (`x`, `y`) y el ancho `w` de la plataforma `t`
/// (le da identificador si no tiene) con lo que se ha movido desde su
/// muestra anterior. Original: `Entity_TrailRecord_099812`.
pub fn record(m: &mut impl Memory, t: u32, x: u16, y: u16, w: u16) {
    let mut id = m.read_u8(t + L::ID) >> 4;
    let (mut dx, mut dy) = (0u16, 0u16);
    if id == L::NONE {
        id = m.read_u8(L::NEXT_ID) + 1;
        if id >= L::IDS {
            id = 0;
        }
        m.write_u8(L::NEXT_ID, id);
        let low = m.read_u8(t + L::ID) & 0x0F;
        m.write_u8(t + L::ID, (id << 4) | low);
    } else {
        let list: Vec<u32> = samples(m).collect();
        if let Some(&s) = list.iter().find(|&&s| m.read_u8(s) == id) {
            dx = x.wrapping_sub(m.read_u16(s + L::S_X));
            dy = y.wrapping_sub(m.read_u16(s + L::S_Y));
        }
    }
    let s = next_head(m);
    m.write_u8(s, id);
    for (off, v) in [(L::S_X, x), (L::S_Y, y), (L::S_W, w), (L::S_DX, dx), (L::S_DY, dy)] {
        m.write_u16(s + off, v);
    }
}

/// Plataforma bajo el punto (`x`, `y`) de la entidad `t` (sin contar la
/// suya): devuelve cuánto queda por debajo de su borde superior y su
/// identificador. Si no hay ninguna y estaba «saliendo» de una, la suelta.
/// Original: `Trail_FindNearest_09993c` (C, d5, d6).
pub fn find_under(m: &mut impl Memory, t: u32, x: u16, y: u16) -> Option<(u16, u8)> {
    let own = m.read_u8(t + L::ID) >> 4;
    let list: Vec<u32> = samples(m).collect();
    for s in list {
        let top = m.read_u16(s + L::S_Y);
        if (y as i16) > (top as i16) || (y as i16) <= (top.wrapping_sub(L::DEPTH) as i16) {
            continue;
        }
        if x.wrapping_sub(m.read_u16(s + L::S_X)) >= m.read_u16(s + L::S_W) {
            continue;
        }
        let id = m.read_u8(s);
        if id == own {
            continue;
        }
        return Some((top.wrapping_sub(y), id));
    }
    let v = m.read_u8(t + L::ID);
    if v & 0x0F == L::LEAVING {
        m.write_u8(t + L::ID, v | L::NONE);
    }
    None
}

/// ¿Pisa (`x`, `y`) el borde superior de alguna plataforma (sin contar la
/// suya)? Busca de la muestra más reciente a la más antigua; si la
/// encuentra, se sube a ella. Original: `Trail_FindByKeyRange_0998ca` (C).
pub fn step_onto(m: &mut impl Memory, t: u32, x: u16, y: u16) -> bool {
    let own = m.read_u8(t + L::ID) >> 4;
    let tail = m.read_u16(L::TAIL);
    let mut i = m.read_u16(L::WRAP);
    while i != tail {
        i = if i < L::SAMPLE_LEN { i + L::RING_LEN - L::SAMPLE_LEN } else { i - L::SAMPLE_LEN };
        let s = L::RING + u32::from(i);
        if m.read_u16(s + L::S_Y) != y || x.wrapping_sub(m.read_u16(s + L::S_X)) >= m.read_u16(s + L::S_W) {
            continue;
        }
        let id = m.read_u8(s);
        if id == own {
            continue;
        }
        let v = m.read_u8(t + L::ID);
        m.write_u8(t + L::ID, (v & 0xF0) | (id & 0x0F));
        return true;
    }
    false
}

/// Lo que se ha movido en este frame la plataforma sobre la que está `t`;
/// si ya no existe, la suelta. Original: `Clipping_Test_0999DE` (C, d2, d3).
pub fn follow(m: &mut impl Memory, t: u32) -> Option<(u16, u16)> {
    let on = m.read_u8(t + L::ID) & 0x0F;
    if on == L::NONE {
        return None;
    }
    let list: Vec<u32> = samples(m).collect();
    if let Some(&s) = list.iter().find(|&&s| m.read_u8(s) == on) {
        return Some((m.read_u16(s + L::S_DX), m.read_u16(s + L::S_DY)));
    }
    let v = m.read_u8(t + L::ID);
    m.write_u8(t + L::ID, v | L::NONE);
    None
}

/// Desplazamiento de un frame para el punto (`x0`, `y0`) de la entidad
/// `t`: el de la plataforma sobre la que está o, si no está en ninguna, el
/// de la capa móvil de la escena. Devuelve `None` si ya se movió este frame
/// (y marca que se ha movido).
fn shift(m: &mut impl Memory, t: u32, x0: u16, y0: u16) -> Option<(u16, u16)> {
    let f = m.read_u8(t + T::FLAGS2);
    if f & L::MOVED != 0 {
        return None;
    }
    let (mut x, mut y) = (x0, y0);
    m.write_u8(t + T::FLAGS2, f | L::MOVED);
    match follow(m, t) {
        None => {
            x = x.wrapping_sub(m.read_u16(L::LAYER_POS[0]));
            y = y.wrapping_add(m.read_u16(L::LAYER_POS[1]));
        }
        Some((dx, dy)) => {
            let free = f & L::RIDE_ALWAYS != 0 || m.read_u8(t + L::STATE) & L::WALL_BITS == 0;
            // Contra una pared sólo se le arrastra hacia el lado libre (se
            // mira la X de la entidad, no la del punto).
            let ex = m.read_u16(t + L::X);
            let push = free || ((ex as i16) > L::EDGE) != ((dx as i16) >= 0);
            if push {
                x = x.wrapping_add(dx);
            }
            y = y.wrapping_add(dy);
        }
    }
    if m.read_u8(L::SCENE) == L::LOCKED_X_SCENE && m.read_u16(L::LOCK_X) != 0 {
        x = x0;
    }
    Some((x, y))
}

/// Una vez por frame: mueve la entidad con la plataforma sobre la que está
/// o, si no está en ninguna, con la capa móvil de la escena.
/// Original: `Handler_ApplyCameraSelf_0440E4`.
pub fn apply_camera(m: &mut impl Memory, t: u32) {
    let (x0, y0) = (m.read_u16(t + L::X), m.read_u16(t + L::Y));
    if let Some((x, y)) = shift(m, t, x0, y0) {
        m.write_u16(t + L::X, x);
        m.write_u16(t + L::Y, y);
    }
}

/// Como [`apply_camera`] pero sobre la posición de trabajo de la colisión
/// (`WORK_POS`). Original: `Handler_ApplyCameraGlobals_044182`.
pub fn apply_camera_work(m: &mut impl Memory, t: u32) {
    let (x0, y0) = (m.read_u16(L::WORK_POS[0]), m.read_u16(L::WORK_POS[1]));
    if let Some((x, y)) = shift(m, t, x0, y0) {
        m.write_u16(L::WORK_POS[0], x);
        m.write_u16(L::WORK_POS[1], y);
    }
}

/// Movimiento por frame de una entidad: plataforma / capa y profundidad.
/// Original: `$02783A`.
pub fn move_with_scene(m: &mut impl Memory, t: u32) {
    apply_camera(m, t);
    anim::apply_depth(m, t);
}

/// Física de un objeto suelto: se mueve con la escena y, si no está
/// sujeto (`HELD` = 0 o `0xFF`) y no pisa suelo, cae.
/// Original: `Item_PhysAndProbe_09b8c0` (siempre C = 0).
pub fn item_physics(m: &mut impl Machine, t: u32) {
    move_with_scene(m, t);
    // Las rutinas de colisión del original usan `a5` como base de su zona
    // de trabajo (lo deja puesto `$02783A`).
    let mut r = m.regs();
    r.a[5] = L::SCRATCH_BASE;
    m.set_regs(r);
    let held = m.read_u8(t + L::HELD);
    if held != 0 && held != 0xFF {
        return;
    }
    if super::collision::probe_floor_here(m) {
        super::collision::fall(m, t);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::TestMachine;

    #[test]
    fn record_then_follow() {
        let mut m = TestMachine::default();
        let (p, rider) = (0x10_0A80, 0x10_0B20);
        m.write_u8(p + L::ID, 0xFF);
        record(&mut m, p, 100, 50, 32);
        // Las muestras válidas llegan hasta la cabeza.
        let h = m.read_u16(L::HEAD);
        m.write_u16(L::WRAP, h);
        record(&mut m, p, 104, 50, 32);
        let h = m.read_u16(L::HEAD);
        m.write_u16(L::WRAP, h);
        let id = m.read_u8(p + L::ID) >> 4;
        m.write_u8(rider + L::ID, 0xF0 | id);
        assert_eq!(follow(&mut m, rider), Some((0, 0)));
        assert_eq!(find_under(&mut m, rider, 110, 45), Some((5, id)));
    }
}
