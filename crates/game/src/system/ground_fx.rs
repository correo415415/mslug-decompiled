//! Efecto de suelo bajo los pies de una entidad (polvo, salpicaduras): una
//! tarea hija que sigue a su dueña y mira el material de la celda que pisa
//! (los dos bits altos del mapa). Al aterrizar suelta una animación de
//! impacto; mientras anda sobre un material con efecto, la anima según
//! hacia dónde se mueve; al despegar deja una copia suelta que acaba sola.
//!
//! Sólo existe en las escenas 0 y 9; en las demás se da de baja al nacer.
//!
//! Original: `MuzzleFx_InitA_077686` … `MuzzleFx_InitE_07773e`,
//! `MuzzleFx_SceneGate_077640`, `MuzzleFx_SndByScene_077670`,
//! `MuzzleFx_Track_0778a2`, `MuzzleFx_WaitAim_0778fc`, `MuzzleFx_Flash_077912`,
//! `MuzzleFx_Attach_07776c`, `MuzzleFx_Detached_077a16`,
//! `MuzzleFx_SetFlipByDir_077bae`, `MuzzleFx_ReadParentAim_077bc6`,
//! `MuzzleFx_ClampGroundY_077c28`, `Entity_TestSolidAtFeet_027fac`
//! (`asm/props/fx_explosions_platforms_crew_076000.s`).

use super::{anim, collision, items, platforms, slope, tasks};
use crate::layout::{ground_fx as L, task as T};
use crate::mem::{Machine, Memory};
use crate::Flow;

fn owner(m: &mut impl Memory, t: u32) -> u32 {
    m.read_u32(t + T::OWNER)
}

/// Restos de `d3`/`d4` con los que el original prueba la máscara si no
/// mira el mapa.
fn leftover(m: &mut impl Machine) -> (u16, u16) {
    let r = m.regs();
    (r.d[3] as u16, r.d[4] as u16)
}

/// ¿Pisa suelo firme la entidad `t`? Mira la máscara sólida de la celda
/// bajo los pies (y que el tipo de tile no lo prohíba). Devuelve si pisa y
/// la celda tal cual (con los bits del material).
/// Original: `Entity_TestSolidAtFeet_027fac` (C = no pisa; `d7`).
pub fn solid_at_feet(m: &mut impl Memory, t: u32, at: (u16, u16)) -> (bool, u8) {
    let (x, y) = (m.read_u16(t + L::X), m.read_u16(t + L::Y).wrapping_sub(1));
    let tile = collision::lookup_tile(m, t, x, y);
    let (col, row) = tile.frac.unwrap_or(at);
    let on = slope::solid_bit(m, tile.record, col, row) && m.read_u8(tile.record + L::T_NO_STAND) == 0;
    (on, tile.raw)
}

/// Material bajo la dueña: lo apunta (`MATERIAL`, y la celda en `CELL` si
/// pisa) y si está en el aire (`IN_AIR`). Devuelve si está en el aire.
/// Original: `MuzzleFx_ReadParentAim_077bc6` (C).
fn read_ground(m: &mut impl Machine, t: u32) -> bool {
    let o = owner(m, t);
    let at = leftover(m);
    let (on, raw) = solid_at_feet(m, o, at);
    m.write_u8(t + L::MATERIAL, raw & L::MATERIAL_BITS);
    if on {
        m.write_u8(t + L::CELL, raw);
    }
    m.write_u8(t + L::IN_AIR, u8::from(!on));
    !on
}

fn material(m: &mut impl Memory, t: u32) -> u8 {
    m.read_u8(t + L::MATERIAL)
}

/// ¿Material sin efecto (suelo normal o el de los bits `$C0`)?
fn plain(m: &mut impl Memory, t: u32) -> bool {
    matches!(material(m, t), L::PLAIN | L::PLAIN_C0)
}

/// Celda «honda» del material `$80` (otra variante de la animación).
fn deep_cell(m: &mut impl Memory, t: u32) -> bool {
    matches!(m.read_u8(t + L::CELL), L::DEEP_A | L::DEEP_B)
}

/// Volteo del sprite según el material. Original: `MuzzleFx_SetFlipByDir_077bae`.
fn set_flip(m: &mut impl Memory, t: u32) {
    let f = m.read_u8(t + T::FLAGS);
    let flip = material(m, t) != L::MATERIAL_80;
    m.write_u8(t + T::FLAGS, if flip { f | L::FLIP } else { f & !L::FLIP });
}

