//! Estallidos y humo de corta vida: el impacto de una bala en el suelo o
//! en el aire, otro impacto en el suelo, una chispa y el humo del tubo de
//! escape del Slug (o el que cae de él). Todos acaban en el mismo bucle:
//! se mueven con el decorado y se dan de baja al acabar el guion.
//!
//! Original: `SlugFx_ExhaustOrDrop_0319f0` (con `$031A86` y el bucle
//! `$031A9C`), `SlugFx_Exhaust_031af6`, `Fx_GroundBurst_031c72`,
//! `Fx_AirBurst_031cca`, `Fx_GroundBurstB_031d26` y `Fx_SparkAnim_031d70`
//! (`asm/player/tables_fx_030696.s`).

use super::{anim, platforms, rng, tasks, trig};
use crate::layout::{burst as L, task as T};
use crate::mem::{Machine, Memory};
use crate::Flow;

fn add16(m: &mut impl Memory, a: u32, d: u16) {
    let v = m.read_u16(a).wrapping_add(d);
    m.write_u16(a, v);
}

/// Gráficos heredados si son los compartidos; si no, `own`.
fn burst_gfx(m: &mut impl Memory, t: u32, own: u16) -> u16 {
    if m.read_u16(t + L::GFX_FIELD) == L::GFX_SHARED { L::GFX_SHARED } else { own }
}

/// Desplazamiento al azar de -8 a +7 (o hasta +23 con `mask = 0x1F`).
fn jitter(m: &mut impl Memory, mask: u16) -> u16 {
    (rng::next(m).0 & mask).wrapping_sub(8)
}

fn enter_run(m: &mut impl Machine, t: u32) -> Flow {
    m.write_u32(t + T::HANDLER, L::RUN);
    run(m)
}

/// Estallido en el suelo: desplazado en X al azar, uno de dos guiones.
/// Original: `Fx_GroundBurst_031c72` / `Fx_GroundBurstB_031d26` (`b`, sin
/// gráficos heredados).
pub fn ground(m: &mut impl Machine, b: bool) -> Flow {
    let t = m.task_register();
    let gfx = if b { L::GFX_GROUND } else { burst_gfx(m, t, L::GFX_GROUND) };
    anim::alloc_entry(m, t, gfx);
    let d = jitter(m, 0xF);
    add16(m, t + L::X, d);
    let scripts = if b { L::GROUND_B_SCRIPTS } else { L::GROUND_SCRIPTS };
    anim::start(m, t, scripts[usize::from(d & 1)]);
    enter_run(m, t)
}

/// Estallido en el aire: desplazado al azar en X e Y.
/// Original: `Fx_AirBurst_031cca`.
pub fn air(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let gfx = burst_gfx(m, t, L::GFX_AIR);
    anim::alloc_entry(m, t, gfx);
    let dx = jitter(m, 0x1F);
    add16(m, t + L::X, dx);
    let dy = jitter(m, 0x1F);
    add16(m, t + L::Y, dy);
    anim::start(m, t, L::AIR_SCRIPT);
    enter_run(m, t)
}

/// Chispa. Original: `Fx_SparkAnim_031d70`.
pub fn spark(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::GFX_GROUND);
    anim::start(m, t, L::SPARK_SCRIPT);
    enter_run(m, t)
}

/// Humo del tubo de escape: en la punta del tubo según la pose y el
/// ángulo del cañón del dueño. Original: `SlugFx_Exhaust_031af6`.
pub fn exhaust(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let owner = m.read_u32(t + L::PARENT);
    exhaust_at_pipe(m, t, owner);
    smoke(m, t)
}

/// Humo del Slug: del tubo si el dueño tiene el escape activo; si no, el
/// que cae, con la capa y el guion que diga el dueño.
/// Original: `SlugFx_ExhaustOrDrop_0319f0`.
pub fn exhaust_or_drop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let owner = m.read_u32(t + L::PARENT);
    if m.read_u8(owner + L::OWNER_FLAGS) & L::OWNER_EXHAUST != 0 {
        exhaust_at_pipe(m, t, owner);
    } else {
        let k = m.read_u16(owner + L::OWNER_DROP);
        m.write_u16(t + L::LAYER_A, k);
        m.write_u16(t + L::LAYER_B, k);
        let script = m.read_u32(L::DROP_SCRIPTS + u32::from(k.wrapping_mul(4)));
        if script != T::NIL {
            anim::start(m, t, script);
        }
    }
    smoke(m, t)
}

fn exhaust_at_pipe(m: &mut impl Memory, t: u32, owner: u32) {
    let pp = m.read_u32(owner + L::OWNER_POSE);
    let pose = m.read_u32(pp);
    let (px, py) = (m.read_u16(pose + 8), m.read_u16(pose + 0xA));
    add16(m, t + L::X, px);
    add16(m, t + L::Y, py);
    let mut a = m.read_u16(owner + L::OWNER_ANGLE);
    if (a as i16) > L::ANGLE_MAX {
        a = L::ANGLE_OVER;
    }
    if (a as i16) < L::ANGLE_MIN {
        a = L::ANGLE_MIN as u16;
    }
    let (dx, dy) = trig::polar(m, a & 0xFF, L::EXHAUST_RADIUS);
    add16(m, t + L::X, dx as u16);
    add16(m, t + L::Y, dy as u16);
    anim::start(m, t, L::EXHAUST_SCRIPT);
}

/// Gráficos del humo, un poco a la izquierda, y el bucle.
/// Original: `SlugFx_ExhaustOrDrop_0319f0__L031a86`.
fn smoke(m: &mut impl Machine, t: u32) -> Flow {
    anim::alloc_entry(m, t, L::GFX_SMOKE);
    add16(m, t + L::X, L::SMOKE_SHIFT.wrapping_neg());
    enter_run(m, t)
}

/// Bucle común: con el decorado y, al acabar el guion, baja (con sus
/// hijas). Original: `SlugFx_ExhaustOrDrop_0319f0__L031a9c`.
pub fn run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if anim::step(m, t) {
        tasks::kill_children(m, t);
        return tasks::finish(m);
    }
    Flow::Return
}
