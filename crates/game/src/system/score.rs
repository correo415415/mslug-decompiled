//! Marcador de puntos flotante: al recoger o destruir algo aparece la
//! cantidad sobre el objeto (suma los puntos al jugador), sus cifras saltan
//! y caen, se apilan y, al cabo de un rato, parpadean y desaparecen.
//!
//! Original: `Score_Popup_Value_09b51e` (con `Score_Popup_Run_09b5e4` y su
//! bucle), `Score_Digit_09b66a`, `Score_Digit_Lift_09b75a` y sus estados,
//! `Jsr5B6ThenJmpScheduler_09b47c`, `Item_SelectAnimParity_09bb22`,
//! `$05E45A` y la suma de puntos `$051A44`
//! (`asm/props/items_score_crates_09a0bc.s`).

use super::{anim, hits, manual_icons, motion, platforms, rng, tasks};
use crate::layout::{score as L, task as T};
use crate::mem::{Flow, Machine, Memory};

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

/// Suma los puntos BCD `points` al jugador de `t` (`+$6E`; negativo =
/// nadie). Original: `$051A44`.
pub fn award(m: &mut impl Memory, t: u32, points: u32) {
    let player = m.read_u8(t + L::LAST_PLAYER);
    if player & 0x80 != 0 {
        return;
    }
    m.write_u32(L::SCORE_POINTS, points);
    hits::add_score(m, L::SCORES[usize::from(player != 0)], L::SCORE_POINTS);
}

/// Como [`award`], pero antes toma como jugador el dueño del último
/// ataque que le ha dado (si lo hay). Original: `$051A28`.
pub fn award_attacker(m: &mut impl Memory, t: u32, points: u32) {
    let a = m.read_u32(t + L::ATTACKER);
    if a != L::NO_ATTACKER {
        let p = m.read_u8(a + L::PLAYER);
        m.write_u8(t + L::LAST_PLAYER, p);
    }
    award(m, t, points);
}

/// Marcador con una cantidad: suma los puntos de su tipo (`+$98`, de la
/// tabla normal o la alternativa según `+$9C`), calcula cuántas cifras
/// tiene, se coloca a la derecha de la última y un poco más abajo al azar
/// y crea una tarea por cifra, de la última a la primera.
/// Original: `Score_Popup_Value_09b51e`.
pub fn popup_value(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let player = m.read_u8(t + L::PLAYER);
    m.write_u8(t + L::LAST_PLAYER, player);
    let kind = u32::from(m.read_u8(t + L::KIND) & 0xF);
    let alt = usize::from(m.read_u8(t + L::ALT) != 0);
    let points = m.read_u32(L::VALUES[alt] + kind * 4);
    m.write_u32(t + L::PENDING, points);
    award(m, t, points);
    let n = m.read_u8(L::COUNTS[alt] + kind);
    m.write_u8(t + L::DIGITS_LEFT, n);
    m.write_u8(t + L::FALLING, n);
    let x = m.read_u16(t + L::X).wrapping_add(L::DIGIT_STEP * u16::from(n & 0xF));
    m.write_u16(t + L::X, x);
    let (r, _) = rng::next(m);
    let y = m.read_u16(t + L::Y).wrapping_add((r & L::Y_JITTER) + L::Y_BASE);
    m.write_u16(t + L::Y, y);
    m.write_u8(t + L::FACING, 0);
    loop {
        let d = tasks::spawn(m, L::DIGIT, t);
        anim::copy_sprite_pos(m, t, d);
        m.write_u8(d + L::PLAYER, player);
        let x = m.read_u16(t + L::X).wrapping_sub(L::DIGIT_STEP);
        m.write_u16(t + L::X, x);
        let pending = m.read_u32(t + L::PENDING);
        m.write_u32(t + L::PENDING, pending >> 4);
        m.write_u16(d + L::DIGIT_VALUE, (pending & 0xF) as u16);
        let left = m.read_u8(t + L::DIGITS_LEFT);
        m.write_u16(d + L::DIGIT_DELAY, u16::from(left & 7) * 2);
        let left = left.wrapping_sub(1);
        m.write_u8(t + L::DIGITS_LEFT, left);
        if (left as i8) <= 0 {
            break;
        }
    }
    popup_run(m, t)
}

