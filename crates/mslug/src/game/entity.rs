//! Entidades y coordenadas de cámara.

use super::cc::*;
use neogeo_hw::cpu::m68k::{Cpu, Size::*, StatusRegister as SR};
use neogeo_hw::native::DynBus;

/// Cámara: delta de scroll del frame (`$106F6C` X, `$106F6E` Y).
const CAM_DX: u32 = 0x106F6C;
const CAM_DY: u32 = 0x106F6E;

/// `x -= dx ; y += dy` sobre `+$22/+$24` de la entidad en `base`.
fn apply_delta(cpu: &mut Cpu, bus: &mut DynBus, base: u32, dx: u32, dy: u32) -> u32 {
    let d = r16(bus, dx);
    set_d(cpu, 0, d, Word); logic(cpu, d, Word);
    let x = sub(cpu, r16(bus, base + 0x22), d, Word, true);
    w16(bus, base + 0x22, x);
    let d = r16(bus, dy);
    set_d(cpu, 0, d, Word); logic(cpu, d, Word);
    let y = add(cpu, r16(bus, base + 0x24), d, Word);
    w16(bus, base + 0x24, y);
    2 * (4 + ea(EA_ABSL, Word)) + 2 * (4 + ea(EA_D16, Word))
}

/// $044048 `Coord_ApplyCameraDeltaToSelf`: a6.x -= dx, a6.y += dy.
pub fn coord_apply_camera_delta_to_self_044048(cpu: &mut Cpu, bus: &mut DynBus) -> u32 {
    let c = apply_delta(cpu, bus, cpu.a[6], CAM_DX, CAM_DY);
    c + rts(cpu, bus)
}

/// $04405E `Coord_ApplyCameraDeltaToA0Mark`: igual sobre a0 y marca
/// `bset #6,$13(a0)` (Z = bit anterior a 0).
pub fn coord_apply_camera_delta_to_a0_mark_04405e(cpu: &mut Cpu, bus: &mut DynBus) -> u32 {
    let base = cpu.a[0];
    let c = apply_delta(cpu, bus, base, CAM_DX, CAM_DY);
    let f = r8(bus, base + 0x13);
    cpu.sr.set(SR::Z, f & 0x40 == 0);
    w8(bus, base + 0x13, f | 0x40);
    c + 8 + ea(EA_D16, Byte) + rts(cpu, bus)
}

/// $04407A `Coord_ApplyCameraTerciaryToSelf`: delta de la cámara 3
/// (`$108064/$108066`) sobre a6.
pub fn coord_apply_camera_terciary_to_self_04407a(cpu: &mut Cpu, bus: &mut DynBus) -> u32 {
    let c = apply_delta(cpu, bus, cpu.a[6], 0x108064, 0x108066);
    c + rts(cpu, bus)
}

/// $044090 `Coord_ApplyCameraDeltaToGlobals`: lo mismo sobre las globales
/// `$106F38/$106F3A`.
pub fn coord_apply_camera_delta_to_globals_044090(cpu: &mut Cpu, bus: &mut DynBus) -> u32 {
    let d = r16(bus, CAM_DX);
    set_d(cpu, 0, d, Word); logic(cpu, d, Word);
    let x = sub(cpu, r16(bus, 0x106F38), d, Word, true);
    w16(bus, 0x106F38, x);
    let d = r16(bus, CAM_DY);
    set_d(cpu, 0, d, Word); logic(cpu, d, Word);
    let y = add(cpu, r16(bus, 0x106F3A), d, Word);
    w16(bus, 0x106F3A, y);
    4 * (4 + ea(EA_ABSL, Word)) + rts(cpu, bus)
}

/// $0440AA `Coord_ApplyCameraDeltaToD1D2`: d1 += dx, d2 -= dy (inverso).
pub fn coord_apply_camera_delta_to_d1d2_0440aa(cpu: &mut Cpu, bus: &mut DynBus) -> u32 {
    let d = r16(bus, CAM_DX);
    set_d(cpu, 0, d, Word); logic(cpu, d, Word);
    let v = add(cpu, cpu.d[1], d, Word); set_d(cpu, 1, v, Word);
    let d = r16(bus, CAM_DY);
    set_d(cpu, 0, d, Word); logic(cpu, d, Word);
    let v = sub(cpu, cpu.d[2], d, Word, true); set_d(cpu, 2, v, Word);
    2 * (4 + ea(EA_ABSL, Word)) + 2 * 4 + rts(cpu, bus)
}

/// $0440BC `Coord_LocalToScreenSecondary`: d0 += camX ; d1 = camY - (d1 - $200).
pub fn coord_local_to_screen_secondary_0440bc(cpu: &mut Cpu, bus: &mut DynBus) -> u32 {
    let v = add(cpu, cpu.d[0], r16(bus, 0x106F50), Word); set_d(cpu, 0, v, Word);
    let v = sub(cpu, cpu.d[1], 0x200, Word, true); set_d(cpu, 1, v, Word);
    let v = neg(cpu, v, Word); set_d(cpu, 1, v, Word);
    let v = add(cpu, v, r16(bus, 0x106F54), Word); set_d(cpu, 1, v, Word);
    2 * (4 + ea(EA_ABSL, Word)) + 8 + 4 + rts(cpu, bus)
}

