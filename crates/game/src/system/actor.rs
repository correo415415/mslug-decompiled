//! Objetos en pantalla: una tarea con posición, una entrada de sprite
//! (gráficos), atributos de sprite y una animación por guion.
//!
//! Original: `Entity_AllocSpriteSlot_00236E` (+ `$0029A8`, `$00296A`),
//! `Fn_00028134` + `Entity_ApplyFadeShade_028108`, `Sub_00028CD4`,
//! `ThunkTarget_05dd02`.

use super::inc_u16;
use crate::layout::{sprites as S, task as T};
use crate::mem::Memory;

fn entry_addr(i: u16) -> u32 {
    S::TABLE + u32::from(i) * S::SIZE
}

/// Dirección de los gráficos `gfx` en la ROM (lo que guarda la entrada).
fn gfx_data(gfx: u16) -> u32 {
    u32::from(gfx) * 64 + S::LOW_LIST_DATA
}

/// La tarea ya tiene una entrada con esos gráficos: la hace la actual.
/// Original: `$0029A8`.
fn reuse_own_entry(m: &mut impl Memory, t: u32, data: u32) -> bool {
    let mut n = m.read_u16(t + T::PAL_SLOT_COUNT);
    while n != 0 {
        n -= 1;
        let slot = t + T::PAL_SLOTS + u32::from(n) * 2;
        let i = m.read_u16(slot);
        if m.read_u32(entry_addr(i) + S::ENTRY_DATA) == data && m.read_u8(t) & S::TASK_NO_REUSE == 0 {
            m.write_u16(t + T::SPRITE_ENTRY, i);
            return true;
        }
    }
    false
}

/// Da a la tarea `t` una entrada de sprite con los gráficos `gfx`: la suya
/// si ya la tenía; si no, una compartida con los mismos gráficos o una
/// libre (como mucho `TASK_MAX_ENTRIES`). Original: `Entity_AllocSpriteSlot_00236E`.
pub fn use_sprite(m: &mut impl Memory, t: u32, gfx: u16) {
    let data = gfx_data(gfx);
    if reuse_own_entry(m, t, data) {
        return;
    }
    if m.read_u16(t + T::PAL_SLOT_COUNT) as i16 >= S::TASK_MAX_ENTRIES as i16 || m.read_u16(S::FREE_TOP) == 0 {
        return;
    }
    let shared = (0..S::SEARCH_COUNT).find(|&k| {
        let e = entry_addr(S::FREE_INDEX_BASE + k);
        m.read_u32(e + S::ENTRY_DATA) == data && m.read_u8(e) & S::NOT_SHARED == 0
    });
    let k = match shared {
        Some(k) => k,
        None => {
            // Una libre del anillo.
            let top = m.read_u16(S::FREE_TOP).wrapping_sub(1);
            m.write_u16(S::FREE_TOP, top);
            let r = m.read_u16(S::FREE_READ);
            let r2 = r.wrapping_add(1);
            m.write_u16(S::FREE_READ, if r2 as i16 >= S::FREE_COUNT as i16 { 0 } else { r2 });
            u16::from(m.read_u8(S::FREE_STACK + u32::from(r)))
        }
    };
    let i = S::FREE_INDEX_BASE + k;
    let e = entry_addr(i);
    m.write_u32(e + S::ENTRY_DATA, data);
    inc_u16(m, e + S::ENTRY_REFS);
    if m.read_u16(e + S::ENTRY_REFS) == 1 {
        m.write_u8(e, S::TAKEN_MARK);
    }
    m.write_u16(t + T::SPRITE_ENTRY, i);
    let n = m.read_u16(t + T::PAL_SLOT_COUNT);
    m.write_u16(t + T::PAL_SLOTS + u32::from(n) * 2, i);
    m.write_u16(t + T::PAL_SLOT_COUNT, n.wrapping_add(1));
}

/// Recalcula la profundidad de los atributos con la posición vertical (sólo
/// en los atributos que la usan). Original: `Entity_ApplyFadeShade_028108`.
pub fn update_depth(m: &mut impl Memory, t: u32) {
    let attr = m.read_u16(t + T::SPRITE_ATTR);
    if attr.wrapping_sub(0x4000) >= 0x7FFF {
        return;
    }
    let y = m.read_u16(t + T::POS_Y) & 0xFF;
    let depth = (0xFF - y) << 5;
    m.write_u16(t + T::SPRITE_ATTR, (attr & T::SPRITE_ATTR_DEPTH_KEEP) | depth);
}

/// Atributos de sprite (y su profundidad). Original: `Fn_00028134`.
pub fn set_sprite_attr(m: &mut impl Memory, t: u32, attr: u16) {
    m.write_u16(t + T::SPRITE_ATTR, attr);
    update_depth(m, t);
}

/// Empieza la animación por guion `script`. Original: `Sub_00028CD4`.
pub fn start_animation(m: &mut impl Memory, t: u32, script: u32) {
    m.write_u32(t + T::ANIM_PC, script);
    m.write_u32(t + T::ANIM_START, script);
    for off in [T::ANIM_STEPS, T::ANIM_WAIT, T::ANIM_LOOP] {
        m.write_u8(t + off, 0);
    }
    let f = m.read_u8(t + T::FLAGS4);
    m.write_u8(t + T::FLAGS4, f | T::FLAGS4_ANIM_STARTED);
}

/// La tarea `child` empieza donde está `parent` (posición, atributos,
/// orientación y capa). Original: `ThunkTarget_05dd02`.
pub fn inherit_position(m: &mut impl Memory, parent: u32, child: u32) {
    for off in [T::POS_X, T::POS_Y, T::SPRITE_ATTR] {
        let v = m.read_u16(parent + off);
        m.write_u16(child + off, v);
    }
    for off in [T::FACING, T::LAYER] {
        let v = m.read_u8(parent + off);
        m.write_u8(child + off, v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::TestMachine;

    #[test]
    fn same_graphics_share_one_entry() {
        let mut m = TestMachine::default();
        super::super::video::init_sprite_table(&mut m);
        let (a, b) = (0x10_1000, 0x10_1100);
        use_sprite(&mut m, a, 0xDA);
        use_sprite(&mut m, b, 0xDA);
        let i = m.read_u16(a + T::SPRITE_ENTRY);
        assert_eq!(i, m.read_u16(b + T::SPRITE_ENTRY));
        assert_eq!(m.read_u16(entry_addr(i) + S::ENTRY_REFS), 2);
        // La segunda vez la misma tarea reutiliza la suya.
        use_sprite(&mut m, a, 0xDA);
        assert_eq!(m.read_u16(a + T::PAL_SLOT_COUNT), 1);
    }
}
