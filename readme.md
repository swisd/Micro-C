# Micro-C

Micro-C is a minimalist, multi-target compiler written in Rust for a C-like systems programming language. It is designed to be small, understandable, and suitable for `no_std` environments (e.g., O[...]


![](https://img.shields.io/badge/X86__64-functional-green)
![](https://img.shields.io/badge/WIN64-semi_functional-yellow)
![](https://img.shields.io/badge/ARM64-not%20working-red)

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
- **Control Flow**: `if`, `else`, `elif`, and `loop` statements.
- **Arithmetic**: Basic operations (`+`, `-`, `*`, `/`) and comparisons.
- **Structs**: Basic structure definitions (WIP).
- **Pointers**: `peek` and `poke` for direct memory access.
- **Imports and externs**: `#include <Sys>` expands built-in declarations, and `extern fn name(args);` declares linker-provided functions.

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

## Syntax

### Comments

Micro-C supports both line and block comments:

```c
// Single-line comment

/* Multi-line
   block comment */
```

### Types

Micro-C has the following built-in types:

```c
i64       // 64-bit signed integer (default numeric type)
bool      // Boolean type
ptr       // Generic pointer type (can point to any type)
```

Type annotations are optional in variable declarations but can be specified explicitly:

```c
let x: i64 = 42;
let flag: bool = 1;
let p: ptr = malloc(64);
```

### Literals

**Integers**: Micro-C supports decimal, hexadecimal, and binary integer literals:

```c
let decimal = 42;      // Decimal literal
let hex = 0xFF;        // Hexadecimal literal (0x prefix)
let binary = 0b1010;   // Binary literal (0b prefix)
```

### Variables and Assignment

**Variable Declaration** with `let`:

```c
let name = value;              // Type inferred from value
let name: type = value;        // Explicit type annotation
```

**Variable Assignment**:

```c
let x = 10;
x = 20;                        // Reassign to new value
```

**Field Assignment** (for structs):

```c
let p: ptr = alloc_struct(Point);
p.x = 10;                      // Assign to struct field
p.y = 20;
```

**Array/Index Assignment**:

```c
arr[0] = 100;                  // Assign to array/indexed element
```

### Operators

**Arithmetic Operators**:
- `+` Addition
- `-` Subtraction
- `*` Multiplication
- `/` Division

**Comparison Operators**:
- `==` Equality
- `!=` Inequality
- `<` Less than
- `>` Greater than
- `<=` Less than or equal
- `>=` Greater than or equal

**Other**:
- `=` Assignment

Operators follow standard precedence:
1. Multiplication (`*`) and Division (`/`)
2. Addition (`+`) and Subtraction (`-`)
3. Comparisons (`<`, `>`, `<=`, `>=`, `==`, `!=`)
4. Assignment (`=`)

### Functions

**Basic Function Definition**:

```c
fn add(a, b) {
    return a + b;
}
```

**Exported Functions** (entry points visible to the linker):

```c
export fn main() {
    return 0;
}
```

**Function Calls**:

```c
let result = add(10, 5);
let nested = add(mul(2, 3), sub(10, 5));
```

**External Functions** (linker-provided, e.g., from libc or kernel):

```c
extern fn malloc(size);
extern fn host_add(a, b);

export fn main() {
    let ptr = malloc(64);
    return host_add(ptr, 1);
}
```

Functions support:
- Arbitrary number of parameters (untyped)
- Return statements with expressions
- Recursion (tail call optimization not guaranteed)
- Nested function calls
- Early returns

### Control Flow

**If / Elif / Else Statements**:

```c
if (x == 5) {
    return 100;
} elif (x > 5) {
    return 200;
} elif (x < 5) {
    return 300;
} else {
    return 0;
}
```

**Loop Statement** (unbounded loop):

```c
loop {
    if (i == 10) {
        return i;
    }
    i = i + 1;
}
```

**Break and Continue**:

```c
loop {
    if (should_exit) {
        break;             // Exit loop
    }
    if (should_skip) {
        continue;          // Skip to next iteration
    }
    // ... loop body
}
```

### Structs

**Struct Definition**:

```c
struct Point {
    x: i64;
    y: i64;
}

struct Complex {
    real: i64;
    imag: i64;
}
```

**Struct Allocation** (using external `alloc_struct` function):

```c
let p: ptr = alloc_struct(Point);
p.x = 10;
p.y = 20;
```

**Field Access**:

```c
let value = p.x;           // Read field
p.y = value + 5;           // Write field
```

**Chained Field Access**:

```c
// Structs containing structs (via pointers)
let nested = outer.inner.field;
```

### Memory Operations

**Peek** (read from memory address):

```c
let value = peek(address);    // Read i64 from address
```

Internally, `peek` is syntactic sugar for a function call:

```c
let byte_at_100 = peek(100);
```

**Poke** (write to memory address):

```c
poke(address, value);         // Write i64 to address
```

Example:

```c
let ptr = malloc(8);
poke(ptr, 42);                // Write 42 to ptr
let result = peek(ptr);       // Read back: result = 42
```

### Array Indexing

**Array/Pointer Indexing**:

```c
let arr = malloc(100);        // Allocate memory
let first = arr[0];           // Read element
arr[10] = 99;                 // Write element
```

Indexing is offset-based (in units of the pointed-to type):

```c
let p: ptr = malloc(64);
p[0] = 10;                    // First i64
p[1] = 20;                    // Second i64 (offset by 8 bytes)
```

### Imports

**Include Directives**:

```c
#include <Sys>                // Include built-in system declarations
```

This expands the available external functions and declarations at compile time. The `<Sys>` module typically provides:
- Memory functions: `malloc`, `free`, `alloc_struct`
- I/O functions (architecture-dependent)
- System calls (architecture-dependent)

### Return Statement

**Return from Function**:

```c
fn example() {
    if (x == 0) {
        return 0;             // Early return
    }
    
    let result = x + 1;
    return result;            // Final return
}
```

Return expressions can be:
- Literals: `return 42;`
- Variables: `return result;`
- Function calls: `return add(10, 5);`
- Complex expressions: `return a + b * c;`

### Expressions

Micro-C supports the following expressions:

**Literals**:
```c
42                    // Integer literal
0xFF                  // Hex literal
0b1010                // Binary literal
```

**Variables**:
```c
x
my_variable
```

**Binary Operations**:
```c
a + b
x * y
p < 100
a == b
```

**Function Calls**:
```c
add(1, 2)
malloc(64)
nested_call(f(x), g(y))
```

**Memory Operations**:
```c
peek(ptr)             // Read memory
```

**Indexing**:
```c
arr[i]
ptr[10]
```

**Field Access**:
```c
struct_ptr.field
```

**Parenthesized Expressions**:
```c
(a + b) * c
(x > 10)
```

### Statements

Micro-C programs consist of statements:

```c
let x = 10;           // Variable declaration
x = 20;               // Assignment
if (...) { }          // Conditional
loop { }              // Loop
return x;             // Return
poke(ptr, val);       // Memory write
fn_call();            // Expression statement (function call)
```

### Example: Complete Program

```c
#include <Sys>

struct Point {
    x: i64;
    y: i64;
}

fn add(a, b) {
    return a + b;
}

fn factorial(n) {
    if (n == 0) {
        return 1;
    }
    return n * factorial(n - 1);
}

export fn main() {
    // Arithmetic
    let sum = add(10, 5);
    
    // Variables and assignment
    let x = 42;
    x = x + 10;
    
    // Control flow
    let result = 0;
    if (x > 50) {
        result = 1;
    } else {
        result = 0;
    }
    
    // Loops
    let i = 0;
    loop {
        if (i == 10) {
            break;
        }
        i = i + 1;
    }
    
    // Structs
    let p: ptr = alloc_struct(Point);
    p.x = 100;
    p.y = 200;
    
    // Memory operations
    let ptr = malloc(64);
    poke(ptr, 999);
    let val = peek(ptr);
    
    // Recursion
    let fact = factorial(5);
    
    return sum + x + result + fact;
}
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
