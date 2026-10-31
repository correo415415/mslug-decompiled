//! Torre de vigía, búnker y puente de la misión 1: decorados pintados en
//! la capa de scroll que se rompen por piezas. Cada pieza tiene vida
//! propia; al morir suelta escombros y una explosión, da puntos al que la
//! ha roto, repinta su trozo del mapa ya en ruinas y avisa a la pieza
//! madre (bits en su `STATE`), que repinta el conjunto según las piezas
//! que falten. Las que tienen lista de sucesos la cortan al romperse.
//!
//! Original: `Prop_Tower_04dad2` … `Prop_BridgePillar_04e12c`, el nido
//! `Prop_Nest_04e248` … `Prop_NestWreck_04e32c` y el cobertizo
//! `Prop_Shed_04e38a` / `Prop_ShedDamaged_04e450`
//! (`asm/enemies/vehicles/turret_car_props_04cbd4.s`) y los pintados
//! `Prop_TowerBlitState10_04fa8a`, `Prop_BunkerBlitState10_04faf8`,
//! `Prop_NestBlitByFacing_04fb3c` y `Prop_NestBlitPair_04fb74`
//! (`asm/enemies/bosses/allen_oneil_04fa58.s`).

use super::{anim, events, fort, layer, platforms, score, sound, spawner, tasks};
use crate::layout::{task as T, tower as L};
use crate::mem::{Flow, Machine, Memory};

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

fn add16(m: &mut impl Memory, a: u32, v: u16) {
    let x = m.read_u16(a).wrapping_add(v);
    m.write_u16(a, x);
}

fn add8(m: &mut impl Memory, a: u32, v: u8) {
    let x = m.read_u8(a).wrapping_add(v);
    m.write_u8(a, x);
}

fn or8(m: &mut impl Memory, a: u32, v: u8) {
    let x = m.read_u8(a) | v;
    m.write_u8(a, x);
}

fn owner(m: &mut impl Memory, t: u32) -> u32 {
    m.read_u32(t + L::OWNER)
}

fn faces_left(m: &mut impl Memory, t: u32) -> bool {
    m.read_u8(t + L::FACING) & 1 != 0
}

/// Hija `handler` colocada donde la tarea, desplazada (`dx`, `dy`).
fn piece(m: &mut impl Memory, t: u32, handler: u32, dx: u16, dy: u16) -> u32 {
    let c = tasks::spawn(m, handler, t);
    fort::place_child(m, t, c);
    add16(m, c + L::X, dx);
    add16(m, c + L::Y, dy);
    c
}

/// Lanzador de trozos `desc` desplazado (`dx`, `dy`).
fn debris(m: &mut impl Memory, t: u32, desc: u32, dx: u16, dy: u16) -> u32 {
    let c = spawner::install(m, t, desc);
    add16(m, c + L::X, dx);
    add16(m, c + L::Y, dy);
    c
}

/// Lista de sucesos de la pieza (se corta al poner `WATCH_FLAG` a `$FF`).
fn watch(m: &mut impl Machine, t: u32, list: u32) {
    m.write_u8(t + u32::from(L::WATCH_FLAG), 0);
    events::spawn_watch(m, list, L::WATCH_FLAG);
}

fn stop_watch(m: &mut impl Memory, t: u32) {
    m.write_u8(t + u32::from(L::WATCH_FLAG), L::GONE);
}

/// Pieza con vida: margen, vida y guion.
fn breakable(m: &mut impl Memory, t: u32, margin: u16, script: u32) {
    m.write_u16(t + L::MARGIN, margin);
    m.write_u16(t + L::HP, L::HP_INIT);
    anim::start(m, t, script);
}

/// Con la escena, el guion y el sonido de los golpes. Devuelve si ha
/// muerto.
fn take_hits(m: &mut impl Machine, t: u32) -> bool {
    platforms::move_with_scene(m, t);
    anim::step(m, t);
    hit_flash(m, t);
    fort::dead(m, t)
}

