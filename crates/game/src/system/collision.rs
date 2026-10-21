//! Colisión de las entidades con el mapa: tipo de tile bajo un punto, test
//! de las máscaras de 8×8 de ese tipo y sondeo del suelo bajo los pies
//! (mapa o plataforma móvil).
//!
//! Cada tipo de tile (`TILE_TYPES`, 32 bytes) tiene dos máscaras de 8 filas
//! (sólido y plataforma), unos flags que la entidad puede pedir ignorar
//! (`+$6B`) y el tipo por el que se sustituye entonces.
//!
//! Original: `CollMap_LookupTile_027db2`, `CollMap_TestSolidOrPlatform_027e28`,
//! `SpritePubEffect_027EBA` (`asm/entity/collmap_hitbox_script_ops_027400.s`).

use super::{platforms, scroll};
use crate::layout::collision as L;
use crate::mem::{Machine, Memory};

/// Tipo de tile bajo un punto.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tile {
    /// Celda del mapa tal cual (con los bits altos).
    pub raw: u8,
    /// Tipo resultante (0–63) y su registro en `TILE_TYPES`.
    pub kind: u8,
    pub record: u32,
    /// Posición dentro de la celda (x, y), si se ha mirado el mapa.
    pub frac: Option<(u16, u16)>,
}

/// Tipo de tile en (`x`, `y`) para la entidad `t` (por debajo de la
/// pantalla, el tipo 0). Original: `CollMap_LookupTile_027db2`.
pub fn lookup_tile(m: &mut impl Memory, t: u32, x: u16, y: u16) -> Tile {
    let (mut cell, frac) = if (y as i16) > L::MAP_BOTTOM {
        (0u8, None)
    } else {
        let h = scroll::test_point(m, x, y);
        (h.cell, Some(h.frac))
    };
    let mut ignore = m.read_u8(t + L::IGNORE);
    loop {
        if m.read_u8(t + L::GROUND) & L::GROUND_PASS != 0 && cell & L::PASS_BITS == L::PASS_BITS {
            cell = 0;
        }
        let raw = cell;
        let kind = cell & L::KIND_MASK;
        let record = L::TILE_TYPES + u32::from(kind) * L::TILE_TYPE_LEN;
        if m.read_u8(record + L::T_FLAGS) & ignore == 0 {
            return Tile { raw, kind, record, frac };
        }
        ignore = 0;
        match m.read_u8(record + L::T_NEXT) {
            0xFF => return Tile { raw, kind: 0, record, frac },
            next => cell = next,
        }
    }
}

/// ¿Está a 1 el bit de la fila `row`, columna `col` de alguna de las dos
/// máscaras del tipo `record`? Devuelve el registro de la máscara que ha
/// dado (la segunda está `+$10` más allá). Original:
/// `CollMap_TestSolidOrPlatform_027e28`.
pub fn test_masks(m: &mut impl Memory, record: u32, col: u16, row: u16) -> Option<u32> {
    let bit = 7u16.wrapping_sub(col);
    for rec in [record, record + L::SECOND_MASK] {
        let mask = m.read_u32(rec);
        let b = m.read_u8(mask.wrapping_add(row as i16 as u32));
        if bit < 8 && b & (1 << bit) != 0 {
            return Some(rec);
        }
    }
    None
}

/// Resultado del sondeo del suelo (lo que necesita el código original que
/// llama).
#[derive(Debug, Clone, Copy)]
pub struct Floor {
    pub in_air: bool,
    pub tile: Tile,
    /// Registro y celda (columna, fila) con los que se ha probado.
    pub record: u32,
    pub at: (u16, u16),
    /// Registro de la máscara que ha dado, si alguna.
    pub hit: Option<u32>,
    pub platform: Option<(u16, u8)>,
}

/// ¿Tiene suelo bajo los pies la entidad `t`? Mira el mapa y las
/// plataformas móviles; deja en `PLATFORM_UNDER` la plataforma (o `$F`) y,
/// si pisa, marca el bit 6 de `+$5A`. Original: `SpritePubEffect_027EBA`
/// (C = en el aire). `frac_default` = columna y fila que deja el
/// original si no mira el mapa (restos de registros).
pub fn probe_floor(m: &mut impl Memory, t: u32, frac_default: (u16, u16)) -> Floor {
    let x = m.read_u16(t + L::X);
    let y = m.read_u16(t + L::Y).wrapping_sub(1);
    let tile = lookup_tile(m, t, x, y);
    let platform = platforms::find_under(m, t, x, y);
    m.write_u8(L::PLATFORM_UNDER, platform.map_or(0x0F, |p| p.1));
    let (record, at) = match platform {
        Some((depth, _)) => (L::PLATFORM_TILE, (0, depth)),
        None => (tile.record, tile.frac.unwrap_or(frac_default)),
    };
    let hit = test_masks(m, record, at.0, at.1);
    if hit.is_some() {
        let g = m.read_u8(t + L::STATE);
        m.write_u8(t + L::STATE, g | L::ON_FLOOR);
    }
    Floor { in_air: hit.is_none(), tile, record, at, hit, platform }
}

/// Como [`probe_floor`] con los restos de `d3`/`d4` de la máquina.
pub fn probe_floor_here(m: &mut impl Machine) -> bool {
    let t = m.task_register();
    let r = m.regs();
    probe_floor(m, t, (r.d[3] as u16, r.d[4] as u16)).in_air
}
