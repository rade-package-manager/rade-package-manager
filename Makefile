CARGO = cargo


install: Cargo.toml src/main.rs
   $(CARGO) build --release
	mv ~/.comrade/build/target/release/rade ~/.comrade/bin/
