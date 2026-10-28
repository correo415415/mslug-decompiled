//! Puente entre el juego escrito a mano (`mslug-game`) y el programa
//! original (ROM / programa recompilado).
//!
//! **Único sitio que relaciona el Rust legible con las rutinas originales**:
//! a cada función de `mslug-game` le corresponde la dirección de entrada de
//! la rutina original y un adaptador que traduce los registros de la llamada
//! a los argumentos Rust ([`PORTS`]).
//!
//! Dos usos:
//! - `mslug verify-manual`: comprobar que cada función Rust hace lo mismo que
//!   la ROM (ver `crates/mslug/src/manual.rs`).
//! - [`install`]: **sustituir** las rutinas originales por las Rust en el
//!   programa que se ejecuta (`mslug-sdl --manual`, `mslug play --manual`),
//!   para jugar con el port escrito a mano.

use mslug_game::input::{self, Buttons, Edge};
use mslug_game::player::{controls, invuln, voice, weapons};
use mslug_game::{Entity, Memory, PlayerId};
use neogeo_hw::cpu::m68k::{Bus, Cpu, StatusRegister};
use neogeo_hw::NeoGeoBus;
use std::collections::HashMap;
use std::sync::OnceLock;

pub mod symbols;
pub mod system;

/// RAM de trabajo vista como `Memory` del juego portado.
pub struct Ram<'a>(pub &'a mut [u8]);

impl Memory for Ram<'_> {
    fn read_u8(&mut self, addr: u32) -> u8 {
        self.0[(addr & 0xFFFF) as usize]
    }
    fn write_u8(&mut self, addr: u32, v: u8) {
        self.0[(addr & 0xFFFF) as usize] = v;
    }
}

/// Resultado observable de una llamada (además de la RAM).
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Ret {
    None,
    /// Valor en d0 (word).
    D0W(u16),
    /// Valor en d0 (byte).
    D0B(u8),
    /// Éxito / fallo (flag C del original: C=0 éxito).
    Ok(bool),
    /// Sí / no (flag C del original: C=1 sí).
    Yes(bool),
    /// Sí / no (C=1) y valor en d0 (word).
    YesD0W(bool, u16),
    /// Valor en d1 (byte).
    D1B(u8),
    /// Valor en todo d0 (el original lo pone a 0 con `moveq` antes de
    /// escribir el byte/word del resultado, y los llamadores usan el word
    /// o el long entero).
    D0L(u32),
    /// Sí / no (C=1) y valor en todo d0.
    YesD0L(bool, u32),
    /// Puntero en a1 (p. ej. la entidad del jugador, que el llamador usa
    /// después).
    A1(u32),
}

