//! Escena de Game Over / Continue. Sale al acabar la partida y, en el
//! attract, cuando no hay créditos. La tarea principal cae desde arriba
//! con su velocidad. Al pasar por dos alturas «golpea» (sonido y efectos);
//! al llegar al suelo asienta la escena y espera a que haya créditos;
//! entonces lanza el texto de CONTINUE y su recorrido.
//!
//! Original: `GameOver_Boot_08f91a` … `GameOver_ContinueEnd_08feb6`.

use super::{anim, hud, palette, rng, scene, scroll, sound, tasks, video};
use crate::layout::{game_over as L, task as T};
use crate::mem::{Machine, Memory};
use crate::Flow;

/// Prepara la escena (fondo, sprites, posición, velocidad y fundido) y
/// pasa a crear las figuras. Original: `GameOver_Boot_08f91a`.
pub fn boot(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scene::load(m, L::SCENE);
    video::load_high_entries(m, L::ENTRIES);
    m.write_u16(t + L::X, L::START.0);
    m.write_u16(t + L::Y, L::START.1);
    m.write_u16(t + L::VEL_Y, L::START.2);
    for s in L::STATE {
        m.write_u8(t + s, 0);
    }
    m.write_u16(t + L::TIMER, L::START_TIMER);
    m.write_u16(t + L::COUNT, 0);
    m.write_u16(t + L::FIELD_84, 0);
    palette::spawn_fade(m, L::FADE.0, t, L::FADE.1);
    m.write_u32(t + T::HANDLER, L::SPAWN);
    Flow::Return
}

/// Hija de `t` con la rutina `routine`; si `follow`, en su posición.
fn child(m: &mut impl Memory, t: u32, routine: u32, follow: bool) -> u32 {
    let c = tasks::spawn(m, routine, t);
    if follow {
        anim::copy_sprite_pos(m, t, c);
    }
    c
}

/// Crea el texto de continuar y todas las figuras (letras, piezas,
/// fondos) y empieza a caer. Original: `GameOver_Spawn_08f96a`.
pub fn spawn(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    child(m, t, L::CONTINUE_TEXT, false);
    for (routine, param) in L::FIGURES {
        let c = child(m, t, routine, true);
        if let Some(p) = param {
            m.write_u16(c + L::CHILD_PARAM, p);
        }
    }
    sound::play(m, L::SOUND_HIT);
    m.write_u32(t + T::HANDLER, L::FALL);
    fall(m)
}

/// Mueve la Y con la velocidad (8.8, con byte de fracción) mientras esté
/// por encima de `MOVE_FLOOR`. Original: `GameOver_IntegrateY_08fcaa`.
pub fn integrate_y(m: &mut impl Memory, t: u32) {
    if (m.read_u16(t + L::Y) as i16) <= L::MOVE_FLOOR {
        return;
    }
    let v = m.read_u16(t + L::VEL_Y);
    let frac = m.read_u8(t + L::Y_FRAC);
    let (f, carry) = frac.overflowing_add(v as u8);
    m.write_u8(t + L::Y_FRAC, f);
    let step = ((v as i16) >> 8) as u16;
    let y = m.read_u16(t + L::Y).wrapping_add(step).wrapping_add(carry as u16);
    m.write_u16(t + L::Y, y);
}

/// Hijas de un golpe, cada una con el temporizador `timer`.
fn hit_fx(m: &mut impl Memory, t: u32, fx: ([u32; 3], u16)) {
    m.write_u8(t + L::STATE[1], 2);
    sound::stop(m, L::SOUND_HIT);
    for r in fx.0 {
        let c = child(m, t, r, true);
        m.write_u16(c + L::CHILD_TIMER, fx.1);
    }
}

/// Cae (un frame de cada dos); al pasar por las dos alturas golpea; al
/// llegar al suelo pasa a [`landed`]. Original: `$08FBA2`.
pub fn fall(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let n = m.read_u16(t + L::COUNT).wrapping_add(1);
    m.write_u16(t + L::COUNT, n);
    if n & 1 != 0 && (m.read_u16(t + L::Y) as i16) > L::GROUND {
        integrate_y(m, t);
        let y = m.read_u16(t + L::Y);
        let hit = m.read_u8(t + L::STATE[1]) == 2;
        if y == L::HIT_HIGH && !hit {
            hit_fx(m, t, L::HIT_HIGH_FX);
        }
        let y = m.read_u16(t + L::Y);
        let hit = m.read_u8(t + L::STATE[1]) == 2;
        if y == L::HIT_LOW && !hit {
            hit_fx(m, t, L::HIT_LOW_FX);
        }
    }
    if (m.read_u16(t + L::Y) as i16) <= L::GROUND {
        m.write_u8(t + L::STATE[1], 1);
        m.write_u16(t + L::TIMER, L::LANDED_TIMER);
        m.write_u32(t + T::HANDLER, L::LANDED);
    }
    Flow::Return
}

/// Resta uno al temporizador; `true` al llegar a 0.
fn tick(m: &mut impl Memory, t: u32) -> bool {
    let v = m.read_u16(t + L::TIMER).wrapping_sub(1);
    m.write_u16(t + L::TIMER, v);
    v == 0
}

/// En el suelo: al acabar la espera, un frame de scroll y los efectos de
/// asentarse. Original: `GameOver_Wait_08fcca`.
pub fn landed(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    integrate_y(m, t);
    if tick(m, t) {
        sound::stop(m, L::SOUND_HIT);
        m.write_u8(t + L::STATE[1], 2);
        scroll::frame(m, (0, 0));
        m.write_u16(t + L::TIMER, L::SETTLE_TIMER);
        for r in L::LANDED_FX {
            child(m, t, r, true);
        }
        m.write_u32(t + T::HANDLER, L::SETTLE);
    }
    Flow::Return
}

/// Original: `GameOver_Wait2_08fd2e`.
pub fn settle(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    integrate_y(m, t);
    if tick(m, t) {
        sound::play(m, L::SOUND_HIT);
        m.write_u8(t + L::STATE[0], 3);
        m.write_u8(t + L::STATE[1], 0);
        m.write_u16(t + L::TIMER, L::FINAL_TIMER);
        scene::hide_layer_sprites(m, L::CAMERA);
        m.write_u32(t + T::HANDLER, L::FINAL);
    }
    Flow::Return
}

