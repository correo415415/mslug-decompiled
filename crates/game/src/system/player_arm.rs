//! Brazo del jugador: una tarea hija que dibuja el brazo y el arma encima
//! del cuerpo. Cada frame se pega al cuerpo (con el desplazamiento que éste
//! pide), copia su parpadeo y, según el estado del cuerpo, ejecuta la pose
//! que el cuerpo le indica (`+$74`). Cada pose elige un guion de la tabla
//! del arma que lleva el jugador (10 entradas: 5 armas × 2 jugadores) según
//! el modo de disparo, la dirección o un contador.
//!
//! Original: `PlayerArm_Spawn_0394a8` (y la variante del segundo jugador en
//! `$0394E6`), `PlayerArm_ResetState_0392f8`, `PlayerArm_FollowParent_03937c`,
//! `Entity_DispatchOpcodeNibble_032D00`, `PlayerArm_WeaponTableIndex_03933a`,
//! `PlayerArm_AnimStepToParentDeath_039354`, `PlayerArm_CopyBit2ToParent_039416`
//! y las 79 poses `PlayerArm_*` (`asm/player/arm_weapon_fx_0388f0.s`,
//! `asm/player/arm_air_death_crouch_03a60a.s`).

use super::{anim, hits, rng, slug, sound};
use crate::layout::{player_arm as L, task as T};
use crate::mem::{Machine, Memory};
use crate::Flow;

fn owner(m: &mut impl Memory, t: u32) -> u32 {
    m.read_u32(t + T::OWNER)
}

fn set_bits(m: &mut impl Memory, a: u32, bits: u8, on: bool) {
    let v = m.read_u8(a);
    m.write_u8(a, if on { v | bits } else { v & !bits });
}

/// Alta del brazo del jugador `p` (0 o 1): sprites, estado limpio, el
/// guion inicial y al bucle. Original: `PlayerArm_Spawn_0394a8` / `$0394E6`.
pub fn spawn(m: &mut impl Machine, p: usize) -> Flow {
    let t = m.task_register();
    for g in L::GFX[p] {
        anim::alloc_entry(m, t, g);
    }
    m.write_u16(t + L::KIND, L::KINDS[p]);
    slug::add_timer(m, t);
    // Tras reservar los sprites, el registro que usa el reinicio apunta a
    // la tabla de sprites.
    let stray = (m.read_u16(t + T::PAL_SLOT_COUNT) != 0).then_some(L::SPRITE_TABLE);
    reset(m, t, stray);
    set_bits(m, t + L::FLAGS_12, L::SECOND, p == 1);
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, a | L::ATTR_PRIO[p]);
    anim::start(m, t, L::START_SCRIPT);
    m.write_u32(t + T::HANDLER, L::RUN);
    run(m)
}

/// Estado inicial del brazo. Por un registro sin inicializar, el original
/// también borra el byte `STRAY` de la zona a la que apunte `stray` (si es
/// RAM). Original: `PlayerArm_ResetState_0392f8`.
pub fn reset(m: &mut impl Memory, t: u32, stray: Option<u32>) {
    let v = m.read_u16(t + L::SPRITE_ATTR_BASE);
    m.write_u16(t + L::SPRITE_ATTR, v);
    set_bits(m, t + L::IGNORE, L::PLAIN_FLOOR, true);
    for f in L::LEVELS {
        m.write_u8(t + f, 0xFF);
    }
    slug::mark_timer_max(m, t);
    if let Some(a) = stray {
        m.write_u8(a + L::STRAY, 0);
    }
    m.write_u8(t + L::STATUS, 0);
    m.write_u16(t + L::MODE, 0);
    m.write_u16(t + L::FIELD_36, 0);
    m.write_u8(t + L::BYTE_91, 0);
    set_bits(m, t + L::FLAGS_13, 1 << 3, false);
    m.write_u32(t + L::ATTACK, L::NO_ATTACK);
}

