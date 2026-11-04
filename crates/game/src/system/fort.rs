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

use super::{anim, events, layer, manual_icons, motion, platforms, score, scroll, sound, spawner, tasks};
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

/// ¿Tiene aún hijas? (la tarea siguiente tiene más prioridad).
/// Original: `Entity_CmpPrioWithSibling_086552`.
pub fn has_children(m: &mut impl Memory, t: u32) -> bool {
    let next = m.read_u32(t + T::NEXT);
    m.read_u8(t + T::PRIORITY) < m.read_u8(next + T::PRIORITY)
}

/// ¿Es la trampilla de un fuerte que ya ha caído?
fn fort_fallen(m: &mut impl Memory, t: u32) -> bool {
    if m.read_u8(t + L::STATE) != L::IN_FORT {
        return false;
    }
    let owner = m.read_u32(t + T::OWNER);
    m.read_u8(owner + L::STATE) == L::STATE_GONE
}

/// Final común de los estados de la trampilla: baja si sale por la
/// izquierda.
fn hatch_tail(m: &mut impl Machine, t: u32) -> Flow {
    if off_left(m, t) {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Arranca la animación de la tabla `table` según la variante.
fn hatch_anim(m: &mut impl Memory, t: u32, table: u32) {
    let v = u32::from(m.read_u8(t + L::VARIANT));
    let script = m.read_u32(table + v * 4);
    if script != L::NO_ANIM {
        anim::start(m, t, script);
    }
}

/// Alta de la trampilla de un fuerte (variantes 0–4).
/// Original: `Hatch_Init_086e4a` … `Hatch_Init_V4_086ef2`.
pub fn hatch_start(m: &mut impl Machine, v: usize) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::MARGIN, L::HATCH_MARGIN[v]);
    m.write_u8(t + L::VARIANT, v as u8);
    m.write_u8(t + L::STATE, L::IN_FORT);
    m.write_u32(t + L::WATCH, L::HATCH_WATCHES[v]);
    m.write_u32(t + L::FINAL_WATCH, L::HATCH_FINAL_WATCHES[v]);
    hatch_wait_closed(m)
}

/// Alta de una trampilla suelta (variantes 5 y 6), colocada desde su
/// registro de aparición. Original: `$086F1C`, `$086F40`.
pub fn hatch_start_placed(m: &mut impl Machine, v: usize) -> Flow {
    let t = m.task_register();
    let rec = m.read_u32(t + L::REC_PTR);
    place_from_record(m, t, rec);
    m.write_u16(t + L::MARGIN, L::HATCH_MARGIN[v]);
    m.write_u8(t + L::VARIANT, v as u8);
    m.write_u32(t + L::WATCH, L::HATCH_WATCHES[v]);
    hatch_wait_closed(m)
}

/// Cerrada un rato. Original: `Hatch_WaitClosed_086f64`.
pub fn hatch_wait_closed(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::TIMER, L::CLOSED_FRAMES);
    m.write_u32(t + T::HANDLER, L::WAIT_CLOSED_RUN);
    hatch_wait_closed_run(m)
}

/// Original: `$086F70`.
pub fn hatch_wait_closed_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    let n = m.read_u16(t + L::TIMER).wrapping_sub(1);
    m.write_u16(t + L::TIMER, n);
    if n == 0 {
        m.write_u32(t + T::HANDLER, L::OPENING);
    }
    if fort_fallen(m, t) {
        m.write_u32(t + T::HANDLER, L::FINAL_OPEN);
    }
    hatch_tail(m, t)
}

/// Se abre. Original: `Hatch_Opening_086fb4`.
pub fn hatch_opening(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    hatch_anim(m, t, L::OPEN_ANIMS);
    m.write_u32(t + T::HANDLER, L::OPENING_RUN);
    hatch_opening_run(m)
}

/// Original: `$086FDC`.
pub fn hatch_opening_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if anim::step(m, t) {
        m.write_u32(t + T::HANDLER, L::SPAWN_TROOPS);
    }
    hatch_tail(m, t)
}

