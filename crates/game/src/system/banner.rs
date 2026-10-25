//! Rótulo «MISSION n START» / «MISSION COMPLETE»: la tarea raíz suelta
//! una tarea hija por letra; cada letra espera su turno, cae hacia su
//! sitio, se frena y parpadea al llegar, y se engancha al rótulo. Cuando
//! han llegado todas, el rótulo espera un poco y las letras salen volando.
//! En la misión final (escena 5) el rótulo lleva una fila más.
//!
//! Original: `asm/mission/banner_07a970.s` (`$07A970` … `$07BA28`),
//! `MissionNum_TileByMission_04707e` / `MissionNum_PalByMission_04709e`,
//! `$05E23A` y `Atan2_Angle256_05e018`.

use super::{anim, items, manual_icons, motion, palette, sound, tasks, trig};
use crate::layout::{banner as L, task as T};
use crate::mem::{Flow, Machine, Memory};

fn set_handler(m: &mut impl Machine, h: u32) -> Flow {
    let t = m.task_register();
    m.write_u32(t + T::HANDLER, h);
    Flow::Return
}

fn final_mission(m: &mut impl Memory) -> bool {
    m.read_u8(L::SCENE) == L::FINAL_SCENE
}

/// Sonido de la tabla `table` según la fase (`STAGE` & 7).
/// Original: `$04707E` (`START_SOUNDS`) y `$04709E` (`END_SOUNDS`).
pub fn stage_sound(m: &mut impl Memory, table: u32) {
    let k = u32::from(m.read_u8(L::STAGE) & 7);
    let code = m.read_u16(table + k * 2);
    sound::play(m, code);
}

/// Suelta las letras de `layout`, cada una con su tarea hija.
/// Original: `BannerLayout_*` (`$07ACD0`, `$07AF50`, `$07B28E`, `$07B598`).
pub fn spawn_letters(m: &mut impl Memory, parent: u32, layout: &[L::Letter]) {
    for &(glyph, row, delay, x, y) in layout {
        let handler = if glyph.is_some() { L::LETTER_DROP } else { L::DIGIT_DROP };
        let t = tasks::spawn(m, handler, parent);
        anim::copy_sprite_pos(m, parent, t);
        if let Some(g) = glyph {
            m.write_u8(t + L::GLYPH, g);
        }
        m.write_u8(t + L::ROW, row);
        m.write_u16(t + L::DELAY, delay);
        m.write_u16(t + L::TARGET_X, x);
        m.write_u16(t + L::TARGET_Y, y);
    }
}

/// Campos comunes de la tarea raíz.
fn init_root(m: &mut impl Memory, t: u32, complete: bool) {
    m.write_u8(t + L::KIND, u8::from(complete));
    m.write_u16(t + L::X, 0);
    m.write_u16(t + L::Y, L::START_Y);
    m.write_u16(t + L::ATTR, 0xFFFF);
    m.write_u16(t + L::DELAY, 0);
}

/// «MISSION START». Original: `BannerStart_Boot_07A970`.
pub fn start(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    stage_sound(m, L::START_SOUNDS);
    init_root(m, t, false);
    let layout: &[L::Letter] = if final_mission(m) { &L::START_FINAL } else { &L::START };
    spawn_letters(m, t, layout);
    m.write_u8(t + L::LANDED, 0);
    m.write_u8(t + L::STATE, 0);
    m.write_u32(t + T::HANDLER, L::START_WAIT);
    start_wait(m)
}

/// Espera a que lleguen todas las letras. Original: `$07A9C0`.
pub fn start_wait(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let total = if final_mission(m) { L::START_FINAL.len() } else { L::START.len() } as u8;
    if m.read_u8(t + L::LANDED) != total {
        return Flow::Return;
    }
    stage_sound(m, L::END_SOUNDS);
    set_handler(m, L::WAIT)
}

/// «MISSION COMPLETE». Original: `BannerComplete_Boot_07A9F0`.
pub fn complete(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    sound::play(m, L::COMPLETE_SOUND);
    init_root(m, t, true);
    let layout: &[L::Letter] = if final_mission(m) { &L::COMPLETE_FINAL } else { &L::COMPLETE };
    spawn_letters(m, t, layout);
    m.write_u8(t + L::LANDED, 0);
    m.write_u8(t + L::STATE, 0);
    m.write_u32(t + T::HANDLER, L::COMPLETE_WAIT);
    complete_wait(m)
}

