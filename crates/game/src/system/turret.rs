//! Torreta del Slug (y de otros vehículos con torreta). Cada frame se pone
//! en el soporte del chasis y lee el mando del piloto. Gira hacia el ángulo
//! que pide la dirección: suavizando o, en la variante rápida, casi de
//! golpe. Además dispara ráfagas de tres balas.
//!
//! Original: `Turret_InitDir0..4` (`$0459A8`…`$045ABE`),
//! `Turret_SetupCommon_045A0A`, `Turret_Track_045A38`,
//! `Turret_AimSmooth_045B02`, `Turret_Tail_045B9C`,
//! `Turret_FollowParent_0458DC`, `Turret_ReadPlayerInput_04593A`,
//! `Turret_FireGroundSnap_04580C`, `Ent_SfxByMode_0457A6`,
//! `Ent_AnimFrame_0457CC` (`asm/enemies/vehicles/turret_boss2_04580c.s`) y
//! los ayudantes `$05CA2A`, `$08F714`, `$05E452`, `$05D674`, `$0517FE`
//! y `$0280C6`.

use super::{anim, motion, sound, sprite, tasks, video};
use crate::layout::{slug, task as T, turret as L};
use crate::mem::{Machine, Memory};
use crate::Flow;

fn owner(m: &mut impl Memory, t: u32) -> u32 {
    m.read_u32(t + T::OWNER)
}

/// Sector del ángulo: el byte alto redondeado a múltiplos de 8.
fn sector(m: &mut impl Memory, t: u32) -> u16 {
    u16::from(m.read_u8(t + L::ANGLE).wrapping_add(4) & 0xF8)
}

/// Pide el sprite de la torreta: el de los jugadores si la controla uno.
/// Original: `Ent_SfxByMode_0457A6`.
pub fn alloc_gfx(m: &mut impl Memory, t: u32) {
    let gfx = match m.read_u8(t + L::CONTROLLER) {
        1 | 2 => L::GFX_PLAYER,
        _ => L::GFX_DEFAULT,
    };
    anim::alloc_entry(m, t, gfx);
}

/// Dibuja la tarea con su sprite, posición, atributos, paleta, reducción
/// y espejo. Original: `$05CA2A`.
pub fn draw(m: &mut impl Memory, t: u32) {
    let req = sprite::Request {
        pos: m.read_u32(t + L::X),
        key: m.read_u16(t + L::ATTR),
        palette: m.read_u16(t + L::PALETTE),
        shrink_v: m.read_u8(t + L::SHRINK[0]),
        shrink_h: m.read_u8(t + L::SHRINK[1]),
        flip: m.read_u8(t + L::FLIP) & 3,
        gfx: m.read_u32(t + L::GFX),
    };
    sprite::dispatch(m, t, req, false);
}

/// Fotograma según el sector del ángulo y el retroceso; lo dibuja y gasta
/// un frame de retroceso. Original: `Ent_AnimFrame_0457CC`.
pub fn anim_frame(m: &mut impl Memory, t: u32) {
    let recoil = u32::from(m.read_u8(t + L::RECOIL));
    let at = L::FRAMES + u32::from(sector(m, t)) * 3 + recoil * 4;
    let gfx = m.read_u32(at);
    m.write_u32(t + L::GFX, gfx);
    draw(m, t);
    let r = m.read_u8(t + L::RECOIL);
    m.write_u8(t + L::RECOIL, r.saturating_sub(1));
}

