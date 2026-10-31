//! Guardia cuerpo a cuerpo: dos variantes con su plantilla. Vigila desde
//! su sitio con una sombra; cuando el jugador se acerca a su zona se
//! «despierta», se vuelve hacia el jugador más cercano, y con el enfriamiento
//! agotado se lanza a por él (aproximación); si lo tiene encima retrocede.
//! Al acabar su tiempo de vida pasa a la rutina de salida de su variante.
//!
//! Original: `TargetFacingTest_0423AC`, `MeleeGuard_*` (`$0423EC`–`$0427F8`,
//! `asm/enemies/soldiers/melee_guard_0422ea.s`).

use super::{anim, charger, pickup, platforms, soldier, tasks};
use crate::layout::{melee_guard as L, task as T};
use crate::mem::{Flow, Machine, Memory};

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

/// ¿Da la espalda al jugador más cercano? Devuelve también el jugador (0
/// si no hay; entonces `false`). Original: `TargetFacingTest_0423AC` (C).
pub fn facing_test(m: &mut impl Memory, t: u32) -> (bool, u32) {
    let (_, p) = soldier::nearest_player(m, t);
    let Some(p) = p else { return (false, 0) };
    let left = (m.read_u16(p + L::X).wrapping_sub(m.read_u16(t + L::X)) as i16) < 0;
    let f = m.read_u8(t + L::FACING) & 1 != 0;
    (f == left, p)
}

/// Física del frame y marca de «apoyado» (sólo con el seguimiento activo).
/// Original: `MeleeGuard_TrackTarget_0423EC`.
pub fn track(m: &mut impl Machine, t: u32) {
    platforms::move_with_scene(m, t);
    if m.read_u8(t + L::TRACKING) == 0 {
        return;
    }
    let f = soldier::probe(m, t);
    let latch = if f.in_air { 0 } else { 0xFF };
    m.write_u8(t + L::GROUNDED, latch);
    if latch == 0 {
        let landed = soldier::fall(m, t);
        m.write_u8(t + L::GROUNDED, if landed { 0xFF } else { 0 });
    }
}

/// Se despierta al entrar el jugador en su zona; después baja el
/// enfriamiento y el tiempo de vida. Original: `MeleeGuard_EngageAndTimers_042436`.
pub fn engage(m: &mut impl Memory, t: u32) {
    if m.read_u8(t + L::AWAKE) == 0 {
        let reach = L::ZONE_EDGE.wrapping_sub(u16::from(m.read_u8(t + L::P_ZONE)) << 3);
        if reach.wrapping_sub(m.read_u16(t + L::X)) as i16 >= 0 {
            m.write_u8(t + L::AWAKE, 1);
        }
        return;
    }
    for f in [L::COOLDOWN, L::LIFE] {
        let v = m.read_u16(t + f);
        if v != 0 {
            m.write_u16(t + f, v - 1);
        }
    }
}

/// Con el enfriamiento agotado, en pantalla y con objetivo, se lanza.
/// Original: `MeleeGuard_MeleeGate_04247C`.
pub fn gate(m: &mut impl Memory, t: u32) {
    if m.read_u16(t + L::COOLDOWN) != 0 {
        return;
    }
    if m.read_u16(t + L::X).wrapping_sub(L::VIEW_X.0) >= L::VIEW_X.1 {
        return;
    }
    if m.read_u32(t + L::TARGET) == 0 {
        return;
    }
    let p = m.read_u8(t + L::P_ATTACK);
    m.write_u8(t + L::ATTACK_TIMER, p);
    set_handler(m, t, L::APPROACH);
}

/// ¿Tiene al jugador más cercano (en X) encima? Devuelve también el
/// jugador mirado. Original: `MeleeGuard_MeleeRange_042512` (C).
pub fn melee_range(m: &mut impl Memory, t: u32) -> (bool, u32) {
    let p = pickup::nearest_player_x(m, t).unwrap_or(crate::layout::PLAYER_ENTITY[1]);
    let dx = m.read_u16(p + L::X).wrapping_sub(m.read_u16(t + L::X));
    let dy = m.read_u16(p + L::Y).wrapping_sub(m.read_u16(t + L::Y));
    (dy < L::MELEE_BOX.1 && dx.wrapping_add(L::MELEE_BOX.0) < L::MELEE_BOX.0 * 2, p)
}

