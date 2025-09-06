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
```

## Syntax

- `INTENT name "message"` - Declare an intention
- `MANIFEST name` - Execute the intention

## Run Examples

```bash
./target/release/anubhav-lang examples/hello.anubhav
./target/release/anubhav-lang examples/multiple.anubhav
```