//! Carga de una escena: sus capas de scroll (cada una con su cámara y una
//! ventana de 32 columnas de sprites hardware), la cámara principal, los
//! límites de la vista y el estado de la misión.
//!
//! Original: `SceneLoader_Main_043568` (y la entrada `$043562`, con un descriptor propio),
//! `$043D6C`, `Entity_AllocAndInit_051ABE`, `CellMap_ClearSprites32_051ed6`,
//! `CellMap_ClearVramBlock_051f02`, `Spawn_TypeA_013982`,
//! `Spawn_TypeB_013952`, `$0139BE`, `$043EDA`, `Camera_ResetSmoothing_0434EA`,
//! `$07707C`, `$08F158`, `$03EE3A`, `$0997B8`, `$04CB5C`.

use super::video;
use crate::layout::{hw, scene as L, sprites as S, system as Y};
use crate::mem::Memory;

/// Carga la escena `n` de la tabla. Original: `SceneLoader_Main_043568`.
pub fn load(m: &mut impl Memory, n: u8) {
    load_descriptor(m, L::TABLE + u32::from(n) * L::TABLE_ENTRY);
}

/// Carga la escena descrita en `entry` (guion de scroll y capas).
/// Original: `$043562` (con el descriptor en a0).
pub fn load_descriptor(m: &mut impl Memory, entry: u32) {
    let script = m.read_u32(entry);
    m.write_u32(L::SCRIPT_PC, script);
    let layers = m.read_u32(entry + 4);
    for c in L::CAMERAS {
        m.write_u32(c + L::C_LINK, 0);
    }

    // Reparto de los sprites hardware: cada capa necesita una ventana.
    let (mut high, mut low) = (0u16, 0u16);
    let mut p = layers;
    while m.read_u8(p + L::LAYER_TYPE) != L::LAYER_END {
        if m.read_u8(p + L::LAYER_TYPE) == L::LAYER_HIGH {
            high += L::LAYER_SPRITES;
        } else {
            low += L::LAYER_SPRITES;
        }
        p += L::LAYER_SIZE;
    }
    let reserve = u16::from(m.read_u8(p + L::LAYER_END_RESERVE));
    video::split_sprites(m, low, reserve, high);

    let mut p = layers;
    while m.read_u8(p + L::LAYER_TYPE) != L::LAYER_END {
        let high = m.read_u8(p + L::LAYER_TYPE) == L::LAYER_HIGH;
        let first = take_sprites(m, high, L::LAYER_SPRITES);
        let cam = m.read_u32(p + L::LAYER_CAMERA);
        init_layer(m, cam, p + L::LAYER_BOX, first);
        m.write_u16(cam + L::C_SCALE[0], L::C_SCALE_ONE);
        m.write_u16(cam + L::C_SCALE[1], L::C_SCALE_ONE);
        m.write_u8(cam + L::C_FLAG72, 0);
        m.write_u32(cam + L::C_LONG78, 0);
        p += L::LAYER_SIZE;
    }

    reset_camera(m);
    for a in L::CLEAR_BYTES {
        m.write_u8(a, 0);
    }
    for a in L::CLEAR_WORDS {
        m.write_u16(a, 0);
    }
    m.write_u16(L::CLEAR_WORD_LAST, 0);
    m.write_u8(L::LOADED, 1);
    let mission = m.read_u8(L::MISSION_KEY);
    let ptr = if mission < L::MISSIONS { m.read_u32(L::MISSION_TABLE + u32::from(mission) * 4) } else { L::NIL };
    m.write_u32(L::MISSION_PTR, ptr);
}

/// Cámara principal, vista y suavizado a cero; mapa de colisión vacío.
fn reset_camera(m: &mut impl Memory) {
    m.write_u16(L::CAMERA_W5E, 0);
    for a in [L::CAMERA_X, L::CAMERA_Y, L::CAMERA_Z] {
        m.write_u32(a, 0);
    }
    for (a, v) in L::VIEW_INIT {
        m.write_u16(a, v);
    }
    m.write_u16(L::CAMERA_W5C, 0);
    m.write_u8(L::FROZEN, 0);
    let w5c = m.read_u16(L::CAMERA_W5C);
    m.write_u16(L::START_W5C, w5c);
    let x = m.read_u16(L::CAMERA_X);
    m.write_u16(L::START_X, x);
    let z = m.read_u16(L::CAMERA_Z);
    m.write_u16(L::START_Z, z);
    for i in 0..L::COLLMAP_LEN / 4 {
        m.write_u32(L::COLLMAP_CELLS + 4 * i, 0);
    }
    for a in L::SMOOTH {
        m.write_u32(a, 0);
    }
}

