//! Tiles de una capa de scroll: cada capa es un anillo de 32×32 celdas de
//! 16 px (32 sprites hardware de 32 tiles) sobre un mapa más grande. Al
//! moverse, se pintan en VRAM las columnas o filas que entran, con la fila
//! de inicio y los atributos de su tramo, y su trozo del mapa de colisión.
//!
//! El mapa (`scroll::C_PATH`) es {ancho.w, alto.w, celdas.l} con las celdas
//! por columnas, 4 bytes cada una (tile, atributos).
//!
//! Original: `$051C08` (ventana entera), `$051C82` (columna nueva),
//! `$051CF6` (fila nueva), `$051D84` (recorte con el mapa), `$051BA8`
//! (tramo de la tabla de tiles), `$051DE2` (escritura en SCB1),
//! `CameraHook_Probe08_043DF4`, `$043E0E`, `$043E24`, `$043E8C` (colisión).

use crate::layout::{hw, scroll as L, system as Y};
use crate::mem::Memory;

/// Trozo del mapa que hay que pintar: posición en el anillo (`ring`, en
/// celdas, aún sin la base de tiles), posición en el mapa (`map`), tamaño
/// y alto del mapa (para saltar de columna).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Window {
    pub ring: (u16, u16),
    pub map: (u16, u16),
    pub size: (u16, u16),
    pub map_h: u16,
}

/// Mapa de la capa: ancho, alto y celdas.
fn map_dims(m: &mut impl Memory, map: u32) -> (u16, u16) {
    (m.read_u16(map), m.read_u16(map + 2))
}

/// Recorta un trozo con el mapa (celdas fuera = no se pintan). `None` si
/// no queda nada. Original: `$051D84`.
pub fn clip(m: &mut impl Memory, map: u32, ring: (u16, u16), at: (u16, u16), size: (u16, u16)) -> Option<Window> {
    let (map_w, map_h) = map_dims(m, map);
    let axis = |ring: u16, at: u16, n: u16, limit: u16| -> Option<(u16, u16, u16)> {
        let (mut ring, mut at) = (ring, at);
        let mut end = at.wrapping_add(n);
        if limit < at {
            // Empieza antes del mapa (o después: entonces no hay nada).
            if end > at {
                return None;
            }
            ring = ring.wrapping_sub(at);
            at = 0;
        }
        if limit < end {
            end = limit;
        }
        let n = end.wrapping_sub(at);
        (n != 0).then_some((ring, at, n))
    };
    let (rx, mx, w) = axis(ring.0, at.0, size.0, map_w)?;
    let (ry, my, h) = axis(ring.1, at.1, size.1, map_h)?;
    Some(Window { ring: (rx, ry), map: (mx, my), size: (w, h), map_h })
}

/// Tramo de la tabla de tiles que cubre la columna `col` del mapa: fija la
/// fila de inicio y los atributos de la capa. La tabla son registros
/// {inicio.w, atributos.b, fila.b} acabados en inicio = `$FFFF`; la capa
/// recuerda el último usado. Original: `$051BA8`.
pub fn tile_lookup(m: &mut impl Memory, cam: u32, col: u16) {
    let mut p = m.read_u32(cam + L::C_TILE_TABLE);
    if p != 0 {
        loop {
            let start = m.read_u16(p);
            let next = m.read_u16(p + 4);
            if col < start {
                if start == 0 {
                    break;
                }
                p -= 4;
                continue;
            }
            if col < next {
                let attr = m.read_u8(p + 2);
                m.write_u8(cam + L::C_TILE_ATTR, attr);
                let row = u16::from(m.read_u8(p + 3)).wrapping_add(m.read_u16(cam + L::C_TILE_BASE[1]));
                m.write_u8(cam + L::C_TILE_ROW, (row.wrapping_neg() & 0x1F) as u8);
                break;
            }
            if next == 0xFFFF {
                break;
            }
            p += 4;
        }
    }
    m.write_u32(cam + L::C_TILE_TABLE, p);
}

/// Desplazamiento (en bytes) de la celda `at` del mapa, como lo calcula el
/// original: columna × alto en 32 bits, más la fila en la palabra baja.
fn cell_index(at: (u16, u16), map_h: u16) -> u32 {
    let p = u32::from(at.0) * u32::from(map_h);
    (p & 0xFFFF_0000) | u32::from((p as u16).wrapping_add(at.1))
}

/// Salto entre columnas del mapa tras leer `h` celdas.
fn column_skip(map_h: u16, h: u16) -> u32 {
    (map_h.wrapping_sub(h).wrapping_mul(4) as i16) as u32
}

