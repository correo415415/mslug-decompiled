//! Eventos de la misión: cada escena tiene en la ROM una lista de órdenes
//! (apariciones, pausas, esperas, ramas al azar...) que se van ejecutando a
//! medida que avanza el scroll (`CAMERA_PROGRESS`).
//!
//! El cursor de la lista es una dirección de la ROM; [`exec`] ejecuta la
//! orden que hay en ella y [`skip`] la salta sin ejecutarla. Las dos
//! devuelven dónde sigue la lista; `None` = la orden de fin (`OP_END`), con
//! la que la tarea que la ejecuta se da de baja.
//!
//! Original: `MissionDriver_Init_0442E6` … `Spawn_FromStream_0448A6`
//! (`asm/mission/event_vm_04422a.s`).

use super::{rng, tasks};
use crate::layout::{events as L, task as T};
use crate::mem::{Flow, Machine, Memory};

/// Arranca los eventos de la escena actual (sin escena: se para con sus
/// hijas). Crea las dos tareas de apoyo y pasa al bucle.
/// Original: `MissionDriver_Init_0442E6`.
pub fn start(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let scene = m.read_u8(L::SCENE);
    if scene & 0x80 != 0 {
        tasks::kill_children(m, t);
        m.write_u32(t + T::HANDLER, L::IDLE);
        return Flow::Return;
    }
    let mut list = m.read_u32(L::STREAMS + u32::from(scene) * 4);
    if list == L::ARCADE_STREAM.0 && m.read_u8(L::BIOS_SLOTS) != 0 {
        list = L::ARCADE_STREAM.1;
    }
    m.write_u32(t + L::CURSOR, list);
    m.write_u8(L::HELPER_FLAG, 0);
    for h in L::HELPERS {
        tasks::spawn(m, h, L::HELPER_PARENT);
    }
    m.write_u8(t + L::PAUSED, 0);
    m.write_u16(t + L::WAIT, 0);
    m.write_u16(t + L::THRESHOLD, 0);
    m.write_u32(t + T::HANDLER, L::RUN);
    run(m)
}

/// Bucle de cada frame: sin pausa, ejecuta las órdenes cuyo umbral ya ha
/// pasado el scroll; en pausa, cuenta los frames de espera de la orden
/// actual y la ejecuta al acabar. Original: `MissionDriver_Loop_044368`.
pub fn run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let mut p = m.read_u32(t + L::CURSOR);
    loop {
        if m.read_u8(t + L::PAUSED) == 0 {
            let at = m.read_u16(p + L::OP_PARAM);
            if at == L::NO_THRESHOLD {
                return Flow::Return;
            }
            m.write_u16(t + L::THRESHOLD, at);
            if at > m.read_u16(L::CAMERA_PROGRESS) {
                return Flow::Return;
            }
        } else {
            let left = match m.read_u16(t + L::WAIT) {
                0 => m.read_u16(p + L::OP_PARAM),
                n => n - 1,
            };
            m.write_u16(t + L::WAIT, left);
            if left != 0 {
                return Flow::Return;
            }
        }
        match exec(m, t, p) {
            Some(next) => p = next,
            None => return tasks::finish(m),
        }
        m.write_u32(t + L::CURSOR, p);
    }
}

/// El original deja basura en `d7` que acaba en la capa de las apariciones
/// (ver [`spawn_from_list`]): se sigue aquí lo que escribe en él.
fn set_d7(m: &mut impl Machine, mask: u32, v: u32) {
    let mut r = m.regs();
    r.d[7] = (r.d[7] & !mask) | (v & mask);
    m.set_regs(r);
}

/// ¿Está jugando el jugador `k`? (estado de la BIOS = 1).
fn playing(m: &mut impl Memory, k: usize) -> bool {
    m.read_u8(L::START_STATE[k]) == 1
}

fn player_xy(m: &mut impl Memory, k: usize) -> (u16, u16) {
    (m.read_u16(L::PLAYERS[k] + L::X), m.read_u16(L::PLAYERS[k] + L::Y))
}

/// Altura (hacia arriba) a partir de la Y de pantalla, sumando la cámara si
/// la orden es relativa al mundo (`byte 1` = 0).
fn height(m: &mut impl Memory, p: u32, y: u16) -> u16 {
    let h = L::SCREEN_BOTTOM.wrapping_sub(y);
    if m.read_u8(p + 1) == 0 {
        h.wrapping_add(m.read_u16(L::CAMERA_Y))
    } else {
        h
    }
}

