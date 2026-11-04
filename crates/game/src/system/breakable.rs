//! Objetos que se rompen (plantillas $1F, $22, $23 y $24).
//!
//! No arrancan hasta entrar por la derecha de la pantalla (X < $140).
//! Siguen a la escena, miran al jugador y reaccionan con una animación
//! cuando se acerca (o, la plantilla $1F, cuando se pone delante). Al
//! recibir un golpe dan puntos según la clase del ataque, hacen una
//! animación de rotura y sueltan trozos, que son a su vez objetos de la
//! plantilla $1F más pequeños (`+$99` cuenta cuántos quedan). Los guiones
//! pueden soltar además esquirlas que vuelan, hacen daño y estallan al
//! tocar el suelo.
//!
//! Original: `asm/props/late_props_turrets_05e000.s`
//! (`Breakable_Tmpl1F_05fd78` … `Breakable_PieceRng_0603c4`).

use super::{anim, banner, fort, hits, items, manual_icons, motion, pickup, platforms, rng, slug, soldier, tasks};
use crate::layout::{breakable as L, collision as C, platforms as P, task as T};
use crate::mem::{Flow, Machine, Memory};

fn set_handler(m: &mut impl Memory, t: u32, h: u32) {
    m.write_u32(t + T::HANDLER, h);
}

fn scene(m: &mut impl Machine, t: u32) {
    platforms::move_with_scene(m, t);
    let mut r = m.regs();
    r.a[5] = P::SCRATCH_BASE;
    m.set_regs(r);
}

/// Jugador más cercano en X (o el segundo, como el original, si no hay
/// ninguno), dejando a0 apuntándolo. Devuelve su X menos la propia.
fn player_dx(m: &mut impl Machine, t: u32) -> u16 {
    let p = pickup::nearest_player_x(m, t).unwrap_or(crate::layout::PLAYER_ENTITY[1]);
    let mut r = m.regs();
    r.a[0] = p;
    m.set_regs(r);
    m.read_u16(p + L::X).wrapping_sub(m.read_u16(t + L::X))
}

/// Cola común: si le han dado, se rompe; si ha salido de la vista, se da
/// de baja. Original: los finales de cada bucle con `Offworld_Box2C21BC_06037c`.
fn tail(m: &mut impl Machine, t: u32) -> Flow {
    let side = fort::hit_side(m, t);
    let mut r = m.regs();
    r.d[0] = (r.d[0] & 0xFFFF_0000) | u32::from(side.unwrap_or(0));
    r.a[0] = L::MARGINS;
    m.set_regs(r);
    if side.is_some() {
        set_handler(m, t, L::BREAK);
    }
    if items::out_of_view_seen(m, t, Some(L::MARGINS)) {
        return tasks::finish(m);
    }
    Flow::Return
}

/// Bucle que espera a que acabe el guion y vuelve a mirar al jugador.
fn script_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scene(m, t);
    if anim::step(m, t) {
        set_handler(m, t, L::IDLE);
    }
    tail(m, t)
}

// ---------------------------------------------------------------------------
// Altas.
// ---------------------------------------------------------------------------

/// Espera a entrar en pantalla; mientras, sólo sigue a la escena.
fn wait_entry(m: &mut impl Machine, t: u32) -> bool {
    if (m.read_u16(t + L::X) as i16) < L::START_X {
        return true;
    }
    scene(m, t);
    false
}

/// Alta común: gráfico, tamaño completo, orientación desde el parámetro,
/// atributo, vida 1 y parado.
fn setup(m: &mut impl Machine, t: u32) {
    anim::alloc_entry(m, t, L::GFX);
    for off in L::SHRINK {
        m.write_u8(t + off, L::FULL);
    }
    let f = m.read_u8(t + L::PARAM) & 1;
    m.write_u8(t + L::FACING, f);
    m.write_u16(t + L::ATTR, L::ATTR_INIT);
    m.write_u16(t + L::HP, 1);
    manual_icons::stop_motion(m, t);
}

/// Plantilla $1F (y los trozos): variante `v` del guion inicial.
/// Original: `Breakable_Tmpl1F_05fd78` (+$0, +$10, +$20) y `Breakable_Init_05fdb6`.
pub fn init(m: &mut impl Machine, v: usize) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::S_1F[v]);
    if !wait_entry(m, t) {
        return Flow::Return;
    }
    setup(m, t);
    set_handler(m, t, L::INIT_LOOP);
    init_loop(m)
}

pub fn init_loop(m: &mut impl Machine) -> Flow {
    script_loop(m)
}

/// Plantillas $22 (`k` = 0) y $23 (1): reaccionan cuando el jugador se
/// acerca. Original: `Breakable_Tmpl22_05ff64` / `Breakable_Tmpl23_060050`.
pub fn init_near(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    if !wait_entry(m, t) {
        return Flow::Return;
    }
    start_near(m, k)
}

