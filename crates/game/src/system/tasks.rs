//! Tareas y planificador.
//!
//! El juego es una lista circular de tareas (bloques de `task::SIZE`
//! bytes). Cada frame, el planificador recorre la lista y llama a la rutina
//! de cada tarea con la tarea en `a6`. Hay unas tareas fijas (las crea el
//! arranque a partir de una tabla de la ROM) y un almacén de bloques libres
//! para las que se crean durante el juego.
//!
//! Original: `Task_FreeListInit_000410`, `Entity_ClearPtrSlots_05DC1C`,
//! `Entity_InitFields_05DC34`, `Scheduler_MainLoop_000656`,
//! `Player_GetEntity_05e3a2`, `Players_CountActive_001b1c`,
//! `Players_StoreActiveCount_001b40`.

use super::{dec_to_zero, inc_u16};
use crate::layout::{pads, routine, task as T};
use crate::mem::{Flow, Machine, Memory};

/// Deja un bloque de tarea como nuevo: todos los punteros a «ninguno» y los
/// campos con su valor inicial. Original: `Entity_ClearPtrSlots_05DC1C`
/// seguido de `Entity_InitFields_05DC34`.
pub fn reset_block(m: &mut impl Memory, t: u32) {
    clear_ptr_slots(m, t);
    init_fields(m, t);
}

/// Original: `Entity_ClearPtrSlots_05DC1C`.
pub fn clear_ptr_slots(m: &mut impl Memory, t: u32) {
    for off in (T::PTR_SLOTS_FIRST..=T::PTR_SLOTS_LAST).step_by(4) {
        m.write_u32(t + off, T::NIL);
    }
}

/// Original: `Entity_InitFields_05DC34`.
pub fn init_fields(m: &mut impl Memory, t: u32) {
    for (off, v) in T::RESET_WORDS {
        m.write_u16(t + off, v);
    }
    for (off, v) in T::RESET_BYTES {
        m.write_u8(t + off, v);
    }
    for off in T::RESET_LONGS {
        m.write_u32(t + off, 0);
    }
    let f = m.read_u8(t + T::FLAGS2);
    m.write_u8(t + T::FLAGS2, f | T::FLAGS2_FRESH);
}

/// Prepara el almacén de bloques libres y la lista de tareas fijas.
/// Original: `Task_FreeListInit_000410`.
pub fn init(m: &mut impl Memory) {
    // Almacén: todos los bloques enlazados, el último sin siguiente.
    m.write_u32(T::FREE_LIST, T::POOL);
    for i in 0..T::POOL_COUNT {
        let b = T::POOL + i * T::SIZE;
        reset_block(m, b);
        let next = if i + 1 == T::POOL_COUNT { T::NIL } else { b + T::SIZE };
        m.write_u32(b + T::NEXT, next);
    }
    // Tareas fijas: (bloque, rutina) de la tabla de la ROM, en una lista
    // circular que acaba en `LAST_FIXED` y vuelve a la primera.
    let mut prev = T::LAST_FIXED;
    let mut entry = T::FIXED_TABLE;
    loop {
        let t = m.read_u32(entry);
        reset_block(m, t);
        m.write_u8(t + T::PRIORITY, 0);
        m.write_u8(t + T::FLAGS, T::FLAGS_FIXED);
        m.write_u32(t + T::OWNER, t);
        let handler = m.read_u32(entry + 4);
        m.write_u32(t + T::HANDLER, handler);
        m.write_u32(t + T::PREV, prev);
        m.write_u32(prev + T::NEXT, t);
        prev = t;
        entry += 8;
        if t == T::LAST_FIXED {
            m.write_u32(t + T::NEXT, T::ARENA);
            break;
        }
    }
}

/// Arranca el planificador en la primera tarea. No vuelve: la ejecución
/// pasa a la rutina de la tarea, y al volver ésta sigue en
/// [`after_task`]. Original: `Scheduler_MainLoop_000656`.
pub fn run(m: &mut impl Machine) -> Flow {
    m.set_task_register(T::ARENA);
    m.write_u32(T::CURRENT, T::ARENA);
    enter_task(m)
}

