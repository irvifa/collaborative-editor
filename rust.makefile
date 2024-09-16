# rust.makefile - Rust-specific configurations and targets

# Variables
RUST_PROJECTS := server client
CARGO := cargo
RUSTUP := rustup

.PHONY: lint-rust format-rust test-rust

# Lint Rust projects using cargo clippy
lint-rust:
	@echo "Running Rust linter (clippy)..."
	@for dir in $(RUST_PROJECTS); do \
		echo "Linting $$dir..."; \
		$(CARGO) clippy --manifest-path=$$dir/Cargo.toml --all-targets --all-features -- -D warnings || exit 1; \
	done
	@echo "Rust linting completed successfully."

# Format Rust projects using cargo fmt
format-rust:
	@echo "Formatting Rust code..."
	@for dir in $(RUST_PROJECTS); do \
		echo "Formatting $$dir..."; \
		$(CARGO) fmt --manifest-path=$$dir/Cargo.toml; \
	done
	@echo "Rust formatting completed successfully."

# Run tests for Rust projects
test-rust:
	@echo "Running Rust tests..."
	@for dir in $(RUST_PROJECTS); do \
		echo "Testing $$dir..."; \
		$(CARGO) test --manifest-path=$$dir/Cargo.toml || exit 1; \
	done
	@echo "Rust tests completed successfully."
