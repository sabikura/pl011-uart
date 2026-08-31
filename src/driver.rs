use crate::registers::{Pl011Registers, DR, FR};
use core::{
    fmt,
    ptr::NonNull,
    sync::atomic::{AtomicBool, Ordering},
};
use tock_registers::interfaces::{Readable, Writeable};

/// Whether the [`Pl011Uart`] struct was already created
static TAKEN: AtomicBool = AtomicBool::new(false);

/// PL011 UART driver
pub struct Pl011Uart {
    registers: NonNull<Pl011Registers>,
}

// SAFETY: Pl011Uart is an opaque struct and the raw pointers inside can't be shared with
//         another thread in safe Rust.
unsafe impl Send for Pl011Uart {}

impl Pl011Uart {
    /// Create a new [`Pl011Uart`] instance.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that the pointer points to a valid MMIO PL011 register block
    pub unsafe fn new(registers: NonNull<Pl011Registers>) -> Option<Self> {
        let is_taken = TAKEN.swap(true, Ordering::AcqRel);
        (!is_taken).then(|| Self { registers })
    }

    /// Writes a byte
    pub fn write_byte(&mut self, byte: u8) {
        let regs = self.regs();
        regs.dr.write(DR::DATA.val(byte as u32));
    }

    /// Write a slice of data, without checking if the FIFO is full
    pub fn write_bytes(&mut self, data: &[u8]) {
        for byte in data {
            self.write_byte(*byte);
        }
    }

    /// Read a byte, `None` if the receive FIFO is empty
    pub fn read_byte(&mut self) -> Option<u8> {
        let regs = self.regs();
        if regs.fr.is_set(FR::RXFE) {
            return None;
        }
        Some(regs.dr.read(DR::DATA) as u8)
    }

    /// Read a byte, spinning until one is received
    pub fn read_byte_blocking(&mut self) -> u8 {
        loop {
            if let Some(byte) = self.read_byte() {
                return byte;
            }
        }
    }

    fn regs(&self) -> &Pl011Registers {
        // SAFETY: The constructor's caller should have guaranteed that the pointer to the PL011
        // registers is valid
        unsafe { self.registers.as_ref() }
    }
}

impl fmt::Write for Pl011Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_bytes(s.as_bytes());
        Ok(())
    }
}
