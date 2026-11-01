//! Entrada en la tabla de récords al acabar la partida: empaqueta la
//! puntuación del jugador, busca su puesto, desplaza las de abajo, la
//! escribe con el nombre en blanco y lanza la tarea de introducir el
//! nombre.
//!
//! Original: `HiScore_DrawHeaderA_097a48`, `HiScore_DrawHeaderB_097a54`,
//! `HiScore_TryEnter_P1_097a60`, `HiScore_TryEnter_P2_097a72`,
//! `HiScore_InsertScore_097aaa`, `HiScore_RankOfPlayer_098176` y el
//! empaquetado `$051AA4` (`asm/frontend/hiscore_memcard_options_09773c.s`).
//!
//! También la pantalla de la tabla: logo que aparece, cabeceras y las diez
//! filas (puesto, puntuación y nombre), de una en una o todas a la vez.
//! Original: `HiScore_Tpl_Loader_0977ea`, `HiScore_Tpl_Common_097816`,
//! `HiScore_WaitLogo_097852` y sus estados, `HiScore_Logo_FadeIn_09794c`,
//! `HiScore_Logo_Static_097962`, `HiScore_DrawHeaders_0979f8`,
//! `HiScore_DrawRow_097b74` y `$097C02`.

use super::{anim, hud, rng, scene, scroll, tasks};
use crate::layout::{hiscore as L, task as T};
use crate::mem::{Flow, Machine, Memory};

/// Empaqueta la puntuación de 8 cifras (una por byte) de `score` en 4
/// bytes BCD, que deja también en `PACKED`. Original: `$051AA4` (con
/// `$051862`).
pub fn packed_score(m: &mut impl Memory, score: u32) -> u32 {
    let mut v = 0u32;
    for i in 0..4 {
        let hi = m.read_u8(score + 2 * i);
        let lo = m.read_u8(score + 2 * i + 1);
        let b = (hi << 4) | lo;
        m.write_u8(L::PACKED + i, b);
        v = (v << 8) | u32::from(b);
    }
    v
}

/// Dirección de la entrada `k` de la tabla.
fn entry(k: u32) -> u32 {
    L::TABLE + k * L::ENTRY_LEN
}

/// Puesto del jugador `player` (1 o 2) en la tabla: el de su primera
/// entrada o, si no tiene ninguna, uno más que el de la última. Devuelve
/// también la entrada consultada. Original: `HiScore_RankOfPlayer_098176`.
pub fn rank_of_player(m: &mut impl Memory, player: u8) -> (u8, u32) {
    for k in 0..L::ENTRIES {
        let e = entry(k);
        if m.read_u8(e + L::E_PLAYER) == player {
            return (m.read_u8(e + L::E_RANK), e);
        }
    }
    let e = entry(L::ENTRIES - 1);
    (m.read_u8(e + L::E_RANK).wrapping_add(1), e)
}

/// Mete la puntuación `score` del jugador `player` (1 o 2) en la tabla si
/// llega: las de debajo bajan un sitio (su puesto sube salvo empate con la
/// nueva) y la nueva queda con el nombre en blanco. Devuelve si ha
/// entrado. `t` es la tarea que llama: por cada entrada que ya es del
/// jugador, el original borra el primer byte de la tarea (no el de la
/// entrada). Original: `HiScore_InsertScore_097aaa`.
pub fn insert_score(m: &mut impl Memory, t: u32, score: u32, player: u8) -> bool {
    for k in 0..L::ENTRIES {
        if m.read_u8(entry(k) + L::E_PLAYER) == player {
            m.write_u8(t, 0);
        }
    }
    // Desde abajo: cuántas no superan a la nueva (comparación con signo).
    let mut below = 0;
    let mut pos = L::ENTRIES as i32 - 1;
    while pos >= 0 && (score as i32) >= (m.read_u32(entry(pos as u32) + L::E_SCORE) as i32) {
        below += 1;
        pos -= 1;
    }
    if below == 0 {
        return false;
    }
    if below > 1 {
        let mut src = L::ENTRIES as i32 - 2;
        while src != pos {
            let (d, s) = (entry(src as u32 + 1), entry(src as u32));
            let p = m.read_u8(s + L::E_PLAYER);
            m.write_u8(d + L::E_PLAYER, p);
            let mut rank = m.read_u8(s + L::E_RANK);
            let sc = m.read_u32(s + L::E_SCORE);
            if sc != score {
                rank = rank.wrapping_add(1);
            }
            m.write_u8(d + L::E_RANK, rank);
            m.write_u32(d + L::E_SCORE, sc);
            for f in L::E_NAME {
                let w = m.read_u16(s + f);
                m.write_u16(d + f, w);
            }
            src -= 1;
        }
    }
    let e = entry((pos + 1) as u32);
    m.write_u8(e + L::E_PLAYER, player);
    m.write_u32(e + L::E_SCORE, score);
    let (rank, _) = rank_of_player(m, player);
    m.write_u8(e + L::E_RANK, rank);
    for f in L::E_NAME {
        m.write_u16(e + f, L::NAME_BLANK);
    }
    true
}

