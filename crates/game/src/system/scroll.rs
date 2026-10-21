//! Scroll de la escena: la máquina que cada frame ejecuta el guion de
//! scroll (`scene::SCRIPT_PC`), mueve la cámara principal dentro de sus
//! límites, mide el avance de la misión, suaviza la cámara en vertical y
//! arrastra las cuatro cámaras de capa (con su repintado de columnas en el
//! vblank). Incluye también la consulta del mapa de colisión de la primera
//! capa.
//!
//! Original: `SceneScriptVM_Frame_0437DA`, `SceneScript_EdgeArrivalTest_043CE2`,
//! `Scroll_ClampToRange_043E3A`, `Camera_ResetSmoothing_0434EA`,
//! `Camera_SmoothingIntegrate_0434F8`, `CameraApplyAll4_043D86` / `$043DAA`,
//! `Buffer_ClearBlock1024L_043EDA`, `CollMap_RowToLocalY_043EEC`,
//! `CollMap_TestPoint_043F02`, `$04CAC4`, `$04CB88`, `$051B1C`, `$051B38`,
//! `$051B3E`, `$051B80`, `$051ECE`, `TransformCommit_MMIO_051F30`, `$051F94`.

use super::{irq, layer, scene, sound, sprite, video};
use crate::layout::{focus as F, hw, scene as SC, scroll as L, sprites as S, system as Y};
use crate::mem::{Machine, Memory};

fn read_i16(m: &mut impl Memory, a: u32) -> i16 {
    m.read_u16(a) as i16
}

/// Un frame del guion de scroll. `vel` son las velocidades X e Y (16.16)
/// que pide el que llama (salvo si hay unas fijadas en `SEED_VEL`).
pub fn frame(m: &mut impl Machine, vel: (u32, u32)) {
    let vel = if m.read_u16(L::SEED_LATCH) != 0 {
        (m.read_u32(L::SEED_VEL[0]), m.read_u32(L::SEED_VEL[1]))
    } else {
        vel
    };
    let mut r = m.regs();
    r.d[0] = vel.0;
    r.d[1] = vel.1;
    for t in L::TICKS {
        m.call_original(t, &mut r);
    }
    m.set_regs(r);
    run_script(m);
    end_frame(m, vel);
}

/// Ejecuta órdenes del guion hasta la que cierra el frame.
fn run_script(m: &mut impl Machine) {
    loop {
        if m.halted() {
            return;
        }
        let pc = m.read_u32(L::PC);
        let op = m.read_u16(pc);
        let next = match op {
            L::OP_END_FRAME => return,
            L::OP_WAIT => {
                // Código del guion: devuelve d0.b != 0 mientras haya que
                // esperar; si no, deja en a1 dónde sigue el guion.
                let r = embedded(m, pc + 2).unwrap_or_else(|| {
                    call(m, pc + 2, |r| {
                        r.a[0] = pc;
                        r.a[1] = pc;
                    })
                });
                if r.d[0] as u8 != 0 {
                    return;
                }
                r.a[1]
            }
            L::OP_CALL_JUMP => {
                embedded(m, pc + 2)
                    .unwrap_or_else(|| {
                        call(m, pc + 2, |r| {
                            r.a[0] = pc;
                            r.a[1] = pc;
                        })
                    })
                    .a[0]
            }
            L::OP_CALL_ARGS => {
                m.write_u32(L::PC, pc + 14);
                let f = m.read_u32(pc + 2);
                call(m, f, |r| {
                    r.a[0] = pc + 6;
                    r.a[1] = f;
                });
                continue;
            }
            L::OP_SPRITE_LIST => video::load_high_entries(m, pc + 2) + 2,
            _ => {
                let len = op_len(op);
                m.write_u32(L::PC, pc + len);
                simple_op(m, op, pc);
                continue;
            }
        };
        m.write_u32(L::PC, next);
    }
}

