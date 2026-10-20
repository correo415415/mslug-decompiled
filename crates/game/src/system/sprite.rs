//! Alta de un sprite en la lista de dibujo del frame.
//!
//! Un sprite de la ROM («gfx») es una lista de trozos; cada trozo es un
//! bloque de columnas de tiles con su desplazamiento. Para dibujarlo se
//! añade una clave de ordenación a una de las dos colas de la lista de
//! dibujo (la de delante o la de detrás) y se escribe un registro de
//! columna por cada sprite hardware que hará falta (ver [`super::render`]).
//! Las columnas que caen fuera de la pantalla no se escriben.
//!
//! Hay ocho variantes: con o sin espejo horizontal, con o sin espejo
//! vertical, y con o sin reducción. Con reducción, las posiciones de los
//! trozos se escalan y la reducción horizontal (4 bits por columna) se
//! reparte entre las columnas con un acumulador de fracción.
//!
//! En la primera misión, si el sprite está sobre agua, antes se añade su
//! reflejo: espejo vertical, la mitad de alto y reflejado respecto a la
//! línea del agua.
//!
//! Original: `Sprite_Dispatch_05A9E2` / `Sprite_Dispatch_05A9D6`,
//! `SpriteDispatchJT_05AA96` (las 8 variantes),
//! `Sprite_DispatchSplashHook_05b1b2`, `Sprite_SplashScreenY_05b212`,
//! `CollMap_TestPoint_043F02`, `Viewport_CoordToScreen_096A5A`.

use crate::layout::{anim as A, sprite_draw as D, sprites as S};
use crate::mem::Memory;

/// Petición de dibujo (los registros con los que se llama al original).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Request {
    /// X de pantalla en la palabra alta, Y en la baja.
    pub pos: u32,
    /// Clave de ordenación (mayor = más delante).
    pub key: u16,
    /// Paleta (byte bajo; va al byte alto de los atributos SCB1).
    pub palette: u16,
    /// Reducción vertical (`$FF` = tamaño real).
    pub shrink_v: u8,
    /// Reducción horizontal (nibble alto) (`$FF` = tamaño real).
    pub shrink_h: u8,
    /// Bit 0 = espejo horizontal, bit 1 = espejo vertical.
    pub flip: u8,
    /// Gráfico de la ROM (negativo = no dibujar).
    pub gfx: u32,
}

/// Dibuja `req` en la cola de delante (`back` = false) o en la de detrás;
/// `task` es la tarea que dibuja (para el reflejo en el agua).
pub fn dispatch(m: &mut impl Memory, task: u32, req: Request, back: bool) {
    let mode = (req.flip & 3) | if back { MODE_BACK } else { 0 };
    if over_water(m, task) {
        let reflected = Request {
            pos: (req.pos & 0xFFFF_0000) | u32::from(reflected_y(m, req.pos as u16)),
            shrink_v: D::REFLECTION_SHRINK_V,
            ..req
        };
        enqueue(m, reflected, mode ^ FLIP_V);
    }
    enqueue(m, req, mode);
}

const FLIP_H: u8 = 1;
const FLIP_V: u8 = 2;
const MODE_BACK: u8 = 4;

/// ¿Hay que dibujar el reflejo? Sólo en las escenas con agua, con la
/// cámara al principio del nivel, si debajo del punto de la tarea está el
/// tile de agua. Original: `Sprite_DispatchSplashHook_05b1b2`.
fn over_water(m: &mut impl Memory, task: u32) -> bool {
    if m.read_u8(D::WATER_OFF) != 0 || m.read_u16(A::CAMERA_X) as i16 > D::WATER_CAMERA_MAX {
        return false;
    }
    let x = m.read_u16(task + A::X);
    let y = m.read_u16(task + A::Y).wrapping_add(D::WATER_PROBE_DY);
    collmap_cell(m, x, y) == D::WATER_CELL
}

/// Y de pantalla del reflejo de un sprite en `y`: simétrica respecto a la
/// línea del agua. Original: `Sprite_SplashScreenY_05b212`.
fn reflected_y(m: &mut impl Memory, y: u16) -> u16 {
    let (_, water) = world_to_screen(m, 0, D::WATER_LINE_Y);
    let d = y.wrapping_add(2).wrapping_sub(water);
    water.wrapping_sub(d)
}

