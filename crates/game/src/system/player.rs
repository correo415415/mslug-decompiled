//! Personaje del jugador (Marco / Tarma): alta de la entidad al empezar
//! una vida y los pasos comunes que hace cada frame (parpadeo de la
//! invulnerabilidad, botones de dirección, historial de la dirección).
//!
//! Original: `PlayerEntitySpawn_0335A6`, `PlayerEntity_InitAuxState_032A02`,
//! `Player_SetIndexFromParent_0517aa`, `Player_FrameCommon_032ff2`,
//! `Player_FrameCommon_NoPrio_033016`, `Player_InvulnBlinkStep_032cc8`,
//! `Slug_UpdateInputFlags_02a720`, `Player_ReadDirNibble_032f3c`,
//! `Player_ReadFireNibble_032f88`, `Player_CheckDeathOrState21_032aa8`,
//! `Input_ForwardByFacing_032e48`, `$027BC8`, `PlayerEntitySpawn_Alt_03364A`,
//! `$0336DC`, `$032AC8` (`asm/player/`).

use super::{anim, motion, slug, tasks, turret};
use crate::entity::Entity;
use crate::input::{self, Buttons, Edge};
use crate::layout::{player as L, task as T};
use crate::mem::{Machine, Memory};
use crate::Flow;

fn set_bits(m: &mut impl Memory, a: u32, bits: u8, on: bool) {
    let v = m.read_u8(a);
    m.write_u8(a, if on { v | bits } else { v & !bits });
}

fn held(m: &mut impl Memory, t: u32, b: Buttons) -> bool {
    input::pressed(m, Entity::from_raw(t), b, Edge::Held)
}

/// Alta del jugador en el inicio de la misión: posición, sprites, estado
/// auxiliar, el brazo (tarea hija), velocidades a cero, el paso común del
/// primer frame, el aterrizaje y el primer fotograma; después queda en el
/// estado de espera. Original: `PlayerEntitySpawn_0335A6`.
pub fn spawn(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::X, L::SPAWN_X);
    m.write_u16(t + L::Y, L::SPAWN_Y);
    for g in L::SPAWN_GFX {
        anim::alloc_entry(m, t, g);
    }
    m.write_u16(t + L::KIND, L::KIND_PLAYER);
    slug::add_timer(m, t);
    init_aux(m, t);
    set_bits(m, t + T::FLAGS, L::PAUSED, false);
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, a | L::ATTR_SPAWNING);
    let arm = tasks::spawn(m, L::ARM_TASK, t);
    anim::copy_sprite_pos(m, t, arm);
    turret::copy_pilot(m, t, arm);
    for f in [L::VEL_X, L::VEL_Y, L::ACC_X, L::ACC_Y] {
        m.write_u16(t + f, 0);
    }
    m.write_u8(t + L::SUB_X, 0);
    m.write_u8(t + L::SUB_Y, 0);
    set_bits(m, t + T::FLAGS2, T::FLAGS2_FRESH, true);
    frame_common(m, t);
    let at = leftover(m);
    motion::on_entity_pos(m, t, |m| slug::move_and_stop(m, t, at));
    anim_done(m, t);
    m.write_u32(t + T::HANDLER, L::IDLE);
    Flow::Return
}

/// Posición de reaparición según el marcador de la misión: P1 a la derecha
/// y P2 a la izquierda de la marca. Devuelve el tipo de entrada.
/// Original: `$032AC8`.
pub fn place_at_mark(m: &mut impl Machine, t: u32) -> u16 {
    let k = super::scroll::mission_mark(m);
    let x = if t == L::P1_ENTITY { k.pos.0.wrapping_add(k.d3) } else { k.pos.0.wrapping_sub(k.d3) };
    m.write_u16(t + L::X, x);
    m.write_u16(t + L::Y, k.pos.1);
    k.d2
}

