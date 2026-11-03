//! Figurantes de las escenas (plantillas 195–222): gente de pie que anda
//! por el suelo y cae si no pisa, la que aparece con una animación y luego
//! se queda de pie, el sentado que da un saltito si le tocan, los que corren
//! hacia la izquierda y se paran, y los que sólo animan. Todos siguen a la
//! escena o andan por el suelo y se dan de baja al salir de la pantalla.
//!
//! Original: `asm/frontend/hiscore_memcard_options_09773c.s`
//! (`Mob_Tmpl195_Init_098f40` … `Mob_PhysLoop_09976a`).
//!
//! Las altas tienen varias entradas seguidas, una por variante, que sólo
//! cambian el gráfico (y en la plantilla 222 también la animación).

use super::{anim, collision, fort, grunt, human_death, items, manual_icons, platforms, prisoner, tasks};
use crate::layout::{mob as L, platforms as P, task as T};
use crate::mem::{Flow, Machine, Memory};

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

/// Sigue a la escena (`$02783A`), que deja `a5` en la zona de trabajo.
fn scene(m: &mut impl Machine, t: u32) {
    platforms::move_with_scene(m, t);
    let mut r = m.regs();
    r.a[5] = P::SCRATCH_BASE;
    m.set_regs(r);
}

/// Cola común de los bucles: si ha salido de la pantalla, el frame
/// siguiente se da de baja (`$099794`).
fn tail(m: &mut impl Machine, t: u32) -> Flow {
    let mut r = m.regs();
    r.a[0] = L::MARGINS;
    m.set_regs(r);
    if items::out_of_view_seen(m, t, Some(L::MARGINS)) {
        set_handler(m, t, L::GONE);
    }
    Flow::Return
}

/// Arranca la animación de la tabla `table` según la variante (`+$70`),
/// salvo que la entrada sea `$FFFFFFFF`.
fn variant_anim(m: &mut impl Machine, t: u32, table: u32) {
    let v = u32::from(m.read_u16(t + L::VARIANT));
    let a = m.read_u32(table.wrapping_add(v << 2));
    let mut r = m.regs();
    r.a[0] = a;
    m.set_regs(r);
    if a != L::NO_ANIM {
        anim::start(m, t, a);
    }
}

/// Atributo de sprite, prioridad 4 y fin del alta (`+$12` bit 1).
fn finish_setup(m: &mut impl Memory, t: u32, attr: u16) {
    anim::set_attr(m, t, attr);
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, (a & L::ATTR_KEEP) | L::PRIORITY);
    let f = m.read_u8(t + L::FLAGS_12);
    m.write_u8(t + L::FLAGS_12, f & !L::SPAWNING);
}

/// Alta habitual: gráfico, parado, al suelo y orientado según el parámetro.
fn ground_setup(m: &mut impl Machine, t: u32, gfx: u16) {
    human_death::alloc_regs(m, t, gfx);
    manual_icons::stop_motion(m, t);
    collision::snap_to_ground(m, t);
    grunt::facing_from_param(m, t);
}

// ---------------------------------------------------------------------------
// De pie / aparece (plantillas 195, 196, 201 y 202).
// ---------------------------------------------------------------------------

/// Plantillas 195 (`variant` 1) y 196 (0): de pie desde el principio.
/// Original: `Mob_Tmpl195_Init_098f40` / `Mob_Tmpl196_Init_098fa2`.
pub fn stand_init(m: &mut impl Machine, k: usize, variant: u16) -> Flow {
    let t = m.task_register();
    init_with(m, t, k, variant);
    set_handler(m, t, L::STAND);
    Flow::Return
}

/// Plantillas 201 (`variant` 1) y 202 (0): aparecen con una animación.
/// Original: `Mob_Tmpl201_Init_0990bc` / `Mob_Tmpl202_Init_09911e`.
pub fn appear_init(m: &mut impl Machine, k: usize, variant: u16) -> Flow {
    let t = m.task_register();
    init_with(m, t, k, variant);
    set_handler(m, t, L::APPEAR);
    Flow::Return
}