/// Un frame del brazo: se pega al cuerpo y, según el estado de éste,
/// ejecuta la pose que pide. Original: `PlayerArm_Spawn_0394a8__L039532`.
pub fn run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    follow(m, t);
    let o = owner(m, t);
    let state = m.read_u8(o + L::OWNER_STATE);
    let wants = m.read_u8(o + L::OWNER_FLAGS_69) & L::OWNER_WANTS_POSE != 0;
    let pose = m.read_u32(o + L::OWNER_POSE);
    let mode = m.read_u16(o + L::MODE);
    if let Some(&(_, pair)) = L::STATES_REPOSE.iter().find(|s| s.0 == state) {
        // De pie o andando: si el brazo no está ocupado sigue al cuerpo; si
        // lo está, sólo repite la pose de pie/andar cuando cambia el modo.
        let busy = m.read_u8(t + L::STATUS) & L::STATUS_BIT2 != 0;
        if !busy && wants {
            take_pose(m, t, o, pose);
        } else if pair.contains(&pose) && mode != m.read_u16(t + L::MODE) {
            m.write_u16(t + L::MODE, mode);
            run_pose(m, t, pose);
        }
    } else if L::STATES_KEEP.contains(&state) {
        if wants {
            take_pose(m, t, o, pose);
        }
    } else if let Some(&(_, air)) = L::STATES_AIR.iter().find(|s| s.0 == state) {
        if wants {
            take_pose(m, t, o, pose);
        } else if pose == air && mode != m.read_u16(t + L::MODE) {
            run_pose(m, t, pose);
            m.write_u16(t + L::MODE, mode);
        }
    } else {
        // Resto de estados (los válidos son < $34; el original se cuelga con
        // uno mayor): la pose, si la pide, y el modo a 0.
        if wants {
            take_pose(m, t, o, pose);
        }
        m.write_u16(t + L::MODE, 0);
    }
    finish(m, t);
    Flow::Return
}

/// Ejecuta la pose y copia el modo del cuerpo.
fn take_pose(m: &mut impl Machine, t: u32, o: u32, pose: u32) {
    run_pose(m, t, pose);
    let mode = m.read_u16(o + L::MODE);
    m.write_u16(t + L::MODE, mode);
}

/// Cierre de cada frame: avanza la animación y le pasa al cuerpo el bit 2.
fn finish(m: &mut impl Machine, t: u32) {
    step(m, t);
    copy_bit2(m, t);
}

/// Se pega al cuerpo: posición más el desplazamiento del brazo (en espejo
/// si mira a la izquierda), orientación, flags y atributo del sprite.
/// Original: `PlayerArm_FollowParent_03937c`.
pub fn follow(m: &mut impl Memory, t: u32) {
    slug::follow_owner(m, t);
    let o = owner(m, t);
    let dx = m.read_u16(o + L::OFFSET_X);
    let dy = m.read_u16(o + L::OFFSET_Y);
    let facing = m.read_u8(o + L::FACING);
    let x = m.read_u16(t + L::X);
    let x = if facing & L::FACING_LEFT == 0 { x.wrapping_add(dx) } else { x.wrapping_sub(dx) };
    m.write_u16(t + L::X, x);
    let y = m.read_u16(t + L::Y).wrapping_add(dy);
    m.write_u16(t + L::Y, y);
    m.write_u8(t + L::FACING, facing);
    if m.read_u8(o + L::FLAGS_5A) & L::FLAG_5A_7 != 0 {
        set_bits(m, t + L::FLAGS_5A, L::FLAG_5A_7, true);
    }
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, a & L::ATTR_CLEAR_PRIO);
    let b = m.read_u8(o + L::BYTE_59);
    m.write_u8(t + L::BYTE_59, b);
    copy_blink(m, t);
}

