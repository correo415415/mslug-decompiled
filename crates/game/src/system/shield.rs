//! Soldado del escudo (plantillas $6C y $6D) y su escudo.
//!
//! El soldado lleva delante un escudo (otra tarea, hija suya) que copia su
//! posición y su postura. Mientras el escudo aguanta, los golpes que recibe
//! no pasan al soldado (salvo que `+$81` lo permita). El soldado avanza o
//! retrocede según la distancia al jugador, se agacha para golpear con el
//! escudo o apunta con el arma, y cae si deja de pisar suelo. Cuando el
//! escudo se rompe, sale volando, rebota, parpadea y desaparece; el soldado
//! suelta un objeto y huye.
//!
//! La plantilla $6D (variante «B», índice 1 de las tablas `[_; 2]`) es la
//! misma máquina de estados sin la parte de avanzar y retroceder: o espera
//! en su sitio o corre hasta una X fija (`+$99` × 16) y se para.
//!
//! Original: `asm/enemies/soldiers/barrel_paratrooper_shield_tank_066000.s`
//! (`ShieldSoldier_Tmpl6C_0670d2` … `ShieldSoldier_DropItemB_068286`).

use super::{anim, collision, contact, fort, hits, items, manual_icons, pickup, platforms, player_death, prisoner, rng, slug, soldier, sound, tasks};
use crate::layout::{collision as C, platforms as P, shield as L, task as T};
use crate::mem::{Flow, Machine, Memory};

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

fn handler(m: &mut impl Memory, t: u32) -> u32 {
    m.read_u32(t + T::HANDLER)
}

fn x(m: &mut impl Memory, t: u32) -> i16 {
    m.read_u16(t + L::X) as i16
}

fn facing_right(m: &mut impl Memory, t: u32) -> bool {
    m.read_u8(t + L::FACING) & 1 != 0
}

/// Arranca el guion `script` y pasa al estado `h`.
fn enter(m: &mut impl Machine, t: u32, script: u32, h: u32) {
    anim::start(m, t, script);
    set_handler(m, t, h);
}

// ---------------------------------------------------------------------------
// Ayudantes del soldado ($067E7E…$068286).
// ---------------------------------------------------------------------------

/// Alta común de las dos plantillas: orientación hacia el centro, al
/// suelo, modo al azar si `+$98` = 2, gráfico según el tipo, cajas, y el
/// escudo como tarea hija. Original: `ShieldSoldier_Setup_067e7e`.
fn setup(m: &mut impl Machine, t: u32) {
    m.write_u8(t + L::DROPPED, 0);
    if x(m, t) <= L::TURN_X {
        let f = m.read_u8(t + L::FACING) ^ 1;
        m.write_u8(t + L::FACING, f);
    }
    collision::snap_to_ground(m, t);
    if m.read_u8(t + L::MODE) == 2 {
        let (v, _) = rng::next(m);
        m.write_u8(t + L::MODE, v as u8 & 1);
    }
    let alt = m.read_u8(t + L::KIND) != 0;
    gfx_by_kind(m, t, alt);
    anim::set_attr(m, t, L::ATTR_SOLDIER);
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, (a & L::ATTR_KEEP) | L::PRIO_SOLDIER);
    m.write_u8(t + L::SHIELD_IMMUNE, 0);
    m.write_u16(t + L::HP, 1);
    m.write_u32(t + L::HITBOX, L::HITBOX_STAND);
    m.write_u32(t + L::HITBOX2, L::HITBOX2_INIT);
    m.write_u32(t + L::ATTACKS, L::ATTACKS_INIT);
    let s = tasks::spawn(m, L::SHIELD_INIT, t);
    anim::copy_sprite_pos(m, t, s);
    m.write_u32(t + L::SHIELD, s);
    let v = soldier::by_difficulty(m, t, L::D_RISE);
    m.write_u16(t + L::TIMER, v);
}

/// Gráfico y retardo de animación según el tipo (soldado o escudo).
fn gfx_by_kind(m: &mut impl Memory, t: u32, alt: bool) {
    let k = usize::from(alt);
    anim::alloc_entry(m, t, L::GFX[k]);
    m.write_u16(t + L::TIMER_1C, L::TIMER_1C_INIT[k]);
    slug::add_timer(m, t);
}

/// Sigue a la escena y, si deja de pisar, cae.
/// Original: `ShieldSoldier_ScrollProbe_067f46`.
fn scroll_probe(m: &mut impl Machine, t: u32) {
    scene(m, t);
    if prisoner::probe(m, t) {
        collision::fall(m, t);
    }
}

/// Sigue a la escena y anda por el suelo, o cae si no lo pisa.
/// Original: `ShieldSoldier_ScrollProbeB_067f62` / `_ProbeRevert_067f7a`.
fn scroll_probe_walk(m: &mut impl Machine, t: u32) {
    scene(m, t);
    if prisoner::probe(m, t) {
        collision::fall(m, t);
    } else {
        prisoner::move_ground(m, t);
    }
}

fn scene(m: &mut impl Machine, t: u32) {
    platforms::move_with_scene(m, t);
    let mut r = m.regs();
    r.a[5] = P::SCRATCH_BASE;
    m.set_regs(r);
}

