//! Slug (SV-001): la tarea fija del tanque y las tareas que lo sueltan en
//! la escena.
//!
//! Original: `Slug_IsAlive_02acfc`, `Slug_SpawnDrop_02ae50`,
//! `Slug_SpawnDropB_02aea4`, `Slug_Init_02a0f8` (con sus ayudantes) y la
//! entrada de `Slug_DropVariant0_02aef8` … `Slug_DropVariant4_02af88`,
//! la caída y la bajada hasta el suelo (`$02B004`, `Slug_DropDescend_02b05e`)
//! y la física del chasis sobre sus tres ruedas (`Slug_PhysicsAir_02a878`,
//! `Slug_GroundContact_02a8c0`, `Slug_TerrainSlope_02a958`,
//! `Slug_UpdateAnimAndChassis_02aa24`, `Slug_UpdateDamageSprite_02fae4`…).
//!
//! El Slug se apoya en tres ruedas (tareas de la cadena). Cada una sabe la
//! Y del suelo bajo ella; el chasis se coloca entre la delantera y la
//! trasera y se inclina según la pendiente que forman.

use super::{anim, collision, hits, motion, tasks};
use crate::entity::Entity;
use crate::input::{self, Buttons, Edge};
use crate::layout::{motion as M, slug as L, task as T};
use crate::mem::{Machine, Memory};
use crate::Flow;

/// ¿Está el Slug en marcha? (su tarea tiene una rutina que no es de libre,
/// baja, espera o destruido). Original: `Slug_IsAlive_02acfc` (C = sí).
pub fn is_alive(m: &mut impl Memory) -> bool {
    let h = m.read_u32(T::SLUG + T::HANDLER);
    !L::INACTIVE_HANDLERS.contains(&h)
}

/// Suelta el Slug (si no está ya en marcha) con la variante que pide el
/// byte `DROP_KIND` de la tarea actual, en su posición, y la tarea se da de
/// baja. Con `always_finish` (variante B) se da de baja aunque el Slug ya
/// estuviera. Original: `Slug_SpawnDrop_02ae50` / `Slug_SpawnDropB_02aea4`.
pub fn spawn_drop(m: &mut impl Machine, always_finish: bool) -> Flow {
    let t = m.task_register();
    if is_alive(m) {
        return if always_finish { tasks::finish(m) } else { Flow::Return };
    }
    let kind = m.read_u8(t + L::DROP_KIND).min(L::DROP_KIND_MAX);
    m.write_u32(T::SLUG + T::HANDLER, L::DROP_VARIANTS[kind as usize]);
    anim::copy_sprite_pos(m, t, T::SLUG);
    let y = m.read_u16(t + L::Y);
    m.write_u16(T::SLUG + L::DROP_Y, y);
    tasks::finish(m)
}

/// Prepara la tarea del Slug: gráficos, flags, temporizadores, daño a
/// cero, su tarea de cajas y la cadena de tres eslabones.
/// Original: `Slug_Init_02a0f8`.
pub fn init(m: &mut impl Memory, t: u32, gfx: u16) {
    anim::alloc_entry(m, t, gfx);
    anim::alloc_entry(m, t, L::EXTRA_GFX);
    m.write_u16(t + L::TIMER_1C, L::TIMER_1C_INIT);
    add_timer(m, t);
    let v = m.read_u16(t + L::FIELD_16);
    m.write_u16(t + L::FIELD_14, v);
    set_bits(m, t + L::FLAGS_12, 1 << 4);
    set_bits(m, t + L::FLAGS_5B, 1 << 2);
    set_bits(m, t + L::FLAGS_6B, 1 << 0);
    for l in L::LEVELS {
        m.write_u8(t + l, 0xFF);
    }
    m.write_u8(t + L::BYTE_3A, 0);
    m.write_u16(t + L::ATTR, L::ATTR_INIT);
    mark_timer_max(m, t);
    m.write_u8(t + L::BYTE_8C, 0);
    m.write_u32(t + L::HANDLER_60, L::HANDLER_60_INIT);
    mask_players(m, t);
    m.write_u16(t + L::FIELD_66, L::FIELD_66_INIT);
    reset_damage(m, t);
    for w in L::CLEAR_WORDS {
        m.write_u16(t + w, 0);
    }
    for b in L::CLEAR_BYTES {
        m.write_u8(t + b, 0);
    }
    m.write_u8(t + L::BYTE_90, L::BYTE_90_INIT);
    let f = m.read_u8(t + L::FLAGS_13);
    m.write_u8(t + L::FLAGS_13, f & !(1 << 3));
    let shadow = tasks::spawn(m, L::SHADOW_TASK, t);
    anim::copy_sprite_pos(m, t, shadow);
    build_chain(m, t);
    m.write_u32(t + L::FIELD_98_LONG, L::FIELD_98_INIT);
}