/// Original: `GameOver_Final_08fd68`.
pub fn final_phase(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    integrate_y(m, t);
    if m.read_u16(t + L::TIMER) == L::FINAL_MARK_AT {
        m.write_u8(L::FINAL_MARK, 1);
        m.write_u8(t + L::STATE[1], 4);
    }
    if tick(m, t) {
        sound::play(m, L::SOUND_END);
        m.write_u8(t + L::STATE[1], 3);
        m.write_u16(t + L::TIMER, L::WAIT_TIMER);
        m.write_u32(t + T::HANDLER, L::WAIT_CREDIT);
    }
    Flow::Return
}

/// Espera a que haya créditos; entonces quita las figuras y lanza el texto
/// de CONTINUE. Original: `GameOver_WaitCredit_08fdaa`.
pub fn wait_credit(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if m.read_u8(L::CREDIT_READY) == 0 || !tick(m, t) {
        return Flow::Return;
    }
    tasks::kill_children(m, t);
    m.write_u8(t + L::STATE[0], 0);
    child(m, t, L::CONTINUE_TEXT, false);
    for r in L::CONTINUE_FX {
        child(m, t, r, true);
    }
    m.write_u32(t + T::HANDLER, L::CONTINUE);
    Flow::Return
}

/// Original: `GameOver_Continue_08fdf8`.
pub fn continue_phase(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if m.read_u8(t + L::STATE[0]) != 1 {
        return Flow::Return;
    }
    child(m, t, L::CONTINUE_MID, true);
    for (o, f) in L::ORIGIN.iter().zip([L::X, L::Y]) {
        let v = m.read_u16(t + f);
        m.write_u16(t + o, v);
    }
    m.write_u16(t + L::TIMER, L::PATH_TIMER);
    m.write_u16(t + L::COUNT, 0);
    m.write_u32(t + T::HANDLER, L::PATH);
    Flow::Return
}

/// Sigue la tabla de desplazamientos desde el origen. Original:
/// `GameOver_ContinuePath_08fe32`.
pub fn path(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let i = m.read_u16(t + L::COUNT);
    if (i as i16) < L::PATH_LEN {
        let e = L::PATH_TABLE.wrapping_add(((i.wrapping_mul(4)) as i16) as u32);
        for (k, (o, f)) in L::ORIGIN.iter().zip([L::X, L::Y]).enumerate() {
            let v = m.read_u16(e + 2 * k as u32).wrapping_add(m.read_u16(t + o));
            m.write_u16(t + f, v);
        }
        m.write_u16(t + L::COUNT, i.wrapping_add(1));
    }
    if tick(m, t) {
        m.write_u16(t + L::TIMER, L::SETTLE_TIMER);
        for r in L::PATH_FX {
            child(m, t, r, true);
        }
        m.write_u32(t + T::HANDLER, L::HOLD);
    }
    Flow::Return
}

/// Original: `GameOver_ContinueHold_08fe9c` (+ `Continue_Text_DrawTitle_091618`).
pub fn hold(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if tick(m, t) {
        m.write_u16(t + L::TIMER, L::HOLD_TIMER);
        draw_title(m);
        m.write_u32(t + T::HANDLER, L::END);
    }
    Flow::Return
}

/// «CONTINUE?» en la capa fija. Original: `Continue_Text_DrawTitle_091618`.
pub fn draw_title(m: &mut impl Memory) -> u32 {
    hud::draw_text(m, L::TITLE.0, L::TITLE.1, L::TITLE.2)
}

/// Al acabar la espera, la escena deja de esperar y la tarea se queda
/// quieta. Original: `GameOver_ContinueEnd_08feb6`.
pub fn end(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if tick(m, t) {
        m.write_u8(L::SCENE_WAIT, 0);
        m.write_u32(t + T::HANDLER, L::IDLE);
    }
    Flow::Return
}

/// ¿Está la madre de `t` en el estado `v` (byte `STATE[k]`)?
/// Original: `GO_ParentState21Is2_0910a2` … `GO_ParentState21Is4_09110a`.
pub fn parent_state_is(m: &mut impl Memory, t: u32, k: usize, v: u8) -> bool {
    let mother = m.read_u32(t + T::OWNER);
    m.read_u8(mother + L::STATE[k]) == v
}

/// Alta de la letra `k` de «GAME OVER» y primer frame. Original:
/// `GO_Letter_V0_090728` … `GO_Letter_V13_090ac6`.
pub fn letter(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    let (_, gfx, prio, state, index, hit_gfx, fall_gfx, wait, script) = L::LETTERS[k];
    anim::alloc_entry(m, t, gfx);
    anim::set_attr(m, t, L::LETTER_ATTR);
    let a = m.read_u16(t + L::ATTR) & L::ATTR_PRIO_MASK;
    m.write_u16(t + L::ATTR, a | prio);
    m.write_u8(t + L::STATE[1], state);
    m.write_u16(t + L::LETTER_INDEX, index);
    m.write_u16(t + L::LETTER_HIT_GFX, hit_gfx);
    m.write_u16(t + L::LETTER_FALL_GFX, fall_gfx);
    if wait != 0 {
        m.write_u16(t + L::LETTER_WAIT, wait);
    }
    anim::start(m, t, script);
    m.write_u8(t + L::STATE[0], 0);
    m.write_u16(t + L::LETTER_TIMER, 1);
    m.write_u32(t + T::HANDLER, L::LETTER_RUN);
    letter_run(m)
}

