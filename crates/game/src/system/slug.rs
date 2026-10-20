//! Slug (SV-001): la tarea fija del tanque y las tareas que lo sueltan en
//! la escena.
//!
//! Original: `Slug_IsAlive_02acfc`, `Slug_SpawnDrop_02ae50`,
//! `Slug_SpawnDropB_02aea4`, `Slug_Init_02a0f8` (con sus ayudantes) y la
//! entrada de `Slug_DropVariant0_02aef8` … `Slug_DropVariant4_02af88`.

use super::{anim, tasks};
use crate::layout::{slug as L, task as T};
use crate::mem::{Machine, Memory};
use crate::Flow;

/// ¿Está el Slug en marcha? (su tarea tiene una rutina que no es de libre,
/// baja, espera o destruido). Original: `Slug_IsAlive_02acfc` (C = sí).
pub fn is_alive(m: &mut impl Memory) -> bool {
    let h = m.read_u32(T::SLUG + T::HANDLER);
    !L::INACTIVE_HANDLERS.contains(&h)
}

/// Suelta el Slug (si no está ya en marcha) con la variante que pide el
/// byte `DROP_KIND` de la tarea actual, en su posición, y la tarea se da de
/// baja. Con `always_finish` (variante B) se da de baja aunque el Slug ya
/// estuviera. Original: `Slug_SpawnDrop_02ae50` / `Slug_SpawnDropB_02aea4`.
pub fn spawn_drop(m: &mut impl Machine, always_finish: bool) -> Flow {
    let t = m.task_register();
    if is_alive(m) {
        return if always_finish { tasks::finish(m) } else { Flow::Return };
    }
    let kind = m.read_u8(t + L::DROP_KIND).min(L::DROP_KIND_MAX);
    m.write_u32(T::SLUG + T::HANDLER, L::DROP_VARIANTS[kind as usize]);
    anim::copy_sprite_pos(m, t, T::SLUG);
    let y = m.read_u16(t + L::Y);
    m.write_u16(T::SLUG + L::DROP_Y, y);
    tasks::finish(m)
}

/// Prepara la tarea del Slug: gráficos, flags, temporizadores, daño a
/// cero, su tarea de cajas y la cadena de tres eslabones.
/// Original: `Slug_Init_02a0f8`.
pub fn init(m: &mut impl Memory, t: u32, gfx: u16) {
    anim::alloc_entry(m, t, gfx);
    anim::alloc_entry(m, t, L::EXTRA_GFX);
    m.write_u16(t + L::TIMER_1C, L::TIMER_1C_INIT);
    add_timer(m, t);
    let v = m.read_u16(t + L::FIELD_16);
    m.write_u16(t + L::FIELD_14, v);
    set_bits(m, t + L::FLAGS_12, 1 << 4);
    set_bits(m, t + L::FLAGS_5B, 1 << 2);
    set_bits(m, t + L::FLAGS_6B, 1 << 0);
    for l in L::LEVELS {
        m.write_u8(t + l, 0xFF);
    }
    m.write_u8(t + L::BYTE_3A, 0);
    m.write_u16(t + L::ATTR, L::ATTR_INIT);
    mark_timer_max(m, t);
    m.write_u8(t + L::BYTE_8C, 0);
    m.write_u32(t + L::HANDLER_60, L::HANDLER_60_INIT);
    mask_players(m, t);
    m.write_u16(t + L::FIELD_66, L::FIELD_66_INIT);
    reset_damage(m, t);
    for w in L::CLEAR_WORDS {
        m.write_u16(t + w, 0);
    }
    for b in L::CLEAR_BYTES {
        m.write_u8(t + b, 0);
    }
    m.write_u8(t + L::BYTE_90, L::BYTE_90_INIT);
    let f = m.read_u8(t + L::FLAGS_13);
    m.write_u8(t + L::FLAGS_13, f & !(1 << 3));
    let shadow = tasks::spawn(m, L::SHADOW_TASK, t);
    anim::copy_sprite_pos(m, t, shadow);
    build_chain(m, t);
    m.write_u32(t + L::FIELD_98_LONG, L::FIELD_98_INIT);
}

fn set_bits(m: &mut impl Memory, a: u32, bits: u8) {
    let v = m.read_u8(a);
    m.write_u8(a, v | bits);
}

