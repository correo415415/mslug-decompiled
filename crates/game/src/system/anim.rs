//! Guiones de animación de sprites.
//!
//! Cada tarea con sprite lleva un cursor en un guion de la ROM. Cada frame
//! [`step`] ejecuta órdenes hasta dibujar un fotograma: las de control
//! (esperar, saltar, bucles, cambiar campos de la tarea, sonido, llamar a
//! una rutina) siguen en el mismo frame; las de fotograma piden el dibujo
//! a [`super::sprite`] y terminan. Un fotograma se mantiene
//! los frames que diga la última espera.
//!
//! Original: `Script_SetPointer_028cd4`, `Script_DispatchOpcode_028d70` y
//! sus órdenes (`ScriptOp00..1F`, `$028CF0`), `Entity_AllocSpriteSlot_00236E`,
//! `Entity_ProbeSpriteSlot_29A8`, `$028134`/`$028108` (atributos y
//! profundidad), `$05DD02` (copiar la posición), `$0440D0` (cámara).

use crate::layout::{anim as A, sprites as Sp, task as T};
use crate::mem::{Machine, Memory};

/// Empieza el guion `script` en la tarea `t`. Original: `$028CD4`.
pub fn start(m: &mut impl Memory, t: u32, script: u32) {
    m.write_u32(t + A::CURSOR, script);
    m.write_u32(t + A::LAST_SPRITE, script);
    m.write_u8(t + A::STEPS, 0);
    m.write_u8(t + A::WAIT, 0);
    m.write_u8(t + A::LOOP, 0);
    let f = m.read_u8(t + T::FLAGS4);
    m.write_u8(t + T::FLAGS4, f | A::FLAGS4_RUNNING);
}

/// Atributos del sprite; en el rango de profundidad, ésta sale de la Y
/// (lo de abajo, delante). Original: `$028134` → `Entity_ApplyFadeShade_028108`.
pub fn set_attr(m: &mut impl Memory, t: u32, attr: u16) {
    m.write_u16(t + A::ATTR, attr);
    apply_depth(m, t);
}

/// Original: `Entity_ApplyFadeShade_028108`.
pub fn apply_depth(m: &mut impl Memory, t: u32) {
    let attr = m.read_u16(t + A::ATTR);
    if attr.wrapping_sub(A::DEPTH_ATTR_FIRST) >= A::DEPTH_ATTR_SPAN {
        return;
    }
    let y = m.read_u16(t + A::Y) & 0xFF;
    let depth = (0xFF - y) << 5;
    m.write_u16(t + A::ATTR, (attr & A::DEPTH_KEEP) | depth);
}

/// La tarea `to` se pone donde está `from` (posición, atributos, giro).
/// Original: `$05DD02`.
pub fn copy_sprite_pos(m: &mut impl Memory, from: u32, to: u32) {
    for off in [A::X, A::Y, A::ATTR] {
        let v = m.read_u16(from + off);
        m.write_u16(to + off, v);
    }
    for off in [A::FLIP, T::PRIORITY + 1] {
        let v = m.read_u8(from + off);
        m.write_u8(to + off, v);
    }
}

/// Del mundo a la pantalla. Original: `$0440D0`.
pub fn world_to_screen(m: &mut impl Memory, x: u16, y: u16) -> (u16, u16) {
    let sx = x.wrapping_sub(m.read_u16(A::CAMERA_X));
    let sy = y.wrapping_sub(m.read_u16(A::CAMERA_Y));
    (sx, A::SCREEN_Y_BASE.wrapping_sub(sy))
}

/// Dirección de los gráficos de la entrada `gfx`.
fn gfx_data(gfx: u16) -> u32 {
    u32::from(gfx) * 64 + Sp::LOW_LIST_DATA
}

fn entry_addr(e: u16) -> u32 {
    Sp::TABLE + u32::from(e) * Sp::SIZE
}

