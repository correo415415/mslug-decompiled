//! Soldado de la torre de vigía y de los pilares del puente: espera
//! asomado hasta que entra en pantalla y, con el jugador a la vista,
//! suelta una granada de mortero (que sale del suelo, estalla, hace un
//! fogonazo y deja un casquillo). Si se rompe su pieza cae: se lleva un
//! compañero tumbado, salta, y si toca suelo huye entre humo; si le dan en
//! el aire cae sin más. Si el soldado ya ha caído, la granada en curso se
//! convierte en un soldado normal.
//!
//! Original: `TowerSoldier_*` (`$05F384`–`$05F9FE`,
//! `asm/props/late_props_turrets_05e000.s`) y `Parent_CopyPos_05e4dc`.

use super::{anim, fort, hits, items, manual_icons, motion, pickup, platforms, scroll, slug, soldier, sound, tasks, turret};
use crate::layout::{task as T, tower_soldier as L};
use crate::mem::{Flow, Machine, Memory};

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

fn owner(m: &mut impl Memory, t: u32) -> u32 {
    m.read_u32(t + L::OWNER)
}

fn add16(m: &mut impl Memory, a: u32, v: u16) {
    let x = m.read_u16(a).wrapping_add(v);
    m.write_u16(a, x);
}

fn faces_left(m: &mut impl Memory, t: u32) -> bool {
    m.read_u8(t + L::FACING) & 1 != 0
}

fn at(m: &mut impl Machine) -> (u16, u16) {
    let r = m.regs();
    (r.d[3] as u16, r.d[4] as u16)
}

fn child(m: &mut impl Memory, t: u32, handler: u32) -> u32 {
    let c = tasks::spawn(m, handler, t);
    anim::copy_sprite_pos(m, t, c);
    c
}

/// Sonido de golpe apagado (el parpadeo se quita).
fn ignore_hit(m: &mut impl Memory, t: u32) {
    fort::hit_side(m, t);
    let f = m.read_u8(t + L::FLAGS_13);
    m.write_u8(t + L::FLAGS_13, f & !L::HIT_FLASH);
}

fn owner_gone(m: &mut impl Memory, t: u32) -> bool {
    let o = owner(m, t);
    m.read_u8(o + L::STATE) == L::GONE
}

/// Avisa al soldado de que puede volver a disparar.
fn reload_owner(m: &mut impl Memory, t: u32) {
    let o = owner(m, t);
    m.write_u8(o + L::STATE, L::RELOAD);
}

/// La tarea se pone donde su dueña. Original: `Parent_CopyPos_05e4dc`.
pub fn copy_owner_pos(m: &mut impl Memory, t: u32) {
    let o = owner(m, t);
    for f in [L::X, L::Y] {
        let v = m.read_u16(o + f);
        m.write_u16(t + f, v);
    }
}

/// ¿Se ha salido? Original: `Offworld_Box2C0EDE_05f9ac` (C).
pub fn off_world(m: &mut impl Memory, t: u32) -> bool {
    items::out_of_view(m, t, Some(L::MARGINS))
}

/// Original: `Offworld_Box2C0ED4_05f990` (C).
pub fn shot_off_world(m: &mut impl Memory, t: u32) -> bool {
    items::out_of_view_seen(m, t, Some(L::MARGINS_SHOT))
}

/// Original: `Offworld_Box2C0EE8_05f9c8` (C).
pub fn mate_off_world(m: &mut impl Memory, t: u32) -> bool {
    items::out_of_view_seen(m, t, Some(L::MARGINS_B))
}

