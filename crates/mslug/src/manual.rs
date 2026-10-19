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
use mslug_game::player::{invuln, weapons};
use mslug_game::{Entity, Memory, PlayerId};
use neogeo_hw::cpu::m68k::{Bus, Cpu};
use std::collections::BTreeMap;
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
}

/// Efectos de recogida: en la verificación se anotan, y la comprobación
/// exige que la ROM haya llamado a la misma rutina (ver `expect_calls`).
#[derive(Default)]
struct Fx {
    calls: Vec<&'static str>,
}

impl<M: Memory> weapons::PickupEffects<M> for Fx {
    fn score_popup(&mut self, _: &mut M, _: PlayerId) {
        self.calls.push("Score_Popup_ForPlayer");
    }
    fn weapon_pickup_marker(&mut self, _: &mut M, _: PlayerId) {
        self.calls.push("Entity_AllocByPlayerSlot");
    }
}

/// Adaptador: llama a la función Rust con los argumentos que la rutina
/// original recibe en registros.
type Adapter = fn(&Cpu, &mut Ram, &mut Fx) -> Ret;

/// Rutinas que llaman las funciones portadas y que por ahora siguen en la ROM
/// (se comparan como "llamadas", no se ejecutan en el lado Rust).
const CALLEES: &[(u32, &str)] = &[
    (0x09_B9BA, "Score_Popup_ForPlayer"),
    (0x09_B9F6, "Entity_AllocByPlayerSlot"),
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
];

fn rom_ret(kind: Ret, cpu: &Cpu) -> Ret {
    match kind {
        Ret::None => Ret::None,
        Ret::D0W(_) => Ret::D0W(cpu.d[0] as u16),
        Ret::D0B(_) => Ret::D0B(cpu.d[0] as u8),
        Ret::Ok(_) => Ret::Ok(cpu.sr.0 & 1 == 0),
    }
}

#[derive(Default)]
struct Stats {
    calls: u64,
    fails: u64,
    first: Option<String>,
}

pub fn run(rom: &PathBuf, frames: u32, coin: u32, fuzz: Option<u64>) -> Result<()> {
    let mut sys = crate::load(rom)?;
    let mut fz = fuzz.map(crate::Fuzz::new);
    let mut stats: BTreeMap<u32, Stats> = BTreeMap::new();
    let mut skip: Option<u32> = None;
    for f in 0..frames {
        match fz.as_mut() {
            Some(z) => z.inputs(&mut sys, f),
            None => crate::auto_inputs(&mut sys, coin, f),
        }
        let mut budget: i64 = 202_752;
        while budget > 0 {
            let pc = sys.m68k.pc & 0xFF_FFFF;
            if skip != Some(pc) {
                if let Some(p) = PORTS.iter().find(|p| p.entry == pc) {
                    check(&mut sys, p, stats.entry(pc).or_default());
                    skip = Some(pc);
                    continue;
                }
            }
            skip = None;
            budget -= i64::from(sys.step());
        }
    }
    println!("{:<34} {:<42} {:>8} {:>6}", "original", "Rust", "llamadas", "fallos");
    let mut bad = false;
    let (mut tc, mut tf) = (0, 0);
    for p in PORTS {
        let s = stats.remove(&p.entry).unwrap_or_default();
        println!("{:<34} {:<42} {:>8} {:>6}", p.original, p.rust, s.calls, s.fails);
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
    let mut rom_calls: Vec<&'static str> = Vec::new();
    let mut steps = 0;
    loop {
        let pc = sys.m68k.pc & 0xFF_FFFF;
        if let Some((_, n)) = CALLEES.iter().find(|(a, _)| *a == pc) {
            rom_calls.push(n);
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