/// Suma `$74` al temporizador `+$1C`. Original: `Entity_AddTimer0_74_0138FE`.
pub fn add_timer(m: &mut impl Memory, t: u32) {
    let v = m.read_u16(t + L::TIMER_1C).wrapping_add(L::TIMER_1C_ADD);
    m.write_u16(t + L::TIMER_1C, v);
}

/// Marca el bit 2 de `+$12` y pone el temporizador `+$30` al máximo.
/// Original: `Entity_MarkFlag2TimerMax_05e98a`.
pub fn mark_timer_max(m: &mut impl Memory, t: u32) {
    set_bits(m, t + L::FLAGS_12, 1 << 2);
    m.write_u16(t + L::TIMER_30, 0xFFFF);
}

/// Máscara de jugadores por defecto. Original: `PlayerSlot_MaskF0F0_08f6d2`.
pub fn mask_players(m: &mut impl Memory, t: u32) {
    m.write_u16(t + L::PLAYER_MASK, L::PLAYER_MASK_INIT);
}

/// Daño a cero. Original: `Slug_ResetDamageIdx_02fada`.
pub fn reset_damage(m: &mut impl Memory, t: u32) {
    m.write_u8(t + L::DAMAGE_IDX, 0);
    m.write_u16(t + L::DAMAGE, 0);
}

/// Crea tres tareas hijas a la altura `DROP_Y` y las enlaza en círculo
/// (cada una apunta a las tres). Devuelve la primera.
/// Original: `Entity_Build3ChainCircular_03060A`.
pub fn build_chain(m: &mut impl Memory, t: u32) -> u32 {
    for (routine, field) in L::CHAIN {
        let c = tasks::spawn(m, routine, t);
        anim::copy_sprite_pos(m, t, c);
        m.write_u32(t + field, c);
        let y = m.read_u16(t + L::DROP_Y);
        m.write_u16(c + L::Y, y);
        m.write_u8(c + L::CHAIN_KIND, 0);
    }
    // Enlaces: +$70 → segundo, +$74 → tercero, +$78 → primero.
    let links = [m.read_u32(t + 0x74), m.read_u32(t + 0x78), m.read_u32(t + 0x7C)];
    for c in [links[0], links[1], links[2]] {
        for (field, to) in L::CHAIN_LINKS.iter().zip(links) {
            m.write_u32(c + field, to);
        }
    }
    links[2]
}

/// Quita los bits 1 y 2 de `+$13`. Original: `Entity_ClearFlags13Bits12`.
pub fn clear_flags13(m: &mut impl Memory, t: u32) {
    let f = m.read_u8(t + L::FLAGS_13);
    m.write_u8(t + L::FLAGS_13, f & !0b110);
}

/// Entrada en escena con la variante `k` (0–4): prepara el Slug, su tarea
/// acompañante y las comunes, el guion y las cajas, y sigue en el bucle de
/// la caída (aún el original). Original: `Slug_DropVariant0_02aef8` …
/// `Slug_DropVariant4_02af88`.
pub fn drop_in(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    init(m, t, L::DROP_GFX);
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, a | L::ATTR_DROP);
    let c = tasks::spawn(m, L::DROP_COMPANIONS[k], t);
    anim::copy_sprite_pos(m, t, c);
    tasks::spawn(m, L::DROP_TASKS[0], t);
    let c = tasks::spawn(m, L::DROP_TASKS[1], t);
    anim::copy_sprite_pos(m, t, c);
    anim::start(m, t, L::DROP_ANIM);
    for w in L::DROP_CLEAR_WORDS {
        m.write_u16(t + w, 0);
    }
    for b in L::DROP_CLEAR_BYTES {
        m.write_u8(t + b, 0);
    }
    m.write_u32(t + L::HITBOX, L::DROP_HITBOX);
    clear_flags13(m, t);
    m.write_u32(t + T::HANDLER, L::DROP_FALL);
    Flow::Goto(L::DROP_FALL)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::TestRam;

    #[test]
    fn alive_unless_inactive_handler() {
        let mut m = TestRam::default();
        for h in L::INACTIVE_HANDLERS {
            m.write_u32(T::SLUG + T::HANDLER, h);
            assert!(!is_alive(&mut m));
        }
        m.write_u32(T::SLUG + T::HANDLER, L::DROP_VARIANTS[0]);
        assert!(is_alive(&mut m));
    }
}
