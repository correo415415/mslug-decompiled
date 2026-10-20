//! Guion de arranque: la tarea que va pasando de escena en escena (logo,
//! demo, título, partida).
//!
//! Según lo que pida la BIOS se elige una lista de rutinas de la ROM
//! (`layout::script`). La tarea del guion llama a la rutina de la entrada
//! actual cada frame; cuando la escena acaba, la rutina salta a [`next`],
//! que avanza a la siguiente entrada y la empieza en el mismo frame.
//! Original: `SchedulerBootstrap_Boot_000E8E`, `SchedulerDispatch_LoopB_000FE0`,
//! `SchedTail_JsrCD4_001020`, `SchedTail_JsrD3C_001026`,
//! `TaskList_ChangeAndRunEight_001CD4`, `Task_ResetAllSlots_001d3c`.

use super::{inc_u16, pads, rng, tasks, video};
use crate::layout::{bios, bios_ram, camera, routine, script as S, task as T};
use crate::mem::{Flow, Machine, Memory};

/// Límites de cámara por defecto. Original: `Camera_ResetCenter_05CACE`.
pub fn reset_camera(m: &mut impl Memory) {
    m.write_u16(camera::LIMIT_X, camera::DEFAULT_LIMITS.0);
    m.write_u16(camera::LIMIT_Y, camera::DEFAULT_LIMITS.1);
}

/// Rutina inicial de la tarea del guion: elige la lista según el modo de
/// la BIOS y empieza. Original: `SchedulerBootstrap_Boot_000E8E`.
pub fn boot(m: &mut impl Machine) -> Flow {
    reset_camera(m);
    m.write_u16(camera::LIMIT_X, camera::BOOT_LIMITS.0);
    m.write_u16(camera::LIMIT_Y, camera::BOOT_LIMITS.1);
    rng::seed(m, 0);
    let t = m.task_register();
    for off in S::TASK_CLEAR {
        m.write_u8(t + off, 0);
    }
    if m.read_u8(bios_ram::USER_MODE) == bios_ram::USER_MODE_GAME {
        return start_game(m);
    }
    for p in crate::layout::pads::BIOS_PLAYER_STATE_ALL {
        m.write_u8(p, 0);
    }
    for a in S::KEY_RESET {
        m.write_u8(a, 0);
    }
    if m.read_u8(bios_ram::USER_REQUEST) == bios_ram::USER_REQUEST_TITLE {
        m.write_u8(bios_ram::HARD_START, 1);
        start_list(m, S::HARD_START)
    } else {
        start_list(m, S::DEMO)
    }
}

/// La BIOS ya tiene partida: sin demo, y la lista de consola o recreativa.
/// Original: `TaskHandler_000ef0`.
pub fn start_game(m: &mut impl Machine) -> Flow {
    pads::start_demo(m, crate::layout::pads::NO_DEMO);
    let list = if m.read_u8(bios_ram::MVS_FLAG) == 0 { S::GAME_AES } else { S::GAME_MVS };
    begin(m, list)
}

/// Vuelta al título. Original: `TaskHandler_000f1a`.
pub fn start_title(m: &mut impl Machine) -> Flow {
    start_list(m, S::TITLE)
}

/// Se pulsó inicio en la demo: para las escenas y va a la lista de
/// partida. Original: `PcThunkTarget_000F76`.
pub fn start_pressed(m: &mut impl Machine) -> Flow {
    latch_keys(m);
    let t = m.task_register();
    m.write_u32(t + T::SCRIPT_CURSOR, S::START_PRESSED);
    stop_scene(m);
    enter(m)
}

fn latch_keys(m: &mut impl Memory) {
    for a in S::KEY_LATCH {
        m.write_u8(a, 0xFF);
    }
}

fn start_list(m: &mut impl Machine, list: u32) -> Flow {
    latch_keys(m);
    begin(m, list)
}

fn begin(m: &mut impl Machine, list: u32) -> Flow {
    let t = m.task_register();
    m.write_u32(t + T::SCRIPT_CURSOR, list);
    enter(m)
}

/// Prepara una escena nueva (semilla aleatoria, sprites, paletas fijas,
/// capa fija) y deja el despacho como rutina de la tarea. Original: `$000F98`.
fn enter(m: &mut impl Machine) -> Flow {
    inc_u16(m, rng_counter());
    let seed = m.read_u16(rng_counter());
    rng::seed(m, seed);
    video::reset_sprites(m);
    video::clear_palette_state(m);
    video::load_fixed_entries(m);
    m.call(bios::FIX_CLEAR);
    let t = m.task_register();
    m.write_u32(t + T::HANDLER, routine::SCRIPT_DISPATCH);
    let mut r = m.regs();
    r.a[1] = routine::SCRIPT_DISPATCH;
    m.set_regs(r);
    dispatch(m)
}

