//! Jugador en el suelo: el aterrizaje (de pie, agachado, recargando o a la
//! espera), la espera con la pistola (que suelta el arma que llevara) y la
//! cola común de los estados de pie, que elige el siguiente estado según
//! el mando: andar, girarse, agacharse, disparar, lanzar, saltar o caer.
//!
//! Original: `Player_SpawnLand_Reset_033ede` (y su bucle `$033FAE`),
//! `Player_Stand_034704` (y su bucle `$034806`), el disparo de pie
//! (`Player_ShootStand_0342c4`, `Player_ShootStandUp_03437e`), el giro
//! (`Player_TurnRight_035aba`, `Player_TurnLeft_035bf8` y sus bucles), la
//! cola de andar (`Player_Walk_Tail_03548c`), las poses de espera
//! (`Player_Crouch_033a5e`, `Player_CrouchB_033afc`), lanzar una granada de
//! pie (`Player_ThrowGrenade_Stand_0360bc`), andar (`Player_WalkRight_034b38`,
//! `Player_WalkLeft_034d32`, y ya lanzado `Player_WalkLoopRight_0351d8`,
//! `Player_WalkLoopLeft_0354f2`), parar (`Player_Idle_StateCheck_034046`), el
//! agacharse
//! (`Player_CrouchEnter_037c74`) y el agachado quieto
//! (`Player_CrouchIdle_03827a`, `$0381A2`, `$03820E`), el disparo agachado
//! (`Player_CrouchShoot_03873c`), el levantarse (`Player_CrouchExit_037ec2`,
//! `asm/player/air_death_crouch_036632.s`),
//! `Player_Idle_033d64` (y su bucle `$033E2E`), `Player_Idle_Tail_0341a4`
//! (`asm/player/core_032aa8.s`), `Player_PlayWeaponMusicIfFlag_032d32`,
//! `$0267E6`, `Entity_ProbeRevertCcr_027a92` y
//! `Entity_MoveAndCollide_A_026b56` (gemela de `$027036`).

use super::{anim, collision, items, player, player_air, rng, slope, slug, sound, tasks};
use crate::entity::Entity;
use crate::input::{self, Buttons, Edge};
use crate::layout::{player_air as A, player_ground as L, task as T};
use crate::mem::{Machine, Memory};
use crate::player::{controls, voice};
use crate::Flow;

fn set_bits(m: &mut impl Memory, a: u32, bits: u8, on: bool) {
    let v = m.read_u8(a);
    m.write_u8(a, if on { v | bits } else { v & !bits });
}

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

fn held(m: &mut impl Memory, t: u32, b: Buttons) -> bool {
    input::pressed(m, Entity::from_raw(t), b, Edge::Held)
}

/// Pose del cuerpo: estado y la pose que hay antes del guion.
fn set_pose(m: &mut impl Memory, t: u32, state: u8, script: u32) {
    m.write_u8(t + L::STATE, state);
    let p = m.read_u32(script - 4);
    m.write_u32(t + L::POSE, p);
}

/// Pose y además el guion desde el principio.
fn restart(m: &mut impl Memory, t: u32, state: u8, script: u32) {
    set_pose(m, t, state, script);
    m.write_u8(t + L::BYTE_21, 0xFF);
    anim::start(m, t, script);
}

fn heavy_needs_reload(m: &mut impl Memory, t: u32) -> bool {
    m.read_u8(t + L::RELOADED) == 0 && m.read_u8(t + L::WEAPON) == L::WEAPON_HEAVY
}

fn wants_crouch(m: &mut impl Memory, t: u32) -> bool {
    m.read_u8(t + L::FLAGS_88) & L::WANTS_CROUCH != 0
}

/// Velocidades y aceleraciones horizontales y fracciones a 0.
/// Original: `$0267E6`.
pub fn clear_motion(m: &mut impl Memory, t: u32) {
    m.write_u16(t + L::ACC_X, 0);
    m.write_u32(t + L::VEL_X, 0);
    m.write_u16(t + L::SUB_X, 0);
}

/// El sistema de sonido como salida de las voces del jugador.
struct SoundOut;

impl<M: Memory> voice::Sound<M> for SoundOut {
    fn play(&mut self, m: &mut M, code: u16) {
        sound::play(m, code);
    }
}

/// Si acaba de coger un arma, la anuncia con su voz.
/// Original: `Player_PlayWeaponMusicIfFlag_032d32` (C = había arma nueva).
pub fn announce_weapon(m: &mut impl Memory, t: u32) -> bool {
    voice::announce_new_weapon(m, &mut SoundOut, Entity::from_raw(t))
}

/// Un paso sobre el suelo con la posición de la entidad (pendientes,
/// rampas, paredes) y el sombreado por profundidad. Devuelve si ha vuelto a
/// sondear el suelo. Original: `Entity_ProbeRevertCcr_027a92`.
pub fn move_ground(m: &mut impl Machine, t: u32) -> bool {
    let r = m.regs();
    slope::move_b_entity(m, t, (r.d[3] as u16, r.d[4] as u16))
}

/// Aterrizaje tras bajarse del Slug: un rato invulnerable (si la escena
/// está cargada, también al golpe) y el aterrizaje normal.
/// Original: `Player_SpawnLand_SetInvuln1E_033ec2`.
pub fn land_invuln(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if m.read_u8(L::LAND_SCENE_LOADED) != 0 {
        m.write_u8(t + L::LAND_TIMER_45, L::LAND_INVULN_FRAMES);
    }
    m.write_u8(t + L::LAND_TIMER_59, L::LAND_INVULN_FRAMES);
    land(m)
}

/// Aterrizaje: limpia el brazo, pose y animación de aterrizar y, según el
/// mando y el arma, se agacha, pasa a la espera, recarga o se queda de pie.
/// Original: `Player_SpawnLand_Reset_033ede`.
pub fn land(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    set_bits(m, t + L::STATUS, L::RIDING, false);
    if m.read_u16(t + L::MODE) == L::MODE_FORWARD {
        m.write_u16(t + L::MODE, 0);
    }
    m.write_u16(t + L::OFFSET_X, 0);
    m.write_u16(t + L::OFFSET_Y, 0);
    set_bits(m, t + L::STATUS, L::ARM_BUSY | L::ARM_FIRING | L::STATUS_BIT3, false);
    restart(m, t, L::STATE_LANDED, L::LAND_SCRIPT);
    set_bits(m, t + L::FLAGS_5B, L::GROUNDED, true);
    if controls::down_held(m, Entity::from_raw(t)) {
        return Flow::Goto(L::CROUCH_IDLE);
    }
    if m.read_u16(t + L::AMMO) == 0 {
        set_handler(m, t, L::IDLE);
        return idle(m);
    }
    if heavy_needs_reload(m, t) {
        set_handler(m, t, L::RELOAD);
        return Flow::Goto(L::RELOAD);
    }
    if wants_crouch(m, t) {
        set_handler(m, t, L::CROUCH);
        return Flow::Goto(L::CROUCH);
    }
    m.write_u32(t + L::HIT_HANDLER, L::HIT_ROUTINE);
    clear_motion(m, t);
    m.write_u16(t + L::VEL_X, 0);
    m.write_u32(t + L::HITBOX_CB, L::HITBOX_STAND);
    set_handler(m, t, L::LAND_LOOP);
    land_loop(m)
}

/// Bucle del aterrizaje: al acabar la animación, de pie (o agachado,
/// recargando o a la espera). Original: `Player_SpawnLand_Reset_033ede__L033fae`.
pub fn land_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    player::frame_common(m, t);
    if announce_weapon(m, t) {
        set_handler(m, t, L::REENTER);
    }
    move_ground(m, t);
    if player::anim_done(m, t) {
        set_handler(m, t, L::STAND);
        stand_aim(m, t);
        if wants_crouch(m, t) {
            set_handler(m, t, L::CROUCH);
        }
        if heavy_needs_reload(m, t) {
            set_handler(m, t, L::RELOAD);
        }
        // El original pide un número al azar y no lo usa.
        rng::next(m);
        if m.read_u8(t + L::STATE) == L::STATE_LANDED && m.read_u16(t + L::AMMO) == L::CROUCH_AMMO {
            set_handler(m, t, L::CROUCH_B);
        }
        if m.read_u16(t + L::AMMO) == 0 {
            set_handler(m, t, L::IDLE);
        }
    }
    idle_tail(m, t);
    Flow::Return
}

/// Apunta adelante; el historial lleva en el nibble alto los flags del
/// último trozo dibujado (resto de registro del original).
fn stand_aim(m: &mut impl Memory, t: u32) {
    m.write_u8(t + L::AIM, L::AIM_FORWARD);
    let prev = anim::last_part_flags(m, t);
    m.write_u8(t + L::AIM_HISTORY, (prev << 4) | L::AIM_FORWARD);
}

