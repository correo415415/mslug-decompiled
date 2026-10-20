//! Render de sprites: ordena la lista de dibujo del frame y escribe en la
//! VRAM (SCB1, SCB2 y SCB4) las columnas de sprite de cada objeto.
//!
//! La lista de dibujo tiene dos colas: la primera (de delante a atrás desde
//! el principio) se ordena con un montículo; la segunda (desde el final) por
//! inserción. Cada entrada apunta a una serie de registros de columna (uno
//! por sprite hardware); el último lleva la marca `R_LAST`. Cada frame una
//! cola se recorre en un sentido y al siguiente en el otro.
//!
//! Original: `SpriteQueue_SortAndRenderSCB1_05b232`,
//! `SpriteQueue_RenderRange_05b370`, `SCB1_WriteTileColumn_05b52e`.

use crate::layout::{hw, sprites as S};
use crate::mem::Memory;

/// Escribe `data` en la palabra `addr` de la VRAM (como un `move.l` a
/// `REG_VRAMADDR`).
fn vram(m: &mut impl Memory, addr: u16, data: u16) {
    m.write_u32(hw::VRAM_ADDR, (u32::from(addr) << 16) | u32::from(data));
}

/// Escribe el sprite hardware `sprite` + 1 con el registro de columna `rec`:
/// sus tiles en SCB1 (en orden o al revés; si no es de altura completa, el
/// resto de la columna a 0), y después SCB4 y SCB2.
/// Original: `SCB1_WriteTileColumn_05b52e`.
pub fn write_column(m: &mut impl Memory, sprite: u16, rec: u32) {
    let scb2 = m.read_u16(rec + S::R_SCB2);
    let scb4 = m.read_u16(rec + S::R_SCB4);
    let attr = m.read_u16(rec + S::R_ATTR);
    let reversed = scb2 & S::R_REVERSED != 0;
    let full = scb2 as u8 == S::FULL_HEIGHT;
    let mut tiles = m.read_u32(rec + S::R_TILES);
    let n = u16::from((scb4 as u8).wrapping_sub(1) & S::MAX_TILES) + 1;
    let mut addr = (sprite << 6).wrapping_add(S::SCB1_WORDS);
    for _ in 0..n {
        let tile = if reversed {
            tiles = tiles.wrapping_sub(2);
            m.read_u16(tiles)
        } else {
            let t = m.read_u16(tiles);
            tiles = tiles.wrapping_add(2);
            t
        };
        vram(m, addr, tile);
        vram(m, addr.wrapping_add(1), attr);
        addr = addr.wrapping_add(2);
    }
    if !full {
        for _ in 0..(S::SCB1_WORDS - 2 * n) {
            vram(m, addr, 0);
            addr = addr.wrapping_add(1);
        }
    }
    let n = sprite.wrapping_add(1);
    vram(m, S::SCB4.wrapping_add(n), scb4);
    vram(m, S::SCB2.wrapping_add(n), scb2);
}

/// Registro de columna al que apunta la entrada `e` de la lista.
fn first_record(m: &mut impl Memory, e: u32) -> u32 {
    let off = m.read_u16(e) as i16;
    S::RECORDS.wrapping_add(off as u32)
}

fn is_last(m: &mut impl Memory, rec: u32) -> bool {
    m.read_u8(rec + S::R_SCB2) & S::R_LAST != 0
}

/// Escribe en la VRAM las entradas `[start, end)` de la lista en la zona de
/// sprites de la cola `q`, alternando el sentido cada frame; anota hasta
/// dónde llegó. Original: `SpriteQueue_RenderRange_05b370`.
pub fn render_range(m: &mut impl Memory, q: u32, mut start: u32, mut end: u32) {
    let limit = m.read_u16(q + S::QUEUE_LIMIT);
    let active = !m.read_u8(q + S::QUEUE_ACTIVE);
    m.write_u8(q + S::QUEUE_ACTIVE, active);
    let first = m.read_u16(q + S::QUEUE_FIRST);
    let last = m.read_u16(q + S::QUEUE_LAST);
    let mut d0;
    if active & 0x80 != 0 {
        // Hacia delante, desde el primer sprite de la zona.
        d0 = first;
        'outer: while end > start {
            let mut rec = first_record(m, start);
            loop {
                if d0 >= last || d0 >= limit {
                    break 'outer;
                }
                write_column(m, d0, rec);
                d0 = d0.wrapping_add(1);
                let stop = is_last(m, rec);
                rec += S::RECORD_SIZE;
                if stop {
                    break;
                }
            }
            start += S::SORT_KEY_SIZE;
        }
    } else {
        // Hacia atrás, desde el último sprite de la zona.
        d0 = last;
        'outer: while end > start {
            end -= S::SORT_KEY_SIZE;
            let mut rec = first_record(m, end);
            loop {
                if first >= d0 || limit >= d0 {
                    break 'outer;
                }
                d0 = d0.wrapping_sub(1);
                write_column(m, d0, rec);
                let stop = is_last(m, rec);
                rec += S::RECORD_SIZE;
                if stop {
                    break;
                }
            }
        }
    }
    m.write_u16(q + S::QUEUE_REACHED, d0);
}

/// Clave en la posición `pos` del montículo (en bytes, desde 4).
fn heap_key(m: &mut impl Memory, pos: u16) -> u32 {
    m.read_u32(S::SORT_KEYS - S::SORT_KEY_SIZE + u32::from(pos))
}

