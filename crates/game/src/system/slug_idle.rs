//! Slug parado: la entrada en la espera (con el guion que toca a la
//! pendiente) y el bucle parado, que pasa a la espera inclinada si las
//! ruedas no están a nivel, al salto si se queda sin suelo, a «montado» si
//! un jugador le cae encima, a la explosión si lo destruyen o se da de
//! baja si sale de la pantalla.
//!
//! Original: `Slug_IdleEnterA_02dd20`, `Slug_IdleEnterB_02dd74`,
//! `Slug_IdleEnterC_02ddc8`, `Slug_Idle_02de18`
//! (`asm/slug/vehicle_02dd20.s`), `Slug_PhysicsF_02a824`,
//! `Slug_CheckPlayersNear_02acb8`, `Slug_SetField89_02aca2`,
//! `Slug_TryStartDestroyed_02a664` (`asm/slug/helpers_0295a6.s`), `$0304C4`,
//! `$02788C`, `$03076A`, `$03EECC`, `$03EF14` y `$032D6C`.

use super::{anim, fort, items, pickup, player, slope, slug, tasks};
use crate::layout::{slug_idle as L, task as T};
use crate::mem::{Machine, Memory};
use crate::Flow;

fn set_bits(m: &mut impl Memory, a: u32, bits: u8, on: bool) {
    let v = m.read_u8(a);
    m.write_u8(a, if on { v | bits } else { v & !bits });
}

fn wheels(m: &mut impl Memory, t: u32) -> (u32, u32) {
    (m.read_u32(t + L::WHEEL_FRONT), m.read_u32(t + L::WHEEL_REAR))
}

/// Entrada en la espera con el juego de guiones `scripts` (A o B): guion
/// según la pendiente, `+$20`, sin aceleración ni rutina extra, caja de
/// golpe de la espera, y sigue en el bucle parado.
/// Original: `Slug_IdleEnterA_02dd20` / `B_02dd74` / `C_02ddc8`.
pub fn enter(m: &mut impl Machine, scripts: u32) -> Flow {
    let t = m.task_register();
    set_bits(m, t + L::FLAGS_8C, L::BIT_8C_5, false);
    let idx = slug::slope_anim_idx(m, t);
    let script = m.read_u32(scripts + u32::from(idx) * 4);
    if script != L::NO_SCRIPT {
        anim::start(m, t, script);
    }
    m.write_u8(t + L::BYTE_20, L::BYTE_20_BASE.wrapping_add(idx as u8));
    m.write_u16(t + L::ACC_X, 0);
    m.write_u32(t + L::HANDLER_60, L::NO_HANDLER);
    m.write_u32(t + L::HITBOX_CB, L::IDLE_HITBOX);
    m.write_u32(t + T::HANDLER, L::IDLE);
    idle(m)
}

/// Bucle parado. Original: `Slug_Idle_02de18`.
pub fn idle(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    slug::update_anim(m, t);
    leave_chassis_regs(m, t);
    set_bits(m, t + L::FLAGS_8D, L::BIT_8D_4 | L::MANNED, true);
    m.write_u16(t + L::VEL_X, 0);
    if !physics(m, t) {
        m.write_u32(t + T::HANDLER, L::ANGLED);
    }
    slug::update_steering(m, t);
    anim::step(m, t);
    if slug::ground_contact(m, t).in_air {
        m.write_u32(t + T::HANDLER, L::JUMP);
    }
    if players_board(m, t) {
        m.write_u32(t + T::HANDLER, L::HUNKER);
    }
    if try_destroy(m, t) {
        m.write_u32(t + T::HANDLER, L::DEATH_EXPLODE);
    }
    let margins = if m.read_u8(L::VIEW_FLAG) != 0 { L::MARGINS_FLAGGED } else { L::MARGINS };
    if items::out_of_view_seen(m, t, Some(margins)) {
        return Flow::Goto(L::DEATH_FINISH);
    }
    Flow::Return
}

/// Registros que deja [`slug::update_anim`] y con los que sigue el
/// original: `d3` = -1 si la rueda delantera pisa más arriba que la
/// trasera (0 si no) y `d4` = suelo bajo la trasera.
fn leave_chassis_regs(m: &mut impl Machine, t: u32) {
    let (a, b) = wheels(m, t);
    let (ga, gb) = (m.read_u16(a + L::WHEEL_GROUND), m.read_u16(b + L::WHEEL_GROUND));
    let mut r = m.regs();
    r.d[3] = if (ga.wrapping_sub(gb) as i16) < 0 { 0xFFFF_FFFF } else { r.d[3] & 0xFFFF_0000 };
    r.d[4] = (r.d[4] & 0xFFFF_0000) | u32::from(gb);
    m.set_regs(r);
}

/// Física de un frame parado: bits de nivel, tope contra paredes, paso
/// con la Y de apoyo y las flechas. Devuelve si las ruedas siguen a nivel
/// (el original deja C = no). Original: `Slug_PhysicsF_02a824`.
pub fn physics(m: &mut impl Machine, t: u32) -> bool {
    update_level_bits(m, t);
    if let Some((_, dir)) = slug::wheel_against_wall(m, t) {
        slug::latch_x(m, dir as u16);
    }
    step_on_base(m, t);
    player::dir_flags(m, t);
    wheels_level(m, t)
}

