//! Cosas comunes de los objetos del suelo (comida, armas, cajas...): la
//! cola de cada frame, que da de baja el objeto cuando sale de la vista, y
//! el test de «fuera de la vista» que usan muchas entidades.
//!
//! Original: `Item_GroundTail_09a75a`, `$05DD56` / `$05DD5C`.

use super::{anim, collision, contact, fort, pickup, platforms, sound, tasks};
use crate::entity::PlayerId;
use crate::player::weapons;
use crate::layout::{items as L, task as T};
use crate::mem::{Flow, Machine, Memory};

/// ¿Ha salido de la vista la entidad `t`? `margins` = {izquierda, derecha,
/// abajo, arriba} (palabras con signo) que amplían la pantalla, o `None`
/// para los de por defecto. Hasta que la entidad no ha estado en pantalla
/// (bit 7 de `FLAGS2`) nunca sale; se marca al entrar o al estar muy lejos.
/// Original: `$05DD5C`.
pub fn out_of_view(m: &mut impl Memory, t: u32, margins: Option<u32>) -> bool {
    let margins = margins.unwrap_or(L::DEFAULT_MARGINS);
    let x = m.read_u16(t + L::X) as i16;
    let y = m.read_u16(t + L::Y) as i16;
    let f = m.read_u8(t + T::FLAGS2);
    if f & L::SEEN != 0 {
        let mut w = |k: u32| m.read_u16(margins + 2 * k) as i16;
        let (l, r, b, top) = (w(0), w(1), w(2), w(3));
        let left = L::VIEW_X.0.wrapping_add(l);
        let right = L::VIEW_X.1.wrapping_add(r);
        let bottom = L::VIEW_Y.0.wrapping_sub(top);
        let upper = L::VIEW_Y.1.wrapping_sub(b);
        return x < left || x > right || y < bottom || y > upper;
    }
    let on_screen = x > L::VIEW_X.0 && x < L::VIEW_X.1 && y > L::VIEW_Y.0 && y < L::VIEW_Y.1;
    let far = x <= L::FAR_X.0 || x >= L::FAR_X.1 || y <= L::FAR_Y.0 || y >= L::FAR_Y.1;
    if on_screen || far {
        m.write_u8(t + T::FLAGS2, f | L::SEEN);
    }
    false
}

/// Como [`out_of_view`], dándola antes por vista. Original: `$05DD56`.
pub fn out_of_view_seen(m: &mut impl Memory, t: u32, margins: Option<u32>) -> bool {
    let f = m.read_u8(t + T::FLAGS2);
    m.write_u8(t + T::FLAGS2, f | L::SEEN);
    out_of_view(m, t, margins)
}

/// Cola de cada frame de un objeto del suelo: cuenta los que están en el
/// suelo y sigue con [`cull`]. Original: `Item_GroundTail_09a75a`.
pub fn ground_tail(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if m.read_u8(t + L::STATE) == L::ON_GROUND {
        let n = m.read_u8(L::ON_GROUND_COUNT);
        m.write_u8(L::ON_GROUND_COUNT, n.wrapping_add(1));
    }
    cull(m)
}

