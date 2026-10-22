//! Fuerte de la escena 4: un contenedor con tres o cuatro casamatas, dos
//! cajas y una trampilla de la que salen soldados. Hay cinco variantes
//! (una por cada fuerte de la escena); todas comparten el bucle: se mueve
//! con la escena, suena al recibir golpes, recupera la vida mientras está
//! lejos y al quedarse sin vida pasa a destruido.
//!
//! También los ayudantes que usan todas las piezas de la escena:
//! colocarse según su registro de aparición, de qué lado le han dado, el
//! sonido del golpe y la baja al salir por la izquierda.
//!
//! Original: `asm/enemies/bosses/fort_scene4_0865be.s` (`Fort_Init_V0…V4`,
//! `Fort_Init_Common_086738`, `Fort_SpawnChildren_V0…V4`) y `$02942A`,
//! `$02870A`, `$04FA70`, `$05DD22`, `$05E770`.

use super::{anim, platforms, sound, tasks};
use crate::layout::{fort as L, task as T};
use crate::mem::{Flow, Machine, Memory};

/// Coloca la tarea según su registro de aparición `rec`: posición en
/// pantalla, atributos, margen y flags; si el registro trae gráficos, le
/// da su entrada de sprite y arranca su animación.
/// Original: `ScriptOp1E_SpawnChild_02942a`.
pub fn place_from_record(m: &mut impl Memory, t: u32, rec: u32) {
    let mut x = m.read_u16(rec + L::REC_X);
    if L::WRAPPED_MISSIONS.contains(&m.read_u8(L::MISSION)) {
        x &= L::X_MASK;
    }
    let y = m.read_u16(rec + L::REC_Y);
    let (x, y) = anim::world_to_screen(m, x, y);
    m.write_u16(t + L::X, x);
    m.write_u16(t + L::Y, y);
    let attr = m.read_u16(rec + L::REC_ATTR);
    m.write_u16(t + L::ATTR, attr);
    let margin = m.read_u16(rec + L::REC_MARGIN);
    m.write_u16(t + L::MARGIN, margin);
    let f = m.read_u8(t + L::FLAGS) | m.read_u8(rec + L::REC_FLAGS);
    m.write_u8(t + L::FLAGS, f);
    if m.read_u8(rec + L::REC_NO_GFX) != 0 {
        return;
    }
    // Un registro con gráficos `NO_GFX` es un error de datos (el original
    // se para en `trap #15`); aquí simplemente no se dibuja.
    let gfx = m.read_u16(rec + L::REC_GFX);
    if gfx == L::NO_GFX {
        return;
    }
    anim::alloc_entry(m, t, gfx);
    let script = m.read_u32(rec + L::REC_ANIM);
    if script != L::NO_ANIM {
        anim::start(m, t, script);
    }
}

/// La tarea se mueve con la escena y la hija `c` se pone donde está ella.
/// Original: `Handler_ApplyCameraSelf_CopyTransform_05dd22`.
pub fn place_child(m: &mut impl Memory, t: u32, c: u32) {
    platforms::apply_camera(m, t);
    anim::copy_sprite_pos(m, t, c);
}

/// Si le han dado este frame, de qué lado: 1 = por la izquierda de su
/// caja de golpe (o no tiene caja), 2 = por la derecha.
/// Original: `Hitbox_SideOfImpact_02870a`.
pub fn hit_side(m: &mut impl Memory, t: u32) -> Option<u16> {
    if m.read_u8(t + L::FLAGS2) & L::HIT == 0 {
        return None;
    }
    let hb = m.read_u32(t + L::HITBOX);
    if hb == L::NO_HITBOX {
        return Some(1);
    }
    let x = m.read_u16(t + L::X);
    let left = m.read_u16(hb + L::HITBOX_LEFT).wrapping_add(x);
    let right = m.read_u16(hb + L::HITBOX_RIGHT).wrapping_add(x);
    let center = (left.wrapping_add(right) as i16) >> 1;
    let impact = m.read_u16(t + L::IMPACT_X);
    Some(if impact < center as u16 { 1 } else { 2 })
}

/// Sonido del golpe según la clase de ataque, de la tabla `table`.
/// Original: `$05E770` (con `Hit_SoundA_05e728`).
pub fn hit_sound(m: &mut impl Memory, t: u32, table: u32) {
    if m.read_u8(t + L::FLAGS2) & L::DEAD != 0 {
        return;
    }
    let class = u16::from(m.read_u8(t + L::ATTACK_CLASS)).wrapping_sub(1);
    if class >= L::HIT_CLASSES {
        return;
    }
    let code = m.read_u16(table + u32::from(class) * 2);
    if m.read_u8(t + L::FLAGS3) & L::HIT_SOUND_ON == 0 || code == L::NO_SOUND {
        return;
    }
    sound::play(m, code);
}

