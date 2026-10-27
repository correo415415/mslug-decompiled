//! Soldado rebelde (infantería): alta según la variante, rasgos que se
//! publican en sus flags, dificultad, percepción del jugador más cercano
//! y el reparto inicial (andar o caer).
//!
//! Original: `asm/enemies/soldiers/helpers_056acc.s`
//! (`Soldier_InitCommon_0570a8`, `Soldier_Think_056b92`,
//! `Soldier_FindNearestPlayer_056b38`, `Soldier_ApproxDist_056b12`,
//! `Soldier_SpawnVariants_057226`, `Soldier_SpawnDispatch_05752c`),
//! `asm/entity/state_publishers_057044.s` (`$05717A` … `$05721E`),
//! `$027F60`, `$0799A4`/`$0799DE` y `PosRing_FindNear_08f344`.

use super::{anim, collision, fort, hits, items, motion, platforms, player_ground, rng, slug, slug_arm, tasks, turret, video};
use crate::layout::{bios_ram, soldier as L, task as T};
use crate::mem::{Flow, Machine, Memory};

fn set_bits(m: &mut impl Memory, a: u32, bits: u8, on: bool) {
    let v = m.read_u8(a);
    m.write_u8(a, if on { v | bits } else { v & !bits });
}

/// Rasgo de la variante que se publica en los flags del soldado.
/// Original: los publicadores `$05717A` … `$05721E`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trait {
    /// Lado al azar (`$05717A`, sin tocar el subestado).
    RandomSide,
    /// Lado al azar y subestado 0 (`$05719C`).
    Random,
    /// Subestado fijo 1, 2 o 3 (`$0571D8`, `$0571C4`, `$0571EC`).
    Sub(u8),
    /// Puede huir (`$057200`).
    Flees,
    /// Anclas de agarre permitidas: 0, 1 o 2 (`$05720E`, `$05721E`,
    /// `$057216`).
    Anchor(u8),
}

/// Original: `$05717A` … `$05721E`.
pub fn publish(m: &mut impl Memory, t: u32, tr: Trait) {
    match tr {
        Trait::RandomSide | Trait::Random => {
            let (r, _) = rng::next(m);
            set_bits(m, t + L::FLAGS_72, L::ACTIVE, (r as i16) >= 0);
            set_bits(m, t + L::FLAGS_74, L::CAN_SURRENDER, true);
            if tr == Trait::Random {
                m.write_u8(t + L::SUBSTATE, 0);
            }
        }
        Trait::Sub(s) => {
            set_bits(m, t + L::FLAGS_72, L::ACTIVE, true);
            set_bits(m, t + L::FLAGS_74, L::CAN_SURRENDER, true);
            m.write_u8(t + L::SUBSTATE, s);
        }
        Trait::Flees => {
            set_bits(m, t + L::FLAGS_72, L::ACTIVE, true);
            set_bits(m, t + L::FLAGS_74, L::FLEES, true);
        }
        Trait::Anchor(k) => set_bits(m, t + L::FLAGS_74, 1 << k, true),
    }
}

/// Nivel de dificultad 0–7: el del ajuste (4 en la demo), uno más si
/// está activo el pilotaje del Slug. Original: `$0799A4` (que además
/// copia el byte `+$68` de `a0` a la tarea si ese ajuste está activo).
pub fn level(m: &mut impl Memory, t: u32, a0: u32) -> u8 {
    let mut d = m.read_u8(bios_ram::SOFT_DIP_DIFFICULTY);
    if m.read_u8(bios_ram::USER_MODE) == bios_ram::USER_MODE_ATTRACT {
        d = 4;
    }
    if slug_arm::take_pilot(m, t, a0) {
        d = d.wrapping_add(1);
    }
    d.min(7)
}

/// Valor de una tabla de la ROM según la dificultad, la variante de la
/// tarea (`+$11` & 3) y los jugadores en juego. Las tablas de tipo 2
/// eligen además una de 8 entradas al azar. Original: `$0799DE`.
pub fn by_difficulty(m: &mut impl Memory, t: u32, table: u32) -> u16 {
    let a0 = table + 2;
    let v = u16::from(m.read_u8(t + L::VARIANT_11) & 3);
    let players = u16::from(m.read_u8(L::ACTIVE_PLAYERS));
    if m.read_u16(table) != L::TABLE_RANDOM {
        let k = v * 4 + (players & 2) + (u16::from(level(m, t, a0) & 7) << 4);
        return m.read_u16(a0 + u32::from(k));
    }
    let k = v * 2 + (players.wrapping_sub(1) & 1) + (u16::from(level(m, t, a0) & 7) << 3);
    let row = u32::from(m.read_u8(a0 + u32::from(k)));
    let base = a0 + L::RANDOM_ROWS + row * 16;
    let (r, _) = rng::next(m);
    m.read_u16(base + u32::from(r & 0xE))
}

/// ¿Está en el suelo? Prueba a bajar un píxel con la colisión y deja la
/// posición y la velocidad como estaban. Original: `$027F60` (deja el
/// resultado en `d3`; los `move.w` finales que restauran los campos borran
/// siempre el flag C, así que quien mira C ve «en el suelo»).
pub fn on_ground(m: &mut impl Machine, t: u32) -> bool {
    let fields = [L::VEL_X, L::VEL_Y, L::X, L::Y];
    let saved = fields.map(|f| m.read_u16(t + f));
    let y = m.read_u16(t + L::Y).wrapping_add(1);
    m.write_u16(t + L::Y, y);
    m.write_u16(t + L::VEL_X, 0);
    m.write_u16(t + L::VEL_Y, L::PROBE_VEL_Y);
    let row = m.regs().d[4] as u16;
    let hit = motion::on_entity_pos(m, t, |m| slug::move_and_stop(m, t, row));
    for (f, v) in fields.into_iter().zip(saved) {
        m.write_u16(t + f, v);
    }
    hit
}

/// Alta común: suelo, temporizadores y agresividad según la dificultad,
/// sprite, capa, la tarea hija y la primera percepción.
/// Original: `Soldier_InitCommon_0570a8`.
pub fn init_common(m: &mut impl Machine, t: u32) {
    // El original hace `scc` tras `$027F60`, que siempre sale con C = 0:
    // la marca queda siempre puesta (la prueba sólo deja sus efectos).
    on_ground(m, t);
    m.write_u8(t + L::GROUNDED, 0xFF);
    set_bits(m, t + L::FLAGS_6B, L::FLAGS_6B_SET, true);
    let d = u32::from(by_difficulty(m, t, L::SURRENDER_TABLE));
    // Con `+$99` = 0 el original multiplica por 0 (su `-1` va al otro
    // registro): el temporizador queda a 0.
    let k = u32::from(m.read_u8(t + L::P_SURRENDER));
    m.write_u16(t + L::TIMER_SURRENDER, ((d * k) >> 8) as u16);
    let d = u32::from(by_difficulty(m, t, L::AGGRO_TABLE));
    let k = match m.read_u8(t + L::P_AGGRO) {
        0 => L::DEFAULT_AGGRO,
        k => k,
    };
    let a = (d * u32::from(k)) >> 8;
    m.write_u8(t + L::P_AGGRO, if a & 0xFFFF >= 0x100 { 0xFF } else { a as u8 });
    let leave = match m.read_u8(t + L::P_LEAVE) {
        0 => L::DEFAULT_LEAVE,
        k => u16::from(k),
    };
    m.write_u16(t + L::TIMER_LEAVE, leave << 6);
    video::release_task_entries(m, t);
    anim::alloc_entry(m, t, L::GFX);
    m.write_u16(t + L::ACC_X, 0);
    m.write_u16(t + L::ACC_Y, 0);
    anim::set_attr(m, t, L::ATTR);
    let a = m.read_u16(t + L::ATTR_38);
    m.write_u16(t + L::ATTR_38, (a & L::LAYER_KEEP) | L::LAYER);
    tasks::spawn(m, L::CHILD, t);
    m.write_u16(t + L::PREF_A, 0);
    m.write_u16(t + L::PREF_B, 0);
    think(m, t);
    m.write_u8(t + L::FLAGS_74, 0);
    let (r, _) = rng::next(m);
    m.write_u16(t + L::SEED, r);
    let (r, _) = rng::next(m);
    m.write_u16(t + L::RADIUS, r);
}