/// Da de baja el objeto si sale de la vista (si `+$9F`, aunque no se haya
/// visto). Original: `Item_GroundTail_09a75a+$10`.
pub fn cull(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let out = if m.read_u8(t + L::ALWAYS_CULL) != 0 {
        out_of_view_seen(m, t, Some(L::MARGINS))
    } else {
        out_of_view(m, t, Some(L::MARGINS))
    };
    if out {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Deja una muestra de rastro bajo el objeto (los jugadores pueden
/// subirse). Original: `Item_TrailRecord_09b8ee`.
pub fn trail(m: &mut impl Memory, t: u32) {
    let x = m.read_u16(t + L::X).wrapping_sub(L::TRAIL_DX);
    let y = m.read_u16(t + L::Y).wrapping_add(L::TRAIL_DY);
    platforms::record(m, t, x, y, L::TRAIL_W);
}

/// Prisionero atado (plantilla 293): sprite, caja y prioridad.
/// Original: `Item_Tmpl293_Pow_09b3c6`.
pub fn pow(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::POW_GFX);
    m.write_u32(t + L::HITBOX, L::POW_HITBOX);
    anim::set_attr(m, t, L::POW_ATTR);
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, a & L::ATTR_KEEP);
    anim::start(m, t, L::S_POW);
    m.write_u32(t + T::HANDLER, L::POW_LOOP);
    pow_loop(m)
}

/// Cae, sirve de plataforma, nota el contacto con los jugadores y se
/// suelta cuando le dan. Original: `Item_Tmpl293_Pow_09b3c6+$3A`.
pub fn pow_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::item_physics(m, t);
    trail(m, t);
    contact::touch_players(m, t);
    m.write_u16(t + L::ATTR, L::POW_ATTR);
    anim::step(m, t);
    if fort::hit_side(m, t).is_some() {
        let f = m.read_u8(t + L::FLAGS_13);
        m.write_u8(t + L::FLAGS_13, f & !L::HITTABLE);
    }
    if fort::dead(m, t) {
        m.write_u32(t + T::HANDLER, L::POW_FREED);
    }
    cull(m)
}

/// Suelto: sonido y animación de liberarse. Original: `Item_Pow_Die_09b43e`.
pub fn pow_freed(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    sound::play(m, L::SND_POW_FREED);
    anim::start(m, t, L::S_POW_FREED);
    m.write_u32(t + T::HANDLER, L::POW_FREED_LOOP);
    pow_freed_loop(m)
}

/// Original: `Item_Pow_Die_09b43e+$1C`.
pub fn pow_freed_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if anim::step(m, t) {
        m.write_u32(t + T::HANDLER, L::POW_GONE);
    }
    cull(m)
}

/// Suelta el objeto de tipo `kind` (el byte de `t` en `field` y el
/// siguiente pasan a sus parámetros) delante de `t`. `None` si `kind` = 0.
/// Original: `Item_SpawnFromParent_09a7cc` (C = 0 si lo crea).
pub fn spawn_from(m: &mut impl Memory, t: u32, kind: u8, field: u16) -> Option<u32> {
    if kind == 0 {
        return None;
    }
    let mut k = kind & L::SPAWN_MASK;
    if k >= L::SPAWN_KINDS {
        k = L::SPAWN_DEFAULT;
    }
    let handler = m.read_u32(L::SPAWN_HANDLERS + u32::from(k) * 4);
    let o = tasks::spawn(m, handler, L::SPAWN_PARENT);
    let y = m.read_u16(t + L::Y);
    m.write_u16(o + L::Y, y);
    let dx = if m.read_u8(t + L::FACING) & 1 != 0 { L::SPAWN_DX } else { L::SPAWN_DX.wrapping_neg() };
    let x = m.read_u16(t + L::X).wrapping_add(dx);
    m.write_u16(o + L::X, x);
    let f = t + u32::from(field);
    let (a, b) = (m.read_u8(f), m.read_u8(f + 1));
    m.write_u8(o + L::P_KIND, a);
    m.write_u8(o + L::P_ARG, b);
    Some(o)
}

/// Como [`spawn_from`] pero el objeto sale en la X de `t`.
/// Original: `Entity_ProbeMoveX_09A7AA`.
pub fn spawn_here(m: &mut impl Memory, t: u32, kind: u8, field: u16) -> Option<u32> {
    let o = spawn_from(m, t, kind, field)?;
    let dx = if m.read_u8(t + L::FACING) & 1 != 0 { L::SPAWN_DX.wrapping_neg() } else { L::SPAWN_DX };
    let x = m.read_u16(o + L::X).wrapping_add(dx);
    m.write_u16(o + L::X, x);
    Some(o)
}

/// Al desaparecer, deja el objeto que lleva (un poco más abajo).
/// Original: `Item_RespawnBelow_09b90a`.
pub fn respawn_below(m: &mut impl Memory, t: u32) {
    let kind = m.read_u8(t + L::P_KIND);
    if let Some(o) = spawn_here(m, t, kind, L::P_ARG as u16) {
        let y = m.read_u16(o + L::Y).wrapping_add(L::RESPAWN_DY);
        m.write_u16(o + L::Y, y);
    }
}

