//! Lectura de los mandos.
//!
//! Reescrito a partir de `asm/input/` de la decompilación
//! (`InputEvtThunk_*`, `InputMask_CheckChannelAvail`,
//! `InputMask_ReadCtxSwitchPlayer`, `InputMask_TestChannelBit`,
//! `InputCtx_DemoOverride`, `InputLayout_ReadField2`).
//!
//! Cada entidad controlable dice qué mando la controla (`CONTROLLER`). El
//! estado de cada mando es una copia de 6 bytes por frame, con los botones
//! **mantenidos** y los **recién pulsados**. Mientras el piloto automático
//! de un tramo de misión conduce a los jugadores, se leen sus mandos
//! sintéticos en lugar de los reales.

use crate::entity::{Entity, PlayerId};
use crate::layout::{self, input as L};
use crate::mem::Memory;

/// Botones (bits del byte de estado del mando).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Buttons(pub u8);

impl Buttons {
    pub const UP: Buttons = Buttons(0x01);
    pub const DOWN: Buttons = Buttons(0x02);
    pub const LEFT: Buttons = Buttons(0x04);
    pub const RIGHT: Buttons = Buttons(0x08);
    pub const A: Buttons = Buttons(0x10);
    pub const B: Buttons = Buttons(0x20);
    pub const C: Buttons = Buttons(0x40);
    pub const D: Buttons = Buttons(0x80);
    pub const LEFT_OR_RIGHT: Buttons = Buttons(0x0C);

    pub const fn or(self, o: Buttons) -> Buttons {
        Buttons(self.0 | o.0)
    }
}

/// Qué estado del botón se consulta.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edge {
    /// Mantenido este frame.
    Held,
    /// Pulsado este frame (no lo estaba en el anterior).
    Pressed,
}

impl Edge {
    fn offset(self) -> u32 {
        match self {
            Edge::Held => L::PAD_HELD,
            Edge::Pressed => L::PAD_PRESSED,
        }
    }
}

/// Durante el arranque de una escena la entrada se ignora un rato (salvo
/// que el modo actual pida leerla siempre).
pub fn input_suspended(m: &mut impl Memory) -> bool {
    m.read_u8(L::INPUT_GATE) & L::INPUT_GATE_ALWAYS == 0 && m.read_u8(L::INPUT_HOLDOFF_TIMER) != 0
}

/// Mando (bloque de estado) que lee la entidad `e`, o `None` si no la
/// controla ningún jugador (equivale a un mando sin nada pulsado).
fn pad_of(m: &mut impl Memory, e: Entity) -> Option<u32> {
    let mut pad = None;
    match e.controller(m) {
        1 => pad = Some(m.read_u32(L::PLAYER_TASK[0] + L::PAD_PTR) & 0xFF_FFFF),
        2 => pad = Some(m.read_u32(L::PLAYER_TASK[1] + L::PAD_PTR) & 0xFF_FFFF),
        _ => {}
    }
    if m.read_u8(L::SCRIPTED_INPUT_ACTIVE) != 0 {
        // El piloto automático sustituye el mando: el de P1 si la entidad
        // leía el de P1 y el de P2 en cualquier otro caso.
        pad = Some(if pad == Some(L::PAD_LIVE[0]) { L::PAD_SCRIPTED[0] } else { L::PAD_SCRIPTED[1] });
    }
    pad
}

fn pad_byte(m: &mut impl Memory, pad: Option<u32>, edge: Edge) -> u8 {
    match pad {
        Some(p) if layout::is_ram(p) => m.read_u8(p + edge.offset()),
        // Sin mando: el original apunta a una tabla de ceros de la ROM.
        _ => 0,
    }
}

/// ¿Está alguno de `b` mantenido/pulsado en el mando que controla `e`?
/// Original: `InputEvtThunk_*` sin mando fijo + `InputMask_CheckChannelAvail`.
pub fn pressed(m: &mut impl Memory, e: Entity, b: Buttons, edge: Edge) -> bool {
    if input_suspended(m) {
        return false;
    }
    let pad = pad_of(m, e);
    pad_byte(m, pad, edge) & b.0 != 0
}

