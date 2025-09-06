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
- `#` - Comments (to end of line)

## Features

- Intention-based programming paradigm
- Arithmetic operations (+, -, *, /)
- Comparison operators (==, !=, <, >, <=, >=)
- Variables and calculations
- String concatenation
- Comments

## Run Examples

```bash
./target/release/anubhav-lang examples/hello.anubhav
./target/release/anubhav-lang examples/calculate.anubhav
./target/release/anubhav-lang examples/variables.anubhav
./target/release/anubhav-lang examples/combine.anubhav
```