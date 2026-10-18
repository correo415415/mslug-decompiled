//! Tabla de récords.

use super::cc::*;
use neogeo_hw::cpu::m68k::{Cpu, Size::*};
use neogeo_hw::native::DynBus;

/// Tabla de récords por defecto en ROM ($2F53B2): 10 entradas de 14 B.
const DEFAULTS: u32 = 0x2F53B2;
/// Destino en RAM de respaldo: `$100002`.
const TABLE: u32 = 0x100002;

/// $09773C `HiScore_LoadDefaults`: copia las 10 entradas por defecto.
/// Cada entrada: 2 bytes, 1 long, 3 words (la ROM lo hace con 6 `move`
/// post-incremento por entrada; se respeta el orden y los anchos).
pub fn hiscore_load_defaults_09773c(cpu: &mut Cpu, bus: &mut DynBus) -> u32 {
    let (mut s, mut d) = (DEFAULTS, TABLE);
    let mut last = 0;
    let mut cyc = 2 * (4 + ea(EA_ABSL, Long));
    for _ in 0..10 {
        for sz in [Byte, Byte, Long, Word, Word, Word] {
            last = rd(bus, s, sz);
            wr(bus, d, last, sz);
            let n = sz.bytes();
            s += n; d += n;
            cyc += 4 + 2 * ea(EA_IND, sz);
        }
    }
    cpu.a[0] = s; cpu.a[1] = d;
    logic(cpu, last, Word);
    cyc + rts(cpu, bus)
}
