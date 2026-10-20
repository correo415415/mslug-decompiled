//! Paletas por entrada y fundidos.
//!
//! Cada entrada de la tabla de sprites (`sprites::TABLE`) tiene su paleta
//! (la misma posición en `palette::SHADOW`). En cada frame, antes de dibujar,
//! [`update_all`] recorre las entradas y, según sus flags, carga los colores
//! de la ROM, avanza un fundido de la entrada hacia otros colores o borra la
//! paleta. Si hay un fundido de toda la pantalla (a negro o a blanco), los
//! colores se oscurecen o aclaran según el nivel de cada componente.
//!
//! Original: `PalSlot_UpdateAll_002c86`, `$0133B0` (cargar), `$0133E6`
//! (borrar), `$013408` (paso del fundido de una entrada), `$013480`
//! (interpolar), `$013504` (un color), `$013624` (oscurecer), `$013694`
//! (aclarar), `$01387E` (borrar a blanco el otro banco).

use super::irq;
use crate::layout::{hw, palette as P, routine, sprites as S};
use crate::mem::{Machine, Memory};

/// Entrada `off` (desplazamiento en bytes dentro de la tabla).
fn flags(m: &mut impl Memory, off: u32) -> u8 {
    m.read_u8(S::TABLE + off)
}

fn set_flags(m: &mut impl Memory, off: u32, f: u8) {
    m.write_u8(S::TABLE + off, f);
}

/// Color Neo Geo de un color RGB555 (`v`, el original indexa con `add.w`).
fn to_neogeo(m: &mut impl Memory, v: u16) -> u16 {
    m.read_u16(P::RGB555_TO_NEOGEO + u32::from(v.wrapping_mul(2)))
}

/// Junta tres componentes como el original (`lsl.w #5` / `or.b`).
fn pack(r: u8, g: u8, b: u8) -> u16 {
    let v = (u16::from(r) << 5) | u16::from(g);
    (v << 5) | u16::from(b)
}

/// Paso `k` de la tabla de avance para una distancia `n`.
fn step_value(m: &mut impl Memory, n: u16, k: u8) -> u8 {
    let table = m.read_u32(P::STEP_TABLES + u32::from(n.wrapping_mul(4)));
    m.read_u8(table + u32::from(k))
}

/// Oscurece un color según los niveles del fundido. Original: `$013624`.
fn darken(m: &mut impl Memory, r: u8, g: u8, b: u8) -> u16 {
    let mut c = [r, g, b];
    for (k, x) in c.iter_mut().enumerate() {
        let level = m.read_u8(P::FADE_LEVEL[k]);
        *x = x.wrapping_sub(step_value(m, u16::from(*x), level));
    }
    let hi = ((u16::from(c[0]) << 5) | u16::from(c[1])) << 5;
    (u16::from(c[2]) & u16::from(P::COMPONENT_MAX)) | hi
}

/// Aclara un color según los niveles del fundido. Original: `$013694`.
fn lighten(m: &mut impl Memory, r: u8, g: u8, b: u8) -> u16 {
    let mut c = [r, g, b];
    for (k, x) in c.iter_mut().enumerate() {
        let level = m.read_u8(P::FADE_LEVEL[k]);
        let n = u16::from(P::COMPONENT_MAX.wrapping_sub(*x));
        *x = x.wrapping_add(step_value(m, n, level));
    }
    let hi = ((u16::from(c[0]) << 5) | u16::from(c[1])) << 5;
    (u16::from(c[2]) & u16::from(P::COMPONENT_MAX)) | hi
}

/// Color `k` (1..15) de la ROM en `colors`.
fn source(m: &mut impl Memory, colors: u32, k: u32) -> (u8, u8, u8) {
    let a = colors + k * P::SOURCE_COLOR_SIZE;
    (m.read_u8(a), m.read_u8(a + 1), m.read_u8(a + 2))
}

