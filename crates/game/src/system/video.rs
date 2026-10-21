//! Paleta, tabla de sprites y capa fija.
//!
//! El juego escribe los colores en una copia en RAM y marca las paletas
//! modificadas; en el vblank se vuelcan al hardware. Original:
//! `Pal_ShadowClearAll_01379a`, `Pal_FlushDirtyToHW_0137c6`,
//! `SpriteTable_Init256_0526b8`, `Pubcleaner_10A2Cx_052712`,
//! `VRAM_FixLayerAutoclear_05A824`, `SpriteSubsystem_Reset_05A88A`,
//! `VRAM_FixAutoclear_Reset_05a9ba`, `Vblank_FlushSpriteQueue_05c9d6`.

use super::{dec_to_zero, fill};
use crate::layout::{hw, palette as P, sprites as S};
use crate::mem::{Machine, Memory};

/// Borra la copia de la paleta y reinicia la tabla de sprites.
/// Original: `Pal_ShadowClearAll_01379a` (sigue en `SpriteTable_Init256`).
pub fn clear_palettes(m: &mut impl Memory) {
    fill(m, P::SHADOW, P::COUNT * P::SIZE, 0);
    m.write_u8(P::FADE_STEP, 0);
    init_sprite_table(m);
}

/// Todas las entradas de sprite libres y la pila de índices libres llena.
/// Original: `SpriteTable_Init256_0526b8` (sigue en `Pubcleaner_10A2Cx`).
pub fn init_sprite_table(m: &mut impl Memory) {
    for i in 0..S::COUNT {
        let e = S::TABLE + i * S::SIZE;
        m.write_u16(e, S::FREE_MARK);
        fill(m, e + 2, S::SIZE - 2, 0);
    }
    m.write_u16(S::FREE_TOP, S::FREE_COUNT);
    for i in 0..S::FREE_COUNT {
        m.write_u8(S::FREE_STACK + u32::from(i), i as u8);
    }
    for a in S::STATE_WORDS {
        m.write_u16(a, 0);
    }
    for a in S::STATE_BYTES {
        m.write_u8(a, 0);
    }
    clear_palette_state(m);
}

/// Original: `Pubcleaner_10A2Cx_052712`.
pub fn clear_palette_state(m: &mut impl Memory) {
    for a in P::STATE_BYTES {
        m.write_u8(a, 0);
    }
}

/// Copia al hardware los colores 1..15 de la paleta `i` y la desmarca.
fn flush_palette(m: &mut impl Memory, i: u32) {
    let src = P::SHADOW + i * P::SIZE;
    let dst = hw::PALETTE_RAM + i * P::SIZE;
    m.write_u16(src, 0);
    for k in (2..P::SIZE).step_by(2) {
        let c = m.read_u16(src + k);
        m.write_u16(dst + k, c);
    }
}

/// Vblank: vuelca las paletas modificadas y hace los cambios de banco
/// pendientes. Original: `Pal_FlushDirtyToHW_0137c6`.
pub fn flush_palettes(m: &mut impl Memory) {
    if m.read_u8(P::ANY_DIRTY) != 0 {
        m.write_u8(hw::PAL_BANK1, 1);
        for i in 0..P::SCANNED {
            if m.read_u16(P::SHADOW + i * P::SIZE) != 0 {
                flush_palette(m, i);
            }
        }
        if m.read_u16(P::SHADOW + P::BACKDROP * P::SIZE) != 0 {
            flush_palette(m, P::BACKDROP);
        }
    }
    if m.read_u8(P::BANK_SWAP_ACTIVE) != 0 {
        m.write_u8(hw::PAL_BANK0, 1);
        dec_to_zero(m, P::BANK_SWAP_ACTIVE);
        return;
    }
    if m.read_u8(P::BANK_SWAP_PENDING) != 0 {
        dec_to_zero(m, P::BANK_SWAP_PENDING);
        m.write_u8(P::BANK_SWAP_ACTIVE, 1);
    }
    m.write_u8(hw::PAL_BANK1, 1);
}

