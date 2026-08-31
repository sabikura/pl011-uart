//! Small ARM PL011 (PrimeCell) UART driver

#![no_std]

mod driver;
pub mod registers;

use core::{cell::RefCell, fmt};

use critical_section::Mutex;
pub use driver::Pl011Uart;

/// Global writer backing the [`print!`] and [`println!`] macros
struct Writer(Mutex<RefCell<Option<Pl011Uart>>>);
static WRITER: Writer = Writer(Mutex::new(RefCell::new(None)));

/// Configure a [`Pl011Uart`] as the global writer used by [`print!`] and [`println!`].
///
/// Returns the previously owned instance, if any. Until this is called, the
/// prints silently discard their input.
pub fn set_writer(uart: Pl011Uart) -> Option<Pl011Uart> {
    critical_section::with(|cs| WRITER.0.borrow_ref_mut(cs).replace(uart))
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    critical_section::with(|cs| {
        if let Some(uart) = WRITER.0.borrow_ref_mut(cs).as_mut() {
            // Writing to the UART is infallible, see `Pl011Uart::write_str`
            let _ = fmt::Write::write_fmt(uart, args);
        }
    });
}

/// Print to the global UART writer installed with [`init`]
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::_print(::core::format_args!($($arg)*))
    };
}

/// Print to the global UART writer installed with [`init`], with a trailing newline
#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {
        $crate::_print(::core::format_args!("{}\n", ::core::format_args!($($arg)*)))
    };
}
