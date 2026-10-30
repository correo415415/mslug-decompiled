//! Muerte del jugador. Según cómo ha muerto (`+$58`) y si estaba en el
//! suelo o en el aire, una tabla elige el estado: salta hacia atrás y cae,
//! se desploma entre humo, cae al agua con salpicadura, sangra… Al acabar
//! la animación (o el tiempo) desaparece y su tarea se vacía; si cae fuera
//! de la pantalla, la caída al vacío hace lo mismo enseguida.
//!
//! Original: `PlayerHandlerA/B` (`$033572`/`$033578`,
//! `asm/player/dispatch_033572.s`), `Player_Death_*`/`Player_DeathPit`
//! (`$0375D2`–`$037C72`, `asm/player/air_death_crouch_036632.s`),
//! `PlayerDeathFx_*` (`$039148`–`$039232`), `Player_SpawnBloodFx_033358`,
//! la sangre `$03D842` y `Entity_NegIfFacing_05dca4`.

use super::{anim, collision, items, manual_icons, motion, platforms, player, player_air, player_ground, rng, slug, sound, tasks, video};
use crate::layout::{player_death as L, task as T};
use crate::mem::{Flow, Machine, Memory};

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

fn set_bits(m: &mut impl Memory, a: u32, bits: u8, on: bool) {
    let v = m.read_u8(a);
    m.write_u8(a, if on { v | bits } else { v & !bits });
}

fn clear_vel(m: &mut impl Memory, t: u32) {
    m.write_u16(t + L::VEL_X, 0);
    m.write_u16(t + L::VEL_Y, 0);
}

/// Resta uno al contador `+$90`: ¿ha llegado a 0?
fn tick(m: &mut impl Memory, t: u32) -> bool {
    let n = m.read_u8(t + L::TIMER).wrapping_sub(1);
    m.write_u8(t + L::TIMER, n);
    n == 0
}

fn child(m: &mut impl Memory, t: u32, handler: u32) -> u32 {
    let c = tasks::spawn(m, handler, t);
    anim::copy_sprite_pos(m, t, c);
    c
}

/// Estado de muerte según `+$58` en la tabla `table` (0 = en el suelo,
/// 1 = en el aire). Original: `PlayerStateDispatch_033572`.
pub fn dispatch(m: &mut impl Machine, table: usize) -> Flow {
    let t = m.task_register();
    let mut kind = m.read_u8(t + L::KIND);
    if kind >= L::KIND_LIMIT {
        kind = L::KIND_CLAMP;
    }
    let h = m.read_u32(L::TABLES[table] + u32::from(kind) * 4);
    Flow::Goto(h)
}

/// Velocidad horizontal `v` (al revés si mira a la izquierda) en `+$28`.
/// Original: `Entity_NegIfFacing_05dca4`.
pub fn set_vx_facing(m: &mut impl Memory, t: u32, v: u16) -> u16 {
    let v = if m.read_u8(t + L::FACING) & 1 != 0 { v.wrapping_neg() } else { v };
    m.write_u16(t + L::VEL_X, v);
    v
}

/// Arranca el guion `script` (con su cabecera) y para el movimiento.
fn start_script(m: &mut impl Memory, t: u32, script: u32) {
    m.write_u8(t + L::BYTE_70, 0);
    let head = m.read_u32(script.wrapping_sub(4));
    m.write_u32(t + L::SCRIPT_HEAD, head);
    m.write_u8(t + L::BYTE_21, 0xFF);
    anim::start(m, t, script);
}

/// Lo común a casi todas las muertes: borra el estado y arranca `script`.
fn die(m: &mut impl Memory, t: u32, script: u32) {
    m.write_u16(t + L::FIELD_7C, 0);
    m.write_u16(t + L::FIELD_7E, 0);
    set_bits(m, t + L::STATUS, L::STATUS_CLEAR, false);
    set_bits(m, t + L::FLAGS_13, L::DYING, true);
    start_script(m, t, script);
    manual_icons::stop_motion(m, t);
}

/// Salto hacia atrás.
fn jump_back(m: &mut impl Memory, t: u32) {
    set_vx_facing(m, t, L::FALL_VEL_X);
    m.write_u16(t + L::VEL_Y, L::FALL_VEL_Y);
    m.write_u16(t + L::ACC_Y, L::FALL_ACC_Y);
    m.write_u16(t + L::ACC_X, 0);
}

