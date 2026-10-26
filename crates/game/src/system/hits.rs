//! Golpes entre entidades: cada entidad que ataca tiene una lista de
//! cajas de ataque (`+$4C`, registros de `$50` bytes terminados en `$FF`).
//! Cada caja dice contra quién va (jugadores, Slug, enemigos de una u otra
//! capa) y se prueba contra las cajas de golpe (`+$48`) de las víctimas:
//! si tocan, la víctima recibe el daño, se apunta quién y con qué le han
//! dado, y el atacante sabe si ha acertado.
//!
//! También la puntuación por daño (los enemigos dan puntos al jugador que
//! los golpea) y la prueba de «en pantalla» de las cajas.
//!
//! Original: `Hitbox_RunAttackList_0283d8` (`$0283D8`…`$02856A`),
//! `Hitbox_TestVictim_028576`, `Hitbox_TestShape_028594`,
//! `Hitbox_SubtractHP_0286ec`, `Hitbox_OverlapRect_02876e`,
//! `Hitbox_OverlapPoint_028886`, `Hitbox_OnScreen_028930` y
//! `$051914` (`asm/entity/collmap_hitbox_script_ops_027400.s`,
//! `asm/player/dispatch_cluster_051914.s`).

use crate::layout::{hits as L, task as T, PLAYER_ENTITY};
use crate::mem::Memory;

/// Resultado de la lista de ataque, como lo deja el original en `d7`: con
/// quién ha chocado en las listas de un jugador (-1 = nada o no aplica,
/// 0 = el jugador 1 ha dado al Slug, 1 = el jugador 2).
pub type Outcome = i8;

/// Ejecuta la lista de cajas de ataque de `t`. Original:
/// `Hitbox_RunAttackList_0283d8`. Devuelve el resultado y la dirección
/// final del cursor (lo que queda en `a0`).
pub fn run_attack_list(m: &mut impl Memory, t: u32) -> (Outcome, u32) {
    let (o, a, _) = run_attack_list_d1(m, t, 0xFF);
    (o, a)
}

/// Como [`run_attack_list`], devolviendo además el byte bajo de `d1` que
/// deja el original (la dureza de la caja golpeada tras un acierto con
/// daño, `$FF` en las demás salidas; `d1_in` si no prueba nada).
pub fn run_attack_list_d1(m: &mut impl Memory, t: u32, d1_in: u8) -> (Outcome, u32, u8) {
    let list = m.read_u32(t + L::ATTACKS);
    let mut out: Outcome = -1;
    let mut d1 = d1_in;
    if list == L::NONE {
        return (out, list, d1);
    }
    let mut a = list;
    loop {
        let kind = m.read_u8(a);
        if kind == L::END {
            return (out, a.wrapping_sub(L::ATTACK_LEN), d1);
        }
        out = match kind {
            0 => {
                for v in [PLAYER_ENTITY[0], PLAYER_ENTITY[1], L::SLUG] {
                    d1 = victim(m, t, a, v).1;
                }
                -1
            }
            1 | 2 => {
                victim(m, t, a, PLAYER_ENTITY[usize::from(kind - 1)]);
                // ¿Ha dado ya a ese jugador (bit «gastada» de `FLAGS3`)?
                let hit_player = m.read_u8(t + L::FLAGS3) & L::SPENT != 0;
                d1 = victim(m, t, a, L::SLUG).1;
                if hit_player { (kind - 1) as i8 } else { -1 }
            }
            3 | 4 => {
                let root = L::ENEMY_ROOTS[usize::from(kind - 3)];
                d1 = test_layer(m, t, a, root, d1);
                -1
            }
            // Un tipo mayor que 4 es un error de datos (`trap #15`).
            _ => -1,
        };
        a += L::ATTACK_LEN;
    }
}

/// Prueba la caja `a` contra todas las tareas hijas de `root` (los
/// enemigos de una capa) mientras no se haya gastado (bit 1 de `FLAGS2`).
/// Devuelve el `d1` del original.
fn test_layer(m: &mut impl Memory, t: u32, a: u32, root: u32, d1: u8) -> u8 {
    let mut d1 = screen_d1(m, a).unwrap_or(d1);
    if !on_screen(m, t, a, None) || spent(m, t) {
        return d1;
    }
    let prio = m.read_u8(root + T::PRIORITY);
    let mut v = m.read_u32(root + T::NEXT);
    while prio < m.read_u8(v + T::PRIORITY) {
        let boxes = m.read_u32(v + L::HITBOX);
        if boxes != L::NONE && v != t {
            d1 = shape(m, t, a, v, boxes).1;
            if spent(m, t) {
                return d1;
            }
        }
        v = m.read_u32(v + T::NEXT);
    }
    d1
}

