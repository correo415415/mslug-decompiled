//! Lanzador de trozos: una tarea hija que, cada cierto número de frames,
//! suelta una o varias tareas (fijas o elegidas al azar de una tabla) con
//! un desplazamiento aleatorio, siguiendo a la capa y copiando la escala
//! de su dueño. Lo usan las muertes de jefes, vehículos y soldados.
//!
//! Y las cuatro esquirlas de la explosión del Slug: salen en direcciones
//! al azar (una por cuadrante) dejando explosiones mientras haya sitio en
//! la cola de vídeo.
//!
//! Original: `Entity_InstallHandlerAndCopyXf` (`$077C7E`),
//! `Spawner_Handler_077c98` (bucle `$077CAE`),
//! `Spawner_OffsetByParent_077da6` (`asm/props/fx_explosions_platforms_crew_076000.s`),
//! `$08189C`, `$0818AA`, `Template_08121C` … `Template_081284` (bucle
//! `$0812BE`) (`asm/enemies/squad/death_handlers_080736.s`) y `$05E804`.

use super::{anim, explosion, platforms, rng, tasks, trig};
use crate::layout::{spawner as L, task as T};
use crate::mem::{Machine, Memory};
use crate::Flow;

/// Crea el lanzador hijo de `t` con la descripción `desc`. Devuelve la
/// tarea. Original: `$077C7E`.
pub fn install(m: &mut impl Memory, t: u32, desc: u32) -> u32 {
    let (c, _) = tasks::spawn_last(m, L::INIT, t);
    anim::copy_sprite_pos(m, t, c);
    m.write_u32(c + L::DESC, desc);
    c
}

fn desc(m: &mut impl Memory, t: u32) -> u32 {
    m.read_u32(t + L::DESC)
}

/// Original: `Spawner_Handler_077c98`.
pub fn init(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let d = desc(m, t);
    let period = m.read_u8(d + L::D_PERIOD);
    m.write_u8(t + L::TIMER, period);
    let count = m.read_u16(d + L::D_COUNT);
    m.write_u16(t + L::COUNT, count);
    m.write_u32(t + T::HANDLER, L::RUN);
    run(m)
}

fn neg_if(v: u16, cond: bool) -> u16 {
    if cond { v.wrapping_neg() } else { v }
}

/// Se coloca respecto al dueño con el desplazamiento de la descripción.
/// Original: `Spawner_OffsetByParent_077da6`.
pub fn offset_by_parent(m: &mut impl Memory, t: u32) {
    let d = desc(m, t);
    let o = m.read_u32(t + T::OWNER);
    let left = m.read_u8(o + L::FLIP) & 1 != 0;
    let dx = neg_if(m.read_u16(d + L::D_DX), left);
    let x = m.read_u16(o + L::X).wrapping_add(dx);
    m.write_u16(t + L::X, x);
    let dy = neg_if(m.read_u16(d + L::D_DY), left);
    let y = m.read_u16(o + L::Y).wrapping_add(dy);
    m.write_u16(t + L::Y, y);
}

/// Bucle. Original: `$077CAE`.
pub fn run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    explosion::follow_layer(m, t);
    let o = m.read_u32(t + T::OWNER);
    for f in L::SCALE {
        let v = m.read_u8(o + f);
        m.write_u8(t + f, v);
    }
    let d = desc(m, t);
    let burst = m.read_u8(t + L::TIMER) == L::BURST;
    if !burst {
        let n = m.read_u8(t + L::TIMER).wrapping_sub(1);
        m.write_u8(t + L::TIMER, n);
        if n != 0 {
            return count_down(m, t);
        }
        let p = m.read_u8(d + L::D_PERIOD);
        m.write_u8(t + L::TIMER, p);
    }
    if m.read_u8(d + L::D_FOLLOW) != 0 {
        offset_by_parent(m, t);
    }
    loop {
        let mut handler = m.read_u32(d + L::D_HANDLER);
        if m.read_u8(d + L::D_RANDOM) != 0 {
            let (k, _) = rng::below(m, L::TABLE_LEN);
            handler = m.read_u32(handler + u32::from(k.wrapping_mul(4)));
        }
        let (c, _) = tasks::spawn_last(m, handler, t);
        anim::copy_sprite_pos(m, t, c);
        let flip = m.read_u8(t + L::FLIP);
        let n = m.read_u16(d + L::D_RAND_X);
        let (r, _) = rng::below(m, n);
        let x = m.read_u16(c + L::X).wrapping_add(neg_if(r, flip & 1 != 0));
        m.write_u16(c + L::X, x);
        let n = m.read_u16(d + L::D_RAND_Y);
        let (r, _) = rng::below(m, n);
        let y = m.read_u16(c + L::Y).wrapping_add(neg_if(r, flip & 2 != 0));
        m.write_u16(c + L::Y, y);
        let b = m.read_u8(t + L::FLAGS_12) & L::FLAGS_12_KEEP;
        let cb = m.read_u8(c + L::FLAGS_12);
        m.write_u8(c + L::FLAGS_12, cb | b);
        for f in L::SCALE {
            let v = m.read_u8(t + f);
            m.write_u8(c + f, v);
        }
        if m.read_u8(t + L::TIMER) != L::BURST {
            return count_down(m, t);
        }
        let n = m.read_u16(t + L::COUNT).wrapping_sub(1);
        m.write_u16(t + L::COUNT, n);
        if n == 0 {
            return tasks::finish(m);
        }
    }
}

