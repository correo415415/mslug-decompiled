//! Tarea del HUD de cada jugador (una por jugador; `PLAYER` = 0 o 1): los
//! mensajes INSERT COIN / PUSH START / PLEASE WAIT / GAME OVER, el enlace con
//! la tarea del soldado al entrar en juego, la vida, las bombas y la
//! puntuación mientras juega, y el CONTINUE con su cuenta atrás.
//!
//! Original: `HUD_Task_StartP1_02575c` … `HUD_LoadPerPlayerPtr_025eac`,
//! `HUD_ResetFields_025ed6`, `HUD_DrawBombGauge_P1/P2`, `$026752` y los
//! ayudantes `$001E28`, `$001E4C`, `$001E56`.

use super::{rank, tasks};
use crate::layout::{bios, bios_ram, mission, player_hud as L, task as T};
use crate::mem::{Flow, Machine, Memory};

fn player(m: &mut impl Machine) -> u32 {
    let t = m.task_register();
    u32::from(m.read_u16(t + L::PLAYER))
}

fn set_handler(m: &mut impl Machine, h: u32) {
    let t = m.task_register();
    m.write_u32(t + T::HANDLER, h);
}

/// Se puede atender al jugador: la tarea del guion no está esperando y no
/// hay pausa de guion. Original: `$001E28` (C = sí).
pub fn input_free(m: &mut impl Memory) -> bool {
    m.read_u8(L::SCRIPT_TASK + T::TIMER_B) == 0 && m.read_u8(L::SCRIPTED_PAUSE) == 0
}

/// Pide que la escena no avance este frame. Original: `$001E4C`.
pub fn hold_scene(m: &mut impl Memory) {
    m.write_u8(L::HOLD_SCENE, 0xFF);
}

/// Retardo (en vblanks) antes del siguiente frame. Original: `$001E56`.
pub fn set_vblank_delay(m: &mut impl Memory, n: u8) {
    m.write_u8(crate::layout::system::VBLANK_DELAY, n);
}

/// Estado de inicio del jugador según la BIOS (`BIOS_PLAYER_STATE`).
/// Original: `HUD_SetStartMask_025e74` (+$10 lee).
fn start_state(m: &mut impl Machine) -> u8 {
    let p = player(m);
    m.read_u8(L::START_STATE + p)
}

fn set_start_state(m: &mut impl Machine, v: u8) {
    let p = player(m);
    m.write_u8(L::START_STATE + p, v);
}

/// Llama a la rutina del jugador en la tabla `table` (una por jugador).
/// Original: `HUD_CallPerPlayer_IfMode2_025e92+$0C`.
fn call_for_player(m: &mut impl Machine, table: u32) {
    let p = player(m);
    let f = m.read_u32(table + p * 4);
    m.call(f);
}

/// Como [`call_for_player`], sólo durante la partida.
/// Original: `HUD_CallPerPlayer_IfMode2_025e92`.
fn draw(m: &mut impl Machine, table: u32) {
    if m.read_u8(bios_ram::USER_MODE) == bios_ram::USER_MODE_GAME {
        call_for_player(m, table);
    }
}

/// Puntero del jugador en una tabla de punteros. Original: `HUD_LoadPerPlayerPtr_025eac`.
fn player_ptr(m: &mut impl Machine, table: u32) -> u32 {
    let p = player(m);
    m.read_u32(table + p * 4)
}

/// ¿Tiene crédito el jugador `p`? En consola mira su contador; en
/// recreativa lo pregunta a la BIOS (pidiendo `want` créditos para los dos
/// primeros jugadores). Original: `$026752` (C = sí). Devuelve también el
/// byte que decide.
pub fn has_credit(m: &mut impl Machine, p: u32, want: u8) -> (bool, u8) {
    let d = if m.read_u8(bios_ram::MVS_FLAG) == 0 {
        let a = m.read_u32(L::CREDITS + p * 4);
        m.read_u8(a)
    } else {
        for (a, v) in bios_ram::CREDIT_REQUEST.into_iter().zip([want, want, 0, 0]) {
            m.write_u8(a, v);
        }
        m.call(bios::CREDIT_CHECK);
        let shared = m.read_u8(L::BIOS_SLOTS) != 1 && m.read_u8(bios_ram::USER_MODE) == bios_ram::USER_MODE_GAME;
        let k = if shared { 0 } else { p };
        m.read_u8(bios_ram::CREDIT_REQUEST[0] + k)
    };
    (d != 0, d)
}

