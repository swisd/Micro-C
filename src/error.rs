//! Error handling and diagnostic printing.
//!
//! This module provides functions for reporting compilation errors
//! and printing messages in a `no_std` environment.

use alloc::format;
use alloc::string::{String, ToString};
use core::fmt::{Write, Error};
use alloc::vec::Vec;

// Static mutable array to collect errors for WASM logging
static mut ERRORS: Option<Vec<String>> = None;

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

/// Initializes the error collection system.
/// Should be called before compilation begins.
pub fn init_errors() {
    unsafe {
        ERRORS = Some(Vec::new());
    }
}

/// Pushes an error message to the static error array.
/// This allows errors to be collected and retrieved for logging in WASM.
pub fn error(message: &str) {
    let formatted = format!("!! {}", message);
    unsafe {
        if let Some(ref mut errors) = ERRORS {
            errors.push(formatted);
        }
    }
}

/// Retrieves all collected errors as a Vec of Strings.
/// Returns a cloned copy of the error array.
pub fn get_errors() -> Vec<String> {
    unsafe {
        ERRORS.as_ref().cloned().unwrap_or_default()
    }
}

/// Clears all collected errors.
pub fn clear_errors() {
    unsafe {
        if let Some(ref mut errors) = ERRORS {
            errors.clear();
        }
    }
}

/// Prints a message to the output.
///
/// In a `no_std` environment, this typically writes to a serial port or
/// other hardware-specific output.
pub fn print(message: &str) {
    let mut writer = MyWriter;
    writer.write_str(message).expect("TODO: panic message");
}