/// Saca la oleada de soldados. Original: `Hatch_SpawnTroops_087004`.
pub fn hatch_spawn_troops(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::TIMER, L::TROOPS_FRAMES);
    let list = m.read_u32(t + L::WATCH);
    m.write_u8(t + L::WATCH_FLAG_BYTE, 0);
    events::spawn_watch(m, list, L::WATCH_FLAG);
    m.write_u32(t + T::HANDLER, L::SPAWN_TROOPS_RUN);
    hatch_spawn_troops_run(m)
}

/// Espera a que la oleada acabe. Original: `$087024`.
pub fn hatch_spawn_troops_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if !has_children(m, t) {
        m.write_u32(t + T::HANDLER, L::WAIT_OPEN);
    }
    if fort_fallen(m, t) {
        m.write_u32(t + T::HANDLER, L::FINAL_SPAWN);
    }
    hatch_tail(m, t)
}

/// Abierta un rato. Original: `Hatch_WaitOpen_087068`.
pub fn hatch_wait_open(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::TIMER, L::OPEN_FRAMES);
    m.write_u32(t + T::HANDLER, L::WAIT_OPEN_RUN);
    hatch_wait_open_run(m)
}

/// Original: `$087074`.
pub fn hatch_wait_open_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    let n = m.read_u16(t + L::TIMER).wrapping_sub(1);
    m.write_u16(t + L::TIMER, n);
    if n == 0 {
        m.write_u32(t + T::HANDLER, L::CLOSING);
    }
    if fort_fallen(m, t) {
        m.write_u32(t + T::HANDLER, L::FINAL_SPAWN);
    }
    hatch_tail(m, t)
}

/// Se cierra. Original: `Hatch_Closing_0870b8`.
pub fn hatch_closing(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    hatch_anim(m, t, L::CLOSE_ANIMS);
    m.write_u32(t + T::HANDLER, L::CLOSING_RUN);
    hatch_closing_run(m)
}

/// Original: `$0870E0`.
pub fn hatch_closing_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if anim::step(m, t) {
        m.write_u32(t + T::HANDLER, L::WAIT_CLOSED);
    }
    hatch_tail(m, t)
}

/// Al caer el fuerte se abre del todo. Original: `Hatch_FinalOpen_087108`.
pub fn hatch_final_open(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    hatch_anim(m, t, L::OPEN_ANIMS);
    m.write_u32(t + T::HANDLER, L::FINAL_SPAWN);
    hatch_final_spawn(m)
}

/// Abierta del todo, saca la última oleada y se da de baja.
/// Original: `Hatch_FinalSpawn_087130`.
pub fn hatch_final_spawn(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if anim::step(m, t) {
        let list = m.read_u32(t + L::FINAL_WATCH);
        m.write_u8(t + L::WATCH_FLAG_BYTE, 0);
        events::spawn_watch(m, list, L::WATCH_FLAG);
        return tasks::finish(m);
    }
    hatch_tail(m, t)
}

/// ¿Está muerta? Original: `Hitbox_CheckBit0_028758`.
pub fn dead(m: &mut impl Memory, t: u32) -> bool {
    m.read_u8(t + L::FLAGS2) & L::DEAD != 0
}

/// La caja de golpe sólo está activa uno de cada cuatro frames: la de la
/// tabla `PULSE_HITBOXES` según `STATE` (`$088438`) o la guardada en
/// `PULSE_HITBOX` (`$08846A`).
pub fn pulse_hitbox(m: &mut impl Memory, t: u32, from_table: bool) {
    let n = m.read_u16(t + L::TIMER).wrapping_add(1);
    m.write_u16(t + L::TIMER, n);
    let hb = if n & L::PULSE_MASK != 0 {
        L::NO_HITBOX
    } else if from_table {
        let k = u32::from(m.read_u8(t + L::STATE));
        m.read_u32(L::PULSE_HITBOXES + k * 4)
    } else {
        m.read_u32(t + L::PULSE_HITBOX)
    };
    m.write_u32(t + L::HITBOX, hb);
}

