//! Prisionero (POW) rescatable, el barbudo atado que da un objeto al
//! rescatarlo. Tres familias: el que deambula (busca al jugador, se gira,
//! se aparta o espera; al tocarlo se gira hacia él, saluda, da el objeto y
//! se va), el que ya está libre (sale andando o saltando, da las gracias y
//! se marcha) y el atado a un poste o colgado (se balancea, forcejea y se
//! suelta cuando le dan a la cuerda). Más sus efectos: el chispazo al
//! recibir un golpe, el sprite que acompaña al saludo y el objeto lanzado.
//!
//! Original: `asm/pow/prisoner_0478fc.s` ($0478FC–$048A3C) y
//! `asm/pow/helpers_048a44.s` ($048A44–$049470).
//!
//! Muchos ayudantes del original dejan en `a0` el jugador que han mirado y
//! los siguientes lo usan sin volver a cargarlo; aquí `a0` se guarda en los
//! registros de la máquina igual que en el original.

use super::{anim, banner, collision, explosion, fort, hits, human_death, items, manual_icons, motion, pickup, platforms};
use super::{player_death, player_ground, rng, slug, soldier, sound, tasks, trig, video};
use crate::layout::{collision as C, prisoner as L, task as T};
use crate::mem::{Flow, Machine, Memory};

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

fn a0(m: &mut impl Machine) -> u32 {
    m.regs().a[0]
}

fn set_a0(m: &mut impl Machine, v: u32) {
    let mut r = m.regs();
    r.a[0] = v;
    m.set_regs(r);
}

fn at(m: &mut impl Machine) -> (u16, u16) {
    let r = m.regs();
    (r.d[3] as u16, r.d[4] as u16)
}

fn x(m: &mut impl Memory, t: u32) -> i16 {
    m.read_u16(t + L::X) as i16
}

fn facing_right(m: &mut impl Memory, t: u32) -> bool {
    m.read_u8(t + L::FACING) & 1 != 0
}

fn add8(m: &mut impl Memory, a: u32, v: u8) -> u8 {
    let n = m.read_u8(a).wrapping_add(v);
    m.write_u8(a, n);
    n
}

fn add16(m: &mut impl Memory, a: u32, v: u16) -> u16 {
    let n = m.read_u16(a).wrapping_add(v);
    m.write_u16(a, n);
    n
}

/// Prioridad de sprite: `ATTR` = (`ATTR` & `$FFE3`) | `p`.
fn set_priority(m: &mut impl Memory, t: u32, p: u16) {
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, (a & L::ATTR_KEEP) | p);
}

fn difficulty(m: &mut impl Memory, t: u32, table: u32) -> u16 {
    soldier::by_difficulty(m, t, table)
}

// ---------------------------------------------------------------------------
// Rutinas de la ROM que sólo usa el prisionero.
// ---------------------------------------------------------------------------

/// Ángulo (1/256 de vuelta) de `t` hacia `p`. Como el original, deja en
/// `a0` la tabla de exponentes si ha tenido que usarla.
/// Original: `$05E070` (con `Atan2_Angle256_05e018`).
fn angle_to(m: &mut impl Machine, t: u32, p: u32) -> u8 {
    let dx = m.read_u16(p + L::X).wrapping_sub(m.read_u16(t + L::X));
    let dy = m.read_u16(p + L::Y).wrapping_sub(m.read_u16(t + L::Y));
    let fold = |v: u16| {
        let h = (v as i16) >> 1;
        (h as u8) ^ if h < 0 { 0xFF } else { 0 }
    };
    if fold(dx).min(fold(dy)) != 0 {
        set_a0(m, L::ATAN_EXP);
    }
    banner::atan2(m, dx, dy)
}

/// ¿Ha perdido al jugador `p` (ya no está en juego, o no es un jugador)?
/// Deja en `a0` su tarea (o `NIL`). Original: `$05E338` (C = perdido).
fn target_lost(m: &mut impl Machine, p: u32) -> bool {
    let Some(k) = L::PLAYER_ENTITIES.iter().position(|&e| e == p) else {
        return true;
    };
    let task = tasks::player_task_if_alive(m, k as u32);
    set_a0(m, task.unwrap_or(L::NIL));
    task.is_none()
}

/// ¿Está el jugador `p` a la izquierda de `t` mirando a la izquierda, o a
/// la derecha mirando a la derecha? Original: `$05E618` (C = sí).
fn facing_target(m: &mut impl Memory, t: u32, p: u32) -> bool {
    let d = m.read_u16(t + L::X).wrapping_sub(m.read_u16(p + L::X)) as i16;
    if facing_right(m, t) {
        d > 0
    } else {
        d < 0
    }
}

/// ¿Es `p` un jugador montado / listo (bit 6 de `+$8D`)?
/// Original: `Player_IsSlotRidingReady_032de0`.
fn player_ready(m: &mut impl Memory, p: u32) -> bool {
    L::PLAYER_ENTITIES.contains(&p) && m.read_u8(p + L::PLAYER_FLAGS) & L::PLAYER_READY != 0
}

/// ¿Ha muerto la tarea madre? Original: `$05E45A` (C = sí).
fn parent_dead(m: &mut impl Memory, t: u32) -> bool {
    let p = m.read_u32(t + T::OWNER);
    L::DEAD_HANDLERS.contains(&m.read_u32(p))
}

/// Se pone donde la madre: posición, profundidad, giro y atributos.
/// Original: `$05E506`.
fn follow_parent(m: &mut impl Machine, t: u32) -> u32 {
    let p = m.read_u32(t + T::OWNER);
    for f in [L::X, L::Y] {
        let v = m.read_u16(p + f);
        m.write_u16(t + f, v);
    }
    for f in L::SPRITE_Z.into_iter().chain([L::FACING]) {
        let v = m.read_u8(p + f);
        m.write_u8(t + f, v);
    }
    let v = m.read_u16(p + L::ATTR);
    m.write_u16(t + L::ATTR, v);
    set_a0(m, p);
    p
}

/// Jugador más cercano en X (en `a0`; si no hay, el 2).
/// Original: `$05E0D4` (C = ninguno).
fn nearest(m: &mut impl Machine, t: u32) -> Option<u32> {
    let p = pickup::nearest_player_x(m, t);
    set_a0(m, p.unwrap_or(L::PLAYER_ENTITIES[1]));
    p
}

/// Jugador dentro de la caja `bbox` (en `a0`; si no hay, el 2).
/// Original: `$05E086` (C = ninguno).
fn player_in_box(m: &mut impl Machine, t: u32, bbox: u32) -> Option<u32> {
    let p = pickup::player_in_box(m, t, bbox);
    set_a0(m, p.unwrap_or(L::PLAYER_ENTITIES[1]));
    p
}

/// Un jugador en juego al azar como objetivo. Original: `$05E1EA`.
fn retarget_random(m: &mut impl Machine, t: u32) {
    let p = tasks::pick_random_alive(m).1;
    set_a0(m, p);
    m.write_u32(t + L::TARGET, p);
}

/// Objetivo = el jugador más cercano. `$05E0D4` y `move.l a0,+$94`.
fn target_nearest(m: &mut impl Machine, t: u32) {
    nearest(m, t);
    let p = a0(m);
    m.write_u32(t + L::TARGET, p);
}

// ---------------------------------------------------------------------------
// Movimiento y sondeo del suelo.
// ---------------------------------------------------------------------------

/// Sondeo del suelo dejando los registros como el original (`$027EBA`).
/// Devuelve si está en el aire.
pub(crate) fn probe(m: &mut impl Machine, t: u32) -> bool {
    let mut r = m.regs();
    let f = collision::probe_floor(m, t, (r.d[3] as u16, r.d[4] as u16));
    r.a[1] = f.hit.unwrap_or(f.record);
    r.d[3] = (r.d[3] & 0xFFFF_0000) | u32::from(f.at.0);
    r.d[4] = (r.d[4] & 0xFFFF_0000) | u32::from(f.at.1);
    m.set_regs(r);
    f.in_air
}

pub(crate) fn move_ground(m: &mut impl Machine, t: u32) {
    let mut r = m.regs();
    r.a[5] = C::SCRATCH_BASE;
    m.set_regs(r);
    player_ground::move_ground(m, t);
}

/// Sigue a la escena y, si no pisa, cae. Original: `Pow_ScrollAndProbe_048fb0`.
fn scroll_and_probe(m: &mut impl Machine, t: u32) {
    platforms::move_with_scene(m, t);
    if probe(m, t) {
        collision::fall(m, t);
    }
}

/// Sigue a la escena y anda por el suelo o cae.
/// Original: `Pow_ScrollProbeOrFall_048f2e`.
fn scroll_walk(m: &mut impl Machine, t: u32) {
    platforms::move_with_scene(m, t);
    if probe(m, t) {
        collision::fall(m, t);
    } else {
        move_ground(m, t);
    }
}

/// Paso con colisión (`$027CEE`).
fn move_probe(m: &mut impl Machine, t: u32) -> bool {
    let at = at(m);
    motion::on_entity_pos(m, t, |m| motion::move_e_probe(m, t, at))
}

