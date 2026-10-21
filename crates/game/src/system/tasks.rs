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

/// Da de alta en el bloque `t` una tarea con rutina `handler`, hija de la
/// tarea `parent`: va justo detrás de ella en la lista, con prioridad una
/// más. Original: `Task_AllocAndMarkBusy_00050e` (la parte común con
/// `Task_Alloc_0004AE` empieza en `$0004C6`).
pub fn spawn_at(m: &mut impl Memory, t: u32, handler: u32, parent: u32) {
    reset_block(m, t);
    m.write_u32(t + T::HANDLER, handler);
    m.write_u32(t + T::OWNER, parent);
    let prio = m.read_u8(parent + T::PRIORITY).wrapping_add(1);
    m.write_u8(t + T::PRIORITY, prio);
    let next = m.read_u32(parent + T::NEXT);
    m.write_u32(parent + T::NEXT, t);
    m.write_u32(t + T::PREV, parent);
    m.write_u32(t + T::NEXT, next);
    m.write_u32(next + T::PREV, t);
    init_fields(m, t);
}

/// Como [`spawn_at`], marcando la tarea como fija.
/// Original: `Task_AllocAndMarkBusy_00050e`.
pub fn spawn_fixed_at(m: &mut impl Memory, t: u32, handler: u32, parent: u32) {
    spawn_at(m, t, handler, parent);
    let f = m.read_u8(t + T::FLAGS);
    m.write_u8(t + T::FLAGS, f | T::FLAGS_FIXED);
}

/// Crea una tarea en un bloque libre. Devuelve el bloque, o el bloque de
/// desbordamiento si no quedan (y entonces no se crea nada).
/// Original: `Task_Alloc_0004AE`.
pub fn spawn(m: &mut impl Memory, handler: u32, parent: u32) -> u32 {
    let t = m.read_u32(T::FREE_LIST);
    if t == T::NIL {
        return T::OVERFLOW_BLOCK;
    }
    let next = m.read_u32(t + T::NEXT);
    m.write_u32(T::FREE_LIST, next);
    spawn_at(m, t, handler, parent);
    t
}

/// Como [`spawn`], pero la hija va detrás de todas las tareas que siguen a
/// `parent` con prioridad igual o mayor que la suya (al final de sus
/// hermanas). Devuelve el bloque y la tarea que queda detrás.
/// Original: `Task_AllocLast_0006FE`.
pub fn spawn_last(m: &mut impl Memory, handler: u32, parent: u32) -> (u32, u32) {
    let t = m.read_u32(T::FREE_LIST);
    if t == T::NIL {
        return (T::OVERFLOW_BLOCK, handler);
    }
    let next = m.read_u32(t + T::NEXT);
    m.write_u32(T::FREE_LIST, next);
    reset_block(m, t);
    m.write_u32(t + T::HANDLER, handler);
    m.write_u32(t + T::OWNER, parent);
    let prio = m.read_u8(parent + T::PRIORITY).wrapping_add(1);
    m.write_u8(t + T::PRIORITY, prio);
    let mut after = parent;
    loop {
        after = m.read_u32(after + T::NEXT);
        if prio > m.read_u8(after + T::PRIORITY) {
            break;
        }
    }
    let before = m.read_u32(after + T::PREV);
    m.write_u32(before + T::NEXT, t);
    m.write_u32(t + T::PREV, before);
    m.write_u32(t + T::NEXT, after);
    m.write_u32(after + T::PREV, t);
    init_fields(m, t);
    (t, after)
}

/// Reinicia una tarea en su sitio (conserva prioridad y enlaces) y la
/// marca como fija. Original: `$0005FE`.
pub fn reset_in_place(m: &mut impl Memory, t: u32) {
    let prio = m.read_u16(t + T::PRIORITY);
    reset_block(m, t);
    m.write_u16(t + T::PRIORITY, prio);
    m.write_u16(t + T::FLAGS, u16::from(T::FLAGS_FIXED) << 8);
}

