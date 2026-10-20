//! Rutinas de sistema portadas (arranque, interrupciones, planificador,
//! bucle de frame...): trabajan con toda la máquina y pueden no «volver»
//! (ver [`Flow`]).
//!
//! [`CpuMachine`] da a `mslug_game` la máquina real (CPU + bus) como
//! [`Machine`]. Las llamadas a rutinas originales aún sin portar
//! ([`Machine::call_original`]) se ejecutan aquí mismo hasta que vuelven;
//! si por el camino se llega a una rutina portada, se ejecuta la función
//! Rust. En el modo sólo manual ([`set_mode`]) llegar a código del juego
//! sin portar para la máquina.

use crate::{call_rust, index, CYCLES_PER_CALL};
use mslug_game::system::{boot, frame, irq, pads, sound, tasks, video};
use mslug_game::{Flow, Machine, Memory, Regs};
use neogeo_hw::cpu::m68k::exec::write_sr;
use neogeo_hw::cpu::m68k::{Bus, Cpu, StatusRegister};
use neogeo_hw::NeoGeoBus;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::OnceLock;

/// Dirección de vuelta ficticia para las llamadas anidadas (nunca se
/// ejecuta: al llegar a ella con la pila como estaba, la llamada ha vuelto).
const RETURN_SENTINEL: u32 = 0x00FF_FFF0;
/// Límite de instrucciones de una llamada anidada.
const MAX_NESTED_STEPS: u32 = 5_000_000;

static ONLY_MANUAL: AtomicBool = AtomicBool::new(false);
static ALLOW_BIOS: AtomicBool = AtomicBool::new(true);
/// Rutina sin portar en la que se paró una llamada anidada (0 = ninguna).
static HALTED_AT: AtomicU32 = AtomicU32::new(0);

/// Modo sólo manual: no se ejecuta ningún código del juego original (sólo
/// la BIOS, si `allow_bios`).
pub fn set_mode(only_manual: bool, allow_bios: bool) {
    ONLY_MANUAL.store(only_manual, Ordering::Relaxed);
    ALLOW_BIOS.store(allow_bios, Ordering::Relaxed);
}

/// Si una llamada anidada se paró en una rutina sin portar, cuál.
pub fn halted_at() -> Option<u32> {
    match HALTED_AT.load(Ordering::Relaxed) {
        0 => None,
        a => Some(a),
    }
}

/// La máquina real vista por el juego portado.
pub struct CpuMachine<'a> {
    pub cpu: &'a mut Cpu,
    pub bus: &'a mut NeoGeoBus,
    /// Ciclos gastados en llamadas anidadas.
    pub cycles: u32,
    /// Se paró una llamada anidada: las siguientes no hacen nada.
    pub halted: Option<u32>,
}

impl Memory for CpuMachine<'_> {
    fn read_u8(&mut self, addr: u32) -> u8 {
        self.bus.read8(addr & 0xFF_FFFF)
    }
    fn write_u8(&mut self, addr: u32, v: u8) {
        self.bus.write8(addr & 0xFF_FFFF, v)
    }
    // Los registros de vídeo son de 16 bits: los accesos de word deben
    // llegar al bus como tales.
    fn read_u16(&mut self, addr: u32) -> u16 {
        self.bus.read16(addr & 0xFF_FFFF)
    }
    fn write_u16(&mut self, addr: u32, v: u16) {
        self.bus.write16(addr & 0xFF_FFFF, v)
    }
    fn read_u32(&mut self, addr: u32) -> u32 {
        self.bus.read32(addr & 0xFF_FFFF)
    }
    fn write_u32(&mut self, addr: u32, v: u32) {
        self.bus.write32(addr & 0xFF_FFFF, v)
    }
}

