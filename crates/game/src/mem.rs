//! Acceso al estado del juego.
//!
//! El juego portado no sabe dónde vive su estado: lo pide a un `Memory`.
//! Durante la migración `Memory` es la RAM de trabajo del sistema original
//! (así cada función se puede comprobar contra la ROM); al final será un
//! almacenamiento nativo.

/// Memoria big-endian direccionada por bytes (como la RAM del 68000).
pub trait Memory {
    fn read_u8(&mut self, addr: u32) -> u8;
    fn write_u8(&mut self, addr: u32, v: u8);

    fn read_u16(&mut self, addr: u32) -> u16 {
        u16::from_be_bytes([self.read_u8(addr), self.read_u8(addr + 1)])
    }
    fn write_u16(&mut self, addr: u32, v: u16) {
        let [h, l] = v.to_be_bytes();
        self.write_u8(addr, h);
        self.write_u8(addr + 1, l);
    }
    fn read_u32(&mut self, addr: u32) -> u32 {
        (u32::from(self.read_u16(addr)) << 16) | u32::from(self.read_u16(addr + 2))
    }
    fn write_u32(&mut self, addr: u32, v: u32) {
        self.write_u16(addr, (v >> 16) as u16);
        self.write_u16(addr + 2, v as u16);
    }
}

/// Memoria plana de prueba (64 KiB de RAM de trabajo en `layout::RAM_BASE`).
#[derive(Clone)]
pub struct TestRam(pub Vec<u8>);

impl Default for TestRam {
    fn default() -> Self {
        Self(vec![0; 0x10000])
    }
}

impl Memory for TestRam {
    fn read_u8(&mut self, addr: u32) -> u8 {
        self.0[(addr - crate::layout::RAM_BASE) as usize]
    }
    fn write_u8(&mut self, addr: u32, v: u8) {
        self.0[(addr - crate::layout::RAM_BASE) as usize] = v;
    }
}

/// Registros con los que se llama a una rutina original todavía sin portar
/// (argumentos de entrada y resultados de salida). Sólo existe durante la
/// migración: cuando la rutina se porta, se llama a la función Rust con
/// argumentos con nombre.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Regs {
    pub d: [u32; 8],
    pub a: [u32; 7],
    /// Flag C a la vuelta (muchas rutinas devuelven sí/no en C).
    pub carry: bool,
}

impl Regs {
    /// Sólo `a6` (la tarea / entidad con la que trabaja la rutina).
    pub fn with_a6(a6: u32) -> Self {
        let mut r = Self::default();
        r.a[6] = a6;
        r
    }
}

/// La máquina completa: memoria (RAM, paleta, registros de vídeo/sonido,
/// ROM) y, durante la migración, la posibilidad de llamar a rutinas
/// originales que aún no se han reescrito.
pub trait Machine: Memory {
    /// Máscara de interrupciones de la CPU (0 = todas permitidas).
    fn set_interrupt_mask(&mut self, level: u8);
    /// Puntero de pila (el planificador lo guarda y lo restaura alrededor
    /// de cada tarea).
    fn stack_pointer(&mut self) -> u32;
    fn set_stack_pointer(&mut self, sp: u32);
    /// Registro de la tarea / entidad en curso (`a6` en el original): el
    /// planificador lo fija antes de llamar a cada tarea.
    fn task_register(&mut self) -> u32;
    fn set_task_register(&mut self, a6: u32);
    /// Registros actuales, como base para llamar a una rutina original.
    fn regs(&mut self) -> Regs;
    fn set_regs(&mut self, r: Regs);

    /// Llama a la rutina original que empieza en `routine` (dirección de
    /// `layout::routine`, o un puntero leído de una tabla del juego). Si
    /// ya está portada, el puente ejecuta directamente la función Rust.
    fn call_original(&mut self, routine: u32, regs: &mut Regs);

    /// Como [`Machine::call_original`] sin argumentos; devuelve el flag C.
    fn call(&mut self, routine: u32) -> bool {
        let mut r = self.regs();
        self.call_original(routine, &mut r);
        r.carry
    }

    /// Si una llamada a código original se ha quedado parada en una rutina
    /// sin portar (`--only-manual`): los bucles que esperan algo de esas
    /// llamadas deben salir para no colgarse.
    fn halted(&mut self) -> bool {
        false
    }
}

/// Máquina de prueba: memoria plana de 16 MiB que anota las llamadas a
/// rutinas originales.
#[derive(Clone)]
pub struct TestMachine {
    pub mem: Vec<u8>,
    pub calls: Vec<u32>,
    pub regs: Regs,
    pub sp: u32,
    pub irq_mask: u8,
}

impl Default for TestMachine {
    fn default() -> Self {
        Self { mem: vec![0; 0x100_0000], calls: Vec::new(), regs: Regs::default(), sp: 0x10_F300, irq_mask: 7 }
    }
}

impl Memory for TestMachine {
    fn read_u8(&mut self, addr: u32) -> u8 {
        self.mem[(addr & 0xFF_FFFF) as usize]
    }
    fn write_u8(&mut self, addr: u32, v: u8) {
        self.mem[(addr & 0xFF_FFFF) as usize] = v;
    }
}

impl Machine for TestMachine {
    fn set_interrupt_mask(&mut self, level: u8) {
        self.irq_mask = level;
    }
    fn stack_pointer(&mut self) -> u32 {
        self.sp
    }
    fn set_stack_pointer(&mut self, sp: u32) {
        self.sp = sp;
    }
    fn task_register(&mut self) -> u32 {
        self.regs.a[6]
    }
    fn set_task_register(&mut self, a6: u32) {
        self.regs.a[6] = a6;
    }
    fn regs(&mut self) -> Regs {
        self.regs
    }
    fn set_regs(&mut self, r: Regs) {
        self.regs = r;
    }
    fn call_original(&mut self, routine: u32, _: &mut Regs) {
        self.calls.push(routine);
    }
}

/// Cómo sigue la ejecución al acabar una rutina de sistema (las del
/// arranque, las interrupciones y el planificador no siempre «vuelven»).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flow {
    /// Vuelve al llamador (`rts`).
    Return,
    /// Fin de la interrupción (`rte`).
    ReturnFromInterrupt,
    /// Sigue en otra rutina sin volver (`jmp`), p. ej. a la BIOS.
    Goto(u32),
    /// Llama a `routine` y, cuando vuelva, sigue en `then` (el planificador
    /// llama así a cada tarea, que puede quedarse esperando al vblank).
    Call { routine: u32, then: u32 },
    /// Espera activa: se vuelve a llamar a la misma rutina en el siguiente
    /// paso (mientras, el resto de la máquina avanza: vídeo, interrupciones).
    Wait,
}
