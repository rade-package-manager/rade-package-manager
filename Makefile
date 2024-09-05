CARGO = cargo


install: Cargo.toml Cargo.lock src/main.rs
	$(CARGO) build --release
	mv ~/.comrade/build/target/release/rade ~/.comrade/bin/