fn init_with(m: &mut impl Machine, t: u32, k: usize, variant: u16) {
    human_death::alloc_regs(m, t, L::GFX[k]);
    manual_icons::stop_motion(m, t);
    collision::snap_to_ground(m, t);
    grunt::facing_from_param(m, t);
    m.write_u16(t + L::VARIANT, variant);
    finish_setup(m, t, L::ATTR_BACK);
}

/// Anda por el suelo; si deja de pisar, cae. Original: `Mob_Stand_099004`.
pub fn stand(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    grunt::vel_x_from_param(m, t);
    m.write_u16(t + L::HP, 1);
    variant_anim(m, t, L::STAND_ANIMS);
    set_handler(m, t, L::STAND_LOOP);
    stand_loop(m)
}

/// Original: `Mob_Stand_099004__L099036`.
pub fn stand_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    prisoner::move_ground(m, t);
    if prisoner::probe(m, t) {
        set_handler(m, t, L::FALL);
    }
    anim::step(m, t);
    tail(m, t)
}

/// Cae hasta pisar y vuelve a estar de pie. Original: `Mob_Flee2_099070`.
pub fn fall(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    manual_icons::stop_motion(m, t);
    m.write_u16(t + L::VEL_Y, L::FALL_VEL);
    set_handler(m, t, L::FALL_LOOP);
    fall_loop(m)
}

/// Original: `Mob_Flee2_099070__L099082`.
pub fn fall_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    collision::fall(m, t);
    if !prisoner::probe(m, t) {
        set_handler(m, t, L::STAND);
    }
    anim::step(m, t);
    tail(m, t)
}

/// Animación de entrada; al acabar se queda de pie. Original: `Mob_Appear_099180`.
pub fn appear(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    variant_anim(m, t, L::APPEAR_ANIMS);
    set_handler(m, t, L::APPEAR_LOOP);
    appear_loop(m)
}

/// Original: `Mob_Appear_099180__L0991a6`.
pub fn appear_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scene(m, t);
    if anim::step(m, t) {
        set_handler(m, t, L::STAND);
    }
    tail(m, t)
}

// ---------------------------------------------------------------------------
// El sentado (plantilla 207).
// ---------------------------------------------------------------------------

/// Original: `Mob_Tmpl207_Init_0991da`.
pub fn sit_init(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    human_death::alloc_regs(m, t, L::GFX_ONE);
    collision::snap_to_ground(m, t);
    manual_icons::stop_motion(m, t);
    grunt::facing_from_param(m, t);
    finish_setup(m, t, L::ATTR_FRONT);
    set_handler(m, t, L::SIT);
    Flow::Return
}

/// Sentado; pasado un rato, si le golpean da un saltito. Los golpes se
/// descartan cada frame. Original: `Mob_Sit_09921a`.
pub fn sit(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::TIMER, L::SIT_WAIT);
    m.write_u16(t + L::HP, 1);
    anim::start(m, t, L::A_SIT);
    set_handler(m, t, L::SIT_LOOP);
    sit_loop(m)
}

/// Original: `Mob_Sit_09921a__L099238`.
pub fn sit_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scene(m, t);
    anim::step(m, t);
    let n = m.read_u16(t + L::TIMER);
    let waiting = n != 0 && {
        m.write_u16(t + L::TIMER, n - 1);
        n - 1 != 0
    };
    if !waiting {
        // `$02870A` deja en `d0` el lado del impacto.
        let side = fort::hit_side(m, t);
        let mut r = m.regs();
        r.d[0] = (r.d[0] & 0xFFFF_0000) | u32::from(side.unwrap_or(0));
        m.set_regs(r);
        if side.is_some() {
            set_handler(m, t, L::SIT_HOP);
        }
    }
    let f = m.read_u8(t + L::FLAGS_13);
    m.write_u8(t + L::FLAGS_13, f & !L::HIT);
    tail(m, t)
}

/// Original: `Mob_Sit_Hop_099288`.
pub fn sit_hop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::TIMER, L::HOP_FRAMES);
    anim::start(m, t, L::A_SIT_HOP);
    set_handler(m, t, L::SIT_HOP_LOOP);
    sit_hop_loop(m)
}

