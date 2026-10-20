//! Paleta, tabla de sprites y capa fija.
//!
//! El juego escribe los colores en una copia en RAM y marca las paletas
//! modificadas; en el vblank se vuelcan al hardware. Original:
//! `Pal_ShadowClearAll_01379a`, `Pal_FlushDirtyToHW_0137c6`,
//! `SpriteTable_Init256_0526b8`, `Pubcleaner_10A2Cx_052712`,
//! `VRAM_FixLayerAutoclear_05A824`, `SpriteSubsystem_Reset_05A88A`,
//! `VRAM_FixAutoclear_Reset_05a9ba`, `Vblank_FlushSpriteQueue_05c9d6`.

use super::{dec_to_zero, fill};
use crate::layout::{hw, palette as P, routine, sprites as S};
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
        let mut r = m.regs();
        r.a[4] = hw::VRAM_ADDR;
        r.a[5] = S::RENDER_BASE;
        r.a[0] = S::QUEUES[k];
        r.a[1] = start;
        r.a[2] = end;
        r.d[7] = (r.d[7] & 0xFFFF_0000) | fill;
        m.call_original(routine::SPRITE_QUEUE_RENDER, &mut r);
        let done = m.read_u16(r.a[0] + S::QUEUE_RESULT);
        m.write_u16(S::QUEUE_DONE[k], done);
    }
}
