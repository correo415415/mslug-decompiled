//! Arranque: lo que pide la BIOS al juego y la inicialización de todo.
//!
//! La BIOS llama a la entrada `USER` de la cabecera con el tipo de petición
//! en `USER_REQUEST`: 0 = iniciar (el juego se prepara y devuelve el
//! control), 1 = modo de prueba, 2 = demo y 3 = título (en las dos últimas
//! el juego arranca su planificador y ya no vuelve).
//!
//! Original: `$0007CC` (`USER`), `$00080C`, `$000832`, `$000836`,
//! `$00085E`/`$000868`, `$00097A`, `Init_MasterSubsystems_0020E2`,
//! `TitleModeInit_024E38`, `HiScore_LoadDefaults_09773c`,
//! `AttractInit_Single_099AE2`, `Global_Clear10E486_099AFC`,
//! `Global_SetDualFlagFrom10FD82_000FFE`, `Pause_Clear_013d12`,
//! `Input_RisingEdgeSnapshot_05CC0E`, `PlayerCtx_ResetAndSetRepeat1002_024fb8`,
//! `PlayerCtx_ResetTwoBlocks_024FEC`.

use super::{copy, fill, sound, tasks, video};
use crate::layout::{bios, bios_ram, hw, pads, pause, routine, system as S};
use crate::mem::{Flow, Machine, Memory};

/// Petición `USER` de la BIOS. Original: `$0007CC`.
pub fn user_request(m: &mut impl Machine) -> Flow {
    m.write_u32(S::TIMER_IRQ_HOOK, routine::NOP);
    let mode = m.read_u8(bios_ram::SYSTEM_MODE);
    m.write_u8(bios_ram::SYSTEM_MODE, mode & !bios_ram::SYSTEM_MODE_GAME);
    m.write_u16(hw::IRQ_ACK, hw::IRQ_ACK_ALL);
    m.set_interrupt_mask(0);
    let req = u32::from(m.read_u8(bios_ram::USER_REQUEST));
    let target = m.read_u32(routine::USER_REQUEST_TABLE + req * 4);
    let mut r = m.regs();
    r.d[0] = req * 4;
    r.a[0] = target;
    m.set_regs(r);
    Flow::Goto(target)
}

/// Petición 0 (iniciar): deja preparado el título y vuelve a la BIOS.
/// Original: `$00080C`.
pub fn user_init(m: &mut impl Machine) -> Flow {
    title_mode_init(m);
    // El original distingue el arranque en frío, pero en los dos casos
    // deja el mismo valor.
    m.write_u16(S::RAM_FIRST_WORD, 0);
    return_to_bios(m)
}

/// Petición 1 (modo de prueba): el juego no tiene; vuelve a la BIOS.
/// Original: `$000832`.
pub fn user_test(m: &mut impl Machine) -> Flow {
    return_to_bios(m)
}

/// Peticiones 2 y 3 (demo y título): inicializa todo y arranca el
/// planificador; no vuelve. Original: `$000836` / `$000840`.
pub fn user_start_game(m: &mut impl Machine) -> Flow {
    init_subsystems(m);
    tasks::run(m)
}

/// Quita el vblank al juego, apaga la pantalla y borra la RAM del juego
/// antes de devolver el control a la BIOS. Original: `$00085E` (+ `$000868`).
pub fn return_to_bios(m: &mut impl Memory) -> Flow {
    let mode = m.read_u8(bios_ram::SYSTEM_MODE);
    m.write_u8(bios_ram::SYSTEM_MODE, mode & !bios_ram::SYSTEM_MODE_GAME);
    m.write_u8(hw::PAL_BANK0, 0);
    m.write_u16(hw::BACKDROP_COLOR, 0);
    m.write_u8(hw::PAL_BANK1, 0);
    m.write_u16(hw::BACKDROP_COLOR, 0);
    fill(m, S::GAME_RAM, S::GAME_RAM_LEN, 0);
    Flow::Goto(bios::SYSTEM_RETURN)
}