fn set_bits(m: &mut impl Memory, a: u32, bits: u8) {
    let v = m.read_u8(a);
    m.write_u8(a, v | bits);
}

/// Suma `$74` al temporizador `+$1C`. Original: `Entity_AddTimer0_74_0138FE`.
pub fn add_timer(m: &mut impl Memory, t: u32) {
    let v = m.read_u16(t + L::TIMER_1C).wrapping_add(L::TIMER_1C_ADD);
    m.write_u16(t + L::TIMER_1C, v);
}

/// Marca el bit 2 de `+$12` y pone el temporizador `+$30` al máximo.
/// Original: `Entity_MarkFlag2TimerMax_05e98a`.
pub fn mark_timer_max(m: &mut impl Memory, t: u32) {
    set_bits(m, t + L::FLAGS_12, 1 << 2);
    m.write_u16(t + L::TIMER_30, 0xFFFF);
}

/// Máscara de jugadores por defecto. Original: `PlayerSlot_MaskF0F0_08f6d2`.
pub fn mask_players(m: &mut impl Memory, t: u32) {
    m.write_u16(t + L::PLAYER_MASK, L::PLAYER_MASK_INIT);
}

/// Daño a cero. Original: `Slug_ResetDamageIdx_02fada`.
pub fn reset_damage(m: &mut impl Memory, t: u32) {
    m.write_u8(t + L::DAMAGE_IDX, 0);
    m.write_u16(t + L::DAMAGE, 0);
}

/// Crea tres tareas hijas a la altura `DROP_Y` y las enlaza en círculo
/// (cada una apunta a las tres). Devuelve la primera.
/// Original: `Entity_Build3ChainCircular_03060A`.
pub fn build_chain(m: &mut impl Memory, t: u32) -> u32 {
    for (routine, field) in L::CHAIN {
        let c = tasks::spawn(m, routine, t);
        anim::copy_sprite_pos(m, t, c);
        m.write_u32(t + field, c);
        let y = m.read_u16(t + L::DROP_Y);
        m.write_u16(c + L::Y, y);
        m.write_u8(c + L::CHAIN_KIND, 0);
    }
    // Enlaces: +$70 → segundo, +$74 → tercero, +$78 → primero.
    let links = [m.read_u32(t + 0x74), m.read_u32(t + 0x78), m.read_u32(t + 0x7C)];
    for c in [links[0], links[1], links[2]] {
        for (field, to) in L::CHAIN_LINKS.iter().zip(links) {
            m.write_u32(c + field, to);
        }
    }
    links[2]
}

/// Quita los bits 1 y 2 de `+$13`. Original: `Entity_ClearFlags13Bits12`.
pub fn clear_flags13(m: &mut impl Memory, t: u32) {
    let f = m.read_u8(t + L::FLAGS_13);
    m.write_u8(t + L::FLAGS_13, f & !0b110);
}

/// Entrada en escena con la variante `k` (0–4): prepara el Slug, su tarea
/// acompañante y las comunes, el guion y las cajas, y sigue en el bucle de
/// la caída (aún el original). Original: `Slug_DropVariant0_02aef8` …
/// `Slug_DropVariant4_02af88`.
pub fn drop_in(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    init(m, t, L::DROP_GFX);
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, a | L::ATTR_DROP);
    let c = tasks::spawn(m, L::DROP_COMPANIONS[k], t);
    anim::copy_sprite_pos(m, t, c);
    tasks::spawn(m, L::DROP_TASKS[0], t);
    let c = tasks::spawn(m, L::DROP_TASKS[1], t);
    anim::copy_sprite_pos(m, t, c);
    anim::start(m, t, L::DROP_ANIM);
    for w in L::DROP_CLEAR_WORDS {
        m.write_u16(t + w, 0);
    }
    for b in L::DROP_CLEAR_BYTES {
        m.write_u8(t + b, 0);
    }
    m.write_u32(t + L::HITBOX, L::DROP_HITBOX);
    clear_flags13(m, t);
    m.write_u32(t + T::HANDLER, L::DROP_FALL);
    Flow::Goto(L::DROP_FALL)
}