/// Paso que se para al tocar suelo (`$027BC8`).
fn move_stop(m: &mut impl Machine, t: u32) -> bool {
    let row = at(m).1;
    motion::on_entity_pos(m, t, |m| slug::move_and_stop(m, t, row))
}

/// «Golpe recibido»: en realidad mira si está en el aire (y si no, borra
/// `+$75`). Original: `Pow_HitReceivedCheck_04932c`.
fn hit_received(m: &mut impl Machine, t: u32) -> bool {
    if probe(m, t) {
        return true;
    }
    m.write_u8(t + L::AIR, 0);
    false
}

/// Velocidad de carrera hacia donde mira y su sprite.
/// Original: `Pow_SetRunVelAndSprite_048f04`.
fn set_run_vel(m: &mut impl Memory, t: u32) {
    let s = m.read_u16(t + L::SPEED);
    let v = if facing_right(m, t) { s } else { s.wrapping_neg() };
    m.write_u16(t + L::VEL_X, v);
    let k = u32::from((s >> 7) & 0xC);
    let sp = m.read_u32(L::RUN_SPRITES + k);
    m.write_u32(t + L::SPRITE_DATA, sp);
}

/// Preparación del prisionero suelto: quieto, mirando al centro, gráficos,
/// caja, sprite, espera y prioridad. Original: `Pow_FreeInit_048ea6`.
fn free_init(m: &mut impl Machine, t: u32) {
    manual_icons::stop_motion(m, t);
    if x(m, t) <= L::LEFT_HALF {
        let f = m.read_u8(t + L::FACING);
        m.write_u8(t + L::FACING, f | 1);
    }
    human_death::alloc_regs(m, t, L::GFX);
    m.write_u16(t + L::HP, 1);
    m.write_u32(t + L::HITBOX, L::BODY);
    m.write_u32(t + L::SPRITE_DATA, L::STILL_SPRITE);
    let w = difficulty(m, t, L::WAIT_TIME);
    m.write_u16(t + L::TIMER, w);
    anim::set_attr(m, t, L::ATTR_BASE);
    set_priority(m, t, L::PRIORITY);
}

// ---------------------------------------------------------------------------
// Decisiones (todas devuelven lo que el original deja en C).
// ---------------------------------------------------------------------------

/// |X de `t` − X de `p`|. Original: `Pow_AbsDistX_049388`.
fn abs_dx(m: &mut impl Memory, t: u32, p: u32) -> i16 {
    (m.read_u16(t + L::X).wrapping_sub(m.read_u16(p + L::X)) as i16).wrapping_abs()
}

/// ¿Está `p` lejos en vertical (más de $18 por debajo o $10 por encima)?
/// Original: `Pow_TargetYNear_04939c` / `Pow_TargetYNearB_0493c0`.
fn y_far(m: &mut impl Memory, t: u32, p: u32) -> bool {
    let d = m.read_u16(p + L::Y).wrapping_sub(m.read_u16(t + L::Y)) as i16;
    d > L::NEAR_Y.1 || d < L::NEAR_Y.0
}

/// ¿Está el objetivo en la caja de la variante (`+$99`)?
/// Original: `Pow_TargetInBox_049128`.
fn target_in_box(m: &mut impl Machine, t: u32) -> bool {
    let k = u32::from(m.read_u8(t + L::BOX) & 3);
    let bbox = m.read_u32(L::BOXES + k * 4);
    let p = m.read_u32(t + L::TARGET);
    let mut r = m.regs();
    r.a[0] = p;
    r.a[1] = bbox;
    m.set_regs(r);
    pickup::in_box(m, t, p, bbox)
}

/// ¿Cae el ángulo hacia el objetivo justo en un múltiplo de la máscara?
/// Original: `Pow_TargetAngleInMask_04914c`.
fn angle_in_mask(m: &mut impl Machine, t: u32) -> bool {
    let p = m.read_u32(t + L::TARGET);
    set_a0(m, p);
    let a = angle_to(m, t, p);
    let d1 = a.wrapping_add(m.read_u8(t + L::ITEM_STEP)) & m.read_u8(t + L::ITEM_MASK);
    d1 == a
}

/// ¿Tiene al objetivo a mano? Original: `Pow_TargetInReach_0490fa`.
fn target_in_reach(m: &mut impl Machine, t: u32) -> bool {
    if nearest(m, t).is_none() {
        return true;
    }
    if !target_in_box(m, t) {
        return false;
    }
    let p = a0(m);
    if !y_far(m, t, p) {
        return true;
    }
    angle_in_mask(m, t)
}

/// ¿Está el objetivo fuera de la caja y a más de $C0 en X?
/// Original: `Pow_TargetFarX_049172`.
fn target_far_x(m: &mut impl Machine, t: u32) -> bool {
    if target_in_box(m, t) {
        return false;
    }
    let p = m.read_u32(t + L::TARGET);
    set_a0(m, p);
    abs_dx(m, t, p) > L::FAR_X
}

/// Distancia `k` de la tabla de la variante (`+$9A`).
fn range(m: &mut impl Memory, t: u32, k: u32) -> i16 {
    let v = u32::from(m.read_u8(t + L::RANGE_SET) & 3) * 8;
    m.read_u16(L::RANGES + v + k * 2) as i16
}

/// Jugador cerca (caja, altura y distancia `k` de la tabla).
fn player_close(m: &mut impl Machine, t: u32, k: u32, check_y: bool) -> bool {
    let Some(p) = player_in_box(m, t, L::BOX_NEAR) else {
        return false;
    };
    if check_y && y_far(m, t, p) {
        return false;
    }
    abs_dx(m, t, p) < range(m, t, k)
}

/// Original: `Pow_ShouldRunAway_049196`.
fn should_run_away(m: &mut impl Machine, t: u32) -> bool {
    !at_screen_edge(m, t) && player_close(m, t, 0, true)
}

/// Original: `Pow_ShouldWait_0491de`.
fn should_wait(m: &mut impl Machine, t: u32) -> bool {
    player_close(m, t, 1, true)
}

/// Original: `Pow_TargetNearX_04921e`.
fn target_near_x(m: &mut impl Machine, t: u32) -> bool {
    player_close(m, t, 2, false)
}

/// Cuenta hasta poder girarse y entonces mira si debe.
/// Original: `Pow_TurnTimerAndCheck_049256`.
fn turn_timer(m: &mut impl Machine, t: u32) -> bool {
    if (add8(m, t + L::TURN_WAIT, 1) as i8) < L::TURN_DELAY {
        return false;
    }
    m.write_u8(t + L::TURN_WAIT, L::TURN_DELAY as u8);
    should_turn(m, t)
}

/// Original: `Pow_ShouldTurn_04926a`.
fn should_turn(m: &mut impl Machine, t: u32) -> bool {
    // Con un jugador en la caja A no se gira; en la B, sí.
    if player_in_box(m, t, L::BOX_TURN_A).is_some() {
        return false;
    }
    if player_in_box(m, t, L::BOX_TURN_B).is_some() {
        return true;
    }
    let p = m.read_u32(t + L::TARGET);
    set_a0(m, p);
    facing_target(m, t, p)
}

/// ¿Hay un jugador a su altura a menos de `d` en X?
/// Original: `Pow_TargetWithin30_0492a4` / `Pow_TargetWithin60_0492ce`.
fn target_within(m: &mut impl Machine, t: u32, d: i16) -> bool {
    let Some(p) = nearest(m, t) else {
        return false;
    };
    !y_far(m, t, p) && abs_dx(m, t, p) <= d
}

/// ¿Está en un borde de la pantalla mirando hacia fuera?
/// Original: `Pow_AtScreenEdge_0492f8`.
fn at_screen_edge(m: &mut impl Memory, t: u32) -> bool {
    let x = x(m, t);
    let right = m.read_u8(t + L::LOOK) & 1 != 0;
    (x <= L::EDGE_X.0 && !right) || (x >= L::EDGE_X.1 && right)
}

/// ¿Lleva atascado contra una pared? Original: `Pow_BlockedTimer_049346`.
fn blocked(m: &mut impl Memory, t: u32) -> bool {
    if m.read_u8(t + L::FLAGS3) & L::BLOCKED_BIT != 0 && (add8(m, t + L::BLOCKED, 1) as i8) >= L::BLOCK_DELAY {
        return true;
    }
    m.write_u8(t + L::BLOCKED, 0);
    false
}

/// Si ha perdido al objetivo, elige otro. Original: `Pow_RetargetIfLost_04936e`.
fn retarget_if_lost(m: &mut impl Machine, t: u32) {
    let p = m.read_u32(t + L::TARGET);
    set_a0(m, p);
    if target_lost(m, p) {
        retarget_random(m, t);
    }
}

/// Animación de espera (0 = ninguna, 1, 2). Original: `Pow_PickIdleSpriteIdx_048fcc`.
fn pick_idle(m: &mut impl Machine, t: u32) -> u16 {
    if target_in_reach(m, t) {
        return 0;
    }
    if m.read_u8(t + L::ITEM_MASK) == L::ROUND_MASK || !target_in_box(m, t) {
        return 1;
    }
    angle_in_mask(m, t);
    if rng::next(m).0 & 1 == 0 {
        1
    } else {
        2
    }
}