/// Resta al padre la vida que esta pieza ha perdido desde la última vez.
/// Original: `Entity_PropagateDamageToParent_08848c`.
pub fn pass_damage_to_parent(m: &mut impl Memory, t: u32) {
    let hp = m.read_u16(t + L::HP);
    let lost = m.read_u16(t + L::LAST_HP).wrapping_sub(hp);
    let owner = m.read_u32(t + T::OWNER);
    let ohp = m.read_u16(owner + L::HP).wrapping_sub(lost);
    m.write_u16(owner + L::HP, ohp);
    m.write_u16(t + L::LAST_HP, hp);
}

/// ¿Ha caído ya el fuerte (el padre)?
fn parent_gone(m: &mut impl Memory, t: u32) -> bool {
    let owner = m.read_u32(t + T::OWNER);
    m.read_u8(owner + L::STATE) == L::STATE_GONE
}

/// Alta de una caja del fuerte (variante `v`).
/// Original: `Crate_Init_086cd0` … `Crate_Init_V4_086d20`.
pub fn crate_start(m: &mut impl Machine, v: usize) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::CRATE_GFX[v]);
    m.write_u8(t + L::STATE, v as u8);
    m.write_u8(t + L::SHRINK_Y, L::FULL_SIZE);
    m.write_u8(t + L::SHRINK_X, L::FULL_SIZE);
    m.write_u8(t + L::FLIP, 0);
    m.write_u16(t + L::HP, L::CRATE_HP);
    m.write_u16(t + L::ATTR, 0);
    m.write_u32(t + T::HANDLER, L::CRATE_RUN);
    crate_run(m)
}

/// Bucle de la caja: se mueve con la escena, pasa el daño al fuerte y,
/// al morir, revienta. Original: `$086D58`.
pub fn crate_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if parent_gone(m, t) {
        return tasks::finish(m);
    }
    platforms::move_with_scene(m, t);
    if hit_side(m, t).is_some() {
        hit_sound(m, t, L::HIT_SOUNDS);
        let f = m.read_u8(t + L::FLAGS2) & !L::HIT;
        m.write_u8(t + L::FLAGS2, f);
        pass_damage_to_parent(m, t);
    }
    if dead(m, t) {
        m.write_u32(t + L::HITBOX, L::NO_HITBOX);
        m.write_u32(t + T::HANDLER, L::CRATE_BURST);
    }
    pulse_hitbox(m, t, true);
    Flow::Return
}

/// La caja revienta: puntos para quien la rompió, sonido y guion de
/// reventar según la variante. Original: `Crate_Burst_086db4`.
pub fn crate_burst(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    score::award_attacker(m, t, L::CRATE_POINTS);
    sound::play(m, L::CRATE_SND_BURST);
    let v = u32::from(m.read_u8(t + L::STATE));
    let s = m.read_u32(L::CRATE_BURST_SCRIPTS + v * 4);
    if s != L::NO_ANIM {
        anim::start(m, t, s);
    }
    m.write_u32(t + T::HANDLER, L::CRATE_BURST_RUN);
    crate_burst_run(m)
}

/// Al acabar el guion: sonido, explosión, la ruina pintada en la capa y
/// humo; luego (o si el fuerte ha caído) se da de baja.
/// Original: `Crate_Burst_086db4__L086df2`.
pub fn crate_burst_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if parent_gone(m, t) {
        return tasks::finish(m);
    }
    platforms::move_with_scene(m, t);
    if !anim::step(m, t) {
        return Flow::Return;
    }
    sound::play(m, L::PILLBOX_SND_DOWN);
    let c = tasks::spawn(m, L::CRATE_BOOM, t);
    place_child(m, t, c);
    let cmd = m.read_u32(t + L::BLIT);
    layer::blit(m, cmd);
    spawner::install(m, t, L::PILLBOX_SMOKE);
    tasks::finish(m)
}

