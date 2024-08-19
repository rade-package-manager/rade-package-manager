CARGO = cargo
TARGET = knife

install:
	$(CARGO) build --release
	mv ~/.knife/build/target/release/knife ~/.knife/bin/