/// `d1` que deja [`on_screen`] con los márgenes por defecto (nada si la
/// caja vale siempre).
fn screen_d1(m: &mut impl Memory, a: u32) -> Option<u8> {
    if m.read_u8(a + L::ALWAYS) != 0 {
        return None;
    }
    Some(L::SCREEN.1.wrapping_add(m.read_u16(L::DEFAULT_MARGINS + 2)) as u8)
}

fn spent(m: &mut impl Memory, t: u32) -> bool {
    m.read_u8(t + L::FLAGS2) & L::SPENT != 0
}

/// Prueba la caja de ataque `a` de `t` contra la víctima `v`.
/// Original: `Hitbox_TestVictim_028576`. Devuelve si ha dado.
pub fn test_victim(m: &mut impl Memory, t: u32, a: u32, v: u32) -> bool {
    victim(m, t, a, v).0
}

/// [`test_victim`] con el `d1` del original.
fn victim(m: &mut impl Memory, t: u32, a: u32, v: u32) -> (bool, u8) {
    if v == t {
        return (false, 0xFF);
    }
    let boxes = m.read_u32(v + L::HITBOX);
    if boxes == L::NONE || spent(m, t) {
        return (false, 0xFF);
    }
    shape(m, t, a, v, boxes)
}

/// Recorre las cajas de golpe `boxes` de la víctima `v` (ver
/// `layout::hits::BOX_LEN`) y prueba la caja de ataque `a` con cada una.
/// Original: `Hitbox_TestShape_028594`.
pub fn test_shape(m: &mut impl Memory, t: u32, a: u32, v: u32, boxes: u32) -> bool {
    shape(m, t, a, v, boxes).0
}

/// [`test_shape`] con el `d1` del original.
fn shape(m: &mut impl Memory, t: u32, a: u32, v: u32, mut boxes: u32) -> (bool, u8) {
    loop {
        if m.read_u32(boxes) == L::NONE || m.read_u8(v + L::FLAGS2) & L::HIT != 0 {
            return (false, 0xFF);
        }
        let immune = match m.read_u32(boxes + L::IMMUNITY) {
            L::NONE => L::DEFAULT_IMMUNITY,
            p => p,
        };
        let rect = boxes + L::BOX_HEADER;
        let class = m.read_u8(a + L::CLASS);
        if m.read_u8(immune + u32::from(class)) != 1 {
            let shape = m.read_u8(a + L::SHAPE);
            let hit = match shape {
                0 => overlap_point(m, t, a + L::BOX_HEADER, v, rect),
                4 => overlap_rect(m, t, a + L::BOX_HEADER, v, rect),
                // Otra forma es un error de datos (`trap #15`).
                _ => false,
            };
            if hit {
                return apply_hit(m, t, a, v, boxes);
            }
        }
        boxes += L::BOX_LEN;
    }
}

/// La caja de ataque `a` de `t` ha dado a `v`. Original: el tramo
/// `$028618`…`$0286E8` de `Hitbox_TestShape_028594`.
fn apply_hit(m: &mut impl Memory, t: u32, a: u32, v: u32, boxes: u32) -> (bool, u8) {
    if m.read_u8(v + L::INVULNERABLE) != 0 {
        set_bits(m, t + L::FLAGS2, L::SPENT);
        set_bits(m, t + L::FLAGS3, L::SPENT);
        return (false, 0xFF);
    }
    let class = m.read_u8(a + L::CLASS);
    m.write_u8(v + L::ATTACK_CLASS, class);
    m.write_u32(v + L::ATTACKER, t);
    let power = m.read_u8(a + L::POWER);
    if power == 0 {
        set_bits(m, t + L::FLAGS2, L::SPENT);
        set_bits(m, t + L::FLAGS3, L::SPENT);
        return (false, 0xFF);
    }
    // Clase 0, o clase `$0B` contra quien la esquiva (bit 0 de `+$6B`):
    // acierta sin hacer daño.
    let dodges = m.read_u8(v + L::DODGE) & 1 != 0 && class == L::DODGED_CLASS;
    let mut d1 = 0xFF;
    if class != 0 && !dodges {
        set_bits(m, v + L::FLAGS2, L::HIT);
        set_bits(m, v + L::FLAGS3, L::HIT_FLASH);
        let damage = m.read_u16(a + L::DAMAGE);
        subtract_hp(m, t, v, damage);
        d1 = m.read_u8(boxes + L::TOUGHNESS);
        if power as i8 > d1 as i8 {
            set_bits(m, t + L::FLAGS2, L::BLOCKED);
            set_bits(m, t + L::FLAGS3, L::BLOCKED);
            return (false, d1);
        }
    }
    set_bits(m, t + L::FLAGS2, L::BLOCKED | L::SPENT);
    set_bits(m, t + L::FLAGS3, L::BLOCKED | L::SPENT);
    (true, d1)
}