/// $0440D0 `Coord_ScreenToLocalSecondary`: inversa de la anterior.
pub fn coord_screen_to_local_secondary_0440d0(cpu: &mut Cpu, bus: &mut DynBus) -> u32 {
    let v = sub(cpu, cpu.d[0], r16(bus, 0x106F50), Word, true); set_d(cpu, 0, v, Word);
    let v = sub(cpu, cpu.d[1], r16(bus, 0x106F54), Word, true); set_d(cpu, 1, v, Word);
    let v = neg(cpu, v, Word); set_d(cpu, 1, v, Word);
    let v = add(cpu, v, 0x200, Word); set_d(cpu, 1, v, Word);
    2 * (4 + ea(EA_ABSL, Word)) + 4 + 8 + rts(cpu, bus)
}

/// $065C64 `Hostage_SinCos`: velocidad polar del rehén. Ángulo `+$88`
/// (índice de 16 B en `SinTab256` $2C072C, coseno a +$80), módulo `+$36`:
/// `vx (+$2A) = sin·r >> 8`, `vy (+$28) = cos·r >> 8`.
pub fn hostage_sincos_065c64(cpu: &mut Cpu, bus: &mut DynBus) -> u32 {
    let e = cpu.a[6];
    let mut cyc = 0;
    let ang = r16(bus, e + 0x88);
    set_d(cpu, 0, ang, Word); logic(cpu, ang, Word); cyc += 4 + ea(EA_D16, Word);
    let off = lsl(cpu, ang, 4, Word); set_d(cpu, 0, off, Word); cyc += shift_cycles(4);
    cpu.a[1] = 0x2C072C; cpu.a[2] = 0x2C07AC; cyc += 2 * (4 + ea(EA_ABSL, Long));
    let s = r16(bus, 0x2C072C + idx_w(off)); set_d(cpu, 1, s, Word); logic(cpu, s, Word);
    let c = r16(bus, 0x2C07AC + idx_w(off)); set_d(cpu, 2, c, Word); logic(cpu, c, Word);
    cyc += 2 * (4 + ea(EA_IDX, Word));
    let r = r16(bus, e + 0x36);
    set_d(cpu, 0, r, Word); logic(cpu, r, Word); cyc += 4 + ea(EA_D16, Word);
    let rs = r as u16 as i16 as i32;
    let p1 = ((s as u16 as i16 as i32) * rs) as u32; cpu.d[1] = p1; logic(cpu, p1, Long);
    let p2 = ((c as u16 as i16 as i32) * rs) as u32; cpu.d[2] = p2; logic(cpu, p2, Long);
    cyc += 2 * 70;
    let vx = asr(cpu, p1, 8, Long); cpu.d[1] = vx;
    let vy = asr(cpu, p2, 8, Long); cpu.d[2] = vy;
    cyc += 2 * shift_cycles(8);
    w16(bus, e + 0x2A, vx); logic(cpu, vx, Word);
    w16(bus, e + 0x28, vy); logic(cpu, vy, Word);
    cyc += 2 * (4 + ea(EA_D16, Word));
    cyc + rts(cpu, bus)
}

/// $02A2AC `Slug_GetGauge`: a0 = Slug ($100580), d0 = medidor (+$90).
pub fn slug_get_gauge_02a2ac(cpu: &mut Cpu, bus: &mut DynBus) -> u32 {
    cpu.a[0] = 0x100580;
    let g = r8(bus, 0x100580 + 0x90);
    cpu.d[0] = g; logic(cpu, g, Byte);
    (4 + ea(EA_ABSL, Long)) + 4 + (4 + ea(EA_D16, Byte)) + rts(cpu, bus)
}

/// $05180C `Clear8Bytes`: 8 bytes a 0 en (a1).
pub fn clear8bytes_05180c(cpu: &mut Cpu, bus: &mut DynBus) -> u32 {
    let a = cpu.a[1];
    w32(bus, a, 0); w32(bus, a + 4, 0);
    logic(cpu, 0, Long);
    (4 + ea(EA_IND, Long)) + (4 + ea(EA_D16, Long)) + rts(cpu, bus)
}

/// $0516BA `Players_SnapshotMods`: copia `$106ED0`, `$10FDB6`, `$10FDB7`
/// (modos de jugador del BIOS) a `$10E3B7..$10E3B9`.
pub fn players_snapshot_mods_0516ba(cpu: &mut Cpu, bus: &mut DynBus) -> u32 {
    cpu.a[4] = 0x10E3A2;
    let mut last = 0;
    for (src, off) in [(0x106ED0, 0x15), (0x10FDB6, 0x16), (0x10FDB7, 0x17)] {
        last = r8(bus, src);
        w8(bus, 0x10E3A2 + off, last);
    }
    logic(cpu, last, Byte);
    (4 + ea(EA_ABSL, Long)) + 3 * (4 + ea(EA_ABSL, Byte) + ea(EA_D16, Byte)) + rts(cpu, bus)
}