/// Los guiones de escena llevan pequeñas rutinas 68000 embebidas tras
/// las órdenes `OP_WAIT` / `OP_CALL_JUMP`. Todas siguen uno de estos
/// moldes, que se ejecutan aquí sin pasar por el código original:
///
/// - espera: `cmpi.{b,w} #v,(abs).l ; s<cc> d0 ; lea sigue(pc),a1 ; rts`
///   (d0.b = $FF mientras haya que seguir esperando);
/// - salto:  `move.{b,l} #v,(abs).l ; lea sigue(pc),a0 ; rts`.
///
/// Devuelve `None` si el código no encaja en ningún molde.
fn embedded(m: &mut impl Machine, at: u32) -> Option<crate::mem::Regs> {
    let mut r = m.regs();
    let op = m.read_u16(at);
    let rest = match op {
        L::EMB_CMPI_B | L::EMB_CMPI_W => {
            let v = m.read_u16(at + 2);
            let addr = m.read_u32(at + 4);
            let (x, v) = if op == L::EMB_CMPI_B {
                (u16::from(m.read_u8(addr)), v & 0xFF)
            } else {
                (m.read_u16(addr), v)
            };
            let set = match m.read_u16(at + 8) {
                L::EMB_SCS => x < v,
                L::EMB_SHI => x > v,
                L::EMB_SNE => x != v,
                L::EMB_SEQ => x == v,
                _ => return None,
            };
            r.d[0] = (r.d[0] & !0xFF) | if set { 0xFF } else { 0 };
            r.carry = x < v;
            at + 10
        }
        L::EMB_MOVE_B => {
            let v = m.read_u16(at + 2) as u8;
            let addr = m.read_u32(at + 4);
            m.write_u8(addr, v);
            at + 8
        }
        L::EMB_MOVE_L => {
            let v = m.read_u32(at + 2);
            let addr = m.read_u32(at + 6);
            m.write_u32(addr, v);
            r.carry = false;
            at + 10
        }
        _ => return None,
    };
    let lea = m.read_u16(rest);
    if m.read_u16(rest + 4) != L::EMB_RTS {
        return None;
    }
    let target = (rest + 2).wrapping_add(m.read_u16(rest + 2) as i16 as u32);
    match lea {
        L::EMB_LEA_A1 if matches!(op, L::EMB_CMPI_B | L::EMB_CMPI_W) => r.a[1] = target,
        L::EMB_LEA_A0 if matches!(op, L::EMB_MOVE_B | L::EMB_MOVE_L) => r.a[0] = target,
        _ => return None,
    }
    m.set_regs(r);
    Some(r)
}

/// Tamaño de las órdenes de longitud fija.
fn op_len(op: u16) -> u32 {
    match op {
        L::OP_BIND_PATH => 22,
        L::OP_SET_FIELDS | L::OP_LIMITS => 12,
        L::OP_TILE_TABLE => 10,
        L::OP_SET_TILE => 8,
        L::OP_WARP | L::OP_DETACH | L::OP_SHOW | L::OP_HIDE | L::OP_PROBE_SHOW | L::OP_CLEAR_TILES
        | L::OP_SPAWN | L::OP_PAIR | L::OP_SET_POS => 6,
        L::OP_LSPC_MODE | L::OP_AXIS | L::OP_SOUND | L::OP_SET_PROGRESS => 4,
        _ => panic!("orden de scroll desconocida {op:#x}"),
    }
}

/// Órdenes que sólo leen sus parámetros (el PC ya apunta a la siguiente).
fn simple_op(m: &mut impl Machine, op: u16, pc: u32) {
    match op {
        L::OP_BIND_PATH => {
            let cam = m.read_u32(pc + 2);
            let src = m.read_u32(pc + 0xE);
            m.write_u32(cam + L::C_COLLMAP_SRC, src);
            let x = m.read_u16(L::POS_X).wrapping_sub(m.read_u16(pc + 0x12));
            let y = m
                .read_u16(L::POS_Y)
                .wrapping_add(m.read_u16(L::SMOOTH_ACC))
                .wrapping_sub(m.read_u16(pc + 0x14));
            bind_path(m, cam, pc + 6, x, y);
        }
        L::OP_SET_FIELDS => {
            let cam = m.read_u32(pc + 2);
            let mode = m.read_u8(pc + 6);
            m.write_u8(cam + L::C_SCROLL_MODE, mode);
            for i in 0..2 {
                let v = m.read_u16(pc + 8 + 2 * i);
                m.write_u16(cam + L::C_SCALE[i as usize], v);
            }
        }
        L::OP_WARP => {
            m.write_u8(L::AXIS, L::AXIS_X_FORWARD);
            m.write_u16(L::PROGRESS, 0);
            set_position(m, pc);
            m.write_u16(L::DELTA_X, 0);
            m.write_u16(L::DELTA_Y, 0);
            publish_bases(m);
            reset_smoothing(m);
        }
        L::OP_SET_POS => {
            set_position(m, pc);
            publish_bases(m);
            m.write_u16(L::DELTA_X, 0);
            m.write_u16(L::DELTA_Y, 0);
            reset_smoothing(m);
        }
        L::OP_SET_TILE => {
            let cam = m.read_u32(pc + 2);
            let (attr, row) = (m.read_u8(pc + 6), m.read_u8(pc + 7));
            set_tile(m, cam, attr, u16::from(row));
        }
        L::OP_DETACH => {
            let cam = m.read_u32(pc + 2);
            m.write_u32(cam + L::C_PATH, 0);
            scene::hide_layer_sprites(m, cam);
            commit(m, cam);
        }
        L::OP_SHOW => {
            let cam = m.read_u32(pc + 2);
            show(m, cam);
        }
        L::OP_HIDE => {
            let cam = m.read_u32(pc + 2);
            scene::hide_layer_sprites(m, cam);
        }
        L::OP_PROBE_SHOW => {
            let cam = m.read_u32(pc + 2);
            layer::refresh_all(m, cam);
            show(m, cam);
        }
        L::OP_CLEAR_TILES => {
            let cam = m.read_u32(pc + 2);
            scene::clear_layer_tiles(m, cam);
            if m.read_u32(cam + L::C_COLLMAP_SRC) != 0 {
                clear_collmap(m, cam);
            }
        }
        L::OP_LSPC_MODE => {
            let v = m.read_u8(pc + 2);
            m.write_u8(Y::LSPC_MODE_SHADOW, v);
        }
        L::OP_SPAWN => {
            // El original escribe la rutina en el primer long del bloque nuevo.
            let handler = m.read_u32(pc + 2);
            let parent = m.task_register();
            let t = super::tasks::spawn(m, handler, parent);
            m.write_u32(t, handler);
        }
        L::OP_LIMITS => {
            for (i, a) in [L::MIN_X, L::MIN_Y, L::MAX_X, L::MAX_Y, L::SLOPE].into_iter().enumerate() {
                let v = m.read_u16(pc + 2 + 2 * i as u32);
                m.write_u16(a, v);
            }
        }
        L::OP_PAIR => {
            for i in 0..2 {
                let v = m.read_u16(pc + 2 + 2 * i as u32);
                m.write_u16(L::PAIR[i], v);
            }
        }
        L::OP_TILE_TABLE => {
            let cam = m.read_u32(pc + 2);
            let table = m.read_u32(pc + 6);
            set_tile_table(m, cam, table);
        }
        L::OP_AXIS => {
            let v = m.read_u8(pc + 2);
            m.write_u8(L::AXIS, v);
            publish_bases(m);
        }
        L::OP_SOUND => {
            let code = m.read_u16(pc + 2);
            sound::play_always(m, code);
        }
        L::OP_SET_PROGRESS => {
            let v = m.read_u16(pc + 2);
            m.write_u16(L::PROGRESS, v);
            publish_bases(m);
        }
        _ => unreachable!(),
    }
}