/// Bits de `b` mantenidos/pulsados en el mando que controla `e` (`None` si
/// ninguno). Original: `$05CFA8` (C = alguno; el byte queda en `d0`).
pub fn pressed_bits(m: &mut impl Memory, e: Entity, b: Buttons, edge: Edge) -> Option<u8> {
    if input_suspended(m) {
        return None;
    }
    let pad = pad_of(m, e);
    let v = pad_byte(m, pad, edge) & b.0;
    (v != 0).then_some(v)
}

/// ¿Están todos los de `b` mantenidos/pulsados en el mando que controla
/// `e`? Original: `$05D00E` (y hermanas `$05D01A`, `$05D026`).
pub fn all_pressed(m: &mut impl Memory, e: Entity, b: Buttons, edge: Edge) -> bool {
    if input_suspended(m) {
        return false;
    }
    let pad = pad_of(m, e);
    pad_byte(m, pad, edge) & b.0 == b.0
}

/// Igual que [`pressed`] pero con el mando real de un jugador concreto (sin
/// pausa de arranque ni piloto automático). Original: `InputEvtThunk_*` con
/// mando fijo + `InputMask_TestChannelBit`.
pub fn player_pad_pressed(m: &mut impl Memory, p: PlayerId, b: Buttons, edge: Edge) -> bool {
    m.read_u8(L::PAD_LIVE[p.index()] + edge.offset()) & b.0 != 0
}

/// Truco de depuración: arriba mantenido y D recién pulsado en el mando
/// real del jugador 1. Original: `InputEvt_ToggleChain_05d288`.
pub fn debug_skip(m: &mut impl Memory) -> bool {
    player_pad_pressed(m, PlayerId::P1, Buttons::UP, Edge::Held)
        && player_pad_pressed(m, PlayerId::P1, Buttons::D, Edge::Pressed)
}

/// Dirección horizontal mantenida en el mando de `e`: 0 ninguna, 1
/// izquierda, 2 derecha, 3 ambas. Original: `InputLayout_ReadField2`.
pub fn horizontal_held(m: &mut impl Memory, e: Entity) -> u8 {
    if input_suspended(m) {
        return 0;
    }
    let pad = pad_of(m, e);
    (pad_byte(m, pad, Edge::Held) & Buttons::LEFT_OR_RIGHT.0) >> 2
}

/// Configuración de controles: `true` si se salta con B (si no, con
/// arriba).
pub fn jump_on_button_b(m: &mut impl Memory) -> bool {
    matches!(m.read_u8(L::CONTROL_LAYOUT), 0 | 4)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::TestRam;

    #[test]
    fn controller_and_script_override() {
        let mut m = TestRam::default();
        let e = Entity::player(PlayerId::P1);
        e.set_controller(&mut m, 1);
        m.write_u32(L::PLAYER_TASK[0] + L::PAD_PTR, L::PAD_LIVE[0]);
        m.write_u8(L::INPUT_GATE, L::INPUT_GATE_ALWAYS);
        m.write_u8(L::PAD_LIVE[0] + L::PAD_HELD, Buttons::RIGHT.0);
        assert!(pressed(&mut m, e, Buttons::RIGHT, Edge::Held));
        assert!(!pressed(&mut m, e, Buttons::RIGHT, Edge::Pressed));
        assert_eq!(horizontal_held(&mut m, e), 2);
        m.write_u8(L::SCRIPTED_INPUT_ACTIVE, 1);
        assert!(!pressed(&mut m, e, Buttons::RIGHT, Edge::Held));
        m.write_u8(L::PAD_SCRIPTED[0] + L::PAD_HELD, Buttons::LEFT.0);
        assert_eq!(horizontal_held(&mut m, e), 1);
    }
}
