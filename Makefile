# Makefile for linting, formatting, and testing Rust and Dart files

include rust.makefile

# Variables
DART_PROJECT := web_client

.PHONY: all lint lint-dart format format-dart test test-dart dart-setup

# Default target
all: dart-setup lint format test

# Lint both Rust and Dart projects
lint: lint-rust lint-dart

# Format both Rust and Dart projects
format: format-rust format-dart

# Run tests for both Rust and Dart projects
test: test-rust test-dart

# Dart setup (if any)
dart-setup:
	@echo "Setting up Dart environment..."
	@cd $(DART_PROJECT) && flutter pub get
	@echo "Dart setup is complete."

# Lint Dart project using dart analyze
lint-dart:
	@echo "Running Dart analyzer..."
	@cd $(DART_PROJECT) && dart analyze
	@echo "Dart linting completed successfully."

# Format Dart project using dart format
format-dart:
	@echo "Formatting Dart code..."
	@cd $(DART_PROJECT) && dart format .
	@echo "Dart formatting completed successfully."

# Run tests for Dart project
test-dart:
	@echo "Running Dart tests..."
	@cd $(DART_PROJECT) && flutter test || exit 1;
	@echo "Dart tests completed successfully."
