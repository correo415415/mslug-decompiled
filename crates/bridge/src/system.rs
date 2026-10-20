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
use mslug_game::system::{anim, boot, frame, hud, irq, pads, palette, render, rng, scene, script, scroll, sound, sprite, tasks, title, video};
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

fn set_carry(m: &mut CpuMachine, c: bool) {
    m.cpu.sr.set(StatusRegister::C | StatusRegister::X, c);
}

/// Sprite `k` del logo del título (deja el flag C de la animación).
fn title_sprite(m: &mut CpuMachine, k: usize) -> Flow {
    let c = title::sprite(m, k);
    m.cpu.sr.set(StatusRegister::C | StatusRegister::X, c);
    Flow::Return
}

/// Registros de `Sprite_Dispatch_05A9E2` / `05A9D6` → petición.
fn sprite_dispatch(m: &mut CpuMachine, back: bool) -> Flow {
    let d = m.cpu.d;
    let req = sprite::Request {
        pos: d[0],
        key: d[1] as u16,
        palette: d[2] as u16,
        shrink_v: d[3] as u8,
        shrink_h: d[4] as u8,
        flip: d[5] as u8,
        gfx: m.cpu.a[0],
    };
    let task = m.cpu.a[6];
    sprite::dispatch(m, task, req, back);
    Flow::Return
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
    SystemPort { entry: 0x00_2152, original: "Sound_Push_002152", rust: "system::sound::push",
                 run: |m| { let c = m.cpu.d[0] as u8; sound::push(m, c); Flow::Return } },
    SystemPort { entry: 0x00_2352, original: "Sound_Play_002352", rust: "system::sound::play",
                 run: |m| { let c = m.cpu.d[0] as u16; sound::play(m, c); Flow::Return } },
    SystemPort { entry: 0x00_219C, original: "Sound_PlayAlways_00219c", rust: "system::sound::play_always",
                 run: |m| { let c = m.cpu.d[0] as u16; sound::play_always(m, c); Flow::Return } },
    SystemPort { entry: 0x00_2222, original: "Sound_Stop_002222", rust: "system::sound::stop",
                 run: |m| { let c = m.cpu.d[0] as u16; sound::stop(m, c); Flow::Return } },
    SystemPort { entry: 0x00_2230, original: "Sound_Lock_002230", rust: "system::sound::lock", run: |m| { sound::lock(m, true); Flow::Return } },
    SystemPort { entry: 0x00_223A, original: "Sound_Unlock_00223a", rust: "system::sound::lock", run: |m| { sound::lock(m, false); Flow::Return } },
    SystemPort { entry: 0x00_22C8, original: "Sound_Push06_0022c8", rust: "system::sound::reset", run: ret!(sound::reset) },
    SystemPort { entry: 0x00_22CE, original: "Sound_Push05_0022ce", rust: "system::sound::silence", run: ret!(sound::silence) },
    SystemPort { entry: 0x00_2308, original: "Sound_Push0A_0B_002308", rust: "system::sound::setting",
                 run: |m| { let v = m.cpu.d[0] as u8; sound::setting(m, false, v); Flow::Return } },
    SystemPort { entry: 0x00_2312, original: "Sound_Push0B_002312", rust: "system::sound::setting (música)",
                 run: |m| { let v = m.cpu.d[0] as u8; sound::setting(m, true, v); Flow::Return } },
    SystemPort { entry: 0x01_3D46, original: "Pause_ShowText_013d46", rust: "system::pads::pause_text", run: |m| { pads::pause_text(m, true); Flow::Return } },
    SystemPort { entry: 0x01_3D3E, original: "Pause_HideText_013d3e", rust: "system::pads::pause_text", run: |m| { pads::pause_text(m, false); Flow::Return } },
    SystemPort { entry: 0x00_22BE, original: "Sound_RequestCoin_0022BE", rust: "system::sound::request_coin_sound",
                 run: ret!(sound::request_coin_sound) },
    SystemPort { entry: 0x00_080C, original: "User_Init_00080C", rust: "system::boot::user_init", run: |m| boot::user_init(m) },
    SystemPort { entry: 0x00_0832, original: "User_Test_000832", rust: "system::boot::user_test", run: |m| boot::user_test(m) },
    SystemPort { entry: 0x00_0836, original: "User_Demo_000836", rust: "system::boot::user_start_game", run: |m| boot::user_start_game(m) },
    SystemPort { entry: 0x00_0840, original: "User_Title_000840", rust: "system::boot::user_start_game", run: |m| boot::user_start_game(m) },
    SystemPort { entry: 0x00_0128, original: "NeoGeoHeader_PLAYER_START", rust: "system::boot::player_start", run: ret!(boot::player_start) },
    SystemPort { entry: 0x00_012E, original: "NeoGeoHeader_DEMO_END", rust: "system::boot::demo_end", run: ret!(boot::demo_end) },
    SystemPort { entry: 0x02_4E76, original: "Player_Start_Inner_024E76", rust: "system::boot::player_start", run: ret!(boot::player_start) },
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
    SystemPort { entry: 0x00_0546, original: "Task_UnlinkAlive_0546", rust: "system::tasks::unlink",
                 run: |m| {
                     let t = m.cpu.a[6];
                     let prev = tasks::unlink(m, t);
                     m.cpu.a[6] = prev;
                     Flow::Return
                 } },
    SystemPort { entry: 0x00_05B6, original: "FUN_000005B6", rust: "system::tasks::kill_children",
                 run: |m| {
                     let t = m.cpu.a[6];
                     tasks::kill_children(m, t);
                     Flow::Return
                 } },
    SystemPort { entry: 0x00_05DA, original: "Task_ChangeHandler_05DA", rust: "system::tasks::kill_children_of",
                 run: |m| {
                     let t = m.cpu.a[0];
                     tasks::kill_children_of(m, t);
                     Flow::Return
                 } },
    SystemPort { entry: 0x00_0626, original: "Task_ChangeAndRun_0626", rust: "system::tasks::stop",
                 run: |m| {
                     let t = m.cpu.a[0];
                     tasks::stop(m, t);
                     Flow::Return
                 } },
    // --- guion de arranque
    SystemPort { entry: 0x00_0E8E, original: "SchedulerBootstrap_Boot_000E8E", rust: "system::script::boot", run: |m| script::boot(m) },
    SystemPort { entry: 0x00_0EF0, original: "TaskHandler_000ef0", rust: "system::script::start_game", run: |m| script::start_game(m) },
    SystemPort { entry: 0x00_0F1A, original: "TaskHandler_000f1a", rust: "system::script::start_title", run: |m| script::start_title(m) },
    SystemPort { entry: 0x00_0F76, original: "PcThunkTarget_000F76", rust: "system::script::start_pressed", run: |m| script::start_pressed(m) },
    SystemPort { entry: 0x00_0FC6, original: "SchedulerLoopA_000FC6", rust: "system::script::dispatch", run: |m| script::dispatch(m) },
    SystemPort { entry: 0x00_0FE0, original: "SchedulerDispatch_LoopB_000FE0", rust: "system::script::next", run: |m| script::next(m) },
    SystemPort { entry: 0x00_1020, original: "SchedTail_JsrCD4_001020", rust: "system::script::step_stop_scene", run: |m| script::step_stop_scene(m) },
    SystemPort { entry: 0x00_1026, original: "SchedTail_JsrD3C_001026", rust: "system::script::step_stop_scene_keep",
                 run: |m| script::step_stop_scene_keep(m) },
    SystemPort { entry: 0x00_1354, original: "BootScr_Clr13d2a_001354", rust: "system::script::step_disable_pause",
                 run: |m| script::step_disable_pause(m) },
    SystemPort { entry: 0x00_135E, original: "BootScr_SetFF13d20_00135e", rust: "system::script::step_enable_pause",
                 run: |m| script::step_enable_pause(m) },
    SystemPort { entry: 0x00_1838, original: "Attract_SoftReset_10FDAF_001838", rust: "system::script::step_back_to_bios",
                 run: |m| script::step_back_to_bios(m) },
    SystemPort { entry: 0x00_1AF8, original: "Attract_StartIfP2Flag_001af8", rust: "system::script::check_game_started",
                 run: ret!(script::check_game_started) },
    SystemPort { entry: 0x00_1CD4, original: "TaskList_ChangeAndRunEight_001CD4", rust: "system::script::stop_scene",
                 run: ret!(script::stop_scene) },
    SystemPort { entry: 0x00_1D3C, original: "Task_ResetAllSlots_001d3c", rust: "system::script::stop_scene_keep",
                 run: ret!(script::stop_scene_keep) },
    // escenas
    SystemPort { entry: 0x00_109C, original: "AttractHandler_00109C", rust: "system::script::scene_logo", run: |m| script::scene_logo(m) },
    SystemPort { entry: 0x00_10E8, original: "SchedTail_JsrDB8_0010E8", rust: "system::script::scene_logo_wait", run: |m| script::scene_logo_wait(m) },
    SystemPort { entry: 0x00_10F2, original: "AttractHandler_2Task_0010F2", rust: "system::script::scene_continue", run: |m| script::scene_continue(m) },
    SystemPort { entry: 0x00_1148, original: "AttractPhase2_Gate106ED5_001148", rust: "system::script::scene_continue_wait",
                 run: |m| script::scene_continue_wait(m) },
    SystemPort { entry: 0x00_1172, original: "AttractHandler_Frame_001172", rust: "system::script::scene_hiscores", run: |m| script::scene_hiscores(m) },
    SystemPort { entry: 0x00_11CA, original: "AttractPhase2_Multiway_0011CA", rust: "system::script::scene_wait", run: |m| script::scene_wait(m) },
    SystemPort { entry: 0x00_11EA, original: "AttractHandler_Loader_0011EA", rust: "system::script::scene_hiscore_table",
                 run: |m| script::scene_hiscore_table(m) },
    SystemPort { entry: 0x00_122E, original: "AttractPhase2_Probes5D0_00122E", rust: "system::script::scene_hiscore_table_wait",
                 run: |m| script::scene_hiscore_table_wait(m) },
    SystemPort { entry: 0x00_1368, original: "BootScr_Mission_Start_001368", rust: "system::script::scene_mission_start", run: |m| script::scene_mission_start(m) },
    SystemPort { entry: 0x00_13C6, original: "BootScr_Mission_WaitStart_0013c6", rust: "system::script::scene_mission_wait", run: |m| script::scene_mission_wait(m) },
    SystemPort { entry: 0x01_38E6, original: "PalBank_ClearDone_0138e6", rust: "system::palette::bank_clear_done",
                 run: |m| { let c = palette::bank_clear_done(m); m.cpu.sr.set(StatusRegister::C | StatusRegister::X, c); Flow::Return } },
    SystemPort { entry: 0x00_12F4, original: "BootScr_GameOver_0012f4", rust: "system::script::scene_game_over", run: |m| script::scene_game_over(m) },
    SystemPort { entry: 0x00_1332, original: "BootScr_WaitHudThenLoop_001332", rust: "system::script::scene_wait", run: |m| script::scene_wait(m) },
    SystemPort { entry: 0x05_D09A, original: "Sub_0005D09A", rust: "system::script::button_pressed",
                 run: |m| {
                     let c = script::button_pressed(m, 0);
                     m.cpu.sr.set(StatusRegister::C | StatusRegister::X, c);
                     Flow::Return
                 } },
    SystemPort { entry: 0x05_D0AC, original: "Sub_0005D0AC", rust: "system::script::button_pressed",
                 run: |m| {
                     let c = script::button_pressed(m, 1);
                     m.cpu.sr.set(StatusRegister::C | StatusRegister::X, c);
                     Flow::Return
                 } },
    // --- guiones de animación de sprites
    SystemPort { entry: 0x02_8D70, original: "Script_DispatchOpcode_028d70", rust: "system::anim::step",
                 run: |m| {
                     let t = m.cpu.a[6];
                     let c = anim::step(m, t);
                     m.cpu.sr.set(StatusRegister::C | StatusRegister::X, c);
                     Flow::Return
                 } },
    SystemPort { entry: 0x02_8CD4, original: "Script_SetPointer_028cd4", rust: "system::anim::start",
                 run: |m| {
                     let (t, s) = (m.cpu.a[6], m.cpu.a[0]);
                     anim::start(m, t, s);
                     Flow::Return
                 } },
    SystemPort { entry: 0x02_8134, original: "Entity_SetAttr_028134", rust: "system::anim::set_attr",
                 run: |m| {
                     let (t, a) = (m.cpu.a[6], m.cpu.d[0] as u16);
                     anim::set_attr(m, t, a);
                     Flow::Return
                 } },
    SystemPort { entry: 0x02_8108, original: "Entity_ApplyFadeShade_028108", rust: "system::anim::apply_depth",
                 run: |m| {
                     let t = m.cpu.a[6];
                     anim::apply_depth(m, t);
                     Flow::Return
                 } },
    SystemPort { entry: 0x00_236E, original: "Entity_AllocSpriteSlot_00236E", rust: "system::anim::alloc_entry",
                 run: |m| {
                     let (t, g) = (m.cpu.a[6], m.cpu.d[1] as u16);
                     anim::alloc_entry(m, t, g);
                     Flow::Return
                 } },
    SystemPort { entry: 0x00_29A8, original: "Entity_ProbeSpriteSlot_29A8", rust: "system::anim::find_entry",
                 run: |m| {
                     let (t, g) = (m.cpu.a[6], m.cpu.d[1] as u16);
                     let found = anim::find_entry(m, t, g);
                     m.cpu.d[2] = (m.cpu.d[2] & 0xFFFF_FF00) | if found { 0xFF } else { 0 };
                     Flow::Return
                 } },
    SystemPort { entry: 0x05_DD02, original: "Entity_CopyPosToChild_05dd02", rust: "system::anim::copy_sprite_pos",
                 run: |m| {
                     let (from, to) = (m.cpu.a[6], m.cpu.a[0]);
                     anim::copy_sprite_pos(m, from, to);
                     Flow::Return
                 } },
    SystemPort { entry: 0x04_40D0, original: "World_ToScreen_0440d0", rust: "system::anim::world_to_screen",
                 run: |m| {
                     let (x, y) = anim::world_to_screen(m, m.cpu.d[0] as u16, m.cpu.d[1] as u16);
                     m.cpu.d[0] = (m.cpu.d[0] & 0xFFFF_0000) | u32::from(x);
                     m.cpu.d[1] = (m.cpu.d[1] & 0xFFFF_0000) | u32::from(y);
                     Flow::Return
                 } },
    // --- render del frame y paletas
    SystemPort { entry: 0x00_2098, original: "VBlank_RenderFrame_002098", rust: "system::frame::render", run: |m| frame::render(m) },
    SystemPort { entry: 0x00_2C86, original: "PalSlot_UpdateAll_002c86", rust: "system::palette::update_all", run: ret!(palette::update_all) },
    SystemPort { entry: 0x01_387E, original: "PalBank_ClearStep_01387e", rust: "system::palette::clear_bank_step", run: ret!(palette::clear_bank_step) },
    SystemPort { entry: 0x01_33B0, original: "PalSlot_Load_0133b0", rust: "system::palette::load",
                 run: |m| { let off = m.cpu.d[1] & 0xFFFF; palette::load(m, off); Flow::Return } },
    SystemPort { entry: 0x01_33E6, original: "PalSlot_Clear_0133e6", rust: "system::palette::clear",
                 run: |m| { let off = m.cpu.d[1] & 0xFFFF; palette::clear(m, off); Flow::Return } },
    SystemPort { entry: 0x01_3408, original: "PalSlot_FadeStep_013408", rust: "system::palette::fade_step",
                 run: |m| { let off = m.cpu.d[1] & 0xFFFF; palette::fade_step(m, off); Flow::Return } },
    SystemPort { entry: 0x05_A9E2, original: "Sprite_Dispatch_05A9E2", rust: "system::sprite::dispatch", run: |m| sprite_dispatch(m, false) },
    SystemPort { entry: 0x05_A9D6, original: "Sprite_Dispatch_05A9D6", rust: "system::sprite::dispatch (detrás)", run: |m| sprite_dispatch(m, true) },
    // --- carga de escena
    SystemPort { entry: 0x04_3568, original: "SceneLoader_Main_043568", rust: "system::scene::load",
                 run: |m| { let n = m.cpu.d[0] as u8; scene::load(m, n); Flow::Return } },
    SystemPort { entry: 0x04_3562, original: "SceneLoader_ByIndex_043562", rust: "system::scene::load_descriptor",
                 run: |m| { let d = m.cpu.a[0]; scene::load_descriptor(m, d); Flow::Return } },
    SystemPort { entry: 0x01_3982, original: "Spawn_TypeA_013982", rust: "system::scene::take_sprites",
                 run: |m| { let n = m.cpu.d[0] as u16; let f = scene::take_sprites(m, true, n); m.cpu.d[0] = (m.cpu.d[0] & 0xFFFF_0000) | u32::from(f); Flow::Return } },
    SystemPort { entry: 0x01_3952, original: "Spawn_TypeB_013952", rust: "system::scene::take_sprites",
                 run: |m| { let n = m.cpu.d[0] as u16; let f = scene::take_sprites(m, false, n); m.cpu.d[0] = (m.cpu.d[0] & 0xFFFF_0000) | u32::from(f); Flow::Return } },
    // --- scroll de la escena
    SystemPort { entry: 0x04_37DA, original: "SceneScriptVM_Frame_0437DA", rust: "system::scroll::frame",
                 run: |m| { let v = (m.cpu.d[0], m.cpu.d[1]); scroll::frame(m, v); Flow::Return } },
    SystemPort { entry: 0x04_3CE2, original: "SceneScript_EdgeArrivalTest_043CE2", rust: "system::scroll::edge_reached",
                 run: |m| { let c = scroll::edge_reached(m); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x04_3E3A, original: "Scroll_ClampToRange_043E3A", rust: "system::scroll::clamp_step",
                 run: |m| {
                     let d = m.cpu.d;
                     let (delta, pos) = scroll::clamp_step(d[0], d[1], d[2] as u16, d[3] as u16);
                     let sat = scroll::saturated(d[0], d[1], d[2] as u16, d[3] as u16);
                     m.cpu.d[0] = if sat { d[0] & 0xFFFF_0000 } else { (pos << 16) | u32::from(delta) };
                     m.cpu.d[1] = pos;
                     Flow::Return
                 } },
    SystemPort { entry: 0x04_34EA, original: "Camera_ResetSmoothing_0434EA", rust: "system::scroll::reset_smoothing", run: ret!(scroll::reset_smoothing) },
    SystemPort { entry: 0x04_34F8, original: "Camera_SmoothingIntegrate_0434F8", rust: "system::scroll::smoothing_step",
                 run: |m| { let d = scroll::smoothing_step(m); m.cpu.d[0] = (m.cpu.d[0] & 0xFFFF_0000) | u32::from(d); Flow::Return } },
    SystemPort { entry: 0x04_3D86, original: "CameraApplyAll4_043D86", rust: "system::scroll::apply_all", run: ret!(scroll::apply_all) },
    SystemPort { entry: 0x04_3DAA, original: "CameraApply_043DAA", rust: "system::scroll::apply",
                 run: |m| { let c = m.cpu.a[0]; scroll::apply(m, c); Flow::Return } },
    SystemPort { entry: 0x04_3EDA, original: "Buffer_ClearBlock1024L_043EDA", rust: "system::scroll::clear_collmap",
                 run: |m| { let c = m.cpu.a[0]; scroll::clear_collmap(m, c); Flow::Return } },
    SystemPort { entry: 0x04_3EEC, original: "CollMap_RowToLocalY_043EEC", rust: "system::scroll::row_to_local_y",
                 run: |m| { let r = m.cpu.d[0] as u16; let y = scroll::row_to_local_y(m, r); m.cpu.d[0] = (m.cpu.d[0] & 0xFFFF_0000) | u32::from(y); Flow::Return } },
    SystemPort { entry: 0x04_3F02, original: "CollMap_TestPoint_043F02", rust: "system::scroll::test_point",
                 run: |m| {
                     let h = scroll::test_point(m, m.cpu.d[0] as u16, m.cpu.d[1] as u16);
                     m.cpu.d[0] = (m.cpu.d[0] & 0xFFFF_FF00) | u32::from(h.cell);
                     m.cpu.d[3] = (m.cpu.d[3] & 0xFFFF_0000) | u32::from(h.frac.0);
                     m.cpu.d[4] = (m.cpu.d[4] & 0xFFFF_0000) | u32::from(h.frac.1);
                     set_carry(m, h.solid);
                     Flow::Return
                 } },
    SystemPort { entry: 0x04_CAC4, original: "MissionMark_04CAC4", rust: "system::scroll::mission_mark",
                 run: |m| {
                     let k = scroll::mission_mark(m);
                     let w = |r: u32, v: u16| (r & 0xFFFF_0000) | u32::from(v);
                     let b = |r: u32, v: u8| (r & 0xFFFF_FF00) | u32::from(v);
                     m.cpu.d[0] = w(m.cpu.d[0], k.pos.0);
                     m.cpu.d[1] = w(m.cpu.d[1], k.pos.1);
                     m.cpu.d[2] = w(m.cpu.d[2], k.d2);
                     m.cpu.d[3] = w(m.cpu.d[3], k.d3);
                     m.cpu.d[4] = b(m.cpu.d[4], k.bytes[0]);
                     m.cpu.d[5] = b(m.cpu.d[5], k.bytes[1]);
                     m.cpu.d[6] = b(m.cpu.d[6], k.bytes[2]);
                     m.cpu.a[0] = m.read_u32(mslug_game::layout::scene::MISSION_PTR);
                     Flow::Return
                 } },
    SystemPort { entry: 0x04_CB88, original: "MissionMark_Frame_04CB88", rust: "system::scroll::mission_mark_frame", run: ret!(scroll::mission_mark_frame) },
    SystemPort { entry: 0x05_1B1C, original: "Layer_SetTile_051B1C", rust: "system::scroll::set_tile",
                 run: |m| { let (c, a, r) = (m.cpu.a[0], m.cpu.d[0] as u8, m.cpu.d[1] as u16); scroll::set_tile(m, c, a, r); Flow::Return } },
    SystemPort { entry: 0x05_1B38, original: "Layer_SetTileTable_051B38", rust: "system::scroll::set_tile_table",
                 run: |m| { let (c, t) = (m.cpu.a[0], m.cpu.a[1]); scroll::set_tile_table(m, c, t); Flow::Return } },
    SystemPort { entry: 0x05_1B3E, original: "Layer_BindPath_051B3E", rust: "system::scroll::bind_path",
                 run: |m| { let (c, p, x, y) = (m.cpu.a[0], m.cpu.a[1], m.cpu.d[0] as u16, m.cpu.d[1] as u16); scroll::bind_path(m, c, p, x, y); Flow::Return } },
    SystemPort { entry: 0x05_1B80, original: "Layer_Move_051B80", rust: "system::scroll::move_layer",
                 run: |m| {
                     let c = m.cpu.a[0];
                     let (dx, dy) = scroll::move_layer(m, c, m.cpu.d[0], m.cpu.d[1]);
                     m.cpu.d[0] = (m.read_u32(c + 4) << 16) | u32::from(dx);
                     m.cpu.d[1] = (m.read_u32(c + 8) << 16) | u32::from(dy);
                     Flow::Return
                 } },
    SystemPort { entry: 0x05_1ECE, original: "Layer_Show_051ECE", rust: "system::scroll::show",
                 run: |m| { let c = m.cpu.a[0]; scroll::show(m, c); Flow::Return } },
    SystemPort { entry: 0x05_1ED6, original: "CellMap_ClearSprites32_051ed6", rust: "system::scene::hide_layer_sprites",
                 run: |m| { let c = m.cpu.a[0]; scene::hide_layer_sprites(m, c); Flow::Return } },
    SystemPort { entry: 0x05_1F02, original: "CellMap_ClearVramBlock_051f02", rust: "system::scene::clear_layer_tiles",
                 run: |m| { let c = m.cpu.a[0]; scene::clear_layer_tiles(m, c); Flow::Return } },
    SystemPort { entry: 0x05_1F30, original: "TransformCommit_MMIO_051F30", rust: "system::scroll::commit",
                 run: |m| { let c = m.cpu.a[0]; scroll::commit(m, c); Flow::Return } },
    SystemPort { entry: 0x05_1F94, original: "Layer_WriteColumns_051F94", rust: "system::scroll::write_columns",
                 run: |m| { let c = m.cpu.a[6]; scroll::write_columns(m, c); Flow::Return } },
    SystemPort { entry: 0x05_1ABE, original: "Entity_AllocAndInit_051ABE", rust: "system::scene::init_layer",
                 run: |m| { let (c, b, f) = (m.cpu.a[0], m.cpu.a[1], m.cpu.d[0] as u16); scene::init_layer(m, c, b, f); Flow::Return } },
    SystemPort { entry: 0x05_B232, original: "SpriteQueue_SortAndRenderSCB1_05b232", rust: "system::render::sort_and_render", run: ret!(render::sort_and_render) },
    SystemPort { entry: 0x05_B370, original: "SpriteQueue_RenderRange_05b370", rust: "system::render::render_range",
                 run: |m| { let (q, a, b) = (m.cpu.a[0], m.cpu.a[1], m.cpu.a[2]); render::render_range(m, q, a, b); Flow::Return } },
    SystemPort { entry: 0x05_B52E, original: "SCB1_WriteTileColumn_05b52e", rust: "system::render::write_column",
                 run: |m| { let (n, rec) = (m.cpu.d[0] as u16, m.cpu.a[3]); render::write_column(m, n, rec); Flow::Return } },
    SystemPort { entry: 0x05_B400, original: "SpriteQueue_RenderSCB234_05b400", rust: "system::render::render_scb234",
                 run: |m| { let (q, a, b) = (m.cpu.a[0], m.cpu.a[1], m.cpu.a[2]); render::render_scb234(m, q, a, b); Flow::Return } },
    SystemPort { entry: 0x05_BF76, original: "SCB1_WriteTileColumnTerm_05bf76", rust: "system::render::write_column_full",
                 run: |m| { let (n, rec) = (m.cpu.d[0] as u16, m.cpu.a[3]); render::write_column_full(m, n, rec); Flow::Return } },
    // --- pantalla de título
    SystemPort { entry: 0x09_1330, original: "TaskTpl_091330", rust: "system::title::begin", run: |m| title::begin(m) },
    SystemPort { entry: 0x09_1338, original: "Continue_Spawn_091338", rust: "system::title::start", run: |m| title::start(m) },
    SystemPort { entry: 0x09_13AC, original: "Continue_Tpl_0913ac", rust: "system::title::start_logo_only", run: |m| title::start_logo_only(m) },
    SystemPort { entry: 0x09_13AA, original: "TaskHandler_0913aa", rust: "system::title::idle", run: |m| title::idle(m) },
    SystemPort { entry: 0x09_148C, original: "Continue_Sprite_Left_09148c", rust: "system::title::sprite", run: |m| title_sprite(m, 0) },
    SystemPort { entry: 0x09_144A, original: "Continue_Sprite_Right_09144a", rust: "system::title::sprite", run: |m| title_sprite(m, 1) },
    SystemPort { entry: 0x09_1408, original: "Continue_Sprite_Mid_091408", rust: "system::title::sprite", run: |m| title_sprite(m, 2) },
    SystemPort { entry: 0x09_14CE, original: "Continue_Sprite_MidB_0914ce", rust: "system::title::sprite", run: |m| title_sprite(m, 3) },
    SystemPort { entry: 0x09_150C, original: "JsrAbsThunk_09150c", rust: "system::title::animate",
                 run: |m| {
                     let c = title::animate(m);
                     m.cpu.sr.set(StatusRegister::C | StatusRegister::X, c);
                     Flow::Return
                 } },
    SystemPort { entry: 0x09_1514, original: "Continue_Text_Init_091514", rust: "system::title::text", run: |m| title::text(m) },
    SystemPort { entry: 0x09_152C, original: "Continue_Text_Clear_09152c", rust: "system::title::text_hidden", run: |m| title::text_hidden(m) },
    SystemPort { entry: 0x09_1558, original: "Continue_Text_Blink_091558", rust: "system::title::text_shown", run: |m| title::text_shown(m) },
    SystemPort { entry: 0x09_1578, original: "Continue_Text_Countdown_091578", rust: "system::title::countdown", run: |m| title::countdown(m) },
    SystemPort { entry: 0x09_1594, original: "Continue_Text_Countdown_091578+1C", rust: "system::title::countdown_tick", run: |m| title::countdown_tick(m) },
    SystemPort { entry: 0x09_15DC, original: "Continue_Text_DrawRows_0915dc", rust: "system::title::draw_message",
                 run: |m| {
                     let s = m.cpu.d[0] as u16;
                     title::draw_message(m, s);
                     Flow::Return
                 } },
    SystemPort { entry: 0x09_1618, original: "Continue_Text_DrawTitle_091618", rust: "system::title::draw_copyright", run: ret!(title::draw_copyright) },
    SystemPort { entry: 0x09_1630, original: "Continue_Text_DrawCredits_091630", rust: "system::title::banner", run: |m| title::banner(m) },
    SystemPort { entry: 0x05_2328, original: "Credits_State_052328", rust: "system::title::credit_state",
                 run: |m| {
                     let s = title::credit_state(m);
                     m.cpu.d[0] = u32::from(s);
                     Flow::Return
                 } },
    // --- bajas de tareas desde la propia tarea
    SystemPort { entry: 0x00_0B90, original: "TaskSlots_BootInstall_000A7C+0x114", rust: "system::tasks::idle", run: |m| tasks::idle(m) },
    SystemPort { entry: 0x00_0400, original: "Task_IdleRts_000400", rust: "system::tasks::idle", run: |m| tasks::idle(m) },
    SystemPort { entry: 0x02_9582, original: "JsrPcThunk_029582", rust: "system::tasks::clear_slug_frame_flag", run: |m| tasks::clear_slug_frame_flag(m) },
    SystemPort { entry: 0x03_2406, original: "JsrPcThunk_032406", rust: "system::tasks::clear_players_frame_flag",
                 run: |m| tasks::clear_players_frame_flag(m, mslug_game::layout::task::FRAME_FLAG_8D) },
    SystemPort { entry: 0x03_240C, original: "JsrPcThunk_03240c", rust: "system::tasks::clear_players_frame_flag",
                 run: |m| tasks::clear_players_frame_flag(m, mslug_game::layout::task::FRAME_FLAG_88) },
    SystemPort { entry: 0x00_0518, original: "Task_Finish_000518", rust: "system::tasks::finish", run: |m| tasks::finish(m) },
    SystemPort { entry: 0x00_052A, original: "Task_Finishing_00052A", rust: "system::tasks::finishing", run: |m| tasks::finishing(m) },
    SystemPort { entry: 0x00_06E2, original: "Scheduler_NextTask_0006E2", rust: "system::tasks::next_task", run: |m| tasks::next_task(m) },
    // --- capa fija: créditos, nivel, marco
    SystemPort { entry: 0x00_1DB8, original: "Hud_DrawCreditsAndOverlay_001db8", rust: "system::hud::draw", run: ret!(hud::draw) },
    SystemPort { entry: 0x05_2050, original: "FixOverlay_DrawCreditsOrFree_052050", rust: "system::hud::draw_credits", run: ret!(hud::draw_credits) },
    SystemPort { entry: 0x05_223A, original: "FixOverlay_DrawLevel_05223A", rust: "system::hud::draw_level", run: ret!(hud::draw_level) },
    SystemPort { entry: 0x04_6AC6, original: "FixLayer_QuadBatch_046AC6", rust: "system::hud::clear_border", run: ret!(hud::clear_border) },
    SystemPort { entry: 0x05_DA9C, original: "Fix_BlitRect_05DA9C", rust: "system::hud::fill_rect",
                 run: |m| {
                     let (at, ch, cols, rows) = (m.cpu.a[1] as u16, m.cpu.d[0] as u16, m.cpu.d[1] as u16, m.cpu.d[2] as u16);
                     hud::fill_rect(m, at, ch, cols, rows);
                     Flow::Return
                 } },
    SystemPort { entry: 0x05_DA56, original: "ThunkTarget_05da56", rust: "system::hud::fill_tiles",
                 run: |m| {
                     let (at, ch, cols, rows) = (m.cpu.a[1] as u16, m.cpu.d[0] as u16, m.cpu.d[1] as u16, m.cpu.d[2] as u16);
                     hud::fill_tiles(m, at, ch, cols, rows);
                     Flow::Return
                 } },
    SystemPort { entry: 0x05_DAD8, original: "Fix_BlitStream_05DAD8", rust: "system::hud::draw_text",
                 run: |m| {
                     let (at, text, attr) = (m.cpu.a[1] as u16, m.cpu.a[2], m.cpu.d[0] as u16);
                     m.cpu.a[2] = hud::draw_text(m, at, text, attr);
                     Flow::Return
                 } },
    SystemPort { entry: 0x05_CACE, original: "Camera_ResetCenter_05CACE", rust: "system::script::reset_camera", run: ret!(script::reset_camera) },
    SystemPort { entry: 0x05_E998, original: "Rng_Seed_05e998", rust: "system::rng::seed",
                 run: |m| {
                     let s = m.cpu.d[0] as u16;
                     rng::seed(m, s);
                     Flow::Return
                 } },
    SystemPort { entry: 0x02_4FEC, original: "PlayerCtx_ResetTwoBlocks_024FEC", rust: "system::pads::start_demo",
                 run: |m| {
                     let d = m.cpu.d[0] as u16;
                     pads::start_demo(m, d);
                     Flow::Return
                 } },
    SystemPort { entry: 0x01_3D20, original: "SetRamByteFF_013d20", rust: "system::pads::set_pause_enabled",
                 run: |m| {
                     pads::set_pause_enabled(m, true);
                     Flow::Return
                 } },
    SystemPort { entry: 0x01_3D2A, original: "Pause_Disable_013d2a", rust: "system::pads::set_pause_enabled",
                 run: |m| {
                     pads::set_pause_enabled(m, false);
                     Flow::Return
                 } },
    // --- vídeo
    SystemPort { entry: 0x01_3600, original: "Entity_FlushSlotHistory_013600", rust: "system::video::release_task_entries",
                 run: |m| {
                     let t = m.cpu.a[6];
                     video::release_task_entries(m, t);
                     Flow::Return
                 } },
    SystemPort { entry: 0x00_2BC4, original: "PalSlot_Release_002bc4", rust: "system::video::release_entry",
                 run: |m| {
                     let i = m.cpu.d[1] as u16;
                     video::release_entry(m, i);
                     Flow::Return
                 } },
    SystemPort { entry: 0x00_2AEC, original: "PalSlot_LoadListLo_002aec", rust: "system::video::load_low_entries",
                 run: |m| {
                     let l = m.cpu.a[0];
                     m.cpu.a[0] = video::load_low_entries(m, l);
                     Flow::Return
                 } },
    SystemPort { entry: 0x00_2B58, original: "PalSlot_LoadListHi_002b58", rust: "system::video::load_high_entries",
                 run: |m| {
                     let l = m.cpu.a[0];
                     m.cpu.a[0] = video::load_high_entries(m, l);
                     Flow::Return
                 } },
    SystemPort { entry: 0x00_1DA4, original: "Task_InstallBootSlots_001da4", rust: "system::video::load_fixed_entries",
                 run: ret!(video::load_fixed_entries) },
    SystemPort { entry: 0x01_390E, original: "Scratch_Alloc_01390E", rust: "system::video::split_sprites",
                 run: |m| {
                     let (a, b, c) = (m.cpu.d[0] as u16, m.cpu.d[2] as u16, m.cpu.d[1] as u16);
                     video::split_sprites(m, a, b, c);
                     Flow::Return
                 } },
    SystemPort { entry: 0x00_1E1C, original: "Scratch_AllocZero_001e1c", rust: "system::video::reset_sprites", run: ret!(video::reset_sprites) },
    SystemPort { entry: 0x01_399C, original: "SpriteRange_DisableAll_01399C", rust: "system::video::hide_sprites",
                 run: |m| {
                     let (a, b) = (m.cpu.d[0] as u16, m.cpu.d[1] as u16);
                     video::hide_sprites(m, a, b);
                     Flow::Return
                 } },
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
