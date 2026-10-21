//! Comida (objetos de puntos que sueltan los prisioneros y las cajas): 20
//! variantes que sólo cambian el guion de animación, el gráfico y el tipo
//! de premio. Cae, se queda un rato, parpadea y desaparece; al tocarla un
//! jugador da puntos.
//!
//! Original: `Item_Tmpl294_Food_09aec4` (las 20 entradas), `Item_Food_Wait_09b0a8`,
//! `Item_Food_Blink_09b0d8`, `Item_Food_BlinkLoop_09b0de`,
//! `Item_LifetimeFromParam_09bb3c`, `Item_Free_09b474`.

use super::{anim, platforms, tasks};
use crate::layout::{food as L, task as T};
use crate::mem::{Flow, Machine, Memory};

/// Vida de un objeto: el parámetro `+$99` en unidades de 16 frames (0 =
/// casi infinita). Original: `Item_LifetimeFromParam_09bb3c`.
pub fn lifetime(m: &mut impl Memory, t: u32) -> u16 {
    match u16::from(m.read_u8(t + L::LIFE_PARAM)) << 4 {
        0 => L::FOREVER,
        n => n,
    }
}

/// Alta de la variante `k`. Original: entrada `k` de `Item_Tmpl294_Food_09aec4`.
pub fn start(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    let (kind, script, gfx) = L::VARIANTS[k];
    m.write_u8(t + L::KIND, kind);
    anim::start(m, t, script);
    if m.read_u8(t + L::NO_FLIP) != 0xFF {
        let f = m.read_u8(t + L::FLIP);
        m.write_u8(t + L::FLIP, f ^ 1);
    }
    anim::alloc_entry(m, t, gfx);
    m.write_u8(t + T::TIMER_B, L::START_DELAY);
    show(m, t);
    let life = lifetime(m, t);
    m.write_u16(t + L::TIMER, life);
    m.write_u32(t + T::HANDLER, L::WAIT);
    wait(m)
}

/// Atributos de dibujo de la comida (sin los bits 2–4 de `+$38`).
fn show(m: &mut impl Memory, t: u32) {
    anim::set_attr(m, t, L::ATTR);
    let a = m.read_u16(t + L::ATTR_WORD);
    m.write_u16(t + L::ATTR_WORD, a & L::ATTR_MASK);
}

/// Si la recoge un jugador, se da de baja; después, la cola común de los
/// objetos. Original: `Item_Food_PickupCheck_09b0c6`.
fn pickup_or_tail(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if super::pickup::food(m, t) {
        m.write_u32(t + T::HANDLER, L::FREE);
    }
    super::items::ground_tail(m)
}

/// Mientras dura: física y animación; al acabarse, a
/// parpadear. Original: `Item_Food_Wait_09b0a8`.
pub fn wait(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::item_physics(m, t);
    anim::step(m, t);
    let left = m.read_u16(t + L::TIMER).wrapping_sub(1);
    m.write_u16(t + L::TIMER, left);
    if left as i16 <= 0 {
        m.write_u32(t + T::HANDLER, L::BLINK);
    }
    pickup_or_tail(m)
}

/// Original: `Item_Food_Blink_09b0d8`.
pub fn blink(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u8(t + L::TIMER, L::BLINK_FRAMES);
    blink_start(m)
}

/// Original: `Item_Food_BlinkLoop_09b0de`.
pub fn blink_start(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    show(m, t);
    m.write_u32(t + T::HANDLER, L::BLINK_RUN);
    blink_run(m)
}

/// Anima sólo en frames impares (parpadeo) y desaparece al acabar.
/// Original: `$09B0FA`.
pub fn blink_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::item_physics(m, t);
    let n = m.read_u8(t + L::TIMER);
    if n & 1 != 0 {
        anim::step(m, t);
    }
    m.write_u8(t + L::TIMER, n.wrapping_sub(1));
    if n == 0 {
        m.write_u32(t + T::HANDLER, L::FREE);
    }
    pickup_or_tail(m)
}

/// Original: `Item_Free_09b474`.
pub fn free(m: &mut impl Machine) -> Flow {
    tasks::finish(m)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::TestMachine;

    #[test]
    fn lifetime_zero_is_forever() {
        let mut m = TestMachine::default();
        assert_eq!(lifetime(&mut m, 0x10_0A80), L::FOREVER);
        m.write_u8(0x10_0A80 + L::LIFE_PARAM, 3);
        assert_eq!(lifetime(&mut m, 0x10_0A80), 0x30);
    }
}
