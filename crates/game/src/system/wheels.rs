//! Ruedas del Slug: los tres eslabones de la cadena del chasis (delantera,
//! trasera y la del medio). Cada una sigue la X del Slug a su distancia
//! fija y busca el suelo bajo ella: si lo pisa, rueda sobre las pendientes
//! con el motor B ([`slope::move_b_entity`]); si no, cae (o sube, si el
//! chasis la empuja hacia arriba) hasta volver a apoyarse. El chasis se
//! coloca después entre ellas (`system::slug`).
//!
//! La del medio no tiene física propia: copia la X y la Y de apoyo del
//! Slug y sólo mira si pisa.
//!
//! Original: `Chain3_Init_030002`, `Chain3_TplA_030068`, `Chain3_TplB_0300ba`,
//! `Chain3_TplC_03010c`, `Chain3_Follow_0301f0`, `Chain3_YDelta_030392`,
//! `Chain3_Step_0303ee`, `Chain3_VelY_030416`, `Chain3_VelX_030462`,
//! `Chain3_CheckSyncB_03050c`, `Chain3_DebugHud_030554`
//! (`asm/slug/vehicle_02dd20.s`).

use super::{anim, collision, hud, motion, platforms, slope, slug};
use crate::layout::{task as T, wheels as L};
use crate::mem::{Machine, Memory};
use crate::Flow;

fn owner(m: &mut impl Memory, t: u32) -> u32 {
    m.read_u32(t + T::OWNER)
}

fn set_bits(m: &mut impl Memory, a: u32, bits: u8, on: bool) {
    let v = m.read_u8(a);
    m.write_u8(a, if on { v | bits } else { v & !bits });
}

/// Restos de `d3`/`d4` con los que el original sondea si no mira el mapa.
fn leftover(m: &mut impl Machine) -> (u16, u16) {
    let r = m.regs();
    (r.d[3] as u16, r.d[4] as u16)
}

/// Preparación común: flags, atributos del sprite, velocidades a cero, si
/// pisa suelo y el guion de animación. Original: `Chain3_Init_030002`.
pub fn init(m: &mut impl Machine, t: u32) {
    if m.read_u8(t + L::KIND) == 0 {
        set_bits(m, t + L::FLAGS_5B, L::PASS_TOP, true);
    }
    set_bits(m, t + L::IGNORE, L::IGNORE_INIT, true);
    set_bits(m, t + T::FLAGS, L::FLAGS_INIT, true);
    anim::set_attr(m, t, L::ATTR_INIT);
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, (a & L::ATTR_KEEP) | L::ATTR_DEPTH);
    m.write_u16(t + L::ACC_X, 0);
    m.write_u16(t + L::ACC_Y, 0);
    m.write_u16(t + L::PUSH, 0);
    m.write_u8(t + L::RISING, 0);
    let at = leftover(m);
    let on = !collision::probe_floor(m, t, at).in_air;
    m.write_u8(t + L::ON_GROUND, u8::from(on));
    anim::start(m, t, L::SCRIPT);
}

/// Sprites de una rueda y su distancia al centro del Slug (que se suma a
/// la X). Devuelve la distancia.
fn place(m: &mut impl Machine, t: u32, table: u32) -> u16 {
    for g in L::GFX {
        anim::alloc_entry(m, t, g);
    }
    let dx = m.read_u16(table + L::TABLE_DX);
    m.write_u16(t + L::OFFSET, dx);
    let x = m.read_u16(t + L::X).wrapping_add(dx);
    m.write_u16(t + L::X, x);
    dx
}

/// Alta de la rueda delantera / trasera (`rear`): sus sprites, distancia,
/// lado al que mira y con qué sentido de marcha toma los permisos de
/// subida del Slug. Original: `Chain3_TplA_030068` / `Chain3_TplB_0300ba`.
pub fn start(m: &mut impl Machine, rear: bool) -> Flow {
    let t = m.task_register();
    for g in L::GFX {
        anim::alloc_entry(m, t, g);
    }
    init(m, t);
    let k = usize::from(rear);
    let dx = m.read_u16(L::TABLES[k] + L::TABLE_DX);
    m.write_u16(t + L::OFFSET, dx);
    let x = m.read_u16(t + L::X).wrapping_add(dx);
    m.write_u16(t + L::X, x);
    m.write_u8(t + L::FACING, L::FACINGS[k]);
    m.write_u16(t + L::CLIMB_SIDE, L::CLIMB_SIDES[k]);
    m.write_u32(t + T::HANDLER, L::RUN);
    run(m)
}

