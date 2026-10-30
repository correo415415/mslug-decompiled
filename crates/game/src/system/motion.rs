//! Movimiento de las entidades con colisión contra el mapa: integración de
//! la velocidad, avance en subpíxeles, tope en los bordes de la pantalla y
//! aterrizaje sobre suelo, pendientes y plataformas móviles.
//!
//! Todo trabaja sobre la posición de trabajo de la colisión (`WORK_POS` y
//! sus fracciones), que quien llama copia desde y hacia la entidad.
//!
//! Original: `Entity_IntegrateVelocity_026814`, `Entity_MoveX_WallStop_026836`,
//! `Entity_ClampXToScreenEdge_028074`, `Entity_ApplyVelLatch_0281c8`,
//! `Entity_MoveAndCollide_D_0274b8`, `Entity_SlopeDirTest_02822c`,
//! `Entity_SlopeResolve_0275f6`, `Entity_SlopeCommit_0275b8`, `$0999CA`
//! (`asm/entity/`).

use super::collision::{self, clamp_velocity};
use super::platforms;
use crate::layout::{collision as C, motion as L, task as T};
use crate::mem::Memory;

/// Velocidad += aceleración en los dos ejes, con tope ±`MAX_SPEED`.
/// Original: `Entity_IntegrateVelocity_026814`.
pub fn integrate_velocity(m: &mut impl Memory, t: u32) {
    for (v, a) in [(L::VEL_X, L::ACC_X), (C::VEL_Y, C::ACC_Y)] {
        let n = m.read_u16(t + a).wrapping_add(m.read_u16(t + v));
        m.write_u16(t + v, clamp_velocity(n, L::MAX_SPEED).0);
    }
}

/// ¿Es `t` un jugador?
fn is_player(t: u32) -> bool {
    crate::layout::PLAYER_ENTITY.contains(&t)
}

/// Los jugadores no pasan de los bordes de la pantalla (salvo en la escena
/// sin bordes): anula la velocidad `vx` hacia fuera y mantiene el bit 7 de
/// `FLAGS2` = dentro. Original: `Entity_ClampXToScreenEdge_028074`.
pub fn clamp_to_screen(m: &mut impl Memory, t: u32, vx: u16) -> u16 {
    let x = m.read_u16(t + C::X) as i16;
    let f = m.read_u8(t + T::FLAGS2);
    let inside = m.read_u8(L::SCENE) == L::OPEN_SCENE || (L::LEFT_EDGE..=L::RIGHT_EDGE).contains(&x);
    if inside {
        m.write_u8(t + T::FLAGS2, f | L::INSIDE);
        return vx;
    }
    m.write_u8(t + T::FLAGS2, f & !L::INSIDE);
    let out = if x < L::LEFT_EDGE { (vx as i16) < 0 } else { (vx as i16) > 0 };
    if out { 0 } else { vx }
}

/// Avanza los subpíxeles: deja en `STEP` los píxeles enteros que se mueve
/// en cada eje y en `NEW_SUB` las fracciones nuevas. Las paredes (`+$69`)
/// cortan la velocidad X hacia ellas. Original: `Entity_MoveX_WallStop_026836`.
pub fn advance(m: &mut impl Memory, t: u32) {
    for k in 0..2 {
        let s = m.read_u8(C::WORK_SUB[k]);
        m.write_u8(L::NEW_SUB[k], s);
    }
    m.write_u16(L::STEP[0], 0);
    let walls = m.read_u8(t + L::WALLS);
    let mut vx = m.read_u16(t + L::VEL_X);
    if (walls & L::WALL_RIGHT != 0 && (vx as i16) > 0) || (walls & L::WALL_LEFT != 0 && (vx as i16) < 0) {
        vx = 0;
    }
    // En la escena del pozo los jugadores no se mueven por debajo del borde.
    let pit = m.read_u8(L::SCENE) == L::PIT_SCENE && is_player(t) && (m.read_u16(t + C::Y) as i16) >= L::PIT_Y;
    if pit {
        vx = 0;
    }
    let edges = is_player(t) || (t == L::SLUG && m.read_u8(t + L::SLUG_FLAGS) & L::SLUG_FREE == 0);
    if edges {
        vx = clamp_to_screen(m, t, vx);
    }
    let mut vy = m.read_u16(t + C::VEL_Y);
    if pit && (vy as i16) > 0 {
        vy = 0;
    }
    for (k, v) in [vx, vy].into_iter().enumerate() {
        let n = u16::from(m.read_u8(L::NEW_SUB[k])).wrapping_add(v);
        m.write_u8(L::NEW_SUB[k], n as u8);
        m.write_u16(L::STEP[k], ((n as i16) >> 8) as u16);
    }
    apply_latch(m, t);
}

