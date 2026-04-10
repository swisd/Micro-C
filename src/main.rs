mod lexer;
mod parser;
mod ast;
mod interpreter;

use std::fs;
use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;

fn main() {
    let input = fs::read_to_string("./micro-src/main.micro").unwrap();
        // export fn main() {
        //     let addr = 0x1000;
        //
        //     poke(addr, 42);
        //
        //     let x = peek(addr);
        //
        //     if (x >= 10) {
        //         x = x + 1;
        //     }
        //
        //     return x;
        // }
//         r#"
//         fn add(a, b) {
//     return a + b;
// }
//
// export fn main() {
//     add(1, 2);          // statement call
//     let x = add(3, 4);  // expression call
//     return x;
// }
//     "#;

    let lexer = Lexer::new(&*input);
    let mut parser = Parser::new(lexer);

    let ast = parser.parse_program();

    let mut interp = Interpreter::new();
    let result = interp.run(&ast);

    println!("Result: {}", result);
}