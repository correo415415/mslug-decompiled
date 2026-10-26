//! Ataque suicida del Slug (A + B sin piloto que lo salve): queda
//! destruido, sale disparado hacia delante golpeando lo que pilla (o rueda
//! cuesta abajo si no está a nivel) y, cuando se acaba la mecha, llega al
//! borde o termina el guion de daños, explota con una onda que golpea en
//! toda la pantalla.
//!
//! Original: `Slug_DestroyedSlideInit_02bbf2` (y `$02BC56`),
//! `Slug_DestroyedSlide_02bba4`, `Slug_DestroyedSlide_Loop_02bc78`
//! (`asm/slug/states_02ae3e.s`), `Slug_Destroyed_02fc70` (`$02FC8E`,
//! `$02FC98`, bucle `$02FD1A`), `Slug_SelfDestructAttack_02fe6a` (bucle
//! `$02FE76`), `Slug_BlastAttack_02ff4a`, `Slug_ClampField92_02ffb0`,
//! `Slug_UpdateDamageSprite_02fae4__L02fb04` (`asm/slug/vehicle_02dd20.s`),
//! `Slug_PhysicsB_02a760` y `Slug_PhysicsC_02a766`.

use super::{anim, hits, items, player, slug, slug_death, slug_drive, slug_hunker, slug_idle, slug_jump, sound, tasks, turret};
use crate::layout::{slug_attack as L, task as T};
use crate::mem::{Machine, Memory};
use crate::Flow;

fn set_bits(m: &mut impl Memory, a: u32, bits: u8, on: bool) {
    let v = m.read_u8(a);
    m.write_u8(a, if on { v | bits } else { v & !bits });
}

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

/// Física C: bits de nivel, tope contra paredes, paso con la Y de apoyo,
/// flechas y, con el bit 5 de `+$5A`, la Y de apoyo baja un poco.
/// Devuelve si las ruedas siguen a nivel (el original deja C = no).
/// Original: `Slug_PhysicsC_02a766` (y `Slug_PhysicsB_02a760`, que antes
/// sólo borra C).
pub fn physics_c(m: &mut impl Machine, t: u32) -> bool {
    slug_idle::update_level_bits(m, t);
    if let Some((_, dir)) = slug::wheel_against_wall(m, t) {
        slug::latch_x(m, dir as u16);
    }
    slug_idle::step_on_base(m, t);
    player::dir_flags(m, t);
    if m.read_u8(t + L::FLAGS_5A) & L::BIT_5A_5 != 0 {
        let y = m.read_u16(t + L::BASE_Y);
        m.write_u16(t + L::BASE_Y, y.wrapping_add(L::BASE_Y_DROP));
    }
    slug_idle::wheels_level(m, t)
}

/// Guion de daños de la destrucción. Devuelve si ha acabado.
/// Original: `Slug_UpdateDamageSprite_02fae4__L02fb04`.
pub fn damage_step(m: &mut impl Memory, t: u32) -> bool {
    slug::damage_script(m, t, L::DAMAGE_SCRIPT_C)
}

/// Cerca del borde derecho y fuera de la vista, la mecha no pasa de
/// `NEAR_EDGE`. Devuelve si la ha recortado. Original: `Slug_ClampField92_02ffb0`.
pub fn clamp_fuse(m: &mut impl Memory, t: u32) -> bool {
    if (m.read_u16(t + L::X) as i16) < L::NEAR_EDGE_X || !items::out_of_view_seen(m, t, None) {
        return false;
    }
    if (m.read_u8(t + L::FUSE) as i8) <= L::NEAR_EDGE {
        return false;
    }
    m.write_u8(t + L::FUSE, L::NEAR_EDGE as u8);
    true
}

/// Guion por pendiente, `+$20` y el sonido del deslizamiento.
fn slide_pose(m: &mut impl Memory, t: u32) {
    let idx = slug::slope_anim_idx(m, t);
    let script = m.read_u32(L::SLIDE_SCRIPTS + u32::from(idx) * 4);
    if script != L::NO_SCRIPT {
        anim::start(m, t, script);
    }
    m.write_u8(t + L::BYTE_20, L::SLIDE_BYTE_20.wrapping_add(idx as u8));
    sound::play(m, L::SLIDE_SOUND);
}