/// Caja de munición (plantillas 284–287, `k` 0–3 da el arma 4, 3, 1 o 2):
/// sprite, unos frames sin poder recogerse, prioridad, guion y el bucle.
/// Original: `Item_Tmpl284_Ammo4_09a918`, `Item_Tmpl285_Ammo3_09a976`,
/// `Item_Tmpl286_Ammo1_09a9d4`, `Item_Tmpl287_Ammo2_09aa32`.
pub fn ammo(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::AMMO_GFX);
    m.write_u8(t + L::NO_PICKUP, L::AMMO_NO_PICKUP);
    anim::set_attr(m, t, L::POW_ATTR);
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, a & L::ATTR_KEEP);
    anim::start(m, t, L::AMMO_SCRIPTS[k]);
    m.write_u32(t + T::HANDLER, L::AMMO_LOOPS[k]);
    ammo_loop(m, k)
}

/// Bucle de la caja de munición: cae, anima y, si la toca un jugador, le
/// da el arma con la cantidad del objeto y pasa a recogido.
/// Original: `Item_Tmpl284_Ammo4_09a918+$38` (y las otras tres).
pub fn ammo_loop(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    collision::fall(m, t);
    anim::step(m, t);
    if let Some((p, _)) = pickup::item_toucher(m, t, L::PICKBOX_A) {
        let amount = m.read_u8(t + L::P_KIND);
        give_weapon(m, PlayerId::from_index(p), L::AMMO_WEAPONS[k], amount);
        m.write_u32(t + T::HANDLER, L::TAKEN[0]);
    }
    ground_tail(m)
}

/// Da al jugador el arma `weapon` con `amount`; si está en el estado 2
/// suena el aviso; y marca el arma nueva con su parpadeo.
/// Original: `Item_GiveAmmo_09a858` (con `$032CBA`).
pub fn give_weapon(m: &mut impl Memory, p: PlayerId, weapon: u8, amount: u8) {
    weapons::set_weapon_and_ammo(m, p, weapon, amount);
    let e = L::PLAYERS[p.index()];
    if m.read_u8(e + L::PLAYER_STATE) == L::PLAYER_STATE_ALERT {
        sound::play(m, L::SND_WEAPON);
    }
    m.write_u8(e + L::PLAYER_FLASH, L::WEAPON_FLASH);
    let s = m.read_u8(e + L::PLAYER_STATUS);
    m.write_u8(e + L::PLAYER_STATUS, s | L::NEW_WEAPON);
}

/// Recogido (`k` 0–3, otro sprite cada uno): un par de frames con la
/// prioridad alta y luego sube. Original: `Item_Taken_SndA_09ae26` …
/// `Item_Taken_SndD_09ae3e` (con `Item_Taken_Common_09ae42`).
pub fn taken(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::TAKEN_GFX + k as u16);
    m.write_u16(t + L::TAKEN_TIMER, L::TAKEN_FRAMES);
    anim::set_attr(m, t, L::TAKEN_ATTR);
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, a & L::ATTR_KEEP);
    m.write_u32(t + T::HANDLER, L::TAKEN_LOOP);
    taken_loop(m)
}

/// Original: `Item_Taken_Common_09ae42+$28`.
pub fn taken_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    anim::step(m, t);
    let n = m.read_u16(t + L::TAKEN_TIMER).wrapping_sub(1);
    m.write_u16(t + L::TAKEN_TIMER, n);
    if (n as i16) <= 0 {
        m.write_u32(t + T::HANDLER, L::TAKEN_RISE);
    }
    cull(m)
}

/// Sube con su animación y termina al acabarla.
/// Original: `Item_Taken_Rise_09ae8e`.
pub fn taken_rise(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::RISE_GFX);
    anim::start(m, t, L::S_RISE);
    m.write_u32(t + T::HANDLER, L::RISE_LOOP);
    taken_rise_loop(m)
}

/// Original: `Item_Taken_Rise_09ae8e+$1C`.
pub fn taken_rise_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if anim::step(m, t) {
        return tasks::finish(m);
    }
    cull(m)
}