/// Da de baja la tarea `t`: devuelve sus ranuras de sprite, la saca de la
/// lista y, si no es fija, devuelve el bloque al almacén (una fija se queda
/// con la rutina vacía). Devuelve la tarea anterior, por la que sigue
/// quien recorre la lista. Original: `Task_UnlinkAlive_0546`.
pub fn unlink(m: &mut impl Memory, t: u32) -> u32 {
    let fixed = m.read_u8(t + T::FLAGS) & T::FLAGS_FIXED != 0;
    super::video::release_task_entries(m, t);
    let prev = m.read_u32(t + T::PREV);
    let next = m.read_u32(t + T::NEXT);
    m.write_u32(prev + T::NEXT, next);
    m.write_u32(next + T::PREV, prev);
    m.write_u32(t + T::LINK, T::NIL);
    if fixed {
        m.write_u32(t + T::HANDLER, routine::IDLE);
        let f = m.read_u8(t + T::FLAGS);
        m.write_u8(t + T::FLAGS, f | T::FLAGS_FIXED);
    } else {
        m.write_u32(t + T::HANDLER, T::NIL);
        let head = m.read_u32(T::FREE_LIST);
        m.write_u32(t + T::NEXT, head);
        m.write_u32(T::FREE_LIST, t);
    }
    prev
}

/// Da de baja todas las tareas que cuelgan de `t` (las que la siguen en la
/// lista con más prioridad). Original: `$0005B6`.
pub fn kill_children(m: &mut impl Memory, t: u32) {
    let prio = m.read_u8(t + T::PRIORITY);
    let mut cur = m.read_u32(t + T::NEXT);
    while prio < m.read_u8(cur + T::PRIORITY) {
        let prev = unlink(m, cur);
        cur = m.read_u32(prev + T::NEXT);
    }
}

/// `t` no es una tarea válida (puntero vacío o tarea vacía).
fn is_none(t: u32) -> bool {
    t == T::NIL || t == routine::IDLE
}

/// Da de baja las hijas de `t` (si es una tarea). Original: `Task_ChangeHandler_05DA`.
pub fn kill_children_of(m: &mut impl Memory, t: u32) {
    if !is_none(t) {
        kill_children(m, t);
    }
}

/// Para la tarea `t`: da de baja sus hijas y la deja vacía y como nueva.
/// Original: `Task_ChangeAndRun_0626`.
pub fn stop(m: &mut impl Memory, t: u32) {
    if is_none(t) {
        return;
    }
    kill_children(m, t);
    m.write_u32(t + T::HANDLER, routine::IDLE);
    reset_in_place(m, t);
}

/// Tarea de arranque: pone rutina a las tareas fijas, reinicia las de los
/// jugadores, enlaza cada jugador con su tarea de control y se queda sin
/// nada que hacer. Original: `TaskSlots_BootInstall_000A7C`.
pub fn boot_install(m: &mut impl Machine) {
    let me = m.task_register();
    for (t, handler) in T::BOOT_INSTALL {
        spawn_fixed_at(m, t, handler, me);
    }
    for t in T::BOOT_RESET {
        reset_in_place(m, t);
    }
    for k in 0..2 {
        let e = crate::layout::PLAYER_ENTITY[k];
        m.write_u32(e + T::OWNER, crate::layout::input::PLAYER_TASK[k]);
    }
    m.write_u32(me + T::HANDLER, routine::BOOT_DONE);
}

/// Arranca el planificador en la primera tarea. No vuelve: la ejecución
/// pasa a la rutina de la tarea, y al volver ésta sigue en
/// [`after_task`]. Original: `Scheduler_MainLoop_000656`.
pub fn run(m: &mut impl Machine) -> Flow {
    m.set_task_register(T::ARENA);
    enter_task(m)
}

/// Prepara la tarea de `a6` (temporizadores, flags) y la llama.
fn enter_task(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u32(T::CURRENT, t);
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
    let mut r = m.regs();
    r.a[0] = handler;
    m.set_regs(r);
    Flow::Call { routine: handler, then: routine::SCHEDULER_AFTER_TASK }
}

