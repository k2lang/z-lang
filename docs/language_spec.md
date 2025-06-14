# Z Language Specification

Z is a high-performance programming language designed to be extremely fast while maintaining memory safety and ease of use.

## 1. Syntax

### 1.1 Comments

```z
// Single line comment

/* 
   Multi-line
   comment
*/
```

### 1.2 Variables

```z
// Type inference
let x = 42;

// Explicit type
let y: float = 3.14;

// Constants
const PI = 3.14159;
```

### 1.3 Primitive Types

- `int`: 64-bit signed integer
- `float`: 64-bit floating point
- `bool`: Boolean (true/false)
- `string`: UTF-8 string
- `char`: Unicode character
- `void`: No value

### 1.4 Compound Types

```z
// Arrays
let numbers: [int] = [1, 2, 3, 4, 5];

// Tuples
let point: (int, int) = (10, 20);

// Structs
struct Point {
    x: float,
    y: float,
}

// Enums
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### 1.5 Control Flow

```z
// If expression
let max = if x > y { x } else { y };

// Match expression
let value = match option {
    Some(v) => v,
    None => 0,
};

// Loops
for i in 0..10 {
    print(i);
}

while condition {
    // do something
}

loop {
    // infinite loop
    if condition {
        break;
    }
}
```

### 1.6 Functions

```z
// Basic function
fn add(a: int, b: int) -> int {
    return a + b;
}

// Function with type inference for return value
fn multiply(a: int, b: int) {
    a * b  // Last expression is implicitly returned
}

// Generic function
fn identity<T>(x: T) -> T {
    x
}

// Lambda/closure
let add = |a, b| a + b;
```

## 2. Memory Management

Z uses a hybrid memory management approach:

1. **Region-based Memory Management**: Memory is allocated in regions that are automatically freed when they go out of scope.
2. **Ownership System**: Similar to Rust, but with more automatic inference to reduce explicit annotations.
3. **Optional Garbage Collection**: For complex data structures when needed.

```z
// Memory is automatically managed
fn process_data() {
    let data = load_large_data();
    // data is automatically freed when function returns
}

// Explicit memory control when needed
fn optimize_memory() {
    let data = @pinned load_large_data();
    process(data);
    @free data;  // Explicitly free before function ends
}
```

## 3. Concurrency

```z
// Async/await
async fn fetch_data(url: string) -> string {
    // Asynchronous operation
}

// Parallel iteration
let results = parallel.map(items, |item| process(item));

// Thread-safe shared state
let counter = @atomic 0;
parallel.for(0, 1000, |_| {
    counter.fetch_add(1);
});
```

## 4. Performance Features

### 4.1 SIMD Operations

```z
// Automatic SIMD optimization
@simd
fn vector_add(a: [float], b: [float]) -> [float] {
    let result = array.new(a.length);
    for i in 0..a.length {
        result[i] = a[i] + b[i];
    }
    return result;
}
```

### 4.2 Compile-time Execution

```z
// Compile-time function execution
@comptime
fn factorial(n: int) -> int {
    if n <= 1 { 1 } else { n * factorial(n - 1) }
}

const LOOKUP_TABLE = @comptime generate_table(1000);
```

### 4.3 Memory Layout Control

```z
// Struct with specific memory layout
@packed
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

// Aligned data for SIMD
@align(32)
let vector: [8]float;
```

## 5. Interoperability

```z
// Foreign Function Interface
@extern "C"
fn c_function(arg: int) -> int;

// Export Z function to be called from other languages
@export
fn z_function(arg: int) -> int {
    // implementation
}
```

## 6. Metaprogramming

```z
// Compile-time code generation
@macro
fn generate_getters(fields: [string]) {
    for field in fields {
        emit!{
            fn get_$(field)(self: &Self) -> Self.$(field).type {
                self.$(field)
            }
        }
    }
}
```

## 7. Error Handling

```z
// Result type for fallible operations
fn divide(a: int, b: int) -> Result<int, string> {
    if b == 0 {
        return Err("Division by zero");
    }
    return Ok(a / b);
}

// Using results
let result = divide(10, 2)?;  // Propagates error if Err

// Try/catch for exceptional cases
try {
    let file = File.open("data.txt")?;
    // use file
} catch e {
    print("Error: {e}");
}
```

## 8. Modules and Imports

```z
// Importing modules
import std.io;
import std.math.{sin, cos};

// Exporting from modules
export fn public_function() {
    // implementation
}

fn private_function() {
    // not exported
}
```

## 9. Standard Library

The Z standard library includes modules for:

- I/O operations
- Collections (arrays, maps, sets)
- Concurrency primitives
- Networking
- File system access
- Math functions
- Text processing
- Time and date handling
- Cryptography
- Serialization formats (JSON, XML, etc.)

## 10. Tooling

- `zc`: Z compiler
- `zrun`: Run Z programs directly
- `ztest`: Test runner
- `zfmt`: Code formatter
- `zlint`: Linter
- `zdoc`: Documentation generator
- `zpkg`: Package manager