/// Se coloca en el soporte del chasis; si cambia quién la controla,
/// cambia de sprite. Original: `Turret_FollowParent_0458DC`.
pub fn follow(m: &mut impl Memory, t: u32) {
    let o = owner(m, t);
    let v = m.read_u8(o + L::TIMER_59);
    m.write_u8(t + L::TIMER_59, v);
    let attr = m.read_u16(o + L::ATTR) | 1;
    let mount = m.read_u32(o + L::OWNER_MOUNT);
    let base = m.read_u32(mount);
    let x = m.read_u16(o + L::X).wrapping_add(m.read_u16(mount + 4)).wrapping_add(m.read_u16(base));
    let y = m.read_u16(o + L::Y).wrapping_add(m.read_u16(mount + 6)).wrapping_add(m.read_u16(base + 2));
    m.write_u16(t + L::ATTR, attr);
    m.write_u16(t + L::X, x);
    m.write_u16(t + L::Y, y);
    let c = m.read_u8(o + L::CONTROLLER);
    if c != m.read_u8(t + L::CONTROLLER) {
        m.write_u8(t + L::CONTROLLER, c);
        let p = m.read_u8(o + L::PILOT);
        m.write_u8(t + L::PILOT, p);
        video::release_task_entries(m, t);
        alloc_gfx(m, t);
    }
}

/// Mando que se lee de verdad: el sintético si conduce el piloto
/// automático. Original: `$05D674`.
pub fn effective_pad(m: &mut impl Memory, pad: u32) -> u32 {
    if m.read_u8(L::SCRIPTED_INPUT_ACTIVE) == 0 {
        pad
    } else if pad == L::PAD_LIVE_P1 {
        L::PAD_SCRIPTED[0]
    } else {
        L::PAD_SCRIPTED[1]
    }
}

/// Dirección del mando del piloto y, al pulsar disparo, una ráfaga.
/// Original: `Turret_ReadPlayerInput_04593A`.
pub fn read_input(m: &mut impl Memory, t: u32) {
    let c = m.read_u8(t + L::CONTROLLER);
    if (c as i8) < 0 {
        m.write_u8(t + L::STICK, 0);
        m.write_u8(t + L::SHOTS, 0);
        return;
    }
    let player = L::PLAYERS[if c == 1 { 0 } else { 1 }];
    let ctl = m.read_u32(player + T::OWNER);
    let pad = m.read_u32(ctl + L::PAD_PTR);
    let pad = effective_pad(m, pad);
    let stick = m.read_u8(pad + L::PAD_HELD) & L::STICK_MASK;
    m.write_u8(t + L::STICK, stick);
    if m.read_u8(pad + L::PAD_PRESSED) & L::FIRE != 0 {
        sound::play(m, L::FIRE_SOUND);
        // Si no había ráfaga en curso, la primera bala sale ya.
        let busy = m.read_u8(t + L::SHOTS) != 0;
        m.write_u8(t + L::SHOTS, L::BURST);
        if !busy {
            m.write_u8(t + L::SHOT_DELAY, 0);
        }
    }
}

/// Copia el piloto y el mando de `from` a `to`. Original: `$0517FE`
/// (con `$05CCC8`).
pub fn copy_pilot(m: &mut impl Memory, from: u32, to: u32) {
    for f in [L::PILOT, L::CONTROLLER] {
        let v = m.read_u8(from + f);
        m.write_u8(to + f, v);
    }
}

/// Tipo de tile (o de plataforma) en un punto. Original: `$0280C6`
/// (C = no vacío).
pub fn kind_at(m: &mut impl Memory, t: u32, x: u16, y: u16) -> u8 {
    motion::surface(m, t, x, y, (0, 0)).kind & 0x3F
}