/// Alta del jugador `p` al reaparecer en mitad de la misión: como
/// [`spawn`], en el sitio que marca la misión; después sigue la entrada
/// del tipo que diga la marca (en paracaídas, cayendo, a pie).
/// Original: `PlayerEntitySpawn_Alt_03364A` (P1) y `$0336DC` (P2).
pub fn respawn(m: &mut impl Machine, p: usize) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::X, L::RESPAWN_X[p]);
    m.write_u16(t + L::Y, L::RESPAWN_Y[p]);
    let kind = place_at_mark(m, t);
    for g in L::RESPAWN_GFX[p] {
        anim::alloc_entry(m, t, g);
    }
    m.write_u16(t + L::KIND, L::RESPAWN_KIND[p]);
    slug::add_timer(m, t);
    init_aux(m, t);
    set_bits(m, t + T::FLAGS, L::PAUSED, L::RESPAWN_PAUSED[p]);
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, a | L::RESPAWN_ATTR[p]);
    let arm = tasks::spawn(m, L::RESPAWN_ARM[p], t);
    anim::copy_sprite_pos(m, t, arm);
    turret::copy_pilot(m, t, arm);
    for f in [L::VEL_X, L::VEL_Y, L::ACC_X, L::ACC_Y] {
        m.write_u16(t + f, 0);
    }
    m.write_u8(t + L::SUB_X, 0);
    m.write_u8(t + L::SUB_Y, 0);
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, (a & L::RESPAWN_ATTR_KEEP) | L::RESPAWN_ATTR_SET);
    if kind != 2 {
        for f in L::RESPAWN_WAIT {
            m.write_u8(t + f, L::RESPAWN_WAIT_FRAMES);
        }
    }
    let next = L::RESPAWN_NEXT[usize::from(kind.min(2))];
    let mut r = m.regs();
    r.a[1] = arm;
    m.set_regs(r);
    Flow::Goto(next)
}

fn leftover(m: &mut impl Machine) -> u16 {
    m.regs().d[4] as u16
}

/// Estado auxiliar de una vida nueva: flags, bombas y munición iniciales,
/// arma de serie, a quién pertenece y el efecto de suelo bajo los pies.
/// Original: `PlayerEntity_InitAuxState_032A02`.
pub fn init_aux(m: &mut impl Machine, t: u32) {
    let v = m.read_u16(t + L::SPRITE_ATTR_BASE);
    m.write_u16(t + L::SPRITE_ATTR, v);
    set_bits(m, t + L::FLAGS_5B, L::PASS_TOP, true);
    set_bits(m, t + L::IGNORE, L::PLAIN_FLOOR, true);
    m.write_u32(t + L::HIT_HANDLER, L::HIT_ROUTINE);
    m.write_u8(t + L::LEVELS[0], 0xFF);
    m.write_u8(t + L::LEVELS[1], 0xFF);
    m.write_u16(t + L::ATTR, L::ATTR_INIT);
    slug::mark_timer_max(m, t);
    m.write_u8(t + L::BOMBS, L::START_BOMBS);
    m.write_u16(t + L::AMMO, 0);
    m.write_u8(t + L::BYTE_85, 1);
    m.write_u8(t + L::STATUS_FLAGS, 0);
    m.write_u8(t + L::INVULN_TIMER, 0);
    slug::mask_players(m, t);
    for f in L::CLEAR_WORDS {
        m.write_u16(t + f, 0);
    }
    for f in L::CLEAR_BYTES {
        m.write_u8(t + f, 0);
    }
    m.write_u8(t + L::AIM_DIR, L::AIM_FORWARD);
    m.write_u16(t + L::FIELD_7C, 0);
    m.write_u16(t + L::FIELD_7E, 0);
    set_bits(m, t + L::FLAGS_13, L::STOPPED, false);
    set_controller(m, t);
    let fx = tasks::spawn(m, L::GROUND_FX_TASK, t);
    anim::copy_sprite_pos(m, t, fx);
}

/// Piloto y mando según la tarea de control de la que cuelga.
/// Original: `Player_SetIndexFromParent_0517aa`.
pub fn set_controller(m: &mut impl Memory, t: u32) {
    let o = m.read_u32(t + T::OWNER);
    match L::CONTROL_TASKS.iter().position(|&c| c == o) {
        Some(k) => {
            m.write_u8(t + L::PILOT, k as u8);
            m.write_u8(t + L::CONTROLLER, k as u8 + 1);
        }
        None => m.write_u8(t + L::CONTROLLER, L::NO_CONTROLLER),
    }
}

