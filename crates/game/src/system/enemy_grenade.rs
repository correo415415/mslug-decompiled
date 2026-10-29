//! Granada enemiga: la que lanzan los soldados. Sale de la mano del
//! lanzador con una de las trayectorias de la tabla (o, en el modo de
//! escena que apunta, directa hacia el pecho del jugador), vuela girando,
//! frena un poco por encima de su altura de salida y estalla al tocar
//! suelo, al darle a algo, al recibir un golpe o cuando se lo manda la
//! escena; si sale de la pantalla se pierde sin más.
//!
//! Original: `Grenade_SpawnFromThrower_055bce`, `Grenade_Task_055cd8` y
//! `Grenade_Explode_055fca` (`asm/enemies/projectiles_grenade_mortar_055b96.s`),
//! con el apuntado `$05E172`.

use super::{anim, banner, explosion, fort, hits, items, motion, pickup, slug, sound, tasks, trig, turret, video};
use crate::layout::{enemy_grenade as L, task as T};
use crate::mem::{Flow, Machine, Memory};

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

fn mode(m: &mut impl Memory) -> u8 {
    m.read_u8(L::MODE)
}

fn spins(m: &mut impl Memory) -> bool {
    L::MODE_SPIN.contains(&mode(m))
}

/// Crea la granada bajo el lanzador `thrower` (misma X, `THROW_DY` más
/// abajo, mismo espejo y trayectoria). Devuelve la tarea nueva.
/// Original: `Grenade_SpawnFromThrower_055bce`.
pub fn spawn_from(m: &mut impl Memory, thrower: u32) -> u32 {
    let g = tasks::spawn(m, L::TASK, L::PARENT);
    let x = m.read_u16(thrower + L::X);
    let y = m.read_u16(thrower + L::Y).wrapping_add(L::THROW_DY);
    m.write_u16(g + L::X, x);
    m.write_u16(g + L::Y, y);
    for f in [L::FACING, L::KIND] {
        let v = m.read_u8(thrower + f);
        m.write_u8(g + f, v);
    }
    g
}

/// Ángulo (1/256 de vuelta) hacia el pecho del jugador en la caja de
/// búsqueda; `None` si no hay ninguno. Devuelve también el jugador.
/// Original: `$05E172` (C = ninguno).
pub fn angle_to_player(m: &mut impl Memory, t: u32) -> Option<(u8, u32)> {
    let p = pickup::player_in_box(m, t, L::AIM_BOX)?;
    let dx = m.read_u16(p + L::X).wrapping_sub(m.read_u16(t + L::X));
    let dy = m.read_u16(p + L::Y).wrapping_add(L::AIM_DY).wrapping_sub(m.read_u16(t + L::Y));
    Some((banner::atan2(m, dx, dy), p))
}

/// Redondea el ángulo de apuntado a octavos de vuelta y acerca un poco a
/// la horizontal los cuatro diagonales.
fn round_angle(a: u16) -> u16 {
    let a = (a + ((a & 0x10) << 1)) & 0xE0;
    match a & 0x60 {
        0x20 => (a + 8) & 0xF8,
        0x60 => (a - 8) & 0xF8,
        _ => a,
    }
}

fn traj(kind: u8) -> u32 {
    L::TRAJ + u32::from(kind) * L::TRAJ_STRIDE
}

/// Alta: atributos, la trayectoria de la tabla (o la que apunta al
/// jugador), gráficos y guion. Original: `Grenade_Task_055cd8`.
pub fn start(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let f = m.read_u8(t + L::FLAGS_6B);
    m.write_u8(t + L::FLAGS_6B, f | L::FLAGS_6B_BIT4);
    m.write_u16(t + L::ATTR, L::ATTR_INIT);
    let row = traj(m.read_u8(t + L::KIND));
    let mut vx = m.read_u16(row);
    if m.read_u8(t + L::FACING) & L::FACING_LEFT != 0 {
        vx = vx.wrapping_neg();
    }
    m.write_u16(t + L::VEL_X, vx);
    m.write_u16(t + L::ACC_X, 0);
    for (k, f) in [L::ACC_Y, L::VEL_Y, L::MOVE].into_iter().enumerate() {
        let v = m.read_u16(row + 2 + 2 * k as u32);
        m.write_u16(t + f, v);
    }
    if mode(m) == L::MODE_AIMED {
        m.write_u8(t + L::FACING, 0);
        let Some((a, _)) = angle_to_player(m, t) else {
            return gone(m);
        };
        let v = m.read_u16(row) as i16;
        let r = v.wrapping_add(v >> 2).wrapping_abs();
        let a = round_angle(u16::from(a));
        m.write_u16(t + L::ANGLE, a);
        let (x, y) = trig::polar(m, a, r);
        m.write_u16(t + L::VEL_X, x as u16);
        m.write_u16(t + L::VEL_Y, y as u16);
        m.write_u16(t + L::ACC_X, 0);
        m.write_u16(t + L::ACC_Y, 0);
    }
    for g in L::GFX {
        anim::alloc_entry(m, t, g);
    }
    let p = m.read_u16(t + L::PAL_A);
    m.write_u16(t + L::PAL, p);
    let md = mode(m);
    if spins(m) || md == L::MODE_AIMED {
        if md != L::MODE_AIMED {
            m.write_u8(t + L::FACING, 0);
        }
        let a = (m.read_u16(t + L::ANGLE) >> 3) & 0x1F;
        m.write_u16(t + L::ANGLE, a);
        anim::start(m, t, L::SCRIPT_SPIN);
    } else {
        anim::start(m, t, L::SCRIPT);
    }
    m.write_u32(t + L::ATTACKS, L::ATTACKS_FLY);
    slug::clear_flags13(m, t);
    let y = m.read_u16(t + L::Y);
    m.write_u16(t + L::START_Y, y);
    set_handler(m, t, L::FLY);
    fly(m)
}

