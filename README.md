# Minimal Arm PL011 (PrimeCell) UART driver

This crate was constructed due to the need to have a minimal PL011 driver that doesn't have a MSRV > 1.72.1 and doesn't depend on
`safe-mmio`. This is not meant to replace in any way [`arm-pl011-uart`](https://crates.io/crates/arm-pl011-uart) and if you are not restricted
to an older Rust version as I am, I suggest you use the `arm-pl011-uart` crate.