/// Velocidad de carrera al azar (por dificultad) hacia donde mira, y la
/// lista de ataques que le corresponde. Original: `ShieldSoldier_RngRunSpeed_067f86`.
fn run_speed(m: &mut impl Memory, t: u32) {
    let v = soldier::by_difficulty(m, t, L::D_RUN);
    let vx = if facing_right(m, t) { v } else { v.wrapping_neg() };
    m.write_u16(t + L::VEL_X, vx);
    let i = u32::from((v >> 7) & 0xC);
    let a = m.read_u32(L::RUN_ATTACKS + i);
    m.write_u32(t + L::ATTACKS, a);
}

/// Los golpes al escudo cuentan como golpes al soldado (con su clase).
/// Original: `ShieldSoldier_TakeShieldHit_067fb8`.
fn take_shield_hit(m: &mut impl Memory, t: u32) {
    if m.read_u8(t + L::SHIELD_IMMUNE) != 0 {
        return;
    }
    let s = m.read_u32(t + L::SHIELD);
    if m.read_u8(s + L::HIT_FLAGS) & 1 == 0 {
        return;
    }
    let f = m.read_u8(t + L::FLAGS_13);
    m.write_u8(t + L::FLAGS_13, f | L::HIT);
    let h = m.read_u8(t + L::HIT_FLAGS);
    m.write_u8(t + L::HIT_FLAGS, h | 1);
    let c = m.read_u8(s + L::ATTACK_CLASS);
    m.write_u8(t + L::ATTACK_CLASS, c);
}

/// Con el escudo aún activo, el soldado hace daño al tocar a los
/// jugadores. Original: `ShieldSoldier_ShieldAlive_0681ee`.
fn shield_contact(m: &mut impl Memory, t: u32) {
    let s = m.read_u32(t + L::SHIELD);
    if m.read_u32(s + L::HITBOX) != L::NO_HITBOX {
        contact::touch_players(m, t);
    }
}

/// ¿Está en el borde de la pantalla mirando hacia fuera?
/// Original: `ShieldSoldier_AtScreenEdge_067fec`.
fn at_edge(m: &mut impl Memory, t: u32) -> bool {
    let (x, right) = (x(m, t), facing_right(m, t));
    (x <= L::EDGE.0 && right) || (x >= L::EDGE.1 && !right)
}

fn in_zone(m: &mut impl Memory, t: u32) -> bool {
    let x = x(m, t);
    x <= L::ZONE.1 && x >= L::ZONE.0
}

/// Distancia horizontal (con signo de 16 bits) al jugador `p`.
/// Original: `ShieldSoldier_AbsDX_06824c`.
fn abs_dx(m: &mut impl Memory, t: u32, p: u32) -> i16 {
    (m.read_u16(t + L::X).wrapping_sub(m.read_u16(p + L::X)) as i16).wrapping_abs()
}

fn range(m: &mut impl Memory, t: u32) -> i16 {
    m.read_u16(t + L::RANGE) as i16
}

fn timer_up(m: &mut impl Memory, t: u32) -> bool {
    (m.read_u16(t + L::TIMER) as i16) <= 0
}

/// Busca jugador en la caja `bbox` dejando a0 como el original.
fn in_box(m: &mut impl Machine, t: u32, bbox: u32) -> Option<u32> {
    let p = pickup::player_in_box(m, t, bbox);
    player_regs(m, p);
    let mut r = m.regs();
    r.a[1] = bbox;
    m.set_regs(r);
    p
}

fn nearest(m: &mut impl Machine, t: u32) -> Option<u32> {
    let p = pickup::nearest_player_x(m, t);
    player_regs(m, p);
    p
}

fn player_regs(m: &mut impl Machine, p: Option<u32>) {
    let mut r = m.regs();
    r.a[0] = p.unwrap_or(crate::layout::PLAYER_ENTITY[1]);
    m.set_regs(r);
}

/// ¿Puede golpear con el escudo (modo 0, jugador bajo y cerca)?
/// `zone`: exige además estar dentro de la pantalla.
/// Original: `ShieldSoldier_CanAttackA_068020` (y `+$14`).
fn can_bash(m: &mut impl Machine, t: u32, zone: bool) -> bool {
    if (zone && !in_zone(m, t)) || m.read_u8(t + L::MODE) != 0 || !timer_up(m, t) {
        return false;
    }
    match in_box(m, t, L::BOX_LOW) {
        Some(p) => abs_dx(m, t, p) <= range(m, t),
        None => false,
    }
}

/// Como [`can_bash`] sin mirar la distancia.
/// Original: `ShieldSoldier_CanAttackB_068070`.
fn can_bash_b(m: &mut impl Machine, t: u32) -> bool {
    in_zone(m, t) && m.read_u8(t + L::MODE) == 0 && timer_up(m, t) && in_box(m, t, L::BOX_LOW).is_some()
}