/// Cada frame de una letra: sigue a la madre con su desplazamiento, anima
/// (salvo mientras espera), al golpe anima su paleta, al caer cambia de
/// gráficos y después se apaga y se da de baja. Original:
/// `GO_Letter_Common_090b08` (+ `GO_Letter_FadeOut_090c12`).
pub fn letter_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if !parent_state_is(m, t, 1, 2) {
        let mother = m.read_u32(t + T::OWNER);
        let index = u32::from(m.read_u16(t + L::LETTER_INDEX));
        let table = m.read_u32(L::LETTER_OFFSETS + index * 4);
        let off = table + u32::from(m.read_u16(mother + L::PHASE)) * 4;
        for (k, f) in [L::X, L::Y].into_iter().enumerate() {
            let k = 2 * k as u32;
            let v = m
                .read_u16(L::LETTER_BASE + k)
                .wrapping_add(m.read_u16(off + k))
                .wrapping_add(m.read_u16(mother + f));
            m.write_u16(t + f, v);
        }
        let waiting = m.read_u8(t + L::STATE[1]) != 0 && m.read_u16(t + L::LETTER_WAIT) != 0;
        if waiting {
            let w = m.read_u16(t + L::LETTER_WAIT) - 1;
            m.write_u16(t + L::LETTER_WAIT, w);
        } else {
            anim::step(m, t);
        }
        if parent_state_is(m, t, 1, 1) && m.read_u8(t + L::STATE[0]) == 0 {
            m.write_u8(t + L::STATE[0], 0x80);
            let entry = m.read_u16(t + L::SPRITE_ENTRY);
            let first = m.read_u16(t + L::LETTER_HIT_GFX);
            video::start_entry_anim(m, entry, first, L::LETTER_PAL_ANIM.0, L::LETTER_PAL_ANIM.1);
        }
    }
    if parent_state_is(m, t, 1, 3) && m.read_u8(t + L::STATE[0]) != 0xFF {
        m.write_u8(t + L::STATE[0], 0xFF);
        let g = m.read_u16(t + L::LETTER_FALL_GFX);
        anim::alloc_entry(m, t, g);
    }
    if !parent_state_is(m, t, 1, 3) {
        return Flow::Return;
    }
    if m.read_u16(t + L::LETTER_TIMER) == 0 {
        return letter_fade(m, t);
    }
    let left = m.read_u16(t + L::LETTER_TIMER) - 1;
    m.write_u16(t + L::LETTER_TIMER, left);
    if left == 0 {
        for l in L::LEVELS {
            m.write_u8(t + l, L::LETTER_DIM);
        }
        if m.read_u16(t + L::LETTER_INDEX) == L::GLOW_LETTER {
            child(m, t, L::GLOW, true);
        }
    }
    Flow::Return
}

fn letter_fade(m: &mut impl Machine, t: u32) -> Flow {
    for l in L::LEVELS {
        let v = m.read_u8(t + l).wrapping_sub(L::LETTER_FADE_STEP);
        m.write_u8(t + l, v);
    }
    if m.read_u8(t + L::LEVELS[0]) > L::LETTER_FADE_END {
        Flow::Return
    } else {
        tasks::finish(m)
    }
}

/// El brillo que sale de una letra: sube de nivel poco a poco.
/// Original: `GO_Glow_090c30`.
pub fn glow(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::GLOW_GFX);
    let x = m.read_u16(t + L::X).wrapping_sub(L::GLOW_OFFSET.0);
    m.write_u16(t + L::X, x);
    let y = m.read_u16(t + L::Y).wrapping_add(L::GLOW_OFFSET.1);
    m.write_u16(t + L::Y, y);
    for l in L::LEVELS {
        m.write_u8(t + l, L::GLOW_LEVEL);
    }
    m.write_u16(t + L::GLOW_RATE, 1);
    m.write_u16(t + L::GLOW_WAIT, 1);
    palette::spawn_fade(m, L::GLOW_FADE.0, t, L::GLOW_FADE.1);
    anim::set_attr(m, t, L::GLOW_ATTR);
    let a = m.read_u16(t + L::ATTR) & L::ATTR_PRIO_MASK;
    m.write_u16(t + L::ATTR, a);
    anim::start(m, t, L::GLOW_ANIM);
    m.write_u32(t + T::HANDLER, L::GLOW_RUN);
    glow_run(m)
}

/// Original: `$090C8E` (dentro de `GO_Glow_090c30`).
pub fn glow_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::step(m, t);
    let w = m.read_u16(t + L::GLOW_WAIT).wrapping_sub(1);
    m.write_u16(t + L::GLOW_WAIT, w);
    if w != 0 {
        return Flow::Return;
    }
    let r = m.read_u16(t + L::GLOW_RATE);
    m.write_u16(t + L::GLOW_WAIT, r);
    if m.read_u8(t + L::LEVELS[0]) != 0xFF {
        for l in L::LEVELS {
            let v = m.read_u8(t + l).wrapping_add(L::LETTER_FADE_STEP);
            m.write_u8(t + l, v);
        }
    }
    Flow::Return
}

/// Brillo que se desplaza hacia un punto y se apaga; al apagarse pone a
/// la madre en el estado 1. Original: `GO_Glow_Drift_090cba`.
pub fn glow_drift(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::GLOW_GFX);
    let (x, y) = (m.read_u16(t + L::X), m.read_u16(t + L::Y));
    m.write_u16(t + L::GLOW_DRIFT_TARGET_POS[0], x.wrapping_add(L::GLOW_DRIFT_TARGET.0));
    m.write_u16(t + L::GLOW_DRIFT_TARGET_POS[1], y.wrapping_add(L::GLOW_DRIFT_TARGET.1));
    m.write_u16(t + L::X, x.wrapping_sub(L::GLOW_DRIFT_START.0));
    m.write_u16(t + L::Y, y.wrapping_add(L::GLOW_DRIFT_START.1));
    for l in L::LEVELS {
        m.write_u8(t + l, 0xFF);
    }
    m.write_u16(t + L::GLOW_RATE, 1);
    m.write_u16(t + L::GLOW_WAIT, 1);
    palette::spawn_fade(m, L::GLOW_DRIFT_FADE.0, t, L::GLOW_DRIFT_FADE.1);
    anim::set_attr(m, t, L::GLOW_ATTR);
    set_priority(m, t, 0);
    anim::start(m, t, L::GLOW_ANIM);
    m.write_u32(t + T::HANDLER, L::GLOW_DRIFT_RUN);
    glow_drift_run(m)
}