/// Intenta meter en la tabla al jugador `p` (0 o 1) desde la tarea `t`:
/// si entra, lanza la tarea del nombre con la posición `vram` en la capa
/// fija y la devuelve; si no, borra `+$21` de `t`. Devuelve también la
/// puntuación empaquetada. Original: `HiScore_TryEnter_P1_097a60` /
/// `HiScore_TryEnter_P2_097a72`.
pub fn try_enter(m: &mut impl Memory, t: u32, p: usize, vram: u32) -> (u32, Option<u32>) {
    let score = packed_score(m, L::SCORES[p]);
    let player = p as u8 + 1;
    if !insert_score(m, t, score, player) {
        m.write_u8(t + L::PENDING, 0);
        return (score, None);
    }
    let o = tasks::spawn(m, L::NAME_ENTRY, t);
    m.write_u32(o + L::NE_VRAM, vram);
    m.write_u16(o + L::NE_PLAYER, u16::from(player));
    (score, Some(o))
}

/// Entrada del jugador `p` con la posición de su cabecera. Original:
/// `HiScore_DrawHeaderA_097a48` / `HiScore_DrawHeaderB_097a54`.
pub fn enter(m: &mut impl Memory, t: u32, p: usize) -> (u32, Option<u32>) {
    try_enter(m, t, p, L::HEADER_VRAM[p])
}

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

/// Suma uno a la cuenta `+$70`; `true` si ha llegado a `n`.
fn tick(m: &mut impl Memory, t: u32, n: i16) -> bool {
    let c = m.read_u16(t + L::TIMER).wrapping_add(1);
    m.write_u16(t + L::TIMER, c);
    (c as i16) >= n
}

/// Pantalla de la tabla de récords: lanza el logo y la espera final; el
/// original borra diez veces el mismo byte (el jugador de la primera
/// entrada). Original: `HiScore_Tpl_Loader_0977ea`.
pub fn table(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    tasks::spawn(m, L::LOGO_FADE_IN, t);
    m.write_u16(t + L::HOLD, L::HOLD_FRAMES);
    for _ in 0..L::ENTRIES {
        m.write_u8(L::TABLE + L::E_PLAYER, 0);
    }
    table_common(m)
}

/// Escena de fondo, título y espera al logo.
/// Original: `HiScore_Tpl_Common_097816`.
pub fn table_common(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scene::load(m, L::SCENE);
    scroll::frame(m, (0, 0));
    m.write_u8(t + L::LOGO_STATE, 0);
    m.write_u16(t + L::TIMER, 0);
    hud::draw_text(m, L::TITLE.0, L::TITLE.1, L::TITLE.2);
    set_handler(m, t, L::WAIT_LOGO);
    Flow::Return
}

/// Cuando el logo acaba, prepara las filas: de una en una (logo con
/// fundido) o todas a la vez. Original: `HiScore_WaitLogo_097852`.
pub fn wait_logo(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let state = m.read_u8(t + L::LOGO_STATE);
    if state == 0 {
        return Flow::Return;
    }
    m.write_u8(t + L::ROWS_PENDING, 0);
    m.write_u32(t + L::ROW_VRAM, L::FIRST_ROW_VRAM);
    m.write_u32(t + L::ROW_PTR, L::TABLE);
    set_handler(m, t, if state == L::ONE_BY_ONE { L::ROWS_DELAY } else { L::ROWS_ALL });
    Flow::Return
}