/// Espera con la pistola: suelta el arma que llevara (con su munición
/// pasa a la de la pistola) y queda quieta con la animación de espera.
/// Original: `Player_Idle_033d64`.
pub fn idle(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::OFFSET_X, 0);
    m.write_u16(t + L::OFFSET_Y, 0);
    set_bits(m, t + L::STATUS, L::ARM_BUSY, true);
    set_bits(m, t + L::STATUS, L::ARM_FIRING | L::STATUS_BIT3, false);
    let state = m.read_u8(t + L::STATE);
    let weapon = m.read_u8(t + L::WEAPON);
    if L::KEEP_ANIM.contains(&state) && weapon != L::WEAPON_PISTOL {
        set_pose(m, t, L::STATE_IDLE, L::IDLE_SCRIPT);
    } else {
        restart(m, t, L::STATE_IDLE, L::IDLE_SCRIPT);
    }
    m.write_u32(t + L::HITBOX_CB, L::HITBOX_STAND);
    if weapon != L::WEAPON_PISTOL {
        m.write_u32(t + L::HITBOX_CB, L::NO_HITBOX);
        let drop = tasks::spawn(m, L::DROPPED_WEAPON, t);
        anim::copy_sprite_pos(m, t, drop);
    }
    m.write_u8(t + L::WEAPON, L::WEAPON_PISTOL);
    m.write_u16(t + L::AMMO, L::PISTOL_AMMO);
    set_bits(m, t + L::FLAGS_5B, L::GROUNDED, true);
    m.write_u32(t + L::HIT_HANDLER, L::HIT_ROUTINE);
    clear_motion(m, t);
    m.write_u16(t + L::VEL_X, 0);
    set_handler(m, t, L::IDLE_LOOP);
    idle_loop(m)
}

/// Bucle de la espera: al coger un arma vuelve a entrar; al acabar la
/// animación, de pie o andando según la velocidad.
/// Original: `Player_Idle_033d64__L033e2e`.
pub fn idle_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if m.read_u8(t + L::STATUS) & L::NEW_WEAPON != 0 {
        announce_weapon(m, t);
        set_handler(m, t, L::REENTER);
        return Flow::Goto(L::REENTER);
    }
    player::frame_common(m, t);
    move_ground(m, t);
    if player::anim_done(m, t) {
        set_handler(m, t, L::STAND);
        let vx = m.read_u16(t + L::VEL_X) as i16;
        if vx > 0 {
            set_handler(m, t, L::WALK_RIGHT);
        } else if vx < 0 {
            set_handler(m, t, L::WALK_LEFT);
        }
    }
    idle_tail(m, t);
    Flow::Return
}

/// Cola común de pie: con el brazo libre, andar o girarse según la
/// dirección, agacharse y la acción (cuchillo, granada, disparo); sin
/// suelo, el golpe; saltar; la publicación del estado y la caída al vacío.
/// Original: `Player_Idle_Tail_0341a4`.
pub fn idle_tail(m: &mut impl Machine, t: u32) {
    if m.read_u8(t + L::STATUS) & L::ARM_BUSY == 0 {
        let left = m.read_u8(t + L::FACING) & L::FACING_LEFT != 0;
        let (to_right, to_left) = if left { (L::TURN_TO_RIGHT, L::WALK_LEFT) } else { (L::WALK_RIGHT, L::TURN_TO_LEFT) };
        if held(m, t, Buttons::RIGHT) {
            set_handler(m, t, to_right);
        }
        if held(m, t, Buttons::LEFT) {
            set_handler(m, t, to_left);
        }
        if controls::down_held(m, Entity::from_raw(t)) {
            set_handler(m, t, L::CROUCH_ENTER);
        }
        if let Some(a) = player_air::action_select(m, t) {
            let h = match a {
                0xFF => L::MELEE,
                3 => L::THROW,
                4 => L::SHOOT_DOWN,
                1 => L::SHOOT_UP,
                _ => L::SHOOT,
            };
            set_handler(m, t, h);
        }
    }
    let r = m.regs();
    if collision::probe_floor(m, t, (r.d[3] as u16, r.d[4] as u16)).in_air {
        // El original pone el golpe y, a continuación (el flag Z de la
        // copia del puntero nunca está a 1), la espera del golpe sin tiempo
        // límite.
        set_handler(m, t, L::KNOCK_HOLD);
    }
    if controls::jump_held(m, Entity::from_raw(t)).is_some() {
        set_handler(m, t, L::JUMP_START);
    }
    player_air::publish_state(m, t);
    let pit = m.read_u8(A::SCENE) == A::PIT_SCENE;
    if items::out_of_view_seen(m, t, Some(A::PIT_MARGINS[usize::from(pit)])) {
        set_handler(m, t, A::DEATH_PIT);
    }
}

/// Acción pedida → estado. Con `arm_bit3` sólo distingue arriba y normal.
fn action_state(a: u8, arm_bit3: bool) -> u32 {
    match a {
        0xFF if !arm_bit3 => L::MELEE,
        3 if !arm_bit3 => L::THROW,
        4 if !arm_bit3 => L::SHOOT_DOWN,
        1 => L::SHOOT_UP,
        _ => L::SHOOT,
    }
}

/// Pose de pie según hacia dónde apunta (arriba o adelante); `restart_fwd`
/// arranca además el guion de adelante. Devuelve si apunta arriba.
fn stand_pose(m: &mut impl Memory, t: u32, restart_fwd: bool) -> bool {
    let up = m.read_u8(t + L::AIM) == L::AIM_UP;
    if up {
        m.write_u16(t + L::MODE, L::MODE_UP);
        set_pose(m, t, L::STATE_STAND, L::STAND_UP_SCRIPT);
    } else {
        m.write_u16(t + L::MODE, 0);
        if restart_fwd {
            restart(m, t, L::STATE_STAND, L::STAND_SCRIPT);
        } else {
            set_pose(m, t, L::STATE_STAND, L::STAND_SCRIPT);
        }
    }
    up
}

/// De pie: con una pose de brazo de las que se conservan sólo rearranca
/// el guion; si no, limpia el brazo y pone la pose de pie. Después, sin
/// movimiento y con la caja de golpe de pie, sigue en el bucle.
/// Original: `Player_Stand_034704` (y `Player_Stand_Setup_0347e0`).
pub fn stand(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::OFFSET_X, 0);
    m.write_u16(t + L::OFFSET_Y, 0);
    if L::KEEP_ANIM.contains(&m.read_u8(t + L::STATE)) {
        anim::start(m, t, L::STAND_SCRIPT);
        set_bits(m, t + L::FLAGS_69, L::WANTS_POSE, false);
    } else {
        set_bits(m, t + L::STATUS, L::ARM_BUSY | L::ARM_FIRING | L::STATUS_BIT3, false);
        restart(m, t, L::STATE_STAND, L::STAND_SCRIPT);
    }
    stand_setup(m, t)
}

/// Segunda mitad de la entrada de pie: en el suelo, sin movimiento, caja
/// de golpe de pie y el bucle de pie. Original: `Player_Stand_Setup_0347e0`.
fn stand_setup(m: &mut impl Machine, t: u32) -> Flow {
    set_bits(m, t + L::FLAGS_5B, L::GROUNDED, true);
    m.write_u32(t + L::HIT_HANDLER, L::HIT_ROUTINE);
    clear_motion(m, t);
    m.write_u16(t + L::VEL_X, 0);
    m.write_u32(t + L::HITBOX_CB, L::HITBOX_STAND);
    set_handler(m, t, L::STAND_LOOP);
    stand_loop(m)
}

/// Bucle de pie: al acabar la animación rehace la pose (y puede pasar a
/// agacharse, recargar o la espera); sigue la pose del brazo si cambia el
/// apuntado; con el brazo libre anda, se gira o se agacha; la acción; y la
/// cola: golpe sin suelo, salto, estado y caída al vacío.
/// Original: `Player_Stand_034704__L034806` (con `Player_Stand_InputMove_034a00`
/// y `Player_Stand_Tail_034ada`).
pub fn stand_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    player::frame_common(m, t);
    if announce_weapon(m, t) {
        set_handler(m, t, L::REENTER);
    }
    move_ground(m, t);
    if player::anim_done(m, t) {
        m.write_u16(t + L::OFFSET_X, 0);
        m.write_u16(t + L::OFFSET_Y, 0);
        set_bits(m, t + L::STATUS, L::ARM_BUSY | L::ARM_FIRING | L::STATUS_BIT3, false);
        if stand_pose(m, t, true) {
            set_bits(m, t + L::FLAGS_69, L::WANTS_POSE, true);
        }
        if wants_crouch(m, t) {
            set_handler(m, t, L::CROUCH);
        }
        if heavy_needs_reload(m, t) {
            set_handler(m, t, L::RELOAD);
        }
        // El original pide un número al azar y no lo usa.
        rng::next(m);
        let ammo = m.read_u16(t + L::AMMO);
        if ammo == L::CROUCH_AMMO {
            set_handler(m, t, L::CROUCH_B);
        }
        if ammo == 0 {
            set_handler(m, t, L::IDLE);
        }
    }
    if m.read_u8(t + L::STATE) == L::STATE_STAND {
        let up = m.read_u8(t + L::AIM) == L::AIM_UP;
        let mode = if up { L::MODE_UP } else { 0 };
        if m.read_u16(t + L::MODE) != mode {
            stand_pose(m, t, false);
            let want = if up { L::AIM_HISTORY_TO_UP } else { L::AIM_HISTORY_TO_FORWARD };
            if m.read_u8(t + L::AIM_HISTORY) == want {
                set_bits(m, t + L::FLAGS_69, L::WANTS_POSE, true);
            }
        }
    }
    stand_input(m, t)
}

