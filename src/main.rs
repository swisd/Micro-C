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

use std::{env, fs};
use std::fs::read_to_string;
use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;
use crate::compiler::compile;

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
    let args: Vec<String> = env::args().collect();

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
    let source = read_to_string(args[1].clone()).expect("issue");
    let asm = compile(&*source, "x86_64");

    println!("{}", asm);
}