/// Espera un poco, dibuja las cabeceras y empieza con las filas.
/// Original: `HiScore_WaitLogo_097852+$3A`.
pub fn rows_delay(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if !tick(m, t, L::ROWS_START_DELAY) {
        return Flow::Return;
    }
    m.write_u16(t + L::TIMER, 0);
    draw_headers(m);
    set_handler(m, t, L::ROWS_STEP);
    Flow::Return
}

/// Una fila cada dos frames; tras la última, la espera final.
/// Original: `HiScore_WaitLogo_097852+$5A`.
pub fn rows_step(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if !tick(m, t, L::ROW_DELAY) {
        return Flow::Return;
    }
    let vram = m.read_u32(t + L::ROW_VRAM);
    let e = m.read_u32(t + L::ROW_PTR);
    draw_row(m, t, e, vram);
    let (e, mut vram) = (e + L::ENTRY_LEN, vram + L::ROW_VRAM_STEP);
    if e == L::TABLE_END {
        set_handler(m, t, L::HOLD_LOOP);
        // El original reutiliza el registro de la posición para la rutina
        // nueva y la guarda también como posición siguiente.
        vram = L::HOLD_LOOP;
    }
    m.write_u16(t + L::TIMER, 0);
    m.write_u32(t + L::ROW_PTR, e);
    m.write_u32(t + L::ROW_VRAM, vram);
    Flow::Return
}

/// Cabeceras y las diez filas de golpe. Original: `HiScore_WaitLogo_097852+$A8`.
pub fn rows_all(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    draw_headers(m);
    let vram = m.read_u32(t + L::ROW_VRAM);
    let e = m.read_u32(t + L::ROW_PTR);
    for k in 0..L::ENTRIES {
        draw_row(m, t, e + k * L::ENTRY_LEN, vram + k * L::ROW_VRAM_STEP);
    }
    set_handler(m, t, L::HOLD_LOOP);
    Flow::Return
}

/// Sin nombres por entrar, cuenta la espera y da la escena por acabada.
/// Original: `HiScore_WaitLogo_097852+$DC`.
pub fn hold(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if m.read_u8(t + L::ROWS_PENDING) != 0 {
        return Flow::Return;
    }
    let n = m.read_u16(t + L::HOLD).wrapping_sub(1);
    m.write_u16(t + L::HOLD, n);
    if n != 0 {
        return Flow::Return;
    }
    m.write_u8(crate::layout::script::SCENE_BUSY, 0);
    set_handler(m, t, L::DONE);
    Flow::Return
}

/// Logo de la tabla (`k` 0 = con fundido, 1 = ya visible): sprite en su
/// sitio y uno de sus dos guiones al azar.
/// Original: `HiScore_Logo_FadeIn_09794c` / `HiScore_Logo_Static_097962`.
pub fn logo(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    let (state, alpha, level) = L::LOGO_STARTS[k];
    m.write_u8(t + L::LOGO_STATE, state);
    m.write_u8(t + L::LOGO_ALPHA, alpha);
    m.write_u16(t + L::LOGO_LEVEL, level);
    anim::alloc_entry(m, t, L::LOGO_GFX);
    m.write_u16(t + L::X, L::LOGO_POS.0);
    m.write_u16(t + L::Y, L::LOGO_POS.1);
    m.write_u8(t + L::LOGO_DONE, 0);
    let (r, _) = rng::next(m);
    let script = m.read_u32(L::LOGO_SCRIPTS + u32::from(r & 1) * 4);
    if script != L::NO_SCRIPT {
        anim::start(m, t, script);
    }
    set_handler(m, t, L::LOGO_LOOP);
    logo_loop(m)
}

/// Sube la opacidad hasta el máximo; entonces pasa su estado a la tabla.
/// Original: `HiScore_Logo_Static_097962+$5A`.
pub fn logo_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if m.read_u8(t + L::LOGO_DONE) == 0 {
        let a = m.read_u8(t + L::LOGO_ALPHA).wrapping_add(L::LOGO_STEP);
        m.write_u8(t + L::LOGO_ALPHA, a);
        let v = m.read_u16(t + L::LOGO_LEVEL).wrapping_add(u16::from(L::LOGO_STEP));
        m.write_u16(t + L::LOGO_LEVEL, v);
        if v >= L::LOGO_FULL {
            m.write_u8(t + L::LOGO_ALPHA, 0xFF);
            m.write_u8(t + L::LOGO_DONE, 0xFF);
            let owner = m.read_u32(t + T::OWNER);
            let state = m.read_u8(t + L::LOGO_STATE);
            m.write_u8(owner + L::LOGO_STATE, state);
        }
    }
    anim::step(m, t);
    Flow::Return
}