/// ¿Puede disparar (modo 1, jugador en la caja alta)?
/// Original: `ShieldSoldier_CanAttackC_0680a4` (y `+$14`).
fn can_shoot(m: &mut impl Machine, t: u32, zone: bool) -> bool {
    if (zone && !in_zone(m, t)) || m.read_u8(t + L::MODE) != 1 || !timer_up(m, t) {
        return false;
    }
    in_box(m, t, L::BOX_HIGH).is_some()
}

/// ¿No hay jugador o está a su alcance? Original: `ShieldSoldier_PlayerFar_0680e8`.
fn player_in_range(m: &mut impl Machine, t: u32) -> bool {
    match nearest(m, t) {
        None => true,
        Some(p) => abs_dx(m, t, p) <= range(m, t),
    }
}

/// ¿Está el jugador más allá del alcance (+16)?
/// Original: `ShieldSoldier_PlayerNear_06810a`.
fn player_beyond(m: &mut impl Machine, t: u32) -> bool {
    match nearest(m, t) {
        None => false,
        Some(p) => abs_dx(m, t, p).wrapping_sub(L::FAR_SLACK as i16) > range(m, t),
    }
}

/// ¿No hay jugador, está fuera de alcance o el soldado está en el borde?
/// Original: `ShieldSoldier_PlayerFarOrEdge_068130`.
fn stop_backing(m: &mut impl Machine, t: u32) -> bool {
    match nearest(m, t) {
        None => true,
        Some(p) => abs_dx(m, t, p) > range(m, t) || at_edge(m, t),
    }
}

/// ¿Está el jugador demasiado cerca (y hay sitio para retroceder)?
/// Original: `ShieldSoldier_PlayerNearEdge_06815a`.
fn too_close(m: &mut impl Machine, t: u32) -> bool {
    match nearest(m, t) {
        None => false,
        Some(_) if at_edge(m, t) => false,
        Some(p) => abs_dx(m, t, p).wrapping_add(L::EDGE_SLACK as i16) <= range(m, t),
    }
}

/// Tras 15 frames, ¿hay jugador en la caja cercana y no en la lejana?
/// Original: `ShieldSoldier_GuardTick_068186`.
fn guard_tick(m: &mut impl Machine, t: u32) -> bool {
    let g = m.read_u16(t + L::GUARD).wrapping_add(1);
    m.write_u16(t + L::GUARD, g);
    if (g as i16) < L::GUARD_MAX {
        return false;
    }
    m.write_u16(t + L::GUARD, L::GUARD_MAX as u16);
    if in_box(m, t, L::BOX_FAR).is_some() {
        return false;
    }
    in_box(m, t, L::BOX_NEAR).is_some()
}

/// ¿Lleva varios frames sin pisar? (tal cual el original, el contador se
/// pone a cero si no llega a 3). Original: `ShieldSoldier_FallCheck_0681c6`.
fn fall_check(m: &mut impl Machine, t: u32) -> bool {
    if prisoner::probe(m, t) {
        let n = m.read_u8(t + L::AIR).wrapping_add(1);
        m.write_u8(t + L::AIR, n);
        if n as i8 >= L::AIR_FRAMES {
            return true;
        }
    }
    m.write_u8(t + L::AIR, 0);
    false
}

/// Suelta su objeto (una vez). Original: `ShieldSoldier_DropItem_068260`.
fn drop_item(m: &mut impl Machine, t: u32, launch: bool) {
    if m.read_u8(t + L::DROPPED) != 0 {
        return;
    }
    let k = m.read_u8(t + L::KIND);
    let Some(o) = items::spawn_here(m, t, k, L::ITEM_FIELD) else { return };
    let mut r = m.regs();
    r.a[0] = o;
    m.set_regs(r);
    let dy = if launch { L::DROP_B_DY } else { L::DROP_DY };
    let y = m.read_u16(o + L::Y).wrapping_add(dy);
    m.write_u16(o + L::Y, y);
    if launch {
        m.write_u16(o + L::VEL_Y, L::DROP_B_VY);
    }
    m.write_u8(t + L::DROPPED, 1);
}

/// El escudo se rompe: humo y trozo. Lo llama el guion del escudo.
/// Original: `Shield_Break_068206`.
pub fn shield_break(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    sound::play(m, L::SND_BREAK);
    let s = tasks::spawn(m, L::SMOKE, t);
    anim::copy_sprite_pos(m, t, s);
    let d = tasks::spawn(m, L::DEBRIS, t);
    anim::copy_sprite_pos(m, t, d);
    let y = m.read_u16(d + L::Y).wrapping_add(L::BREAK_DY);
    m.write_u16(d + L::Y, y);
    let dx = if facing_right(m, t) { L::BREAK_DX.wrapping_neg() } else { L::BREAK_DX };
    let nx = m.read_u16(d + L::X).wrapping_add(dx);
    m.write_u16(d + L::X, nx);
    let mut r = m.regs();
    r.a[0] = d;
    m.set_regs(r);
    Flow::Return
}

// ---------------------------------------------------------------------------
// Colas comunes de los estados del soldado.
// ---------------------------------------------------------------------------

