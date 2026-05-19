//! File system abstractions for `no_std`.
//!
//! This module provides basic file loading capabilities, which may
//! be mapped to internal libraries or actual file systems depending
//! on the environment.

use alloc::format;
use alloc::string::{String, ToString};
use crate::error::error;

/// Attempts to open a file from the given path or a standard library.
///
/// Returns the file content as a String.
pub fn open_file_or_lib(path: &str) -> String {
    match path {
        "Sys" | "sys" => {
            "extern fn printf(fmt, value);\nextern fn malloc(size);\nextern fn free(ptr);\n".to_string()
        }
        _ => {
            error(&format!("cant open file {}", path));
            "".to_string()
        }
    }
}