/// Alta de las plantillas $22/$23 sin esperar a entrar en pantalla.
/// Original: `Breakable22_Init_05ff76` / `Breakable23_Init_060062`.
pub fn start_near(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    setup(m, t);
    if let Some(s) = L::S_NEAR[k] {
        anim::start(m, t, s);
    }
    set_handler(m, t, L::NEAR_LOOP[k]);
    near_loop(m, k)
}

/// Plantilla $24 (`k` = 2): variante `v` del guion inicial.
/// Original: `Breakable_Tmpl24_06013c` (+$0, +$10, +$20, +$30).
pub fn init_24(m: &mut impl Machine, v: usize) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::S_24[v]);
    if !wait_entry(m, t) {
        return Flow::Return;
    }
    setup(m, t);
    set_handler(m, t, L::NEAR_LOOP[2]);
    near_loop(m, 2)
}

/// Cuando el jugador está a 32 píxeles o menos, pasa a su reacción.
/// Original: `Breakable22_Init_05ff76+$46`, `Breakable23_Init_060062+$46`,
/// `Breakable24_Init_06018a+$3A`.
pub fn near_loop(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    scene(m, t);
    anim::step(m, t);
    let d = (player_dx(m, t) as i16).wrapping_abs();
    if d <= L::NEAR {
        set_handler(m, t, L::NEAR_NEXT[k]);
    }
    tail(m, t)
}

// ---------------------------------------------------------------------------
// Estados.
// ---------------------------------------------------------------------------

/// Mira al jugador; al acabar la cuenta se gira y, si el jugador queda
/// delante, reacciona. Original: `Breakable_Idle_05fe26`.
pub fn idle(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let n = soldier::by_difficulty(m, t, L::D_IDLE);
    m.write_u16(t + L::TIMER, n);
    anim::start(m, t, L::S_IDLE);
    set_handler(m, t, L::IDLE_LOOP);
    idle_loop(m)
}

pub fn idle_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scene(m, t);
    anim::step(m, t);
    if m.read_u16(t + L::TIMER) != L::NO_TIMER && countdown(m, t) {
        set_handler(m, t, L::TURN);
    }
    let behind = player_dx(m, t) >> 15;
    if u16::from(m.read_u8(t + L::FACING) & 1) == behind {
        set_handler(m, t, L::REACT[0]);
    }
    tail(m, t)
}

/// Resta dos a la cuenta (comprobando el cero tras cada resta).
fn countdown(m: &mut impl Memory, t: u32) -> bool {
    let a = m.read_u16(t + L::TIMER).wrapping_sub(1);
    m.write_u16(t + L::TIMER, a);
    if a == 0 {
        return true;
    }
    let b = a.wrapping_sub(1);
    m.write_u16(t + L::TIMER, b);
    b == 0
}

/// Se gira hacia el jugador. Original: `Breakable_Turn_05fec0`.
pub fn turn(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let dx = player_dx(m, t);
    m.write_u16(t + L::VEL_X, dx << 1);
    anim::start(m, t, L::S_TURN);
    set_handler(m, t, L::TURN_LOOP);
    turn_loop(m)
}

pub fn turn_loop(m: &mut impl Machine) -> Flow {
    script_loop(m)
}

/// Reacción de la plantilla `k` ($1F, $22, $23).
/// Original: `Breakable_Destroyed_05ff1c`, `Breakable22_Destroyed_060008`,
/// `Breakable23_Destroyed_0600f4`.
pub fn react(m: &mut impl Machine, k: usize) -> Flow {
    let t = m.task_register();
    anim::start(m, t, L::S_REACT[k]);
    set_handler(m, t, L::REACT_LOOP[k]);
    react_loop(m)
}

pub fn react_loop(m: &mut impl Machine) -> Flow {
    script_loop(m)
}

/// Golpeado: puntos, guion de rotura al azar y, al acabar, un trozo.
/// Original: `Breakable_SpawnPiece_060210`.
pub fn shatter(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    hits::award_own_points(m, t);
    let (i, _) = rng::below(m, 2);
    let s = m.read_u32(L::BREAK_SCRIPTS + (u32::from(i) << 2));
    let mut r = m.regs();
    r.a[0] = s;
    m.set_regs(r);
    if s != L::NO_SCRIPT {
        anim::start(m, t, s);
    }
    set_handler(m, t, L::BREAK_LOOP);
    shatter_loop(m)
}

pub fn shatter_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    scene(m, t);
    if anim::step(m, t) {
        spawn_piece(m, t);
        return tasks::finish(m);
    }
    Flow::Return
}

