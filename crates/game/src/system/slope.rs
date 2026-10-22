//! Motor de colisión «B»: el movimiento sobre pendientes que usan los
//! vehículos (las ruedas del Slug). A diferencia del motor «D»
//! ([`motion::move_and_collide`]) aquí la entidad ya está apoyada: el paso
//! en Y sale de la pendiente del suelo y, si el tipo de tile cambia (otra
//! pendiente o una rampa por la que puede subir), se vuelve a sondear el
//! suelo para pegarla a la nueva superficie.
//!
//! Todo trabaja sobre la posición de trabajo de la colisión (`WORK_POS` y
//! sus fracciones).
//!
//! Original: `Entity_MoveAndCollide_B_027036`, `Entity_FloorProbe_026e0a`,
//! `Entity_WallProbeBoth_027020`, `CollMap_TestSolidBitC_027e7e`,
//! `CollMap_TestPlatformBitC_027e9c`, `Entity_StepB_UpdatePos_027902`
//! (`asm/entity/`).

use super::{anim, collision, motion, platforms};
use crate::layout::slope as L;
use crate::mem::Memory;

/// Resultado del sondeo del suelo: el punto ya pegado a la superficie, el
/// tipo de tile y su registro, la columna dentro de la celda, la altura de
/// esa columna (lo que deja el original en `d4`) y la tabla de pendiente.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Probe {
    pub x: u16,
    pub y: u16,
    pub kind: u8,
    pub record: u32,
    pub col: u16,
    pub height: u16,
    pub slope: u32,
}

/// ¿Bit (`col`, `row`) de la máscara sólida del tipo `record`?
/// Original: `CollMap_TestSolidBitC_027e7e` (C).
pub fn solid_bit(m: &mut impl Memory, record: u32, col: u16, row: u16) -> bool {
    mask_bit(m, record, 0, col, row)
}

/// ¿Bit (`col`, `row`) de la máscara de plataforma del tipo `record`?
/// Original: `CollMap_TestPlatformBitC_027e9c` (C).
pub fn platform_bit(m: &mut impl Memory, record: u32, col: u16, row: u16) -> bool {
    mask_bit(m, record, L::PLATFORM_MASK, col, row)
}

/// Byte de la fila `row` de la máscara (más `off`) y si tiene a 1 el bit de
/// la columna `col` (contando desde la izquierda).
pub fn mask_byte(m: &mut impl Memory, record: u32, off: u32, row: u16) -> u8 {
    let mask = m.read_u32(record);
    m.read_u8(mask.wrapping_add(off).wrapping_add(row as i16 as u32))
}

fn mask_bit(m: &mut impl Memory, record: u32, off: u32, col: u16, row: u16) -> bool {
    let b = u32::from(mask_byte(m, record, off, row));
    let bit = 7u16.wrapping_sub(col) as u32 & 31;
    b & (1 << bit) != 0
}

/// ¿Toca la pared (sólida o de plataforma) en (`col`, `row`)?
/// Original: `Entity_WallProbeBoth_027020` (C; quien llama no lo mira).
pub fn wall_probe(m: &mut impl Memory, record: u32, col: u16, row: u16) -> bool {
    platform_bit(m, record, col, row) || solid_bit(m, record, col, row)
}

/// ¿Mira la entidad hacia un lado y se mueve hacia el otro?
fn moving_against_facing(m: &mut impl Memory, t: u32) -> bool {
    let facing = m.read_u8(t + L::FACING) as i8;
    let vx = m.read_u16(t + L::VEL_X) as i16;
    facing != 0 && (facing < 0) != (vx < 0)
}

/// ¿Puede subir la entidad por la rampa / el escalón? Hace falta el
/// permiso de `+$13` (`ok`), que se esté moviendo y que, si no lo fuerza
/// `+$5B` (`force`), no vaya de espaldas.
fn may_climb(m: &mut impl Memory, t: u32, ok: u8, force: u8) -> bool {
    m.read_u8(t + L::FLAGS_13) & ok != 0
        && m.read_u16(L::STEP[0]) != 0
        && (m.read_u8(t + L::FLAGS_5B) & force != 0 || !moving_against_facing(m, t))
}