/// Dispara una bala en el ángulo actual y, si la punta del cañón no está
/// metida en una pared, el destello; si lo está, sólo retrocede.
/// Original: `Turret_FireGroundSnap_04580C`.
pub fn fire(m: &mut impl Memory, t: u32) {
    let o = owner(m, t);
    let p = m.read_u8(o + L::PILOT);
    m.write_u8(t + L::PILOT, p);
    let (b, _) = tasks::spawn_last(m, L::SHOT, t);
    anim::copy_sprite_pos(m, t, b);
    copy_pilot(m, t, b);
    let a = m.read_u8(t + L::ANGLE).wrapping_add(4);
    m.write_u8(b + L::SHOT_ANGLE, a);
    for (k, f) in [L::SHOT_COUNT, L::SHOT_INDEX].into_iter().enumerate() {
        let v = m.read_u8(t + f);
        m.write_u8(b + L::SHOT_SLOT[k], v);
    }
    let i = m.read_u8(t + L::SHOT_INDEX).wrapping_add(1);
    m.write_u8(t + L::SHOT_INDEX, i);
    let o = owner(m, t);
    let f = m.read_u8(o + L::FLAGS_8C);
    m.write_u8(o + L::FLAGS_8C, f | L::FLAGS_8C_FIRED);
    let n = m.read_u8(t + L::SHOT_COUNT).wrapping_sub(1);
    m.write_u8(t + L::SHOT_COUNT, n);
    let s = u32::from(sector(m, t) >> 1);
    let x = m.read_u16(t + L::X).wrapping_add(m.read_u16(L::MUZZLE + s));
    let y = m.read_u16(t + L::Y).wrapping_add(m.read_u16(L::MUZZLE + s + 2));
    let kind = kind_at(m, t, x, y);
    if kind != 0 && (L::PASS_KINDS.contains(&kind) || kind & 0xF0 == 0) {
        let fl = tasks::spawn(m, L::FLASH, t);
        anim::copy_sprite_pos(m, t, fl);
        let d = (sector(m, t) >> 3) as u8;
        m.write_u8(fl + L::SHOT_ANGLE, d);
    } else {
        m.write_u8(t + L::RECOIL, L::RECOIL_FRAMES);
    }
}

/// Alta de la torreta con la variante `k` (0–4). Original:
/// `Turret_InitDir0_0459A8` … `Turret_InitDir4_045ABE` y
/// `Turret_SetupCommon_045A0A`.
pub fn init(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    m.write_u8(t + L::VARIANT, k as u8);
    m.write_u32(t + L::TABLE, L::TABLES[k]);
    m.write_u16(t + L::ANGLE, L::START_ANGLE[k]);
    let snap = k == L::SNAP;
    if snap {
        m.write_u16(t + L::LAST_TARGET, 0);
    }
    video::release_task_entries(m, t);
    anim::alloc_entry(m, t, L::GFX_DEFAULT);
    m.write_u8(t + L::PILOT, L::NONE);
    m.write_u8(t + L::CONTROLLER, L::NONE);
    if !snap {
        m.write_u8(t + L::SHOT_INDEX, 0);
    }
    m.write_u8(t + L::RECOIL, 0);
    m.write_u16(t + L::TURN_SPEED, 0);
    if snap {
        m.write_u32(t + T::HANDLER, L::SNAP_TRACK);
        snap_track(m)
    } else {
        m.write_u32(t + T::HANDLER, L::TRACK);
        track(m)
    }
}

/// Ángulo que pide el mando (`None` = ninguno).
fn target(m: &mut impl Memory, t: u32) -> Option<u16> {
    let table = m.read_u32(t + L::TABLE);
    let stick = u32::from(m.read_u8(t + L::STICK) & L::STICK_MASK);
    let v = m.read_u32(table + stick * 4);
    ((v as i32) >= 0).then_some(v as u16)
}

/// Gira hacia `to`. La velocidad crece `accel` por frame y como mucho es
/// lo que falta desplazado `shift` bits. En la variante rápida acelera más
/// si está lejos.
fn turn(m: &mut impl Memory, t: u32, to: u16, accel: u16, shift: u32, far_accel: bool) {
    let diff = to.wrapping_sub(m.read_u16(t + L::ANGLE)) as i16;
    let dist = diff.wrapping_abs();
    let accel = if far_accel && dist as u16 >= L::SNAP_FAR { L::SNAP_ACCEL_FAR } else { accel };
    let left = (dist >> shift) as u16;
    let speed = m.read_u16(t + L::TURN_SPEED).wrapping_add(accel).min(left);
    m.write_u16(t + L::TURN_SPEED, speed);
    let step = if diff < 0 { speed.wrapping_neg() } else { speed };
    let a = m.read_u16(t + L::ANGLE).wrapping_add(step);
    m.write_u16(t + L::ANGLE, a);
}

