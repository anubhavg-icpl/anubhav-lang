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
- `CALCULATE name expression` - Perform arithmetic calculations

## Run Examples

```bash
./target/release/anubhav-lang examples/hello.anubhav
./target/release/anubhav-lang examples/multiple.anubhav
```