fn wheel(m: &mut impl Memory, t: u32, k: usize) -> u32 {
    m.read_u32(t + L::WHEELS[k])
}

fn set_bits8(m: &mut impl Memory, a: u32, bits: u8, on: bool) {
    let v = m.read_u8(a);
    m.write_u8(a, if on { v | bits } else { v & !bits });
}

/// Rueda (delantera o trasera) que ha chocado con una pared y sentido del
/// choque (1 delante, -1 detrás). Original: `$030704` (Z = ninguna).
pub fn wheel_against_wall(m: &mut impl Memory, t: u32) -> Option<(u32, i16)> {
    for (k, dir) in [(0, 1), (1, -1)] {
        let w = wheel(m, t, k);
        if m.read_u8(w + L::WHEEL_STATE) & L::WHEEL_HIT_WALL != 0 {
            return Some((w, dir));
        }
    }
    None
}

/// Bloquea el avance en X en el sentido `dir` durante este frame.
/// Original: `$0281B0`.
pub fn latch_x(m: &mut impl Memory, dir: u16) {
    m.write_u16(M::LATCH[0], dir);
}

/// Movimiento con colisión usando la Y de apoyo (`+$82`) en lugar de la
/// del sprite. Devuelve si ha tocado suelo (y entonces para la caída).
/// Original: `$027A18` (con `$027444`).
pub fn move_on_base(m: &mut impl Memory, t: u32, row: u16) -> bool {
    use crate::layout::collision as C;
    let x = m.read_u16(t + C::X);
    let y = m.read_u16(t + M::BASE_Y);
    m.write_u16(C::WORK_POS[0], x);
    m.write_u16(C::WORK_POS[1], y);
    for (w, f) in C::WORK_SUB.into_iter().zip([C::SUB_X, C::SUB_Y]) {
        let v = m.read_u8(t + f);
        m.write_u8(w, v);
    }
    let landed = move_and_stop(m, t, row);
    let (x, y) = (m.read_u16(C::WORK_POS[0]), m.read_u16(C::WORK_POS[1]));
    m.write_u16(t + C::X, x);
    m.write_u16(t + M::BASE_Y, y & M::BASE_Y_MASK);
    for (w, f) in C::WORK_SUB.into_iter().zip([C::SUB_X, C::SUB_Y]) {
        let v = m.read_u8(w);
        m.write_u8(t + f, v);
    }
    if landed {
        m.write_u16(t + C::ACC_Y, 0);
        m.write_u16(t + C::VEL_Y, 0);
    }
    landed
}

/// [`motion::move_and_collide`] y, si aterriza, para la caída.
/// Original: `$027444`.
pub fn move_and_stop(m: &mut impl Memory, t: u32, row: u16) -> bool {
    use crate::layout::collision as C;
    let landed = motion::move_and_collide(m, t, row);
    if landed {
        m.write_u16(t + C::ACC_Y, 0);
        m.write_u16(t + C::VEL_Y, 0);
    }
    landed
}

/// Física de un frame en el aire: tope contra paredes, movimiento con la
/// Y de apoyo y contacto con el suelo. Devuelve si está apoyado (y
/// actualiza el bit de «en el aire»). `row` = fila que deja el original
/// en `d4`. Original: `Slug_PhysicsAir_02a878`.
pub fn physics_air(m: &mut impl Machine, t: u32, row: u16) -> bool {
    if let Some((_, dir)) = wheel_against_wall(m, t) {
        latch_x(m, dir as u16);
    }
    let landed = move_on_base(m, t, row)
        || ((m.read_u16(t + L::VEL_Y) as i16) < 0 && !ground_contact(m, t).in_air);
    set_bits8(m, t + L::FLAGS_8D, L::AIRBORNE, !landed);
    landed
}

/// Resultado de [`ground_contact`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Contact {
    pub in_air: bool,
    /// Lo que deja el original en `d1`: ruedas que pisan (3 si pisa el
    /// propio chasis).
    pub wheels: i8,
}