/// Paleta marcada como modificada.
fn mark(m: &mut impl Memory, off: u32) {
    m.write_u16(P::SHADOW + off, 1);
}

/// Carga los colores 1..15 de la ROM tal cual. Original: `$0133B0`.
pub fn load(m: &mut impl Memory, off: u32) {
    let colors = m.read_u32(S::TABLE + off + P::ENTRY_COLORS);
    for k in 1..P::COLORS {
        let (r, g, b) = source(m, colors, k);
        let c = to_neogeo(m, pack(r, g, b));
        m.write_u16(P::SHADOW + off + k * 2, c);
    }
    let f = flags(m, off);
    set_flags(m, off, f & !P::E_LOAD);
    mark(m, off);
}

/// Carga los colores 1..15 pasados por el fundido de pantalla.
/// Original: bucles de `PalSlot_UpdateAll` (`+$178`, `+$24A`).
fn load_faded(m: &mut impl Memory, off: u32, white: bool) {
    let colors = m.read_u32(S::TABLE + off + P::ENTRY_COLORS);
    for k in 1..P::COLORS {
        let (r, g, b) = source(m, colors, k);
        let v = if white { lighten(m, r, g, b) } else { darken(m, r, g, b) };
        let c = to_neogeo(m, v);
        m.write_u16(P::SHADOW + off + k * 2, c);
    }
    let f = flags(m, off);
    set_flags(m, off, f & !P::E_LOAD);
    mark(m, off);
}

/// Borra la paleta (colores a 0) y deja la entrada sin uso.
/// Original: `$0133E6`.
pub fn clear(m: &mut impl Memory, off: u32) {
    for k in 1..P::COLORS {
        m.write_u16(P::SHADOW + off + k * 2, 0);
    }
    set_flags(m, off, P::E_CLEARED);
    mark(m, off);
}

/// Un color a mitad de camino entre `src` y `dst` en el paso `k`; dice
/// también si ya ha llegado. Original: un tercio de `$013504`.
fn approach(m: &mut impl Memory, src: u8, dst: u8, k: u8) -> (u8, bool) {
    let diff = u16::from(dst).wrapping_sub(u16::from(src));
    if diff == 0 {
        return (dst, true);
    }
    let (n, neg) = if (diff as i16) < 0 { (diff.wrapping_neg(), true) } else { (diff, false) };
    let mut q = step_value(m, n, k);
    if neg {
        q = q.wrapping_neg();
    }
    let v = src.wrapping_add(q);
    if v == dst {
        (dst, true)
    } else {
        (v, false)
    }
}

/// Escribe en la paleta los 16 colores a mitad de camino entre los de la
/// entrada y los destino en el paso `step`; devuelve si todos han llegado.
/// Original: `$013480` + `$013504`.
fn interpolate(m: &mut impl Memory, off: u32, step: u8) -> bool {
    let src = m.read_u32(S::TABLE + off + P::ENTRY_COLORS);
    let dst = m.read_u32(S::TABLE + off + P::ENTRY_TARGET);
    let mode = m.read_u8(P::FADE_MODE);
    let mut all = true;
    // Con el paso 0 el original no calcula nada (repite el color anterior).
    let mut c = [0u8; 3];
    for k in 0..P::COLORS {
        let mut done = false;
        if step != 0 {
            let k5 = (step & P::COMPONENT_MAX).wrapping_sub(1);
            let mut reached = 0;
            for i in 0..3 {
                let a = src + k * P::SOURCE_COLOR_SIZE + i;
                let b = dst + k * P::SOURCE_COLOR_SIZE + i;
                let (s, d) = (m.read_u8(a), m.read_u8(b));
                let (v, r) = approach(m, s, d, k5);
                c[i as usize] = v;
                reached += u8::from(r);
            }
            done = reached == 3;
        }
        let [r, g, b] = c.map(|x| x & P::COMPONENT_MAX);
        let v = match mode {
            0 => pack(r, g, b),
            P::FADE_TO_WHITE => lighten(m, r, g, b),
            _ => darken(m, r, g, b),
        };
        let color = m.read_u16(P::RGB555_TO_NEOGEO + u32::from(v) * 2);
        m.write_u16(P::SHADOW + off + k * 2, color);
        all &= done;
    }
    mark(m, off);
    all
}