fn set_heap_key(m: &mut impl Memory, pos: u16, v: u32) {
    m.write_u32(S::SORT_KEYS - S::SORT_KEY_SIZE + u32::from(pos), v);
}

/// Hunde `v` desde `pos` en un montículo de `size` bytes.
fn sift_down(m: &mut impl Memory, mut pos: u16, size: u16, v: u32) {
    let mut child = pos.wrapping_add(pos);
    loop {
        let mut c = heap_key(m, child);
        if child < size {
            let right = heap_key(m, child + 4);
            if right >= c {
                c = right;
                child += 4;
            }
        }
        if c < v {
            break;
        }
        set_heap_key(m, pos, c);
        pos = child;
        child = child.wrapping_add(child);
        if size < child {
            break;
        }
    }
    set_heap_key(m, pos, v);
}

/// Ordena de menor a mayor (sin signo) con un montículo las claves de la
/// primera cola (`size` bytes). Original: primera mitad de
/// `SpriteQueue_SortAndRenderSCB1_05b232`.
fn heap_sort(m: &mut impl Memory, mut size: u16) {
    let mut pos = (size >> 1) & 0xFFFC;
    if size == 0 || pos == 0 {
        return;
    }
    while pos != 0 {
        let v = heap_key(m, pos);
        sift_down(m, pos, size, v);
        pos -= 4;
    }
    loop {
        let top = heap_key(m, 4);
        let v = heap_key(m, size);
        set_heap_key(m, size, top);
        size -= 4;
        if size == 4 {
            set_heap_key(m, 4, v);
            return;
        }
        sift_down(m, 4, size, v);
    }
}

/// Ordena por inserción (sin signo) las claves de la segunda cola, que
/// empieza en el byte `from`. Original: segunda mitad de
/// `SpriteQueue_SortAndRenderSCB1_05b232`.
fn insertion_sort(m: &mut impl Memory, from: u16) {
    if from >= S::SECOND_SORT_LIMIT {
        return;
    }
    let lo = S::SORT_KEYS + u32::from(from);
    let end = S::SORT_KEYS + S::DRAW_LIST_SECOND;
    let mut next = lo + 4;
    while next < end {
        let v = m.read_u32(next);
        let mut a3 = next - 4;
        next += 4;
        let mut d1 = m.read_u32(a3);
        if d1 <= v {
            continue;
        }
        loop {
            m.write_u32(a3 + 4, d1);
            a3 -= 4;
            d1 = m.read_u32(a3);
            if lo > a3 || d1 <= v {
                break;
            }
        }
        m.write_u32(a3 + 4, v);
    }
}

/// Ordena las dos colas y escribe sus sprites (SCB1, SCB2 y SCB4) en la
/// VRAM; deja las colas listas para el vblank.
/// Original: `SpriteQueue_SortAndRenderSCB1_05b232`.
pub fn sort_and_render(m: &mut impl Memory) {
    let fill = [m.read_u16(S::QUEUE_FILL[0]), m.read_u16(S::QUEUE_FILL[1])];
    heap_sort(m, fill[0]);
    insertion_sort(m, fill[1]);
    let q = S::QUEUES[0];
    let v = m.read_u16(S::BAND_FIRST);
    m.write_u16(q + S::QUEUE_FIRST, v);
    let v = m.read_u16(S::BAND_SPLIT[0]);
    m.write_u16(q + S::QUEUE_LAST, v);
    render_range(m, q, S::DRAW_LIST, S::DRAW_LIST + u32::from(fill[0]));
    let q = S::QUEUES[1];
    let v = m.read_u16(S::BAND_END);
    m.write_u16(q + S::QUEUE_FIRST, v);
    m.write_u16(q + S::QUEUE_LAST, S::BAND_TOTAL);
    render_range(m, q, S::DRAW_LIST + u32::from(fill[1]), S::DRAW_LIST + S::DRAW_LIST_SECOND);
    m.write_u8(S::FLUSH_PENDING, 0xFF);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::TestMachine;

    #[test]
    fn heap_sort_orders_keys() {
        let mut m = TestMachine::default();
        let keys = [5u32, 1, 9, 3, 7, 2];
        for (i, k) in keys.iter().enumerate() {
            m.write_u32(S::SORT_KEYS + 4 * i as u32, *k);
        }
        heap_sort(&mut m, 4 * keys.len() as u16);
        let got: Vec<u32> = (0..keys.len()).map(|i| m.read_u32(S::SORT_KEYS + 4 * i as u32)).collect();
        assert_eq!(got, vec![1, 2, 3, 5, 7, 9]);
    }

    #[test]
    fn insertion_sort_orders_second_queue() {
        let mut m = TestMachine::default();
        let from = (S::DRAW_LIST_SECOND - 16) as u16;
        for (i, k) in [8u32, 4, 6, 1].iter().enumerate() {
            m.write_u32(S::SORT_KEYS + u32::from(from) + 4 * i as u32, *k);
        }
        insertion_sort(&mut m, from);
        let got: Vec<u32> = (0..4).map(|i| m.read_u32(S::SORT_KEYS + u32::from(from) + 4 * i)).collect();
        assert_eq!(got, vec![1, 4, 6, 8]);
    }
}