/// Alta de la rueda del medio. Original: `Chain3_TplC_03010c`.
pub fn middle_start(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    place(m, t, L::TABLES[2]);
    init(m, t);
    let v = m.read_u16(t + L::FIELD_16);
    m.write_u16(t + L::FIELD_14, v);
    m.write_u32(t + T::HANDLER, L::MIDDLE_RUN);
    middle_run(m)
}

/// Si el Slug se ha parado en seco, se olvida el empujón pendiente.
fn drop_push_if_stopped(m: &mut impl Memory, t: u32) {
    let o = owner(m, t);
    if m.read_u8(o + L::OWNER_FLAGS_13) & L::OWNER_STOPPED != 0 {
        m.write_u16(t + L::PUSH, 0);
    }
}

/// Y del suelo bajo la rueda (más el empujón pendiente, que se gasta).
fn apply_push(m: &mut impl Memory, t: u32) {
    let y = m.read_u16(t + L::Y);
    m.write_u16(t + L::GROUND_Y, y);
    let push = m.read_u16(t + L::PUSH);
    if push == 0 {
        return;
    }
    if m.read_u16(L::PUSH_GROUND_ONLY) == 0 {
        m.write_u16(t + L::GROUND_Y, y.wrapping_add(push));
    } else {
        m.write_u16(t + L::Y, y.wrapping_add(push));
    }
}

/// Con el modo de depuración, el sprite se pinta encima de todo y avanza
/// su animación.
fn debug_tail(m: &mut impl Machine, t: u32) -> Flow {
    if m.read_u8(L::DEBUG) & L::DEBUG_WHEELS != 0 {
        m.write_u16(t + L::ATTR, L::DEBUG_ATTR);
        anim::step(m, t);
    }
    Flow::Return
}

/// Rueda del medio: va con el Slug, mira si pisa y apunta las Y de las
/// otras dos. Original: `Chain3_TplC_03010c` (bucle `$030156`).
pub fn middle_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    drop_push_if_stopped(m, t);
    remember_pos(m, t);
    let o = owner(m, t);
    let (y, x) = (m.read_u16(o + L::OWNER_BASE_Y), m.read_u16(o + L::X));
    m.write_u16(t + L::Y, y);
    m.write_u16(t + L::X, x);
    let at = leftover(m);
    let on = !collision::probe_floor(m, t, at).in_air;
    m.write_u8(t + L::ON_GROUND, u8::from(on));
    apply_push(m, t);
    for (f, w) in [(L::FRONT_Y, L::LINKS[0]), (L::REAR_Y, L::LINKS[1])] {
        let wheel = m.read_u32(t + w);
        let v = m.read_u16(wheel + L::Y);
        m.write_u16(t + f, v);
    }
    debug_hud(m, t);
    m.write_u16(t + L::PUSH, 0);
    debug_tail(m, t)
}

fn remember_pos(m: &mut impl Memory, t: u32) {
    let (x, y) = (m.read_u16(t + L::X), m.read_u16(t + L::Y));
    m.write_u16(t + L::PREV_X, x);
    m.write_u16(t + L::PREV_Y, y);
}

/// Rueda delantera / trasera, cada frame. Original: `Chain3_Follow_0301f0`
/// (bucle `$0301F6`).
pub fn run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    drop_push_if_stopped(m, t);
    remember_pos(m, t);
    platforms::move_with_scene(m, t);
    follow_x(m, t);
    let mut grounded = m.read_u8(t + L::ON_GROUND) != 0;
    if grounded {
        let v = m.read_u16(t + L::FIELD_16);
        m.write_u16(t + L::FIELD_14, v);
        sync_climb(m, t);
        let at = leftover(m);
        if slope::move_b_entity(m, t, at) {
            set_bits(m, t + L::STATE, L::SURFACE_CHANGED, true);
        }
        let at = leftover(m);
        if collision::probe_floor(m, t, at).in_air {
            m.write_u8(t + L::ON_GROUND, 0);
        }
        let o = owner(m, t);
        if m.read_u16(o + L::GRAVITY) == L::NO_SLOPE {
            m.write_u8(t + L::ON_GROUND, 0);
        }
        if too_far(m, t) {
            m.write_u16(t + L::VEL_Y, 0);
            m.write_u8(t + L::ON_GROUND, 0);
            m.write_u8(t + L::RISING, L::RISING_ON);
        }
        grounded = m.read_u8(t + L::ON_GROUND) != 0;
        if !grounded {
            m.write_u16(t + L::VEL_X, 0);
        }
    }
    if !grounded {
        airborne(m, t);
    }
    apply_push(m, t);
    m.write_u16(t + L::PUSH, 0);
    debug_tail(m, t)
}

