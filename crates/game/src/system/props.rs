//! Decorados: carteles de la escena del fuerte que estampan su silueta en
//! el mapa de colisión y decorados fijos colocados por su registro de
//! aparición.
//!
//! Original: `asm/props/fortress_04e580.s` (`Prop_SignA_04f70e`,
//! `Prop_SignB_04f76c`, `Prop_Sensor_04f46a`) y `Prop_Static_04d6ec`
//! (`asm/enemies/vehicles/turret_car_props_04cbd4.s`).

use super::{anim, fort, items, manual_icons, motion, pickup, platforms, scroll, tasks};
use crate::layout::{props as L, task as T};
use crate::mem::{Flow, Machine};

/// Cartel `k` (0 = A, 1 = B): sprite, colisión estampada y bucle.
/// Original: `Prop_SignA_04f70e` / `Prop_SignB_04f76c`.
pub fn sign(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::SIGN_GFX[k]);
    anim::start(m, t, L::S_SIGN[k]);
    m.write_u16(t + L::ATTR, 0);
    platforms::move_with_scene(m, t);
    scroll::blit_cells(m, t, L::SIGN_CELLS[k]);
    m.write_u32(t + T::HANDLER, L::SIGN_LOOP[k]);
    sign_loop(m)
}

/// Se mueve con la escena, anima y se da de baja al salir de la vista.
/// Original: `Prop_SignA_04f70e+$34` / `Prop_SignB_04f76c+$34`.
pub fn sign_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    anim::step(m, t);
    if items::out_of_view(m, t, Some(L::MARGINS)) {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Decorado fijo: se coloca según su registro de aparición (`+$3C`),
/// atributos por defecto, quieto y un paso con colisión para asentarlo.
/// Original: `Prop_Static_04d6ec`.
pub fn fixed(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let rec = m.read_u32(t + L::SPAWN_RECORD);
    fort::place_from_record(m, t, rec);
    m.write_u16(t + L::ATTR, L::FIXED_ATTR);
    manual_icons::stop_motion(m, t);
    // `$027CEE`: el resultado no se mira.
    let r = m.regs();
    let at = (r.d[3] as u16, r.d[4] as u16);
    motion::on_entity_pos(m, t, |m| motion::move_e_probe(m, t, at));
    fixed_start(m, t)
}

/// Como [`fixed`] pero sólo se coloca (conserva sus atributos y su
/// movimiento). Original: `Prop_Static_04d6ec+$20`.
pub fn fixed_placed(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let rec = m.read_u32(t + L::SPAWN_RECORD);
    fort::place_from_record(m, t, rec);
    fixed_start(m, t)
}

fn fixed_start(m: &mut impl Machine, t: u32) -> Flow {
    m.write_u8(t + L::SHRINK_Y, 0xFF);
    m.write_u8(t + L::SHRINK_X, 0xFF);
    m.write_u8(t + L::FLIP, 0);
    m.write_u32(t + T::HANDLER, L::FIXED_LOOP);
    fixed_loop(m)
}

/// Se mueve con la escena, anima y se da de baja al salir por la
/// izquierda. Original: `Prop_Static_04d6ec+$42`.
pub fn fixed_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    anim::step(m, t);
    if fort::off_left(m, t) {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Sensor: se coloca por su registro de aparición, con su margen, y
/// vigila. Original: `Prop_Sensor_04f46a`.
pub fn sensor(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let rec = m.read_u32(t + L::SPAWN_RECORD);
    fort::place_from_record(m, t, rec);
    m.write_u16(t + L::MARGIN, L::SENSOR_MARGIN);
    sensor_watch(m)
}

/// Vuelve a vigilar. Original: `Prop_Sensor_04f46a+$10`.
pub fn sensor_watch(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u32(t + T::HANDLER, L::SENSOR_LOOP);
    sensor_loop(m)
}

/// Con la escena; si el jugador más cercano dentro de su caja tiene el
/// bit 2 de `+$8C`, salta. Se da de baja al salir por la izquierda.
/// Original: `Prop_Sensor_04f46a+$16`.
pub fn sensor_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if let Some(p) = pickup::player_in_box(m, t, L::SENSOR_BOX) {
        // `$031FC2`.
        if m.read_u8(p + L::PLAYER_STATUS) & L::SENSOR_TRIGGER != 0 {
            m.write_u32(t + T::HANDLER, L::SENSOR_TRIP);
        }
    }
    if fort::off_left(m, t) {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Salta: su animación y vuelta a vigilar al acabarla.
/// Original: `Prop_Sensor_04f46a+$4C`.
pub fn sensor_trip(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::S_SENSOR_TRIP);
    m.write_u32(t + T::HANDLER, L::SENSOR_TRIP_LOOP);
    sensor_trip_loop(m)
}

/// Original: `Prop_Sensor_04f46a+$5E`.
pub fn sensor_trip_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if anim::step(m, t) {
        m.write_u32(t + T::HANDLER, L::SENSOR_WATCH);
    }
    if fort::off_left(m, t) {
        return tasks::finish(m);
    }
    Flow::Return
}