/// Distancia aproximada (mayor + menor / 2, con el valor absoluto «a la
/// baja» del original). Original: `Soldier_ApproxDist_056b12`.
pub fn approx_dist(dx: u16, dy: u16) -> u16 {
    let abs = |v: u16| if (v as i16) < 0 { !v } else { v };
    let (a, b) = (abs(dx), abs(dy));
    if b > a {
        (a >> 1).wrapping_add(b)
    } else {
        a.wrapping_add(b >> 1)
    }
}

/// El jugador más cercano y su distancia (`0xFFFF` y `None` si no hay).
/// Tal cual el original, la distancia al 1P sólo mira la X.
/// Original: `Soldier_FindNearestPlayer_056b38`.
pub fn nearest_player(m: &mut impl Memory, t: u32) -> (u16, Option<u32>) {
    let mut best = (0xFFFF, None);
    if let Some(p) = tasks::player_task_if_alive(m, 0) {
        let dx = m.read_u16(t + L::X).wrapping_sub(m.read_u16(p + L::X));
        best = (approx_dist(dx, 0), Some(p));
    }
    if let Some(p) = tasks::player_task_if_alive(m, 1) {
        let dx = m.read_u16(t + L::X).wrapping_sub(m.read_u16(p + L::X));
        let dy = m.read_u16(t + L::Y).wrapping_sub(m.read_u16(p + L::Y));
        let d = approx_dist(dx, dy);
        if d <= best.0 {
            best = (d, Some(p));
        }
    }
    best
}

/// ¿Hay alguna posición reciente del anillo cerca del soldado? Devuelve
/// si queda «a su espalda» (bit 7). Original: `PosRing_FindNear_08f344`.
pub fn ring_near(m: &mut impl Memory, t: u32) -> Option<u8> {
    let mut k = m.read_u16(L::RING);
    let end = m.read_u16(L::RING + 4);
    while k != end {
        let e = L::RING + 8 + u32::from(k);
        let dx = m.read_u16(e).wrapping_sub(m.read_u16(t + L::X));
        let behind: u8 = if (dx as i16) < 0 { 0xFF } else { 0 };
        let dy = m.read_u16(e + 2).wrapping_sub(m.read_u16(t + L::Y));
        if dx.wrapping_add(0x40) < 0x80 && dy.wrapping_add(0x30) < 0x60 {
            let facing: u8 = if m.read_u8(t + L::FACING) & 1 == 0 { 0xFF } else { 0 };
            return Some(facing ^ behind);
        }
        k += 4;
        if k >= L::RING_SIZE {
            k = 0;
        }
    }
    None
}

/// Margen de distancia que da el radio (`+$90`: dos campos de 5 bits).
fn reach(m: &mut impl Memory, t: u32) -> u16 {
    let r = m.read_u16(t + L::RADIUS);
    (r & 0x1F) + ((r >> 8) & 0x1F) + 0x10
}

/// Percepción de cada frame: temporizadores, bordes de plataforma, el
/// jugador objetivo y de qué lado está, preferencias de movimiento y los
/// umbrales de acercarse / alejarse. Original: `Soldier_Think_056b92`.
pub fn think(m: &mut impl Memory, t: u32) {
    for f in [L::TIMER_SURRENDER, L::TIMER_LEAVE] {
        let v = m.read_u16(t + f);
        if v != 0 {
            m.write_u16(t + f, v - 1);
        }
    }
    set_bits(m, t + L::FLAGS_72, L::EDGE_SOLID | L::EDGE_GAP, false);
    if m.read_u8(t + L::FLAGS_5A) & L::PROBE_EDGE != 0 {
        let x = m.read_u16(t + L::X);
        let x = if m.read_u8(t + L::FACING) & 1 != 0 { x.wrapping_add(3) } else { x.wrapping_sub(3) };
        let y = m.read_u16(t + L::Y).wrapping_add(0x18);
        let solid = turret::kind_at(m, t, x, y) != 0;
        set_bits(m, t + L::FLAGS_72, if solid { L::EDGE_SOLID } else { L::EDGE_GAP }, true);
    }
    let (dist, target) = nearest_player(m, t);
    m.write_u16(t + L::DISTANCE, dist);
    let Some(p) = target else {
        set_bits(m, t + L::FLAGS_73, L::NO_TARGET, true);
        m.write_u32(t + L::TARGET, 0);
        set_bits(m, t + L::FLAGS_72, L::SIDE, true);
        for f in [L::NEAR, L::FAR, L::PREF_B, L::PREF_A, L::PREF_C, L::PREF_D] {
            m.write_u16(t + f, L::NO_TARGET_PREF);
        }
        return;
    };
    set_bits(m, t + L::FLAGS_73, L::NO_TARGET, false);
    m.write_u32(t + L::TARGET, p);
    let dx = m.read_u16(t + L::X).wrapping_sub(m.read_u16(p + L::X));
    let facing = m.read_u8(t + L::FACING) & 1 != 0;
    let behind = (dx as i16) < 0;
    set_bits(m, t + L::FLAGS_72, L::BEHIND, behind);
    set_bits(m, t + L::FLAGS_72, L::SIDE, facing == behind);
    for f in [L::PREF_C, L::PREF_A] {
        m.write_u32(t + f, 0);
    }
    match ring_near(m, t) {
        Some(d) => {
            if d & 0x80 != 0 {
                m.write_u16(t + L::PREF_A, L::PREF_FULL);
                set_bits(m, t + L::FLAGS_73, L::PREF_FWD, true);
                set_bits(m, t + L::FLAGS_73, L::PREF_BACK, false);
            } else {
                let (r, _) = rng::next(m);
                if (r as u8 as i8) >= 0 {
                    m.write_u16(t + L::PREF_B, L::PREF_FULL);
                    set_bits(m, t + L::FLAGS_73, L::PREF_FWD, false);
                    set_bits(m, t + L::FLAGS_73, L::PREF_BACK, true);
                } else {
                    m.write_u16(t + L::PREF_A, r & 0x3FF);
                    let (r, _) = rng::next(m);
                    m.write_u16(t + L::PREF_B, r & 0x3FF);
                    m.write_u16(t + L::PREF_C, L::PREF_HALF);
                    m.write_u16(t + L::PREF_D, L::PREF_HALF);
                }
            }
            set_bits(m, t + L::FLAGS_73, L::PREF_SET, true);
        }
        None => {
            let f = m.read_u8(t + L::FLAGS_73);
            m.write_u8(t + L::FLAGS_73, f & !(L::PREF_SET | L::PREF_FWD | L::PREF_BACK));
        }
    }
    match m.read_u8(t + L::P_KIND) {
        0 => set_bits(m, t + L::FLAGS2, L::FLAGS2_UP | L::FLAGS2_DOWN, false),
        1 => set_bits(m, t + L::FLAGS2, L::FLAGS2_UP | L::FLAGS2_DOWN, true),
        2 => {
            let up = (m.read_u8(t + L::SEED) as i8) >= 0;
            set_bits(m, t + L::FLAGS2, L::FLAGS2_UP, up);
            set_bits(m, t + L::FLAGS2, L::FLAGS2_DOWN, !up);
        }
        _ => {
            let both = rng::next(m).0 & 1 != 0;
            set_bits(m, t + L::FLAGS2, L::FLAGS2_UP | L::FLAGS2_DOWN, both);
        }
    }
    thresholds(m, t);
}