/// Si la tarea ya tiene una entrada con los gráficos `gfx`, la elige.
/// Original: `Entity_ProbeSpriteSlot_29A8`.
pub fn find_entry(m: &mut impl Memory, t: u32, gfx: u16) -> bool {
    let data = gfx_data(gfx);
    let n = m.read_u16(t + T::PAL_SLOT_COUNT);
    for k in (0..n).rev() {
        let e = m.read_u16(t + T::PAL_SLOTS + u32::from(k) * 2);
        // El original mira el bit 2 del primer byte de la tarea.
        if m.read_u32(entry_addr(e) + Sp::ENTRY_DATA) == data && m.read_u8(t) & 4 == 0 {
            m.write_u16(t + A::SPRITE_ENTRY, e);
            return true;
        }
    }
    false
}

/// Da a la tarea una entrada de sprite con los gráficos `gfx` (comparte
/// una que ya los tenga o toma una libre). Máximo 3 por tarea.
/// Original: `Entity_AllocSpriteSlot_00236E`.
pub fn alloc_entry(m: &mut impl Memory, t: u32, gfx: u16) {
    if find_entry(m, t, gfx) {
        return;
    }
    let n = m.read_u16(t + T::PAL_SLOT_COUNT);
    if n as i16 >= A::MAX_ENTRIES as i16 || m.read_u16(Sp::FREE_TOP) == 0 {
        return;
    }
    let data = gfx_data(gfx);
    let first = Sp::FREE_INDEX_BASE;
    let shared = (0..A::SHARED_SEARCH).find(|&i| {
        let e = entry_addr(first + i);
        m.read_u32(e + Sp::ENTRY_DATA) == data && m.read_u8(e) & A::ENTRY_IN_USE == 0
    });
    let i = match shared {
        Some(i) => i,
        None => {
            let top = m.read_u16(Sp::FREE_TOP);
            m.write_u16(Sp::FREE_TOP, top.wrapping_sub(1));
            let r = m.read_u16(A::FREE_READ);
            let next = r.wrapping_add(1);
            m.write_u16(A::FREE_READ, if next as i16 >= Sp::FREE_COUNT as i16 { 0 } else { next });
            u16::from(m.read_u8(Sp::FREE_STACK + u32::from(r)))
        }
    };
    let e = first.wrapping_add(i);
    let a = entry_addr(e);
    m.write_u32(a + Sp::ENTRY_DATA, data);
    let refs = m.read_u16(a + Sp::ENTRY_REFS).wrapping_add(1);
    m.write_u16(a + Sp::ENTRY_REFS, refs);
    if refs == 1 {
        m.write_u8(a, 1);
    }
    m.write_u16(t + A::SPRITE_ENTRY, e);
    m.write_u16(t + T::PAL_SLOTS + u32::from(n) * 2, e);
    m.write_u16(t + T::PAL_SLOT_COUNT, n + 1);
}

fn set_bits(m: &mut impl Memory, addr: u32, bits: u8) {
    let v = m.read_u8(addr);
    m.write_u8(addr, v | bits);
}

fn clear_bits(m: &mut impl Memory, addr: u32, bits: u8) {
    let v = m.read_u8(addr);
    m.write_u8(addr, v & !bits);
}

/// Contador que baja hasta 0 y se queda.
fn dec_sat(v: u8) -> u8 {
    v.saturating_sub(1)
}

/// Lo que se le pasa al renderizador para un fotograma.
struct Draw {
    x: u16,
    y: u16,
    attr: u16,
    entry: u16,
    flip: u8,
    gfx: u32,
}

