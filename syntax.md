# Micro-C Syntax Reference

This document provides a comprehensive guide to the Micro-C language syntax, with expanded examples and detailed explanations for each language construct.

## Table of Contents

1. [Comments](#comments)
2. [Types](#types)
3. [Literals](#literals)
4. [Variables and Assignment](#variables-and-assignment)
5. [Operators](#operators)
6. [Functions](#functions)
7. [Control Flow](#control-flow)
8. [Structs](#structs)
9. [Memory Operations](#memory-operations)
10. [Array Indexing](#array-indexing)
11. [Imports](#imports)
12. [Return Statement](#return-statement)
13. [Expressions](#expressions)
14. [Statements](#statements)
15. [Monostatememts](#monostatements)
15. [Complete Examples](#complete-examples)

---

## Comments

Micro-C supports both single-line and multi-line comments:

### Single-Line Comments

```c
// This is a single-line comment
let x = 10;  // Comments can appear inline too
```

Single-line comments extend from `//` to the end of the line and are ignored by the compiler.

### Multi-Line / Block Comments

```c
/* This is a multi-line
   block comment that can
   span several lines */
```

Block comments begin with `/*` and end with `*/`. They can span multiple lines and cannot be nested.

### Common Usage Patterns

```c
/*
 * Function to calculate the sum of two numbers
 * Parameters:
 *   a - first number
 *   b - second number
 * Returns:
 *   The sum of a and b
 */
fn add(a, b) {
    return a + b;  // Simply return the result
}
```

---

## Types

Micro-C has five built-in types. All types are fixed-size and designed for low-level systems programming:

### i64 (64-bit Signed Integer)

The default numeric type. Represents signed integers from `-2^63` to `2^63 - 1`.

```c
let count: i64 = 100;
let negative: i64 = -42;
let zero: i64 = 0;
```

**Key Points:**
- Default type when a numeric literal is used without explicit type annotation
- Used for arithmetic, comparisons, and most computations
- On 64-bit systems, maps directly to hardware registers

### bool (Boolean Type)

Represents true/false values. Internally represented as 0 (false) or non-zero (true).

```c
let flag: bool = 1;        // true
let disabled: bool = 0;    // false
```

**Key Points:**
- Comparison operators return boolean values
- Any non-zero value is considered true in conditionals
- Useful for flags and status checks

### ptr (Generic Pointer Type)

A generic pointer that can point to any data in memory. All pointers are 64-bit addresses.

```c
let buffer: ptr = malloc(64);
let addr: ptr = 0x1000;      // Direct address
```

**Key Points:**
- Used for dynamic memory allocation
- Can be indexed like arrays
- Can access struct fields via `.` notation
- Can be read/written with `peek` and `poke`

### array (List/Array type)

A generic array type that can be used to store data, and can be nested.

```c
let list: array[3] = [0, 1, 2]; // list eith size of 3
let buffer: array[0] = [...]; // unsized list. '...' is a placeholder
let nest: array[3[0]] = [list, buffer, [1, 2]];
```

### str (Sized/Unsized string)

A generic string type. it can be used with a length definer to use it as a set length slice or a char.

```c
let text: str = "text";
let ch: str[1] = "A";
let sized: str[5] = "hello";
```

### Type Annotations

Type annotations are optional but can be specified explicitly for clarity:
> [!WARNING]
> It is recommended to use explicit types as inferred types may be dropped in upcoming updates.


```c
let x = 42;              // Type inferred as i64
let x: i64 = 42;        // Explicit type annotation
let flag: bool = 1;     // Explicit boolean
let p: ptr = malloc(8); // Explicit pointer

// Mixed usage
let a = 10;             // Inferred as i64
let b: i64 = a;        // Explicit for clarity
```

---

## Literals

### Integer Literals

Micro-C supports three formats for integer literals:

#### Decimal (Base 10)

```c
let decimal = 42;
let large = 1000000;
let negative = -99;
```

#### Hexadecimal (Base 16)

Prefix with `0x`:

```c
let hex = 0xFF;          // 255 in decimal
let color = 0xABCDEF;   // Color value
let mask = 0x0F;        // Low nibble mask
```

#### Binary (Base 2)

Prefix with `0b`:

```c
let binary = 0b1010;     // 10 in decimal
let flags = 0b11110000; // Bitmask
let bit_three = 0b00001000;
```

### Type Inference

```c
// All of these work without explicit type annotation
let a = 123;        // Inferred as i64
let b = 0xFF;       // Inferred as i64 from hex literal
let c = 0b1010;     // Inferred as i64 from binary literal
```

### Signed Values

Negative values are supported:

```c
let positive = 42;
let negative = -42;
let double_negative = -(-10);  // Valid expression
```

### Type definitions

New types can be define from a series of existing types.

```c
type char = str[1];
```
---

## Variables and Assignment

### Variable Declaration with `let`

Declare a new variable using the `let` keyword:


```c
let name = value;              // Type inferred from value
let name: type = value;        // Explicit type annotation
```

Constants can be declared with the `let static` phrase.

```c
let static name = value;              // Type inferred from value
let static name: type = value;        // Explicit type annotation
```

**Basic Examples:**

```c
let x = 10;                 // Declare and initialize
let message_length = 256;   // Descriptive names are good
let status: i64 = 1;       // Explicit type annotation

let ptr: ptr = malloc(64);  // Allocate memory
let flag: bool = 1;         // Boolean variable
```

**Scope Rules:**

Variables are scoped to the block they're declared in:

```c
fn example() {
    let x = 10;
    if (x > 5) {
        let y = 20;        // y is local to this if block
        y = 30;            // Valid
    }
    // y is not accessible here
    y = 40;                // ERROR: y is out of scope
}
```

### Variable Assignment (Reassignment)

Reassign a variable to a new value:

```c
let x = 10;
x = 20;                    // Reassign to new value
x = x + 5;                 // Use current value in new assignment
x = x * 2;
```

**Multiple Operations:**

```c
let counter = 0;
counter = counter + 1;
counter = counter + 1;
counter = counter + 1;     // counter now equals 3
```

**Assignment vs. Declaration:**

```c
let x = 5;    // Declaration (must use 'let')
x = 10;       // Assignment (no 'let' keyword)
x = 15;       // Assignment again
```

### Field Assignment (for Structs)

Assign values to struct fields:

```c
let p: ptr = alloc_struct(Point);
p.x = 10;                      // Assign to x field
p.y = 20;                      // Assign to y field

// Read-modify-write pattern
let current = p.x;
p.x = current + 5;

// Direct chained assignment (if supported)
let outer = alloc_struct(Container);
// outer.inner.value = 42;    // Depends on nesting support
```

### Array/Index Assignment

Assign to array or pointer-indexed elements:

```c
let arr: ptr = malloc(100);
arr[0] = 100;                  // Assign to first element
arr[1] = 200;                  // Assign to second element
arr[10] = 999;                 // Assign to arbitrary index

// Indexed assignment with expressions
let index = 5;
arr[index] = 50;
arr[index + 1] = 60;
```

**Offset Calculation:**

```c
let p: ptr = malloc(64);
p[0] = 10;                    // First i64 (byte offset 0)
p[1] = 20;                    // Second i64 (byte offset 8)
p[2] = 30;                    // Third i64 (byte offset 16)
```

---

## Operators

### Arithmetic Operators

Perform mathematical operations on numeric values:

| Operator | Name | Example | Result |
|----------|------|---------|--------|
| `+` | Addition | `10 + 5` | `15` |
| `-` | Subtraction | `10 - 5` | `5` |
| `*` | Multiplication | `10 * 5` | `50` |
| `/` | Division | `10 / 5` | `2` |

**Examples:**

```c
let a = 10;
let b = 3;

let sum = a + b;           // 13
let diff = a - b;          // 7
let product = a * b;       // 30
let quotient = a / b;      // 3 (integer division)
let remainder_calc = a - (b * (a / b));  // Workaround for modulo
```

**Operator Chaining:**

```c
let result = 2 + 3 * 4;    // 14 (multiplication first)
let result2 = (2 + 3) * 4; // 20 (parentheses override)
```

### Comparison Operators

Compare two values and return a boolean result:

| Operator | Name | Example | Result |
|----------|------|---------|--------|
| `==` | Equality | `5 == 5` | `1` (true) |
| `!=` | Inequality | `5 != 3` | `1` (true) |
| `<` | Less than | `3 < 5` | `1` (true) |
| `>` | Greater than | `5 > 3` | `1` (true) |
| `<=` | Less than or equal | `3 <= 3` | `1` (true) |
| `>=` | Greater than or equal | `5 >= 3` | `1` (true) |

**Examples:**

```c
let x = 10;
let y = 5;

if (x == y) { }           // false (0)
if (x != y) { }           // true (1)
if (x < y) { }            // false
if (x > y) { }            // true
if (x <= 10) { }          // true
if (x >= 10) { }          // true
```

**Chained Comparisons:**

```c
// Note: Micro-C doesn't support chained comparisons like C
// You must write: (x > 0) and (x < 10)
if (x > 0) {
    if (x < 10) {
        // x is between 0 and 10
    }
}
```

### Assignment Operator

Assign a value to a variable:

```c
x = 10;                    // Simple assignment
x = y + 5;                 // Expression on right side
```

### Operator Precedence

Operators are evaluated in the following order (highest to lowest):

1. **Multiplication (`*`) and Division (`/`)**
   - Evaluated left to right
   - Higher precedence than addition/subtraction

2. **Addition (`+`) and Subtraction (`-`)**
   - Evaluated left to right
   - Higher precedence than comparisons

3. **Comparisons (`<`, `>`, `<=`, `>=`, `==`, `!=`)**
   - Evaluated left to right
   - Results are used in conditionals

4. **Assignment (`=`)**
   - Lowest precedence
   - Right-associative (groups right to left)

**Precedence Examples:**

```c
let a = 2 + 3 * 4;        // 14 (not 20) - mult before add
let b = (2 + 3) * 4;      // 20 - parens override
let c = 10 - 5 - 2;       // 3 (left to right: (10-5)-2)
let d = 10 > 5 + 2;       // true (5+2=7, 10>7)
let e = 2 * 3 + 4 * 5;    // 26 ((2*3)+(4*5))
```

---

## Functions

### Basic Function Definition

Define a function with a name, parameters, and a body:

```c
fn add(a, b) {
    return a + b;
}
```

**Breakdown:**
- `fn` - keyword to declare a function
- `add` - function name
- `(a, b)` - parameter list (untyped)
- `{ }` - function body

### Function Parameters

Parameters are untyped and can receive any value:

```c
// Single parameter
fn square(x) {
    return x * x;
}

// Multiple parameters
fn multiply(a, b) {
    return a * b;
}

// No parameters
fn get_constant() {
    return 42;
}

// Many parameters
fn complex_calc(a, b, c, d, e) {
    return (a + b) * (c - d) / e;
}
```

### Return Statements

Return from a function with a value:

```c
fn example(x) {
    return x + 1;          // Return expression
}

fn early_exit(n) {
    if (n == 0) {
        return 1;          // Early return
    }
    return n * 2;
}
```

**Valid Return Expressions:**
- Literals: `return 42;`
- Variables: `return x;`
- Function calls: `return add(a, b);`
- Arithmetic: `return x + y * z;`
- Comparisons: `return x > 10;`
- Complex: `return factorial(n) + fib(n);`

### Exported Functions (Entry Points)

Export a function to make it visible to the linker:

```c
export fn main() {
    return 0;
}
```

**Key Points:**
- Exported functions are visible to the linker
- Usually used for `main` or other entry points
- Non-exported functions are internal to the compilation unit

### Function Calls

Call a function with arguments:

```c
let result = add(10, 5);
let nested = add(mul(2, 3), sub(10, 5));
let single = square(7);
```

**Function Call Evaluation:**
1. Arguments are evaluated left to right
2. Function is called with evaluated arguments
3. Result is returned and used in the expression

### External Functions (Linker-Provided)

Declare functions provided by the linker or external libraries:

```c
extern fn malloc(size);
extern fn free(ptr);
extern fn memcpy(dest, src, count);
extern fn host_add(a, b);
```

**Usage Example:**

```c
extern fn malloc(size);
extern fn syscall(num, ...);

export fn main() {
    let buffer = malloc(1024);
    return syscall(60);  // exit syscall
}
```

### Function Features

**Recursion:**

```c
fn factorial(n) {
    if (n == 0) {
        return 1;
    }
    return n * factorial(n - 1);
}

fn fibonacci(n) {
    if (n <= 1) {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}
```

**Nested Calls:**

```c
fn outer(a, b) {
    return inner(a) + inner(b);
}

fn inner(x) {
    return x * 2;
}

export fn main() {
    let result = outer(3, 4);  // 3*2 + 4*2 = 14
    return result;
}
```

**Tail Recursion:**

```c
// Tail recursive (may be optimized)
fn sum_tail(n, acc) {
    if (n == 0) {
        return acc;
    }
    return sum_tail(n - 1, acc + n);
}
```

Note: Tail call optimization is not guaranteed by Micro-C.

---

## Control Flow

### If / Elif / Else Statements

Make decisions based on conditions:

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

**Structure:**
- `if` - condition and body (required)
- `elif` - additional conditions (optional, multiple allowed)
- `else` - default case (optional)

**Simple If:**

```c
if (x > 10) {
    let result = 1;
}
```

**If-Else:**

```c
if (flag == 1) {
    result = "enabled";
} else {
    result = "disabled";
}
```

**Multiple Elif Chains:**

```c
if (status == 0) {
    handle_idle();
} elif (status == 1) {
    handle_running();
} elif (status == 2) {
    handle_paused();
} elif (status == 3) {
    handle_error();
} else {
    handle_unknown();
}
```

**Nested Conditions:**

```c
if (x > 0) {
    if (y > 0) {
        result = "positive quadrant";
    } else {
        result = "negative y";
    }
} else {
    result = "non-positive x";
}
```

### Loop Statement (Unbounded Loop)

Repeat code indefinitely (until break/return):

```c
loop {
    if (i == 10) {
        return i;
    }
    i = i + 1;
}
```

**Key Points:**
- `loop` keyword starts an infinite loop
- Must be exited with `break` or `return`
- Condition is checked inside the loop body

**Loop with Break:**

```c
let i = 0;
loop {
    if (i == 100) {
        break;             // Exit loop
    }
    i = i + 1;
}
// Continue here after break
```

**Loop with Return:**

```c
let sum = 0;
let i = 0;
loop {
    sum = sum + i;
    i = i + 1;
    if (i > 100) {
        return sum;        // Exit loop and function
    }
}
```

**Nested Loops:**

```c
let i = 0;
loop {
    if (i == 10) {
        break;
    }
    let j = 0;
    loop {
        if (j == 10) {
            break;
        }
        let result = i * j;
        j = j + 1;
    }
    i = i + 1;
}
```

### Break Statement

Exit a loop:

```c
loop {
    if (should_exit) {
        break;             // Exit loop
    }
    // ... loop body
}
```

**Search Loop:**

```c
let i = 0;
let found = 0;
loop {
    if (arr[i] == target) {
        found = 1;
        break;            // Found, exit
    }
    i = i + 1;
    if (i >= 100) {
        break;            // Limit reached, exit
    }
}
```

### Continue Statement

Skip to the next iteration:

```c
loop {
    if (should_skip) {
        continue;          // Skip to next iteration
    }
    // ... process item
}
```

**Example - Sum Non-Zero Values:**

```c
let sum = 0;
let i = 0;
loop {
    if (i >= 100) {
        break;
    }
    if (arr[i] == 0) {
        i = i + 1;
        continue;         // Skip zero values
    }
    sum = sum + arr[i];
    i = i + 1;
}
```

---

## Structs

### Struct Definition

Define a new structure type with named fields:

```c
struct Point {
    x: i64;
    y: i64;
}

struct Complex {
    real: i64;
    imag: i64;
}

struct Color {
    r: i64;
    g: i64;
    b: i64;
}
```

**Key Points:**
- Struct names should be capitalized by convention
- Fields must have explicit type annotations
- Currently supported field type is `i64`

### Struct Allocation

Allocate memory for a struct using an external function:

```c
extern fn alloc_struct(type);

let p: ptr = alloc_struct(Point);
let c: ptr = alloc_struct(Color);
```

**Note:** You need to provide or link against `alloc_struct` implementation.

### Field Access and Assignment

Read from and write to struct fields:

```c
let p: ptr = alloc_struct(Point);
p.x = 10;                      // Write field
p.y = 20;

let value = p.x;              // Read field
p.y = value + 5;
```

**Field Operations:**

```c
// Read-modify-write
let current_x = p.x;
p.x = current_x + 10;

// Using fields in expressions
let distance_squared = p.x * p.x + p.y * p.y;

// Field-to-field operations
p.x = p.y;
```

### Chained Field Access

Access fields of nested structures:

```c
struct Point {
    x: i64;
    y: i64;
}

struct Line {
    start: ptr;    // Points to a Point struct
    end: ptr;      // Points to a Point struct
}

let line: ptr = alloc_struct(Line);
let start_point = line.start;
start_point.x = 0;
start_point.y = 0;

// Chained access (if supported)
// line.start.x = 0;
```

### Struct Usage Example

```c
struct Rectangle {
    width: i64;
    height: i64;
}

fn area(rect) {
    return rect.width * rect.height;
}

export fn main() {
    let rect: ptr = alloc_struct(Rectangle);
    rect.width = 10;
    rect.height = 20;
    
    let a = area(rect);  // 200
    return a;
}
```

---

## Memory Operations

### Peek (Read from Memory)

Read an i64 value from a memory address:

```c
let value = peek(address);    // Read i64 from address
```

**How It Works:**
- Takes a memory address (usually a `ptr`)
- Returns the i64 value stored at that address
- The address must be valid and readable

**Examples:**

```c
// Read from allocated buffer
let ptr = malloc(64);
poke(ptr, 42);
let value = peek(ptr);       // value = 42

// Read from specific address
let bootloader_magic = peek(0x1000);

// Read sequence of values
let first = peek(base);
let second = peek(base + 8);  // +8 for next i64
```

### Poke (Write to Memory)

Write an i64 value to a memory address:

```c
poke(address, value);         // Write i64 to address
```

**How It Works:**
- Takes a memory address and a value
- Writes the value (as i64) to that address
- The address must be valid and writable

**Examples:**

```c
let ptr = malloc(8);
poke(ptr, 42);                // Write 42 to ptr
let result = peek(ptr);       // Read back: result = 42

// Multiple writes
poke(ptr, 10);
poke(ptr + 8, 20);
poke(ptr + 16, 30);

// Pattern: Initialize buffer
let buffer = malloc(100);
let i = 0;
loop {
    if (i >= 10) {
        break;
    }
    poke(buffer + i * 8, i);  // Write index to each slot
    i = i + 1;
}
```

### Peek-Poke Pattern

Combine peek and poke for read-modify-write operations:

```c
let current = peek(address);
let new_value = current + 1;
poke(address, new_value);

// Or more concisely:
poke(address, peek(address) + 1);
```

---

## Array Indexing

### Pointer Indexing

Access array elements using index notation:

```c
let arr: ptr = malloc(100);
let first = arr[0];           // Read element
arr[10] = 99;                 // Write element
```

**Mechanism:**
- Index `[n]` accesses memory at offset `n * sizeof(i64)` bytes
- For i64 values, `arr[0]` is at `base + 0`, `arr[1]` is at `base + 8`, etc.

### Index Calculation

```c
let p: ptr = malloc(80);      // 10 i64s
p[0] = 10;                    // byte offset: 0
p[1] = 20;                    // byte offset: 8
p[2] = 30;                    // byte offset: 16
p[5] = 50;                    // byte offset: 40
```

### Dynamic Indexing

Use variables and expressions for indices:

```c
let arr: ptr = malloc(100);
let index = 5;
arr[index] = 42;              // Write to index 5
let value = arr[index + 1];   // Read from index 6
```

### Loop-Based Array Access

```c
let buffer: ptr = malloc(1000);
let i = 0;
loop {
    if (i >= 100) {
        break;
    }
    buffer[i] = i * i;        // Write square of i
    i = i + 1;
}

// Read back
let i2 = 0;
let sum = 0;
loop {
    if (i2 >= 100) {
        break;
    }
    sum = sum + buffer[i2];
    i2 = i2 + 1;
}
```

---

## Imports

### Include Directives

Include built-in system declarations:

```c
#include <Sys>                // Include built-in system declarations
```

**What <Sys> Provides:**
- Memory functions: `malloc`, `free`, `alloc_struct`
- I/O functions (architecture-dependent)
- System calls (architecture-dependent)

**Example Usage:**

```c
#include <Sys>

export fn main() {
    let buffer = malloc(1024);   // Available after include
    let status = 0;
    return status;
}
```

**Future Extensibility:**

The include system is designed to be extensible:
- `<Sys>` is a built-in standard library
- Additional modules may be supported in future versions
- Custom includes may be added for different targets

---

## Return Statement

### Basic Return

Return a value from a function:

```c
fn example() {
    return 42;
}
```

### Early Return

Exit a function before the end of the block:

```c
fn example(x) {
    if (x == 0) {
        return 0;             // Early return
    }
    
    let result = x + 1;
    return result;            // Normal return
}
```

### Return Expressions

Return various types of expressions:

```c
// Literals
fn get_max() {
    return 1000;
}

// Variables
fn return_param(x) {
    return x;
}

// Function calls
fn return_sum(a, b) {
    return add(a, b);
}

// Arithmetic expressions
fn double_value(x) {
    return x * 2;
}

// Comparisons
fn is_valid(x) {
    return x > 0;
}

// Complex expressions
fn complex_calc(a, b, c) {
    return (a + b) * c - 10;
}
```

### Return in Loops

```c
fn find_match(arr, target) {
    let i = 0;
    loop {
        if (arr[i] == target) {
            return i;          // Return index when found
        }
        i = i + 1;
        if (i >= 100) {
            return -1;         // Return -1 if not found
        }
    }
}
```

---

## Expressions

### Expression Types

Micro-C supports the following expression forms:

### Literals

```c
42                    // Integer literal
0xFF                  // Hexadecimal literal
0b1010                // Binary literal
```

### Variables

```c
x
my_variable
count
ptr
```

### Binary Operations

```c
a + b                 // Addition
x * y                 // Multiplication
p < 100               // Comparison
a == b                // Equality
x - y + z             // Chained operations
```

### Function Calls

```c
add(1, 2)
malloc(64)
nested_call(f(x), g(y))
recursive_func(n - 1)
```

### Memory Operations

```c
peek(ptr)             // Read from address
```

### Indexing

```c
arr[i]                // Array indexing
ptr[10]               // Pointer indexing
buffer[index + 1]     // Computed index
```

### Field Access

```c
struct_ptr.field
point.x
color.r
```

### Parenthesized Expressions

```c
(a + b) * c           // Override precedence
(x > 10)              // Clarify comparison
(a + b) / (c - d)     // Complex expression
```

### Expression Evaluation

Expressions are evaluated following these rules:
1. Parentheses first (highest precedence)
2. Multiplication and Division
3. Addition and Subtraction
4. Comparisons
5. No logical operators (AND, OR) in expressions
6. No ternary operator (`? :`)

---

## Statements

Micro-C programs consist of statements. Each statement performs an action:

### Variable Declaration

```c
let x = 10;           // Declare and initialize variable
let result: i64 = 0;  // With explicit type
```

### Assignment

```c
x = 20;               // Assign new value
arr[i] = 100;         // Assign to array element
p.field = 50;         // Assign to struct field
```

### Conditional (If-Elif-Else)

```c
if (condition) {
    // statements
} elif (other_condition) {
    // statements
} else {
    // statements
}
```

### Loop

```c
loop {
    // statements
    if (exit_condition) {
        break;
    }
}
```

### Loop Control

```c
break;                // Exit loop
continue;             // Next iteration
```

### Return

```c
return value;         // Return from function
return x + y;         // Return expression
```

### Memory Write (Poke)

```c
poke(ptr, val);       // Write to memory
```

### Expression Statement

```c
fn_call();            // Call function (for side effects)
add(1, 2);            // Evaluate expression
```

### Block Statements

Groups of statements in `{ }`:

```c
{
    let x = 10;
    x = x + 1;
    return x;
}
```

---

## Monostatements

Monostatements are on-the-go statements and locally defined functions.

### where

`where` is used similar to lambda, to locally create and evaluate a function.

```c
let x = 10;
let y = add(x) where add(a) = { a + 10 };
```

The declared function can also be used multiple times per line.
```c

let x = 10;
let b = 3;
let y = add( b * add(x) ) where add(a) = { a + 10 };
```

`where` can also be used to define multiple single-arg functions per line. It can acess local variables upon closure of the function, but they must be static.
```c

let x = 10;
let static c = 3;
let y = add(mul(x)) where add(a) = { a + 10 } and mul(b) = { b * c };
```


## Complete Examples

### Example 1: Simple Arithmetic

```c
fn add(a, b) {
    return a + b;
}

fn multiply(a, b) {
    return a * b;
}

export fn main() {
    let x = add(10, 5);           // 15
    let y = multiply(x, 2);       // 30
    return y;
}
```

### Example 2: Factorials and Recursion

```c
fn factorial(n) {
    if (n == 0) {
        return 1;
    }
    return n * factorial(n - 1);
}

export fn main() {
    let result = factorial(5);    // 120
    return result;
}
```

### Example 3: Arrays and Loops

```c
#include <Sys>

export fn main() {
    let arr: ptr = malloc(1000);
    
    // Initialize array with squares
    let i = 0;
    loop {
        if (i >= 100) {
            break;
        }
        arr[i] = i * i;
        i = i + 1;
    }
    
    // Sum all elements
    let sum = 0;
    let j = 0;
    loop {
        if (j >= 100) {
            break;
        }
        sum = sum + arr[j];
        j = j + 1;
    }
    
    return sum;
}
```

### Example 4: Structs and Memory

```c
#include <Sys>

struct Point {
    x: i64;
    y: i64;
}

fn distance_squared(p) {
    return p.x * p.x + p.y * p.y;
}

export fn main() {
    let p1: ptr = alloc_struct(Point);
    p1.x = 3;
    p1.y = 4;
    
    let p2: ptr = alloc_struct(Point);
    p2.x = 0;
    p2.y = 0;
    
    let d1 = distance_squared(p1);  // 25
    let d2 = distance_squared(p2);  // 0
    
    return d1 + d2;
}
```

### Example 5: Conditionals and Control Flow

```c
fn grade_letter(score) {
    if (score >= 90) {
        return 65;              // 'A'
    } elif (score >= 80) {
        return 66;              // 'B'
    } elif (score >= 70) {
        return 67;              // 'C'
    } elif (score >= 60) {
        return 68;              // 'D'
    } else {
        return 70;              // 'F'
    }
}

export fn main() {
    let score1 = grade_letter(95);   // 65 ('A')
    let score2 = grade_letter(75);   // 67 ('C')
    let score3 = grade_letter(55);   // 70 ('F')
    return score1 + score2 + score3;
}
```

### Example 6: Complex Program

```c
#include <Sys>

struct Node {
    value: i64;
    next: ptr;
}

fn sum_list(head) {
    let sum = 0;
    let current = head;
    loop {
        if (current == 0) {
            break;
        }
        sum = sum + current.value;
        current = current.next;
    }
    return sum;
}

fn create_node(value, next_node) {
    let node: ptr = alloc_struct(Node);
    node.value = value;
    node.next = next_node;
    return node;
}

export fn main() {
    // Create a simple linked list: 1 -> 2 -> 3
    let node3 = create_node(3, 0);
    let node2 = create_node(2, node3);
    let node1 = create_node(1, node2);
    
    let total = sum_list(node1);   // 6
    return total;
}
```

---

## Summary

This syntax reference covers all major features of Micro-C. For more information, see the main [README.md](readme.md) and examine the compiler source code in the `src/` directory.