fn credit(m: &mut impl Machine) -> bool {
    let p = player(m);
    has_credit(m, p, 1).0
}

/// Pone a cero los campos del HUD (barra, munición, parpadeos).
/// Original: `HUD_ResetFields_025ed6`.
pub fn reset_fields(m: &mut impl Memory, t: u32) {
    m.write_u8(t + L::GAUGE_CACHE, 0);
    m.write_u8(t + L::BLINK[0], 1);
    m.write_u8(t + L::BLINK[1], 1);
    m.write_u8(t + L::IN_SLUG, 0);
    m.write_u16(t + L::AMMO, 0);
    m.write_u16(t + L::BOMBS, 0);
    m.write_u32(t + L::BAR, 0);
    m.write_u32(t + L::BAR_VEL, 0);
    m.write_u8(t + L::BAR_STATE, 0);
}

/// Prepara el HUD del jugador `p` (vidas iniciales si `lives`), lo dibuja en
/// partida y devuelve la fila de descriptores de su soldado para la escena
/// actual: (tabla, desplazamiento). Original: `$0256B0` / `$025706` (y
/// `+$0E` sin vidas).
pub fn init(m: &mut impl Machine, p: usize, lives: bool) -> (u32, u16) {
    let t = m.task_register();
    if lives {
        let n = m.read_u8(bios_ram::SOFT_DIP_LIVES);
        m.write_u8(t + L::LIVES, n);
    }
    reset_fields(m, t);
    if m.read_u8(bios_ram::USER_MODE) == bios_ram::USER_MODE_GAME {
        m.call(L::DRAW_LABELS[p]);
        m.call(L::DRAW_CONTINUES[p]);
        m.call(L::DRAW_LIFE_BAR[p]);
    }
    draw_bomb_gauge(m, p);
    let scene = m.read_u8(L::SCENE);
    if scene == 0xFF {
        (L::IDLE_ROW[p], 0)
    } else {
        (L::ROWS[p], u16::from(scene) << 4)
    }
}

/// Indicador de bombas (14 casillas, llenas a la izquierda para el
/// jugador 1 y a la derecha para el 2); sólo se redibuja si cambia.
/// Original: `HUD_DrawBombGauge_P1_02630e` / `_P2_026372`.
pub fn draw_bomb_gauge(m: &mut impl Machine, p: usize) {
    let t = m.task_register();
    let n = m.read_u16(L::BOMB_GAUGE[p]) as u8;
    if n == m.read_u8(t + L::GAUGE_CACHE) {
        return;
    }
    m.write_u8(t + L::GAUGE_CACHE, n);
    let full = n.min(L::GAUGE_CELLS);
    let empty = L::GAUGE_CELLS - full;
    let runs = if p == 0 { [(full, L::GAUGE_FULL), (empty, L::GAUGE_EMPTY)] } else { [(empty, L::GAUGE_EMPTY), (full, L::GAUGE_FULL)] };
    let mut at = L::GAUGE_AT[p];
    for (k, ch) in runs {
        for _ in 0..k {
            m.write_u16(crate::layout::hw::VRAM_ADDR, at);
            m.write_u16(crate::layout::hw::VRAM_RW, ch);
            at = at.wrapping_add(L::COLUMN);
        }
    }
}

/// Alta del HUD del jugador `p`: INSERT COIN, o directamente a enlazar con
/// el soldado si ya juega (o en la demo, con el estado de la tabla de la
/// demo). Borra su puntuación. Original: `HUD_Task_StartP1_02575c` /
/// `HUD_Task_StartP2_025766`.
pub fn start(m: &mut impl Machine, p: u16) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::PLAYER, p);
    set_handler(m, L::INSERT_COIN);
    let s = start_state(m);
    if s == 1 {
        set_handler(m, L::BIND[2]);
    }
    if m.read_u8(bios_ram::USER_MODE) == bios_ram::USER_MODE_ATTRACT && s == 0 {
        set_handler(m, L::BIND[2]);
        let k = u32::from(m.read_u8(L::DEMO_SELECT) & 1) * 8 + u32::from(p) * 4;
        let h = m.read_u32(L::DEMO_STATES + k);
        if h != T::NIL {
            set_handler(m, h);
        }
    }
    let score = player_ptr(m, L::SCORES);
    m.write_u32(score, 0);
    m.write_u32(score + 4, 0);
    Flow::Return
}

