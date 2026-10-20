//! Slug (SV-001): la tarea fija del tanque y las tareas que lo sueltan en
//! la escena.
//!
//! Original: `Slug_IsAlive_02acfc`, `Slug_SpawnDrop_02ae50`,
//! `Slug_SpawnDropB_02aea4`.

use super::{anim, tasks};
use crate::layout::{slug as L, task as T};
use crate::mem::{Machine, Memory};
use crate::Flow;

/// ¿Está el Slug en marcha? (su tarea tiene una rutina que no es de libre,
/// baja, espera o destruido). Original: `Slug_IsAlive_02acfc` (C = sí).
pub fn is_alive(m: &mut impl Memory) -> bool {
    let h = m.read_u32(T::SLUG + T::HANDLER);
    !L::INACTIVE_HANDLERS.contains(&h)
}

/// Suelta el Slug (si no está ya en marcha) con la variante que pide el
/// byte `DROP_KIND` de la tarea actual, en su posición, y la tarea se da de
/// baja. Con `always_finish` (variante B) se da de baja aunque el Slug ya
/// estuviera. Original: `Slug_SpawnDrop_02ae50` / `Slug_SpawnDropB_02aea4`.
pub fn spawn_drop(m: &mut impl Machine, always_finish: bool) -> Flow {
    let t = m.task_register();
    if is_alive(m) {
        return if always_finish { tasks::finish(m) } else { Flow::Return };
    }
    let kind = m.read_u8(t + L::DROP_KIND).min(L::DROP_KIND_MAX);
    m.write_u32(T::SLUG + T::HANDLER, L::DROP_VARIANTS[kind as usize]);
    anim::copy_sprite_pos(m, t, T::SLUG);
    let y = m.read_u16(t + L::Y);
    m.write_u16(T::SLUG + L::DROP_Y, y);
    tasks::finish(m)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::TestRam;

    #[test]
    fn alive_unless_inactive_handler() {
        let mut m = TestRam::default();
        for h in L::INACTIVE_HANDLERS {
            m.write_u32(T::SLUG + T::HANDLER, h);
            assert!(!is_alive(&mut m));
        }
        m.write_u32(T::SLUG + T::HANDLER, L::DROP_VARIANTS[0]);
        assert!(is_alive(&mut m));
    }
}