/// Efectos de recogida: en la verificación se anotan, y la comprobación
/// exige que la ROM haya llamado a la misma rutina (ver `expect_calls`).
#[derive(Default)]
pub struct Fx {
    pub calls: Vec<(&'static str, u32)>,
}

impl<M: Memory> weapons::PickupEffects<M> for Fx {
    fn score_popup(&mut self, _: &mut M, _: PlayerId) {
        self.calls.push(("Score_Popup_ForPlayer", 0));
    }
    fn weapon_pickup_marker(&mut self, _: &mut M, _: PlayerId) {
        self.calls.push(("Entity_AllocByPlayerSlot", 0));
    }
}

impl<M: Memory> voice::Sound<M> for Fx {
    fn play(&mut self, _: &mut M, code: u16) {
        self.calls.push(("Sound_Request", u32::from(code)));
    }
}

/// Adaptador: llama a la función Rust con los argumentos que la rutina
/// original recibe en registros.
pub type Adapter = fn(&Cpu, &mut Ram, &mut Fx) -> Ret;

/// Rutinas que llaman las funciones portadas y que por ahora siguen en la ROM
/// (se comparan como "llamadas", no se ejecutan en el lado Rust).
/// El último campo extrae el argumento que se compara (0 = ninguno).
pub type ArgFn = fn(&Cpu) -> u32;
pub const CALLEES: &[(u32, &str, ArgFn)] = &[
    (0x09_B9BA, "Score_Popup_ForPlayer", |_| 0),
    (0x09_B9F6, "Entity_AllocByPlayerSlot", |_| 0),
    (0x00_2352, "Sound_Request", |c| c.d[0] & 0xFFFF),
];

pub struct Port {
    pub entry: u32,
    pub original: &'static str,
    pub rust: &'static str,
    pub run: Adapter,
}

pub fn player_of(d0: u32) -> PlayerId {
    PlayerId::from_index(d0 as u8)
}

/// `InputEvtThunk_*` sin mando fijo → `input::pressed`.
macro_rules! evt {
    ($a:expr, $mask:expr, $edge:ident) => {
        Port { entry: $a, original: concat!("InputEvtThunk_", stringify!($a)), rust: "input::pressed",
               run: |c, m, _| Ret::Yes(input::pressed(m, e6(c), Buttons($mask), Edge::$edge)) }
    };
}

/// `InputEvtThunk_*` con mando fijo → `input::player_pad_pressed`.
macro_rules! pad {
    ($a:expr, $p:ident, $mask:expr, $edge:ident) => {
        Port { entry: $a, original: concat!("InputEvtThunk_", stringify!($a)), rust: "input::player_pad_pressed",
               run: |_, m, _| Ret::Yes(input::player_pad_pressed(m, PlayerId::$p, Buttons($mask), Edge::$edge)) }
    };
}

pub const PORTS: &[Port] = &[
    Port { entry: 0x03_2B58, original: "Player_SetWeaponAndAmmo", rust: "player::weapons::set_weapon_and_ammo",
           run: |c, m, _| {
               weapons::set_weapon_and_ammo(m, player_of(c.d[0]), c.d[1] as u8, c.d[2] as u8);
               Ret::A1(Entity::player(player_of(c.d[0])).raw())
           } },
    Port { entry: 0x03_2BC6, original: "Player_RefillAmmoClamp999", rust: "player::weapons::refill_ammo",
           run: |c, m, _| { weapons::refill_ammo(m, player_of(c.d[0])); Ret::None } },
    Port { entry: 0x03_2C12, original: "Item_GiveAmmo_ToPlayer", rust: "player::weapons::give_ammo_from_item",
           run: |c, m, fx| { weapons::give_ammo_from_item(m, fx, player_of(c.d[0]), Entity::from_raw(c.a[6])); Ret::None } },
    Port { entry: 0x03_2C7E, original: "Item_GiveBombs_ToPlayer", rust: "player::weapons::give_bombs_from_item",
           run: |c, m, _| { weapons::give_bombs_from_item(m, player_of(c.d[0]), Entity::from_raw(c.a[6])); Ret::None } },
    Port { entry: 0x03_2FBA, original: "Player_GetAmmoOrFFFF", rust: "player::weapons::ammo_for_hud",
           run: |c, m, _| Ret::D0W(weapons::ammo_for_hud(m, Entity::from_raw(c.a[0]))) },
    Port { entry: 0x03_2FCE, original: "Player_GetBombs", rust: "Entity::bombs",
           run: |c, m, _| Ret::D0B(Entity::from_raw(c.a[0]).bombs(m)) },
    Port { entry: 0x03_2FD4, original: "Player_DecBombs", rust: "player::weapons::use_bomb",
           run: |c, m, _| Ret::Ok(weapons::use_bomb(m, Entity::from_raw(c.a[0]))) },
    Port { entry: 0x03_2CBA, original: "Player_StartInvuln5", rust: "player::invuln::start_pickup_blink",
           run: |c, m, _| { invuln::start_pickup_blink(m, Entity::from_raw(c.a[1])); Ret::None } },
    Port { entry: 0x03_2CC8, original: "Player_InvulnBlinkStep", rust: "player::invuln::blink_step",
           run: |c, m, _| { invuln::blink_step(m, Entity::from_raw(c.a[6])); Ret::None } },
    // --- player::voice
    Port { entry: 0x03_2D32, original: "Player_PlayWeaponMusicIfFlag", rust: "player::voice::announce_new_weapon",
           run: |c, m, fx| Ret::Yes(voice::announce_new_weapon(m, fx, e6(c))) },
    Port { entry: 0x03_2ED0, original: "Player_PlayLifeMusic", rust: "player::voice::play_life_sound",
           run: |c, m, fx| { voice::play_life_sound(m, fx, e6(c)); Ret::None } },
    // --- player::controls
    Port { entry: 0x03_2AEA, original: "Player_ConsumeFlag81", rust: "player::controls::grenade_limit_reached",
           run: |c, m, _| Ret::Yes(controls::grenade_limit_reached(m, e6(c))) },
    Port { entry: 0x03_2B1C, original: "Player_ConsumeFireFlag84", rust: "player::controls::shot_limit_reached",
           run: |c, m, _| Ret::Yes(controls::shot_limit_reached(m, e6(c))) },
    Port { entry: 0x03_2AFA, original: "Player_IncCounterAt81", rust: "player::controls::count_live_grenade",
           run: |c, m, _| { controls::count_live_grenade(m, e6(c)); Ret::None } },
    Port { entry: 0x03_2B36, original: "Player_IncCounterAt84", rust: "player::controls::count_live_shot",
           run: |c, m, _| { controls::count_live_shot(m, e6(c)); Ret::None } },
    Port { entry: 0x03_2E48, original: "Input_ForwardByFacing", rust: "player::controls::forward_held",
           run: |c, m, _| Ret::Yes(controls::forward_held(m, e6(c))) },
    Port { entry: 0x03_2E5A, original: "Input_BackwardByFacing", rust: "player::controls::backward_held",
           run: |c, m, _| Ret::Yes(controls::backward_held(m, e6(c))) },
    Port { entry: 0x03_2E90, original: "Input_DownPressed", rust: "player::controls::down_held",
           run: |c, m, _| Ret::Yes(controls::down_held(m, e6(c))) },
    Port { entry: 0x03_2E70, original: "Input_JumpOrFire", rust: "player::controls::up_jump_pressed",
           run: |c, m, _| Ret::Yes(controls::up_jump_pressed(m, e6(c))) },
    Port { entry: 0x03_2F3C, original: "Player_ReadDirNibble", rust: "player::controls::read_aim_airborne",
           run: |c, m, _| Ret::D0B(controls::read_aim_airborne(m, e6(c))) },
    Port { entry: 0x03_2F88, original: "Player_ReadFireNibble", rust: "player::controls::read_aim_grounded",
           run: |c, m, _| Ret::D0B(controls::read_aim_grounded(m, e6(c))) },
    Port { entry: 0x03_3172, original: "Player_DirNibbleToAction", rust: "player::controls::shot_action_for_aim",
           run: |c, m, _| Ret::D1B(controls::shot_action_for_aim(m, e6(c))) },
    Port { entry: 0x03_3034, original: "Input_FireByMode", rust: "player::controls::jump_held",
           run: |c, m, _| jump_ret(controls::jump_held(m, e6(c))) },
    Port { entry: 0x03_3080, original: "Input_JumpByMode", rust: "player::controls::jump_pressed",
           run: |c, m, _| jump_ret(controls::jump_pressed(m, e6(c))) },
    // --- input
    Port { entry: 0x05_D5B6, original: "InputLayout_ReadField2", rust: "input::horizontal_held",
           run: |c, m, _| Ret::D0L(u32::from(input::horizontal_held(m, e6(c)))) },
    evt!(0x05_CDA8, 0x10, Pressed), evt!(0x05_CDB4, 0x20, Pressed), evt!(0x05_CDC0, 0x40, Pressed),
    evt!(0x05_CDCC, 0x80, Pressed), evt!(0x05_CDD8, 0x30, Pressed), evt!(0x05_CDE4, 0x50, Pressed),
    evt!(0x05_CDF0, 0x70, Pressed), evt!(0x05_CDFC, 0xF0, Pressed), evt!(0x05_CE08, 0xE0, Pressed),
    evt!(0x05_CEEC, 0x01, Held), evt!(0x05_CEF8, 0x02, Held), evt!(0x05_CF04, 0x08, Held),
    evt!(0x05_CF10, 0x04, Held), evt!(0x05_CF1C, 0x0C, Held), evt!(0x05_CF2C, 0x09, Held),
    evt!(0x05_CF3C, 0x0A, Held), evt!(0x05_CF4C, 0x05, Held), evt!(0x05_CF5C, 0x06, Held),
    evt!(0x05_CF6C, 0x08, Pressed), evt!(0x05_CF78, 0x04, Pressed), evt!(0x05_CF84, 0x01, Pressed),
    evt!(0x05_CF90, 0x02, Pressed), evt!(0x05_CF9C, 0x0C, Pressed),
    pad!(0x05_CE14, P1, 0x10, Pressed), pad!(0x05_CE26, P1, 0x20, Pressed), pad!(0x05_CE38, P1, 0x40, Pressed),
    pad!(0x05_CE4A, P1, 0x80, Pressed), pad!(0x05_CE5C, P2, 0x10, Pressed), pad!(0x05_CE6E, P2, 0x20, Pressed),
    pad!(0x05_CE80, P2, 0x40, Pressed), pad!(0x05_CE92, P2, 0x80, Pressed),
    pad!(0x05_CEA4, P1, 0x01, Held), pad!(0x05_CEB6, P1, 0x02, Held), pad!(0x05_CEC8, P1, 0x08, Held),
    pad!(0x05_CEDA, P1, 0x04, Held),
];

pub fn e6(c: &Cpu) -> Entity {
    Entity::from_raw(c.a[6] & 0xFF_FFFF)
}

/// Salto: C=1 y d0 = dirección + 1, o C=0 y d0 = 0.
pub fn jump_ret(dir: Option<u8>) -> Ret {
    match dir {
        Some(d) => Ret::YesD0L(true, u32::from(d) + 1),
        None => Ret::YesD0L(false, 0),
    }
}

pub fn rom_ret(kind: Ret, cpu: &Cpu) -> Ret {
    match kind {
        Ret::None => Ret::None,
        Ret::D0W(_) => Ret::D0W(cpu.d[0] as u16),
        Ret::D0B(_) => Ret::D0B(cpu.d[0] as u8),
        Ret::Ok(_) => Ret::Ok(cpu.sr.0 & 1 == 0),
        Ret::Yes(_) => Ret::Yes(cpu.sr.0 & 1 != 0),
        Ret::YesD0W(..) => Ret::YesD0W(cpu.sr.0 & 1 != 0, cpu.d[0] as u16),
        Ret::D1B(_) => Ret::D1B(cpu.d[1] as u8),
        Ret::D0L(_) => Ret::D0L(cpu.d[0]),
        Ret::YesD0L(..) => Ret::YesD0L(cpu.sr.0 & 1 != 0, cpu.d[0]),
        Ret::A1(_) => Ret::A1(cpu.a[1]),
    }
}


// ---------------------------------------------------------------------------
// Sustitución en el programa que se ejecuta

/// Rutinas portadas que llaman a otras que siguen en la ROM (marcadores,
/// sonido...). Mientras esas no estén portadas, no se pueden sustituir: la
/// función Rust no puede llamar a la ROM a mitad de su ejecución.
pub const NEEDS_ROM_CALLEES: &[u32] = &[0x03_2C12, 0x03_2D32, 0x03_2ED0];

/// Ciclos que se cuentan por llamada a una función Rust (el original tarda
/// algo parecido; sólo afecta al reparto de tiempo dentro del frame).
pub const CYCLES_PER_CALL: u32 = 60;

/// Funciones Rust que pueden sustituir a su rutina original.
///
/// Para depurar, `MSLUG_MANUAL_ONLY=NOMBRE[,NOMBRE...]` limita la
/// sustitución a las rutinas originales cuyo nombre contenga alguno de esos
/// textos, y `MSLUG_MANUAL_SKIP=...` excluye las que lo contengan.
pub fn replaceable() -> impl Iterator<Item = &'static Port> {
    PORTS.iter().filter(|p| !NEEDS_ROM_CALLEES.contains(&p.entry) && selected(p.original))
}

