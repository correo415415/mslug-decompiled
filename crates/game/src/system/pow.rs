//! Prisionero (POW) suelto por el escenario: sentado o atado hasta que le
//! dan, se suelta y se va andando; si un jugador lo toca, le da las
//! gracias y se va corriendo (el salto y la caída siguen en el original).
//!
//! Original: `Pow_AnimTblB_03f636__L03f9b8` (preparación común), el
//! sentado (`$03FC38`, que el desensamblado dejó como datos dentro de
//! `Pow_AnimTblC_03fa0e`), `Pow_Freed_03fc6a`, `Pow_FreedCheer_03fca4`,
//! `Pow_FreedWait_03fcc0`, `Pow_Physics_03fece` y `Pow_OffworldFree_03efce`
//! (`asm/pow/results_squadleader_03daa8.s`).

use super::{anim, fort, items, pickup, platforms, soldier, sound, tasks, video};
use crate::layout::{pow as L, task as T};
use crate::mem::{Flow, Machine, Memory};

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

/// Preparación común: mira a la izquierda si lo pide `+$9A`, sus gráficos,
/// prioridad, quieto, apoyado y su sombra (tarea hija).
/// Original: `Pow_AnimTblB_03f636__L03f9b8`.
pub fn setup(m: &mut impl Machine, t: u32) {
    if m.read_u8(t + L::SPAWN_LEFT) != 0 {
        let f = m.read_u8(t + L::FACING);
        m.write_u8(t + L::FACING, f | L::FACING_LEFT);
    }
    anim::alloc_entry(m, t, L::GFX);
    anim::set_attr(m, t, L::ATTR_BASE);
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, (a & L::ATTR_KEEP) | L::PRIORITY);
    for f in L::MOTION {
        m.write_u16(t + f, 0);
    }
    // `$027F60` vuelve siempre con C = 0: el `scc` deja $FF.
    soldier::on_ground(m, t);
    m.write_u8(t + L::GROUNDED, 0xFF);
    tasks::spawn(m, L::SHADOW, t);
}

/// Física del prisionero: apoyado anda por el suelo; si no, cae.
/// Original: `Pow_Physics_03fece`.
fn physics(m: &mut impl Machine, t: u32) {
    soldier::physics_flag(m, t, L::GROUNDED);
}

/// Se va si sale de la vista (con sus márgenes).
/// Original: `Pow_OffworldFree_03efce`.
fn offworld(m: &mut impl Machine, t: u32) -> Flow {
    if items::out_of_view_seen(m, t, Some(L::MARGINS)) {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Sentado: preparación, su animación y el bucle.
/// Original: `$03FC38` (`Pow_AnimTblC_03fa0e+$22A`).
pub fn seated(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    setup(m, t);
    anim::start(m, t, L::S_SEATED);
    set_handler(m, t, L::SEATED_LOOP);
    seated_loop(m)
}

/// Bucle sentado: física, animación y, si le dan, liberado.
/// Original: `$03FC4C` (`Pow_AnimTblC_03fa0e+$23E`).
pub fn seated_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    physics(m, t);
    anim::step(m, t);
    if fort::hit_side(m, t).is_some() {
        set_handler(m, t, L::FREED);
    }
    offworld(m, t)
}

/// De pie (suelto desde el principio): preparación; la espera empieza el
/// frame siguiente. Original: `Pow_Entry_03fe5a`.
pub fn entry(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    setup(m, t);
    set_handler(m, t, L::IDLE);
    Flow::Return
}

/// Como [`entry`] pero echa a andar enseguida. Original: `Pow_EntryB_03fec6`.
pub fn entry_walk(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    setup(m, t);
    walk_off(m)
}

/// Espera de pie, quieto. Original: `Pow_Idle_03fe66`.
pub fn idle(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::S_IDLE);
    m.write_u16(t + L::VEL_X, 0);
    m.write_u16(t + L::VEL_Y, 0);
    set_handler(m, t, L::IDLE_LOOP);
    idle_loop(m)
}

/// Física y animación; si le dan, saluda. Original: `Pow_Idle_03fe66+$18`.
pub fn idle_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    physics(m, t);
    anim::step(m, t);
    if fort::hit_side(m, t).is_some() {
        set_handler(m, t, L::IDLE_WAVE);
    }
    offworld(m, t)
}