/// ¿Se le puede rescatar ya? Original: `Pow_CanBeRescued_049010`.
fn can_be_rescued(m: &mut impl Machine, t: u32) -> bool {
    if (m.read_u8(t + L::TURNS) as i8) <= L::MAX_TURNS {
        if m.read_u16(t + L::TIMER) as i16 > 0 {
            return false;
        }
        m.write_u16(t + L::TIMER, 0);
        if !target_in_box(m, t) {
            return false;
        }
    }
    if m.read_u8(t + L::WALKS) == 0 {
        return true;
    }
    let p = a0(m);
    if !y_far(m, t, p) {
        return true;
    }
    angle_in_mask(m, t)
}

/// Atado: se gira poco a poco hacia el objetivo; C cuando ya lo mira y
/// ha acabado la espera. Original: `Pow_TiedTurnTowardTarget_049054` y
/// `Pow_TiedTurnStep_0490e2`.
fn tied_turn(m: &mut impl Machine, t: u32) -> bool {
    let p = m.read_u32(t + L::TARGET);
    set_a0(m, p);
    let a = angle_to(m, t, p);
    let d0 = ((u16::from(a) + 8) >> 4) & 0xF;
    let steps = m.read_u16(t + L::STEPS);
    let dir: i16 = if d0 == 0 {
        cmp_steps(16, steps)
    } else if d0 < 4 {
        1
    } else if d0 < 8 {
        -1
    } else {
        cmp_steps((d0 - 8) * 2, steps)
    };
    match dir {
        0 => {
            if m.read_u16(t + L::TIMER) as i16 > 0 {
                return false;
            }
            m.write_u16(t + L::TIMER, 0);
            true
        }
        1 if steps == L::TURN_FULL => false,
        -1 if steps == 0 => false,
        d => {
            if m.read_u8(L::FRAME_COUNTER) & 3 == 0 {
                add16(m, t + L::STEPS, d as u16);
            }
            false
        }
    }
}

/// Paso hacia `goal` desde `steps`: 0 = ya está, 1 = sube, -1 = baja.
fn cmp_steps(goal: u16, steps: u16) -> i16 {
    match (goal as i16).cmp(&(steps as i16)) {
        core::cmp::Ordering::Equal => 0,
        core::cmp::Ordering::Greater => 1,
        core::cmp::Ordering::Less => -1,
    }
}

/// Balanceo del atado: ángulo `+$7F` (de 2 en 2), velocidad del péndulo,
/// paso con colisión y, si está en el poste, si pisa firme (si no, si
/// pisa suelo). Original: `Pow_TiedSwingStep_048f54` (C = pisa).
fn swing(m: &mut impl Machine, t: u32) -> bool {
    let a = m.read_u8(t + L::SWING).wrapping_sub(L::SWING_STEP);
    m.write_u8(t + L::SWING, a);
    let (d1, d2) = trig::polar(m, u16::from(a), L::SWING_RADIUS);
    let vx = (d1 as u16).wrapping_add(m.read_u16(t + L::SWING_X));
    m.write_u16(t + L::VEL_X, vx);
    let mut d2 = (d2 as u16 as i16) >> 1;
    if d2 > 0 {
        d2 = -d2;
    }
    let vy = (d2 as u16).wrapping_add(L::SWING_LIFT).wrapping_sub(m.read_u16(t + L::SWING_Y));
    m.write_u16(t + L::VEL_Y, vy);
    move_probe(m, t);
    if m.read_u8(t + L::ON_POLE) != 0 {
        let at = at(m);
        ground_fx_on(m, t, at)
    } else {
        !probe(m, t)
    }
}

fn ground_fx_on(m: &mut impl Machine, t: u32, at: (u16, u16)) -> bool {
    super::ground_fx::solid_at_feet(m, t, at).0
}

// ---------------------------------------------------------------------------
// Colas comunes de cada frame.
// ---------------------------------------------------------------------------

/// Chispazo del golpe encima del prisionero (a `dy` píxeles).
fn spawn_burst(m: &mut impl Machine, t: u32, dy: u16) -> u32 {
    let c = tasks::spawn(m, L::BURST, t);
    anim::copy_sprite_pos(m, t, c);
    add16(m, c + L::Y, dy);
    c
}

/// Cola común: objetivo, espera, golpe (chispazo y muerte) y baja al
/// salir de la vista. Original: `Pow_TiedFreed_0489c6__L0489f8` (`free` =
/// `Pow_FreeStateTail_048a44`, con el chispazo más bajo y la otra muerte).
fn tail_with(m: &mut impl Machine, t: u32, free: bool) -> Flow {
    retarget_if_lost(m, t);
    add16(m, t + L::TIMER, 0xFFFF);
    if fort::hit_side(m, t).is_some() {
        if free {
            spawn_burst(m, t, L::BURST_DY_FREE);
            soldier::check_death_b(m, t);
        } else {
            spawn_burst(m, t, L::BURST_DY);
            soldier::check_death(m, t);
        }
    }
    if items::out_of_view(m, t, Some(L::MARGINS)) {
        set_handler(m, t, L::GONE);
    }
    Flow::Return
}

fn tail(m: &mut impl Machine, t: u32) -> Flow {
    tail_with(m, t, false)
}

fn free_tail(m: &mut impl Machine, t: u32) -> Flow {
    tail_with(m, t, true)
}

/// Cola del atado: el chispazo sale con parte de la velocidad del que le
/// ha dado; se va si sale por su lado o de la vista.
/// Original: `Pow_TiedStateTail_048a90`.
fn tied_tail(m: &mut impl Machine, t: u32) -> Flow {
    retarget_if_lost(m, t);
    add16(m, t + L::TIMER, 0xFFFF);
    if fort::hit_side(m, t).is_some() {
        let c = spawn_burst(m, t, L::BURST_DY);
        let by = m.read_u32(t + L::ATTACKER);
        let v = (m.read_u16(by + L::VEL_X) as i16) >> 4;
        m.write_u16(c + L::VEL_X, v as u16);
        soldier::check_death(m, t);
        m.write_u8(t + L::TIED_PHASE, 2);
    }
    let x = x(m, t);
    if (m.read_u8(t + L::SIDE) as i8) < 0 {
        if x <= L::TIED_EDGE.0 {
            set_handler(m, t, L::GONE);
        }
    } else if x >= L::TIED_EDGE.1 {
        set_handler(m, t, L::GONE);
    }
    if items::out_of_view(m, t, Some(L::TIED_MARGINS)) {
        set_handler(m, t, L::GONE);
    }
    Flow::Return
}

/// Se da de baja. Original: `JmpToScheduler_048b1e`.
pub fn gone(m: &mut impl Machine) -> Flow {
    tasks::finish(m)
}

/// Da de baja a sus hijas y a sí misma. Original: `Jsr5B6ThenJmpScheduler_048b26`.
pub fn gone_all(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    tasks::kill_children(m, t);
    tasks::finish(m)
}

// ---------------------------------------------------------------------------
// Estados del que deambula.
// ---------------------------------------------------------------------------

/// Arranca un estado: guion y rutina del bucle.
fn enter(m: &mut impl Memory, t: u32, script: u32, lp: u32) {
    anim::start(m, t, script);
    set_handler(m, t, lp);
}

fn vel0(m: &mut impl Memory, t: u32) {
    m.write_u16(t + L::VEL_X, 0);
}

fn halve_vel(m: &mut impl Memory, t: u32) {
    let v = (m.read_u16(t + L::VEL_X) as i16) >> 1;
    m.write_u16(t + L::VEL_X, v as u16);
}

fn in_rescue_x(m: &mut impl Memory, t: u32) -> bool {
    let x = x(m, t);
    x >= L::RESCUE_X.0 && x <= L::RESCUE_X.1
}

fn timer_up(m: &mut impl Memory, t: u32) -> bool {
    m.read_u16(t + L::TIMER) as i16 <= 0
}

/// Resta una repetición; `true` si ya no quedan.
fn last_repeat(m: &mut impl Memory, t: u32) -> bool {
    (add8(m, t + L::REPEATS, 0xFF) as i8) <= 0
}

/// Si está en el aire pasa a `h` (el «golpe» del original).
fn hurt_to(m: &mut impl Machine, t: u32, h: u32) {
    if hit_received(m, t) {
        set_handler(m, t, h);
    }
}

fn set_speed(m: &mut impl Machine, t: u32, table: u32) {
    let v = difficulty(m, t, table);
    m.write_u16(t + L::SPEED, v);
    set_run_vel(m, t);
}

fn set_look(m: &mut impl Memory, t: u32, flip: bool) {
    let f = m.read_u8(t + L::FACING);
    m.write_u8(t + L::LOOK, if flip { f ^ 1 } else { f });
}

// --- Aparición -------------------------------------------------------------

/// Entradas de aparición por plantilla (`k` = 0..7): máscara y paso del
/// objeto y cómo empieza. Original: `Pow_SpawnVariantTbl_0478fc` + `k`·$10.
pub fn spawn(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    let (mask, step) = L::ITEM_SETS[k];
    m.write_u8(t + L::ITEM_MASK, mask);
    m.write_u8(t + L::ITEM_STEP, step);
    match k {
        0..=2 => {
            collision::snap_to_ground(m, t);
            start_roaming(m, t)
        }
        3..=5 => spawn_intro(m, t, 0),
        6 => {
            m.write_u8(t + L::HURTABLE, 0);
            m.write_u8(t + L::FREED, 1);
            collision::snap_to_ground(m, t);
            free_init(m, t);
            target_nearest(m, t);
            first_state(m, t)
        }
        _ => spawn_intro(m, t, 1),
    }
}