/// Filtros de depuración `MSLUG_MANUAL_ONLY` / `MSLUG_MANUAL_SKIP`.
pub fn selected(original: &str) -> bool {
    static LISTS: OnceLock<(Vec<String>, Vec<String>)> = OnceLock::new();
    let (only, skip) = LISTS.get_or_init(|| {
        let list = |v: &str| -> Vec<String> {
            std::env::var(v).map(|s| s.split(',').filter(|x| !x.is_empty()).map(String::from).collect()).unwrap_or_default()
        };
        (list("MSLUG_MANUAL_ONLY"), list("MSLUG_MANUAL_SKIP"))
    });
    (only.is_empty() || only.iter().any(|o| original.contains(o.as_str())))
        && !skip.iter().any(|o| original.contains(o.as_str()))
}

/// Índice dirección de entrada -> función portada.
pub fn index() -> &'static HashMap<u32, &'static Port> {
    static IDX: OnceLock<HashMap<u32, &'static Port>> = OnceLock::new();
    IDX.get_or_init(|| replaceable().map(|p| (p.entry, p)).collect())
}

/// Ejecuta la función Rust que sustituye a la rutina que empieza en el PC
/// actual y vuelve al llamador (como el `rts` del original). Misma firma que
/// las instrucciones del programa recompilado (`mslug_code::InsFn`).
pub fn call_rust(cpu: &mut Cpu, bus: &mut NeoGeoBus) -> u32 {
    let pc = cpu.pc & 0xFF_FFFF;
    let p = index().get(&pc).expect("call_rust: dirección sin función portada");
    let cycles = if exact_cycles() { original_cycles(cpu, bus) } else { CYCLES_PER_CALL };
    let mut fx = Fx::default();
    let ret = (p.run)(cpu, &mut Ram(&mut bus.work_ram[..]), &mut fx);
    debug_assert!(fx.calls.is_empty(), "{}: llamó a la ROM", p.original);
    let ret_pc = bus.read32(cpu.a[7] & 0xFF_FFFF);
    apply_ret(cpu, ret, ret_pc);
    cycles
}