/// Copia el contador de parpadeo del cuerpo y elige el atributo del sprite
/// con su nibble bajo. Original: `Entity_DispatchOpcodeNibble_032D00`.
pub fn copy_blink(m: &mut impl Memory, t: u32) {
    let o = owner(m, t);
    let n = m.read_u8(o + L::INVULN);
    m.write_u8(t + L::INVULN, n);
    let field = m.read_u16(L::ATTR_FIELDS + u32::from(n & 0x0F) * 2);
    let v = m.read_u16(t.wrapping_add(field as i16 as u32));
    m.write_u16(t + L::SPRITE_ATTR, v);
}

/// Índice (×4) en las tablas de guiones: el arma, desplazada en el segundo
/// jugador. Original: `PlayerArm_WeaponTableIndex_03933a`.
pub fn weapon_index(m: &mut impl Memory, t: u32) -> u16 {
    let o = owner(m, t);
    let mut w = u16::from(m.read_u8(o + L::OWNER_WEAPON));
    if o == L::SECOND_PLAYER {
        w += L::SECOND_WEAPON_BASE;
    }
    w << 2
}

/// Avanza la animación; `+$21` del cuerpo dice si sigue (FF) o acabó (0).
/// Original: `PlayerArm_AnimStepToParentDeath_039354` (C = acabó).
pub fn step(m: &mut impl Machine, t: u32) -> bool {
    let done = anim::step(m, t);
    let o = owner(m, t);
    m.write_u8(o + L::OWNER_BYTE_21, if done { 0 } else { 0xFF });
    done
}

/// El bit 2 de `+$8C` del brazo pasa al cuerpo.
/// Original: `PlayerArm_CopyBit2ToParent_039416`.
pub fn copy_bit2(m: &mut impl Memory, t: u32) {
    let o = owner(m, t);
    let bit = m.read_u8(t + L::STATUS) & L::STATUS_BIT2;
    let v = m.read_u8(o + L::STATUS) & !L::STATUS_BIT2;
    m.write_u8(o + L::STATUS, v | bit);
}