fn leave(m: &mut impl Machine, t: u32) -> Flow {
    if off_world(m, t) {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Alta: asomado, con su animación. `armed` = dispara desde el principio
/// (la torre; el pilar entra por `$05F38A` sin tocar `MODE`).
/// Original: `TowerSoldier_Init_05f384`.
pub fn init(m: &mut impl Machine, armed: bool) -> Flow {
    let t = m.task_register();
    if armed {
        m.write_u8(t + L::MODE, L::ARMED);
    }
    anim::alloc_entry(m, t, L::GFX);
    for a in L::SCALE {
        m.write_u8(t + a, 0xFF);
    }
    m.write_u16(t + L::ATTR, L::ATTR_INIT);
    manual_icons::stop_motion(m, t);
    anim::start(m, t, L::SCRIPT_IDLE);
    m.write_u8(t + L::COOLDOWN, 0);
    set_handler(m, t, L::LOOP);
    init_loop(m)
}

/// Al entrar en pantalla se pone a vigilar (o a disparar).
/// Original: `TowerSoldier_Init_05f384+$46`.
pub fn init_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if m.read_u16(t + L::X) as i16 > L::WAKE_X {
        return Flow::Return;
    }
    let armed = m.read_u8(t + L::MODE) == L::ARMED;
    set_handler(m, t, if armed { L::FIRE } else { L::WATCH });
    Flow::Return
}

/// Vigila. Original: `TowerSoldier_Watch_05f3f2`.
pub fn watch(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u8(t + L::DOWN, 0);
    set_handler(m, t, L::WATCH_LOOP);
    watch_loop(m)
}

/// Con el jugador a la vista (y la espera cumplida) lanza una granada.
/// Original: `TowerSoldier_Watch_05f3f2+$C`.
pub fn watch_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    anim::step(m, t);
    ignore_hit(m, t);
    let sight = L::SIGHT[usize::from(faces_left(m, t))];
    if pickup::player_in_box(m, t, sight).is_some() {
        let n = m.read_u8(t + L::COOLDOWN);
        if n as i8 > 0 {
            m.write_u8(t + L::COOLDOWN, n - 1);
        } else {
            child(m, t, L::GRENADE);
            set_handler(m, t, L::FIRE);
        }
    }
    if owner_gone(m, t) {
        set_handler(m, t, L::DIE);
    }
    leave(m, t)
}

/// Espera a que su granada acabe. Original: `TowerSoldier_Fire_05f482`.
pub fn fire(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u8(t + L::STATE, 0);
    set_handler(m, t, L::FIRE_LOOP);
    fire_loop(m)
}

/// Original: `TowerSoldier_Fire_05f482+$C`.
pub fn fire_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    anim::step(m, t);
    ignore_hit(m, t);
    let armed = m.read_u8(t + L::MODE) == L::ARMED;
    if (armed && owner_gone(m, t)) || m.read_u8(t + L::STATE) == L::GONE {
        set_handler(m, t, L::DIE);
    }
    if m.read_u8(t + L::STATE) == L::RELOAD {
        m.write_u8(t + L::COOLDOWN, L::COOLDOWN_INIT);
        set_handler(m, t, L::WATCH);
    }
    if owner_gone(m, t) {
        set_handler(m, t, L::DIE);
    }
    leave(m, t)
}

/// Su pieza se ha roto: salta (con el compañero tumbado) y daña al caer.
/// Original: `TowerSoldier_Die_05f508`.
pub fn die(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    child(m, t, L::INIT_B);
    manual_icons::stop_motion(m, t);
    m.write_u16(t + L::VEL_X, 0);
    m.write_u16(t + L::VEL_Y, 0);
    m.write_u16(t + L::ACC_X, 0);
    m.write_u16(t + L::ACC_Y, L::DIE_ACC_Y);
    m.write_u8(t + L::DOWN, L::GONE);
    anim::start(m, t, L::SCRIPT_DIE);
    m.write_u32(t + L::ATTACKS, L::DIE_ATTACKS);
    slug::clear_flags13(m, t);
    set_handler(m, t, L::DIE_LOOP);
    die_loop(m)
}

/// Al tocar suelo huye; si acierta a alguien cae sin más.
/// Original: `TowerSoldier_Die_05f508+$56`.
pub fn die_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let at = at(m);
    if motion::on_entity_pos(m, t, |m| motion::move_e(m, t, at)) {
        set_handler(m, t, L::FLEE);
    }
    anim::step(m, t);
    trail(m, t);
    ignore_hit(m, t);
    hits::run_attack_list(m, t);
    if m.read_u8(t + L::FLAGS_13) & L::ATTACK_HIT != 0 {
        set_handler(m, t, L::FALL);
    }
    leave(m, t)
}

