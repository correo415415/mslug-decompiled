//! Barrera de la misión 1 (la que cierra el paso antes de la caseta): un
//! decorado pintado en la capa de scroll con piezas que se rompen por
//! separado. Cada pieza es una tarea hija que, a cada tanda de golpes,
//! pinta el siguiente trozo de ruina de su lista; al acabarla (o cuando la
//! barrera se rompe entera) avisa a su madre con su bit. Con las cuatro
//! piezas rotas, o si la barrera se queda sin vida, pasa a dañada y luego
//! a ruina, que deja paso a la caseta. El poste y la luz siguen a la
//! barrera y desaparecen con su ruina.
//!
//! Tras la barrera viene la caseta: otro decorado de piezas (ocho, en dos
//! filas) con su tejado, su puerta y un bloqueo que no deja pasar hasta
//! su ruina; al romperla suelta un cascote que rebota y da paso a la
//! fortaleza.
//!
//! Original: `Prop_Barrier_04e512` (`asm/enemies/vehicles/turret_car_props_04cbd4.s`),
//! `Prop_BarrierActive_04e580`, `Prop_BarrierDamaged_04e68a`,
//! `Prop_BarrierWreck_04e74c`, `Prop_BarrierPost_04ed90`,
//! `Prop_BarrierLight_04ee0a`, `Prop_BarrierLightWreck_04eeca` y
//! `Prop_BlitSequence_04f1f4`, la caseta `Prop_Gatehouse_04e7fe` …
//! `Prop_GatehouseDoor_04efe6`, los bloqueos `Prop_GatehouseBlocker_04eae8`
//! y `Prop_FortressBlocker_04edda` y el cascote `Debris_Bouncer_04f5be` …
//! `Debris_BouncerRoll_04f692` (`asm/props/fortress_04e580.s`), `$044022`,
//! `Gatehouse_BlitDamaged_0501d8`,
//! `Barrier_SpawnPiece01_04fb8a` … `Barrier_SpawnPieceRight_04fd2c` y
//! `Gatehouse_SpawnPiece01_04fd8a` … `Gatehouse_SpawnPiece80_04ffd6`
//! (`asm/enemies/bosses/allen_oneil_04fa58.s`).

use super::{anim, collision, contact, events, fort, hits, items, layer, manual_icons, platforms, player_ground, rng, score, scroll, slug, sound, spawner, tasks};
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

/// Con la escena, el guion y el sonido de los golpes. Devuelve si ha
/// muerto.
fn take_hits(m: &mut impl Machine, t: u32) -> bool {
    platforms::move_with_scene(m, t);
    anim::step(m, t);
    hit_flash(m, t);
    fort::dead(m, t)
}

/// De coordenadas de pantalla (`x`, `y` hacia abajo) a las del mundo.
/// Original: `$044022`.
pub fn from_screen(m: &mut impl Memory, x: u16, y: u16) -> (u16, u16) {
    let wx = x.wrapping_sub(m.read_u16(L::SCREEN_ORIGIN.0));
    let wy = L::SCREEN_HEIGHT.wrapping_add(m.read_u16(L::SCREEN_ORIGIN.1)).wrapping_sub(y);
    (wx, wy)
}

/// Bloqueo: caja con la que chocan los jugadores hasta que su madre da la
/// señal `until`. Originales: `Prop_GatehouseBlocker_04eae8`,
/// `Prop_FortressBlocker_04edda`.
pub fn blocker(m: &mut impl Machine, b: &L::Blocker) -> Flow {
    let t = m.task_register();
    m.write_u32(t + L::CONTACT_BOX, b.contact_box);
    set_handler(m, t, b.run);
    blocker_run(m, b)
}

