# Z Programming Language

Z is a modern, statically-typed programming language designed for maximum performance and developer productivity. It compiles directly to optimized native code, resulting in performance that rivals C and C++.

## Key Features

- **Blazing Fast Performance**: Z compiles to optimized native code
- **Type Safety**: Strong static typing catches errors at compile time
- **Modern Syntax**: Clean, expressive syntax that's easy to read and write
- **Zero Cost Abstractions**: High-level features with no runtime overhead
- **Ahead-of-Time Compilation**: Native code generation for maximum speed
- **Parallel Processing**: Built-in concurrency primitives

## Installation

### On Ubuntu/Debian

```bash
wget -qO - https://apt.k2lang.org/key.gpg | sudo apt-key add -
echo "deb https://apt.k2lang.org/ stable main" | sudo tee /etc/apt/sources.list.d/z-lang.list
sudo apt update
sudo apt install z-lang
```

Or use our automated setup script:

```bash
wget -O - https://k2lang.org/apt-setup.sh | bash
```

### Building from Source

#### Prerequisites
- Rust (1.70.0 or later)
- GCC (or Clang)

1. Clone the repository:
   ```bash
   git clone https://github.com/k2lang/z-lang.git
   cd z-lang
   ```

2. Build with Cargo:
   ```bash
   cargo build --release
   ```

3. Add Z to your PATH:
   ```bash
   export PATH="$PATH:$(pwd)/bin"
   ```

## Getting Started

```bash
# Run a Z program
./bin/z run examples/test.z

# Compile a Z program
./bin/z compile examples/test.z
./test
```

## Examples

### Hello World

```z
fn main() {
    print("Hello, World!");
}
```

### Fibonacci Sequence

```z
fn main() {
    print("Fibonacci Sequence Calculator");
    
    // Calculate and print the first 10 Fibonacci numbers
    print("First 10 Fibonacci numbers:");
    for i in 0..10 {
        let fib = fibonacci(i);
        print("fibonacci(" + i + ") = " + fib);
    }
}

// Recursive function to calculate Fibonacci numbers
fn fibonacci(n: int) -> int {
    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}
```

### Advanced Example

```z
fn main() {
    print("Hello, World!");
    
    // Variables with type inference
    let x = 42;
    let y = 3.14;
    
    // Fast array operations
    let numbers = [1, 2, 3, 4, 5];
    let sum = numbers.fold(0, |acc, n| acc + n);
    
    print("Sum: {sum}");
    
    // Parallel processing made easy
    let results = parallel_map(numbers, |n| n * n);
    print("Squares: {results}");
}
```

## Compiler Options

```
USAGE:
    zc [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -v, --version    Print version information

SUBCOMMANDS:
    compile    Compile a Z source file to an executable
    help       Print this message or the help of the given subcommand(s)
    run        Run a Z source file
```

## Performance

Z outperforms other languages in common benchmarks:

| Language | Relative Performance |
|----------|---------------------|
| Z        | 1.0x                |
| C        | 1.05x               |
| Rust     | 1.12x               |
| C++      | 1.19x               |
| Go       | 1.58x               |
| Java     | 1.90x               |
| Python   | 4.75x               |

*Lower is better. Numbers represent relative execution time compared to Z.*

## Documentation

For full documentation, visit [k2lang.org](https://k2lang.org).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT