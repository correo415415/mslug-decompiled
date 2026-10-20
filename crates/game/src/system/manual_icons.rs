//! Iconos del manual de juego: tareas hijas que crea `manual::start`.
//!
//! - La **base** (el mando) cambia de guion según la dirección que mantiene
//!   la demostración (nibble bajo del byte `+$92` de la madre).
//! - Cuatro **ranuras** (botones A–D): al pulsarse su bit en `+$93` se
//!   hunden y al soltarlo en `+$92` vuelven; cada una tiene una hija
//!   «encendida» que sólo anima.
//! - El **ancla** del texto elige el idioma según la BIOS y deja listo el
//!   cursor de texto; su bucle (`$08C678`) todavía es el original.
//!
//! Original: `Icon_Base_08c2b8` … `Icon_Anchor_Init_08c5b2` y `$0267E2`.

use super::{anim, hud, tasks};
use crate::layout::{manual_icons as L, task as T};
use crate::mem::{Machine, Memory};
use crate::Flow;

/// Atributos, gráficos, guion y posición; lo común a todos los iconos.
fn place(m: &mut impl Memory, t: u32, attr: u16, gfx: u16, script: u32, pos: (u16, u16)) {
    m.write_u16(t + L::ATTR, attr);
    let f = m.read_u8(t + L::FLAGS_12);
    m.write_u8(t + L::FLAGS_12, f | L::FLAGS_12_SET);
    anim::alloc_entry(m, t, gfx);
    anim::start(m, t, script);
    m.write_u16(t + L::X, pos.0);
    m.write_u16(t + L::Y, pos.1);
    m.write_u16(t + L::Z, 0);
}

/// Alta de la base y primera vuelta de su bucle. Original: `Icon_Base_08c2b8`.
pub fn base(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    place(m, t, L::ATTR_ICON, L::BASE_GFX, L::BASE_ANIM, (L::BASE_X, L::ROW_Y));
    m.write_u32(t + T::HANDLER, L::BASE_RUN);
    base_run(m)
}

/// Cada frame: si la dirección de la madre tiene guion, lo reempieza; y
/// anima. Original: `$08C2F0` (dentro de `Icon_Base_08c2b8`).
pub fn base_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let mother = m.read_u32(t + T::OWNER);
    let dir = u32::from(m.read_u8(mother + L::MOTHER_HELD) & 0x0F);
    let script = m.read_u32(L::BASE_TABLE + dir * 4);
    if script != L::NO_ANIM {
        anim::start(m, t, script);
    }
    animate(m)
}

/// Alta de la ranura `k` (0–3): su hija encendida y su bit. La cuarta borra
/// antes un trozo de la capa fija y entra ya en el bucle; las otras sólo
/// animan este frame. Original: `Icon_Slot1_08c322` … `Icon_Slot4_08c436`.
pub fn slot(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    let (_, gfx, x, lit, _, bit) = L::SLOTS[k];
    if k == 3 {
        let (at, ch, cols, rows) = L::SLOT4_TILES;
        hud::fill_tiles(m, at, ch, cols, rows);
    }
    place(m, t, L::ATTR_ICON, gfx, L::SLOT_ANIM, (x, L::ROW_Y));
    let child = tasks::spawn(m, lit, t);
    anim::copy_sprite_pos(m, t, child);
    m.write_u8(t + L::SLOT_BIT, bit);
    m.write_u16(t + L::SLOT_DOWN, 0);
    m.write_u32(t + T::HANDLER, L::SLOT_RUN);
    if k == 3 {
        slot_run(m)
    } else {
        animate(m)
    }
}

/// Cada frame de una ranura: se hunde al pulsarse su botón y vuelve al
/// soltarlo. Original: `Icon_Slot_Run_08c4a0` + `Icon_Slot_Tail_08c4ca`.
pub fn slot_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let bit = m.read_u8(t + L::SLOT_BIT);
    let mother = m.read_u32(t + T::OWNER);
    // El original lee después `+$92` a través de a0, que tras pulsar ya no
    // apunta a la madre sino al guion recién empezado.
    let mut held_from = mother;
    if m.read_u8(mother + L::MOTHER_NEW) & bit != 0 {
        m.write_u16(t + L::SLOT_DOWN, 1);
        anim::start(m, t, L::SLOT_PRESSED);
        held_from = L::SLOT_PRESSED;
    }
    if m.read_u8(held_from + L::MOTHER_HELD) & bit == 0 && m.read_u16(t + L::SLOT_DOWN) == 1 {
        m.write_u16(t + L::SLOT_DOWN, 0);
        anim::start(m, t, L::SLOT_RELEASED);
    }
    animate(m)
}