fn count_down(m: &mut impl Machine, t: u32) -> Flow {
    let n = m.read_u16(t + L::COUNT).wrapping_sub(1);
    m.write_u16(t + L::COUNT, n);
    if n == 0 {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Trozos de la explosión del Slug. Original: `$08189C`.
pub fn slug_debris(m: &mut impl Memory, t: u32) -> u32 {
    install(m, t, L::SLUG_DEBRIS)
}

/// Las cuatro esquirlas, hijas de `t`. Original: `$0818AA`.
pub fn shards(m: &mut impl Memory, t: u32) {
    for h in L::SHARDS {
        let c = tasks::spawn(m, h, t);
        anim::copy_sprite_pos(m, t, c);
    }
}

/// Esquirla del cuadrante `k`: dirección al azar dentro de él.
/// Original: `Template_08121C` + `k`·`$20`.
pub fn shard(m: &mut impl Machine, k: u16) -> Flow {
    let t = m.task_register();
    let (r, _) = rng::next(m);
    let angle = (r & L::QUADRANT_MASK).wrapping_add(k * L::QUADRANT);
    let (vx, vy) = trig::polar(m, angle, L::SHARD_SPEED);
    m.write_u16(t + L::VEL_X, vx as u16);
    m.write_u16(t + L::VEL_Y, vy as u16);
    m.write_u16(t + L::STEP_TIMER, L::SHARD_STEP);
    m.write_u16(t + L::STEP_PERIOD, L::SHARD_STEP);
    m.write_u16(t + L::BLASTS, L::SHARD_BLASTS);
    m.write_u32(t + T::HANDLER, L::SHARD_RUN);
    Flow::Return
}

/// Suma a una coordenada una velocidad en 1/256 con su fracción.
fn advance(m: &mut impl Memory, t: u32, pos: u32, sub: u32, v: u16) {
    let whole = ((v as i16) >> 8) as u16;
    let (s, carry) = m.read_u8(t + sub).overflowing_add(v as u8);
    m.write_u8(t + sub, s);
    let p = m.read_u16(t + pos).wrapping_add(whole).wrapping_add(u16::from(carry));
    m.write_u16(t + pos, p);
}

/// ¿Queda poco sitio en la cola de vídeo? Original: `$05E804` (C).
pub fn video_busy(m: &mut impl Memory) -> bool {
    let used = m.read_u16(L::QUEUE_USED);
    let room = m.read_u16(L::QUEUE_SPAN[0]).wrapping_sub(m.read_u16(L::QUEUE_SPAN[1])) >> 2;
    used < room
}

/// Vuelo de la esquirla: cada pocos frames avanza y deja una explosión.
/// Original: `$0812BE`.
pub fn shard_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    let n = m.read_u16(t + L::STEP_TIMER).wrapping_sub(1);
    m.write_u16(t + L::STEP_TIMER, n);
    if n != 0 {
        return Flow::Return;
    }
    let p = m.read_u16(t + L::STEP_PERIOD);
    m.write_u16(t + L::STEP_TIMER, p);
    let vx = m.read_u16(t + L::VEL_X);
    advance(m, t, L::X, L::SUB[0], vx);
    let vy = m.read_u16(t + L::VEL_Y);
    advance(m, t, L::Y, L::SUB[1], vy);
    if video_busy(m) {
        return tasks::finish(m);
    }
    let c = tasks::spawn(m, L::BLAST, t);
    anim::copy_sprite_pos(m, t, c);
    for f in L::SCALE {
        let v = m.read_u8(t + f);
        m.write_u8(c + f, v);
    }
    let n = m.read_u16(t + L::BLASTS).wrapping_sub(1);
    m.write_u16(t + L::BLASTS, n);
    if n == 0 {
        return tasks::finish(m);
    }
    Flow::Return
}
