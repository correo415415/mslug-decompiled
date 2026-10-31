//! Barrera de la misión 1 (la que cierra el paso antes de la caseta): un
//! decorado pintado en la capa de scroll con piezas que se rompen por
//! separado. Cada pieza es una tarea hija que, a cada tanda de golpes,
//! pinta el siguiente trozo de ruina de su lista; al acabarla (o cuando la
//! barrera se rompe entera) avisa a su madre con su bit. Con las cuatro
//! piezas rotas, o si la barrera se queda sin vida, pasa a dañada y luego
//! a ruina, que deja paso a la caseta. El poste y la luz siguen a la
//! barrera y desaparecen con su ruina.
//!
//! Original: `Prop_Barrier_04e512` (`asm/enemies/vehicles/turret_car_props_04cbd4.s`),
//! `Prop_BarrierActive_04e580`, `Prop_BarrierDamaged_04e68a`,
//! `Prop_BarrierWreck_04e74c`, `Prop_BarrierPost_04ed90`,
//! `Prop_BarrierLight_04ee0a`, `Prop_BarrierLightWreck_04eeca` y
//! `Prop_BlitSequence_04f1f4` (`asm/props/fortress_04e580.s`) y
//! `Barrier_SpawnPiece01_04fb8a` … `Barrier_SpawnPieceRight_04fd2c`
//! (`asm/enemies/bosses/allen_oneil_04fa58.s`).

use super::{anim, contact, events, fort, hits, layer, manual_icons, platforms, score, scroll, slug, sound, spawner, tasks};
use crate::layout::{fortress as L, task as T};
use crate::mem::{Flow, Machine, Memory};

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

fn add16(m: &mut impl Memory, a: u32, v: u16) {
    let x = m.read_u16(a).wrapping_add(v);
    m.write_u16(a, x);
}

fn clear8(m: &mut impl Memory, a: u32, bits: u8) {
    let x = m.read_u8(a) & !bits;
    m.write_u8(a, x);
}

fn set8(m: &mut impl Memory, a: u32, bits: u8) {
    let x = m.read_u8(a) | bits;
    m.write_u8(a, x);
}

/// Señal que ha puesto la madre de `t` (`+$20`).
fn owner_signal(m: &mut impl Memory, t: u32) -> u8 {
    let o = m.read_u32(t + L::OWNER);
    m.read_u8(o + L::SIGNAL)
}

/// Hija `handler` colocada donde la tarea, desplazada (`dx`, `dy`).
fn child(m: &mut impl Memory, t: u32, handler: u32, dx: u16, dy: u16) -> u32 {
    let c = tasks::spawn(m, handler, t);
    fort::place_child(m, t, c);
    add16(m, c + L::X, dx);
    add16(m, c + L::Y, dy);
    c
}

/// Si le han dado este frame, el sonido del golpe (y quita el destello).
fn hit_flash(m: &mut impl Memory, t: u32) {
    if fort::hit_side(m, t).is_some() {
        fort::hit_sound(m, t, L::HIT_SOUNDS);
        clear8(m, t + L::FLAGS_13, L::HIT_FLASH);
    }
}

/// Un frame de la barrera mientras se le puede dar: corta la prueba de
/// llegada al borde, sigue a la escena, empuja a los jugadores, avanza el
/// guion y encaja golpes. Devuelve si ha muerto.
fn barrier_frame(m: &mut impl Machine, t: u32) -> bool {
    m.write_u8(L::EDGE_ENABLE, 0);
    platforms::move_with_scene(m, t);
    contact::touch_players(m, t);
    anim::step(m, t);
    hit_flash(m, t);
    fort::dead(m, t)
}

/// Los tres lanzadores de trozos de la barrera.
fn barrier_debris(m: &mut impl Memory, t: u32) {
    for d in L::BARRIER_DEBRIS {
        spawner::install(m, t, d);
    }
}

