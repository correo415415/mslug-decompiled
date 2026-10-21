//! Cosas comunes de los objetos del suelo (comida, armas, cajas...): la
//! cola de cada frame, que da de baja el objeto cuando sale de la vista, y
//! el test de «fuera de la vista» que usan muchas entidades.
//!
//! Original: `Item_GroundTail_09a75a`, `$05DD56` / `$05DD5C`.

use super::tasks;
use crate::layout::{items as L, task as T};
use crate::mem::{Flow, Machine, Memory};

/// ¿Ha salido de la vista la entidad `t`? `margins` = {izquierda, derecha,
/// abajo, arriba} (palabras con signo) que amplían la pantalla, o `None`
/// para los de por defecto. Hasta que la entidad no ha estado en pantalla
/// (bit 7 de `FLAGS2`) nunca sale; se marca al entrar o al estar muy lejos.
/// Original: `$05DD5C`.
pub fn out_of_view(m: &mut impl Memory, t: u32, margins: Option<u32>) -> bool {
    let margins = margins.unwrap_or(L::DEFAULT_MARGINS);
    let x = m.read_u16(t + L::X) as i16;
    let y = m.read_u16(t + L::Y) as i16;
    let f = m.read_u8(t + T::FLAGS2);
    if f & L::SEEN != 0 {
        let mut w = |k: u32| m.read_u16(margins + 2 * k) as i16;
        let (l, r, b, top) = (w(0), w(1), w(2), w(3));
        let left = L::VIEW_X.0.wrapping_add(l);
        let right = L::VIEW_X.1.wrapping_add(r);
        let bottom = L::VIEW_Y.0.wrapping_sub(top);
        let upper = L::VIEW_Y.1.wrapping_sub(b);
        return x < left || x > right || y < bottom || y > upper;
    }
    let on_screen = x > L::VIEW_X.0 && x < L::VIEW_X.1 && y > L::VIEW_Y.0 && y < L::VIEW_Y.1;
    let far = x <= L::FAR_X.0 || x >= L::FAR_X.1 || y <= L::FAR_Y.0 || y >= L::FAR_Y.1;
    if on_screen || far {
        m.write_u8(t + T::FLAGS2, f | L::SEEN);
    }
    false
}

/// Como [`out_of_view`], dándola antes por vista. Original: `$05DD56`.
pub fn out_of_view_seen(m: &mut impl Memory, t: u32, margins: Option<u32>) -> bool {
    let f = m.read_u8(t + T::FLAGS2);
    m.write_u8(t + T::FLAGS2, f | L::SEEN);
    out_of_view(m, t, margins)
}

/// Cola de cada frame de un objeto del suelo: cuenta los que están en el
/// suelo y lo da de baja si sale de la vista (si `+$9F`, aunque no se haya
/// visto). Original: `Item_GroundTail_09a75a`.
pub fn ground_tail(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if m.read_u8(t + L::STATE) == L::ON_GROUND {
        let n = m.read_u8(L::ON_GROUND_COUNT);
        m.write_u8(L::ON_GROUND_COUNT, n.wrapping_add(1));
    }
    let out = if m.read_u8(t + L::ALWAYS_CULL) != 0 {
        out_of_view_seen(m, t, Some(L::MARGINS))
    } else {
        out_of_view(m, t, Some(L::MARGINS))
    };
    if out {
        return tasks::finish(m);
    }
    Flow::Return
}