/// Llama a código original (o del guion) con los registros actuales
/// retocados por `set`; devuelve los registros a la vuelta.
fn call(m: &mut impl Machine, routine: u32, set: impl FnOnce(&mut crate::mem::Regs)) -> crate::mem::Regs {
    let mut r = m.regs();
    set(&mut r);
    m.call_original(routine, &mut r);
    m.set_regs(r);
    r
}

/// Posición (X, Y enteros en `pc+2`, `pc+4`) de la cámara principal.
fn set_position(m: &mut impl Memory, pc: u32) {
    let x = u32::from(m.read_u16(pc + 2)) << 16;
    let y = u32::from(m.read_u16(pc + 4)) << 16;
    m.write_u32(L::POS_X, x);
    m.write_u32(L::POS_Y_SMOOTH, y);
    m.write_u32(L::POS_Y, y);
}

/// El avance se vuelve a medir desde aquí.
fn publish_bases(m: &mut impl Memory) {
    for (to, from) in [(L::BASE_PROGRESS, L::PROGRESS), (L::BASE_X, L::POS_X), (L::BASE_Y, L::POS_Y)] {
        let v = m.read_u16(from);
        m.write_u16(to, v);
    }
}

/// Caja que encierra a los jugadores en juego: devuelve ((x, y) del
/// segundo, (x, y) del primero), como longs; si a uno le falta un dato se
/// usa el del otro (negativo = nadie). Original: `$0967C0`.
pub fn focus(m: &mut impl Memory) -> ((u32, u32), (u32, u32)) {
    let (mut x2, mut y2) = player_point(m, F::PLAYERS[0]);
    let (mut x0, mut y0) = player_point(m, F::PLAYERS[1]);
    if (x0 as i32) < 0 {
        x0 = x2;
    }
    if (x2 as i32) < 0 {
        x2 = x0;
    }
    if (y0 as i32) < 0 {
        y0 = y2;
    }
    if (y2 as i32) < 0 {
        y2 = y0;
    }
    ((x0, y0), (x2, y2))
}

/// Posición de un jugador (longs; `0xFFFFFFFF` si no está en juego o su
/// Y no cuenta). Original: `$05E2D8`.
pub fn player_point(m: &mut impl Memory, t: u32) -> (u32, u32) {
    if F::INACTIVE.contains(&m.read_u32(t)) {
        return (u32::MAX, u32::MAX);
    }
    let x = u32::from(m.read_u16(t + F::X));
    let y = if m.read_u8(t + F::FLAGS) & F::HAS_Y != 0 { u32::from(m.read_u16(t + F::Y)) } else { u32::MAX };
    (x, y)
}