/// Umbrales de acercarse (`NEAR`) y alejarse (`FAR`) según la distancia
/// menos el radio; fuera de la zona activa, siempre alejarse.
fn thresholds(m: &mut impl Memory, t: u32) {
    let dist = m.read_u16(t + L::DISTANCE);
    let r = reach(m, t);
    let v = dist.wrapping_sub(r);
    let v = if (v as i16) < 0 { 0 } else { v };
    let v = (v << 2).min(0x100);
    m.write_u16(t + L::NEAR, 0x100u16.wrapping_sub(v));
    let v = dist.wrapping_sub(r);
    let mut v = if (v as i16) < 0 { 0u32 } else { u32::from(v) };
    let k = m.read_u8(t + L::P_SHIFT) & 7;
    v <<= k;
    v <<= 1;
    m.write_u16(t + L::FAR, if v >= 0x100 { 0x100 } else { v as u16 });
    let x = m.read_u16(t + L::X);
    if (x.wrapping_sub(L::ACTIVE_X.0) as i16) < 0 || (x.wrapping_sub(L::ACTIVE_X.1) as i16) >= 0 {
        m.write_u16(t + L::FAR, 0x100);
        m.write_u16(t + L::NEAR, 0);
        m.write_u16(t + L::PREF_A, 0);
        m.write_u16(t + L::PREF_B, 0);
    }
}

/// Al acabar el alta: mira hacia el centro y anda (en el suelo) o cae.
/// Original: `Soldier_SpawnDispatch_05752c`.
pub fn dispatch(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if (m.read_u16(t + L::X).wrapping_sub(L::CENTER_X) as i16) < 0 {
        set_bits(m, t + L::FACING, 1, true);
    }
    let next = if m.read_u8(t + L::GROUNDED) != 0 { L::WALK_START } else { L::FALL_START };
    m.write_u32(t + T::HANDLER, next);
    Flow::Return
}

/// Alta de una variante: el alta común, sus rasgos y el reparto.
/// Original: los stubs de `Soldier_SpawnVariants_057226` (`$057276` …
/// `$0573A2`).
pub fn spawn(m: &mut impl Machine, traits: &[Trait]) -> Flow {
    let t = m.task_register();
    init_common(m, t);
    for &tr in traits {
        publish(m, t, tr);
    }
    dispatch(m)
}

/// Variantes con parámetros fijos (tipo 2, rendición 10, agresividad
/// `$1E`, retirada 10). Original: `Soldier_SpawnVariants_057226` y
/// `$05724E`.
pub fn spawn_preset(m: &mut impl Machine, traits: &[Trait]) -> Flow {
    let t = m.task_register();
    video::release_task_entries(m, t);
    set_bits(m, t + T::FLAGS, L::FLAGS_PRESET, true);
    for (f, v) in L::PRESET {
        m.write_u8(t + f, v);
    }
    spawn(m, traits)
}

/// Variantes que nacen cayendo de cara a un lado (`left`), sin el reparto.
/// Original: `$05736A` / `$057374`.
pub fn spawn_facing(m: &mut impl Machine, left: bool) -> Flow {
    let t = m.task_register();
    set_bits(m, t + L::FACING, 1, left);
    init_common(m, t);
    publish(m, t, Trait::Random);
    Flow::Goto(L::FACE_FALL)
}

// --- Andar (`Soldier_WalkStart_057558` y su bucle) ---------------------

/// ¿Pasa la tirada? Los dos bytes de un número aleatorio por debajo de `n`.
fn roll(m: &mut impl Memory, n: u16) -> bool {
    let (r, _) = rng::next(m);
    (r & 0xFF) < n && (r >> 8) < n
}

/// Registros que deja el sondeo del suelo (los restos que miran después
/// el movimiento sobre el suelo y la caída).
fn probe_regs(m: &mut impl Machine, f: &collision::Floor, ground: bool) {
    let mut r = m.regs();
    let col = if f.platform.is_some() { 0 } else { f.at.0 & 0xFF00 };
    r.d[3] = (r.d[3] & 0xFFFF_0000) | u32::from(col) | if ground { 0xFF } else { 0 };
    r.d[4] = (r.d[4] & 0xFFFF_0000) | u32::from(f.at.1);
    m.set_regs(r);
}

/// Sigue a la escena y mira si pisa suelo (C = en el aire; deja en `d3`
/// `$FF` si pisa). Original: `$027F08`.
pub fn probe(m: &mut impl Machine, t: u32) -> collision::Floor {
    platforms::move_with_scene(m, t);
    let r = m.regs();
    let f = collision::probe_floor(m, t, (r.d[3] as u16, r.d[4] as u16));
    probe_regs(m, &f, !f.in_air);
    f
}

/// Ejecuta `step` con la gravedad de reserva (`+$62`) y la guarda después.
/// Original: el envoltorio de `$028292` / `$028364`.
fn with_alt_gravity<M: Machine>(m: &mut M, t: u32, step: impl FnOnce(&mut M) -> bool) -> bool {
    if m.read_u16(t + L::GRAVITY_SAVE) != 0xFFFF {
        m.write_u32(t + L::GRAVITY_SAVE, 0xFFFF_FFFF);
    }
    let g = m.read_u16(t + L::GRAVITY);
    let alt = m.read_u16(t + L::GRAVITY_ALT);
    m.write_u16(t + L::GRAVITY, alt);
    let c = step(m);
    let alt = m.read_u16(t + L::GRAVITY);
    m.write_u16(t + L::GRAVITY_ALT, alt);
    m.write_u16(t + L::GRAVITY, g);
    c
}

/// Paso sobre el suelo con la gravedad de reserva. Original: `$028292`.
pub fn move_ground(m: &mut impl Machine, t: u32) -> bool {
    with_alt_gravity(m, t, |m| player_ground::move_ground(m, t))
}

/// Paso de caída con la gravedad de reserva. Original: `$028364`.
pub fn fall(m: &mut impl Machine, t: u32) -> bool {
    with_alt_gravity(m, t, |m| collision::fall(m, t))
}

/// Física de un frame: si pisa suelo anda sobre él; si no, cae (y queda
/// apoyado si toca suelo). Mantiene `GROUNDED`.
/// Original: `Soldier_PhysicsStep_056acc`.
pub fn physics(m: &mut impl Machine, t: u32) {
    let f = probe(m, t);
    // En el suelo el original compara el tipo de tile con la marca, pero
    // en los dos casos la deja a `$FF`.
    m.write_u8(t + L::GROUNDED, if f.in_air { 0 } else { 0xFF });
    if !f.in_air {
        move_ground(m, t);
    } else {
        let landed = fall(m, t);
        m.write_u8(t + L::GROUNDED, if landed { 0xFF } else { 0 });
    }
}

/// Velocidad X hacia donde mira. Original: `Soldier_SetVelXByFacing_056f8a`.
pub fn set_speed(m: &mut impl Memory, t: u32) {
    let v = m.read_u16(t + L::SPEED);
    let v = if m.read_u8(t + L::FACING) & 1 != 0 { v } else { v.wrapping_neg() };
    m.write_u16(t + L::VEL_X, v);
}

/// ¿Se le ha acabado el tiempo para marcharse?
/// Original: `Soldier_LeaveTimerExpired_056e36`.
pub fn leave_expired(m: &mut impl Memory, t: u32) -> bool {
    m.read_u16(t + L::TIMER_LEAVE) == 0
}

/// Posición de la entidad en coordenadas del mundo (Y hacia arriba).
fn world_pos(m: &mut impl Memory, t: u32) -> (u16, u16) {
    let x = m.read_u16(t + L::X).wrapping_add(m.read_u16(L::CAMERA_X));
    let y = L::WORLD_Y.wrapping_sub(m.read_u16(t + L::Y)).wrapping_add(m.read_u16(L::CAMERA_Y));
    (x, y)
}