/// Vuelve a empezar (tras soltarse del poste). Original: `Pow_SpawnInit_04797c`.
pub fn spawn_init(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    video::release_task_entries(m, t);
    m.write_u8(t + L::ITEM_MASK, L::DEFAULT_ITEM.0);
    m.write_u8(t + L::ITEM_STEP, L::DEFAULT_ITEM.1);
    start_roaming(m, t)
}

/// Original: `Pow_SpawnInit_04797c__L047998`.
fn start_roaming(m: &mut impl Machine, t: u32) -> Flow {
    m.write_u8(t + L::HURTABLE, 0);
    m.write_u8(t + L::FREED, 0);
    free_init(m, t);
    target_nearest(m, t);
    first_state(m, t)
}

fn first_state(m: &mut impl Machine, t: u32) -> Flow {
    if m.read_u8(t + L::WALKS) == 0 {
        idle(m)
    } else {
        walk(m)
    }
}

/// Con animación de entrada. Original: `Pow_SpawnInit_04797c__L0479e4` / `__L0479ec`.
fn spawn_intro(m: &mut impl Machine, t: u32, freed: u8) -> Flow {
    m.write_u8(t + L::FREED, freed);
    m.write_u8(t + L::HURTABLE, 0);
    collision::snap_to_ground(m, t);
    free_init(m, t);
    target_nearest(m, t);
    enter(m, t, L::S_SPAWN, L::SPAWN_LOOP);
    spawn_loop(m)
}

/// Original: `Pow_SpawnInit_04797c+$A0`.
pub fn spawn_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scroll_and_probe(m, t);
    if anim::step(m, t) {
        set_handler(m, t, L::IDLE);
    }
    hurt_to(m, t, L::HURT);
    tail(m, t)
}

// --- Deambular -------------------------------------------------------------

/// Otro objetivo al azar y a esperar. Original: `Pow_RetargetPlayer_047a42`.
pub fn retarget(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    retarget_random(m, t);
    idle(m)
}

/// Original: `Pow_Idle_047a4c`.
pub fn idle(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    vel0(m, t);
    set_look(m, t, true);
    enter(m, t, L::S_IDLE, L::IDLE_LOOP);
    idle_loop(m)
}

/// Original: `Pow_Idle_047a4c+$22`.
pub fn idle_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scroll_and_probe(m, t);
    anim::step(m, t);
    if m.read_u8(t + L::WALKS) != 0 {
        let p = m.read_u32(t + L::TARGET);
        set_a0(m, p);
        if target_lost(m, p) {
            retarget_random(m, t);
        } else {
            let k = pick_idle(m, t);
            tasks::goto_by_table(m, t, L::IDLE_CHOICES, k, None);
            if target_far_x(m, t) {
                set_handler(m, t, L::WALK);
            }
            if turn_timer(m, t) {
                set_handler(m, t, L::TURN);
            }
            if should_run_away(m, t) {
                set_handler(m, t, L::RUN_AWAY);
            }
            if should_wait(m, t) {
                set_handler(m, t, L::WAIT);
            }
        }
    }
    if in_rescue_x(m, t) && can_be_rescued(m, t) {
        set_handler(m, t, L::RESCUE);
    }
    hurt_to(m, t, L::HURT);
    tail(m, t)
}

/// Original: `Pow_WalkToward_047b1e`.
pub fn walk(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    set_look(m, t, false);
    set_speed(m, t, L::WALK_SPEED);
    enter(m, t, L::S_WALK, L::WALK_LOOP);
    walk_loop(m)
}

/// Original: `Pow_WalkToward_047b1e+$2C`.
pub fn walk_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scroll_walk(m, t);
    anim::step(m, t);
    if blocked(m, t) {
        set_handler(m, t, L::STOP);
    }
    if target_in_reach(m, t) {
        set_handler(m, t, L::STOP);
    }
    if turn_timer(m, t) {
        set_handler(m, t, L::STOP);
    }
    if should_run_away(m, t) {
        set_handler(m, t, L::STOP);
    }
    if should_wait(m, t) {
        set_handler(m, t, L::STOP);
    }
    hurt_to(m, t, L::HURT);
    tail(m, t)
}

/// Carrera hacia un lado (`left`) y su bucle.
/// Original: `Pow_RunRight_047bac` / `Pow_RunLeft_047c50`.
fn run_side(m: &mut impl Machine, left: bool) -> Flow {
    let t = m.task_register();
    set_look(m, t, left);
    if at_screen_edge(m, t) {
        return run_side(m, !left);
    }
    set_speed(m, t, L::RUN_SPEED);
    if left {
        let v = m.read_u16(t + L::VEL_X).wrapping_neg();
        m.write_u16(t + L::VEL_X, v);
        enter(m, t, L::S_RUN_LEFT, L::RUN_LEFT_LOOP);
    } else {
        enter(m, t, L::S_RUN_RIGHT, L::RUN_RIGHT_LOOP);
    }
    run_side_loop(m, left)
}

fn run_side_loop(m: &mut impl Machine, left: bool) -> Flow {
    let t = m.task_register();
    let other = if left { L::RUN_RIGHT } else { L::RUN_LEFT };
    scroll_walk(m, t);
    anim::step(m, t);
    if blocked(m, t) {
        set_handler(m, t, other);
    }
    if target_in_reach(m, t) {
        set_handler(m, t, L::IDLE);
    }
    if turn_timer(m, t) {
        set_handler(m, t, L::TURN);
    }
    if should_run_away(m, t) {
        set_handler(m, t, L::RUN_AWAY);
    }
    if at_screen_edge(m, t) {
        set_handler(m, t, other);
    }
    if should_wait(m, t) {
        set_handler(m, t, L::WAIT);
    }
    hurt_to(m, t, L::HURT);
    tail(m, t)
}

/// Original: `Pow_RunRight_047bac`.
pub fn run_right(m: &mut impl Machine) -> Flow {
    run_side(m, false)
}

/// Original: `Pow_RunRight_047bac+$34`.
pub fn run_right_loop(m: &mut impl Machine) -> Flow {
    run_side_loop(m, false)
}

/// Original: `Pow_RunLeft_047c50`.
pub fn run_left(m: &mut impl Machine) -> Flow {
    run_side(m, true)
}

/// Original: `Pow_RunLeft_047c50+$3E`.
pub fn run_left_loop(m: &mut impl Machine) -> Flow {
    run_side_loop(m, true)
}

/// Se aparta del jugador que tiene cerca. Original: `Pow_RunAway_047cfe`.
pub fn run_away(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    set_look(m, t, true);
    if at_screen_edge(m, t) {
        return idle(m);
    }
    player_in_box(m, t, L::BOX_NEAR);
    let p = a0(m);
    m.write_u32(t + L::TARGET, p);
    set_speed(m, t, L::RUN_SPEED);
    let v = m.read_u16(t + L::VEL_X).wrapping_neg();
    m.write_u16(t + L::VEL_X, v);
    enter(m, t, L::S_RUN_LEFT, L::RUN_AWAY_LOOP);
    run_away_loop(m)
}

/// Original: `Pow_RunAway_047cfe+$4E`.
pub fn run_away_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scroll_walk(m, t);
    anim::step(m, t);
    if blocked(m, t) {
        set_handler(m, t, L::RESCUE);
    }
    if can_be_rescued(m, t) {
        set_handler(m, t, L::IDLE);
    }
    if turn_timer(m, t) {
        set_handler(m, t, L::TURN);
    }
    if !should_run_away(m, t) {
        set_handler(m, t, L::IDLE);
    }
    if at_screen_edge(m, t) {
        set_handler(m, t, L::IDLE);
    }
    if should_wait(m, t) {
        set_handler(m, t, L::WAIT);
    }
    hurt_to(m, t, L::HURT);
    tail(m, t)
}

/// Original: `Pow_Wait_047dbc`.
pub fn wait(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    vel0(m, t);
    let f = m.read_u8(t + L::FACING) & 1;
    m.write_u8(t + L::LOOK, f);
    if at_screen_edge(m, t) {
        return turn(m);
    }
    if should_turn(m, t) {
        return walk_free(m);
    }
    player_in_box(m, t, L::BOX_NEAR);
    let p = a0(m);
    m.write_u32(t + L::TARGET, p);
    enter(m, t, L::S_WAIT, L::WAIT_LOOP);
    wait_loop(m)
}

/// Original: `Pow_Wait_047dbc+$42`.
pub fn wait_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scroll_and_probe(m, t);
    if anim::step(m, t) {
        set_handler(m, t, L::WALK_FREE);
    }
    hurt_to(m, t, L::HURT);
    tail(m, t)
}

/// Original: `Pow_WalkFree_047e24`.
pub fn walk_free(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    set_look(m, t, false);
    set_speed(m, t, L::WALK_SPEED);
    enter(m, t, L::S_WALK, L::WALK_FREE_LOOP);
    walk_free_loop(m)
}