/// Suelta un trozo al azar (uno de los tres guiones de la plantilla $1F)
/// si quedan. Original: `Breakable_PieceRng_0603c4`.
fn spawn_piece(m: &mut impl Machine, t: u32) {
    let n = m.read_u8(t + L::PIECES).wrapping_sub(1);
    m.write_u8(t + L::PIECES, n);
    if n & 0x80 != 0 {
        return;
    }
    let (v, _) = rng::next(m);
    let k = u32::from(v & 3).min(2);
    let h = m.read_u32(L::PIECE_HANDLERS + (k << 2));
    let o = tasks::spawn(m, h, t);
    anim::copy_sprite_pos(m, t, o);
    m.write_u8(o + L::PIECES, n);
    let mut r = m.regs();
    r.a[0] = o;
    r.a[1] = h;
    m.set_regs(r);
}

// ---------------------------------------------------------------------------
// Esquirlas.
// ---------------------------------------------------------------------------

/// Suelta una esquirla (la llama un guion). Original: `Breakable_SpawnShards_06034c`.
pub fn spawn_shard(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let o = tasks::spawn(m, L::SHARD, t);
    anim::copy_sprite_pos(m, t, o);
    let y = m.read_u16(o + L::Y).wrapping_add(L::SHARD_DY);
    m.write_u16(o + L::Y, y);
    let vx = m.read_u16(t + L::VEL_X);
    m.write_u16(o + L::VEL_X, vx);
    m.write_u16(o + L::VEL_Y, L::SHARD_VY);
    m.write_u16(o + L::ACC_X, 0);
    let ay = m.read_u16(o + L::ACC_Y).wrapping_add(L::SHARD_AY);
    m.write_u16(o + L::ACC_Y, ay);
    let mut r = m.regs();
    r.a[0] = o;
    m.set_regs(r);
    Flow::Return
}

/// Esquirla que vuela girando, hace daño y estalla.
/// Original: `Breakable_Shard_06025e`.
pub fn shard(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    for g in L::SHARD_GFX {
        anim::alloc_entry(m, t, g);
    }
    for off in L::SHRINK {
        m.write_u8(t + off, L::FULL);
    }
    m.write_u16(t + L::ATTR, L::ATTR_SHARD);
    m.write_u8(t + L::FACING, 0);
    m.write_u32(t + L::ATTACKS, L::SHARD_ATTACKS);
    let mut r = m.regs();
    r.a[0] = L::SHARD_ATTACKS;
    m.set_regs(r);
    slug::clear_flags13(m, t);
    m.write_u16(t + L::TIMER, 0);
    anim::start(m, t, L::S_SHARD);
    set_handler(m, t, L::SHARD_LOOP);
    shard_loop(m)
}

pub fn shard_loop(m: &mut impl Machine) -> Flow {
    let t = m.task_register();
    let mut r = m.regs();
    let at = (r.d[3] as u16, r.d[4] as u16);
    r.a[5] = C::SCRATCH_BASE;
    m.set_regs(r);
    if motion::on_entity_pos(m, t, |m| motion::move_e_probe(m, t, at)) {
        return boom(m, t);
    }
    cycle_color(m, t);
    let vx = ((m.read_u16(t + L::VEL_X) as i16) >> 4) as u16;
    let vy = ((m.read_u16(t + L::VEL_Y) as i16) >> 4) as u16;
    let a = u16::from(banner::atan2(m, vx, vy)) >> 3;
    m.write_u16(t + L::ANGLE, a);
    anim::step(m, t);
    hits::run_attack_list(m, t);
    if m.read_u8(t + L::FLAGS_13) & L::SPENT != 0 {
        return boom(m, t);
    }
    let mut r = m.regs();
    r.a[0] = L::SHARD_MARGINS;
    m.set_regs(r);
    if items::out_of_view(m, t, Some(L::SHARD_MARGINS)) {
        return boom(m, t);
    }
    Flow::Return
}

/// Rota el color entre los tres guardados. Original: `Breakable_ShardVel_060322`.
fn cycle_color(m: &mut impl Memory, t: u32) {
    let i = u32::from(m.read_u16(t + L::TIMER) & 3) * 2;
    let c = m.read_u16(t + L::COLORS + i);
    m.write_u16(t + L::COLOR, c);
    let n = m.read_u16(t + L::TIMER).wrapping_add(1);
    let n = if (n as i16) < L::COLOR_COUNT as i16 { n } else { 0 };
    m.write_u16(t + L::TIMER, n);
}

/// Estalla y se da de baja.
fn boom(m: &mut impl Machine, t: u32) -> Flow {
    let o = tasks::spawn(m, L::BOOM, t);
    anim::copy_sprite_pos(m, t, o);
    m.write_u16(o + L::ATTR, L::ATTR_BOOM);
    let mut r = m.regs();
    r.a[0] = o;
    m.set_regs(r);
    tasks::finish(m)
}