/// Golpe recibido: deja en d0 el lado del impacto, suena y limpia la
/// marca. Devuelve si le han dado.
pub(super) fn take_hit(m: &mut impl Machine, t: u32, sound_table: Option<u32>) -> bool {
    let side = hit_side(m, t);
    let mut r = m.regs();
    r.d[0] = (r.d[0] & 0xFFFF_0000) | u32::from(side.unwrap_or(0));
    m.set_regs(r);
    if side.is_none() {
        return false;
    }
    if let Some(table) = sound_table {
        let mut r = m.regs();
        r.a[0] = table;
        m.set_regs(r);
        hit_sound(m, t, table);
    }
    true
}

pub(super) fn clear_hit(m: &mut impl Memory, t: u32) {
    let f = m.read_u8(t + L::FLAGS2);
    m.write_u8(t + L::FLAGS2, f & !L::HIT);
}

/// Objeto rompible del fuerte `v` (A, B, C): aguanta golpes y, al morir,
/// echa humo, dibuja su destrozo y se da de baja.
/// Original: `Prop_Breakable_A_08716a` / `_B_0871fa` / `_C_087280`.
pub fn prop(m: &mut impl Machine, v: usize) -> Flow {
    let t = m.task_register();
    let rec = m.read_u32(t + L::REC_PTR);
    place_from_record(m, t, rec);
    m.write_u16(t + L::MARGIN, L::PROP_MARGIN);
    m.write_u32(t + L::HITBOX, L::PROP_HITBOX);
    m.write_u32(t + T::HANDLER, L::PROP_RUN[v]);
    prop_run(m, v)
}

