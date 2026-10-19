//! Verificación del port escrito a mano (`mslug-game`) contra la ROM.
//!
//! **Único sitio que relaciona el juego en Rust legible con el original**: a
//! cada función de `mslug-game` le corresponde aquí la rutina original
//! (dirección de entrada) y un adaptador que traduce los registros de la
//! llamada original a los argumentos Rust.
//!
//! Durante una partida (que sigue siempre por la ROM), cada vez que el PC
//! llega a una de esas rutinas:
//!   1. se ejecuta la ROM hasta su `rts` → RAM de referencia;
//!   2. desde el mismo estado inicial se llama a la función Rust sobre una
//!      copia de la RAM → RAM portada;
//!   3. se comparan las RAM (y el valor devuelto, si lo hay).
//!
//! No se comparan registros ni flags ni ciclos: el port legible no los
//! reproduce, sólo el **comportamiento observable**.

use anyhow::Result;
use mslug_game::input::{self, Buttons, Edge};
use mslug_game::player::{controls, invuln, voice, weapons};
use mslug_game::{Entity, Memory, PlayerId};
use neogeo_hw::cpu::m68k::{Bus, Cpu};
use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;

/// RAM de trabajo vista como `Memory` del juego portado.
struct Ram<'a>(&'a mut [u8]);

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
enum Ret {
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
}