/// Mientras el guion no atiende: pasa a PLEASE WAIT (y vuelve a `resume`)
/// con la tecla de crédito del jugador bloqueada. Después hace parpadear el
/// mensaje `msg`. Original: el tramo común de `$0257FA` / `$025890`.
fn wait_and_blink(m: &mut impl Machine, resume: u32, msg: u32) {
    if !input_free(m) {
        let t = m.task_register();
        set_handler(m, L::PLEASE_WAIT);
        m.write_u32(t + L::RESUME, resume);
        let key = L::KEYS + player(m);
        let k = m.read_u8(key);
        m.write_u8(t + L::SAVED_KEY, k);
        m.write_u8(key, 1);
    }
    let f = m.read_u8(crate::layout::system::FRAME_COUNTER) & L::BLINK_MASK;
    if f == 0 {
        draw(m, msg);
    } else if f == L::BLINK_OFF {
        draw(m, L::MSG_CLEAR);
    }
}

/// INSERT COIN. Original: `HUD_State_InsertCoin_0257ec`.
pub fn insert_coin(m: &mut impl Machine) -> Flow {
    draw(m, L::MSG_INSERT_COIN);
    set_handler(m, L::INSERT_COIN_RUN);
    insert_coin_run(m)
}

/// Bucle de INSERT COIN: con START a jugar; con crédito, a PUSH START.
/// Original: `$0257FA`.
pub fn insert_coin_run(m: &mut impl Machine) -> Flow {
    wait_and_blink(m, L::INSERT_COIN, L::MSG_INSERT_COIN);
    if start_state(m) == 1 {
        set_handler(m, L::BIND[2]);
        let t = m.task_register();
        m.write_u8(t + L::BUSY, 0);
        return Flow::Return;
    }
    if credit(m) {
        set_handler(m, L::PUSH_START);
    }
    Flow::Return
}

/// PUSH START. Original: `HUD_State_Continue_025882`.
pub fn push_start(m: &mut impl Machine) -> Flow {
    draw(m, L::MSG_PUSH_START);
    set_handler(m, L::PUSH_START_RUN);
    push_start_run(m)
}

/// Bucle de PUSH START: con START borra la puntuación y entra; sin crédito
/// vuelve a INSERT COIN. Original: `$025890`.
pub fn push_start_run(m: &mut impl Machine) -> Flow {
    wait_and_blink(m, L::PUSH_START, L::MSG_PUSH_START);
    if start_state(m) == 1 {
        let score = player_ptr(m, L::SCORES);
        m.write_u32(score, 0);
        m.write_u32(score + 4, 0);
        set_handler(m, L::BIND[2]);
        return Flow::Return;
    }
    if !credit(m) {
        set_handler(m, L::INSERT_COIN);
    }
    Flow::Return
}

/// Espera a que haya tiempo de misión. Original: `HUD_State_WaitPlayerSpawn_025952`.
pub fn wait_spawn(m: &mut impl Machine) -> Flow {
    hold_scene(m);
    if m.read_u16(L::RANK_TIMER) == 0 {
        return Flow::Return;
    }
    set_handler(m, L::WAIT_SPAWN_RUN);
    wait_spawn_run(m)
}

/// Espera a que el soldado quede libre; entonces vuelve a entrar (pierde
/// una vida en partida) o, sin vidas, GAME OVER. Original: `$02596A`.
pub fn wait_spawn_run(m: &mut impl Machine) -> Flow {
    hold_scene(m);
    let t = m.task_register();
    let e = m.read_u32(t + L::ENTITY);
    if m.read_u32(e) != L::FREE_HANDLER {
        return Flow::Return;
    }
    if m.read_u8(bios_ram::USER_MODE) == bios_ram::USER_MODE_ATTRACT {
        return bind(m, 0);
    }
    let lives = m.read_u8(t + L::LIVES).wrapping_sub(1);
    m.write_u8(t + L::LIVES, lives);
    if lives != 0 {
        return bind(m, 0);
    }
    set_handler(m, L::GAME_OVER);
    Flow::Return
}