fn hit_flash(m: &mut impl Memory, t: u32) {
    if fort::hit_side(m, t).is_some() {
        fort::hit_sound(m, t, L::HIT_SOUNDS);
        let f = m.read_u8(t + L::FLAGS_13);
        m.write_u8(t + L::FLAGS_13, f & !L::HIT_FLASH);
    }
}

/// Rota: puntos, sin caja de golpe y su sonido.
fn wrecked(m: &mut impl Memory, t: u32, points: u32, snd: u16) {
    score::award_attacker(m, t, points);
    m.write_u32(t + L::HITBOX, L::NO_HITBOX);
    sound::play(m, snd);
}

/// Pinta el trozo de `table` que corresponde al estado.
fn blit_state(m: &mut impl Memory, t: u32, table: &[(u8, u32)]) {
    let s = m.read_u8(t + L::STATE);
    if let Some(&(_, cmd)) = table.iter().find(|e| e.0 == s) {
        layer::blit(m, cmd);
    }
}

/// Repinta la torre según sus piezas rotas. Original: `Prop_TowerBlitState10_04fa8a`.
pub fn tower_blit(m: &mut impl Memory, t: u32) {
    blit_state(m, t, &L::TOWER_BLITS);
}

/// Repinta el búnker. Original: `Prop_BunkerBlitState10_04faf8`.
pub fn bunker_blit(m: &mut impl Memory, t: u32) {
    blit_state(m, t, &L::BUNKER_BLITS);
}

/// Ruina del pilar según el lado. Original: `Prop_NestBlitByFacing_04fb3c`.
pub fn nest_blit(m: &mut impl Memory, t: u32) {
    let side = usize::from(faces_left(m, t));
    for cmd in L::NEST_BLITS[side] {
        layer::blit(m, cmd);
    }
}

/// Llamada del guion: pinta la pareja fija. Original: `Prop_NestBlitPair_04fb74`.
pub fn nest_blit_pair(m: &mut impl Memory) {
    for cmd in L::NEST_PAIR {
        layer::blit(m, cmd);
    }
}

/// Torre: su sitio, el tejado, la parte de arriba y la base.
/// Original: `Prop_Tower_04dad2`.
pub fn tower(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let rec = m.read_u32(t + L::RECORD);
    fort::place_from_record(m, t, rec);
    let (roof, dx, dy) = L::ROOF;
    piece(m, t, roof, dx, dy);
    piece(m, t, L::TOP, 0, 0);
    let (base, dx, dy) = L::BASE;
    piece(m, t, base, dx, dy);
    m.write_u8(t + L::STATE, 0);
    m.write_u8(t + L::DRAWN, 0);
    set_handler(m, t, L::TOWER_LOOP);
    tower_loop(m)
}

/// Repinta cuando cambia el estado; acaba cuando se han ido las piezas o
/// la torre está entera rota. Original: `Prop_Tower_04dad2+$64`.
pub fn tower_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let s = m.read_u8(t + L::STATE);
    if s == L::GONE {
        return tasks::finish(m);
    }
    if m.read_u8(t + L::DRAWN) != s {
        tower_blit(m, t);
    }
    if m.read_u8(t + L::STATE) == L::TOWER_DONE {
        return tasks::finish(m);
    }
    let s = m.read_u8(t + L::STATE);
    m.write_u8(t + L::DRAWN, s);
    Flow::Return
}

/// Parte de arriba de la torre. Original: `Prop_TowerTop_04db72`.
pub fn top(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    watch(m, t, L::TOP_WATCH);
    breakable(m, t, L::TOP_MARGIN, L::TOP_SCRIPT);
    m.write_u8(t + L::DRAWN, 0);
    set_handler(m, t, L::TOP_LOOP);
    top_loop(m)
}