/// La cámara sigue la caja de los jugadores (`a` = d0/d1, `b` = d2/d3,
/// como los deja [`focus`]) y corre el frame del guion de scroll.
/// Original: `Geom_Proj_Clamp_0436DE`.
pub fn follow(m: &mut impl Machine, a: (u32, u32), b: (u32, u32)) {
    let vel = if m.read_u8(L::EDGE_OFF) != 0 || (a.0 as i32) < 0 || (a.1 as i32) < 0 {
        (0, 0)
    } else {
        let order = |p: u16, q: u16| if q < p { (q, p) } else { (p, q) };
        let (x0, x1) = order(a.0 as u16, b.0 as u16);
        // Y de pantalla hacia arriba.
        let (y0, y1) = order(F::SCREEN_H.wrapping_sub(a.1 as u16), F::SCREEN_H.wrapping_sub(b.1 as u16));
        let vx = follow_axis(x0, x1, m.read_u16(L::PAIR[0]), F::WINDOW_X);
        let center_y = m.read_u16(L::PAIR[1]).wrapping_sub(m.read_u16(L::SMOOTH_ACC));
        let vy = follow_axis(y0, y1, center_y, F::WINDOW_Y);
        (vx, vy)
    };
    frame(m, vel);
}

/// Velocidad (16.16) de un eje: si el centro de la caja `lo..hi` se aleja
/// más de un píxel de `center`, 1,5 veces el desvío sin que ningún extremo
/// salga de `window`, en 1/16 y con tope. Original: `$04374C`.
fn follow_axis(lo: u16, hi: u16, center: u16, window: (u16, u16)) -> u32 {
    let fx = |v: u16| (u32::from(v) << 16) as i32;
    let (lo, hi, center) = (fx(lo), fx(hi), fx(center));
    let d = (lo.wrapping_add(hi) >> 1).wrapping_sub(center).wrapping_add(0x1_0000);
    if (d as u32) < 0x2_0000 {
        return 0;
    }
    let d = d.wrapping_sub(0x1_0000);
    let room_lo = lo.wrapping_sub(fx(window.0));
    let room_hi = hi.wrapping_sub(fx(window.1));
    let mut v = d.wrapping_add(d >> 1);
    if v >= 0 {
        if room_lo < 0 {
            v = 0;
        } else if v > room_lo {
            v = room_lo;
        }
    } else if room_hi > 0 {
        v = 0;
    } else if v < room_hi {
        v = room_hi;
    }
    (v >> 4).clamp(-F::SPEED_LIMIT, F::SPEED_LIMIT) as u32
}

/// Cierre del frame: mueve la cámara principal, mide el avance, suaviza la
/// Y y arrastra las capas.
fn end_frame(m: &mut impl Machine, vel: (u32, u32)) {
    let (dx, x) = clamp_step(vel.0, m.read_u32(L::POS_X), m.read_u16(L::MIN_X), m.read_u16(L::MAX_X));
    m.write_u16(L::DELTA_X, dx);
    m.write_u32(L::POS_X, x);

    let (mut vy, mut min_y, mut max_y) = (vel.1, m.read_u16(L::MIN_Y), m.read_u16(L::MAX_Y));
    let slope = read_i16(m, L::SLOPE);
    if slope != 0 {
        // Tramo en diagonal: la Y sigue a la X y la ventana se desplaza.
        let dx = m.read_u16(L::DELTA_X);
        let dy = if slope > 0 { dx.wrapping_neg() } else { dx };
        vy = u32::from(dy) << 16;
        let left = m.read_u16(L::MAX_X).wrapping_sub(m.read_u16(L::POS_X)) as i16;
        let adj = ((i32::from(left) * i32::from(slope)) >> 8) as u16;
        min_y = min_y.wrapping_add(adj);
        max_y = max_y.wrapping_add(adj);
    }
    let (dy, y) = clamp_step(vy, m.read_u32(L::POS_Y), min_y, max_y);
    m.write_u16(L::DELTA_Y, dy);
    m.write_u32(L::POS_Y, y);

    let x = m.read_u16(L::POS_X);
    let y = m.read_u16(L::POS_Y);
    let advance = match m.read_u8(L::AXIS) {
        L::AXIS_Y_FORWARD => y.wrapping_sub(m.read_u16(L::BASE_Y)),
        L::AXIS_Y_BACK => m.read_u16(L::BASE_Y).wrapping_sub(y),
        L::AXIS_X_BACK => m.read_u16(L::BASE_X).wrapping_sub(x),
        _ => x.wrapping_sub(m.read_u16(L::BASE_X)),
    };
    let progress = advance.wrapping_add(m.read_u16(L::BASE_PROGRESS));
    if progress > m.read_u16(L::PROGRESS) {
        m.write_u16(L::PROGRESS, progress);
    }

    smoothing_step(m);
    let smooth = m.read_u32(L::SMOOTH_ACC).wrapping_add(m.read_u32(L::POS_Y));
    m.write_u32(L::POS_Y_SMOOTH, smooth);
    let dy = m.read_u16(L::DELTA_Y).wrapping_add(m.read_u16(L::SMOOTH_DELTA));
    m.write_u16(L::DELTA_Y, dy);
    mission_mark_frame(m);
    apply_all(m);
}