/// Cómo elige cada pose su tabla.
#[derive(Clone, Copy)]
enum Rule {
    /// Siempre la primera.
    Plain,
    /// La primera y luego cierra el frame (modo a 0, animación, bit 2).
    Finish,
    /// Marca el brazo como muerto.
    Death,
    /// Según el modo propio: la del primer valor que coincide; si ninguno,
    /// la última.
    Mode(&'static [u16]),
    /// De pie / andando: si el modo del cuerpo es `mode`, la pose gemela
    /// `alt`; si no, la primera si el historial de la dirección es
    /// `history` y la segunda si no.
    Stand { mode: u16, alt: u32, history: u8 },
    /// La segunda si el cuerpo apunta arriba.
    AimUp,
    /// La segunda si el cuerpo tiene bombas.
    Bombs,
    /// Alterna entre las dos cada vez y fija la lista de golpes cuerpo a
    /// cuerpo.
    Toggle,
}

const POSES: &[(L::Pose, Rule)] = &[
    (L::POSE00, Rule::Finish),
    (L::CROUCH, Rule::Plain),
    (L::CROUCH_B, Rule::Plain),
    (L::IDLE, Rule::Plain),
    (L::CROUCH_WEAPON_EMPTY, Rule::Plain),
    (L::SPAWN_LAND, Rule::Plain),
    (L::SPAWN_LAND_B, Rule::Plain),
    (L::SHOOT_STAND_A, Rule::Plain),
    (L::SHOOT_STAND_B, Rule::Plain),
    (L::SHOOT_STAND_C, Rule::Plain),
    (L::SHOOT_STAND_D, Rule::Plain),
    (L::SHOOT_STAND_UP_A, Rule::AimUp),
    (L::SHOOT_STAND_UP_B, Rule::AimUp),
    (L::MELEE_A, Rule::Plain),
    (L::MELEE_B, Rule::Plain),
    (L::POSE, Rule::Plain),
    (L::STAND, Rule::Stand { mode: 1, alt: L::STAND_ALT.0, history: 0x41 }),
    (L::STAND_ALT, Rule::Stand { mode: 0, alt: L::STAND.0, history: 0x14 }),
    (L::WALK, Rule::Stand { mode: 1, alt: L::WALK_ALT.0, history: 0x41 }),
    (L::WALK_ALT, Rule::Stand { mode: 0, alt: L::WALK.0, history: 0x14 }),
    (L::WALK_LOOP, Rule::Stand { mode: 1, alt: L::WALK_LOOP_ALT.0, history: 0x41 }),
    (L::WALK_LOOP_ALT, Rule::Stand { mode: 0, alt: L::WALK_LOOP.0, history: 0x14 }),
    (L::WALK_SHOOT_A, Rule::Plain),
    (L::WALK_SHOOT_B, Rule::Plain),
    (L::WALK_SHOOT_C, Rule::Plain),
    (L::WALK_SHOOT_D, Rule::Plain),
    (L::WALK_SHOOT_E, Rule::Plain),
    (L::WALK_SHOOT_F, Rule::Plain),
    (L::TURN_A, Rule::Plain),
    (L::TURN_B, Rule::Plain),
    (L::JUMP_MELEE, Rule::Toggle),
    (L::JUMP, Rule::Plain),
    (L::FALL, Rule::Plain),
    (L::AIR_UNREF, Rule::Plain),
    (L::AIR_B, Rule::Plain),
    (L::AIR_SHOOT_A, Rule::Mode(&[1, 3])),
    (L::AIR_SHOOT_A2, Rule::Mode(&[1, 3])),
    (L::AIR_SHOOT_B, Rule::Mode(&[0, 1])),
    (L::AIR_SHOOT_C, Rule::Mode(&[0, 1])),
    (L::JUMP_SHOOT_A, Rule::Mode(&[1, 2, 3])),
    (L::FALL_SHOOT_A, Rule::Mode(&[1, 2, 3])),
    (L::JUMP_SHOOT_B, Rule::Mode(&[1, 2, 3])),
    (L::FALL_SHOOT_B, Rule::Mode(&[1, 2, 3])),
    (L::JUMP_SHOOT_C, Rule::Mode(&[1, 0, 3])),
    (L::FALL_SHOOT_C, Rule::Mode(&[1, 0, 3])),
    (L::JUMP_SHOOT_D, Rule::Mode(&[1, 0, 3])),
    (L::FALL_SHOOT_D, Rule::Mode(&[1, 0, 3])),
    (L::JUMP_SHOOT_DOWN_A, Rule::Mode(&[0, 2])),
    (L::FALL_SHOOT_DOWN_A, Rule::Mode(&[0, 2])),
    (L::JUMP_SHOOT_DOWN_B, Rule::Mode(&[0, 2])),
    (L::FALL_SHOOT_DOWN_B, Rule::Mode(&[0, 2])),
    (L::JUMP_SHOOT_DOWN_C, Rule::Mode(&[0, 2])),
    (L::FALL_SHOOT_DOWN_C, Rule::Mode(&[0, 2])),
    (L::SHOOT_DOWN_UNREF_A, Rule::Mode(&[0, 2])),
    (L::SHOOT_DOWN_UNREF_B, Rule::Mode(&[0, 2])),
    (L::DEATH_A, Rule::Death),
    (L::DEATH_B, Rule::Death),
    (L::DEATH_C, Rule::Death),
    (L::DEATH_D, Rule::Death),
    (L::DEATH_E, Rule::Death),
    (L::DEATH_F, Rule::Death),
    (L::DEATH_G, Rule::Death),
    (L::DEATH_H, Rule::Death),
    (L::SPAWN_FALL, Rule::Plain),
    (L::CROUCH_ENTER_A, Rule::Plain),
    (L::CROUCH_UNREF, Rule::Plain),
    (L::CROUCH_ENTER_B, Rule::Plain),
    (L::CROUCH_IDLE_A, Rule::Plain),
    (L::CROUCH_IDLE_B, Rule::Plain),
    (L::CROUCH_SHOOT, Rule::Plain),
    (L::CRAWL_A, Rule::Plain),
    (L::CRAWL_UNREF, Rule::Plain),
    (L::CRAWL_B, Rule::Plain),
    (L::CROUCH_GRENADE, Rule::Plain),
    (L::MELEE_C, Rule::Bombs),
    (L::AIR_MELEE_A, Rule::Mode(&[1, 2])),
    (L::AIR_MELEE_B, Rule::Mode(&[1, 0])),
    (L::CROUCH_MELEE, Rule::Plain),
    (L::CROUCH_RELOAD, Rule::Toggle),
];

/// Ejecuta la pose `routine` (o la rutina original si no es del brazo).
pub fn run_pose(m: &mut impl Machine, t: u32, routine: u32) {
    match POSES.iter().find(|((e, _), _)| *e == routine) {
        Some(&((_, tables), rule)) => pose(m, t, tables, rule),
        None => {
            let mut r = m.regs();
            r.a[1] = routine;
            m.call_original(routine, &mut r);
            m.set_regs(r);
        }
    }
}

/// Una pose del brazo. Original: `PlayerArm_<pose>`.
pub fn pose_at(m: &mut impl Machine, routine: u32) -> Flow {
    let t = m.task_register();
    run_pose(m, t, routine);
    Flow::Return
}

fn pose(m: &mut impl Machine, t: u32, tables: &[u32], rule: Rule) {
    set_bits(m, t + L::STATUS, L::STATUS_BIT2, false);
    let o = owner(m, t);
    let k = match rule {
        Rule::Plain | Rule::Finish => 0,
        Rule::Death => {
            set_bits(m, t + L::FLAGS_13, L::DEAD, true);
            0
        }
        Rule::Mode(modes) => {
            let mode = m.read_u16(t + L::MODE);
            modes.iter().position(|&v| v == mode).unwrap_or(modes.len())
        }
        Rule::Stand { mode, alt, history } => {
            if m.read_u16(o + L::MODE) == mode {
                return run_pose(m, t, alt);
            }
            usize::from(m.read_u8(o + L::OWNER_HISTORY) != history)
        }
        Rule::AimUp => usize::from(m.read_u8(o + L::OWNER_AIM) & L::OWNER_AIM_UP != 0),
        Rule::Bombs => usize::from(m.read_u8(o + L::OWNER_BOMBS) != 0),
        Rule::Toggle => {
            let n = m.read_u8(t + L::TOGGLE).wrapping_add(1);
            m.write_u8(t + L::TOGGLE, n);
            usize::from(n & 1)
        }
    };
    let i = weapon_index(m, t);
    let script = m.read_u32(tables[k] + u32::from(i));
    anim::start(m, t, script);
    match rule {
        Rule::Finish => {
            m.write_u16(t + L::MODE, 0);
            finish(m, t);
        }
        Rule::Toggle => {
            m.write_u32(t + L::HIT_LIST, L::MELEE_HITS);
            slug::clear_flags13(m, t);
        }
        _ => {}
    }
}

/// Llamada del guion: un valor al azar (0, 2, 8 o 10) en `+$47`.
/// Original: `AnimCb_RandField47_039450`. Devuelve lo que da el generador.
pub fn random_47(m: &mut impl Memory, t: u32) -> (u16, u16) {
    let (v, j) = rng::next(m);
    m.write_u8(t + L::RAND_47, v as u8 & L::RAND_47_MASK);
    (v, j)
}

/// Llamada del guion: golpe cuerpo a cuerpo (limpia los flags del frame y
/// prueba su lista de golpes); si acierta y toca, suena el impacto.
/// Original: `AnimCb_AttackProbeMusic_03942e`.
pub fn melee_probe(m: &mut impl Memory, t: u32) -> (hits::Outcome, u32) {
    slug::clear_flags13(m, t);
    let r = hits::run_attack_list(m, t);
    if m.read_u8(t + L::FLAGS_5A) & L::HIT_SOUND != 0 {
        sound::play(m, L::MELEE_SOUND);
    }
    r
}