/// El borde de plataforma de la lista `list` más cercano a (`x`, `y`)
/// (en coordenadas del mundo): su entrada y la X de su arista.
/// Original: `$07700C`.
pub fn nearest_edge(m: &mut impl Memory, list: u32, x: u16, y: u16) -> Option<(u32, u16)> {
    let mut best: Option<(u32, u16)> = None;
    let mut best_d = 0xFFFFu16;
    let n = m.read_u16(list + L::EDGE_COUNT);
    let mut k = 0u16;
    while k < n {
        let e = list + u32::from(k);
        let dy = m.read_u16(e + 2).wrapping_sub(y);
        if dy <= u16::from(m.read_u8(e + 4)) {
            let ex = m.read_u16(e).wrapping_sub(dy >> 3);
            let d = (ex.wrapping_sub(x) as i16).unsigned_abs();
            if d < best_d {
                best_d = d;
                best = Some((e, ex));
            }
        }
        k += L::EDGE_LEN;
    }
    best
}

/// Borde de plataforma que cruza este frame: su columna y por dónde (1 =
/// por debajo de su mitad, 2 = por encima). Original: `$0770CC`.
pub fn edge_ahead(m: &mut impl Memory, t: u32) -> Option<(u8, u8)> {
    let list = L::EDGES[usize::from(m.read_u8(L::FRAME) & 1)];
    let (x, y) = world_pos(m, t);
    let (e, ex) = nearest_edge(m, list, x, y)?;
    let step = (u16::from(m.read_u8(t + L::SUB_X)).wrapping_add(m.read_u16(t + L::VEL_X)) as i16) >> 8;
    let after = (step as u16).wrapping_add(x).wrapping_sub(ex) as i16 >= 0;
    let before = x.wrapping_sub(ex) as i16 >= 0;
    if after == before {
        return None;
    }
    let mid = m.read_u16(e + 2).wrapping_sub(u16::from(m.read_u8(e + 4)) >> 1);
    let side = if (y.wrapping_sub(mid) as i16) >= 0 { 1 } else { 2 };
    Some((m.read_u8(e + 5), side))
}

/// Si anda hacia delante y cruza un borde, lanza la granada (por arriba o
/// por abajo según lo que le dejen `+$13` bits 4/5).
/// Original: `Soldier_ProbeWalkEdge_056fec` / `_Bit5_05702a`.
pub fn walk_edge(m: &mut impl Memory, t: u32) {
    let facing = m.read_u8(t + L::FACING) & 1 != 0;
    let back = (m.read_u16(t + L::VEL_X) as i16) < 0;
    if facing == back {
        return;
    }
    let Some((col, side)) = edge_ahead(m, t) else { return };
    m.write_u8(t + L::EDGE_COL, col);
    let f = m.read_u8(t + L::FLAGS2);
    if f & L::FLAGS2_UP != 0 && side & 1 != 0 {
        m.write_u32(t + T::HANDLER, L::GRENADE_HIGH);
    } else if f & L::FLAGS2_DOWN != 0 && side & 2 != 0 {
        m.write_u32(t + T::HANDLER, L::GRENADE_LOW);
    }
}

/// ¿Está (`x`, `y`) del mundo en alguna zona del anillo en que no se rinde?
/// Original: `ZoneRing_HitTest_08f2a0`.
pub fn in_zone(m: &mut impl Memory, t: u32) -> bool {
    let (x, y) = world_pos(m, t);
    let end = m.read_u16(L::ZONES + 4);
    let mut k = m.read_u16(L::ZONES);
    while k != end {
        let z = L::ZONES + 8 + u32::from(k);
        if x >= m.read_u16(z) && x <= m.read_u16(z + 4) && y >= m.read_u16(z + 2) && y <= m.read_u16(z + 6) {
            return true;
        }
        k += L::ZONE_LEN;
        if k >= L::ZONES_SIZE {
            k = 0;
        }
    }
    false
}

/// ¿Se rinde? Si puede, está en la zona activa, se le ha acabado el
/// tiempo, mira al jugador y no está en una zona prohibida; entonces
/// recarga el temporizador con la agresividad.
/// Original: `Soldier_TestSurrender_056fa0`.
pub fn surrenders(m: &mut impl Memory, t: u32) -> bool {
    if m.read_u8(t + L::FLAGS_74) & L::CAN_SURRENDER == 0 {
        return false;
    }
    let x = m.read_u16(t + L::X);
    if (x.wrapping_sub(L::ACTIVE_X.0) as i16) < 0 || (x.wrapping_sub(L::ACTIVE_X.1) as i16) >= 0 {
        return false;
    }
    if m.read_u16(t + L::TIMER_SURRENDER) != 0 || m.read_u8(t + L::FLAGS_72) & L::SIDE == 0 || in_zone(m, t) {
        return false;
    }
    let a = m.read_u8(t + L::P_AGGRO);
    m.write_u8(t + L::TIMER_SURRENDER + 1, a);
    true
}

/// ¿Alcanza a alguien cuerpo a cuerpo? Prueba sus cajas de ataque.
/// Original: `Soldier_TestMeleeRange_0574e8`.
pub fn melee(m: &mut impl Memory, t: u32) -> bool {
    let x = m.read_u16(t + L::X);
    if (x.wrapping_sub(L::MELEE_X.0) as i16) < 0 || (x.wrapping_sub(L::MELEE_X.1) as i16) >= 0 {
        return false;
    }
    m.write_u32(t + L::ATTACKS, L::MELEE_ATTACKS);
    slug::clear_flags13(m, t);
    slug::clear_flags13(m, t);
    hits::run_attack_list(m, t);
    m.read_u8(t + L::FLAGS2) & L::MELEE_HIT != 0
}

/// Si le han dado (o no hay escena), pasa a morir en el suelo o en el aire.
/// Original: `HumanProbe_InstallHandler_049fd0`.
pub fn check_death(m: &mut impl Machine, t: u32) {
    if m.read_u8(L::HITS_LIVE) != 0 && fort::hit_side(m, t).is_none() {
        return;
    }
    let r = m.regs();
    let f = collision::probe_floor(m, t, (r.d[3] as u16, r.d[4] as u16));
    m.write_u32(t + T::HANDLER, if f.in_air { L::DEATH_AIR } else { L::DEATH_GROUND });
}

/// Si ha salido de la pantalla, se da de baja.
/// Original: `Soldier_DespawnIfOffscreen_056e1e`.
pub fn despawn(m: &mut impl Machine, t: u32) -> Flow {
    despawn_from(m, t, None)
}

/// Como [`despawn`] llamada con `bsr` desde un estado: si se da de baja,
/// deja en la pila la vuelta `ret` como el original.
pub fn despawn_from(m: &mut impl Machine, t: u32, ret: Option<u32>) -> Flow {
    if !items::out_of_view_seen(m, t, Some(L::DESPAWN_MARGINS)) {
        return Flow::Return;
    }
    match ret {
        Some(then) => Flow::Call { routine: L::TASK_FINISH, then },
        None => tasks::finish(m),
    }
}

/// ¿Va el jugador `target` en el Slug? (y entonces copia su piloto).
fn rides_slug(m: &mut impl Memory, t: u32, target: u32) -> bool {
    match L::PLAYERS.iter().position(|&p| p == target) {
        Some(p) => slug_arm::ridden_by(m, t, p),
        None => false,
    }
}

/// Posición del ancla `k` de la entidad `e` (si la tiene).
/// Original: `PlayerSlot_AnchorPos_08f738`.
pub fn anchor_pos(m: &mut impl Memory, e: u32, k: u8) -> Option<(u16, u16)> {
    let table = m.read_u32(e + L::SLUG_ANCHORS);
    if (table as i32) < 0 {
        return None;
    }
    let a = m.read_u32(table) + u32::from(k) * 4;
    let (dx, dy) = (m.read_u16(a), m.read_u16(a + 2));
    if dx | dy == 0 {
        return None;
    }
    let x = dx.wrapping_add(m.read_u16(e + L::X)).wrapping_add(m.read_u16(table + 4));
    let y = dy.wrapping_add(m.read_u16(e + L::Y)).wrapping_add(m.read_u16(table + 6));
    Some((x, y))
}