/// En la escena del empuje (con el scroll empujando) dy no se escala; en
/// las pendientes suaves sí, a 2/3. Original: `Entity_ScaleDyForGravity_02800e`.
pub fn scale_dy(m: &mut impl Memory, t: u32, dy: u16) -> u16 {
    let g = m.read_u16(t + L::GRAVITY);
    if g != L::GENTLE_UP && g != L::GENTLE_DOWN {
        return dy;
    }
    if m.read_u8(L::SCENE) == L::PUSH_SCENE && m.read_u16(L::PUSH_ACTIVE) != 0 {
        return dy;
    }
    ((i32::from(dy as i16) * L::GENTLE_SCALE) >> 8) as u16
}

/// Avance de un frame siguiendo la pendiente: el paso en X sale de la
/// velocidad (cortada por las paredes y los bordes de pantalla) y el paso
/// en Y depende de la pendiente del suelo (`+$34`): en llano no hay; en
/// pendiente suave es el mismo que en X; en la de 45°, ×64 en subpíxeles.
/// Deja los pasos en `STEP` y las fracciones en `NEW_SUB`.
/// Original: `Entity_MoveXY_Probe_026992`.
pub fn advance_on_slope(m: &mut impl Memory, t: u32) {
    for k in 0..2 {
        let s = m.read_u8(C::WORK_SUB[k]);
        m.write_u8(L::NEW_SUB[k], s);
    }
    m.write_u16(L::STEP[0], 0);
    let walls = m.read_u8(t + L::WALLS);
    let mut vx = m.read_u16(t + L::VEL_X);
    if (walls & L::WALL_RIGHT != 0 && (vx as i16) > 0) || (walls & L::WALL_LEFT != 0 && (vx as i16) < 0) {
        vx = 0;
    }
    let edges = is_player(t) || (t == L::SLUG && m.read_u8(t + L::SLUG_FLAGS) & L::SLUG_FREE == 0);
    if edges {
        vx = clamp_to_screen(m, t, vx);
    }
    let vx = scale_dy(m, t, vx);
    let n = u16::from(m.read_u8(L::NEW_SUB[0])).wrapping_add(vx);
    m.write_u8(L::NEW_SUB[0], n as u8);
    m.write_u16(L::STEP[0], ((n as i16) >> 8) as u16);
    apply_latch(m, t);
    let dx = m.read_u16(L::STEP[0]);
    let g = m.read_u16(t + L::GRAVITY);
    let neg = (g as i16) < 0;
    let signed = |v: u16| if neg { v.wrapping_neg() } else { v };
    match (g as i16).unsigned_abs() {
        // Sin pendiente (en el aire) o llano: en llano tampoco avanza en X.
        _ if g == L::NO_SLOPE || (g as i16).unsigned_abs() == L::SLOPE_FLAT => {
            if g != L::NO_SLOPE {
                m.write_u16(L::STEP[0], 0);
            }
            if m.read_u8(t + L::FLAGS_5B) & L::NO_VERTICAL != 0 {
                m.write_u16(L::STEP[1], 0);
            }
        }
        0 => m.write_u16(L::STEP[1], 0),
        a if a == L::SLOPE_GENTLE => m.write_u16(L::STEP[1], signed(dx)),
        a if a == L::SLOPE_STEEP => {
            let n = u16::from(m.read_u8(L::NEW_SUB[1])).wrapping_add(signed(dx.wrapping_shl(6)));
            m.write_u8(L::NEW_SUB[1], n as u8);
            m.write_u16(L::STEP[1], ((n as i16) >> 8) as u16);
        }
        _ => debug_assert!(false, "pendiente desconocida"),
    }
    let pit = m.read_u8(L::SCENE) == L::PIT_SCENE
        && is_player(t)
        && (m.read_u16(t + C::Y) as i16) >= L::PIT_FLOOR
        && (m.read_u16(L::STEP[1]) as i16) > 0;
    if pit {
        m.write_u16(L::STEP[0], 0);
        m.write_u16(L::STEP[1], 0);
        for k in 0..2 {
            let s = m.read_u8(C::WORK_SUB[k]);
            m.write_u8(L::NEW_SUB[k], s);
        }
    }
}