/// Pinta en SCB1 las celdas de `win` (dos palabras por celda), cada
/// columna empezando en su fila. Original: `$051DE2` (mapa en a1).
pub fn write_tiles(m: &mut impl Memory, cam: u32, map: u32, win: Window) {
    let mut src = m.read_u32(map + 4).wrapping_add(cell_index(win.map, win.map_h).wrapping_mul(4));
    let skip = column_skip(win.map_h, win.size.1);
    let col0 = win.ring.0.wrapping_add(m.read_u16(cam + L::C_TILE_BASE[0]));
    let row0 = win.ring.1.wrapping_add(m.read_u16(cam + L::C_TILE_BASE[1]));
    let base = m.read_u16(cam + L::C_SPRITE_BASE);
    for i in 0..win.size.0 {
        let col = col0.wrapping_add(i) & 0x1F;
        let start = (row0 as u8).wrapping_add(m.read_u8(cam + L::C_COL_ROW + u32::from(col))) & 0x1F;
        let sprite = (col.wrapping_add(base) << 6).wrapping_add(0x40);
        let mut r = u16::from(start) * 2;
        for _ in 0..win.size.1 {
            let a = sprite.wrapping_add(r);
            for k in 0..2 {
                let v = m.read_u16(src);
                src += 2;
                vram(m, a.wrapping_add(k), v);
            }
            r = (r + 2) & 0x3F;
        }
        src = src.wrapping_add(skip);
    }
}

fn vram(m: &mut impl Memory, addr: u16, data: u16) {
    m.write_u16(Y::VRAM_ADDR_SHADOW, addr);
    m.write_u16(hw::VRAM_ADDR, addr);
    m.write_u16(hw::VRAM_RW, data);
}

/// Celda de la esquina de la ventana visible de la capa.
fn corner(m: &mut impl Memory, cam: u32) -> (u16, u16) {
    let x = m.read_u16(cam + L::C_POS[0]).wrapping_add(m.read_u16(cam + L::C_BOX[0]));
    let y = m.read_u16(cam + L::C_POS[1]).wrapping_add(m.read_u16(cam + L::C_BOX[1]));
    (((x as i16) >> 4) as u16, ((y as i16) >> 4) as u16)
}

/// Tramos y tiles de las columnas de `win`.
fn fill(m: &mut impl Memory, cam: u32, map: u32, win: Window) {
    let base = m.read_u16(cam + L::C_TILE_BASE[0]);
    for i in 0..win.size.0 {
        let col = win.ring.0.wrapping_add(i);
        tile_lookup(m, cam, col);
        let c = u32::from(col.wrapping_add(base) & 0x1F);
        let row = m.read_u8(cam + L::C_TILE_ROW);
        let attr = m.read_u8(cam + L::C_TILE_ATTR);
        m.write_u8(cam + L::C_COL_ROW + c, row);
        m.write_u8(cam + L::C_COL_ATTR + c, attr);
    }
    write_tiles(m, cam, map, win);
}

/// Pinta toda la ventana visible de la capa. Original: `$051C08`.
pub fn refresh(m: &mut impl Memory, cam: u32) -> Option<Window> {
    let map = m.read_u32(cam + L::C_PATH);
    let at = corner(m, cam);
    m.write_u16(cam + L::C_ORIGIN_CELL[0], at.0);
    m.write_u16(cam + L::C_ORIGIN_CELL[1], at.1);
    let size = (m.read_u16(cam + L::C_BOX[2]), m.read_u16(cam + L::C_BOX[3]));
    let win = clip(m, map, at, at, size)?;
    fill(m, cam, map, win);
    Some(win)
}

/// Si la capa ha pasado a otra columna de celdas, pinta la que entra (por
/// la derecha si avanza, por la izquierda si retrocede). Original: `$051C82`.
pub fn scroll_x(m: &mut impl Memory, cam: u32) -> Option<Window> {
    let map = m.read_u32(cam + L::C_PATH);
    let (mut x, y) = corner(m, cam);
    if x == m.read_u16(cam + L::C_ORIGIN_CELL[0]) {
        return None;
    }
    m.write_u16(cam + L::C_ORIGIN_CELL[0], x);
    if (m.read_u16(cam + L::C_DELTA[0]) as i16) >= 0 {
        x = x.wrapping_add(m.read_u16(cam + L::C_BOX[2])).wrapping_sub(1);
    }
    let h = m.read_u16(cam + L::C_BOX[3]);
    let win = clip(m, map, (x, y), (x, y), (1, h))?;
    fill(m, cam, map, win);
    Some(win)
}

