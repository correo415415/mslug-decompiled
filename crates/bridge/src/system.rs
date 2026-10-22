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
use mslug_game::system::{anim, autopilot, boot, collision, events, food, fort, frame, game_over, go_arrow, ground_fx, grunt, hits, hud, icons, irq, items, layer, lists, manual, manual_icons, mission, motion, pads, pickup, platforms, player, player_hud, palette, rank, render, rng, scene, script, scroll, slope, slug, sound, sprite, tasks, title, turret, video, wheels};
use mslug_game::{Flow, Machine, Memory, Regs};
use mslug_game::input::{Buttons, Edge};
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
    fn halted(&mut self) -> bool {
        self.halted.is_some()
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

/// Alta de una tarea de fundido (registros como `Task_Alloc_0004AE`).
fn fade_spawn(m: &mut CpuMachine, kind: usize) -> Flow {
    use mslug_game::layout::task as T;
    let parent = m.cpu.a[6];
    let next = m.read_u32(parent + T::NEXT);
    let t = palette::spawn_fade(m, kind, parent, m.cpu.d[0] as u16);
    m.cpu.a[0] = t;
    if t != T::OVERFLOW_BLOCK {
        m.cpu.a[1] = next;
    } else {
        m.cpu.a[1] = mslug_game::layout::fade::SPAWNERS[kind].1;
    }
    Flow::Return
}

/// Registros que deja una ventana pintada (los de `$051D84`).
fn window_regs(m: &mut CpuMachine, w: layer::Window) {
    let w16 = |r: u32, v: u16| (r & 0xFFFF_0000) | u32::from(v);
    m.cpu.d[0] = w16(m.cpu.d[0], w.ring.0);
    m.cpu.d[1] = w16(m.cpu.d[1], w.ring.1);
    m.cpu.d[2] = w16(m.cpu.d[2], w.map.0);
    m.cpu.d[3] = w16(m.cpu.d[3], w.map.1);
    m.cpu.d[4] = u32::from(w.size.0);
    m.cpu.d[5] = u32::from(w.size.1);
    m.cpu.d[6] = w16(m.cpu.d[6], w.map_h);
}

#[derive(Clone, Copy)]
enum Paint {
    Refresh,
    X,
    Y,
}

fn paint(m: &mut CpuMachine, p: Paint, cam: u32) -> Option<layer::Window> {
    match p {
        Paint::Refresh => layer::refresh(m, cam),
        Paint::X => layer::scroll_x(m, cam),
        Paint::Y => layer::scroll_y(m, cam),
    }
}

fn layer_port(m: &mut CpuMachine, p: Paint) -> Flow {
    let cam = m.cpu.a[0];
    m.cpu.a[1] = m.read_u32(cam + mslug_game::layout::scroll::C_PATH);
    let w = paint(m, p, cam);
    if let Some(w) = w {
        window_regs(m, w);
    }
    set_carry(m, w.is_some());
    Flow::Return
}

/// Ganchos de cámara: pintan y, si hay mapa de colisión, copian su trozo.
fn hook_port(m: &mut CpuMachine, p: Paint) -> Flow {
    let cam = m.cpu.a[0];
    let w = paint(m, p, cam);
    if let Some(w) = w {
        window_regs(m, w);
        let src = m.read_u32(cam + mslug_game::layout::scroll::C_COLLMAP_SRC);
        m.cpu.d[7] = src;
        if src != 0 {
            layer::copy_collision(m, cam, src, w);
        }
    }
    set_carry(m, w.is_some());
    Flow::Return
}

/// Registros que deja `$03EE48`: d0 = jugador, d2 = jugador + 1, a0 =
/// su entidad; C = alguno toca.
fn toucher_regs(m: &mut CpuMachine, r: Option<(u8, u32)>) {
    if let Some((p, e)) = r {
        m.cpu.d[0] = (m.cpu.d[0] & 0xFFFF_FF00) | u32::from(p);
        m.cpu.d[2] = (m.cpu.d[2] & 0xFFFF_FF00) | u32::from(p + 1);
        m.cpu.a[0] = e;
    }
    set_carry(m, r.is_some());
}

/// `$05DD5C` deja en a0 los márgenes usados.
fn out_regs(m: &mut CpuMachine, a: u32) {
    if a == 0xFFFF_FFFF {
        m.cpu.a[0] = mslug_game::layout::items::DEFAULT_MARGINS;
    }
}

/// Registros que deja el pulso de la caja de golpe (`$088438`/`$08846A`).
fn pulse_regs(m: &mut CpuMachine, t: u32, from_table: bool) {
    let n = u32::from(m.read_u16(t + mslug_game::layout::fort::TIMER));
    if n & 3 != 0 {
        m.cpu.d[0] = (m.cpu.d[0] & 0xFFFF_0000) | (n & 0xFF03);
        m.cpu.a[0] = mslug_game::layout::fort::NO_HITBOX;
    } else if from_table {
        m.cpu.d[0] = u32::from(m.read_u8(t + mslug_game::layout::fort::STATE)) * 4;
        m.cpu.a[0] = m.read_u32(t + mslug_game::layout::fort::HITBOX);
    } else {
        m.cpu.d[0] = (m.cpu.d[0] & 0xFFFF_0000) | (n & 0xFF00);
    }
}

/// Registros de `$05E4CA`: a0 = dueño, d0/d1/d2 = atributos, X, Y.
fn owner_regs(m: &mut CpuMachine, t: u32, a: u16, x: u16, y: u16) {
    m.cpu.a[0] = m.read_u32(t + mslug_game::layout::task::OWNER);
    for (k, v) in [a, x, y].into_iter().enumerate() {
        m.cpu.d[k] = (m.cpu.d[k] & 0xFFFF_0000) | u32::from(v);
    }
}

/// Registros que dejan los tests de bit de máscara (`a0` = máscara, `d0` =
/// byte de la fila, `d6` = número de bit).
fn mask_regs(m: &mut CpuMachine, record: u32, off: u32, row: u16) {
    let b = slope::mask_byte(m, record, off, row);
    m.cpu.a[0] = m.read_u32(record);
    m.cpu.d[0] = (m.cpu.d[0] & 0xFFFF_FF00) | u32::from(b);
    m.cpu.d[6] = (m.cpu.d[6] & 0xFFFF_0000) | u32::from(7u16.wrapping_sub(m.cpu.d[3] as u16));
}

/// ¿Alguno de los botones `b` en el mando que controla la entidad `a6`?
/// (deja la respuesta en C).
fn pad_test(m: &mut CpuMachine, b: u8, edge: Edge) -> Flow {
    let e = mslug_game::entity::Entity::from_raw(m.cpu.a[6]);
    let c = mslug_game::input::pressed(m, e, Buttons(b), edge);
    set_carry(m, c);
    Flow::Return
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
    SystemPort { entry: 0x00_06FE, original: "Task_AllocLast_0006FE", rust: "system::tasks::spawn_last",
                 run: |m| {
                     let (t, after) = tasks::spawn_last(m, m.cpu.a[1], m.cpu.a[6]);
                     m.cpu.a[0] = t;
                     if t != mslug_game::layout::task::OVERFLOW_BLOCK {
                         m.cpu.a[1] = after;
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
    SystemPort { entry: 0x00_1846, original: "Attract_DoubleCheck_400_Publish_001846", rust: "system::script::step_start_players",
                 run: |m| script::step_start_players(m) },
    SystemPort { entry: 0x00_1260, original: "Init_ModeToggle_001260", rust: "system::script::scene_demo", run: |m| script::scene_demo(m) },
    SystemPort { entry: 0x00_12CA, original: "Init_ModeToggle_001260 (+$6A)", rust: "system::script::scene_demo_wait", run: |m| script::scene_demo_wait(m) },
    SystemPort { entry: 0x00_1922, original: "Dispatcher_ModeTable_001922", rust: "system::script::mode(0)", run: |m| script::mode(m, 0) },
    SystemPort { entry: 0x00_1940, original: "Dispatcher_ModeTable_001922 (estado 1)", rust: "system::script::mode(1)", run: |m| script::mode(m, 1) },
    SystemPort { entry: 0x00_195E, original: "Dispatcher_ModeTable_001922 (estado 2)", rust: "system::script::mode(2)", run: |m| script::mode(m, 2) },
    SystemPort { entry: 0x00_197C, original: "Dispatcher_ModeTable_001922 (estado 3)", rust: "system::script::mode(3)", run: |m| script::mode(m, 3) },
    SystemPort { entry: 0x00_199A, original: "Dispatcher_ModeTable_001922 (estado 4)", rust: "system::script::mode(4)", run: |m| script::mode(m, 4) },
    SystemPort { entry: 0x00_19B8, original: "Dispatcher_ModeTable_001922 (estado 5)", rust: "system::script::mode(5)", run: |m| script::mode(m, 5) },
    SystemPort { entry: 0x00_19D6, original: "Dispatcher_ModeTable_001922 (estado 6)", rust: "system::script::mode(6)", run: |m| script::mode(m, 6) },
    SystemPort { entry: 0x00_19FE, original: "Dispatcher_ModeTable_001922 (estado 7)", rust: "system::script::mode(7)", run: |m| script::mode(m, 7) },
    SystemPort { entry: 0x00_1A1C, original: "Dispatcher_ModeTable_001922 (estado 8)", rust: "system::script::mode(8)", run: |m| script::mode(m, 8) },
    SystemPort { entry: 0x00_1A3A, original: "Dispatcher_ModeTable_001922 (estado 9)", rust: "system::script::mode(9)", run: |m| script::mode(m, 9) },
    SystemPort { entry: 0x00_1A58, original: "Dispatcher_ModeTable_001922 (estado 10)", rust: "system::script::mode(10)", run: |m| script::mode(m, 10) },
    SystemPort { entry: 0x00_1DCC, original: "Task_InstallMissionSlots_001dcc", rust: "system::script::install_mission_slots", run: ret!(script::install_mission_slots) },
    SystemPort { entry: 0x00_1C88, original: "Hud_Delay1200_ClearDirty_001c88", rust: "system::script::demo_timeout", run: |m| script::demo_timeout(m) },
    SystemPort { entry: 0x00_1C94, original: "$001C94", rust: "system::script::demo_timeout_run", run: |m| script::demo_timeout_run(m) },
    SystemPort { entry: 0x00_1C44, original: "Banner_DelayThenStart_001c44", rust: "system::script::mission_credits", run: |m| script::mission_credits(m) },
    SystemPort { entry: 0x00_1C74, original: "$001C74", rust: "system::script::mission_credits_wait", run: |m| script::mission_credits_wait(m) },
    SystemPort { entry: 0x04_69E2, original: "Continue_StoreCount_Exit_0469e2", rust: "system::script::store_hud_mode", run: |m| script::store_hud_mode(m) },
    SystemPort { entry: 0x28_DB5A, original: "$28DB5A", rust: "system::script::set_flag_task(0, on)", run: |m| script::set_flag_task(m, 0, true) },
    SystemPort { entry: 0x28_DB6A, original: "$28DB6A", rust: "system::script::set_flag_task(0, off)", run: |m| script::set_flag_task(m, 0, false) },
    SystemPort { entry: 0x02_9588, original: "ChildRank_CmpByte10_029566 (+$22)", rust: "system::script::set_flag_task(1, on)", run: |m| script::set_flag_task(m, 1, true) },
    SystemPort { entry: 0x02_9598, original: "ChildRank_CmpByte10_029566 (+$32)", rust: "system::script::set_flag_task(1, off)", run: |m| script::set_flag_task(m, 1, false) },
    SystemPort { entry: 0x00_1B4C, original: "Rank_DelayStart_001b4c", rust: "system::rank::task", run: |m| rank::task(m) },
    SystemPort { entry: 0x00_1B70, original: "Rank_DelayTick_001b70", rust: "system::rank::delay", run: |m| rank::delay(m) },
    SystemPort { entry: 0x00_1B80, original: "Rank_DelayGate_001b80", rust: "system::rank::run", run: |m| rank::run(m) },
    SystemPort { entry: 0x00_1C34, original: "Timer_WaitFlag21_001c34", rust: "system::rank::wait", run: |m| rank::wait(m) },
    SystemPort { entry: 0x04_7482, original: "$047482", rust: "system::rank::draw", run: |m| { let show = m.cpu.d[1] as u8 != 0; rank::draw(m, show); Flow::Return } },
    SystemPort { entry: 0x07_9970, original: "$079970", rust: "system::rank::start_time",
                 run: |m| { let v = rank::start_time(m); m.cpu.d[0] = u32::from(v); m.cpu.a[3] = 0x10_FD84; m.cpu.a[0] = mslug_game::layout::rank::START_TIMES; Flow::Return } },
    SystemPort { entry: 0x02_F85A, original: "$02F85A", rust: "system::rank::player_flag3",
                 run: |m| { let c = rank::player_flag3(m); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x09_67FE, original: "Attract_State0_Handler_0967FE", rust: "system::mission::start(0)", run: |m| mission::start(m, 0) },
    SystemPort { entry: 0x09_6840, original: "Attract_State1_Handler_096840", rust: "system::mission::start(1)", run: |m| mission::start(m, 1) },
    SystemPort { entry: 0x09_6882, original: "Attract_State2_Handler_096882", rust: "system::mission::start(2)", run: |m| mission::start(m, 2) },
    SystemPort { entry: 0x09_68C4, original: "Attract_State3_Handler_0968C4", rust: "system::mission::start(3)", run: |m| mission::start(m, 3) },
    SystemPort { entry: 0x09_6906, original: "Attract_State4_Handler_096906", rust: "system::mission::start(4)", run: |m| mission::start(m, 4) },
    SystemPort { entry: 0x09_6948, original: "Attract_State5_Handler_096948", rust: "system::mission::start(5)", run: |m| mission::start(m, 5) },
    SystemPort { entry: 0x09_698A, original: "Attract_State7_Handler_09698A", rust: "system::mission::start(7)", run: |m| mission::start(m, 7) },
    SystemPort { entry: 0x09_6820, original: "$096820", rust: "system::mission::run", run: |m| mission::run(m) },
    SystemPort { entry: 0x09_6862, original: "$096862", rust: "system::mission::run", run: |m| mission::run(m) },
    SystemPort { entry: 0x09_68A4, original: "$0968A4", rust: "system::mission::run", run: |m| mission::run(m) },
    SystemPort { entry: 0x09_68E6, original: "$0968E6", rust: "system::mission::run", run: |m| mission::run(m) },
    SystemPort { entry: 0x09_6928, original: "$096928", rust: "system::mission::run", run: |m| mission::run(m) },
    SystemPort { entry: 0x09_696A, original: "$09696A", rust: "system::mission::run", run: |m| mission::run(m) },
    SystemPort { entry: 0x09_69A6, original: "$0969A6", rust: "system::mission::run", run: |m| mission::run(m) },
    SystemPort { entry: 0x09_6B7E, original: "$096B7E", rust: "system::mission::spawn_table",
                 run: |m| { let k = m.cpu.d[0] as u8; m.cpu.d[0] = u32::from(k.min(7)) * 4; m.cpu.a[0] = mission::spawn_table(m, k); Flow::Return } },
    SystemPort { entry: 0x05_DCCE, original: "$05DCCE", rust: "system::mission::spawn",
                 run: |m| { let e = m.cpu.a[1]; let c = mission::spawn(m, e); m.cpu.a[0] = c; Flow::Return } },
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
    // --- manual de juego
    SystemPort { entry: 0x08_C008, original: "Anim_State_F1_08C008", rust: "system::manual::start", run: |m| manual::start(m) },
    SystemPort { entry: 0x08_C10A, original: "Anim_State_F1_08C008 (+$102)", rust: "system::manual::fade_in", run: |m| manual::fade_in(m) },
    SystemPort { entry: 0x08_C15E, original: "Anim_State_F2_08C15E", rust: "system::manual::light_up", run: |m| manual::light_up(m) },
    SystemPort { entry: 0x08_C1AC, original: "Anim_State_F3_08C1AC", rust: "system::manual::steps", run: |m| manual::steps(m) },
    SystemPort { entry: 0x08_C1EA, original: "Anim_State_F4_08C1EA", rust: "system::manual::dim", run: |m| manual::dim(m) },
    SystemPort { entry: 0x08_C23A, original: "Anim_State_F5_08C23A", rust: "system::manual::fade_out", run: |m| manual::fade_out(m) },
    SystemPort { entry: 0x08_C296, original: "Anim_State_F6_08C296", rust: "system::manual::wait_end", run: |m| manual::wait_end(m) },
    SystemPort { entry: 0x08_BC74, original: "Anim_ScriptStep_08bc74", rust: "system::manual::step_script",
                 run: |m| { let t = m.cpu.a[6]; let c = manual::step_script(m, t); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x05_239E, original: "Fade_Spawn_05239E", rust: "system::palette::spawn_fade", run: |m| fade_spawn(m, 0) },
    SystemPort { entry: 0x05_23B2, original: "Fade_Spawn_0523B2", rust: "system::palette::spawn_fade", run: |m| fade_spawn(m, 1) },
    SystemPort { entry: 0x05_23C6, original: "Fade_Spawn_0523C6", rust: "system::palette::spawn_fade", run: |m| fade_spawn(m, 2) },
    SystemPort { entry: 0x05_23DA, original: "Fade_Spawn_0523DA", rust: "system::palette::spawn_fade", run: |m| fade_spawn(m, 3) },
    // --- iconos del manual
    SystemPort { entry: 0x08_C2B8, original: "Icon_Base_08c2b8", rust: "system::manual_icons::base", run: |m| manual_icons::base(m) },
    SystemPort { entry: 0x08_C2F0, original: "Icon_Base_08c2b8 (+$38)", rust: "system::manual_icons::base_run", run: |m| manual_icons::base_run(m) },
    SystemPort { entry: 0x08_C322, original: "Icon_Slot1_08c322", rust: "system::manual_icons::slot", run: |m| manual_icons::slot(m, 0) },
    SystemPort { entry: 0x08_C37E, original: "Icon_Slot2_08c37e", rust: "system::manual_icons::slot", run: |m| manual_icons::slot(m, 1) },
    SystemPort { entry: 0x08_C3DA, original: "Icon_Slot3_08c3da", rust: "system::manual_icons::slot", run: |m| manual_icons::slot(m, 2) },
    SystemPort { entry: 0x08_C436, original: "Icon_Slot4_08c436", rust: "system::manual_icons::slot", run: |m| manual_icons::slot(m, 3) },
    SystemPort { entry: 0x08_C4A0, original: "Icon_Slot_Run_08c4a0", rust: "system::manual_icons::slot_run", run: |m| manual_icons::slot_run(m) },
    SystemPort { entry: 0x08_C4FA, original: "Icon_Slot1_Lit_08c4fa", rust: "system::manual_icons::slot_lit", run: |m| manual_icons::slot_lit(m, 0) },
    SystemPort { entry: 0x08_C526, original: "Icon_Slot2_Lit_08c526", rust: "system::manual_icons::slot_lit", run: |m| manual_icons::slot_lit(m, 1) },
    SystemPort { entry: 0x08_C552, original: "Icon_Slot3_Lit_08c552", rust: "system::manual_icons::slot_lit", run: |m| manual_icons::slot_lit(m, 2) },
    SystemPort { entry: 0x08_C57E, original: "Icon_Slot4_Lit_08c57e", rust: "system::manual_icons::slot_lit", run: |m| manual_icons::slot_lit(m, 3) },
    SystemPort { entry: 0x08_C5AA, original: "JsrAbsThunk_08c5aa", rust: "system::manual_icons::animate", run: |m| manual_icons::animate(m) },
    SystemPort { entry: 0x08_C5B2, original: "Icon_Anchor_Init_08c5b2", rust: "system::manual_icons::anchor", run: |m| manual_icons::anchor(m) },
    SystemPort { entry: 0x02_67E2, original: "$0267E2", rust: "system::manual_icons::stop_motion",
                 run: |m| { let t = m.cpu.a[6]; manual_icons::stop_motion(m, t); Flow::Return } },
    // --- Slug
    SystemPort { entry: 0x02_ACFC, original: "Slug_IsAlive_02acfc", rust: "system::slug::is_alive",
                 run: |m| { let c = slug::is_alive(m); m.cpu.a[0] = mslug_game::layout::task::SLUG; set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x02_AE50, original: "Slug_SpawnDrop_02ae50", rust: "system::slug::spawn_drop", run: |m| slug::spawn_drop(m, false) },
    SystemPort { entry: 0x02_AEA4, original: "Slug_SpawnDropB_02aea4", rust: "system::slug::spawn_drop", run: |m| slug::spawn_drop(m, true) },
    SystemPort { entry: 0x02_A0F8, original: "Slug_Init_02a0f8", rust: "system::slug::init",
                 run: |m| { let (t, g) = (m.cpu.a[6], m.cpu.d[1] as u16); slug::init(m, t, g); Flow::Return } },
    SystemPort { entry: 0x01_38FE, original: "Entity_AddTimer0_74_0138FE", rust: "system::slug::add_timer",
                 run: |m| { let t = m.cpu.a[6]; slug::add_timer(m, t); Flow::Return } },
    SystemPort { entry: 0x05_E98A, original: "Entity_MarkFlag2TimerMax_05e98a", rust: "system::slug::mark_timer_max",
                 run: |m| { let t = m.cpu.a[6]; slug::mark_timer_max(m, t); Flow::Return } },
    SystemPort { entry: 0x08_F6D2, original: "PlayerSlot_MaskF0F0_08f6d2", rust: "system::slug::mask_players",
                 run: |m| { let t = m.cpu.a[6]; slug::mask_players(m, t); Flow::Return } },
    SystemPort { entry: 0x02_FADA, original: "Slug_ResetDamageIdx_02fada", rust: "system::slug::reset_damage",
                 run: |m| { let t = m.cpu.a[6]; slug::reset_damage(m, t); Flow::Return } },
    SystemPort { entry: 0x03_060A, original: "Entity_Build3ChainCircular_03060A", rust: "system::slug::build_chain",
                 run: |m| {
                     let t = m.cpu.a[6];
                     let first = slug::build_chain(m, t);
                     m.cpu.a[0] = first;
                     m.cpu.a[2] = m.read_u32(t + 0x74);
                     m.cpu.a[3] = m.read_u32(t + 0x78);
                     Flow::Return
                 } },
    SystemPort { entry: 0x02_83CA, original: "Entity_ClearFlags13Bits12", rust: "system::slug::clear_flags13",
                 run: |m| { let t = m.cpu.a[6]; slug::clear_flags13(m, t); Flow::Return } },
    SystemPort { entry: 0x02_AEF8, original: "Slug_DropVariant0_02aef8", rust: "system::slug::drop_in", run: |m| slug::drop_in(m, 0) },
    SystemPort { entry: 0x02_AF1C, original: "Slug_DropVariant1_02af1c", rust: "system::slug::drop_in", run: |m| slug::drop_in(m, 1) },
    SystemPort { entry: 0x02_AF40, original: "Slug_DropVariant2_02af40", rust: "system::slug::drop_in", run: |m| slug::drop_in(m, 2) },
    SystemPort { entry: 0x02_AF64, original: "Slug_DropVariant3_02af64", rust: "system::slug::drop_in", run: |m| slug::drop_in(m, 3) },
    SystemPort { entry: 0x02_AF88, original: "Slug_DropVariant4_02af88", rust: "system::slug::drop_in", run: |m| slug::drop_in(m, 4) },
    // --- Game Over / Continue
    SystemPort { entry: 0x08_F91A, original: "GameOver_Boot_08f91a", rust: "system::game_over::boot", run: |m| game_over::boot(m) },
    SystemPort { entry: 0x08_F96A, original: "GameOver_Spawn_08f96a", rust: "system::game_over::spawn", run: |m| game_over::spawn(m) },
    SystemPort { entry: 0x08_FBA2, original: "GameOver_Spawn_08f96a (+$238)", rust: "system::game_over::fall", run: |m| game_over::fall(m) },
    SystemPort { entry: 0x08_FCAA, original: "GameOver_IntegrateY_08fcaa", rust: "system::game_over::integrate_y",
                 run: |m| { let t = m.cpu.a[6]; game_over::integrate_y(m, t); Flow::Return } },
    SystemPort { entry: 0x08_FCCA, original: "GameOver_Wait_08fcca", rust: "system::game_over::landed", run: |m| game_over::landed(m) },
    SystemPort { entry: 0x08_FD2E, original: "GameOver_Wait2_08fd2e", rust: "system::game_over::settle", run: |m| game_over::settle(m) },
    SystemPort { entry: 0x08_FD68, original: "GameOver_Final_08fd68", rust: "system::game_over::final_phase", run: |m| game_over::final_phase(m) },
    SystemPort { entry: 0x08_FDAA, original: "GameOver_WaitCredit_08fdaa", rust: "system::game_over::wait_credit", run: |m| game_over::wait_credit(m) },
    SystemPort { entry: 0x08_FDF8, original: "GameOver_Continue_08fdf8", rust: "system::game_over::continue_phase", run: |m| game_over::continue_phase(m) },
    SystemPort { entry: 0x08_FE32, original: "GameOver_ContinuePath_08fe32", rust: "system::game_over::path", run: |m| game_over::path(m) },
    SystemPort { entry: 0x08_FE9C, original: "GameOver_ContinueHold_08fe9c", rust: "system::game_over::hold", run: |m| game_over::hold(m) },
    SystemPort { entry: 0x08_FEB6, original: "GameOver_ContinueEnd_08feb6", rust: "system::game_over::end", run: |m| game_over::end(m) },
    SystemPort { entry: 0x08_FECA, original: "TaskHandler_08feca", rust: "system::tasks::idle", run: |m| tasks::idle(m) },
    SystemPort { entry: 0x09_1618, original: "Continue_Text_DrawTitle_091618", rust: "system::game_over::draw_title",
                 run: |m| { m.cpu.a[2] = game_over::draw_title(m); Flow::Return } },
    SystemPort { entry: 0x05_23EE, original: "Template_0523EE", rust: "system::palette::fade_task", run: |m| palette::fade_task(m, true, false) },
    SystemPort { entry: 0x05_23FA, original: "Template_0523FA", rust: "system::palette::fade_task", run: |m| palette::fade_task(m, true, true) },
    SystemPort { entry: 0x05_24AA, original: "Template_0524AA", rust: "system::palette::fade_task", run: |m| palette::fade_task(m, false, false) },
    SystemPort { entry: 0x05_24B6, original: "Template_0524B6", rust: "system::palette::fade_task", run: |m| palette::fade_task(m, false, true) },
    SystemPort { entry: 0x05_2454, original: "Template_0523EE (+$66)", rust: "system::palette::fade_loop", run: |m| palette::fade_loop(m, true) },
    SystemPort { entry: 0x05_2514, original: "Template_0524AA (+$6A)", rust: "system::palette::fade_loop", run: |m| palette::fade_loop(m, false) },
    SystemPort { entry: 0x00_2C30, original: "$002C30", rust: "system::video::start_entry_anim",
                 run: |m| {
                     let (i, g, n, md) = (m.cpu.d[1] as u16, m.cpu.d[2] as u16, m.cpu.d[3] as u16, m.cpu.d[4] as u8);
                     video::start_entry_anim(m, i, g, n, md);
                     let e = mslug_game::layout::sprites::TABLE;
                     m.cpu.a[1] = e;
                     m.cpu.a[2] = mslug_game::layout::sprites::ANIM_DATA;
                     m.cpu.d[1] = (m.cpu.d[1] & 0xFFFF_0000) | u32::from(i.wrapping_shl(5));
                     m.cpu.d[2] = u32::from(g) * 64 + mslug_game::layout::sprites::ANIM_DATA;
                     Flow::Return
                 } },
    SystemPort { entry: 0x00_2C26, original: "$002C26", rust: "system::video::start_high_entry_anim",
                 run: |m| {
                     let (i, g, n, md) = (m.cpu.d[1] as u16, m.cpu.d[2] as u16, m.cpu.d[3] as u16, m.cpu.d[4] as u8);
                     video::start_high_entry_anim(m, i, g, n, md);
                     m.cpu.a[1] = mslug_game::layout::sprites::TABLE;
                     m.cpu.a[2] = mslug_game::layout::sprites::HIGH_LIST_DATA;
                     m.cpu.d[1] = (m.cpu.d[1] & 0xFFFF_0000) | u32::from(i.wrapping_shl(5));
                     m.cpu.d[2] = u32::from(g) * 64 + mslug_game::layout::sprites::HIGH_LIST_DATA;
                     Flow::Return
                 } },
    SystemPort { entry: 0x09_0B08, original: "GO_Letter_Common_090b08", rust: "system::game_over::letter_run",
                 run: |m| { let t = m.cpu.a[6]; m.write_u8(t + 0x20, 0); m.write_u16(t + 0x82, 1);
                            m.write_u32(t, mslug_game::layout::game_over::LETTER_RUN); game_over::letter_run(m) } },
    SystemPort { entry: 0x09_0B18, original: "GO_Letter_Common_090b08 (+$52)", rust: "system::game_over::letter_run", run: |m| game_over::letter_run(m) },
    SystemPort { entry: 0x09_0C30, original: "GO_Glow_090c30", rust: "system::game_over::glow", run: |m| game_over::glow(m) },
    SystemPort { entry: 0x09_0C8E, original: "GO_Glow_090c30 (+$5E)", rust: "system::game_over::glow_run", run: |m| game_over::glow_run(m) },
    SystemPort { entry: 0x09_0728, original: "GO_Letter_V0_090728", rust: "system::game_over::letter", run: |m| game_over::letter(m, 0) },
    SystemPort { entry: 0x09_076E, original: "GO_Letter_V1_09076e", rust: "system::game_over::letter", run: |m| game_over::letter(m, 1) },
    SystemPort { entry: 0x09_07B4, original: "GO_Letter_V2_0907b4", rust: "system::game_over::letter", run: |m| game_over::letter(m, 2) },
    SystemPort { entry: 0x09_07FA, original: "GO_Letter_V3_0907fa", rust: "system::game_over::letter", run: |m| game_over::letter(m, 3) },
    SystemPort { entry: 0x09_0840, original: "GO_Letter_V4_090840", rust: "system::game_over::letter", run: |m| game_over::letter(m, 4) },
    SystemPort { entry: 0x09_088E, original: "GO_Letter_V5_09088e", rust: "system::game_over::letter", run: |m| game_over::letter(m, 5) },
    SystemPort { entry: 0x09_08D4, original: "GO_Letter_V6_0908d4", rust: "system::game_over::letter", run: |m| game_over::letter(m, 6) },
    SystemPort { entry: 0x09_091A, original: "GO_Letter_V7_09091a", rust: "system::game_over::letter", run: |m| game_over::letter(m, 7) },
    SystemPort { entry: 0x09_0968, original: "GO_Letter_V8_090968", rust: "system::game_over::letter", run: |m| game_over::letter(m, 8) },
    SystemPort { entry: 0x09_09AE, original: "GO_Letter_V9_0909ae", rust: "system::game_over::letter", run: |m| game_over::letter(m, 9) },
    SystemPort { entry: 0x09_09F4, original: "GO_Letter_V10_0909f4", rust: "system::game_over::letter", run: |m| game_over::letter(m, 10) },
    SystemPort { entry: 0x09_0A3A, original: "GO_Letter_V11_090a3a", rust: "system::game_over::letter", run: |m| game_over::letter(m, 11) },
    SystemPort { entry: 0x09_0A80, original: "GO_Letter_V12_090a80", rust: "system::game_over::letter", run: |m| game_over::letter(m, 12) },
    SystemPort { entry: 0x09_0AC6, original: "GO_Letter_V13_090ac6", rust: "system::game_over::letter", run: |m| game_over::letter(m, 13) },
    SystemPort { entry: 0x09_10A2, original: "GO_ParentState_0910a2", rust: "system::game_over::parent_state_is",
                 run: |m| { let t = m.cpu.a[6]; let c = game_over::parent_state_is(m, t, 1, 2); m.cpu.a[0] = m.read_u32(t + 0x0C); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x09_10BC, original: "GO_ParentState_0910bc", rust: "system::game_over::parent_state_is",
                 run: |m| { let t = m.cpu.a[6]; let c = game_over::parent_state_is(m, t, 0, 2); m.cpu.a[0] = m.read_u32(t + 0x0C); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x09_10D6, original: "GO_ParentState_0910d6", rust: "system::game_over::parent_state_is",
                 run: |m| { let t = m.cpu.a[6]; let c = game_over::parent_state_is(m, t, 0, 3); m.cpu.a[0] = m.read_u32(t + 0x0C); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x09_10F0, original: "GO_ParentState_0910f0", rust: "system::game_over::parent_state_is",
                 run: |m| { let t = m.cpu.a[6]; let c = game_over::parent_state_is(m, t, 1, 3); m.cpu.a[0] = m.read_u32(t + 0x0C); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x09_110A, original: "GO_ParentState_09110a", rust: "system::game_over::parent_state_is",
                 run: |m| { let t = m.cpu.a[6]; let c = game_over::parent_state_is(m, t, 1, 4); m.cpu.a[0] = m.read_u32(t + 0x0C); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x05_E9B6, original: "$05E9B6", rust: "system::rng::next",
                 run: |m| {
                     let (v, j) = rng::next(m);
                     m.cpu.d[0] = (m.cpu.d[0] & 0xFFFF_0000) | u32::from(v);
                     m.cpu.d[6] = (m.cpu.d[6] & 0xFFFF_0000) | 0x3E;
                     m.cpu.d[7] = (m.cpu.d[7] & 0xFFFF_0000) | u32::from(j);
                     m.cpu.a[4] = mslug_game::layout::rng::TABLE + u32::from(j);
                     Flow::Return
                 } },
    SystemPort { entry: 0x05_E9E4, original: "$05E9E4", rust: "system::rng::below",
                 run: |m| {
                     let n = m.cpu.d[0] as u16;
                     let (_, j) = rng::below(m, n);
                     let w = m.read_u16(mslug_game::layout::rng::TABLE + u32::from(j));
                     let prod = u32::from(w) * u32::from(n);
                     m.cpu.d[0] = prod.rotate_left(16);
                     m.cpu.d[5] = (m.cpu.d[5] & 0xFFFF_0000) | u32::from(n);
                     m.cpu.d[6] = (m.cpu.d[6] & 0xFFFF_0000) | 0x3E;
                     m.cpu.d[7] = (m.cpu.d[7] & 0xFFFF_0000) | u32::from(j);
                     m.cpu.a[4] = mslug_game::layout::rng::TABLE;
                     Flow::Return
                 } },
    SystemPort { entry: 0x05_EA1C, original: "$05EA1C", rust: "system::rng::masked",
                 run: |m| {
                     let n = m.cpu.d[0] as u16;
                     let (v, j) = rng::masked(m, n);
                     m.cpu.d[0] = (m.cpu.d[0] & 0xFFFF_0000) | u32::from(v);
                     m.cpu.d[5] = (m.cpu.d[5] & 0xFFFF_0000) | u32::from(n);
                     m.cpu.d[6] = (m.cpu.d[6] & 0xFFFF_0000) | 0x3E;
                     m.cpu.d[7] = (m.cpu.d[7] & 0xFFFF_0000) | u32::from(j);
                     m.cpu.a[4] = mslug_game::layout::rng::TABLE;
                     Flow::Return
                 } },
    SystemPort { entry: 0x09_0DB0, original: "GO_Figure_A_090db0", rust: "system::game_over::figure", run: |m| game_over::figure(m, false) },
    SystemPort { entry: 0x09_0DE2, original: "GO_Figure_B_090de2", rust: "system::game_over::figure", run: |m| game_over::figure(m, true) },
    SystemPort { entry: 0x09_0E22, original: "GO_Figure_Run_090e1c (+$06)", rust: "system::game_over::figure_run", run: |m| game_over::figure_run(m) },
    SystemPort { entry: 0x09_0E7E, original: "$090E7E", rust: "system::game_over::finish", run: |m| game_over::finish(m) },
    SystemPort { entry: 0x09_0E86, original: "GO_Figure_Part_090e86", rust: "system::game_over::part", run: |m| game_over::part(m) },
    SystemPort { entry: 0x09_0EC6, original: "GO_Figure_Part_090e86 (+$40)", rust: "system::game_over::part_run", run: |m| game_over::part_run(m) },
    SystemPort { entry: 0x09_0F00, original: "GO_Scroller_A_090f00", rust: "system::game_over::scroller", run: |m| game_over::scroller(m, 0) },
    SystemPort { entry: 0x09_0F68, original: "GO_Scroller_B_090f68", rust: "system::game_over::scroller", run: |m| game_over::scroller(m, 1) },
    SystemPort { entry: 0x09_0FCE, original: "GO_Scroller_Run_090fce", rust: "system::game_over::scroller_run", run: |m| game_over::scroller_run(m) },
    SystemPort { entry: 0x09_1096, original: "GO_CopyParam84_091096", rust: "system::game_over::copy_phase",
                 run: |m| { let t = m.cpu.a[6]; m.cpu.a[0] = game_over::copy_phase(m, t); Flow::Return } },
    SystemPort { entry: 0x09_1130, original: "GO_Particle_091130", rust: "system::game_over::particle", run: |m| game_over::particle(m) },
    SystemPort { entry: 0x09_128A, original: "GO_Particle_Run_09128a", rust: "system::game_over::particle_run", run: |m| game_over::particle_run(m) },
    SystemPort { entry: 0x09_034E, original: "GO_Zoom_09034e", rust: "system::game_over::zoom", run: |m| game_over::zoom(m) },
    SystemPort { entry: 0x09_03A4, original: "$0903A4", rust: "system::game_over::zoom_run", run: |m| game_over::zoom_run(m) },
    SystemPort { entry: 0x09_0452, original: "GO_Zoom_Path_090452", rust: "system::game_over::zoom_path", run: |m| game_over::zoom_path(m) },
    SystemPort { entry: 0x09_0474, original: "$090474", rust: "system::game_over::zoom_path_run", run: |m| game_over::zoom_path_run(m) },
    SystemPort { entry: 0x09_04C6, original: "GO_Zoom_Wait_0904c6", rust: "system::game_over::wait_timer", run: |m| game_over::wait_timer(m) },
    SystemPort { entry: 0x09_04E4, original: "GO_Shake_0904e4", rust: "system::game_over::shake", run: |m| game_over::shake(m) },
    SystemPort { entry: 0x09_0544, original: "$090544", rust: "system::game_over::shake_run", run: |m| game_over::shake_run(m) },
    SystemPort { entry: 0x09_0592, original: "GO_Flash_090592", rust: "system::game_over::flash", run: |m| game_over::flash(m) },
    SystemPort { entry: 0x09_05BA, original: "$0905BA", rust: "system::game_over::flash_run", run: |m| game_over::flash_run(m) },
    SystemPort { entry: 0x09_05C2, original: "GO_Banner_0905c2", rust: "system::game_over::banner(0)", run: |m| game_over::banner(m, 0) },
    SystemPort { entry: 0x09_0600, original: "$090600", rust: "system::game_over::banner_run", run: |m| game_over::banner_run(m) },
    SystemPort { entry: 0x09_0630, original: "GO_Banner_B_090630", rust: "system::game_over::banner(1)", run: |m| game_over::banner(m, 1) },
    SystemPort { entry: 0x09_066E, original: "$09066E", rust: "system::game_over::banner_b_run", run: |m| game_over::banner_b_run(m) },
    SystemPort { entry: 0x09_068C, original: "GO_Banner_C_09068c", rust: "system::game_over::banner(2)", run: |m| game_over::banner(m, 2) },
    SystemPort { entry: 0x09_06CA, original: "GO_Banner_Tick_0906ca", rust: "system::game_over::wait_timer", run: |m| game_over::wait_timer(m) },
    SystemPort { entry: 0x09_06E8, original: "GO_Banner_D_0906e8", rust: "system::game_over::banner(3)", run: |m| game_over::banner(m, 3) },
    SystemPort { entry: 0x09_1124, original: "GO_Particle_Add_091124", rust: "system::game_over::add_particle",
                 run: |m| {
                     let (t, after) = game_over::add_particle(m, m.cpu.a[6]);
                     m.cpu.a[0] = t;
                     if t != mslug_game::layout::task::OVERFLOW_BLOCK {
                         m.cpu.a[1] = after;
                         m.cpu.d[0] = (m.cpu.d[0] & 0xFFFF_FF00) | u32::from(m.read_u8(t + mslug_game::layout::task::PRIORITY));
                     } else {
                         m.cpu.a[1] = mslug_game::layout::game_over::PARTICLE;
                     }
                     Flow::Return
                 } },
    SystemPort { entry: 0x08_FECC, original: "GO_Sprite_Right_08fecc", rust: "system::game_over::slider(0)", run: |m| game_over::slider(m, 0) },
    SystemPort { entry: 0x08_FF2C, original: "GO_Sprite_Left_08ff2c", rust: "system::game_over::slider(1)", run: |m| game_over::slider(m, 1) },
    SystemPort { entry: 0x08_FF88, original: "GO_Sprite_Slide_08ff88", rust: "system::game_over::slide", run: |m| game_over::slide(m) },
    SystemPort { entry: 0x08_FFF6, original: "GO_Sprite_Mid_08fff6", rust: "system::game_over::follower(0)", run: |m| game_over::follower(m, 0) },
    SystemPort { entry: 0x09_004C, original: "GO_Sprite_MidB_09004c", rust: "system::game_over::follower(1)", run: |m| game_over::follower(m, 1) },
    SystemPort { entry: 0x09_0098, original: "GO_Sprite_FollowParent_090098", rust: "system::game_over::follow", run: |m| game_over::follow(m) },
    SystemPort { entry: 0x09_00E4, original: "GO_Prop_A_0900e4", rust: "system::game_over::prop_a", run: |m| game_over::prop_a(m) },
    SystemPort { entry: 0x09_0128, original: "$090128", rust: "system::game_over::prop_a_run", run: |m| game_over::prop_a_run(m) },
    SystemPort { entry: 0x09_016C, original: "GO_Prop_B_09016c", rust: "system::game_over::prop_b", run: |m| game_over::prop_b(m) },
    SystemPort { entry: 0x09_01B6, original: "$0901B6", rust: "system::game_over::prop_b_run", run: |m| game_over::prop_b_run(m) },
    SystemPort { entry: 0x09_02FE, original: "GO_Prop_C_0902fe", rust: "system::game_over::prop_c", run: |m| game_over::prop_c(m) },
    SystemPort { entry: 0x09_033C, original: "$09033C", rust: "system::game_over::prop_c_run", run: |m| game_over::prop_c_run(m) },
    SystemPort { entry: 0x09_0CBA, original: "GO_Glow_Drift_090cba", rust: "system::game_over::glow_drift", run: |m| game_over::glow_drift(m) },
    SystemPort { entry: 0x09_0D2E, original: "$090D2E", rust: "system::game_over::glow_drift_run", run: |m| game_over::glow_drift_run(m) },
    SystemPort { entry: 0x02_575C, original: "HUD_Task_StartP1_02575c", rust: "system::player_hud::start", run: |m| player_hud::start(m, 0) },
    SystemPort { entry: 0x02_5766, original: "HUD_Task_StartP2_025766", rust: "system::player_hud::start", run: |m| player_hud::start(m, 1) },
    SystemPort { entry: 0x02_57EC, original: "HUD_State_InsertCoin_0257ec", rust: "system::player_hud::insert_coin", run: |m| player_hud::insert_coin(m) },
    SystemPort { entry: 0x02_57FA, original: "$0257FA", rust: "system::player_hud::insert_coin_run", run: |m| player_hud::insert_coin_run(m) },
    SystemPort { entry: 0x02_5882, original: "HUD_State_Continue_025882", rust: "system::player_hud::push_start", run: |m| player_hud::push_start(m) },
    SystemPort { entry: 0x02_5890, original: "$025890", rust: "system::player_hud::push_start_run", run: |m| player_hud::push_start_run(m) },
    SystemPort { entry: 0x02_5952, original: "HUD_State_WaitPlayerSpawn_025952", rust: "system::player_hud::wait_spawn", run: |m| player_hud::wait_spawn(m) },
    SystemPort { entry: 0x02_596A, original: "$02596A", rust: "system::player_hud::wait_spawn_run", run: |m| player_hud::wait_spawn_run(m) },
    SystemPort { entry: 0x02_59A0, original: "HUD_State_BindPlayer_0259a0", rust: "system::player_hud::bind", run: |m| player_hud::bind(m, 0) },
    SystemPort { entry: 0x02_59AC, original: "$0259AC", rust: "system::player_hud::bind", run: |m| player_hud::bind(m, 1) },
    SystemPort { entry: 0x02_59B8, original: "$0259B8", rust: "system::player_hud::bind", run: |m| player_hud::bind(m, 2) },
    SystemPort { entry: 0x02_59FC, original: "$0259FC", rust: "system::player_hud::playing", run: |m| player_hud::playing(m) },
    SystemPort { entry: 0x02_5A0E, original: "HUD_State_PlayerDeath_025a0e", rust: "system::player_hud::in_play", run: |m| player_hud::in_play(m) },
    SystemPort { entry: 0x02_5AD8, original: "HUD_State_Respawn_025ad8", rust: "system::player_hud::respawn", run: |m| player_hud::respawn(m) },
    SystemPort { entry: 0x02_5B34, original: "HUD_State_GameOverEntry_025b34", rust: "system::player_hud::game_over", run: |m| player_hud::game_over(m) },
    SystemPort { entry: 0x02_5BCE, original: "$025BCE", rust: "system::player_hud::game_over_wait", run: |m| player_hud::game_over_wait(m) },
    SystemPort { entry: 0x02_5C44, original: "HUD_State_ContinueCountdown_025c44", rust: "system::player_hud::countdown", run: |m| player_hud::countdown(m) },
    SystemPort { entry: 0x02_5C80, original: "$025C80", rust: "system::player_hud::countdown_run", run: |m| player_hud::countdown_run(m) },
    SystemPort { entry: 0x02_5CCE, original: "$025CCE", rust: "system::player_hud::watch", run: |m| player_hud::watch(m) },
    SystemPort { entry: 0x02_5D0E, original: "$025D0E", rust: "system::player_hud::watch_run", run: |m| player_hud::watch_run(m) },
    SystemPort { entry: 0x02_5D5C, original: "JmpToScheduler_025d5c", rust: "system::tasks::finish", run: |m| tasks::finish(m) },
    SystemPort { entry: 0x02_5D64, original: "HUD_State_GameOverFinal_025d64", rust: "system::player_hud::final_game_over", run: |m| player_hud::final_game_over(m) },
    SystemPort { entry: 0x02_5D8A, original: "$025D8A", rust: "system::player_hud::final_run", run: |m| player_hud::final_run(m) },
    SystemPort { entry: 0x02_5DD2, original: "HUD_State_GameOverDone_025dd2", rust: "system::player_hud::done", run: |m| player_hud::done(m) },
    SystemPort { entry: 0x02_5E00, original: "$025E00", rust: "system::player_hud::done_run", run: |m| player_hud::done_run(m) },
    SystemPort { entry: 0x02_5E48, original: "HUD_State_PushStartBlink_025e48", rust: "system::player_hud::please_wait", run: |m| player_hud::please_wait(m) },
    SystemPort { entry: 0x02_5E56, original: "$025E56", rust: "system::player_hud::please_wait_run", run: |m| player_hud::please_wait_run(m) },
    SystemPort { entry: 0x02_630E, original: "HUD_DrawBombGauge_P1_02630e", rust: "system::player_hud::draw_bomb_gauge", run: |m| { player_hud::draw_bomb_gauge(m, 0); Flow::Return } },
    SystemPort { entry: 0x02_6372, original: "HUD_DrawBombGauge_P2_026372", rust: "system::player_hud::draw_bomb_gauge", run: |m| { player_hud::draw_bomb_gauge(m, 1); Flow::Return } },
    SystemPort { entry: 0x02_5ED6, original: "HUD_ResetFields_025ed6", rust: "system::player_hud::reset_fields", run: |m| { let t = m.cpu.a[6]; player_hud::reset_fields(m, t); Flow::Return } },
    SystemPort { entry: 0x00_1E28, original: "$001E28", rust: "system::player_hud::input_free", run: |m| { let c = player_hud::input_free(m); m.cpu.a[0] = mslug_game::layout::player_hud::SCRIPT_TASK; set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x00_1E4C, original: "$001E4C", rust: "system::player_hud::hold_scene", run: ret!(player_hud::hold_scene) },
    SystemPort { entry: 0x00_1E56, original: "$001E56", rust: "system::player_hud::set_vblank_delay", run: |m| { let n = m.cpu.d[0] as u8; player_hud::set_vblank_delay(m, n); Flow::Return } },
    SystemPort { entry: 0x02_6752, original: "$026752", rust: "system::player_hud::has_credit", run: |m| {
                     let p = m.cpu.d[1] & 0xFFFF;
                     let (c, d) = player_hud::has_credit(m, p, m.cpu.d[2] as u8);
                     m.cpu.d[2] = (m.cpu.d[2] & !0xFF) | u32::from(d);
                     if m.read_u8(mslug_game::layout::bios_ram::MVS_FLAG) == 0 {
                         m.cpu.d[1] = (m.cpu.d[1] & 0xFFFF_0000) | ((p << 2) & 0xFFFF);
                         m.cpu.a[4] = m.read_u32(mslug_game::layout::player_hud::CREDITS + p * 4);
                     } else {
                         m.cpu.a[4] = mslug_game::layout::bios_ram::CREDIT_REQUEST[0];
                     }
                     set_carry(m, c);
                     Flow::Return
                 } },
    SystemPort { entry: 0x02_56B0, original: "$00256B0", rust: "system::player_hud::init", run: |m| { let (a, d) = player_hud::init(m, 0, true); m.cpu.a[0] = a; m.cpu.d[0] = u32::from(d); Flow::Return } },
    SystemPort { entry: 0x02_5706, original: "$0025706", rust: "system::player_hud::init", run: |m| { let (a, d) = player_hud::init(m, 1, true); m.cpu.a[0] = a; m.cpu.d[0] = u32::from(d); Flow::Return } },
    SystemPort { entry: 0x02_56BE, original: "$00256BE", rust: "system::player_hud::init", run: |m| { let (a, d) = player_hud::init(m, 0, false); m.cpu.a[0] = a; m.cpu.d[0] = u32::from(d); Flow::Return } },
    SystemPort { entry: 0x02_5714, original: "$0025714", rust: "system::player_hud::init", run: |m| { let (a, d) = player_hud::init(m, 1, false); m.cpu.a[0] = a; m.cpu.d[0] = u32::from(d); Flow::Return } },
    SystemPort { entry: 0x04_42E6, original: "MissionDriver_Init_0442E6", rust: "system::events::start", run: |m| events::start(m) },
    SystemPort { entry: 0x04_4368, original: "MissionDriver_Loop_044368", rust: "system::events::run", run: |m| events::run(m) },
    SystemPort { entry: 0x04_42E2, original: "MissionWatch_Resume_0442E2", rust: "system::events::run", run: |m| events::run(m) },
    SystemPort { entry: 0x04_42C4, original: "MissionWatch_Handler_0442C4", rust: "system::events::watch", run: |m| events::watch(m) },
    SystemPort { entry: 0x04_482A, original: "PeriodicSpawner_Task_04482A", rust: "system::events::periodic", run: |m| events::periodic(m) },
    SystemPort { entry: 0x04_429E, original: "MissionWatch_Spawn_04429E", rust: "system::events::spawn_watch",
                 run: |m| { let c = events::spawn_watch(m, m.cpu.a[1], m.cpu.d[0] as u16); m.cpu.a[0] = c; Flow::Return } },
    SystemPort { entry: 0x04_46B6, original: "MissionVM_SkipOp_0446B6", rust: "system::events::skip",
                 run: |m| { m.cpu.a[1] = events::skip(m, m.cpu.a[1]); Flow::Return } },
    SystemPort { entry: 0x04_405E, original: "$04405E", rust: "system::events::to_layer",
                 run: |m| { let e = m.cpu.a[0]; events::to_layer(m, e); Flow::Return } },
    SystemPort { entry: 0x04_48A6, original: "Spawn_FromStream_0448A6", rust: "system::events::spawn_from_list",
                 run: |m| { let (t, p) = (m.cpu.a[6], m.cpu.a[1]); events::spawn_from_list(m, t, p); m.cpu.a[1] = p + 0x12; Flow::Return } },
    SystemPort { entry: mslug_game::layout::food::ENTRIES[0], original: "Item_Tmpl294_Food_09aec4+0", rust: "system::food::start", run: |m| food::start(m, 0) },
    SystemPort { entry: mslug_game::layout::food::ENTRIES[1], original: "Item_Tmpl294_Food_09aec4+1", rust: "system::food::start", run: |m| food::start(m, 1) },
    SystemPort { entry: mslug_game::layout::food::ENTRIES[2], original: "Item_Tmpl294_Food_09aec4+2", rust: "system::food::start", run: |m| food::start(m, 2) },
    SystemPort { entry: mslug_game::layout::food::ENTRIES[3], original: "Item_Tmpl294_Food_09aec4+3", rust: "system::food::start", run: |m| food::start(m, 3) },
    SystemPort { entry: mslug_game::layout::food::ENTRIES[4], original: "Item_Tmpl294_Food_09aec4+4", rust: "system::food::start", run: |m| food::start(m, 4) },
    SystemPort { entry: mslug_game::layout::food::ENTRIES[5], original: "Item_Tmpl294_Food_09aec4+5", rust: "system::food::start", run: |m| food::start(m, 5) },
    SystemPort { entry: mslug_game::layout::food::ENTRIES[6], original: "Item_Tmpl294_Food_09aec4+6", rust: "system::food::start", run: |m| food::start(m, 6) },
    SystemPort { entry: mslug_game::layout::food::ENTRIES[7], original: "Item_Tmpl294_Food_09aec4+7", rust: "system::food::start", run: |m| food::start(m, 7) },
    SystemPort { entry: mslug_game::layout::food::ENTRIES[8], original: "Item_Tmpl294_Food_09aec4+8", rust: "system::food::start", run: |m| food::start(m, 8) },
    SystemPort { entry: mslug_game::layout::food::ENTRIES[9], original: "Item_Tmpl294_Food_09aec4+9", rust: "system::food::start", run: |m| food::start(m, 9) },
    SystemPort { entry: mslug_game::layout::food::ENTRIES[10], original: "Item_Tmpl294_Food_09aec4+10", rust: "system::food::start", run: |m| food::start(m, 10) },
    SystemPort { entry: mslug_game::layout::food::ENTRIES[11], original: "Item_Tmpl294_Food_09aec4+11", rust: "system::food::start", run: |m| food::start(m, 11) },
    SystemPort { entry: mslug_game::layout::food::ENTRIES[12], original: "Item_Tmpl294_Food_09aec4+12", rust: "system::food::start", run: |m| food::start(m, 12) },
    SystemPort { entry: mslug_game::layout::food::ENTRIES[13], original: "Item_Tmpl294_Food_09aec4+13", rust: "system::food::start", run: |m| food::start(m, 13) },
    SystemPort { entry: mslug_game::layout::food::ENTRIES[14], original: "Item_Tmpl294_Food_09aec4+14", rust: "system::food::start", run: |m| food::start(m, 14) },
    SystemPort { entry: mslug_game::layout::food::ENTRIES[15], original: "Item_Tmpl294_Food_09aec4+15", rust: "system::food::start", run: |m| food::start(m, 15) },
    SystemPort { entry: mslug_game::layout::food::ENTRIES[16], original: "Item_Tmpl294_Food_09aec4+16", rust: "system::food::start", run: |m| food::start(m, 16) },
    SystemPort { entry: mslug_game::layout::food::ENTRIES[17], original: "Item_Tmpl294_Food_09aec4+17", rust: "system::food::start", run: |m| food::start(m, 17) },
    SystemPort { entry: mslug_game::layout::food::ENTRIES[18], original: "Item_Tmpl294_Food_09aec4+18", rust: "system::food::start", run: |m| food::start(m, 18) },
    SystemPort { entry: mslug_game::layout::food::ENTRIES[19], original: "Item_Tmpl294_Food_09aec4+19", rust: "system::food::start", run: |m| food::start(m, 19) },
    SystemPort { entry: 0x09_B0A8, original: "Item_Food_Wait_09b0a8", rust: "system::food::wait", run: |m| food::wait(m) },
    SystemPort { entry: 0x09_B0D8, original: "Item_Food_Blink_09b0d8", rust: "system::food::blink", run: |m| food::blink(m) },
    SystemPort { entry: 0x09_B0DE, original: "Item_Food_BlinkLoop_09b0de", rust: "system::food::blink_start", run: |m| food::blink_start(m) },
    SystemPort { entry: 0x09_B0FA, original: "$09B0FA", rust: "system::food::blink_run", run: |m| food::blink_run(m) },
    SystemPort { entry: 0x09_B474, original: "Item_Free_09b474", rust: "system::food::free", run: |m| food::free(m) },
    SystemPort { entry: 0x09_BB3C, original: "Item_LifetimeFromParam_09bb3c", rust: "system::food::lifetime",
                 run: |m| { let t = m.cpu.a[6]; let v = food::lifetime(m, t); m.cpu.d[0] = (m.cpu.d[0] & 0xFFFF_0000) | u32::from(v); Flow::Return } },
    SystemPort { entry: 0x09_9812, original: "Entity_TrailRecord_099812", rust: "system::platforms::record",
                 run: |m| { let (t, x, y, w) = (m.cpu.a[6], m.cpu.d[0] as u16, m.cpu.d[1] as u16, m.cpu.d[2] as u16); platforms::record(m, t, x, y, w); Flow::Return } },
    SystemPort { entry: 0x09_993C, original: "Trail_FindNearest_09993c", rust: "system::platforms::find_under",
                 run: |m| {
                     let (t, x, y) = (m.cpu.a[6], m.cpu.d[1] as u16, m.cpu.d[2] as u16);
                     let r = platforms::find_under(m, t, x, y);
                     match r {
                         Some((d, id)) => {
                             m.cpu.d[5] = (m.cpu.d[5] & 0xFFFF_0000) | u32::from(d);
                             m.cpu.d[6] = (m.cpu.d[6] & !0xFF) | u32::from(id);
                         }
                         None => m.cpu.d[6] = (m.cpu.d[6] & !0xFF) | 0x0F,
                     }
                     set_carry(m, r.is_some());
                     Flow::Return
                 } },
    SystemPort { entry: 0x09_99DE, original: "Clipping_Test_0999DE", rust: "system::platforms::follow",
                 run: |m| {
                     let t = m.cpu.a[6];
                     let r = platforms::follow(m, t);
                     if let Some((dx, dy)) = r {
                         m.cpu.d[2] = (m.cpu.d[2] & 0xFFFF_0000) | u32::from(dx);
                         m.cpu.d[3] = (m.cpu.d[3] & 0xFFFF_0000) | u32::from(dy);
                     }
                     set_carry(m, r.is_some());
                     Flow::Return
                 } },
    SystemPort { entry: 0x04_40E4, original: "Handler_ApplyCameraSelf_0440E4", rust: "system::platforms::apply_camera",
                 run: |m| { let t = m.cpu.a[6]; platforms::apply_camera(m, t); Flow::Return } },
    SystemPort { entry: 0x02_783A, original: "$02783A", rust: "system::platforms::move_with_scene",
                 run: |m| { let t = m.cpu.a[6]; platforms::move_with_scene(m, t); m.cpu.a[5] = mslug_game::layout::platforms::SCRATCH_BASE; Flow::Return } },
    SystemPort { entry: 0x09_B8C0, original: "Item_PhysAndProbe_09b8c0", rust: "system::platforms::item_physics",
                 run: |m| { let t = m.cpu.a[6]; platforms::item_physics(m, t); set_carry(m, false); Flow::Return } },
    SystemPort { entry: 0x02_7DB2, original: "CollMap_LookupTile_027db2", rust: "system::collision::lookup_tile",
                 run: |m| {
                     let (t, x, y) = (m.cpu.a[6], m.cpu.d[1] as u16, m.cpu.d[2] as u16);
                     let tile = collision::lookup_tile(m, t, x, y);
                     m.cpu.a[1] = tile.record;
                     m.cpu.d[0] = (m.cpu.d[0] & !0xFF) | u32::from(tile.kind);
                     if let Some((c, r)) = tile.frac {
                         m.cpu.d[3] = (m.cpu.d[3] & 0xFFFF_0000) | u32::from(c);
                         m.cpu.d[4] = (m.cpu.d[4] & 0xFFFF_0000) | u32::from(r);
                     }
                     Flow::Return
                 } },
    SystemPort { entry: 0x02_7E28, original: "CollMap_TestSolidOrPlatform_027e28", rust: "system::collision::test_masks",
                 run: |m| {
                     let (rec, c, r) = (m.cpu.a[1], m.cpu.d[3] as u16, m.cpu.d[4] as u16);
                     let hit = collision::test_masks(m, rec, c, r);
                     if let Some(h) = hit { m.cpu.a[1] = h; }
                     set_carry(m, hit.is_some());
                     Flow::Return
                 } },
    SystemPort { entry: 0x02_7EBA, original: "SpritePubEffect_027EBA", rust: "system::collision::probe_floor",
                 run: |m| {
                     let t = m.cpu.a[6];
                     let f = collision::probe_floor(m, t, (m.cpu.d[3] as u16, m.cpu.d[4] as u16));
                     m.cpu.a[1] = f.hit.unwrap_or(f.record);
                     m.cpu.d[3] = (m.cpu.d[3] & 0xFFFF_0000) | u32::from(f.at.0);
                     m.cpu.d[4] = (m.cpu.d[4] & 0xFFFF_0000) | u32::from(f.at.1);
                     set_carry(m, f.in_air);
                     Flow::Return
                 } },
    SystemPort { entry: 0x04_4182, original: "Handler_ApplyCameraGlobals_044182", rust: "system::platforms::apply_camera_work",
                 run: |m| { let t = m.cpu.a[6]; platforms::apply_camera_work(m, t); Flow::Return } },
    SystemPort { entry: 0x02_7C8C, original: "Entity_ProbeTransformFreeCcr_027c8c", rust: "system::collision::fall",
                 run: |m| { let t = m.cpu.a[6]; let c = collision::fall(m, t); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x02_67F8, original: "ClampVelocity_0267f8", rust: "system::collision::clamp_velocity",
                 run: |m| {
                     let (v, l) = collision::clamp_velocity(m.cpu.d[0] as u16, 0x800);
                     m.cpu.d[0] = (m.cpu.d[0] & 0xFFFF_0000) | u32::from(v);
                     m.cpu.d[1] = (m.cpu.d[1] & 0xFFFF_0000) | u32::from(l);
                     Flow::Return
                 } },
    // --- movimiento con colisión
    SystemPort { entry: 0x02_6814, original: "Entity_IntegrateVelocity_026814", rust: "system::motion::integrate_velocity",
                 run: |m| { let t = m.cpu.a[6]; motion::integrate_velocity(m, t); Flow::Return } },
    SystemPort { entry: 0x02_6836, original: "Entity_MoveX_WallStop_026836", rust: "system::motion::advance",
                 run: |m| { let t = m.cpu.a[6]; motion::advance(m, t); m.cpu.d[3] &= 0xFFFF_0000; Flow::Return } },
    SystemPort { entry: 0x02_8074, original: "Entity_ClampXToScreenEdge_028074", rust: "system::motion::clamp_to_screen",
                 run: |m| {
                     let t = m.cpu.a[6];
                     let before = m.read_u8(t + 0x13);
                     let v = motion::clamp_to_screen(m, t, m.cpu.d[2] as u16);
                     let _ = before;
                     m.cpu.d[2] = (m.cpu.d[2] & 0xFFFF_0000) | u32::from(v);
                     let inside = m.read_u8(t + 0x13) & 0x80 != 0;
                     set_carry(m, !inside);
                     Flow::Return
                 } },
    SystemPort { entry: 0x02_81C8, original: "Entity_ApplyVelLatch_0281c8", rust: "system::motion::apply_latch",
                 run: |m| { let t = m.cpu.a[6]; motion::apply_latch(m, t); m.cpu.a[5] = 0x10_8080; Flow::Return } },
    SystemPort { entry: 0x02_822C, original: "Entity_SlopeDirTest_02822c", rust: "system::motion::leaving_slope",
                 run: |m| { let (t, s) = (m.cpu.a[6], m.cpu.a[3]); let c = motion::leaving_slope(m, t, s); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x09_99CA, original: "Trail_SetPlatform_0999ca", rust: "system::motion::set_platform",
                 run: |m| { let t = m.cpu.a[6]; motion::set_platform(m, t, m.cpu.d[0] as u8); Flow::Return } },
    SystemPort { entry: 0x02_74B8, original: "Entity_MoveAndCollide_D_0274b8", rust: "system::motion::move_and_collide",
                 run: |m| { let t = m.cpu.a[6]; let c = motion::move_and_collide(m, t, m.cpu.d[4] as u16); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x02_7464, original: "Entity_MoveFall_027464", rust: "system::motion::move_fall",
                 run: |m| { let t = m.cpu.a[6]; let c = motion::move_fall(m, t, m.cpu.d[4] as u16); set_carry(m, c); Flow::Return } },
    // --- recogida de objetos
    SystemPort { entry: 0x03_EF5C, original: "Player_TouchBox_03ef5c", rust: "system::pickup::touches",
                 run: |m| { let (t, e, b) = (m.cpu.a[6], m.cpu.a[0], m.cpu.a[1]); let c = pickup::touches(m, t, e, b); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x03_EE48, original: "Player_Toucher_03ee48", rust: "system::pickup::toucher",
                 run: |m| { let (t, b) = (m.cpu.a[6], m.cpu.a[1]); let r = pickup::toucher(m, t, b); toucher_regs(m, r); Flow::Return } },
    SystemPort { entry: 0x09_A836, original: "Item_PickupTestA_09a836", rust: "system::pickup::item_toucher",
                 run: |m| { let t = m.cpu.a[6]; let r = pickup::item_toucher(m, t, 0x09_A6CE); m.cpu.a[1] = 0x09_A6CE; toucher_regs(m, r); Flow::Return } },
    SystemPort { entry: 0x09_A84E, original: "Item_PickupTestB_09a84e", rust: "system::pickup::item_toucher",
                 run: |m| { let t = m.cpu.a[6]; let r = pickup::item_toucher(m, t, 0x09_A6D6); m.cpu.a[1] = 0x09_A6D6; toucher_regs(m, r); Flow::Return } },
    SystemPort { entry: 0x09_B920, original: "Item_Food_Pickup_09b920", rust: "system::pickup::food",
                 run: |m| { let t = m.cpu.a[6]; let c = pickup::food(m, t); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x09_A75A, original: "Item_GroundTail_09a75a", rust: "system::items::ground_tail", run: |m| items::ground_tail(m) },
    SystemPort { entry: 0x05_DD56, original: "Entity_OutOfViewSeen_05dd56", rust: "system::items::out_of_view_seen",
                 run: |m| { let t = m.cpu.a[6]; let a = m.cpu.a[0]; let c = items::out_of_view_seen(m, t, (a != 0xFFFF_FFFF).then_some(a)); out_regs(m, a); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x05_DD5C, original: "Entity_OutOfView_05dd5c", rust: "system::items::out_of_view",
                 run: |m| { let t = m.cpu.a[6]; let a = m.cpu.a[0]; let c = items::out_of_view(m, t, (a != 0xFFFF_FFFF).then_some(a)); out_regs(m, a); set_carry(m, c); Flow::Return } },
    // --- soldados rasos
    SystemPort { entry: 0x08_D2B0, original: "Pos_IntegrateX88_08d2b0", rust: "system::grunt::integrate_axis",
                 run: |m| { let t = m.cpu.a[6]; let p = grunt::integrate_axis(m, t, 0); m.cpu.d[0] = (m.cpu.d[0] & 0xFFFF_0000) | u32::from(p); Flow::Return } },
    SystemPort { entry: 0x08_D2D4, original: "Pos_IntegrateY88_08d2d4", rust: "system::grunt::integrate_axis",
                 run: |m| { let t = m.cpu.a[6]; let p = grunt::integrate_axis(m, t, 1); m.cpu.d[0] = (m.cpu.d[0] & 0xFFFF_0000) | u32::from(p); Flow::Return } },
    SystemPort { entry: 0x08_D2F8, original: "Pos_IntegrateXY88_08d2f8", rust: "system::grunt::integrate",
                 run: |m| { let t = m.cpu.a[6]; grunt::integrate(m, t); Flow::Return } },
    SystemPort { entry: 0x08_D34E, original: "Pos_IntegrateXY88_Accel_08d34e", rust: "system::grunt::integrate_accel",
                 run: |m| { let t = m.cpu.a[6]; grunt::integrate_accel(m, t); Flow::Return } },
    SystemPort { entry: 0x08_EFB0, original: "Grunt_Cull_08efb0", rust: "system::grunt::cull", run: |m| grunt::cull(m) },
    SystemPort { entry: 0x08_EFCE, original: "Phys_PlayerNearX_08efce", rust: "system::grunt::player_near_x",
                 run: |m| { let t = m.cpu.a[6]; let c = grunt::player_near_x(m, t, m.cpu.d[0] as u16); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x08_F002, original: "Phys_FacingFromParam_08f002", rust: "system::grunt::facing_from_param",
                 run: |m| { let t = m.cpu.a[6]; grunt::facing_from_param(m, t); Flow::Return } },
    SystemPort { entry: 0x08_F010, original: "Phys_VelXFromParam_08f010", rust: "system::grunt::vel_x_from_param",
                 run: |m| { let t = m.cpu.a[6]; grunt::vel_x_from_param(m, t); Flow::Return } },
    SystemPort { entry: 0x08_F02C, original: "Phys_SetTrigger_08f02c", rust: "system::grunt::set_trigger",
                 run: |m| { let t = m.cpu.a[6]; grunt::set_trigger(m, t); Flow::Return } },
    SystemPort { entry: 0x08_F040, original: "Phys_Triggered_08f040", rust: "system::grunt::triggered",
                 run: |m| { let t = m.cpu.a[6]; let c = grunt::triggered(m, t); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x08_F070, original: "Phys_AllocByKind_08f070", rust: "system::grunt::alloc_by_kind",
                 run: |m| { let t = m.cpu.a[6]; grunt::alloc_by_kind(m, t); Flow::Return } },
    SystemPort { entry: 0x08_F084, original: "Phys_AllocA_08f084", rust: "system::grunt::alloc_a",
                 run: |m| { let t = m.cpu.a[6]; grunt::alloc_a(m, t); Flow::Return } },
    SystemPort { entry: 0x08_F0D0, original: "Phys_AllocB_08f0d0", rust: "system::grunt::alloc_b",
                 run: |m| { let t = m.cpu.a[6]; grunt::alloc_b(m, t); Flow::Return } },
    SystemPort { entry: 0x08_F108, original: "Phys_SetAttr_08f108", rust: "system::grunt::set_attr",
                 run: |m| { let t = m.cpu.a[6]; grunt::set_attr(m, t); Flow::Return } },
    SystemPort { entry: 0x08_DB50, original: "Grunt_Tmpl162_08db50", rust: "system::grunt::hopper", run: |m| grunt::hopper(m) },
    SystemPort { entry: 0x08_DB7A, original: "Grunt_HopDown_08db7a", rust: "system::grunt::hop_down", run: |m| grunt::hop_down(m) },
    SystemPort { entry: 0x08_DBA6, original: "Grunt_HopDown_08db7a+0x2C", rust: "system::grunt::hop_down_run", run: |m| grunt::hop_down_run(m) },
    SystemPort { entry: 0x08_DBE2, original: "Grunt_HopUp_08dbe2", rust: "system::grunt::hop_up", run: |m| grunt::hop_up(m) },
    SystemPort { entry: 0x08_DC0E, original: "Grunt_HopUp_08dbe2+0x2C", rust: "system::grunt::hop_up_run", run: |m| grunt::hop_up_run(m) },
    SystemPort { entry: 0x09_B82C, original: "Item_Tmpl316_ComboTimer_09b82c", rust: "system::pickup::streak_start", run: |m| pickup::streak_start(m) },
    SystemPort { entry: 0x09_B83E, original: "Item_Tmpl316_ComboTimer_09b82c+0x12", rust: "system::pickup::streak_run", run: |m| pickup::streak_run(m) },
    SystemPort { entry: 0x05_EA96, original: "Task_Clear106F42_05ea96", rust: "system::scroll::clear_lock_x_start", run: |m| scroll::clear_lock_x_start(m) },
    SystemPort { entry: 0x05_EA9C, original: "Task_Clear106F42_05ea96+6", rust: "system::scroll::clear_lock_x", run: |m| scroll::clear_lock_x(m) },
    SystemPort { entry: 0x05_EA6E, original: "Slot1008A0_SetHandlerIfA5_05ea6e", rust: "system::scroll::set_lock_x_clearer",
                 run: |m| {
                     if m.cpu.a[5] != 0xFFFF_FFFF { scroll::set_lock_x_clearer(m, true); m.cpu.a[0] = 0x10_08A0; }
                     Flow::Return
                 } },
    SystemPort { entry: 0x05_EA86, original: "Slot1008A0_SetHandler400_05ea86", rust: "system::scroll::set_lock_x_clearer",
                 run: |m| { scroll::set_lock_x_clearer(m, false); m.cpu.a[0] = 0x10_08A0; Flow::Return } },
    // --- piloto automático
    SystemPort { entry: 0x07_9298, original: "AutoDemo_Tmpl_Driver_079298", rust: "system::autopilot::start", run: |m| autopilot::start(m) },
    SystemPort { entry: 0x07_92F0, original: "AutoDemo_Tmpl_Driver_079298+0x58", rust: "system::autopilot::run", run: |m| autopilot::run(m) },
    SystemPort { entry: 0x07_962C, original: "AutoDemo_StartLevelScript_07962c", rust: "system::autopilot::start_level_script",
                 run: |m| { let t = m.cpu.a[6]; let c = autopilot::start_level_script(m, t); set_carry(m, c); Flow::Return } },
    // --- indicador GO y animaciones de la capa fija
    SystemPort { entry: 0x08_C864, original: "Cut_Watcher_Init_08c864", rust: "system::go_arrow::start", run: |m| go_arrow::start(m) },
    SystemPort { entry: 0x08_C880, original: "Cut_Watcher_Run_08c880", rust: "system::go_arrow::run", run: |m| go_arrow::run(m) },
    SystemPort { entry: 0x08_C8FA, original: "Cut_Watcher_Play_08c8fa", rust: "system::go_arrow::play", run: |m| go_arrow::play(m) },
    SystemPort { entry: 0x08_C90E, original: "Cut_Watcher_Play_08c8fa+0x14", rust: "system::go_arrow::play_run", run: |m| go_arrow::play_run(m) },
    SystemPort { entry: 0x08_C820, original: "Cut_InstallListByVariant_08c820", rust: "system::go_arrow::start_anim",
                 run: |m| { let t = m.cpu.a[6]; go_arrow::start_anim(m, t); Flow::Return } },
    SystemPort { entry: 0x05_E1AA, original: "Players_InPlayMask_05e1aa", rust: "system::go_arrow::players_mask",
                 run: |m| { let v = go_arrow::players_mask(m); m.cpu.d[0] = u32::from(v); set_carry(m, v == 0); Flow::Return } },
    SystemPort { entry: 0x05_DB58, original: "FixAnim_Start_05db58", rust: "system::hud::fix_anim_start",
                 run: |m| { let (t, l) = (m.cpu.a[6], m.cpu.a[0]); hud::fix_anim_start(m, t, l); Flow::Return } },
    SystemPort { entry: 0x05_DB6A, original: "FixAnim_Step_05db6a", rust: "system::hud::fix_anim_step",
                 run: |m| { let t = m.cpu.a[6]; let c = hud::fix_anim_step(m, t); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x05_DBC2, original: "FixAnim_Draw_05dbc2", rust: "system::hud::fix_anim_draw",
                 run: |m| { let t = m.cpu.a[6]; hud::fix_anim_draw(m, t); Flow::Return } },
    SystemPort { entry: 0x05_DBDC, original: "FixAnim_Erase_05dbdc", rust: "system::hud::fix_anim_erase",
                 run: |m| { let t = m.cpu.a[6]; hud::fix_anim_erase(m, t); Flow::Return } },
    SystemPort { entry: 0x02_942A, original: "ScriptOp1E_SpawnChild_02942a", rust: "system::fort::place_from_record",
                 run: |m| { let (t, r) = (m.cpu.a[6], m.cpu.a[1]); fort::place_from_record(m, t, r); Flow::Return } },
    SystemPort { entry: 0x05_DD22, original: "Handler_ApplyCameraSelf_CopyTransform_05dd22", rust: "system::fort::place_child",
                 run: |m| { let (t, c) = (m.cpu.a[6], m.cpu.a[0]); fort::place_child(m, t, c); Flow::Return } },
    SystemPort { entry: 0x02_870A, original: "Hitbox_SideOfImpact_02870a", rust: "system::fort::hit_side",
                 run: |m| { let t = m.cpu.a[6]; let s = fort::hit_side(m, t);
                            m.cpu.d[0] = (m.cpu.d[0] & 0xFFFF_0000) | u32::from(s.unwrap_or(0));
                            set_carry(m, s.is_some()); Flow::Return } },
    SystemPort { entry: 0x05_E770, original: "$05E770", rust: "system::fort::hit_sound",
                 run: |m| { let (t, a) = (m.cpu.a[6], m.cpu.a[0]); fort::hit_sound(m, t, a); Flow::Return } },
    SystemPort { entry: 0x04_FA70, original: "Prop_OffscreenLeftCheck_04fa70", rust: "system::fort::off_left",
                 run: |m| { let t = m.cpu.a[6]; let c = fort::off_left(m, t); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x08_65DC, original: "Fort_Init_V0_0865dc", rust: "system::fort::start", run: |m| fort::start(m, 0) },
    SystemPort { entry: 0x08_6624, original: "Fort_Init_V1_086624", rust: "system::fort::start", run: |m| fort::start(m, 1) },
    SystemPort { entry: 0x08_6666, original: "Fort_Init_V2_086666", rust: "system::fort::start", run: |m| fort::start(m, 2) },
    SystemPort { entry: 0x08_66AE, original: "Fort_Init_V3_0866ae", rust: "system::fort::start", run: |m| fort::start(m, 3) },
    SystemPort { entry: 0x08_66F0, original: "Fort_Init_V4_0866f0", rust: "system::fort::start", run: |m| fort::start(m, 4) },
    SystemPort { entry: 0x08_675A, original: "Fort_Init_Common_086738 (bucle)", rust: "system::fort::run", run: |m| fort::run(m) },
    SystemPort { entry: 0x08_6E4A, original: "Hatch_Init_086e4a", rust: "system::fort::hatch_start", run: |m| fort::hatch_start(m, 0) },
    SystemPort { entry: 0x08_6E74, original: "Hatch_Init_V1_086e74", rust: "system::fort::hatch_start", run: |m| fort::hatch_start(m, 1) },
    SystemPort { entry: 0x08_6E9E, original: "Hatch_Init_V2_086e9e", rust: "system::fort::hatch_start", run: |m| fort::hatch_start(m, 2) },
    SystemPort { entry: 0x08_6EC8, original: "Hatch_Init_V3_086ec8", rust: "system::fort::hatch_start", run: |m| fort::hatch_start(m, 3) },
    SystemPort { entry: 0x08_6EF2, original: "Hatch_Init_V4_086ef2", rust: "system::fort::hatch_start", run: |m| fort::hatch_start(m, 4) },
    SystemPort { entry: 0x08_6F1C, original: "$086F1C", rust: "system::fort::hatch_start_placed", run: |m| fort::hatch_start_placed(m, 5) },
    SystemPort { entry: 0x08_6F40, original: "$086F40", rust: "system::fort::hatch_start_placed", run: |m| fort::hatch_start_placed(m, 6) },
    SystemPort { entry: 0x08_6F64, original: "Hatch_WaitClosed_086f64", rust: "system::fort::hatch_wait_closed", run: |m| fort::hatch_wait_closed(m) },
    SystemPort { entry: 0x08_6F70, original: "$086F70", rust: "system::fort::hatch_wait_closed_run", run: |m| fort::hatch_wait_closed_run(m) },
    SystemPort { entry: 0x08_6FB4, original: "Hatch_Opening_086fb4", rust: "system::fort::hatch_opening", run: |m| fort::hatch_opening(m) },
    SystemPort { entry: 0x08_6FDC, original: "$086FDC", rust: "system::fort::hatch_opening_run", run: |m| fort::hatch_opening_run(m) },
    SystemPort { entry: 0x08_7004, original: "Hatch_SpawnTroops_087004", rust: "system::fort::hatch_spawn_troops", run: |m| fort::hatch_spawn_troops(m) },
    SystemPort { entry: 0x08_7024, original: "$087024", rust: "system::fort::hatch_spawn_troops_run", run: |m| fort::hatch_spawn_troops_run(m) },
    SystemPort { entry: 0x08_7068, original: "Hatch_WaitOpen_087068", rust: "system::fort::hatch_wait_open", run: |m| fort::hatch_wait_open(m) },
    SystemPort { entry: 0x08_7074, original: "$087074", rust: "system::fort::hatch_wait_open_run", run: |m| fort::hatch_wait_open_run(m) },
    SystemPort { entry: 0x08_70B8, original: "Hatch_Closing_0870b8", rust: "system::fort::hatch_closing", run: |m| fort::hatch_closing(m) },
    SystemPort { entry: 0x08_70E0, original: "$0870E0", rust: "system::fort::hatch_closing_run", run: |m| fort::hatch_closing_run(m) },
    SystemPort { entry: 0x08_7108, original: "Hatch_FinalOpen_087108", rust: "system::fort::hatch_final_open", run: |m| fort::hatch_final_open(m) },
    SystemPort { entry: 0x08_7130, original: "Hatch_FinalSpawn_087130", rust: "system::fort::hatch_final_spawn", run: |m| fort::hatch_final_spawn(m) },
    SystemPort { entry: 0x08_6552, original: "Entity_CmpPrioWithSibling_086552", rust: "system::fort::has_children",
                 run: |m| { let t = m.cpu.a[6]; m.cpu.a[1] = m.read_u32(t + mslug_game::layout::task::NEXT); let c = fort::has_children(m, t); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x08_6CD0, original: "Crate_Init_086cd0", rust: "system::fort::crate_start", run: |m| fort::crate_start(m, 0) },
    SystemPort { entry: 0x08_6CE4, original: "Crate_Init_V1_086ce4", rust: "system::fort::crate_start", run: |m| fort::crate_start(m, 1) },
    SystemPort { entry: 0x08_6CF8, original: "Crate_Init_V2_086cf8", rust: "system::fort::crate_start", run: |m| fort::crate_start(m, 2) },
    SystemPort { entry: 0x08_6D0C, original: "Crate_Init_V3_086d0c", rust: "system::fort::crate_start", run: |m| fort::crate_start(m, 3) },
    SystemPort { entry: 0x08_6D20, original: "Crate_Init_V4_086d20", rust: "system::fort::crate_start", run: |m| fort::crate_start(m, 4) },
    SystemPort { entry: 0x08_6D58, original: "$086D58", rust: "system::fort::crate_run", run: |m| fort::crate_run(m) },
    SystemPort { entry: 0x02_8758, original: "Hitbox_CheckBit0_028758", rust: "system::fort::dead",
                 run: |m| { let t = m.cpu.a[6]; let c = fort::dead(m, t); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x08_8438, original: "Entity_HitboxPulseTable_088438", rust: "system::fort::pulse_hitbox",
                 run: |m| { let t = m.cpu.a[6]; fort::pulse_hitbox(m, t, true); pulse_regs(m, t, true); Flow::Return } },
    SystemPort { entry: 0x08_846A, original: "$08846A", rust: "system::fort::pulse_hitbox",
                 run: |m| { let t = m.cpu.a[6]; fort::pulse_hitbox(m, t, false); pulse_regs(m, t, false); Flow::Return } },
    SystemPort { entry: 0x08_848C, original: "Entity_PropagateDamageToParent_08848c", rust: "system::fort::pass_damage_to_parent",
                 run: |m| { let t = m.cpu.a[6]; fort::pass_damage_to_parent(m, t); Flow::Return } },
    // --- golpes entre entidades
    SystemPort { entry: 0x02_83D8, original: "Hitbox_RunAttackList_0283d8", rust: "system::hits::run_attack_list",
                 run: |m| { let t = m.cpu.a[6]; let (o, a) = hits::run_attack_list(m, t);
                            m.cpu.d[7] = o as i32 as u32; m.cpu.a[0] = a; Flow::Return } },
    SystemPort { entry: 0x02_8576, original: "Hitbox_TestVictim_028576", rust: "system::hits::test_victim",
                 run: |m| { let (t, a, v) = (m.cpu.a[6], m.cpu.a[0], m.cpu.a[1]); let c = hits::test_victim(m, t, a, v); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x02_8594, original: "Hitbox_TestShape_028594", rust: "system::hits::test_shape",
                 run: |m| { let (t, a, v, b) = (m.cpu.a[6], m.cpu.a[0], m.cpu.a[1], m.cpu.a[2]); let c = hits::test_shape(m, t, a, v, b); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x02_86EC, original: "Hitbox_SubtractHP_0286ec", rust: "system::hits::subtract_hp",
                 run: |m| { let (t, v) = (m.cpu.a[6], m.cpu.a[1]); hits::subtract_hp(m, t, v, m.cpu.d[0] as u16); Flow::Return } },
    SystemPort { entry: 0x02_876E, original: "Hitbox_OverlapRect_02876e", rust: "system::hits::overlap_rect",
                 run: |m| { let (t, a, v, b) = (m.cpu.a[6], m.cpu.a[0], m.cpu.a[1], m.cpu.a[2]); let c = hits::overlap_rect(m, t, a, v, b); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x02_8886, original: "Hitbox_OverlapPoint_028886", rust: "system::hits::overlap_point",
                 run: |m| { let (t, a, v, b) = (m.cpu.a[6], m.cpu.a[0], m.cpu.a[1], m.cpu.a[2]); let c = hits::overlap_point(m, t, a, v, b); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x02_8930, original: "Hitbox_OnScreen_028930", rust: "system::hits::on_screen",
                 run: |m| { let (t, a) = (m.cpu.a[6], m.cpu.a[0]); let mg = m.cpu.a[4];
                            let c = hits::on_screen(m, t, a, (mg != 0xFFFF_FFFF).then_some(mg)); set_carry(m, !c); Flow::Return } },
    SystemPort { entry: 0x05_1914, original: "$051914", rust: "system::hits::award_damage_points",
                 run: |m| { let (t, v) = (m.cpu.a[6], m.cpu.a[1]); hits::award_damage_points(m, t, v); Flow::Return } },
    // --- Slug: caída de la entrada en escena y física del chasis
    SystemPort { entry: 0x02_B004, original: "Slug_DropVariant4_02af88__L02b004", rust: "system::slug::fall_run", run: |m| slug::fall_run(m) },
    SystemPort { entry: 0x02_B05E, original: "Slug_DropDescend_02b05e", rust: "system::slug::descend", run: |m| slug::descend(m) },
    SystemPort { entry: 0x02_B09E, original: "Slug_DropDescend_02b05e__L02b09e", rust: "system::slug::descend_run", run: |m| slug::descend_run(m) },
    SystemPort { entry: 0x02_A878, original: "Slug_PhysicsAir_02a878", rust: "system::slug::physics_air",
                 run: |m| { let t = m.cpu.a[6]; let c = slug::physics_air(m, t, m.cpu.d[4] as u16); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x02_A8C0, original: "Slug_GroundContact_02a8c0", rust: "system::slug::ground_contact",
                 run: |m| { let t = m.cpu.a[6];
                            let c = slug::ground_contact(m, t);
                            m.cpu.d[1] = (m.cpu.d[1] & 0xFFFF_FF00) | u32::from(c.wheels as u8);
                            set_carry(m, c.in_air); Flow::Return } },
    SystemPort { entry: 0x02_A958, original: "Slug_TerrainSlope_02a958", rust: "system::slug::terrain_slope",
                 run: |m| { let t = m.cpu.a[6]; let (d0, d1, d2) = slug::terrain_slope(m, t);
                            m.cpu.d[0] = (m.cpu.d[0] & 0xFFFF_0000) | u32::from(d0);
                            m.cpu.d[1] = if d1 == 0 { m.cpu.d[1] & 0xFFFF_0000 } else { d1 };
                            m.cpu.d[2] = (m.cpu.d[2] & 0xFFFF_0000) | u32::from(d2); Flow::Return } },
    SystemPort { entry: 0x02_A9A0, original: "Slug_SlopeToAnimIdx_02a9a0", rust: "system::slug::slope_anim_idx",
                 run: |m| { let t = m.cpu.a[6]; let i = slug::slope_anim_idx(m, t); m.cpu.d[0] = (m.cpu.d[0] & 0xFFFF_0000) | u32::from(i); Flow::Return } },
    SystemPort { entry: 0x02_AA24, original: "Slug_UpdateAnimAndChassis_02aa24", rust: "system::slug::update_anim",
                 run: |m| { let t = m.cpu.a[6]; slug::update_anim(m, t); Flow::Return } },
    SystemPort { entry: 0x02_AA0E, original: "Slug_UpdateAnimKeepIdx_02aa0e", rust: "system::slug::update_anim_keep_idx",
                 run: |m| { let t = m.cpu.a[6]; slug::update_anim_keep_idx(m, t); Flow::Return } },
    SystemPort { entry: 0x02_FAE4, original: "Slug_UpdateDamageSprite_02fae4", rust: "system::slug::damage_anim",
                 run: |m| { let t = m.cpu.a[6]; let c = slug::damage_anim(m, t); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x02_A478, original: "Slug_UpdateAirFlag_02a478", rust: "system::slug::update_steering",
                 run: |m| { let t = m.cpu.a[6]; slug::update_steering(m, t); Flow::Return } },
    SystemPort { entry: 0x03_0704, original: "Slug_WheelAgainstWall_030704", rust: "system::slug::wheel_against_wall",
                 run: |m| { let t = m.cpu.a[6]; let r = slug::wheel_against_wall(m, t);
                            let (a0, d0) = r.unwrap_or((t, 0));
                            m.cpu.a[0] = a0; m.cpu.d[0] = (m.cpu.d[0] & 0xFFFF_0000) | u32::from(d0 as u16);
                            m.cpu.sr.set(StatusRegister::Z, r.is_none()); Flow::Return } },
    SystemPort { entry: 0x02_81B0, original: "Entity_LatchX_0281b0", rust: "system::slug::latch_x",
                 run: |m| { m.cpu.a[5] = mslug_game::layout::collision::SCRATCH_BASE; slug::latch_x(m, m.cpu.d[0] as u16); Flow::Return } },
    SystemPort { entry: 0x02_7A18, original: "Entity_MoveOnBaseY_027a18", rust: "system::slug::move_on_base",
                 run: |m| { m.cpu.a[5] = mslug_game::layout::collision::SCRATCH_BASE; let t = m.cpu.a[6]; let c = slug::move_on_base(m, t, m.cpu.d[4] as u16); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x02_7444, original: "Entity_MoveAndStop_027444", rust: "system::slug::move_and_stop",
                 run: |m| { let t = m.cpu.a[6]; let c = slug::move_and_stop(m, t, m.cpu.d[4] as u16); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x02_95A6, original: "Slug_AngleToSpriteIdx_0295a6", rust: "system::slug::angle_sprite",
                 run: |m| { let k = m.cpu.d[0] as u16; let i = slug::angle_sprite(m, k); m.cpu.a[0] = mslug_game::layout::slug::ANGLE_SPRITES;
                            m.cpu.d[0] = (m.cpu.d[0] & 0xFFFF_0000) | u32::from(i); Flow::Return } },
    SystemPort { entry: 0x02_FB9A, original: "Slug_WheelAnim_02fb92__L02fb9a", rust: "system::slug::track_start", run: |m| slug::track_start(m) },
    SystemPort { entry: 0x02_FBCC, original: "Slug_WheelAnim_02fb92+0x3A", rust: "system::slug::track_run", run: |m| slug::track_run(m) },
    SystemPort { entry: 0x02_FC6A, original: "Slug_WheelAnim_02fb92+0xD8", rust: "system::slug::track_run_plain", run: |m| slug::track_run_plain(m) },
    SystemPort { entry: 0x05_E4B2, original: "Entity_FollowOwner_05e4b2", rust: "system::slug::follow_owner",
                 run: |m| { let t = m.cpu.a[6]; let (a, x, y) = slug::follow_owner(m, t); owner_regs(m, t, a, x, y); Flow::Return } },
    SystemPort { entry: 0x05_E4CA, original: "Entity_OwnerPos_05e4ca", rust: "system::slug::owner_pos",
                 run: |m| { let t = m.cpu.a[6]; let (a, x, y) = slug::owner_pos(m, t); owner_regs(m, t, a, x, y); Flow::Return } },
    // --- iconos sobre una entidad
    SystemPort { entry: 0x03_1FCA, original: "PlayerIcon_Pow_031fca", rust: "system::icons::pow_start", run: |m| icons::pow_start(m) },
    SystemPort { entry: 0x03_2010, original: "PlayerIcon_Pow_031fca+0x46", rust: "system::icons::pow_run", run: |m| icons::pow_run(m) },
    SystemPort { entry: 0x03_207C, original: "PlayerIcon_Bubble_03207c", rust: "system::icons::bubble_start", run: |m| icons::bubble_start(m) },
    SystemPort { entry: 0x03_20AC, original: "PlayerIcon_Bubble_03207c+0x30", rust: "system::icons::bubble_run", run: |m| icons::bubble_run(m) },
    SystemPort { entry: 0x03_21BC, original: "PlayerIcon_Gone_0321bc", rust: "system::icons::gone", run: |m| icons::gone(m) },
    // --- torreta del Slug
    SystemPort { entry: 0x04_59A8, original: "Turret_InitDir0_0459A8", rust: "system::turret::init", run: |m| turret::init(m, 0) },
    SystemPort { entry: 0x04_59C0, original: "Turret_InitDir1_0459C0", rust: "system::turret::init", run: |m| turret::init(m, 1) },
    SystemPort { entry: 0x04_59D8, original: "Turret_InitDir2_0459D8", rust: "system::turret::init", run: |m| turret::init(m, 2) },
    SystemPort { entry: 0x04_59F0, original: "Turret_InitDir3_0459F0", rust: "system::turret::init", run: |m| turret::init(m, 3) },
    SystemPort { entry: 0x04_5ABE, original: "Turret_InitDir4_045ABE", rust: "system::turret::init", run: |m| turret::init(m, 4) },
    SystemPort { entry: 0x04_5A38, original: "Turret_Track_045A38", rust: "system::turret::track", run: |m| turret::track(m) },
    SystemPort { entry: 0x04_5B02, original: "Turret_AimSmooth_045B02", rust: "system::turret::snap_track", run: |m| turret::snap_track(m) },
    SystemPort { entry: 0x04_5B9C, original: "Turret_Tail_045B9C", rust: "system::turret::tail", run: |m| turret::tail(m) },
    SystemPort { entry: 0x04_58DC, original: "Turret_FollowParent_0458DC", rust: "system::turret::follow",
                 run: |m| { let t = m.cpu.a[6]; turret::follow(m, t); Flow::Return } },
    SystemPort { entry: 0x04_593A, original: "Turret_ReadPlayerInput_04593A", rust: "system::turret::read_input",
                 run: |m| { let t = m.cpu.a[6]; turret::read_input(m, t); Flow::Return } },
    SystemPort { entry: 0x04_580C, original: "Turret_FireGroundSnap_04580C", rust: "system::turret::fire",
                 run: |m| { let t = m.cpu.a[6]; turret::fire(m, t); Flow::Return } },
    SystemPort { entry: 0x04_57A6, original: "Ent_SfxByMode_0457A6", rust: "system::turret::alloc_gfx",
                 run: |m| { let t = m.cpu.a[6]; turret::alloc_gfx(m, t); Flow::Return } },
    SystemPort { entry: 0x04_57CC, original: "Ent_AnimFrame_0457CC", rust: "system::turret::anim_frame",
                 run: |m| { let t = m.cpu.a[6]; turret::anim_frame(m, t); Flow::Return } },
    SystemPort { entry: 0x05_CA2A, original: "Entity_Draw_05ca2a", rust: "system::turret::draw",
                 run: |m| { let t = m.cpu.a[6]; turret::draw(m, t); Flow::Return } },
    SystemPort { entry: 0x08_F714, original: "PlayerMask_Test_08f714", rust: "system::turret::player_mask_hit",
                 run: |m| { let t = m.cpu.a[6]; let c = turret::player_mask_hit(m, t, m.cpu.d[1] as u8); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x05_E452, original: "Entity_OwnerAlive_05e452", rust: "system::turret::owner_alive",
                 run: |m| { let t = m.cpu.a[6]; let c = turret::owner_alive(m, t);
                            m.cpu.a[0] = if c { m.read_u32(t + mslug_game::layout::task::OWNER) } else { 0xFFFF_FFFF };
                            set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x05_D674, original: "Input_EffectivePad_05d674", rust: "system::turret::effective_pad",
                 run: |m| { let p = m.cpu.a[2]; m.cpu.a[2] = turret::effective_pad(m, p); Flow::Return } },
    SystemPort { entry: 0x05_17FE, original: "Entity_CopyPilot_0517fe", rust: "system::turret::copy_pilot",
                 run: |m| { let (t, a) = (m.cpu.a[6], m.cpu.a[0]); turret::copy_pilot(m, t, a); Flow::Return } },
    SystemPort { entry: 0x02_6992, original: "Entity_MoveXY_Probe_026992", rust: "system::motion::advance_on_slope",
                 run: |m| { m.cpu.a[5] = mslug_game::layout::collision::SCRATCH_BASE; let t = m.cpu.a[6]; motion::advance_on_slope(m, t); Flow::Return } },
    SystemPort { entry: 0x02_800E, original: "Entity_ScaleDyForGravity_02800e", rust: "system::motion::scale_dy",
                 run: |m| { let t = m.cpu.a[6]; let d = motion::scale_dy(m, t, m.cpu.d[2] as u16); m.cpu.d[2] = (m.cpu.d[2] & 0xFFFF_0000) | u32::from(d); Flow::Return } },
    // --- motor de colisión sobre pendientes
    SystemPort { entry: 0x02_7036, original: "Entity_MoveAndCollide_B_027036", rust: "system::slope::move_b",
                 run: |m| { let t = m.cpu.a[6]; let at = (m.cpu.d[3] as u16, m.cpu.d[4] as u16); let c = slope::move_b(m, t, at); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x02_7902, original: "Entity_StepB_UpdatePos_027902", rust: "system::slope::move_b_entity",
                 run: |m| { m.cpu.a[5] = mslug_game::layout::collision::SCRATCH_BASE; let t = m.cpu.a[6]; let at = (m.cpu.d[3] as u16, m.cpu.d[4] as u16);
                            let c = slope::move_b_entity(m, t, at); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x02_7E7E, original: "CollMap_TestSolidBitC_027e7e", rust: "system::slope::solid_bit",
                 run: |m| { let (r, c, w) = (m.cpu.a[1], m.cpu.d[3] as u16, m.cpu.d[4] as u16); mask_regs(m, r, 0, w); let b = slope::solid_bit(m, r, c, w); set_carry(m, b); Flow::Return } },
    SystemPort { entry: 0x02_7E9C, original: "CollMap_TestPlatformBitC_027e9c", rust: "system::slope::platform_bit",
                 run: |m| { let (r, c, w) = (m.cpu.a[1], m.cpu.d[3] as u16, m.cpu.d[4] as u16); mask_regs(m, r, 8, w); let b = slope::platform_bit(m, r, c, w); set_carry(m, b); Flow::Return } },
    SystemPort { entry: 0x09_98CA, original: "Trail_FindByKeyRange_0998ca", rust: "system::platforms::step_onto",
                 run: |m| { let t = m.cpu.a[6]; let c = platforms::step_onto(m, t, m.cpu.d[1] as u16, m.cpu.d[2] as u16); set_carry(m, c); Flow::Return } },
    // --- ruedas del Slug
    SystemPort { entry: 0x03_0002, original: "Chain3_Init_030002", rust: "system::wheels::init",
                 run: |m| { let t = m.cpu.a[6]; wheels::init(m, t); m.cpu.a[0] = mslug_game::layout::wheels::SCRIPT; Flow::Return } },
    SystemPort { entry: 0x03_0068, original: "Chain3_TplA_030068", rust: "system::wheels::start", run: |m| wheels::start(m, false) },
    SystemPort { entry: 0x03_00BA, original: "Chain3_TplB_0300ba", rust: "system::wheels::start", run: |m| wheels::start(m, true) },
    SystemPort { entry: 0x03_010C, original: "Chain3_TplC_03010c", rust: "system::wheels::middle_start", run: |m| wheels::middle_start(m) },
    SystemPort { entry: 0x03_0156, original: "Chain3_TplC_03010c+0x4A", rust: "system::wheels::middle_run", run: |m| wheels::middle_run(m) },
    SystemPort { entry: 0x03_01F0, original: "Chain3_Follow_0301f0", rust: "system::wheels::run",
                 run: |m| { let t = m.cpu.a[6]; m.write_u32(t, mslug_game::layout::wheels::RUN); wheels::run(m) } },
    SystemPort { entry: 0x03_01F6, original: "Chain3_Follow_0301f0+6", rust: "system::wheels::run", run: |m| wheels::run(m) },
    SystemPort { entry: 0x03_0392, original: "Chain3_YDelta_030392", rust: "system::wheels::too_far",
                 run: |m| { let t = m.cpu.a[6]; let c = wheels::too_far(m, t); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x03_03EE, original: "Chain3_Step_0303ee", rust: "system::wheels::step",
                 run: |m| { let t = m.cpu.a[6]; let y = wheels::step(m, t, m.cpu.d[0] as u16, m.cpu.d[1] as u16); m.cpu.d[0] = (m.cpu.d[0] & 0xFFFF_0000) | u32::from(y); Flow::Return } },
    SystemPort { entry: 0x03_0416, original: "Chain3_VelY_030416", rust: "system::wheels::vel_y",
                 run: |m| { let t = m.cpu.a[6]; wheels::vel_y(m, t); Flow::Return } },
    SystemPort { entry: 0x03_0462, original: "Chain3_VelX_030462", rust: "system::wheels::follow_x",
                 run: |m| { let t = m.cpu.a[6]; wheels::follow_x(m, t); Flow::Return } },
    SystemPort { entry: 0x03_050C, original: "Chain3_CheckSyncB_03050c", rust: "system::wheels::sync_climb",
                 run: |m| { let t = m.cpu.a[6]; wheels::sync_climb(m, t); Flow::Return } },
    SystemPort { entry: 0x03_0554, original: "Chain3_DebugHud_030554", rust: "system::wheels::debug_hud",
                 run: |m| { let t = m.cpu.a[6]; wheels::debug_hud(m, t); Flow::Return } },
    SystemPort { entry: 0x02_ABCC, original: "ClearXN_02abcc", rust: "(C = 0)", run: |m| { set_carry(m, false); Flow::Return } },
    SystemPort { entry: 0x02_773C, original: "Entity_MoveAndCollide_E_02773c", rust: "system::motion::move_e",
                 run: |m| { let t = m.cpu.a[6]; let at = (m.cpu.d[3] as u16, m.cpu.d[4] as u16); let c = motion::move_e(m, t, at); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x02_7D50, original: "$027D50", rust: "system::motion::on_entity_pos(move_e)",
                 run: |m| { m.cpu.a[5] = mslug_game::layout::collision::SCRATCH_BASE; let t = m.cpu.a[6]; let at = (m.cpu.d[3] as u16, m.cpu.d[4] as u16);
                            let c = motion::on_entity_pos(m, t, |m| motion::move_e(m, t, at)); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x02_7C2A, original: "$027C2A", rust: "system::motion::on_entity_pos(move_and_stop)",
                 run: |m| { m.cpu.a[5] = mslug_game::layout::collision::SCRATCH_BASE; let t = m.cpu.a[6]; let row = m.cpu.d[4] as u16;
                            let c = motion::on_entity_pos(m, t, |m| slug::move_and_stop(m, t, row)); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x02_80C6, original: "$0280C6", rust: "system::turret::kind_at",
                 run: |m| { let t = m.cpu.a[6]; let k = turret::kind_at(m, t, m.cpu.d[1] as u16, m.cpu.d[2] as u16); set_carry(m, k != 0); Flow::Return } },
    SystemPort { entry: 0x05_D6C2, original: "$05D6C2", rust: "system::hud::put_hex4",
                 run: |m| { let (a, v) = (m.cpu.a[1] as u16, m.cpu.d[0] as u16); hud::put_hex4(m, a, v); Flow::Return } },
    // --- efecto de suelo bajo los pies
    SystemPort { entry: 0x07_7686, original: "MuzzleFx_InitA_077686", rust: "system::ground_fx::start", run: |m| ground_fx::start(m, 0) },
    SystemPort { entry: 0x07_76B4, original: "MuzzleFx_InitB_0776b4", rust: "system::ground_fx::start", run: |m| ground_fx::start(m, 1) },
    SystemPort { entry: 0x07_76E2, original: "MuzzleFx_InitC_0776e2", rust: "system::ground_fx::start", run: |m| ground_fx::start(m, 2) },
    SystemPort { entry: 0x07_7710, original: "MuzzleFx_InitD_077710", rust: "system::ground_fx::start", run: |m| ground_fx::start(m, 3) },
    SystemPort { entry: 0x07_773E, original: "MuzzleFx_InitE_07773e", rust: "system::ground_fx::start", run: |m| ground_fx::start(m, 4) },
    SystemPort { entry: 0x07_78A2, original: "MuzzleFx_Track_0778a2", rust: "system::ground_fx::track", run: |m| ground_fx::track(m) },
    SystemPort { entry: 0x07_78FC, original: "MuzzleFx_WaitAim_0778fc", rust: "system::ground_fx::wait", run: |m| ground_fx::wait(m) },
    SystemPort { entry: 0x07_7912, original: "MuzzleFx_Flash_077912", rust: "system::ground_fx::land", run: |m| ground_fx::land(m) },
    SystemPort { entry: 0x07_7958, original: "MuzzleFx_Flash_077912+0x46", rust: "system::ground_fx::land_run", run: |m| ground_fx::land_run(m) },
    SystemPort { entry: 0x07_776C, original: "MuzzleFx_Attach_07776c", rust: "system::ground_fx::walk_start", run: |m| ground_fx::walk_start(m) },
    SystemPort { entry: 0x07_7778, original: "MuzzleFx_Attach_07776c+0xC", rust: "system::ground_fx::walk", run: |m| ground_fx::walk(m) },
    SystemPort { entry: 0x07_7A16, original: "MuzzleFx_Detached_077a16", rust: "system::ground_fx::detached", run: |m| ground_fx::detached(m) },
    SystemPort { entry: 0x07_7A60, original: "MuzzleFx_Detached_077a16+0x4A", rust: "system::ground_fx::detached_run", run: |m| ground_fx::detached_run(m) },
    SystemPort { entry: 0x07_7A8E, original: "JmpToScheduler_077a8e", rust: "system::tasks::finish", run: |m| tasks::finish(m) },
    SystemPort { entry: 0x02_7FAC, original: "Entity_TestSolidAtFeet_027fac", rust: "system::ground_fx::solid_at_feet",
                 run: |m| { let t = m.cpu.a[6]; let at = (m.cpu.d[3] as u16, m.cpu.d[4] as u16); let (on, raw) = ground_fx::solid_at_feet(m, t, at);
                            m.cpu.d[7] = (m.cpu.d[7] & 0xFFFF_FF00) | u32::from(raw); set_carry(m, !on); Flow::Return } },
    // --- botones del mando de la entidad (InputEvtThunk_*)
    SystemPort { entry: 0x05_CD60, original: "InputEvtThunk_05cd60", rust: "input::pressed", run: |m| pad_test(m, 0x10, Edge::Held) },
    SystemPort { entry: 0x05_CD6C, original: "InputEvtThunk_05cd6c", rust: "input::pressed", run: |m| pad_test(m, 0x20, Edge::Held) },
    SystemPort { entry: 0x05_CD78, original: "InputEvtThunk_05cd78", rust: "input::pressed", run: |m| pad_test(m, 0x40, Edge::Held) },
    SystemPort { entry: 0x05_CD84, original: "InputEvtThunk_05cd84", rust: "input::pressed", run: |m| pad_test(m, 0x80, Edge::Held) },
    SystemPort { entry: 0x05_CD90, original: "InputEvtThunk_05cd90", rust: "input::pressed", run: |m| pad_test(m, 0xF0, Edge::Held) },
    SystemPort { entry: 0x05_CD9C, original: "InputEvtThunk_05cd9c", rust: "input::pressed", run: |m| pad_test(m, 0xE0, Edge::Held) },
    SystemPort { entry: 0x05_CDA8, original: "InputEvtThunk_05cda8", rust: "input::pressed", run: |m| pad_test(m, 0x10, Edge::Pressed) },
    SystemPort { entry: 0x05_CDB4, original: "InputEvtThunk_05cdb4", rust: "input::pressed", run: |m| pad_test(m, 0x20, Edge::Pressed) },
    SystemPort { entry: 0x05_CDC0, original: "InputEvtThunk_05cdc0", rust: "input::pressed", run: |m| pad_test(m, 0x40, Edge::Pressed) },
    SystemPort { entry: 0x05_CDCC, original: "InputEvtThunk_05cdcc", rust: "input::pressed", run: |m| pad_test(m, 0x80, Edge::Pressed) },
    SystemPort { entry: 0x05_CDD8, original: "InputEvtThunk_05cdd8", rust: "input::pressed", run: |m| pad_test(m, 0x30, Edge::Pressed) },
    SystemPort { entry: 0x05_CDE4, original: "InputEvtThunk_05cde4", rust: "input::pressed", run: |m| pad_test(m, 0x50, Edge::Pressed) },
    SystemPort { entry: 0x05_CDF0, original: "InputEvtThunk_05cdf0", rust: "input::pressed", run: |m| pad_test(m, 0x70, Edge::Pressed) },
    SystemPort { entry: 0x05_CDFC, original: "InputEvtThunk_05cdfc", rust: "input::pressed", run: |m| pad_test(m, 0xF0, Edge::Pressed) },
    SystemPort { entry: 0x05_CE08, original: "InputEvtThunk_05ce08", rust: "input::pressed", run: |m| pad_test(m, 0xE0, Edge::Pressed) },
    SystemPort { entry: 0x05_CEEC, original: "InputEvtThunk_05ceec", rust: "input::pressed", run: |m| pad_test(m, 0x01, Edge::Held) },
    SystemPort { entry: 0x05_CEF8, original: "InputEvtThunk_05cef8", rust: "input::pressed", run: |m| pad_test(m, 0x02, Edge::Held) },
    SystemPort { entry: 0x05_CF04, original: "InputEvtThunk_05cf04", rust: "input::pressed", run: |m| pad_test(m, 0x08, Edge::Held) },
    SystemPort { entry: 0x05_CF10, original: "InputEvtThunk_05cf10", rust: "input::pressed", run: |m| pad_test(m, 0x04, Edge::Held) },
    SystemPort { entry: 0x05_CF1C, original: "InputEvtThunk_05cf1c", rust: "input::pressed", run: |m| pad_test(m, 0x0C, Edge::Held) },
    SystemPort { entry: 0x05_CF2C, original: "InputEvtThunk_05cf2c", rust: "input::pressed", run: |m| pad_test(m, 0x09, Edge::Held) },
    SystemPort { entry: 0x05_CF3C, original: "InputEvtThunk_05cf3c", rust: "input::pressed", run: |m| pad_test(m, 0x0A, Edge::Held) },
    SystemPort { entry: 0x05_CF4C, original: "InputEvtThunk_05cf4c", rust: "input::pressed", run: |m| pad_test(m, 0x05, Edge::Held) },
    SystemPort { entry: 0x05_CF5C, original: "InputEvtThunk_05cf5c", rust: "input::pressed", run: |m| pad_test(m, 0x06, Edge::Held) },
    SystemPort { entry: 0x05_CF6C, original: "InputEvtThunk_05cf6c", rust: "input::pressed", run: |m| pad_test(m, 0x08, Edge::Pressed) },
    SystemPort { entry: 0x05_CF78, original: "InputEvtThunk_05cf78", rust: "input::pressed", run: |m| pad_test(m, 0x04, Edge::Pressed) },
    SystemPort { entry: 0x05_CF84, original: "InputEvtThunk_05cf84", rust: "input::pressed", run: |m| pad_test(m, 0x01, Edge::Pressed) },
    SystemPort { entry: 0x05_CF90, original: "InputEvtThunk_05cf90", rust: "input::pressed", run: |m| pad_test(m, 0x02, Edge::Pressed) },
    SystemPort { entry: 0x05_CF9C, original: "InputEvtThunk_05cf9c", rust: "input::pressed", run: |m| pad_test(m, 0x0C, Edge::Pressed) },
    // --- jugador: alta y pasos comunes
    SystemPort { entry: 0x03_35A6, original: "PlayerEntitySpawn_0335A6", rust: "system::player::spawn", run: |m| player::spawn(m) },
    SystemPort { entry: 0x03_2A02, original: "PlayerEntity_InitAuxState_032A02", rust: "system::player::init_aux",
                 run: |m| { let t = m.cpu.a[6]; player::init_aux(m, t); Flow::Return } },
    SystemPort { entry: 0x05_17AA, original: "Player_SetIndexFromParent_0517aa", rust: "system::player::set_controller",
                 run: |m| { let t = m.cpu.a[6]; player::set_controller(m, t); Flow::Return } },
    SystemPort { entry: 0x03_2FF2, original: "Player_FrameCommon_032ff2", rust: "system::player::frame_common",
                 run: |m| { let t = m.cpu.a[6]; player::frame_common(m, t); Flow::Return } },
    SystemPort { entry: 0x03_3016, original: "Player_FrameCommon_NoPrio_033016", rust: "system::player::frame_common_no_prio",
                 run: |m| { let t = m.cpu.a[6]; player::frame_common_no_prio(m, t); Flow::Return } },
    SystemPort { entry: 0x03_2CC8, original: "Player_InvulnBlinkStep_032cc8", rust: "system::player::invuln_blink",
                 run: |m| { let t = m.cpu.a[6]; player::invuln_blink(m, t); Flow::Return } },
    SystemPort { entry: 0x02_A720, original: "Slug_UpdateInputFlags_02a720", rust: "system::player::dir_flags",
                 run: |m| { let t = m.cpu.a[6]; player::dir_flags(m, t); Flow::Return } },
    SystemPort { entry: 0x03_2F3C, original: "Player_ReadDirNibble_032f3c", rust: "system::player::dir_nibble",
                 run: |m| { let t = m.cpu.a[6]; player::dir_nibble(m, t); Flow::Return } },
    SystemPort { entry: 0x03_2F88, original: "Player_ReadFireNibble_032f88", rust: "system::player::fire_nibble",
                 run: |m| { let t = m.cpu.a[6]; player::fire_nibble(m, t); Flow::Return } },
    SystemPort { entry: 0x03_2E48, original: "Input_ForwardByFacing_032e48", rust: "system::player::forward_held",
                 run: |m| { let t = m.cpu.a[6]; let c = player::forward_held(m, t); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x03_2AA8, original: "Player_CheckDeathOrState21_032aa8", rust: "system::player::anim_done",
                 run: |m| { let t = m.cpu.a[6]; let c = player::anim_done(m, t); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x02_7BC8, original: "$027BC8", rust: "system::motion::on_entity_pos(move_and_stop)",
                 run: |m| { m.cpu.a[5] = mslug_game::layout::collision::SCRATCH_BASE; let t = m.cpu.a[6]; let row = m.cpu.d[4] as u16;
                            let c = motion::on_entity_pos(m, t, |m| slug::move_and_stop(m, t, row)); set_carry(m, c); Flow::Return } },
    SystemPort { entry: 0x03_2FF0, original: "Stub_00032FF0", rust: "(rts)", run: |_| Flow::Return },
    SystemPort { entry: 0x03_2E8E, original: "Stub_00032E8E", rust: "(rts)", run: |_| Flow::Return },
    // --- foco de la cámara
    SystemPort { entry: 0x09_67C0, original: "Attract_Sub_setup_967C0", rust: "system::scroll::focus",
                 run: |m| {
                     let ((x0, y0), (x2, y2)) = scroll::focus(m);
                     m.cpu.d[0] = x0; m.cpu.d[1] = y0; m.cpu.d[2] = x2; m.cpu.d[3] = y2;
                     m.cpu.a[0] = mslug_game::layout::focus::PLAYERS[1];
                     Flow::Return
                 } },
    SystemPort { entry: 0x05_E2D8, original: "Player_Point_05E2D8", rust: "system::scroll::player_point",
                 run: |m| { let t = m.cpu.a[0]; let (x, y) = scroll::player_point(m, t); m.cpu.d[0] = x; m.cpu.d[1] = y; Flow::Return } },
    SystemPort { entry: 0x04_36DE, original: "Geom_Proj_Clamp_0436DE", rust: "system::scroll::follow",
                 run: |m| { let d = m.cpu.d; scroll::follow(m, (d[0], d[1]), (d[2], d[3])); Flow::Return } },
    // --- listas por frame (las renueva el frame del scroll)
    SystemPort { entry: 0x07_7092, original: "Platform_ClearListByScroll_077092", rust: "system::lists::clear_platforms", run: ret!(lists::clear_platforms) },
    SystemPort { entry: 0x08_F172, original: "Rings_CompactAll_08f172", rust: "system::lists::advance_rings", run: ret!(lists::advance_rings) },
    SystemPort { entry: 0x08_F148, original: "Ring_Compact_08f148", rust: "system::lists::advance_ring",
                 run: |m| { let r = m.cpu.a[0]; lists::advance_ring(m, r); Flow::Return } },
    SystemPort { entry: 0x09_97CC, original: "Trail_RingAdvance_0997cc", rust: "system::lists::advance_trail", run: ret!(lists::advance_trail) },
    SystemPort { entry: 0x08_F8E2, original: "PlayerSlot_ClearP3Cur_08f8e2", rust: "system::lists::clear_parity_byte", run: ret!(lists::clear_parity_byte) },
    // --- tiles y colisión de las capas
    SystemPort { entry: 0x05_1C08, original: "Layer_Refresh_051C08", rust: "system::layer::refresh", run: |m| layer_port(m, Paint::Refresh) },
    SystemPort { entry: 0x05_1C82, original: "Layer_ScrollX_051C82", rust: "system::layer::scroll_x", run: |m| layer_port(m, Paint::X) },
    SystemPort { entry: 0x05_1CF6, original: "Layer_ScrollY_051CF6", rust: "system::layer::scroll_y", run: |m| layer_port(m, Paint::Y) },
    SystemPort { entry: 0x04_3DF4, original: "CameraHook_Probe08_043DF4", rust: "system::layer::refresh_all", run: |m| hook_port(m, Paint::Refresh) },
    SystemPort { entry: 0x04_3E0E, original: "CameraHook_ScrollX_043E0E", rust: "system::layer::scroll_x_all", run: |m| hook_port(m, Paint::X) },
    SystemPort { entry: 0x04_3E24, original: "CameraHook_ScrollY_043E24", rust: "system::layer::scroll_y_all", run: |m| hook_port(m, Paint::Y) },
    SystemPort { entry: 0x05_1BA8, original: "Layer_TileLookup_051BA8", rust: "system::layer::tile_lookup",
                 run: |m| { let (c, col) = (m.cpu.a[0], m.cpu.d[0] as u16); layer::tile_lookup(m, c, col); Flow::Return } },
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
