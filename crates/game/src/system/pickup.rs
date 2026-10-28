//! Recogida de objetos por los jugadores: qué jugador toca la caja de un
//! objeto y el premio de puntos de la comida (con la racha de recogidas).
//!
//! Original: `$03EE48` (jugador que toca), `$03EF5C` (caja contra caja),
//! `$05E086` / `$05E260` (jugador dentro de una caja relativa),
//! `$09A836` / `$09A84E` (cajas de los objetos), `Item_Food_Pickup_09b920`.

use super::{anim, sound, tasks};
use crate::layout::{pickup as L, task as T};
use crate::mem::{Flow, Machine, Memory};

/// ¿Toca la entidad `e` la caja `bbox` = {dx, ancho, dy, alto} (palabras)
/// de `t`? Las entidades con los bits 0 o 3 de `FLAGS2` no tocan.
/// Original: `$03EF5C`.
pub fn touches(m: &mut impl Memory, t: u32, e: u32, bbox: u32) -> bool {
    if m.read_u8(e + T::FLAGS2) & L::UNTOUCHABLE != 0 {
        return false;
    }
    let dx = m.read_u16(e + L::X).wrapping_sub(m.read_u16(t + L::X)).wrapping_add(m.read_u16(bbox));
    if dx >= m.read_u16(bbox + 2) {
        return false;
    }
    let dy = m.read_u16(e + L::Y).wrapping_sub(m.read_u16(t + L::Y)).wrapping_add(m.read_u16(bbox + 4));
    dy < m.read_u16(bbox + 6)
}

/// ¿Está el jugador `p` dentro de la caja `bbox` = {x mín., x máx.,
/// y mín., y máx.} (palabras con signo) relativa a `t`, reflejada según
/// los espejos de `t` (`+$3A` bits 0 y 1)? Original: `$05E260` (C = fuera).
pub fn in_box(m: &mut impl Memory, t: u32, p: u32, bbox: u32) -> bool {
    let flip = m.read_u8(t + L::FLIP);
    let inside = |m: &mut dyn FnMut(u32) -> u16, d: u16, mirror: bool, lo: u32| {
        let d = if mirror { d.wrapping_neg() } else { d } as i16;
        d >= m(lo) as i16 && d <= m(lo + 2) as i16
    };
    let dx = m.read_u16(p + L::X).wrapping_sub(m.read_u16(t + L::X));
    let dy = m.read_u16(p + L::Y).wrapping_sub(m.read_u16(t + L::Y));
    let mut rd = |a: u32| m.read_u16(a);
    inside(&mut rd, dx, flip & L::FLIP_X != 0, bbox) && inside(&mut rd, dy, flip & L::FLIP_Y != 0, bbox + 4)
}

/// Jugador en juego dentro de la caja `bbox` de `t` (ver [`in_box`]); si
/// están los dos, el más cercano en X (el 1 si empatan).
/// Original: `$05E086` (C = ninguno; la entidad en `a0`).
pub fn player_in_box(m: &mut impl Memory, t: u32, bbox: u32) -> Option<u32> {
    let mut found = [None, None];
    for p in 0..2 {
        if let Some(e) = tasks::player_task_if_alive(m, p) {
            if in_box(m, t, e, bbox) {
                found[p as usize] = Some(e);
            }
        }
    }
    match found {
        [Some(a), Some(b)] => {
            let x = m.read_u16(t + L::X);
            let da = (m.read_u16(a + L::X).wrapping_sub(x) as i16).unsigned_abs();
            let db = (m.read_u16(b + L::X).wrapping_sub(x) as i16).unsigned_abs();
            Some(if db < da { b } else { a })
        }
        [a, b] => a.or(b),
    }
}