/// Sprite inclinado según la pendiente guardada `angle`.
fn roll_pose(m: &mut impl Memory, t: u32, angle: u16) {
    let k = (L::ANGLE_CENTER.wrapping_sub((angle as i16) >> 1) as u16).min(L::ANGLE_MAX);
    let sprite = slug::angle_sprite(m, k);
    let script = m.read_u32(L::ANGLE_SCRIPTS.wrapping_add((sprite << 2) as i16 as u32));
    if script != L::NO_SCRIPT {
        anim::start(m, t, script);
    }
    m.write_u8(t + L::BYTE_20, (k >> 2) as u8);
    m.write_u32(t + L::HITBOX_CB, L::NIL);
}

/// En los frames pares activa las cajas de embestida.
fn ram_attacks(m: &mut impl Memory, t: u32) -> bool {
    if m.read_u8(L::FRAME_COUNTER) & 1 != 0 {
        return false;
    }
    m.write_u32(t + L::ATTACKS, L::ATTACKS_RAM);
    slug::clear_flags13(m, t);
    slug::clear_flags13(m, t);
    true
}

/// Explota si ha acabado la mecha (o la ha recortado y llegado a 0) o
/// ha pasado el borde derecho; sin movimiento vertical.
fn fuse_and_edge(m: &mut impl Memory, t: u32) {
    let n = m.read_u8(t + L::FUSE);
    if n == 0 || n == 1 {
        m.write_u8(t + L::FUSE, n.saturating_sub(1));
        set_handler(m, t, L::SELF_DESTRUCT);
    } else {
        m.write_u8(t + L::FUSE, n - 1);
    }
    edge_and_stop(m, t);
}

fn edge_and_stop(m: &mut impl Memory, t: u32) {
    if (m.read_u16(t + L::X) as i16) >= L::MAX_X {
        set_handler(m, t, L::SELF_DESTRUCT);
    }
    m.write_u16(t + L::VEL_Y, 0);
    m.write_u16(t + L::ACC_Y, 0);
    slug_drive::left_alone(m, t);
}

/// Empieza el ataque: música, destruido, velocidad de salida, mecha,
/// pose y rutina extra. Original: `Slug_DestroyedSlideInit_02bbf2`.
pub fn slide_init(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    sound::play(m, L::START_MUSIC);
    slug_death::kill_init(m, t);
    m.write_u16(t + L::VEL_X, L::SLIDE_VEL_X);
    m.write_u8(t + L::FUSE, L::FUSE_FRAMES);
    slide_pose(m, t);
    m.write_u32(t + L::HANDLER_60, L::SLIDE_HANDLER_60);
    set_handler(m, t, L::SLIDE_START);
    slide_start(m)
}

/// Sin caja de golpe; cuando la mecha baja de `SLIDE_RESET` vuelve a la
/// velocidad de salida y queda en el bucle. Original: `$02BC56`.
pub fn slide_start(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u32(t + L::HITBOX_CB, L::NIL);
    if m.read_u8(t + L::FUSE) <= L::SLIDE_RESET {
        m.write_u16(t + L::VEL_X, L::SLIDE_VEL_X);
        m.write_u16(t + L::ACC_X, 0);
        set_handler(m, t, L::SLIDE_LOOP);
    }
    slide_loop(m)
}

/// De rodar a deslizarse. Original: `Slug_DestroyedSlide_02bba4`.
pub fn slide(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::VEL_X, L::SLIDE_VEL_X);
    m.write_u16(t + L::ACC_X, 0);
    slide_pose(m, t);
    set_handler(m, t, L::SLIDE_LOOP);
    slide_loop(m)
}

/// Se desliza embistiendo; si deja de estar a nivel pasa a rodar.
/// Original: `Slug_DestroyedSlide_Loop_02bc78`.
pub fn slide_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    slug::update_anim(m, t);
    if !slug_hunker::physics(m, t) {
        set_handler(m, t, L::ROLL);
    }
    anim::step(m, t);
    ram_attacks(m, t);
    hits::run_attack_list(m, t);
    if damage_step(m, t) {
        set_handler(m, t, L::SELF_DESTRUCT);
    }
    clamp_fuse(m, t);
    fuse_and_edge(m, t);
    Flow::Return
}

/// Destruido rodando: música, destruido, mecha y sonido.
/// Original: `Slug_Destroyed_02fc70`.
pub fn destroyed(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    sound::play(m, L::START_MUSIC);
    slug_death::kill_init(m, t);
    m.write_u8(t + L::FUSE, L::FUSE_FRAMES);
    sound::play(m, L::ROLL_SOUND);
    roll(m, L::SLIDE_VEL_X)
}