/// Variante A: cada 32 frames se vuelve hacia el jugador y elige el ataque
/// por la distancia vertical; si lo tiene encima retrocede; si hay un caído
/// reciente cerca, sale. Original: `MeleeGuard_Think_0424AA`.
pub fn think(m: &mut impl Machine, t: u32) {
    if m.read_u8(L::FRAME) & L::THINK_MASK == 0 {
        let (away, p) = facing_test(m, t);
        if away {
            let f = m.read_u8(t + L::FACING);
            m.write_u8(t + L::FACING, f ^ 1);
        }
        m.write_u32(t + L::TARGET, p);
        if p != 0 {
            let dy = m.read_u16(t + L::Y).wrapping_sub(m.read_u16(p + L::Y)) as i16;
            m.write_u8(t + L::ATTACK, if dy >= L::NEAR_DY { L::ATTACK_FAR } else { L::ATTACK_NEAR });
        }
    }
    if melee_range(m, t).0 {
        set_handler(m, t, L::RETREAT);
    }
    if soldier::ring_near(m, t).is_some() {
        set_handler(m, t, L::EXIT_A);
    }
}

/// Variante B: guion de ataque según mire o no al jugador.
/// Original: `MeleeGuard_FaceSelect_04253A`.
pub fn face_select(m: &mut impl Memory, t: u32) {
    let (away, p) = facing_test(m, t);
    let (script, attack) = L::FACE_ATTACK[usize::from(away)];
    m.write_u32(t + L::ALT_SCRIPT, script);
    m.write_u8(t + L::ATTACK, attack);
    m.write_u32(t + L::TARGET, p);
}

/// Sin tiempo de vida, pasa a su rutina de salida.
/// Original: `MeleeGuard_TimerHandoff_042568`.
pub fn handoff(m: &mut impl Memory, t: u32) {
    if m.read_u16(t + L::LIFE) == 0 {
        let h = m.read_u32(t + L::EXIT);
        set_handler(m, t, h);
    }
}

fn tail(m: &mut impl Machine, t: u32) -> Flow {
    soldier::check_death(m, t);
    charger::tail_despawn(m)
}

/// `table · p / 256` (de la tabla según la dificultad).
fn by_difficulty_scaled(m: &mut impl Memory, t: u32, table: u32, p: u8) -> u32 {
    let base = soldier::by_difficulty(m, t, table);
    (u32::from(base) * u32::from(p)) >> 8
}

/// Alta de la variante `v` (0 = A, 1 = B): plantillas, sombra, sprite,
/// enfriamiento, ataque y vida según la dificultad.
/// Original: `MeleeGuard_Handler_04258E` / `MeleeGuard_HandlerB_0425B6`.
pub fn start(m: &mut impl Machine, v: usize) -> Flow {
    let t = m.task_register();
    let k = L::VARIANTS[v];
    m.write_u8(t + L::VARIANT, v as u8);
    m.write_u32(t + L::BASE_SCRIPT, k.0);
    m.write_u32(t + L::ALT_SCRIPT, k.1);
    m.write_u8(t + L::TRACKING, k.2);
    m.write_u32(t + L::EXIT, k.3);
    tasks::spawn(m, L::SHADOW, t);
    anim::alloc_entry(m, t, L::GFX);
    // `scc` tras `$027F60`, que siempre sale con C = 0.
    soldier::on_ground(m, t);
    m.write_u8(t + L::GROUNDED, 0xFF);
    m.write_u8(t + L::AWAKE, 0);
    m.write_u8(t + L::ATTACK, L::ATTACK_NEAR);
    let p = m.read_u8(t + L::P_COOLDOWN);
    let cool = if p == 0 { 0xFFFF } else { by_difficulty_scaled(m, t, L::COOLDOWN_TABLE, p) as u16 };
    m.write_u16(t + L::COOLDOWN, cool);
    let p = m.read_u8(t + L::P_ATTACK);
    let a = by_difficulty_scaled(m, t, L::ATTACK_TABLE, p);
    m.write_u8(t + L::P_ATTACK, if a >= 0x100 { 0xFF } else { a as u8 });
    let p = m.read_u8(t + L::P_LIFE);
    m.write_u16(t + L::LIFE, if p == 0 { 0xFFFF } else { u16::from(p) << 6 });
    anim::set_attr(m, t, L::ATTR_INIT);
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, (a & L::ATTR_KEEP) | L::ATTR_SET);
    reapply_base(m)
}