/// Resta a `v` su 1/32 (sin llegar a dejarlo igual si no es 0, en la X).
fn brake(v: u16, at_least_one: bool) -> u16 {
    let mut d = (v as i16) >> 5;
    if at_least_one && d == 0 {
        d = 1;
    }
    v.wrapping_sub(d as u16)
}

/// Vuelo de cada frame. Original: `Grenade_Task_055cd8+$1A2`.
pub fn fly(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let r = m.regs();
    let at = (r.d[3] as u16, r.d[4] as u16);
    let mv = m.read_u16(t + L::MOVE);
    if mv == L::MOVE_ROLL {
        let f = m.read_u8(t + L::FLAGS_13);
        m.write_u8(t + L::FLAGS_13, f | L::ROLLING);
    }
    let landed = if mv == L::MOVE_ROLL || mv == 0 {
        motion::on_entity_pos(m, t, |m| slug::move_and_stop(m, t, at.1))
    } else {
        motion::on_entity_pos(m, t, |m| motion::move_e(m, t, at))
    };
    if landed {
        set_handler(m, t, L::EXPLODE);
    }
    let (x, y) = (m.read_u16(t + L::X), m.read_u16(t + L::Y));
    if turret::kind_at(m, t, x, y) == L::TILE_SOLID {
        return explode(m);
    }
    if mode(m) != L::MODE_AIMED && m.read_u16(t + L::Y) < m.read_u16(t + L::START_Y) {
        let vy = m.read_u16(t + L::VEL_Y);
        m.write_u16(t + L::VEL_Y, brake(vy, false));
        let vx = m.read_u16(t + L::VEL_X);
        if vx != 0 {
            m.write_u16(t + L::VEL_X, brake(vx, true));
        }
    }
    let i = u32::from(m.read_u8(L::FRAME) & 3) * 2;
    let field = u32::from(m.read_u16(L::PAL_FIELDS + i));
    let p = m.read_u16(t + field);
    m.write_u16(t + L::PAL, p);
    if spins(m) {
        let vx = ((m.read_u16(t + L::VEL_X) as i16) >> 4) as u16;
        let vy = ((m.read_u16(t + L::VEL_Y) as i16) >> 4) as u16;
        let a = u16::from(banner::atan2(m, vx, vy)) >> 3;
        m.write_u16(t + L::ANGLE, a);
    }
    anim::step(m, t);
    hits::run_attack_list(m, t);
    if m.read_u8(t + L::FLAGS_13) & L::ATTACK_HIT != 0 {
        set_handler(m, t, L::EXPLODE_QUIET);
    }
    if fort::hit_side(m, t).is_some() {
        set_handler(m, t, L::EXPLODE_HIT);
    }
    if m.read_u8(L::STOP_ALL) != 0 {
        set_handler(m, t, L::EXPLODE_QUIET);
    }
    if items::out_of_view_seen(m, t, Some(L::PROBE_BOX)) {
        set_handler(m, t, L::GONE);
    }
    Flow::Return
}

fn finale(m: &mut impl Machine, attr: u16, prio: u16, snd: u16) -> Flow {
    let t = m.task_register();
    anim::set_attr(m, t, attr);
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, (a & L::ATTR_PRIO_KEEP) | prio);
    sound::play(m, snd);
    blast(m)
}

/// Estalla contra el suelo. Original: `Grenade_Explode_055fca`.
pub fn explode(m: &mut impl Machine) -> Flow {
    finale(m, L::ATTR_GROUND, L::PRIO_BACK, L::SOUND)
}

/// Estalla en el aire al recibir un golpe. Original: `Grenade_Explode_055fca+$24`.
pub fn explode_hit(m: &mut impl Machine) -> Flow {
    finale(m, L::ATTR_AIR, L::PRIO_HIT, L::SOUND)
}

/// Estalla en el aire sin sonido (ha dado a algo o lo manda la escena).
/// Original: `Grenade_Explode_055fca+$48`.
pub fn explode_quiet(m: &mut impl Machine) -> Flow {
    finale(m, L::ATTR_AIR, L::PRIO_BACK, L::SOUND_NONE)
}

/// Deja de dañar, suelta sus sprites y pasa a ser la explosión.
/// Original: `Grenade_Explode_055fca+$68`.
pub fn blast(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u32(t + L::HITBOX, L::NONE);
    let f = m.read_u8(t + L::FLAGS_13);
    m.write_u8(t + L::FLAGS_13, f & !L::BLAST_CLEAR);
    video::release_task_entries(m, t);
    explosion::start(m, 1)
}

/// Se pierde: baja a sus hijas y a sí misma. Original: `$056058`.
pub fn gone(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    tasks::kill_children(m, t);
    tasks::finish(m)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rounds_to_octants() {
        assert_eq!(round_angle(0x00), 0x00);
        assert_eq!(round_angle(0x0F), 0x00);
        assert_eq!(round_angle(0x10), 0x28);
        assert_eq!(round_angle(0x40), 0x40);
        assert_eq!(round_angle(0x60), 0x58);
        assert_eq!(round_angle(0xF0), 0x00);
    }

    #[test]
    fn brake_x_never_stalls() {
        assert_eq!(brake(0x10, true), 0x0F);
        assert_eq!(brake(0x40, true), 0x3E);
        assert_eq!(brake(0xFFF0, false), 0xFFF0);
    }
}