/// Original: `$090D2E` (dentro de `GO_Glow_Drift_090cba`).
pub fn glow_drift_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::step(m, t);
    // Se acerca la mitad de lo que falta (al menos +1 en X y -1 en Y).
    for (k, f) in [L::X, L::Y].into_iter().enumerate() {
        let target = m.read_u16(t + L::GLOW_DRIFT_TARGET_POS[k]);
        let pos = m.read_u16(t + f);
        if target != pos {
            let mut d = ((target.wrapping_sub(pos) as i16) >> 1) as u16;
            if d == 0 {
                d = if k == 0 { 1 } else { 0xFFFF };
            }
            m.write_u16(t + f, pos.wrapping_add(d));
        }
    }
    let w = m.read_u16(t + L::GLOW_WAIT).wrapping_sub(1);
    m.write_u16(t + L::GLOW_WAIT, w);
    if w != 0 {
        return Flow::Return;
    }
    let r = m.read_u16(t + L::GLOW_RATE);
    m.write_u16(t + L::GLOW_WAIT, r);
    for l in L::LEVELS {
        let v = m.read_u8(t + l).wrapping_sub(L::GLOW_DRIFT_DIM);
        m.write_u8(t + l, v);
    }
    if m.read_u8(t + L::LEVELS[0]) > L::GLOW_DRIFT_END {
        return Flow::Return;
    }
    let mother = m.read_u32(t + T::OWNER);
    m.write_u8(mother + L::STATE[0], 1);
    tasks::finish(m)
}

/// Prioridad de sprite: los bits 2–4 de `ATTR`.
fn set_priority(m: &mut impl Memory, t: u32, prio: u16) {
    let a = m.read_u16(t + L::ATTR) & L::ATTR_PRIO_MASK;
    m.write_u16(t + L::ATTR, a | prio);
}

/// Al golpe (la madre en estado 1) anima la paleta de su entrada una vez.
fn flash_on_hit(m: &mut impl Memory, t: u32) {
    if parent_state_is(m, t, 1, 1) && m.read_u8(t + L::STATE[1]) == 0 {
        m.write_u8(t + L::STATE[1], 0xFF);
        let entry = m.read_u16(t + L::SPRITE_ENTRY);
        let first = m.read_u16(t + L::LETTER_HIT_GFX);
        video::start_entry_anim(m, entry, first, L::LETTER_PAL_ANIM.0, L::LETTER_PAL_ANIM.1);
    }
}

/// Al acabar la escena (madre en estado 4) pasa a darse de baja.
fn finish_when_done(m: &mut impl Memory, t: u32) {
    if parent_state_is(m, t, 1, 4) {
        m.write_u32(t + T::HANDLER, L::FINISH);
    }
}

/// Figura grande (`b` = la segunda). Original: `GO_Figure_A_090db0`,
/// `GO_Figure_B_090de2`.
pub fn figure(m: &mut impl Machine, b: bool) -> Flow {
    let t = m.task_register();
    let (gfx, attr, prio, script) = if b { L::FIGURE_B } else { L::FIGURE_A };
    anim::alloc_entry(m, t, gfx);
    m.write_u8(t + L::STATE[1], 0);
    m.write_u8(t + L::STATE[0], 0);
    anim::set_attr(m, t, attr);
    if let Some(p) = prio {
        set_priority(m, t, p);
    }
    m.write_u16(t + L::LETTER_HIT_GFX, L::FIGURE_HIT_GFX);
    anim::start(m, t, script);
    m.write_u32(t + T::HANDLER, L::FIGURE_RUN);
    figure_run(m)
}

/// Sigue a la madre y anima. Original: `GO_Figure_Run_090e1c`.
pub fn figure_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if !parent_state_is(m, t, 1, 2) {
        let mother = m.read_u32(t + T::OWNER);
        for f in [L::X, L::Y] {
            let v = m.read_u16(mother + f);
            m.write_u16(t + f, v);
        }
        anim::step(m, t);
        flash_on_hit(m, t);
    }
    finish_when_done(m, t);
    Flow::Return
}

/// Rutina final: se da de baja. Original: `$090E7E`.
pub fn finish(m: &mut impl Machine) -> Flow {
    tasks::finish(m)
}

/// Pieza `+$80` de la figura. Original: `GO_Figure_Part_090e86`.
pub fn part(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::PART.0);
    m.write_u8(t + L::STATE[1], 0);
    m.write_u8(t + L::STATE[0], 0);
    anim::set_attr(m, t, 0);
    set_priority(m, t, 0);
    m.write_u16(t + L::LETTER_HIT_GFX, L::PART.1);
    anim::start(m, t, L::PART.2);
    m.write_u32(t + T::HANDLER, L::PART_RUN);
    part_run(m)
}

/// Original: `$090EC6` (dentro de `GO_Figure_Part_090e86`).
pub fn part_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if parent_state_is(m, t, 1, 2) {
        return Flow::Return;
    }
    let mother = m.read_u32(t + T::OWNER);
    let e = L::PART_OFFSETS + u32::from(m.read_u16(t + L::CHILD_PARAM)) * 4;
    for (k, f) in [L::X, L::Y].into_iter().enumerate() {
        let v = m.read_u16(e + 2 * k as u32).wrapping_add(m.read_u16(mother + f));
        m.write_u16(t + f, v);
    }
    anim::step(m, t);
    Flow::Return
}

/// Fondo que se desplaza `k` (0 o 1). Original: `GO_Scroller_A_090f00`,
/// `GO_Scroller_B_090f68`.
pub fn scroller(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    let (gfx, state, prio, vel, xs, script) = L::SCROLLERS[k];
    anim::alloc_entry(m, t, gfx);
    m.write_u8(t + L::STATE[1], state);
    m.write_u8(t + L::STATE[0], 0);
    anim::set_attr(m, t, 0);
    set_priority(m, t, prio);
    m.write_u16(t + L::VEL_X, vel);
    let i = u32::from(m.read_u16(t + L::CHILD_PARAM)) * 2;
    let x = m.read_u16(xs + i).wrapping_add(m.read_u16(t + L::X));
    m.write_u16(t + L::X, x);
    m.write_u16(t + L::LETTER_HIT_GFX, L::FIGURE_HIT_GFX);
    m.write_u16(t + L::CHILD_TIMER, L::SCROLLER_WAIT);
    anim::start(m, t, script);
    m.write_u32(t + T::HANDLER, L::SCROLLER_RUN);
    scroller_run(m)
}

/// Suma la velocidad 8.8 `vel` a la posición `pos` con el byte de fracción.
fn integrate(m: &mut impl Memory, t: u32, vel: u16, frac: u32, pos: u32) {
    let (f, carry) = m.read_u8(t + frac).overflowing_add(vel as u8);
    m.write_u8(t + frac, f);
    let step = (((vel as i16) >> 8) as u16).wrapping_add(carry as u16);
    let v = m.read_u16(t + pos).wrapping_add(step);
    m.write_u16(t + pos, v);
}