/// Cabeceras de la tabla. Original: `HiScore_DrawHeaders_0979f8`.
pub fn draw_headers(m: &mut impl Memory) {
    for (at, list) in L::HEADERS {
        hud::draw_glyph_list(m, at, list, L::HEADER_ATTR, 2, 2);
    }
}

/// Fila de la entrada `e` en `vram`: si es de un jugador lanza el
/// parpadeo de su nombre (y la tabla `t` espera por él); puesto,
/// puntuación y nombre. Original: `HiScore_DrawRow_097b74`.
pub fn draw_row(m: &mut impl Machine, t: u32, e: u32, vram: u32) {
    let player = m.read_u8(e + L::E_PLAYER);
    if player != 0 {
        let o = tasks::spawn(m, L::NAME_BLINK, t);
        m.write_u16(o + L::NE_PLAYER, u16::from(player));
        let n = m.read_u8(t + L::ROWS_PENDING).wrapping_add(1);
        m.write_u8(t + L::ROWS_PENDING, n);
    }
    let rank = m.read_u8(e + L::E_RANK);
    draw_rank(m, vram as u16, rank);
    let at = vram + L::ROW_SCORE_DX;
    let score = m.read_u32(e + L::E_SCORE);
    hud::draw_number(m, at as u16, score, L::SCORE_DIGITS, L::SCORE_ATTR);
    let mut at = (at + L::ROW_NAME_DX) as u16;
    for f in L::E_NAME {
        let g = m.read_u16(e + f) | L::NAME_ATTR;
        hud::fill_tiles(m, at, g, 2, 2);
        at = at.wrapping_add(L::NAME_STEP);
    }
}

/// Glifo y texto del puesto `rank` (1–11) en `at`. Original: `$097C02`.
pub fn draw_rank(m: &mut impl Memory, at: u16, rank: u8) {
    let k = u16::from(rank).wrapping_sub(1);
    let k = if (k as i16) < 0 || k > L::RANK_MAX { 0 } else { u32::from(k) };
    let g = m.read_u16(L::RANK_GLYPHS + k * 2) | L::RANK_ATTR;
    hud::fill_tiles(m, at, g, 2, 2);
    let text = m.read_u32(L::RANK_TEXTS + k * 4);
    hud::draw_text(m, at.wrapping_add(L::RANK_TEXT_DX), text, L::RANK_TEXT_ATTR);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::TestRam;

    fn fill(m: &mut TestRam) {
        for k in 0..L::ENTRIES {
            let e = entry(k);
            m.write_u8(e + L::E_PLAYER, 0);
            m.write_u8(e + L::E_RANK, k as u8 + 1);
            m.write_u32(e + L::E_SCORE, 0x1000 * (10 - k));
        }
    }

    #[test]
    fn inserts_in_order_and_shifts() {
        let mut m = TestRam::default();
        fill(&mut m);
        assert!(insert_score(&mut m, 0x10_0800, 0x5500, 1));
        assert_eq!(m.read_u32(entry(5) + L::E_SCORE), 0x5500);
        assert_eq!(m.read_u8(entry(5) + L::E_PLAYER), 1);
        assert_eq!(m.read_u8(entry(5) + L::E_RANK), 6);
        assert_eq!(m.read_u32(entry(6) + L::E_SCORE), 0x5000);
        assert_eq!(m.read_u8(entry(6) + L::E_RANK), 7);
        assert_eq!(m.read_u32(entry(9) + L::E_SCORE), 0x2000);
        assert_eq!(m.read_u16(entry(5) + L::E_NAME[0]), L::NAME_BLANK);
    }

    #[test]
    fn last_place_replaces_without_shift() {
        let mut m = TestRam::default();
        fill(&mut m);
        assert!(insert_score(&mut m, 0x10_0800, 0x1800, 2));
        assert_eq!(m.read_u32(entry(9) + L::E_SCORE), 0x1800);
        assert_eq!(m.read_u32(entry(8) + L::E_SCORE), 0x2000);
    }

    #[test]
    fn too_low_does_not_enter() {
        let mut m = TestRam::default();
        fill(&mut m);
        assert!(!insert_score(&mut m, 0x10_0800, 0x0500, 1));
    }
}