/// ¿Pisa suelo el Slug? Primero bajo el chasis (en la Y de apoyo); si no,
/// cuenta como apoyado si pisan al menos dos ruedas, y entonces la Y de
/// apoyo pasa a ser la del sprite. Las ruedas se quedan con la Y en
/// pantalla. Original: `Slug_GroundContact_02a8c0` (C = en el aire).
pub fn ground_contact(m: &mut impl Machine, t: u32) -> Contact {
    let r = m.regs();
    let frac = (r.d[3] as u16, r.d[4] as u16);
    if m.read_u8(t + L::FLAGS_6B) & L::PLAIN_FLOOR != 0 {
        let f = collision::probe_floor(m, t, frac);
        let x = m.read_u16(t + L::X);
        return Contact { in_air: f.in_air, wheels: x as u8 as i8 };
    }
    let f5b = m.read_u8(t + L::FLAGS_5B);
    m.write_u8(t + L::FLAGS_5B, f5b & !L::ON_WHEELS);
    let (x, y) = (m.read_u16(t + L::X), m.read_u16(t + M::BASE_Y));
    let contact = if !collision::probe_at(m, t, x, y, frac).in_air {
        Contact { in_air: false, wheels: 3 }
    } else {
        let mut n = 0u8;
        for k in [2, 0, 1] {
            let w = wheel(m, t, k);
            n = n.wrapping_add(m.read_u8(w + L::WHEEL_ON_GROUND));
        }
        let in_air = (n as i8) < L::WHEELS_TO_LAND as i8;
        if !in_air {
            let f5b = m.read_u8(t + L::FLAGS_5B);
            m.write_u8(t + L::FLAGS_5B, f5b | L::ON_WHEELS);
            let y = m.read_u16(t + L::Y);
            m.write_u16(t + M::BASE_Y, y);
        }
        Contact { in_air, wheels: n as i8 }
    };
    for k in 0..3 {
        let w = wheel(m, t, k);
        let y = m.read_u16(w + L::WHEEL_Y);
        m.write_u16(w + L::WHEEL_Y, y & L::WHEEL_Y_MASK);
    }
    contact
}

/// Pendiente entre la rueda delantera y la trasera: (dx, dx·dy, dy con
/// tope ±`MAX_SLOPE`), como `d0`, `d1` y `d2` del original.
/// Original: `Slug_TerrainSlope_02a958`.
pub fn terrain_slope(m: &mut impl Memory, t: u32) -> (u16, u32, u16) {
    let (a, b) = (wheel(m, t, 0), wheel(m, t, 1));
    let dx = m.read_u16(a + L::WHEEL_X).wrapping_sub(m.read_u16(b + L::WHEEL_X));
    let dy = m.read_u16(a + L::WHEEL_GROUND).wrapping_sub(m.read_u16(b + L::WHEEL_GROUND));
    let prod = if dy == 0 { 0 } else { (i32::from(dy as i16) * i32::from(dx as i16)) as u32 };
    let dy = (dy as i16).clamp(-L::MAX_SLOPE, L::MAX_SLOPE) as u16;
    (dx, prod, dy)
}

/// Índice de animación según la pendiente (0 llano, 1/2 suave arriba o
/// abajo, 3/4 fuerte). Original: `Slug_SlopeToAnimIdx_02a9a0`.
pub fn slope_anim_idx(m: &mut impl Memory, t: u32) -> u16 {
    let p = terrain_slope(m, t).1 as u16 as i16;
    match p {
        p if p >= L::STEEP_SLOPE => 3,
        p if p <= -L::STEEP_SLOPE => 4,
        p if p >= L::GENTLE_SLOPE => 1,
        p if p <= -L::GENTLE_SLOPE => 2,
        _ => 0,
    }
}

/// Animación y chasis de un frame: piloto, sprite de daños, índice por la
/// pendiente, contador y la Y del chasis entre las dos ruedas.
/// Original: `Slug_UpdateAnimAndChassis_02aa24`.
pub fn update_anim(m: &mut impl Memory, t: u32) {
    clear_flags13(m, t);
    let pilot = m.read_u8(t + L::PILOT);
    if pilot <= 1 {
        m.write_u8(t + L::LAST_PILOT, pilot);
    }
    m.write_u8(t + L::TURN_TIMER, 0);
    if m.read_u8(t + L::FLAGS_13) & L::NO_DAMAGE_SPRITE == 0 {
        damage_anim(m, t);
    }
    let idx = slope_anim_idx(m, t);
    m.write_u16(t + L::ANIM_IDX, idx);
    let p = m.read_u8(t + L::PULSE);
    m.write_u8(t + L::PULSE, p.saturating_sub(1));
    let y = chassis_y(m, t);
    m.write_u16(t + L::Y, y);
}