/// Saluda (cayendo si está en el aire) y al acabar se va andando.
/// Original: `Pow_IdleWave_03fe9c`.
pub fn idle_wave(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::S_IDLE_WAVE);
    set_handler(m, t, L::IDLE_WAVE_LOOP);
    idle_wave_loop(m)
}

/// Original: `Pow_IdleWave_03fe9c+$10`.
pub fn idle_wave_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    soldier::fall(m, t);
    if anim::step(m, t) {
        set_handler(m, t, L::WALK_OFF);
    }
    offworld(m, t)
}

/// Liberado: animación de soltarse y el bucle.
/// Original: `Pow_Freed_03fc6a`.
pub fn freed(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::S_FREED);
    set_handler(m, t, L::FREED_LOOP);
    freed_loop(m)
}

/// Al acabar de soltarse se va andando y deja una tarea que vitorea en su
/// sitio. Original: `Pow_Freed_03fc6a+$10`.
pub fn freed_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if anim::step(m, t) {
        set_handler(m, t, L::WALK_OFF);
        let (c, _) = tasks::spawn_last(m, L::CHEER, t);
        anim::copy_sprite_pos(m, t, c);
    }
    offworld(m, t)
}

/// Vitoreo: sus gráficos y la animación; empieza a moverse el frame
/// siguiente. Original: `Pow_FreedCheer_03fca4`.
pub fn cheer(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::GFX);
    anim::start(m, t, L::S_CHEER);
    set_handler(m, t, L::CHEER_LOOP);
    Flow::Return
}

/// Se mueve con la escena y termina al acabar la animación.
/// Original: `Pow_FreedWait_03fcc0`.
pub fn cheer_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if anim::step(m, t) {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Bit 0 de `+$3A`: a 1 anda con velocidad positiva (hacia la derecha).
fn set_facing_bit(m: &mut impl Memory, t: u32, on: bool) {
    let f = m.read_u8(t + L::FACING);
    m.write_u8(t + L::FACING, if on { f | L::FACING_LEFT } else { f & !L::FACING_LEFT });
}

fn flip(m: &mut impl Memory, t: u32) {
    let f = m.read_u8(t + L::FACING);
    m.write_u8(t + L::FACING, f ^ L::FACING_LEFT);
}

fn facing_bit(m: &mut impl Memory, t: u32) -> bool {
    m.read_u8(t + L::FACING) & L::FACING_LEFT != 0
}

// --- Andar (`Pow_Despawn_03ff14` … `Pow_WalkStop_040054`) -------------

/// Final común de los estados de andar: si un jugador lo toca, rescatado
/// (apunta qué jugador); si le dan, desaparece; si sale de la vista, se va.
/// Original: `Pow_Walk_03ff62__L03ffea`.
fn walk_tail(m: &mut impl Machine, t: u32) -> Flow {
    if let Some((p, _)) = pickup::toucher(m, t, L::RESCUE_BOX) {
        m.write_u8(t + L::RESCUER, p);
        set_handler(m, t, L::RESCUED);
    }
    if fort::hit_side(m, t).is_some() {
        return tasks::finish(m);
    }
    offworld(m, t)
}

/// Suelto: cambia a los gráficos de andar y elige hacia dónde.
/// Original: `Pow_Despawn_03ff14`.
pub fn walk_off(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    video::release_task_entries(m, t);
    anim::alloc_entry(m, t, L::GFX_WALK);
    pick_walk(m)
}

/// Hacia dónde anda (`+$98`: 0 = como mira, 1 = bit de dirección a 0,
/// otro = a 1) y cuánto antes de pararse. Original: `Pow_PickWalk_03ff24`.
pub fn pick_walk(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u8(t + L::GROUNDED, 0);
    m.write_u32(t + L::ANIM_BASE, L::ANIM_BASE_WALK);
    let f = m.read_u8(t + L::FLAGS_13);
    m.write_u8(t + L::FLAGS_13, f & !L::FLAGS_13_CLEAR);
    match m.read_u8(t + L::KIND) {
        0 => {}
        1 => set_facing_bit(m, t, false),
        _ => set_facing_bit(m, t, true),
    }
    m.write_u16(t + L::WALK_TIMER, L::WALK_FRAMES);
    walk(m)
}

/// Andar: animación y velocidad hacia donde mira.
/// Original: `Pow_Walk_03ff62`.
pub fn walk(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::S_WALK);
    let v = if facing_bit(m, t) { L::WALK_SPEED } else { L::WALK_SPEED.wrapping_neg() };
    m.write_u16(t + L::VEL_X, v);
    set_handler(m, t, L::WALK_LOOP);
    walk_loop(m)
}