/// Si se ha salido de la pantalla, a la caída al vacío.
/// Original: `Player_Death_GroundTest_03761e`.
pub fn pit_check(m: &mut impl Memory, t: u32) {
    let pit = m.read_u8(L::SCENE) == L::PIT_SCENE;
    if items::out_of_view_seen(m, t, Some(L::PIT_MARGINS[usize::from(pit)])) {
        set_handler(m, t, L::PIT);
    }
}

fn scream(m: &mut impl Memory, t: u32) {
    sound::play(m, L::SCREAM[usize::from(t != L::P1)]);
}

/// ¿Ha aterrizado? (`$027BC8`).
fn land(m: &mut impl Machine, t: u32) -> bool {
    let row = m.regs().d[4] as u16;
    motion::on_entity_pos(m, t, |m| slug::move_and_stop(m, t, row))
}

/// En la escena del agua, si cae en el agua, se ahoga.
fn water_check(m: &mut impl Machine, t: u32) {
    if m.read_u8(L::SCENE) != L::WATER_SCENE {
        return;
    }
    let (x, y) = (m.read_u16(t + L::X), m.read_u16(t + L::Y));
    if (y as i16) >= L::WATER_Y {
        return;
    }
    let tile = collision::lookup_tile(m, t, x, y);
    if let Some((c, r)) = tile.frac {
        let mut regs = m.regs();
        regs.d[3] = (regs.d[3] & 0xFFFF_0000) | u32::from(c);
        regs.d[4] = (regs.d[4] & 0xFFFF_0000) | u32::from(r);
        m.set_regs(regs);
    }
    if tile.raw == L::WATER_TILE {
        player_air::publish_state(m, t);
    }
}

/// Muerte entre humo: grito y desaparición. Original: `Player_Death_0375d2`.
pub fn smoke(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u8(t + L::BYTE_6C, 0xFF);
    child(m, t, L::SMOKE_TASK);
    sound::play(m, L::SMOKE_SOUND);
    start_script(m, t, L::SCRIPT_SMOKE);
    set_handler(m, t, L::SCREAM_NEXT);
    pit_check(m, t);
    Flow::Return
}

/// Grita y pasa a desaparecer. Original: `Player_Death_PlayMusic_03765a`.
pub fn scream_next(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    set_handler(m, t, L::DESPAWN);
    scream(m, t);
    pit_check(m, t);
    Flow::Return
}

/// Casi siempre, entre humo; si no, se desploma con otro efecto.
/// Original: `Player_Death_Alt_037684`.
pub fn alt(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if rng::next(m).0 as u8 & L::ALT_ODDS != 0 {
        return smoke(m);
    }
    m.write_u8(t + L::BYTE_6C, 0xFF);
    child(m, t, L::FX_ALT);
    let s = child(m, t, L::SMOKE_TASK);
    m.write_u16(s + L::ATTR, L::SMOKE_ATTR);
    let y = m.read_u16(s + L::Y).wrapping_add(L::SMOKE_DY);
    m.write_u16(s + L::Y, y);
    sound::play(m, L::SMOKE_SOUND);
    die(m, t, L::SCRIPT_ALT);
    set_handler(m, t, L::ALT_LOOP);
    alt_loop(m)
}

/// Original: `Player_Death_Alt_037684+$A0`.
pub fn alt_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    player::frame_common(m, t);
    platforms::move_with_scene(m, t);
    if player::anim_done(m, t) {
        set_handler(m, t, L::DESPAWN);
    }
    pit_check(m, t);
    Flow::Return
}

/// Salta hacia atrás y cae. Original: `Player_Death_Fall_037778`.
pub fn fall(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    die(m, t, L::SCRIPT_FALL);
    jump_back(m, t);
    set_handler(m, t, L::FALL_LOOP);
    fall_loop(m)
}

fn falling(m: &mut impl Machine, landed: u32) -> Flow {
    let t = m.task_register();
    set_bits(m, t + L::AIR, L::FALLING, true);
    player::frame_common(m, t);
    if land(m, t) {
        set_handler(m, t, landed);
    }
    water_check(m, t);
    player::anim_done(m, t);
    pit_check(m, t);
    Flow::Return
}

/// Original: `Player_Death_Fall_037778+$74`.
pub fn fall_loop(m: &mut impl Machine) -> Flow {
    falling(m, L::FALL_LANDED)
}

fn lying(m: &mut impl Machine, script: u32, next: u32) -> Flow {
    let t = m.task_register();
    clear_vel(m, t);
    start_script(m, t, script);
    manual_icons::stop_motion(m, t);
    set_handler(m, t, next);
    lying_loop(m)
}

