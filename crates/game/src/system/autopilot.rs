//! Piloto automático de los tramos guiados de la misión: una tarea que,
//! según el tramo pedido (`LEVEL`, 1–4), arranca un guion que escribe
//! entradas sintéticas en los mandos de los jugadores (andar, saltar).
//! Mientras dura, los jugadores no reciben daño (`+$45` = 2).
//!
//! Original: `AutoDemo_Tmpl_Driver_079298`, `AutoDemo_StartLevelScript_07962c`
//! (`asm/props/fx_explosions_platforms_crew_076000.s`).

use super::tasks;
use crate::layout::{autopilot as L, input as I, task as T};
use crate::mem::{Flow, Machine, Memory};

/// Suelta los mandos sintéticos (lo mantenido pasa a «antes» y todo a 0).
pub fn release_pads(m: &mut impl Memory) {
    for pad in I::PAD_SCRIPTED {
        let held = m.read_u8(pad + L::PAD_NOW);
        m.write_u8(pad + L::PAD_BEFORE, held);
        for off in [L::PAD_BEFORE, L::PAD_NOW, L::PAD_PRESSED, L::PAD_EXTRA] {
            m.write_u8(pad + off, 0);
        }
    }
}

/// Si se ha pedido un tramo distinto del que corre, arranca su guion
/// (hija de la tarea actual). Devuelve si ha arrancado uno.
/// Original: `AutoDemo_StartLevelScript_07962c`.
pub fn start_level_script(m: &mut impl Memory, t: u32) -> bool {
    let level = m.read_u8(L::LEVEL);
    if level == 0 {
        m.write_u8(I::SCRIPTED_INPUT_ACTIVE, 0);
        return false;
    }
    if level == m.read_u8(I::SCRIPTED_INPUT_ACTIVE) {
        return false;
    }
    m.write_u8(I::SCRIPTED_INPUT_ACTIVE, level);
    let live = m.read_u8(I::PAD_LIVE[0]);
    for pad in I::PAD_SCRIPTED {
        m.write_u8(pad, live);
    }
    release_pads(m);
    m.write_u8(I::INPUT_HOLDOFF_TIMER, L::HOLDOFF);
    let script = m.read_u32(L::SCRIPTS + u32::from(level) * 4);
    if script == 0xFFFF_FFFF {
        return false;
    }
    tasks::spawn(m, script, t);
    true
}

/// Alta de la tarea: suelta los mandos y no hay tramo en curso.
/// Original: `AutoDemo_Tmpl_Driver_079298`.
pub fn start(m: &mut impl Machine) -> Flow {
    release_pads(m);
    m.write_u8(L::LEVEL, 0);
    m.write_u8(I::SCRIPTED_INPUT_ACTIVE, 0);
    let t = m.task_register();
    m.write_u32(t + T::HANDLER, L::RUN);
    run(m)
}

/// Cada frame: arranca el guion pedido y, durante el tramo, hace a los
/// jugadores invulnerables. Original: `$0792F0`.
pub fn run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    start_level_script(m, t);
    if m.read_u8(L::LEVEL) != 0 {
        for p in 0..2 {
            if let Some(e) = tasks::player_task_if_alive(m, p) {
                m.write_u8(e + L::SHIELD, L::SHIELD_ON);
            }
        }
    }
    Flow::Return
}