/// A la altura de la madre; se oscurece al avanzar, vuelve a la izquierda
/// al salirse, anima tras la espera y al golpe anima la paleta.
/// Original: `GO_Scroller_Run_090fce`.
pub fn scroller_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if !parent_state_is(m, t, 1, 2) {
        let mother = m.read_u32(t + T::OWNER);
        let y = m.read_u16(mother + L::Y).wrapping_add(L::SCROLLER_Y_OFFSET);
        m.write_u16(t + L::Y, y);
        let x = m.read_u16(t + L::X) as i16;
        if (m.read_u8(t + L::LEVELS[0]) as i8) > L::SCROLLER_DIM_FROM && x >= L::SCROLLER_DIM_X && x & 1 != 0 {
            for l in L::LEVELS {
                let v = m.read_u8(t + l).wrapping_sub(2);
                m.write_u8(t + l, v);
            }
        }
        let vel = m.read_u16(t + L::VEL_X);
        integrate(m, t, vel, L::X_FRAC, L::X);
        if (m.read_u16(t + L::X) as i16) > L::SCROLLER_WRAP.0 {
            for l in L::LEVELS {
                m.write_u8(t + l, 0xFF);
            }
            m.write_u16(t + L::X, L::SCROLLER_WRAP.1);
            m.write_u16(t + L::X_FRAC, 0);
        }
        let w = m.read_u16(t + L::CHILD_TIMER);
        if w != 0 {
            m.write_u16(t + L::CHILD_TIMER, w - 1);
        } else {
            anim::step(m, t);
        }
        flash_on_hit(m, t);
    }
    finish_when_done(m, t);
    Flow::Return
}

/// La madre copia su fase en la suya. Original: `GO_CopyParam84_091096`
/// (orden de un guion de animación).
pub fn copy_phase(m: &mut impl Memory, t: u32) -> u32 {
    let mother = m.read_u32(t + T::OWNER);
    let v = m.read_u16(t + L::PHASE);
    m.write_u16(mother + L::PHASE, v);
    mother
}

/// Añade una chispa al final de las hijas de `t`; devuelve lo mismo que
/// [`tasks::spawn_last`]. Original: `GO_Particle_Add_091124`.
pub fn add_particle(m: &mut impl Memory, t: u32) -> (u32, u32) {
    tasks::spawn_last(m, L::PARTICLE, t)
}

/// Chispa con velocidad y dirección al azar. Original: `GO_Particle_091130`.
pub fn particle(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let kind = (rng::next(m).0 & 1) as usize;
    let (vel_x, gravity, script) = L::PARTICLE_KINDS[kind];
    anim::alloc_entry(m, t, L::PARTICLE_GFX);
    m.write_u16(t + L::X, L::PARTICLE_POS.0);
    m.write_u16(t + L::Y, L::PARTICLE_POS.1);
    let dx = (rng::next(m).0 & 0xF).wrapping_sub(8);
    let x = m.read_u16(t + L::X).wrapping_add(dx);
    m.write_u16(t + L::X, x);
    m.write_u16(t + L::VEL_X, vel_x);
    m.write_u16(t + L::VEL_Y, L::PARTICLE_VEL_Y);
    m.write_u16(t + L::GRAVITY, gravity);
    m.write_u16(t + L::CHILD_TIMER, 0);
    let a = (rng::next(m).0 & 0xF0).wrapping_sub(0x80);
    let b = rng::next(m).0 & 0x30;
    let v = m.read_u16(t + L::VEL_X).wrapping_add(a).wrapping_add(b);
    m.write_u16(t + L::VEL_X, v);
    anim::set_attr(m, t, L::PARTICLE_ATTR);
    set_priority(m, t, 0);
    let s = if rng::next(m).0 & 3 != 0 { script } else { L::PARTICLE_RARE };
    anim::start(m, t, s);
    m.write_u32(t + T::HANDLER, L::PARTICLE_RUN);
    particle_run(m)
}

/// Se mueve el doble de rápido con gravedad, se apaga y se da de baja al
/// caer (o si la escena acaba). Original: `GO_Particle_Run_09128a`.
pub fn particle_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let mother = m.read_u32(t + T::OWNER);
    let scene = m.read_u32(mother + T::OWNER);
    if m.read_u8(scene + L::STATE[0]) == 2 {
        return tasks::finish(m);
    }
    for _ in 0..2 {
        let vel = m.read_u16(t + L::VEL_X);
        integrate(m, t, vel, L::X_FRAC, L::X);
    }
    for _ in 0..2 {
        let g = m.read_u16(t + L::GRAVITY);
        let v = m.read_u16(t + L::VEL_Y).wrapping_sub(g);
        m.write_u16(t + L::VEL_Y, v);
        integrate(m, t, v, L::Y_FRAC, L::Y);
    }
    anim::step(m, t);
    let n = m.read_u16(t + L::CHILD_TIMER).wrapping_add(1);
    m.write_u16(t + L::CHILD_TIMER, n);
    if (n as i16) >= 1 {
        m.write_u16(t + L::CHILD_TIMER, 0);
        for l in L::LEVELS {
            let v = m.read_u8(t + l).wrapping_sub(L::PARTICLE_FADE);
            m.write_u8(t + l, v);
        }
    }
    if (m.read_u16(t + L::Y) as i16) > L::PARTICLE_FLOOR {
        Flow::Return
    } else {
        tasks::finish(m)
    }
}


/// Suma a la posición la entrada `i` (4 bytes: dX, dY) de `table`.
fn add_table_offset(m: &mut impl Memory, t: u32, table: u32, i: u16) {
    let e = table.wrapping_add(i32::from(i.wrapping_mul(4) as i16) as u32);
    for (k, f) in [L::X, L::Y].into_iter().enumerate() {
        let v = m.read_u16(t + f).wrapping_add(m.read_u16(e + 2 * k as u32));
        m.write_u16(t + f, v);
    }
}