/// Sobre el material `$40` (agua) el efecto se queda en la superficie.
/// Original: `MuzzleFx_ClampGroundY_077c28`.
fn clamp_to_surface(m: &mut impl Memory, t: u32) {
    if material(m, t) != L::MATERIAL_40 {
        return;
    }
    let scene = m.read_u8(L::SCENE);
    let level = if L::LOW_WATER_SCENES.contains(&scene) { L::LOW_WATER } else { L::WATER };
    let (_, y) = anim::world_to_screen(m, 0, level);
    m.write_u16(t + L::Y, y);
}

fn save_pos(m: &mut impl Memory, t: u32) {
    let (x, y) = (m.read_u16(t + L::X), m.read_u16(t + L::Y));
    m.write_u16(t + L::PREV_X, x);
    m.write_u16(t + L::PREV_Y, y);
}

fn follow(m: &mut impl Memory, t: u32) {
    let o = owner(m, t);
    let (x, y) = (m.read_u16(o + L::X), m.read_u16(o + L::Y));
    m.write_u16(t + L::X, x);
    m.write_u16(t + L::Y, y);
}

/// Arranca el guion `k` de la tabla de la tarea guardada en `table`
/// (`NO_SCRIPT` = ninguno).
fn start_from(m: &mut impl Memory, t: u32, table: u32, k: u16) {
    let s = m.read_u32(table.wrapping_add(u32::from(k) * 4));
    if s != L::NO_SCRIPT {
        anim::start(m, t, s);
    }
}

/// Si la dueña se está dando de baja, ésta también. Original: `$0007B2`.
fn tail(m: &mut impl Memory, t: u32) -> Flow {
    let o = owner(m, t);
    if m.read_u32(o + T::HANDLER) == L::OWNER_FINISHING {
        m.write_u32(t + T::HANDLER, L::FINISH);
    }
    Flow::Return
}

/// Sprites del efecto (otros en la escena 5) y su capa.
/// Original: `MuzzleFx_SndByScene_077670`.
fn gfx(m: &mut impl Memory) -> u16 {
    if m.read_u8(L::SCENE) == L::ALT_GFX_SCENE { L::ALT_GFX } else { L::GFX }
}

/// Alta de la variante `k` (sus cuatro tablas de guiones) y al seguimiento.
/// Original: `MuzzleFx_InitA_077686` … `MuzzleFx_InitE_07773e`.
pub fn start(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    if !L::SCENES.contains(&m.read_u8(L::SCENE)) {
        return tasks::finish(m);
    }
    let g = gfx(m);
    anim::alloc_entry(m, t, g);
    m.write_u16(t + L::ATTR, L::ATTR_INIT);
    for (f, v) in L::TABLE_FIELDS.into_iter().zip(L::VARIANTS[k]) {
        m.write_u32(t + f, v);
    }
    m.write_u32(t + T::HANDLER, L::TRACK);
    track(m)
}

/// Sigue a la dueña en el aire hasta que aterriza. Original: `MuzzleFx_Track_0778a2`.
pub fn track(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    save_pos(m, t);
    follow(m, t);
    if !read_ground(m, t) {
        set_flip(m, t);
        m.write_u32(t + T::HANDLER, L::LAND);
    } else if !matches!(material(m, t), L::MATERIAL_40 | L::MATERIAL_80) {
        m.write_u32(t + T::HANDLER, L::WAIT);
    }
    tail(m, t)
}

/// Espera (sin seguirla) a que la dueña aterrice. Original: `MuzzleFx_WaitAim_0778fc`.
pub fn wait(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if !read_ground(m, t) {
        set_flip(m, t);
        m.write_u32(t + T::HANDLER, L::LAND);
    }
    tail(m, t)
}

/// Aterrizaje: arranca la animación de impacto. Original: `MuzzleFx_Flash_077912`.
pub fn land(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    read_ground(m, t);
    let k = u16::from(material(m, t) == L::MATERIAL_80 && deep_cell(m, t));
    let table = m.read_u32(t + L::LAND_SCRIPTS);
    start_from(m, t, table, k);
    m.write_u32(t + T::HANDLER, L::LAND_RUN);
    land_run(m)
}