pub fn prop_run(m: &mut impl Machine, v: usize) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if take_hit(m, t, Some(L::HIT_SOUNDS)) {
        clear_hit(m, t);
    }
    if dead(m, t) {
        if v == 0 {
            sound::play(m, L::PROP_SND_BREAK);
        }
        spawner::install(m, t, L::PROP_SMOKE);
        for &cmd in L::PROP_WRECK[v] {
            let mut r = m.regs();
            r.a[2] = cmd;
            m.set_regs(r);
            layer::blit(m, cmd);
        }
        return tasks::finish(m);
    }
    if off_left(m, t) {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Letrero: no se rompe, sólo se mueve al recibir un golpe.
/// Original: `Prop_Signboard_0872fa`.
pub fn sign(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let rec = m.read_u32(t + L::REC_PTR);
    place_from_record(m, t, rec);
    m.write_u16(t + L::MARGIN, L::PROP_MARGIN);
    anim::start(m, t, L::SIGN_SCRIPT);
    m.write_u32(t + T::HANDLER, L::SIGN_RUN);
    sign_run(m)
}

pub fn sign_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    anim::step(m, t);
    if take_hit(m, t, None) {
        sound::play(m, L::SIGN_SND_HIT);
        anim::start(m, t, L::SIGN_HIT_SCRIPT);
        clear_hit(m, t);
    }
    m.write_u16(t + L::HP, L::SIGN_HP);
    if off_left(m, t) {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Muro de 13 cajas: crea las piezas en sus sitios y se da de baja.
/// Original: `CrateWall_Spawn13_087366`.
pub fn wall(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let rec = m.read_u32(t + L::REC_PTR);
    place_from_record(m, t, rec);
    for &(routine, dx, dy) in L::WALL_PIECES.iter() {
        let c = tasks::spawn(m, routine, t);
        place_child(m, t, c);
        let x = m.read_u16(c + L::X).wrapping_add(dx);
        m.write_u16(c + L::X, x);
        let y = m.read_u16(c + L::Y).wrapping_add(dy);
        m.write_u16(c + L::Y, y);
    }
    tasks::finish(m)
}

/// Pieza `k` del muro: su ruina, la oleada que suelta, caja de golpe y
/// humo. Original: `CrateWall_Piece00_0874ba` … `CrateWall_Piece12_087700`
/// y `CrateWall_Piece_Common_087730`.
pub fn wall_piece(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    let (blit, list, hitbox, smoke) = L::WALL_DATA[k];
    m.write_u32(t + L::BLIT, blit);
    m.write_u32(t + L::WATCH, list);
    m.write_u16(t + L::INDEX, k as u16);
    m.write_u32(t + L::HITBOX, hitbox);
    m.write_u32(t + L::WALL_SMOKE, smoke);
    let mut r = m.regs();
    r.a[0] = hitbox;
    r.a[1] = L::WALL_RUN;
    m.set_regs(r);
    m.write_u16(t + L::MARGIN, L::WALL_MARGIN);
    m.write_u16(t + L::HP, L::WALL_HP);
    m.write_u32(t + L::PULSE_HITBOX, hitbox);
    m.write_u32(t + T::HANDLER, L::WALL_RUN);
    wall_piece_run(m)
}

/// Bucle de la pieza: suena con los golpes y, al romperse, pinta su
/// ruina, echa humo y, si la tiene, suelta su oleada.
/// Original: `CrateWall_Piece_Common_087730` (desde `$087748`).
pub fn wall_piece_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    platforms::move_with_scene(m, t);
    if take_hit(m, t, Some(L::HIT_SOUNDS)) {
        clear_hit(m, t);
    }
    if dead(m, t) {
        m.write_u32(t + L::HITBOX, L::NO_HITBOX);
        sound::play(m, L::WALL_SND_BREAK);
        let cmd = m.read_u32(t + L::BLIT);
        layer::blit(m, cmd);
        let smoke = m.read_u32(t + L::WALL_SMOKE);
        spawner::install(m, t, smoke);
        let list = m.read_u32(t + L::WATCH);
        if list != L::NO_LIST {
            m.write_u8(t + L::WATCH_FLAG_BYTE, 0);
            events::spawn_watch(m, list, L::WATCH_FLAG);
        }
        return tasks::finish(m);
    }
    if off_left(m, t) {
        return tasks::finish(m);
    }
    pulse_hitbox(m, t, false);
    Flow::Return
}

// ---------------------------------------------------------------------
// Casamatas del fuerte: cuatro estados de daño (cada uno con su guion),
// pasan el daño al fuerte y, al morir, o revientan del todo o (las de
// variante impar) se desmoronan con un trozo que cae antes de reventar.
// Original: `Pillbox_Init_08686a` … `Pillbox_Fragment_086c62`.
// ---------------------------------------------------------------------

/// Arranca el guion `k` (0 = intacta, 1 y 2 = dañada) de la casamata
/// según la fila de su estado (`STATE >> 4`).
fn pillbox_script(m: &mut impl Memory, t: u32, k: u32) {
    let row = u32::from(m.read_u8(t + L::STATE)) >> 4;
    let s = m.read_u32(L::PILLBOX_SCRIPTS + (row + k) * 4);
    if s != L::NO_ANIM {
        anim::start(m, t, s);
    }
}

/// Alta de una casamata (variante `v`, ver `PILLBOX_INITS`).
/// Original: `Pillbox_Init_08686a` (+ las variantes) y `.L0868E2`.
pub fn pillbox_start(m: &mut impl Machine, v: usize) -> Flow {
    let t = m.task_register();
    let (gfx, state) = L::PILLBOX_INITS[v];
    anim::alloc_entry(m, t, gfx);
    m.write_u8(t + L::STATE, state);
    m.write_u8(t + L::SHRINK_Y, L::FULL_SIZE);
    m.write_u8(t + L::SHRINK_X, L::FULL_SIZE);
    m.write_u8(t + L::FLIP, 0);
    m.write_u16(t + L::HP, L::PILLBOX_HP);
    m.write_u16(t + L::LAST_HP, L::PILLBOX_HP);
    m.write_u16(t + L::ATTR, L::PILLBOX_ATTR);
    manual_icons::stop_motion(m, t);
    let r = m.regs();
    let at = (r.d[3] as u16, r.d[4] as u16);
    motion::on_entity_pos(m, t, |m| motion::move_e_probe(m, t, at));
    pillbox_script(m, t, 0);
    let i = m.read_u16(t + L::INDEX) as u8;
    m.write_u8(t + L::SLOT, i);
    m.write_u32(t + T::HANDLER, L::PILLBOX_RUN);
    pillbox_run(m)
}

/// Se borra del mapa de colisión y se da de baja.
fn pillbox_gone(m: &mut impl Machine, t: u32) -> Flow {
    scroll::blit_cells(m, t, L::PILLBOX_CELLS);
    tasks::finish(m)
}

/// Frame común: si el fuerte ha caído se va; si no, se mueve con la
/// escena, anima y, si le dan, suena y pasa el daño al fuerte.
fn pillbox_frame(m: &mut impl Machine, t: u32) -> Option<Flow> {
    if parent_gone(m, t) {
        return Some(pillbox_gone(m, t));
    }
    platforms::move_with_scene(m, t);
    anim::step(m, t);
    if hit_side(m, t).is_some() {
        sound::play(m, L::PILLBOX_SND_HIT);
        let f = m.read_u8(t + L::FLAGS2) & !L::HIT;
        m.write_u8(t + L::FLAGS2, f);
        pass_damage_to_parent(m, t);
    }
    None
}

/// Bucle del estado `k`: con la vida por debajo de su umbral pasa al
/// siguiente.
fn pillbox_stage(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    if let Some(f) = pillbox_frame(m, t) {
        return f;
    }
    if m.read_u16(t + L::HP) as i16 <= L::PILLBOX_THRESHOLDS[k] {
        m.write_u32(t + T::HANDLER, L::PILLBOX_NEXT[k]);
    }
    Flow::Return
}

/// Intacta. Original: `$086944`.
pub fn pillbox_run(m: &mut impl Machine) -> Flow {
    pillbox_stage(m, 0)
}

/// Primer daño: su guion y su bucle. Original: `Pillbox_Damaged1_0869a4`.
pub fn pillbox_damaged1(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    pillbox_script(m, t, 1);
    m.write_u32(t + T::HANDLER, L::PILLBOX_DAMAGED1_RUN);
    pillbox_damaged1_run(m)
}

/// Original: `$0869D0`.
pub fn pillbox_damaged1_run(m: &mut impl Machine) -> Flow {
    pillbox_stage(m, 1)
}

/// Segundo daño. Original: `Pillbox_Damaged2_086a30`.
pub fn pillbox_damaged2(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    pillbox_script(m, t, 2);
    m.write_u32(t + T::HANDLER, L::PILLBOX_DAMAGED2_RUN);
    pillbox_damaged2_run(m)
}

/// Original: `$086A5C`.
pub fn pillbox_damaged2_run(m: &mut impl Machine) -> Flow {
    pillbox_stage(m, 2)
}

/// Destruida: puntos, sonido `snd`, explosión, ruina pintada en la capa,
/// humo, la oleada que tenga asignada y baja.
fn pillbox_destroyed(m: &mut impl Machine, t: u32, snd: u16) -> Flow {
    score::award_attacker(m, t, L::PILLBOX_POINTS);
    sound::play(m, snd);
    let c = tasks::spawn(m, L::PILLBOX_BOOM, t);
    place_child(m, t, c);
    lower(m, c, L::PILLBOX_BOOM_DY);
    let cmd = m.read_u32(t + L::BLIT);
    layer::blit(m, cmd);
    let s = spawner::install(m, t, L::PILLBOX_SMOKE);
    lower(m, s, L::PILLBOX_BOOM_DY);
    let w = m.read_u32(t + L::WATCH);
    if w != L::NO_WATCH {
        m.write_u8(t + L::WATCH_FLAG_BYTE, 0);
        events::spawn_watch(m, w, L::WATCH_FLAG);
    }
    pillbox_gone(m, t)
}

fn lower(m: &mut impl Memory, t: u32, dy: u16) {
    let y = m.read_u16(t + L::Y).wrapping_add(dy);
    m.write_u16(t + L::Y, y);
}

/// Crítica: al morir apaga su caja de golpe y revienta (o se desmorona,
/// las de estado impar). Original: `Pillbox_Critical_086abc`.
pub fn pillbox_critical(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u32(t + T::HANDLER, L::PILLBOX_CRITICAL_RUN);
    pillbox_critical_run(m)
}

/// Original: `$086AC2`.
pub fn pillbox_critical_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if let Some(f) = pillbox_frame(m, t) {
        return f;
    }
    if !dead(m, t) {
        return Flow::Return;
    }
    let f = m.read_u8(t + L::FLAGS2) & !L::DEAD;
    m.write_u8(t + L::FLAGS2, f);
    m.write_u32(t + L::HITBOX, L::NO_HITBOX);
    if m.read_u8(t + L::STATE) & L::PILLBOX_COLLAPSES != 0 {
        m.write_u32(t + T::HANDLER, L::PILLBOX_COLLAPSE);
        return Flow::Return;
    }
    pillbox_destroyed(m, t, L::PILLBOX_SND_DOWN)
}