/// Golpe al escudo, contacto, sigue a la escena y anima: el principio de
/// casi todos los bucles. Devuelve si el guion ha terminado.
fn pre(m: &mut impl Machine, t: u32) -> bool {
    take_shield_hit(m, t);
    shield_contact(m, t);
    scroll_probe(m, t);
    anim::step(m, t)
}

/// Si lleva varios frames sin pisar, pasa a caer (`fall` = estado).
fn check_fall(m: &mut impl Machine, t: u32, fall: u32) {
    if fall_check(m, t) {
        set_handler(m, t, fall);
    }
}

/// Cola: temporizador, sonido del golpe, muerte y baja al salir de la
/// vista. Con `hurt` (sólo en el bucle principal) el golpe le hace
/// retroceder. Original: `ShieldSoldier_Tail_0675e4__L06760e` /
/// `ShieldSoldierB_Tail_067b0a__L067b34`.
fn tail(m: &mut impl Machine, t: u32, hurt: Option<usize>) -> Flow {
    let n = m.read_u16(t + L::TIMER).wrapping_sub(1);
    m.write_u16(t + L::TIMER, n);
    let side = fort::hit_side(m, t);
    let mut r = m.regs();
    r.d[0] = (r.d[0] & 0xFFFF_0000) | u32::from(side.unwrap_or(0));
    m.set_regs(r);
    if side.is_some() {
        let f = m.read_u8(t + L::FLAGS_13);
        m.write_u8(t + L::FLAGS_13, f & !L::HIT);
        fort::hit_sound(m, t, L::HIT_SOUNDS);
        if let Some(v) = hurt {
            let h = if m.read_u8(t + L::ATTACK_CLASS) == 1 { L::HURT_A[v] } else { L::HURT_B[v] };
            set_handler(m, t, h);
        }
    }
    if fort::dead(m, t) {
        return die(m, t);
    }
    let mut r = m.regs();
    r.a[0] = L::MARGINS;
    m.set_regs(r);
    if items::out_of_view(m, t, Some(L::MARGINS)) {
        set_handler(m, t, L::GONE);
    }
    Flow::Return
}

/// Muere: suelta el objeto y pasa a la muerte del soldado (o del tipo
/// alternativo). Original: `ShieldSoldier_Die_067692`.
fn die(m: &mut impl Machine, t: u32) -> Flow {
    m.write_u8(t + L::ALIVE, 0);
    let f = m.read_u8(t + L::FLAGS_13);
    m.write_u8(t + L::FLAGS_13, f | L::HIT);
    drop_item(m, t, false);
    if m.read_u8(t + L::KIND) == 0 {
        soldier::check_death(m, t);
    } else {
        set_handler(m, t, L::DEAD_HANDLER);
    }
    Flow::Return
}

/// Estado que espera a que acabe el guion y vuelve a `next`.
fn wait_script(m: &mut impl Machine, v: usize, next: u32) -> Flow {
    let t = m.task_register();
    if pre(m, t) {
        set_handler(m, t, next);
    }
    check_fall(m, t, L::FALL[v]);
    tail(m, t, None)
}

// ---------------------------------------------------------------------------
// Altas.
// ---------------------------------------------------------------------------

/// Plantilla $6C: soldado del escudo que avanza y retrocede.
/// Original: `ShieldSoldier_Tmpl6C_0670d2`.
pub fn init(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    setup(m, t);
    let r = soldier::by_difficulty(m, t, L::D_RANGE);
    m.write_u16(t + L::RANGE, r);
    if m.read_u8(t + L::MODE) != 0 {
        m.write_u16(t + L::RANGE, L::RANGE_HIGH);
    }
    if m.read_u8(t + L::RUN) == 0 {
        return run(m, 0);
    }
    main(m, 0)
}

/// Plantilla $6D: soldado del escudo quieto o que corre hasta su sitio.
/// Original: `ShieldSoldier_Tmpl6D_0676ba`.
pub fn init_b(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    setup(m, t);
    if m.read_u8(t + L::RUN) != 0 {
        return run(m, 1);
    }
    main(m, 1)
}

// ---------------------------------------------------------------------------
// Estados del soldado (`v` = 0 plantilla $6C, 1 plantilla $6D).
// ---------------------------------------------------------------------------

/// En guardia, decidiendo qué hacer.
/// Original: `ShieldSoldier_Tmpl6C_0670d2__L0670fe` / `ShieldSoldier_Tmpl6D_0676ba__L0676c6`.
pub fn main(m: &mut impl Machine, v: usize) -> Flow {
    let t = m.task_register();
    enter(m, t, L::S_MAIN, L::MAIN_LOOP[v]);
    main_loop(m, v)
}

pub fn main_loop(m: &mut impl Machine, v: usize) -> Flow {
    let t = m.task_register();
    pre(m, t);
    if v == 0 {
        if m.read_u8(t + L::RUN) == 0 {
            if too_close(m, t) {
                set_handler(m, t, L::BACK);
            }
            if player_beyond(m, t) {
                set_handler(m, t, L::RUN_ST[0]);
            }
        }
        if guard_tick(m, t) {
            set_handler(m, t, L::GUARD_ST[0]);
        }
        if can_bash(m, t, true) {
            set_handler(m, t, L::CROUCH[0]);
        }
    } else {
        if guard_tick(m, t) {
            set_handler(m, t, L::GUARD_ST[1]);
        }
        if can_bash_b(m, t) {
            set_handler(m, t, L::CROUCH[1]);
        }
    }
    if can_shoot(m, t, true) {
        set_handler(m, t, L::AIM[v]);
    }
    check_fall(m, t, L::FALL[v]);
    tail(m, t, Some(v))
}