/// Efectos de recogida: en la verificación se anotan, y la comprobación
/// exige que la ROM haya llamado a la misma rutina (ver `expect_calls`).
#[derive(Default)]
struct Fx {
    calls: Vec<(&'static str, u32)>,
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
type Adapter = fn(&Cpu, &mut Ram, &mut Fx) -> Ret;

/// Rutinas que llaman las funciones portadas y que por ahora siguen en la ROM
/// (se comparan como "llamadas", no se ejecutan en el lado Rust).
/// El último campo extrae el argumento que se compara (0 = ninguno).
type ArgFn = fn(&Cpu) -> u32;
const CALLEES: &[(u32, &str, ArgFn)] = &[
    (0x09_B9BA, "Score_Popup_ForPlayer", |_| 0),
    (0x09_B9F6, "Entity_AllocByPlayerSlot", |_| 0),
    (0x00_2352, "Sound_Request", |c| c.d[0] & 0xFFFF),
];

struct Port {
    entry: u32,
    original: &'static str,
    rust: &'static str,
    run: Adapter,
}

fn player_of(d0: u32) -> PlayerId {
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

const PORTS: &[Port] = &[
    Port { entry: 0x03_2B58, original: "Player_SetWeaponAndAmmo", rust: "player::weapons::set_weapon_and_ammo",
           run: |c, m, _| { weapons::set_weapon_and_ammo(m, player_of(c.d[0]), c.d[1] as u8, c.d[2] as u8); Ret::None } },
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
           run: |c, m, _| Ret::D0B(input::horizontal_held(m, e6(c))) },
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

fn e6(c: &Cpu) -> Entity {
    Entity::from_raw(c.a[6] & 0xFF_FFFF)
}

/// Salto: C=1 y d0 = dirección + 1, o C=0 y d0 = 0.
fn jump_ret(dir: Option<u8>) -> Ret {
    match dir {
        Some(d) => Ret::YesD0W(true, u16::from(d) + 1),
        None => Ret::YesD0W(false, 0),
    }
}

fn rom_ret(kind: Ret, cpu: &Cpu) -> Ret {
    match kind {
        Ret::None => Ret::None,
        Ret::D0W(_) => Ret::D0W(cpu.d[0] as u16),
        Ret::D0B(_) => Ret::D0B(cpu.d[0] as u8),
        Ret::Ok(_) => Ret::Ok(cpu.sr.0 & 1 == 0),
        Ret::Yes(_) => Ret::Yes(cpu.sr.0 & 1 != 0),
        Ret::YesD0W(..) => Ret::YesD0W(cpu.sr.0 & 1 != 0, cpu.d[0] as u16),
        Ret::D1B(_) => Ret::D1B(cpu.d[1] as u8),
    }
}

#[derive(Default)]
struct Stats {
    /// Veces que el juego ha llamado a la rutina.
    seen: u64,
    /// Llamadas comprobadas (muestreo: todas las primeras 1000 y luego
    /// cada vez menos, para cubrir toda la partida sin disparar el coste).
    calls: u64,
    fails: u64,
    first: Option<String>,
}

pub fn run(rom: &PathBuf, frames: u32, coin: u32, fuzz: Option<u64>) -> Result<()> {
    let mut sys = crate::load(rom)?;
    let mut fz = fuzz.map(crate::Fuzz::new);
    let mut stats: BTreeMap<u32, Stats> = BTreeMap::new();
    let mut skip: Option<u32> = None;
    let index: HashMap<u32, usize> = PORTS.iter().enumerate().map(|(i, p)| (p.entry, i)).collect();
    for f in 0..frames {
        match fz.as_mut() {
            Some(z) => z.inputs(&mut sys, f),
            None => crate::auto_inputs(&mut sys, coin, f),
        }
        let mut budget: i64 = 202_752;
        while budget > 0 {
            let pc = sys.m68k.pc & 0xFF_FFFF;
            if skip != Some(pc) {
                if let Some(&i) = index.get(&pc) {
                    let st = stats.entry(pc).or_default();
                    st.seen += 1;
                    if st.seen % (st.seen / 1000).max(1) == 0 {
                        check(&mut sys, &PORTS[i], st);
                        skip = Some(pc);
                        continue;
                    }
                }
            }
            skip = None;
            budget -= i64::from(sys.step());
        }
    }
    println!("{:<34} {:<42} {:>9} {:>8} {:>6}", "original", "Rust", "llamadas", "comprob.", "fallos");
    let mut bad = false;
    let (mut tc, mut tf) = (0, 0);
    for p in PORTS {
        let s = stats.remove(&p.entry).unwrap_or_default();
        println!("{:<34} {:<42} {:>9} {:>8} {:>6}", p.original, p.rust, s.seen, s.calls, s.fails);
        tc += s.calls;
        tf += s.fails;
        if let Some(m) = s.first {
            println!("    primer fallo: {m}");
            bad = true;
        }
    }
    println!("total: {tc} llamadas comprobadas, {tf} fallos ({frames} frames)");
    if bad {
        anyhow::bail!("hay funciones del port manual que no se comportan como la ROM");
    }
    Ok(())
}

fn check(sys: &mut neogeo_hw::System, p: &Port, st: &mut Stats) {
    let cpu0 = sys.m68k.clone();
    let ram0 = sys.bus.work_ram.to_vec();
    let irq = sys.m68k.pending_irq;
    sys.m68k.pending_irq = 0;

    // --- ROM hasta el rts, anotando las llamadas a rutinas aún no portadas
    let ret_sp = cpu0.a[7].wrapping_add(4);
    let ret_pc = sys.bus.read32(cpu0.a[7] & 0xFF_FFFF) & 0xFF_FFFF;
    let mut rom_calls: Vec<(&'static str, u32)> = Vec::new();
    let mut steps = 0;
    loop {
        let pc = sys.m68k.pc & 0xFF_FFFF;
        if let Some((_, n, arg)) = CALLEES.iter().find(|(a, _, _)| *a == pc) {
            rom_calls.push((n, arg(&sys.m68k)));
        }
        sys.m68k.step(&mut sys.bus);
        steps += 1;
        if (sys.m68k.pc & 0xFF_FFFF == ret_pc && sys.m68k.a[7] == ret_sp) || steps > 1_000_000 {
            break;
        }
    }
    let ram_rom = sys.bus.work_ram.to_vec();
    let cpu_rom = sys.m68k.clone();

    // --- Rust desde el mismo estado
    let mut ram_rust = ram0.clone();
    let mut fx = Fx::default();
    let got = (p.run)(&cpu0, &mut Ram(&mut ram_rust), &mut fx);
    let want = rom_ret(got, &cpu_rom);

    // Las rutinas no portadas que llama la ROM escriben su propia RAM: sólo
    // se compara la RAM si no hubo llamadas (si las hubo, se compara que
    // sean las mismas y la RAM fuera del bloque de entidades nuevas).
    let mut diffs = Vec::new();
    if got != want {
        diffs.push(format!("devuelve rom={want:?} rust={got:?}"));
    }
    if rom_calls != fx.calls {
        diffs.push(format!("llamadas rom={rom_calls:?} rust={:?}", fx.calls));
    }
    if rom_calls.is_empty() {
        let bad: Vec<usize> = (0..ram_rom.len()).filter(|&i| ram_rom[i] != ram_rust[i]).collect();
        // La pila (cerca de A7) no es estado del juego.
        let sp = (cpu0.a[7] & 0xFFFF) as usize;
        let bad: Vec<usize> = bad.into_iter().filter(|&i| !(sp.saturating_sub(0x200)..sp + 8).contains(&i)).collect();
        if let Some(&i) = bad.first() {
            diffs.push(format!("RAM: {} bytes distintos, primero ${:06X} rom={:02X} rust={:02X}",
                               bad.len(), 0x10_0000 + i, ram_rom[i], ram_rust[i]));
        }
    }
    st.calls += 1;
    if !diffs.is_empty() {
        st.fails += 1;
        if st.first.is_none() {
            st.first = Some(format!("llamada {}: {}", st.calls, diffs.join("; ")));
        }
    }

    // --- el juego sigue por la ROM desde el estado inicial
    sys.m68k = cpu0;
    sys.m68k.pending_irq = irq;
    sys.bus.work_ram.copy_from_slice(&ram0);
}