/// Se desmorona: suelta el trozo que cae y espera a que éste avise
/// (`STATE` con los cuatro bits bajos a 1). Original: `Pillbox_Collapse_086ba4`.
pub fn pillbox_collapse(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let c = tasks::spawn(m, L::PILLBOX_FRAGMENT, t);
    place_child(m, t, c);
    m.write_u32(t + T::HANDLER, L::PILLBOX_COLLAPSE_RUN);
    pillbox_collapse_run(m)
}

/// Original: `$086BBA`.
pub fn pillbox_collapse_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    if parent_gone(m, t) {
        return pillbox_gone(m, t);
    }
    platforms::move_with_scene(m, t);
    anim::step(m, t);
    if m.read_u8(t + L::STATE) & L::PILLBOX_FALLEN != L::PILLBOX_FALLEN {
        return Flow::Return;
    }
    pillbox_destroyed(m, t, L::PILLBOX_SND_COLLAPSE)
}

/// Trozo que cae de la casamata. Original: `Pillbox_Fragment_086c62`.
pub fn pillbox_fragment(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::alloc_entry(m, t, L::FRAGMENT_GFX);
    m.write_u8(t + L::SHRINK_Y, L::FULL_SIZE);
    m.write_u8(t + L::SHRINK_X, L::FULL_SIZE);
    m.write_u8(t + L::FLIP, 0);
    m.write_u16(t + L::ATTR, 0);
    lower(m, t, L::FRAGMENT_DY);
    anim::start(m, t, L::FRAGMENT_SCRIPT);
    m.write_u32(t + T::HANDLER, L::PILLBOX_FRAGMENT_RUN);
    pillbox_fragment_run(m)
}