/// Reserva `n` sprites hardware de la zona alta (`high`) o baja y los deja
/// encadenados en columna y a tamaño real. Devuelve el primero.
/// Original: `Spawn_TypeA_013982` / `Spawn_TypeB_013952`.
pub fn take_sprites(m: &mut impl Memory, high: bool, n: u16) -> u16 {
    let (cursor, limit) = if high {
        (S::BAND_SPLIT[1], S::BAND_TOTAL)
    } else {
        (S::BAND_CLEAR[1], m.read_u16(S::BAND_FIRST))
    };
    let first = m.read_u16(cursor);
    let end = first.wrapping_add(n);
    debug_assert!(end <= limit, "sin sprites hardware");
    m.write_u16(cursor, end);
    chain_sprites(m, first, end.wrapping_sub(1));
    first
}

/// Sprites `first..=last`: el primero empieza columna y los demás se
/// encadenan (SCB3), todos sin reducción (SCB2). Original: `$0139BE`.
fn chain_sprites(m: &mut impl Memory, first: u16, last: u16) {
    let mut n = first;
    loop {
        let scb3 = if n == first { 0 } else { hw::SCB3_STICKY };
        vram(m, S::SCB3_FIRST.wrapping_add(n), scb3);
        vram(m, S::SCB2.wrapping_add(1).wrapping_add(n), hw::SCB2_FULL_SIZE);
        if n >= last {
            return;
        }
        n += 1;
    }
}

fn vram(m: &mut impl Memory, addr: u16, data: u16) {
    m.write_u32(hw::VRAM_ADDR, (u32::from(addr) << 16) | u32::from(data));
}

/// Prepara la cámara de una capa: caja (4 palabras en `bx`), ventana de
/// sprites desde `first`, todo lo demás a 0; oculta sus sprites y borra
/// sus tiles. Original: `Entity_AllocAndInit_051ABE`.
pub fn init_layer(m: &mut impl Memory, cam: u32, bx: u32, first: u16) {
    m.write_u16(cam + L::C_SPRITE_BASE, first);
    m.write_u32(cam + L::C_LINK, 0);
    for i in 0..4 {
        let v = m.read_u16(bx + 2 * i);
        m.write_u16(cam + L::C_BOX + 2 * i, v);
    }
    for a in L::C_ORIGIN {
        m.write_u16(cam + a, 0);
    }
    for a in [L::C_WORDS[2], L::C_WORDS[3], L::C_FLAGS] {
        m.write_u16(cam + a, 0);
    }
    for a in L::C_LONGS {
        m.write_u32(cam + a, 0);
    }
    for a in L::C_POS.into_iter().chain([L::C_WORDS[0], L::C_WORDS[1]]) {
        m.write_u16(cam + a, 0);
    }
    hide_layer_sprites(m, cam);
    clear_layer_tiles(m, cam);
    for i in 0..L::C_ROWS_LEN {
        m.write_u8(cam + L::C_ROWS + i, 0);
    }
}

/// Oculta las 32 columnas de la capa (SCB3 = 0) y anota la dirección de
/// VRAM que hay que restaurar. Original: `CellMap_ClearSprites32_051ed6`.
fn hide_layer_sprites(m: &mut impl Memory, cam: u32) {
    let f = m.read_u8(cam + L::C_FLAGS);
    m.write_u8(cam + L::C_FLAGS, f & !1);
    let base = m.read_u16(cam + L::C_SPRITE_BASE).wrapping_add(S::SCB3_FIRST);
    for i in 0..L::LAYER_SPRITES {
        let a = base.wrapping_add(i);
        m.write_u16(Y::VRAM_ADDR_SHADOW, a);
        m.write_u16(hw::VRAM_ADDR, a);
        m.write_u16(hw::VRAM_RW, 0);
    }
}

/// Tiles (SCB1) de las 32 columnas de la capa a 0.
/// Original: `CellMap_ClearVramBlock_051f02`.
fn clear_layer_tiles(m: &mut impl Memory, cam: u32) {
    let start = (m.read_u16(cam + L::C_SPRITE_BASE) << 6).wrapping_add(S::SCB1_WORDS);
    for i in 0..L::LAYER_ROWS * 2 {
        vram(m, start.wrapping_add(i as u16), 0);
    }
}