/// Original: `Prop_GatehouseBlocker_04eae8+$E`, `Prop_FortressBlocker_04edda+$E`.
pub fn blocker_run(m: &mut impl Machine, b: &L::Blocker) -> Flow {
    let t = m.task_register();
    contact::touch_players(m, t);
    platforms::move_with_scene(m, t);
    if owner_signal(m, t) == b.until {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Caseta: el bloqueo; espera a que llegue la cámara.
/// Original: `Prop_Gatehouse_04e7fe`.
pub fn gatehouse(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    let (h, dx, dy) = L::GATEHOUSE_BLOCKER;
    child(m, t, h, dx, dy);
    set_handler(m, t, L::GATEHOUSE_WAIT);
    gatehouse_wait(m)
}

/// Al llegar la cámara se coloca en su sitio fijo y monta todo: cascote,
/// gráfico, vida, sus ocho piezas, tejado, puerta, guion, lista de sucesos
/// y su trozo pintado. Original: `Prop_Gatehouse_04e7fe+$22`.
pub fn gatehouse_wait(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if m.read_u16(L::CAMERA_X) as i16 <= L::GATEHOUSE_WAKE_X {
        return Flow::Return;
    }
    let (x, y) = from_screen(m, L::GATEHOUSE_AT.0, L::GATEHOUSE_AT.1);
    m.write_u16(t + L::X, x);
    m.write_u16(t + L::Y, y);
    m.write_u8(t + L::STATE, 0);
    m.write_u8(t + L::SIGNAL, 0);
    let (h, dx, dy) = L::BOUNCER;
    child(m, t, h, dx, dy);
    anim::alloc_entry(m, t, L::BARRIER_GFX);
    m.write_u16(t + L::MARGIN, L::GATEHOUSE_MARGIN);
    m.write_u16(t + L::HP, L::GATEHOUSE_HP);
    set8(m, t + L::FLAGS, L::FLAGS_SOLID);
    m.write_u16(t + L::FIELD_38, L::GATEHOUSE_FIELD_38);
    for p in &L::GATEHOUSE_PIECES {
        blit_piece(m, t, p);
    }
    for h in [L::GATEHOUSE_ROOF, L::GATEHOUSE_DOOR] {
        child(m, t, h, 0, 0);
    }
    anim::start(m, t, L::GATEHOUSE_SCRIPT);
    m.write_u8(t + u32::from(L::WATCH_FLAG), 0);
    events::spawn_watch(m, L::GATEHOUSE_WATCH, L::WATCH_FLAG);
    platforms::move_with_scene(m, t);
    scroll::blit_cells(m, t, L::GATEHOUSE_CELLS);
    set_handler(m, t, L::GATEHOUSE_LOOP);
    gatehouse_loop(m)
}

/// Encaja golpes; sin vida, o con las ocho piezas rotas, avisa a sus
/// hijas, suelta trozos y pasa a dañada. Original: `Prop_Gatehouse_04e7fe+$114`.
pub fn gatehouse_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let broken = if take_hits(m, t) {
        clear8(m, t + L::FLAGS_13, L::DEAD);
        true
    } else {
        m.read_u8(t + L::STATE) == L::GATEHOUSE_ALL
    };
    if broken {
        m.write_u8(t + L::SIGNAL, L::SIGNAL_BROKEN);
        barrier_debris(m, t);
        set_handler(m, t, L::GATEHOUSE_DAMAGED);
    }
    off_left(m, t)
}

/// Los ocho trozos de la caseta dañada. Original: `Gatehouse_BlitDamaged_0501d8`.
pub fn gatehouse_blit_damaged(m: &mut impl Memory) {
    for b in L::GATEHOUSE_DAMAGED_BLITS {
        layer::blit(m, b);
    }
}

/// Dañada: sonido, vida nueva, sus trozos y su guion.
/// Original: `Prop_GatehouseDamaged_04e998`.
pub fn gatehouse_damaged(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    sound::play(m, L::SND_PILLAR);
    m.write_u16(t + L::HP, L::GATEHOUSE_HP);
    clear8(m, t + L::FLAGS_13, L::DEAD);
    gatehouse_blit_damaged(m);
    anim::start(m, t, L::GATEHOUSE_DAMAGED_SCRIPT);
    set_handler(m, t, L::GATEHOUSE_DAMAGED_LOOP);
    gatehouse_damaged_loop(m)
}

/// Segunda tanda: al romperse suelta trozos, avisa a sus hijas y para el
/// control del jugador. Original: `Prop_GatehouseDamaged_04e998+$2C`.
pub fn gatehouse_damaged_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if take_hits(m, t) {
        barrier_debris(m, t);
        m.write_u8(t + L::SIGNAL, L::SIGNAL_WRECKED);
        m.write_u8(L::SCRIPTED_PAD, 1);
        set_handler(m, t, L::GATEHOUSE_WRECK);
    }
    off_left(m, t)
}