/// Mundo → pantalla (Y hacia arriba en el mundo).
/// Original: `Viewport_CoordToScreen_096A5A`.
pub fn world_to_screen(m: &mut impl Memory, x: u16, y: u16) -> (u16, u16) {
    let cx = m.read_u16(A::CAMERA_X);
    let cy = m.read_u16(A::CAMERA_Y);
    let sx = x.wrapping_sub(cx).wrapping_add(1);
    let sy = D::SCREEN_HEIGHT.wrapping_sub(y.wrapping_sub(cy)).wrapping_add(1);
    (sx, sy)
}

/// Celda del mapa de colisión en el punto `(x, y)` de pantalla (0 = libre).
/// Original: `CollMap_TestPoint_043F02` (el original además pone C si la
/// celda no es 0).
pub fn collmap_cell(m: &mut impl Memory, x: u16, y: u16) -> u8 {
    let c = D::COLLMAP;
    let x = x
        .wrapping_add(m.read_u16(c + D::CM_ORIGIN_X))
        .wrapping_add(m.read_u16(c + D::CM_TILE_X) << 4);
    let y = m
        .read_u16(c + D::CM_ORIGIN_Y)
        .wrapping_sub(y)
        .wrapping_add(m.read_u16(c + D::CM_TILE_Y) << 4);
    let index = ((x & 0x1F0) << 3) | ((y & 0x1F8) >> 2) | ((x & 8) >> 3);
    m.read_u8(c + D::CM_CELLS + u32::from(index))
}

/// Añade la clave a su cola y escribe los registros de columna; si no se
/// escribe ninguno, deshace la clave. Original: `Sprite_Dispatch_05A9D6`
/// desde `$05A9EA`.
fn enqueue(m: &mut impl Memory, req: Request, mode: u8) {
    if req.gfx & 0x8000_0000 != 0 {
        return;
    }
    let head = m.read_u16(S::DRAW_COUNT);
    if head >= D::RECORDS_BYTES {
        return;
    }
    let front = m.read_u16(S::QUEUE_FILL[0]);
    let back = m.read_u16(S::QUEUE_FILL[1]);
    if front >= back {
        return;
    }
    let entry = (u32::from(req.key) << 16) | u32::from(head);
    let is_back = mode & MODE_BACK != 0;
    if is_back {
        let back = back - S::SORT_KEY_SIZE as u16;
        m.write_u32(S::SORT_KEYS + u32::from(back), entry);
        m.write_u16(S::QUEUE_FILL[1], back);
    } else {
        m.write_u32(S::SORT_KEYS + u32::from(front), entry);
        m.write_u16(S::QUEUE_FILL[0], front + S::SORT_KEY_SIZE as u16);
    }

    let start = S::RECORDS + u32::from(head);
    let shrink = req.shrink_v & req.shrink_h != 0xFF;
    let end = write_records(m, &req, start, mode & 3, shrink);

    let written = end.wrapping_sub(S::RECORDS) as u16;
    if m.read_u16(S::DRAW_COUNT) == written {
        if is_back {
            let v = m.read_u16(S::QUEUE_FILL[1]);
            m.write_u16(S::QUEUE_FILL[1], v.wrapping_add(S::SORT_KEY_SIZE as u16));
        } else {
            let v = m.read_u16(S::QUEUE_FILL[0]);
            m.write_u16(S::QUEUE_FILL[0], v.wrapping_sub(S::SORT_KEY_SIZE as u16));
        }
    } else {
        m.write_u16(S::DRAW_COUNT, written);
    }
}

/// ¿La palabra SCB4 (X << 7) cae en la zona invisible entre la derecha de
/// la pantalla y la vuelta por la izquierda?
fn off_screen(scb4: u16) -> bool {
    (D::OFFSCREEN_FIRST..D::OFFSCREEN_END).contains(&scb4)
}