/// Anclas del Slug ocupadas o inexistentes (bits 0–2), si el jugador
/// `target` va en él. Original: `PlayerSlot_FindFreeAnchor_08f7ce`.
pub fn taken_anchors(m: &mut impl Memory, t: u32, target: u32) -> Option<u8> {
    if target == 0 || !rides_slug(m, t, target) {
        return None;
    }
    let s = L::SLUG;
    let mut d = m.read_u8(s + L::SLUG_TAKEN[0]) | m.read_u8(s + L::SLUG_TAKEN[1]);
    d |= d >> 4;
    for k in 0..3 {
        if d & (1 << k) == 0 && anchor_pos(m, s, k).is_none() {
            d |= 1 << k;
        }
    }
    Some(d)
}

/// Si el objetivo va en el Slug y queda libre alguna de sus anclas
/// permitidas, elige una al azar. Original: `Soldier_PickGrabAnchor_056e4a`.
pub fn pick_anchor(m: &mut impl Memory, t: u32) -> Option<u16> {
    let allowed = m.read_u8(t + L::FLAGS_74) & L::ANCHOR_MASK;
    if allowed == 0 || m.read_u8(t + L::FLAGS_72) & L::SIDE == 0 {
        return None;
    }
    let target = m.read_u32(t + L::TARGET);
    if target == 0 {
        return None;
    }
    let taken = taken_anchors(m, t, target).unwrap_or(0xFF);
    let free = !taken & allowed;
    if free == 0 {
        return None;
    }
    // El original las apila: la última que mete queda la primera.
    let list: std::vec::Vec<u16> = (0..3u16).rev().filter(|&k| free & (1 << k) != 0).collect();
    let (i, _) = rng::below(m, list.len() as u16);
    Some(list[usize::from(i)])
}

/// ¿Puede ir a por el jugador a pie (no va en el Slug)?
/// Original: `Soldier_TestPlayerInSlug_056eda`.
pub fn player_on_foot(m: &mut impl Memory, t: u32) -> bool {
    if m.read_u8(t + L::FLAGS_74) & L::ANCHOR_MASK == 0 || m.read_u8(t + L::FLAGS_72) & L::SIDE == 0 {
        return false;
    }
    let target = m.read_u32(t + L::TARGET);
    target != 0 && !rides_slug(m, t, target)
}

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

/// Echa a andar hacia donde mira. Original: `Soldier_WalkStart_057558`.
pub fn walk_start(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::WALK_ANIM);
    let v = by_difficulty(m, t, L::WALK_SPEED_TABLE);
    m.write_u16(t + L::SPEED, v);
    set_speed(m, t);
    m.write_u16(t + L::VEL_Y, 0);
    set_handler(m, t, L::WALK_LOOP);
    walk(m)
}

/// Andar: física y percepción; si puede, va a por el Slug o el jugador;
/// si no, tiradas para correr, huir, frenar o saltar, y los bordes, la
/// retirada, la rendición, la caída y el cuerpo a cuerpo.
/// Original: `Soldier_Walk_Loop_057582`.
pub fn walk(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    physics(m, t);
    think(m, t);
    if pick_anchor(m, t).is_some() {
        set_handler(m, t, L::APPROACH_SLUG);
        return approach_slug(m);
    }
    if player_on_foot(m, t) {
        set_handler(m, t, L::APPROACH_PLAYER);
        return approach_player(m);
    }
    anim::step(m, t);
    walk_edge(m, t);
    if m.read_u8(t + L::FLAGS_72) & L::SIDE != 0 {
        let a = m.read_u16(t + L::FAR).max(m.read_u16(t + L::PREF_A));
        if roll(m, 0x100u16.wrapping_sub(a)) {
            set_handler(m, t, L::RUN_TOWARD);
        }
        let n = m.read_u16(t + L::PREF_D);
        if roll(m, n) {
            set_handler(m, t, L::FLEE);
        }
        let n = m.read_u16(t + L::PREF_B).max(m.read_u16(t + L::NEAR));
        if roll(m, n) {
            set_handler(m, t, L::BRAKE);
        }
        let n = m.read_u16(t + L::PREF_C);
        if roll(m, n) {
            set_handler(m, t, L::JUMP);
        }
    } else {
        let n = m.read_u16(t + L::FAR);
        if roll(m, n) {
            set_handler(m, t, L::RUN_TOWARD);
        }
    }
    let f = m.read_u8(t + L::FLAGS_72);
    if f & L::EDGE_SOLID != 0 {
        set_handler(m, t, L::BRAKE);
    }
    if f & L::EDGE_GAP != 0 {
        set_handler(m, t, L::EDGE_FLEE);
    }
    if leave_expired(m, t) {
        set_handler(m, t, L::LEAVE);
    }
    if surrenders(m, t) {
        set_handler(m, t, L::SURRENDER);
    }
    if m.read_u8(t + L::GROUNDED) == 0 {
        set_handler(m, t, L::AIRBORNE);
    }
    if melee(m, t) {
        set_handler(m, t, L::MELEE);
    }
    tail(m, t, Some(L::AFTER_DESPAWN_WALK))
}

/// Tirada para saltar según la cercanía (`NEAR`² / 256) y el resto de
/// condiciones comunes de los dos acercamientos.
fn approach_tail(m: &mut impl Machine, t: u32, leap: u32) {
    let n = m.read_u16(t + L::NEAR) as u32;
    if roll(m, ((n * n) >> 8) as u16) {
        set_handler(m, t, leap);
    }
    let f = m.read_u8(t + L::FLAGS_72);
    if f & L::SIDE == 0 {
        set_handler(m, t, L::BRAKE);
    }
    if m.read_u8(t + L::GROUNDED) == 0 {
        set_handler(m, t, leap);
    }
    if f & L::EDGE_GAP != 0 {
        set_handler(m, t, L::EDGE_FLEE);
    }
    if f & L::EDGE_SOLID != 0 {
        set_handler(m, t, L::RUN_TOWARD);
    }
}

/// Va hacia un ancla libre del Slug y salta a ella; si ya no hay, vuelve a
/// andar. Original: `Soldier_GrabApproach_057728`.
pub fn approach_slug(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    physics(m, t);
    think(m, t);
    anim::step(m, t);
    if pick_anchor(m, t).is_some() {
        approach_tail(m, t, L::LEAP_SLUG);
    } else {
        set_handler(m, t, L::WALK_LOOP);
    }
    tail(m, t, Some(L::AFTER_DESPAWN_APPROACH_SLUG))
}

/// Va hacia el jugador a pie y salta sobre él.
/// Original: `Soldier_SlugApproach_0577bc`.
pub fn approach_player(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    physics(m, t);
    think(m, t);
    anim::step(m, t);
    approach_tail(m, t, L::LEAP_PLAYER);
    tail(m, t, Some(L::AFTER_DESPAWN_APPROACH_PLAYER))
}

/// Sin suelo bajo los pies: uno de cuatro estados al azar.
/// Original: `Soldier_AirborneDispatch_057706`.
pub fn airborne(m: &mut impl Machine) -> Flow {
    let (r, _) = rng::next(m);
    Flow::Goto(L::AIRBORNE_NEXT[usize::from((r & 0xC) >> 2)])
}

// --- Estados del soldado (`asm/enemies/soldiers/states_057d04.s`) -------

/// `VEL_X` −= `VEL_X` >> `k` (frenada gradual).
fn damp(m: &mut impl Memory, t: u32, k: u32) {
    let v = m.read_u16(t + L::VEL_X);
    m.write_u16(t + L::VEL_X, v.wrapping_sub(((v as i16) >> k) as u16));
}

/// `n`² / 256 (las tiradas encadenadas del original).
fn square(n: u16) -> u16 {
    ((u32::from(n) * u32::from(n)) >> 8) as u16
}