/// Inicializa todos los subsistemas. Original: `$00097A`.
pub fn init_subsystems(m: &mut impl Machine) {
    init_master(m);
    m.write_u32(S::TIMER_IRQ_HOOK, routine::NOP);
    m.call(bios::FIX_CLEAR);
    sound::init_queue(m);
    m.write_u16(S::LSPC_MODE_SHADOW, 0);
    if m.read_u8(bios_ram::MVS_FLAG) != 0 {
        m.write_u8(S::ATTRACT_FLAG, 0);
    }
}

/// Paleta, pausa, VRAM, tareas y mandos, alimentando al perro guardián
/// entre cada paso. Original: `Init_MasterSubsystems_0020E2`.
pub fn init_master(m: &mut impl Machine) {
    m.write_u8(hw::WATCHDOG, 0);
    video::clear_palettes(m);
    m.write_u8(hw::WATCHDOG, 0);
    clear_pause(m);
    m.write_u8(hw::WATCHDOG, 0);
    video::clear_vram(m);
    m.write_u8(hw::WATCHDOG, 0);
    tasks::init(m);
    m.write_u8(hw::WATCHDOG, 0);
    reset_pads(m);
    set_dual_flag(m);
}

/// Original: `Pause_Clear_013d12`.
pub fn clear_pause(m: &mut impl Memory) {
    m.write_u8(pause::ACTIVE, 0);
    m.write_u8(pause::ENABLED, 0);
}

/// Mandos a cero, sin demo, repetición por defecto.
/// Original: `Input_RisingEdgeSnapshot_05CC0E` →
/// `PlayerCtx_ResetAndSetRepeat1002_024fb8` → `PlayerCtx_ResetTwoBlocks_024FEC`.
pub fn reset_pads(m: &mut impl Memory) {
    m.write_u16(pads::SNAPSHOT_FLAGS, 0);
    for p in pads::WORK {
        fill(m, p, 6, 0);
    }
    m.write_u8(pads::DEMO_MODE, 0);
    m.write_u16(pads::REPEAT, pads::REPEAT_DEFAULT);
}

/// Original: `Global_SetDualFlagFrom10FD82_000FFE`.
pub fn set_dual_flag(m: &mut impl Memory) {
    let v = if m.read_u8(bios_ram::MVS_FLAG) != 0 { 0 } else { 4 };
    for a in S::DUAL_FLAG {
        m.write_u8(a, v);
    }
}

/// Prepara el título: récords por defecto y, en consola, los ajustes por
/// defecto. Original: `TitleModeInit_024E38`.
pub fn title_mode_init(m: &mut impl Memory) {
    sound::cancel_coin_sound(m);
    m.write_u8(S::TITLE_FLAG_A, 1);
    m.write_u8(S::TITLE_FLAG_B, 1);
    m.write_u8(S::TITLE_FLAG_C, 0);
    load_default_hiscores(m);
    if m.read_u8(bios_ram::MVS_FLAG) == 0 {
        m.write_u8(bios_ram::SOFT_DIP_LIVES, 3);
        m.write_u8(bios_ram::SOFT_DIP_DIFFICULTY, 2);
    }
    m.write_u8(S::ATTRACT_FLAG, 0);
}

/// Original: `HiScore_LoadDefaults_09773c`.
pub fn load_default_hiscores(m: &mut impl Memory) {
    copy(m, S::DEFAULT_HISCORES, S::HISCORES, S::HISCORES_LEN);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::TestMachine;

    #[test]
    fn user_request_jumps_by_table() {
        let mut m = TestMachine::default();
        m.write_u32(routine::USER_REQUEST_TABLE + 12, 0x840);
        m.write_u8(bios_ram::USER_REQUEST, 3);
        m.write_u8(bios_ram::SYSTEM_MODE, 0x80);
        assert_eq!(user_request(&mut m), Flow::Goto(0x840));
        assert_eq!(m.read_u8(bios_ram::SYSTEM_MODE), 0);
        assert_eq!(m.irq_mask, 0);
    }
}