/// Original: `Prop_TowerTop_04db72+$3A`.
pub fn top_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if take_hits(m, t) {
        add16(m, t + L::X, L::TOP_DX);
        debris(m, t, L::DEBRIS_A, 0, 0);
        debris(m, t, L::DEBRIS_B, 0, 0);
        let (dx, dy) = L::TOP_BLAST;
        debris(m, t, L::BLAST, dx, dy);
        let o = owner(m, t);
        add8(m, o + L::STATE, L::TOP_BIT);
        m.write_u8(t + L::DRAWN, L::GONE);
        set_handler(m, t, L::TOP_WRECK);
    }
    if fort::off_left(m, t) {
        m.write_u8(t + L::DRAWN, L::GONE);
        let o = owner(m, t);
        or8(m, o + L::STATE, L::TOP_GONE);
        set_handler(m, t, L::TOP_WRECK_LOOP);
    }
    Flow::Return
}

/// Rota: puntos, corta sus sucesos y pinta la ruina.
/// Original: `Prop_TowerTopWreck_04dc4a`.
pub fn top_wreck(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    score::award_attacker(m, t, L::POINTS);
    m.write_u32(t + L::HITBOX, L::NO_HITBOX);
    stop_watch(m, t);
    sound::play(m, L::SND_BREAK);
    for cmd in L::TOP_WRECK_BLITS {
        layer::blit(m, cmd);
    }
    set_handler(m, t, L::TOP_WRECK_LOOP);
    wreck_loop(m)
}

/// Con la escena y a esperar a sus hijas. Original:
/// `Prop_TowerTopWreckLoop_04dc88` / `Prop_TowerBaseWreckLoop_04de32`.
pub fn wreck_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    set_handler(m, t, L::WAIT_CHILDREN);
    Flow::Return
}

/// Bandera: sigue la escena hasta que se rompe la parte de arriba.
/// Original: `Prop_TowerFlag_04dc96`.
pub fn flag(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::FLAG_GFX);
    for a in L::SCALE {
        m.write_u8(t + a, 0xFF);
    }
    m.write_u8(t + L::FACING, 0);
    anim::start(m, t, L::FLAG_SCRIPT);
    set_handler(m, t, L::FLAG_LOOP);
    flag_loop(m)
}

/// Original: `Prop_TowerFlag_04dc96+$2E`.
pub fn flag_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    anim::step(m, t);
    let o = owner(m, t);
    if m.read_u8(o + L::DRAWN) == L::GONE {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Base de la torre, con su soldado. Original: `Prop_TowerBase_04dce6`.
pub fn base(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let (soldier, dx, dy) = L::BASE_SOLDIER;
    piece(m, t, soldier, dx, dy);
    breakable(m, t, L::TOP_MARGIN, L::BASE_SCRIPT);
    m.write_u8(t + L::STATE, 0);
    m.write_u8(t + L::DRAWN, 0);
    set_handler(m, t, L::BASE_LOOP);
    base_loop(m)
}

/// A media vida se agrieta; al morir se rompe.
/// Original: `Prop_TowerBase_04dce6+$48`.
pub fn base_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    anim::step(m, t);
    if (m.read_u8(t + L::DRAWN) as i8) < 1 && m.read_u16(t + L::HP) as i16 <= L::HP_DAMAGED {
        sound::play(m, L::SND_CRUMBLE);
        debris(m, t, L::BASE_DEBRIS, 0, 0);
        layer::blit(m, L::BASE_CRACK_BLIT);
        m.write_u8(t + L::STATE, L::GONE);
        let o = owner(m, t);
        add8(m, o + L::STATE, 1);
        m.write_u8(t + L::DRAWN, 1);
    }
    hit_flash(m, t);
    if fort::dead(m, t) {
        debris(m, t, L::BASE_DEBRIS, 0, 0);
        debris(m, t, L::BASE_DEBRIS_B, 0, 0);
        let o = owner(m, t);
        add8(m, o + L::STATE, 1);
        set_handler(m, t, L::BASE_WRECK);
    }
    if fort::off_left(m, t) {
        let o = owner(m, t);
        or8(m, o + L::STATE, L::BASE_GONE);
        set_handler(m, t, L::BASE_WRECK_LOOP);
    }
    Flow::Return
}