/// Borra la capa fija y los sprites en VRAM y reinicia el renderizador.
/// Original: `VRAM_FixLayerAutoclear_05A824` (sigue en
/// `SpriteSubsystem_Reset`).
pub fn clear_vram(m: &mut impl Memory) {
    m.write_u16(hw::VRAM_MOD, 1);
    m.write_u16(hw::VRAM_ADDR, S::VRAM_FIX_CLEAR);
    for _ in 0..S::VRAM_FIX_CLEAR_WORDS {
        m.write_u16(hw::VRAM_RW, 0);
    }
    m.write_u16(hw::VRAM_MOD, 1);
    m.write_u16(hw::VRAM_ADDR, 0);
    for _ in 0..S::VRAM_SCB1_CLEAR_WORDS {
        m.write_u16(hw::VRAM_RW, 0);
    }
    for a in S::FIX_STATE {
        m.write_u16(a, 0);
    }
    reset_sprite_queues(m);
}

/// Original: `SpriteSubsystem_Reset_05A88A`.
pub fn reset_sprite_queues(m: &mut impl Memory) {
    m.write_u8(S::FLUSH_PENDING, 0);
    for a in S::QUEUE_DONE {
        m.write_u16(a, 0);
    }
    for q in S::QUEUES {
        m.write_u8(q + S::QUEUE_ACTIVE, 0);
        m.write_u16(q + S::QUEUE_LIMIT, 0xFFFF);
    }
}

/// Suelta una referencia a la entrada de sprite `i`; si era la última, la
/// entrada queda libre y su índice vuelve al anillo de libres.
/// Original: `PalSlot_Release_002bc4`.
pub fn release_entry(m: &mut impl Memory, i: u16) {
    let e = S::TABLE + u32::from(i) * S::SIZE;
    let refs = m.read_u16(e + S::ENTRY_REFS).wrapping_sub(1);
    m.write_u16(e + S::ENTRY_REFS, refs);
    if refs != 0 {
        return;
    }
    m.write_u8(e, S::RELEASED_MARK);
    m.write_u32(e + S::ENTRY_DATA, 0);
    m.write_u16(e + S::ENTRY_AUX, 0);
    let w = m.read_u16(S::FREE_WRITE);
    m.write_u8(S::FREE_STACK + u32::from(w), i.wrapping_sub(S::FREE_INDEX_BASE) as u8);
    let w = w.wrapping_add(1);
    m.write_u16(S::FREE_WRITE, if w as i16 >= S::FREE_COUNT as i16 { 0 } else { w });
    super::inc_u16(m, S::FREE_TOP);
}

/// Suelta todas las entradas de sprite que tiene la tarea `t`, de la
/// última a la primera. Original: `Entity_FlushSlotHistory_013600`.
pub fn release_task_entries(m: &mut impl Memory, t: u32) {
    use crate::layout::task as T;
    loop {
        let n = m.read_u16(t + T::PAL_SLOT_COUNT);
        if n == 0 {
            return;
        }
        let n = n - 1;
        m.write_u16(t + T::PAL_SLOT_COUNT, n);
        let slot = t + T::PAL_SLOTS + u32::from(n) * 2;
        let i = m.read_u16(slot);
        m.write_u16(slot, 0);
        release_entry(m, i);
    }
}

/// Carga una lista de entradas fijas de la ROM (ver `sprites::LOW_LIST`).
/// Devuelve dónde está su marca de fin.
fn load_entries(m: &mut impl Memory, list: u32, first: u32, data: u32) -> u32 {
    let mut a = list;
    loop {
        let w = m.read_u16(a);
        if w == S::LIST_END {
            return a;
        }
        let i = u32::from(w & 0xFF) + first;
        let e = S::TABLE + i * S::SIZE;
        let gfx = m.read_u16(a + 2);
        if gfx == S::LIST_END {
            m.write_u16(e + S::ENTRY_REFS, 0);
            m.write_u8(e, S::EMPTY_MARK);
        } else {
            m.write_u32(e + S::ENTRY_DATA, u32::from(gfx) * 64 + data);
            m.write_u16(e + S::ENTRY_REFS, 1);
            m.write_u8(e, (w >> 8) as u8 | 1);
        }
        a += 4;
    }
}

/// Original: `$002AEC` (lista baja) y `PalSlot_LoadListHi_002b58` (alta).
pub fn load_low_entries(m: &mut impl Memory, list: u32) -> u32 {
    load_entries(m, list, S::LOW_LIST_FIRST, S::LOW_LIST_DATA)
}
pub fn load_high_entries(m: &mut impl Memory, list: u32) -> u32 {
    load_entries(m, list, 0, S::HIGH_LIST_DATA)
}

