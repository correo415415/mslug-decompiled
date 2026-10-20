//! Interrupciones: vblank y temporizador.
//!
//! Original: vector de vblank (`$0008F6`), `VBlankTick_Master_001E5E`,
//! `Deferred_RunQueue_001efe`, `Deferred_Push_001f4a`, `$000960`
//! (detección de 50 Hz) e interrupción del temporizador (`$0008D6`).

use super::{inc_u16, inc_u8, sound, video};
use crate::layout::{bios, bios_ram, hw, routine, system as S};
use crate::mem::{Flow, Machine, Memory, Regs};

/// Vblank. Mientras manda la BIOS (en el arranque, en el menú de la
/// recreativa) el vblank es suyo; con el juego en marcha se hace el trabajo
/// del frame, se deja a la BIOS leer mandos y monedas, y se restauran los
/// registros de vídeo. Original: `$0008F6`.
pub fn vblank(m: &mut impl Machine) -> Flow {
    if m.read_u8(bios_ram::SYSTEM_MODE) & bios_ram::SYSTEM_MODE_GAME == 0 {
        return Flow::Goto(bios::SYSTEM_INT1);
    }
    let saved = m.regs();
    vblank_tick(m);
    detect_pal(m);
    let vram_mod = m.read_u16(hw::VRAM_MOD);
    let mode = m.read_u16(S::LSPC_MODE_SHADOW);
    m.write_u16(hw::LSPC_MODE, mode & !hw::LSPC_MODE_TIMER_IRQ);
    m.call(bios::MESS_OUT);
    m.write_u16(hw::VRAM_MOD, vram_mod);
    m.call(bios::SYSTEM_IO);
    m.write_u16(hw::LSPC_MODE, mode);
    let addr = m.read_u16(S::VRAM_ADDR_SHADOW);
    m.write_u16(hw::VRAM_ADDR, addr);
    m.set_regs(saved);
    Flow::ReturnFromInterrupt
}

/// Trabajo de cada vblank: perro guardián, cola de sonido y, si el juego
/// ha terminado un frame, volcado de paleta, llamadas diferidas y sprites.
/// Original: `VBlankTick_Master_001E5E`.
pub fn vblank_tick(m: &mut impl Machine) {
    m.write_u16(hw::IRQ_ACK, hw::IRQ_ACK_VBLANK);
    inc_u8(m, S::STALL_VBLANKS);
    // Si el juego lleva más de 10 vblanks sin avanzar, se deja que el perro
    // guardián reinicie la máquina.
    if m.read_u8(S::STALL_VBLANKS) <= 10 {
        m.write_u8(hw::WATCHDOG, 0);
    }
    inc_u8(m, S::VBLANKS_THIS_FRAME);
    sound::pump(m);
    let delay = m.read_u8(S::VBLANK_DELAY);
    if delay != 0 {
        m.write_u8(S::VBLANK_DELAY, delay - 1);
        inc_u16(m, S::SKIPPED_VBLANKS);
        return;
    }
    // Ya se hizo el trabajo de este frame, o (tras el primero) aún no han
    // pasado dos vblanks: el juego va a 30 frames por segundo.
    let too_soon = m.read_u8(S::FRAME_STARTED) != 0 && m.read_u8(S::VBLANKS_THIS_FRAME) <= 1;
    if too_soon || m.read_u8(S::FRAME_READY) != 0 {
        inc_u16(m, S::SKIPPED_VBLANKS);
        return;
    }
    if m.read_u8(S::PAL_FLUSH_BLOCK) == 0 && m.read_u8(S::PAL_FLUSH_REQUEST) != 0 {
        m.write_u8(S::PAL_FLUSH_REQUEST, 0);
        video::flush_palettes(m);
    }
    run_deferred(m);
    video::flush_sprite_queues(m);
    m.write_u8(S::FRAME_READY, 1);
}

/// Ejecuta las llamadas que el juego ha dejado para el vblank (cada una con
/// su contexto en `a6`). Original: `Deferred_RunQueue_001efe`.
pub fn run_deferred(m: &mut impl Machine) {
    let vram_addr = m.read_u16(S::VRAM_ADDR_SHADOW);
    loop {
        let r = m.read_u8(S::DEFERRED_READ);
        if r == m.read_u8(S::DEFERRED_WRITE) {
            break;
        }
        let r = (r + 1) % S::DEFERRED_LEN;
        m.write_u8(S::DEFERRED_READ, r);
        let e = S::DEFERRED_QUEUE + u32::from(r) * 8;
        let ctx = m.read_u32(e + 4);
        let f = m.read_u32(e);
        let mut regs = m.regs();
        regs.a[6] = ctx;
        regs.a[0] = f;
        m.call_original(f, &mut regs);
        m.set_regs(regs);
    }
    m.write_u16(S::VRAM_ADDR_SHADOW, vram_addr);
}

/// Deja `routine` para el próximo vblank, con `ctx` como contexto. Si la
/// cola está llena, se pierde. Original: `Deferred_Push_001f4a`.
pub fn defer(m: &mut impl Memory, routine: u32, ctx: u32) {
    let w = (m.read_u8(S::DEFERRED_WRITE) + 1) % S::DEFERRED_LEN;
    if w == m.read_u8(S::DEFERRED_READ) {
        return;
    }
    let e = S::DEFERRED_QUEUE + u32::from(w) * 8;
    m.write_u32(e + 4, ctx);
    m.write_u32(e, routine);
    m.write_u8(S::DEFERRED_WRITE, w);
}

/// Anota si la máquina es de 50 Hz. Original: `$000960`.
pub fn detect_pal(m: &mut impl Memory) -> bool {
    let pal = m.read_u16(hw::LSPC_MODE) & hw::LSPC_MODE_PAL != 0;
    m.write_u8(S::IS_PAL, u8::from(pal));
    pal
}

/// Interrupción del temporizador: llama al gancho que haya puesto el juego.
/// Original: `$0008D6`.
pub fn timer(m: &mut impl Machine) -> Flow {
    m.write_u16(hw::IRQ_ACK, hw::IRQ_ACK_TIMER);
    let hook = m.read_u32(S::TIMER_IRQ_HOOK);
    if hook != routine::NOP {
        let saved = m.regs();
        let mut r: Regs = saved;
        m.call_original(hook, &mut r);
        // El original sólo guarda a0.
        r.a[0] = saved.a[0];
        m.set_regs(r);
    }
    Flow::ReturnFromInterrupt
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::TestMachine;

    #[test]
    fn deferred_queue_runs_in_order() {
        let mut m = TestMachine::default();
        defer(&mut m, 0x1000, 1);
        defer(&mut m, 0x2000, 2);
        run_deferred(&mut m);
        assert_eq!(m.calls, vec![0x1000, 0x2000]);
        run_deferred(&mut m);
        assert_eq!(m.calls.len(), 2);
    }

    #[test]
    fn bios_owns_vblank_until_game_starts() {
        let mut m = TestMachine::default();
        assert_eq!(vblank(&mut m), Flow::Goto(bios::SYSTEM_INT1));
        m.write_u8(bios_ram::SYSTEM_MODE, bios_ram::SYSTEM_MODE_GAME);
        assert_eq!(vblank(&mut m), Flow::ReturnFromInterrupt);
        assert_eq!(m.calls, vec![bios::MESS_OUT, bios::SYSTEM_IO]);
    }
}