/// Original: `Prop_TowerBaseWreck_04ddec`.
pub fn base_wreck(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    wrecked(m, t, L::POINTS, L::SND_CRUMBLE);
    layer::blit(m, L::BASE_WRECK_BLIT);
    let (dx, dy) = L::BASE_BLAST;
    debris(m, t, L::BLAST, dx, dy);
    set_handler(m, t, L::BASE_WRECK_LOOP);
    wreck_loop(m)
}

/// Búnker. Original: `Prop_Bunker_04de40`.
pub fn bunker(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let rec = m.read_u32(t + L::RECORD);
    fort::place_from_record(m, t, rec);
    watch(m, t, L::BUNKER_WATCH);
    breakable(m, t, L::BUNKER_MARGIN, L::BUNKER_SCRIPT);
    m.write_u8(t + L::DRAWN, 0);
    m.write_u8(t + L::STATE, 0);
    set_handler(m, t, L::BUNKER_LOOP);
    bunker_loop(m)
}

/// Repinta si el estado no es el pintado.
fn bunker_redraw(m: &mut impl Memory, t: u32) {
    if m.read_u8(t + L::WANTED) != m.read_u8(t + L::STATE) {
        bunker_blit(m, t);
    }
}

/// Original: `Prop_Bunker_04de40+$4A`.
pub fn bunker_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if take_hits(m, t) {
        stop_watch(m, t);
        add8(m, t + L::STATE, L::TOP_BIT);
        m.write_u8(t + L::DRAWN, L::GONE);
        add16(m, t + L::X, L::TOP_DX);
        debris(m, t, L::DEBRIS_A, 0, 0);
        debris(m, t, L::DEBRIS_B, 0, 0);
        let (dx, dy) = L::BUNKER_BLAST;
        debris(m, t, L::BLAST, dx, dy);
        set_handler(m, t, L::BUNKER_WRECK);
    }
    if fort::off_left(m, t) {
        m.write_u8(t + L::DRAWN, L::GONE);
        set_handler(m, t, L::BUNKER_WRECK_LOOP);
    }
    bunker_redraw(m, t);
    Flow::Return
}

/// Ruina del búnker (según en qué mitad de la pantalla está).
/// Original: `Prop_BunkerWreck_04df30`.
pub fn bunker_wreck(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    wrecked(m, t, L::POINTS, L::SND_BREAK);
    let (right, left) = L::BUNKER_WRECK_BLITS;
    let cmd = if (m.read_u16(t + L::X) as i16) < 0 { left } else { right };
    layer::blit(m, cmd);
    set_handler(m, t, L::BUNKER_WRECK_LOOP);
    bunker_wreck_loop(m)
}

/// Original: `Prop_BunkerWreck_04df30__L04df72`.
pub fn bunker_wreck_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if fort::off_left(m, t) {
        set_handler(m, t, L::WAIT_CHILDREN);
    }
    bunker_redraw(m, t);
    Flow::Return
}

/// Puente con sus dos pilares. Original: `Prop_Bridge_04df98`.
pub fn bridge(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let rec = m.read_u32(t + L::RECORD);
    fort::place_from_record(m, t, rec);
    piece(m, t, L::PILLAR, 0, 0);
    let c = piece(m, t, L::PILLAR, L::PILLAR_DX, 0);
    or8(m, c + L::FACING, 1);
    m.write_u16(t + L::MARGIN, L::BRIDGE_MARGIN);
    m.write_u8(t + L::STATE, 0);
    watch(m, t, L::BRIDGE_WATCH);
    set_handler(m, t, L::BRIDGE_LOOP);
    bridge_loop(m)
}