/// Jugador (índice y entidad) que toca la caja `bbox` de `t`; si la tocan
/// los dos, el que está más a la derecha (el 2 si empatan). Sólo cuando se
/// puede recoger (`ENABLED`). Original: `$03EE48`.
pub fn toucher(m: &mut impl Memory, t: u32, bbox: u32) -> Option<(u8, u32)> {
    if m.read_u8(L::ENABLED) == 0 {
        return None;
    }
    let mut found: Option<u32> = None;
    for p in 0..2 {
        let Some(e) = tasks::player_task_if_alive(m, p) else { continue };
        if !touches(m, t, e, bbox) {
            continue;
        }
        found = Some(match found {
            Some(prev) if m.read_u16(e + L::X) < m.read_u16(prev + L::X) => prev,
            _ => e,
        });
    }
    found.map(|e| (u8::from(e != L::PLAYER_ONE), e))
}

/// Jugador que toca el objeto `t` con la caja `bbox`, salvo si el objeto
/// no se puede recoger todavía (`+$45`). Original: `$09A836` / `$09A84E`.
pub fn item_toucher(m: &mut impl Memory, t: u32, bbox: u32) -> Option<(u8, u32)> {
    if m.read_u8(t + L::NO_PICKUP) != 0 {
        return None;
    }
    toucher(m, t, bbox)
}

/// Si un jugador toca la comida `t`: suena, sale el marcador de puntos y
/// se apunta la racha. Devuelve si se ha recogido.
/// Original: `Item_Food_Pickup_09b920`.
pub fn food(m: &mut impl Memory, t: u32) -> bool {
    let Some((p, _)) = item_toucher(m, t, L::FOOD_BOX) else { return false };
    m.write_u8(t + L::PLAYER, p);
    sound::play(m, L::FOOD_SOUND);
    let popup = tasks::spawn(m, L::SCORE_POPUP, t);
    anim::copy_sprite_pos(m, t, popup);
    m.write_u8(popup + L::PLAYER, p);
    let kind = m.read_u8(t + L::KIND);
    m.write_u8(popup + L::KIND, kind);
    let mut value = m.read_u8(t + L::VALUE) as i8;
    match kind {
        0 => value = value.min(L::MAX_VALUE),
        L::KIND_BIG => {
            value = L::BIG_VALUE;
            m.write_u8(L::STREAK, L::MAX_VALUE as u8);
            m.write_u8(L::STREAK_TIMER, L::STREAK_FRAMES);
        }
        _ => {
            // Racha: cada recogida seguida vale uno más.
            value = value.max(m.read_u8(L::STREAK) as i8);
            m.write_u8(L::STREAK, (value as u8).wrapping_add(1));
            m.write_u8(L::STREAK_TIMER, L::STREAK_FRAMES);
            value = value.min(L::MAX_VALUE);
        }
    }
    m.write_u8(popup + L::VALUE, value as u8);
    true
}

/// Tarea de la racha de recogidas (plantilla 316): la pone a cero y la
/// mantiene. Original: `Item_Tmpl316_ComboTimer_09b82c`.
pub fn streak_start(m: &mut impl Machine) -> Flow {
    m.write_u8(L::STREAK_TIMER, 0);
    m.write_u8(L::STREAK, 0);
    let t = m.task_register();
    m.write_u32(t + T::HANDLER, L::STREAK_RUN);
    streak_run(m)
}

/// Cada frame: cuenta atrás de la racha (al acabarse vuelve a 0) y tope
/// del valor. Original: `$09B83E`.
pub fn streak_run(m: &mut impl Machine) -> Flow {
    let n = m.read_u8(L::STREAK_TIMER).wrapping_sub(1);
    m.write_u8(L::STREAK_TIMER, n);
    if n as i8 <= 0 {
        m.write_u8(L::STREAK_TIMER, 0);
        m.write_u8(L::STREAK, 0);
    }
    if m.read_u8(L::STREAK) as i8 > L::MAX_VALUE {
        m.write_u8(L::STREAK, L::MAX_VALUE as u8);
    }
    Flow::Return
}