/// Mando de pie: con el brazo libre anda, se gira o se agacha; la acción;
/// y la cola: golpe sin suelo, salto, estado y caída al vacío.
/// Original: `Player_Stand_InputMove_034a00` (y `Player_Stand_Tail_034ada`).
fn stand_input(m: &mut impl Machine, t: u32) -> Flow {
    let status = m.read_u8(t + L::STATUS);
    if status & L::ARM_BUSY == 0 {
        let left = m.read_u8(t + L::FACING) & L::FACING_LEFT != 0;
        let (to_right, to_left) = if left { (L::TURN_TO_RIGHT, L::WALK_LEFT) } else { (L::WALK_RIGHT, L::TURN_TO_LEFT) };
        if held(m, t, Buttons::RIGHT) {
            set_handler(m, t, to_right);
        }
        if held(m, t, Buttons::LEFT) {
            set_handler(m, t, to_left);
        }
        if controls::down_held(m, Entity::from_raw(t)) {
            set_handler(m, t, L::CROUCH_ENTER);
        }
    }
    if m.read_u8(t + L::WEAPON) == L::WEAPON_FLAME || status & L::ARM_BUSY == 0 {
        if let Some(a) = player_air::action_select(m, t) {
            let bit3 = m.read_u8(t + L::STATUS) & L::STATUS_BIT3 != 0;
            set_handler(m, t, action_state(a, bit3));
        }
    }
    let r = m.regs();
    if collision::probe_floor(m, t, (r.d[3] as u16, r.d[4] as u16)).in_air {
        set_handler(m, t, L::KNOCKBACK);
    }
    if controls::jump_held(m, Entity::from_raw(t)).is_some() {
        set_handler(m, t, L::JUMP_START);
    }
    player_air::publish_state(m, t);
    let pit = m.read_u8(A::SCENE) == A::PIT_SCENE;
    if items::out_of_view_seen(m, t, Some(A::PIT_MARGINS[usize::from(pit)])) {
        set_handler(m, t, A::DEATH_PIT);
    }
    Flow::Return
}

/// Parada tras andar: sin desplazamiento horizontal (y el vertical a -1),
/// brazo libre, en el suelo y sin movimiento, con la caja de golpe de pie,
/// y el bucle. Original: `Player_StandFromWalk_0345fa`.
pub fn stand_from_walk(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::OFFSET_X, 0);
    m.write_u16(t + L::OFFSET_Y, 0xFFFF);
    set_bits(m, t + L::STATUS, L::ARM_BUSY | L::ARM_FIRING | L::STATUS_BIT3, false);
    set_bits(m, t + L::FLAGS_5B, L::GROUNDED, true);
    m.write_u32(t + L::HIT_HANDLER, L::HIT_ROUTINE);
    clear_motion(m, t);
    m.write_u16(t + L::VEL_X, 0);
    m.write_u32(t + L::HITBOX_CB, L::HITBOX_STAND);
    set_handler(m, t, L::STAND_FROM_WALK_LOOP);
    stand_from_walk_loop(m)
}

/// Bucle de la parada: al acabar la animación pasa a de pie; si la pose
/// sigue siendo la de andar parado la cambia por la de pie (adelante o
/// arriba según el apuntado) y, apuntando adelante, puede agacharse; y el
/// mando de pie. Original: `Player_StandFromWalk_0345fa__L03463e`.
pub fn stand_from_walk_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    player::frame_common(m, t);
    // El original no mira si hay que anunciar el arma.
    announce_weapon(m, t);
    move_ground(m, t);
    if player::anim_done(m, t) {
        set_handler(m, t, L::STAND);
    }
    let up = m.read_u8(t + L::AIM) == L::AIM_UP;
    if m.read_u32(t + L::POSE) == L::STAND_FROM_WALK_POSE {
        let (mode, script) = if up { (L::MODE_UP, L::STAND_UP_SCRIPT) } else { (0, L::STAND_SCRIPT) };
        m.write_u16(t + L::MODE, mode);
        set_pose(m, t, L::STATE_STAND, script);
    }
    if !up && wants_crouch(m, t) {
        set_handler(m, t, L::CROUCH);
    }
    stand_input(m, t)
}

/// Disparo de pie, adelante (`up = false`) o hacia arriba: marca el brazo
/// disparando; si cambia de modo, pone la pose de disparo desde el
/// principio, y sigue como de pie.
/// Original: `Player_ShootStand_0342c4` / `Player_ShootStandUp_03437e`.
pub fn shoot_stand(m: &mut impl Machine, up: bool) -> Flow {
    let t = m.task_register();
    set_bits(m, t + L::STATUS, L::ARM_FIRING, true);
    let mode = if up { L::MODE_UP } else { 0 };
    let changed = m.read_u16(t + L::MODE) != mode;
    let script = match (up, changed) {
        (false, true) => L::SHOOT_SCRIPT_FROM_UP,
        (false, false) => L::SHOOT_SCRIPT,
        (true, true) => L::SHOOT_UP_SCRIPT_FROM_FWD,
        (true, false) => L::SHOOT_UP_SCRIPT,
    };
    if changed {
        m.write_u16(t + L::MODE, mode);
    }
    m.write_u16(t + L::OFFSET_X, 0);
    m.write_u16(t + L::OFFSET_Y, 0);
    restart(m, t, L::STATE_IDLE, script);
    stand_setup(m, t)
}

/// Giro hacia la derecha (`left = false`) o la izquierda: pose de giro
/// (apuntando arriba si toca), velocidad hacia ese lado y el bucle.
/// Original: `Player_TurnRight_035aba` / `Player_TurnLeft_035bf8`.
pub fn turn(m: &mut impl Machine, left: bool) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::OFFSET_X, 0);
    m.write_u16(t + L::OFFSET_Y, 0);
    set_bits(m, t + L::STATUS, L::ARM_BUSY | L::ARM_FIRING | L::STATUS_BIT3, false);
    let script = if m.read_u16(t + L::MODE) == L::MODE_UP { L::TURN_UP_SCRIPT } else { L::TURN_SCRIPT };
    restart(m, t, L::STATE_TURN, script);
    m.write_u16(t + L::VEL_X, turn_speed(left));
    m.write_u16(t + L::ACC_X, 0);
    m.write_u32(t + L::HIT_HANDLER, L::HIT_ROUTINE);
    m.write_u32(t + L::HITBOX_CB, L::HITBOX_STAND);
    let lp = if left { L::TURN_LEFT_LOOP } else { L::TURN_RIGHT_LOOP };
    set_handler(m, t, lp);
    set_bits(m, t + L::FACING, L::FACING_LEFT, left);
    turn_loop(m, left)
}

fn turn_speed(left: bool) -> u16 {
    if left { L::TURN_SPEED.wrapping_neg() } else { L::TURN_SPEED }
}

/// Bucle del giro: al acabar la animación, de pie (o agachado); con el
/// brazo libre, la acción; y la cola de andar.
/// Original: `Player_TurnRight_035aba__L035b54` / `Player_TurnLeft_Run_035c98`
/// (con `Player_Turn_Actions_035b90`).
pub fn turn_loop(m: &mut impl Machine, left: bool) -> Flow {
    let t = m.task_register();
    player::frame_common(m, t);
    if announce_weapon(m, t) {
        set_handler(m, t, L::REENTER);
    }
    m.write_u16(t + L::VEL_X, turn_speed(left));
    move_ground(m, t);
    if player::anim_done(m, t) {
        set_handler(m, t, L::STAND);
        if wants_crouch(m, t) {
            set_handler(m, t, L::CROUCH);
        }
    }
    if m.read_u8(t + L::STATUS) & L::ARM_BUSY == 0 {
        if let Some(a) = player_air::action_select(m, t) {
            set_handler(m, t, action_state(a, false));
        }
    }
    walk_tail(m, t);
    Flow::Return
}