/// Original: `Mob_Sit_Hop_099288__L0992a0`.
pub fn sit_hop_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scene(m, t);
    anim::step(m, t);
    let n = m.read_u16(t + L::TIMER).wrapping_sub(1);
    m.write_u16(t + L::TIMER, n);
    if n == 0 {
        set_handler(m, t, L::SIT);
    }
    tail(m, t)
}

// ---------------------------------------------------------------------------
// Los que sólo animan (plantillas 208 y 222).
// ---------------------------------------------------------------------------

/// Original: `Mob_Tmpl208_Init_0992d8`.
pub fn idle_init(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    human_death::alloc_regs(m, t, L::GFX[k]);
    manual_icons::stop_motion(m, t);
    collision::snap_to_ground(m, t);
    grunt::facing_from_param(m, t);
    anim::set_attr(m, t, L::ATTR_BACK);
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, (a & L::ATTR_KEEP) | L::PRIORITY);
    m.write_u16(t + L::HP, 1);
    anim::start(m, t, L::A_LOOP208);
    let f = m.read_u8(t + L::FLAGS_12);
    m.write_u8(t + L::FLAGS_12, f & !L::SPAWNING);
    set_handler(m, t, L::LOOP);
    Flow::Return
}

/// Plantilla 222: cinco variantes de gráfico y animación.
/// Original: `Mob_Tmpl222_Init_0996b6`.
pub fn loop_init(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    human_death::alloc_regs(m, t, L::GFX_LOOP[k]);
    anim::start(m, t, L::A_LOOP222[k]);
    manual_icons::stop_motion(m, t);
    collision::snap_to_ground(m, t);
    grunt::facing_from_param(m, t);
    finish_setup(m, t, L::ATTR_BACK);
    set_handler(m, t, L::LOOP);
    Flow::Return
}

/// Sigue a la escena y anima. Original: `Mob_PhysLoop_09976a`.
pub fn idle_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scene(m, t);
    anim::step(m, t);
    tail(m, t)
}

// ---------------------------------------------------------------------------
// Los que andan (plantillas 209, 210 y 211).
// ---------------------------------------------------------------------------

/// Plantilla 209. Original: `Mob_Tmpl209_Init_099346`.
pub fn walker_init(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    human_death::alloc_regs(m, t, L::GFX[k]);
    walker_reset(m)
}

/// Vuelve a posarse en el suelo y a andar. Original: `Mob_Tmpl209_Reset_09936c`.
pub fn walker_reset(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::A_WALKER);
    anim::set_attr(m, t, L::ATTR_BACK);
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, (a & L::ATTR_KEEP) | L::PRIORITY);
    collision::snap_to_ground(m, t);
    let f = m.read_u8(t + L::FLAGS_12);
    m.write_u8(t + L::FLAGS_12, f & !L::SPAWNING);
    set_handler(m, t, L::WALKER);
    Flow::Return
}

/// Plantillas 210 y 211: como la 209 con otra animación.
/// Original: `Mob_Tmpl210_Init_099446` / `Mob_Tmpl211_Init_0994a2`.
pub fn walker_init_with(m: &mut impl Machine, k: usize, script: u32) -> Flow {
    let t = m.task_register();
    human_death::alloc_regs(m, t, L::GFX[k]);
    collision::snap_to_ground(m, t);
    anim::set_attr(m, t, L::ATTR_BACK);
    let a = m.read_u16(t + L::ATTR);
    m.write_u16(t + L::ATTR, (a & L::ATTR_KEEP) | L::PRIORITY);
    anim::start(m, t, script);
    let f = m.read_u8(t + L::FLAGS_12);
    m.write_u8(t + L::FLAGS_12, f & !L::SPAWNING);
    set_handler(m, t, L::WALKER);
    Flow::Return
}

/// Plantilla 210. Original: `Mob_Tmpl210_Init_099446`.
pub fn walker210_init(m: &mut impl Machine, k: usize) -> Flow {
    walker_init_with(m, k, L::A_WALKER210)
}

/// Plantilla 211. Original: `Mob_Tmpl211_Init_0994a2`.
pub fn walker211_init(m: &mut impl Machine, k: usize) -> Flow {
    walker_init_with(m, k, L::A_WALKER211)
}

