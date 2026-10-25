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

use super::{anim, motion, rng, slug, slug_arm, tasks, turret, video};
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
