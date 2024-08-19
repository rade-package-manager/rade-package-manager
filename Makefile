CARGO = cargo
TARGET = knife

install:
	$(CARGO) build --release
	mv ~/.knife/build/target/releases/knife ~/.knife/bin/