/// Empieza a rodar con velocidad `speed` (`$02FC8E` con `SLIDE_VEL_X`,
/// `$02FC98` parado).
pub fn roll(m: &mut impl Machine, speed: u16) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::SPEED, speed);
    set_bits(m, t + L::FLAGS_8C, L::BIT_8C_5, false);
    m.write_u16(t + L::ACC_X, 0);
    let angle = slug::terrain_slope(m, t).2;
    m.write_u16(t + L::SLOPE, angle);
    roll_pose(m, t, angle);
    m.write_u32(t + L::HANDLER_60, L::SLIDE_HANDLER_60);
    set_handler(m, t, L::ROLL_LOOP);
    roll_loop(m)
}

/// Rueda siguiendo el terreno (con la configuración `LAYOUT_BOOST`, más
/// rápido cuesta abajo), embistiendo; en llano con mecha y avanzando
/// vuelve a deslizarse. Original: `Slug_Destroyed_02fc70+$AA` (`$02FD1A`).
pub fn roll_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    set_bits(m, t + L::FLAGS_8D, L::BIT_8D_2, true);
    slug::update_anim(m, t);
    let mut v = m.read_u16(t + L::SPEED);
    if m.read_u8(L::CONTROL_LAYOUT) == L::LAYOUT_BOOST {
        let d = slug::terrain_slope(m, t).2.wrapping_sub(m.read_u16(t + L::SLOPE));
        if (d ^ v) & 0x8000 == 0 {
            v = v.wrapping_add(((v as i16) >> 1) as u16);
        }
    }
    m.write_u16(t + L::VEL_X, v);
    slug::update_steering(m, t);
    physics_c(m, t);
    let angle = slug::terrain_slope(m, t).2;
    if angle != m.read_u16(t + L::SLOPE) {
        m.write_u16(t + L::SLOPE, angle);
        roll_pose(m, t, angle);
    }
    anim::step(m, t);
    if ram_attacks(m, t) {
        hits::run_attack_list(m, t);
    }
    if !slug_jump::stored_slope_is_hill(m, t)
        && m.read_u8(t + L::FUSE) != 0
        && (m.read_u16(t + L::SPEED) as i16) > 0
    {
        set_handler(m, t, L::SLIDE);
    }
    if m.read_u8(t + L::STEPS) == 0 {
        hits::run_attack_list(m, t);
    }
    if damage_step(m, t) {
        set_handler(m, t, L::SELF_DESTRUCT);
    }
    clamp_fuse(m, t);
    let n = m.read_u8(t + L::FUSE);
    if n == 0 {
        m.write_u16(t + L::SPEED, 0);
        set_handler(m, t, L::SELF_DESTRUCT);
    } else {
        m.write_u8(t + L::FUSE, n - 1);
    }
    edge_and_stop(m, t);
    Flow::Return
}

/// Explota: golpe alrededor, efectos, la onda que barre la pantalla y a
/// apagarse. Original: `Slug_SelfDestructAttack_02fe6a`.
pub fn self_destruct(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    set_bits(m, t + L::FLAGS_8C, L::BIT_8C_5, false);
    set_handler(m, t, L::SELF_DESTRUCT_LOOP);
    self_destruct_loop(m)
}

/// Original: `$02FE76`.
pub fn self_destruct_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    slug::update_anim(m, t);
    set_bits(m, t + L::FLAGS_8D, L::BIT_8D_4, true);
    m.write_u8(L::EXPLODED, 1);
    m.write_u32(t + L::ATTACKS, L::ATTACKS_BOOM);
    slug::clear_flags13(m, t);
    slug::clear_flags13(m, t);
    hits::run_attack_list(m, t);
    m.write_u32(t + L::ATTACKS, L::NIL);
    slug::clear_flags13(m, t);
    slug_death::explode_fx(m, t);
    let c = tasks::spawn(m, L::BLAST, t);
    turret::copy_pilot(m, t, c);
    slug_death::physics_a(m, t);
    anim::step(m, t);
    set_handler(m, t, L::DEATH_FADE);
    slug_drive::left_alone(m, t);
    Flow::Return
}

/// La onda: una caja grande en el centro que golpea una vez y se va al
/// frame siguiente. Original: `Slug_BlastAttack_02ff4a` (y `$02FF86`).
pub fn blast(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::X, L::BLAST_POS.0);
    m.write_u16(t + L::Y, L::BLAST_POS.1);
    m.write_u32(t + L::ATTACKS, L::ATTACKS_BLAST);
    slug::clear_flags13(m, t);
    slug::clear_flags13(m, t);
    hits::run_attack_list(m, t);
    m.write_u32(t + L::ATTACKS, L::NIL);
    slug::clear_flags13(m, t);
    set_handler(m, t, L::BLAST_END);
    Flow::Return
}
