//! Funciones de Metal Slug portadas a Rust.
//!
//! Cada función reproduce 1:1 una rutina 68000 de la P-ROM (`201-p1.bin`):
//! mismos efectos en registros, flags (X N Z V C), memoria y ciclos. El
//! nombre y la dirección vienen del proyecto de decompilación
//! (`tools/registry.py` / `asm/` en la rama principal).
//!
//! Para añadir una función: escribirla en el submódulo temático, añadirla a
//! `FUNCTIONS` y pasar `mslug verify`.

pub mod cc;
mod kernel;
mod entity;
mod hiscore;

use neogeo_hw::native::{NativeFn, NativeTable};

/// (dirección de entrada, nombre en la decompilación, implementación)
pub const FUNCTIONS: &[(u32, &str, NativeFn)] = &[
    (0x001E56, "Store106EDA_001e56", kernel::store_106eda_001e56),
    (0x05E9E4, "RNG_LFSRStep_05E9E4", kernel::rng_lfsr_step_05e9e4),
    (0x043EDA, "Buffer_ClearBlock1024L_043EDA", kernel::buffer_clear_block_1024l_043eda),
    (0x044048, "Coord_ApplyCameraDeltaToSelf_044048", entity::coord_apply_camera_delta_to_self_044048),
    (0x04405E, "Coord_ApplyCameraDeltaToA0Mark_04405E", entity::coord_apply_camera_delta_to_a0_mark_04405e),
    (0x04407A, "Coord_ApplyCameraTerciaryToSelf_04407A", entity::coord_apply_camera_terciary_to_self_04407a),
    (0x044090, "Coord_ApplyCameraDeltaToGlobals_044090", entity::coord_apply_camera_delta_to_globals_044090),
    (0x0440AA, "Coord_ApplyCameraDeltaToD1D2_0440AA", entity::coord_apply_camera_delta_to_d1d2_0440aa),
    (0x0440BC, "Coord_LocalToScreenSecondary_0440BC", entity::coord_local_to_screen_secondary_0440bc),
    (0x0440D0, "Coord_ScreenToLocalSecondary_0440D0", entity::coord_screen_to_local_secondary_0440d0),
    (0x065C64, "Hostage_SinCos_065c64", entity::hostage_sincos_065c64),
    (0x02A2AC, "Slug_GetGauge_02a2ac", entity::slug_get_gauge_02a2ac),
    (0x05180C, "Clear8Bytes_05180c", entity::clear8bytes_05180c),
    (0x0516BA, "Players_SnapshotMods_0516ba", entity::players_snapshot_mods_0516ba),
    (0x09773C, "HiScore_LoadDefaults_09773c", hiscore::hiscore_load_defaults_09773c),
];

pub fn register(t: &mut NativeTable) {
    for (addr, _, f) in FUNCTIONS {
        t.insert(*addr, *f);
    }
}