/// Alta de la versión encendida de la ranura `k` (hereda la posición de la
/// madre). Original: `Icon_Slot1_Lit_08c4fa` … `Icon_Slot4_Lit_08c57e`.
pub fn slot_lit(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    let (_, gfx, _, _, script, _) = L::SLOTS[k];
    m.write_u16(t + L::ATTR, L::ATTR_ICON);
    let f = m.read_u8(t + L::FLAGS_12);
    m.write_u8(t + L::FLAGS_12, f | L::FLAGS_12_SET);
    anim::alloc_entry(m, t, gfx);
    anim::start(m, t, script);
    m.write_u32(t + T::HANDLER, L::LIT_RUN);
    animate(m)
}

/// Sólo anima. Original: `$08C5AA` (y los finales `$08C31A`, `$08C4F2`).
pub fn animate(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::step(m, t);
    Flow::Return
}

/// Idioma del texto según la BIOS: Japón → japonés; Europa → el idioma
/// elegido (inglés, portugués o español; `None` si es otro, y el original
/// no toca el campo); el resto → inglés.
pub fn language(m: &mut impl Memory) -> Option<u8> {
    match m.read_u8(L::BIOS_REGION) {
        L::REGION_JAPAN => Some(L::LANG_JP),
        L::REGION_EUROPE => match m.read_u8(L::BIOS_LANGUAGE) {
            0 => Some(L::LANG_EN),
            1 => Some(L::LANG_PT),
            2 => Some(L::LANG_ES),
            _ => None,
        },
        _ => Some(L::LANG_EN),
    }
}

/// Alta del ancla del texto: idioma, sprite, movimiento a cero y cursor de
/// texto al principio. Original: `Icon_Anchor_Init_08c5b2`.
pub fn anchor(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::ATTR, L::ATTR_ANCHOR);
    if let Some(lang) = language(m) {
        m.write_u8(t + L::LANGUAGE, lang);
    }
    anim::alloc_entry(m, t, L::ANCHOR_GFX);
    anim::start(m, t, L::ANCHOR_ANIM);
    m.write_u16(t + L::X, L::ANCHOR_POS.0);
    m.write_u16(t + L::Y, L::ANCHOR_POS.1);
    m.write_u16(t + L::Z, 0);
    stop_motion(m, t);
    m.write_u32(t + L::MOVE_FIELD, 0);
    m.write_u32(t + L::TEXT_AT, L::TEXT_AT_INIT);
    m.write_u16(t + L::TEXT_TIMER, 0);
    m.write_u16(t + L::TEXT_CHAR, 0);
    m.write_u32(t + L::TEXT_OFFSET, 0);
    m.write_u32(t + L::TEXT_SCRIPT, L::TEXT_FIRST);
    m.write_u16(t + L::TEXT_PAGE, 0);
    m.write_u32(t + T::HANDLER, L::ANCHOR_RUN);
    Flow::Return
}

/// Pone a cero las velocidades de la tarea. Original: `$0267E2`.
pub fn stop_motion(m: &mut impl Memory, t: u32) {
    for w in L::MOTION_WORDS {
        m.write_u16(t + w, 0);
    }
    m.write_u32(t + L::MOTION_LONG, 0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::TestRam;

    #[test]
    fn language_follows_bios_region() {
        let mut m = TestRam::default();
        m.write_u8(L::BIOS_REGION, L::REGION_JAPAN);
        assert_eq!(language(&mut m), Some(L::LANG_JP));
        m.write_u8(L::BIOS_REGION, 1);
        assert_eq!(language(&mut m), Some(L::LANG_EN));
        m.write_u8(L::BIOS_REGION, L::REGION_EUROPE);
        m.write_u8(L::BIOS_LANGUAGE, 2);
        assert_eq!(language(&mut m), Some(L::LANG_ES));
        m.write_u8(L::BIOS_LANGUAGE, 7);
        assert_eq!(language(&mut m), None);
    }

    #[test]
    fn stop_motion_clears_speeds() {
        let mut m = TestRam::default();
        let t = 0x10_0500;
        for o in 0x26..0x30 {
            m.write_u8(t + o, 0xAA);
        }
        stop_motion(&mut m, t);
        for o in 0x26..0x30 {
            assert_eq!(m.read_u8(t + o), 0);
        }
    }
}