/// En el aire: con el Slug también en el aire, sigue su velocidad
/// vertical; si no, un muelle hacia la Y a la que debería estar. Después
/// cae (o sube) hasta apoyarse.
fn airborne(m: &mut impl Machine, t: u32) {
    let o = owner(m, t);
    let base = m.read_u16(o + L::OWNER_BASE_Y);
    let g = m.read_u16(o + L::GRAVITY);
    if g == L::NO_SLOPE {
        let v = m.read_u16(t + L::FIELD_18);
        m.write_u16(t + L::FIELD_14, v);
        vel_y(m, t);
        let (x, y) = (m.read_u16(t + L::X), m.read_u16(t + L::Y));
        if motion::surface(m, t, x, y, (0, 0)).kind & L::KIND_MASK == 0
            && motion::surface(m, t, x, y.wrapping_sub(L::HEAD_ROOM), (0, 0)).kind & L::KIND_MASK == 0
        {
            m.write_u8(t + L::RISING, 0);
        }
    } else {
        let v = m.read_u16(t + L::FIELD_1A);
        m.write_u16(t + L::FIELD_14, v);
        let d = step(m, t, base, g).wrapping_sub(m.read_u16(t + L::Y));
        if (d as i16) >= 0 {
            m.write_u8(t + L::RISING, 0);
        }
        let vy = m.read_u16(t + L::VEL_Y);
        let d = (d << 6).wrapping_sub(((vy as i16) >> 1) as u16);
        m.write_u16(t + L::VEL_Y, vy.wrapping_add(d));
    }
    let at = leftover(m);
    let landed = if m.read_u8(t + L::RISING) != 0 {
        motion::on_entity_pos(m, t, |m| motion::move_e(m, t, at))
    } else {
        motion::on_entity_pos(m, t, |m| slug::move_and_stop(m, t, at.1))
    };
    if landed {
        m.write_u8(t + L::ON_GROUND, 1);
        m.write_u8(t + L::RISING, 0);
    } else if m.read_u16(t + L::VEL_Y) < L::SLOW_FALL {
        let at = leftover(m);
        if !collision::probe_floor(m, t, at).in_air {
            m.write_u8(t + L::ON_GROUND, 1);
            m.write_u8(t + L::RISING, 0);
        }
    }
}

/// ¿Se ha separado demasiado la rueda de la Y de apoyo del Slug? En el
/// aire: por debajo (o, si va hacia el lado contrario al que mira, por
/// encima o igual); en el suelo: más de `MAX_GAP` arriba o abajo.
/// Original: `Chain3_YDelta_030392` (C).
pub fn too_far(m: &mut impl Memory, t: u32) -> bool {
    let o = owner(m, t);
    let d = m.read_u16(t + L::Y).wrapping_sub(m.read_u16(o + L::OWNER_BASE_Y)) as i16;
    if m.read_u16(o + L::GRAVITY) == L::NO_SLOPE {
        let vx = m.read_u16(t + L::VEL_X);
        if vx != 0 && ((m.read_u16(t + L::OFFSET) ^ vx) as i16) < 0 {
            d >= 0
        } else {
            d >= L::AIR_GAP
        }
    } else {
        d > L::MAX_GAP || d < -L::MAX_GAP
    }
}

/// Y a la que debería estar la rueda sobre la pendiente `g` del Slug,
/// desde la Y de apoyo `y`. Original: `Chain3_Step_0303ee`.
pub fn step(m: &mut impl Memory, t: u32, y: u16, g: u16) -> u16 {
    if g == 0 {
        return y;
    }
    let mut d = m.read_u16(t + L::OFFSET);
    if (g as i16) < 0 {
        d = d.wrapping_neg();
    }
    d = d.wrapping_sub(1);
    if g == L::STEEP_UP || g == L::STEEP_DOWN {
        d = ((d as i16) >> 2) as u16;
    }
    y.wrapping_add(d)
}