/// Signo de una velocidad como lo guarda el cerrojo (0, 1 o `$FFFF`).
fn sign(v: u16) -> u16 {
    match (v as i16).signum() {
        0 => 0,
        1 => 1,
        _ => 0xFFFF,
    }
}

/// Si se había bloqueado el avance en X en el sentido de la velocidad,
/// este frame no se mueve en X; los cerrojos duran un frame.
/// Original: `Entity_ApplyVelLatch_0281c8`.
pub fn apply_latch(m: &mut impl Memory, t: u32) {
    let lx = m.read_u16(L::LATCH[0]);
    if lx != 0 && sign(m.read_u16(t + L::VEL_X)) == lx {
        m.write_u16(L::STEP[0], 0);
        m.write_u8(L::NEW_SUB[0], 0);
    }
    m.write_u16(L::LATCH[0], 0);
    m.write_u16(L::LATCH[1], 0);
}

/// Superficie bajo un punto: tile del mapa o, si hay una plataforma móvil,
/// el tipo de plataforma. Deja la plataforma en `PLATFORM_UNDER`.
#[derive(Debug, Clone, Copy)]
pub struct Surface {
    /// Celda del mapa tal cual (el `d7` que deja el original).
    pub raw: u8,
    pub kind: u8,
    pub record: u32,
    pub at: (u16, u16),
}

/// Original: el bloque `$027DB2` + `$09993C` que se repite en la colisión.
pub fn surface(m: &mut impl Memory, t: u32, x: u16, y: u16, at: (u16, u16)) -> Surface {
    let tile = collision::lookup_tile(m, t, x, y);
    let platform = platforms::find_under(m, t, x, y);
    m.write_u8(C::PLATFORM_UNDER, platform.map_or(0x0F, |p| p.1));
    match platform {
        Some((depth, _)) => Surface { raw: tile.raw, kind: L::PLATFORM_KIND, record: C::PLATFORM_TILE, at: (0, depth) },
        None => Surface { raw: tile.raw, kind: tile.kind, record: tile.record, at: tile.frac.unwrap_or(at) },
    }
}

/// ¿Se aleja de la pendiente `slope` (no aterriza) la entidad con su
/// velocidad actual? Original: `Entity_SlopeDirTest_02822c` (C = se aleja).
pub fn leaving_slope(m: &mut impl Memory, t: u32, slope: u32) -> bool {
    let vx = m.read_u16(t + L::VEL_X) as i16;
    let mut vy = m.read_u16(t + C::VEL_Y) as i16;
    match m.read_u16(slope) {
        0 => vy > 0,
        0xFFFF => true,
        d if (d as i16) > 0 => {
            if d == L::STEEP_UP {
                vy = vy.wrapping_mul(4);
            }
            vy.wrapping_sub(vx) > 0
        }
        d => {
            if d == L::STEEP_DOWN {
                vy = vy.wrapping_mul(4);
            }
            vy.wrapping_add(vx) > 0
        }
    }
}

/// Altura del suelo de una pendiente: en las celdas de pendiente mira
/// 8 o 16 píxeles más abajo, toma la gravedad del tipo de tile y ajusta
/// `y` a la altura de la columna. `Ok((y, columna))` = aterriza;
/// `Err(y)` = no (objeto sujeto sin gravedad), con la `y` ya bajada.
/// Original: `Entity_SlopeResolve_0275f6` / `Entity_SlopeTail_02772c`.
pub(crate) fn slope_floor(m: &mut impl Memory, t: u32, x: u16, y: u16, mut s: Surface) -> Result<(u16, u16), u16> {
    let mut y = y;
    let drop = match s.kind {
        0x0C..=0x0F | 0x1C..=0x1F | 0x34 | 0x20 | 0x21 | 0x30 | 0x31 => 8,
        0x2C..=0x2F | 0x3C..=0x3F => 16,
        _ => 0,
    };
    if drop != 0 {
        y = y.wrapping_add(drop);
        s = surface(m, t, x, y, s.at);
    }
    let table = m.read_u32(s.record + L::T_SLOPE);
    let g = m.read_u16(table);
    m.write_u16(t + C::GRAVITY, g);
    let g2 = m.read_u16(table + L::T_GRAVITY2);
    m.write_u16(t + L::GRAVITY2, g2);
    if m.read_u8(t + T::FLAGS2) & L::INSIDE != 0 && g == C::NO_GRAVITY {
        return Err(y);
    }
    let h = m.read_u16(table + 2 + u32::from(s.at.0.wrapping_mul(2)));
    Ok((y.wrapping_add(s.at.1).wrapping_sub(h), s.at.0))
}