/// Un paso de scroll con tope: avanza `pos` (16.16) en `vel` sin pasar de
/// `min`/`max` (enteros; `max` incluye su último subpíxel). Devuelve el
/// desplazamiento entero y la posición nueva. Original: `Scroll_ClampToRange_043E3A`.
pub fn clamp_step(vel: u32, pos: u32, min: u16, max: u16) -> (u16, u32) {
    if saturated(vel, pos, min, max) {
        return (0, pos);
    }
    let limit = if (vel as i32) > 0 { (u32::from(max) << 16) | 0xFFFF } else { u32::from(min) << 16 };
    let mut new = pos.wrapping_add(vel);
    if limit != u32::MAX && ((limit.wrapping_sub(new) ^ limit.wrapping_sub(pos)) as i32) < 0 {
        new = limit;
    }
    (((new >> 16) as u16).wrapping_sub((pos >> 16) as u16), new)
}

/// ¿No hay nada que mover? (velocidad 0, o ya en el límite hacia el que va).
pub fn saturated(vel: u32, pos: u32, min: u16, max: u16) -> bool {
    match (vel as i32).signum() {
        0 => true,
        1 => ((u32::from(max) << 16) | 0xFFFF) < pos,
        _ => (u32::from(min) << 16) >= pos,
    }
}

/// Suavizado vertical a cero. Original: `Camera_ResetSmoothing_0434EA`.
pub fn reset_smoothing(m: &mut impl Memory) {
    m.write_u32(L::SMOOTH_ACC, 0);
    m.write_u32(L::SMOOTH_VEL, 0);
}

/// Un paso del suavizado vertical (muelle amortiguado con tope). Devuelve
/// el paso entero (también en `SMOOTH_DELTA`). Original: `Camera_SmoothingIntegrate_0434F8`.
pub fn smoothing_step(m: &mut impl Memory) -> u16 {
    let acc = m.read_u32(L::SMOOTH_ACC) as i32;
    let vel = m.read_u32(L::SMOOTH_VEL) as i32;
    let damp = if vel >= L::SMOOTH_DAMP || vel <= -L::SMOOTH_DAMP { vel >> 3 } else { vel };
    let vel = vel.wrapping_sub(damp.wrapping_add(acc));
    m.write_u32(L::SMOOTH_VEL, vel as u32);
    let new = vel.wrapping_add(acc).clamp(-L::SMOOTH_LIMIT, L::SMOOTH_LIMIT);
    m.write_u32(L::SMOOTH_ACC, new as u32);
    let d = ((new >> 16) as u16).wrapping_sub((acc >> 16) as u16);
    m.write_u16(L::SMOOTH_DELTA, d);
    d
}

/// ¿Ha llegado el scroll al borde hacia el que avanza? (también sí si la
/// prueba está desactivada). Original: `SceneScript_EdgeArrivalTest_043CE2`.
pub fn edge_reached(m: &mut impl Memory) -> bool {
    if m.read_u8(L::EDGE_OFF) != 0 || m.read_u8(L::EDGE_ENABLE) == 0 {
        return true;
    }
    let (pos, limit) = match m.read_u8(L::AXIS) {
        L::AXIS_Y_FORWARD => (L::POS_Y, L::MAX_Y),
        L::AXIS_Y_BACK => (L::POS_Y, L::MIN_Y),
        L::AXIS_X_BACK => (L::POS_X, L::MIN_X),
        _ => (L::POS_X, L::MAX_X),
    };
    m.read_u16(pos) == m.read_u16(limit)
}

/// Arrastra las cuatro cámaras de capa. Original: `CameraApplyAll4_043D86`.
pub fn apply_all(m: &mut impl Machine) {
    for cam in SC::CAMERAS {
        apply(m, cam);
    }
}

/// Mueve una capa según el desplazamiento del frame y su escala (8.8),
/// avisa a su ruta y prepara el repintado. Original: `$043DAA`.
pub fn apply(m: &mut impl Machine, cam: u32) {
    if m.read_u32(cam + L::C_PATH) == 0 {
        return;
    }
    let dx = i32::from(read_i16(m, L::DELTA_X)) * i32::from(read_i16(m, cam + L::C_SCALE[0]));
    let dy = i32::from(read_i16(m, L::DELTA_Y)) * i32::from(read_i16(m, cam + L::C_SCALE[1]));
    let (dx, dy) = move_layer(m, cam, (dx << 8) as u32, (dy << 8) as u32);
    let _ = (dx, dy);
    // Modo de la capa: bit 0 = pinta las columnas que entran, bit 1 = las filas.
    let mode = m.read_u8(cam + L::C_SCROLL_MODE);
    if mode & 1 != 0 {
        layer::scroll_x_all(m, cam);
    }
    if mode & 2 != 0 {
        layer::scroll_y_all(m, cam);
    }
    commit(m, cam);
}

