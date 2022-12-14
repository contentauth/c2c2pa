OS := $(shell uname)

CFLAGS = -I. -Wall 
ifeq ($(OS), Darwin)
CFLAGS += -framework Security
endif
ifeq ($(OS), Linux)
CFLAGS = -pthread -Wl,--no-as-needed -ldl -lm
endif
release:
	cargo b --release
	$(CC) $(CFLAGS) src/main.c -o target/c2c2pa -lc2c2pa -L./target/release 

run: release
	./target/c2c2pa


release-linux-arm-static:
	rustup target add aarch64-unknown-linux-musl
	cargo build --target=aarch64-unknown-linux-musl --release
	$(CC) $(CFLAGS)  -static src/main.c -o target/c2c2pa -lc2c2pa -lm -L./target/aarch64-unknown-linux-musl/release && strip target/c2c2pa 

release-linux-x86-static:
	rustup target add x86_64-unknown-linux-musl
	cargo build --target=x86_64-unknown-linux-musl --release
	$(CC) $(CFLAGS) -static src/main.c -o target/c2c2pa -lc2c2pa -lm -L./target/x86_64-unknown-linux-musl/release && strip target/c2c2pa 