impl Machine for CpuMachine<'_> {
    fn set_interrupt_mask(&mut self, level: u8) {
        self.cpu.sr.set_interrupt_mask(level);
    }
    fn stack_pointer(&mut self) -> u32 {
        self.cpu.a[7]
    }
    fn set_stack_pointer(&mut self, sp: u32) {
        self.cpu.a[7] = sp;
    }
    fn task_register(&mut self) -> u32 {
        self.cpu.a[6]
    }
    fn set_task_register(&mut self, a6: u32) {
        self.cpu.a[6] = a6;
    }
    fn regs(&mut self) -> Regs {
        let mut r = Regs { d: self.cpu.d, carry: self.cpu.sr.0 & 1 != 0, ..Regs::default() };
        r.a.copy_from_slice(&self.cpu.a[..7]);
        r
    }
    fn set_regs(&mut self, r: Regs) {
        self.cpu.d = r.d;
        self.cpu.a[..7].copy_from_slice(&r.a);
        self.cpu.sr.set(StatusRegister::C | StatusRegister::X, r.carry);
    }
    fn call_original(&mut self, routine: u32, regs: &mut Regs) {
        if self.halted.is_some() {
            return;
        }
        self.set_regs(*regs);
        match nested_call(self.cpu, self.bus, routine) {
            Ok(c) => self.cycles += c,
            Err(at) => self.halted = Some(at),
        }
        *regs = self.regs();
    }
}

/// Ejecuta la rutina `routine` hasta que vuelve. Err(dirección) si en el
/// modo sólo manual llega a código sin portar.
fn nested_call(cpu: &mut Cpu, bus: &mut NeoGeoBus, routine: u32) -> Result<u32, u32> {
    let irq = cpu.pending_irq;
    cpu.pending_irq = 0;
    let ret_sp = cpu.a[7];
    cpu.push32(bus, RETURN_SENTINEL);
    let ret_pc = cpu.pc;
    cpu.pc = routine & 0xFF_FFFF;
    let only = ONLY_MANUAL.load(Ordering::Relaxed);
    let bios_ok = ALLOW_BIOS.load(Ordering::Relaxed);
    let mut cycles = 0u32;
    let mut steps = 0u32;
    let result = loop {
        let pc = cpu.pc & 0xFF_FFFF;
        if pc == RETURN_SENTINEL && cpu.a[7] == ret_sp {
            break Ok(cycles);
        }
        steps += 1;
        if steps > MAX_NESTED_STEPS {
            break Err(pc);
        }
        if let Some(f) = system_index().get(&pc) {
            cycles += run_system(f, cpu, bus);
            continue;
        }
        if index().contains_key(&pc) {
            cycles += call_rust(cpu, bus);
            continue;
        }
        let is_bios = (0xC0_0000..0xD0_0000).contains(&pc);
        if only && (!is_bios || !bios_ok) {
            HALTED_AT.store(pc, Ordering::Relaxed);
            break Err(pc);
        }
        cycles += cpu.step(bus);
    };
    if result.is_ok() {
        cpu.pc = ret_pc;
    }
    cpu.pending_irq = irq;
    result
}

/// Rutina de sistema portada.
pub type SystemFn = fn(&mut CpuMachine) -> Flow;

pub struct SystemPort {
    pub entry: u32,
    pub original: &'static str,
    pub rust: &'static str,
    pub run: SystemFn,
}

macro_rules! ret {
    ($f:expr) => {
        |m: &mut CpuMachine| {
            $f(m);
            Flow::Return
        }
    };
}