/// Velocidad vertical en el aire: si el Slug está apoyado sobre las
/// ruedas, gravedad; si no, la del Slug más un muelle hacia su Y de apoyo
/// (sólo si las otras dos ruedas están casi a la misma altura).
/// Original: `Chain3_VelY_030416`.
pub fn vel_y(m: &mut impl Memory, t: u32) {
    let o = owner(m, t);
    if m.read_u8(o + L::FLAGS_5B) & L::OWNER_ON_WHEELS != 0 {
        let v = m.read_u16(t + L::VEL_Y).wrapping_sub(L::GRAVITY_STEP);
        m.write_u16(t + L::VEL_Y, v);
        return;
    }
    let v = m.read_u16(o + L::VEL_Y);
    m.write_u16(t + L::VEL_Y, v);
    let a = m.read_u32(t + L::LINKS[1]);
    let b = m.read_u32(t + L::LINKS[0]);
    let gap = (m.read_u16(a + L::Y).wrapping_sub(m.read_u16(b + L::Y)) as i16).unsigned_abs();
    if gap < L::LEVEL_GAP {
        let d = m.read_u16(o + L::OWNER_BASE_Y).wrapping_sub(m.read_u16(t + L::Y));
        m.write_u16(t + L::VEL_Y, v.wrapping_add(d << 4));
    }
}

/// Velocidad X para ponerse a su distancia del Slug y permisos de subida
/// (rampas / escalones): los del Slug si éste va en el sentido de esta
/// rueda; si no, los que diga la sincronía de las ruedas.
/// Original: `Chain3_VelX_030462`.
pub fn follow_x(m: &mut impl Memory, t: u32) {
    let o = owner(m, t);
    let d = m.read_u16(o + L::X).wrapping_add(m.read_u16(t + L::OFFSET)).wrapping_sub(m.read_u16(t + L::X));
    m.write_u16(t + L::VEL_X, d << 8);
    let side = m.read_u16(t + L::CLIMB_SIDE);
    let f = m.read_u8(t + L::FLAGS_13);
    if side != L::ANY_SIDE && m.read_u16(o + L::VEL_X) & 0x8000 == side {
        let climb = m.read_u8(o + L::FLAGS_13) & L::CLIMB_BITS;
        m.write_u8(t + L::FLAGS_13, (f & !L::CLIMB_BITS) | climb);
    } else {
        // El original limpia aquí más bits de los que pone (`neg` en vez
        // de `not`): sólo conserva los bits 7, 6 y 4.
        let climb = m.read_u8(t + L::FLAGS_5B) & L::CLIMB_BITS;
        m.write_u8(t + L::FLAGS_13, (f & L::CLIMB_BITS.wrapping_neg()) | climb);
    }
}

/// Permiso para subir aunque vaya de espaldas: sólo si las dos ruedas
/// están a la misma Y de apoyo, o si ninguna rueda tiene empujón
/// pendiente. Original: `Chain3_CheckSyncB_03050c`.
pub fn sync_climb(m: &mut impl Memory, t: u32) {
    let [a, b, c] = L::LINKS.map(|f| m.read_u32(t + f));
    let level = m.read_u16(a + L::PREV_Y) == m.read_u16(b + L::PREV_Y);
    let pushed = (m.read_u16(c + L::PUSH) | m.read_u16(a + L::PUSH) | m.read_u16(b + L::PUSH)) != 0;
    set_bits(m, t + L::FLAGS_5B, L::CLIMB_BITS, !level && !pushed);
}

/// Con el modo de depuración, escribe en la capa fija empujones, X e Y de
/// las ruedas. Original: `Chain3_DebugHud_030554`.
pub fn debug_hud(m: &mut impl Memory, t: u32) {
    if m.read_u8(L::DEBUG) & L::DEBUG_WHEELS == 0 {
        return;
    }
    let [a, b] = [L::LINKS[0], L::LINKS[1]].map(|f| m.read_u32(t + f));
    let rows = [
        (a, L::PUSH), (b, L::PUSH),
        (a, L::X), (t, L::X), (b, L::X),
        (a, L::Y), (t, L::Y), (b, L::Y),
    ];
    for ((e, f), at) in rows.into_iter().zip(L::DEBUG_AT) {
        let v = m.read_u16(e + f);
        hud::put_hex4(m, at, v);
    }
}