/// Bits 4 y 5 de `+$13`: se quitan si las dos ruedas están a la misma
/// altura o si alguna tiene el contador `+$8E` en marcha; si no, se ponen.
/// Original: `$0304C4`.
pub fn update_level_bits(m: &mut impl Memory, t: u32) {
    let (a, b) = wheels(m, t);
    let on = m.read_u16(a + L::WHEEL_Y) != m.read_u16(b + L::WHEEL_Y)
        && (m.read_u16(b + L::WHEEL_PULSE) | m.read_u16(a + L::WHEEL_PULSE)) == 0;
    set_bits(m, t + L::FLAGS_13, L::LEVEL_BITS, on);
}

/// Paso de movimiento ([`slope::move_b`]) con la Y de apoyo (`+$82`) en
/// lugar de la del sprite, y el sombreado por profundidad. Devuelve el C
/// de `move_b`. Original: `$02788C`.
pub fn step_on_base(m: &mut impl Machine, t: u32) -> bool {
    let r = m.regs();
    let at = (r.d[3] as u16, r.d[4] as u16);
    let (x, y) = (m.read_u16(t + L::X), m.read_u16(t + L::BASE_Y));
    m.write_u16(L::WORK_POS[0], x);
    m.write_u16(L::WORK_POS[1], y);
    for (w, k) in L::WORK_SUB.into_iter().zip(0..) {
        let v = m.read_u8(t + L::SUB + k);
        m.write_u8(w, v);
    }
    let c = slope::move_b(m, t, at);
    let (x, y) = (m.read_u16(L::WORK_POS[0]), m.read_u16(L::WORK_POS[1]));
    m.write_u16(t + L::X, x);
    m.write_u16(t + L::BASE_Y, y & L::BASE_Y_MASK);
    for (w, k) in L::WORK_SUB.into_iter().zip(0..) {
        let v = m.read_u8(w);
        m.write_u8(t + L::SUB + k, v);
    }
    anim::apply_depth(m, t);
    c
}

/// ¿Están las ruedas delantera y trasera a nivel? Sí si alguna pisa suelo
/// blando o la diferencia de altura es una de `LEVEL_DIFFS`.
/// Original: `$03076A` (Z = sí).
pub fn wheels_level(m: &mut impl Memory, t: u32) -> bool {
    let (a, b) = wheels(m, t);
    if (m.read_u8(a + L::WHEEL_STATE) | m.read_u8(b + L::WHEEL_STATE)) & L::WHEEL_SOFT != 0 {
        return true;
    }
    let (ya, yb) = (m.read_u16(a + L::WHEEL_Y), m.read_u16(b + L::WHEEL_Y));
    let d = if ya < yb { yb - ya } else { ya - yb };
    L::LEVEL_DIFFS.contains(&d)
}

/// Jugador `p` (0 o 1) en juego que toca la caja de montarse del Slug.
/// Original: `$03EECC` (p = 0) / `$03EF14` (p = 1).
pub fn player_near(m: &mut impl Memory, t: u32, p: u32) -> Option<u32> {
    if m.read_u8(L::ENABLED) == 0 {
        return None;
    }
    let e = tasks::player_task_if_alive(m, p)?;
    pickup::touches(m, t, e, L::BOARD_BOX).then_some(e)
}

/// El jugador `n` (1 o 2) se monta si está cayendo sobre el Slug sin
/// estar ya montado: el Slug pasa a ser suyo y se avisa del montado.
/// Original: `$032D6C` (C = se ha montado).
pub fn board(m: &mut impl Memory, t: u32, n: u8) -> bool {
    let Some(&p) = L::PLAYERS.get(usize::from(n).wrapping_sub(1)) else { return false };
    let status = m.read_u8(p + L::P_STATUS);
    let air = m.read_u8(p + L::P_AIR);
    if status & L::P_RIDING != 0
        || air & L::P_DESCENDING == 0
        || air & L::P_IN_AIR == 0
        || (m.read_u16(p + L::P_VEL_Y) as i16) >= 0
    {
        return false;
    }
    m.write_u8(t + L::CONTROLLER, n);
    m.write_u8(p + L::P_STATUS, status | L::P_RIDING);
    let pilot = m.read_u8(p + L::PILOT);
    m.write_u8(t + L::PILOT, pilot);
    mark_boarded(m);
    true
}

/// Original: `Slug_SetField89_02aca2`.
pub fn mark_boarded(m: &mut impl Memory) {
    m.write_u8(L::SLUG + L::BOARDED, L::BOARDED_ON);
}

/// ¿Se ha montado algún jugador (primero el 1, luego el 2)?
/// Original: `Slug_CheckPlayersNear_02acb8`.
pub fn players_board(m: &mut impl Memory, t: u32) -> bool {
    if m.read_u8(L::SCENE_LOADED) == 0 {
        return false;
    }
    (0..2u8).any(|p| player_near(m, t, u32::from(p)).is_some() && board(m, t, p + 1))
}

/// Sin piloto y sin el temporizador de rango, empieza la destrucción
/// (y devuelve que no); si no, devuelve si está marcado como destruido.
/// Original: `Slug_TryStartDestroyed_02a664`.
pub fn try_destroy(m: &mut impl Memory, t: u32) -> bool {
    if m.read_u8(t + L::FLAGS_8D) & L::MANNED == 0 && m.read_u16(L::RANK_TIMER) == 0 {
        set_bits(m, t + L::FLAGS_13, L::DESTROYED, true);
        m.write_u32(t + T::HANDLER, L::DEATH_START);
        return false;
    }
    fort::dead(m, t)
}
