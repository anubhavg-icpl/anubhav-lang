# Anubhav Language

A unique intention-based programming language where you declare what you intend to do, then manifest it.

## Installation

```bash
./build.sh
```

## Usage

```bash
./target/release/anubhav-lang <file.anubhav>
```

## Example

```anubhav
INTENT greeting "Hello, World!"
MANIFEST greeting

CALCULATE sum 10 + 20
MANIFEST sum
```

## Syntax

- `INTENT name "message"` - Declare an intention
- `MANIFEST name` - Execute the intention
- `MANIFEST name WITH "context"` - Execute with additional context
- `CALCULATE name expression` - Perform arithmetic calculations
- `STORE name value` - Store a variable
- `RECALL name` - Use a stored variable in expressions
- `COMBINE name strings...` - Concatenate strings and variables
- `REPEAT n TIMES DO...END` - Loop execution
- `IF condition THEN...ELSE...END` - Conditional execution
- `PRINT items...` - Formatted output
- `AND/OR` - Logical operators
- `#` - Comments (to end of line)

## Features

- Intention-based programming paradigm
- Arithmetic operations (+, -, *, /)
- Comparison operators (==, !=, <, >, <=, >=)
- Logical operators (AND, OR)
- Variables and calculations
- String concatenation
- Comments (#)
- Loops (REPEAT)
- Conditionals (IF/THEN/ELSE)
- Formatted output (PRINT)

## Run Examples

```bash
./target/release/anubhav-lang examples/hello.anubhav
./target/release/anubhav-lang examples/calculate.anubhav
./target/release/anubhav-lang examples/variables.anubhav
./target/release/anubhav-lang examples/combine.anubhav
```