/// Suma (dx, dy) (16.16) a la posición de la capa y guarda el
/// desplazamiento entero. Original: `$051B80`.
pub fn move_layer(m: &mut impl Memory, cam: u32, dx: u32, dy: u32) -> (u16, u16) {
    let mut out = [0u16; 2];
    for (i, d) in [dx, dy].into_iter().enumerate() {
        let old = m.read_u32(cam + L::C_POS[i]);
        let new = old.wrapping_add(d);
        m.write_u32(cam + L::C_POS[i], new);
        out[i] = ((new >> 16) as u16).wrapping_sub((old >> 16) as u16);
        m.write_u16(cam + L::C_DELTA[i], out[i]);
    }
    (out[0], out[1])
}

/// Tile de relleno de la capa (fila y atributos). Original: `$051B1C`.
pub fn set_tile(m: &mut impl Memory, cam: u32, attr: u8, row: u16) {
    m.write_u8(cam + L::C_TILE_ATTR, attr & 0x3F);
    let r = row.wrapping_add(m.read_u16(cam + L::C_TILE_BASE[1])).wrapping_neg() & 0x1F;
    m.write_u8(cam + L::C_TILE_ROW, r as u8);
    m.write_u32(cam + L::C_TILE_TABLE, 0);
}

/// Tabla de tiles de la capa. Original: `$051B38`.
pub fn set_tile_table(m: &mut impl Memory, cam: u32, table: u32) {
    m.write_u32(cam + L::C_TILE_TABLE, table);
}

/// Ata la capa a una ruta: rehace la base de tiles para que la posición
/// pase a ser (x, y) sin moverse en pantalla. Original: `$051B3E`.
pub fn bind_path(m: &mut impl Memory, cam: u32, path: u32, x: u16, y: u16) {
    m.write_u32(cam + L::C_PATH, path);
    for (i, v) in [x, y].into_iter().enumerate() {
        let off = m.read_u16(cam + L::C_POS[i]).wrapping_sub(v) >> 4;
        let base = off.wrapping_add(m.read_u16(cam + L::C_TILE_BASE[i])) & 0x1F;
        m.write_u16(cam + L::C_TILE_BASE[i], base);
    }
    m.write_u16(cam + L::C_POS[0], x);
    m.write_u16(cam + L::C_POS[1], y);
    for a in L::C_ORIGIN_CELL {
        m.write_u16(cam + a, 0);
    }
    set_tile(m, cam, 0x20, 0);
}

/// La capa se repinta. Original: `$051ECE`.
pub fn show(m: &mut impl Memory, cam: u32) {
    let f = m.read_u8(cam + L::C_FLAGS);
    m.write_u8(cam + L::C_FLAGS, f | L::C_VISIBLE);
}

/// Si la capa es visible, calcula las columnas a repintar y su posición
/// de pantalla, y deja el repintado para el vblank.
/// Original: `TransformCommit_MMIO_051F30`.
pub fn commit(m: &mut impl Memory, cam: u32) {
    if m.read_u8(cam + L::C_FLAGS) & L::C_VISIBLE == 0 {
        return;
    }
    let x = m.read_u16(cam + L::C_POS[0]).wrapping_add(m.read_u16(cam + L::C_TILE_BASE[0]) << 4);
    let first = (x >> 4).wrapping_sub(1) & (L::COLUMNS - 1);
    m.write_u16(cam + L::C_COL_FIRST, first);
    m.write_u16(cam + L::C_COL_END, (first + L::VISIBLE_COLUMNS) & (L::COLUMNS - 1));
    m.write_u16(cam + L::C_SCREEN_X, x.wrapping_neg() << 7);
    let y = m.read_u16(cam + L::C_POS[1]).wrapping_add(m.read_u16(cam + L::C_TILE_BASE[1]) << 4);
    m.write_u16(cam + L::C_SCREEN_Y, y << 7);
    irq::defer(m, L::COLUMN_WRITE, cam);
}

/// En el vblank: oculta la columna anterior a la primera, coloca las
/// visibles (SCB3: Y, fila de tile y atributos; SCB4: X) y oculta la que
/// sigue a la última. Original: `$051F94` (contexto `cam` en a6).
pub fn write_columns(m: &mut impl Memory, cam: u32) {
    let base = S::SCB3_FIRST.wrapping_add(m.read_u16(cam + L::C_SPRITE_BASE));
    let screen_y = m.read_u16(cam + L::C_SCREEN_Y);
    let end = m.read_u16(cam + L::C_COL_END);
    let mut col = m.read_u16(cam + L::C_COL_FIRST);
    vram(m, base.wrapping_add(col), 0);
    col = (col + 1) & (L::COLUMNS - 1);
    let mut x = (col << 11).wrapping_add(m.read_u16(cam + L::C_SCREEN_X));
    while col != end {
        let row = u16::from(m.read_u8(cam + L::C_COL_ROW + u32::from(col))) << 11;
        let attr = u16::from(m.read_u8(cam + L::C_COL_ATTR + u32::from(col)));
        let scb3 = row.wrapping_add(screen_y) | attr;
        let a = base.wrapping_add(col);
        vram(m, a, scb3);
        vram(m, a.wrapping_add(0x200), x);
        x = x.wrapping_add(0x800);
        col = (col + 1) & (L::COLUMNS - 1);
    }
    vram(m, base.wrapping_add(col), 0);
}