/// Original: `Pow_WalkFree_047e24+$2C`.
pub fn walk_free_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scroll_walk(m, t);
    anim::step(m, t);
    if !target_near_x(m, t) {
        set_handler(m, t, L::STOP);
    }
    if at_screen_edge(m, t) {
        set_handler(m, t, L::STOP);
    }
    hurt_to(m, t, L::HURT);
    tail(m, t)
}

/// Original: `Pow_Stop_047e88`.
pub fn stop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let p = m.read_u32(t + L::TARGET);
    set_a0(m, p);
    if target_lost(m, p) {
        return retarget(m);
    }
    vel0(m, t);
    enter(m, t, L::S_STOP, L::STOP_LOOP);
    simple_loop(m, L::IDLE, L::HURT)
}

/// Bucle típico: sigue a la escena, anima; al acabar pasa a `next`; si
/// está en el aire, a `hurt`; y la cola común.
fn simple_loop(m: &mut impl Machine, next: u32, hurt: u32) -> Flow {
    let t = m.task_register();
    scroll_and_probe(m, t);
    if anim::step(m, t) {
        set_handler(m, t, next);
    }
    hurt_to(m, t, hurt);
    tail(m, t)
}

/// Original: `Pow_Stop_047e88+$24`.
pub fn stop_loop(m: &mut impl Machine) -> Flow {
    simple_loop(m, L::IDLE, L::HURT)
}

/// Original: `Pow_Turn_047ed2`.
pub fn turn(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    add8(m, t + L::TURNS, 1);
    vel0(m, t);
    enter(m, t, L::S_TURN, L::TURN_LOOP);
    turn_loop(m)
}

/// Original: `Pow_Turn_047ed2+$1A`.
pub fn turn_loop(m: &mut impl Machine) -> Flow {
    simple_loop(m, L::IDLE, L::HURT)
}

/// Original: `Pow_Hurt_047f12` (y `Pow_FreeHurt_048420` con `free`).
fn hurt_with(m: &mut impl Machine, free: bool) -> Flow {
    let t = m.task_register();
    halve_vel(m, t);
    enter(m, t, L::S_HURT, if free { L::FREE_HURT_LOOP } else { L::HURT_LOOP });
    hurt_loop_with(m, free)
}

fn hurt_loop_with(m: &mut impl Machine, free: bool) -> Flow {
    let t = m.task_register();
    if collision::fall(m, t) {
        set_handler(m, t, if free { L::FREE_GET_UP } else { L::GET_UP });
    }
    anim::step(m, t);
    tail(m, t)
}

pub fn hurt(m: &mut impl Machine) -> Flow {
    hurt_with(m, false)
}

/// Original: `Pow_Hurt_047f12+$1C`.
pub fn hurt_loop(m: &mut impl Machine) -> Flow {
    hurt_loop_with(m, false)
}

/// Original: `Pow_GetUp_047f48`.
pub fn get_up(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    vel0(m, t);
    enter(m, t, L::S_GET_UP, L::GET_UP_LOOP);
    get_up_loop(m)
}

/// Original: `Pow_GetUp_047f48+$16`.
pub fn get_up_loop(m: &mut impl Machine) -> Flow {
    simple_loop(m, L::IDLE, L::HURT)
}

// --- Rescate ---------------------------------------------------------------

/// Le han tocado: elige el objeto por el ángulo hacia el jugador.
/// Original: `Pow_RescueStart_047f84`.
pub fn rescue(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if m.read_u8(t + L::FREED) != 0 {
        let p = m.read_u32(t + L::TARGET);
        set_a0(m, p);
        if player_ready(m, p) && m.read_u16(p + L::Y) == m.read_u16(t + L::Y) {
            return hand_over(m);
        }
    }
    m.write_u8(t + L::TURNS, 0);
    vel0(m, t);
    let p = m.read_u32(t + L::TARGET);
    set_a0(m, p);
    let a = angle_to(m, t, p).wrapping_add(m.read_u8(t + L::ITEM_STEP));
    m.write_u8(t + L::ITEM, a);
    let q = a0(m);
    let mask = if y_far(m, t, q) { m.read_u8(t + L::ITEM_MASK) } else { L::NEAR_Y_MASK };
    m.write_u8(t + L::ITEM, (a & mask) >> 4);
    m.write_u8(t + L::SPRITE_SET, 0);
    anim::start(m, t, L::S_RESCUE[0]);
    let item = m.read_u8(t + L::ITEM) & 0xF;
    m.write_u8(t + L::ITEM, item);
    m.write_u8(t + L::ITEM_DIR, item);
    if (item as i8) > 8 {
        m.write_u8(t + L::SPRITE_SET, 1);
        anim::start(m, t, L::S_RESCUE[1]);
    }
    set_handler(m, t, L::RESCUE_LOOP);
    rescue_loop(m)
}

/// Original: `Pow_RescueStart_047f84+$9A`.
pub fn rescue_loop(m: &mut impl Machine) -> Flow {
    simple_loop(m, L::FACE, L::HURT)
}

/// Se gira hacia el jugador contando pasos. Original: `Pow_RescueFaceCount_048044`.
pub fn face(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    vel0(m, t);
    let mut item = m.read_u8(t + L::ITEM) & 0xF;
    let set = m.read_u8(t + L::SPRITE_SET) != 0;
    anim::start(m, t, L::S_FACE[usize::from(set)]);
    m.write_u16(t + L::STEPS, 0);
    let right = facing_right(m, t);
    if !set {
        if !right {
            item = 8u8.wrapping_sub(item);
        }
    } else {
        item = item.wrapping_sub(9);
        if right {
            item = 6u8.wrapping_sub(item);
        }
    }
    m.write_u8(t + L::ITEM, item.wrapping_add(item));
    set_handler(m, t, L::FACE_LOOP);
    face_loop(m)
}

/// Original: `Pow_RescueFaceCount_048044+$76`.
pub fn face_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scroll_and_probe(m, t);
    anim::step(m, t);
    if u16::from(m.read_u8(t + L::ITEM) & 0x1F) == m.read_u16(t + L::STEPS) {
        set_handler(m, t, L::SALUTE_START);
    }
    add16(m, t + L::STEPS, 1);
    hurt_to(m, t, L::HURT);
    tail(m, t)
}

/// Original: `Pow_RescueStartSalute_0480f0`.
pub fn salute_start(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    vel0(m, t);
    add16(m, t + L::STEPS, 0xFFFF);
    let n = difficulty(m, t, L::REPEAT_COUNT);
    m.write_u8(t + L::REPEATS, n as u8);
    salute(m)
}

/// Saluda (según el objeto) con su efecto. Original: `Pow_RescueSalute_048108`.
pub fn salute(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let k = u32::from(m.read_u8(t + L::ITEM_DIR) & 0xF);
    let table = if facing_right(m, t) { L::SALUTE_RIGHT } else { L::SALUTE_LEFT };
    let s = m.read_u32(table + k * 4);
    if s != L::NIL {
        anim::start(m, t, s);
    }
    fx_by_dir(m, t);
    let w = difficulty(m, t, L::PAUSE_TIME);
    m.write_u16(t + L::TIMER, w);
    set_handler(m, t, L::SALUTE_LOOP);
    salute_loop(m)
}

/// Original: `Pow_RescueSalute_048108+$68`.
pub fn salute_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scroll_and_probe(m, t);
    let done = anim::step(m, t);
    if (done || (m.read_u8(t + L::REPEATS) as i8) > 1) && timer_up(m, t) {
        set_handler(m, t, L::SALUTE);
        if last_repeat(m, t) {
            set_handler(m, t, L::TURN_BACK);
        }
    }
    hurt_to(m, t, L::HURT);
    tail(m, t)
}

/// Deshace el giro. Original: `Pow_RescueTurnBack_0481be`.
pub fn turn_back(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    vel0(m, t);
    let set = m.read_u8(t + L::SPRITE_SET) != 0;
    anim::start(m, t, L::S_FACE[usize::from(set)]);
    let half: u16 = if set { 6 } else { 8 };
    let steps = m.read_u16(t + L::STEPS);
    if steps as i16 >= half as i16 {
        m.write_u16(t + L::STEPS, (half * 2).wrapping_sub(steps));
        let f = m.read_u8(t + L::FACING);
        m.write_u8(t + L::FACING, f ^ 1);
    }
    set_handler(m, t, L::TURN_BACK_LOOP);
    turn_back_loop(m)
}

/// Original: `Pow_RescueTurnBack_0481be+$66`.
pub fn turn_back_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scroll_and_probe(m, t);
    anim::step(m, t);
    if m.read_u16(t + L::STEPS) == 0 {
        set_handler(m, t, L::GIVE);
    }
    if m.read_u8(L::FRAME_COUNTER) & 1 == 0 {
        add16(m, t + L::STEPS, 0xFFFF);
    }
    hurt_to(m, t, L::HURT);
    tail(m, t)
}

/// Original: `Pow_RescueGiveItem_048260`.
pub fn give(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    vel0(m, t);
    let w = difficulty(m, t, L::WAIT_TIME);
    m.write_u16(t + L::TIMER, w);
    anim::start(m, t, L::S_GIVE[0]);
    if m.read_u8(t + L::SPRITE_SET) != 0 {
        anim::start(m, t, L::S_GIVE[1]);
    }
    set_handler(m, t, L::GIVE_LOOP);
    give_loop(m)
}

