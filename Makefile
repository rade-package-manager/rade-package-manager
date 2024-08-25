CARGO = cargo


install:
	$(CARGO) build --release
	mv ~/.comrade/build/target/release/knife ~/.comrade/bin/