fn vram(m: &mut impl Memory, addr: u16, data: u16) {
    m.write_u16(Y::VRAM_ADDR_SHADOW, addr);
    m.write_u16(hw::VRAM_ADDR, addr);
    m.write_u16(hw::VRAM_RW, data);
}

/// Mapa de colisión de la capa (4 KB) a 0. Original: `Buffer_ClearBlock1024L_043EDA`.
pub fn clear_collmap(m: &mut impl Memory, cam: u32) {
    for i in 0..SC::COLLMAP_LEN / 4 {
        m.write_u32(cam + L::C_COLLMAP + 4 * i, 0);
    }
}

/// Fila de celda → Y local (eje invertido). Original: `CollMap_RowToLocalY_043EEC`.
pub fn row_to_local_y(m: &mut impl Memory, row: u16) -> u16 {
    let c = L::COLL_CAMERA;
    (row.wrapping_add(m.read_u16(c + L::C_TILE_BASE[1])) << 4)
        .wrapping_sub(m.read_u16(c + L::COLL_ORIGIN[1]))
        .wrapping_neg()
}

/// Resultado de [`test_point`]: si la celda es sólida y el resto dentro
/// de la celda de 8×8.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CellHit {
    pub solid: bool,
    pub frac: (u16, u16),
    pub cell: u8,
}

/// ¿Cae el punto (x, y) (Y hacia arriba) en una celda sólida del mapa de
/// colisión? Las celdas de 8×8 van por parejas de columnas de 64 filas.
/// Original: `CollMap_TestPoint_043F02`.
pub fn test_point(m: &mut impl Memory, x: u16, y: u16) -> CellHit {
    let c = L::COLL_CAMERA;
    let x = x
        .wrapping_add(m.read_u16(c + L::COLL_ORIGIN[0]))
        .wrapping_add(m.read_u16(c + L::C_TILE_BASE[0]) << 4);
    let y = m
        .read_u16(c + L::COLL_ORIGIN[1])
        .wrapping_sub(y)
        .wrapping_add(m.read_u16(c + L::C_TILE_BASE[1]) << 4);
    let idx = ((x & 0x1F0) << 3) + ((y & 0x1F8) >> 2) + ((x >> 3) & 1);
    let cell = m.read_u8(c + L::C_COLLMAP + u32::from(idx));
    CellHit { solid: cell != 0, frac: (x & 7, y & 7), cell }
}

/// Marcador de depuración de la misión en curso: avanza al siguiente
/// cuando el avance lo alcanza y devuelve su posición de pantalla y sus
/// datos. Original: `$04CAC4`.
pub fn mission_mark(m: &mut impl Memory) -> Mark {
    let mut p = m.read_u32(SC::MISSION_PTR);
    if (p as i32) < 0 {
        return Mark { pos: (0x50, 0x1F8), d2: 0, d3: 0x10, bytes: [0; 3] };
    }
    if m.read_u16(L::PROGRESS) >= m.read_u16(p + L::MARK_PROGRESS) {
        p += L::MARK_SIZE;
        m.write_u32(SC::MISSION_PTR, p);
    }
    let mut x = m.read_u16(p + L::MARK_POS[0]);
    if m.read_u8(p + L::MARK_REL[0]) != 0 {
        x = x.wrapping_sub(m.read_u16(L::POS_X));
    }
    let mut y = m.read_u16(p + L::MARK_POS[1]);
    if m.read_u8(p + L::MARK_REL[1]) != 0 {
        y = y.wrapping_sub(m.read_u16(L::POS_Y_SMOOTH));
    }
    Mark {
        pos: (x, y.wrapping_neg().wrapping_add(L::MARK_Y_BASE)),
        d2: m.read_u16(p + 8),
        d3: m.read_u16(p + 0xA),
        bytes: [m.read_u8(p + 0xC), m.read_u8(p + 0xD), m.read_u8(p + 0xE)],
    }
}

/// Lo que devuelve [`mission_mark`] (los registros d0–d6 del original).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mark {
    pub pos: (u16, u16),
    pub d2: u16,
    pub d3: u16,
    pub bytes: [u8; 3],
}