/// Cola de andar: agacharse, recargar, la espera sin munición, el golpe
/// sin suelo, el salto y la publicación del estado.
/// Original: `Player_Walk_Tail_03548c`.
pub fn walk_tail(m: &mut impl Machine, t: u32) {
    if controls::down_held(m, Entity::from_raw(t)) {
        set_handler(m, t, L::CROUCH_ENTER);
    }
    if heavy_needs_reload(m, t) {
        set_handler(m, t, L::RELOAD);
    }
    if m.read_u16(t + L::AMMO) == 0 && m.read_u8(t + L::WEAPON) != L::WEAPON_PISTOL {
        set_handler(m, t, L::IDLE);
    }
    let r = m.regs();
    if collision::probe_floor(m, t, (r.d[3] as u16, r.d[4] as u16)).in_air {
        set_handler(m, t, L::KNOCKBACK);
    }
    if controls::jump_held(m, Entity::from_raw(t)).is_some() {
        set_handler(m, t, L::JUMP_START);
    }
    player_air::publish_state(m, t);
}

/// Acción pedida agachado → estado.
fn crouch_action(a: u8) -> u32 {
    match a {
        0xFF => L::CROUCH_MELEE,
        3 => L::CROUCH_THROW,
        _ => L::SHOOT_DOWN,
    }
}

/// Sin munición con un arma que no es la pistola.
fn weapon_empty(m: &mut impl Memory, t: u32) -> bool {
    m.read_u16(t + L::AMMO) == 0 && m.read_u8(t + L::WEAPON) != L::WEAPON_PISTOL
}

/// Velocidad al agacharse según el lado pulsado (izquierda gana).
fn crawl_speed(m: &mut impl Memory, t: u32) -> u16 {
    let mut v = 0;
    if held(m, t, Buttons::RIGHT) {
        v = L::CRAWL_SPEED;
    }
    if held(m, t, Buttons::LEFT) {
        v = L::CRAWL_SPEED.wrapping_neg();
    }
    v
}

/// Agacharse: limpia el brazo, pose y guion de agacharse, golpe y caja de
/// golpe agachado y el bucle. Original: `Player_CrouchEnter_037c74`.
pub fn crouch_enter(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::OFFSET_X, 0);
    m.write_u16(t + L::OFFSET_Y, 0);
    set_bits(m, t + L::STATUS, L::ARM_BUSY | L::ARM_FIRING | L::STATUS_BIT3, false);
    restart(m, t, L::STATE_CROUCH, L::CROUCH_SCRIPT);
    m.write_u32(t + L::HIT_HANDLER, L::CROUCH_HIT_ROUTINE);
    m.write_u16(t + L::ACC_X, 0);
    m.write_u32(t + L::HITBOX_CB, L::HITBOX_CROUCH);
    set_handler(m, t, L::CROUCH_ENTER_LOOP);
    crouch_enter_loop(m)
}

/// Bucle de agacharse: se desliza hacia el lado pulsado; al acabar la
/// animación, agachado (o gateando); la acción, recargar, arma vacía,
/// golpe sin suelo, salto y estado.
/// Original: `Player_CrouchEnter_037c74__L037cd0`.
pub fn crouch_enter_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    player::frame_common(m, t);
    announce_weapon(m, t);
    let v = crawl_speed(m, t);
    m.write_u16(t + L::VEL_X, v);
    move_ground(m, t);
    m.write_u16(t + L::VEL_X, 0);
    if player::anim_done(m, t) {
        set_handler(m, t, L::CROUCH_IDLE);
        if held(m, t, Buttons::RIGHT) {
            set_handler(m, t, L::CRAWL_RIGHT);
        }
        if held(m, t, Buttons::LEFT) {
            set_handler(m, t, L::CRAWL_LEFT);
        }
    }
    if let Some(a) = player_air::action_select(m, t) {
        set_handler(m, t, crouch_action(a));
    }
    if heavy_needs_reload(m, t) {
        set_handler(m, t, L::RELOAD);
    }
    if weapon_empty(m, t) {
        set_handler(m, t, L::CROUCH_EMPTY);
    }
    let r = m.regs();
    if collision::probe_floor(m, t, (r.d[3] as u16, r.d[4] as u16)).in_air {
        set_handler(m, t, L::KNOCKBACK);
    }
    if controls::jump_held(m, Entity::from_raw(t)).is_some() {
        set_handler(m, t, L::JUMP_START);
    }
    player_air::publish_state(m, t);
    Flow::Return
}

/// Agachado quieto: limpia el brazo, pose agachada desde el principio, sin
/// movimiento, golpe y caja de golpe agachado y el bucle.
/// Original: `Player_CrouchIdle_03827a` (con `Player_CrouchIdle_Setup_0382e2`;
/// la otra pose, `$279A82`, es de la rama de `$02ABCC` que nunca se toma).
pub fn crouch_idle(m: &mut impl Machine) -> Flow {
    crouch_idle_with(m, L::CROUCH_IDLE_SCRIPT)
}

/// Agachado quieto con otro guion: tras disparar (`$279850`) o lanzar
/// (`$27985A`). Original: `Player_CrouchIdleB_0381a2` /
/// `Player_CrouchIdleC_03820e`.
pub fn crouch_idle_with(m: &mut impl Machine, script: u32) -> Flow {
    let t = m.task_register();
    set_bits(m, t + L::STATUS, L::ARM_BUSY | L::ARM_FIRING | L::STATUS_BIT3, false);
    restart(m, t, L::STATE_CROUCH, script);
    m.write_u32(t + L::HIT_HANDLER, L::CROUCH_HIT_ROUTINE);
    clear_motion(m, t);
    m.write_u16(t + L::VEL_X, 0);
    m.write_u16(t + L::ACC_X, 0);
    m.write_u32(t + L::HITBOX_CB, L::HITBOX_CROUCH);
    set_handler(m, t, L::CROUCH_IDLE_LOOP);
    crouch_idle_loop(m)
}

/// Bucle agachado: gatea hacia el lado pulsado (hacia atrás con otra
/// rutina), se levanta al soltar abajo, la acción, arma vacía, golpe sin
/// suelo, salto, estado y caída al vacío.
/// Original: `Player_CrouchIdle_03827a__L038306`.
pub fn crouch_idle_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    set_bits(m, t + L::FLAGS_8D, L::CROUCHED, true);
    player::frame_common(m, t);
    if announce_weapon(m, t) {
        set_handler(m, t, L::CROUCH_REENTER);
    }
    move_ground(m, t);
    player::anim_done(m, t);
    let left = m.read_u8(t + L::FACING) & L::FACING_LEFT != 0;
    let (to_right, to_left) =
        if left { (L::CRAWL_RIGHT_BACK, L::CRAWL_LEFT) } else { (L::CRAWL_RIGHT, L::CRAWL_LEFT_BACK) };
    if held(m, t, Buttons::RIGHT) {
        set_handler(m, t, to_right);
    }
    if held(m, t, Buttons::LEFT) {
        set_handler(m, t, to_left);
    }
    if !controls::down_held(m, Entity::from_raw(t)) {
        set_handler(m, t, L::CROUCH_EXIT);
    }
    if let Some(a) = player_air::action_select(m, t) {
        set_handler(m, t, crouch_action(a));
    }
    if weapon_empty(m, t) {
        set_handler(m, t, L::CROUCH_EMPTY);
    }
    let r = m.regs();
    if collision::probe_floor(m, t, (r.d[3] as u16, r.d[4] as u16)).in_air {
        set_handler(m, t, L::KNOCKBACK_DELAY);
    }
    if controls::jump_held(m, Entity::from_raw(t)).is_some() {
        set_handler(m, t, L::JUMP_START);
    }
    player_air::publish_state(m, t);
    let pit = m.read_u8(A::SCENE) == A::PIT_SCENE;
    if items::out_of_view_seen(m, t, Some(A::PIT_MARGINS[usize::from(pit)])) {
        set_handler(m, t, A::DEATH_PIT);
    }
    Flow::Return
}

/// Gatear hacia la derecha (`left = false`) o la izquierda, hacia delante
/// o hacia atrás (`back`, otro guion): pose de gatear, brazo libre, golpe
/// y caja de golpe agachado, velocidad de gateo, mira hacia ese lado y el
/// bucle. Original: `Player_CrawlRight_03842c` / `Player_CrawlLeft_03855a`
/// (con sus `_Setup`), `Player_CrawlRight_Alt_038688` /
/// `Player_CrawlLeft_Alt_0386e2` (las dos ramas de `$02ABCC` usan el mismo
/// guion).
pub fn crawl(m: &mut impl Machine, left: bool, back: bool) -> Flow {
    let t = m.task_register();
    let script = if back { L::CRAWL_BACK_SCRIPT } else { L::CRAWL_SCRIPT };
    restart(m, t, L::STATE_CROUCH, script);
    set_bits(m, t + L::STATUS, L::ARM_BUSY | L::ARM_FIRING | L::STATUS_BIT3, false);
    m.write_u32(t + L::HIT_HANDLER, L::CROUCH_HIT_ROUTINE);
    m.write_u16(t + L::VEL_X, crawl_velocity(left));
    m.write_u16(t + L::ACC_X, 0);
    m.write_u32(t + L::HITBOX_CB, L::HITBOX_CROUCH);
    set_bits(m, t + L::FACING, L::FACING_LEFT, left);
    set_handler(m, t, if left { L::CRAWL_LEFT_LOOP } else { L::CRAWL_RIGHT_LOOP });
    crawl_loop(m, left)
}

