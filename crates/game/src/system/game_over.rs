//! Escena de Game Over / Continue. Sale al acabar la partida y, en el
//! attract, cuando no hay créditos. La tarea principal cae desde arriba
//! con su velocidad. Al pasar por dos alturas «golpea» (sonido y efectos);
//! al llegar al suelo asienta la escena y espera a que haya créditos;
//! entonces lanza el texto de CONTINUE y su recorrido.
//!
//! Original: `GameOver_Boot_08f91a` … `GameOver_ContinueEnd_08feb6`.

use super::{anim, hud, palette, scene, scroll, sound, tasks, video};
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