/// Original: `Pow_RescueGiveItem_048260+$3A`.
pub fn give_loop(m: &mut impl Machine) -> Flow {
    simple_loop(m, L::RETARGET, L::HURT)
}

/// El jugador montado le toca a su altura: le da el objeto directamente.
/// Original: `Pow_RescueGiveItem_048260__L0482c0`.
pub fn hand_over(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u32(t + L::HITBOX, L::TOUCH_BOX);
    enter(m, t, L::S_HAND_OVER, L::HAND_OVER_LOOP);
    hand_over_loop(m)
}

/// Original: `Pow_RescueGiveItem_048260+$7A`.
pub fn hand_over_loop(m: &mut impl Machine) -> Flow {
    simple_loop(m, L::THANKS_START, L::HURT)
}

/// Original: `Pow_RescueThanksInit_048300` (`free`: `Pow_FreeThanksInit_0485e8`).
fn thanks_start_with(m: &mut impl Machine, free: bool) -> Flow {
    let t = m.task_register();
    vel0(m, t);
    let n = difficulty(m, t, L::REPEAT_COUNT);
    m.write_u8(t + L::REPEATS, n as u8);
    thanks_with(m, free)
}

/// Original: `Pow_RescueThanks_048314` / `Pow_FreeThanks_0485fc`.
fn thanks_with(m: &mut impl Machine, free: bool) -> Flow {
    let t = m.task_register();
    let w = difficulty(m, t, L::PAUSE_TIME);
    m.write_u16(t + L::TIMER, w);
    m.write_u32(t + L::HITBOX, L::THANKS_BOX);
    enter(m, t, L::S_THANKS, if free { L::FREE_THANKS_LOOP } else { L::THANKS_LOOP });
    thanks_loop_with(m, free)
}

fn thanks_loop_with(m: &mut impl Machine, free: bool) -> Flow {
    let t = m.task_register();
    scroll_and_probe(m, t);
    if anim::step(m, t) && timer_up(m, t) {
        set_handler(m, t, if free { L::FREE_THANKS } else { L::THANKS });
        if last_repeat(m, t) {
            set_handler(m, t, if free { L::FREE_WAIT } else { L::LEAVE });
            let w = difficulty(m, t, L::WAIT_TIME);
            m.write_u16(t + L::TIMER, w);
        }
    }
    hurt_to(m, t, if free { L::FREE_HURT } else { L::HURT });
    free_tail(m, t)
}

pub fn thanks_start(m: &mut impl Machine) -> Flow {
    thanks_start_with(m, false)
}

pub fn thanks(m: &mut impl Machine) -> Flow {
    thanks_with(m, false)
}

/// Original: `Pow_RescueThanks_048314+$2A`.
pub fn thanks_loop(m: &mut impl Machine) -> Flow {
    thanks_loop_with(m, false)
}

/// Original: `Pow_RescueLeave_048392`.
pub fn leave(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    enter(m, t, L::S_LEAVE, L::LEAVE_LOOP);
    leave_loop(m)
}

/// Original: `Pow_RescueLeave_048392+$12`.
pub fn leave_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scroll_and_probe(m, t);
    if anim::step(m, t) {
        m.write_u32(t + L::HITBOX, L::BODY);
        set_handler(m, t, L::RETARGET);
    }
    hurt_to(m, t, L::HURT);
    free_tail(m, t)
}

// --- Ya libre --------------------------------------------------------------

/// Original: `Pow_SpawnFreeVariantA_0483d2` (agradecido y vulnerable).
pub fn free_a(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u8(t + L::GRATEFUL, 1);
    m.write_u8(t + L::HURTABLE, 1);
    free_common(m, t)
}

/// Original: `Pow_SpawnFreeVariantB_0483e2`.
pub fn free_b(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u8(t + L::HURTABLE, 0);
    free_common(m, t)
}

fn free_common(m: &mut impl Machine, t: u32) -> Flow {
    collision::snap_to_ground(m, t);
    free_init(m, t);
    if m.read_u8(t + L::WALKS) == 0 {
        return if m.read_u8(t + L::GRATEFUL) != 0 { free_wait(m) } else { free_stand(m) };
    }
    let d = m.read_u8(t + L::DELAY);
    if d != 0 {
        m.write_u16(t + L::TIMER, u16::from(d));
    }
    if hit_received(m, t) {
        free_hurt(m)
    } else {
        free_walk_out(m)
    }
}

pub fn free_hurt(m: &mut impl Machine) -> Flow {
    hurt_with(m, true)
}

/// Original: `Pow_FreeHurt_048420+$1C`.
pub fn free_hurt_loop(m: &mut impl Machine) -> Flow {
    hurt_loop_with(m, true)
}

/// Original: `Pow_FreeGetUp_048456`.
pub fn free_get_up(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    vel0(m, t);
    enter(m, t, L::S_GET_UP, L::FREE_GET_UP_LOOP);
    free_get_up_loop(m)
}

/// Original: `Pow_FreeGetUp_048456+$16`.
pub fn free_get_up_loop(m: &mut impl Machine) -> Flow {
    simple_loop(m, L::FREE_WALK_OUT, L::FREE_HURT)
}

/// Sale andando hasta la X `+$98`·16. Original: `Pow_FreeWalkOut_048492`.
pub fn free_walk_out(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    set_speed(m, t, L::WALK_SPEED);
    enter(m, t, L::S_WALK, L::FREE_WALK_OUT_LOOP);
    free_walk_out_loop(m)
}

/// Original: `Pow_FreeWalkOut_048492+$26`.
pub fn free_walk_out_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scroll_walk(m, t);
    anim::step(m, t);
    let goal = (u16::from(m.read_u8(t + L::WALKS)) << 4) as i16;
    let beyond = goal > x(m, t);
    if beyond != facing_right(m, t) {
        set_handler(m, t, L::FREE_JUMP);
    }
    hurt_to(m, t, L::FREE_HURT);
    tail(m, t)
}

/// Salta hacia delante (o, si no da las gracias, retrocede).
/// Original: `Pow_FreeJumpOut_048502`.
pub fn free_jump(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if m.read_u8(t + L::GRATEFUL) == 0 {
        return free_step_back(m);
    }
    player_death::set_vx_facing(m, t, L::JUMP_VX);
    m.write_u16(t + L::VEL_Y, L::JUMP_VY);
    m.write_u16(t + L::ACC_Y, L::JUMP_G);
    m.write_u16(t + L::ACC_X, 0);
    enter(m, t, L::S_HAND_OVER, L::FREE_JUMP_LOOP);
    free_jump_loop(m)
}

/// Original: `Pow_FreeJumpOut_048502+$3A`.
pub fn free_jump_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if move_stop(m, t) {
        vel0(m, t);
    }
    if anim::step(m, t) && m.read_u16(t + L::VEL_X) == 0 {
        set_handler(m, t, L::FREE_WAIT);
    }
    if m.read_u8(t + L::HURTABLE) != 0 {
        hurt_to(m, t, L::FREE_HURT);
    }
    tail(m, t)
}

/// Espera a que se acerque un jugador. Original: `Pow_FreeThanksLoop_04857c`.
pub fn free_wait(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    vel0(m, t);
    m.write_u32(t + L::HITBOX, L::TOUCH_BOX);
    enter(m, t, L::S_FREE_WAIT, L::FREE_WAIT_LOOP);
    free_wait_loop(m)
}

/// Original: `Pow_FreeThanksLoop_04857c+$1E`.
pub fn free_wait_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scroll_and_probe(m, t);
    anim::step(m, t);
    if target_within(m, t, L::CLOSE[0]) {
        set_handler(m, t, L::FREE_LEAVE);
    }
    if in_rescue_x(m, t) && timer_up(m, t) {
        set_handler(m, t, L::FREE_THANKS_START);
    }
    hurt_to(m, t, L::FREE_HURT);
    free_tail(m, t)
}

pub fn free_thanks_start(m: &mut impl Machine) -> Flow {
    thanks_start_with(m, true)
}

pub fn free_thanks(m: &mut impl Machine) -> Flow {
    thanks_with(m, true)
}

/// Original: `Pow_FreeThanks_0485fc+$2A`.
pub fn free_thanks_loop(m: &mut impl Machine) -> Flow {
    thanks_loop_with(m, true)
}

/// Al acabar la animación aparece el soldado (el prisionero era falso)
/// con un chispazo. Original: el final de `Pow_FreeLeave_04867a` y
/// `Pow_FreeExit_04883e`.
fn unmask(m: &mut impl Machine, t: u32) {
    let mut r = m.regs();
    r.a[5] = C::SCRATCH_BASE;
    m.set_regs(r);
    soldier::spawn_variant(m, L::SOLDIER_VARIANT);
    spawn_burst(m, t, L::BURST_DY);
}

/// Original: `Pow_FreeLeave_04867a`.
pub fn free_leave(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    vel0(m, t);
    m.write_u32(t + L::HITBOX, L::BODY);
    enter(m, t, L::S_LEAVE, L::FREE_LEAVE_LOOP);
    free_leave_loop(m)
}