fn crawl_velocity(left: bool) -> u16 {
    if left { L::CRAWL_MOVE_SPEED.wrapping_neg() } else { L::CRAWL_MOVE_SPEED }
}

/// Bucle de gatear: avanza; al soltar el lado, agachado quieto; al soltar
/// abajo, se levanta; la acción agachado, golpe sin suelo, salto y estado.
/// Original: `Player_CrawlRight_03842c__L0384ba` /
/// `Player_CrawlLeft_03855a__L0385e8`.
pub fn crawl_loop(m: &mut impl Machine, left: bool) -> Flow {
    let t = m.task_register();
    set_bits(m, t + L::FLAGS_8D, L::CROUCHED, true);
    player::frame_common(m, t);
    if announce_weapon(m, t) {
        set_handler(m, t, L::CROUCH_REENTER);
    }
    m.write_u16(t + L::VEL_X, crawl_velocity(left));
    move_ground(m, t);
    player::anim_done(m, t);
    if !held(m, t, if left { Buttons::LEFT } else { Buttons::RIGHT }) {
        set_handler(m, t, L::CROUCH_IDLE);
    }
    if !controls::down_held(m, Entity::from_raw(t)) {
        set_handler(m, t, L::CROUCH_EXIT);
    }
    if let Some(a) = player_air::action_select(m, t) {
        set_handler(m, t, crouch_action(a));
    }
    let r = m.regs();
    if collision::probe_floor(m, t, (r.d[3] as u16, r.d[4] as u16)).in_air {
        set_handler(m, t, L::KNOCKBACK_DELAY);
    }
    if controls::jump_held(m, Entity::from_raw(t)).is_some() {
        set_handler(m, t, L::JUMP_START);
    }
    player_air::publish_state(m, t);
    Flow::Return
}

/// Lanzar una granada agachado: el brazo no dispara, golpe agachado, sin
/// movimiento, pose y guion de lanzar agachado (las dos ramas de `$02ABCC`
/// usan el mismo), caja de golpe agachado y el bucle.
/// Original: `Player_CrouchThrowGrenade_0388f0`.
pub fn crouch_throw(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    // El original enciende y apaga el bit seguidos.
    set_bits(m, t + L::STATUS, L::ARM_FIRING, false);
    m.write_u32(t + L::HIT_HANDLER, L::CROUCH_HIT_ROUTINE);
    clear_motion(m, t);
    m.write_u16(t + L::VEL_X, 0);
    m.write_u16(t + L::ACC_X, 0);
    restart(m, t, L::STATE_CROUCH, L::CROUCH_THROW_SCRIPT);
    m.write_u32(t + L::HITBOX_CB, L::HITBOX_CROUCH);
    set_handler(m, t, L::CROUCH_THROW_LOOP);
    crouch_throw_loop(m)
}

/// Bucle de lanzar agachado: al acabar, agachado quieto (guion C); con el
/// brazo libre gatea hacia atrás y la acción; se levanta al soltar abajo,
/// golpe sin suelo y estado.
/// Original: `Player_CrouchThrowGrenade_0388f0__L038976` (con
/// `JsrPcThunk_038a22`).
pub fn crouch_throw_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    set_bits(m, t + L::FLAGS_8D, L::CROUCHED, true);
    player::frame_common(m, t);
    announce_weapon(m, t);
    move_ground(m, t);
    if player::anim_done(m, t) {
        set_handler(m, t, L::CROUCH_IDLE_C);
    }
    if m.read_u8(t + L::STATUS) & L::ARM_BUSY == 0 {
        if m.read_u8(t + L::FACING) & L::FACING_LEFT == 0 {
            if held(m, t, Buttons::LEFT) {
                set_handler(m, t, L::CRAWL_LEFT_BACK);
            }
        } else if held(m, t, Buttons::RIGHT) {
            set_handler(m, t, L::CRAWL_RIGHT_BACK);
        }
        if let Some(a) = player_air::action_select(m, t) {
            set_handler(m, t, crouch_action(a));
        }
    }
    if !controls::down_held(m, Entity::from_raw(t)) {
        set_handler(m, t, L::CROUCH_EXIT);
    }
    let r = m.regs();
    if collision::probe_floor(m, t, (r.d[3] as u16, r.d[4] as u16)).in_air {
        set_handler(m, t, L::KNOCKBACK_DELAY);
    }
    player_air::publish_state(m, t);
    Flow::Return
}

/// Caída al vacío con los márgenes de la escena.
pub fn check_pit(m: &mut impl Memory, t: u32) {
    let pit = m.read_u8(A::SCENE) == A::PIT_SCENE;
    if items::out_of_view_seen(m, t, Some(A::PIT_MARGINS[usize::from(pit)])) {
        set_handler(m, t, A::DEATH_PIT);
    }
}

/// Disparo agachado: el brazo no dispara, sin movimiento, pose y guion de
/// disparo agachado, caja de golpe agachado y el bucle.
/// Original: `Player_CrouchShoot_03873c` (las dos ramas de `$02ABCC` usan
/// el mismo guion).
pub fn crouch_shoot(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    set_bits(m, t + L::STATUS, L::ARM_FIRING, false);
    m.write_u32(t + L::HIT_HANDLER, L::CROUCH_HIT_ROUTINE);
    clear_motion(m, t);
    m.write_u16(t + L::VEL_X, 0);
    m.write_u16(t + L::ACC_X, 0);
    restart(m, t, L::STATE_CROUCH, L::CROUCH_SHOOT_SCRIPT);
    m.write_u32(t + L::HITBOX_CB, L::HITBOX_CROUCH);
    set_handler(m, t, L::CROUCH_SHOOT_LOOP);
    crouch_shoot_loop(m)
}

/// Bucle del disparo agachado: al acabar, agachado quieto; con el brazo
/// libre gatea hacia atrás, la acción, recarga o arma vacía; se levanta al
/// soltar abajo, salto, golpe sin suelo, estado y caída al vacío.
/// Original: `Player_CrouchShoot_03873c__L0387c2`.
pub fn crouch_shoot_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    set_bits(m, t + L::FLAGS_8D, L::CROUCHED, true);
    player::frame_common(m, t);
    announce_weapon(m, t);
    move_ground(m, t);
    if player::anim_done(m, t) {
        set_handler(m, t, L::CROUCH_IDLE_B);
    }
    if m.read_u8(t + L::STATUS) & L::ARM_BUSY == 0 {
        if m.read_u8(t + L::FACING) & L::FACING_LEFT == 0 {
            if held(m, t, Buttons::LEFT) {
                set_handler(m, t, L::CRAWL_LEFT_BACK);
            }
        } else if held(m, t, Buttons::RIGHT) {
            set_handler(m, t, L::CRAWL_RIGHT_BACK);
        }
        if let Some(a) = player_air::action_select(m, t) {
            set_handler(m, t, crouch_action(a));
        }
        if heavy_needs_reload(m, t) {
            set_handler(m, t, L::CROUCH_RELOAD);
        }
        if weapon_empty(m, t) {
            set_handler(m, t, L::CROUCH_EMPTY);
        }
    }
    if !controls::down_held(m, Entity::from_raw(t)) {
        set_handler(m, t, L::CROUCH_EXIT);
    }
    if controls::jump_held(m, Entity::from_raw(t)).is_some() {
        set_handler(m, t, L::JUMP_START);
    }
    let r = m.regs();
    if collision::probe_floor(m, t, (r.d[3] as u16, r.d[4] as u16)).in_air {
        set_handler(m, t, L::KNOCKBACK_DELAY);
    }
    player_air::publish_state(m, t);
    check_pit(m, t);
    Flow::Return
}

/// Levantarse: limpia el brazo, pose y guion de levantarse, caja de golpe
/// agachado y el bucle. Original: `Player_CrouchExit_037ec2`.
pub fn crouch_exit(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::OFFSET_X, 0);
    m.write_u16(t + L::OFFSET_Y, 0);
    set_bits(m, t + L::STATUS, L::ARM_BUSY | L::ARM_FIRING | L::STATUS_BIT3, false);
    restart(m, t, L::STATE_CROUCH, L::CROUCH_EXIT_SCRIPT);
    m.write_u16(t + L::ACC_X, 0);
    m.write_u32(t + L::HITBOX_CB, L::HITBOX_CROUCH);
    set_handler(m, t, L::CROUCH_EXIT_LOOP);
    crouch_exit_loop(m)
}