/// Se cubre con el escudo hasta que acaba el guion.
/// Original: `ShieldSoldier_Guard_0671dc` / `ShieldSoldierB_Guard_067780`.
pub fn guard(m: &mut impl Machine, v: usize) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::GUARD, 0);
    enter(m, t, L::S_GUARD, L::GUARD_LOOP[v]);
    guard_loop(m, v)
}

pub fn guard_loop(m: &mut impl Machine, v: usize) -> Flow {
    wait_script(m, v, L::MAIN[v])
}

/// Cae hasta tocar suelo. Original: `ShieldSoldier_Fall_067220` /
/// `ShieldSoldierB_Fall_067aae`.
pub fn fall(m: &mut impl Machine, v: usize) -> Flow {
    let t = m.task_register();
    enter(m, t, L::S_FALL, L::FALL_LOOP[v]);
    fall_loop(m, v)
}

pub fn fall_loop(m: &mut impl Machine, v: usize) -> Flow {
    let t = m.task_register();
    take_shield_hit(m, t);
    shield_contact(m, t);
    if collision::fall(m, t) {
        set_handler(m, t, if v == 0 { L::MAIN[0] } else { L::RUN_ST[1] });
    }
    anim::step(m, t);
    tail(m, t, None)
}

/// Corre: la $6C hacia el jugador, la $6D hasta su X de destino.
/// Original: `ShieldSoldier_Fall_067220__L067254` / `ShieldSoldierB_Run_0677c4`.
pub fn run(m: &mut impl Machine, v: usize) -> Flow {
    let t = m.task_register();
    run_speed(m, t);
    m.write_u32(t + L::HITBOX, L::HITBOX_RUN);
    enter(m, t, L::S_RUN, L::RUN_LOOP[v]);
    run_loop(m, v)
}

pub fn run_loop(m: &mut impl Machine, v: usize) -> Flow {
    let t = m.task_register();
    take_shield_hit(m, t);
    shield_contact(m, t);
    scroll_probe_walk(m, t);
    anim::step(m, t);
    if v == 0 {
        if can_bash(m, t, false) {
            set_handler(m, t, L::LAND[0]);
        }
        if can_shoot(m, t, false) {
            set_handler(m, t, L::LAND[0]);
        }
        if guard_tick(m, t) {
            set_handler(m, t, L::LAND[0]);
        }
        if player_in_range(m, t) {
            set_handler(m, t, L::LAND[0]);
        }
        check_fall(m, t, L::FALL[0]);
        stand_hitbox_unless_running(m, t, 0);
    } else {
        let goal = (u16::from(m.read_u8(t + L::RUN)) << 4) as i16;
        let x = x(m, t);
        if (goal > x) != facing_right(m, t) {
            set_handler(m, t, L::LAND[1]);
        }
        stand_hitbox_unless_running(m, t, 1);
        check_fall(m, t, L::FALL[1]);
    }
    tail(m, t, None)
}

fn stand_hitbox_unless_running(m: &mut impl Memory, t: u32, v: usize) {
    if handler(m, t) != L::RUN_LOOP[v] {
        m.write_u32(t + L::HITBOX, L::HITBOX_STAND);
    }
}

/// Frena al dejar de correr. Original: `ShieldSoldier_Land_0672e0` /
/// `ShieldSoldierB_Land_067846`.
pub fn land(m: &mut impl Machine, v: usize) -> Flow {
    let t = m.task_register();
    manual_icons::stop_motion(m, t);
    enter(m, t, L::S_LAND, L::LAND_LOOP[v]);
    land_loop(m, v)
}

pub fn land_loop(m: &mut impl Machine, v: usize) -> Flow {
    let t = m.task_register();
    if v == 0 {
        scroll_probe(m, t);
    } else {
        scene(m, t);
    }
    if anim::step(m, t) {
        set_handler(m, t, L::MAIN[v]);
    }
    check_fall(m, t, L::FALL[v]);
    tail(m, t, None)
}

/// Retrocede (sólo la $6C) mientras el jugador esté demasiado cerca.
/// Original: `ShieldSoldier_Land_0672e0__L06731e`.
pub fn back(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    run_speed(m, t);
    let v = m.read_u16(t + L::VEL_X).wrapping_neg();
    m.write_u16(t + L::VEL_X, v);
    enter(m, t, L::S_BACK, L::BACK_LOOP);
    back_loop(m)
}

pub fn back_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    take_shield_hit(m, t);
    shield_contact(m, t);
    scroll_probe_walk(m, t);
    anim::step(m, t);
    if stop_backing(m, t) {
        set_handler(m, t, L::MAIN[0]);
    }
    if can_bash(m, t, true) {
        set_handler(m, t, L::CROUCH[0]);
    }
    if can_shoot(m, t, true) {
        set_handler(m, t, L::AIM[0]);
    }
    if guard_tick(m, t) {
        set_handler(m, t, L::GUARD_ST[0]);
    }
    check_fall(m, t, L::FALL[0]);
    tail(m, t, None)
}