/// Como [`check_death`], con la variante de muerte en el suelo de los
/// estados de huida. Original: `$049FF2`.
pub fn check_death_b(m: &mut impl Machine, t: u32) {
    if m.read_u8(L::HITS_LIVE) != 0 && fort::hit_side(m, t).is_none() {
        return;
    }
    let r = m.regs();
    let f = collision::probe_floor(m, t, (r.d[3] as u16, r.d[4] as u16));
    set_handler(m, t, if f.in_air { L::DEATH_AIR } else { L::DEATH_GROUND_B });
}

/// Final común: la muerte y desaparecer fuera de la pantalla.
fn tail(m: &mut impl Machine, t: u32, ret: Option<u32>) -> Flow {
    check_death(m, t);
    despawn_from(m, t, ret)
}

fn tail_b(m: &mut impl Machine, t: u32, ret: Option<u32>) -> Flow {
    check_death_b(m, t);
    despawn_from(m, t, ret)
}

/// Paso con colisión «A» con la gravedad de reserva. Original: `$02831E`.
pub fn move_stop(m: &mut impl Machine, t: u32) -> bool {
    let row = m.regs().d[4] as u16;
    with_alt_gravity(m, t, |m| motion::on_entity_pos(m, t, |m| slug::move_and_stop(m, t, row)))
}

/// Pose de rendición según hacia dónde mira y dónde está el objetivo.
/// Original: `EntityFrame_FrameSelectByGate3A_057044`.
pub fn pose(m: &mut impl Memory, t: u32, target: u32) {
    let [mid, above, below] = L::POSES[usize::from(m.read_u8(t + L::FACING) & 1)];
    let y = m.read_u16(t + L::Y);
    let p = if y >= L::POSE_LOW_Y {
        below
    } else if target == 0 {
        mid
    } else {
        let dy = m.read_u16(target + L::Y).wrapping_sub(y) as i16;
        if dy > L::POSE_ABOVE {
            above
        } else if dy < L::POSE_BELOW {
            below
        } else {
            mid
        }
    };
    m.write_u8(t + L::POSE, p);
}

/// Correr hacia el jugador (o la huida a saltos, si huye).
/// Original: `Soldier_RunToward_05804c`.
pub fn run_toward(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if m.read_u8(t + L::FLAGS_74) & L::FLEES != 0 {
        return hop_flee(m);
    }
    anim::start(m, t, L::ANIM_RUN);
    m.write_u16(t + L::SPEED, L::RUN_SPEED);
    set_speed(m, t);
    set_handler(m, t, L::RUN_LOOP);
    run(m)
}

/// Correr hacia atrás con la velocidad de la dificultad.
/// Original: `Soldier_RunByTable_058076`.
pub fn run_by_table(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::ANIM_RUN_TABLE);
    let v = by_difficulty(m, t, L::WALK_SPEED_TABLE);
    m.write_u16(t + L::SPEED, v.wrapping_neg());
    set_speed(m, t);
    set_handler(m, t, L::RUN_LOOP);
    run(m)
}

/// Original: `Soldier_Run_Loop_05809e`.
pub fn run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    physics(m, t);
    think(m, t);
    damp(m, t, 4);
    if anim::step(m, t) {
        set_handler(m, t, L::IDLE);
    }
    walk_edge(m, t);
    let n = m.read_u16(t + L::PREF_D);
    if roll(m, n) {
        set_handler(m, t, L::FLEE);
    }
    if leave_expired(m, t) {
        set_handler(m, t, L::LEAVE);
    }
    if m.read_u8(t + L::GROUNDED) == 0 {
        set_handler(m, t, L::FALL_START);
    }
    let n = m.read_u16(t + L::PREF_C);
    if roll(m, n) {
        set_handler(m, t, L::JUMP);
    }
    if melee(m, t) {
        set_handler(m, t, L::MELEE);
    }
    tail(m, t, None)
}

/// Quieto. Original: `Soldier_Idle_058144`.
pub fn idle(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    // Las dos ramas del original con gravedad usan la misma animación.
    let a = if m.read_u16(t + L::GRAVITY) == 0 { L::ANIM_IDLE } else { L::ANIM_IDLE_ALT };
    anim::start(m, t, a);
    m.write_u16(t + L::VEL_X, 0);
    m.write_u16(t + L::VEL_Y, 0);
    set_handler(m, t, L::IDLE_LOOP);
    idle_loop(m)
}

/// Quieto: tiradas para dar un paso, correr, frenar, andar, inquietarse,
/// huir, saltar…; la retirada, la rendición, la burla sin objetivo, el
/// cuerpo a cuerpo y la caída. Original: `Soldier_Idle_058144__L058196`.
pub fn idle_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    physics(m, t);
    think(m, t);
    anim::step(m, t);
    walk_edge(m, t);
    if m.read_u8(t + L::FLAGS_72) & L::SIDE != 0 {
        let mut n = m.read_u16(t + L::NEAR).max(m.read_u16(t + L::PREF_B));
        for h in [L::STEP_LEFT, L::RUN_BY_TABLE, L::BRAKE] {
            if roll(m, n) {
                set_handler(m, t, h);
            }
            n = square(n);
        }
        let mut n = m.read_u16(t + L::FAR).max(m.read_u16(t + L::PREF_A));
        for h in [L::STEP_RIGHT, L::WALK_START] {
            if roll(m, n) {
                set_handler(m, t, h);
            }
            n = square(n);
        }
    } else {
        let n = m.read_u16(t + L::NEAR);
        if roll(m, n) {
            set_handler(m, t, L::WALK_START);
        }
        let n = m.read_u16(t + L::FAR);
        if roll(m, n) {
            set_handler(m, t, L::FIDGET);
        }
    }
    let n = m.read_u16(t + L::PREF_B);
    if roll(m, n) {
        set_handler(m, t, L::BRAKE);
    }
    let n = m.read_u16(t + L::PREF_A);
    if roll(m, n) {
        set_handler(m, t, L::WALK_START);
    }
    if m.read_u16(t + L::GRAVITY) == 0 {
        let n = m.read_u16(t + L::PREF_D);
        if roll(m, n) {
            set_handler(m, t, L::FLEE);
        }
    }
    if leave_expired(m, t) {
        set_handler(m, t, L::LEAVE);
    }
    if surrenders(m, t) {
        set_handler(m, t, L::SURRENDER);
    }
    if m.read_u8(t + L::FLAGS_73) & L::NO_TARGET != 0 {
        set_handler(m, t, L::TAUNT);
    }
    let n = m.read_u16(t + L::PREF_C);
    if roll(m, n) {
        set_handler(m, t, L::JUMP);
    }
    if melee(m, t) {
        set_handler(m, t, L::MELEE);
    }
    if m.read_u8(t + L::GROUNDED) == 0 {
        set_handler(m, t, L::FALL_START);
    }
    tail(m, t, Some(L::AFTER_DESPAWN_IDLE))
}

/// Se inquieta (una de cada 16 veces; si no, frena).
/// Original: `Soldier_IdleFidget_0583cc`.
pub fn fidget(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let (r, _) = rng::next(m);
    if r & 0xF != 0 {
        return brake(m);
    }
    anim::start(m, t, L::ANIM_FIDGET);
    set_handler(m, t, L::FIDGET_LOOP);
    fidget_loop(m)
}

/// Original: `Soldier_IdleFidget_0583cc__L0583ec`.
pub fn fidget_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    think(m, t);
    if anim::step(m, t) {
        set_handler(m, t, L::IDLE);
    }
    tail(m, t, Some(L::AFTER_DESPAWN_FIDGET))
}

/// Cayendo. Original: `Soldier_Hurt_058412`.
pub fn hurt(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::ANIM_HURT);
    set_handler(m, t, L::HURT_LOOP);
    hurt_loop(m)
}

/// Original: `Soldier_Hurt_Loop_058424`.
pub fn hurt_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    damp(m, t, 6);
    let landed = fall(m, t);
    m.write_u8(t + L::GROUNDED, if landed { 0xFF } else { 0 });
    if landed {
        set_handler(m, t, L::LAND);
    }
    anim::step(m, t);
    tail(m, t, Some(L::AFTER_DESPAWN_HURT))
}