/// Añade el fotograma a la lista de dibujo del frame.
fn draw(m: &mut impl Machine, t: u32, d: Draw) {
    let req = super::sprite::Request {
        pos: (u32::from(d.x) << 16) | u32::from(d.y.wrapping_sub(1)),
        key: d.attr,
        palette: d.entry,
        // `SHRINK_X` es en realidad la reducción vertical (escala la Y).
        shrink_v: m.read_u8(t + A::SHRINK_X),
        shrink_h: m.read_u8(t + A::SHRINK_Y),
        flip: d.flip,
        gfx: d.gfx,
    };
    let back = m.read_u8(t + T::FLAGS) & A::FLAGS_BACK_LAYER != 0;
    super::sprite::dispatch(m, t, req, back);
}

fn on_screen(x: u16, y: u16) -> bool {
    let (x, y) = (x as i16, y.wrapping_sub(1) as i16);
    x > A::CLIP_X.0 && x < A::CLIP_X.1 && y > A::CLIP_Y.0 && y < A::CLIP_Y.1
}

/// Un trozo de fotograma con desplazamiento (orden `OP_SPRITE`). Devuelve
/// la orden siguiente.
fn sprite_part(m: &mut impl Machine, t: u32, op: u32) -> u32 {
    let flags = m.read_u8(op + A::SPR_FLAGS);
    let flip = m.read_u8(t + A::FLIP) ^ flags;
    let (x, y) = if flags & A::SPR_ABSOLUTE != 0 {
        (m.read_u16(t + A::X), m.read_u16(t + A::Y))
    } else {
        let mut dx = m.read_u16(op + A::SPR_DX);
        if flip & 1 != 0 {
            dx = dx.wrapping_neg();
        }
        let mut dy = m.read_u16(op + A::SPR_DY);
        if flip & 2 != 0 {
            dy = dy.wrapping_neg();
        }
        (dx.wrapping_add(m.read_u16(t + A::X)), dy.wrapping_add(m.read_u16(t + A::Y)))
    };
    if should_draw(m, t, flags) {
        let attr = match m.read_u16(op + A::SPR_ATTR) {
            A::KEEP => m.read_u16(t + A::ATTR),
            a => a,
        };
        let entry = sprite_entry(m, t);
        let mut fl = m.read_u8(t + A::FLIP) | (m.read_u8(t + T::FLAGS) & A::FLAGS_HIDDEN);
        fl ^= flags & A::SPR_FLIP;
        let gfx = if on_screen(x, y) { m.read_u32(op + A::SPR_GFX) } else { T::NIL };
        draw(m, t, Draw { x, y, attr, entry, flip: fl, gfx });
    }
    op + if flags & A::SPR_ABSOLUTE != 0 { A::SPR_LEN_ABS } else { A::SPR_LEN_REL }
}

/// ¿Se dibuja este trozo? (parpadeo, ocultos, depuración).
fn should_draw(m: &mut impl Machine, t: u32, flags: u8) -> bool {
    if m.read_u8(t + T::TIMER_C) & A::TIMER_C_GATED != 0 {
        let st = m.read_u8(t + A::STATE_6B);
        if st & (A::STATE_6B_DRAW | A::STATE_6B_PLAYER) == 0 && m.read_u8(A::DEBUG) & A::DEBUG_DRAW_ALL == 0 {
            return false;
        }
        set_bits(m, t + T::FLAGS3, A::FLAGS3_ALT);
    }
    let tf = m.read_u8(t + T::FLAGS);
    if tf & A::FLAGS_PARITY_CHECK != 0 {
        clear_bits(m, t + T::FLAGS, A::FLAGS_PARITY_CHECK);
        if (tf ^ flags) & A::FLAGS_PARITY != 0 {
            return false;
        }
    }
    !(flags & A::SPR_SKIP_IF_HIDDEN != 0 && tf & A::FLAGS_SKIP_HIDDEN != 0)
}

