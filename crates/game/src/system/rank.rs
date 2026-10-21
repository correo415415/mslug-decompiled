//! Tiempo de la misión: el contador de dos cifras de la capa fija, su
//! cuenta atrás (un segundo cada 128 frames, con pitido al final) y
//! el aviso de «TIME UP».
//!
//! Original: `Rank_DelayStart_001b4c` y siguientes, `$047482`,
//! `$047656`, `$079970`, `$02F85A`.

use super::{sound, tasks};
use crate::layout::{bios_ram, pads as P, rank as R, task as T};
use crate::mem::{Flow, Machine, Memory};

/// Tiempo inicial según el ajuste de dificultad de la BIOS.
/// Original: `$079970`.
pub fn start_time(m: &mut impl Memory) -> u16 {
    let k = u32::from(m.read_u8(R::BIOS_SETTING));
    m.read_u16(R::START_TIMES + k * 2)
}

/// Decenas y unidades de `n` (máximo 99). Original: `$047656`.
pub fn digits(n: u16) -> (u16, u16) {
    let n = n.min(R::MAX);
    (n / 10, n % 10)
}

/// Escribe en la capa fija las dos cifras (cuatro caracteres cada una: dos
/// columnas por dos filas) con los primeros caracteres `tens`, `units`.
fn draw_digits(m: &mut impl Memory, tens: u16, units: u16) {
    for (at, ch) in [(R::AT[0], tens), (R::AT[1], units)] {
        let mut a = at;
        let mut c = ch;
        for k in 0..4 {
            m.write_u16(crate::layout::hw::VRAM_ADDR, a);
            m.write_u16(crate::layout::hw::VRAM_RW, c);
            // Orden del original: (0,0) (1,0) (0,1) (1,1).
            if k % 2 == 0 {
                a = a.wrapping_add(R::COLUMN);
                c = c.wrapping_add(1);
            } else {
                a = a.wrapping_sub(R::COLUMN - 1);
                c = c.wrapping_add(R::ROW_STEP - 1);
            }
        }
    }
}

/// Dibuja el tiempo; sin `show` (`d1` = 0 en el original) dibuja
/// espacios (parpadeo del final); con más de 99 dibuja «∞». En modo título no
/// dibuja nada. Original: `$047482`.
pub fn draw(m: &mut impl Memory, show: bool) {
    if m.read_u8(bios_ram::USER_MODE) == bios_ram::USER_MODE_ATTRACT {
        return;
    }
    let time = m.read_u16(R::TIME);
    if time > R::MAX {
        draw_digits(m, R::INFINITE.0, R::INFINITE.1);
        return;
    }
    if !show {
        draw_digits(m, R::BLANK, R::BLANK);
        return;
    }
    let (t, u) = digits(time);
    let tens = m.read_u16(R::DIGITS + u32::from(t) * 2);
    let units = m.read_u16(R::DIGITS + u32::from(u) * 2);
    draw_digits(m, tens, units);
}

/// Algún jugador está jugando. Original: `$001CAC`
/// (con C a 0).
fn player_starting(m: &mut impl Memory) -> bool {
    P::BIOS_PLAYER_STATE.iter().any(|&a| m.read_u8(a) == P::BIOS_PLAYER_PLAYING)
}

/// Alguno de los dos jugadores tiene el bit 3 de `+$8D`. Original: `$02F85A`.
pub fn player_flag3(m: &mut impl Memory) -> bool {
    R::PLAYER_TASKS.iter().any(|&t| m.read_u8(t + R::PLAYER_FLAGS) & R::PLAYER_FLAG3 != 0)
}

/// Pone el tiempo inicial y lo dibuja; espera `START_DELAY` frames.
/// Original: `Rank_DelayStart_001b4c`.
pub fn task(m: &mut impl Machine) -> Flow {
    let time = start_time(m);
    m.write_u16(R::TIME, time);
    draw(m, true);
    let t = m.task_register();
    m.write_u16(t + R::DELAY, R::START_DELAY);
    m.write_u32(t + T::HANDLER, R::DELAY_RUN);
    Flow::Return
}

/// Original: `Rank_DelayTick_001b70`.
pub fn delay(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let n = m.read_u16(t + R::DELAY).wrapping_sub(1);
    m.write_u16(t + R::DELAY, n);
    if n == 0 {
        m.write_u32(t + T::HANDLER, R::RUN);
    }
    Flow::Return
}

/// Cuenta atrás: un segundo cada 128 frames (cada 16 con 10 o menos, con
/// un pitido por segundo); al llegar a 0 crea el aviso de «TIME UP» y
/// espera a que acabe. Original: `Rank_DelayGate_001b80`.
pub fn run(m: &mut impl Machine) -> Flow {
    if !player_starting(m)
        || m.read_u8(R::SCENE_LOADED) == 0
        || player_flag3(m)
        || m.read_u8(R::SCRIPTED_PAUSE) != 0
    {
        return Flow::Return;
    }
    let mut show = true;
    let frame = m.read_u8(R::FRAME);
    let time = m.read_u16(R::TIME);
    if (time as i16) <= R::HURRY as i16 && frame & R::HURRY_MASK == R::HURRY_MASK {
        sound::play(m, R::SOUND_TICK);
        show = false;
    }
    let time = m.read_u16(R::TIME);
    if time < R::COUNTING_BELOW {
        if frame & 0xF != 0 {
            return Flow::Return;
        }
        if frame & 0x7F == 0 {
            m.write_u16(R::TIME, time.wrapping_sub(1));
        }
    }
    if (m.read_u16(R::TIME) as i16) <= 0 {
        let t = m.task_register();
        m.write_u8(t + R::WAITING, 0xFF);
        tasks::spawn(m, R::TIME_UP, t);
        m.write_u16(R::TIME, 0);
        show = false;
        m.write_u32(t + T::HANDLER, R::WAIT_RUN);
    }
    draw(m, show);
    Flow::Return
}

/// Espera a que el aviso termine y vuelve a empezar. Original:
/// `Timer_WaitFlag21_001c34`.
pub fn wait(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if m.read_u8(t + R::WAITING) == 0 {
        m.write_u32(t + T::HANDLER, R::TASK);
    }
    Flow::Return
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digits_cap_at_99() {
        assert_eq!(digits(57), (5, 7));
        assert_eq!(digits(200), (9, 9));
        assert_eq!(digits(0), (0, 0));
    }
}