/// Si pierde el suelo, se gira en el aire; contra una pared o al acabarse
/// el tiempo (sólo los que andan como miran), se para; al cruzar un borde,
/// salta o se deja caer. Original: `Pow_Walk_03ff62+$24`.
pub fn walk_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    physics(m, t);
    if m.read_u8(t + L::GROUNDED) == 0 {
        set_handler(m, t, L::WALK_TURN);
    } else if m.read_u8(t + L::STATE) & L::HIT_WALL != 0 {
        set_handler(m, t, L::WALK_STOP);
    } else if m.read_u8(t + L::KIND) == 0 {
        let n = m.read_u16(t + L::WALK_TIMER);
        m.write_u16(t + L::WALK_TIMER, n.wrapping_sub(1));
        if n == 0 {
            set_handler(m, t, L::WALK_STOP);
        }
    }
    if let Some((col, side)) = soldier::edge_ahead(m, t) {
        m.write_u8(t + L::EDGE, col);
        set_handler(m, t, if side & 1 != 0 { L::JUMP } else { L::FALL });
    }
    anim::step(m, t);
    walk_tail(m, t)
}

/// En el aire tras perder el suelo: quieto y cae hasta apoyarse; entonces
/// vuelve a andar. Original: `Pow_WalkTurn_040014`.
pub fn walk_turn(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u32(t + L::ANIM_BASE, L::ANIM_BASE_WALK);
    anim::start(m, t, L::S_WALK_TURN);
    m.write_u16(t + L::VEL_X, 0);
    m.write_u16(t + L::VEL_Y, 0);
    set_handler(m, t, L::WALK_TURN_LOOP);
    walk_turn_loop(m)
}

/// Original: `Pow_WalkTurn_040014+$20`.
pub fn walk_turn_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let landed = soldier::fall(m, t);
    m.write_u8(t + L::GROUNDED, if landed { 0xFF } else { 0 });
    if landed {
        set_handler(m, t, L::WALK);
    }
    anim::step(m, t);
    walk_tail(m, t)
}

/// Se para; al acabar la animación, se da la vuelta y sigue andando.
/// Original: `Pow_WalkStop_040054`.
pub fn walk_stop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::VEL_X, 0);
    anim::start(m, t, L::S_WALK_STOP);
    set_handler(m, t, L::WALK_STOP_LOOP);
    walk_stop_loop(m)
}

/// Original: `Pow_WalkStop_040054+$14`.
pub fn walk_stop_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    physics(m, t);
    if anim::step(m, t) {
        flip(m, t);
        m.write_u16(t + L::WALK_TIMER, L::WALK_AGAIN_FRAMES);
        set_handler(m, t, L::WALK);
    }
    walk_tail(m, t)
}

// --- Rescatado (`Pow_Rescued_040268` … `Pow_RescuedExit_040392`) --------

/// Suma un prisionero rescatado al jugador `p` si sigue en juego.
/// Original: `Pow_AddRescued_03ef9a__L03efa2`.
pub fn add_rescued(m: &mut impl Memory, p: u8) {
    if tasks::player_task_if_alive(m, u32::from(p & 1)).is_some() {
        let a = L::RESCUED_COUNT[usize::from(p != 0)];
        let n = m.read_u16(a);
        m.write_u16(a, n.wrapping_add(1));
    }
}

/// Final común del rescate: si le dan, desaparece; si sale de la vista,
/// se va. Original: `Pow_Rescued_040268__L0402f0`.
fn rescued_tail(m: &mut impl Machine, t: u32) -> Flow {
    if fort::hit_side(m, t).is_some() {
        return tasks::finish(m);
    }
    offworld(m, t)
}

