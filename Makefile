# Makefile for linting, formatting, and testing Rust and Dart files

include rust.makefile
include dart.makefile

.PHONY: all lint format test

# Default target
all: dart-setup lint format test

# Lint both Rust and Dart projects
lint: lint-rust lint-dart

# Format both Rust and Dart projects
format: format-rust format-dart

# Run tests for both Rust and Dart projects
test: test-rust test-dart
