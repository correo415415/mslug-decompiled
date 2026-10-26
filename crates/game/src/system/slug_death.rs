//! Destrucción del Slug: explota (con trozos, esquirlas y la caja de la
//! explosión que golpea alrededor), si estaba en el aire sale despedido y
//! al caer explota, se apaga durante unos frames y la tarea queda vacía.
//!
//! Original: `Slug_DeathExplode_02da38` (bucle `$02DAE4`),
//! `Slug_DeathLaunch_02db52` (bucle `$02DC0C`), `Slug_DeathFade_02dc5c`
//! (bucle `$02DC7E`, final `$02DC8C`), `Slug_DeathSetHandler400_02dcaa`,
//! `Slug_DeathFinishJmp_02dcbc`, `Slug_DeathStart_02dcc0`
//! (`asm/slug/states_02ae3e.s`), `Slug_KillInit_02feda`,
//! `Slug_ExplodeFx_02ff22` (`asm/slug/vehicle_02dd20.s`),
//! `Slug_PhysicsA_02a752` y `$02785C`.

use super::{anim, hits, items, manual_icons, platforms, player_air, slug, slug_drive, slug_idle, sound, spawner, tasks, video};
use crate::layout::{slug_death as L, task as T};
use crate::mem::{Machine, Memory};
use crate::Flow;

fn set_bits(m: &mut impl Memory, a: u32, bits: u8, on: bool) {
    let v = m.read_u8(a);
    m.write_u8(a, if on { v | bits } else { v & !bits });
}

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

/// Queda sin vida, destruido y con los sprites de la explosión.
/// Original: `Slug_KillInit_02feda`.
pub fn kill_init(m: &mut impl Memory, t: u32) {
    m.write_u8(t + L::TIMER_59, 0);
    m.write_u8(t + L::TIMER_45, L::KILL_TIMER_45);
    m.write_u16(t + L::HP, 0);
    set_bits(m, t + L::FLAGS_13, L::DESTROYED, true);
    video::release_task_entries(m, t);
    for gfx in L::KILL_GFX {
        anim::alloc_entry(m, t, gfx);
    }
    slug::reset_damage(m, t);
}

/// Sonido, humo, trozos y esquirlas. Original: `Slug_ExplodeFx_02ff22`.
pub fn explode_fx(m: &mut impl Memory, t: u32) {
    sound::play(m, L::EXPLODE_SOUND);
    let c = tasks::spawn(m, L::SMOKE, t);
    anim::copy_sprite_pos(m, t, c);
    spawner::slug_debris(m, t);
    spawner::shards(m, t);
}

/// La caja de la explosión golpea una vez alrededor y salen los efectos.
fn blast(m: &mut impl Memory, t: u32) {
    m.write_u32(t + L::ATTACKS, L::BLAST_ATTACKS);
    slug::clear_flags13(m, t);
    slug::clear_flags13(m, t);
    hits::run_attack_list(m, t);
    m.write_u32(t + L::ATTACKS, T::NIL);
    slug::clear_flags13(m, t);
    explode_fx(m, t);
}

/// Mueve con la cámara usando la Y de apoyo. Original: `Slug_PhysicsA_02a752`
/// (→ `$02785C`).
pub fn physics_a(m: &mut impl Memory, t: u32) {
    let (x, y) = (m.read_u16(t + L::X), m.read_u16(t + L::BASE_Y));
    m.write_u16(L::WORK_POS[0], x);
    m.write_u16(L::WORK_POS[1], y);
    platforms::apply_camera_work(m, t);
    let (x, y) = (m.read_u16(L::WORK_POS[0]), m.read_u16(L::WORK_POS[1]));
    m.write_u16(t + L::X, x);
    m.write_u16(t + L::BASE_Y, y & L::BASE_Y_MASK);
    anim::apply_depth(m, t);
}

/// Guion de la sacudida según de qué lado le dieron (el punto de impacto
/// `+$54` respecto al centro de su caja), `+$20` y sin movimiento.
fn hit_pose(m: &mut impl Memory, t: u32) {
    set_bits(m, t + L::FLAGS_8C, L::BIT_8C_5, false);
    set_bits(m, t + L::FLAGS_13, L::DESTROYED, true);
    let hb = m.read_u32(t + L::HITBOX_CB);
    let mut behind = false;
    if hb != T::NIL {
        let x = m.read_u16(t + L::X);
        let l = m.read_u16(hb + L::BOX_LEFT).wrapping_add(x);
        let r = m.read_u16(hb + L::BOX_LEFT + 2).wrapping_add(x);
        let center = (l.wrapping_add(r) as i16 >> 1) as u16;
        behind = m.read_u16(t + L::HIT_X) < center;
    }
    let scripts = L::POSE_SCRIPTS[usize::from(behind)];
    let idx = slug::slope_anim_idx(m, t);
    let s = m.read_u32(scripts + u32::from(idx) * 4);
    if s != L::NO_SCRIPT {
        anim::start(m, t, s);
    }
    m.write_u8(t + L::BYTE_20, L::BYTE_20_HIT);
    manual_icons::stop_motion(m, t);
}