/// Original: `$07AA44`.
pub fn complete_wait(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let total = if final_mission(m) { L::COMPLETE_FINAL.len() } else { L::COMPLETE.len() } as u8;
    if m.read_u8(t + L::LANDED) != total {
        return Flow::Return;
    }
    sound::play(m, L::COMPLETE_DONE_SOUND);
    set_handler(m, L::WAIT)
}

/// Pausa con el rótulo completo. Original: `Banner_Wait26_07AA78`.
pub fn wait(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u32(t + T::HANDLER, L::WAIT_RUN);
    wait_run(m)
}

/// Original: `$07AA7E`.
pub fn wait_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let n = m.read_u16(t + L::DELAY).wrapping_add(1);
    m.write_u16(t + L::DELAY, n);
    if (n as i16) < L::HOLD_FRAMES {
        return Flow::Return;
    }
    set_handler(m, L::CLOSE)
}

/// Las letras salen volando; «START» avisa al padre y acaba, «COMPLETE»
/// funde a negro y espera. Original: `Banner_Close_07AA94`.
pub fn close(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u8(t + L::STATE, L::CLOSING);
    if m.read_u8(t + L::KIND) == 0 {
        let parent = m.read_u32(t + T::OWNER);
        m.write_u8(parent + L::OWNER_BUSY, 0);
        return tasks::finish(m);
    }
    palette::spawn_fade(m, L::FADE_KIND, t, L::FADE_PARAM);
    m.write_u16(t + L::DELAY, L::CLOSE_FRAMES);
    m.write_u32(t + T::HANDLER, L::CLOSE_RUN);
    close_run(m)
}

/// Original: `$07AACA`.
pub fn close_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let n = m.read_u16(t + L::DELAY).wrapping_sub(1);
    m.write_u16(t + L::DELAY, n);
    if (n as i16) >= 0 {
        return Flow::Return;
    }
    m.write_u8(L::SCENE_BUSY, 0);
    tasks::finish(m)
}

/// Al llegar su turno, la letra empieza a caer. Devuelve `false` si aún
/// espera.
fn release_letter(m: &mut impl Machine, t: u32) -> bool {
    let n = m.read_u16(t + L::DELAY).wrapping_sub(1);
    m.write_u16(t + L::DELAY, n);
    if (n as i16) >= 0 {
        return false;
    }
    anim::alloc_entry(m, t, L::GFX);
    let f = m.read_u8(t + L::FLIP);
    m.write_u8(t + L::FLIP, f | 1);
    m.write_u16(t + L::BLINK, L::BLINK_START);
    m.write_u8(t + L::SHRINK_X, 0xFF);
    m.write_u8(t + L::LANDED_SOUND, 0);
    true
}

fn start_glyph(m: &mut impl Memory, t: u32, table: u32, k: u8) {
    let script = m.read_u32(table + u32::from(k) * 4);
    if script != T::NIL {
        anim::start(m, t, script);
    }
}

/// Letra. Original: `BannerLetter_Drop_07AAE0`.
pub fn letter_drop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let d = m.read_u16(t + L::DELAY);
    m.write_u16(t + L::START_DELAY, d);
    m.write_u32(t + T::HANDLER, L::LETTER_DROP_RUN);
    letter_drop_run(m)
}

/// Original: `$07AAEC`.
pub fn letter_drop_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if !release_letter(m, t) {
        return Flow::Return;
    }
    m.write_u16(t + L::ATTR, 0xFFFF);
    manual_icons::stop_motion(m, t);
    let g = m.read_u8(t + L::GLYPH);
    let g = if g <= L::LAST_LETTER { g } else { 0 };
    start_glyph(m, t, L::LETTERS, g);
    m.write_u32(t + T::HANDLER, L::LETTER_TICK);
    letter_tick(m)
}

/// Número de la misión. Original: `BannerDigit_Drop_07AB5C`.
pub fn digit_drop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let d = m.read_u16(t + L::DELAY);
    m.write_u16(t + L::START_DELAY, d);
    m.write_u32(t + T::HANDLER, L::DIGIT_DROP_RUN);
    digit_drop_run(m)
}