/// Enlaza el HUD con el soldado del jugador: arranca su tarea con la rutina
/// de la escena y fija el tiempo de misión si es mayor que el actual.
/// `k`: 0 y 1 sin cargar vidas, 2 cargándolas. Original:
/// `HUD_State_BindPlayer_0259a0` (+$0C, +$18).
pub fn bind(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    let p = player(m) as usize;
    let (table, off) = init(m, p, k == 2);
    let row = table + u32::from(off);
    let slot = m.read_u32(row + 8);
    m.write_u32(t + L::SLOT, slot);
    let routine = m.read_u32(row);
    let e = m.read_u32(row + 4);
    m.write_u32(e + T::OWNER, t);
    m.write_u32(t + L::ENTITY, e);
    m.write_u32(e + T::HANDLER, routine);
    m.write_u16(e + L::ENTITY_ACTIVE, 1);
    let time = rank::start_time(m);
    if time >= m.read_u16(L::RANK_TIMER) {
        m.write_u16(L::RANK_TIMER, time);
    }
    set_handler(m, L::PLAYING);
    playing(m)
}

/// En juego mientras la escena esté activa; si no, a reaparecer.
/// Original: `$0259FC`.
pub fn playing(m: &mut impl Machine) -> Flow {
    if m.read_u8(L::SCENE_BUSY) != 0 {
        return in_play(m);
    }
    set_handler(m, L::RESPAWN);
    Flow::Return
}

/// Un frame en juego: si el soldado ha muerto, vacía sus bombas, ajusta el
/// retardo de vblank (partida a dos) y espera a que reaparezca; dibuja vida,
/// bombas y puntuación. Original: `HUD_State_PlayerDeath_025a0e`.
pub fn in_play(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let e = m.read_u32(t + L::ENTITY);
    if m.read_u8(L::SCENE_LOADED) == 0 {
        let v = m.read_u8(e + T::TIMER_B);
        m.write_u8(e + T::TIMER_B, v | L::FREEZE_TIMER);
        let f = m.read_u8(e + T::FLAGS2);
        m.write_u8(e + T::FLAGS2, f & !(L::DEAD | L::FLAG2_BIT3));
    }
    if m.read_u8(e + T::FLAGS2) & L::DEAD != 0 {
        let gauge = player_ptr(m, L::BOMB_GAUGES);
        m.write_u16(gauge, 0);
        if m.read_u8(L::CONTROL_LAYOUT) == L::TWO_PLAYER_LAYOUT {
            let alive = (0..2).filter(|&q| tasks::player_task_if_alive(m, q).is_some()).count();
            set_vblank_delay(m, if alive == 1 { L::ONE_ALIVE_DELAY } else { 0 });
        }
        hold_scene(m);
        set_handler(m, L::WAIT_SPAWN);
    }
    draw(m, L::DRAW_LIFE);
    draw(m, L::DRAW_BOMBS);
    draw(m, L::DRAW_SCORE);
    Flow::Return
}

/// Reaparición tras una escena: cuando el soldado está libre, la escena
/// activa y el guion atiende, reinicia su tarea y vuelve a enlazar.
/// Original: `HUD_State_Respawn_025ad8`.
pub fn respawn(m: &mut impl Machine) -> Flow {
    draw(m, L::DRAW_BOMBS);
    draw(m, L::DRAW_SCORE);
    let t = m.task_register();
    let e = m.read_u32(t + L::ENTITY);
    if m.read_u32(e) != L::FREE_HANDLER || m.read_u8(L::SCENE_BUSY) == 0 || !input_free(m) {
        return Flow::Return;
    }
    let p = player(m);
    let task = m.read_u32(L::SOLDIERS + p * 8);
    let routine = m.read_u32(L::SOLDIERS + p * 8 + 4);
    m.write_u32(task, routine);
    tasks::reset_in_place(m, task);
    bind(m, 1)
}

/// Crea las tareas del CONTINUE: la del jugador y la que vigila al otro.
fn spawn_continue(m: &mut impl Machine) {
    let t = m.task_register();
    let p = player(m);
    let h = m.read_u32(L::CONTINUE_TASKS + p * 4);
    tasks::spawn(m, h, t);
    m.write_u8(t + L::WAITING, 0xFF);
    m.write_u8(t + L::BUSY, 0xFF);
    let c = tasks::spawn(m, L::WATCH, t);
    let w = m.read_u8(t + L::WAITING);
    m.write_u8(c + L::WAITING, w);
    let b = m.read_u8(t + L::BUSY);
    m.write_u8(c + L::BUSY, b);
}