/// `MSLUG_MANUAL_EXACT_CYCLES=1` (depuración): cada llamada cuenta los
/// ciclos que habría tardado el original, para separar las diferencias de
/// tiempo de las de comportamiento.
fn exact_cycles() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("MSLUG_MANUAL_EXACT_CYCLES").is_some())
}

/// Referencia para depurar la sustitución: ejecuta la rutina **original**
/// entera en un solo paso (con el intérprete), igual de atómica que
/// [`call_rust`]. Si `lockstep-manual --atomic-ref` no encuentra
/// diferencias, las de la comparación normal son sólo de tiempo.
pub fn call_rom_atomic(cpu: &mut Cpu, bus: &mut NeoGeoBus) -> u32 {
    let ret_sp = cpu.a[7].wrapping_add(4);
    let ret_pc = bus.read32(cpu.a[7] & 0xFF_FFFF) & 0xFF_FFFF;
    let mut c = 0u32;
    for _ in 0..1_000_000 {
        c += cpu.step(bus);
        if cpu.pc & 0xFF_FFFF == ret_pc && cpu.a[7] == ret_sp {
            break;
        }
    }
    c
}

/// Ciclos del original: lo ejecuta con el intérprete hasta su `rts` y
/// deshace los efectos (CPU y RAM de trabajo).
fn original_cycles(cpu: &mut Cpu, bus: &mut NeoGeoBus) -> u32 {
    let cpu0 = cpu.clone();
    let ram0 = bus.work_ram.clone();
    let ret_sp = cpu.a[7].wrapping_add(4);
    let ret_pc = bus.read32(cpu.a[7] & 0xFF_FFFF) & 0xFF_FFFF;
    let mut c = 0u32;
    for _ in 0..1_000_000 {
        c += cpu.step(bus);
        if cpu.pc & 0xFF_FFFF == ret_pc && cpu.a[7] == ret_sp {
            break;
        }
    }
    *cpu = cpu0;
    bus.work_ram = ram0;
    c
}