/// Prepara la tarea de `a6` (temporizadores, flags) y la llama.
fn enter_task(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let f3 = m.read_u8(t + T::FLAGS3);
    m.write_u8(t + T::FLAGS3, f3 & T::FLAGS3_KEEP_BEFORE);
    let f4 = m.read_u8(t + T::FLAGS4);
    m.write_u8(t + T::FLAGS4, f4 & T::FLAGS4_KEEP_BEFORE);
    dec_to_zero(m, t + T::TIMER_A);
    if m.read_u8(t + T::FLAGS) & T::FLAGS_MARKED != 0 {
        inc_u16(m, T::MARKED);
    }
    dec_to_zero(m, t + T::TIMER_C);
    dec_to_zero(m, t + T::TIMER_B);
    let handler = m.read_u32(t + T::HANDLER);
    let sp = m.stack_pointer();
    m.write_u32(T::SAVED_SP, sp);
    Flow::Call { routine: handler, then: routine::SCHEDULER_AFTER_TASK }
}

/// Vuelta de una tarea: limpia sus flags del frame y pasa a la siguiente.
/// Original: `Scheduler_MainLoop_000656` + `$1E` (`$000674`).
pub fn after_task(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let f2 = m.read_u8(t + T::FLAGS2);
    m.write_u8(t + T::FLAGS2, f2 & !T::FLAGS2_FRESH);
    let f3 = m.read_u8(t + T::FLAGS3);
    m.write_u8(t + T::FLAGS3, f3 & T::FLAGS3_KEEP_AFTER);
    let f4 = m.read_u8(t + T::FLAGS4);
    m.write_u8(t + T::FLAGS4, f4 & T::FLAGS4_KEEP_AFTER);
    inc_u16(m, T::RUN_COUNT);
    // La tarea puede haber dejado la pila como sea.
    let sp = m.read_u32(T::SAVED_SP);
    m.set_stack_pointer(sp);
    let next = m.read_u32(t + T::NEXT);
    m.set_task_register(next);
    enter_task(m)
}

/// Tarea de control del jugador `p` (0 o 1) si está en juego.
/// Original: `Player_GetEntity_05e3a2` (C = sí, tarea en `a0`).
pub fn player_task_if_alive(m: &mut impl Memory, p: u32) -> Option<u32> {
    let t = m.read_u32(routine::PLAYER_TASKS + (p & 1) * 4);
    if m.read_u8(t + pads::PLAYER_TASK_ALIVE) & 0x80 == 0 {
        return None;
    }
    let handler = m.read_u32(t + T::HANDLER);
    if pads::INACTIVE_HANDLERS.contains(&handler) {
        return None;
    }
    Some(t)
}

/// Jugadores en juego (0, 1 o 2). Original: `Players_CountActive_001b1c`.
pub fn count_active_players(m: &mut impl Memory) -> u16 {
    (0..2).filter(|&p| player_task_if_alive(m, p).is_some()).count() as u16
}

/// Original: `Players_StoreActiveCount_001b40`.
pub fn store_active_players(m: &mut impl Memory) -> u16 {
    let n = count_active_players(m);
    m.write_u8(pads::ACTIVE_PLAYERS, n as u8);
    n
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::TestMachine;

    #[test]
    fn pool_is_linked_and_scheduler_calls_first_task() {
        let mut m = TestMachine::default();
        // Tabla de tareas fijas: sólo la última.
        m.write_u32(T::FIXED_TABLE, T::LAST_FIXED);
        m.write_u32(T::FIXED_TABLE + 4, 0x1234);
        init(&mut m);
        assert_eq!(m.read_u32(T::POOL + T::NEXT), T::POOL + T::SIZE);
        assert_eq!(m.read_u32(T::POOL + (T::POOL_COUNT - 1) * T::SIZE + T::NEXT), T::NIL);
        assert_eq!(m.read_u32(T::LAST_FIXED + T::NEXT), T::ARENA);
        m.write_u32(T::ARENA + T::HANDLER, 0x5678);
        let f = run(&mut m);
        assert_eq!(f, Flow::Call { routine: 0x5678, then: routine::SCHEDULER_AFTER_TASK });
    }
}