/// Original: `Pow_FreeLeave_04867a+$1E`.
pub fn free_leave_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scroll_and_probe(m, t);
    if anim::step(m, t) {
        unmask(m, t);
    }
    hurt_to(m, t, L::FREE_HURT);
    free_tail(m, t)
}

/// Original: `Pow_FreeLeave_04867a__L0486d4`.
pub fn free_step_back(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    vel0(m, t);
    enter(m, t, L::S_STEP_BACK, L::FREE_STEP_BACK_LOOP);
    free_step_back_loop(m)
}

/// Original: `Pow_FreeLeave_04867a+$70`.
pub fn free_step_back_loop(m: &mut impl Machine) -> Flow {
    simple_loop(m, L::FREE_STAND, L::FREE_HURT)
}

/// Original: `Pow_FreeStand_048710`.
pub fn free_stand(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    vel0(m, t);
    m.write_u32(t + L::HITBOX, L::STAND_BOX);
    enter(m, t, L::S_STAND, L::FREE_STAND_LOOP);
    free_stand_loop(m)
}

/// Original: `Pow_FreeStand_048710+$1E`.
pub fn free_stand_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scroll_and_probe(m, t);
    anim::step(m, t);
    if target_within(m, t, L::CLOSE[0]) {
        set_handler(m, t, L::FREE_EXIT);
    }
    if timer_up(m, t) {
        set_handler(m, t, L::FREE_SALUTE_START);
    }
    hurt_to(m, t, L::FREE_HURT);
    tail(m, t)
}

/// Original: `Pow_FreeSaluteInit_048768`.
pub fn free_salute_start(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    vel0(m, t);
    m.write_u32(t + L::HITBOX, L::BODY);
    let n = difficulty(m, t, L::REPEAT_COUNT);
    m.write_u8(t + L::REPEATS, n as u8);
    enter(m, t, L::S_SALUTE_FIRST, L::FREE_SALUTE_LOOP);
    free_salute_loop(m)
}

/// Original: `Pow_FreeSalute_048794`.
pub fn free_salute(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    enter(m, t, L::S_SALUTE_AGAIN, L::FREE_SALUTE_LOOP);
    free_salute_loop(m)
}

/// Original: `Pow_FreeSalute_048794+$12`.
pub fn free_salute_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scroll_and_probe(m, t);
    if anim::step(m, t) && timer_up(m, t) {
        set_handler(m, t, L::FREE_SALUTE);
        if last_repeat(m, t) {
            set_handler(m, t, L::FREE_IDLE);
            let w = difficulty(m, t, L::WAIT_TIME);
            m.write_u16(t + L::TIMER, w);
        }
    }
    hurt_to(m, t, L::FREE_HURT);
    tail(m, t)
}

/// Original: `Pow_FreeIdle_0487fa`.
pub fn free_idle(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    vel0(m, t);
    m.write_u32(t + L::HITBOX, L::STAND_BOX);
    enter(m, t, L::S_FREE_IDLE, L::FREE_IDLE_LOOP);
    free_idle_loop(m)
}

/// Original: `Pow_FreeIdle_0487fa+$1E`.
pub fn free_idle_loop(m: &mut impl Machine) -> Flow {
    simple_loop(m, L::FREE_STAND, L::FREE_HURT)
}

/// Original: `Pow_FreeExit_04883e`.
pub fn free_exit(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    vel0(m, t);
    m.write_u32(t + L::HITBOX, L::BODY);
    enter(m, t, L::S_EXIT, L::FREE_EXIT_LOOP);
    free_exit_loop(m)
}

/// Original: `Pow_FreeExit_04883e+$1E`.
pub fn free_exit_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scroll_and_probe(m, t);
    if anim::step(m, t) {
        unmask(m, t);
    }
    hurt_to(m, t, L::FREE_HURT);
    tail(m, t)
}

// --- Atado -----------------------------------------------------------------

/// Atado colgando (`pole` = en un poste). Original:
/// `Pow_SpawnTiedVariant_048898` (y su entrada `+$8`).
pub fn tied(m: &mut impl Machine, pole: bool) -> Flow {
    let t = m.task_register();
    m.write_u8(t + L::ON_POLE, u8::from(pole));
    human_death::alloc_regs(m, t, L::GFX);
    m.write_u16(t + L::HP, 1);
    anim::set_attr(m, t, L::ATTR_BASE);
    set_priority(m, t, L::PRIORITY);
    m.write_u32(t + L::HITBOX, L::TIED_BOX);
    let side = m.read_u8(t + L::SIDE) as i8 as i16;
    m.write_u16(t + L::SWING_X, (side << 4) as u16);
    let lift = difficulty(m, t, L::TIED_LIFT);
    m.write_u16(t + L::SWING_Y, lift);
    target_nearest(m, t);
    m.write_u16(t + L::STEPS, 0);
    m.write_u8(t + L::ITEM_STEP, L::DEFAULT_ITEM.1);
    m.write_u8(t + L::ITEM_MASK, L::DEFAULT_ITEM.0);
    let c = tasks::spawn(m, L::ROPE, t);
    anim::copy_sprite_pos(m, t, c);
    tied_idle(m)
}

/// Original: `Pow_TiedIdle_04891a`.
pub fn tied_idle(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let w = difficulty(m, t, L::TIED_WAIT);
    m.write_u16(t + L::TIMER, w);
    m.write_u8(t + L::TIED_PHASE, 0);
    enter(m, t, L::S_TIED, L::TIED_IDLE_LOOP);
    tied_idle_loop(m)
}

/// Original: `Pow_TiedIdle_04891a+$28`.
pub fn tied_idle_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if swing(m, t) {
        set_handler(m, t, L::TIED_FREED);
    }
    anim::step(m, t);
    if tied_turn(m, t) {
        m.write_u8(t + L::TIED_PHASE, 1);
        set_handler(m, t, L::TIED_STRUGGLE);
    }
    tied_tail(m, t)
}

/// Forcejea según hacia dónde mira. Original: `Pow_TiedStruggle_04896e`.
pub fn tied_struggle(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u8(t + L::TIED_PHASE, 1);
    fx_from_turn(m, t);
    let k = u32::from((m.read_u16(t + L::STEPS) >> 1) & 0xF);
    let s = m.read_u32(L::STRUGGLE + k * 4);
    if s != L::NIL {
        anim::start(m, t, s);
    }
    set_handler(m, t, L::TIED_STRUGGLE_LOOP);
    tied_struggle_loop(m)
}

/// Original: `Pow_TiedStruggle_04896e+$36`.
pub fn tied_struggle_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if swing(m, t) {
        set_handler(m, t, L::TIED_FREED);
    }
    if anim::step(m, t) {
        set_handler(m, t, L::TIED_IDLE);
    }
    tied_tail(m, t)
}

/// Se suelta y vuelve a empezar como prisionero suelto.
/// Original: `Pow_TiedFreed_0489c6`.
pub fn tied_freed(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u8(t + L::TIED_PHASE, 3);
    enter(m, t, L::S_FREED, L::TIED_FREED_LOOP);
    tied_freed_loop(m)
}

/// Original: `Pow_TiedFreed_0489c6+$18`.
pub fn tied_freed_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if anim::step(m, t) {
        set_handler(m, t, L::SPAWN_INIT);
    }
    tied_tail(m, t)
}

// --- Efectos ---------------------------------------------------------------

/// Efecto del saludo y el objeto lanzado, desplazados según la tabla
/// `table` y la dirección `dir`. Original: `Pow_SpawnFxByDir_04940e__L049418`.
fn fx_spawn(m: &mut impl Machine, t: u32, table: u32, dir: u32) {
    let at = table.wrapping_add((dir & 0xF) * 4);
    m.write_u32(t + L::SPRITE_DATA, at);
    for h in [L::DIR_FX, L::TOSS_DIR] {
        let c = tasks::spawn(m, h, t);
        anim::copy_sprite_pos(m, t, c);
        let (dx, dy) = (m.read_u16(at), m.read_u16(at + 2));
        add16(m, c + L::X, dx);
        add16(m, c + L::Y, dy);
        if h == L::TOSS_DIR {
            let d = m.read_u8(t + L::ITEM_DIR);
            m.write_u8(c + L::ITEM_DIR, d);
        }
    }
}

/// Original: `Pow_SpawnFxByDir_04940e`.
fn fx_by_dir(m: &mut impl Machine, t: u32) {
    let d = u32::from(m.read_u8(t + L::ITEM_DIR));
    fx_spawn(m, t, L::FX_OFFSETS, d);
}

/// Original: `Pow_SpawnFxFromTurnAngle_0493e4`.
fn fx_from_turn(m: &mut impl Machine, t: u32) {
    let d = (m.read_u16(t + L::STEPS) >> 1).wrapping_add(8);
    m.write_u8(t + L::ITEM_DIR, d as u8);
    fx_spawn(m, t, L::FX_TURN_OFFSETS, u32::from(d));
}

/// El objeto lanzado se queda: explota en su sitio.
/// Original: `PowItem_Settle_048b34`.
pub fn settle(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::set_attr(m, t, L::ATTR_SETTLED);
    set_priority(m, t, L::PRIORITY_ITEM);
    video::release_task_entries(m, t);
    explosion::start(m, L::EXPLOSION)
}