/// Aterrizaje duro. Original: `Soldier_Hurt_Loop_058424__L058454`.
pub fn land_hard(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::ANIM_LAND_HARD);
    settle(m, t)
}

/// Aterriza: si llega más rápido en horizontal que en vertical, sigue
/// corriendo. Original: `Soldier_Land_058464`.
pub fn land(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let vx = (m.read_u16(t + L::VEL_X) as i16).wrapping_abs() as u16;
    let vy = (m.read_u16(t + L::VEL_Y) as i16).wrapping_abs() as u16;
    if vy < vx {
        return run_toward(m);
    }
    anim::start(m, t, L::ANIM_LAND);
    settle(m, t)
}

fn settle(m: &mut impl Machine, t: u32) -> Flow {
    m.write_u16(t + L::VEL_X, 0);
    set_handler(m, t, L::LAND_LOOP);
    land_loop(m)
}

/// Original: `Soldier_Land_058464__L058494`.
pub fn land_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    physics(m, t);
    if anim::step(m, t) {
        let h = if m.read_u8(t + L::GROUNDED) != 0 { L::WALK_START } else { L::FALL_START };
        set_handler(m, t, h);
    }
    tail(m, t, None)
}

/// Un paso adelante (`back` = atrás). Original: `Soldier_StepRight_0584c4`
/// / `Soldier_StepLeft_058500`.
pub fn step(m: &mut impl Machine, back: bool) -> Flow {
    let t = m.task_register();
    let anims = if back { L::ANIM_STEP_LEFT } else { L::ANIM_STEP_RIGHT };
    let alt = m.read_u16(t + L::GRAVITY) != 0;
    anim::start(m, t, anims[usize::from(alt)]);
    let v = if back { L::STEP_SPEED.wrapping_neg() } else { L::STEP_SPEED };
    m.write_u16(t + L::SPEED, v);
    set_speed(m, t);
    m.write_u16(t + L::VEL_Y, 0);
    set_handler(m, t, L::STEP_LOOP);
    step_loop(m)
}

/// Original: `Soldier_Step_Loop_058538`.
pub fn step_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    physics(m, t);
    think(m, t);
    if anim::step(m, t) {
        set_handler(m, t, L::IDLE);
    }
    walk_edge(m, t);
    let n = m.read_u16(t + L::PREF_C);
    if roll(m, n) {
        set_handler(m, t, L::JUMP);
    }
    if m.read_u8(t + L::GROUNDED) == 0 {
        set_handler(m, t, L::FALL_START);
    }
    if surrenders(m, t) {
        set_handler(m, t, L::SURRENDER);
    }
    if melee(m, t) {
        set_handler(m, t, L::MELEE);
    }
    tail(m, t, None)
}

/// Golpe cuerpo a cuerpo. Original: `Soldier_MeleeAttack_0585ae`.
pub fn melee_attack(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::VEL_X, 0);
    anim::start(m, t, L::ANIM_MELEE);
    m.write_u32(t + L::ATTACKS, L::MELEE_BOXES);
    slug::clear_flags13(m, t);
    slug::clear_flags13(m, t);
    set_handler(m, t, L::MELEE_LOOP);
    melee_loop(m)
}

/// Original: `Soldier_MeleeAttack_0585ae__L0585d8`.
pub fn melee_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    physics(m, t);
    if anim::step(m, t) {
        set_handler(m, t, L::IDLE);
    }
    tail(m, t, None)
}

/// Frena; al acabar corre hacia el jugador o vuelve a andar.
/// Original: `Soldier_Brake_0585f6`.
pub fn brake(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::ANIM_BRAKE);
    set_handler(m, t, L::BRAKE_LOOP);
    brake_loop(m)
}

/// Original: `Soldier_Brake_0585f6__L058608`.
pub fn brake_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    think(m, t);
    damp(m, t, 4);
    physics(m, t);
    if anim::step(m, t) {
        let h = if m.read_u8(t + L::FLAGS_72) & L::SIDE != 0 { L::RUN_TOWARD } else { L::WALK_START };
        set_handler(m, t, h);
    }
    if m.read_u8(t + L::GROUNDED) == 0 {
        set_handler(m, t, L::FALL_START);
    }
    tail(m, t, Some(L::AFTER_DESPAWN_BRAKE))
}

/// Se para a burlarse. Original: `Soldier_TauntInit_058e08`.
pub fn taunt_init(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::TAUNT_LOST, 0);
    m.write_u16(t + L::VEL_X, 0);
    m.write_u16(t + L::VEL_Y, 0);
    taunt(m)
}

/// Una burla al azar. Original: `Soldier_Taunt_058e14`.
pub fn taunt(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let (r, _) = rng::next(m);
    let script = m.read_u32(L::TAUNT_ANIMS + u32::from(r & L::TAUNT_ANIM_MASK));
    anim::start(m, t, script);
    set_handler(m, t, L::TAUNT_LOOP_RUN);
    taunt_loop(m)
}

/// Repite burlas hasta perder al objetivo, tenerlo cerca o que cambie
/// de preferencia: entonces acaba la burla o frena.
/// Original: `Soldier_Taunt_058e14+$1E`.
pub fn taunt_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    physics(m, t);
    think(m, t);
    if anim::step(m, t) {
        set_handler(m, t, L::TAUNT_LOOP);
    }
    let f73 = m.read_u8(t + L::FLAGS_73);
    if f73 & L::NO_TARGET != 0 {
        m.write_u8(t + L::TAUNT_LOST, 1);
    } else {
        if m.read_u8(t + L::TAUNT_LOST) != 0 {
            set_handler(m, t, L::TAUNT_END);
        }
        let n = m.read_u16(t + L::NEAR);
        if roll(m, n) {
            let h = if m.read_u8(t + L::FLAGS_72) & L::SIDE != 0 { L::TAUNT_END } else { L::BRAKE };
            set_handler(m, t, h);
        }
        if f73 & L::PREF_BACK != 0 {
            set_handler(m, t, L::TAUNT_END);
        }
        if f73 & L::PREF_FWD != 0 {
            set_handler(m, t, L::BRAKE);
        }
    }
    tail(m, t, Some(L::AFTER_DESPAWN_TAUNT))
}

/// Fin de la burla; luego frena o vuelve a andar.
/// Original: `Soldier_TauntEnd_058ed2`.
pub fn taunt_end(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::ANIM_TAUNT_END);
    set_handler(m, t, L::TAUNT_END_LOOP);
    taunt_end_loop(m)
}

/// Original: `Soldier_TauntEnd_058ed2+$12`.
pub fn taunt_end_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    think(m, t);
    if anim::step(m, t) {
        let h = if m.read_u8(t + L::FLAGS_72) & L::SIDE != 0 { L::BRAKE } else { L::WALK_START };
        set_handler(m, t, h);
    }
    tail(m, t, Some(L::AFTER_DESPAWN_TAUNT_END))
}

/// Huye (o a saltos, si es de los que huyen). Original: `Soldier_Flee_058658`.
pub fn flee(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if m.read_u8(t + L::FLAGS_74) & L::FLEES != 0 {
        return hop_flee(m);
    }
    anim::start(m, t, L::ANIM_FLEE);
    let v = m.read_u16(t + L::VEL_X);
    m.write_u16(t + L::VEL_X, v.wrapping_add(v));
    m.write_u16(t + L::VEL_Y, 0);
    set_handler(m, t, L::FLEE_LOOP);
    flee_loop(m)
}

/// Original: `Soldier_Flee_058658__L058682`.
pub fn flee_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    damp(m, t, 5);
    physics(m, t);
    think(m, t);
    if anim::step(m, t) && m.read_u8(t + L::GROUNDED) != 0 {
        set_handler(m, t, L::FLEE_STOP);
    }
    tail(m, t, Some(L::AFTER_DESPAWN_FLEE))
}