/// Lo mismo con las filas. Original: `$051CF6`.
pub fn scroll_y(m: &mut impl Memory, cam: u32) -> Option<Window> {
    let map = m.read_u32(cam + L::C_PATH);
    let (x, mut y) = corner(m, cam);
    if y == m.read_u16(cam + L::C_ORIGIN_CELL[1]) {
        return None;
    }
    m.write_u16(cam + L::C_ORIGIN_CELL[1], y);
    if (m.read_u16(cam + L::C_DELTA[1]) as i16) >= 0 {
        y = y.wrapping_add(m.read_u16(cam + L::C_BOX[3])).wrapping_sub(1);
    }
    let w = m.read_u16(cam + L::C_BOX[2]);
    let win = clip(m, map, (x, y), (x, y), (w, 1))?;
    fill(m, cam, map, win);
    Some(win)
}

/// Copia al mapa de colisión de la capa (anillo de 32×32 longs) las celdas
/// de `win` del mapa de colisión `src`. Original: `$043E8C`.
pub fn copy_collision(m: &mut impl Memory, cam: u32, src: u32, win: Window) {
    let off = ((cell_index(win.map, win.map_h) as u16).wrapping_mul(4) as i16) as u32;
    let mut a = src.wrapping_add(off);
    let skip = column_skip(win.map_h, win.size.1);
    let mut col = win.ring.0.wrapping_add(m.read_u16(cam + L::C_TILE_BASE[0])) << 7;
    let row = win.ring.1.wrapping_add(m.read_u16(cam + L::C_TILE_BASE[1])) << 2;
    for _ in 0..win.size.0 {
        let dst = cam + L::C_COLLMAP + u32::from(col & 0xF80);
        let mut r = row;
        for _ in 0..win.size.1 {
            let v = m.read_u32(a);
            a += 4;
            m.write_u32(dst + u32::from(r & 0x7C), v);
            r = r.wrapping_add(4);
        }
        a = a.wrapping_add(skip);
        col = col.wrapping_add(0x80);
    }
}

/// Tras pintar `win`, si la capa tiene mapa de colisión, copia su trozo.
fn with_collision(m: &mut impl Memory, cam: u32, win: Option<Window>) -> bool {
    let Some(win) = win else { return false };
    let src = m.read_u32(cam + L::C_COLLMAP_SRC);
    if src != 0 {
        copy_collision(m, cam, src, win);
    }
    true
}

/// Ventana entera con su colisión. Original: `CameraHook_Probe08_043DF4`.
pub fn refresh_all(m: &mut impl Memory, cam: u32) -> bool {
    let w = refresh(m, cam);
    with_collision(m, cam, w)
}

/// Columna nueva con su colisión. Original: `$043E0E`.
pub fn scroll_x_all(m: &mut impl Memory, cam: u32) -> bool {
    let w = scroll_x(m, cam);
    with_collision(m, cam, w)
}

/// Fila nueva con su colisión. Original: `$043E24`.
pub fn scroll_y_all(m: &mut impl Memory, cam: u32) -> bool {
    let w = scroll_y(m, cam);
    with_collision(m, cam, w)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::TestRam;

    #[test]
    fn clip_cuts_to_the_map() {
        let mut m = TestRam::default();
        let map = 0x10_2000;
        m.write_u16(map, 10);
        m.write_u16(map + 2, 4);
        // Dentro.
        let w = clip(&mut m, map, (2, 1), (2, 1), (3, 2)).unwrap();
        assert_eq!((w.map, w.size, w.map_h), ((2, 1), (3, 2), 4));
        // Se sale por la derecha y por abajo.
        let w = clip(&mut m, map, (8, 3), (8, 3), (5, 5)).unwrap();
        assert_eq!(w.size, (2, 1));
        // Empieza antes del mapa.
        let w = clip(&mut m, map, (0xFFFE, 0), (0xFFFE, 0), (5, 1)).unwrap();
        assert_eq!((w.ring.0, w.map.0, w.size.0), (0, 0, 3));
        // Del todo fuera.
        assert!(clip(&mut m, map, (12, 0), (12, 0), (2, 1)).is_none());
    }

    #[test]
    fn collision_lands_in_the_ring() {
        let mut m = TestRam::default();
        let cam = 0x10_6F6C;
        let src = 0x10_3000;
        // Mapa de 3 de alto: celda (1, 2) = índice 5.
        m.write_u32(src + 5 * 4, 0xAABB_CCDD);
        let win = Window { ring: (1, 2), map: (1, 2), size: (1, 1), map_h: 3 };
        copy_collision(&mut m, cam, src, win);
        assert_eq!(m.read_u32(cam + L::C_COLLMAP + 0x80 + 8), 0xAABB_CCDD);
    }
}