/// Mientras dura el impacto: si la dueña vuelve a despegar, deja una copia
/// suelta y vuelve al seguimiento; al acabar, pasa a la marcha.
/// Original: `MuzzleFx_Flash_077912+0x46`.
pub fn land_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    save_pos(m, t);
    follow(m, t);
    if read_ground(m, t) {
        clamp_to_surface(m, t);
        m.write_u32(t + T::HANDLER, L::TRACK);
        read_ground(m, t);
        if !plain(m, t) {
            let c = detach(m, t);
            let o = owner(m, t);
            let g = m.read_u16(o + L::GRAVITY);
            m.write_u16(c + L::GRAVITY, g);
            let cell = m.read_u16(t + L::CELL);
            m.write_u16(c + L::CELL, cell);
            let flip = m.read_u8(t + T::FLAGS) & L::FLIP;
            let f = m.read_u8(c + T::FLAGS) & !L::FLIP;
            m.write_u8(c + T::FLAGS, f | flip);
        }
    } else if !plain(m, t) {
        clamp_to_surface(m, t);
        set_flip(m, t);
        if anim::step(m, t) {
            m.write_u32(t + T::HANDLER, L::WALK);
        }
    }
    tail(m, t)
}

/// Copia suelta del efecto en la posición actual.
fn detach(m: &mut impl Machine, t: u32) -> u32 {
    let (c, _) = tasks::spawn_last(m, L::DETACHED, t);
    anim::copy_sprite_pos(m, t, c);
    c
}

/// Entrada en la marcha. Original: `MuzzleFx_Attach_07776c`.
pub fn walk_start(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::WALK_IDX, L::NO_IDX);
    m.write_u32(t + T::HANDLER, L::WALK_RUN);
    walk(m)
}

/// Marcha: la animación depende de si la dueña está quieta o va a un lado
/// (y, sobre el material `$80`, de la celda y de un tramo del mapa); si
/// despega, deja una copia suelta y vuelve al seguimiento.
/// Original: `MuzzleFx_Attach_07776c+0xC`.
pub fn walk(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    save_pos(m, t);
    read_ground(m, t);
    follow(m, t);
    clamp_to_surface(m, t);
    let o = owner(m, t);
    let vx = m.read_u16(o + L::VEL_X) as i16;
    let mut k = match vx {
        0 => 0,
        v if v > 0 => L::IDX_RIGHT,
        _ => L::IDX_LEFT,
    };
    if material(m, t) == L::MATERIAL_80 {
        if deep_cell(m, t) {
            k += L::IDX_DEEP;
        }
        let x = m.read_u16(t + L::X).wrapping_add(m.read_u16(L::CAMERA_X));
        if (L::SHALLOW_FROM..L::SHALLOW_TO).contains(&x) {
            k += 1;
        }
    }
    if k != m.read_u16(t + L::WALK_IDX) {
        m.write_u16(t + L::WALK_IDX, k);
        let table = m.read_u32(t + L::WALK_SCRIPTS);
        start_from(m, t, table, k);
    }
    if !plain(m, t) {
        if m.read_u8(t + L::IN_AIR) != 1 {
            set_flip(m, t);
            let k = u32::from(m.read_u16(t + L::WALK_IDX));
            let s = m.read_u32(L::WALK_FRAMES + k * 4);
            m.write_u32(t + L::FRAMES, s);
            anim::step(m, t);
        } else {
            m.write_u32(t + T::HANDLER, L::TRACK);
            let c = detach(m, t);
            let cell = m.read_u16(t + L::CELL);
            m.write_u16(c + L::CELL, cell);
            let flip = m.read_u8(t + T::FLAGS) & L::FLIP;
            let f = m.read_u8(c + T::FLAGS);
            m.write_u8(c + T::FLAGS, f | flip);
        }
    }
    tail(m, t)
}

/// Copia suelta: la animación de despegue (de la tabla de su madre) hasta
/// que acaba o sale de la vista. Original: `MuzzleFx_Detached_077a16`.
pub fn detached(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let k = u16::from(m.read_u8(t + L::CELL) == L::DEEP_B);
    let o = owner(m, t);
    let table = m.read_u32(o + L::DETACH_SCRIPTS);
    start_from(m, t, table, k);
    let g = gfx(m);
    anim::alloc_entry(m, t, g);
    let a = m.read_u16(t + L::ATTR).wrapping_add(1);
    m.write_u16(t + L::ATTR, a);
    m.write_u32(t + T::HANDLER, L::DETACHED_RUN);
    detached_run(m)
}

/// Original: `MuzzleFx_Detached_077a16+0x4A`.
pub fn detached_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if anim::step(m, t) {
        m.write_u32(t + T::HANDLER, L::FINISH);
    }
    if items::out_of_view_seen(m, t, None) {
        m.write_u32(t + T::HANDLER, L::FINISH);
    }
    Flow::Return
}