/// Original: `$07AB68`.
pub fn digit_drop_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if !release_letter(m, t) {
        return Flow::Return;
    }
    manual_icons::stop_motion(m, t);
    let s = m.read_u8(L::SCENE);
    let s = if s <= L::LAST_DIGIT { s } else { 0 };
    start_glyph(m, t, L::DIGITS, s);
    m.write_u32(t + T::HANDLER, L::LETTER_TICK);
    letter_tick(m)
}

/// Paso de movimiento con colisión (el original no mira el resultado).
/// Original: `$027CEE`.
fn move_step(m: &mut impl Machine, t: u32) {
    let r = m.regs();
    let at = (r.d[3] as u16, r.d[4] as u16);
    motion::on_entity_pos(m, t, |m| motion::move_e_probe(m, t, at));
}

/// Hacia su sitio: velocidad = distancia × 64. Devuelve si ya está.
/// Original: `BannerLetter_Seek_07B960`.
pub fn seek(m: &mut impl Machine, t: u32) -> bool {
    let dx = m.read_u16(t + L::TARGET_X).wrapping_sub(m.read_u16(t + L::X));
    let dy = m.read_u16(t + L::TARGET_Y).wrapping_sub(m.read_u16(t + L::Y));
    if dx == 0 && dy == 0 {
        return true;
    }
    m.write_u16(t + L::VEL_X, dx << 6);
    m.write_u16(t + L::VEL_Y, dy << 6);
    let f = m.read_u8(t + T::FLAGS2);
    m.write_u8(t + T::FLAGS2, f | T::FLAGS2_FRESH);
    move_step(m, t);
    false
}

/// Distancia aproximada (mayor + menor / 2). Original: `$05E23A`.
pub fn distance(a: (u16, u16), b: (u16, u16)) -> u16 {
    let dx = (a.0.wrapping_sub(b.0) as i16).wrapping_abs() as u16;
    let dy = (a.1.wrapping_sub(b.1) as i16).wrapping_abs() as u16;
    let (big, small) = if (dx as i16) > (dy as i16) { (dx, dy) } else { (dy, dx) };
    big.wrapping_add(small >> 1)
}

/// ¿Ha llegado la letra (parpadeo acabado)? Original:
/// `BannerLetter_CheckSettle_07B9A4`.
pub fn settle(m: &mut impl Memory, t: u32) -> bool {
    m.read_u16(t + L::BLINK) == 0 || approach(m, t)
}

/// Cerca del destino, el parpadeo se va apagando. Original:
/// `BannerLetter_Approach_07B9B4`.
pub fn approach(m: &mut impl Memory, t: u32) -> bool {
    let target = (m.read_u16(t + L::TARGET_X), m.read_u16(t + L::TARGET_Y));
    let at = (m.read_u16(t + L::X), m.read_u16(t + L::Y));
    if (distance(at, target) as i16) <= L::NEAR {
        let b = m.read_u16(t + L::BLINK).wrapping_sub(L::BLINK_STEP);
        m.write_u16(t + L::BLINK, b);
        if (b as i16) < 0 {
            m.write_u16(t + L::BLINK, 0);
            return true;
        }
    }
    blink(m, t)
}

/// Escala parpadeante y, la primera vez que se encoge, el sonido de
/// llegada. Original: `BannerLetter_Blink_07B9EA` (siempre «no ha llegado»).
pub fn blink(m: &mut impl Memory, t: u32) -> bool {
    let b = m.read_u16(t + L::BLINK);
    let lo = b as u8;
    let shrinking = b & L::BLINK_GROW == 0;
    m.write_u8(t + L::SHRINK_Y, if shrinking { lo.wrapping_neg() } else { lo });
    if shrinking && m.read_u8(t + L::LANDED_SOUND) == 0 {
        anim::alloc_entry(m, t, L::LANDED_GFX);
        m.write_u8(t + L::LANDED_SOUND, 0xFF);
        m.write_u8(t + L::FLIP, 0);
    }
    false
}

