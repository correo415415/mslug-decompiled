//! Tareas del modo atracción tras la partida: el destello blanco (funde a
//! cian y luego a blanco y avisa a la escena) y la recarga de la escena C
//! con sus dos fundidos.
//!
//! Original: `Fade_WhiteFlash_Task_046608`, `Fade_WhiteFlash_Done_046664`,
//! `SceneC_Load_Task_046682`, `SceneC_Load_Spawn2_0466b4`,
//! `SceneC_Load_Finish_0466da`
//! (`asm/enemies/vehicles/enemy46_drops_banners_046260.s`).

use super::{palette, scene, scroll, sound, tasks};
use crate::layout::{attract as L, task as T};
use crate::mem::{Flow, Machine, Memory};

/// Descuenta el temporizador `+$70`; `true` cuando llega a 0 (o menos).
fn countdown(m: &mut impl Memory, t: u32) -> bool {
    let n = m.read_u16(t + L::TIMER).wrapping_sub(1);
    m.write_u16(t + L::TIMER, n);
    (n as i16) <= 0
}

/// Empieza el destello: funde hacia cian y espera.
/// Original: `Fade_WhiteFlash_Task_046608`.
pub fn white_flash(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    palette::spawn_to_color(m, t, L::FLASH_COLORS[0], L::FLASH_FRAMES);
    m.write_u8(L::MISSION_FLAG_B1, 0xFF);
    m.write_u16(t + L::TIMER, L::FLASH_WAIT);
    m.write_u32(t + T::HANDLER, L::WHITE_FLASH_LOOP);
    white_flash_loop(m)
}

/// Al acabar la espera funde hacia blanco. Original:
/// `Fade_WhiteFlash_Task_046608+$2A`.
pub fn white_flash_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if !countdown(m, t) {
        return Flow::Return;
    }
    palette::spawn_to_color(m, t, L::FLASH_COLORS[1], L::FLASH_FRAMES);
    m.write_u16(t + L::TIMER, L::FLASH_DONE_WAIT);
    m.write_u32(t + T::HANDLER, L::WHITE_FLASH_DONE);
    Flow::Return
}

/// Tras el blanco avisa a quien la creó (`+$21` = 0) y termina.
/// Original: `Fade_WhiteFlash_Done_046664`.
pub fn white_flash_done(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if !countdown(m, t) {
        return Flow::Return;
    }
    let owner = m.read_u32(t + T::OWNER);
    m.write_u8(owner + L::FLASH_PENDING, 0);
    tasks::finish(m)
}

/// Recarga la escena C (sonido reiniciado, primer frame del scroll) y
/// funde desde blanco. Original: `SceneC_Load_Task_046682`.
pub fn scene_c(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    sound::reset(m);
    scene::load(m, L::SCENE_C);
    scroll::frame(m, (0, 0));
    let (kind, param) = L::SCENE_C_FADES[0];
    palette::spawn_fade(m, kind, t, param);
    m.write_u16(t + L::TIMER, L::SCENE_C_WAITS[0]);
    m.write_u32(t + T::HANDLER, L::SCENE_C_FADE);
    Flow::Return
}

/// Al cabo de un rato, el segundo fundido. Original:
/// `SceneC_Load_Spawn2_0466b4`.
pub fn scene_c_fade(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if !countdown(m, t) {
        return Flow::Return;
    }
    let (kind, param) = L::SCENE_C_FADES[1];
    palette::spawn_fade(m, kind, t, param);
    m.write_u16(t + L::TIMER, L::SCENE_C_WAITS[1]);
    m.write_u32(t + T::HANDLER, L::SCENE_C_FINISH);
    Flow::Return
}

/// Da la escena por terminada y acaba. Original:
/// `SceneC_Load_Finish_0466da`.
pub fn scene_c_finish(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if !countdown(m, t) {
        return Flow::Return;
    }
    m.write_u8(crate::layout::script::SCENE_BUSY, 0);
    tasks::finish(m)
}
