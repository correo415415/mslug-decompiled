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

use super::{hud, inc_u16, palette, pads, rng, sound, tasks, video};
use crate::layout::{bios, bios_ram, camera, pads as P, routine, script as S, task as T};
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

// --- Escenas ------------------------------------------------------------
//
// Cada escena tiene un paso de preparación (borra la capa fija, crea la
// tarea de la escena) que sigue en el mismo frame con la entrada siguiente,
// y un paso de espera que se repite cada frame (`Flow::Return`) hasta que la
// tarea de la escena termina (`SCENE_BUSY` a 0).

/// Crea la tarea de la escena (hija de la del guion).
fn spawn_scene_task(m: &mut impl Machine, handler: u32) {
    let me = m.task_register();
    tasks::spawn(m, handler, me);
}

/// Pide una música o efecto.
fn play(m: &mut impl Machine, code: u16) {
    sound::play(m, code);
}

/// Capa fija y estado de paleta limpios para una escena nueva.
fn clear_screen(m: &mut impl Machine) {
    m.call(bios::FIX_CLEAR);
    video::clear_palette_state(m);
}

/// Final común de la preparación: teclas, (¿empezó la partida?), marco y
/// escena en curso; sigue con la entrada siguiente.
fn scene_started(m: &mut impl Machine, check_start: bool) -> Flow {
    latch_keys(m);
    if check_start {
        check_game_started(m);
    }
    hud::clear_border(m);
    m.write_u8(S::SCENE_BUSY, 0xFF);
    next(m)
}

/// La escena terminó: da de baja sus tareas y pasa a la siguiente.
fn scene_done(m: &mut impl Machine) -> Flow {
    let me = m.task_register();
    tasks::kill_children(m, me);
    next(m)
}

fn scene_running(m: &mut impl Machine) -> bool {
    m.read_u8(S::SCENE_BUSY) != 0
}

/// Logo del arranque en caliente. Original: `AttractHandler_00109C`.
pub fn scene_logo(m: &mut impl Machine) -> Flow {
    pads::start_demo(m, crate::layout::pads::NO_DEMO);
    sound::init_queue(m);
    clear_screen(m);
    spawn_scene_task(m, S::LOGO_TASK);
    scene_started(m, true)
}

/// Espera del logo: sólo acaba cuando empieza la partida.
/// Original: `SchedTail_JsrDB8_0010E8`.
pub fn scene_logo_wait(m: &mut impl Machine) -> Flow {
    hud::draw(m);
    check_game_started(m);
    Flow::Return
}

/// Pantalla de continuar. Original: `AttractHandler_2Task_0010F2`.
pub fn scene_continue(m: &mut impl Machine) -> Flow {
    sound::init_queue(m);
    clear_screen(m);
    spawn_scene_task(m, S::CONTINUE_TASK);
    spawn_scene_task(m, S::CONTINUE_OPTIONS_TASK);
    play(m, S::CONTINUE_MUSIC);
    scene_started(m, false)
}

/// Espera de continuar; al acabar puede pedir volver al título.
/// Original: `AttractPhase2_Gate106ED5_001148`.
pub fn scene_continue_wait(m: &mut impl Machine) -> Flow {
    hud::draw(m);
    if scene_running(m) {
        return Flow::Return;
    }
    let me = m.task_register();
    tasks::kill_children(m, me);
    if m.read_u8(S::BACK_TO_TITLE) != 0 {
        m.write_u32(me + T::HANDLER, routine::SCRIPT_TITLE);
        return Flow::Return;
    }
    next(m)
}

/// Récords (con semilla aleatoria nueva). Original: `AttractHandler_Frame_001172`.
pub fn scene_hiscores(m: &mut impl Machine) -> Flow {
    inc_u16(m, rng_counter());
    let seed = m.read_u16(rng_counter());
    rng::seed(m, seed);
    m.call(bios::FIX_CLEAR);
    hud::clear_border(m);
    video::clear_palette_state(m);
    spawn_scene_task(m, S::HISCORE_TASK);
    scene_started(m, true)
}

/// Espera común de las escenas de la demo (se puede empezar a jugar).
/// Original: `AttractPhase2_Multiway_0011CA`, `BootScr_WaitHudThenLoop_001332`.
pub fn scene_wait(m: &mut impl Machine) -> Flow {
    hud::draw(m);
    if !scene_running(m) {
        return scene_done(m);
    }
    check_game_started(m);
    Flow::Return
}

/// Tabla de récords con sus entradas de sprite. Original: `AttractHandler_Loader_0011EA`.
pub fn scene_hiscore_table(m: &mut impl Machine) -> Flow {
    clear_screen(m);
    video::load_high_entries(m, crate::layout::sprites::HIGH_LIST);
    spawn_scene_task(m, S::HISCORE_TABLE_TASK);
    scene_started(m, false)
}

/// El jugador `player` acaba de pulsar A, B, C o D.
/// Original: `Sub_0005D09A` / `Sub_0005D0AC`.
pub fn button_pressed(m: &mut impl Memory, player: usize) -> bool {
    m.read_u8(crate::layout::input::PAD_LIVE[player] + S::PAD_PRESSED) & S::BUTTONS != 0
}

/// Espera de la tabla de récords: acaba sola o con un botón.
/// Original: `AttractPhase2_Probes5D0_00122E`.
pub fn scene_hiscore_table_wait(m: &mut impl Machine) -> Flow {
    hud::draw(m);
    if !scene_running(m) || button_pressed(m, 0) || button_pressed(m, 1) {
        return scene_done(m);
    }
    Flow::Return
}

/// Fin de partida. Original: `BootScr_GameOver_0012f4`.
pub fn scene_game_over(m: &mut impl Machine) -> Flow {
    clear_screen(m);
    spawn_scene_task(m, S::GAME_OVER_TASK);
    scene_started(m, true)
}

/// Principio de la partida: limpia la pantalla y, si el ajuste `PLAY
/// MANUAL` está activo, crea la tarea del manual de juego (la escena espera
/// a que acabe). Original: `BootScr_Mission_Start_001368`.
pub fn scene_mission_start(m: &mut impl Machine) -> Flow {
    clear_screen(m);
    sound::init_queue(m);
    m.write_u8(S::KEY_LATCH[0], S::MISSION_KEY_LATCH);
    m.write_u8(S::KEY_LATCH[1], 0xFF);
    m.write_u8(S::SCENE_BUSY, 0);
    if m.read_u8(S::PLAY_MANUAL) != 0 {
        let t = S::MANUAL_TASK;
        m.write_u32(t + T::HANDLER, S::MANUAL_HANDLER);
        tasks::reset_in_place(m, t);
        m.write_u16(S::RANK_TIMER, 1);
        m.write_u8(S::SCENE_BUSY, 0xFF);
    }
    next(m)
}

/// Espera del manual de juego: sigue cuando acaba, o cuando un jugador que
/// ya juega pulsa un botón (con la paleta lista).
/// Original: `BootScr_Mission_WaitStart_0013c6`.
pub fn scene_mission_wait(m: &mut impl Machine) -> Flow {
    hud::draw(m);
    if scene_running(m) {
        if !palette::bank_clear_done(m) {
            return Flow::Return;
        }
        let skip = (0..2).any(|p| {
            m.read_u8(P::BIOS_PLAYER_STATE[p]) == P::BIOS_PLAYER_PLAYING && button_pressed(m, p)
        });
        if !skip {
            return Flow::Return;
        }
    }
    stop_scene(m);
    next(m)
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