/// Letra en vuelo. Original: `BannerLetter_Tick_07ABD0`.
pub fn letter_tick(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    seek(m, t);
    anim::step(m, t);
    if !settle(m, t) {
        return Flow::Return;
    }
    anim::alloc_entry(m, t, 0);
    set_handler(m, L::ATTACH)
}

/// Ya en su sitio: avisa al rótulo y espera a que se cierre (la fila
/// del número parpadea). Original: `Banner_Attach_07ABF4`.
pub fn attach(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let v = m.read_u16(t + L::PALETTE_SRC);
    m.write_u16(t + L::PALETTE, v);
    let parent = m.read_u32(t + T::OWNER);
    let n = m.read_u8(parent + L::LANDED).wrapping_add(1);
    m.write_u8(parent + L::LANDED, n);
    m.write_u8(t + L::SHRINK_X, 0xFF);
    m.write_u8(t + L::SHRINK_Y, 0xFF);
    m.write_u32(t + T::HANDLER, L::ATTACH_RUN);
    attach_run(m)
}

/// Original: `$07AC14`.
pub fn attach_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let parent = m.read_u32(t + T::OWNER);
    let hidden = m.read_u8(t + L::ROW) != 0 && m.read_u16(parent + L::DELAY) & L::ROW_BLINK != 0;
    if !hidden {
        anim::step(m, t);
    }
    if m.read_u8(parent + L::STATE) != L::CLOSING {
        return Flow::Return;
    }
    set_handler(m, L::FLY_OFF)
}

/// Sale volando, alejándose de un punto bajo la pantalla, cada vez más
/// rápido. Original: `BannerLetter_FlyOff_07AC4A`.
pub fn fly_off(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::SPEED, 0);
    m.write_u32(t + T::HANDLER, L::FLY_OFF_RUN);
    fly_off_run(m)
}

/// Original: `$07AC56`.
pub fn fly_off_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let mut speed = m.read_u16(t + L::SPEED).wrapping_add(L::SPEED_STEP);
    if speed & 0x8000 != 0 {
        speed = 0x7FFF;
    }
    m.write_u16(t + L::SPEED, speed);
    let dx = L::AWAY_FROM.0.wrapping_sub(m.read_u16(t + L::X));
    let dy = L::AWAY_FROM.1.wrapping_sub(m.read_u16(t + L::Y));
    let a = atan2(m, dx, dy).wrapping_add(0x7F);
    let (vx, vy) = trig::polar(m, u16::from(a), speed as i16);
    m.write_u16(t + L::VEL_X, vx as u16);
    m.write_u16(t + L::VEL_Y, vy as u16);
    let f = m.read_u8(t + T::FLAGS2);
    m.write_u8(t + T::FLAGS2, f | T::FLAGS2_FRESH);
    move_step(m, t);
    anim::step(m, t);
    if items::out_of_view(m, t, Some(L::MARGINS)) {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Ángulo (1/256 de vuelta) del vector (`dx`, `dy`) con las tablas de la
/// ROM. Original: `Atan2_Angle256_05e018`.
pub fn atan2(m: &mut impl Memory, dx: u16, dy: u16) -> u8 {
    let x = (dx as i16) >> 1;
    let y = (dy as i16) >> 1;
    let sx: u8 = if x < 0 { 0xFF } else { 0 };
    let sy: u8 = if y < 0 { 0xFF } else { 0 };
    let mut a = u16::from(x as u8 ^ sx);
    let mut b = u16::from(y as u8 ^ sy);
    let s4: u8 = if (a as u8).wrapping_sub(b as u8) & 0x80 != 0 { 0xFF } else { 0 };
    // `a` = el menor, `b` = el mayor.
    if a >= b {
        core::mem::swap(&mut a, &mut b);
    }
    let mut r: u8 = if a == 0 {
        s4 & 0x40
    } else {
        let la = m.read_u8(L::LOG + u32::from(a));
        let lb = m.read_u8(L::LOG + u32::from(b));
        let e = m.read_u8(L::EXP + u32::from(la.wrapping_sub(lb)));
        (e ^ s4).wrapping_sub(s4).wrapping_add(0x20)
    };
    let s3 = sx ^ sy;
    r = (r ^ s3).wrapping_sub(s3);
    r.wrapping_add(sx & 0x80)
}