/// Entrada de sprite con la que se dibuja (la alternativa si se pide).
fn sprite_entry(m: &mut impl Machine, t: u32) -> u16 {
    let mut entry = m.read_u16(t + A::SPRITE_ENTRY);
    let f3 = m.read_u8(t + T::FLAGS3);
    if f3 & (A::FLAGS3_ALT | A::FLAGS3_ALT_REQUEST) != 0
        && m.read_u8(t + T::FLAGS2) & A::FLAGS2_NO_ALT == 0
        && m.read_u16(t + A::SPRITE_ENTRY_ALT) != A::KEEP
    {
        entry = m.read_u16(t + A::SPRITE_ENTRY_ALT);
        set_bits(m, t + T::FLAGS3, A::FLAGS3_ALT);
    }
    if m.read_u8(A::DEBUG) & A::DEBUG_HITBOX != 0 && m.read_u8(t + A::STATE_6B) & A::STATE_6B_PLAYER != 0 {
        let mut r = m.regs();
        r.d[2] = (r.d[2] & 0xFFFF_0000) | u32::from(entry);
        m.call_original(A::DEBUG_ENTRY, &mut r);
        m.set_regs(r);
        entry = r.d[2] as u16;
    }
    entry
}

/// Un trozo de fotograma en la posición de la tarea (`OP_SPRITE_AT_TASK`).
fn sprite_at_task(m: &mut impl Machine, t: u32, op: u32) -> u32 {
    let x = m.read_u16(t + A::X);
    let y = m.read_u16(t + A::Y);
    let attr = m.read_u16(t + A::ATTR);
    let entry = m.read_u16(t + A::SPRITE_ENTRY);
    let flip = m.read_u8(t + A::FLIP) | (m.read_u8(t + T::FLAGS) & A::FLAGS_HIDDEN);
    let gfx = m.read_u32(op + A::SPR_GFX);
    draw(m, t, Draw { x, y, attr, entry, flip, gfx });
    let flags = m.read_u8(op + A::SPR_FLAGS);
    op + if flags & A::SPR_ABSOLUTE != 0 { A::SPR_LEN_ABS } else { A::SPR_LEN_REL }
}

/// Fotograma (uno o varios trozos seguidos) y espera. Devuelve el flag C
/// del original: `true` = el fotograma sigue (no hay espera o la orden
/// siguiente lo repite).
fn frame(m: &mut impl Machine, t: u32, mut op: u32, at_task: bool) -> bool {
    loop {
        op = if at_task { sprite_at_task(m, t, op) } else { sprite_part(m, t, op) };
        match m.read_u8(op) {
            A::OP_SPRITE_PARITY_CONT => {
                set_bits(m, t + T::FLAGS, A::FLAGS_PARITY_CHECK);
                op += 2;
                // Siempre sigue como trozo con desplazamiento.
                return frame(m, t, op, false);
            }
            A::OP_SPRITE => continue,
            _ => break,
        }
    }
    let wait = m.read_u8(t + A::WAIT);
    if wait == 0 {
        return true;
    }
    if !at_task && m.read_u8(t + T::TIMER_A) != 0 {
        return false;
    }
    let wait = wait - 1;
    m.write_u8(t + A::WAIT, wait);
    if wait != 0 {
        return false;
    }
    if m.read_u8(op) == A::OP_REPEAT {
        return true;
    }
    m.write_u32(t + A::CURSOR, op);
    false
}

/// Campo de la tarea que cambia una orden.
fn field(t: u32, off: u16) -> u32 {
    debug_assert!(off < A::TASK_FIELDS, "orden de guion con campo fuera de la tarea");
    t + u32::from(off)
}