/// Bucle de levantarse: se desliza hacia el lado pulsado; al acabar la
/// animación, de pie (o agachado, recargando, la espera o andando); la
/// acción, golpe sin suelo, salto y estado.
/// Original: `Player_CrouchExit_037ec2__L037f16`.
pub fn crouch_exit_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    player::frame_common(m, t);
    announce_weapon(m, t);
    let v = crawl_speed(m, t);
    m.write_u16(t + L::VEL_X, v);
    move_ground(m, t);
    m.write_u16(t + L::VEL_X, 0);
    if player::anim_done(m, t) {
        set_handler(m, t, L::STAND);
        if wants_crouch(m, t) {
            set_handler(m, t, L::CROUCH);
        }
        if heavy_needs_reload(m, t) {
            set_handler(m, t, L::RELOAD);
        }
        // El original pide un número al azar y no lo usa.
        rng::next(m);
        let ammo = m.read_u16(t + L::AMMO);
        if ammo == L::CROUCH_AMMO {
            set_handler(m, t, L::CROUCH_B);
        }
        if ammo == 0 {
            set_handler(m, t, L::IDLE);
        }
        if held(m, t, Buttons::RIGHT) {
            set_handler(m, t, L::WALK_LOOP_RIGHT);
        }
        if held(m, t, Buttons::LEFT) {
            set_handler(m, t, L::WALK_LOOP_LEFT);
        }
    }
    if let Some(a) = player_air::action_select(m, t) {
        set_handler(m, t, action_state(a, false));
    }
    let r = m.regs();
    if collision::probe_floor(m, t, (r.d[3] as u16, r.d[4] as u16)).in_air {
        set_handler(m, t, L::KNOCKBACK);
    }
    if controls::jump_held(m, Entity::from_raw(t)).is_some() {
        set_handler(m, t, L::JUMP_START);
    }
    player_air::publish_state(m, t);
    Flow::Return
}

/// Pose de espera de pie (`b` = la segunda): limpia el brazo, pose y guion,
/// caja de golpe de pie, sin movimiento y el bucle. El original la llama
/// «Crouch», pero el estado (5) y la caja de golpe son de pie.
/// Original: `Player_Crouch_033a5e` / `Player_CrouchB_033afc`.
pub fn idle_pose(m: &mut impl Machine, b: bool) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::OFFSET_X, 0);
    m.write_u16(t + L::OFFSET_Y, 0);
    set_bits(m, t + L::STATUS, L::ARM_BUSY | L::ARM_FIRING | L::STATUS_BIT3, false);
    let script = if b { L::POSE_B_SCRIPT } else { L::POSE_A_SCRIPT };
    restart(m, t, L::STATE_POSE, script);
    m.write_u32(t + L::HITBOX_CB, L::HITBOX_STAND);
    m.write_u32(t + L::HIT_HANDLER, L::HIT_ROUTINE);
    clear_motion(m, t);
    m.write_u16(t + L::VEL_X, 0);
    set_handler(m, t, if b { L::POSE_B_LOOP } else { L::POSE_A_LOOP });
    idle_pose_loop(m)
}

/// Bucle de la pose de espera: al acabar, de pie (o andando si se mueve);
/// y la cola de la espera. Original: `$033AC0` / `$033B5E`.
pub fn idle_pose_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    player::frame_common(m, t);
    announce_weapon(m, t);
    move_ground(m, t);
    if player::anim_done(m, t) {
        set_handler(m, t, L::STAND);
        let vx = m.read_u16(t + L::VEL_X) as i16;
        if vx > 0 {
            set_handler(m, t, L::WALK_RIGHT);
        } else if vx < 0 {
            set_handler(m, t, L::WALK_LEFT);
        }
    }
    idle_tail(m, t);
    Flow::Return
}

/// Cola de los estados de pie: golpe sin suelo, salto, estado y caída al
/// vacío. Original: `Player_Stand_Tail_034ada`.
fn stand_tail(m: &mut impl Machine, t: u32) {
    let r = m.regs();
    if collision::probe_floor(m, t, (r.d[3] as u16, r.d[4] as u16)).in_air {
        set_handler(m, t, L::KNOCKBACK);
    }
    if controls::jump_held(m, Entity::from_raw(t)).is_some() {
        set_handler(m, t, L::JUMP_START);
    }
    player_air::publish_state(m, t);
    check_pit(m, t);
}

/// Entrada común del cuchillo: desplazamiento, brazo libre, la lista de
/// ataques del cuchillo y los bits 1-2 de `+$13` (las dos ramas de
/// `$02ABCC` que siguen usan el mismo guion).
fn melee_setup(m: &mut impl Machine, t: u32, offset_y: u16) {
    m.write_u16(t + L::OFFSET_X, 0);
    m.write_u16(t + L::OFFSET_Y, offset_y);
    set_bits(m, t + L::STATUS, L::ARM_BUSY | L::ARM_FIRING | L::STATUS_BIT3, false);
    m.write_u32(t + L::ATTACKS, L::MELEE_ATTACKS);
    slug::clear_flags13(m, t);
}

/// Cuchillo de pie: sin movimiento, pose de cuchillo y sigue en el frame
/// (el primer frame no mira el mando).
/// Original: `Player_Melee_035d34`.
pub fn melee(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    melee_setup(m, t, 0);
    for f in [L::VEL_X, L::ACC_X, L::VEL_Y, L::ACC_Y] {
        m.write_u16(t + f, 0);
    }
    m.write_u32(t + L::HIT_HANDLER, L::HIT_ROUTINE);
    restart(m, t, L::STATE_MELEE, L::MELEE_SCRIPT);
    m.write_u32(t + L::HITBOX_CB, L::HITBOX_MELEE);
    set_handler(m, t, L::MELEE_RUN);
    melee_frame(m, t)
}

/// Bucle del cuchillo: la velocidad sigue al mando (derecha, izquierda o
/// quieto) y al cambiar arranca el guion de quieto, de avance o de
/// retroceso. Original: `Player_Melee_Run_035de0`.
pub fn melee_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let mut vel = 0u16;
    if held(m, t, Buttons::RIGHT) {
        vel = L::MELEE_SPEED;
    }
    if held(m, t, Buttons::LEFT) {
        vel = L::MELEE_SPEED.wrapping_neg();
    }
    let cur = m.read_u16(t + L::VEL_X);
    if vel != cur {
        let script = if vel == 0 {
            L::MELEE_SCRIPT
        } else if (vel as i16) > (cur as i16) {
            L::MELEE_FWD_SCRIPT
        } else {
            L::MELEE_BACK_SCRIPT
        };
        anim::start(m, t, script);
        set_bits(m, t + L::FLAGS_69, L::WANTS_POSE, false);
    }
    m.write_u16(t + L::VEL_X, vel);
    melee_frame(m, t)
}

/// Frame del cuchillo: al acabar, de pie apuntando adelante (o la pose de
/// espera o la espera sin munición); la acción y la cola de pie.
/// Original: `Player_Melee_Frame_035e54` (con `Player_ActionDispatch_035cd8`).
fn melee_frame(m: &mut impl Machine, t: u32) -> Flow {
    player::frame_common(m, t);
    announce_weapon(m, t);
    move_ground(m, t);
    if player::anim_done(m, t) {
        set_handler(m, t, L::STAND);
        stand_aim(m, t);
        if wants_crouch(m, t) {
            set_handler(m, t, L::CROUCH);
        }
        if m.read_u16(t + L::AMMO) == 0 {
            set_handler(m, t, L::IDLE);
        }
    }
    if let Some(a) = player_air::action_select(m, t) {
        set_handler(m, t, action_state(a, false));
    }
    stand_tail(m, t);
    Flow::Return
}

/// Cuchillo sin parar el movimiento: pose del otro cuchillo y el bucle.
/// Original: `Player_MeleeAlt_035ea8`.
pub fn melee_alt(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    melee_setup(m, t, 0);
    m.write_u32(t + L::HIT_HANDLER, L::HIT_ROUTINE);
    restart(m, t, L::STATE_MELEE, L::MELEE_ALT_SCRIPT);
    m.write_u32(t + L::HITBOX_CB, L::HITBOX_MELEE);
    set_handler(m, t, L::MELEE_ALT_LOOP);
    melee_alt_loop(m)
}

/// Bucle del otro cuchillo: al acabar, de pie apuntando adelante; y la
/// cola de andar. Original: `Player_MeleeAlt_035ea8__L035f40`.
pub fn melee_alt_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    player::frame_common(m, t);
    announce_weapon(m, t);
    move_ground(m, t);
    if player::anim_done(m, t) {
        set_handler(m, t, L::STAND);
        stand_aim(m, t);
    }
    walk_tail(m, t);
    Flow::Return
}