/// Cadencia de la ráfaga: una bala cada dos frames mientras queden.
fn burst(m: &mut impl Memory, t: u32) {
    let d = m.read_u8(t + L::SHOT_DELAY);
    if d != 0 {
        m.write_u8(t + L::SHOT_DELAY, d - 1);
        return;
    }
    m.write_u8(t + L::SHOT_DELAY, 1);
    let n = m.read_u8(t + L::SHOTS);
    if n != 0 {
        m.write_u8(t + L::SHOTS, n - 1);
        fire(m, t);
    }
}

/// Torreta con giro suavizado. Original: `Turret_Track_045A38`.
pub fn track(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    follow(m, t);
    read_input(m, t);
    match target(m, t) {
        Some(to) => turn(m, t, to, L::TURN_ACCEL, 2, false),
        None => m.write_u16(t + L::TURN_SPEED, 0),
    }
    burst(m, t);
    anim_frame(m, t);
    tail(m)
}

/// Torreta de giro rápido (sigue hacia el último ángulo pedido).
/// Original: `Turret_AimSmooth_045B02`.
pub fn snap_track(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    follow(m, t);
    read_input(m, t);
    let to = match target(m, t) {
        Some(to) => {
            m.write_u16(t + L::LAST_TARGET, to);
            to
        }
        None => m.read_u16(t + L::LAST_TARGET),
    };
    turn(m, t, to, L::SNAP_ACCEL, 1, true);
    burst(m, t);
    anim_frame(m, t);
    tail(m)
}

/// ¿Tiene `t` el bit `bit` en la máscara de jugadores de este frame (la de
/// `+$96` o la de `+$97`, alternando)? Original: `$08F714` (C = sí).
pub fn player_mask_hit(m: &mut impl Memory, t: u32, bit: u8) -> bool {
    let k = u32::from(m.read_u8(L::FRAME_COUNTER) & 1);
    m.read_u8(t + L::PLAYER_MASK + k) & bit != 0
}

/// ¿Sigue en marcha el dueño de `t`? Original: `$05E452` (con `$05E3BA`).
pub fn owner_alive(m: &mut impl Memory, t: u32) -> bool {
    let o = owner(m, t);
    !slug::INACTIVE_HANDLERS.contains(&m.read_u32(o + T::HANDLER))
}

/// Cola de cada frame: si el chasis pierde la torreta, explota. Si el
/// chasis desaparece o acaba, deja una bala suelta y la torreta se da de
/// baja. Original: `Turret_Tail_045B9C`.
pub fn tail(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let o = owner(m, t);
    let p = m.read_u8(o + L::PILOT);
    m.write_u8(t + L::PILOT, p);
    let f = m.read_u8(t + L::FLAGS_8C);
    m.write_u8(t + L::FLAGS_8C, f & !L::FLAGS_8C_HIT);
    if player_mask_hit(m, o, L::LOST_BIT) {
        m.write_u32(t + T::HANDLER, L::EXPLODE);
    }
    if owner_alive(m, t) && m.read_u8(o + L::OWNER_FLAGS_13) & L::OWNER_DONE == 0 {
        return Flow::Return;
    }
    let s = tasks::spawn(m, L::LOOSE_SHOT, L::ENEMY_ROOT);
    for f in [L::X, L::Y] {
        let v = m.read_u16(t + f);
        m.write_u16(s + f, v);
    }
    copy_pilot(m, t, s);
    tasks::kill_children(m, t);
    tasks::finish(m)
}
