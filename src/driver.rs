use crate::registers::{Pl011Registers, DR};
use core::{
    fmt,
    ptr::NonNull,
    sync::atomic::{AtomicBool, Ordering},
};
use tock_registers::interfaces::Writeable;

#[cfg(target_abi = "purecap")]
use cheri::{prelude::*, ptr::Perms};

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
    /// The caller must guarantee that the base address points to a valid MMIO PL011 region
    pub unsafe fn new(base: NonNull<u32>) -> Option<Self> {
        let is_taken = TAKEN.swap(true, Ordering::AcqRel);
        (!is_taken).then(|| Self {
            registers: base.cast(),
        })
    }

    /// Create a new [`Pl011Uart`] instance from an address, deriving the DDC
    ///
    /// # Safety
    ///
    /// The caller must guarantee that the base address points to a valid MMIO PL011 region
    #[cfg(target_abi = "purecap")]
    pub unsafe fn from_address(addr: usize) -> Option<Self> {
        let ddc: *mut u32 = cheri::ptr::default_data_mut();
        let ptr = ddc
            .with_addr(addr)
            .with_perms_clear_except(Perms::LOAD | Perms::STORE)
            .with_bounds(4);

        // SAFETY: Safety contract should be guaranteed by the caller
        unsafe { Self::new(NonNull::new_unchecked(ptr)) }
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