fn lying_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    player::frame_common(m, t);
    player_ground::move_ground(m, t);
    if player::anim_done(m, t) {
        set_handler(m, t, L::DESPAWN);
    }
    Flow::Return
}

/// En el suelo, tumbado. Original: `Player_Death_FallLanded_037874`.
pub fn fall_landed(m: &mut impl Machine) -> Flow {
    lying(m, L::SCRIPT_FALL_LANDED, L::FALL_LANDED_LOOP)
}

/// Original: `Player_Death_FallLanded_037874+$38`.
pub fn fall_landed_loop(m: &mut impl Machine) -> Flow {
    lying_loop(m)
}

/// Como [`fall`] con salpicadura y onda.
/// Original: `Player_Death_FallSpawnFx_0378c6`.
pub fn splash(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    child(m, t, L::FX_RIPPLE);
    child(m, t, L::FX_SPLASH);
    die(m, t, L::SCRIPT_SPLASH);
    jump_back(m, t);
    set_handler(m, t, L::SPLASH_LOOP);
    splash_loop(m)
}

/// Original: `Player_Death_FallSpawnFx_0378c6+$94`.
pub fn splash_loop(m: &mut impl Machine) -> Flow {
    falling(m, L::SPLASH_LANDED)
}

/// Original: `Player_Death_FallLandedB_0379e2`.
pub fn splash_landed(m: &mut impl Machine) -> Flow {
    lying(m, L::SCRIPT_SPLASH_LANDED, L::SPLASH_LANDED_LOOP)
}

/// Original: `Player_Death_FallLandedB_0379e2+$38`.
pub fn splash_landed_loop(m: &mut impl Machine) -> Flow {
    lying_loop(m)
}

/// [`generic`] delante de todo. Original: `Player_Death_PrioE000_037a34`.
pub fn front(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::ATTR, L::ATTR_FRONT);
    generic(m)
}

/// Salta hacia atrás y desaparece al acabar la animación o el tiempo.
/// Original: `Player_Death_Generic_037a3e`.
pub fn generic(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    die(m, t, L::SCRIPT_GENERIC);
    m.write_u8(t + L::TIMER, L::TIMER_INIT);
    jump_back(m, t);
    set_handler(m, t, L::GENERIC_LOOP);
    generic_loop(m)
}

/// Original: `Player_Death_Generic_037a3e+$7A`.
pub fn generic_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    set_bits(m, t + L::AIR, L::FALLING, true);
    player::frame_common(m, t);
    if land(m, t) {
        clear_vel(m, t);
    }
    if player::anim_done(m, t) || tick(m, t) {
        set_handler(m, t, L::DESPAWN);
    }
    Flow::Return
}

/// Sangre y [`timed`]. Original: `Player_Death_Blood_037af2`.
pub fn blood(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    spawn_blood(m, t);
    sound::play(m, L::BLOOD_SOUND);
    timed(m)
}

/// Se desploma donde está. Original: `Player_Death_Timed_037b00`.
pub fn timed(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    die(m, t, L::SCRIPT_TIMED);
    m.write_u8(t + L::TIMER, L::TIMER_INIT);
    set_handler(m, t, L::TIMED_LOOP);
    timed_loop(m)
}

/// Original: `Player_Death_Timed_037b00+$5A`.
pub fn timed_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    player::frame_common(m, t);
    if player_ground::move_ground(m, t) {
        clear_vel(m, t);
    }
    if player::anim_done(m, t) || tick(m, t) {
        set_handler(m, t, L::DESPAWN);
    }
    Flow::Return
}

/// La tarea del jugador se vacía (y sus hijas se van).
fn release(m: &mut impl Memory, t: u32) {
    tasks::kill_children(m, t);
    set_bits(m, t + L::FLAGS_5B, L::ACTIVE_5B, false);
    video::release_task_entries(m, t);
    set_handler(m, t, L::FREE);
    tasks::reset_in_place(m, t);
}

/// Caída al vacío. Original: `Player_DeathPit_037b8e`.
pub fn pit(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if m.read_u8(t + L::FLAGS_13) & L::DYING == 0 {
        scream(m, t);
    }
    set_bits(m, t + L::FLAGS_13, L::DYING, true);
    m.write_u16(t + L::FIELD_7C, 0);
    m.write_u16(t + L::FIELD_7E, 0);
    m.write_u8(t + L::TIMER, L::PIT_TIMER);
    m.write_u32(t + L::HITBOX, L::NO_HITBOX);
    set_handler(m, t, L::PIT_LOOP);
    pit_loop(m)
}