/// Deja rastro de plataforma bajo él (para que lo pisen).
/// Original: `TowerSoldier_OffsetPlus10_05f91e`.
pub fn trail(m: &mut impl Memory, t: u32) {
    let (dx, dy, w) = L::TRAIL;
    let x = m.read_u16(t + L::X).wrapping_add(dx);
    let y = m.read_u16(t + L::Y).wrapping_add(dy);
    platforms::record(m, t, x, y, w);
}

/// Cae por delante del escenario hasta acabar la animación.
/// Original: `TowerSoldier_Fall_05f5aa`.
pub fn fall(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::ATTR, L::ATTR_FALL);
    let a = m.read_u16(t + L::ACC_Y) as i16 >> 1;
    m.write_u16(t + L::ACC_Y, a as u16);
    m.write_u16(t + L::VEL_Y, 0);
    m.write_u32(t + L::ATTACKS, L::NO_ATTACKS);
    anim::start(m, t, L::SCRIPT_FALL);
    set_handler(m, t, L::FALL_LOOP);
    fall_loop(m)
}

/// Original: `TowerSoldier_Fall_05f5aa+$30`.
pub fn fall_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let at = at(m);
    motion::on_entity_pos(m, t, |m| motion::move_e(m, t, at));
    if anim::step(m, t) {
        return tasks::finish(m);
    }
    leave(m, t)
}

/// Huye por el suelo entre una columna de humo.
/// Original: `TowerSoldier_Flee_05f5fa`.
pub fn flee(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    sound::play(m, L::SND_FLEE);
    m.write_u16(t + L::ATTR, L::ATTR_FLEE);
    let c = child(m, t, L::SMOKE_COLUMN);
    add16(m, c + L::ATTR, 1);
    scroll::blit_cells(m, t, L::FLEE_CELLS);
    manual_icons::stop_motion(m, t);
    anim::start(m, t, L::SCRIPT_FLEE);
    set_handler(m, t, L::FLEE_LOOP);
    flee_loop(m)
}

/// Original: `TowerSoldier_Flee_05f5fa+$52`.
pub fn flee_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    motion::walk_c(m, t);
    anim::step(m, t);
    leave(m, t)
}

/// Compañero tumbado: se va cuando se acaba la torre.
/// Original: `TowerSoldier_InitB_05f668`.
pub fn mate(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::GFX);
    for a in L::SCALE {
        m.write_u8(t + a, 0xFF);
    }
    manual_icons::stop_motion(m, t);
    anim::start(m, t, L::SCRIPT_B);
    set_handler(m, t, L::INIT_B_LOOP);
    mate_loop(m)
}

/// Original: `TowerSoldier_InitB_05f668+$2E`.
pub fn mate_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    anim::step(m, t);
    let o = owner(m, t);
    let g = owner(m, o);
    if m.read_u8(g + L::STATE) == L::GONE || mate_off_world(m, t) {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Granada de mortero: aparece delante, sube hasta la superficie.
/// Original: `TowerSoldier_Grenade_05f6c4`.
pub fn grenade(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::GRENADE_GFX);
    let dx = L::GRENADE_DX[usize::from(faces_left(m, t))];
    add16(m, t + L::X, dx);
    add16(m, t + L::Y, L::GRENADE_DY);
    manual_icons::stop_motion(m, t);
    anim::start(m, t, L::SCRIPT_GRENADE);
    m.write_u16(t + L::VEL_Y, L::GRENADE_VEL_Y);
    loop {
        add16(m, t + L::Y, 0xFFFF);
        let (x, y) = (m.read_u16(t + L::X), m.read_u16(t + L::Y));
        if turret::kind_at(m, t, x, y) != 0 {
            break;
        }
    }
    m.write_u8(t + L::DOWN, 0);
    set_handler(m, t, L::GRENADE_LOOP);
    grenade_loop(m)
}

/// Fuera de la pantalla, el soldado puede volver a disparar.
fn shot_tail(m: &mut impl Machine, t: u32) -> Flow {
    if shot_off_world(m, t) {
        reload_owner(m, t);
        return tasks::finish(m);
    }
    Flow::Return
}