/// Y del chasis: la del suelo de la rueda más baja más la mitad de lo que
/// sube la recta entre las dos ruedas (en 1/32).
fn chassis_y(m: &mut impl Memory, t: u32) -> u16 {
    let (a, b) = (wheel(m, t, 0), wheel(m, t, 1));
    let (ga, gb) = (m.read_u16(a + L::WHEEL_GROUND), m.read_u16(b + L::WHEEL_GROUND));
    let d = ga.wrapping_sub(gb) as i16;
    let (base, neg) = if d < 0 { (ga, true) } else { (gb, false) };
    let num = d.wrapping_shl(5);
    let dx = m.read_u16(a + L::WHEEL_X).wrapping_sub(m.read_u16(b + L::WHEEL_X)) as i16;
    let mut q = if dx == 0 {
        0
    } else {
        // `divs.w`: si el cociente no cabe en 16 bits no cambia nada.
        let q = i32::from(num) / i32::from(dx);
        if (-0x8000..=0x7FFF).contains(&q) { q as i16 } else { num }
    };
    q >>= 1;
    if neg {
        q = q.wrapping_neg();
    }
    (q as u16).wrapping_add(base)
}

/// Como [`update_anim`] pero sin cambiar el índice de animación.
/// Original: `Slug_UpdateAnimKeepIdx_02aa0e`.
pub fn update_anim_keep_idx(m: &mut impl Memory, t: u32) {
    let idx = m.read_u16(t + L::ANIM_IDX);
    update_anim(m, t);
    m.write_u16(t + L::ANIM_IDX, idx);
}

/// Guion del sprite de daños (sólo con la vida a 1 o menos): órdenes de
/// 4 bytes — 0 = poner el bit de daños y esperar, 1 = copiar un campo a
/// `+$14`, 2 = saltar, 3 = fin. Devuelve si ha llegado al fin.
/// Original: `Slug_UpdateDamageSprite_02fae4`.
pub fn damage_anim(m: &mut impl Memory, t: u32) -> bool {
    if m.read_u16(t + L::FIELD_66) > L::DAMAGE_HP {
        return false;
    }
    let script = L::DAMAGE_SCRIPT;
    loop {
        let at = script.wrapping_add(m.read_u16(t + L::DAMAGE) as i16 as u32);
        let skip = u16::from(m.read_u8(at + 1));
        match m.read_u8(at) {
            0 => {
                let f = m.read_u8(t + L::FLAGS_12) & !L::DAMAGE_BIT;
                let bits = m.read_u8(at + 3);
                m.write_u8(t + L::FLAGS_12, f | bits);
                let n = m.read_u8(t + L::DAMAGE_IDX).wrapping_add(1);
                m.write_u8(t + L::DAMAGE_IDX, n);
                if (n as i8) >= (m.read_u8(at + 2) as i8) {
                    m.write_u8(t + L::DAMAGE_IDX, 0);
                    let i = m.read_u16(t + L::DAMAGE);
                    m.write_u16(t + L::DAMAGE, i.wrapping_add(skip));
                }
                return false;
            }
            1 => {
                let field = m.read_u16(at + 2) as i16 as u32;
                let v = m.read_u16(t.wrapping_add(field));
                m.write_u16(t + L::FIELD_14, v);
                let i = m.read_u16(t + L::DAMAGE);
                m.write_u16(t + L::DAMAGE, i.wrapping_add(skip));
            }
            2 => {
                let to = m.read_u16(at + 2);
                m.write_u16(t + L::DAMAGE, to);
            }
            _ => return true,
        }
    }
}

/// Apunta en `+$12` si se está pulsando izquierda o derecha.
/// Original: `Slug_UpdateAirFlag_02a478`.
pub fn update_steering(m: &mut impl Memory, t: u32) {
    let on = input::pressed(m, Entity::from_raw(t), Buttons::LEFT_OR_RIGHT, Edge::Held);
    set_bits8(m, t + L::FLAGS_12, L::STEERING, on);
}