/// Rescatado: «¡gracias!», se apunta al jugador y da unos pasos rápidos
/// hacia el centro de la pantalla. Original: `Pow_Rescued_040268`.
pub fn rescued(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    sound::play(m, L::THANKS_SOUND);
    let p = m.read_u8(t + L::RESCUER);
    add_rescued(m, p);
    m.write_u32(t + L::ANIM_BASE, 0xFFFF_FFFF);
    anim::start(m, t, L::S_WALK);
    let right = (L::SCREEN_MID.wrapping_sub(m.read_u16(t + L::X)) as i16) >= 0;
    set_facing_bit(m, t, right);
    m.write_u16(t + L::VEL_X, if right { L::RESCUED_SPEED } else { L::RESCUED_SPEED.wrapping_neg() });
    m.write_u16(t + L::STEPS, L::RESCUED_STEPS);
    set_handler(m, t, L::RESCUED_LOOP);
    rescued_loop(m)
}

/// Original: `Pow_Rescued_040268+$58`.
pub fn rescued_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    physics(m, t);
    anim::step(m, t);
    let n = m.read_u16(t + L::STEPS).wrapping_sub(1);
    m.write_u16(t + L::STEPS, n);
    if n == 0 {
        flip(m, t);
        let h = if m.read_u8(t + L::BOWS) != 0 { L::RESCUED_BOW } else { L::RESCUED_RUN };
        set_handler(m, t, h);
    }
    rescued_tail(m, t)
}

/// Reverencia (los que la hacen) y después a correr.
/// Original: `Pow_RescuedBow_040304`.
pub fn rescued_bow(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::S_BOW);
    m.write_u16(t + L::VEL_X, 0);
    set_handler(m, t, L::RESCUED_BOW_LOOP);
    rescued_bow_loop(m)
}

/// Original: `Pow_RescuedBow_040304+$14`.
pub fn rescued_bow_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    physics(m, t);
    if anim::step(m, t) {
        set_handler(m, t, L::RESCUED_RUN);
    }
    rescued_tail(m, t)
}

/// Se prepara para correr. Original: `Pow_RescuedRun_04032e`.
pub fn rescued_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::S_RUN_START);
    m.write_u16(t + L::VEL_X, 0);
    set_handler(m, t, L::RESCUED_RUN_LOOP);
    rescued_run_loop(m)
}

/// Original: `Pow_RescuedRun_04032e+$14`.
pub fn rescued_run_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    physics(m, t);
    if anim::step(m, t) {
        set_handler(m, t, L::RESCUED_DASH);
    }
    rescued_tail(m, t)
}

/// Sale corriendo (velocidad negativa) hasta chocar con algo.
/// Original: `Pow_RescuedRun_04032e+$2A`.
pub fn rescued_dash(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::S_DASH);
    set_facing_bit(m, t, false);
    m.write_u16(t + L::VEL_X, L::DASH_SPEED);
    set_handler(m, t, L::RESCUED_DASH_LOOP);
    rescued_dash_loop(m)
}

/// Original: `Pow_RescuedRun_04032e+$46`.
pub fn rescued_dash_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    physics(m, t);
    anim::step(m, t);
    if m.read_u8(t + L::STATE) & L::HIT_WALL != 0 {
        set_handler(m, t, L::RESCUED_EXIT);
    }
    rescued_tail(m, t)
}

/// Contra la pared: parpadea (anima sólo los frames impares) y desaparece.
/// Original: `Pow_RescuedExit_040392`.
pub fn rescued_exit(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u8(t + L::EXIT_TIMER, L::EXIT_FRAMES);
    set_handler(m, t, L::RESCUED_EXIT_LOOP);
    rescued_exit_loop(m)
}

/// Original: `Pow_RescuedExit_040392+$C`.
pub fn rescued_exit_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    physics(m, t);
    let n = m.read_u8(t + L::EXIT_TIMER).wrapping_sub(1);
    m.write_u8(t + L::EXIT_TIMER, n);
    if n & 1 != 0 {
        anim::step(m, t);
    }
    if n == 0 {
        return tasks::finish(m);
    }
    rescued_tail(m, t)
}