fn set_state(m: &mut impl Memory, t: u32, bit: u8, on: bool) {
    let s = m.read_u8(t + L::STATE);
    m.write_u8(t + L::STATE, if on { s | bit } else { s & !bit });
}

/// Pega el punto a la superficie del suelo: desde la posición de trabajo
/// menos la columna `col` (y más la fila `row`) de la celda de partida,
/// mira un punto desplazado según hacia dónde se mueve y la pendiente
/// nueva (`new_slope`) frente a la actual; en las celdas de pendiente
/// baja 8 o 16 píxeles, toma la gravedad de la tabla de pendiente y ajusta
/// la Y a la altura de la columna. Original: `Entity_FloorProbe_026e0a`.
pub fn floor_probe(m: &mut impl Memory, t: u32, new_slope: u32, col: u16, row: u16) -> Probe {
    let step = m.read_u16(L::STEP[0]);
    let dir = if step != 0 { step } else { m.read_u16(t + L::VEL_X) };
    let table = match dir as i16 {
        0 => L::PROBE_NONE,
        d if d > 0 => L::PROBE_RIGHT,
        _ => L::PROBE_LEFT,
    };
    let g = m.read_u16(t + L::GRAVITY);
    let mut k: u32 = match g as i16 {
        _ if g == L::NO_SLOPE || g == 0 => 8,
        v if v > 0 => 0,
        _ => 0x10,
    };
    let s = m.read_u16(new_slope);
    if s == L::NO_SLOPE || (s as i16) < (g as i16) {
        k += 4;
    }
    let dx = m.read_u16(table + k);
    let dy = m.read_u16(table + k + 2);
    let x = m.read_u16(L::WORK_POS[0]).wrapping_sub(col).wrapping_add(dx);
    let mut y = m.read_u16(L::WORK_POS[1]).wrapping_sub(1).wrapping_add(row).wrapping_add(dy);
    let mut surf = motion::surface(m, t, x, y, (col, row));
    let drop = if L::DEEP_KINDS.contains(&surf.kind) {
        8
    } else if L::DEEPER_KINDS.contains(&surf.kind) {
        16
    } else if L::RAMP_KINDS.contains(&surf.kind) && m.read_u8(t + L::FLAGS_13) & L::RAMP_OK != 0 {
        8
    } else {
        0
    };
    if drop != 0 {
        y = y.wrapping_add(drop);
        surf = motion::surface(m, t, x, y, surf.at);
    }
    let slope = m.read_u32(surf.record + L::T_SLOPE);
    let g = m.read_u16(slope);
    m.write_u16(t + L::GRAVITY, g);
    let g2 = m.read_u16(slope + L::S_GRAVITY2);
    m.write_u16(t + L::GRAVITY2, g2);
    let col = surf.at.0;
    let height = m.read_u16(slope + L::S_HEIGHTS + u32::from(col.wrapping_mul(2)));
    Probe { x, y: y.wrapping_sub(height), kind: surf.kind, record: surf.record, col, height, slope }
}