/// Original: `TowerSoldier_Grenade_05f6c4+$5E`.
pub fn grenade_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if anim::step(m, t) {
        set_handler(m, t, L::BURST);
    }
    shot_tail(m, t)
}

/// Si el soldado ya ha caído, pasa a soldado normal con un fogonazo.
fn to_soldier(m: &mut impl Memory, t: u32) {
    set_handler(m, t, L::TO_SOLDIER);
    child(m, t, L::FLASH);
}

/// Golpe recibido: el soldado puede volver a disparar.
fn hit_reload(m: &mut impl Memory, t: u32) {
    if fort::hit_side(m, t).is_some() {
        reload_owner(m, t);
    }
}

/// Estalla y avanza. Original: `TowerSoldier_GrenadeBurst_05f752`.
pub fn burst(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    manual_icons::stop_motion(m, t);
    anim::start(m, t, L::SCRIPT_BURST);
    let v = if faces_left(m, t) { L::BURST_VEL_X.wrapping_neg() } else { L::BURST_VEL_X };
    m.write_u16(t + L::VEL_X, v);
    set_handler(m, t, L::BURST_LOOP);
    burst_loop(m)
}

/// Original: `TowerSoldier_GrenadeBurst_05f752+$2C`.
pub fn burst_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let at = at(m);
    motion::on_entity_pos(m, t, |m| motion::move_e_probe(m, t, at));
    anim::step(m, t);
    hit_reload(m, t);
    let o = owner(m, t);
    if m.read_u8(o + L::DOWN) == L::GONE {
        to_soldier(m, t);
    }
    if near_owner(m, t) {
        set_handler(m, t, L::MUZZLE);
    }
    soldier::check_death(m, t);
    shot_tail(m, t)
}

/// ¿Está junto a su soldado? Original: `TowerSoldier_SpawnDebris_05f94c` (C).
pub fn near_owner(m: &mut impl Memory, t: u32) -> bool {
    let o = owner(m, t);
    let mut dist = |f: u32| super::barrel::abs16(m.read_u16(o + f).wrapping_sub(m.read_u16(t + f))) as i16;
    dist(L::X) <= L::NEAR.0 && dist(L::Y) <= L::NEAR.1
}

/// Fogonazo en la boca del mortero. Original: `TowerSoldier_Muzzle_05f7f4`.
pub fn muzzle(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    copy_owner_pos(m, t);
    let dx = if faces_left(m, t) { L::MUZZLE_DX.wrapping_neg() } else { L::MUZZLE_DX };
    add16(m, t + L::X, dx);
    add16(m, t + L::Y, L::MUZZLE_DY);
    manual_icons::stop_motion(m, t);
    anim::start(m, t, L::SCRIPT_MUZZLE);
    set_handler(m, t, L::MUZZLE_LOOP);
    muzzle_loop(m)
}

/// Original: `TowerSoldier_Muzzle_05f7f4+$38`.
pub fn muzzle_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if anim::step(m, t) {
        set_handler(m, t, L::SHELL);
    }
    hit_reload(m, t);
    let o = owner(m, t);
    if m.read_u8(o + L::DOWN) == L::GONE && m.read_u8(t + L::DOWN) != L::GONE {
        to_soldier(m, t);
    }
    soldier::check_death(m, t);
    shot_tail(m, t)
}

/// Casquillo. Original: `TowerSoldier_Shell_05f8a8`.
pub fn shell(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    manual_icons::stop_motion(m, t);
    anim::start(m, t, L::SCRIPT_SHELL);
    set_handler(m, t, L::SHELL_LOOP);
    shell_loop(m)
}

/// Original: `TowerSoldier_Shell_05f8a8+$18`.
pub fn shell_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if anim::step(m, t) {
        to_soldier(m, t);
    }
    hit_reload(m, t);
    soldier::check_death(m, t);
    shot_tail(m, t)
}

/// Llamada del guion: el soldado da su pieza por rota y se tumba.
/// Original: `TowerSoldier_Helper_05f93a`.
pub fn give_up(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let o = owner(m, t);
    m.write_u8(o + L::STATE, L::GONE);
    m.write_u8(t + L::DOWN, L::GONE);
    Flow::Return
}