/// Paso común de cada frame (con la prioridad de sprite a 0).
/// Original: `Player_FrameCommon_032ff2`.
pub fn frame_common(m: &mut impl Machine, t: u32) {
    slug::clear_flags13(m, t);
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, a & L::ATTR_CLEAR_PRIO);
    invuln_blink(m, t);
    dir_flags(m, t);
    fire_nibble(m, t);
}

/// Como [`frame_common`] sin tocar la prioridad y con la dirección
/// completa. Original: `Player_FrameCommon_NoPrio_033016`.
pub fn frame_common_no_prio(m: &mut impl Machine, t: u32) {
    slug::clear_flags13(m, t);
    invuln_blink(m, t);
    dir_flags(m, t);
    dir_nibble(m, t);
}

/// Parpadeo mientras dura la invulnerabilidad: el atributo del sprite sale
/// de una tabla de campos según el contador; al acabar, el de siempre.
/// Original: `Player_InvulnBlinkStep_032cc8`.
pub fn invuln_blink(m: &mut impl Memory, t: u32) {
    let n = m.read_u8(t + L::INVULN_TIMER);
    let v = if n == 0 {
        m.read_u16(t + L::SPRITE_ATTR_BASE)
    } else {
        let n = n - 1;
        m.write_u8(t + L::INVULN_TIMER, n);
        // La tabla sólo tiene sentido en sus primeras entradas; el resto
        // son bytes de código que se usan tal cual (desplazamiento con signo).
        let field = m.read_u16(L::BLINK_FIELDS + u32::from(n & 0x0F) * 2);
        m.read_u16(t.wrapping_add(field as i16 as u32))
    };
    m.write_u16(t + L::SPRITE_ATTR, v);
}

/// Bits de `+$13`: arriba (4) y abajo (5) mantenidos.
/// Original: `Slug_UpdateInputFlags_02a720`.
pub fn dir_flags(m: &mut impl Memory, t: u32) {
    let up = held(m, t, Buttons::UP);
    let down = held(m, t, Buttons::DOWN);
    set_bits(m, t + L::FLAGS_13, L::UP_HELD, up);
    set_bits(m, t + L::FLAGS_13, L::DOWN_HELD, down);
}

/// Apunta la dirección (adelante, arriba o abajo) y el historial (la
/// anterior en el nibble alto). Original: `Player_ReadDirNibble_032f3c`.
pub fn dir_nibble(m: &mut impl Memory, t: u32) {
    let prev = m.read_u8(t + L::AIM_DIR);
    m.write_u8(t + L::AIM_DIR, L::AIM_FORWARD);
    let _ = forward_held(m, t);
    if held(m, t, Buttons::DOWN) {
        m.write_u8(t + L::AIM_DIR, L::AIM_DOWN);
    }
    if held(m, t, Buttons::UP) {
        m.write_u8(t + L::AIM_DIR, L::AIM_UP);
    }
    push_history(m, t, prev);
}

/// Como [`dir_nibble`] pero sólo distingue arriba de adelante.
/// Original: `Player_ReadFireNibble_032f88`.
pub fn fire_nibble(m: &mut impl Memory, t: u32) {
    let prev = m.read_u8(t + L::AIM_DIR);
    let d = if held(m, t, Buttons::UP) { L::AIM_UP } else { L::AIM_FORWARD };
    m.write_u8(t + L::AIM_DIR, d);
    push_history(m, t, prev);
}

fn push_history(m: &mut impl Memory, t: u32, prev: u8) {
    let d = m.read_u8(t + L::AIM_DIR);
    m.write_u8(t + L::AIM_HISTORY, (prev << 4) | d);
}

/// ¿Se pulsa hacia donde mira? Original: `Input_ForwardByFacing_032e48`.
pub fn forward_held(m: &mut impl Memory, t: u32) -> bool {
    let left = m.read_u8(t + L::FACING_FLAGS) & L::FACING_LEFT != 0;
    held(m, t, if left { Buttons::LEFT } else { Buttons::RIGHT })
}

/// Avanza la animación: devuelve si ha acabado o si `+$21` está a 0.
/// Original: `Player_CheckDeathOrState21_032aa8` (C).
pub fn anim_done(m: &mut impl Machine, t: u32) -> bool {
    anim::step(m, t) || m.read_u8(t + L::BYTE_21) == 0
}