/// Rotos los dos pilares se viene abajo. Original: `Prop_Bridge_04df98+$5E`.
pub fn bridge_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if m.read_u8(t + L::STATE) == L::BRIDGE_FALL {
        sound::play(m, L::SND_BREAK);
        for d in L::BRIDGE_DEBRIS {
            debris(m, t, d, 0, 0);
        }
        for cmd in L::BRIDGE_FALL_BLITS {
            layer::blit(m, cmd);
        }
        set_handler(m, t, L::COLLAPSE);
    }
    if fort::off_left(m, t) {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Tablero caído: aún se puede romper del todo.
/// Original: `Prop_BridgeCollapse_04e05e`.
pub fn collapse(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::HP, L::HP_INIT);
    anim::start(m, t, L::COLLAPSE_SCRIPT);
    set_handler(m, t, L::COLLAPSE_LOOP);
    collapse_loop(m)
}

/// Original: `Prop_BridgeCollapse_04e05e+$18`.
pub fn collapse_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if take_hits(m, t) {
        m.write_u32(t + L::HITBOX, L::NO_HITBOX);
        score::award_attacker(m, t, L::BRIDGE_POINTS);
        sound::play(m, L::SND_CRUMBLE);
        for d in L::COLLAPSE_DEBRIS {
            debris(m, t, d, 0, 0);
        }
        for cmd in L::COLLAPSE_BLITS {
            layer::blit(m, cmd);
        }
    } else if !fort::off_left(m, t) {
        return Flow::Return;
    }
    stop_watch(m, t);
    tasks::finish(m)
}

/// Pilar del puente, con su soldado. Original: `Prop_BridgePillar_04e12c`.
pub fn pillar(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let (dx, dy) = L::PILLAR_SOLDIER_AT[usize::from(faces_left(m, t))];
    piece(m, t, L::PILLAR_SOLDIER, dx, dy);
    m.write_u8(t + L::STATE, 0);
    breakable(m, t, L::PILLAR_MARGIN, L::PILLAR_SCRIPT);
    set_handler(m, t, L::PILLAR_LOOP);
    pillar_loop(m)
}

/// Al romperse avisa al puente con su bit. Original: `Prop_BridgePillar_04e12c+$5C`.
pub fn pillar_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if take_hits(m, t) {
        sound::play(m, L::SND_PILLAR);
        m.write_u32(t + L::HITBOX, L::NO_HITBOX);
        for d in L::PILLAR_DEBRIS {
            debris(m, t, d, 0, L::PILLAR_DEBRIS_DY);
        }
        let left = faces_left(m, t);
        let dx = if left { L::PILLAR_BLAST_DX.wrapping_neg() } else { L::PILLAR_BLAST_DX };
        debris(m, t, L::BLAST, dx, 0);
        nest_blit(m, t);
        let o = owner(m, t);
        or8(m, o + L::STATE, L::PILLAR_BITS[usize::from(left)]);
        m.write_u8(t + L::STATE, L::GONE);
        return tasks::finish(m);
    }
    if fort::off_left(m, t) {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Nido del enjambre: su sitio, gráfico, vida, guion y el generador de
/// bichos encima. Original: `Prop_Nest_04e248`.
pub fn nest_prop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let rec = m.read_u32(t + L::RECORD);
    fort::place_from_record(m, t, rec);
    anim::alloc_entry(m, t, L::NEST_GFX);
    m.write_u16(t + L::MARGIN, L::NEST_MARGIN);
    m.write_u16(t + L::HP, L::NEST_HP);
    anim::start(m, t, L::NEST_SCRIPT);
    let (h, dx, dy) = L::NEST_SWARM;
    piece(m, t, h, dx, dy);
    set_handler(m, t, L::NEST_WAIT);
    nest_prop_wait(m)
}

/// Quieto con la escena hasta que entra en la pantalla.
/// Original: `Prop_Nest_04e248+$50`.
pub fn nest_prop_wait(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if (m.read_u16(t + L::X) as i16) < L::NEST_WAKE_X {
        return nest_prop_active(m);
    }
    platforms::move_with_scene(m, t);
    anim::step(m, t);
    Flow::Return
}

/// Ya se le puede dar. Original: `Prop_NestActive_04e2b0`.
pub fn nest_prop_active(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    set_handler(m, t, L::NEST_LOOP);
    nest_prop_loop(m)
}