fn air_attr(m: &mut impl Memory, t: u32) {
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, (a & L::AIR_ATTR_KEEP) | L::AIR_ATTR);
}

/// Fila que deja en `d4` la medida de la pendiente (suelo de la rueda
/// trasera), con la que se llama a la física.
fn rear_ground(m: &mut impl Memory, t: u32) -> u16 {
    let b = wheel(m, t, 1);
    m.read_u16(b + L::WHEEL_GROUND)
}

/// Caída desde lo alto: invulnerable, velocidad fija, animación y golpes;
/// cuando baja de `DESCEND_Y` o toca suelo pasa a la bajada.
/// Original: `Slug_DropVariant4_02af88__L02b004`.
pub fn fall_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u8(t + L::INVULNERABLE, L::DROP_INVULNERABLE);
    update_anim_keep_idx(m, t);
    m.write_u16(t + L::VEL_Y, L::DROP_VEL_Y);
    let row = rear_ground(m, t);
    physics_air(m, t, row);
    update_steering(m, t);
    air_attr(m, t);
    anim::step(m, t);
    hits::run_attack_list(m, t);
    if (m.read_u16(t + L::Y) as i16) <= L::DESCEND_Y {
        m.write_u32(t + T::HANDLER, L::DROP_DESCEND);
    }
    if ground_contact(m, t).in_air {
        return Flow::Return;
    }
    descend(m)
}

/// Empieza la bajada hasta el suelo. Original: `Slug_DropDescend_02b05e`.
pub fn descend(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::DROP_ANIM);
    let base = m.read_u16(t + M::BASE_Y).wrapping_add(L::DESCEND_STEP);
    m.write_u16(t + M::BASE_Y, base);
    let y = m.read_u16(t + L::Y);
    if (y as i16) > (base as i16) {
        m.write_u16(t + M::BASE_Y, y);
    }
    let angle = terrain_slope(m, t).2;
    m.write_u16(t + L::ANGLE, angle);
    update_steering(m, t);
    m.write_u16(t + L::VEL_Y, L::DROP_VEL_Y);
    m.write_u32(t + T::HANDLER, L::DESCEND_RUN);
    descend_run(m)
}

/// Bajada: hasta apoyarse (y pasar al estado parado) inclina el sprite con
/// la pendiente y, si pisan ruedas, para el avance.
/// Original: `Slug_DropDescend_02b05e__L02b09e`.
pub fn descend_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u8(t + L::INVULNERABLE, L::DESCEND_INVULNERABLE);
    set_bits8(m, t + L::FLAGS_8D, L::AIRBORNE, true);
    update_anim_keep_idx(m, t);
    let vy = collision::clamp_velocity(m.read_u16(t + L::VEL_Y), L::MAX_FALL).0;
    m.write_u16(t + L::VEL_Y, vy);
    let row = rear_ground(m, t);
    if physics_air(m, t, row) {
        m.write_u32(t + T::HANDLER, L::IDLE_ENTER);
    }
    update_steering(m, t);
    air_attr(m, t);
    anim::step(m, t);
    hits::run_attack_list(m, t);
    if (m.read_u16(t + L::VEL_Y) as i16) > 0 {
        return Flow::Return;
    }
    if ground_contact(m, t).wheels > 0 {
        m.write_u16(t + L::VEL_X, 0);
        m.write_u16(t + L::ACC_X, 0);
    }
    tilt(m, t);
    Flow::Return
}

/// Si ha cambiado la pendiente, pone el sprite inclinado que le toca.
fn tilt(m: &mut impl Memory, t: u32) {
    let angle = terrain_slope(m, t).2;
    if angle == m.read_u16(t + L::ANGLE) {
        return;
    }
    m.write_u16(t + L::ANGLE, angle);
    let k = (L::ANGLE_CENTER - ((angle as i16) >> 1)) as u16;
    let k = k.min(L::ANGLE_MAX);
    let sprite = angle_sprite(m, k);
    let script = m.read_u32(L::ANGLE_SCRIPTS.wrapping_add((sprite << 2) as i16 as u32));
    if script != L::NO_SCRIPT {
        anim::start(m, t, script);
    }
    m.write_u8(t + L::BYTE_20, L::BYTE_20_TILT);
}

/// Índice de sprite para la inclinación `k` (0–`ANGLE_MAX`).
/// Original: `Slug_AngleToSpriteIdx_0295a6`.
pub fn angle_sprite(m: &mut impl Memory, k: u16) -> u16 {
    m.read_u8(L::ANGLE_SPRITES.wrapping_add(u32::from(k))) as i8 as i16 as u16
}