/// Mueve la posición de trabajo con el paso de este frame y mira si
/// aterriza. Devuelve si ha tocado suelo. `at` = columna y fila que deja el
/// original si no mira el mapa (restos de registros).
/// Original: `Entity_MoveAndCollide_D_0274b8__L0274cc`.
pub fn land(m: &mut impl Memory, t: u32, mut at: (u16, u16)) -> bool {
    platforms::apply_camera_work(m, t);
    let wx = m.read_u16(C::WORK_POS[0]);
    let wy = m.read_u16(C::WORK_POS[1]);
    let x = wx.wrapping_add(m.read_u16(L::STEP[0]));
    let ahead = collision::lookup_tile(m, t, x, wy);
    if let Some(f) = ahead.frac {
        at = f;
    }
    if L::WALL_KINDS.contains(&ahead.kind) {
        let s = m.read_u8(t + C::STATE);
        m.write_u8(t + C::STATE, s | L::HIT_WALL);
        m.write_u16(L::STEP[0], 0);
        m.write_u8(L::NEW_SUB[0], 0);
    }
    let x = wx.wrapping_add(m.read_u16(L::STEP[0]));
    let mut y = wy.wrapping_add(m.read_u16(L::STEP[1])).wrapping_sub(1);
    let s = surface(m, t, x, y, at);
    if s.kind == L::CEILING_KIND && (m.read_u16(t + C::VEL_Y) as i16) > 0 {
        m.write_u16(t + C::VEL_Y, 0);
    }
    if let Some(hit) = collision::test_masks(m, s.record, s.at.0, s.at.1) {
        let slope = m.read_u32(hit + L::T_SLOPE);
        if !leaving_slope(m, t, slope) {
            match slope_floor(m, t, x, y, Surface { record: hit, ..s }) {
                Ok((floor, col)) => {
                    m.write_u16(C::WORK_POS[0], x);
                    m.write_u16(C::WORK_POS[1], floor.wrapping_add(1));
                    m.write_u8(C::WORK_SUB[0], 0);
                    let sub = m.read_u8(slope + L::T_SUB_Y + u32::from(col));
                    m.write_u8(C::WORK_SUB[1], sub);
                    let st = m.read_u8(t + C::STATE);
                    m.write_u8(t + C::STATE, st | C::ON_FLOOR);
                    let p = m.read_u8(C::PLATFORM_UNDER);
                    set_platform(m, t, p);
                    return true;
                }
                Err(dropped) => y = dropped,
            }
        }
    }
    m.write_u16(C::WORK_POS[0], x);
    m.write_u16(C::WORK_POS[1], y.wrapping_add(1));
    for k in 0..2 {
        let v = m.read_u8(L::NEW_SUB[k]);
        m.write_u8(C::WORK_SUB[k], v);
    }
    false
}

/// Apunta en la entidad la plataforma sobre la que está (nibble bajo del
/// byte de identificadores). Original: `$0999CA`.
pub fn set_platform(m: &mut impl Memory, t: u32, id: u8) {
    let v = m.read_u8(t + L::ID);
    m.write_u8(t + L::ID, (v & 0xF0) | (id & 0x0F));
}

/// Movimiento con colisión de un frame: quita la gravedad propia, integra
/// la velocidad, avanza y mira si aterriza. Original:
/// `Entity_MoveAndCollide_D_0274b8`.
/// `row` = fila que deja el original en `d4` (por si no mira el mapa;
/// `advance` deja la columna a 0).
pub fn move_and_collide(m: &mut impl Memory, t: u32, row: u16) -> bool {
    m.write_u16(t + C::GRAVITY, C::NO_GRAVITY);
    m.write_u16(t + L::GRAVITY2, C::NO_GRAVITY);
    integrate_velocity(m, t);
    advance(m, t);
    land(m, t, (0, row))
}

