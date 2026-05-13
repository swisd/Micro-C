//! Micro-C: A minimalist, multi-target compiler.
//!
//! This crate provides a compiler for a small C-like language, designed for
//! use in `no_std` environments. It supports multiple architectures including
//! x86_64, Windows x64, and ARM64.

#![no_std]
extern crate alloc;

mod lexer;
mod parser;
mod ast;
mod interpreter;
mod ir;
mod codegen_ir;
mod backend;
mod regalloc;
mod emitter;
mod compiler;
mod arch;
mod stackframe;
mod error;
mod fs;

use alloc::fmt::format;
use alloc::format;
// use std::{env, fs};
// use std::fs::read_to_string;
use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;
use crate::compiler::compile;
use crate::error::print;

fn main() {
//     let input = fs::read_to_string("./micro-src/main.micro").unwrap();
//         // export fn main() {
//         //     let addr = 0x1000;
//         //
//         //     poke(addr, 42);
//         //
//         //     let x = peek(addr);
//         //
//         //     if (x >= 10) {
//         //         x = x + 1;
//         //     }
//         //
//         //     return x;
//         // }
// //         r#"
// //         fn add(a, b) {
// //     return a + b;
// // }
// //
// // export fn main() {
// //     add(1, 2);          // statement call
// //     let x = add(3, 4);  // expression call
// //     return x;
// // }
// //     "#;
//
//     let lexer = Lexer::new(&*input);
//     let mut parser = Parser::new(lexer);
//
//     let ast = parser.parse_program();
//
//     let mut interp = Interpreter::new();
//     let result = interp.run(&ast);
//
//     println!("Result: {}", result);
    //let args: Vec<String> = env::args().collect();

    // args[0] is the program name, args[1] is the first user argument

//     let source = r#"
// fn add(a, b) {
//     return a + b;
// }
//
// export fn main() {
//     let x = add(5, 3);
//     return x;
// }
// "#;
    let source = r#"
    #include <Sys>
    /*
    adds two numbers together (a, b)
*/
fn add(a, b) {
    return a + b;
}

/*
    2d point struct
*/
//struct Point {
//    x: i64;
//    y: i64;
//}

// Main entry func
export fn main() {
    //let p: ptr = alloc_struct(Point);

    //p.x = 10;
    //p.y = 20;

    let z = add(10, 5);

    return z; //p.x + p.y
}
    "#;
    let asm = compile(&*source, "x86_64");

    print(&format!("{}", asm));
}