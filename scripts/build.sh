#!/bin/bash

echo "Building Anubhav Language..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "Build successful!"
    echo "Binary location: ./target/release/anubhav-lang"
    echo ""
    echo "Usage: ./target/release/anubhav-lang <file.anubhav>"
else
    echo "Build failed!"
    exit 1
fi