/// Espera a que caigan las cifras y luego cuenta un rato antes de que
/// parpadeen; también hay un tiempo máximo de vida.
/// Original: `Score_Popup_Run_09b5e4`.
fn popup_run(m: &mut impl Machine, t: u32) -> Flow {
    m.write_u16(t + L::TIMER, L::HOLD_FRAMES);
    m.write_u16(t + L::LIFETIME, L::LIFE_FRAMES);
    set_handler(m, t, L::RUN_LOOP);
    popup_loop(m)
}

/// Bucle del marcador: con las cifras ya en su sitio (`+$21` = 0) cuenta
/// y pasa a parpadear (`+$21` = $FF); al acabar el parpadeo, o la vida, o
/// al salir de la vista, se va con sus cifras.
/// Original: `Score_Popup_Run_09b5e4+$12`.
pub fn popup_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    let falling = m.read_u8(t + L::FALLING);
    if falling == L::BLINKING || falling == 0 {
        let n = m.read_u16(t + L::TIMER).wrapping_sub(1);
        m.write_u16(t + L::TIMER, n);
        if (n as i16) <= 0 {
            if falling != 0 {
                set_handler(m, t, L::FREE);
            } else {
                m.write_u8(t + L::FALLING, L::BLINKING);
                m.write_u16(t + L::TIMER, L::BLINK_FRAMES);
            }
        }
    }
    let n = m.read_u16(t + L::LIFETIME).wrapping_sub(1);
    m.write_u16(t + L::LIFETIME, n);
    if (n as i16) <= 0 {
        set_handler(m, t, L::FREE);
    }
    if super::items::out_of_view_seen(m, t, Some(L::MARGINS)) {
        set_handler(m, t, L::FREE);
    }
    Flow::Return
}

/// Se va con todas sus hijas. Original: `Jsr5B6ThenJmpScheduler_09b47c`.
pub fn free(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    tasks::kill_children(m, t);
    tasks::finish(m)
}

/// ¿Ya no está el marcador (padre)? Original: `$05E45A`.
fn parent_gone(m: &mut impl Memory, t: u32) -> bool {
    let p = m.read_u32(t + L::PARENT);
    L::INACTIVE.contains(&m.read_u32(p))
}

/// Sprite del frame: el par o el impar según el contador de frames.
/// Original: `Item_SelectAnimParity_09bb22`.
fn parity(m: &mut impl Memory, t: u32) {
    let odd = m.read_u8(L::FRAME_COUNTER) & 1 != 0;
    let s = m.read_u16(t + if odd { L::SPRITE_ODD } else { L::SPRITE_EVEN });
    m.write_u16(t + L::SPRITE, s);
}

/// Paso con colisión (el original no mira el resultado). Original: `$027CEE`.
fn move_step(m: &mut impl Machine, t: u32) {
    let r = m.regs();
    let at = (r.d[3] as u16, r.d[4] as u16);
    motion::on_entity_pos(m, t, |m| motion::move_e_probe(m, t, at));
}