/// Para de huir. Original: `Soldier_FleeStop_0586b8`.
pub fn flee_stop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::ANIM_FLEE_STOP);
    m.write_u16(t + L::VEL_X, 0);
    set_handler(m, t, L::FLEE_STOP_LOOP);
    flee_stop_loop(m)
}

/// Original: `Soldier_FleeStop_0586b8__L0586ce`.
pub fn flee_stop_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    physics(m, t);
    think(m, t);
    anim::step(m, t);
    let n = 0x100u16.wrapping_sub(m.read_u16(t + L::PREF_D));
    if roll(m, n) {
        set_handler(m, t, L::STAND);
    }
    if m.read_u8(t + L::FLAGS_72) & L::SIDE != 0 {
        let n = m.read_u16(t + L::FAR);
        if roll(m, n) {
            set_handler(m, t, L::RETREAT_TURN);
        }
        let n = m.read_u16(t + L::NEAR);
        if roll(m, n) {
            set_handler(m, t, L::RETREAT);
        }
    } else {
        set_handler(m, t, L::RETREAT);
    }
    if leave_expired(m, t) {
        set_handler(m, t, L::SPAWN_STAND);
    }
    if surrenders(m, t) {
        set_handler(m, t, L::SURRENDER_FLEE);
    }
    if m.read_u8(t + L::GROUNDED) == 0 {
        set_handler(m, t, L::FALL_START);
    }
    tail_b(m, t, Some(L::AFTER_DESPAWN_FLEE_STOP))
}

/// De pie; al acabar la animación, a andar. Original: `Soldier_Stand_0588ae`.
pub fn stand(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::ANIM_STAND);
    m.write_u16(t + L::VEL_X, 0);
    set_handler(m, t, L::STAND_LOOP);
    stand_loop(m)
}

/// Original: `Soldier_Stand_0588ae__L0588c4`.
pub fn stand_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    physics(m, t);
    think(m, t);
    if anim::step(m, t) {
        set_handler(m, t, L::WALK_START);
    }
    if m.read_u8(t + L::GROUNDED) == 0 {
        set_handler(m, t, L::FALL_START);
    }
    tail(m, t, Some(L::AFTER_DESPAWN_STAND))
}

/// Salta hacia delante o atrás según las preferencias.
/// Original: `Soldier_Jump_0588f6`.
pub fn jump(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let r = |m: &mut _, f| Memory::read_u16(m, t + f);
    let d = r(m, L::PREF_A).wrapping_add(r(m, L::FAR)).wrapping_sub(r(m, L::PREF_B)).wrapping_sub(r(m, L::NEAR));
    let d = ((d as i16) >> 1) as u16;
    let mask = if m.read_u8(t + L::FACING) & 1 == 0 { 0xFFFF } else { 0 };
    m.write_u16(t + L::VEL_X, d ^ mask);
    let vy = m.read_u16(t + L::PREF_C) << 3;
    if (d as i16) >= 0 {
        anim::start(m, t, L::ANIM_JUMP_FWD);
        m.write_u16(t + L::VEL_Y, vy);
    } else {
        anim::start(m, t, L::ANIM_JUMP_BACK);
        m.write_u16(t + L::VEL_Y, vy);
        m.write_u16(t + L::SPEED, L::BACK_SPEED);
        set_speed(m, t);
    }
    set_handler(m, t, L::HURT_LOOP);
    hurt_loop(m)
}

/// Se rinde (con la animación de su subestado). Original:
/// `Soldier_Surrender_058968`.
pub fn surrender(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let a = match m.read_u8(t + L::SUBSTATE) {
        3 => {
            let (r, _) = rng::next(m);
            L::ANIM_SURRENDER[if (r as i16) < 0 { 1 } else { 2 }]
        }
        1 => L::ANIM_SURRENDER[1],
        2 => L::ANIM_SURRENDER[2],
        _ => L::ANIM_SURRENDER[0],
    };
    anim::start(m, t, a);
    let target = m.read_u32(t + L::TARGET);
    pose(m, t, target);
    m.write_u16(t + L::VEL_X, 0);
    set_handler(m, t, L::SURRENDER_LOOP);
    surrender_loop(m)
}

/// Original: `Soldier_Surrender_058968__L0589c8`.
pub fn surrender_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    surrenders(m, t);
    physics(m, t);
    if anim::step(m, t) {
        set_handler(m, t, L::IDLE);
    }
    if m.read_u8(t + L::GROUNDED) == 0 {
        set_handler(m, t, L::FALL_START);
    }
    tail(m, t, None)
}

/// Rendición al huir (sólo el subestado 0; los demás se retiran).
/// Original: `Soldier_SurrenderFlee_0589f8`.
pub fn surrender_flee(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if m.read_u8(t + L::SUBSTATE) != 0 {
        return Flow::Goto(L::RETREAT);
    }
    anim::start(m, t, L::ANIM_SURRENDER_FLEE);
    m.write_u8(t + L::POSE, L::POSE_SURRENDER_FLEE);
    m.write_u16(t + L::VEL_X, 0);
    set_handler(m, t, L::SURRENDER_FLEE_LOOP);
    surrender_flee_loop(m)
}

/// Original: `Soldier_SurrenderFlee_0589f8__L058a20`.
pub fn surrender_flee_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    surrenders(m, t);
    physics(m, t);
    if anim::step(m, t) {
        set_handler(m, t, L::FLEE_STOP);
    }
    if m.read_u8(t + L::GROUNDED) == 0 {
        set_handler(m, t, L::FALL_START);
    }
    tail_b(m, t, None)
}

/// Salta hacia atrás al ver un hueco delante.
/// Original: `Soldier_SurrenderFlee_0589f8__L058a50`.
pub fn edge_flee(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::ANIM_JUMP_FWD);
    let (vx, vy, ay) = L::EDGE_JUMP;
    let vx = if m.read_u8(t + L::FACING) & 1 != 0 { vx.wrapping_neg() } else { vx };
    m.write_u16(t + L::VEL_X, vx);
    m.write_u16(t + L::VEL_Y, vy);
    m.write_u16(t + L::ACC_Y, ay);
    m.write_u16(t + L::ACC_X, 0);
    set_handler(m, t, L::EDGE_FLEE_LOOP);
    edge_flee_loop(m)
}

/// Original: `Soldier_SurrenderFlee_0589f8__L058a82`.
pub fn edge_flee_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let landed = move_stop(m, t);
    m.write_u8(t + L::GROUNDED, if landed { 0xFF } else { 0 });
    if landed {
        set_handler(m, t, L::LAND);
    }
    anim::step(m, t);
    tail(m, t, Some(L::AFTER_DESPAWN_EDGE_FLEE))
}

/// Huida a saltitos (si ve al jugador; si no, frena).
/// Original: `Soldier_SurrenderFlee_0589f8__L058aa8`.
pub fn hop_flee(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    think(m, t);
    if m.read_u8(t + L::FLAGS_72) & L::SIDE == 0 {
        return brake(m);
    }
    m.write_u16(t + L::SPEED, L::HOP_SPEED);
    set_speed(m, t);
    anim::start(m, t, L::ANIM_HOP_FLEE);
    set_handler(m, t, L::HOP_FLEE_LOOP);
    hop_flee_loop(m)
}

/// Original: `Soldier_SurrenderFlee_0589f8__L058ad2`.
pub fn hop_flee_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let v = m.read_u16(t + L::VEL_X);
    if v != 0 {
        let d = match ((v as i16) >> 4) as u16 {
            0 => 1,
            d => d,
        };
        m.write_u16(t + L::VEL_X, v.wrapping_sub(d));
    }
    physics(m, t);
    if anim::step(m, t) {
        let h = if m.read_u8(t + L::FLAGS_74) & L::ANCHOR_MASK != 0 { L::STAND } else { L::SPAWN_STAND };
        set_handler(m, t, h);
    }
    tail(m, t, Some(L::AFTER_DESPAWN_HOP_FLEE))
}