/// Sprite que acompaña al saludo. Original: `PowFx_DirSprite_048b56`.
pub fn dir_fx(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    human_death::alloc_regs(m, t, L::DIR_FX_GFX);
    enter(m, t, L::S_DIR_FX, L::DIR_FX_LOOP);
    dir_fx_loop(m)
}

/// Original: `PowFx_DirSprite_048b56+$1C`.
pub fn dir_fx_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if anim::step(m, t) {
        set_handler(m, t, L::GONE_ALL);
    }
    if items::out_of_view_seen(m, t, None) {
        set_handler(m, t, L::GONE_ALL);
    }
    Flow::Return
}

/// Objeto lanzado hacia delante. Original: `PowItem_Toss_048ba0`.
pub fn toss(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let v = difficulty(m, t, L::TOSS_SPEED);
    let v = if facing_right(m, t) { v } else { v.wrapping_neg() };
    m.write_u16(t + L::VEL_X, v);
    m.write_u16(t + L::VEL_Y, 0);
    toss_common(m, t)
}

/// Original: `PowItem_Toss_048ba0+$24`.
pub fn toss_from_walks(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let d = m.read_u8(t + L::WALKS);
    m.write_u8(t + L::ITEM_DIR, d);
    toss_dir(m)
}

/// Lanzado en la dirección `+$7B` (1/16 de vuelta).
/// Original: `PowItem_Toss_048ba0__L048bca`.
pub fn toss_dir(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let v = difficulty(m, t, L::TOSS_SPEED) as i16 as i32;
    let k = u32::from(m.read_u8(t + L::ITEM_DIR) & 0xF) << 5;
    let c = m.read_u16(L::COS_TABLE + k) as i16 as i32;
    let s = m.read_u16(L::SIN_TABLE + k) as i16 as i32;
    m.write_u16(t + L::VEL_X, ((c * v) >> 8) as u16);
    m.write_u16(t + L::VEL_Y, ((s * v) >> 8) as u16);
    toss_common(m, t)
}

fn toss_common(m: &mut impl Machine, t: u32) -> Flow {
    m.write_u16(t + L::TOSS_TIMER, L::TOSS_TIME);
    human_death::alloc_regs(m, t, L::TOSS_GFX);
    anim::set_attr(m, t, L::ATTR_ITEM);
    set_priority(m, t, L::PRIORITY_ITEM);
    let f = m.read_u8(t + L::FLAGS6B);
    m.write_u8(t + L::FLAGS6B, f | L::TOSS_FLAG);
    sound::play(m, L::TOSS_SOUND);
    enter(m, t, L::S_TOSS, L::TOSS_LOOP);
    toss_loop(m)
}

/// Original: `PowItem_Toss_048ba0+$AC`.
pub fn toss_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if move_probe(m, t) {
        set_handler(m, t, L::SETTLE);
    }
    anim::step(m, t);
    if add16(m, t + L::TOSS_TIMER, 0xFFFF) as i16 <= 0 {
        set_handler(m, t, L::SETTLE);
    }
    hits::run_attack_list(m, t);
    if m.read_u8(t + L::FLAGS2) & L::SPENT != 0 {
        set_handler(m, t, L::SETTLE);
    }
    if items::out_of_view_seen(m, t, None) {
        set_handler(m, t, L::GONE_ALL);
    }
    Flow::Return
}

/// Chispazo que salta al recibir un golpe. Original: `PowFx_HitBurst_048ca4`.
pub fn burst(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    player_death::set_vx_facing(m, t, L::BURST_VX);
    m.write_u16(t + L::VEL_Y, L::BURST_VY);
    m.write_u16(t + L::ACC_Y, L::BURST_G);
    m.write_u16(t + L::ACC_X, 0);
    human_death::alloc_regs(m, t, L::GFX);
    enter(m, t, L::S_BURST, L::BURST_LOOP);
    burst_loop(m)
}

/// Original: `PowFx_HitBurst_048ca4+$3C`.
pub fn burst_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if move_stop(m, t) {
        set_handler(m, t, L::GONE_ALL);
    }
    anim::step(m, t);
    if items::out_of_view_seen(m, t, None) {
        set_handler(m, t, L::GONE_ALL);
    }
    Flow::Return
}

// --- Cuerda ----------------------------------------------------------------

/// Cuerda del atado (tarea hija). Original: `PowRope_Spawn_048d0e`.
pub fn rope(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    human_death::alloc_regs(m, t, L::ROPE_GFX);
    m.write_u32(t + L::HITBOX, L::ROPE_BOX);
    let hp = difficulty(m, t, L::ROPE_HP);
    m.write_u16(t + L::HP, hp);
    rope_idle(m)
}

/// Original: `PowRope_Idle_048d30`.
pub fn rope_idle(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    enter(m, t, L::S_ROPE, L::ROPE_IDLE_LOOP);
    rope_idle_loop(m)
}

/// Sigue a la madre justo por detrás; `idle` copia además su giro.
fn rope_follow(m: &mut impl Machine, t: u32, idle: bool) -> Option<u32> {
    if parent_dead(m, t) {
        return None;
    }
    let p = follow_parent(m, t);
    if idle {
        let s = m.read_u16(p + L::STEPS);
        m.write_u16(t + L::STEPS, s);
    }
    add16(m, t + L::ATTR, 0xFFFF);
    anim::step(m, t);
    Some(p)
}

/// Original: `PowRope_Idle_048d30+$12`.
pub fn rope_idle_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let Some(p) = rope_follow(m, t, true) else {
        return gone_all(m);
    };
    if m.read_u8(p + L::TIED_PHASE) == 1 {
        set_handler(m, t, L::ROPE_STRUGGLE_START);
    }
    rope_check(m, t)
}

/// Original: `PowRope_Struggle_048d7a`.
pub fn rope_struggle(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let p = m.read_u32(t + T::OWNER);
    let k = u32::from((m.read_u16(p + L::STEPS) >> 1) & 0xF);
    let s = m.read_u32(L::ROPE_STRUGGLE + k * 4);
    if s != L::NIL {
        anim::start(m, t, s);
    }
    set_handler(m, t, L::ROPE_STRUGGLE_LOOP);
    rope_struggle_loop(m)
}

/// Original: `PowRope_Struggle_048d7a+$30`.
pub fn rope_struggle_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let Some(p) = rope_follow(m, t, false) else {
        return gone_all(m);
    };
    if m.read_u8(p + L::TIED_PHASE) == 0 {
        set_handler(m, t, L::ROPE_IDLE);
    }
    rope_check(m, t)
}

/// Fase de la madre (suelta o golpeada: la cuerda se rompe) y golpes a la
/// cuerda: si se rompe, el golpe pasa a la madre.
/// Original: `PowRope_CheckPhase3_048e2c` … `PowRope_HitCheck_048e54`.
fn rope_check(m: &mut impl Machine, t: u32) -> Flow {
    let p = m.read_u32(t + T::OWNER);
    set_a0(m, p);
    match m.read_u8(p + L::TIED_PHASE) {
        3 => {
            set_handler(m, t, L::ROPE_BROKEN_A);
            return Flow::Return;
        }
        2 => {
            set_handler(m, t, L::ROPE_BROKEN_B);
            return Flow::Return;
        }
        _ => {}
    }
    if fort::hit_side(m, t).is_some() {
        let f = m.read_u8(t + L::FLAGS2);
        m.write_u8(t + L::FLAGS2, f & !L::HIT);
    }
    if fort::dead(m, t) {
        let p = m.read_u32(t + T::OWNER);
        set_a0(m, p);
        m.write_u32(p + L::ATTACKER, p);
        let k = m.read_u8(t + L::ATTACK_KIND);
        m.write_u8(p + L::ATTACK_KIND, k);
        let (px, py) = (m.read_u16(p + L::X), m.read_u16(p + L::Y));
        m.write_u16(p + L::IMPACT, px);
        m.write_u16(p + L::IMPACT + 2, py);
        let f = m.read_u8(p + L::FLAGS2);
        m.write_u8(p + L::FLAGS2, f | L::HIT);
        set_handler(m, t, L::ROPE_BROKEN_B);
    }
    if parent_dead(m, t) {
        set_handler(m, t, L::GONE_ALL);
    }
    Flow::Return
}

/// Cuerda rota (soltándose / cortada). Original: `PowRope_BrokenA_048ddc`
/// y `PowRope_BrokenB_048dec`.
fn rope_broken(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::S_ROPE_BROKEN[k]);
    m.write_u32(t + L::HITBOX, L::NIL);
    set_handler(m, t, L::ROPE_BROKEN_LOOP);
    rope_broken_loop(m)
}

pub fn rope_broken_a(m: &mut impl Machine) -> Flow {
    rope_broken(m, 0)
}

pub fn rope_broken_b(m: &mut impl Machine) -> Flow {
    rope_broken(m, 1)
}

/// Original: `PowRope_BrokenB_048dec+$1A` (anima a medio ritmo).
pub fn rope_broken_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if m.read_u8(L::FRAME_COUNTER) & 1 != 0 && anim::step(m, t) {
        set_handler(m, t, L::GONE_ALL);
    }
    Flow::Return
}