/// Anda por el suelo; si deja de pisar, se vuelve a posar.
/// Original: `Mob_Tmpl209_Body_0993a2`.
pub fn walker(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    manual_icons::stop_motion(m, t);
    grunt::facing_from_param(m, t);
    grunt::vel_x_from_param(m, t);
    m.write_u16(t + L::HP, 1);
    set_handler(m, t, L::WALKER_LOOP);
    walker_loop(m)
}

/// Original: `Mob_Tmpl209_Body_0993a2__L0993c0`.
pub fn walker_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    prisoner::move_ground(m, t);
    if prisoner::probe(m, t) {
        set_handler(m, t, L::WALKER_RESET);
    }
    anim::step(m, t);
    tail(m, t)
}

/// Cae hasta pisar. Original: `Mob_Flee3_0993fa`.
pub fn walker_fall(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    manual_icons::stop_motion(m, t);
    m.write_u16(t + L::VEL_Y, L::FALL_VEL);
    set_handler(m, t, L::WALKER_FALL_LOOP);
    walker_fall_loop(m)
}

/// Original: `Mob_Flee3_0993fa__L09940c`.
pub fn walker_fall_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    collision::fall(m, t);
    if !prisoner::probe(m, t) {
        set_handler(m, t, L::WALKER_RESET);
    }
    anim::step(m, t);
    tail(m, t)
}

// ---------------------------------------------------------------------------
// Los que corren a la izquierda (plantillas 220 y 221).
// ---------------------------------------------------------------------------

/// Original: `Mob_Tmpl220_Init_0994fe`.
pub fn runner_init(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    ground_setup(m, t, L::GFX_ONE);
    finish_setup(m, t, L::ATTR_BACK);
    set_handler(m, t, L::RUN);
    Flow::Return
}

/// Corre hasta la X `RUN_STOP_X` y se para. Original: `Mob_RunLeft_09953e`.
pub fn run(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::HP, 1);
    anim::start(m, t, L::A_RUN);
    set_handler(m, t, L::RUN_LOOP);
    run_loop(m)
}

fn reached_stop(m: &mut impl Memory, t: u32) -> bool {
    m.read_u16(t + L::X) as i16 <= L::RUN_STOP_X
}

/// Original: `Mob_RunLeft_09953e__L099556`.
pub fn run_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scene(m, t);
    anim::step(m, t);
    if reached_stop(m, t) {
        set_handler(m, t, L::RUN_STOP);
    }
    tail(m, t)
}

/// Original: `Mob_RunLeft_Stop_099590`.
pub fn run_stop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::A_RUN_STOP);
    set_handler(m, t, L::RUN_STOP_LOOP);
    idle_loop(m)
}

/// Original: `Mob_Tmpl221_Init_0995cc`.
pub fn runner2_init(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    ground_setup(m, t, L::GFX_RUNNER2);
    m.write_u8(t + L::STOPPED, 0);
    finish_setup(m, t, L::ATTR_BACK);
    set_handler(m, t, L::RUN2);
    Flow::Return
}

/// Corre; la primera vez que llega a `RUN_STOP_X` se para un momento.
/// Original: `Mob_RunLeft2_099610`.
pub fn run2(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    m.write_u16(t + L::HP, 1);
    anim::start(m, t, L::A_RUN2);
    set_handler(m, t, L::RUN2_LOOP);
    run2_loop(m)
}

/// Original: `Mob_RunLeft2_099610__L099628`.
pub fn run2_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scene(m, t);
    anim::step(m, t);
    if m.read_u8(t + L::STOPPED) == 0 && reached_stop(m, t) {
        m.write_u8(t + L::STOPPED, 0xFF);
        set_handler(m, t, L::RUN2_WAIT);
    }
    tail(m, t)
}

/// Original: `Mob_RunLeft2_Wait_099670`.
pub fn run2_wait(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::A_RUN2_WAIT);
    set_handler(m, t, L::RUN2_WAIT_LOOP);
    run2_wait_loop(m)
}

/// Original: `Mob_RunLeft2_Wait_099670__L099682`.
pub fn run2_wait_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scene(m, t);
    if anim::step(m, t) {
        set_handler(m, t, L::RUN2);
    }
    tail(m, t)
}

/// Se da de baja (`$099794`, salto al planificador).
pub fn gone(m: &mut impl Machine) -> Flow {
    tasks::finish(m)
}