/// Un frame de movimiento sobre pendientes. Devuelve si ha vuelto a
/// sondear el suelo (C del original: la superficie ha cambiado). `at` =
/// columna y fila que deja el original si no mira el mapa (restos de
/// registros). Original: `Entity_MoveAndCollide_B_027036`.
pub fn move_b(m: &mut impl Memory, t: u32, at: (u16, u16)) -> bool {
    motion::integrate_velocity(m, t);
    motion::advance_on_slope(m, t);
    platforms::apply_camera_work(m, t);
    let (wx, wy) = (m.read_u16(L::WORK_POS[0]), m.read_u16(L::WORK_POS[1]).wrapping_sub(1));
    let here = collision::lookup_tile(m, t, wx, wy);
    let (sx, sy) = (m.read_u16(L::STEP[0]), m.read_u16(L::STEP[1]));
    let (x, y) = (wx.wrapping_add(sx), wy.wrapping_add(sy));
    // Celda de partida (para el sondeo), tipo y tabla de pendiente nuevos.
    let (from, kind, mut slope);
    if platforms::step_onto(m, t, x, y) {
        from = (0, 0);
        kind = L::PLATFORM_KIND;
        slope = m.read_u32(L::PLATFORM_TILE + L::T_SLOPE);
        let sub = m.read_u8(slope + L::S_SUB_Y);
        m.write_u8(L::NEW_SUB[1], sub);
    } else {
        let flat = m.read_u16(t + L::GRAVITY) == 0;
        from = here.frac.unwrap_or(at);
        let ahead = collision::lookup_tile(m, t, x, y);
        let col = ahead.frac.unwrap_or(from).0;
        let mut record = ahead.record;
        if m.read_u8(record + L::T_KIND_FLAGS) == 1 && flat {
            let steps = if (sx as i16) >= 0 { L::STEP_UP_RIGHT } else { L::STEP_UP_LEFT };
            if steps.contains(&ahead.kind) && may_climb(m, t, L::STEP_OK, L::STEP_FORCE) {
                set_state(m, t, L::ON_RAMP, true);
            } else {
                record += L::SECOND_MASK;
            }
        }
        kind = ahead.kind;
        slope = m.read_u32(record + L::T_SLOPE);
        let sub = m.read_u8(slope + L::S_SUB_Y + u32::from(col));
        m.write_u8(L::NEW_SUB[1], sub);
    }
    // Rampas: sólo se sube de cara a ellas y con permiso; si no, llano.
    let (up, down) = if (sx as i16) >= 0 { (L::RAMP_A, L::RAMP_B) } else { (L::RAMP_B, L::RAMP_A) };
    if up.contains(&kind) && may_climb(m, t, L::RAMP_OK, L::RAMP_FORCE) {
        set_state(m, t, L::ON_RAMP, true);
    } else if up.contains(&kind) || down.contains(&kind) {
        slope = L::FLAT_SLOPE;
    }
    if !moving_against_facing(m, t) && L::WALL_KINDS.contains(&kind) {
        set_state(m, t, L::HIT_WALL, true);
        return false;
    }
    let same = m.read_u16(t + L::GRAVITY2) == m.read_u16(slope + L::S_GRAVITY2)
        && m.read_u16(slope) == m.read_u16(t + L::GRAVITY);
    if same {
        set_state(m, t, L::ON_RAMP, false);
        m.write_u16(L::WORK_POS[0], x);
        m.write_u16(L::WORK_POS[1], y.wrapping_add(1));
        for k in 0..2 {
            let v = m.read_u8(L::NEW_SUB[k]);
            m.write_u8(L::WORK_SUB[k], v);
        }
        return false;
    }
    let p = floor_probe(m, t, slope, from.0, from.1);
    m.write_u8(L::WORK_SUB[0], 0);
    let sub = m.read_u8(p.slope + L::S_SUB_Y + u32::from(p.col));
    m.write_u8(L::WORK_SUB[1], sub);
    m.write_u16(L::WORK_POS[0], p.x);
    m.write_u16(L::WORK_POS[1], p.y.wrapping_add(1));
    true
}

/// [`move_b`] sobre la posición de la entidad: la copia a la zona de
/// trabajo, mueve, la devuelve y aplica el sombreado por profundidad.
/// Original: `Entity_StepB_UpdatePos_027902` (C igual que `move_b`).
pub fn move_b_entity(m: &mut impl Memory, t: u32, at: (u16, u16)) -> bool {
    for (w, f) in L::WORK_POS.into_iter().zip([L::X, L::Y]) {
        let v = m.read_u16(t + f);
        m.write_u16(w, v);
    }
    for (w, f) in L::WORK_SUB.into_iter().zip([L::SUB, L::SUB + 1]) {
        let v = m.read_u8(t + f);
        m.write_u8(w, v);
    }
    let c = move_b(m, t, at);
    for (w, f) in L::WORK_POS.into_iter().zip([L::X, L::Y]) {
        let v = m.read_u16(w);
        m.write_u16(t + f, v);
    }
    for (w, f) in L::WORK_SUB.into_iter().zip([L::SUB, L::SUB + 1]) {
        let v = m.read_u8(w);
        m.write_u8(t + f, v);
    }
    anim::apply_depth(m, t);
    c
}
