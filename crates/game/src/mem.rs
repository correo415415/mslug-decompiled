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
