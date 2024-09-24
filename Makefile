CARGO = cargo
BIN_DIR = ~/.comrade/bin
TARGET_DIR = target/release

install: Cargo.toml src/main.rs
	$(CARGO) build --release
	mkdir -p $(BIN_DIR)
	mv $(TARGET_DIR)/rade $(BIN_DIR)/