/// Ruina: puntos, devuelve el control, sus trozos pintados y la fortaleza
/// que aparece detrás. Original: `Prop_GatehouseWreck_04ea3e`.
pub fn gatehouse_wreck(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    score::award_attacker(m, t, L::GATEHOUSE_POINTS);
    m.write_u8(L::EDGE_ENABLE, L::EDGE_AFTER_GATEHOUSE);
    m.write_u8(L::SCRIPTED_PAD, 0);
    platforms::move_with_scene(m, t);
    scroll::blit_cells(m, t, L::GATEHOUSE_WRECK_CELLS[0]);
    m.write_u32(t + L::HITBOX, L::NO_HITBOX);
    sound::play(m, L::SND_BREAK);
    scroll::blit_cells(m, t, L::GATEHOUSE_WRECK_CELLS[1]);
    for b in L::GATEHOUSE_WRECK_BLITS {
        layer::blit(m, b);
    }
    let (h, dx, dy) = L::FORTRESS;
    child(m, t, h, dx, dy);
    anim::start(m, t, L::GATEHOUSE_WRECK_SCRIPT);
    m.write_u8(t + u32::from(L::WATCH_FLAG), L::WATCH_DONE);
    set_handler(m, t, L::GATEHOUSE_WRECK_LOOP);
    gatehouse_wreck_loop(m)
}

/// Original: `Prop_GatehouseWreck_04ea3e+$8E`.
pub fn gatehouse_wreck_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    anim::step(m, t);
    off_left(m, t)
}

/// Tejado de la caseta. Original: `Prop_GatehouseRoof_04ef0a`.
pub fn roof(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::POST_GFX);
    m.write_u16(t + L::MARGIN, L::ROOF_MARGIN);
    m.write_u16(t + L::HP, L::ROOF_HP);
    anim::start(m, t, L::ROOF_SCRIPT);
    m.write_u16(t + L::FIELD_38, L::ROOF_FIELD_38);
    set_handler(m, t, L::ROOF_LOOP);
    roof_loop(m)
}

/// Se rompe a golpes o cuando la caseta se rompe entera.
/// Original: `Prop_GatehouseRoof_04ef0a+$2E`.
pub fn roof_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    anim::step(m, t);
    let broken = owner_signal(m, t) == L::SIGNAL_BROKEN || {
        hit_flash(m, t);
        fort::dead(m, t) && {
            clear8(m, t + L::FLAGS_13, L::DEAD);
            true
        }
    };
    if broken {
        layer::blit(m, L::ROOF_WRECK_BLIT);
        set_handler(m, t, L::ROOF_WRECK);
    }
    off_left(m, t)
}

/// Tejado roto. Original: `Prop_GatehouseRoofWreck_04efa2`.
pub fn roof_wreck(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::ROOF_WRECK_SCRIPT);
    m.write_u32(t + L::HITBOX, L::NO_HITBOX);
    set_handler(m, t, L::ROOF_WRECK_LOOP);
    roof_wreck_loop(m)
}

/// Original: `Prop_GatehouseRoofWreck_04efa2+$1A`.
pub fn roof_wreck_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    anim::step(m, t);
    if owner_signal(m, t) == L::SIGNAL_WRECKED {
        return tasks::finish(m);
    }
    off_left(m, t)
}

/// Puerta de la caseta. Original: `Prop_GatehouseDoor_04efe6`.
pub fn door(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::BARRIER_GFX);
    anim::start(m, t, L::DOOR_SCRIPT);
    set8(m, t + L::FLAGS, L::FLAGS_SOLID);
    m.write_u16(t + L::FIELD_38, L::ROOF_FIELD_38);
    set_handler(m, t, L::DOOR_LOOP);
    post_loop(m)
}

/// Sondeo del suelo dejando los registros como el original (`$027EBA`).
/// Devuelve si está en el aire.
fn probe(m: &mut impl Machine, t: u32) -> bool {
    let mut r = m.regs();
    let f = collision::probe_floor(m, t, (r.d[3] as u16, r.d[4] as u16));
    r.a[1] = f.hit.unwrap_or(f.record);
    r.d[3] = (r.d[3] & 0xFFFF_0000) | u32::from(f.at.0);
    r.d[4] = (r.d[4] & 0xFFFF_0000) | u32::from(f.at.1);
    m.set_regs(r);
    f.in_air
}