/// Escribe un registro de columna en `*rec` y avanza; devuelve si aún
/// queda sitio.
fn put(m: &mut impl Memory, rec: &mut u32, tiles: u32, attr: u16, scb2: u16, scb34: u32) -> bool {
    m.write_u32(*rec + S::R_TILES, tiles);
    m.write_u16(*rec + S::R_ATTR, attr);
    m.write_u16(*rec + S::R_SCB2, scb2);
    m.write_u32(*rec + S::R_SCB3, scb34);
    *rec += S::RECORD_SIZE;
    *rec < D::RECORDS_END
}

/// Las 8 variantes: recorre los trozos del gráfico y escribe un registro
/// por columna visible. Devuelve dónde acaba. Original:
/// `SpriteDispatchJT_05AA96`.
fn write_records(m: &mut impl Memory, req: &Request, start: u32, flip: u8, shrink: bool) -> u32 {
    let mut rec = start;
    if rec >= D::RECORDS_END {
        return rec;
    }
    let flip_h = flip & FLIP_H != 0;
    let flip_v = flip & FLIP_V != 0;
    let mut gfx = req.gfx;
    let parts = m.read_u8(gfx);
    let attr = (req.palette << 8) | u16::from(m.read_u8(gfx + 1) ^ flip);
    gfx += 2;
    let reversed = if flip_v { u32::from(S::R_REVERSED) } else { 0 };

    // Reducción: escala de cada eje, reducción horizontal por columna
    // (normal y una menos) y paso del acumulador de fracción.
    let scale_v = u32::from(req.shrink_v) + 1;
    let scale_h = u32::from(req.shrink_h) + 1;
    let step = (((req.shrink_h & 0x0F) ^ 0x0F) << 4) as u8;
    let col_shrink = (u16::from(req.shrink_h >> 4) << 8) | u16::from(req.shrink_v & 0xF0);
    let col_shrink = (u32::from(col_shrink.wrapping_sub(0x100)) << 16 | u32::from(col_shrink)) | reversed << 16 | reversed;
    if shrink {
        m.write_u8(D::SHRINK_STEP, step);
    }

    let x0 = (req.pos >> 16) as u16;
    let y0 = req.pos as u16;
    let mut left = parts;
    loop {
        let mut dx = m.read_u16(gfx);
        let mut dy = m.read_u16(gfx + 2);
        let cols = m.read_u8(gfx + 4);
        let height = m.read_u8(gfx + 5);
        gfx += 6;
        if flip_h {
            dx = !dx;
        }
        if flip_v {
            dy = dy.wrapping_neg().wrapping_add((u16::from(height) << 4).wrapping_sub(1));
        }
        let column_bytes = u32::from(height) * 2;
        let columns = u16::from(cols.wrapping_sub(1)) + 1;

        // SCB2 de cada columna (y la de «una menos») y altura en SCB3.
        let (scb2, scb3_height) = if shrink {
            let p = i32::from(dy as i16) * scale_v as i32;
            let p = if p >= 0 { p + 0xFF } else { p };
            dy = (p >> 8) as u16;
            let q = i32::from(dx as i16) * scale_h as i32;
            m.write_u8(D::SHRINK_FRACTION, q as u8);
            dx = (q >> 8) as u16;
            let v = if height < 0x11 {
                let i = (height.wrapping_sub(1) << 4) | (req.shrink_v & 0x0F);
                m.read_u8(D::SHRINK_V_TABLE + u32::from(i))
            } else {
                req.shrink_v & 0x0F
            };
            let v = (col_shrink as u8 & 0xF0) | v;
            let s = (col_shrink & 0xFF00_FF00) | u32::from(v) << 16 | u32::from(v);
            let h = (((scale_v * u32::from(height)) as u16).wrapping_add(0xFF)) >> 8;
            (s, h)
        } else {
            let s = if flip_v { 0x1FFF } else { 0x0FFF };
            (s << 16 | s, u16::from(height))
        };

        let x = dx.wrapping_add(x0) & 0x1FF;
        let y = dy.wrapping_add(y0);
        let mut scb34 = ((u32::from(y) << 16 | u32::from(x)) << 7) | u32::from(scb3_height) << 16 | u32::from(height);

        if flip_v {
            gfx += column_bytes;
        }
        for _ in 0..columns {
            let mut col = scb2 as u16;
            let mut draw = true;
            if shrink {
                let f = m.read_u8(D::SHRINK_FRACTION);
                let (nf, carry) = if flip_h { f.overflowing_add(step) } else { f.overflowing_sub(step) };
                m.write_u8(D::SHRINK_FRACTION, nf);
                if carry {
                    col = (scb2 >> 16) as u16;
                    draw = col & 0x8000 == 0;
                }
            }
            if draw {
                let width = if shrink { ((col & 0x0F00) + 0x100) >> 1 } else { D::COLUMN_STEP };
                let lo = scb34 as u16;
                let lo = if flip_h { lo.wrapping_sub(width) } else { lo };
                scb34 = (scb34 & 0xFFFF_0000) | u32::from(lo);
                if !off_screen(lo) && !put(m, &mut rec, gfx, attr, col, scb34) {
                    return finish(m, rec);
                }
                if !flip_h {
                    scb34 = (scb34 & 0xFFFF_0000) | u32::from(lo.wrapping_add(width));
                }
            }
            gfx += column_bytes;
        }
        if flip_v {
            gfx -= column_bytes;
        }
        left = left.wrapping_sub(1);
        if left == 0 {
            return finish(m, rec);
        }
    }
}

