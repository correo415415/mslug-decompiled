//! Decorados: carteles de la escena del fuerte que estampan su silueta en
//! el mapa de colisión y decorados fijos colocados por su registro de
//! aparición.
//!
//! Original: `asm/props/fortress_04e580.s` (`Prop_SignA_04f70e`,
//! `Prop_SignB_04f76c`, `Prop_Sensor_04f46a`, `FixTile_Set11C2_04fa40`),
//! `FixTile_Set11C1_04fa58` y `Prop_Static_04d6ec`
//! (`asm/enemies/vehicles/turret_car_props_04cbd4.s`), y los objetos
//! decorativos `ItemProp_Tmpl114_06063a` … `ItemProp_Wait_0606fe` y el
//! obstáculo `Obstacle095_*` (`asm/props/late_props_turrets_05e000.s`).

use super::{anim, explosion, fort, items, manual_icons, motion, pickup, platforms, scroll, sound, tasks, turret, video};
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

/// Cambia el gráfico de la entrada alta fija `FIX_TILE_ENTRY` al paso `k`
/// (0 = `$C2`, 1 = `$C1`), sin animar. Original: `FixTile_Set11C2_04fa40`
/// / `FixTile_Set11C1_04fa58` (rutinas que llaman los guiones de sprite).
pub fn fix_tile(m: &mut impl Machine, k: usize) -> Flow {
    video::start_high_entry_anim(m, L::FIX_TILE_ENTRY, L::FIX_TILE_GFX[k], L::FIX_TILE_LEN, L::FIX_TILE_MODE);
    Flow::Return
}

/// Objeto decorativo `k` (plantillas 114–118): estampa sus celdas, toma
/// el gráfico y fija su sprite. Original: `ItemProp_Tmpl114_06063a` …
/// `ItemProp_Tmpl118_0606c2`.
pub fn item_prop(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    scroll::blit_cells(m, t, L::ITEM_PROP_CELLS[k]);
    anim::alloc_entry(m, t, L::ITEM_PROP_GFX);
    m.write_u32(t + L::SPRITE, L::ITEM_PROP_SPRITES[k]);
    m.write_u32(t + T::HANDLER, L::ITEM_PROP_LOOP);
    Flow::Return
}

/// Sigue a la escena, se dibuja y, al salir de la vista, se da de baja.
/// Original: `ItemProp_Idle_0606ee`.
pub fn item_prop_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    turret::draw(m, t);
    item_prop_cull(m)
}

/// Original: `ItemProp_Wait_0606fe__L060706`.
pub fn item_prop_cull(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let mut r = m.regs();
    r.a[0] = L::ITEM_PROP_MARGINS;
    m.set_regs(r);
    if items::out_of_view(m, t, Some(L::ITEM_PROP_MARGINS)) {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Obstáculo de la plantilla 95 (`alt` = variante, `+$70`): gráfico según
/// el parámetro, guion, celdas estampadas, vida y atributos.
/// Original: `Obstacle095_SoundTable_060b74` (+$6 y +$10, la tabla de
/// gráficos va delante del código).
pub fn obstacle95(m: &mut impl Machine, alt: bool) -> Flow {
    let t = m.task_register();
    m.write_u8(t + L::OBST95_ALT, u8::from(alt));
    let k = usize::from(m.read_u8(t + L::OBST95_PARAM));
    let gfx = L::OBST95_GFX.get(k).copied().unwrap_or(L::OBST95_GFX[0]);
    anim::alloc_entry(m, t, gfx);
    anim::start(m, t, L::OBST95_SCRIPT);
    let alt = m.read_u8(t + L::OBST95_ALT) != 0;
    scroll::blit_cells(m, t, L::OBST95_CELLS[usize::from(alt)]);
    m.write_u16(t + L::HP, L::OBST95_HP);
    anim::set_attr(m, t, L::OBST95_ATTR);
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, (a & L::ATTR_KEEP) | L::OBST95_PRIORITY);
    m.write_u32(t + T::HANDLER, L::OBST95_IDLE);
    Flow::Return
}

/// Sigue a la escena; al morir (o con un ataque que lo rompe de golpe)
/// pasa a romperse y, si sólo le dan, anima el golpe.
/// Original: `Obstacle095_Idle_060bf6`.
pub fn obstacle95_idle(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if fort::dead(m, t) {
        obstacle95_wreck(m, t);
    } else {
        // `$02870A` deja en `d0` el lado del impacto.
        let side = fort::hit_side(m, t);
        let mut r = m.regs();
        r.d[0] = (r.d[0] & 0xFFFF_0000) | u32::from(side.unwrap_or(0));
        m.set_regs(r);
        if side.is_some() {
            if m.read_u8(t + L::ATTACK_KIND) == L::OBST95_BREAKER {
                obstacle95_wreck(m, t);
            } else {
                anim::start(m, t, L::OBST95_S_HIT);
            }
        }
    }
    anim::step(m, t);
    item_prop_cull(m)
}

/// Guion de romperse según el lado del impacto. Original: `Obstacle095_Wreck_060c6a`.
fn obstacle95_wreck(m: &mut impl Machine, t: u32) {
    let d = m.read_u16(t + L::X).wrapping_sub(m.read_u16(t + L::IMPACT_X));
    let mut r = m.regs();
    r.d[0] = (r.d[0] & 0xFFFF_0000) | u32::from(d);
    m.set_regs(r);
    anim::start(m, t, L::OBST95_S_WRECK[usize::from((d as i16) < 0)]);
    m.write_u32(t + T::HANDLER, L::OBST95_WRECK_LOOP);
}

/// Al acabar el guion suelta sus sprites, suena y se convierte en una
/// explosión. Original: `Obstacle_FlushMusicJmp77FD6_060c40`.
pub fn obstacle95_wreck_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if anim::step(m, t) {
        video::release_task_entries(m, t);
        sound::play(m, L::OBST95_SND_WRECK);
        return explosion::start(m, 8);
    }
    item_prop_cull(m)
}

/// Al registrarse el decorado estampa sus otras celdas.
/// Original: `Obstacle095_Register_060c9a`.
pub fn obstacle95_register(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let alt = m.read_u8(t + L::OBST95_ALT) != 0;
    scroll::blit_cells(m, t, L::OBST95_REG_CELLS[usize::from(alt)]);
    Flow::Return
}