/// Lanzar una granada de pie: limpia el brazo, sin movimiento, pose y guion
/// de lanzar (las dos ramas de `$02ABCC` usan el mismo) y el bucle.
/// Original: `Player_ThrowGrenade_Stand_0360bc`.
pub fn throw_stand(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::OFFSET_X, 0);
    m.write_u16(t + L::OFFSET_Y, 0);
    set_bits(m, t + L::STATUS, L::ARM_BUSY | L::ARM_FIRING | L::STATUS_BIT3, false);
    for f in [L::VEL_X, L::ACC_X, L::VEL_Y, L::ACC_Y] {
        m.write_u16(t + f, 0);
    }
    m.write_u32(t + L::HIT_HANDLER, L::HIT_ROUTINE);
    restart(m, t, L::STATE_THROW, L::THROW_SCRIPT);
    m.write_u8(t + L::BYTE_3B, 0);
    set_handler(m, t, L::THROW_LOOP);
    throw_stand_loop(m)
}

/// Bucle de lanzar de pie: al acabar, de pie apuntando adelante (o la pose
/// de espera o la espera sin munición); con el brazo libre anda, se gira,
/// se agacha o salta; la acción y la cola de pie.
/// Original: `Player_ThrowGrenade_Stand_0360bc__L036152`.
pub fn throw_stand_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    player::frame_common(m, t);
    announce_weapon(m, t);
    move_ground(m, t);
    if player::anim_done(m, t) {
        set_handler(m, t, L::STAND);
        stand_aim(m, t);
        if wants_crouch(m, t) {
            set_handler(m, t, L::CROUCH);
        }
        if m.read_u16(t + L::AMMO) == 0 {
            set_handler(m, t, L::IDLE);
        }
    }
    if m.read_u8(t + L::STATUS) & L::ARM_BUSY == 0 {
        let left = m.read_u8(t + L::FACING) & L::FACING_LEFT != 0;
        let (to_right, to_left) = if left { (L::TURN_TO_RIGHT, L::WALK_LEFT) } else { (L::WALK_RIGHT, L::TURN_TO_LEFT) };
        if held(m, t, Buttons::RIGHT) {
            set_handler(m, t, to_right);
        }
        if held(m, t, Buttons::LEFT) {
            set_handler(m, t, to_left);
        }
        if controls::down_held(m, Entity::from_raw(t)) {
            set_handler(m, t, L::CROUCH_ENTER);
        }
        if controls::jump_held(m, Entity::from_raw(t)).is_some() {
            set_handler(m, t, L::JUMP_START);
        }
    }
    if let Some(a) = player_air::action_select(m, t) {
        set_handler(m, t, action_state(a, false));
    }
    stand_tail(m, t);
    Flow::Return
}

fn walk_speed(left: bool) -> u16 {
    if left { L::WALK_SPEED.wrapping_neg() } else { L::WALK_SPEED }
}

/// Andar hacia la derecha (`left = false`) o la izquierda: si venía de una
/// pose de brazo de las que se conservan sólo rearranca el guion; si no,
/// limpia el brazo y pone la pose de andar. Después la velocidad, la caja
/// de golpe de pie, hacia dónde mira y el bucle.
/// Original: `Player_WalkRight_034b38` / `Player_WalkLeft_034d32` (con sus
/// `_Setup`).
pub fn walk(m: &mut impl Machine, left: bool) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::OFFSET_X, 0);
    m.write_u16(t + L::OFFSET_Y, 0xFFFF);
    if L::KEEP_ANIM.contains(&m.read_u8(t + L::STATE)) {
        anim::start(m, t, L::WALK_SCRIPT);
        set_bits(m, t + L::FLAGS_69, L::WANTS_POSE, false);
    } else {
        set_bits(m, t + L::STATUS, L::ARM_BUSY | L::ARM_FIRING | L::STATUS_BIT3, false);
        restart(m, t, L::STATE_WALK, L::WALK_SCRIPT);
    }
    walk_setup(m, t, left)
}

/// Segunda mitad de la entrada de andar. Original:
/// `Player_WalkRight_Setup_034ba4` / `Player_WalkLeft_Setup_034d9e`.
pub fn walk_setup(m: &mut impl Machine, t: u32, left: bool) -> Flow {
    m.write_u16(t + L::VEL_X, walk_speed(left));
    m.write_u16(t + L::ACC_X, 0);
    m.write_u32(t + L::HIT_HANDLER, L::HIT_ROUTINE);
    m.write_u32(t + L::HITBOX_CB, L::HITBOX_STAND);
    set_bits(m, t + L::FACING, L::FACING_LEFT, left);
    set_handler(m, t, if left { L::WALK_LEFT_LOOP } else { L::WALK_RIGHT_LOOP });
    walk_loop(m, left)
}

/// Bucle de andar: al acabar la animación, el bucle largo de andar; sigue
/// la pose del brazo si cambia el apuntado; con el brazo libre se para al
/// soltar, se gira y la acción; y la cola de andar.
/// Original: `$034BCA` / `$034DC4` (con `Player_WalkRight_Tail_034ccc`,
/// igual que `Player_Walk_Tail_03548c`).
pub fn walk_loop(m: &mut impl Machine, left: bool) -> Flow {
    let t = m.task_register();
    player::frame_common(m, t);
    if announce_weapon(m, t) {
        set_handler(m, t, L::REENTER);
    }
    m.write_u16(t + L::VEL_X, walk_speed(left));
    move_ground(m, t);
    if player::anim_done(m, t) {
        set_handler(m, t, if left { L::WALK_LOOP_LEFT } else { L::WALK_LOOP_RIGHT });
    }
    if m.read_u32(t + L::POSE) == L::WALK_AIM_POSE {
        let up = m.read_u8(t + L::AIM) == L::AIM_UP;
        m.write_u16(t + L::MODE, if up { L::MODE_UP } else { 0 });
        set_pose(m, t, L::STATE_WALK, if up { L::WALK_UP_SCRIPT } else { L::WALK_SCRIPT });
    }
    if m.read_u8(t + L::STATUS) & L::ARM_BUSY == 0 {
        let (fwd, back, turn) = if left {
            (Buttons::LEFT, Buttons::RIGHT, L::TURN_TO_RIGHT)
        } else {
            (Buttons::RIGHT, Buttons::LEFT, L::TURN_TO_LEFT)
        };
        if !held(m, t, fwd) {
            set_handler(m, t, L::STAND_FROM_WALK);
        }
        if held(m, t, back) {
            set_handler(m, t, turn);
        }
        if let Some(a) = player_air::action_select(m, t) {
            let side = usize::from(left);
            let h = match a {
                0xFF => L::MELEE,
                3 => L::THROW,
                4 => L::SHOOT_DOWN,
                1 => L::WALK_SHOOT_UP[side],
                _ => L::WALK_SHOOT[side],
            };
            set_handler(m, t, h);
        }
    }
    walk_tail(m, t);
    Flow::Return
}

/// Andar ya lanzado, hacia la derecha (`left = false`) o la izquierda: si
/// venía de una pose de brazo de las que se conservan sólo rearranca el
/// guion (con el brazo algo más bajo); si no, limpia el brazo y pone la
/// pose. Original: `Player_WalkLoopRight_0351d8` / `Player_WalkLoopLeft_0354f2`.
pub fn walk_long(m: &mut impl Machine, left: bool) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::OFFSET_X, 0);
    if L::KEEP_ANIM.contains(&m.read_u8(t + L::STATE)) {
        anim::start(m, t, L::WALK_LONG_SCRIPT);
        m.write_u16(t + L::OFFSET_Y, 3);
        set_bits(m, t + L::FLAGS_69, L::WANTS_POSE, false);
    } else {
        m.write_u16(t + L::OFFSET_Y, 0xFFFF);
        set_bits(m, t + L::STATUS, L::ARM_BUSY | L::ARM_FIRING | L::STATUS_BIT3, false);
        restart(m, t, L::STATE_WALK_LONG, L::WALK_LONG_SCRIPT);
    }
    walk_long_setup(m, t, left)
}

/// Segunda mitad. Original: `Player_WalkLoopRight_Setup_035250` /
/// `Player_WalkLoopLeft_Setup_03556a`.
pub fn walk_long_setup(m: &mut impl Machine, t: u32, left: bool) -> Flow {
    m.write_u16(t + L::VEL_X, walk_speed(left));
    m.write_u16(t + L::ACC_X, 0);
    m.write_u32(t + L::HIT_HANDLER, L::HIT_ROUTINE);
    m.write_u32(t + L::HITBOX_CB, L::HITBOX_STAND);
    set_bits(m, t + L::FACING, L::FACING_LEFT, left);
    set_handler(m, t, if left { L::WALK_LONG_LEFT_RUN } else { L::WALK_LONG_RIGHT_RUN });
    walk_long_run(m, left)
}