fn rng_counter() -> u32 {
    crate::layout::rng::SEED_COUNTER
}

/// Entrada del cursor; `END` salta a otra lista.
fn resolve(m: &mut impl Memory, at: u32) -> u32 {
    if m.read_u32(at) == S::END {
        m.read_u32(at + 4)
    } else {
        at
    }
}

/// Ejecuta la escena actual (la rutina de la tarea en cada frame).
/// Original: `$000FC6`.
pub fn dispatch(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let at = m.read_u32(t + T::SCRIPT_CURSOR);
    let at2 = resolve(m, at);
    if at2 != at {
        m.write_u32(t + T::SCRIPT_CURSOR, at2);
    }
    let handler = m.read_u32(at2);
    let mut r = m.regs();
    r.a[0] = handler;
    m.set_regs(r);
    Flow::Goto(handler)
}

/// La escena actual acabó: pasa a la siguiente.
/// Original: `SchedulerDispatch_LoopB_000FE0`.
pub fn next(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let at = m.read_u32(t + T::SCRIPT_CURSOR) + 4;
    let at = resolve(m, at);
    m.write_u32(t + T::SCRIPT_CURSOR, at);
    dispatch(m)
}

/// Para todas las tareas de la escena (y las hijas de la del guion).
/// Original: `TaskList_ChangeAndRunEight_001CD4`.
pub fn stop_scene(m: &mut impl Machine) {
    for t in S::SCENE_TASKS {
        tasks::stop(m, t);
    }
    let me = m.task_register();
    tasks::kill_children(m, me);
}

/// Como [`stop_scene`], pero las dos últimas tareas sólo pierden sus
/// hijas. Original: `Task_ResetAllSlots_001d3c`.
pub fn stop_scene_keep(m: &mut impl Machine) {
    let (stop, keep) = S::SCENE_TASKS.split_at(6);
    for &t in stop {
        tasks::stop(m, t);
    }
    for &t in keep {
        tasks::kill_children_of(m, t);
    }
    let me = m.task_register();
    tasks::kill_children(m, me);
}

/// Entradas de una línea del guion: paran la escena y siguen.
/// Original: `SchedTail_JsrCD4_001020` / `SchedTail_JsrD3C_001026`.
pub fn step_stop_scene(m: &mut impl Machine) -> Flow {
    stop_scene(m);
    next(m)
}
pub fn step_stop_scene_keep(m: &mut impl Machine) -> Flow {
    stop_scene_keep(m);
    next(m)
}

/// Sin pausa / con pausa en esta escena.
/// Original: `BootScr_Clr13d2a_001354` / `BootScr_SetFF13d20_00135e`.
pub fn step_disable_pause(m: &mut impl Machine) -> Flow {
    pads::set_pause_enabled(m, false);
    next(m)
}
pub fn step_enable_pause(m: &mut impl Machine) -> Flow {
    pads::set_pause_enabled(m, true);
    next(m)
}

/// Fin del guion de demo: vuelve a la BIOS en modo título.
/// Original: `Attract_SoftReset_10FDAF_001838`.
pub fn step_back_to_bios(m: &mut impl Machine) -> Flow {
    m.write_u8(bios_ram::USER_MODE, bios_ram::USER_MODE_ATTRACT);
    Flow::Goto(routine::RETURN_TO_BIOS)
}

/// Si la BIOS ya está en partida (alguien metió crédito y pulsó inicio),
/// para la escena y pasa a la lista de partida el próximo frame.
/// Original: `Attract_StartIfP2Flag_001af8`.
pub fn check_game_started(m: &mut impl Machine) {
    if m.read_u8(bios_ram::USER_MODE) != bios_ram::USER_MODE_GAME {
        return;
    }
    stop_scene(m);
    pads::start_demo(m, crate::layout::pads::NO_DEMO);
    let t = m.task_register();
    m.write_u32(t + T::HANDLER, routine::SCRIPT_GAME);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::TestMachine;

    #[test]
    fn next_follows_end_marker() {
        let mut m = TestMachine::default();
        m.set_task_register(0x10_01C0);
        // Lista: [h1, END, lista2], lista2: [h2].
        m.write_u32(0x2000, 0x1111);
        m.write_u32(0x2004, S::END);
        m.write_u32(0x2008, 0x3000);
        m.write_u32(0x3000, 0x2222);
        m.write_u32(0x10_01C0 + T::SCRIPT_CURSOR, 0x2000);
        assert_eq!(dispatch(&mut m), Flow::Goto(0x1111));
        assert_eq!(next(&mut m), Flow::Goto(0x2222));
        assert_eq!(m.read_u32(0x10_01C0 + T::SCRIPT_CURSOR), 0x3000);
    }
}