/// Fuera por la izquierda: se da de baja.
fn off_left(m: &mut impl Machine, t: u32) -> Flow {
    if fort::off_left(m, t) {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Pieza que se rompe por tandas: hija con su bit de aviso, su lista de
/// trozos de ruina (acabada en `NO_BLIT`), su sitio y su guion. Devuelve
/// la pieza. Originales: `Barrier_SpawnPiece*`, `Gatehouse_SpawnPiece*`.
pub fn blit_piece(m: &mut impl Memory, t: u32, p: &L::Piece) -> u32 {
    let c = tasks::spawn(m, L::BLIT_SEQUENCE, t);
    fort::place_child(m, t, c);
    m.write_u8(c + L::STATE, p.bit);
    let mut at = c + L::BLITS;
    for &b in p.blits {
        m.write_u32(at, b);
        at += 4;
    }
    m.write_u32(at, L::NO_BLIT);
    add16(m, c + L::X, p.at.0);
    add16(m, c + L::Y, p.at.1);
    anim::start(m, c, p.script);
    c
}

/// Barrera: su sitio, gráfico, guion, caja de choque y el poste.
/// Original: `Prop_Barrier_04e512`.
pub fn barrier(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let rec = m.read_u32(t + L::RECORD);
    fort::place_from_record(m, t, rec);
    anim::alloc_entry(m, t, L::BARRIER_GFX);
    anim::start(m, t, L::BARRIER_SCRIPT);
    set8(m, t + L::FLAGS, L::FLAGS_SOLID);
    m.write_u32(t + L::CONTACT_BOX, L::BARRIER_BOX);
    let (h, dx, dy) = L::POST;
    child(m, t, h, dx, dy);
    set_handler(m, t, L::BARRIER_WAIT);
    barrier_wait(m)
}

/// Quieta (sólo empuja) hasta que la cámara llega a la barrera.
/// Original: `Prop_Barrier_04e512+$4A`.
pub fn barrier_wait(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if m.read_u16(L::CAMERA_X) as i16 > L::BARRIER_WAKE_X {
        return barrier_active(m);
    }
    contact::touch_players(m, t);
    anim::step(m, t);
    Flow::Return
}

/// Ya se le puede dar: caja de golpe, vida, sus piezas, la luz y su lista
/// de sucesos. Original: `Prop_BarrierActive_04e580`.
pub fn barrier_active(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u32(t + L::HITBOX, L::BARRIER_HITBOX);
    m.write_u16(t + L::MARGIN, L::BARRIER_MARGIN);
    m.write_u16(t + L::HP, L::BARRIER_HP);
    m.write_u8(t + L::STATE, 0);
    m.write_u8(t + L::SIGNAL, 0);
    for p in &L::BARRIER_PIECES {
        blit_piece(m, t, p);
    }
    let (h, dx, dy) = L::LIGHT;
    child(m, t, h, dx, dy);
    m.write_u8(t + u32::from(L::WATCH_FLAG), 0);
    events::spawn_watch(m, L::BARRIER_WATCH, L::WATCH_FLAG);
    set_handler(m, t, L::BARRIER_LOOP);
    barrier_loop(m)
}

/// Encaja golpes; sin vida, o con las cuatro piezas rotas, avisa a las
/// piezas, suelta trozos y pasa a dañada. Original: `Prop_BarrierActive_04e580+$72`.
pub fn barrier_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let broken = if barrier_frame(m, t) {
        m.write_u8(t + L::SIGNAL, L::SIGNAL_BROKEN);
        clear8(m, t + L::FLAGS_13, L::DEAD);
        true
    } else {
        m.read_u8(t + L::STATE) == L::ALL_PIECES
    };
    if broken {
        barrier_debris(m, t);
        m.write_u8(t + L::SIGNAL, L::SIGNAL_BROKEN);
        set_handler(m, t, L::BARRIER_DAMAGED);
    }
    off_left(m, t)
}

/// Dañada: sonido, vida nueva y el primer derrumbe pintado.
/// Original: `Prop_BarrierDamaged_04e68a`.
pub fn barrier_damaged(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    sound::play(m, L::SND_PILLAR);
    m.write_u16(t + L::HP, L::BARRIER_HP);
    clear8(m, t + L::FLAGS_13, L::DEAD);
    for b in L::BARRIER_DAMAGED_BLITS {
        layer::blit(m, b);
    }
    set_handler(m, t, L::BARRIER_DAMAGED_LOOP);
    barrier_damaged_loop(m)
}

/// Segunda tanda: al romperse suelta trozos, pinta su ruina, se lo dice
/// al poste y a la luz y para el control del jugador mientras cae.
/// Original: `Prop_BarrierDamaged_04e68a+$30`.
pub fn barrier_damaged_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if barrier_frame(m, t) {
        barrier_debris(m, t);
        scroll::blit_cells(m, t, L::BARRIER_FALL_CELLS);
        m.write_u8(t + L::SIGNAL, L::SIGNAL_WRECKED);
        m.write_u8(L::SCRIPTED_PAD, 1);
        set_handler(m, t, L::BARRIER_WRECK);
    }
    off_left(m, t)
}

/// Ruina: puntos, devuelve el control, un último golpe a lo que tenga
/// encima, la ruina pintada y la caseta que aparece detrás.
/// Original: `Prop_BarrierWreck_04e74c`.
pub fn barrier_wreck(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    score::award_attacker(m, t, L::POINTS);
    m.write_u8(L::SCRIPTED_PAD, 0);
    m.write_u32(t + L::ATTACKS, L::BARRIER_WRECK_ATTACKS);
    slug::clear_flags13(m, t);
    slug::clear_flags13(m, t);
    hits::run_attack_list(m, t);
    platforms::move_with_scene(m, t);
    scroll::blit_cells(m, t, L::BARRIER_WRECK_CELLS);
    sound::play(m, L::SND_BREAK);
    for b in L::BARRIER_WRECK_BLITS {
        layer::blit(m, b);
    }
    let (h, dx, dy) = L::GATEHOUSE;
    child(m, t, h, dx, dy);
    anim::start(m, t, L::BARRIER_WRECK_SCRIPT);
    m.write_u8(t + u32::from(L::WATCH_FLAG), L::WATCH_DONE);
    m.write_u32(t + L::HITBOX, L::NO_HITBOX);
    set_handler(m, t, L::BARRIER_WRECK_LOOP);
    barrier_wreck_loop(m)
}

