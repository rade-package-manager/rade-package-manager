CARGO = cargo
TARGET = knife

install: $(TARGET)
	$(CARGO) build --release
	mv ~/.knife/build/target/releases/knife ~/.knife/bin/
