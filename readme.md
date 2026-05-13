# Micro-C

Micro-C is a minimalist, multi-target compiler written in Rust for a C-like systems programming language. It is designed to be small, understandable, and suitable for `no_std` environments (e.g., OS kernels or embedded systems).

## Features

- **C-like Syntax**: Familiar syntax for systems programmers.
- **Multiple Backends**:
  - `x86_64` (Raw Assembly)
  - `win64` (Windows x64 ABI)
  - `arm64` (AArch64)
- **Minimal Dependencies**: Built with `no_std` and `alloc` only.
- **IR-based Compilation**: Uses a custom Intermediate Representation (IR) for optimization and easier backend targeting.
- **Register Allocation**: Basic register allocator for efficient code generation.

## Language Support

- **Functions**: Support for function definitions, parameters, and calls.
- **Variables**: Local variables with `let`.
- **Control Flow**: `if`, `else`, and `loop` statements.
- **Arithmetic**: Basic operations (`+`, `-`, `*`, `/`) and comparisons.
- **Structs**: Basic structure definitions (WIP).
- **Pointers**: `peek` and `poke` for direct memory access.

## Architecture

The compiler follows a traditional pipeline:

1.  **Lexer (`src/lexer.rs`)**: Converts source text into a stream of tokens.
2.  **Parser (`src/parser.rs`)**: Transforms tokens into an Abstract Syntax Tree (AST) defined in `src/ast.rs`.
3.  **IR Generator (`src/codegen_ir.rs`)**: Converts the AST into a flat, three-address-style Intermediate Representation (`src/ir.rs`).
4.  **Backend (`src/arch/`)**: Translates IR instructions into target-specific assembly code, handling register allocation and ABI-specific details.

## Usage

The compiler can be invoked via the `compile` function in `src/compiler.rs`.

```rust
use micro_c::compiler::compile;

let source = r#"
fn add(a, b) {
    return a + b;
}

export fn main() {
    return add(10, 5);
}
"#;

let asm = compile(source, "x86_64");
println!("{}", asm);
```

## Project Structure

- `src/main.rs`: Entry point and example usage.
- `src/lexer.rs`: Lexical analyzer.
- `src/parser.rs`: Recursive descent parser.
- `src/ast.rs`: AST node definitions.
- `src/ir.rs`: Intermediate Representation instructions.
- `src/codegen_ir.rs`: AST to IR lowering.
- `src/regalloc.rs`: Register allocation logic.
- `src/arch/`: Architecture-specific backends.
- `src/error.rs`: Error handling and printing.
- `src/fs.rs`: Basic file system abstractions for `no_std`.
