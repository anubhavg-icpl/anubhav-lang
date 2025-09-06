.PHONY: build run test clean install release docs

# Build the project
build:
	cargo build

# Build in release mode
release:
	cargo build --release

# Run with example file
run:
	cargo run examples/basic/hello.anubhav

# Run tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean
	rm -f examples/**/*.out

# Install locally
install: release
	cargo install --path .

# Format code
fmt:
	cargo fmt

# Check code
check:
	cargo check
	cargo clippy

# Build documentation
docs:
	cargo doc --no-deps --open

# Run all checks before commit
pre-commit: fmt check test
	@echo "All checks passed!"