/// Rutinas de sistema portadas, con la dirección de entrada original.
pub const SYSTEM_PORTS: &[SystemPort] = &[
    // --- arranque (lo que pide la BIOS)
    SystemPort { entry: 0x00_0122, original: "NeoGeoHeader_USER", rust: "system::boot::user_request", run: |m| boot::user_request(m) },
    SystemPort { entry: 0x00_07CC, original: "User_Request_0007CC", rust: "system::boot::user_request", run: |m| boot::user_request(m) },
    SystemPort { entry: 0x00_0134, original: "NeoGeoHeader_COIN_SOUND", rust: "system::sound::request_coin_sound",
                 run: ret!(sound::request_coin_sound) },
    SystemPort { entry: 0x00_22BE, original: "Sound_RequestCoin_0022BE", rust: "system::sound::request_coin_sound",
                 run: ret!(sound::request_coin_sound) },
    SystemPort { entry: 0x00_080C, original: "User_Init_00080C", rust: "system::boot::user_init", run: |m| boot::user_init(m) },
    SystemPort { entry: 0x00_0832, original: "User_Test_000832", rust: "system::boot::user_test", run: |m| boot::user_test(m) },
    SystemPort { entry: 0x00_0836, original: "User_Demo_000836", rust: "system::boot::user_start_game", run: |m| boot::user_start_game(m) },
    SystemPort { entry: 0x00_0840, original: "User_Title_000840", rust: "system::boot::user_start_game", run: |m| boot::user_start_game(m) },
    SystemPort { entry: 0x00_085E, original: "ReturnToBios_00085E", rust: "system::boot::return_to_bios", run: |m| boot::return_to_bios(m) },
    SystemPort { entry: 0x00_097A, original: "Init_Subsystems_00097A", rust: "system::boot::init_subsystems", run: ret!(boot::init_subsystems) },
    SystemPort { entry: 0x00_20E2, original: "Init_MasterSubsystems_0020E2", rust: "system::boot::init_master", run: ret!(boot::init_master) },
    SystemPort { entry: 0x02_4E38, original: "TitleModeInit_024E38", rust: "system::boot::title_mode_init", run: ret!(boot::title_mode_init) },
    SystemPort { entry: 0x09_773C, original: "HiScore_LoadDefaults_09773c", rust: "system::boot::load_default_hiscores",
                 run: ret!(boot::load_default_hiscores) },
    SystemPort { entry: 0x01_3D12, original: "Pause_Clear_013d12", rust: "system::boot::clear_pause", run: ret!(boot::clear_pause) },
    SystemPort { entry: 0x00_0FFE, original: "Global_SetDualFlagFrom10FD82_000FFE", rust: "system::boot::set_dual_flag",
                 run: ret!(boot::set_dual_flag) },
    SystemPort { entry: 0x05_CC0E, original: "Input_RisingEdgeSnapshot_05CC0E", rust: "system::boot::reset_pads", run: ret!(boot::reset_pads) },
    // --- interrupciones y frame
    SystemPort { entry: 0x00_08F6, original: "VBlank_IRQ_0008F6", rust: "system::irq::vblank", run: |m| irq::vblank(m) },
    SystemPort { entry: 0x00_08D6, original: "Timer_IRQ_0008D6", rust: "system::irq::timer", run: |m| irq::timer(m) },
    SystemPort { entry: 0x00_1E5E, original: "VBlankTick_Master_001E5E", rust: "system::irq::vblank_tick", run: ret!(irq::vblank_tick) },
    SystemPort { entry: 0x00_1EFE, original: "Deferred_RunQueue_001efe", rust: "system::irq::run_deferred", run: ret!(irq::run_deferred) },
    SystemPort { entry: 0x00_0960, original: "Video_DetectPal_000960", rust: "system::irq::detect_pal",
                 run: |m| {
                     let pal = irq::detect_pal(m);
                     m.cpu.d[0] = u32::from(pal);
                     Flow::Return
                 } },
    SystemPort { entry: 0x00_1F84, original: "VBlank_FrameLoop_001f84", rust: "system::frame::start", run: |m| frame::start(m) },
    SystemPort { entry: 0x00_1FE2, original: "VBlank_FrameLoop_Wait_001fe2", rust: "system::frame::wait", run: |m| frame::wait(m) },
    // --- mandos y pausa
    SystemPort { entry: 0x05_CC1A, original: "Input_ReadFrame_05CC1A", rust: "system::pads::read_frame", run: ret!(pads::read_frame) },
    SystemPort { entry: 0x02_5066, original: "Input_Poll_LiveMode_025066", rust: "system::pads::read_live",
                 run: |m| {
                     if m.read_u8(mslug_game::layout::pads::DEMO_MODE) != 0 {
                         // Con demo, el original sigue en la rutina de demo.
                         return Flow::Goto(mslug_game::layout::pads::DEMO_READ);
                     }
                     pads::read_live(m);
                     Flow::Return
                 } },
    SystemPort { entry: 0x01_3C3C, original: "Pause_Poll_013c3c", rust: "system::pads::poll_pause",
                 run: |m| {
                     let p = pads::poll_pause(m);
                     m.cpu.sr.set(StatusRegister::C, p);
                     Flow::Return
                 } },
    // --- sonido
    SystemPort { entry: 0x00_212E, original: "InputQueue_InitAndPushOp4_00212E", rust: "system::sound::init_queue",
                 run: ret!(sound::init_queue) },
    SystemPort { entry: 0x00_2126, original: "Sound_CancelCoin_002126", rust: "system::sound::cancel_coin_sound",
                 run: ret!(sound::cancel_coin_sound) },
    SystemPort { entry: 0x00_226A, original: "Sound_PumpQueue_00226a", rust: "system::sound::pump", run: ret!(sound::pump) },
    // --- tareas
    SystemPort { entry: 0x00_0410, original: "Task_FreeListInit_000410", rust: "system::tasks::init", run: ret!(tasks::init) },
    SystemPort { entry: 0x00_0A7C, original: "TaskSlots_BootInstall_000A7C", rust: "system::tasks::boot_install",
                 run: |m| {
                     tasks::boot_install(m);
                     // El original deja la última tarea reiniciada en a0 y su
                     // propia rutina nueva en a1.
                     m.cpu.a[0] = mslug_game::layout::task::BOOT_RESET[2];
                     m.cpu.a[1] = mslug_game::layout::routine::BOOT_DONE;
                     Flow::Return
                 } },
    SystemPort { entry: 0x00_04AE, original: "Task_Alloc_0004AE", rust: "system::tasks::spawn",
                 run: |m| {
                     let parent = m.cpu.a[6];
                     let handler = m.cpu.a[1];
                     let next = m.read_u32(parent + mslug_game::layout::task::NEXT);
                     let t = tasks::spawn(m, handler, parent);
                     m.cpu.a[0] = t;
                     if t != mslug_game::layout::task::OVERFLOW_BLOCK {
                         m.cpu.a[1] = next;
                         m.cpu.d[0] = (m.cpu.d[0] & 0xFFFF_FF00) | u32::from(m.read_u8(t + mslug_game::layout::task::PRIORITY));
                     }
                     Flow::Return
                 } },
    SystemPort { entry: 0x00_050E, original: "Task_AllocAndMarkBusy_00050e", rust: "system::tasks::spawn_fixed_at",
                 run: |m| {
                     let (t, handler, parent) = (m.cpu.a[0], m.cpu.a[1], m.cpu.a[6]);
                     let next = m.read_u32(parent + mslug_game::layout::task::NEXT);
                     tasks::spawn_fixed_at(m, t, handler, parent);
                     m.cpu.a[1] = next;
                     m.cpu.d[0] = (m.cpu.d[0] & 0xFFFF_FF00) | u32::from(m.read_u8(t + mslug_game::layout::task::PRIORITY));
                     Flow::Return
                 } },
    SystemPort { entry: 0x00_05FE, original: "Task_ResetInPlace_0005FE", rust: "system::tasks::reset_in_place",
                 run: |m| {
                     let t = m.cpu.a[0];
                     let prio = m.read_u16(t + mslug_game::layout::task::PRIORITY);
                     tasks::reset_in_place(m, t);
                     m.cpu.d[0] = (m.cpu.d[0] & 0xFFFF_0000) | u32::from(prio);
                     Flow::Return
                 } },
    SystemPort { entry: 0x00_0656, original: "Scheduler_MainLoop_000656", rust: "system::tasks::run", run: |m| tasks::run(m) },
    SystemPort { entry: 0x00_06CA, original: "Scheduler_AfterTask_0006CA", rust: "system::tasks::after_task", run: |m| tasks::after_task(m) },
    SystemPort { entry: 0x00_1B40, original: "Players_StoreActiveCount_001b40", rust: "system::tasks::store_active_players",
                 run: |m| {
                     let n = tasks::store_active_players(m);
                     m.cpu.d[0] = (m.cpu.d[0] & 0xFFFF_FF00) | u32::from(n as u8);
                     Flow::Return
                 } },
    // --- vídeo
    SystemPort { entry: 0x01_379A, original: "Pal_ShadowClearAll_01379a", rust: "system::video::clear_palettes", run: ret!(video::clear_palettes) },
    SystemPort { entry: 0x01_37C6, original: "Pal_FlushDirtyToHW_0137c6", rust: "system::video::flush_palettes", run: ret!(video::flush_palettes) },
    SystemPort { entry: 0x05_26B8, original: "SpriteTable_Init256_0526b8", rust: "system::video::init_sprite_table",
                 run: ret!(video::init_sprite_table) },
    SystemPort { entry: 0x05_2712, original: "Pubcleaner_10A2Cx_052712", rust: "system::video::clear_palette_state",
                 run: ret!(video::clear_palette_state) },
    SystemPort { entry: 0x05_A824, original: "VRAM_FixLayerAutoclear_05A824", rust: "system::video::clear_vram", run: ret!(video::clear_vram) },
    SystemPort { entry: 0x05_A88A, original: "SpriteSubsystem_Reset_05A88A", rust: "system::video::reset_sprite_queues",
                 run: ret!(video::reset_sprite_queues) },
    SystemPort { entry: 0x05_A9BA, original: "VRAM_FixAutoclear_Reset_05a9ba", rust: "system::video::begin_draw_frame",
                 run: ret!(video::begin_draw_frame) },
    SystemPort { entry: 0x05_C9D6, original: "Vblank_FlushSpriteQueue_05c9d6", rust: "system::video::flush_sprite_queues",
                 run: ret!(video::flush_sprite_queues) },
];