/// Deja la CPU como la deja la rutina original al volver: resultado en
/// d0/d1 o en el flag C (y X, que el original pone igual), y `rts` a
/// `ret_pc` (la dirección de retorno que está en la pila).
pub fn apply_ret(cpu: &mut Cpu, ret: Ret, ret_pc: u32) {
    let flag = |cpu: &mut Cpu, on: bool| cpu.sr.set(StatusRegister::C | StatusRegister::X, on);
    match ret {
        Ret::None => {}
        Ret::D0W(v) => cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | u32::from(v),
        Ret::D0B(v) => cpu.d[0] = (cpu.d[0] & 0xFFFF_FF00) | u32::from(v),
        Ret::D1B(v) => cpu.d[1] = (cpu.d[1] & 0xFFFF_FF00) | u32::from(v),
        Ret::Ok(ok) => flag(cpu, !ok),
        Ret::Yes(y) => flag(cpu, y),
        Ret::YesD0W(y, v) => {
            cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | u32::from(v);
            flag(cpu, y);
        }
        Ret::D0L(v) => cpu.d[0] = v,
        Ret::A1(v) => cpu.a[1] = v,
        Ret::YesD0L(y, v) => {
            cpu.d[0] = v;
            flag(cpu, y);
        }
    }
    cpu.pc = ret_pc;
    cpu.a[7] = cpu.a[7].wrapping_add(4);
}