/// Ejecuta el guion de la tarea `t` hasta el fotograma de este frame.
/// Devuelve el flag C del original. Original: `$028D70`.
pub fn step(m: &mut impl Machine, t: u32) -> bool {
    if m.read_u32(t + A::LAST_SPRITE) == T::NIL || m.read_u32(t + A::CURSOR) == T::NIL {
        return false;
    }
    let mut op = m.read_u32(t + A::CURSOR);
    if op != m.read_u32(t + A::LAST_SPRITE) {
        let s = m.read_u8(t + A::STEPS);
        if s != 0xFF {
            m.write_u8(t + A::STEPS, s + 1);
        }
    }
    // Las órdenes de control siguen en el mismo frame (`next` = orden
    // siguiente, que pasa a ser el cursor).
    for _ in 0..0x1_0000 {
        let code = m.read_u8(op);
        let arg = m.read_u8(op + 1);
        let next = match code {
            A::OP_WAIT => {
                if m.read_u8(t + A::WAIT) == 0 {
                    m.write_u8(t + A::WAIT, arg);
                }
                op + 2
            }
            A::OP_JUMP => m.read_u32(op + 2),
            A::OP_SPRITE => {
                m.write_u32(t + A::LAST_SPRITE, op);
                return frame(m, t, op, false);
            }
            A::OP_LOOP => {
                if m.read_u8(t + A::LOOP) == 0 {
                    m.write_u8(t + A::LOOP, arg.wrapping_add(1));
                }
                let left = dec_sat(m.read_u8(t + A::LOOP));
                m.write_u8(t + A::LOOP, left);
                if left != 0 {
                    m.read_u32(op + 2)
                } else {
                    op + 6
                }
            }
            A::OP_SOUND | A::OP_SOUND_STOP => {
                let routine = if code == A::OP_SOUND {
                    crate::layout::pause::SOUND_REQUEST
                } else {
                    crate::layout::pause::SOUND_STOP
                };
                let mut r = m.regs();
                r.d[0] = (r.d[0] & 0xFFFF_0000) | u32::from(m.read_u16(op + 2));
                r.a[1] = op;
                m.call_original(routine, &mut r);
                m.set_regs(r);
                op + 4
            }
            A::OP_SET_BYTE | A::OP_AND_BYTE | A::OP_OR_BYTE | A::OP_EOR_BYTE => {
                let a = field(t, m.read_u16(op + 2));
                let v = m.read_u8(a);
                let r = match code {
                    A::OP_SET_BYTE => arg,
                    A::OP_AND_BYTE => v & arg,
                    A::OP_OR_BYTE => v | arg,
                    _ => v ^ arg,
                };
                m.write_u8(a, r);
                op + 4
            }
            A::OP_SET_WORD | A::OP_AND_WORD | A::OP_OR_WORD | A::OP_EOR_WORD => {
                let a = field(t, m.read_u16(op + 4));
                let x = m.read_u16(op + 2);
                let v = m.read_u16(a);
                let r = match code {
                    A::OP_SET_WORD => x,
                    A::OP_AND_WORD => v & x,
                    A::OP_OR_WORD => v | x,
                    _ => v ^ x,
                };
                m.write_u16(a, r);
                op + 6
            }
            A::OP_SET_LONG | A::OP_AND_LONG | A::OP_OR_LONG | A::OP_EOR_LONG => {
                let a = field(t, m.read_u16(op + 6));
                let x = m.read_u32(op + 2);
                let v = m.read_u32(a);
                let r = match code {
                    A::OP_SET_LONG => x,
                    A::OP_AND_LONG => v & x,
                    A::OP_OR_LONG => v | x,
                    _ => v ^ x,
                };
                m.write_u32(a, r);
                op + 8
            }
            A::OP_CALL => {
                let routine = m.read_u32(op + 2);
                let mut r = m.regs();
                r.a[1] = routine;
                m.call_original(routine, &mut r);
                m.set_regs(r);
                op + 6
            }
            A::OP_SET_HITBOX | A::OP_SET_4C | A::OP_SET_60 => {
                let off = match code {
                    A::OP_SET_HITBOX => A::HITBOX,
                    A::OP_SET_4C => A::FIELD_4C,
                    _ => A::FIELD_60,
                };
                let v = m.read_u32(op + 2);
                m.write_u32(t + off, v);
                op + 6
            }
            A::OP_COPY_ENTRY => {
                let v = m.read_u16(t + u32::from(arg));
                m.write_u16(t + A::SPRITE_ENTRY, v);
                op + 2
            }
            A::OP_REPEAT => {
                let last = m.read_u32(t + A::LAST_SPRITE);
                return frame(m, t, last, false);
            }
            A::OP_SPRITE_PARITY => {
                set_bits(m, t + T::FLAGS, A::FLAGS_PARITY_CHECK);
                m.write_u32(t + A::LAST_SPRITE, op + 2);
                return frame(m, t, op + 2, false);
            }
            A::OP_SPRITE_PARITY_CONT => {
                set_bits(m, t + T::FLAGS, A::FLAGS_PARITY_CHECK);
                return frame(m, t, op + 2, false);
            }
            A::OP_SET_70 => {
                m.write_u32(t + T::SCRIPT_CURSOR, op + 2);
                op + 10
            }
            A::OP_WAIT_TABLE => {
                if m.read_u8(t + A::WAIT) == 0 {
                    let table = m.read_u32(t + u32::from(arg));
                    let steps = m.read_u8(t + A::STEPS);
                    let w = m.read_u8(table + u32::from(steps));
                    m.write_u8(t + A::WAIT, w);
                }
                op + 2
            }
            A::OP_INDEX_18 | A::OP_INDEX_1C => {
                let i = u32::from(m.read_u16(t + u32::from(arg)));
                let stride = if code == A::OP_INDEX_18 { 0x12 } else { 0x1C };
                op + 2 + i * stride
            }
            A::OP_BACK => {
                let back = m.read_u32(A::BACK_TABLE + u32::from(arg & 0xF) * 4);
                let to = op.wrapping_sub(back);
                m.write_u32(t + A::CURSOR, to);
                return frame(m, t, to, false);
            }
            A::OP_SPRITE_AT_TASK => {
                m.write_u32(t + A::LAST_SPRITE, op);
                return frame(m, t, op, true);
            }
            _ => {
                debug_assert!(false, "orden de guion de animación desconocida {code:#x}");
                return false;
            }
        };
        op = next;
        m.write_u32(t + A::CURSOR, op);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::TestMachine;

    #[test]
    fn wait_then_frame_holds_for_count() {
        let mut m = TestMachine::default();
        let t = 0x10_1000;
        let s = 0x2000;
        // espera 2, fotograma absoluto, salto al principio.
        m.write_u8(s, A::OP_WAIT);
        m.write_u8(s + 1, 2);
        m.write_u8(s + 2, A::OP_SPRITE);
        m.write_u8(s + 3, A::SPR_ABSOLUTE);
        m.write_u32(s + 4, 0x3000);
        m.write_u16(s + 8, A::KEEP);
        m.write_u8(s + 10, A::OP_JUMP);
        m.write_u32(s + 12, s);
        m.write_u16(t + A::X, 0x50);
        m.write_u16(t + A::Y, 0x50);
        start(&mut m, t, s);
        assert!(!step(&mut m, t));
        // El dibujo ya es Rust: no se llama a ninguna rutina original.
        assert!(m.calls.is_empty());
        assert_eq!(m.read_u32(t + A::CURSOR), s + 2);
        assert!(!step(&mut m, t));
        // Acabó la espera: el cursor pasa al salto.
        assert_eq!(m.read_u32(t + A::CURSOR), s + 10);
    }

    #[test]
    fn depth_from_y() {
        let mut m = TestMachine::default();
        let t = 0x10_1000;
        m.write_u16(t + A::Y, 0x10);
        set_attr(&mut m, t, 0x4000);
        assert_eq!(m.read_u16(t + A::ATTR), 0x4000 | ((0xFF - 0x10) << 5));
        set_attr(&mut m, t, 0x2000);
        assert_eq!(m.read_u16(t + A::ATTR), 0x2000);
    }
}