/// Se agacha para golpear con el escudo (`+$82` golpes).
/// Original: `ShieldSoldier_Crouch_067394` / `ShieldSoldierB_Land_067846__L067886`.
pub fn crouch(m: &mut impl Machine, v: usize) -> Flow {
    let t = m.task_register();
    let n = soldier::by_difficulty(m, t, L::D_CROUCH);
    m.write_u8(t + L::COUNT, n as u8);
    enter(m, t, L::S_CROUCH, L::CROUCH_LOOP[v]);
    crouch_loop(m, v)
}

pub fn crouch_loop(m: &mut impl Machine, v: usize) -> Flow {
    wait_script(m, v, L::HOLD[v])
}

/// Repite el golpe agachado hasta agotar `+$82`.
/// Original: `ShieldSoldier_CrouchHold_0673e4` / `ShieldSoldierB_CrouchHold_0678d6`.
pub fn hold(m: &mut impl Machine, v: usize) -> Flow {
    let t = m.task_register();
    let n = soldier::by_difficulty(m, t, L::D_HOLD);
    m.write_u16(t + L::TIMER, n);
    enter(m, t, L::S_HOLD, L::HOLD_LOOP[v]);
    hold_loop(m, v)
}

pub fn hold_loop(m: &mut impl Machine, v: usize) -> Flow {
    let t = m.task_register();
    let fall = repeat(m, t, v, L::HOLD[v], L::RISE[v]);
    check_fall(m, t, L::FALL[fall]);
    tail(m, t, None)
}

/// Al acabar el guion (y el temporizador), repite el estado `again` o,
/// agotado `+$82`, pasa a `next`. Devuelve qué tabla de caída usa: la
/// $6D, con el guion acabado pero el temporizador aún en marcha, salta al
/// código de la $6C (y cae con su estado).
fn repeat(m: &mut impl Machine, t: u32, v: usize, again: u32, next: u32) -> usize {
    if !pre(m, t) {
        return v;
    }
    if !timer_up(m, t) {
        return 0;
    }
    set_handler(m, t, again);
    let n = m.read_u8(t + L::COUNT).wrapping_sub(1);
    m.write_u8(t + L::COUNT, n);
    if n as i8 <= 0 {
        set_handler(m, t, next);
    }
    v
}

/// Se levanta. Original: `ShieldSoldier_Rise_067452` / `ShieldSoldierB_Rise_067944`.
pub fn rise(m: &mut impl Machine, v: usize) -> Flow {
    let t = m.task_register();
    let n = soldier::by_difficulty(m, t, L::D_RISE);
    m.write_u16(t + L::TIMER, n);
    enter(m, t, L::S_RISE, L::RISE_LOOP[v]);
    rise_loop(m, v)
}

pub fn rise_loop(m: &mut impl Machine, v: usize) -> Flow {
    wait_script(m, v, L::MAIN[v])
}

/// Apunta (`+$82` disparos). Original: `ShieldSoldier_Aim_0674a2` /
/// `ShieldSoldierB_Rise_067944__L067994`.
pub fn aim(m: &mut impl Machine, v: usize) -> Flow {
    let t = m.task_register();
    let n = soldier::by_difficulty(m, t, L::D_AIM);
    m.write_u8(t + L::COUNT, n as u8);
    enter(m, t, L::S_AIM, L::AIM_LOOP[v]);
    aim_loop(m, v)
}

pub fn aim_loop(m: &mut impl Machine, v: usize) -> Flow {
    wait_script(m, v, L::ATTACK[v])
}

/// Dispara hasta agotar `+$82`. Original: `ShieldSoldier_Attack_0674f2` /
/// `ShieldSoldierB_Attack_0679e4`.
pub fn attack(m: &mut impl Machine, v: usize) -> Flow {
    let t = m.task_register();
    let n = soldier::by_difficulty(m, t, L::D_ATTACK);
    m.write_u16(t + L::TIMER, n);
    enter(m, t, L::S_ATTACK, L::ATTACK_LOOP[v]);
    attack_loop(m, v)
}

pub fn attack_loop(m: &mut impl Machine, v: usize) -> Flow {
    let t = m.task_register();
    let fall = repeat(m, t, v, L::ATTACK[v], L::RECOVER[v]);
    slug::clear_flags13(m, t);
    let d1 = m.regs().d[1] as u8;
    let (o, a, d1) = hits::run_attack_list_d1(m, t, d1);
    let mut r = m.regs();
    r.d[7] = o as i32 as u32;
    r.a[0] = a;
    r.d[1] = (r.d[1] & 0xFFFF_FF00) | u32::from(d1);
    m.set_regs(r);
    check_fall(m, t, L::FALL[fall]);
    tail(m, t, None)
}