/// Avanza el fundido de una entrada hacia sus colores destino; al acabar,
/// los destino pasan a ser los suyos y se cargan.
/// Original: `$013408`.
pub fn fade_step(m: &mut impl Memory, off: u32) {
    let e = S::TABLE + off;
    let finished = 'step: {
        if m.read_u16(e + P::ENTRY_PERIOD) == P::NO_FADE {
            break 'step true;
        }
        let t = m.read_u16(e + P::ENTRY_TIMER).wrapping_sub(1);
        m.write_u16(e + P::ENTRY_TIMER, t);
        if t != 0 {
            break 'step false;
        }
        let period = m.read_u16(e + P::ENTRY_PERIOD);
        m.write_u16(e + P::ENTRY_TIMER, period);
        let inc = match m.read_u8(e + P::ENTRY_STEP_INC) {
            0 => 1,
            x => x,
        };
        let step = m.read_u8(e + P::ENTRY_STEP).wrapping_add(inc);
        m.write_u8(e + P::ENTRY_STEP, step);
        if step as i8 >= P::FADE_STEPS {
            break 'step true;
        }
        interpolate(m, off, step)
    };
    if finished {
        let target = m.read_u32(e + P::ENTRY_TARGET);
        m.write_u32(e + P::ENTRY_COLORS, target);
        let f = flags(m, off);
        set_flags(m, off, (f & !P::E_FADING) | P::E_LOAD);
        m.write_u16(e + P::ENTRY_PERIOD, 0);
    }
}

/// Entradas que se revisan: las `SCANNED` primeras y después la del fondo.
fn for_each_entry<M: Memory>(m: &mut M, mut f: impl FnMut(&mut M, u32)) {
    for i in 0..P::SCANNED {
        f(m, i * S::SIZE);
    }
    m.write_u8(P::BACKDROP_PASS, 0xFF);
    f(m, P::BACKDROP * S::SIZE);
}

/// Sin fundido de pantalla: fundidos de entrada, cargas y borrados.
fn update_plain(m: &mut impl Memory, off: u32) {
    let f = flags(m, off);
    if f & P::E_SKIP != 0 {
        return;
    }
    if f & P::E_FADING != 0 {
        fade_step(m, off);
        m.write_u8(P::ANY_DIRTY, 0xFF);
    }
    if flags(m, off) & P::E_LOAD != 0 {
        load(m, off);
        m.write_u8(P::ANY_DIRTY, 0xFF);
    }
    if flags(m, off) & P::E_CLEAR != 0 {
        clear(m, off);
        m.write_u8(P::ANY_DIRTY, 0xFF);
    }
}

/// Con fundido de pantalla: todas las paletas se recalculan cada frame
/// (salvo las que no lo siguen, que sólo cargan si toca).
fn update_faded(m: &mut impl Memory, off: u32, white: bool) {
    if flags(m, off) & P::E_SKIP != 0 {
        return;
    }
    if flags(m, off) & P::E_FADING != 0 {
        fade_step(m, off);
    }
    let f = flags(m, off);
    if f & P::E_FADING != 0 {
        return;
    }
    if f & P::E_CLEAR != 0 {
        clear(m, off);
    } else if f & P::E_NO_SCREEN_FADE != 0 {
        if f & P::E_LOAD != 0 {
            load(m, off);
        }
    } else {
        load_faded(m, off, white);
    }
}