/// Resultado de comparar `v` con el umbral de la orden (`p+4`): `v` está
/// por debajo (resta negativa).
fn below_threshold(m: &mut impl Memory, p: u32, v: u16) -> bool {
    (v.wrapping_sub(m.read_u16(p + 4)) as i16) < 0
}

/// Ejecuta la primera de las dos órdenes que siguen y salta la otra
/// (`first`), o al revés.
fn one_of_two(m: &mut impl Machine, t: u32, p: u32, first: bool) -> Option<u32> {
    if first {
        let p = exec(m, t, p)?;
        Some(skip(m, p))
    } else {
        let p = skip(m, p);
        exec(m, t, p)
    }
}

/// Ejecuta la orden en `p` (con `t` = tarea que la ejecuta).
/// Original: `MissionVM_ExecOp_0443C0`.
pub fn exec(m: &mut impl Machine, t: u32, p: u32) -> Option<u32> {
    let op = m.read_u8(p);
    Some(match op {
        L::OP_SPAWN => {
            spawn_from_list(m, t, p);
            p + L::SPAWN_LEN
        }
        L::OP_PERIODIC => {
            let body = start_periodic(m, t, p);
            skip(m, body)
        }
        L::OP_BLOCK => {
            let mut p = p + 4;
            while m.read_u8(p) != L::OP_BLOCK_END {
                p = exec(m, t, p)?;
            }
            p + 2
        }
        L::OP_RANDOM => {
            let mut options = Vec::new();
            let mut p = p + 4;
            while m.read_u8(p) != L::OP_RANDOM_END {
                options.push(p);
                p = skip(m, p);
            }
            set_d7(m, 0xFFFF_FFFF, options.len() as u32);
            if !options.is_empty() {
                set_d7(m, 0xFFFF, (options.len() as u32) * 4);
                let (k, _) = rng::below(m, options.len() as u16);
                // El original las apila: el índice 0 es la última.
                let pick = options[options.len() - 1 - usize::from(k)];
                exec(m, t, pick)?;
            }
            p + 2
        }
        L::OP_PAST_Y => {
            let (mut a, mut b) = ((0u16, 0u16), (0u16, 0u16));
            if playing(m, 0) {
                a = player_xy(m, 0);
            }
            if playing(m, 1) {
                b = player_xy(m, 1);
            }
            // Cuenta la Y del jugador que va más adelantado.
            let y = if (a.0 as i16) < (b.0 as i16) { b.1 } else { a.1 };
            let h = height(m, p, y);
            let below = below_threshold(m, p, h);
            return one_of_two(m, t, p + 6, below);
        }
        L::OP_PAUSE_FLAG => {
            m.write_u8(t + L::PAUSED, 1);
            m.write_u8(L::PAUSE_FLAG, 1);
            p + 4
        }
        L::OP_PAUSE => {
            m.write_u8(t + L::PAUSED, 1);
            p + 4
        }
        L::OP_WAIT_ENEMIES => {
            let alive = m.read_u16(L::ENEMY_COUNT) as u8;
            if alive > m.read_u8(p + 1) {
                p
            } else {
                p + 4
            }
        }
        L::OP_RESUME_FLAG => {
            m.write_u8(t + L::PAUSED, 0);
            m.write_u8(L::PAUSE_FLAG, 0);
            p + 4
        }
        L::OP_NOP => p + 4,
        L::OP_PAST_X => {
            let a = if playing(m, 0) { m.read_u16(L::PLAYERS[0] + L::X) } else { 0xFFFF };
            let b = if playing(m, 1) { m.read_u16(L::PLAYERS[1] + L::X) } else { 0xFFFF };
            // Cuenta el jugador más retrasado.
            let mut x = a.min(b);
            if m.read_u8(p + 1) == 0 {
                x = x.wrapping_add(m.read_u16(L::CAMERA_X));
            }
            let below = below_threshold(m, p, x);
            return one_of_two(m, t, p + 6, below);
        }
        L::OP_BOTH_PAST_Y => {
            let mut a = if playing(m, 0) { m.read_u16(L::PLAYERS[0] + L::Y) } else { 0 };
            let mut b = if playing(m, 1) { m.read_u16(L::PLAYERS[1] + L::Y) } else { 0 };
            if a == 0 {
                a = b;
            }
            if b == 0 {
                b = a;
            }
            let ha = height(m, p, a);
            let hb = height(m, p, b);
            let below_a = below_threshold(m, p, ha);
            let below_b = below_threshold(m, p, hb);
            let p = p + 6;
            let (sa, sb) = (if below_a { 0xFF } else { 0 }, if below_b { 0xFF } else { 0 });
            set_d7(m, 0xFF, if below_a && below_b { sa & sb } else { sa });
            // Tres ramas: los dos por debajo, el primero (el original
            // sólo mira al primero aquí), ninguno.
            return if below_a && below_b {
                let p = exec(m, t, p)?;
                let p = skip(m, p);
                Some(skip(m, p))
            } else if below_a {
                let p = skip(m, p);
                let p = exec(m, t, p)?;
                Some(skip(m, p))
            } else {
                let p = skip(m, p);
                let p = skip(m, p);
                exec(m, t, p)
            };
        }
        _ => return None,
    })
}