/// Empieza a animar la paleta de la entrada `i` desde el paso `first`:
/// `len` frames por paso, en modo `mode`. Original: `$002C30`.
pub fn start_entry_anim(m: &mut impl Memory, i: u16, first: u16, len: u16, mode: u8) {
    let e = S::TABLE + u32::from(i.wrapping_shl(5));
    m.write_u32(e + S::ENTRY_ANIM_DATA, u32::from(first) * 64 + S::ANIM_DATA);
    m.write_u16(e + S::ENTRY_ANIM_LEN, len);
    m.write_u16(e + S::ENTRY_ANIM_LEFT, len);
    m.write_u8(e + S::ENTRY_ANIM_STEP, 0);
    m.write_u8(e + S::ENTRY_ANIM_MODE, mode);
    let f = m.read_u8(e);
    m.write_u8(e, f | S::ANIMATING);
}

/// Entradas fijas de sprite de todas las escenas.
/// Original: `Task_InstallBootSlots_001da4`.
pub fn load_fixed_entries(m: &mut impl Memory) {
    load_low_entries(m, S::LOW_LIST);
    load_high_entries(m, S::HIGH_LIST);
}

/// Oculta los sprites hardware `from..=to` (SCB3 = 0).
/// Original: `SpriteRange_DisableAll_01399C`.
pub fn hide_sprites(m: &mut impl Memory, from: u16, to: u16) {
    m.write_u16(hw::VRAM_MOD, 0);
    let (from, to) = (from.wrapping_add(S::VRAM_SCB3_FIRST), to.wrapping_add(S::VRAM_SCB3_FIRST));
    let mut a = from;
    loop {
        m.write_u16(hw::VRAM_ADDR, a);
        m.write_u16(hw::VRAM_RW, 0);
        a = a.wrapping_add(1);
        if a > to {
            return;
        }
    }
}

/// Reparte los sprites hardware: el juego empieza en `first`, se reservan
/// `top` al final y `bottom` antes de ellos; oculta todos.
/// Original: `Scratch_Alloc_01390E` (d0, d2, d1).
pub fn split_sprites(m: &mut impl Memory, first: u16, top: u16, bottom: u16) {
    reset_sprite_queues(m);
    for a in S::BAND_CLEAR {
        m.write_u16(a, 0);
    }
    m.write_u16(S::BAND_FIRST, first);
    let end = S::BAND_TOTAL.wrapping_sub(top);
    m.write_u16(S::BAND_END, end);
    let split = end.wrapping_sub(bottom);
    for a in S::BAND_SPLIT {
        m.write_u16(a, split);
    }
    hide_sprites(m, 0, S::BAND_TOTAL - 1);
}

/// Todos los sprites para el juego. Original: `Scratch_AllocZero_001e1c`.
pub fn reset_sprites(m: &mut impl Memory) {
    split_sprites(m, 0, 0, 0);
}

/// Empieza un frame de dibujo vacío. Original: `VRAM_FixAutoclear_Reset_05a9ba`.
pub fn begin_draw_frame(m: &mut impl Machine) {
    m.write_u8(S::FLUSH_PENDING, 0);
    m.write_u16(S::DRAW_COUNT, 0);
    m.write_u16(S::QUEUE_FILL[0], 0);
    m.write_u16(S::QUEUE_FILL[1], S::DRAW_LIST_SECOND as u16);
    // El original deja a5 apuntando a las variables del renderizador.
    let mut r = m.regs();
    r.a[5] = S::RENDER_BASE;
    m.set_regs(r);
}

/// Vblank: si hay sprites listos, vuelca las dos colas de dibujo a la VRAM.
/// Original: `Vblank_FlushSpriteQueue_05c9d6`.
pub fn flush_sprite_queues(m: &mut impl Machine) {
    if m.read_u8(S::FLUSH_PENDING) == 0 {
        return;
    }
    m.write_u8(S::FLUSH_PENDING, 0);
    for k in 0..2 {
        let fill = u32::from(m.read_u16(S::QUEUE_FILL[k]));
        let (start, end) = if k == 0 {
            (S::DRAW_LIST, S::DRAW_LIST + fill)
        } else {
            (S::DRAW_LIST + fill, S::DRAW_LIST + S::DRAW_LIST_SECOND)
        };
        super::render::render_scb234(m, S::QUEUES[k], start, end);
        let done = m.read_u16(S::QUEUES[k] + S::QUEUE_RESULT);
        m.write_u16(S::QUEUE_DONE[k], done);
    }
}