/// Original: `Player_DeathPit_037b8e+$52`.
pub fn pit_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let r = m.regs();
    let at = (r.d[3] as u16, r.d[4] as u16);
    motion::on_entity_pos(m, t, |m| motion::move_e_probe(m, t, at));
    if tick(m, t) {
        manual_icons::stop_motion(m, t);
        release(m, t);
    }
    Flow::Return
}

/// Desaparece al cabo de un rato. Original: `Player_Death_Despawn_037c1a`.
pub fn despawn(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    manual_icons::stop_motion(m, t);
    m.write_u16(t + L::FIELD_7C, 0);
    m.write_u16(t + L::FIELD_7E, 0);
    m.write_u8(t + L::TIMER, L::TIMER_INIT);
    m.write_u32(t + L::HITBOX, L::NO_HITBOX);
    set_handler(m, t, L::DESPAWN_LOOP);
    despawn_loop(m)
}

/// Original: `Player_Death_Despawn_037c1a+$26`.
pub fn despawn_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if tick(m, t) {
        release(m, t);
    }
    Flow::Return
}

// --- efectos

/// Sigue al jugador (con su tinte `+$5C`); C = la animación ha acabado.
fn follow(m: &mut impl Machine, t: u32) -> bool {
    let (attr, x, y) = slug::owner_pos(m, t);
    m.write_u16(t + L::X, x);
    m.write_u16(t + L::Y, y);
    let tint = m.read_u16(t + L::TINT);
    m.write_u16(t + L::ATTR, (attr & L::ATTR_KEEP) | tint);
    anim::step(m, t)
}

/// Salpicadura: mientras el jugador cae. Original: `PlayerDeathFx_Splash_039148`.
pub fn fx_splash(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::FX_GFX);
    anim::start(m, t, L::FX_SCRIPT_SPLASH);
    m.write_u16(t + L::TINT, 0);
    set_handler(m, t, L::FX_SPLASH_LOOP);
    fx_splash_loop(m)
}

/// Original: `PlayerDeathFx_Splash_039148+$22`.
pub fn fx_splash_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if follow(m, t) {
        set_handler(m, t, L::FX_FADE);
    }
    let o = m.read_u32(t + T::OWNER);
    if m.read_u8(o + L::AIR) & L::FALLING == 0 {
        set_handler(m, t, L::FX_FADE);
    }
    Flow::Return
}

/// Se apaga y acaba. Original: `PlayerDeathFx_Loop_0391aa`.
pub fn fx_fade(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::FX_SCRIPT_FADE);
    m.write_u16(t + L::TINT, 0);
    set_handler(m, t, L::FX_FOLLOW);
    fx_follow(m)
}

/// Original: `PlayerDeathFx_Loop_0391aa+$18`.
pub fn fx_follow(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if follow(m, t) {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Onda. Original: `PlayerDeathFx_Ripple_0391ee`.
pub fn fx_ripple(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::FX_GFX);
    anim::start(m, t, L::FX_SCRIPT_RIPPLE);
    m.write_u16(t + L::TINT, L::FX_RIPPLE_TINT);
    set_handler(m, t, L::FX_RIPPLE_LOOP);
    fx_follow(m)
}

/// Original: `PlayerDeathFx_Alt_039214`.
pub fn fx_alt(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::FX_ALT_GFX);
    anim::start(m, t, L::FX_SCRIPT_ALT);
    set_handler(m, t, L::FX_ALT_LOOP);
    fx_follow(m)
}

/// Sangre, si el ajuste `BLOOD` está activo.
/// Original: `Player_SpawnBloodFx_033358`.
pub fn spawn_blood(m: &mut impl Memory, t: u32) {
    if m.read_u8(L::BLOOD_DIP) != 0 {
        child(m, t, L::BLOOD_TASK);
    }
}

/// Mancha de sangre: se queda en el escenario. Original: `$03D842`.
pub fn blood_fx(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::BLOOD_SCRIPT);
    anim::alloc_entry(m, t, L::BLOOD_GFX);
    anim::set_attr(m, t, L::BLOOD_ATTR);
    let a = m.read_u16(t + L::ATTR);
    let a = ((a & L::ATTR_KEEP) | L::BLOOD_ATTR_SET).wrapping_add(L::BLOOD_ATTR_ADD);
    m.write_u16(t + L::ATTR, a);
    set_handler(m, t, L::BLOOD_LOOP);
    blood_fx_loop(m)
}

/// Original: `$03D87C`.
pub fn blood_fx_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if anim::step(m, t) {
        return tasks::finish(m);
    }
    Flow::Return
}
