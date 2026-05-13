//! Error handling and diagnostic printing.
//!
//! This module provides functions for reporting compilation errors
//! and printing messages in a `no_std` environment.

use alloc::format;
use alloc::string::{String, ToString};
use core::fmt::{Write, Error};

struct MyWriter;

impl Write for MyWriter {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        for byte in s.bytes() {
            // Your hardware-specific code to send one byte
            // e.g., unsafe { send_to_uart(byte) };
        }
        Ok(())
    }
}

/// Reports a fatal error and panics.
pub fn error(message: &str) {
    panic!("!! {}", message)
}

/// Prints a message to the output.
///
/// In a `no_std` environment, this typically writes to a serial port or
/// other hardware-specific output.
pub fn print(message: &str) {
    let mut writer = MyWriter;
    writer.write_str(message).expect("TODO: panic message");
}