/// Sin vidas: borra el mensaje y, si queda crédito (o en recreativa con
/// continuaciones), lanza el CONTINUE; si no, GAME OVER definitivo.
/// Original: `HUD_State_GameOverEntry_025b34`.
pub fn game_over(m: &mut impl Machine) -> Flow {
    draw(m, L::MSG_CLEAR);
    let can_continue = if m.read_u8(bios_ram::MVS_FLAG) != 0 { m.read_u8(L::BIOS_CONTINUE) != 0 } else { credit(m) };
    if !can_continue {
        return final_game_over(m);
    }
    let t = m.task_register();
    let c = tasks::spawn(m, L::CONTINUE_MARK, mission::ENEMY_PARENT);
    let f = m.read_u8(c + T::FLAGS);
    m.write_u8(c + T::FLAGS, f | mission::FIXED_FLAG);
    let p = m.read_u16(t + L::PLAYER);
    m.write_u16(c + L::MARK_PLAYER, p);
    set_start_state(m, 2);
    if credit(m) {
        return countdown(m);
    }
    spawn_continue(m);
    set_handler(m, L::GAME_OVER_WAIT);
    game_over_wait(m)
}

/// Con START vuelve a jugar (cuenta la continuación); con crédito, cuenta
/// atrás; sin nada y acabada la espera, GAME OVER. Original: `$025BCE`.
pub fn game_over_wait(m: &mut impl Machine) -> Flow {
    hold_scene(m);
    let t = m.task_register();
    if start_state(m) == 1 {
        set_handler(m, L::BIND[2]);
        m.write_u8(t + L::BUSY, 0);
        count_continue(m);
        return Flow::Return;
    }
    if credit(m) {
        set_handler(m, L::COUNTDOWN);
        m.write_u8(t + L::BUSY, 0);
        return Flow::Return;
    }
    if start_state(m) == 1 || m.read_u8(t + L::WAITING) != 0 {
        return Flow::Return;
    }
    set_handler(m, L::FINAL);
    set_start_state(m, 1);
    Flow::Return
}

/// Suma una continuación en el contador del jugador. Original: `$051A86`
/// sobre la puntuación del jugador.
fn count_continue(m: &mut impl Machine) {
    let score = player_ptr(m, L::SCORES);
    let mut r = m.regs();
    r.a[1] = score;
    m.call_original(L::COUNT_CONTINUE, &mut r);
    m.set_regs(r);
}

/// Cuenta atrás del CONTINUE. Original: `HUD_State_ContinueCountdown_025c44`.
pub fn countdown(m: &mut impl Machine) -> Flow {
    spawn_continue(m);
    set_handler(m, L::COUNTDOWN_RUN);
    countdown_run(m)
}

/// Con START vuelve a jugar; acabada la cuenta, GAME OVER. Original: `$025C80`.
pub fn countdown_run(m: &mut impl Machine) -> Flow {
    hold_scene(m);
    let t = m.task_register();
    if start_state(m) == 1 {
        m.write_u8(t + L::BUSY, 0);
        count_continue(m);
        set_handler(m, L::BIND[2]);
        return Flow::Return;
    }
    if m.read_u8(t + L::WAITING) != 0 {
        return Flow::Return;
    }
    set_start_state(m, 1);
    set_handler(m, L::FINAL);
    Flow::Return
}

/// Estado de inicio del otro jugador (el que no es la madre de `t`).
fn other_state(m: &mut impl Memory, owner: u32) -> u8 {
    let q = u32::from(m.read_u16(owner + L::PLAYER) ^ 1);
    m.read_u8(L::START_STATE + q)
}

/// Hija del CONTINUE que vigila al otro jugador: si está jugando o
/// entrando, se da de baja; si no, crea su mensaje. Original: `$025CCE`.
pub fn watch(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let owner = m.read_u32(t + T::OWNER);
    let s = other_state(m, owner);
    if s == 1 || s == 2 {
        return tasks::finish(m);
    }
    let p = u32::from(m.read_u16(owner + L::PLAYER));
    let h = m.read_u32(L::WATCH_TASKS + p * 4);
    tasks::spawn_last(m, h, t);
    set_handler(m, L::WATCH_RUN);
    watch_run(m)
}