/// Sprite que entra deslizándose (`k`: 0 = derecha, 1 = izquierda).
/// Original: `GO_Sprite_Right_08fecc`, `GO_Sprite_Left_08ff2c`.
pub fn slider(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    let (gfx, x, y, offset, step, column, attr, script) = L::SLIDERS[k];
    anim::alloc_entry(m, t, gfx);
    m.write_u16(t + L::X, x);
    m.write_u16(t + L::Y, y);
    m.write_u16(t + L::ORIGIN[0], x);
    m.write_u16(t + L::SLIDE_OFFSET, offset);
    m.write_u16(t + L::SLIDE_STEP, step);
    m.write_u16(t + L::SLIDE_COLUMN, column);
    m.write_u16(t + L::COUNT, 0);
    anim::set_attr(m, t, attr);
    set_priority(m, t, 0);
    anim::start(m, t, script);
    m.write_u32(t + T::HANDLER, L::SLIDE);
    slide(m)
}

/// Se acerca hasta el origen y al llegar rebota siguiendo una tabla (la
/// primera vez marca la escena y suena). Original: `GO_Sprite_Slide_08ff88`.
pub fn slide(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let offset = m.read_u16(t + L::SLIDE_OFFSET);
    let x = m.read_u16(t + L::ORIGIN[0]).wrapping_add(offset);
    m.write_u16(t + L::X, x);
    if offset != 0 {
        let v = offset.wrapping_add(m.read_u16(t + L::SLIDE_STEP));
        m.write_u16(t + L::SLIDE_OFFSET, v);
    } else {
        let n = m.read_u16(t + L::COUNT);
        if (n as i16) < L::SLIDE_STEPS {
            if n == 0 {
                m.write_u8(L::FINAL_MARK, L::SLIDE_MARK);
                sound::play(m, L::SOUND_SLIDE);
            }
            let col = m.read_u16(t + L::SLIDE_COLUMN);
            let e = L::SLIDE_TABLE + u32::from(n) * 4;
            let a = e.wrapping_add(i32::from(col as i16) as u32);
            let x = m.read_u16(t + L::X).wrapping_add(m.read_u16(a));
            m.write_u16(t + L::X, x);
            m.write_u16(t + L::COUNT, n.wrapping_add(1));
        }
    }
    anim::step(m, t);
    Flow::Return
}

/// Sprite que sigue a la madre a una distancia fija (`k` = 0 o 1).
/// Original: `GO_Sprite_Mid_08fff6`, `GO_Sprite_MidB_09004c`.
pub fn follower(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    let (gfx, dx, dy, no_pal, attr, snd, script) = L::FOLLOWERS[k];
    anim::alloc_entry(m, t, gfx);
    m.write_u16(t + L::FOLLOW_OFFSET[0], dx);
    m.write_u16(t + L::FOLLOW_OFFSET[1], dy);
    m.write_u16(t + L::FOLLOW_NO_PAL, no_pal);
    m.write_u16(t + L::FOLLOW_PAL_DONE, 0);
    anim::set_attr(m, t, attr);
    set_priority(m, t, 0);
    if let Some(c) = snd {
        sound::play(m, c);
    }
    anim::start(m, t, script);
    m.write_u32(t + T::HANDLER, L::FOLLOW);
    Flow::Return
}

/// Sigue a la madre, anima y (si toca) anima su paleta una vez.
/// Original: `GO_Sprite_FollowParent_090098`.
pub fn follow(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let mother = m.read_u32(t + T::OWNER);
    for (k, f) in [L::X, L::Y].into_iter().enumerate() {
        let v = m.read_u16(mother + f).wrapping_add(m.read_u16(t + L::FOLLOW_OFFSET[k]));
        m.write_u16(t + f, v);
    }
    anim::step(m, t);
    if m.read_u16(t + L::FOLLOW_NO_PAL) == 0 && m.read_u16(t + L::FOLLOW_PAL_DONE) == 0 {
        m.write_u16(t + L::FOLLOW_PAL_DONE, 0xFFFF);
        let entry = m.read_u16(t + L::SPRITE_ENTRY);
        let (first, len, mode) = L::FOLLOW_PAL;
        video::start_entry_anim(m, entry, first, len, mode);
    }
    Flow::Return
}

/// Alta común de las piezas A, B y C.
fn prop(m: &mut impl Machine, t: u32, p: (u16, u16, u16, u16, u32, u32)) {
    let (gfx, x, y, attr, _, _) = p;
    anim::alloc_entry(m, t, gfx);
    m.write_u16(t + L::X, x);
    m.write_u16(t + L::Y, y);
    anim::set_attr(m, t, attr);
    set_priority(m, t, 0);
}

/// Pieza A. Original: `GO_Prop_A_0900e4`.
pub fn prop_a(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    prop(m, t, L::PROP_A);
    m.write_u16(t + L::TIMER, 0);
    anim::start(m, t, L::PROP_A.4);
    m.write_u32(t + T::HANDLER, L::PROP_A.5);
    prop_a_run(m)
}

/// Sale despedida siguiendo una tabla; se da de baja al acabar la escena.
/// Original: `$090128`.
pub fn prop_a_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if parent_state_is(m, t, 0, 3) {
        return tasks::finish(m);
    }
    let n = m.read_u16(t + L::TIMER);
    if (n as i16) < L::PROP_STEPS {
        add_table_offset(m, t, L::PROP_A_TABLE, n);
        m.write_u16(t + L::TIMER, n.wrapping_add(1));
    }
    anim::step(m, t);
    Flow::Return
}

/// Pieza B. Original: `GO_Prop_B_09016c`.
pub fn prop_b(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    prop(m, t, L::PROP_B);
    m.write_u16(t + L::TIMER, 0);
    m.write_u16(t + L::COUNT, 0);
    anim::start(m, t, L::PROP_B.4);
    m.write_u32(t + T::HANDLER, L::PROP_B.5);
    prop_b_run(m)
}

/// Anima la paleta de la pieza B y de sus tres entradas altas.
fn prop_b_flash(m: &mut impl Memory, t: u32, (len, mode): (u16, u8)) {
    let entry = m.read_u16(t + L::SPRITE_ENTRY);
    let (first, high) = L::PROP_B_PAL;
    video::start_entry_anim(m, entry, first, len, mode);
    for (i, g) in high {
        video::start_high_entry_anim(m, i, g, len, mode);
    }
}