/// Salta la orden en `p` sin ejecutarla. Original: `MissionVM_SkipOp_0446B6`.
pub fn skip(m: &mut impl Memory, p: u32) -> u32 {
    match m.read_u8(p) {
        L::OP_SPAWN => p + L::SPAWN_LEN,
        L::OP_PERIODIC => skip(m, p + L::PERIODIC_LEN),
        op @ (L::OP_BLOCK | L::OP_RANDOM) => {
            let end = if op == L::OP_BLOCK { L::OP_BLOCK_END } else { L::OP_RANDOM_END };
            let mut p = p + 4;
            while m.read_u8(p) != end {
                p = skip(m, p);
            }
            p + 2
        }
        L::OP_PAST_Y | L::OP_PAST_X => {
            let p = skip(m, p + 6);
            skip(m, p)
        }
        L::OP_BOTH_PAST_Y => {
            let p = skip(m, p + 6);
            let p = skip(m, p);
            skip(m, p)
        }
        L::OP_END => p,
        _ => p + 4,
    }
}

/// Byte `v` de la orden, ×32 si el byte `flag` no es 0.
fn scaled(m: &mut impl Memory, v: u32, flag: u32) -> u16 {
    let n = u16::from(m.read_u8(v));
    if m.read_u8(flag) != 0 {
        n << 5
    } else {
        n
    }
}

/// Crea la tarea que repite la orden que sigue cada cierto tiempo y
/// devuelve dónde empieza esa orden. Original: `PeriodicSpawner_Setup_0447C6`.
pub fn start_periodic(m: &mut impl Machine, t: u32, p: u32) -> u32 {
    let c = tasks::spawn(m, L::PERIODIC, t);
    let first = scaled(m, p + 5, p + 8);
    let every = scaled(m, p + 6, p + 9);
    let (times, max_alive, until) = (m.read_u8(p + 1), m.read_u8(p + 7), m.read_u8(p + 4));
    m.write_u16(c + L::P_TIMER, first);
    m.write_u8(c + L::P_TIMES, times);
    m.write_u16(c + L::P_EVERY, every);
    m.write_u8(c + L::P_MAX_ALIVE, max_alive);
    let limit = (u16::from(until) << 4).wrapping_add(m.read_u16(t + L::THRESHOLD));
    m.write_u16(c + L::P_UNTIL, limit);
    let body = p + L::PERIODIC_LEN;
    m.write_u32(c + L::CURSOR, body);
    body
}

/// Tarea que repite una orden: mientras la escena siga y el scroll no pase
/// su límite, cada `P_EVERY` frames (si no hay demasiados enemigos) la
/// ejecuta; al agotar las repeticiones se da de baja.
/// Original: `PeriodicSpawner_Task_04482A` (+ `_Tick`, `_Fire`).
pub fn periodic(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if m.read_u8(L::SCENE_LOADED) == 0 || m.read_u16(L::CAMERA_PROGRESS) >= m.read_u16(t + L::P_UNTIL) {
        tasks::kill_children(m, t);
        return tasks::finish(m);
    }
    let timer = m.read_u16(t + L::P_TIMER);
    if timer != 0 {
        m.write_u16(t + L::P_TIMER, timer - 1);
        return Flow::Return;
    }
    let max = m.read_u8(t + L::P_MAX_ALIVE);
    if max != 0xFF && u16::from(max) < m.read_u16(L::ENEMY_COUNT) {
        return Flow::Return;
    }
    let p = m.read_u32(t + L::CURSOR);
    if exec(m, t, p).is_none() {
        return tasks::finish(m);
    }
    let every = m.read_u16(t + L::P_EVERY);
    m.write_u16(t + L::P_TIMER, every);
    let times = m.read_u8(t + L::P_TIMES);
    if times == 0 {
        return Flow::Return;
    }
    m.write_u8(t + L::P_TIMES, times - 1);
    if times - 1 != 0 {
        return Flow::Return;
    }
    tasks::kill_children(m, t);
    tasks::finish(m)
}

