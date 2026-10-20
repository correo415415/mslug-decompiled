//! Bucle de frame: la tarea que marca el ritmo del juego.
//!
//! Es la primera tarea de la lista. Espera a que el vblank haya hecho su
//! trabajo (paleta, sprites) y entonces lee los mandos, mira la pausa y
//! prepara el frame siguiente; después el planificador pasa al resto de
//! tareas. Original: `VBlank_FrameLoop_001f84`.

use super::{inc_u16, inc_u8, pads, palette, sound, tasks, video};
use crate::layout::{bios_ram, hw, routine, system as S, task as T};
use crate::mem::{Flow, Machine};

/// Preparación (una sola vez): pone a cero los contadores del vblank, da
/// el vblank al juego y deja la espera como rutina de la tarea.
/// Original: `VBlank_FrameLoop_001f84`.
pub fn start(m: &mut impl Machine) -> Flow {
    m.write_u8(S::FRAME_PARAM_A, 0);
    m.write_u8(S::FRAME_PARAM_B, 3);
    let t = m.task_register();
    for off in [T::FLAGS, T::FLAGS2, T::FLAGS5, T::FLAGS3] {
        m.write_u8(t + off, 0);
    }
    for a in [
        S::DEFERRED_READ,
        S::DEFERRED_WRITE,
        S::VBLANK_DELAY,
        S::PAL_FLUSH_REQUEST,
        S::PAL_FLUSH_BLOCK,
        S::STALL_VBLANKS,
        S::FRAME_READY,
        S::FRAME_FLAG,
    ] {
        m.write_u8(a, 0);
    }
    let mode = m.read_u8(bios_ram::SYSTEM_MODE);
    m.write_u8(bios_ram::SYSTEM_MODE, mode | bios_ram::SYSTEM_MODE_GAME);
    m.write_u32(t + T::HANDLER, routine::FRAME_WAIT);
    let mut r = m.regs();
    r.a[1] = routine::FRAME_WAIT;
    m.set_regs(r);
    Flow::Goto(routine::FRAME_WAIT)
}

/// Una vuelta de la espera del frame. Original: `$001FE2`.
pub fn wait(m: &mut impl Machine) -> Flow {
    inc_u16(m, S::IDLE_SPINS);
    m.write_u8(S::STALL_VBLANKS, 0);
    let too_soon = m.read_u8(S::FRAME_STARTED) != 0 && m.read_u8(S::VBLANKS_THIS_FRAME) <= 1;
    if too_soon || m.read_u8(S::FRAME_READY) == 0 {
        return Flow::Wait;
    }
    m.write_u8(S::VBLANKS_THIS_FRAME, 0);
    pads::read_frame(m);
    if pads::poll_pause(m) {
        // En pausa: el frame no avanza.
        m.write_u16(S::SKIPPED_VBLANKS, 0);
        m.write_u8(S::FRAME_READY, 0);
        return Flow::Wait;
    }
    let reply = sound::poll_reply(m);
    m.write_u8(S::SOUND_REPLY_FRAME, reply);
    inc_u8(m, S::FRAME_COUNTER);
    let flag = m.read_u8(S::FRAME_FLAG);
    m.write_u8(S::FRAME_FLAG_PREV, flag);
    m.write_u8(S::FRAME_FLAG, 0);
    tasks::store_active_players(m);
    let marked = m.read_u16(T::MARKED);
    m.write_u16(T::MARKED_PREV, marked);
    m.write_u16(T::MARKED, 0);
    m.write_u16(T::RUN_COUNT, 0);
    m.write_u8(S::FRAME_STARTED, 1);
    let a6 = m.task_register();
    video::begin_draw_frame(m);
    let spins = m.read_u16(S::IDLE_SPINS);
    m.write_u16(S::IDLE_SPINS, 0);
    m.write_u16(S::SKIPPED_VBLANKS, 0);
    let mut r = m.regs();
    r.d[0] = u32::from(spins);
    r.a[6] = a6;
    m.set_regs(r);
    Flow::Return
}

/// Tarea del render (la última de la lista): actualiza las paletas (sin
/// que el vblank las vuelque a medias), pide volcarlas, prepara los
/// sprites y deja el frame a la espera del vblank.
/// Original: `VBlank_RenderFrame_002098`.
pub fn render(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    for off in [T::FLAGS, T::FLAGS2, T::FLAGS5, T::FLAGS3] {
        m.write_u8(t + off, 0);
    }
    m.write_u8(S::PAL_FLUSH_BLOCK, 0xFF);
    palette::update_all(m);
    m.write_u8(hw::NO_SHADOW, 0xFF);
    m.write_u8(S::PAL_FLUSH_REQUEST, 0xFF);
    m.write_u8(S::PAL_FLUSH_BLOCK, 0);
    let mut r = m.regs();
    m.call_original(routine::SPRITE_SORT_RENDER, &mut r);
    m.set_regs(r);
    m.write_u8(hw::NO_SHADOW, r.d[0] as u8);
    m.set_task_register(t);
    m.write_u8(S::FRAME_READY, 0);
    Flow::Return
}