/// Original: `Prop_BarrierWreck_04e74c+$96`.
pub fn barrier_wreck_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    anim::step(m, t);
    off_left(m, t)
}

/// Poste de la barrera. Original: `Prop_BarrierPost_04ed90`.
pub fn post(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::POST_GFX);
    manual_icons::stop_motion(m, t);
    m.write_u16(t + L::FIELD_38, 0);
    anim::start(m, t, L::POST_SCRIPT);
    set_handler(m, t, L::POST_LOOP);
    post_loop(m)
}

/// Desaparece con la ruina de la barrera. Original: `Prop_BarrierPost_04ed90+$28`.
pub fn post_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    anim::step(m, t);
    if owner_signal(m, t) == L::SIGNAL_WRECKED {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Luz de la barrera: margen, vida y guion. Original: `Prop_BarrierLight_04ee0a`.
pub fn light(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::MARGIN, L::LIGHT_MARGIN);
    m.write_u16(t + L::HP, L::LIGHT_HP);
    anim::start(m, t, L::LIGHT_SCRIPT);
    set8(m, t + L::FLAGS, L::FLAGS_SOLID);
    set_handler(m, t, L::LIGHT_LOOP);
    light_loop(m)
}

/// Con la ruina de la barrera borra su hueco y se va.
fn light_gone(m: &mut impl Machine) -> Flow {
    layer::blit(m, L::LIGHT_GONE_BLIT);
    tasks::finish(m)
}

/// Se rompe a golpes (o de una vez con un ataque de tipo `HIT_KIND_SMASH`)
/// y pinta su ruina. Original: `Prop_BarrierLight_04ee0a+$24`.
pub fn light_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    anim::step(m, t);
    if owner_signal(m, t) == L::SIGNAL_WRECKED {
        return light_gone(m);
    }
    let broken = m.read_u8(t + L::HIT_KIND) == L::HIT_KIND_SMASH || {
        hit_flash(m, t);
        fort::dead(m, t)
    };
    if broken {
        clear8(m, t + L::FLAGS_13, L::HIT_FLASH);
        for b in L::LIGHT_WRECK_BLITS {
            layer::blit(m, b);
        }
        set_handler(m, t, L::LIGHT_WRECK);
    }
    off_left(m, t)
}

/// Luz rota: sin caja de golpe. Original: `Prop_BarrierLightWreck_04eeca`.
pub fn light_wreck(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u32(t + L::HITBOX, L::NO_HITBOX);
    set_handler(m, t, L::LIGHT_WRECK_LOOP);
    light_wreck_loop(m)
}

/// Original: `Prop_BarrierLightWreck_04eeca+$E`.
pub fn light_wreck_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if owner_signal(m, t) == L::SIGNAL_WRECKED {
        return light_gone(m);
    }
    off_left(m, t)
}

/// Pieza: empieza en su primer trozo con vida. Original: `Prop_BlitSequence_04f1f4`.
pub fn sequence(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u8(t + L::SIGNAL, 0);
    m.write_u16(t + L::HP, L::PIECE_HP);
    set_handler(m, t, L::SEQUENCE_LOOP);
    sequence_loop(m)
}

/// Cada tanda de golpes pinta el siguiente trozo; al acabar la lista, o si
/// la madre se ha roto entera, le da su bit y se va.
/// Original: `Prop_BlitSequence_04f1f4+$12`.
pub fn sequence_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let done = owner_signal(m, t) == L::SIGNAL_BROKEN || {
        platforms::move_with_scene(m, t);
        anim::step(m, t);
        hit_flash(m, t);
        fort::dead(m, t) && {
            m.write_u16(t + L::HP, L::PIECE_HP);
            clear8(m, t + L::FLAGS_13, L::DEAD);
            let k = m.read_u8(t + L::SIGNAL);
            let b = m.read_u32(t + L::BLITS + u32::from(k) * 4);
            layer::blit(m, b);
            let k = k.wrapping_add(1);
            m.write_u8(t + L::SIGNAL, k);
            m.read_u32(t + L::BLITS + u32::from(k) * 4) == L::NO_BLIT
        }
    };
    if done {
        let bit = m.read_u8(t + L::STATE);
        let o = m.read_u32(t + L::OWNER);
        set8(m, o + L::STATE, bit);
        return tasks::finish(m);
    }
    off_left(m, t)
}