fn set_bits(m: &mut impl Memory, a: u32, bits: u8) {
    let v = m.read_u8(a);
    m.write_u8(a, v | bits);
}

/// Resta `damage` a la vida de `v` (si llega a 0, muere) y da los puntos
/// al jugador de `t`. Original: `Hitbox_SubtractHP_0286ec`.
pub fn subtract_hp(m: &mut impl Memory, t: u32, v: u32, damage: u16) {
    let old = m.read_u16(v + L::HP);
    let hp = old.wrapping_sub(damage);
    // `bgt` tras la resta: comparación con signo de los valores originales.
    let hp = if i32::from(old as i16) - i32::from(damage as i16) <= 0 {
        set_bits(m, v + L::FLAGS2, L::DEAD);
        0
    } else {
        hp
    };
    m.write_u16(v + L::HP, hp);
    award_damage_points(m, t, v);
}

/// Puntos por golpear a un enemigo (no a los jugadores ni al Slug): según
/// la clase de ataque, al jugador dueño del ataque (`+$68` de `t`), si el
/// enemigo da puntos (bit 1 de `FLAGS`). Original: `$051914`.
pub fn award_damage_points(m: &mut impl Memory, t: u32, v: u32) {
    if v == PLAYER_ENTITY[0] || v == PLAYER_ENTITY[1] || v == L::SLUG {
        return;
    }
    let class = m.read_u8(v + L::ATTACK_CLASS);
    if class == 0 {
        return;
    }
    let player = m.read_u8(t + L::OWNER_PLAYER);
    if player & 0x80 != 0 {
        return;
    }
    m.write_u8(v + L::LAST_PLAYER, player);
    if m.read_u8(v + T::FLAGS) & L::GIVES_POINTS == 0 {
        return;
    }
    let score = L::SCORES[usize::from(player != 0)];
    let points = L::DAMAGE_POINTS + u32::from(class & 0x1F) * 4;
    add_score(m, score, points);
}

/// Suma a la puntuación `score` (8 dígitos, uno por byte) los puntos BCD
/// de `points` (4 bytes), con tope 99 999 999.
/// Original: `$051862` + `$051A10` + `$051828`.
pub fn add_score(m: &mut impl Memory, score: u32, points: u32) {
    let mut packed = [0u8; 4];
    for (i, p) in packed.iter_mut().enumerate() {
        let hi = m.read_u8(score + i as u32 * 2);
        let lo = m.read_u8(score + i as u32 * 2 + 1);
        *p = (hi << 4) | lo;
    }
    let mut carry = false;
    for i in (0..4).rev() {
        let (s, c) = bcd_add(packed[i], m.read_u8(points + i as u32), carry);
        packed[i] = s;
        carry = c;
    }
    if carry {
        packed = [0x99; 4];
    }
    for (i, p) in packed.iter().enumerate() {
        m.write_u8(L::SCORE_SCRATCH + i as u32, *p);
    }
    for (i, p) in packed.iter().enumerate() {
        m.write_u8(score + i as u32 * 2, p >> 4);
        m.write_u8(score + i as u32 * 2 + 1, p & 0x0F);
    }
}