/// Sigue a la madre; se da de baja al acabar ella o si el otro jugador
/// entra. Original: `$025D0E`.
pub fn watch_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let owner = m.read_u32(t + T::OWNER);
    let b = m.read_u8(owner + L::BUSY);
    m.write_u8(t + L::BUSY, b);
    let s = other_state(m, owner);
    if s != 0 && s != 3 {
        m.write_u8(t + L::BUSY, 0);
    }
    if m.read_u8(t + L::BUSY) == 0 {
        set_handler(m, L::WATCH_END);
    }
    if m.read_u8(t + L::WAITING) == 0 {
        set_handler(m, L::WATCH_END);
    }
    Flow::Return
}

/// GAME OVER definitivo. Original: `HUD_State_GameOverFinal_025d64`.
pub fn final_game_over(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    set_start_state(m, 1);
    m.write_u8(t + L::WAITING, 0xFF);
    m.write_u8(t + L::BUSY, 0xFF);
    call_for_player(m, L::GAME_OVER_TEXT);
    set_handler(m, L::FINAL_RUN);
    final_run(m)
}

/// Al acabar el texto: GAME OVER del jugador o, tras la última misión,
/// fin de la tarea. Original: `$025D8A`.
pub fn final_run(m: &mut impl Machine) -> Flow {
    hold_scene(m);
    let t = m.task_register();
    if m.read_u8(t + L::WAITING) != 0 {
        return Flow::Return;
    }
    draw(m, L::MSG_CLEAR);
    if m.read_u8(L::MISSION) < L::LAST_MISSION {
        set_handler(m, L::DONE);
    } else {
        tasks::kill_children(m, t);
        set_handler(m, L::FREE_HANDLER);
    }
    m.write_u8(t + L::BUSY, 0);
    set_start_state(m, 3);
    Flow::Return
}

/// GAME OVER en el HUD; tras una espera, si el guion atiende, vuelve a
/// PUSH START (con crédito) o a INSERT COIN (recreativa).
/// Original: `HUD_State_GameOverDone_025dd2`.
pub fn done(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    call_for_player(m, L::MSG_GAME_OVER);
    let gauge = player_ptr(m, L::BOMB_GAUGES);
    m.write_u16(gauge, 0);
    draw(m, L::DRAW_BOMBS);
    m.write_u8(t + T::TIMER_A, L::DONE_DELAY);
    set_handler(m, L::DONE_RUN);
    done_run(m)
}

/// Original: `$025E00`.
pub fn done_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if m.read_u8(t + T::TIMER_A) != 0 || !input_free(m) {
        return Flow::Return;
    }
    set_start_state(m, 0);
    if credit(m) {
        set_handler(m, L::PUSH_START);
        return Flow::Return;
    }
    if m.read_u8(bios_ram::MVS_FLAG) != 0 {
        set_handler(m, L::INSERT_COIN);
    }
    Flow::Return
}

/// PLEASE WAIT mientras el guion no atiende; luego vuelve al estado
/// guardado y desbloquea la tecla. Original: `HUD_State_PushStartBlink_025e48`.
pub fn please_wait(m: &mut impl Machine) -> Flow {
    draw(m, L::MSG_PLEASE_WAIT);
    set_handler(m, L::PLEASE_WAIT_RUN);
    please_wait_run(m)
}

/// Original: `$025E56`.
pub fn please_wait_run(m: &mut impl Machine) -> Flow {
    if !input_free(m) {
        return Flow::Return;
    }
    let t = m.task_register();
    let h = m.read_u32(t + L::RESUME);
    set_handler(m, h);
    let k = m.read_u8(t + L::SAVED_KEY);
    let key = L::KEYS + player(m);
    m.write_u8(key, k);
    Flow::Return
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::TestMachine;

    #[test]
    fn bomb_gauge_p1_fills_left() {
        let mut m = TestMachine::default();
        m.regs.a[6] = 0x10_0300;
        m.write_u16(L::BOMB_GAUGE[0], 3);
        draw_bomb_gauge(&mut m, 0);
        assert_eq!(m.read_u8(0x10_0300 + L::GAUGE_CACHE), 3);
    }
}