/// Sustituye todas las rutinas que tienen función Rust: llama a
/// `set(dirección, call_rust)` por cada una (p. ej. con
/// `|a, f| prog.set_override(a, f)` sobre `mslug_code::Program`). Devuelve
/// cuántas.
pub fn install(set: impl FnMut(u32, fn(&mut Cpu, &mut NeoGeoBus) -> u32)) -> usize {
    install_with(set, call_rust)
}

/// Como [`install`], pero con otra función en cada entrada (p. ej.
/// [`call_rom_atomic`]).
pub fn install_with(
    mut set: impl FnMut(u32, fn(&mut Cpu, &mut NeoGeoBus) -> u32),
    f: fn(&mut Cpu, &mut NeoGeoBus) -> u32,
) -> usize {
    let mut n = 0;
    for p in replaceable() {
        set(p.entry, f);
        n += 1;
    }
    n
}

/// Como [`install`], y además las rutinas de sistema portadas (arranque,
/// interrupciones, planificador, bucle de frame: ver [`system`]).
pub fn install_all(mut set: impl FnMut(u32, fn(&mut Cpu, &mut NeoGeoBus) -> u32)) -> usize {
    let mut n = install(&mut set);
    for &entry in system::system_index().keys() {
        set(entry, system::call_system);
        n += 1;
    }
    n
}

/// Informe del modo sólo manual: dónde se paró y por qué.
pub fn halt_report(pc: u32, frame: u32, cpu: &Cpu) -> String {
    let ported = PORTS.iter().any(|p| p.entry == pc) || system::SYSTEM_PORTS.iter().any(|p| p.entry == pc);
    let mut s = format!(
        "sólo port manual: parado en el frame {frame}, PC ${pc:06X} = {}\n",
        symbols::describe(pc)
    );
    if ported {
        s += "  (rutina portada pero no sustituible todavía: llama a rutinas sin portar)\n";
    } else {
        s += "  siguiente rutina que portar para avanzar.\n";
    }
    s += &format!("  a6=${:06X} a7=${:06X}", cpu.a[6] & 0xFF_FFFF, cpu.a[7] & 0xFF_FFFF);
    s
}

/// Informe del modo seguir (`--keep-going`): rutinas sin portar que se
/// ejecutaron con el juego recompilado, agrupadas por rutina (las vueltas de
/// un `jsr` a código portado cuentan para la rutina que lo contiene) y por
/// orden de primera aparición. `hits`: PC de entrada -> (primer frame,
/// entradas, instrucciones); `native` / `fallback`: instrucciones del port y
/// del relleno.
pub fn keep_going_report(
    hits: &std::collections::BTreeMap<u32, (u32, u64, u64)>,
    native: u64,
    fallback: u64,
    top: usize,
) -> String {
    use std::collections::BTreeMap;
    use std::fmt::Write;
    let mut by: BTreeMap<u32, (u32, u64, u64)> = BTreeMap::new();
    for (&pc, &(f, n, ins)) in hits {
        let r = symbols::containing(pc).map_or(pc, |(a, _)| a);
        let e = by.entry(r).or_insert((f, 0, 0));
        e.0 = e.0.min(f);
        e.1 += n;
        e.2 += ins;
    }
    let mut v: Vec<(u32, (u32, u64, u64))> = by.into_iter().collect();
    v.sort_by_key(|&(pc, (f, _, _))| (f, pc));
    let total = native + fallback;
    let pct = if total == 0 { 100.0 } else { 100.0 * native as f64 / total as f64 };
    let mut s = String::new();
    let _ = writeln!(
        s,
        "--keep-going: {} rutinas sin portar usadas; instrucciones por el port {pct:.2} % ({native} de {total})",
        v.len()
    );
    if !v.is_empty() {
        let _ = writeln!(s, "  primer frame   entradas  instrucciones  PC       rutina");
    }
    for (pc, (f, n, ins)) in v.iter().take(top) {
        let _ = writeln!(s, "  {f:>12}  {n:>9}  {ins:>13}  ${pc:06X}  {}", symbols::describe(*pc));
    }
    if v.len() > top {
        let _ = writeln!(s, "  ... y {} más (--keep-top N)", v.len() - top);
    }
    let nested = system::nested_fallback();
    if !nested.is_empty() {
        let _ = writeln!(s, "  llamadas anidadas desde el port a código sin portar: {}", nested.len());
        for (pc, n) in nested.iter().take(top) {
            let _ = writeln!(s, "                 {n:>9}  ${pc:06X}  {}", symbols::describe(*pc));
        }
    }
    s
}