/// Suma BCD de un byte (`abcd`).
pub fn bcd_add(a: u8, b: u8, carry: bool) -> (u8, bool) {
    let mut lo = (a & 0x0F) + (b & 0x0F) + u8::from(carry);
    let mut hi = (a >> 4) + (b >> 4);
    if lo > 9 {
        lo -= 10;
        hi += 1;
    }
    let c = hi > 9;
    if c {
        hi -= 10;
    }
    ((hi << 4) | (lo & 0x0F), c)
}

/// Extensión horizontal (o vertical, `k` = 1) de una caja {izq, der} de la
/// entidad `e`, girada si ésta mira al otro lado.
fn span(m: &mut impl Memory, e: u32, rect: u32, k: u32) -> (u16, u16) {
    let lo = m.read_u16(rect + k * 4);
    let hi = m.read_u16(rect + k * 4 + 2);
    let flipped = m.read_u8(e + L::FLIP) & (1 << k) != 0;
    let (lo, hi) = if flipped { (hi.wrapping_neg(), lo.wrapping_neg()) } else { (lo, hi) };
    let base = m.read_u16(e + L::POS[k as usize]);
    (lo.wrapping_add(base), hi.wrapping_add(base))
}

/// ¿Se solapan la caja `ra` de `t` y la `rv` de `v`? Si sí, deja en la
/// víctima el centro del solapamiento (`IMPACT`). Original:
/// `Hitbox_OverlapRect_02876e`.
pub fn overlap_rect(m: &mut impl Memory, t: u32, ra: u32, v: u32, rv: u32) -> bool {
    let mut center = [0u16; 2];
    for k in 0..2 {
        let (a0, a1) = span(m, t, ra, k);
        let (v0, v1) = span(m, v, rv, k);
        if (v0 as i16) > (a1 as i16) || (a0 as i16) > (v1 as i16) {
            return false;
        }
        // Las comparaciones del original aquí son sin signo.
        let lo = if a0 < v0 { v0 } else { a0 };
        let hi = if a1 >= v1 { v1 } else { a1 };
        center[k as usize] = (lo.wrapping_add(hi) as i16 >> 1) as u16;
    }
    m.write_u16(v + L::IMPACT[0], center[0]);
    m.write_u16(v + L::IMPACT[1], center[1]);
    true
}

/// ¿Cae el punto {dx, -, dy} de `t` dentro de la caja `rv` de `v`?
/// Original: `Hitbox_OverlapPoint_028886`.
pub fn overlap_point(m: &mut impl Memory, t: u32, pa: u32, v: u32, rv: u32) -> bool {
    let mut p = [0u16; 2];
    for k in 0..2 {
        let x = m.read_u16(pa + k * 4).wrapping_add(m.read_u16(t + L::POS[k as usize]));
        let (v0, v1) = span(m, v, rv, k);
        if (v0 as i16) > (x as i16) || (x as i16) > (v1 as i16) {
            return false;
        }
        p[k as usize] = x;
    }
    m.write_u16(v + L::IMPACT[0], p[0]);
    m.write_u16(v + L::IMPACT[1], p[1]);
    true
}

/// ¿Está `t` dentro de la pantalla ampliada con los márgenes `margins`
/// ({izq, der, abajo, arriba}; por defecto 1 píxel)? Las cajas con el
/// byte `+6` distinto de 0 valen siempre. Original: `Hitbox_OnScreen_028930`.
pub fn on_screen(m: &mut impl Memory, t: u32, a: u32, margins: Option<u32>) -> bool {
    if m.read_u8(a + L::ALWAYS) != 0 {
        return true;
    }
    let mg = margins.unwrap_or(L::DEFAULT_MARGINS);
    let left = m.read_u16(mg) as i16;
    let right = (L::SCREEN.1 as i16).wrapping_add(m.read_u16(mg + 2) as i16);
    let bottom = (L::SCREEN.2 as i16).wrapping_sub(m.read_u16(mg + 6) as i16);
    let top = (L::SCREEN.3 as i16).wrapping_sub(m.read_u16(mg + 4) as i16);
    let x = m.read_u16(t + L::POS[0]) as i16;
    let y = m.read_u16(t + L::POS[1]) as i16;
    (left..=right).contains(&x) && (bottom..=top).contains(&y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bcd() {
        assert_eq!(bcd_add(0x19, 0x01, false), (0x20, false));
        assert_eq!(bcd_add(0x99, 0x01, false), (0x00, true));
        assert_eq!(bcd_add(0x45, 0x54, true), (0x00, true));
    }
}