/// Índice dirección de entrada -> rutina de sistema (respetando
/// `MSLUG_MANUAL_ONLY` / `MSLUG_MANUAL_SKIP`, como las demás).
pub fn system_index() -> &'static HashMap<u32, SystemFn> {
    static IDX: OnceLock<HashMap<u32, SystemFn>> = OnceLock::new();
    IDX.get_or_init(|| {
        SYSTEM_PORTS.iter().filter(|p| crate::selected(p.original)).map(|p| (p.entry, p.run)).collect()
    })
}

/// Deja la CPU como la dejaría la rutina original al acabar.
pub fn apply_flow(cpu: &mut Cpu, bus: &mut NeoGeoBus, flow: Flow, entry: u32) {
    match flow {
        Flow::Return => {
            let pc = cpu.pop32(bus);
            cpu.pc = pc & 0xFF_FFFF;
        }
        Flow::ReturnFromInterrupt => {
            let sr = cpu.pop16(bus);
            let pc = cpu.pop32(bus);
            write_sr(cpu, sr);
            cpu.pc = pc & 0xFF_FFFF;
        }
        Flow::Goto(t) => cpu.pc = t & 0xFF_FFFF,
        Flow::Call { routine, then } => {
            cpu.push32(bus, then);
            cpu.pc = routine & 0xFF_FFFF;
        }
        Flow::Wait => cpu.pc = entry,
    }
}

fn run_system(f: &SystemFn, cpu: &mut Cpu, bus: &mut NeoGeoBus) -> u32 {
    let entry = cpu.pc & 0xFF_FFFF;
    let (flow, cycles, halted) = {
        let mut m = CpuMachine { cpu: &mut *cpu, bus: &mut *bus, cycles: 0, halted: None };
        let flow = f(&mut m);
        (flow, m.cycles, m.halted)
    };
    if let Some(at) = halted {
        // Se para en la rutina sin portar (el modo sólo manual lo informa).
        cpu.pc = at;
        return CYCLES_PER_CALL;
    }
    apply_flow(cpu, bus, flow, entry);
    CYCLES_PER_CALL + cycles
}

/// Ejecuta la rutina de sistema portada que empieza en el PC actual.
/// Misma firma que las instrucciones del programa recompilado.
pub fn call_system(cpu: &mut Cpu, bus: &mut NeoGeoBus) -> u32 {
    let pc = cpu.pc & 0xFF_FFFF;
    let f = system_index().get(&pc).expect("call_system: dirección sin rutina de sistema portada");
    run_system(f, cpu, bus)
}