/// Como [`move_and_collide`] pero la velocidad vertical sólo se limita
/// hacia arriba, y si aterriza se para. Original: `Entity_MoveFall_027464`.
pub fn move_fall(m: &mut impl Memory, t: u32, row: u16) -> bool {
    m.write_u16(t + C::GRAVITY, C::NO_GRAVITY);
    m.write_u16(t + L::GRAVITY2, C::NO_GRAVITY);
    let vx = m.read_u16(t + L::ACC_X).wrapping_add(m.read_u16(t + L::VEL_X));
    m.write_u16(t + L::VEL_X, clamp_velocity(vx, L::MAX_SPEED).0);
    let mut vy = m.read_u16(t + C::ACC_Y).wrapping_add(m.read_u16(t + C::VEL_Y));
    if (vy as i16) <= 0 {
        vy = clamp_velocity(vy, L::MAX_SPEED).0;
    }
    m.write_u16(t + C::VEL_Y, vy);
    advance(m, t);
    if land(m, t, (0, row)) {
        m.write_u16(t + C::ACC_Y, 0);
        m.write_u16(t + C::VEL_Y, 0);
        return true;
    }
    false
}

/// Ejecuta `step` sobre la posición de la entidad: la copia a la zona de
/// trabajo y la devuelve después (con lo que diga `step`). Original: las
/// envolturas `$027C2A` (con `$027444`) y `$027D50` (con `$02773C`).
pub fn on_entity_pos<M: Memory>(m: &mut M, t: u32, step: impl FnOnce(&mut M) -> bool) -> bool {
    let x = m.read_u16(t + C::X);
    let y = m.read_u16(t + C::Y);
    m.write_u16(C::WORK_POS[0], x);
    m.write_u16(C::WORK_POS[1], y);
    for (w, f) in C::WORK_SUB.into_iter().zip([C::SUB_X, C::SUB_Y]) {
        let v = m.read_u8(t + f);
        m.write_u8(w, v);
    }
    let c = step(m);
    let (x, y) = (m.read_u16(C::WORK_POS[0]), m.read_u16(C::WORK_POS[1]));
    m.write_u16(t + C::X, x);
    m.write_u16(t + C::Y, y);
    for (w, f) in C::WORK_SUB.into_iter().zip([C::SUB_X, C::SUB_Y]) {
        let v = m.read_u8(w);
        m.write_u8(t + f, v);
    }
    c
}

/// Movimiento con colisión «E» (subida): avanza y, si el punto nuevo es
/// sólido, va hacia arriba y el tipo de tile no lo prohíbe, se apoya en la
/// superficie. Devuelve si se ha apoyado. `at` = columna y fila que deja
/// el original si no mira el mapa. Original: `Entity_MoveAndCollide_E_02773c`
/// (con `Entity_SlopeCommit_0275b8`).
pub fn move_e(m: &mut impl Memory, t: u32, at: (u16, u16)) -> bool {
    platforms::apply_camera_work(m, t);
    integrate_velocity(m, t);
    advance(m, t);
    let x = m.read_u16(C::WORK_POS[0]).wrapping_add(m.read_u16(L::STEP[0]));
    let mut y = m.read_u16(C::WORK_POS[1]).wrapping_add(m.read_u16(L::STEP[1])).wrapping_sub(1);
    let s = surface(m, t, x, y, at);
    if s.kind != 0 && super::slope::solid_bit(m, s.record, s.at.0, s.at.1) {
        let step = m.read_u16(L::STEP[1]);
        let dir = if step != 0 { step } else { m.read_u16(t + C::VEL_Y) };
        if (dir as i16) < 0 && m.read_u8(s.record + L::T_NO_CLIMB) == 0 {
            let slope = m.read_u32(s.record + L::T_SLOPE);
            match slope_floor(m, t, x, y, s) {
                Ok((floor, col)) => {
                    m.write_u16(C::WORK_POS[0], x);
                    m.write_u16(C::WORK_POS[1], floor.wrapping_add(1));
                    m.write_u8(C::WORK_SUB[0], 0);
                    let sub = m.read_u8(slope + L::T_SUB_Y + u32::from(col));
                    m.write_u8(C::WORK_SUB[1], sub);
                    let st = m.read_u8(t + C::STATE);
                    m.write_u8(t + C::STATE, st | C::ON_FLOOR);
                    let p = m.read_u8(C::PLATFORM_UNDER);
                    set_platform(m, t, p);
                    return true;
                }
                Err(dropped) => y = dropped,
            }
        }
    }
    m.write_u16(C::WORK_POS[0], x);
    m.write_u16(C::WORK_POS[1], y.wrapping_add(1));
    for k in 0..2 {
        let v = m.read_u8(L::NEW_SUB[k]);
        m.write_u8(C::WORK_SUB[k], v);
    }
    false
}