/// Marcador de la misión del frame; con la depuración activa, lo dibuja.
/// Original: `$04CB88`.
pub fn mission_mark_frame(m: &mut impl Machine) {
    let mark = mission_mark(m);
    if m.read_u8(L::DEBUG) & L::DEBUG_MARK == 0 {
        return;
    }
    let req = sprite::Request {
        pos: (u32::from(mark.pos.0) << 16) | u32::from(mark.pos.1),
        key: L::MARK_KEY,
        palette: 0,
        shrink_v: 0xFF,
        shrink_h: 0xFF,
        flip: 0,
        gfx: L::MARK_GFX,
    };
    let task = m.task_register();
    sprite::dispatch(m, task, req, true);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::{TestMachine, TestRam};

    #[test]
    fn clamp_step_stops_at_limits() {
        // Avanza libre.
        assert_eq!(clamp_step(0x1_8000, 0x10_0000, 0, 0x20), (1, 0x11_8000));
        // Cruza el máximo: se queda en su último subpíxel.
        assert_eq!(clamp_step(0x5_0000, 0x1E_0000, 0, 0x20), (2, 0x20_FFFF));
        // Ya en el mínimo y retrocediendo: nada.
        assert_eq!(clamp_step(0xFFFF_0000, 0x10_0000, 0x10, 0x20), (0, 0x10_0000));
        assert_eq!(clamp_step(0, 0x10_0000, 0, 0x20), (0, 0x10_0000));
        // Retrocede y cruza el mínimo.
        assert_eq!(clamp_step(0xFFFC_0000, 0x12_0000, 0x10, 0x20), (0xFFFE, 0x10_0000));
    }

    #[test]
    fn smoothing_spring_is_clamped() {
        let mut m = TestRam::default();
        m.write_u32(L::SMOOTH_ACC, 0);
        m.write_u32(L::SMOOTH_VEL, 0x10_0000);
        smoothing_step(&mut m);
        assert_eq!(m.read_u32(L::SMOOTH_ACC) as i32, L::SMOOTH_LIMIT);
        assert_eq!(m.read_u16(L::SMOOTH_DELTA), 8);
    }

    #[test]
    fn script_runs_until_end_of_frame() {
        let mut m = TestMachine::default();
        let pc = 0x10_2000;
        // $0F límites, $16 avance = 5, $02 fin.
        let words = [0x0F, 0, 0, 0x100, 0x100, 0, 0x16, 5, 0x02];
        for (i, w) in words.iter().enumerate() {
            m.write_u16(pc + 2 * i as u32, *w);
        }
        m.write_u32(L::PC, pc);
        frame(&mut m, (0x2_0000, 0));
        assert_eq!(m.read_u32(L::PC), pc + 16);
        assert_eq!(m.read_u16(L::MAX_X), 0x100);
        assert_eq!(m.read_u16(L::DELTA_X), 2);
        assert_eq!(m.read_u16(L::POS_X), 2);
        // Eje X hacia atrás (0): el avance no crece.
        assert_eq!(m.read_u16(L::PROGRESS), 5);
        assert_eq!(&m.calls[..4], &L::TICKS);
    }

    #[test]
    fn collision_point_index() {
        let mut m = TestRam::default();
        let c = L::COLL_CAMERA;
        m.write_u16(c + L::COLL_ORIGIN[1], 0x100);
        // x = 0x18, y = 0x100 - 0xF0 = 0x10 → (0x10<<3) + (0x10>>2) + 1.
        m.write_u8(c + L::C_COLLMAP + 0x80 + 4 + 1, 1);
        let hit = test_point(&mut m, 0x18, 0xF0);
        assert!(hit.solid);
        assert_eq!(hit.frac, (0, 0));
        assert!(!test_point(&mut m, 0x10, 0xF0).solid);
    }
}

/// Tarea fija `$1008A0`: borra cada frame el límite X de la cámara que
/// publica quien la bloquea (jefes). Original: `Task_Clear106F42_05ea96`.
pub fn clear_lock_x_start(m: &mut impl crate::mem::Machine) -> crate::mem::Flow {
    let t = m.task_register();
    m.write_u32(t + crate::layout::task::HANDLER, L::CLEAR_LOCK_X_RUN);
    clear_lock_x(m)
}

/// Original: `$05EA9C`.
pub fn clear_lock_x(m: &mut impl crate::mem::Machine) -> crate::mem::Flow {
    m.write_u16(L::CAMERA_LOCK_X, 0);
    crate::mem::Flow::Return
}

/// Arranca (`on`) o para la tarea fija que borra el límite X de la
/// cámara. Original: `$05EA6E` (sólo si `a5` es válido) / `$05EA86`.
pub fn set_lock_x_clearer(m: &mut impl Memory, on: bool) {
    let h = if on { L::CLEAR_LOCK_X } else { crate::layout::routine::IDLE };
    m.write_u32(L::LOCK_X_TASK, h);
}