/// Actualiza todas las paletas para este frame. Original:
/// `PalSlot_UpdateAll_002c86`.
pub fn update_all(m: &mut impl Machine) {
    if m.read_u8(P::FADE_STEP) != P::FADE_STEP_DONE {
        let t = m.task_register();
        irq::defer(m, routine::PAL_BANK_CLEAR_STEP, t);
    }
    m.write_u8(P::BACKDROP_PASS, 0);
    m.write_u8(P::ANY_DIRTY, 0);
    let mut mode = m.read_u8(P::FADE_MODE);
    if mode == 0 {
        let prev = m.read_u8(P::FADE_MODE_PREV);
        if prev == 0 || prev == P::FADE_PREV_IGNORED {
            for_each_entry(m, |m, off| update_plain(m, off));
            let mode = m.read_u8(P::FADE_MODE);
            m.write_u8(P::FADE_MODE_PREV, mode);
            return;
        }
        // Acaba de terminar un fundido: una pasada más con los niveles a 0
        // (los colores vuelven a ser los de la ROM).
        for a in P::FADE_LEVEL {
            m.write_u8(a, 0);
        }
        mode = prev;
    }
    let white = mode == P::FADE_TO_WHITE;
    for_each_entry(m, |m, off| {
        update_faded(m, off, white);
    });
    // El original lo marca después de cada pasada (la normal y la del fondo).
    m.write_u8(P::ANY_DIRTY, 0xFF);
    let mode = m.read_u8(P::FADE_MODE);
    m.write_u8(P::FADE_MODE_PREV, mode);
}

/// Borra a blanco 16 paletas del otro banco de la RAM de paleta (una
/// tanda por vblank, hasta las 256). Original: `$01387E`.
pub fn clear_bank_step(m: &mut impl Memory) {
    m.write_u8(hw::PAL_BANK0, 1);
    let step = u32::from(m.read_u8(P::FADE_STEP));
    let base = hw::PALETTE_RAM + step * P::BANK_CLEAR_PALETTES * P::SIZE;
    for p in 0..P::BANK_CLEAR_PALETTES {
        for k in 0..P::COLORS {
            let c = if k == 0 { 0 } else { P::WHITE };
            m.write_u16(base + p * P::SIZE + k * 2, c);
        }
    }
    m.write_u8(hw::PAL_BANK1, 1);
    let n = m.read_u8(P::FADE_STEP).wrapping_add(1);
    m.write_u8(P::FADE_STEP, if (n as i8) < P::BANK_CLEAR_STEPS as i8 { n } else { P::FADE_STEP_DONE });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::TestMachine;

    #[test]
    fn load_converts_and_marks() {
        let mut m = TestMachine::default();
        let off = 3 * S::SIZE;
        m.write_u32(S::TABLE + off + P::ENTRY_COLORS, 0x20_0000);
        // Color 1 = (1, 2, 3); la tabla de conversión devuelve 0xABCD.
        m.write_u8(0x20_0004, 1);
        m.write_u8(0x20_0005, 2);
        m.write_u8(0x20_0006, 3);
        m.write_u16(P::RGB555_TO_NEOGEO + u32::from(pack(1, 2, 3)) * 2, 0xABCD);
        set_flags(&mut m, off, P::E_LOAD);
        load(&mut m, off);
        assert_eq!(m.read_u16(P::SHADOW + off + 2), 0xABCD);
        assert_eq!(m.read_u16(P::SHADOW + off), 1);
        assert_eq!(flags(&mut m, off), 0);
    }

    #[test]
    fn bank_clear_ends_after_sixteen_steps() {
        let mut m = TestMachine::default();
        m.write_u8(P::FADE_STEP, 15);
        clear_bank_step(&mut m);
        assert_eq!(m.read_u8(P::FADE_STEP), P::FADE_STEP_DONE);
        assert_eq!(m.read_u16(hw::PALETTE_RAM + 15 * 16 * P::SIZE + 2), P::WHITE);
    }
}