/// Como [`move_e`] pero sin apoyarse: avanza y sólo dice si la celda bajo
/// los pies es sólida y el tipo de tile lo permite (sin mirar hacia dónde
/// va). Original: `$0277C4`.
pub fn move_e_probe(m: &mut impl Memory, t: u32, at: (u16, u16)) -> bool {
    move_e_probe_surface(m, t, at).0
}

/// [`move_e_probe`] devolviendo además la superficie mirada (el original
/// deja su tipo en `d0` y la celda en `d7`).
pub fn move_e_probe_surface(m: &mut impl Memory, t: u32, at: (u16, u16)) -> (bool, Surface) {
    platforms::apply_camera_work(m, t);
    integrate_velocity(m, t);
    advance(m, t);
    let x = m.read_u16(C::WORK_POS[0]).wrapping_add(m.read_u16(L::STEP[0]));
    let y = m.read_u16(C::WORK_POS[1]).wrapping_add(m.read_u16(L::STEP[1])).wrapping_sub(1);
    let s = surface(m, t, x, y, at);
    let hit = s.kind != 0
        && super::slope::solid_bit(m, s.record, s.at.0, s.at.1)
        && m.read_u8(s.record + L::T_NO_CLIMB) == 0;
    m.write_u16(C::WORK_POS[0], x);
    m.write_u16(C::WORK_POS[1], y.wrapping_add(1));
    for k in 0..2 {
        let v = m.read_u8(L::NEW_SUB[k]);
        m.write_u8(C::WORK_SUB[k], v);
    }
    (hit, s)
}

/// Movimiento con colisión «C» (andar por el suelo): integra, avanza por
/// la pendiente y, si el punto nuevo es una plataforma o un tile con la
/// misma pendiente que la que lleva, lo acepta. Devuelve si se ha
/// bloqueado (pared o cambio de pendiente), sin moverse.
/// Original: `Entity_MoveAndCollide_C_0272a8` (C = bloqueado).
pub fn move_c(m: &mut impl Memory, t: u32) -> bool {
    use crate::layout::slope as S;
    integrate_velocity(m, t);
    advance_on_slope(m, t);
    platforms::apply_camera_work(m, t);
    let x = m.read_u16(S::WORK_POS[0]).wrapping_add(m.read_u16(S::STEP[0]));
    let y = m.read_u16(S::WORK_POS[1]).wrapping_sub(1).wrapping_add(m.read_u16(S::STEP[1]));
    let (kind, mut slope) = if platforms::step_onto(m, t, x, y) {
        let table = m.read_u32(S::PLATFORM_TILE + S::T_SLOPE);
        let sub = m.read_u8(table + S::S_SUB_Y);
        m.write_u8(S::NEW_SUB[1], sub);
        (S::PLATFORM_KIND, table)
    } else {
        let tile = collision::lookup_tile(m, t, x, y);
        let mut rec = tile.record;
        if m.read_u8(rec + S::T_KIND_FLAGS) == 1 && m.read_u16(t + S::GRAVITY) == 0 {
            rec += S::SECOND_MASK;
        }
        (tile.kind, m.read_u32(rec + S::T_SLOPE))
    };
    if S::RAMP_KINDS.contains(&kind) && m.read_u8(t + S::FLAGS_13) & S::RAMP_OK == 0 {
        slope = S::FLAT_SLOPE;
    }
    if S::WALL_KINDS.contains(&kind) {
        let st = m.read_u8(t + S::STATE);
        m.write_u8(t + S::STATE, st | S::HIT_WALL);
        return true;
    }
    if m.read_u16(t + S::GRAVITY2) != m.read_u16(slope + S::S_GRAVITY2) || m.read_u16(t + S::GRAVITY) != m.read_u16(slope) {
        return true;
    }
    m.write_u16(S::WORK_POS[0], x);
    m.write_u16(S::WORK_POS[1], y.wrapping_add(1));
    for k in 0..2 {
        let v = m.read_u8(S::NEW_SUB[k]);
        m.write_u8(S::WORK_SUB[k], v);
    }
    false
}

/// [`move_c`] sobre la posición de la entidad y la profundidad según la
/// altura. Original: `Entity_ProbeRevertCcr_027AFC` (C = bloqueado).
pub fn walk_c(m: &mut impl Memory, t: u32) -> bool {
    let c = on_entity_pos(m, t, |m| move_c(m, t));
    super::anim::apply_depth(m, t);
    c
}
