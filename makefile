# Name of the binary
BINARY_NAME=token_balance_watcher

# Installation directory
INSTALL_DIR=/usr/local/bin

# Default target
all: build

# Build the binary
build:
	@echo "Building..."
	@cargo build --release

# Install the binary
install: build
	@echo "Installing to $(INSTALL_DIR)..."
	@sudo cp target/release/$(BINARY_NAME) $(INSTALL_DIR)

# Clean up the build artifacts
clean:
	@echo "Cleaning up..."
	@cargo clean

# Uninstall the binary
uninstall:
	@echo "Uninstalling from $(INSTALL_DIR)..."
	@rm -f $(INSTALL_DIR)/$(BINARY_NAME)

# Run the binary (useful for testing)
run:
	@echo "Running..."
	@cargo run --release

# Additional functionality: Check the code with clippy (Rust's linting tool)
lint:
	@echo "Linting..."
	@cargo clippy

# Additional functionality: Format the code with rustfmt
fmt:
	@echo "Formatting..."
	@cargo fmt

.PHONY: all build install clean uninstall run lint fmt
