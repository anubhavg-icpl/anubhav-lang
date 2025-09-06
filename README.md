<div align="center">

[![MIT License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange.svg)](https://www.rust-lang.org/)
[![Version](https://img.shields.io/badge/Version-0.1.0-blue.svg)](https://github.com/anubhavg-icpl/anubhav-lang)

A feature-rich, intention-based educational programming language with 100+ operations

</div>

## Overview

Anubhav is a unique intention-based programming language where you declare what you intend to do, then manifest it. Built with Rust for performance and reliability, it provides an intuitive syntax perfect for educational purposes while maintaining the power needed for complex operations.

### Key Features
- **Intention-based paradigm** - Declare intentions, then manifest them
- **100+ built-in operations** - Comprehensive standard library
- **Full recursion support** - With proper call stack management
- **Module system** - Import/export capabilities
- **Error handling** - Try/catch blocks for robust programs
- **String interpolation** - Built-in template syntax
- **Advanced data structures** - Arrays, dictionaries, functions

## Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/anubhavg-icpl/anubhav-lang.git
cd anubhav-lang

# Build using the provided script
./scripts/build.sh

# Or build manually with Cargo
cargo build --release
```

### Running Your First Program

```bash
# Run a program
./target/release/anubhav examples/basic/hello.anubhav

# Or using cargo run
cargo run -- examples/basic/hello.anubhav
```

### Hello World Example

```anubhav
# Traditional intention-based approach
INTENT greeting "Hello, World!"
MANIFEST greeting

# Direct print approach
PRINT "Hello, World!"
```

## Language Guide

### Core Concepts

#### 1. Intention-Based Programming
```anubhav
# Declare an intention
INTENT message "Welcome to Anubhav!"

# Manifest it later
MANIFEST message

# Manifest with context
MANIFEST message WITH " - Let's code!"
```

#### 2. Variables and Storage
```anubhav
# Store values
STORE name "Alice"
STORE age 25

# Recall and use them
PRINT "Name: " RECALL name
CALCULATE next_age RECALL age + 1
```

#### 3. Mathematical Operations
```anubhav
# Basic arithmetic
CALCULATE sum 10 + 20
CALCULATE product 5 * 6
CALCULATE power 2 ** 8

# Complex expressions
CALCULATE result (10 + 5) * 2 / 3

# Mathematical functions
CALCULATE minimum MIN(10, 5)
CALCULATE maximum MAX(10, 5)
CALCULATE rounded ROUND(3.14159)
```

### Control Flow

#### Conditionals
```anubhav
STORE score 85

IF RECALL score >= 90 THEN
    PRINT "Grade: A"
ELSE IF RECALL score >= 80 THEN
    PRINT "Grade: B"
ELSE
    PRINT "Grade: C"
END
```

#### Loops
```anubhav
# For loop with step
FOR i FROM 1 TO 10 STEP 2 DO
    PRINT "Odd number: " RECALL i
END

# While loop
STORE counter 0
WHILE RECALL counter < 5 DO
    PRINT "Count: " RECALL counter
    INCREMENT counter
END

# Repeat loop
REPEAT 3 TIMES DO
    PRINT "Hello!"
END
```

#### Switch Statements
```anubhav
STORE day "Monday"

SWITCH RECALL day DO
    CASE "Monday":
        PRINT "Start of the week"
    CASE "Friday":
        PRINT "TGIF!"
    DEFAULT:
        PRINT "Regular day"
END
```

### Data Structures

#### Arrays
```anubhav
# Create and manipulate arrays
ARRAY numbers
PUSH numbers 1
PUSH numbers 2
PUSH numbers 3

# Array operations
SIZE numbers count
PRINT "Array size: " RECALL count

GET numbers 0 first
PRINT "First element: " RECALL first

# Advanced array operations
MAP numbers (x * 2) doubled
FILTER numbers (x > 1) filtered
SUM numbers total
AVERAGE numbers avg
```

#### Dictionaries
```anubhav
# Create dictionary
DICT person

# Add key-value pairs
PUT person "name" "Bob"
PUT person "age" 30
PUT person "city" "New York"

# Retrieve values
FETCH person "name" name
PRINT "Name: " RECALL name

# Get all keys/values
KEYS person all_keys
VALUES person all_values
```

### Functions

#### Function Definition
```anubhav
# Define a function
FUNCTION greet(name) DO
    COMBINE greeting "Hello, " RECALL name "!"
    PRINT RECALL greeting
    RETURN RECALL greeting
END

# Call the function
CALL greet("Alice") result
```

#### Recursive Functions
```anubhav
# Fibonacci with recursion
FUNCTION fibonacci(n) DO
    IF RECALL n <= 1 THEN
        RETURN RECALL n
    END
    CALCULATE n1 RECALL n - 1
    CALCULATE n2 RECALL n - 2
    CALL fibonacci(RECALL n1) fib1
    CALL fibonacci(RECALL n2) fib2
    CALCULATE result RECALL fib1 + RECALL fib2
    RETURN RECALL result
END

CALL fibonacci(10) fib_result
PRINT "Fibonacci(10) = " RECALL fib_result
```

### String Operations

```anubhav
STORE text "  Hello World  "

# String manipulation
TRIM RECALL text trimmed
UPPERCASE RECALL trimmed upper
LOWERCASE RECALL upper lower
LENGTH RECALL text len

# String interpolation
STORE user "Alice"
STORE age 25
PRINT "User ${user} is ${age} years old"

# String methods
SPLIT "apple,banana,orange" "," fruits
REPLACE "hello world" "world" "universe" new_text
SUBSTRING "hello world" 0 5 substr
```

### Error Handling

```anubhav
TRY DO
    # Potentially dangerous operation
    CALCULATE result 10 / 0
CATCH error DO
    PRINT "Error caught: " RECALL error
    # Handle the error
    STORE result 0
END

# Assertions
ASSERT RECALL result >= 0 "Result must be non-negative"
```

### File I/O

```anubhav
# Read from file
READ_FILE "input.txt" content
PRINT "File content: " RECALL content

# Write to file
STORE data "Hello, File!"
WRITE_FILE "output.txt" RECALL data

# Append to file
APPEND_FILE "log.txt" "New log entry\n"

# Check if file exists
EXISTS "config.txt" file_exists
IF RECALL file_exists THEN
    PRINT "Config file found"
END
```

## Project Structure

```
anubhav-lang/
├── src/
│   ├── main.rs           # Entry point
│   ├── lib.rs            # Library exports
│   ├── core/
│   │   ├── interpreter.rs # Core interpreter logic
│   │   ├── extensions.rs  # Extended operations
│   │   └── mod.rs
│   ├── lang/
│   │   ├── lexer.rs      # Tokenization
│   │   ├── parser.rs     # AST generation
│   │   └── mod.rs
│   └── cli/
│       ├── main.rs       # CLI interface
│       └── mod.rs
├── examples/
│   ├── basic/            # Simple examples
│   ├── advanced/         # Complex features
│   └── showcase/         # Full demonstrations
├── tests/
│   ├── unit/            # Unit tests
│   └── integration/     # Integration tests
├── docs/
│   └── FEATURES.md     # Complete feature list
└── benchmarks/          # Performance tests
```

## Complete Feature List

### Categories
- **Core Operations**: 15+ basic language features
- **Data Types**: Numbers, Strings, Arrays, Dictionaries, Functions
- **Control Flow**: 10+ flow control structures
- **Array Operations**: 20+ array manipulation functions
- **String Operations**: 15+ string processing functions
- **Mathematical Functions**: 15+ math operations
- **I/O Operations**: File and console I/O
- **Error Handling**: Try/catch blocks and assertions
- **Module System**: Import/export capabilities

For a complete list of all 100+ operations, see [docs/FEATURES.md](docs/FEATURES.md).

## Examples

### Running Examples

```bash
# Basic examples
./target/release/anubhav examples/basic/hello.anubhav
./target/release/anubhav examples/basic/math.anubhav
./target/release/anubhav examples/basic/variables.anubhav

# Advanced examples
./target/release/anubhav examples/advanced/functions_test.anubhav
./target/release/anubhav examples/advanced/arrays.anubhav
./target/release/anubhav examples/advanced/module_test.anubhav

# Complete showcase
./target/release/anubhav examples/showcase/showcase_complete.anubhav
```

### Example Programs

The `examples/` directory contains:
- **basic/**: Simple programs demonstrating core features
- **advanced/**: Complex programs showing advanced capabilities
- **showcase/**: Comprehensive demonstrations of all features

## Building from Source

### Prerequisites
- Rust 1.70+ (2024 edition)
- Cargo

### Build Commands

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run with example
cargo run -- examples/basic/hello.anubhav

# Build documentation
cargo doc --open
```

### Build Script

The provided build script handles everything:
```bash
./scripts/build.sh
```

## Integration

### As a Library

Add to your `Cargo.toml`:
```toml
[dependencies]
anubhav_lang = { path = "path/to/anubhav-lang" }
```

Use in your Rust code:
```rust
use anubhav_lang::{Interpreter, lexer, parser};

fn main() {
    let mut interpreter = Interpreter::new();
    let source = "PRINT \"Hello from Rust!\"";

    let tokens = lexer::tokenize(source);
    let ast = parser::parse(tokens);
    interpreter.execute(ast);
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

**Anubhav G**
- GitHub: [@anubhavg-icpl](https://github.com/anubhavg-icpl)
- Repository: [anubhav-lang](https://github.com/anubhavg-icpl/anubhav-lang)

## Acknowledgments

- Built with Rust for performance and safety
- Inspired by educational programming languages
- Designed for clarity and expressiveness

---

<div align="center">
Made for the programming education community
</div>