/// Sale despedida con una tabla, luego rebota con otra (al empezar crea la
/// pieza C y cambia de gráfico). Original: `$0901B6`.
pub fn prop_b_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if parent_state_is(m, t, 0, 3) {
        return tasks::finish(m);
    }
    let n = m.read_u16(t + L::TIMER);
    if (n as i16) < L::PROP_STEPS {
        if n as i16 == L::PROP_B_FLASH_AT {
            prop_b_flash(m, t, L::PROP_B_PAL_MODES[0]);
        }
        add_table_offset(m, t, L::PROP_B_TABLE, n);
        m.write_u16(t + L::TIMER, n.wrapping_add(1));
    } else {
        let k = m.read_u16(t + L::COUNT);
        if (k as i16) < L::PROP_B_STEPS2 {
            if k == 0 {
                child(m, t, L::PROP_C_TASK, true);
                anim::alloc_entry(m, t, L::PROP_B_GFX2);
                video::load_high_entries(m, L::PROP_B_ENTRIES);
            }
            if k == 2 {
                prop_b_flash(m, t, L::PROP_B_PAL_MODES[1]);
            }
            add_table_offset(m, t, L::PROP_B_TABLE2, k);
            m.write_u16(t + L::COUNT, k.wrapping_add(1));
        }
    }
    anim::step(m, t);
    Flow::Return
}

/// Pieza C (sale de la B). Original: `GO_Prop_C_0902fe`.
pub fn prop_c(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    prop(m, t, L::PROP_C);
    anim::start(m, t, L::PROP_C.4);
    m.write_u32(t + T::HANDLER, L::PROP_C.5);
    prop_c_run(m)
}

/// Anima y se da de baja al acabar la animación. Original: `$09033C`.
pub fn prop_c_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if anim::step(m, t) {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Sprite de efectos de los golpes con su guion de animación.
fn fx_sprite(m: &mut impl Machine, t: u32, gfx: u16, attr: u16, script: u32) {
    anim::alloc_entry(m, t, gfx);
    anim::set_attr(m, t, attr);
    set_priority(m, t, L::FX_SPRITE.2);
    let _ = script;
}

/// Resta uno al temporizador; al llegar a cero se da de baja con sus
/// hijas y si no, anima.
fn animate_until_timer(m: &mut impl Machine, t: u32) -> Flow {
    let n = m.read_u16(t + L::TIMER).wrapping_sub(1);
    m.write_u16(t + L::TIMER, n);
    if n == 0 {
        tasks::kill_children(m, t);
        return tasks::finish(m);
    }
    anim::step(m, t);
    Flow::Return
}

/// Sólo anima. Original: `$0905BA`, `$090628`, `$090684`, `$0906E0`.
fn animate_only(m: &mut impl Machine, t: u32) -> Flow {
    anim::step(m, t);
    Flow::Return
}

/// Resta a `field` la mitad (`shift` = 1) o la cuarta parte (2) de sí
/// mismo, al menos uno; devuelve si ha quedado negativo (y lo pone a 0).
fn shrink(m: &mut impl Memory, t: u32, field: u32, shift: u32) -> bool {
    let v = m.read_u16(t + field);
    let d = (v >> shift).max(1);
    let r = v.wrapping_sub(d);
    if (r as i16) < 0 {
        m.write_u16(t + field, 0);
        true
    } else {
        m.write_u16(t + field, r);
        false
    }
}

/// Zoom del golpe alto: se acerca desde grande y encoge al final.
/// Original: `GO_Zoom_09034e`.
pub fn zoom(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let (gfx, attr, _) = L::FX_SPRITE;
    fx_sprite(m, t, gfx, attr, L::ZOOM_SCRIPT);
    m.write_u8(t + L::STATE[1], 0);
    m.write_u8(t + L::STATE[0], 0);
    m.write_u16(t + L::ORIGIN[0], L::ZOOM_CENTER.0);
    m.write_u16(t + L::ORIGIN[1], L::ZOOM_CENTER.1);
    m.write_u16(t + L::ZOOM_SIZE, L::ZOOM_START_SIZE);
    m.write_u16(t + L::ZOOM_SIZE_Y, 0);
    m.write_u16(t + L::COUNT, 0);
    anim::start(m, t, L::ZOOM_SCRIPT);
    m.write_u32(t + T::HANDLER, L::ZOOM_RUN);
    zoom_run(m)
}

/// Original: `$0903A4` (dentro de `GO_Zoom_09034e`).
pub fn zoom_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    for (k, f) in [L::X, L::Y].into_iter().enumerate() {
        let size = [L::ZOOM_SIZE, L::ZOOM_SIZE_Y][k];
        let v = m.read_u16(t + L::ORIGIN[k]).wrapping_sub(m.read_u16(t + size));
        m.write_u16(t + f, v);
    }
    anim::step(m, t);
    if (m.read_u16(t + L::TIMER) as i16) <= L::ZOOM_SHRINK_AT {
        let (drop_at, y, size_y) = L::ZOOM_DROP;
        if (m.read_u16(t + L::ZOOM_SIZE) as i16) < drop_at && m.read_u8(t + L::STATE[1]) == 0 {
            m.write_u8(t + L::STATE[1], 0xFF);
            m.write_u16(t + L::ORIGIN[1], y);
            m.write_u16(t + L::ZOOM_SIZE_Y, size_y);
        }
        if shrink(m, t, L::ZOOM_SIZE, 1) {
            m.write_u16(t + L::X, L::ZOOM_REST.0);
        }
        if shrink(m, t, L::ZOOM_SIZE_Y, 2) {
            m.write_u16(t + L::Y, L::ZOOM_REST.1);
        }
        if m.read_u8(t + L::STATE[1]) != 0 {
            let n = m.read_u16(t + L::COUNT).wrapping_add(1);
            m.write_u16(t + L::COUNT, n);
            if (n as i16) >= L::ZOOM_SETTLE_FRAMES {
                m.write_u32(t + T::HANDLER, L::ZOOM_PATH);
            }
        }
    }
    dec_timer(m, t);
    Flow::Return
}

fn dec_timer(m: &mut impl Memory, t: u32) {
    let n = m.read_u16(t + L::TIMER).wrapping_sub(1);
    m.write_u16(t + L::TIMER, n);
}