/// Vuelta de una tarea: limpia sus flags del frame y pasa a la siguiente.
/// Original: `Scheduler_MainLoop_000656` + `$74` (`$0006CA`).
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

/// Rutina de una tarea que no hace nada. Original: `Task_IdleRts_000400`
/// (y `$000B90`, la que deja la tarea de arranque).
pub fn idle(_: &mut impl Machine) -> Flow {
    Flow::Return
}

/// Tarea fija: cada frame borra el byte `FRAME_FLAG_8D` del Slug.
/// Original: `JsrPcThunk_029582` → `$02A46C`.
pub fn clear_slug_frame_flag(m: &mut impl Machine) -> Flow {
    m.write_u8(T::SLUG + T::FRAME_FLAG_8D, 0);
    Flow::Return
}

/// Tareas fijas: cada frame borran el byte `field` de las entidades de los
/// dos jugadores. Original: `$032406` → `Player_ClearBoth8D_032ea4` y
/// `$03240C` → `Player_ClearBoth88_032eba`.
pub fn clear_players_frame_flag(m: &mut impl Machine, field: u32) -> Flow {
    for e in crate::layout::PLAYER_ENTITY {
        m.write_u8(e + field, 0);
    }
    Flow::Return
}

/// La tarea actual (`a6`) se da de baja a sí misma: suelta sus entradas de
/// sprite, deja de contar como marcada y pasa a [`finishing`].
/// Original: `$000518`.
pub fn finish(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    super::video::release_task_entries(m, t);
    let f = m.read_u8(t + T::FLAGS);
    m.write_u8(t + T::FLAGS, f & !T::FLAGS_MARKED);
    m.write_u32(t + T::HANDLER, routine::TASK_FINISHING);
    // Sigue en `finishing` (la rutina que acaba de ponerse).
    Flow::Goto(routine::TASK_FINISHING)
}

/// Rutina de una tarea que se está dando de baja: mientras le queden hijas
/// (la siguiente de la lista tiene más prioridad), espera a la vuelta
/// siguiente; si no, se quita de la lista y el planificador sigue por la
/// siguiente. Original: `$00052A`.
pub fn finishing(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let next = m.read_u32(t + T::NEXT);
    if m.read_u8(t + T::PRIORITY) < m.read_u8(next + T::PRIORITY) {
        return after_task(m);
    }
    let prev = unlink(m, t);
    m.set_task_register(prev);
    next_task(m)
}

/// El planificador sigue por la tarea que va detrás de `a6` (sin las
/// limpiezas de [`after_task`]). Original: `$0006E2`.
pub fn next_task(m: &mut impl Machine) -> Flow {
    let sp = m.read_u32(T::SAVED_SP);
    m.set_stack_pointer(sp);
    let t = m.task_register();
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
    fn kill_children_frees_descendants_only() {
        let mut m = TestMachine::default();
        m.write_u32(T::FREE_LIST, T::NIL);
        // Lista circular: padre(0) -> hija(1) -> nieta(2) -> otra(0).
        let [p, c, g, o] = [0x10_1000, 0x10_1100, 0x10_1200, 0x10_1300];
        for (t, prio, next, prev) in [(p, 0, c, o), (c, 1, g, p), (g, 2, o, c), (o, 0, p, g)] {
            m.write_u8(t + T::PRIORITY, prio);
            m.write_u32(t + T::NEXT, next);
            m.write_u32(t + T::PREV, prev);
            m.write_u32(t + T::HANDLER, 0x1234);
        }
        kill_children(&mut m, p);
        assert_eq!(m.read_u32(p + T::NEXT), o);
        assert_eq!(m.read_u32(o + T::PREV), p);
        assert_eq!(m.read_u32(T::FREE_LIST), g);
        assert_eq!(m.read_u32(g + T::NEXT), c);
        assert_eq!(m.read_u32(c + T::HANDLER), T::NIL);
    }

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