/// Crea una tarea que ejecuta la lista `list` mientras el byte `flag` de la
/// tarea actual siga a 0. Original: `MissionWatch_Spawn_04429E`.
pub fn spawn_watch(m: &mut impl Machine, list: u32, flag: u16) -> u32 {
    let t = m.task_register();
    let c = tasks::spawn(m, L::WATCH, t);
    m.write_u16(c + L::WATCH_FLAG, flag);
    m.write_u32(c + L::CURSOR, list);
    m.write_u8(c + L::PAUSED, 0);
    m.write_u16(c + L::WAIT, 0);
    m.write_u16(c + L::THRESHOLD, 0);
    c
}

/// Original: `MissionWatch_Handler_0442C4`.
pub fn watch(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let owner = m.read_u32(t + T::OWNER);
    let flag = u32::from(m.read_u16(t + L::WATCH_FLAG));
    if m.read_u8(owner + flag) != 0 {
        tasks::kill_children(m, t);
        return tasks::finish(m);
    }
    run(m)
}

/// Pasa a coordenadas de la capa `CAMERA_LAYER` la tarea `e` recién creada.
/// Original: `$04405E`.
pub fn to_layer(m: &mut impl Memory, e: u32) {
    let dx = m.read_u16(L::LAYER_POS[0]);
    let x = m.read_u16(e + L::X).wrapping_sub(dx);
    m.write_u16(e + L::X, x);
    let dy = m.read_u16(L::LAYER_POS[1]);
    let y = m.read_u16(e + L::Y).wrapping_add(dy);
    m.write_u16(e + L::Y, y);
    let f = m.read_u8(e + T::FLAGS2);
    m.write_u8(e + T::FLAGS2, f | L::ON_LAYER);
}

/// Aparición de la lista (registro de 18 bytes): la plantilla `+4` de la
/// tabla de la ROM, en `+6`/`+8` (de pantalla o de mundo según los bits),
/// con 8 bytes de parámetros en `+$98`. Los bits 0–1 de `+1` la limitan a
/// partidas de 1 o 2 jugadores, o a que el Slug esté en juego.
/// Original: `Spawn_FromStream_0448A6`.
pub fn spawn_from_list(m: &mut impl Machine, t: u32, p: u32) {
    let flags = m.read_u8(p + 1);
    set_d7(m, 0xFF, u32::from(flags & L::ONLY_MASK));
    match flags & L::ONLY_MASK {
        0 => {}
        L::ONLY_WITH_SLUG => {
            if m.read_u8(L::SLUG_ACTIVE) == 0 {
                return;
            }
        }
        n => {
            if n != m.read_u8(L::ACTIVE_PLAYERS) {
                return;
            }
        }
    }
    let k = u32::from(m.read_u16(p + 4));
    let routine = m.read_u32(L::TEMPLATES + k * 4);
    let c = tasks::spawn(m, routine, L::SPAWN_PARENT);
    if flags & L::KEEP_LOOSE == 0 {
        let f = m.read_u8(c + T::FLAGS);
        m.write_u8(c + T::FLAGS, f | L::COUNTED);
    }
    let mut x = m.read_u16(p + 6);
    if flags & L::SCREEN_X == 0 {
        x = x.wrapping_sub(m.read_u16(L::CAMERA_X));
    }
    m.write_u16(c + L::X, x);
    let mut y = m.read_u16(p + 8);
    if flags & L::SCREEN_Y == 0 {
        y = y.wrapping_sub(m.read_u16(L::CAMERA_Y));
    }
    m.write_u16(c + L::Y, L::SCREEN_BOTTOM.wrapping_sub(y));
    if t != L::SPAWN_PARENT {
        if m.read_u8(c + T::FLAGS) & L::COUNTED != 0 {
            let n = m.read_u16(L::SPAWNED);
            m.write_u16(L::SPAWNED, n.wrapping_add(1));
        }
        to_layer(m, c);
    }
    // `andi.b #$C,d7 ; lsr.w #2,d7`: los bits 8–9 de d7 entran en el byte.
    let d7 = (m.regs().d[7] & 0xFF00) | u32::from(flags & L::LAYER_MASK);
    m.write_u8(c + L::LAYER, (d7 >> 2) as u8);
    // La copia de los parámetros termina con `dbra` (d7.w = $FFFF).
    set_d7(m, 0xFFFF, 0xFFFF);
    for i in 0..L::PARAMS_LEN {
        let b = m.read_u8(p + 0xA + i);
        m.write_u8(c + L::PARAMS + i, b);
    }
}