/// Fuera de la vista (con sus márgenes): se da de baja.
fn bouncer_cull(m: &mut impl Machine, t: u32) -> Flow {
    if items::out_of_view(m, t, Some(L::BOUNCER_MARGINS)) {
        return tasks::finish(m);
    }
    Flow::Return
}

/// A veces (cuando el azar con `mask` da 0) cambia de animación.
fn bouncer_chance(m: &mut impl Memory, mask: u16) -> bool {
    rng::next(m).0 & mask == 0
}

/// Cascote que rebota: se posa, gráfico, escala completa, quieto y su
/// guion. Original: `Debris_Bouncer_04f5be`.
pub fn bouncer(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    collision::snap_to_ground(m, t);
    anim::alloc_entry(m, t, L::BOUNCER_GFX);
    m.write_u8(t + L::SCALE.0, L::FULL_SCALE);
    m.write_u8(t + L::SCALE.1, L::FULL_SCALE);
    m.write_u8(t + L::FACING, 0);
    manual_icons::stop_motion(m, t);
    m.write_u16(t + L::FIELD_38, 0);
    anim::start(m, t, L::BOUNCER_SCRIPT);
    set_handler(m, t, L::BOUNCER_LOOP);
    bouncer_loop(m)
}

/// Quieto en el suelo; si se queda sin suelo, salta.
/// Original: `Debris_Bouncer_04f5be+$40`.
pub fn bouncer_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    anim::step(m, t);
    if probe(m, t) {
        set_handler(m, t, L::BOUNCER_HOP);
    }
    if bouncer_chance(m, L::BOUNCER_IDLE_CHANCE) {
        anim::start(m, t, L::BOUNCER_TWITCH_SCRIPT);
    }
    bouncer_cull(m, t)
}

/// Salto hacia arriba. Original: `Debris_BouncerHop_04f64c`.
pub fn bouncer_hop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::VEL_Y, L::BOUNCER_HOP_VY);
    anim::start(m, t, L::BOUNCER_HOP_SCRIPT);
    set_handler(m, t, L::BOUNCER_HOP_LOOP);
    bouncer_hop_loop(m)
}

/// Cae hasta tocar suelo y rueda. Original: `Debris_BouncerHop_04f64c+$18`.
pub fn bouncer_hop_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if collision::fall(m, t) {
        set_handler(m, t, L::BOUNCER_ROLL);
    }
    anim::step(m, t);
    bouncer_cull(m, t)
}

/// Rueda por el suelo. Original: `Debris_BouncerRoll_04f692`.
pub fn bouncer_roll(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    manual_icons::stop_motion(m, t);
    m.write_u16(t + L::VEL_X, L::BOUNCER_ROLL_VX);
    anim::start(m, t, L::BOUNCER_SCRIPT);
    if m.read_u8(t + L::FACING) & 1 == 0 {
        anim::start(m, t, L::BOUNCER_TWITCH_SCRIPT);
    }
    set_handler(m, t, L::BOUNCER_ROLL_LOOP);
    bouncer_roll_loop(m)
}

/// Sin suelo, o de vez en cuando, vuelve a saltar.
/// Original: `Debris_BouncerRoll_04f692+$34`.
pub fn bouncer_roll_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let mut r = m.regs();
    r.a[5] = L::SCRATCH_BASE;
    m.set_regs(r);
    player_ground::move_ground(m, t);
    anim::step(m, t);
    if probe(m, t) {
        set_handler(m, t, L::BOUNCER_HOP);
    }
    if bouncer_chance(m, L::BOUNCER_ROLL_CHANCE) {
        set_handler(m, t, L::BOUNCER_HOP);
    }
    bouncer_cull(m, t)
}

#[cfg(test)]
mod tests {
    use crate::layout::fortress as L;

    /// Las cuatro piezas con bit cubren justo `ALL_PIECES`, sin repetir, y
    /// ninguna lista de trozos pisa la marca de fin (caben antes de `+$7C`).
    #[test]
    fn barrier_pieces_cover_all_bits() {
        let mut seen = 0u8;
        for p in &L::BARRIER_PIECES {
            assert_eq!(seen & p.bit, 0);
            seen |= p.bit;
            assert!(!p.blits.is_empty() && p.blits.len() <= 2);
            assert!(p.blits.iter().all(|&b| b != L::NO_BLIT));
        }
        assert_eq!(seen, L::ALL_PIECES);
    }
}