/// Recupera la guardia tras disparar. Original: `ShieldSoldier_Recover_06756c` /
/// `ShieldSoldierB_Recover_067a5e`.
pub fn recover(m: &mut impl Machine, v: usize) -> Flow {
    let t = m.task_register();
    let n = soldier::by_difficulty(m, t, L::D_RECOVER);
    m.write_u16(t + L::TIMER, n);
    enter(m, t, L::S_RECOVER, L::RECOVER_LOOP[v]);
    recover_loop(m, v)
}

pub fn recover_loop(m: &mut impl Machine, v: usize) -> Flow {
    wait_script(m, v, L::MAIN[v])
}

/// Retrocede por un golpe (`heavy`: clase de ataque 1).
/// Original: `ShieldSoldier_Recover_06756c__L0675bc` / `__L0675d2` y
/// `ShieldSoldierB_Fall_067aae__L067ae2` / `__L067af8`.
pub fn hurt(m: &mut impl Machine, v: usize, heavy: bool) -> Flow {
    let t = m.task_register();
    enter(m, t, if heavy { L::S_HURT_A } else { L::S_HURT_B }, L::TAIL[v]);
    after_hurt(m, v)
}

/// Original: `ShieldSoldier_Tail_0675e4` / `ShieldSoldierB_Tail_067b0a`.
pub fn after_hurt(m: &mut impl Machine, v: usize) -> Flow {
    wait_script(m, v, L::MAIN[v])
}

/// Ha perdido el escudo: suelta el objeto y, al acabar el guion, huye.
/// Original: `ShieldSoldier_ShieldLost_067656`.
pub fn lost(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    drop_item(m, t, true);
    enter(m, t, L::S_LOST, L::LOST_LOOP);
    lost_loop(m)
}

pub fn lost_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scroll_probe(m, t);
    if anim::step(m, t) {
        set_handler(m, t, L::FLEE);
    }
    m.write_u32(t + L::HITBOX, L::NO_HITBOX);
    tail(m, t, None)
}

/// Huye como un soldado normal. Original: `ShieldSoldier_ToFlee_06768c`.
pub fn flee(m: &mut impl Machine) -> Flow {
    let mut r = m.regs();
    r.a[5] = C::SCRATCH_BASE;
    m.set_regs(r);
    soldier::spawn_variant(m, 2)
}

/// Fin de la tarea (`$067B7C`).
pub fn gone(m: &mut impl Machine) -> Flow {
    tasks::finish(m)
}

/// Fin de la tarea y sus hijas (`$067B84`, `Shield_Free_067e7a`).
pub fn free(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    tasks::kill_children(m, t);
    tasks::finish(m)
}

// ---------------------------------------------------------------------------
// El escudo y sus trozos.
// ---------------------------------------------------------------------------

/// El escudo: copia posición, postura y ataques del soldado; si le
/// destrozan, el soldado pasa a huir y el escudo sale volando.
/// Original: `Shield_Init_067b92`.
pub fn shield_init(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let o = m.read_u32(t + T::OWNER);
    let alt = m.read_u8(o + L::KIND) != 0;
    gfx_by_kind(m, t, alt);
    let hp = soldier::by_difficulty(m, t, L::D_SHIELD_HP);
    m.write_u16(t + L::HP, hp);
    m.write_u32(t + L::HITBOX, L::SHIELD_HITBOX);
    set_handler(m, t, L::SHIELD_LOOP);
    shield_loop(m)
}

pub fn shield_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let o = m.read_u32(t + T::OWNER);
    for off in [L::X, L::Y] {
        let v = m.read_u16(o + off);
        m.write_u16(t + off, v);
    }
    for off in [L::POS_BYTES[0], L::POS_BYTES[1], L::FACING] {
        let v = m.read_u8(o + off);
        m.write_u8(t + off, v);
    }
    let a = m.read_u16(o + L::ATTR);
    m.write_u16(t + L::ATTR, a);
    let atk = m.read_u32(o + L::ATTACKS);
    m.write_u32(t + L::ATTACKS, atk);
    let pose = m.read_u8(o + L::POSE);
    let mut a0 = o;
    if pose != m.read_u8(t + L::POSE) {
        m.write_u8(t + L::POSE, pose);
        a0 = m.read_u32(L::SHIELD_POSES + (u32::from(pose & 7) << 2));
        if a0 != L::NO_SCRIPT {
            anim::start(m, t, a0);
        }
    }
    let mut r = m.regs();
    r.a[0] = a0;
    m.set_regs(r);
    anim::step(m, t);
    if fort::hit_side(m, t).is_some() {
        let f = m.read_u8(t + L::FLAGS_13);
        m.write_u8(t + L::FLAGS_13, f & !L::HIT);
    }
    if fort::dead(m, t) {
        set_handler(m, o, L::LOST);
        set_handler(m, t, L::FLY);
    }
    if m.read_u8(o + L::ALIVE) == 0 {
        set_handler(m, t, L::FLY);
    }
    let h = m.read_u32(o + T::HANDLER);
    let mut r = m.regs();
    r.a[0] = o;
    m.set_regs(r);
    if L::INACTIVE.contains(&h) {
        set_handler(m, t, L::FREE);
    }
    Flow::Return
}