/// ¿Ha salido por la izquierda más allá de su margen?
/// Original: `Prop_OffscreenLeftCheck_04fa70`.
pub fn off_left(m: &mut impl Memory, t: u32) -> bool {
    let margin = m.read_u16(t + L::MARGIN).wrapping_neg() as i16;
    margin > m.read_u16(t + L::X) as i16
}

/// Crea las piezas del fuerte de la variante `v`.
/// Original: `Fort_SpawnChildren_V0_0884a6` … `Fort_SpawnChildren_V4_08891a`.
pub fn spawn_pieces(m: &mut impl Machine, t: u32, v: usize) {
    for (i, &(routine, dx, dy, attr, blit, watch)) in L::PIECES[v].iter().enumerate() {
        let c = tasks::spawn(m, routine, t);
        place_child(m, t, c);
        let x = m.read_u16(c + L::X).wrapping_add(dx);
        m.write_u16(c + L::X, x);
        let y = m.read_u16(c + L::Y).wrapping_add(dy);
        m.write_u16(c + L::Y, y);
        if let Some(a) = attr {
            m.write_u16(c + L::ATTR, a);
        }
        m.write_u32(c + L::BLIT, blit);
        m.write_u16(c + L::INDEX, i as u16);
        if let Some(w) = watch {
            m.write_u32(c + L::WATCH, w);
        }
    }
}

/// Alta del fuerte de la variante `v` (0–4).
/// Original: `Fort_Init_V0_0865dc` … `Fort_Init_V4_0866f0` y
/// `Fort_Init_Common_086738`.
pub fn start(m: &mut impl Machine, v: usize) -> Flow {
    let t = m.task_register();
    let rec = m.read_u32(t + L::REC_PTR);
    place_from_record(m, t, rec);
    m.write_u8(t + L::VARIANT, v as u8);
    spawn_pieces(m, t, v);
    m.write_u32(t + L::WRECK, L::WRECKS[v]);
    m.write_u32(t + L::WATCH, L::WATCHES[v]);
    let hatch = tasks::spawn(m, L::HATCHES[v], t);
    place_child(m, t, hatch);
    let x = m.read_u16(hatch + L::X).wrapping_add(L::HATCH_DX);
    m.write_u16(hatch + L::X, x);
    if L::HATCH_LOW[v] {
        let y = m.read_u16(hatch + L::Y).wrapping_add(L::HATCH_DY);
        m.write_u16(hatch + L::Y, y);
    }
    m.write_u16(t + L::MARGIN, L::FORT_MARGIN);
    m.write_u16(t + L::HP, L::FORT_HP);
    m.write_u8(t + L::STATE, 0);
    m.write_u32(t + L::HITBOX, L::FORT_HITBOX);
    m.write_u32(t + T::HANDLER, L::RUN);
    run(m)
}

/// Bucle del fuerte. Original: `$08675A`.
pub fn run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if m.read_u16(t + L::X) as i16 <= L::NO_HIT_X {
        m.write_u32(t + L::HITBOX, L::NO_HITBOX);
    }
    platforms::move_with_scene(m, t);
    if hit_side(m, t).is_some() {
        hit_sound(m, t, L::HIT_SOUNDS);
        let f = m.read_u8(t + L::FLAGS2) & !L::HIT;
        m.write_u8(t + L::FLAGS2, f);
    }
    if m.read_u16(t + L::X) as i16 >= L::FULL_HP_X {
        m.write_u16(t + L::HP, L::FORT_HP);
    }
    if m.read_u16(t + L::HP) as i16 <= 0 {
        m.write_u32(t + L::HITBOX, L::NO_HITBOX);
        m.write_u8(t + L::STATE, L::STATE_GONE);
        m.write_u32(t + T::HANDLER, L::DESTROYED);
    }
    if off_left(m, t) {
        m.write_u8(t + L::STATE, L::STATE_GONE);
        return tasks::finish(m);
    }
    Flow::Return
}

#[cfg(test)]
mod tests {
    use crate::layout::fort as L;

    #[test]
    fn pieces_have_sequential_indices_and_known_routines() {
        for (v, ps) in L::PIECES.iter().enumerate() {
            assert!(ps.len() >= 5);
            for &(r, ..) in ps.iter() {
                assert!([L::PILLBOX, L::PILLBOX_LOWER, L::PILLBOX_RIGHT, L::PILLBOX_RIGHT_UPPER, L::CRATE[v]].contains(&r));
            }
        }
    }
}