/// Rebota siguiendo una tabla y avisa a la madre. Original:
/// `GO_Zoom_Path_090452`.
pub fn zoom_path(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::FX_SPRITE.0);
    m.write_u16(t + L::X, L::ZOOM_REST.0);
    m.write_u16(t + L::Y, L::ZOOM_REST.1);
    m.write_u16(t + L::COUNT, 0);
    m.write_u32(t + T::HANDLER, L::ZOOM_PATH_RUN);
    zoom_path_run(m)
}

/// Original: `$090474` (dentro de `GO_Zoom_Path_090452`).
pub fn zoom_path_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let n = m.read_u16(t + L::COUNT);
    if (n as i16) < L::ZOOM_PATH_STEPS {
        let e = L::ZOOM_PATH_TABLE.wrapping_add(i32::from(n.wrapping_mul(4) as i16) as u32);
        for (k, f) in [L::X, L::Y].into_iter().enumerate() {
            let v = m.read_u16(t + f).wrapping_add(m.read_u16(e + 2 * k as u32));
            m.write_u16(t + f, v);
        }
    }
    anim::step(m, t);
    let n = m.read_u16(t + L::COUNT).wrapping_add(1);
    m.write_u16(t + L::COUNT, n);
    if (n as i16) >= L::ZOOM_PATH_END {
        let mother = m.read_u32(t + T::OWNER);
        m.write_u8(mother + L::STATE[0], 1);
        m.write_u32(t + T::HANDLER, L::ZOOM_WAIT);
    }
    dec_timer(m, t);
    Flow::Return
}

/// Espera a que acabe el temporizador. Original: `GO_Zoom_Wait_0904c6`,
/// `GO_Banner_Tick_0906ca`.
pub fn wait_timer(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    animate_until_timer(m, t)
}

/// Temblor del golpe bajo, con un destello. Original: `GO_Shake_0904e4`.
pub fn shake(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let (gfx, attr, _) = L::FX_SPRITE;
    fx_sprite(m, t, gfx, attr, L::SHAKE_SCRIPT);
    m.write_u16(t + L::X, L::SHAKE_POS.0);
    m.write_u16(t + L::Y, L::SHAKE_POS.1);
    m.write_u16(t + L::ORIGIN[0], L::SHAKE_POS.0);
    m.write_u16(t + L::ORIGIN[1], L::SHAKE_POS.1);
    m.write_u16(t + L::COUNT, 0);
    child(m, t, L::FLASH, true);
    anim::start(m, t, L::SHAKE_SCRIPT);
    m.write_u32(t + T::HANDLER, L::SHAKE_RUN);
    shake_run(m)
}

/// Original: `$090544`, `GO_Shake_Step_09055a`.
pub fn shake_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let n = m.read_u16(t + L::TIMER).wrapping_sub(1);
    m.write_u16(t + L::TIMER, n);
    if n == 0 {
        tasks::kill_children(m, t);
        return tasks::finish(m);
    }
    let i = m.read_u16(t + L::COUNT);
    let e = L::SHAKE_TABLE.wrapping_add(i32::from(i.wrapping_mul(4) as i16) as u32);
    for (k, f) in [L::X, L::Y].into_iter().enumerate() {
        let v = m.read_u16(e + 2 * k as u32).wrapping_add(m.read_u16(t + L::ORIGIN[k]));
        m.write_u16(t + f, v);
    }
    m.write_u16(t + L::COUNT, i.wrapping_add(2) & 7);
    anim::step(m, t);
    Flow::Return
}

/// Destello del temblor. Original: `GO_Flash_090592`.
pub fn flash(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let (gfx, x, y, script) = L::FLASH_SPRITE;
    anim::alloc_entry(m, t, gfx);
    m.write_u16(t + L::X, x);
    m.write_u16(t + L::Y, y);
    anim::start(m, t, script);
    m.write_u32(t + T::HANDLER, L::FLASH_RUN);
    animate_only(m, t)
}

/// Original: `$0905BA`, `$090628`, `$090684`, `$0906E0` (sólo animan).
pub fn flash_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    animate_only(m, t)
}

/// Cartel `k` (A–D). Original: `GO_Banner_0905c2`, `GO_Banner_B_090630`,
/// `GO_Banner_C_09068c`, `GO_Banner_D_0906e8`.
pub fn banner(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    let (gfx, attr, script, run) = L::BANNERS[k];
    fx_sprite(m, t, gfx, attr, script);
    m.write_u16(t + L::X, L::BANNER_POS.0);
    m.write_u16(t + L::Y, L::BANNER_POS.1);
    anim::start(m, t, script);
    m.write_u32(t + T::HANDLER, run);
    match k {
        0 => banner_run(m),
        1 => banner_b_run(m),
        _ => wait_timer(m),
    }
}

/// Al acabar el temporizador suena el golpe, avisa a la madre y se da de
/// baja; si no, anima. Original: `$090600`.
pub fn banner_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let n = m.read_u16(t + L::TIMER).wrapping_sub(1);
    m.write_u16(t + L::TIMER, n);
    if n != 0 {
        return animate_only(m, t);
    }
    sound::play(m, L::SOUND_HIT);
    let mother = m.read_u32(t + T::OWNER);
    m.write_u8(mother + L::STATE[1], 0);
    tasks::kill_children(m, t);
    tasks::finish(m)
}

/// Hasta que la madre pase al estado 3. Original: `$09066E`.
pub fn banner_b_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if parent_state_is(m, t, 0, 3) {
        tasks::kill_children(m, t);
        return tasks::finish(m);
    }
    animate_only(m, t)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::TestRam;

    #[test]
    fn integrate_carries_fraction() {
        let mut m = TestRam::default();
        let t = 0x10_0500;
        m.write_u16(t + L::Y, 0x1C0);
        m.write_u16(t + L::VEL_Y, 0xFF80);
        m.write_u8(t + L::Y_FRAC, 0x80);
        integrate_y(&mut m, t);
        // 0x80 + 0x80 = 0x100: lleva 1; parte entera -1 → la Y no cambia.
        assert_eq!(m.read_u16(t + L::Y), 0x1C0);
        assert_eq!(m.read_u8(t + L::Y_FRAC), 0);
        integrate_y(&mut m, t);
        assert_eq!(m.read_u16(t + L::Y), 0x1BF);
        m.write_u16(t + L::Y, 0x140);
        integrate_y(&mut m, t);
        assert_eq!(m.read_u16(t + L::Y), 0x140);
    }
}