/// Una cifra: sus gráficos (según el jugador), prioridad alta, salta hacia
/// arriba con gravedad y el guion de su valor.
/// Original: `Score_Digit_09b66a`.
pub fn digit(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let (a, b) = L::DIGIT_GFX[usize::from(m.read_u8(t + L::PLAYER) != 0)];
    anim::alloc_entry(m, t, a);
    anim::alloc_entry(m, t, b);
    anim::set_attr(m, t, L::DIGIT_ATTR);
    let attr = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, (attr & L::ATTR_KEEP) | L::DIGIT_PRIORITY);
    m.write_u16(t + L::VEL_X, 0);
    m.write_u16(t + L::VEL_Y, L::DIGIT_VEL_Y);
    m.write_u16(t + L::ACC_Y, L::DIGIT_ACC_Y);
    m.write_u16(t + L::ACC_X, 0);
    let v = u32::from(m.read_u16(t + L::DIGIT_VALUE) & 0x1F);
    let script = m.read_u32(L::DIGIT_SCRIPTS + v * 4);
    if script != T::NIL {
        anim::start(m, t, script);
    }
    set_handler(m, t, L::DIGIT_LOOP);
    digit_loop(m)
}

/// Bucle de la cifra: tras su retardo salta; al bajar hasta la altura del
/// marcador se queda encima (y descuenta una de las que caen).
/// Original: `Score_Digit_09b66a+$98`.
pub fn digit_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    let n = m.read_u16(t + L::DIGIT_DELAY).wrapping_sub(1);
    m.write_u16(t + L::DIGIT_DELAY, n);
    if (n as i16) <= 0 {
        move_step(m, t);
        let p = m.read_u32(t + L::PARENT);
        let py = m.read_u16(p + L::Y);
        if (py as i16) > (m.read_u16(t + L::Y) as i16) {
            m.write_u16(t + L::Y, py.wrapping_add(L::LIFT_DY));
            let f = m.read_u8(p + L::FALLING);
            m.write_u8(p + L::FALLING, f.wrapping_sub(1));
            set_handler(m, t, L::LIFT);
        }
        parity(m, t);
        anim::step(m, t);
    }
    if parent_gone(m, t) {
        set_handler(m, t, L::FREE);
    }
    Flow::Return
}

/// Encima del marcador: un pequeño rebote hacia arriba.
/// Original: `Score_Digit_Lift_09b75a`.
pub fn lift(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    manual_icons::stop_motion(m, t);
    m.write_u16(t + L::VEL_Y, L::LIFT_VEL);
    m.write_u16(t + L::DIGIT_DELAY, L::LIFT_FRAMES);
    set_handler(m, t, L::LIFT_LOOP);
    lift_loop(m)
}

/// Pasa a parpadear con el marcador, o a quieta si acaba el rebote.
fn follow(m: &mut impl Memory, t: u32) {
    let p = m.read_u32(t + L::PARENT);
    if m.read_u8(p + L::FALLING) == L::BLINKING {
        set_handler(m, t, L::BLINK);
    }
    if parent_gone(m, t) {
        set_handler(m, t, L::FREE);
    }
}

/// Original: `Score_Digit_Lift_09b75a+$18`.
pub fn lift_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    move_step(m, t);
    parity(m, t);
    anim::step(m, t);
    let n = m.read_u16(t + L::DIGIT_DELAY).wrapping_sub(1);
    m.write_u16(t + L::DIGIT_DELAY, n);
    if (n as i16) <= 0 {
        set_handler(m, t, L::HOLD);
    }
    follow(m, t);
    Flow::Return
}

/// Quieta encima del marcador. Original: `$09B7BC` (y su bucle `$09B7C2`).
pub fn hold(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    set_handler(m, t, L::HOLD_LOOP);
    hold_loop(m)
}

pub fn hold_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    parity(m, t);
    anim::step(m, t);
    follow(m, t);
    Flow::Return
}

/// Parpadea: sólo se anima en los frames impares.
/// Original: `$09B7F8` (y su bucle `$09B7FE`).
pub fn blink(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    set_handler(m, t, L::BLINK_LOOP);
    blink_loop(m)
}

pub fn blink_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if m.read_u8(L::FRAME_COUNTER) & 1 != 0 {
        parity(m, t);
        anim::step(m, t);
    }
    if parent_gone(m, t) {
        set_handler(m, t, L::FREE);
    }
    Flow::Return
}