/// La tarea `t` se pone donde su dueño (atributos y posición) y se marca
/// como recién colocada. Original: `$05E4B2` (con `$05E4CA`).
pub fn follow_owner(m: &mut impl Memory, t: u32) -> (u16, u16, u16) {
    let f = m.read_u8(t + T::FLAGS2);
    m.write_u8(t + T::FLAGS2, f | T::FLAGS2_FRESH);
    let p = owner_pos(m, t);
    m.write_u16(t + L::ATTR, p.0);
    m.write_u16(t + L::X, p.1);
    m.write_u16(t + L::Y, p.2);
    p
}

/// Atributos y posición del dueño de `t`. Original: `$05E4CA`.
pub fn owner_pos(m: &mut impl Memory, t: u32) -> (u16, u16, u16) {
    let o = m.read_u32(t + T::OWNER);
    (m.read_u16(o + L::ATTR), m.read_u16(o + L::X), m.read_u16(o + L::Y))
}

/// Alta de las orugas: sus cuatro sprites y al bucle.
/// Original: `Slug_WheelAnim_02fb92__L02fb9a`.
pub fn track_start(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::TRACK_FRAME, 0);
    for g in L::TRACK_GFX {
        anim::alloc_entry(m, t, g);
    }
    m.write_u32(t + T::HANDLER, L::TRACK_RUN);
    track_run(m)
}

/// Orugas: mientras el Slug está tocado (vida baja) y el guion de daños
/// lo pide, avanzan un fotograma y siguen al chasis; si el Slug ya no
/// tiene sprite de daños pasan al bucle simple. Cuando el Slug queda
/// destruido, se dan de baja. Original: `$02FBCC`.
pub fn track_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let o = m.read_u32(t + T::OWNER);
    if m.read_u8(o + L::FLAGS_13) & L::NO_DAMAGE_SPRITE != 0 {
        m.write_u32(t + T::HANDLER, L::TRACK_RUN_PLAIN);
        m.write_u16(t + L::TRACK_FRAME, 0);
        track_step(m, t);
    } else if m.read_u16(o + L::FIELD_66) <= L::TRACK_MAX_HP {
        track_step(m, t);
    }
    track_end(m, t)
}

/// Orugas con el Slug sin sprite de daños. Original: `$02FC6A`.
pub fn track_run_plain(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    track_step(m, t);
    track_end(m, t)
}

fn track_step(m: &mut impl Machine, t: u32) {
    let o = m.read_u32(t + T::OWNER);
    if m.read_u8(o + L::FLAGS_12) & L::DAMAGE_BIT == 0 {
        return;
    }
    let f = m.read_u16(t + L::TRACK_FRAME).wrapping_add(2) & L::TRACK_FRAME_MASK;
    m.write_u16(t + L::TRACK_FRAME, f);
    let v = m.read_u16(t + L::TRACK_FIELDS[usize::from(f / 2)]);
    m.write_u16(t + L::FIELD_14, v);
    let cursor = if m.read_u16(o + L::LAST_SPRITE) == L::NO_SPRITE {
        m.read_u32(o + L::SCRIPT_CURSOR)
    } else {
        m.read_u32(o + L::LAST_SPRITE)
    };
    m.write_u32(t + L::SCRIPT_CURSOR, cursor);
    m.write_u32(t + L::LAST_SPRITE, cursor);
    follow_owner(m, t);
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, a.wrapping_sub(L::TRACK_DEPTH));
    anim::step(m, t);
}

fn track_end(m: &mut impl Machine, t: u32) -> Flow {
    let o = m.read_u32(t + T::OWNER);
    if m.read_u32(o + T::HANDLER) == L::DESTROYED {
        return tasks::finish(m);
    }
    Flow::Return
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::TestRam;

    #[test]
    fn alive_unless_inactive_handler() {
        let mut m = TestRam::default();
        for h in L::INACTIVE_HANDLERS {
            m.write_u32(T::SLUG + T::HANDLER, h);
            assert!(!is_alive(&mut m));
        }
        m.write_u32(T::SLUG + T::HANDLER, L::DROP_VARIANTS[0]);
        assert!(is_alive(&mut m));
    }
}