/// Cola de los estados del escudo suelto: se borra al salir de la vista.
fn piece_tail(m: &mut impl Machine, t: u32) -> Flow {
    if items::out_of_view_seen(m, t, None) {
        set_handler(m, t, L::FREE);
    }
    let mut r = m.regs();
    r.a[0] = crate::layout::items::DEFAULT_MARGINS;
    m.set_regs(r);
    Flow::Return
}

fn launch(m: &mut impl Machine, t: u32, vy: u16, ay: u16, h: u32) {
    let vx = player_death::set_vx_facing(m, t, L::SHIELD_VX);
    let mut r = m.regs();
    r.d[0] = (r.d[0] & 0xFFFF_0000) | u32::from(vx);
    m.set_regs(r);
    m.write_u16(t + L::VEL_Y, vy);
    m.write_u16(t + L::ACC_Y, ay);
    m.write_u16(t + L::ACC_X, 0);
    enter(m, t, L::S_FLY, h);
}

/// El escudo sale despedido. Original: `Shield_FlyOff_067c7c`.
pub fn fly(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let nx = m.read_u16(t + L::X).wrapping_add(L::FLY_DX);
    m.write_u16(t + L::X, nx);
    let ny = m.read_u16(t + L::Y).wrapping_add(L::FLY_DY);
    m.write_u16(t + L::Y, ny);
    launch(m, t, L::FLY_VY, L::FLY_AY, L::FLY_LOOP);
    fly_loop(m)
}

pub fn fly_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if collision::fall(m, t) {
        set_handler(m, t, L::BOUNCE);
    }
    anim::step(m, t);
    piece_tail(m, t)
}

/// Rebota una vez. Original: `Shield_Bounce_067ce8`.
pub fn bounce(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    launch(m, t, L::BOUNCE_VY, L::BOUNCE_AY, L::BOUNCE_LOOP);
    bounce_loop(m)
}

pub fn bounce_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if collision::fall(m, t) {
        set_handler(m, t, L::BLINK_ST);
    }
    anim::step(m, t);
    piece_tail(m, t)
}

/// Parpadea en el suelo y desaparece. Original: `Shield_Blink_067d32`.
pub fn blink(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::BLINK, L::BLINK_FRAMES);
    set_handler(m, t, L::BLINK_LOOP);
    blink_loop(m)
}

pub fn blink_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scene(m, t);
    if m.read_u8(L::FRAME_COUNTER) & 1 != 0 {
        anim::step(m, t);
    }
    let n = m.read_u16(t + L::BLINK).wrapping_sub(1);
    m.write_u16(t + L::BLINK, n);
    if n as i16 <= 0 {
        set_handler(m, t, L::FREE);
    }
    piece_tail(m, t)
}

/// Trozo del escudo roto, que sale despedido y hace daño.
/// Original: `Shield_Debris_067d6e`.
pub fn debris(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let f = m.read_u8(t + L::FLAGS_6B);
    m.write_u8(t + L::FLAGS_6B, f | L::FLAGS_6B_DEBRIS);
    piece_attr(m, t, L::ATTR_DEBRIS);
    let v = soldier::by_difficulty(m, t, L::D_DEBRIS_VX);
    let vx = if facing_right(m, t) { v } else { v.wrapping_neg() };
    m.write_u16(t + L::VEL_X, vx);
    for g in L::GFX_DEBRIS {
        anim::alloc_entry(m, t, g);
    }
    enter(m, t, L::S_DEBRIS, L::DEBRIS_LOOP);
    debris_loop(m)
}

fn piece_attr(m: &mut impl Memory, t: u32, attr: u16) {
    anim::set_attr(m, t, attr);
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, (a & L::ATTR_KEEP) | L::PRIO_PIECE);
}

pub fn debris_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let mut r = m.regs();
    let at = (r.d[3] as u16, r.d[4] as u16);
    r.a[5] = C::SCRATCH_BASE;
    m.set_regs(r);
    if super::motion::on_entity_pos(m, t, |m| super::motion::move_e_probe(m, t, at)) {
        set_handler(m, t, L::FREE2);
    }
    anim::step(m, t);
    let d1 = m.regs().d[1] as u8;
    let (o, a, d1) = hits::run_attack_list_d1(m, t, d1);
    let mut r = m.regs();
    r.d[7] = o as i32 as u32;
    r.a[0] = a;
    r.d[1] = (r.d[1] & 0xFFFF_FF00) | u32::from(d1);
    m.set_regs(r);
    if m.read_u8(t + L::FLAGS_13) & L::SPENT != 0 {
        set_handler(m, t, L::FREE2);
    }
    piece_tail(m, t)
}

/// Humo del escudo al romperse. Original: `Shield_Smoke_067e1a`.
pub fn smoke(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    piece_attr(m, t, L::ATTR_SMOKE);
    anim::alloc_entry(m, t, L::GFX_SMOKE);
    enter(m, t, L::S_SMOKE, L::SMOKE_LOOP);
    smoke_loop(m)
}

pub fn smoke_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scene(m, t);
    if anim::step(m, t) {
        set_handler(m, t, L::FREE);
    }
    piece_tail(m, t)
}