/// Marca el último registro escrito como final del objeto.
fn finish(m: &mut impl Memory, rec: u32) -> u32 {
    if rec != S::RECORDS {
        let a = rec - S::RECORD_SIZE + S::R_SCB2;
        let v = m.read_u8(a);
        m.write_u8(a, v | S::R_LAST);
    }
    rec
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::TestRam as Ram;

    fn setup(m: &mut Ram) {
        m.write_u8(D::WATER_OFF, 1);
        m.write_u16(S::DRAW_COUNT, 0);
        m.write_u16(S::QUEUE_FILL[0], 0);
        m.write_u16(S::QUEUE_FILL[1], 0x348);
    }

    /// Un trozo de 2 columnas × 3 tiles en (5, 7).
    fn gfx(m: &mut Ram, at: u32) {
        let bytes = [1u8, 0x01, 0, 5, 0, 7, 2, 3];
        for (i, b) in bytes.iter().enumerate() {
            m.write_u8(at + i as u32, *b);
        }
    }

    fn req(gfx: u32) -> Request {
        Request { pos: 0x0010_0020, key: 0x1234, palette: 0x22, shrink_v: 0xFF, shrink_h: 0xFF, flip: 0, gfx }
    }

    #[test]
    fn two_columns_front_queue() {
        let mut m = Ram::default();
        setup(&mut m);
        gfx(&mut m, 0x10_F000);
        dispatch(&mut m, 0x10_0400, req(0x10_F000), false);
        assert_eq!(m.read_u32(S::SORT_KEYS), 0x1234_0000);
        assert_eq!(m.read_u16(S::QUEUE_FILL[0]), 4);
        assert_eq!(m.read_u16(S::DRAW_COUNT), 24);
        let r = S::RECORDS;
        assert_eq!(m.read_u32(r), 0x10_F008);
        assert_eq!(m.read_u16(r + 4), 0x2201);
        assert_eq!(m.read_u16(r + 6), 0x0FFF);
        assert_eq!(m.read_u16(r + 8), (0x27 << 7) | 3);
        assert_eq!(m.read_u16(r + 10), (0x15 << 7) | 3);
        assert_eq!(m.read_u32(r + 12), 0x10_F00E);
        assert_eq!(m.read_u16(r + 12 + 6), 0x8FFF);
        assert_eq!(m.read_u16(r + 12 + 10), (0x25 << 7) | 3);
    }

    #[test]
    fn nothing_visible_undoes_key() {
        let mut m = Ram::default();
        setup(&mut m);
        gfx(&mut m, 0x10_F000);
        let mut r = req(0x10_F000);
        r.pos = 0x0160_0020; // X fuera de pantalla
        dispatch(&mut m, 0x10_0400, r, true);
        assert_eq!(m.read_u16(S::QUEUE_FILL[1]), 0x348);
        assert_eq!(m.read_u16(S::DRAW_COUNT), 0);
    }
}