/// Encaja golpes; al romperse suelta sus trozos y pasa a ruina.
/// Original: `Prop_NestActive_04e2b0+$6`.
pub fn nest_prop_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if take_hits(m, t) {
        sound::play(m, L::SND_PILLAR);
        for d in L::NEST_DEBRIS {
            spawner::install(m, t, d);
        }
        set_handler(m, t, L::NEST_WRECK);
    }
    if fort::off_left(m, t) {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Ruina del nido: puntos, sin caja de golpe, su trozo pintado, sonido y
/// guion. Original: `Prop_NestWreck_04e32c`.
pub fn nest_prop_wreck(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    score::award_attacker(m, t, L::NEST_POINTS);
    m.write_u32(t + L::HITBOX, L::NO_HITBOX);
    platforms::move_with_scene(m, t);
    super::scroll::blit_cells(m, t, L::NEST_WRECK_CELLS);
    sound::play(m, L::SND_BREAK);
    anim::start(m, t, L::NEST_WRECK_SCRIPT);
    set_handler(m, t, L::NEST_WRECK_LOOP);
    nest_prop_wreck_loop(m)
}

/// Original: `Prop_NestWreck_04e32c+$42`.
pub fn nest_prop_wreck_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    anim::step(m, t);
    if fort::off_left(m, t) {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Los dos lanzadores del cobertizo al romperse.
fn shed_debris(m: &mut impl Memory, t: u32) {
    let (dx, dy) = L::SHED_DEBRIS_AT;
    for d in L::SHED_DEBRIS {
        debris(m, t, d, dx, dy);
    }
}

/// Cobertizo con su tejado. Original: `Prop_Shed_04e38a`.
pub fn shed(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let rec = m.read_u32(t + L::RECORD);
    fort::place_from_record(m, t, rec);
    let (h, dx, dy) = L::SHED_ROOF;
    piece(m, t, h, dx, dy);
    m.write_u16(t + L::MARGIN, L::SHED_MARGIN);
    m.write_u16(t + L::HP, L::SHED_HP);
    anim::start(m, t, L::SHED_SCRIPT);
    set_handler(m, t, L::SHED_LOOP);
    shed_loop(m)
}

/// Primera tanda de golpes: al acabarla revive y pasa a dañado.
/// Original: `Prop_Shed_04e38a+$48`.
pub fn shed_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if take_hits(m, t) {
        let f = m.read_u8(t + L::FLAGS_13);
        m.write_u8(t + L::FLAGS_13, f & !L::DEAD);
        shed_debris(m, t);
        set_handler(m, t, L::SHED_DAMAGED);
    }
    if fort::off_left(m, t) {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Cobertizo dañado: se derrumba un trozo y vuelve a tener vida.
/// Original: `Prop_ShedDamaged_04e450`.
pub fn shed_damaged(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    sound::play(m, L::SND_CRUMBLE);
    layer::blit(m, L::SHED_DAMAGED_BLIT);
    m.write_u16(t + L::HP, L::SHED_HP);
    set_handler(m, t, L::SHED_DAMAGED_LOOP);
    shed_damaged_loop(m)
}

/// Segunda tanda: al romperse da puntos, suelta trozos, pinta la ruina y
/// se da de baja. Original: `Prop_ShedDamaged_04e450+$24`.
///
/// La baja la hace como la ROM: *llama* a `Prop_FreeOrRts_04e50a` (que
/// salta a `$518` y ya no vuelve), así que deja una dirección de vuelta
/// más en la pila antes de pasar al planificador.
pub fn shed_damaged_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if take_hits(m, t) {
        score::award_attacker(m, t, L::POINTS);
        sound::play(m, L::SND_BREAK);
        m.write_u32(t + L::HITBOX, L::NO_HITBOX);
        shed_debris(m, t);
        layer::blit(m, L::SHED_WRECK_BLIT);
        return Flow::Call { routine: L::FREE_OR_RTS, then: L::SHED_DAMAGED_AFTER_FREE };
    }
    if fort::off_left(m, t) {
        return tasks::finish(m);
    }
    Flow::Return
}