/// Vuelve al guion base. Original: `MeleeGuard_ReapplyBase_042682`.
pub fn reapply_base(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let s = m.read_u32(t + L::BASE_SCRIPT);
    anim::start(m, t, s);
    set_handler(m, t, L::RUN);
    run(m)
}

/// Bucle de guardia. Original: `MeleeGuard_Handler_04258E+$104`.
pub fn run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    engage(m, t);
    track(m, t);
    anim::step(m, t);
    if m.read_u8(t + L::VARIANT) == 0 {
        think(m, t);
        if m.read_u8(t + L::GROUNDED) == 0 {
            set_handler(m, t, L::IDLE);
        }
    } else {
        face_select(m, t);
    }
    gate(m, t);
    handoff(m, t);
    tail(m, t)
}

/// En el aire: espera hasta apoyarse. Original: `MeleeGuard_IdleState_0426D8`.
pub fn idle(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::S_IDLE);
    set_handler(m, t, L::IDLE_RUN);
    idle_run(m)
}

/// Original: `MeleeGuard_IdleState_0426D8+$12`.
pub fn idle_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    track(m, t);
    if anim::step(m, t) && m.read_u8(t + L::GROUNDED) != 0 {
        set_handler(m, t, L::RECOVER);
    }
    tail(m, t)
}

/// Se recompone al apoyarse. Original: `MeleeGuard_RecoverState_042710`.
pub fn recover(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::S_RECOVER);
    set_handler(m, t, L::RECOVER_RUN);
    recover_run(m)
}

/// Original: `MeleeGuard_RecoverState_042710+$12`.
pub fn recover_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    track(m, t);
    if anim::step(m, t) {
        set_handler(m, t, L::REAPPLY);
    }
    tail(m, t)
}

/// Se lanza con el ataque elegido. Original: `MeleeGuard_ApproachState_042740`.
pub fn approach(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let a = m.read_u8(t + L::ATTACK);
    m.write_u8(t + L::ANIM_SLOT, a);
    let s = m.read_u32(t + L::ALT_SCRIPT);
    anim::start(m, t, s);
    set_handler(m, t, L::APPROACH_RUN);
    approach_run(m)
}

/// Original: `MeleeGuard_ApproachState_042740+$16`.
pub fn approach_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    engage(m, t);
    track(m, t);
    if anim::step(m, t) {
        set_handler(m, t, L::REAPPLY);
    }
    // El original compara la palabra baja de `d0` tras cargar sólo el byte
    // de la variante: el byte alto es el que dejó la animación.
    let hi = m.regs().d[0] as u16 & 0xFF00;
    if hi | u16::from(m.read_u8(t + L::VARIANT)) == 0 && melee_range(m, t).0 {
        set_handler(m, t, L::RETREAT);
    }
    tail(m, t)
}

/// Retrocede mientras lo tenga encima. Original: `MeleeGuard_RetreatState_042794`.
pub fn retreat(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::S_RETREAT);
    set_handler(m, t, L::RETREAT_RUN);
    retreat_run(m)
}

/// Original: `MeleeGuard_RetreatState_042794+$12`.
pub fn retreat_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    track(m, t);
    anim::step(m, t);
    if !melee_range(m, t).0 {
        set_handler(m, t, L::REAPPLY);
    }
    tail(m, t)
}

/// Pasa a soldado normal (sprite y su rutina de alta).
/// Original: `MeleeGuard_DeathToExtern_0427CA`.
pub fn to_soldier(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::GFX);
    anim::set_attr(m, t, L::ATTR_INIT);
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, (a & L::ATTR_KEEP) | L::ATTR_SET);
    platforms::move_with_scene(m, t);
    set_handler(m, t, L::SOLDIER);
    Flow::Return
}