/// Cae con su guion; al acabar avisa a la casamata. Se va también si
/// cae el fuerte. Original: `$086C9C`.
pub fn pillbox_fragment_run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let owner = m.read_u32(t + T::OWNER);
    if parent_gone(m, owner) {
        return tasks::finish(m);
    }
    platforms::move_with_scene(m, t);
    if anim::step(m, t) {
        let s = m.read_u8(owner + L::STATE) | L::PILLBOX_FALLEN;
        m.write_u8(owner + L::STATE, s);
        return tasks::finish(m);
    }
    Flow::Return
}

/// Pinta en la capa la ruina del fuerte según su variante.
/// Original: `Fort_BlitWreck_088a28` (y `$088A64`…`$088B00`).
pub fn blit_wreck(m: &mut impl Memory, t: u32) {
    let v = usize::from(m.read_u8(t + L::VARIANT));
    if let Some(list) = L::WRECK_BLITS.get(v) {
        for &cmd in list.iter() {
            layer::blit(m, cmd);
        }
    }
}

/// Pinta el trozo de ruina `k` (ver `PIECE_BLITS`).
/// Original: las rutinas de un solo blit de `$088B3E` a `$088F4A`.
pub fn blit_piece(m: &mut impl Memory, k: usize) {
    layer::blit(m, L::PIECE_BLITS[k]);
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
