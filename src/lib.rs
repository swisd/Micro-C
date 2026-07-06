extern crate alloc;
mod compiler; 


#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn compile_from_web(source_code: &str, target_arch: &str) -> String {
    // Bridges the string from JavaScript into your internal no_std compiler pipeline
    match crate::compiler::compile(source_code, target_arch) {
        Ok(assembly) => assembly,
        Err(e) => alloc::format!("Compilation Error: {:?}", e),
    }
}