/// Bucle de andar lanzado: al acabar la animación rehace la pose; sigue la
/// pose del brazo si cambia el apuntado; con el brazo ocupado se para al
/// soltar o se gira sin cambiar de estado; libre, pasa a la espera al
/// soltar, se gira y la acción; y la cola de andar.
/// Original: `$035276` / `Player_WalkLoopLeft_Run_035590`.
pub fn walk_long_run(m: &mut impl Machine, left: bool) -> Flow {
    let t = m.task_register();
    player::frame_common(m, t);
    if announce_weapon(m, t) {
        set_handler(m, t, L::REENTER);
    }
    m.write_u16(t + L::VEL_X, walk_speed(left));
    move_ground(m, t);
    if player::anim_done(m, t) {
        m.write_u16(t + L::OFFSET_X, 0);
        m.write_u16(t + L::OFFSET_Y, 0xFFFF);
        set_bits(m, t + L::STATUS, L::ARM_BUSY | L::ARM_FIRING | L::STATUS_BIT3, false);
        if L::KEEP_ANIM.contains(&m.read_u8(t + L::STATE)) {
            set_pose(m, t, L::STATE_WALK_LONG, L::WALK_LONG_SCRIPT);
        } else {
            restart(m, t, L::STATE_WALK_LONG, L::WALK_LONG_SCRIPT);
        }
    }
    if m.read_u8(t + L::STATE) == L::STATE_WALK_LONG {
        let up = m.read_u8(t + L::AIM) == L::AIM_UP;
        let mode = if up { L::MODE_UP } else { 0 };
        if m.read_u16(t + L::MODE) != mode {
            m.write_u16(t + L::MODE, mode);
            let script = if up { L::WALK_LONG_UP_SCRIPT } else { L::WALK_LONG_SCRIPT };
            set_pose(m, t, L::STATE_WALK_LONG, script);
            let want = if up { L::AIM_HISTORY_TO_UP } else { L::AIM_HISTORY_TO_FORWARD };
            if m.read_u8(t + L::AIM_HISTORY) == want {
                set_bits(m, t + L::FLAGS_69, L::WANTS_POSE, true);
            }
        }
    }
    let (fwd, back, turn) = if left {
        (Buttons::LEFT, Buttons::RIGHT, L::TURN_TO_RIGHT)
    } else {
        (Buttons::RIGHT, Buttons::LEFT, L::TURN_TO_LEFT)
    };
    if m.read_u8(t + L::STATUS) & L::ARM_BUSY != 0 {
        if !held(m, t, fwd) {
            set_handler(m, t, L::STAND);
        }
        if held(m, t, back) {
            let f = m.read_u8(t + L::FACING) ^ L::FACING_LEFT;
            m.write_u8(t + L::FACING, f);
        }
    } else {
        if !held(m, t, fwd) {
            set_handler(m, t, L::IDLE_STATE_CHECK);
        }
        if held(m, t, back) {
            set_handler(m, t, turn);
        }
        if let Some(a) = player_air::action_select(m, t) {
            let side = usize::from(left);
            let h = match a {
                0xFF => L::MELEE,
                3 => L::THROW,
                4 => L::SHOOT_DOWN,
                1 => L::WALK_LONG_SHOOT_UP[side],
                _ => L::WALK_LONG_SHOOT[side],
            };
            set_handler(m, t, h);
        }
    }
    walk_tail(m, t);
    Flow::Return
}

/// Parar de andar: con una pose de brazo de las que se conservan o
/// apuntando arriba, de pie; si no, la pose de parar, sin movimiento, en el
/// suelo, y según el arma la espera, recargar, la pose de espera o el bucle.
/// Original: `Player_Idle_StateCheck_034046`.
pub fn stop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if L::KEEP_ANIM.contains(&m.read_u8(t + L::STATE)) || m.read_u16(t + L::MODE) == L::MODE_UP {
        set_handler(m, t, L::STAND);
        return stand(m);
    }
    m.write_u16(t + L::OFFSET_X, 0);
    m.write_u16(t + L::OFFSET_Y, 0);
    restart(m, t, L::STATE_POSE, L::STOP_SCRIPT);
    set_bits(m, t + L::STATUS, L::ARM_BUSY | L::ARM_FIRING | L::STATUS_BIT3, false);
    set_bits(m, t + L::FLAGS_5B, L::GROUNDED, true);
    m.write_u32(t + L::HIT_HANDLER, L::HIT_ROUTINE);
    clear_motion(m, t);
    m.write_u16(t + L::VEL_X, 0);
    m.write_u32(t + L::HITBOX_CB, L::HITBOX_STAND);
    if m.read_u16(t + L::AMMO) == 0 {
        set_handler(m, t, L::IDLE);
        return idle(m);
    }
    if heavy_needs_reload(m, t) {
        set_handler(m, t, L::RELOAD);
        return Flow::Goto(L::RELOAD);
    }
    if wants_crouch(m, t) {
        set_handler(m, t, L::CROUCH);
        return idle_pose(m, false);
    }
    set_handler(m, t, L::STOP_LOOP);
    stop_loop(m)
}

/// Bucle de parar: al acabar la animación, de pie apuntando adelante (o la
/// pose de espera, recargar o la espera); y la cola de la espera.
/// Original: `$03411A`.
pub fn stop_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    player::frame_common(m, t);
    announce_weapon(m, t);
    move_ground(m, t);
    if player::anim_done(m, t) {
        set_handler(m, t, L::STAND);
        stand_aim(m, t);
        if wants_crouch(m, t) {
            set_handler(m, t, L::CROUCH);
        }
        if heavy_needs_reload(m, t) {
            set_handler(m, t, L::RELOAD);
        }
        // El original pide un número al azar y no lo usa.
        rng::next(m);
        if m.read_u8(t + L::STATE) == L::STATE_LANDED && m.read_u16(t + L::AMMO) == L::CROUCH_AMMO {
            set_handler(m, t, L::CROUCH_B);
        }
        if m.read_u16(t + L::AMMO) == 0 {
            set_handler(m, t, L::IDLE);
        }
    }
    idle_tail(m, t);
    Flow::Return
}

// --- Andar disparando ----------------------------------------------------

/// Pose de andar disparando (hacia delante o hacia arriba): `state` es el
/// estado de andar (normal o lanzado) y `same` el guion si el apuntado no
/// cambia; si cambia, el de transición. Si el estado actual es de los
/// «bajos» sólo se cambia la pose y se pide rehacerla; si no, se reinicia
/// la animación. Original: `Player_WalkShootPose_034ede`,
/// `Player_WalkShootUpPose_035052`, `Player_WalkLoopShootPose_0357fa`,
/// `_Alt_0358aa`, `Player_WalkLoopShootUpPose_03595a` y `_Alt_035a0a`.
pub fn walk_shoot_pose(m: &mut impl Memory, t: u32, up: bool, state: u8, same: u32) {
    set_bits(m, t + L::STATUS, L::ARM_FIRING, true);
    let mode = if up { L::MODE_UP } else { 0 };
    // Viene del otro apuntado (el original sólo mira ese valor exacto).
    let other = if up { 0 } else { L::MODE_UP };
    let changed = m.read_u16(t + L::MODE) == other;
    let script = match (changed, up) {
        (true, false) => L::WALK_SHOOT_FROM_UP,
        (true, true) => L::WALK_SHOOT_UP_FROM_FWD,
        (false, _) => same,
    };
    if m.read_u8(t + L::STATE) < L::STATE_SHOOT_KEEP {
        set_bits(m, t + L::FLAGS_69, L::WANTS_POSE, true);
        set_pose(m, t, state, script);
    } else {
        restart(m, t, state, script);
    }
    m.write_u16(t + L::MODE, mode);
}

/// Echa a andar disparando. `long` = andar lanzado.
/// Original: `Player_WalkRight_Shoot_034eca`, `_ShootUp_03503e`,
/// `Player_WalkLeft_Shoot_0351b0`, `_ShootUp_0351c4`,
/// `Player_WalkLoopRight_Shoot_0357aa`, `_ShootUp_0357be`,
/// `Player_WalkLoopLeft_Shoot_0357d2` y `_ShootUp_0357e6`.
pub fn walk_shoot(m: &mut impl Machine, left: bool, up: bool, long: bool) -> Flow {
    let t = m.task_register();
    let (off, state, same) = match (long, up) {
        (false, false) => (L::WALK_SHOOT_OFFSET, L::STATE_WALK_SHOOT, L::WALK_SHOOT_SCRIPT),
        (false, true) => (L::WALK_SHOOT_OFFSET, L::STATE_WALK_SHOOT, L::WALK_SHOOT_UP_SCRIPT),
        (true, false) => (L::WALK_LONG_SHOOT_OFFSET, L::STATE_WALK_LONG_SHOOT, L::WALK_SHOOT_SCRIPT),
        (true, true) => (L::WALK_LONG_SHOOT_UP_OFFSET, L::STATE_WALK_LONG_SHOOT, L::WALK_SHOOT_UP_SCRIPT),
    };
    m.write_u16(t + L::OFFSET_X, off.0);
    m.write_u16(t + L::OFFSET_Y, off.1);
    walk_shoot_pose(m, t, up, state, same);
    if long {
        walk_long_setup(m, t, left)
    } else {
        walk_setup(m, t, left)
    }
}