fn tail(m: &mut impl Machine, t: u32) -> Flow {
    anim::step(m, t);
    if items::out_of_view_seen(m, t, Some(L::MARGINS)) {
        return finish(m);
    }
    slug_drive::left_alone(m, t);
    Flow::Return
}

/// Explota en tierra. Original: `Slug_DeathExplode_02da38`.
pub fn explode(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    kill_init(m, t);
    hit_pose(m, t);
    set_handler(m, t, L::EXPLODE_LOOP);
    explode_loop(m)
}

/// Original: `$02DAE4`.
pub fn explode_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    slug::update_anim(m, t);
    set_bits(m, t + L::FLAGS_8D, L::BIT_8D_4, true);
    if slug::damage_script(m, t, L::DAMAGE_SCRIPT_B) {
        set_handler(m, t, L::FADE);
        blast(m, t);
    }
    physics_a(m, t);
    tail(m, t)
}

/// Despedido por el aire; al caer explota. Original: `Slug_DeathLaunch_02db52`.
pub fn launch(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    hit_pose(m, t);
    m.write_u16(t + L::VEL_Y, L::LAUNCH_VEL_Y);
    m.write_u16(t + L::ACC_Y, L::LAUNCH_GRAVITY);
    m.write_u8(t + L::FADE_TIMER, L::LAUNCH_FRAMES);
    set_handler(m, t, L::LAUNCH_LOOP);
    launch_loop(m)
}

/// Original: `$02DC0C`.
pub fn launch_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    slug::update_anim(m, t);
    slug_idle::leave_chassis_regs(m, t);
    set_bits(m, t + L::FLAGS_8D, L::BIT_8D_4, true);
    let v = player_air::clamp(m.read_u16(t + L::VEL_Y), L::MAX_VEL_Y);
    m.write_u16(t + L::VEL_Y, v);
    let row = m.regs().d[4] as u16;
    if slug::physics_air(m, t, row) {
        set_handler(m, t, L::EXPLODE);
    }
    tail(m, t)
}

/// Sin caja ni rutina extra, se apaga unos frames. Original: `Slug_DeathFade_02dc5c`.
pub fn fade(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    set_bits(m, t + L::FLAGS_8C, L::BIT_8C_5, false);
    m.write_u8(t + L::FADE_TIMER, L::FADE_FRAMES);
    m.write_u32(t + L::HITBOX_CB, T::NIL);
    m.write_u32(t + L::HANDLER_60, T::NIL);
    set_handler(m, t, L::FADE_LOOP);
    fade_loop(m)
}

/// Original: `$02DC7E`.
pub fn fade_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    physics_a(m, t);
    let n = m.read_u8(t + L::FADE_TIMER).wrapping_sub(1);
    m.write_u8(t + L::FADE_TIMER, n);
    if n == 0 {
        return finish(m);
    }
    slug_drive::left_alone(m, t);
    Flow::Return
}

/// Fin: sin hijas ni sprites; el frame siguiente queda vacía.
/// Original: `Slug_DeathFade_02dc5c__L02dc8c` (y `$02DCBC`).
pub fn finish(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    tasks::kill_children(m, t);
    set_bits(m, t + L::FLAGS_5B, L::BIT_5B_7, false);
    video::release_task_entries(m, t);
    set_handler(m, t, L::SET_IDLE);
    slug_drive::left_alone(m, t);
    Flow::Return
}

/// La tarea queda vacía y como nueva. Original: `Slug_DeathSetHandler400_02dcaa`.
pub fn set_idle(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    set_handler(m, t, L::IDLE);
    tasks::reset_in_place(m, t);
    Flow::Return
}

/// Destruido sin piloto: explosión inmediata. Original: `Slug_DeathStart_02dcc0`.
pub fn start(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    blast(m, t);
    set_handler(m, t, L::FADE);
    slug::update_anim(m, t);
    physics_a(m, t);
    tail(m, t)
}

/// Desde los guiones de la explosión: si está destruido, música de la
/// pérdida y la burbuja del piloto que sale. Original:
/// `Slug_DestroyedMusicAndBubble_02a638`.
pub fn lost_music_and_bubble(m: &mut impl Memory, t: u32) {
    if m.read_u8(t + L::FLAGS_13) & L::DESTROYED == 0 {
        return;
    }
    sound::play(m, L::LOST_MUSIC);
    tasks::spawn(m, L::BUBBLE, t);
}

/// Humo del Slug (`k` = 0 el pequeño, 1 el de la explosión): sprite, guion
/// y atributos; se mueve con la escena y se va al acabar o al salir.
/// Original: `SlugFx_Smoke_02f64c` y `$02F672`.
pub fn smoke(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    let (gfx, script, attr) = L::SMOKES[k];
    anim::alloc_entry(m, t, gfx);
    anim::start(m, t, script);
    m.write_u16(t + L::ATTR, attr);
    set_handler(m, t, L::SMOKE_RUN);
    smoke_run(m)
}

/// Original: `$02F692`.
pub fn smoke_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if anim::step(m, t) || items::out_of_view_seen(m, t, None) {
        return tasks::finish(m);
    }
    Flow::Return
}
