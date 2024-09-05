CARGO = cargo


install: Cargo.toml Cargo.lock
	$(CARGO) build --release
	mv ~/.comrade/build/target/release/rade ~/.comrade/bin/
